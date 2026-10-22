"""🩹️ Independent Python oracle of the tool patch rules (standard library only): it decides admission of a patch, the lines it adds per file, the exact post-image of one file and the sha256 receipt, from the written unified-diff rules. Requests arrive as JSON on stdin."""
import hashlib, json, re, sys

HEADER = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")
FILE = re.compile(r"^diff --git a/(\S+) b/(\S+)$")


def bad_path(path):
    return path.startswith("/") or any(ch in path for ch in ":\\\0") or any(part in ("", ".", "..") for part in path.split("/"))


def validate(patch):
    rows = patch.split("\n")
    seen, current, delta, old_end, new_end, hunks, index = set(), None, 0, 0, 0, 0, 0
    while index < len(rows):
        row = rows[index]
        if row.startswith("diff --git "):
            match = FILE.match(row)
            if not match or match.group(1) != match.group(2):
                return "Original tooling patch file required"
            current = match.group(1)
            if bad_path(current):
                return "Relative tooling patch file required"
            if current in seen:
                return "Tooling patch file repeated: " + current
            seen.add(current)
            delta = old_end = new_end = 0
        elif row.startswith("@@"):
            header = HEADER.match(row)
            if not current or not header:
                return "Original tooling patch hunk required"
            old_start, old_count = int(header.group(1)), int(header.group(2) if header.group(2) is not None else 1)
            new_start, new_count = int(header.group(3)), int(header.group(4) if header.group(4) is not None else 1)
            if max(old_start, old_count, new_start, new_count) > 2 ** 53 - 1:
                return "Bounded tooling patch coordinates required"
            if old_start < old_end or new_start < new_end or new_start + (1 if new_count == 0 else 0) != old_start + (1 if old_count == 0 else 0) + delta:
                return "Tooling patch cumulative coordinates changed"
            before = after = 0
            while index + 1 < len(rows) and not rows[index + 1].startswith("diff --git ") and not rows[index + 1].startswith("@@"):
                index += 1
                line = rows[index]
                if line.startswith(" "):
                    before += 1
                    after += 1
                elif line.startswith("-"):
                    before += 1
                elif line.startswith("+"):
                    after += 1
                elif line.startswith("\\ No newline at end of file"):
                    continue
                elif line == "" and (index + 1 == len(rows) or before == old_count and after == new_count):
                    continue
                else:
                    return "Tooling patch unprefixed hunk line"
            if before != old_count or after != new_count:
                return "Tooling patch hunk counts changed"
            old_end, new_end, delta, hunks = old_start + old_count, new_start + new_count, delta + new_count - old_count, hunks + 1
        index += 1
    if not seen or not hunks:
        return "Original tooling patch sections required"
    return ""


def additions(patch):
    files, current = {}, None
    for row in patch.split("\n"):
        if row.startswith("+++ b/"):
            path = row[6:]
            if not path or path.startswith("/") or "\\" in path or "\0" in path or any(part in ("", ".", "..") for part in path.split("/")):
                return {"error": "Relative tool patch file required"}
            current = files.setdefault(path, set())
        elif row.startswith("diff --git") or row.startswith("--- "):
            current = None
        elif current is not None and row.startswith("+") and row[1:].strip():
            current.add(row[1:])
        elif current is not None and row.startswith("-"):
            current.discard(row[1:])
    if not files:
        return {"error": "Owned tool patch file required"}
    return {"files": {path: sorted(lines) for path, lines in files.items()}}


def split_lines(text):
    pieces = text.split("\n")
    if text.endswith("\n"):
        pieces.pop()
    return [[line, index < len(pieces) - 1 or text.endswith("\n")] for index, line in enumerate(pieces)] if text != "" else []


def apply(before, patch):
    if validate(patch):
        return None
    original, result, cursor = split_lines(before), [], 0
    rows = patch.split("\n")
    index = 0
    while index < len(rows):
        header = HEADER.match(rows[index])
        index += 1
        if not header:
            continue
        old_count = int(header.group(2) if header.group(2) is not None else 1)
        start = int(header.group(1)) - (0 if old_count == 0 else 1)
        if start < cursor or start > len(original):
            return None
        result.extend(original[cursor:start])
        cursor = start
        operations = []
        while index < len(rows) and not rows[index].startswith("diff --git ") and not rows[index].startswith("@@"):
            row = rows[index]
            index += 1
            if row.startswith("\\ No newline"):
                operations[-1][2] = False
            elif row[:1] in (" ", "+", "-"):
                operations.append([row[0], row[1:], True])
        for kind, text, newline in operations:
            if kind != "+":
                if cursor >= len(original) or original[cursor] != [text, newline]:
                    return None
                cursor += 1
            if kind != "-":
                result.append([text, newline])
    result.extend(original[cursor:])
    return "".join(line + ("\n" if newline else "") for line, newline in result)


def main():
    request = json.load(sys.stdin)
    op = request["op"]
    if op == "validate":
        print(json.dumps([validate(patch) for patch in request["patches"]]))
    elif op == "additions":
        print(json.dumps([additions(patch) for patch in request["patches"]]))
    elif op == "apply":
        print(json.dumps([apply(row["before"], row["patch"]) for row in request["rows"]]))
    elif op == "sha256":
        print(json.dumps([hashlib.sha256(text.encode("utf8")).hexdigest() for text in request["texts"]]))
    else:
        raise SystemExit("unknown op " + op)


main()
