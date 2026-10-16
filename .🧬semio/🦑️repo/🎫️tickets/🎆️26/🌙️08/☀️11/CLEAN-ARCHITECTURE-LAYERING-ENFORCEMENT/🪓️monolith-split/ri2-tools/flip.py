#!/usr/bin/env python3
# 🔀️ Shell crate flip for RENDERER-INTEGRATION-2: turns the staging modules of one Shell crate into the sources of its (skeleton) crate.
# usage: flip.py <crate-suffix> [--dry] [--no-parent]
#   1. transforms the module files in place (renderer-root paths -> owning crates, `pub(crate|super|in …)` -> `pub`, retired `use super::<chunk>::*;` lines)
#   2. writes the crate root (docstring, extern-crate aliases, explicit imports, the mounts exactly as the Shell parent mounted them, the test stub)
#   3. removes the module mounts from the Shell parent and adds `pub use <crate>::*;`
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from paths import *
import items as itm
from rs import expand_use

HERE = os.path.dirname(os.path.abspath(__file__))
TABLE = crate_table()
CHUNKS = {m["name"] for m in original_mounts() if m["kind"] == "chunk"}
ORIG = original_mounts()
EXT_ALIASES = {
    "infinite_world": ('extern crate infinite_canvas as infinite_world;', ""),
    "dsl": ('extern crate semio_framework_os_kernel as dsl;', ""),
    "dsl_core": ('extern crate semio_framework_os_kernel as dsl_core;', '#[cfg(not(target_arch = "wasm32"))]'),
    "protocol": ('extern crate semio_framework_os_kernel as protocol;', ""),
    "store": ('extern crate semio_framework_os_kernel as store;', '#[cfg(any(test, not(target_arch = "wasm32")))]'),
    "store_sync": ('extern crate semio_framework_os_kernel as store_sync;', ""),
    "camera_storage": ('use semio_framework_os_renderer_camera as camera_storage;', ""),
    "wasm_bindgen_futures": ('extern crate semio_framework_async as wasm_bindgen_futures;', '#[cfg(target_arch = "wasm32")]'),
}
TRAIT_HINTS = {"to_value": "ToValue", "from_value": "FromValue", "into_value": "ToValue"}


def lib(suffix):
    return PFX + suffix.replace("-", "_")


def vis_widen(text):
    clean = blank(text)
    out, last = [], 0
    for m in re.finditer(r"\bpub\s*\(\s*(?:crate|super|self|in\s+[A-Za-z_:][A-Za-z0-9_:]*)\s*\)", clean):
        out.append(text[last:m.start()])
        out.append("pub")
        last = m.end()
    out.append(text[last:])
    return "".join(out)


USE_STMT = re.compile(r"(?m)^([ \t]*)((?:#\[[^\n\]]*\][ \t]*\n[ \t]*)*)(pub(?:\([^)]*\))?[ \t]+)?use[ \t]+([^;]+);[ \t]*(?:\n|$)")


def map_leaf(path):
    if path and path[0] == "crate":
        return map_crate_path(path)
    return None


def rewrite_use_stmt(attrs, vis, body):
    leaves = expand_use(body)
    changed = False
    groups, order = {}, []
    for path, alias, glob in leaves:
        new = map_leaf(path) if len(path) >= 2 else None
        if new is not None:
            changed = True
        p = new if new is not None else path
        if glob:
            order.append(("*", "::".join(p)))
        else:
            leaf = p[-1]
            piece = leaf if alias is None or alias == leaf else f"{leaf} as {alias}"
            order.append(("l", "::".join(p[:-1]), piece))
    if not changed:
        return None
    lines = []
    seen = {}
    for o in order:
        if o[0] == "*":
            lines.append(f"{attrs}{vis}use {o[1]}::*;")
        else:
            seen.setdefault(o[1], []).append(o[2])
    for parent, pieces in seen.items():
        body = pieces[0] if len(pieces) == 1 else "{" + ", ".join(pieces) + "}"
        lines.append(f"{attrs}{vis}use {parent}::{body};" if parent else f"{attrs}{vis}use {body};")
    return "\n".join(lines)


