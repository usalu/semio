#!/usr/bin/env python3
"""🪚️ Decomposes the `semio-framework-plugin` crate root (`🔌️plugin/🦀️.rs`) into owned facet modules (wave E2, step 1).

Input: `🗂️destinations.json` (destination modules: key, directory, module name, facet, docstring; and the head-line
ranges / item names of the inline `app` module that each destination owns; plus the inline component-level
modules that move to their own file unchanged in path).

What it does, all in one read-modify-write of the root file:

- every top-level item of `pub mod app { … }` moves, with its leading docs and attributes, to the destination
  module that owns its head line (or its name); `include!`/`#[path]` literals are rebased to the new file and
  the item is dedented by one level (string-literal interiors are never touched);
- `use` items stay as the shared preamble of every destination (private), `pub use` items additionally stay in
  `app`; a `use` rooted at an app-local module follows that module;
- each destination gets explicit `use crate::<owner>::{…}` imports for the crate items it names that another
  destination owns (gated with the same `cfg` as the referencing items);
- `app` becomes the public vocabulary of the crate, composed from the destination modules
  (`pub use super::<module>::*;`), and the destination modules are declared at component level, grouped by facet;
- inline component-level modules listed under `files` move verbatim (dedented) to their own file.

Usage: decompose.py [--write]   (default: dry run, prints the destination census)
"""
import importlib.util, json, os, re, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rust_items", os.path.join(HERE, "🧱️rust_items.py"))
ri = importlib.util.module_from_spec(spec); spec.loader.exec_module(ri)

PLUGIN = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
ROOT = f"{PLUGIN}/🦀️.rs"
CFG = json.load(open(os.path.join(HERE, "🗂️destinations.json"), encoding="utf-8"))
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
INC = re.compile(r'include!\(\s*"([^"]+)"\s*\)')
PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
REGION = re.compile(r"^\s*//\s*#(?:end)?region\b.*$")


def literal_mask(src):
    """🧵️ 1 for every offset inside a string or char literal (delimiters excluded), comments never count."""
    mask = bytearray(len(src))
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        if c == "/" and i + 1 < n and src[i + 1] == "/":
            j = src.find("\n", i); i = n if j < 0 else j; continue
        if c == "/" and i + 1 < n and src[i + 1] == "*":
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j): depth += 1; j += 2
                elif src.startswith("*/", j): depth -= 1; j += 2
                else: j += 1
            i = j; continue
        m = re.match(r'b?r(#*)"', src[i:i + 20]) if c in "rb" and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")) else None
        if m:
            start = i + m.end(); end = src.find('"' + m.group(1), start); end = n if end < 0 else end
            for k in range(start, end): mask[k] = 1
            i = end + 1 + len(m.group(1)); continue
        if c == '"' or (c == "b" and i + 1 < n and src[i + 1] == '"' and not (i and (src[i - 1].isalnum() or src[i - 1] == "_"))):
            j = i + (2 if c == "b" else 1); start = j
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            for k in range(start, min(j, n)): mask[k] = 1
            i = j + 1; continue
        if c == "'":
            m = re.match(r"'(\\.[^']*|[^'\\])'", src[i:i + 12])
            if m:
                i += m.end(); continue
        i += 1
    return mask


def dedent(text, mask, offset, width=4):
    """⬅️ Removes up to `width` leading spaces from every line that does not start inside a string literal."""
    out, pos = [], 0
    for line in text.split("\n"):
        start = offset + pos
        inside = start > 0 and mask[start - 1]
        lead = len(line) - len(line.lstrip(" "))
        out.append(line if inside else line[min(width, lead):])
        pos += len(line) + 1
    return "\n".join(out)


def undent(text, width=4):
    return "\n".join(l[min(width, len(l) - len(l.lstrip(" "))):] for l in text.split("\n"))


def rebase(text, src_dir, dst_dir):
    def fix(m, kind):
        lit = m.group(1)
        if lit.startswith("/") or "CARGO_MANIFEST_DIR" in lit: return m.group(0)
        target = os.path.normpath(os.path.join(src_dir, lit))
        if not os.path.exists(target):
            raise SystemExit(f"[decompose] unresolved {kind} {lit!r} from {src_dir}")
        rel = os.path.relpath(target, dst_dir)
        return m.group(0).replace(f'"{lit}"', f'"{rel}"')
    text = INC.sub(lambda m: fix(m, "include"), text)
    return PATH.sub(lambda m: fix(m, "path"), text)


def strip_regions(text):
    return "\n".join(l for l in text.split("\n") if not REGION.match(l))


def cfg_of(raw, cl, it):
    head = cl[it["start"]:it["head"]]
    attrs = re.findall(r"#\[\s*cfg\((.*?)\)\s*\]", raw[it["start"]:it["head"]], re.S)
    for a in attrs:
        a = re.sub(r"\s+", " ", a.strip())
        if a == "test" or re.search(r"\btest\b", a):
            return a
    return None


