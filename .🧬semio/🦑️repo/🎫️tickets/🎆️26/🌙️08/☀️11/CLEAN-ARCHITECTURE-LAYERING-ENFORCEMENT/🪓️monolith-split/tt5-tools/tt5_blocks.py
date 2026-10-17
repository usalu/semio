"""Group tsc output into diagnostic blocks (first line plus indented continuation lines)."""
import re, sys

def blocks(path):
    out = []
    for line in open(path, encoding="utf8").read().split("\n"):
        m = re.match(r"(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)", line)
        if m:
            out.append({"file": m.group(1), "line": int(m.group(2)), "col": int(m.group(3)), "code": m.group(4), "msg": m.group(5), "text": line})
        elif line.startswith(" ") and out:
            out[-1]["text"] += "\n" + line
    return out

if __name__ == "__main__":
    needle = sys.argv[2]
    files = sorted({b["file"] for b in blocks(sys.argv[1]) if needle in b["text"]})
    print("\n".join(files))


def offset(text, starts, line, col):
    """Python string offset of a TypeScript (line, column): columns count UTF-16 code units, not code points."""
    base = starts[line - 1]
    units = 0
    index = base
    while units < col - 1:
        units += 2 if ord(text[index]) > 0xFFFF else 1
        index += 1
    return index