CODE_PATH = re.compile(r"(?<![A-Za-z0-9_$])(?<!::)crate(?:::[A-Za-z_][A-Za-z0-9_]*)+")


def rewrite_paths(text):
    clean = blank(text)
    # use statements first (they may carry brace groups)
    pieces, last = [], 0
    spans = []
    for m in USE_STMT.finditer(clean):
        if "crate" not in m.group(4):
            continue
        attrs = text[m.start(2):m.end(2)]
        vis = text[m.start(3):m.end(3)] if m.group(3) else ""
        body = text[m.start(4):m.end(4)]
        new = rewrite_use_stmt(attrs, vis, body)
        if new is None:
            continue
        spans.append((m.start(2), m.end(), m.group(1) + new + ("\n" if text[m.end() - 1:m.end()] == "\n" else "")))
    for s, e, new in spans:
        pieces.append(text[last:s])
        pieces.append(new)
        last = e
    pieces.append(text[last:])
    text = "".join(pieces)
    clean = blank(text)
    out, last = [], 0
    for m in CODE_PATH.finditer(clean):
        toks = m.group(0).split("::")
        new = map_crate_path(toks)
        if new is None:
            continue
        out.append(text[last:m.start()])
        out.append("::".join(new))
        last = m.end()
    out.append(text[last:])
    return "".join(out)


def drop_chunk_globs(text):
    clean = blank(text)
    out, last = [], 0
    for m in re.finditer(r"(?m)^use\s+super::([A-Za-z_][A-Za-z0-9_]*)::\*;[ \t]*\n", clean):
        if m.group(1) in CHUNKS:
            out.append(text[last:m.start()])
            last = m.end()
    out.append(text[last:])
    return "".join(out)


def transform(text):
    text = drop_chunk_globs(text)
    text = rewrite_paths(text)
    text = vis_widen(text)
    return text


_DEFCFG = {}


def def_cfg(crate_lib, name):
    """cfg attribute lines shared by every top-level definition of `name` in the crate (empty when any definition is ungated or none is found)."""
    import modtree
    suffix = crate_lib[len(PFX):].replace("_", "-")
    if suffix not in TABLE:
        return ""
    if suffix not in _DEFCFG:
        idx = collections.defaultdict(list)
        for f in modtree.tree(T + "/" + TABLE[suffix] + "/🦀️.rs"):
            for it in itm.table(read(f)):
                if it["kind"] in ("impl", "use"):
                    continue
                idx[it["name"]].append(" ".join(a for a in it["attrs"] if a.startswith("#[cfg(")))
        _DEFCFG[suffix] = idx
    cfgs = _DEFCFG[suffix].get(name)
    if not cfgs or "" in cfgs:
        return ""
    u = sorted(set(cfgs))
    if len(u) == 1:
        return u[0]
    inner = sorted(re.sub(r"^#\[cfg\((.*)\)\]$", r"\1", c) for c in u)
    if set(inner) == {'target_arch = "wasm32"', 'not(target_arch = "wasm32")'}:
        return ""
    return "#[cfg(any(" + ", ".join(inner) + "))]"


def p1_entries():
    """local name -> list of (cfg, mapped path tokens) from the Shell parent's `use` items (all top-level)."""
    res = collections.defaultdict(list)
    for cfg, vis, p, local, glob in uses(read(PARENT), top_only=True):
        if glob or not local or not p:
            continue
        if p[0] == "crate":
            try:
                p = map_crate_path(p)
            except KeyError:
                continue
        elif p[0] == "shell_vocab":
            p = [PFX + "shell_vocab"] + p[1:]
        elif p[0] in ("self", "super") or p[0] in CHUNKS or p[0].startswith("staged_"):
            continue
        res[local].append((cfg, p))
    for m in ORIG:
        if m["kind"] not in ("chunk", "staging") or not m["reexport"]:
            continue
        for cfg, vis, p, local, glob in uses(read(m["file"]), top_only=True):
            if glob or not local or not vis.startswith("pub") or not p or p[0] in ("super", "crate", "self") or not p[0].startswith(PFX):
                continue
            res[local].append((cfg, p))
    return res


