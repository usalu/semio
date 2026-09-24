"""⏸️ Moves the `no-mutation` row of every `@id-mutate`/`@id-inverse` Scenario Outline into its own plain
`@id-no-mutation-baseline-<base>` Scenario, so the identity baseline stops posing as a catalog kind.

Usage: baseline-scenarios.py <feature>... (rewrites in place, prints what moved)."""
import re, sys

def split_row(line):
    return [cell.strip() for cell in line.strip().strip("|").split("|")]

def transform(text):
    lines = text.split("\n")
    out, i, moved = [], 0, 0
    while i < len(lines):
        line = lines[i]
        tag = re.match(r"^(\s*)@id-(mutate|inverse)\s*$", line)
        if not tag:
            out.append(line); i += 1; continue
        start = i
        j = i
        while not lines[j].strip().startswith("Scenario Outline:"):
            j += 1
        k = j + 1
        while not lines[k].strip().startswith("Examples:"):
            k += 1
        header_index = k + 1
        end = header_index + 1
        while end < len(lines) and lines[end].strip().startswith("|"):
            end += 1
        header = split_row(lines[header_index])
        rows = [(r, split_row(lines[r])) for r in range(header_index + 1, end)]
        baseline = [(r, cells) for r, cells in rows if cells[0] == "no-mutation"]
        if not baseline:
            out.extend(lines[start:end]); i = end; continue
        (row_index, cells), = baseline
        values = dict(zip(header, cells))
        block = [l for n, l in enumerate(lines[start:end], start) if n != row_index]
        out.extend(block)
        base = tag.group(2)
        tags = [l.replace(f"@id-{base}", f"@id-no-mutation-baseline-{base}") for l in lines[start:j]]
        title = lines[j].replace("Scenario Outline:", "Scenario:")
        steps = lines[j + 1:k]
        def fill(s):
            for key, value in values.items():
                s = s.replace(f"<{key}>", value)
            return s
        out.append("")
        out.extend(tags)
        out.append(fill(title))
        out.extend(fill(s) for s in steps)
        moved += 1
        i = end
    return "\n".join(out), moved

for path in sys.argv[1:]:
    text = open(path, encoding="utf-8").read()
    new, moved = transform(text)
    open(path, "w", encoding="utf-8").write(new)
    print(moved, path)
