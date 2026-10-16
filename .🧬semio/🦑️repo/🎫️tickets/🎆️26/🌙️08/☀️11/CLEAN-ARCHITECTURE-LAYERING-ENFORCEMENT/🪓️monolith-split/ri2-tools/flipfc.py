#!/usr/bin/env python3
# 🧵️ Frame-core flip of RENDERER-INTEGRATION-2: the nine WP3 crates are mounted from the files WP3 staged (wp3-tools/crate-map.json), against the renderer-root namespace.
# usage: flipfc.py <crate> [--dry|--apply|--reroot]
#   transforms the member files (renderer-root paths -> owning crates, visibility -> pub, cross-crate child mounts and test mounts cut out),
#   writes the crate root, replaces the hub mounts by `pub(crate) use <crate>::*;` and moves the test mounts to the hub.
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/w1")
sys.path.insert(0, HERE)
import flip as F
from flip import *
import paths as P

HUB = R + "/📥️mounts/🧵️wp3-frame-core/🦀️.rs"
CM = json.load(open(SPLIT + "/wp3-tools/crate-map.json"))
FRAME = ["host-plumbing", "frame-actions", "runtime-mailbox", "app-runtime", "frame-presentation", "frame-transaction", "frame-job", "native-probe", "host-winit", "browser-worker"]
OWNER = {}
for c in FRAME:
    for f in CM[c]:
        OWNER[os.path.join(REPO, f)] = c

# top-level modules as the WP3 hub mounts them: file -> (module name, glob visibility or None, cfg)
HUBMODS = {
    R + "/🛎️frame-actions/🦀️.rs": ("frame_actions", "pub(crate)", []),
    R + "/🎨️presentation-raster/🦀️.rs": ("presentation_raster", "pub(crate)", []),
    R + "/📮️mailbox-impl/🦀️.rs": ("mailbox_impl", "pub(crate)", []),
    R + "/🧾️frame-build/🦀️.rs": ("frame_build", "pub(crate)", []),
    R + "/🖥️frame-presentation/🦀️.rs": ("frame_presentation", "pub(crate)", []),
    R + "/🧠️app-runtime/🦀️.rs": ("app_runtime", "pub(crate)", []),
    R + "/🔬️native-socket-probe/🦀️.rs": ("native_socket_probe", "pub", []),
    T + "/🏠️os-host/🦀️.rs": ("os_host", None, []),
    T + "/🧵️frame-job/🦀️.rs": ("frame_job", None, []),
    E + "/🪙️authority/🖼️frame/🦀️.rs": ("frame_authority", None, []),
    T + "/🌐️browser-worker/🦀️.rs": ("browser_worker", None, ['#[cfg(target_arch = "wasm32")]']),
    T + "/🪟️winit-app/🦀️.rs": ("winit_app", None, []),
    # children mounted by a parent file of another crate: they become top-level modules of their own crate
    R + "/🛎️frame-actions/🔁️runtime-apply/🦀️.rs": ("runtime_apply", "pub(crate)", []),
    R + "/🧾️frame-build/🧮️frame-transaction/🦀️.rs": ("frame_transaction", "pub(crate)", []),
}
CROSS_CHILD = {  # parent file -> module names of children owned by another crate (their mount lines are cut from the parent)
    R + "/🛎️frame-actions/🦀️.rs": ["runtime_apply"],
    R + "/🧾️frame-build/🦀️.rs": ["frame_transaction"],
}


def lib_of(c):
    return PFX + c.replace("-", "_")


def top_modules(c):
    """The files of crate c that its root mounts directly: hub modules plus cross-crate children."""
    out = []
    for f in CM[c]:
        fp = os.path.join(REPO, f)
        if fp in HUBMODS:
            out.append(fp)
    return out


def member_names(c):
    names = {}
    for f in CM[c]:
        fp = os.path.join(REPO, f)
        names[fp] = HUBMODS[fp][0] if fp in HUBMODS else os.path.basename(os.path.dirname(fp))
    return names


# -- exports index of the frame-core crates (name -> crate), for the lower crates -------------------------------------------------
def fc_exports(c):
    res = collections.defaultdict(list)
    for f in CM[c]:
        fp = os.path.join(REPO, f)
        text = read(fp)
        for it in itm.table(text):
            if it["kind"] in ("impl", "use") or (not it["vis"].startswith("pub") and not it.get("thread_local")):
                continue
            res[it["name"]].append(" ".join(a for a in it["attrs"] if a.startswith("#[cfg(")))
    for fp, (name, glob, cfg) in HUBMODS.items():
        if OWNER.get(fp) == c and glob is None:
            res[name].append(" ".join(cfg))
    return res