P1 = p1_entries()


def exports_of(suffix, mounts=None):
    """name -> list of cfg strings: items a Shell crate makes visible at its root (globbed modules, `pub` vis)."""
    res = collections.defaultdict(list)
    for m in (mounts if mounts is not None else module_list(suffix)):
        if not m.get("glob", False):
            continue
        text = read(m["file"])
        for it in itm.table(text):
            if it["kind"] in ("impl", "use"):
                continue
            if not it["vis"].startswith("pub") and not it.get("thread_local"):
                continue
            res[it["name"]].append(" ".join(a for a in it["attrs"] if a.startswith("#[cfg(")))
    return res


STATE = HERE + "/state.json"


def load_state():
    if os.path.exists(STATE):
        return json.load(open(STATE))
    return {"flipped": [], "modules": {}, "pulled": {}}


def save_state(st):
    json.dump(st, open(STATE, "w"), ensure_ascii=False, indent=1)


def module_list(suffix):
    st = load_state()
    if suffix in st["modules"]:
        return st["modules"][suffix]
    out = []
    for m in ORIG:
        if m["kind"] == "staging" and m["crate"] == suffix:
            out.append(dict(name=m["name"], file=m["file"], cfg=m["cfg"], glob=bool(m["reexport"]), slug=m["slug"], vis=m["vis"], reexport=m["reexport"], reexport_cfg=m["reexport_cfg"]))
    for pm in st["pulled"].get(suffix, []):
        out.append(pm)
    return out


def defined_names(files):
    names = set()
    for f in files:
        text = read(f)
        for it in itm.table(text):
            if it["kind"] not in ("impl", "use"):
                names.add(it["name"])
    return names


def cfg_join(cfgs):
    cfgs = [c for c in cfgs if c]
    if not cfgs:
        return ""
    if len(cfgs) != len(set(cfgs)):
        pass
    return cfgs[0] if len(set(cfgs)) == 1 else ""


def render_use(cfg, path_tokens, names):
    names = sorted(set(names))
    body = names[0] if len(names) == 1 else "{" + ", ".join(names) + "}"
    parent = "::".join(path_tokens)
    return (cfg + "\n" if cfg else "") + f"use {parent}::{body};"


def needs(suffix, files, lower_crates):
    """Explicit root imports for the files of a crate: ('aliases', [...]), ('uses', [...]), ('missing', [...])."""
    toks = set()
    allclean = ""
    for f in files:
        t = read(f)
        toks |= tokens(t)
        allclean += blank(t) + "\n"
    local = defined_names(files)
    lines, missing, used_aliases = [], [], []
    emitted = set()
    lower_exports = {}
    for y in lower_crates:
        lower_exports[y] = exports_of(y)
    # lower shell crate names
    by_crate = collections.defaultdict(lambda: collections.defaultdict(set))
    resolved = set()
    for t in sorted(toks):
        if t in local:
            continue
        hit = False
        for y in lower_crates:
            if t in lower_exports[y]:
                cfgs = lower_exports[y][t]
                cfg = cfgs[0] if len(set(cfgs)) == 1 else ""
                if len(set(cfgs)) > 1 and "" not in cfgs:
                    inner = sorted(re.sub(r"^#\[cfg\((.*)\)\]$", r"\1", c) for c in set(cfgs))
                    if set(inner) == {'target_arch = "wasm32"', 'not(target_arch = "wasm32")'}:
                        cfg = ""
                    else:
                        cfg = "#[cfg(any(" + ", ".join(inner) + "))]"
                by_crate[(y, cfg)][t] = None
                resolved.add(t)
                hit = True
                break
        if hit:
            continue
        if t in P1:
            seen = set()
            for cfg, p in P1[t]:
                if not cfg and len(p) == 2 and p[0].startswith(PFX):
                    cfg = def_cfg(p[0], p[1])
                key = (cfg, tuple(p))
                if key in seen:
                    continue
                seen.add(key)
                resolved.add(t)
                parent, leaf = p[:-1], p[-1]
                piece = leaf if leaf == t else f"{leaf} as {t}"
                by_crate[("P1", cfg, tuple(parent))][piece] = None
    for hint, trait in TRAIT_HINTS.items():
        if re.search(r"\b" + hint + r"\b", allclean) and trait in P1 and trait not in resolved:
            for cfg, p in P1[trait]:
                by_crate[("P1", cfg, tuple(p[:-1]))][p[-1]] = None
    for key in sorted(by_crate, key=lambda k: (str(k[0]) if k[0] != "P1" else "~" + "::".join(k[2]), k[1] if len(k) > 1 else "")):
        names = sorted(by_crate[key])
        if key[0] == "P1":
            lines.append(render_use(key[1], list(key[2]), names))
        else:
            y, cfg = key
            lines.append(render_use(cfg, [lib(y)], names))
    aliases = []
    for a, (line, cfg) in EXT_ALIASES.items():
        if re.search(r"(?<![A-Za-z0-9_])" + a + r"(?:::|\b\s*::)", allclean) or any(a in "::".join(p) for ps in P1.values() for c, p in ps if p and p[0] == a and a in resolved_p1(P1, toks)):
            aliases.append(((cfg + "\n") if cfg else "") + line)
    return aliases, lines


