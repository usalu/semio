#!/usr/bin/env python3
"""✂️ Cuts one `#[path]`-tree artifact crate into its census layer crates (wave E3b, step 2).

The monolith keeps every module at `standards::…`/`editor::…`/`viewer::…`/`examples::…`. Each layer crate gets a root
`📦️packages/🦀️rust/🦀️.rs` that mounts exactly its layer's files at the SAME module paths, so the cut is a crate-prefix
rewrite: every `crate::`/`super::`/`self::` path (and every root-relative `standards::…` path in the aggregator root)
whose target file lives in another layer becomes `<that layer's crate>::<absolute module path>`; every external
`<monolith crate>::…` path is retargeted to the crate that owns its item. Nothing is re-exported.

usage: ✂️crate-cut.py <config.json> plan|apply
config: {"artifact": <artifact dir>, "layers": <census layers json>, "crate": <monolith crate ident>,
         "crates": {<layer>: {"package": <name>, "dir": <package dir relative to artifact>}}, "aggregator": <layer>,
         "header": [<crate-root attribute and extern lines>], "consumers": [<repo-relative roots to scan>]}
"""
import json, os, re, sys, collections

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CENSUS = os.path.join(HERE, "🕸️module-graph.py")
census = {}
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), census)
def strip(src):
    """🧹️ Blanks comments, string, raw-string and char literals while keeping every byte offset (the census strip
    shortens raw strings by two bytes, which would shift every later rewrite)."""
    out, i, n = [], 0, len(src)
    blank = lambda text: re.sub(r"[^\n]", " ", text)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i))
            i = j
        elif src.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif src.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            out.append(blank(src[i:j]))
            i = j
        elif c == "r" and re.match(r'r#*"', src[i:i + 40]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = re.match(r'r(#*)"', src[i:])
            close = '"' + m.group(1)
            j = src.find(close, i + len(m.group(0)))
            j = n if j < 0 else j + len(close)
            out.append('""' + blank(src[i + 2:j]))
            i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            j += 1
            keep = re.search(r"(?:#\[path\s*=|\binclude\s*!\s*\()\s*$", "".join(out[-12:]))
            out.append(src[i:j] if keep else '"' + blank(src[i + 1:j - 1]) + '"')
            i = j
        elif c == "'" and (re.match(r"'(\\.|[^\\'])'", src[i:i + 4]) or re.match(r"'\\(x..|u\{[0-9a-fA-F]+\})'", src[i:i + 12])):
            m = re.match(r"'(\\x..|\\u\{[0-9a-fA-F]+\}|\\.|[^\\'])'", src[i:])
            out.append(" " * len(m.group(0)))
            i += len(m.group(0))
        else:
            out.append(c)
            i += 1
    result = "".join(out)
    assert len(result) == len(src)
    return result


census["strip"] = strip
Crate, test_spans = census["Crate"], census["test_spans"]

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
SEP = r"\s*::\s*"


def load(config_path):
    config = json.load(open(config_path, encoding="utf-8"))
    base = os.path.dirname(os.path.abspath(config_path))
    artifact = os.path.join(REPO, config["artifact"])
    crate = Crate(os.path.join(artifact, "🦀️.rs"))
    rules_cfg = json.load(open(os.path.join(base, config["layers"]), encoding="utf-8"))
    rules = [(re.compile(r), l) for r, l in rules_cfg["rules"]]

    def layer(path):
        rel = os.path.relpath(path, artifact)
        if rel.startswith(".."):
            return "external"
        for rule, name in rules:
            if rule.search(rel):
                return name
        raise SystemExit(f"unlayered file {rel}")

    return config, artifact, crate, layer


layer_of = None


def module_of(crate, segs):
    """🧭️ Walks absolute module segments through inline modules and private `component` glob modules."""
    cur = ()
    for seg in segs:
        if cur + (seg,) in crate.mods:
            cur = cur + (seg,)
        elif cur + ("component", seg) in crate.mods:
            cur = cur + ("component", seg)
        else:
            break
    while cur:
        if cur + ("component",) in crate.mods and crate.mods[cur + ("component",)] != crate.mods.get(cur):
            return crate.mods[cur + ("component",)]
        if cur in crate.mods:
            if crate.mods[cur] == crate.root and cur:
                inside = [file for file, path in crate.files.items() if path[:len(cur)] == cur and file != crate.root]
                if inside and len({layer_of(file) for file in inside}) == 1:
                    return inside[0]
            return crate.mods[cur]
        cur = cur[:-1]
    return crate.root


def inline_spans(text):
    spans = []
    for m in re.finditer(r"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+(" + IDENT + r")\s*\{", text):
        depth, j = 0, m.end() - 1
        while j < len(text):
            if text[j] == "{":
                depth += 1
            elif text[j] == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        spans.append((m.end(), j, m.group(1)))
    return spans


def base_at(crate, file, spans, offset):
    base = list(crate.files[file])
    for a, b, name in spans:
        if a <= offset < b:
            base.append(name)
    return base


def absolute(segs, base):
    if segs[0] == "crate":
        return segs[1:]
    base = list(base)
    k = 0
    while k < len(segs) and segs[k] in ("super", "self"):
        if segs[k] == "super":
            base.pop()
        k += 1
    return base + segs[k:]


def split_top(text):
    parts, depth, cur = [], 0, ""
    for ch in text:
        if ch == "{":
            depth += 1
        if ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    return [p.strip() for p in parts if p.strip()]


def leaves(prefix, body):
    m = re.match(r"^((?:" + IDENT + SEP + r")*)\{(.*)\}$", body.strip(), re.S)
    if m:
        pre = prefix + [x for x in re.split(SEP, m.group(1)) if x]
        out = []
        for part in split_top(m.group(2)):
            out += leaves(pre, part)
        return out
    alias = re.search(r"\s+as\s+(" + IDENT + r")$", body.strip())
    core = re.sub(r"\s+as\s+" + IDENT + r"$", "", body.strip())
    return [(prefix + [x for x in re.split(SEP, core) if x], alias.group(1) if alias else None)]


USE = re.compile(r"(?m)^(?P<attrs>(?:[ \t]*#\[[^\n]*\]\n)*)(?P<indent>[ \t]*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+(?P<body>(?:crate|super|self)\b[^;]*);")
PATH = re.compile(r"(?<![\w:$])(?P<path>(?:crate|super|self)(?:" + SEP + r"(?:" + IDENT + r"|super))+)")
ROOT_PATH = re.compile(r"(?<![\w:$])(?P<path>(?:standards|editor|viewer|examples)(?:" + SEP + IDENT + r")+)")


def file_rewrites(crate, layer, idents, file, aggregator_root):
    """🔁️ (start, end, replacement) for every cross-layer path in one file, use statements first."""
    raw = open(file, encoding="utf-8").read()
    text = crate.text[file]
    own = layer(file)
    spans = inline_spans(text)
    edits, covered = [], []

    def target(segs, offset):
        abs_segs = absolute(segs, base_at(crate, file, spans, offset))
        dest = layer(module_of(crate, abs_segs))
        if dest == "external":
            dest = own
        if dest != own:
            references.append((file, raw.count("\n", 0, offset) + 1, own, dest, "::".join(abs_segs)))
        return abs_segs, dest

    for m in USE.finditer(text):
        body_start = m.start("body")
        flat = leaves([], m.group("body"))
        rows = []
        for segs, alias in flat:
            glob = segs and segs[-1] == "*"
            path = [s for s in segs if s != "*"]
            abs_segs, dest = target(path, body_start)
            rows.append((abs_segs, glob, alias, dest))
        if all(dest == own for *_, dest in rows):
            covered.append((m.start(), m.end()))
            continue
        lines = []
        for abs_segs, glob, alias, dest in rows:
            head = idents[dest] if dest != own else "crate"
            if dest == "?" or (dest == aggregator_layer and own != aggregator_layer):
                violations.append(f"{os.path.relpath(file, REPO)} reaches {dest} through {'::'.join(abs_segs)}")
            tail = "::*" if glob else (f" as {alias}" if alias else "")
            if abs_segs and abs_segs[-1] == "self":
                abs_segs = abs_segs[:-1]
            shown = abs_segs if head == "crate" else [seg for seg in abs_segs if seg != "component"]
            lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {head}::{'::'.join(shown)}{tail};")
        edits.append((m.start(), m.end(), "\n".join(lines)))
        covered.append((m.start(), m.end()))
    for m in PATH.finditer(text):
        if any(a <= m.start() < b for a, b in covered) or re.search(r"pub\s*\(\s*in\s+$", text[max(0, m.start() - 12):m.start()]):
            continue
        segs = [x for x in re.split(SEP, m.group("path")) if x]
        abs_segs, dest = target(segs, m.start())
        if dest == own:
            continue
        if dest == "?" or (dest == aggregator_layer and own != aggregator_layer):
            violations.append(f"{os.path.relpath(file, REPO)} reaches {dest} through {'::'.join(abs_segs)}")
            continue
        edits.append((m.start(), m.end(), f"{idents[dest]}::{'::'.join(seg for seg in abs_segs if seg != 'component')}"))
    if file == crate.root:
        tops = sorted({path[0] for path in crate.mods if len(path) == 1}, key=len, reverse=True)
        root_path = re.compile(r"(?<![\w:$])(?P<path>(?:" + "|".join(map(re.escape, tops)) + r")(?:" + SEP + IDENT + r")+)") if tops else ROOT_PATH
        for m in root_path.finditer(text):
            if any(a <= m.start() < b for a, b in covered) or any(a <= m.start() < b for a, b, _ in spans if _ in tops):
                continue
            segs = [x for x in re.split(SEP, m.group("path")) if x]
            dest = layer(module_of(crate, segs))
            if dest != own:
                references.append((file, raw.count("\n", 0, m.start()) + 1, own, dest, "::".join(segs)))
                edits.append((m.start(), m.end(), f"{idents[dest]}::{'::'.join(segs)}"))
    return raw, sorted(edits)


def apply_edits(raw, edits):
    out, last = [], 0
    for a, b, rep in edits:
        if a < last:
            raise SystemExit("overlapping edits")
        out.append(raw[last:a])
        out.append(rep)
        last = b
    out.append(raw[last:])
    return "".join(out)


TOKENS = re.compile(r"#\[path\s*=\s*\"(?P<path>[^\"]+)\"\]|#\[cfg\((?P<cfg>[^\]]*)\)\]|(?P<vis>pub(?:\s*\([^)]*\))?\s+)?mod\s+(?P<mod>" + IDENT + r")\s*(?P<kind>[;{])|pub\s+use\s+component\s*::\s*\*\s*;|\}")


def mount_tree(root_file, root_raw):
    """🌳️ The root's `#[path = "."]` inline module trees: nodes (name, children) and mounts (attrs, vis, name, file)."""
    text = strip(root_raw)
    base = os.path.dirname(root_file)
    trees = []
    for m in re.finditer(r"#\[path\s*=\s*\"\.\"\]\s*pub\s+mod\s+(" + IDENT + r")\s*\{", text):
        if any(a <= m.start() < b for a, b, _ in trees):
            continue
        depth, j = 0, m.end() - 1
        while True:
            if text[j] == "{":
                depth += 1
            elif text[j] == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        trees.append((m.start(), j + 1, m.group(1)))

    def parse(start, end, directory):
        children, pending, cfg = [], None, None
        pos = start
        while pos < end:
            t = TOKENS.search(text, pos, end)
            if not t:
                break
            pos = t.end()
            if t.group("path") is not None:
                pending = t.group("path")
            elif t.group("cfg") is not None:
                cfg = t.group("cfg")
            elif t.group("mod"):
                name, vis = t.group("mod"), (t.group("vis") or "").strip()
                if t.group("kind") == "{":
                    depth, j = 0, t.end() - 1
                    while True:
                        if text[j] == "{":
                            depth += 1
                        elif text[j] == "}":
                            depth -= 1
                            if depth == 0:
                                break
                        j += 1
                    sub = os.path.normpath(os.path.join(directory, pending)) if pending else os.path.join(directory, name)
                    children.append(("node", name, vis, cfg, parse(t.end(), j, sub)))
                    pos = j + 1
                else:
                    children.append(("mount", name, vis, cfg, os.path.normpath(os.path.join(directory, pending))))
                pending, cfg = None, None
            elif t.group(0).startswith("pub"):
                children.append(("glob", cfg))
                cfg = None
        return children

    out = []
    for a, b, name in trees:
        body_start = text.index("{", a) + 1
        out.append(((a, b), ("node", name, "pub", None, parse(body_start, b - 1, base))))
    depth, level = 0, []
    for i, ch in enumerate(text):
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
        level.append(depth)
    for m in re.finditer(r"(?m)^(?:#\[cfg\((?P<cfg>[^\]]*)\)\]\s*)?#\[path\s*=\s*\"(?P<path>[^\"]+)\"\]\s*(?P<vis>pub(?:\s*\([^)]*\))?\s+)?mod\s+(?P<name>" + IDENT + r")\s*;", text):
        if level[m.start()] != 0 or m.group("path") == "." or any(a <= m.start() < b for a, b, _ in trees):
            continue
        end = m.end() + (1 if text[m.end():m.end() + 1] == "\n" else 0)
        out.append(((m.start(), end), ("mount", m.group("name"), (m.group("vis") or "").strip(), m.group("cfg"), os.path.normpath(os.path.join(base, m.group("path"))))))
    return out


def has_layer(node, layer, want):
    if node[0] == "mount":
        return layer(node[4]) == want
    if node[0] == "node":
        return any(has_layer(child, layer, want) for child in node[4])
    return False


def emit(node, layer, want, root_dir, indent):
    pad = "    " * indent
    kind = node[0]
    if kind == "mount":
        _, name, vis, cfg, path = node
        lines = []
        if cfg:
            lines.append(f"{pad}#[cfg({cfg})]")
        lines.append(f'{pad}#[path = "{os.path.relpath(path, root_dir)}"]')
        lines.append(f"{pad}{(vis + ' ') if vis else ''}mod {name};")
        return lines
    _, name, vis, cfg, children = node
    lines = []
    if cfg:
        lines.append(f"{pad}#[cfg({cfg})]")
    lines += [f'{pad}#[path = "."]', f"{pad}{vis} mod {name} {{"]
    component = any(child[0] == "mount" and child[1] == "component" and layer(child[4]) == want for child in children)
    for child in children:
        if child[0] == "glob":
            if component:
                if child[1]:
                    lines.append(f"{pad}    #[cfg({child[1]})]")
                lines.append(f"{pad}    pub use component::*;")
            continue
        if has_layer(child, layer, want):
            lines += emit(child, layer, want, root_dir, indent + 1)
    lines.append(f"{pad}}}")
    return lines


def main():
    global aggregator_layer
    config_path, mode = sys.argv[1], sys.argv[2]
    global layer_of
    config, artifact, crate, layer = load(config_path)
    layer_of = layer
    aggregator_layer = config["aggregator"]
    idents = {name: spec["package"].replace("-", "_") for name, spec in config["crates"].items()}
    idents[aggregator_layer] = config["crate"]
    root_raw = open(crate.root, encoding="utf-8").read()
    trees = mount_tree(crate.root, root_raw)
    nested, removals = [], collections.defaultdict(list)
    pattern_nested = re.compile(r'(?:#\[cfg\((?P<pre>[^\]]*)\)\]\s*)?#\[path\s*=\s*"(?P<path>[^"]+)"\]\s*(?:#\[cfg\((?P<post>[^\]]*)\)\]\s*)?(?P<vis>pub(?:\s*\([^)]*\))?\s+)?mod\s+(?P<name>' + IDENT + r')\s*;')
    for file in crate.files:
        if file == crate.root:
            continue
        for m in pattern_nested.finditer(crate.text[file]):
            target = os.path.normpath(os.path.join(os.path.dirname(file), m.group("path")))
            if target in crate.files and layer(target) not in (layer(file), "external") and layer(file) != "external":
                segs = [seg for seg in crate.files[target] if seg != "component"]
                nested.append((segs, ("mount", m.group("name"), (m.group("vis") or "").strip(), m.group("pre") or m.group("post"), target)))
                removals[file].append((m.start(), m.end() + (1 if crate.text[file][m.end():m.end() + 1] == "\n" else 0), ""))
                print(f"relocate {os.path.relpath(target, REPO)} ({layer(file)} -> {layer(target)}) at {'::'.join(segs)}")
    for segs, mount in nested:
        holder = next((tree for _, tree in trees if tree[0] == "node" and tree[1] == segs[0]), None)
        if holder is None:
            holder = ("node", segs[0], "pub", None, [])
            trees.append(((None, None), holder))
        for seg in segs[1:-1]:
            child = next((node for node in holder[4] if node[0] == "node" and node[1] == seg), None)
            if child is None:
                child = ("node", seg, "pub", None, [])
                holder[4].append(child)
            holder = child
        holder[4].append(mount)
    default_header = [line for line in root_raw.splitlines() if line.startswith("#![") or line.startswith("extern crate ")]
    moved, moved_spans = {}, []
    for rule in config.get("move", []):
        for m in re.finditer(rule["match"], root_raw, re.M | re.S):
            moved[rule["to"]] = moved.get(rule["to"], "") + "\n" + m.group(0).rstrip("\n") + "\n"
            moved_spans.append((m.start(), m.end() + (1 if root_raw[m.end():m.end() + 1] == "\n" else 0)))
    writes = {}
    for name, spec in config["crates"].items():
        if name == aggregator_layer:
            continue
        root_dir = os.path.join(artifact, spec["dir"])
        body = []
        for _, tree in trees:
            if has_layer(tree, layer, name):
                body += emit(tree, layer, name, root_dir, 0)
        layer_text = "\n".join(crate.text[f] for f in crate.files if layer(f) == name) + moved.get(name, "")
        def wanted(line):
            alias = re.match(r"extern crate \S+ as (\w+);", line)
            return not alias or re.search(r"(?<![\w:])" + alias.group(1) + r"\s*::", layer_text)
        header = config.get("header", {}).get(name) or ([config["crates"][name].get("doc", "//! " + config["crates"][name]["package"])] + [""] + [line for line in default_header if wanted(line)])
        writes[os.path.join(root_dir, "🦀️.rs")] = "\n".join(header) + "\n\n" + "\n".join(body) + moved.get(name, "") + "\n"
    rewrites = {}
    for file in crate.files:
        if layer(file) == "external":
            continue
        raw, edits = file_rewrites(crate, layer, idents, file, crate.root)
        if removals.get(file):
            edits = sorted([edit for edit in edits if not any(a <= edit[0] < b for a, b, _ in removals[file])] + removals[file])
        if file == crate.root:
            for (a, b), tree in trees:
                if a is None:
                    continue
                start, end = root_raw.rfind("\n", 0, a) + 1, b + (1 if root_raw[b:b + 1] == "\n" else 0)
                kept = "\n".join(emit(tree, layer, aggregator_layer, artifact, 0)) + "\n" if has_layer(tree, layer, aggregator_layer) else ""
                edits = [edit for edit in edits if not start <= edit[0] < end] + [(start, end, kept)]
            edits = [edit for edit in edits if not any(a <= edit[0] < b for a, b in moved_spans)] + [(a, b, "") for a, b in moved_spans]
            edits.sort()
        if edits:
            rewrites[file] = apply_edits(raw, edits)
    pattern = re.compile(r"(?<![\w:])" + re.escape(config["crate"]) + r"(?P<rest>(?:" + SEP + IDENT + r")+)")
    group = re.compile(r"(?m)^(?P<attrs>(?:[ \t]*#\[[^\n]*\]\n)*)(?P<indent>[ \t]*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+" + re.escape(config["crate"]) + r"(?P<body>\s*::\s*[^;]*\{[^;]*);")
    external = {}
    roots = [os.path.join(REPO, root) for root in config["consumers"]]
    index_path = os.path.join(HERE, "🗑️generated", "stdio-consumers.json")
    if os.path.exists(index_path):
        candidates = [os.path.normpath(os.path.join(REPO, f)) for f in json.load(open(index_path, encoding="utf-8")).get(config["crate"], [])]
    else:
        candidates = [os.path.normpath(os.path.join(d, f)) for root in roots for d, dn, fs in os.walk(root) for f in fs if f.endswith(".rs")]
    for path in candidates:
        if True:
            if True:
                if path in crate.files:
                    continue
                raw = open(path, encoding="utf-8").read()
                if config["crate"] not in raw:
                    continue
                text = strip(raw)
                edits, used = [], collections.Counter()
                covered = []
                for m in group.finditer(text):
                    body = m.group("body").strip()[2:].strip()
                    lines = []
                    for segs, alias in leaves([], body):
                        glob = segs and segs[-1] == "*"
                        path_segs = [s for s in segs if s != "*"]
                        dest = layer(module_of(crate, path_segs))
                        used[dest] += 1
                        tail = "::*" if glob else (f" as {alias}" if alias else "")
                        lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {idents[dest]}::{'::'.join(path_segs)}{tail};")
                    edits.append((m.start(), m.end(), "\n".join(lines)))
                    covered.append((m.start(), m.end()))
                for m in pattern.finditer(text):
                    if any(a <= m.start() < b for a, b in covered):
                        continue
                    segs = [x for x in re.split(SEP, m.group("rest")) if x]
                    dest = layer(module_of(crate, segs))
                    used[dest] += 1
                    if dest != aggregator_layer:
                        edits.append((m.start(), m.start() + len(config["crate"]), idents[dest]))
                if edits:
                    external[path] = (apply_edits(raw, sorted(edits)), dict(used))
                elif used:
                    external[path] = (raw, dict(used))
    for violation in sorted(set(violations)):
        print(f"VIOLATION {violation}")
    if violations and mode != "plan" and not os.environ.get("FORCE"):
        raise SystemExit("cut refused: cross-layer violations")
    allow = json.load(open(os.path.join(os.path.dirname(os.path.abspath(config_path)), config["layers"]), encoding="utf-8"))["allow"]
    upward = sorted({(os.path.relpath(f, REPO), line, own, dest, path) for f, line, own, dest, path in references if dest not in allow.get(own, []) and dest != aggregator_layer})
    for f, line, own, dest, path in upward:
        print(f"UPWARD {own}->{dest} {f}:{line} {path}")
    for want in filter(None, os.environ.get("SHOW_EDGES", "").split(",")):
        a, b = want.split(">")
        for f, line, own, dest, path in sorted(set(references)):
            if own == a and dest == b:
                print(f"EDGE {own}->{dest} {os.path.relpath(f, artifact)}:{line} {path}")
    edges = collections.defaultdict(lambda: {"lib": set(), "dev": set()})
    for f, line, own, dest, path in references:
        if dest not in ("external", "?") and dest != own:
            edges[own]["dev" if (f in crate.test_files or "🧪️tests" in f) else "lib"].add(dest)
    os.makedirs(os.path.join(HERE, "🗑️generated"), exist_ok=True)
    json.dump({k: {u: sorted(v) for u, v in d.items()} for k, d in edges.items()}, open(os.path.join(HERE, "🗑️generated", config["crate"] + "-layer-edges.json"), "w"), indent=1)
    if mode == "plan":
        for path, text in writes.items():
            print(f"== root {os.path.relpath(path, REPO)} ({text.count(chr(10))} lines)")
        for path in sorted(rewrites):
            print(f"rewrite {layer(path)} {os.path.relpath(path, REPO)}")
        for path, (_, used) in sorted(external.items()):
            print(f"consumer {json.dumps(used, sort_keys=True)} {os.path.relpath(path, REPO)}")
        return
    import shutil, time
    backup = os.path.join(REPO, ".🧬semio/🦑️repo/⚡️cache/p8/backup", config["crate"] + "-" + time.strftime("%H%M%S"))
    for path in list(writes) + list(rewrites) + list(external):
        if os.path.exists(path):
            target = os.path.join(backup, os.path.relpath(path, REPO))
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copy2(path, target)
    print(f"backup {os.path.relpath(backup, REPO)}")
    for path, text in writes.items():
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(text)
    for path, text in rewrites.items():
        open(path, "w", encoding="utf-8").write(text)
    usage = collections.defaultdict(collections.Counter)
    for path, (text, used) in external.items():
        open(path, "w", encoding="utf-8").write(text)
        usage[path] = used
    cache = os.path.join(HERE, "🗑️generated")
    os.makedirs(cache, exist_ok=True)
    json.dump({os.path.relpath(path, REPO): used for path, used in usage.items()}, open(os.path.join(cache, os.path.basename(config_path).replace(".json", ".consumers.json")), "w", encoding="utf-8"), indent=1, sort_keys=True, ensure_ascii=False)
    print(f"roots={len(writes)} rewritten={len(rewrites)} consumers={len(external)}")


aggregator_layer = None
violations = []
references = []
main()