def shell_index():
    idx = {}
    st = F.load_state()
    for c in st["flipped"]:
        for n in F.exports_of(c):
            idx.setdefault(n, c)
    return idx


SHELL_IDX = shell_index()
FC_IDX = {}
FC_MOD = {}
for _c in FRAME[1:]:
    for _n in fc_exports(_c):
        FC_IDX.setdefault(_n, _c)


def _export_module(c, name):
    for f in CM[c]:
        fp = os.path.join(REPO, f)
        top = fp
        while top not in HUBMODS:
            parent = None
            for g in CM[c]:
                gp = os.path.join(REPO, g)
                if gp != top and top in modtree_children(gp):
                    parent = gp
            if parent is None:
                break
            top = parent
        if HUBMODS.get(top, (None, "x"))[1] is None and any(it["name"] == name and it["kind"] not in ("impl", "use") and it["vis"].startswith("pub") for it in itm.table(read(fp))):
            return HUBMODS[top][0]
    return None


def modtree_children(fp):
    import modtree
    return set(modtree.children(fp))


for _n, _c in list(FC_IDX.items()):
    _m = _export_module(_c, _n)
    if _m:
        FC_MOD[_n] = _m
for _fp, (_mname, _glob, _cfg) in HUBMODS.items():
    if _glob is None and OWNER.get(_fp) in FRAME[1:]:
        FC_IDX.setdefault(_mname, OWNER[_fp])
        FC_MOD.setdefault(_mname, None)

ROOT_GLOB = P.root_glob_index()
ROOT_ALIASES = dict(P.MODULE_TO_PATH)
for _m in ("interpreter", "scenes", "engine_canvas"):
    ROOT_ALIASES.setdefault(_m, None)

CURRENT = [None]  # the crate being flipped; its own names keep `crate::`


def map_root_path(tokens):
    """tokens: ['crate', head, ...] (or [head, ...] with head a root alias) -> new tokens, or None to leave untouched."""
    crate_prefixed = tokens and tokens[0] == "crate"
    body = tokens[1:] if crate_prefixed else tokens
    if not body:
        return None
    head, rest = body[0], body[1:]
    cur = CURRENT[0]
    # frame-core module names of the hub resolve through the module's name or the item's name
    for fp, (mname, glob, cfg) in HUBMODS.items():
        if head == mname:
            owner = OWNER.get(fp)
            if not rest:
                return None
            if owner == cur:
                return (["crate"] + body) if crate_prefixed else None
            if rest[0] in FC_IDX and FC_IDX[rest[0]] != cur:
                mod = FC_MOD.get(rest[0])
                return [lib_of(FC_IDX[rest[0]])] + ([mod] if mod else []) + rest
            return [lib_of(owner), head] + rest if glob is None else [lib_of(owner)] + rest
    if head == "shell":
        if not rest:
            return None
        c = SHELL_IDX.get(rest[0])
        if c is None and rest[0] in F.P1:
            return list(F.P1[rest[0]][0][1]) + rest[1:]
        if c is None:
            raise KeyError(f"shell::{rest[0]} has no owning Shell crate")
        return [lib_of(c)] + rest
    if head in FC_IDX and (crate_prefixed or head in FC_IDX):
        c = FC_IDX[head]
        if c == cur:
            return (["crate"] + body) if crate_prefixed else None
        mod = FC_MOD.get(head)
        return [lib_of(c)] + ([mod] if mod else []) + body
    if head in SHELL_IDX and crate_prefixed:
        return [lib_of(SHELL_IDX[head])] + body
    if head in ("interpreter", "scenes", "engine_canvas"):
        if not rest:
            return None
        table = {"interpreter": P.INTERP, "scenes": P.SCENES, "engine_canvas": P.ENGINE}[head]
        ent = table.get(rest[0])
        if not ent:
            raise KeyError(f"no facade entry for {head}::{rest[0]}")
        return list(ent[0][1]) + rest[1:]
    if head in P.MODULE_TO_PATH:
        return P.MODULE_TO_PATH[head] + rest
    if crate_prefixed and head in P1R and len({tuple(p) for c, p in P1R[head]}) == 1 and len(P1R[head][0][1]) >= 2:
        return list(P1R[head][0][1]) + rest
    if head in P.HB:
        return [PFX + "host_base", head] + rest
    if head in ROOT_GLOB and crate_prefixed:
        return [ROOT_GLOB[head], head] + rest
    if crate_prefixed:
        raise KeyError(f"unknown root path crate::{head}")
    return None


