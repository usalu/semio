#!/usr/bin/env python3
import re, sys, json

SUPPORTED_KEYS = {
    "rename_all", "tag", "content", "default", "deny_unknown_fields",
    "transparent", "bound", "rename", "skip_serializing_if",
    "serialize_with", "deserialize_with",
}
DROP_KEYS = {"rename_all_fields"}

def parse_serde_args(inner):
    # split on top-level commas (no nested parens expected in these attrs)
    parts = []
    depth = 0
    cur = ""
    for ch in inner:
        if ch == '(' or ch == '[':
            depth += 1
        elif ch == ')' or ch == ']':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())
    return parts

def translate_serde_attr(line):
    # returns (value_line_or_None, unsupported_bool, is_skip_bool)
    m = re.search(r'#\[serde\((.*)\)\]\s*$', line.strip())
    if not m:
        return None, False, False
    inner = m.group(1)
    parts = parse_serde_args(inner)
    out_parts = []
    unsupported = False
    is_skip = False
    for p in parts:
        key = p.split('=')[0].strip()
        if key == "skip":
            is_skip = True
            unsupported = True
            continue
        if key in DROP_KEYS:
            continue
        if key not in SUPPORTED_KEYS:
            unsupported = True
            continue
        out_parts.append(p)
    if unsupported:
        return None, True, is_skip
    if not out_parts:
        return None, False, False
    indent = re.match(r'^(\s*)', line).group(1)
    return f"{indent}#[value({', '.join(out_parts)})]", False, False

def add_to_derive(line):
    has_ser = re.search(r'\bSerialize\b', line) is not None
    has_de = re.search(r'\bDeserialize\b', line) is not None
    if not has_ser:
        return line, False
    if 'ToValue' in line:
        return line, False
    new_line = line
    if has_de:
        new_line = re.sub(r'\bDeserialize\b', 'Deserialize, FromValue', new_line, count=1)
    new_line = re.sub(r'\bSerialize\b(?!,\s*FromValue)', 'Serialize, ToValue', new_line, count=1)
    return new_line, True

def process_file(path, apply=False):
    with open(path, encoding='utf-8') as f:
        lines = f.readlines()

    n = len(lines)
    i = 0
    edits = []  # list of (line_index, 'replace'|'insert_after', text)
    report = {"converted": [], "skipped_tuple": [], "skipped_unsupported": [], "skipped_composed": []}

    while i < n:
        line = lines[i]
        if re.search(r'#\[derive\(', line) and re.search(r'\bSerialize\b', line) and 'ToValue' not in line:
            derive_idx = i
            derive_indent = re.match(r'^(\s*)', line).group(1)
            # gather full derive(...) possibly multi-line - assume single line (verified pattern in this repo)
            # scan forward for attribute lines and item line
            j = i + 1
            attr_lines = []  # (idx)
            item_idx = None
            item_kind = None
            while j < n:
                l = lines[j]
                stripped = l.strip()
                if stripped.startswith('#[') or stripped.startswith('///') or stripped.startswith('//!'):
                    if stripped.startswith('#[serde('):
                        attr_lines.append(j)
                    j += 1
                    continue
                if re.match(r'^\s*(pub(\([^)]*\))?\s+)?(struct|enum)\s+\w', l):
                    item_idx = j
                    item_kind = 'struct' if re.search(r'\bstruct\b', l) else 'enum'
                    break
                # blank line, keep scanning (rare) else bail
                if stripped == "":
                    j += 1
                    continue
                break
            if item_idx is None:
                i += 1
                continue

            item_line = lines[item_idx]
            name_m = re.search(r'(?:struct|enum)\s+(\w+)', item_line)
            name = name_m.group(1) if name_m else "?"

            # detect tuple struct: struct Name(...)  before any '{'
            after_name = item_line.split(name, 1)[1] if name in item_line else ""
            is_tuple_struct = item_kind == 'struct' and re.search(r'^\s*\(', after_name.lstrip()) is not None
            # need to also handle case where generics come before '(' e.g. struct Name<T>(T)
            if item_kind == 'struct' and not is_tuple_struct:
                # strip generics <...> then check next non-space char
                stripped_after = re.sub(r'^\s*<[^>]*>', '', after_name)
                stripped_after = stripped_after.lstrip()
                if stripped_after.startswith('('):
                    is_tuple_struct = True

            if is_tuple_struct:
                report["skipped_tuple"].append((path, derive_idx + 1, name))
                i = item_idx + 1
                continue

            # find matching closing brace for the item body, scanning from item_idx
            # find first '{' at or after item_idx
            body_start = None
            depth = 0
            k = item_idx
            found_open = False
            while k < n:
                for ch in lines[k]:
                    if ch == '{':
                        depth += 1
                        found_open = True
                    elif ch == '}':
                        depth -= 1
                        if found_open and depth == 0:
                            body_end = k
                            break
                else:
                    k += 1
                    continue
                break
            if not found_open:
                # unit struct with no body, e.g. `struct Name;`
                body_end = item_idx
            else:
                body_end = k

            # collect all #[serde(...)] lines within derive_idx..body_end inclusive (container + fields/variants)
            serde_line_idxs = [x for x in attr_lines]
            for idx2 in range(item_idx, body_end + 1):
                if idx2 in serde_line_idxs:
                    continue
                if re.search(r'#\[serde\(', lines[idx2]):
                    serde_line_idxs.append(idx2)
            serde_line_idxs = sorted(set(serde_line_idxs))

            unsupported_found = False
            skip_found = False
            for idx2 in serde_line_idxs:
                _, unsupported, is_skip = translate_serde_attr(lines[idx2])
                if unsupported:
                    unsupported_found = True
                if is_skip:
                    skip_found = True

            if unsupported_found:
                report["skipped_unsupported"].append((path, derive_idx + 1, name))
                i = body_end + 1
                continue

            # queue derive line replacement
            new_derive, changed = add_to_derive(lines[derive_idx])
            if changed:
                edits.append((derive_idx, 'replace', new_derive))

            # queue value(...) insertions after each serde line, only if next non-consumed line isn't already #[value(
            for idx2 in serde_line_idxs:
                nxt = lines[idx2 + 1] if idx2 + 1 < n else ""
                if nxt.strip().startswith('#[value('):
                    continue
                value_line, unsupported, _ = translate_serde_attr(lines[idx2])
                if value_line is not None:
                    edits.append((idx2, 'insert_after', value_line + "\n"))

            report["converted"].append((path, derive_idx + 1, name))
            i = body_end + 1
            continue
        i += 1

    if apply and edits:
        # apply from bottom to top
        edits.sort(key=lambda e: e[0], reverse=True)
        for idx, kind, text in edits:
            if kind == 'replace':
                lines[idx] = text if text.endswith("\n") else text + "\n"
            elif kind == 'insert_after':
                lines.insert(idx + 1, text)
        with open(path, 'w', encoding='utf-8') as f:
            f.writelines(lines)

    return report

if __name__ == '__main__':
    apply = '--apply' in sys.argv
    args = [a for a in sys.argv[1:] if a != '--apply']
    files = []
    for a in args:
        if a.startswith('@'):
            with open(a[1:], encoding='utf-8') as lf:
                files.extend([l.strip() for l in lf if l.strip()])
        else:
            files.append(a)
    total_report = {"converted": [], "skipped_tuple": [], "skipped_unsupported": [], "skipped_composed": []}
    for path in files:
        r = process_file(path, apply=apply)
        for k in total_report:
            total_report[k].extend(r[k])
    print(json.dumps(total_report, ensure_ascii=False, indent=2))
