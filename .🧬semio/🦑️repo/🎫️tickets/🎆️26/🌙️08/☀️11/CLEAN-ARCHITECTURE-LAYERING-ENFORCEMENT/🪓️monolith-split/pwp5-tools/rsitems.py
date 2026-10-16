"""🧩 Top-level Rust item splitter for the ui-scene carve: a small brace-aware lexer that cuts a file into attribute/doc-prefixed top-level items and region markers."""
import re
from dataclasses import dataclass

@dataclass
class Item:
    start: int
    end: int
    kind: str
    name: str
    text: str


def scan_depth(text, depth, state):
    i = 0
    n = len(text)
    while i < n:
        c = text[i]
        if state.get("block"):
            if text.startswith("*/", i):
                state["block"] -= 1
                i += 2
                continue
            if text.startswith("/*", i):
                state["block"] += 1
                i += 2
                continue
            i += 1
            continue
        if state.get("raw") is not None:
            term = '"' + "#" * state["raw"]
            j = text.find(term, i)
            if j < 0:
                return depth
            i = j + len(term)
            state["raw"] = None
            continue
        if state.get("str"):
            if c == "\\":
                i += 2
                continue
            if c == '"':
                state["str"] = False
            i += 1
            continue
        if text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
            continue
        if text.startswith("/*", i):
            state["block"] = 1
            i += 2
            continue
        m = re.match(r'b?r(#*)"', text[i:i + 12])
        if m and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            state["raw"] = len(m.group(1))
            i += m.end()
            continue
        if c == '"':
            state["str"] = True
            i += 1
            continue
        if c == "'":
            m = re.match(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\}|.)|[^\\'\n])'", text[i:i + 14])
            if m:
                i += m.end()
                continue
            i += 1
            continue
        if c in "{([":
            depth += 1
        elif c in "})]":
            depth -= 1
        i += 1
    return depth


KINDS = re.compile(r"^(?:pub(?:\([^)]*\))?\s+)?(?:(?:const|unsafe|async|extern\s+\"[^\"]*\")\s+)*(fn|struct|enum|trait|type|const|static|mod|use|impl|union|macro_rules!|thread_local!)\b\s*<?(\w*)")


def split_items(text):
    lines = text.split("\n")
    items = []
    i = 0
    n = len(lines)
    while i < n:
        line = lines[i]
        if not line.strip():
            i += 1
            continue
        start = i
        if re.match(r"\s*//#(end)?region", line):
            items.append(Item(i, i, "marker", line.strip(), line))
            i += 1
            continue
        if line.lstrip().startswith("//!"):
            items.append(Item(i, i, "innerdoc", line.strip(), line))
            i += 1
            continue
        j = i
        state = {}
        depth = 0
        kind = None
        name = ""
        while j < n:
            l = lines[j]
            if depth == 0 and kind is None:
                s = l.strip()
                if not s:
                    j += 1
                    continue
                if s.startswith("/*"):
                    while "*/" not in lines[j]:
                        j += 1
                    j += 1
                    continue
                if s.startswith("#[") or s.startswith("///") or s.startswith("//!") or s.startswith("//") :
                    if s.startswith("#["):
                        depth = scan_depth(l + "\n", depth, state)
                        if depth != 0:
                            j += 1
                            while depth != 0 and j < n:
                                depth = scan_depth(lines[j] + "\n", depth, state)
                                j += 1
                            continue
                    j += 1
                    continue
                m = KINDS.match(s)
                if m:
                    kind = m.group(1)
                    name = m.group(2)
                    if kind == "impl":
                        name = s
                else:
                    kind = "other"
                    name = s
            depth = scan_depth(l + "\n", depth, state)
            j += 1
            if depth == 0 and not state.get("str") and not state.get("block") and state.get("raw") is None:
                stripped = l.rstrip()
                if stripped.endswith(";") or stripped.endswith("}"):
                    break
                if kind in ("impl", "fn", "struct", "enum", "trait", "mod", "other") and not stripped.endswith(("(", ",", "=", "{", "->", "+", "&&", "||", ":", "where")) and (j >= n or not lines[j].strip().startswith(("{", "where", ".", "->"))):
                    if "{" in "\n".join(lines[start:j]):
                        break
        items.append(Item(start, j - 1, kind or "other", name, "\n".join(lines[start:j])))
        i = j
    return items


def regions(text):
    lines = text.split("\n")
    out = {}
    stack = []
    for i, l in enumerate(lines):
        m = re.match(r"\s*//#region\s+(.*?)\s*$", l)
        if m:
            stack.append((m.group(1), i))
            continue
        m = re.match(r"\s*//#endregion\s+(.*?)\s*$", l)
        if m:
            name, s = stack.pop()
            out[name] = (s, i)
    return out


def region_text(text, name):
    s, e = regions(text)[name]
    return "\n".join(text.split("\n")[s:e + 1])

if __name__ == "__main__":
    import sys
    t = open(sys.argv[1], encoding="utf-8").read()
    for it in split_items(t):
        print(it.start + 1, it.end + 1, it.kind, it.name[:60])