def map_crate_path_fc(tokens):
    return map_root_path(tokens)


F.map_crate_path = map_crate_path_fc

# unqualified heads in code (`interpreter::x`, `shell::X`, `scenes::x`, `engine_canvas::x`, hub module names)
HEAD_RE = re.compile(r"(?<![A-Za-z0-9_$.])(?<!::)(interpreter|scenes|engine_canvas|shell)(?:::[A-Za-z_][A-Za-z0-9_]*)+")


def rewrite_heads(text):
    clean = blank(text)
    out, last = [], 0
    for m in HEAD_RE.finditer(clean):
        toks = m.group(0).split("::")
        new = map_root_path(toks)
        if new is None:
            continue
        out.append(text[last:m.start()])
        out.append("::".join(new))
        last = m.end()
    out.append(text[last:])
    return "".join(out)


def fix_modpaths(text):
    """`<lib>::NAME` where NAME is defined in a non-glob pub module of that crate -> `<lib>::<module>::NAME`."""
    def sub(m):
        lib, name = m.group(1), m.group(2)
        c = None
        for cc in FRAME[1:]:
            if lib_of(cc) == lib:
                c = cc
        if c and FC_IDX.get(name) == c and FC_MOD.get(name):
            return f"{lib}::{FC_MOD[name]}::{name}"
        return m.group(0)
    return re.sub(r"(semio_framework_os_renderer_wgpu_[a-z_]+)::([A-Za-z_][A-Za-z0-9_]*)", lambda m: sub(m), text)


def cut_cross_children(text, names):
    for n in names:
        pat = re.compile(r"(?:#\[cfg[^\n]*\]\n)*#\[path = \"[^\"\n]*\"\]\nmod " + re.escape(n) + r";\n(?:pub(?:\([a-z]+\))? use " + re.escape(n) + r"::\*;\n)?(?:\n)?")
        found = pat.findall(text)
        assert len(found) == 1, (n, len(found))
        text = pat.sub("", text)
    return text


TEST_MOUNT = re.compile(r"((?:#\[cfg[^\n]*\]\n)+)#\[path\s*=\s*\"([^\"\n]+)\"\]\n(?:pub(?:\([a-z]+\))? )?mod ([A-Za-z_][A-Za-z0-9_]*);\n")


INC_MOUNT = re.compile(r"((?:#\[cfg[^\n]*\]\n)+)include!\(\"([^\"\n]+)\"\);\n")


def cut_test_mounts(text, src_file):
    """test mounts of a member file move to the hub; returns (new text, [(cfg, absolute target, module name)])."""
    moved = []

    def sub(m):
        if "test" not in m.group(1):
            return m.group(0)
        target = os.path.normpath(os.path.join(os.path.dirname(src_file), m.group(2)))
        moved.append((m.group(1), target, m.group(3)))
        return ""
    def sub_inc(m):
        if "test" not in m.group(1):
            return m.group(0)
        moved.append((m.group(1), os.path.normpath(os.path.join(os.path.dirname(src_file), m.group(2))), None))
        return ""
    return INC_MOUNT.sub(sub_inc, TEST_MOUNT.sub(sub, text)), moved


def transform_member(path):
    text = read(path)
    if path in CROSS_CHILD:
        text = cut_cross_children(text, CROSS_CHILD[path])
    text, moved = cut_test_mounts(text, path)
    text = rewrite_heads(text)
    text = F.transform(text)
    text = fix_modpaths(text)
    if "native-asset-stream" in path and CURRENT[0] == "runtime-mailbox":
        text = re.sub(r'(CARGO_MANIFEST_DIR"\)\)\.join\(")((?:\.\./)*\.\.)("\))', lambda m: m.group(1) + "../" + m.group(2) + m.group(3), text)
    return text, moved


