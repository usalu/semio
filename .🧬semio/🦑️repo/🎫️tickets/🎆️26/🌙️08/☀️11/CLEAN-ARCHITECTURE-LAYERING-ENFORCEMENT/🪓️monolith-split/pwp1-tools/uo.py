#!/usr/bin/env python3
# 🔬️ Stitched (union) full-resolution differential of the PLUGIN-WP1 carve.
# usage: uo.py <tag> [--configs lib,test,wasip2,lib-features,test-features] [--crates a,b] [--seed name]... [--mode full|crates]
# A = the live plugin tree where every carved crate is mounted as a module `semio_framework_plugin_<key>` of the lib root, `crate::` and
#     `pub(crate)` rewritten per crate, `extern crate semio_framework_plugin_<key> as x;` rewritten to `use`; B = the pre-WP1 snapshot.
# Both get the sandbox patches of 📓️RESOLUTION-ORACLE.md (extern crates become empty modules, external glob imports become `const _: () = ();`).
import collections, json, os, re, shutil, subprocess, sys, time
from lib import *
import wp1files
from rs import blank

EXCLUDE = ['node_modules', 'target', '.cache', '🖥️host', '🧠️interpreter', '🏇️mounted-owner', '🏛️ownership', '🧬️guest-contract', '🖨️describe', '📦️packages', '.🧬semio', '🌐️browser-bundle', '🏗️build', '🧷️companions']
FEATS = ['artifact-app-testing', 'media-turn-testing', 'reserved-producer-testing', 'original-actor-context-testing']
FEATARGS = [x for f in FEATS for x in ('--cfg', 'feature="%s"' % f)]
CFGS = {'lib': [], 'test': ['--test'], 'wasip2': ['--target', 'wasm32-wasip2'], 'wasip2-test': ['--test', '--target', 'wasm32-wasip2'], 'lib-features': FEATARGS, 'test-features': ['--test'] + FEATARGS, 'wasm': ['--target', 'wasm32-unknown-unknown']}
CARVED = TOOLS + "/carved.json"


def carved():
    return json.load(open(CARVED)) if os.path.exists(CARVED) else []


def sub_outside_strings(text, regex, repl):
    clean = blank(text)
    out, last = [], 0
    for m in regex.finditer(clean):
        out.append(text[last:m.start()])
        out.append(m.expand(repl) if isinstance(repl, str) else repl(m))
        last = m.end()
    out.append(text[last:])
    return "".join(out)


R_CRATE = re.compile(r"(?<![A-Za-z0-9_$:])crate::")
R_PUBCRATE = re.compile(r"\bpub\(crate\)")
R_EXT = re.compile(r"(?<![A-Za-z0-9_:])" + PREFIX + r"([A-Za-z0-9_]+)(?=::)")
R_EXTERN = re.compile(r"(pub(?:\([^)]*\))?\s+)?extern\s+crate\s+(" + PREFIX + r"[A-Za-z0-9_]+)\s+as\s+([A-Za-z0-9_]+)\s*;")
R_EXTERN_BARE = re.compile(r"(pub(?:\([^)]*\))?\s+)?extern\s+crate\s+(" + PREFIX + r"[A-Za-z0-9_]+)\s*;")


def rewrite(path, libname, names, owned):
    text = open(path, encoding="utf8").read()
    new = text
    if owned:
        new = sub_outside_strings(new, R_CRATE, f"crate::{libname}::")
        new = sub_outside_strings(new, R_PUBCRATE, f"pub(in crate::{libname})")
    new = sub_outside_strings(new, R_EXTERN, lambda m: f"{m.group(1) or ''}use crate::{m.group(2)} as {m.group(3)};" if m.group(2) in names else m.group(0))
    new = sub_outside_strings(new, R_EXTERN_BARE, lambda m: f"{m.group(1) or ''}use crate::{m.group(2)};" if m.group(2) in names else m.group(0))
    new = sub_outside_strings(new, R_EXT, lambda m: "crate::" + PREFIX + m.group(1) if PREFIX + m.group(1) in names else m.group(0))
    if new != text:
        open(path, "w", encoding="utf8").write(new)