def main():
    write = "--write" in sys.argv
    raw = open(ROOT, encoding="utf-8").read()
    cl = ri.clean(raw)
    mask = literal_mask(raw)
    top, _ = ri.items(raw, cl, 0, len(cl))
    by_name = {it["name"]: it for it in top if it["kind"] == "mod"}
    app = by_name["app"]
    ob = cl.index("{", app["head"]); cb = ri.skip_group(cl, ob) - 1
    items, tail = ri.items(raw, cl, ob + 1, cb)
    dests = {d["key"]: d for d in CFG["destinations"]}
    ranges = [(a, b, k) for k, d in dests.items() for a, b in d.get("lines", [])]
    names = {n: k for k, d in dests.items() for n in d.get("names", [])}
    local_mods = {it["name"]: it for it in items if it["kind"] == "mod"}
    chunks = defaultdict(list)
    preamble, facade_uses = [], []
    unassigned = []
    for it in items:
        line = ri.line_of(raw, it["head"])
        text = raw[it["start"]:it["end"]]
        if it["kind"] == "use":
            body = cl[it["head"]:it["end"]]
            m = re.match(r"\s*(pub(?:\s*\([^)]*\))?\s+)?use\s+([\w:{]+)", body)
            first = re.split(r"::|\{", m.group(2))[0] if m else ""
            if first in local_mods or first == "self":
                it["dest_mod"] = first
                chunks[("__follow__", first)].append((it, text))
                continue
            stripped = re.sub(r"(^|\n)(\s*)pub(\s*\([^)]*\))?\s+use", r"\1\2use", strip_regions(text), count=1)
            preamble.append(undent(stripped.strip("\n")).strip("\n"))
            if m and m.group(1):
                facade_uses.append(text.strip("\n"))
            continue
        key = names.get(it["name"]) if it["name"] else None
        if key is None:
            for a, b, k in ranges:
                if a <= line <= b: key = k; break
        if key is None:
            unassigned.append((line, it["kind"], it["name"])); continue
        chunks[key].append((it, text))
    mod_dest = {}
    for k, lst in chunks.items():
        if isinstance(k, tuple): continue
        for it, _ in lst:
            if it["kind"] == "mod": mod_dest[it["name"]] = k
    for k in [k for k in chunks if isinstance(k, tuple)]:
        for it, text in chunks.pop(k):
            d = mod_dest.get(k[1])
            if d is None: unassigned.append((ri.line_of(raw, it["head"]), "use->" + k[1], None)); continue
            chunks[d].append((it, text))
    if unassigned:
        for u in unassigned: print("[unassigned]", u)
        raise SystemExit(1)
    defs = defaultdict(set)
    for k, lst in chunks.items():
        for it, text in lst:
            if it["name"] and it["kind"] in ("struct", "enum", "trait", "type", "fn", "const", "static", "union", "mod"):
                defs[it["name"]].add(k)
            if it["kind"] == "use":
                body = cl[it["head"]:it["end"]]
                if re.match(r"\s*pub", body):
                    for nm in IDENT.findall(body.split("::")[-1] if "{" not in body else body[body.index("{"):]):
                        if nm not in ("pub", "use", "crate", "self", "super", "as"): defs[nm].add(k)
    macro_names = {it["name"] for lst in chunks.values() for it, _ in lst if it["kind"] == "macro_rules"}
    census = []
    files = {}
    for k, lst in sorted(chunks.items(), key=lambda kv: kv[0]):
        d = dests[k]
        dst_dir = os.path.join(PLUGIN, d["dir"])
        lst.sort(key=lambda x: x[0]["start"])
        own = {it["name"] for it, _ in lst if it["name"]}
        needs = defaultdict(lambda: defaultdict(set))
        bodies = []
        for it, text in lst:
            gate = cfg_of(raw, cl, it)
            seg = cl[it["start"]:it["end"]]
            tokens = set(IDENT.findall(seg))
            for inc in INC.finditer(raw[it["start"]:it["end"]]):
                p = os.path.normpath(os.path.join(PLUGIN, inc.group(1)))
                if os.path.exists(p): tokens |= set(IDENT.findall(ri.clean(open(p, encoding="utf-8").read())))
            for t in tokens:
                if t in own or t in macro_names: continue
                owners = defs.get(t)
                if owners and len(owners) == 1:
                    o = next(iter(owners))
                    if o != k: needs[gate][o].add(t)
            body = dedent(text, mask, it["start"])
            body = strip_regions(body)
            body = rebase(body, PLUGIN, dst_dir)
            bodies.append(body.strip("\n"))
        prod = needs.get(None, {})
        imports = []
        for gate in sorted(needs, key=lambda g: (g is not None, g or "")):
            for o in sorted(needs[gate]):
                ns = sorted(needs[gate][o] - (set() if gate is None else prod.get(o, set())))
                if not ns: continue
                line = f"use crate::{dests[o]['module']}::{{{', '.join(ns)}}};"
                imports.append(line if gate is None else f"#[cfg({gate})]\n{line}")
        doc = f"//! {d['doc']}"
        content = doc + "\n\n" + "\n".join(preamble) + "\n" + "\n".join(imports) + "\n\n" + "\n\n".join(bodies) + "\n"
        files[os.path.join(dst_dir, "🦀️.rs")] = content
        census.append((d["facet"], d["module"], len(lst), content.count("\n"), d["dir"]))
    for c in sorted(census): print(f"{c[0]:12} {c[1]:24} items={c[2]:4} lines={c[3]:6} {c[4]}")
    print("total moved lines", sum(c[3] for c in census))
    order = CFG["facets"]
    decl = []
    for f in order:
        mods = [dests[k] for k in sorted(chunks) if dests[k]["facet"] == f]
        if not mods: continue
        decl.append(f"//#region {CFG['facetTitles'][f]}")
        for d in mods:
            gate = f'#[cfg({d["cfg"]})]\n' if d.get("cfg") else ""
            decl.append(f'{gate}#[path = "{d["dir"]}/🦀️.rs"]\npub(crate) mod {d["module"]};')
        decl.append(f"//#endregion {CFG['facetTitles'][f]}")
    facade = ["pub mod app {", f"    //! {CFG['appDoc']}"]
    for u in facade_uses:
        u = "\n".join(l for l in strip_regions(u).split("\n") if not l.strip().startswith("//!"))
        facade.append("    " + undent(u).strip().replace("\n", "\n    "))
    for f in order:
        for k in sorted(chunks):
            if dests[k]["facet"] == f:
                gate = f'#[cfg({dests[k]["cfg"]})]\n    ' if dests[k].get("cfg") else ""
                facade.append(f"    {gate}pub use super::{dests[k]['module']}::*;")
    facade.append("}")
    app_start = app["start"]
    while raw[app_start] == "\n": app_start += 1
    new_root = raw[:app_start] + "\n".join(decl) + "\n\n" + "\n".join(facade) + "\n" + raw[app["end"]:]
    for spec_ in CFG.get("files", []):
        it = by_name[spec_["module"]]
        mob = cl.index("{", it["head"]); mcb = ri.skip_group(cl, mob) - 1
        body = raw[mob + 1:mcb]
        dst_dir = os.path.join(PLUGIN, spec_["dir"])
        src_dir = os.path.normpath(os.path.join(PLUGIN, spec_.get("pathAttr", ".")))
        body = dedent(body, mask, mob + 1)
        body = strip_regions(body)
        body = rebase_mixed(body, PLUGIN, src_dir, dst_dir)
        files[os.path.join(dst_dir, "🦀️.rs")] = body.strip("\n") + "\n"
        head_attrs = raw[it["start"]:it["head"]]
        head_attrs = PATH.sub("", head_attrs)
        head = cl[it["head"]:mob]
        vis = re.match(r"\s*(pub(?:\s*\([^)]*\))?\s+)?mod", head).group(1) or ""
        decl_text = head_attrs.rstrip() + f'\n#[path = "{spec_["dir"]}/🦀️.rs"]\n{vis}mod {spec_["module"]};'
        seg = raw[it["start"]:it["end"]]
        new_root = new_root.replace(seg, "\n" + decl_text.lstrip("\n"), 1)
        print(f"file-move {spec_['module']:24} lines={body.count(chr(10)):6} -> {spec_['dir']}")
    print("root lines", raw.count("\n"), "->", new_root.count("\n"))
    if "--out" in sys.argv:
        out = sys.argv[sys.argv.index("--out") + 1]
        for p, c in list(files.items()) + [(ROOT, new_root)]:
            q = os.path.join(out, os.path.relpath(p, PLUGIN))
            os.makedirs(os.path.dirname(q), exist_ok=True)
            open(q, "w", encoding="utf-8").write(c)
        print("previewed into", out)
    if write:
        if open(ROOT, encoding="utf-8").read() != raw:
            raise SystemExit("[decompose] root changed during the run; nothing written")
        for p, c in files.items():
            if os.path.exists(p): raise SystemExit(f"[decompose] destination exists: {p}")
        for p, c in files.items():
            os.makedirs(os.path.dirname(p), exist_ok=True)
            open(p, "w", encoding="utf-8").write(c)
        open(ROOT, "w", encoding="utf-8").write(new_root)
        print("written", len(files), "files")


def rebase_mixed(body, file_dir, path_dir, dst_dir):
    """🧭️ `include!` literals are relative to the source FILE, `#[path]` literals to the module's resolved directory."""
    def inc(m):
        lit = m.group(1)
        if lit.startswith("/") or "CARGO_MANIFEST_DIR" in lit: return m.group(0)
        target = os.path.normpath(os.path.join(file_dir, lit))
        if not os.path.exists(target): raise SystemExit(f"[decompose] unresolved include {lit!r}")
        return m.group(0).replace(f'"{lit}"', f'"{os.path.relpath(target, dst_dir)}"')
    def pth(m):
        lit = m.group(1)
        target = os.path.normpath(os.path.join(path_dir, lit))
        if not os.path.exists(target): raise SystemExit(f"[decompose] unresolved path {lit!r}")
        return m.group(0).replace(f'"{lit}"', f'"{os.path.relpath(target, dst_dir)}"')
    body = INC.sub(inc, body)
    return PATH.sub(pth, body)


if __name__ == "__main__":
    main()