def p1_root():
    """local name -> [(cfg, path tokens)] from the renderer root, the hubs and their mapped paths."""
    res = collections.defaultdict(list)
    files = [R + "/🦀️.rs"] + [R + "/📥️mounts/" + h + "/🦀️.rs" for h in ("🌱️wp1-foundation", "🎞️wp2-scenes", "🧵️wp3-frame-core", "🐚️wp4-shell-state", "🎛️wp5-shell-views")]
    for f in files:
        text = read(f)
        for cfg, vis, p, local, glob in uses(text, top_only=True):
            if glob or not local or not p:
                continue
            try:
                if p[0] == "crate":
                    p = map_root_path(p)
                    if p is None:
                        continue
                elif p[0] in ("shell", "interpreter", "scenes", "engine_canvas") or p[0] in P.MODULE_TO_PATH:
                    q = map_root_path(p)
                    if q is None:
                        continue
                    p = q
                elif p[0] in ("self", "super"):
                    continue
                elif p[0] in {v[0] for v in HUBMODS.values()}:
                    continue
            except KeyError:
                continue
            if "cfg(test)" in cfg and False:
                continue
            res[local].append((cfg, p))
        for m in re.finditer(r"((?:#\[cfg[^\n]*\]\n)*)extern crate ([A-Za-z0-9_]+) as ([A-Za-z0-9_]+);", blank(text)):
            cfg = text[m.start(1):m.end(1)].strip()
            if m.group(2).startswith(PFX):
                res[m.group(3)].append((cfg, [m.group(2)]))
    return res


P1R = p1_root()
ALIAS_EXTERN = {"infinite_world", "dsl", "dsl_core", "protocol", "store", "store_sync", "camera_storage", "wasm_bindgen_futures"}


def needs_fc(suffix, files, lower):
    toks = set()
    allclean = ""
    for f in files:
        t = read(f)
        toks |= tokens(t)
        allclean += blank(t) + "\n"
    local = defined_names(files) | {HUBMODS[f][0] for f in files if f in HUBMODS}
    by = collections.defaultdict(lambda: collections.defaultdict(set))
    plain, missing = [], []
    for t in sorted(toks):
        if t in local:
            continue
        hit = False
        if t in FC_IDX and FC_IDX[t] != suffix and RANK[FC_IDX[t]] < RANK[suffix]:
            y = FC_IDX[t]
            cfgs = fc_exports(y)[t]
            cfg = cfgs[0] if len(set(cfgs)) == 1 else ""
            mod = FC_MOD.get(t)
            by[(lib_of(y) + ("::" + mod if mod else ""), cfg)][t] = None
            hit = True
        if not hit and t in ROOT_GLOB:
            by[(ROOT_GLOB[t], "")][t] = None
            hit = True
        if not hit and t in SHELL_IDX:
            y = SHELL_IDX[t]
            cfgs = F.exports_of(y)[t]
            cfg = cfgs[0] if len(set(cfgs)) == 1 else ""
            by[(lib_of(y), cfg)][t] = None
            hit = True
        if hit:
            continue
        if t in P1R:
            seen = set()
            for cfg, p in P1R[t]:
                if len(p) == 1:
                    alias = f"use {p[0]} as {t};"
                    plain.append((cfg + "\n" if cfg else "") + alias)
                    continue
                if not cfg and len(p) == 2 and p[0].startswith(PFX):
                    cfg = def_cfg(p[0], p[1])
                key = (cfg, tuple(p))
                if key in seen:
                    continue
                seen.add(key)
                parent, leaf = p[:-1], p[-1]
                piece = leaf if leaf == t else f"{leaf} as {t}"
                by[("P1", cfg, tuple(parent))][piece] = None
    for hint, trait in TRAIT_HINTS.items():
        if re.search(r"\b" + hint + r"\b", allclean) and trait in P1R:
            for cfg, p in P1R[trait]:
                by[("P1", cfg, tuple(p[:-1]))][p[-1]] = None
    lines = []
    for key in sorted(by, key=lambda k: (str(k[0]) if k[0] != "P1" else "~" + "::".join(k[2]), k[1] if len(k) > 1 else "")):
        names = sorted(by[key])
        if key[0] == "P1":
            lines.append(render_use(key[1], list(key[2]), names))
        else:
            lines.append(render_use(key[1], [key[0]], names))
    aliases = []
    for a, (line, cfg) in EXT_ALIASES.items():
        if re.search(r"(?<![A-Za-z0-9_])" + a + r"\s*::", allclean) or any(p and p[0] == a for t2 in toks for c, p in P1R.get(t2, [])):
            aliases.append(((cfg + "\n") if cfg else "") + line)
    return aliases, lines + sorted(set(plain))


