"""📚 Shared helpers of the ui-scene carve: paths, API extraction, use-tree rendering and staged file writing."""
import os, re, shutil
from rsitems import split_items, regions

REPO = "/Users/ueli/Documents/semio"
SCENE_REL = "🧰️framework/🔨️modules/🖱️ui/🎬️scene"
SP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uiscene"
BASE = SP + "/base/🎬️scene"
NEW = SP + "/new/🎬️scene"


def rd(path):
    return open(path, encoding="utf-8").read()


def base(rel):
    return rd(BASE + "/" + rel)


def wr(root, rel, text):
    path = root + "/" + rel
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(text)


def fresh_new():
    shutil.rmtree(SP + "/new", ignore_errors=True)
    os.makedirs(SP + "/new")
    shutil.copytree(BASE, NEW)


PUB_KINDS = ("fn", "struct", "enum", "trait", "type", "const", "static", "mod", "union")


def top_items(text):
    return [it for it in split_items(text) if it.kind not in ("marker",)]


def is_pub(it):
    return bool(re.match(r"^\s*pub\s", _strip_attrs(it.text))) and not re.match(r"^\s*pub\s*\(", _strip_attrs(it.text))


def _strip_attrs(text):
    out = []
    started = False
    depth = 0
    inblock = False
    for l in text.split("\n"):
        s = l.strip()
        if not started and inblock:
            if "*/" in s:
                inblock = False
            continue
        if not started and s.startswith("/*"):
            if "*/" not in s:
                inblock = True
            continue
        if not started:
            if s.startswith("//") or not s:
                continue
            if s.startswith("#["):
                depth += s.count("[") - s.count("]")
                if depth > 0:
                    continue
                continue
            if depth > 0:
                depth += s.count("[") - s.count("]")
                continue
            started = True
        out.append(l)
    return "\n".join(out)


def use_names(stmt):
    stmt = re.sub(r"\s+", " ", stmt.strip().rstrip(";"))
    stmt = re.sub(r"^pub(\([^)]*\))?\s+", "", stmt)
    assert stmt.startswith("use "), stmt
    return flatten(stmt[4:].strip())


def flatten(tree, prefix=""):
    tree = tree.strip()
    if "{" not in tree:
        full = prefix + tree
        m = re.match(r"(.*?)\s+as\s+(\w+)$", full)
        if m:
            return [(m.group(2), m.group(1))]
        return [(full.split("::")[-1], full)]
    head, rest = tree.split("{", 1)
    inner = rest[: rest.rindex("}")]
    parts, depth, cur = [], 0, ""
    for ch in inner:
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    out = []
    for p in parts:
        p = p.strip()
        if p == "self":
            out.append((prefix.rstrip(":").split("::")[-1] if not head else head.rstrip(":").split("::")[-1], prefix + head.rstrip(":")))
            continue
        out += flatten(p, prefix + head)
    return out


def api_names(text):
    names = {}
    for it in top_items(text):
        if it.kind in PUB_KINDS and is_pub(it):
            names[it.name] = it.kind
        elif it.kind == "other" and it.name.startswith("string_enum!"):
            m = re.search(r"string_enum!\s*\{(?:\s*///[^\n]*\n|\s*#\[[^\n]*\]\n)*\s*(\w+)\s*\{", it.text)
            names[m.group(1)] = "enum"
        elif it.kind == "use" and is_pub(it):
            for n, full in use_names(_strip_attrs(it.text)):
                names[n] = "use"
    return names


def private_names(text):
    names = {}
    for it in top_items(text):
        if it.kind in PUB_KINDS and not is_pub(it):
            names[it.name] = it.kind
    return names


def render_use(paths, pub=False):
    paths = sorted(set(paths))
    trie = {}
    for p in paths:
        node = trie
        for seg in p.split("::"):
            node = node.setdefault(seg, {})

    def render(node):
        parts = []
        for k in sorted(node):
            sub = node[k]
            if not sub:
                parts.append(k)
            elif len(sub) == 1 and False:
                pass
            else:
                inner = render(sub)
                if len(sub) == 1:
                    parts.append(k + "::" + inner)
                else:
                    parts.append(k + "::{" + inner + "}")
        return ", ".join(parts)

    out = []
    for k in sorted(trie):
        sub = trie[k]
        if len(sub) == 1:
            (kk, vv), = sub.items()
            body = render(sub)
            out.append(("pub " if pub else "") + "use " + k + "::" + body + ";")
        else:
            out.append(("pub " if pub else "") + "use " + k + "::{" + render(sub) + "};")
    return "\n".join(out)


def idents(text):
    return set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", text))


def strip_comments(text):
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return re.sub(r"//[^\n]*", "", text)


FMT = ["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--config-path", REPO + "/rustfmt.toml", "--config", "skip_children=true,unstable_features=true", "--emit", "stdout"]


def rustfmt_text(text):
    import subprocess
    p = subprocess.run(FMT, input=text, capture_output=True, text=True)
    if p.returncode != 0:
        raise RuntimeError("rustfmt failed: " + p.stderr[:600] + "\n---\n" + text[:1500])
    out = p.stdout
    if out.startswith("<stdin>:") or out.startswith("stdin:"):
        out = out.split("\n", 2)[2] if out.count("\n") > 2 else out
    return out


def fmt_header(text):
    out = rustfmt_text(text)
    lines = out.split("\n")
    if lines and re.match(r"^(<stdin>|stdin):?\s*$", lines[0].strip()):
        lines = lines[1:]
    if lines and lines[0] == "":
        lines = lines[1:]
    return "\n".join(lines).rstrip("\n") + "\n"
