#!/usr/bin/env python3
"""🧾️ S3-CLOSURE step 3: replace every plugin `protocol::Edit { .. }` one-item literal by
`ArtifactStoreOneItemLiveAuthority::next_edit(forward, inverse)` (design §20, census §(d) step 3).

Usage: python3 🧪️s3-closure-next-edit-sweep.py [--apply] <file.rs>...

A literal is converted only when every field is the authority-minted default (line, actor, sequence, clock, base version,
`<id>#0` mutation id, empty dependencies, ExactBaseOnly, no hash/kind/label/transaction, group None or the authority's).
A wrapper `fn` whose whole body is `let id = ..; <literal>` is deleted and its same-file calls are rewritten; an inline
literal is replaced in place and its now-unused `let id = ..;` is dropped. Anything else is reported, never touched.
"""
import re
import sys


def skip_string(text, i):
    """Returns the index after the string/char literal or comment starting at i, or None."""
    if text.startswith("//", i):
        end = text.find("\n", i)
        return len(text) if end < 0 else end
    if text.startswith("/*", i):
        return text.index("*/", i) + 2
    m = re.match(r'b?r(#*)"', text[i:])
    if m and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
        hashes = m.group(1)
        end = text.index('"' + hashes, i + m.end())
        return end + 1 + len(hashes)
    if text[i] == '"' or text.startswith('b"', i) and (i == 0 or not text[i - 1].isalnum()):
        j = i + (2 if text[i] == "b" else 1)
        while text[j] != '"':
            j += 2 if text[j] == "\\" else 1
        return j + 1
    if text[i] == "'":
        m = re.match(r"'(\\.|[^\\'])'", text[i:]) or re.match(r"'\\u\{[0-9a-fA-F]+\}'", text[i:])
        if m:
            return i + m.end()
    return None


def match_close(text, open_index):
    """Index of the bracket closing the one at open_index."""
    pairs = {"{": "}", "(": ")", "[": "]"}
    stack = [pairs[text[open_index]]]
    i = open_index + 1
    while stack:
        skipped = skip_string(text, i)
        if skipped is not None:
            i = skipped
            continue
        c = text[i]
        if c in pairs:
            stack.append(pairs[c])
        elif c in ")]}":
            assert c == stack.pop(), (text[open_index:i + 1][-200:])
        i += 1
    return i - 1