def crate_root_text(suffix, hub_moved):
    folder = TABLE[suffix]
    files = [os.path.join(REPO, f) for f in CM[suffix]]
    lower = [c for c in FRAME if RANK[c] < RANK[suffix]]
    aliases, uses_ = needs_fc(suffix, files, lower)
    head = [l for l in read(T + "/" + folder + "/🦀️.rs").split("\n") if l.startswith("//!")][:1] or [f"//! {suffix}"]
    gate = ""
    if suffix == "browser-worker":
        gate = '#![cfg(target_arch = "wasm32")]\n'
    mounts = []
    for fp in top_modules(suffix):
        name, glob, cfg = HUBMODS[fp]
        rel = os.path.relpath(fp, T + "/" + folder)
        c = ("\n".join(cfg) + "\n") if cfg and suffix != "browser-worker" else ""
        if glob is None:
            mounts.append(f'{c}#[path = "{rel}"]\npub mod {name};')
        else:
            mounts.append(f'{c}#[path = "{rel}"]\nmod {name};\n{c}pub use {name}::*;')
    body = [head[0], "//!", f"//! 🧭️ The files WP3 staged for this crate (`wp3-tools/crate-map.json`) are mounted here from their place in the renderer tree; the renderer root re-exports this crate.", ""]
    if gate:
        body += [gate.rstrip("\n"), ""]
    body += aliases + uses_ + [""] + mounts + ["", '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;', ""]
    return "\n".join(body)


def update_hub(suffix):
    text = read(HUB)
    first = None
    for fp in top_modules(suffix):
        name, glob, cfg = HUBMODS[fp]
        if name in ("runtime_apply", "frame_transaction"):
            continue
        pat = re.compile(r"(?:#\[cfg[^\n]*\]\n)*#\[path\s*=\s*\"[^\"\n]*\"\]\n(?:pub )?mod " + re.escape(name) + r";\n(?:pub(?:\([a-z]+\))? use " + re.escape(name) + r"::\*;\n)?")
        found = pat.findall(text)
        assert len(found) == 1, (name, len(found))
        mm = pat.search(text)
        if first is None:
            text = text[:mm.start()] + f"@@FC:{suffix}@@\n" + text[mm.end():]
            first = True
        else:
            text = text[:mm.start()] + text[mm.end():]
    if first is None:
        text = text.rstrip("\n") + f"\n@@FC:{suffix}@@\n"
    text = text.replace(f"@@FC:{suffix}@@\n", f"pub(crate) use {lib_of(suffix)}::*;\n")
    return text


def add_hub_tests(text, moved):
    add = ""
    for cfg, target, mname in moved:
        rel = os.path.relpath(target, os.path.dirname(HUB))
        add += (f'{cfg}include!("{rel}");\n' if mname is None else f'{cfg}#[path = "{rel}"]\nmod {mname}_host;\n')
    marker = "//#region 🧷️InterpreterTestHosts"
    assert marker in text
    return text.replace(marker, "//#region 🧵️FrameCoreTestHosts\n" + add + "//#endregion 🧵️FrameCoreTestHosts\n" + marker, 1) if add else text


def flip_fc(suffix, apply=False):
    CURRENT[0] = suffix
    files = [os.path.join(REPO, f) for f in CM[suffix]]
    moved_all = []
    new_texts = {}
    for f in files:
        t, moved = transform_member(f)
        new_texts[f] = t
        moved_all += moved
    root_text = None
    if apply:
        F.backup(suffix, files + [HUB, T + "/" + TABLE[suffix] + "/🦀️.rs", T + "/" + TABLE[suffix] + "/📦️packages/🦀️rust/Cargo.toml"])
        for f, t in new_texts.items():
            if t != read(f):
                write(f, t)
    root_text = crate_root_text(suffix, moved_all)
    if apply:
        write(T + "/" + TABLE[suffix] + "/🦀️.rs", root_text)
        hub = update_hub(suffix)
        hub = add_hub_tests(hub, moved_all)
        write(HUB, hub)
        added = ensure_deps(suffix, first_party_used(files), root_text)
        return root_text, moved_all, added
    return root_text, moved_all, []


if __name__ == "__main__":
    suffix = sys.argv[1]
    if "--reroot" in sys.argv:
        CURRENT[0] = suffix
        files = [os.path.join(REPO, f) for f in CM[suffix]]
        for f in files:
            t = fix_modpaths(F.transform(rewrite_heads(read(f))))
            if t != read(f):
                write(f, t)
        root_text = crate_root_text(suffix, [])
        write(T + "/" + TABLE[suffix] + "/🦀️.rs", root_text)
        print("deps added:", ensure_deps(suffix, first_party_used(files), root_text))
    else:
        rt, moved, added = flip_fc(suffix, apply="--apply" in sys.argv)
        print(rt[:6000])
        print("test mounts moved:", [(m[2], os.path.basename(os.path.dirname(m[1]))) for m in moved])
        print("deps added:", added)
