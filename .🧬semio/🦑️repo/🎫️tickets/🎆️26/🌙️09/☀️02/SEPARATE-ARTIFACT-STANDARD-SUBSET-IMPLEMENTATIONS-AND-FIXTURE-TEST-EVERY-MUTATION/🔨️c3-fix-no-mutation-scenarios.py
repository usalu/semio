#!/usr/bin/env python3
"""Extract the 'no-mutation' control row out of mutate-/inverse- Scenario Outlines into standalone
Scenarios, so it stops tripping mutation-kind-undeclared (needs the id declared) or test-only-mutation
(no manifest can honestly claim a dispatch that dsl::Mutations forbids) no matter which way the v1
catalog's kinds list goes."""
import re, sys

def process(path, dry=False):
    with open(path, encoding="utf-8") as f:
        lines = f.read().split("\n")

    # Find blocks: locate "Scenario Outline:" lines, walk back to collect tags, walk fwd to Examples end.
    blocks = []  # (tag_start_idx, scenario_line_idx, examples_end_idx_exclusive, kind: 'mutate'/'inverse')
    i = 0
    while i < len(lines):
        line = lines[i]
        if re.match(r"^\s*Scenario Outline:", line):
            # collect tags backward
            tag_start = i
            j = i - 1
            tags = []
            while j >= 0 and re.match(r"^\s*@\S+\s*$", lines[j]):
                tags.append(lines[j].strip())
                tag_start = j
                j -= 1
            kind = None
            if "@id-mutate" in tags:
                kind = "mutate"
            elif "@id-inverse" in tags:
                kind = "inverse"
            if kind is not None:
                # find "Examples:" line
                k = i + 1
                while k < len(lines) and not re.match(r"^\s*Examples:\s*$", lines[k]):
                    k += 1
                if k < len(lines):
                    ex_header = k + 1
                    ex_row = ex_header + 1
                    # find no-mutation row and end of table (blank line or next non-'|' line)
                    end = ex_row
                    nomut_idx = None
                    while end < len(lines) and lines[end].strip().startswith("|"):
                        if re.match(r"^\s*\|\s*no-mutation\s*\|", lines[end]):
                            nomut_idx = end
                        end += 1
                    if nomut_idx is not None:
                        blocks.append({
                            "tag_start": tag_start,
                            "scenario_idx": i,
                            "examples_idx": k,
                            "table_end": end,  # exclusive
                            "nomut_idx": nomut_idx,
                            "kind": kind,
                        })
            i = i + 1 if 'k' not in dir() else i + 1
        i += 1

    if not blocks:
        return False

    # Process blocks in reverse order (by position) so earlier edits don't shift later indices.
    blocks.sort(key=lambda b: b["tag_start"])
    new_lines = list(lines)
    insertions = []  # (insert_after_idx_in_ORIGINAL, block_text)
    deletions = []   # line idx (in ORIGINAL) to delete (the no-mutation row)

    level_re = re.compile(r"^\s*@level-\S+\s*$")
    mode_re = re.compile(r"^\s*@mode-\S+\s*$")

    used_ids = set()
    for b in blocks:
        # capture every column's cell value from the no-mutation row, keyed by the header row's
        # column names -- substitution in Gherkin step text is `<columnName>`, not fixed `<id>`/`<params>`.
        header_line = lines[b["examples_idx"] + 1]
        header_cells = [c.strip() for c in header_line.strip().strip("|").split("|")]
        row_line = lines[b["nomut_idx"]]
        row_cells = [c.strip() for c in row_line.strip().strip("|").split("|")]
        row = dict(zip(header_cells, row_cells))
        row.setdefault("id", "no-mutation")
        params_val = row.get("params", "{}")

        tag_lines = lines[b["tag_start"]:b["scenario_idx"]]
        level_tag = next((t.strip() for t in tag_lines if level_re.match(t)), "@level-fundamental")
        mode_tag = next((t.strip() for t in tag_lines if mode_re.match(t)), "@mode-property")

        def substitute(text):
            for key, value in row.items():
                text = text.replace(f"<{key}>", value)
            return text

        scenario_title_line = lines[b["scenario_idx"]]
        indent_match = re.match(r"^(\s*)", scenario_title_line)
        indent = indent_match.group(1) if indent_match else "  "
        new_title = substitute(scenario_title_line.replace("Scenario Outline:", "Scenario:"))

        step_lines_raw = lines[b["scenario_idx"] + 1 : b["examples_idx"]]
        # drop trailing blank lines right before Examples:
        while step_lines_raw and step_lines_raw[-1].strip() == "":
            step_lines_raw = step_lines_raw[:-1]
        new_steps = [substitute(sl) for sl in step_lines_raw]

        base_id = f"no-mutation-baseline-{b['kind']}"
        n = base_id
        suffix = 2
        while n in used_ids:
            n = f"{base_id}-{suffix}"
            suffix += 1
        used_ids.add(n)

        block_text_lines = [
            "",
            f"{indent}@id-{n}",
            f"{indent}{level_tag}",
            f"{indent}{mode_tag}",
            new_title,
        ] + new_steps

        insertions.append((b["table_end"] - 1, "\n".join(block_text_lines)))
        deletions.append(b["nomut_idx"])

    # Apply deletions first (mark None), then rebuild with insertions.
    result = []
    insert_map = {}
    for idx, text in insertions:
        insert_map.setdefault(idx, []).append(text)
    delset = set(deletions)
    for idx, line in enumerate(lines):
        if idx in delset:
            continue
        result.append(line)
        if idx in insert_map:
            for text in insert_map[idx]:
                result.append(text)

    new_content = "\n".join(result)
    if dry:
        print(new_content)
        return True
    with open(path, "w", encoding="utf-8") as f:
        f.write(new_content)
    return True

if __name__ == "__main__":
    args = sys.argv[1:]
    dry = "--dry" in args
    args = [a for a in args if a != "--dry"]
    for p in args:
        ok = process(p, dry=dry)
        print(("DRY " if dry else "") + ("OK " if ok else "SKIP(no blocks found) ") + p, file=sys.stderr)
