"""🩹 `nested_patch!` blocks of the diff Rust keep one `set`, `within`, `optional` and `rows` section each, in the macro's required order; sections a layer appended a second time are merged in place (later item wins) and the block is rewritten in order."""
import re

ORDER = ("set", "within", "optional", "rows")
BLOCK = re.compile(r"^nested_patch! \{\n((?:.*\n)*?)\}\n", re.M)
HEAD = re.compile(r"^( *)(?:///[^\n]*\n *)*(\w+ for [\w:]+) \{\n", re.M)


def balanced(text, start):
    depth = 0
    for i in range(start, len(text)):
        depth += text[i] == "{"
        depth -= text[i] == "}"
        if depth == 0:
            return i
    raise ValueError("unbalanced block")


def split_top(body):
    items, depth, cur = [], 0, ""
    for ch, prev in zip(body, " " + body):
        if prev == "=" and ch == ">":
            cur += ch
            continue
        depth += ch in "<([{"
        depth -= ch in ">)]}"
        if ch == "," and depth == 0:
            items.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        items.append(cur.strip())
    return items


def key_of(item):
    return re.split(r"\s*(?:=>|:)", item, 1)[0].strip()


def sections(inner):
    """📑 (name, items) of every section in text order, duplicates included."""
    out, pos = [], 0
    for m in re.finditer(r"\b(set|within|optional|rows)\s*\{", inner):
        if m.start() < pos:
            continue
        end = balanced(inner, m.end() - 1)
        out.append((m.group(1), split_top(inner[m.end():end])))
        pos = end
    return out


def normalize_block(block):
    """🧹 One `Name for Row { sections }` body rewritten in the macro order; returns the text unchanged when it is already normal."""
    head = re.match(r"(\s*(?:///[^\n]*\n\s*)*\w+ for [\w:]+ \{)", block)
    if not head:
        return block
    inner = block[head.end():block.rindex("}")]
    secs = sections(inner)
    if [n for n, _ in secs] == [n for n in ORDER if any(n == m for m, _ in secs)] and len({n for n, _ in secs}) == len(secs):
        return block
    merged = {}
    for name, items in secs:
        slot = merged.setdefault(name, {})
        for item in items:
            slot[key_of(item)] = item
    indent = re.search(r"\n( +)\S", block[head.end():]).group(1) if re.search(r"\n( +)\S", block[head.end():]) else "        "
    lines = "".join(f"{indent}{n} {{ {', '.join(merged[n].values())} }}\n" if merged.get(n) else f"{indent}{n} {{  }}\n" for n in ORDER if n in merged)
    close = block.rindex("}")
    return block[:head.end()] + "\n" + lines + block[block.rfind("\n", 0, close) + 1:]


def normalize(text):
    """🩺 Every `nested_patch!` macro call of a Rust file normalised; text without such calls is returned as is."""
    out, last = [], 0
    for m in BLOCK.finditer(text):
        body = m.group(1)
        new = normalize_block(body)
        out.append(text[last:m.start(1)])
        out.append(new if new.endswith("\n") else new + "\n")
        last = m.end(1)
    out.append(text[last:])
    return "".join(out)


def run(ctx):
    import l_rust
    for path in l_rust.diff_tree_files(ctx):
        text = ctx.plan.read(path)
        if "nested_patch! {" in text and "macro_rules! nested_patch" not in text.split("nested_patch! {")[0][-200:]:
            new = normalize(text)
            if new != text:
                ctx.plan.write(path, new)
                ctx.plan.note(f"diff Rust: nested_patch! sections merged in place, order set, within, optional, rows ({path.rsplit('/', 2)[-2]})")