def neutralize(tree):
    files, names = [], set()
    for dp, dn, fn in os.walk(tree):
        dn[:] = [d for d in dn if d != 'node_modules']
        for f in fn:
            if f.endswith('.rs'):
                p = os.path.join(dp, f); files.append(p)
                t = open(p, encoding='utf-8', errors='ignore').read()
                for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:mod|enum|struct|trait|type|union)\s+(\w+)', t, re.M): names.add(m.group(1))
    keep = {'crate', 'super', 'self', 'std', 'core', 'alloc'} | names
    rx = re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$', re.M)
    ex = re.compile(r'^(\s*)extern\s+crate\s+(\w+)(?:\s+as\s+(\w+))?;[ \t]*$')
    for p in files:
        t = open(p, encoding='utf-8', errors='ignore').read()
        t2 = rx.sub(lambda m: m.group(0) if m.group(2) in keep else m.group(1) + 'const _: () = ();', t)
        out = []
        for l in t2.split('\n'):
            em = ex.match(l)
            out.append(em.group(1) + 'mod ' + (em.group(3) or em.group(2)) + ' {}' if em else l)
        t2 = '\n'.join(out)
        if t2 != t: open(p, 'w', encoding='utf-8').write(t2)


def chain(sb, src_root):
    shutil.rmtree(sb, ignore_errors=True)
    os.makedirs(sb + '/' + PLUG_REL)
    ex = []
    for e in EXCLUDE: ex += ['--exclude', e]
    subprocess.check_call(['rsync', '-a', *ex, src_root + '/' + PLUG_REL + '/', sb + '/' + PLUG_REL + '/'])
    os.makedirs(sb + '/' + PK_REL, exist_ok=True)
    for f in ('🦀️.rs', 'Cargo.toml'):
        shutil.copy(src_root + '/' + PK_REL + '/' + f, sb + '/' + PK_REL + '/' + f)


def mount_cfg(f, parent):
    """cfg attributes (non-test) on the mount of file f inside its parent file, as attribute lines."""
    if not parent:
        return ""
    ptext = open(parent, encoding="utf8").read()
    base = os.path.dirname(parent)
    for m in wp1files.MOUNT.finditer(blank(ptext)):
        attrs = ptext[m.start(1):m.end(1)]
        pm = wp1files.PATH_ATTR.search(attrs)
        if pm and os.path.normpath(os.path.join(base, pm.group(1))) == f:
            lines = [a for a in re.findall(r"#\[\s*cfg\((?:[^\[\]]|\[[^\]]*\])*\)\s*\]", attrs) if "test" not in a]
            return "".join(l + "\n" for l in lines)
    return ""