def resolved_p1(p1, toks):
    return {a for t in toks for c, p in p1.get(t, []) for a in p[:1]}


def mount_text(suffix, mods, folder):
    out = []
    for m in mods:
        rel = os.path.relpath(m["file"], T + "/" + folder)
        cfg = "\n".join(m["cfg"])
        out.append((cfg + "\n" if cfg else "") + f'#[path = "{rel}"]\n{"pub " if m.get("pub") else ""}mod {m["name"]};')
        if m.get("glob"):
            out.append((("\n".join(m["reexport_cfg"][:1]) + "\n") if m.get("reexport_cfg") and m["reexport_cfg"][0] else "") + f"pub use {m['name']}::*;")
    return "\n".join(out)


def doc_for(suffix, folder):
    root = T + "/" + folder + "/🦀️.rs"
    first = read(root).split("\n")
    head = [l for l in first if l.startswith("//!")]
    return head[0] if head else f"//! 🐚️ {suffix}"


def flip(suffix, dry=False, parent=True):
    folder = TABLE[suffix]
    mods = list(module_list(suffix))
    files = [m["file"] for m in mods]
    new_texts = {f: transform(read(f)) for f in files}
    if not dry:
        for f, t in new_texts.items():
            if t != read(f):
                write(f, t)
    lower = [c for c in SHELL_ORDER if c != suffix and RANK[c] < RANK[suffix]]
    aliases, uselines = needs(suffix, files, lower)
    doc = doc_for(suffix, folder)
    body = [doc, "//!", f"//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/{strip_emoji_name(files[0])}`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.", ""]
    body += aliases
    body += uselines
    body += ["", mount_text(suffix, mods, folder), "", '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;', ""]
    root_text = "\n".join(body)
    return root_text, mods, files


def strip_emoji_name(f):
    return os.path.basename(os.path.dirname(os.path.dirname(f)))


PRE = RS + "/pre"


def backup(suffix, paths):
    import shutil
    for p in paths:
        if not os.path.exists(p):
            continue
        dst = PRE + "/" + suffix + "/" + os.path.relpath(p, REPO)
        if not os.path.exists(dst):
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copy2(p, dst)


def update_parent(suffix, mods):
    text = read(PARENT)
    first = None
    for m in mods:
        name = m["name"]
        if m.get("new"):
            continue
        pat = re.compile(r"(?:#\[cfg[^\n]*\]\n)*#\[path = \"[^\"\n]*\"\]\nmod " + re.escape(name) + r";\n(?:#\[cfg[^\n]*\]\n)?(?:pub(?:\([a-z]+\))? use " + re.escape(name) + r"::\*;\n)?")
        found = pat.findall(text)
        assert len(found) == 1, (name, len(found))
        mm = pat.search(text)
        if first is None:
            first = mm.start()
            text = text[:mm.start()] + f"@@FLIP:{suffix}@@\n" + text[mm.end():]
        else:
            text = text[:mm.start()] + text[mm.end():]
    text = text.replace(f"@@FLIP:{suffix}@@\n", f"pub use {lib(suffix)}::*;\n")
    write(PARENT, text)