def split_top(body, sep=","):
    """Splits at top-level separators."""
    parts, depth, start, i = [], 0, 0, 0
    while i < len(body):
        skipped = skip_string(body, i)
        if skipped is not None:
            i = skipped
            continue
        c = body[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == sep and depth == 0:
            parts.append(body[start:i])
            start = i + 1
        i += 1
    parts.append(body[start:])
    return [part.strip() for part in parts if part.strip()]


def fields(body):
    out = {}
    for part in split_top(body):
        if ":" in part and not part.startswith("..") and re.match(r"^\w+\s*:", part) and not part.startswith("::"):
            key, value = part.split(":", 1)
            out[key.strip()] = value.strip()
        else:
            out[part] = part
    return out


def norm(value):
    return re.sub(r"\s+", "", value).replace("protocol::", "").replace("::store::", "store::")


def classify(literal_body):
    """Returns (authority, forward, inverse, group_from_authority) or a reason string."""
    f = fields(literal_body)
    line = norm(f.get("line", ""))
    m = re.match(r"^(.+)\.line_id\(\)\.map\(str::to_owned\)$", line)
    if not m:
        return f"line {f.get('line')!r}"
    a = m.group(1)
    if norm(f.get("actor", "")) != f"Some({a}.actor().to_string())" and norm(f.get("actor", "")) != f"Some({a}.actor().into())":
        return f"actor {f.get('actor')!r}"
    forwards = f.get("forwards", "")
    fm = re.match(r"^vec!\[(.*)\]$", forwards, re.S)
    if not fm or len(split_top(fm.group(1))) != 1:
        return f"forwards {forwards!r}"
    forward = fm.group(1).strip()
    inverse = f.get("inverse", "")
    for key, want in [("sequence_number", f"{a}.next_sequence_number()"), ("started_at", "String::new()"), ("finished_at", "None"), ("verb", "None"), ("coalesce_key", "None")]:
        if norm(f.get(key, "")) != norm(want):
            return f"{key} {f.get(key)!r}"
    if norm(f.get("id", "")) not in ("id", "id.clone()"):
        return f"id {f.get('id')!r}"
    meta = f.get("mutation_meta", "")
    mm = re.match(r"^vec!\[\s*(?:protocol::|::protocol::)?MutationMeta\s*\{(.*)\}\s*,?\s*\]$", meta, re.S)
    if not mm:
        return f"meta {meta[:80]!r}"
    g = fields(mm.group(1))
    expect = {
        "mutation_id": 'Some(MutationId(format!("{id}#0")))',
        "dependencies": "Vec::new()",
        "base_version": f"{a}.base_applied_edit_count()asu64",
        "author_id": f"Some(ActorId({a}.actor().to_string()))",
        "timestamp": f"{a}.next_clock()",
        "undo_policy": "UndoPolicy::ExactBaseOnly",
        "payload_hash": "None",
        "semantic_kind": "None",
        "label": "None",
        "origin": "Default::default()",
        "transaction": "None",
    }
    for key, want in expect.items():
        have = norm(g.get(key, "")).replace("::ActorId", "ActorId").replace("::MutationId", "MutationId").replace("::UndoPolicy", "UndoPolicy")
        if have != norm(want) and not (key == "author_id" and have == f"Some(ActorId({a}.actor().into()))"):
            return f"meta.{key} {g.get(key)!r}"
    group = norm(g.get("group_id", ""))
    if group not in ("None", f"{a}.group_id().map(str::to_owned)"):
        return f"meta.group_id {g.get('group_id')!r}"
    extra = set(f) - {"line", "id", "actor", "forwards", "inverse", "mutation_meta", "description", "verb", "coalesce_key", "sequence_number", "started_at", "finished_at"}
    if extra:
        return f"extra fields {sorted(extra)}"
    return (a, forward, inverse)


def enclosing_fn(text, pos):
    """(name, params_open, body_open, body_close, header_start) of the innermost fn whose body holds pos."""
    headers = list(re.finditer(r"\n[ \t]*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)\s*(?:<[^>{]*>)?\s*\(", text[:pos]))
    for header in reversed(headers):
        params_open = header.end() - 1
        params_close = match_close(text, params_open)
        body_open = text.find("{", params_close)
        if body_open < 0 or body_open > pos:
            continue
        body_close = match_close(text, body_open)
        if body_open < pos < body_close:
            return (header.group(1), params_open, body_open, body_close, header.start())
    return None


LITERAL = re.compile(r"(?<![\w:])(?:::)?(?:protocol::|store::)?Edit\s*\{\s*line\s*:")


def sweep(path, apply):
    text = open(path, encoding="utf-8").read()
    original = text
    report = []
    while True:
        m = LITERAL.search(text)
        if not m:
            break
        open_brace = text.index("{", m.start())
        close = match_close(text, open_brace)
        verdict = classify(text[open_brace + 1:close])
        line_no = text[:m.start()].count("\n") + 1
        if isinstance(verdict, str):
            report.append(f"SKIP {path}:{line_no} {verdict}")
            e = text.index("Edit", m.start()); text = text[:e] + "⟦EDIT⟧" + text[e + 4:]
            continue
        authority, forward, inverse = verdict
        enclosing = enclosing_fn(text, m.start())
        fn_name, params_open, body_open, body_close, _ = enclosing if enclosing else (None, None, None, None, None)
        before = text[body_open + 1:m.start()] if enclosing else "x"
        after = text[close + 1:body_close] if enclosing else "x"
        wrapper = re.fullmatch(r"\s*(let\s+id\s*=[^;]*;\s*)?", before) is not None and after.strip() == ""
        fn_start = enclosing[4] if enclosing else None
        if wrapper:
            params = split_top(text[params_open + 1:match_close(text, params_open)])
            names = [p.split(":", 1)[0].strip() for p in params]
            used = set(re.findall(r"\w+", f"{forward} {inverse} {authority}")) & set(names)
            if authority not in names or not used:
                report.append(f"SKIP {path}:{line_no} wrapper {fn_name} args not plain params {names}")
                e = text.index("Edit", m.start()); text = text[:e] + "⟦EDIT⟧" + text[e + 4:]
                continue
            doc_start = fn_start
            while True:
                prev = text.rfind("\n", 0, doc_start)
                line = text[prev + 1:doc_start]
                if line.strip().startswith("///") or line.strip().startswith("#["):
                    doc_start = prev
                else:
                    break
            text = text[:doc_start] + text[body_close + 1:]
            text = text[:doc_start] + re.sub(r"^\n\n+", "\n", text[doc_start:], count=1) if text[doc_start:doc_start + 2] == "\n\n" and text[doc_start - 1:doc_start] == "\n" else text
            calls = 0
            for call in list(re.finditer(rf"(?<![\w:]){fn_name}\s*\(", text))[::-1]:
                open_paren = call.end() - 1
                close_paren = match_close(text, open_paren)
                args = split_top(text[open_paren + 1:close_paren])
                bound = dict(zip(names, args))
                bind = lambda expr: re.sub(r"(?<![\w.])(\w+)(?!\w)", lambda word: bound.get(word.group(1), word.group(1)), expr)
                replacement = f"{bind(authority).lstrip('&')}.next_edit({bind(forward)}, {bind(inverse)})"
                text = text[:call.start()] + replacement + text[close_paren + 1:]
                calls += 1
            report.append(f"WRAPPER {path}:{line_no} fn {fn_name} deleted, {calls} call(s) rewritten")
        else:
            replacement = f"{authority}.next_edit({forward}, {inverse})"
            text = text[:m.start()] + replacement + text[close + 1:]
            if enclosing:
                body_open = text.index("{", match_close(text, params_open))
                body_close = match_close(text, body_open)
                body = text[body_open + 1:body_close]
                let = re.search(r"\n\s*let\s+id\s*=[^;]*;", body)
                if let and not re.search(r"(?<![\w.])id(?!\w)", body[:let.start()] + body[let.end():]):
                    text = text[:body_open + 1 + let.start()] + text[body_open + 1 + let.end():]
            report.append(f"INLINE {path}:{line_no} in fn {fn_name}")
    text = text.replace("⟦EDIT⟧", "Edit")
    for line in report:
        print(line)
    if apply and text != original:
        open(path, "w", encoding="utf-8").write(text)


if __name__ == "__main__":
    apply = "--apply" in sys.argv
    for path in [arg for arg in sys.argv[1:] if arg != "--apply"]:
        sweep(path, apply)