OWNED_DIRS = ["🚪️io", "🌐host", "🪪️identity", "🕹️interaction", "♻️cancellation", "⏳️operation-progress", "🌱️initialization", "📬️completion", "🪟️window", "🪟️window-kits", "🖱️context-menu", "⚛️reactor/🧵️executor", "📡️backbone", "🧬️schema", "🪝️mounts/🌱️wp1-foundation/🧪️test-hosts"]
OWNED_FILES = ["🪝️mounts/🌱️wp1-foundation/🦀️.rs", "📣️macros/🌱️wp1-window/🦀️.rs", "🧪️tests/🔬️app-window-kits/🦀️.rs", "🧪️tests/🔬️app-panel-kit/🦀️.rs"] + [f"📜️declaration/🧩️app/🛬️lifted/{d}/🦀️.rs" for d in ("⚠️fault", "🚫️refusal", "♻️retirement", "⏳️operation-progress")]
# foreign files WP1 touches with a few anchored lines: (file, old, new) replayed onto the snapshot copy, not copied from the live tree (peers edit them too)
FOREIGN = [
    ("🦀️.rs", "REGION:🔖️ComponentPersistentLocal", "#[doc(hidden)]\npub use semio_framework_plugin_abi::component_persistent_local;\n"),
    ("🦀️.rs", "MODULE:owned_abi", ""),
    ("🦀️.rs", '#[cfg(all(target_arch = "wasm32", target_env = "p2"))]\n#[path = "🧬️schema/🦀️.rs"]\npub mod wasip2;\n', '#[cfg(all(target_arch = "wasm32", target_env = "p2"))]\npub use semio_framework_plugin_abi::wasip2;\n#[cfg(all(target_arch = "wasm32", target_env = "p2"))]\n#[doc(hidden)]\npub use semio_framework_plugin_abi::__semio_export_component_guest;\n'),
    ("🦀️.rs", '#[path = "🪟️window/🫧️transient/🦀️.rs"]\npub(crate) mod window_transient;\n', 'pub(crate) use semio_framework_plugin_window_transient::window_transient;\n#[doc(hidden)]\npub use semio_framework_plugin_window_transient::window_transient_transfer;\n'),
    ("⚛️reactor/🦀️.rs", '#[path = "🧵️executor/🦀️.rs"]\npub mod executor;\n', 'pub use semio_framework_plugin_actor_host::executor;\n'),
    ("🗃️document/🦀️.rs", '#[path="../🌱️initialization/🏪️catalog/🦀️.rs"]\nmod original_catalog_constructor;\n', 'use semio_framework_plugin_operation::original_catalog_constructor;\n'),
]


def isolate(sb):
    """sb holds a copy of the snapshot; overlay the WP1-owned trees from the live tree and replay the foreign anchored edits."""
    for d in OWNED_DIRS:
        src = REPO + "/" + PLUG_REL + "/" + d + "/"
        dst = sb + "/" + PLUG_REL + "/" + d + "/"
        os.makedirs(dst, exist_ok=True)
        ex = []
        for e in EXCLUDE:
            ex += ["--exclude", e]
        subprocess.check_call(["rsync", "-a", "--delete", *ex, src, dst])
    for f in OWNED_FILES:
        s, d = REPO + "/" + PLUG_REL + "/" + f, sb + "/" + PLUG_REL + "/" + f
        os.makedirs(os.path.dirname(d), exist_ok=True)
        shutil.copy(s, d)
    for f, old, new in FOREIGN:
        path = sb + "/" + PLUG_REL + "/" + f
        txt = open(path, encoding="utf8").read()
        if old.startswith("MODULE:"):
            import defs as _defs
            head = '#[cfg(all(target_arch = "wasm32", target_env = "p2"))]\n#[doc(hidden)]\npub mod owned_abi {\n'
            assert txt.count(head) == 1
            i = txt.index(head)
            j = _defs.matching_brace(txt, i + len(head) - 2) + 1
            live = open(REPO + "/" + PLUG_REL + "/" + f, encoding="utf8").read()
            li = live.index(head)
            lj = _defs.matching_brace(live, li + len(head) - 2) + 1
            txt = txt[:i] + live[li:lj] + txt[j:]
        elif old.startswith("REGION:"):
            name = old[7:]
            a, b = "//#region " + name + "\n", "//#endregion " + name + "\n"
            assert txt.count(a) == 1 and txt.count(b) == 1
            i, j = txt.index(a), txt.index(b) + len(b)
            txt = txt[:i] + new + txt[j:]
        else:
            assert txt.count(old) == 1, (f, old[:60])
            txt = txt.replace(old, new)
        open(path, "w", encoding="utf8").write(txt)
    # root-level include lines of the live hub set are those of the snapshot: the snapshot root already includes the hub files


