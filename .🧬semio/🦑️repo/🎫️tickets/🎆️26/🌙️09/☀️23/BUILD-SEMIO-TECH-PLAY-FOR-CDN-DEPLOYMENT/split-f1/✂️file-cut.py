#!/usr/bin/env python3
"""✂️ Cuts one monolithic Rust source file (plus its single unit-test file) into domain crates along line ranges.

Each target crate gets `<dir>/🦀️.rs` (crate root, its line ranges verbatim), `<dir>/🧪️tests/🔬️unit/🦀️.rs` (the test
functions whose references resolve highest in that crate) and `<dir>/📦️packages/🦀️rust/Cargo.toml`. Cross-crate
references become explicit `use <owner crate>::{…};` lines; private top-level items another crate names become `pub`.
Nothing is re-exported. Test helpers listed under `"helpers"` move to the named support crate.

usage: ✂️file-cut.py <config.json> plan|apply
config: {"source": <repo-relative .rs>, "tests": <repo-relative test .rs>, "root": <repo-relative dir of the family>,
         "header": [<lines kept at the top of every crate root>], "crates": [{"name", "dir", "doc", "ranges": [[a, b]],
         "attrs": [<crate attributes>], "deps": {<crate>: <dependency spec>}, "dev": {…}}],
         "moves": [{"from": [a, b], "to": <crate>}], "helpers": {"<fn>": <crate>}, "rename": [[<old>, <new>]]}
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"


def blank(src):
    """🧹️ Blanks comments, strings and char literals keeping every byte offset."""
    out, i, n = [], 0, len(src)
    while i < n:
        if src.startswith("//", i):
            j = src.find("\n", i); j = n if j < 0 else j; out.append(" " * (j - i)); i = j
        elif src.startswith("/*", i):
            j = src.find("*/", i + 2); j = n if j < 0 else j + 2; out.append(re.sub(r"[^\n]", " ", src[i:j])); i = j
        elif src[i] == "r" and re.match(r'r#*"', src[i:i + 8]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = re.match(r'r(#*)"', src[i:]); close = '"' + m.group(1)
            j = src.find(close, i + len(m.group(0))); j = n if j < 0 else j + len(close)
            out.append(re.sub(r"[^\n]", " ", src[i:j])); i = j
        elif src[i] == '"':
            j = i + 1
            while j < n and src[j] != '"': j += 2 if src[j] == "\\" else 1
            out.append(" " + re.sub(r"[^\n]", " ", src[i + 1:j]) + " "); i = j + 1
        elif src[i] == "'" and (re.match(r"'(\\.|[^\\'])'", src[i:i + 4]) or re.match(r"'\\(x..|u\{[0-9a-fA-F]+\})'", src[i:i + 12])):
            m = re.match(r"'(\\x..|\\u\{[0-9a-fA-F]+\}|\\.|[^\\'])'", src[i:]); out.append(" " * len(m.group(0))); i += len(m.group(0))
        else:
            out.append(src[i]); i += 1
    return "".join(out)


DEF = re.compile(r"^(pub(?:\([^)]*\))?\s+)?((?:async\s+)?(?:fn|struct|enum|trait|const|type|static)\s+)(" + IDENT + r")", re.M)
CLOSE = re.compile(r"^\s+(pub )?enum (" + IDENT + r")\s*:", re.M)
TEST = re.compile(r"^#\[semio_framework_async_macros::async_test\]\n(?:#\[[^\n]*\]\n)*async fn (" + IDENT + r")", re.M)
HELPER = re.compile(r"^(?:async fn|fn|struct|enum|impl(?:<[^>]*>)?)\s+(" + IDENT + r")(?:\s+for\s+(" + IDENT + r"))?", re.M)


def items(text):
    """🧾️ Top-level item names defined in `text` → whether each is already public."""
    found = {}
    for m in DEF.finditer(text):
        found.setdefault(m.group(3), bool(m.group(1)))
    for m in CLOSE.finditer(text):
        found.setdefault(m.group(2), bool(m.group(1)))
    return found


VARIANT = re.compile(r"(?m)^(\s+)[A-Z][A-Za-z0-9_]*(?=\s*[{(,]|\s*$)")


_stripped = {}


def referenced(name, text):
    if text not in _stripped:
        _stripped[text] = VARIANT.sub(r"\1", text)
    text = _stripped[text]
    return re.search(r"(?<![A-Za-z0-9_:.])" + re.escape(name) + r"(?![A-Za-z0-9_])", text) is not None


def blocks(src, first_line):
    """🧱️ Splits a test source into top-level blocks: (kind, name, text, region) with attached attributes/docs."""
    lines = src.split("\n")
    out, i, region = [], 0, None
    clean = blank(src).split("\n")
    while i < len(lines):
        line = lines[i]
        m = re.match(r"// #region (\S+)", line)
        if m:
            region = m.group(1); i += 1; continue
        if re.match(r"// #endregion", line) or not line.strip() or line.startswith("use super::*"):
            i += 1; continue
        start = i
        while lines[i].startswith("#[") or lines[i].startswith("///") or lines[i].startswith("//"):
            i += 1
        depth, j, opened = 0, i, False
        while j < len(lines):
            depth += clean[j].count("{") - clean[j].count("}")
            opened = opened or "{" in clean[j]
            if (opened and depth == 0) or (not opened and clean[j].rstrip().endswith(";")):
                break
            j += 1
        text = "\n".join(lines[start:j + 1])
        head = "\n".join(lines[i:j + 1])
        t = TEST.search("\n".join(lines[start:j + 1]))
        if t:
            out.append(("test", t.group(1), text, region))
        else:
            h = HELPER.search(head)
            out.append(("helper", (h.group(2) or h.group(1)) if h else head[:40], text, region))
        i = j + 1
    return out


def main():
    config_path, mode = sys.argv[1], sys.argv[2]
    config = json.load(open(config_path, encoding="utf-8"))
    src_lines = open(os.path.join(REPO, config["source"]), encoding="utf-8").read().split("\n")
    crates = config["crates"]
    names = [c["name"] for c in crates]
    ident = {c["name"]: c["name"].replace("-", "_") for c in crates}
    body = {c["name"]: [] for c in crates}
    moved = set()
    for move in config.get("moves", []):
        a, b = move["from"]
        moved.update(range(a, b + 1))
    for c in crates:
        for a, b in c["ranges"]:
            body[c["name"]].append("\n".join(src_lines[k - 1] for k in range(a, b + 1) if k not in moved))
    for move in config.get("moves", []):
        a, b = move["from"]
        body[move["to"]].append("\n".join(src_lines[a - 1:b]))
    text = {n: "\n\n".join(body[n]) for n in names}
    for old, new in config.get("rename", []):
        text = {n: t.replace(old, new) for n, t in text.items()}
    test_src = open(os.path.join(REPO, config["tests"]), encoding="utf-8").read()
    for old, new in config.get("rename", []):
        test_src = test_src.replace(old, new)
    tblocks = blocks(test_src, 1)
    helpers = config.get("helpers", {})
    for kind, name, block, region in tblocks:
        if kind == "helper" and name in helpers:
            text[helpers[name]] += "\n\n" + re.sub(r"(?m)^(async fn|fn|struct|enum) ", r"pub \1 ", block, count=1)
    clean = {n: blank(t) for n, t in text.items()}
    owned = {n: items(clean[n]) for n in names}
    owner = {}
    for n in names:
        for item in owned[n]:
            owner.setdefault(item, n)

    helper_names = {b[1] for b in tblocks if b[0] == "helper"}
    tests = {n: [] for n in names}
    support = {}
    level = {n: i for i, n in enumerate(names)}
    local = {name for kind, name, block, region in tblocks if kind == "helper" and name not in helpers}
    place, own = {}, {}
    for index, (kind, name, block, region) in enumerate(tblocks):
        if kind == "helper" and name in helpers:
            continue
        cb = blank(block)
        reach = [level[owner[item]] + (owner[item] == config.get("support")) for item in owner if referenced(item, cb)]
        reach += [level[helpers[h]] for h in helpers if referenced(h, cb) and helpers[h] != config.get("support")]
        own[index] = max(reach, default=0)
    place = dict(own)
    for _ in range(10):
        key = {}
        for index, (kind, name, block, region) in enumerate(tblocks):
            if index in place and kind == "helper":
                key[name] = max(key.get(name, 0), place[index])
        for index, (kind, name, block, region) in enumerate(tblocks):
            if index in place:
                cb = blank(block)
                place[index] = max([own[index]] + [key[h] for h in key if h != name and referenced(h, cb)] + ([key[name]] if kind == "helper" else []))
        for index, (kind, name, block, region) in enumerate(tblocks):
            if index in place and kind == "test":
                for h in local:
                    if referenced(h, blank(block)):
                        for j, other in enumerate(tblocks):
                            if j in place and other[0] == "helper" and other[1] == h:
                                place[j] = max(place[j], place[index])
    for index, (kind, name, block, region) in enumerate(tblocks):
        if index in place:
            tests[names[place[index]]].append((region, block))
    for h in local:
        users = {names[place[i]] for i, b in enumerate(tblocks) if i in place and b[0] == "test" and referenced(h, blank(b[2]))}
        if len(users) > 1:
            print(f"WARN helper {h} shared by {sorted(users)}")

    need_pub = {n: set() for n in names}
    imports = {n: {} for n in names}
    test_imports = {n: {} for n in names}
    for n in names:
        tclean = blank("\n".join(b for r, b in tests[n]))
        for item, home in owner.items():
            if home == n:
                continue
            if referenced(item, clean[n]):
                imports[n].setdefault(home, set()).add(item); need_pub[home].add(item)
            elif referenced(item, tclean):
                test_imports[n].setdefault(home, set()).add(item); need_pub[home].add(item)
    for n in names:
        for item in need_pub[n]:
            if not owned[n][item]:
                text[n] = re.sub(r"(?m)^((?:async\s+)?(?:fn|struct|enum|trait|const|type|static)\s+" + re.escape(item) + r"\b)", r"pub \1", text[n], count=1)
                text[n] = re.sub(r"(?m)^(\s+)enum (" + re.escape(item) + r")\s*:", r"\1pub enum \2:", text[n], count=1)
    trait_methods = {}
    for n in names:
        for m in re.finditer(r"(?ms)^pub trait (" + IDENT + r")[^{]*\{(.*?)^\}", text[n]):
            trait_methods[m.group(1)] = (n, set(re.findall(r"fn (" + IDENT + r")", m.group(2))))
    for n in names:
        for trait, (home, methods) in trait_methods.items():
            if home != n and any(re.search(r"\." + re.escape(meth) + r"\(", clean[n]) for meth in methods):
                if any(referenced(t, clean[n]) for t in owned[home]):
                    imports[n].setdefault(home, set()).add(trait)

    support_crate = config.get("support")
    plan = []
    for c in crates:
        n = c["name"]
        d = os.path.join(REPO, config["root"], c["dir"])
        uses = [f"use {ident[h]}::{{{', '.join(sorted(v))}}};" for h, v in sorted(imports[n].items(), key=lambda kv: level[kv[0]])]
        header = [f"//! {c['doc']}", ""] + c.get("attrs", []) + ([""] if c.get("attrs") else []) + c.get("header", []) + uses
        root = "\n".join(header).rstrip() + "\n\n" + text[n].strip() + "\n"
        rows = tests[n]
        test_text = ""
        if rows:
            root += "\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit/🦀️.rs\"]\nmod tests;\n"
            tuse = ["use super::*;"] + [f"use {ident[h]}::{{{', '.join(sorted(v))}}};" for h, v in sorted(test_imports[n].items(), key=lambda kv: level[kv[0]])]
            parts, current = [], None
            for region, block in rows:
                if region != current:
                    if current:
                        parts.append(f"// #endregion {current}\n")
                    parts.append(f"// #region {region}")
                    current = region
                parts.append(block + "\n")
            parts.append(f"// #endregion {current}")
            test_text = "\n".join(tuse) + "\n\n" + "\n".join(parts) + "\n"
        plan.append((c, d, root, test_text, rows))
        print(f"{n}: lib {root.count(chr(10))} lines, {len(rows)} tests {test_text.count(chr(10))} lines, imports {sum(len(v) for v in imports[n].values())}, pub+ {len([i for i in need_pub[n] if not owned[n][i]])}")
    if mode != "apply":
        return
    for c, d, root, test_text, rows in plan:
        os.makedirs(d, exist_ok=True)
        open(os.path.join(d, "🦀️.rs"), "w", encoding="utf-8").write(root)
        if test_text:
            os.makedirs(os.path.join(d, "🧪️tests", "🔬️unit"), exist_ok=True)
            open(os.path.join(d, "🧪️tests", "🔬️unit", "🦀️.rs"), "w", encoding="utf-8").write(test_text)
        pkg = os.path.join(d, "📦️packages", "🦀️rust")
        os.makedirs(pkg, exist_ok=True)
        depth = len(os.path.relpath(os.path.join(REPO, config["workspace"]), pkg).split(os.sep))
        rel = os.path.relpath(os.path.join(REPO, config["workspace"]), pkg)
        toml = [ "[package]", f'workspace = "{rel}"', f'name = "{c["name"]}"', 'version = "0.1.0"', 'edition = "2021"', "rust-version.workspace = true", f'description = "{c["description"]}"', "",
                 "[package.metadata.semio]", 'role = "framework"', f'id = "{c["name"].removeprefix("semio-framework-")}"', "",
                 "[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", "",
                 "[lints]", "workspace = true", "", "[lib]", f'name = "{ident[c["name"]]}"', 'path = "../../🦀️.rs"', "", "[dependencies]"]
        deps = dict(c.get("deps", {}))
        for h in sorted(imports[c["name"]], key=lambda k: level[k]):
            deps[h] = {"path": os.path.relpath(os.path.join(REPO, config["root"], next(x["dir"] for x in crates if x["name"] == h), "📦️packages", "🦀️rust"), pkg)}
        dev = dict(c.get("dev", {}))
        if rows:
            for h in sorted(test_imports[c["name"]], key=lambda k: level[k]):
                if h not in deps:
                    dev[h] = {"path": os.path.relpath(os.path.join(REPO, config["root"], next(x["dir"] for x in crates if x["name"] == h), "📦️packages", "🦀️rust"), pkg)}
        render = lambda k, v: f'{k} = {{ path = "{v["path"]}" }}' if isinstance(v, dict) else f"{k} = {v}"
        toml += [render(k, v) for k, v in deps.items()]
        if dev:
            toml += ["", "[dev-dependencies]"] + [render(k, v) for k, v in dev.items()]
        open(os.path.join(pkg, "Cargo.toml"), "w", encoding="utf-8").write("\n".join(toml) + "\n")


main()