def first_party_used(files):
    used = set()
    for f in files:
        used |= set(re.findall(r"(?<![A-Za-z0-9_])" + PFX + r"([a-z0-9_]+)", blank(read(f))))
    return {u.replace("_", "-") for u in used}


def ensure_deps(suffix, used, root_text):
    folder = TABLE[suffix]
    man = T + "/" + folder + "/📦️packages/🦀️rust/Cargo.toml"
    text = read(man)
    have = set(re.findall(r'^semio-framework-os-renderer-wgpu-([a-z0-9-]+)\s*=', text, re.M))
    add = []
    used = set(used) | {x.replace("_", "-") for x in re.findall(PFX + r"([a-z0-9_]+)", root_text)}
    for u in sorted(used):
        if u == suffix.replace("-", "_") or u == suffix or u in have:
            continue
        if u not in TABLE:
            continue
        add.append(u)
    for u in add:
        if RANK[u] >= RANK[suffix]:
            raise SystemExit(f"upward dependency {suffix} -> {u}")
        line = f'semio-framework-os-renderer-wgpu-{u} = {{ path = "../../../{TABLE[u]}/📦️packages/🦀️rust" }}'
        native = u in NATIVE_ONLY
        hdr = '[target.\'cfg(not(target_arch = "wasm32"))\'.dependencies]' if native else "[dependencies]"
        if hdr not in text:
            raise SystemExit(f"{hdr} missing in {man}")
        i = text.index(hdr) + len(hdr)
        body_end = text.find("\n[", i)
        body_end = len(text) if body_end < 0 else body_end
        lines = text[i:body_end].strip("\n").split("\n")
        lines.append(line)
        fp = sorted(l for l in lines if l.startswith("semio-framework-os-renderer-wgpu-"))
        rest = [l for l in lines if not l.startswith("semio-framework-os-renderer-wgpu-")]
        text = text[:i] + "\n" + "\n".join(fp + rest) + "\n" + text[body_end:]
    if add:
        write(man, text)
    return add


def apply(suffix, glob_files=()):
    mods0 = module_list(suffix)
    for m in mods0:
        if m["file"] in glob_files and not m.get("glob"):
            m["glob"] = True
            m["reexport"] = ["pub"]
            m["reexport_cfg"] = [""]
    st0 = load_state()
    st0["modules"][suffix] = mods0
    save_state(st0)
    root_text, mods, files = flip(suffix, dry=True)
    folder = TABLE[suffix]
    root = T + "/" + folder + "/🦀️.rs"
    backup(suffix, files + [root, PARENT, T + "/" + folder + "/📦️packages/🦀️rust/Cargo.toml", STATE_PATH()])
    for f in files:
        t = transform(read(f))
        if t != read(f):
            write(f, t)
    write(root, root_text)
    update_parent(suffix, mods)
    added = ensure_deps(suffix, first_party_used(files), root_text)
    st = load_state()
    st["modules"][suffix] = [dict(m) for m in mods]
    if suffix not in st["flipped"]:
        st["flipped"].append(suffix)
    save_state(st)
    return added


def reroot(suffix):
    root_text, mods, files = flip(suffix, dry=True)
    folder = TABLE[suffix]
    root = T + "/" + folder + "/🦀️.rs"
    backup(suffix, [root])
    for f in files:
        t = transform(read(f))
        if t != read(f):
            backup(suffix, [f])
            write(f, t)
    if read(root) != root_text:
        write(root, root_text)
    return ensure_deps(suffix, first_party_used(files), root_text)


def STATE_PATH():
    return STATE


if __name__ == "__main__":
    suffix = sys.argv[1]
    if "--reroot" in sys.argv:
        print("deps added:", reroot(suffix))
    elif "--apply" in sys.argv:
        print("deps added:", apply(suffix))
    else:
        root_text, mods, files = flip(suffix, dry=True)
        print(root_text)