def prepare(sb, crates, mode):
    names = {lib_name(c) for c in crates}
    import crates_wp1 as C
    owners = {}
    for c in crates:
        root = sb + "/" + PLUG_REL + "/" + C.ROOT[c] + "/🦀️.rs"
        for f, parent in wp1files.tree(root):
            owners.setdefault(f, c)
    for base, dirs, files in os.walk(sb + "/" + PLUG_REL):
        dirs[:] = [d for d in dirs if d != "node_modules"]
        for f in files:
            if f.endswith(".rs"):
                p = os.path.join(base, f)
                if p not in owners:
                    rewrite(p, None, names, False)
    for p, c in owners.items():
        rewrite(p, lib_name(c), names, True)
    for c in crates:
        rootf = sb + "/" + PLUG_REL + "/" + C.ROOT[c] + "/🦀️.rs"
        shims = {}
        for f, parent in wp1files.tree(rootf):
            txt = open(f, encoding="utf8").read()
            for m in re.finditer(r"#\[macro_export\][^\n]*\n(?:\s*#\[[^\n]*\n)*\s*macro_rules!\s+([A-Za-z_0-9]+)", txt):
                shims[m.group(1)] = mount_cfg(f, parent)
        if shims:
            with open(rootf, "a", encoding="utf8") as h:
                h.write("\n" + "".join(f"{cfg}pub use crate::{s};\n" for s, cfg in sorted(shims.items())))
    pk = sb + "/" + PK_REL + "/🦀️.rs"
    mounts = "".join(f'#[path = "{os.path.relpath(sb + "/" + PLUG_REL + "/" + C.ROOT[c] + "/🦀️.rs", os.path.dirname(pk))}"]\npub mod {lib_name(c)};\n' for c in crates)
    if mode == "crates":
        open(pk, "w", encoding="utf8").write("#![allow(unused)]\n" + mounts)
    else:
        open(pk, "a", encoding="utf8").write("\n" + mounts)


PATHRX = re.compile(r'((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;')
PATHATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')


def stub_missing(tree):
    """peers add mounts before their files exist: create empty stand-ins so rustc does not abort (logged)."""
    made = []
    for base, dirs, files in os.walk(tree):
        dirs[:] = [d for d in dirs if d != "node_modules"]
        for f in files:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(base, f)
            text = open(p, encoding="utf8", errors="ignore").read()
            clean = blank(text)
            for m in PATHRX.finditer(clean):
                pm = PATHATTR.search(text[m.start(1):m.end(1)])
                if pm:
                    target = os.path.normpath(os.path.join(base, pm.group(1)))
                    if not os.path.exists(target):
                        os.makedirs(os.path.dirname(target), exist_ok=True)
                        open(target, "w").write("")
                        made.append(target.replace(tree + "/", ""))
    return made


def run(sb, label, extra, tag):
    pk = sb + "/" + PK_REL
    env = dict(os.environ, OUT_DIR=sb + "/outdir", CARGO_PKG_VERSION="0.1.0", CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME="semio-framework-plugin")
    started = time.time()
    p = subprocess.run(["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "semio_framework_plugin", "--error-format=json", "--emit=metadata", "-o", "/dev/null", *extra, pk + "/🦀️.rs"], capture_output=True, text=True, env=env, cwd=pk)
    open(f"{GEN}/pwp1-oracle-{tag}-{label}.jsonl", "w").write(p.stderr)
    print("[DEBUG] rustc", label, "rc", p.returncode, "bytes", len(p.stderr), flush=True)
    return p.stderr, time.time() - started


SBPREFIX = re.compile(re.escape(SB) + r"/sb-[^/]+/[AB]/")


def parse(err, sb):
    c = collections.Counter()
    err = SBPREFIX.sub("", err)
    for line in err.split("\n"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") != "error" or str(d.get("message", "")).startswith(("aborting due to", "couldn't create a temp dir")):
            continue
        code = (d.get("code") or {}).get("code") or ""
        sp = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
        file = label = text = ""
        if sp:
            file = os.path.normpath(sp.get("file_name", "").replace(sb + "/", "").replace(REPO + "/", ""))
            label = sp.get("label") or ""
            t = sp.get("text") or []
            if t:
                l = t[0]
                text = (l["text"][l["highlight_start"] - 1:l["highlight_end"] - 1] if len(t) == 1 else l["text"][l["highlight_start"] - 1:]).strip()[:80]
        msg = str(d.get("message", "")).replace(sb + "/", "").replace(REPO + "/", "")
        m2 = re.match(r"couldn't read `([^`]*)`(.*)", msg, re.S)
        if m2:
            msg = "couldn't read `" + os.path.normpath(m2.group(1)) + "`" + m2.group(2)
        c[(code, msg, text, label, file)] += 1
    return c


def short(file):
    return file.replace(PLUG_REL + "/", "P/")


def ext_names():
    carved_libs = "|".join(re.escape(lib_name(c)) for c in carved()) or "NOPE"
    return r"(?:store|protocol|dsl|vcs|pack|serde|serde_json|wit_bindgen|replication|base64_codec|semio_framework(?!_plugin_(?:" + "|".join(re.escape(c.replace("-", "_")) for c in carved()) + r")\b)[A-Za-z0-9_]*)"


EXT_NAMES = None


_NOISE = None


def is_noise(key):
    global _NOISE
    if _NOISE is None:
        e = ext_names()
        _NOISE = [
            re.compile(r"cannot find module or crate `" + e + r"` in this scope"),
            re.compile(r"cannot find [a-z ,]*`[A-Za-z_0-9]+` in (?:module )?`" + e + r"`"),
            re.compile(r"cannot find [a-z ]*`[A-Za-z_0-9]+` in the crate root"),
            re.compile(r"unresolved imports? `" + e + r"(?:::[^`]*)?`(?:, `" + e + r"(?:::[^`]*)?`)*$"),
            re.compile(r"unresolved imports? `" + e + r"::\{[^`]*\}`$"),
            re.compile(r"^cannot find (?:attribute|derive macro|macro)"),
            re.compile(r"^failed to resolve: use of unresolved module or unlinked crate `" + e + r"`"),
            re.compile(r"^unresolved imports? `crate::" + PREFIX + r"[a-z_0-9]+::[^`]*`"),
        ]
    code, msg, text, label = key[:4]
    return any(r.search(msg) for r in _NOISE)


def norm(key):
    code, msg, text, label, file = key
    msg = re.sub(r"crate::" + PREFIX + r"[a-z_0-9]+::", "crate::", msg)
    msg = msg.replace("crate::" + PREFIX, "")
    text = re.sub(r"crate::" + PREFIX + r"[a-z_0-9]+::", "crate::", text)
    label = re.sub(r"crate::" + PREFIX + r"[a-z_0-9]+::", "crate::", label)
    return (code, msg, text, "")


def baseline_only(configs):
    B = SB + "/sb-B"
    if not os.path.isdir(B + "/outdir"):
        chain(B, SNAP.rsplit("/pwp1/snap0", 1)[0] + "/pwp1/snap0")
        neutralize(B + "/" + PLUG_REL)
        os.makedirs(B + "/outdir", exist_ok=True)
    for name in configs:
        cache = f"{GEN}/pwp1-oracle-Bcache-{name}.jsonl"
        if os.path.exists(cache):
            print("cached", name)
            continue
        eb, tb = run(B, "B-" + name, CFGS[name], "B")
        complete = '"aborting due to' in eb[-3000:] or "aborting due to" in eb[-3000:]
        if not complete:
            print("INCOMPLETE baseline run for", name, "- not cached", flush=True)
            continue
        open(cache, "w").write(eb)
        print("baseline", name, "cached", round(tb), "s", flush=True)


def main():
    args = sys.argv[1:]
    if args and args[0] == "--baseline-only":
        baseline_only(args[1].split(","))
        return
    tag = args[0]
    mode = "full"
    configs = ["lib", "test", "wasip2", "lib-features", "test-features"]
    crates = carved()
    seeds = []
    for i, a in enumerate(args):
        if a == "--mode": mode = args[i + 1]
        if a == "--configs": configs = args[i + 1].split(",")
        if a == "--crates": crates = args[i + 1].split(",")
        if a == "--seed": seeds.append(args[i + 1])
    base = f"{SB}/sb-{tag}"
    A, B = base + "/A", SB + "/sb-B"
    chain(A, SNAP.rsplit("/pwp1/snap0", 1)[0] + "/pwp1/snap0") if "--live" not in args else chain(A, REPO)
    if "--live" not in args:
        isolate(A)
    prepare(A, crates, mode)
    for s in seeds:
        rel, old, new = s.split("|||")
        path = A + "/" + PLUG_REL + "/" + rel
        txt = open(path, encoding="utf8").read()
        assert txt.count(old) >= 1, (rel, old)
        open(path, "w", encoding="utf8").write(txt.replace(old, new, 1))
        print("seeded", rel, flush=True)
    made = stub_missing(A + "/" + PLUG_REL)
    if made:
        print("stubbed missing mount targets (peer work in flight):", made, flush=True)
    neutralize(A + "/" + PLUG_REL)
    os.makedirs(A + "/outdir", exist_ok=True)
    if mode == "full" and not os.path.isdir(B + "/outdir"):
        chain(B, SNAP.rsplit("/pwp1/snap0", 1)[0] + "/pwp1/snap0")
        neutralize(B + "/" + PLUG_REL)
        os.makedirs(B + "/outdir", exist_ok=True)
    summary = {}
    for name in configs:
        ea, ta = run(A, "A-" + name, CFGS[name], tag)
        a = parse(ea, A)
        if mode == "crates":
            kept = {k: v for k, v in a.items() if not is_noise(norm(k))}
            print(f"== {name}: errors={sum(a.values())} non-noise={sum(kept.values())} sec={round(ta)}", flush=True)
            for k, v in sorted(kept.items(), key=lambda kv: (kv[0][4], kv[0][0], kv[0][1])):
                print("  ", v, k[0], k[1][:150], "|", k[2][:50], "|", short(k[4]))
            continue
        cache = f"{GEN}/pwp1-oracle-Bcache-{name}.jsonl"
        if os.path.exists(cache):
            eb, tb = open(cache).read(), 0
        else:
            eb, tb = run(B, "B-" + name, CFGS[name], tag)
            if "aborting due to" not in eb[-3000:]:
                raise SystemExit("baseline run incomplete (killed?) for " + name)
            open(cache, "w").write(eb)
        b = parse(eb, B)
        fa, fb = collections.Counter(), collections.Counter()
        where = {}
        for k, v in a.items():
            fa[norm(k)] += v
            where.setdefault(norm(k), collections.Counter())[short(k[4])] += v
        for k, v in b.items():
            fb[norm(k)] += v
        fa = collections.Counter({k: v for k, v in fa.items() if not is_noise(k)})
        fb = collections.Counter({k: v for k, v in fb.items() if not is_noise(k)})
        new, old = fa - fb, fb - fa
        summary[name] = dict(a=sum(a.values()), b=sum(b.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()))
        print(f"== {name}: A errors={sum(a.values())} B errors={sum(b.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}", flush=True)
        for k, v in sorted(new.items(), key=lambda kv: (-kv[1], kv[0]))[:60]:
            print("  +new", v, k[0], k[1][:140], "|", k[2][:40], "|", dict(where[k].most_common(2)))
        for k, v in sorted(old.items(), key=lambda kv: (-kv[1], kv[0]))[:40]:
            print("  -old", v, k[0], k[1][:140], "|", k[2][:40])
    if summary:
        json.dump(summary, open(f"{GEN}/pwp1-oracle-{tag}-summary.json", "w"), indent=1)

if __name__ == "__main__":
    main()
