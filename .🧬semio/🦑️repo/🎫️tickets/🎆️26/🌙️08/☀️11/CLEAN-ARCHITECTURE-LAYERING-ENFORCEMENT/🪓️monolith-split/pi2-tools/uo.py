#!/usr/bin/env python3
"""🔬️ Union differential of PLUGIN-INTEGRATION-1 (generalised from the pwp1/pwp2/ri1 oracles).
usage: uo.py <tag> [--configs lib,test,wasip2,lib-features,test-features] [--seed rel|||old|||new]... [--crates-only]
A = the live plugin tree; every flipped crate (manifest `[lib] path = "../../🦀️.rs"`, non-stub root) is mounted as module `semio_framework_plugin_<key>` of the facade crate, its `crate::` and `pub(crate)` rewritten per crate, hub `extern crate` units rewritten to `use`.
B = the same preparation of the live tree with every file listed in touched.json restored from the pre-edit copy (created files removed) and the crates named in flipped.json treated as not flipped.
Both get the sandbox patches of 📓️RESOLUTION-ORACLE.md. Prints the multiset difference of normalised signal rows."""
import collections, json, os, re, shutil, subprocess, sys, time
from lib import *
from rs import blank
import tree as T
import ed

EXCLUDE = ['node_modules', 'target', '.cache', '🖥️host', '🧠️interpreter', '🏇️mounted-owner', '🏛️ownership', '🧬️guest-contract', '🖨️describe', '📦️packages', '.🧬semio', '🌐️browser-bundle', '🏗️build', '🧷️companions']
FEATS = ['artifact-app-testing', 'media-turn-testing', 'reserved-producer-testing', 'original-actor-context-testing']
FEATARGS = [x for f in FEATS for x in ('--cfg', 'feature="%s"' % f)]
CFGS = {'lib': [], 'test': ['--test'], 'wasip2': ['--target', 'wasm32-wasip2'], 'wasip2-test': ['--test', '--target', 'wasm32-wasip2'], 'lib-features': FEATARGS, 'test-features': ['--test'] + FEATARGS, 'wasm': ['--target', 'wasm32-unknown-unknown'], 'wasmp2': ['--target', 'wasm32-unknown-unknown', '--cfg', 'target_env="p2"', '-A', 'explicit_builtin_cfgs_in_flags'], 'wasmp2-test': ['--test', '--target', 'wasm32-unknown-unknown', '--cfg', 'target_env="p2"', '-A', 'explicit_builtin_cfgs_in_flags']}
PK_REL = PLUG_REL + "/" + PKGDIR
SBROOT = SCRATCH + "/pi2"
FLIPPED = TOOLS + "/flipped.json"
CR = json.load(open(TOOLS + "/crates.json"))

R_CRATE = re.compile(r"(?<![A-Za-z0-9_$])(?<!::)crate::")
R_PUBCRATE = re.compile(r"\bpub\(crate\)")
R_EXT = re.compile(r"(?<![A-Za-z0-9_])(?<!::)" + PFX + r"([A-Za-z0-9_]+)(?=::)")
R_EXTERN = re.compile(r"(pub(?:\([^)]*\))?\s+)?extern\s+crate\s+(" + PFX + r"[A-Za-z0-9_]+)\s+as\s+([A-Za-z0-9_]+)\s*;")
R_USE_SELF_CRATE = re.compile(r"((?:pub(?:\([^)]*\))?\s+)?)use\s+crate\s+as\s+([A-Za-z0-9_]+)\s*;")
R_USE_CRATE = re.compile(r"(?<![A-Za-z0-9_:])use\s+(" + PFX + r"[A-Za-z0-9_]+)(\s+as\s+[A-Za-z0-9_]+\s*;)")
R_EXTERN_BARE = re.compile(r"(pub(?:\([^)]*\))?\s+)?extern\s+crate\s+(" + PFX + r"[A-Za-z0-9_]+)\s*;")


def flipped_by_me():
    if os.environ.get("PI2_NOFLIP"):
        return []
    return json.load(open(FLIPPED)) if os.path.exists(FLIPPED) else []


def sub_outside_strings(text, regex, repl):
    clean = blank(text)
    out, last = [], 0
    for m in regex.finditer(clean):
        out.append(text[last:m.start()])
        out.append(m.expand(repl) if isinstance(repl, str) else repl(m))
        last = m.end()
    out.append(text[last:])
    return "".join(out)


def is_stub(path):
    t = read(path)
    body = [l for l in t.split("\n") if l.strip() and not l.lstrip().startswith("//") and not l.lstrip().startswith("extern crate")]
    return not body


BASE_REAL = ["child", "composition-emission", "composition-publication", "publication-mounted", "document-media", "job", "command", "peer", "retained-command", "reactor-app", "reactor-tasks", "reactor-lifecycle", "runtime-instances", "reactor-patches", "runtime-ops", "extension", "reactor-surfaces", "reactor-core", "reactor-jobs", "runtime-exchange", "reactor-turn", "reactor-wit-bridge", "reactor-context", "time-travel-panel", "abi", "actor-host", "identity", "interaction", "operation", "window-config", "window-transient", "window-kits", "window-kits-rich", "decl-views", "decl-emission", "decl-app", "app-builder", "decl-surface", "decl-registry", "plugin-program"]


def real_crates(exclude=()):
    """crates whose sources are owned by the crate itself: the WP1/WP2 flips plus every crate this pass flipped (flipped.json)"""
    out = {}
    for k in BASE_REAL + flipped_by_me():
        v = CR[k]
        root = REPO + "/" + v["dir"] + "/🦀️.rs"
        if k not in exclude and os.path.exists(root):
            out[k] = root
    return out


def libn(k):
    return PFX + k.replace("-", "_")


MANIFEST_DIR = {}


def rewrite_owned(path, lib, names):
    original = text = read(path)
    if lib in MANIFEST_DIR and 'env!("CARGO_MANIFEST_DIR")' in text:
        text = text.replace('env!("CARGO_MANIFEST_DIR")', json.dumps(MANIFEST_DIR[lib], ensure_ascii=False))
    new = sub_outside_strings(text, R_CRATE, f"crate::{lib}::")
    new = sub_outside_strings(new, R_USE_SELF_CRATE, lambda m: f"{m.group(1)}use crate::{lib} as {m.group(2)};")
    new = sub_outside_strings(new, R_PUBCRATE, f"pub(in crate::{lib})")
    new = sub_outside_strings(new, R_EXTERN, lambda m: f"{m.group(1) or ''}use crate::{m.group(2)} as {m.group(3)};" if m.group(2) in names else m.group(0))
    new = sub_outside_strings(new, R_EXTERN_BARE, lambda m: f"{m.group(1) or ''}use crate::{m.group(2)};" if m.group(2) in names else m.group(0))
    new = sub_outside_strings(new, R_USE_CRATE, lambda m: f"use crate::{m.group(1)}{m.group(2)}" if m.group(1) in names else m.group(0))
    new = sub_outside_strings(new, R_EXT, lambda m: "crate::" + PFX + m.group(1) if PFX + m.group(1) in names else m.group(0))
    if new != original:
        write(path, new)


def rewrite_foreign(path, names):
    text = read(path)
    new = sub_outside_strings(text, R_EXTERN, lambda m: f"{m.group(1) or ''}use crate::{m.group(2)} as {m.group(3)};" if m.group(2) in names else m.group(0))
    new = sub_outside_strings(new, R_EXTERN_BARE, lambda m: f"{m.group(1) or ''}use crate::{m.group(2)};" if m.group(2) in names else m.group(0))
    new = sub_outside_strings(new, R_USE_CRATE, lambda m: f"use crate::{m.group(1)}{m.group(2)}" if m.group(1) in names else m.group(0))
    new = sub_outside_strings(new, R_EXT, lambda m: "crate::" + PFX + m.group(1) if PFX + m.group(1) in names else m.group(0))
    if new != text:
        write(path, new)


def neutralize(tree):
    files, names = [], set()
    for dp, dn, fn in os.walk(tree):
        dn[:] = [d for d in dn if d != 'node_modules']
        for f in fn:
            if f.endswith('.rs'):
                p = os.path.join(dp, f)
                files.append(p)
                t = open(p, encoding='utf-8', errors='ignore').read()
                for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:mod|enum|struct|trait|type|union)\s+(\w+)', t, re.M):
                    names.add(m.group(1))
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
        if t2 != t:
            open(p, 'w', encoding='utf-8').write(t2)


def copy_tree(sb):
    shutil.rmtree(sb, ignore_errors=True)
    os.makedirs(sb + '/' + PLUG_REL)
    ex = []
    for e in EXCLUDE:
        ex += ['--exclude', e]
    for attempt in range(8):
        try:
            subprocess.check_call(['rsync', '-a', *ex, PLUG + '/', sb + '/' + PLUG_REL + '/'])
            break
        except subprocess.CalledProcessError:
            time.sleep(20)
    else:
        raise SystemExit('rsync kept failing (file table overflow?)')
    os.makedirs(sb + '/' + PK_REL, exist_ok=True)
    for f in ('🦀️.rs', 'Cargo.toml'):
        shutil.copy(REPO + '/' + PK_REL + '/' + f, sb + '/' + PK_REL + '/' + f)


def restore_touched(sb):
    st = ed._state()
    for key in st["existed"]:
        if not key.startswith(PLUG_REL + "/") or "/" + PKGDIR + "/" in key:
            continue
        dst = sb + "/" + key
        if os.path.exists(os.path.dirname(dst)) or True:
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copy2(ed.ORIG + "/" + key, dst)
    for key in st["created"]:
        if not key.startswith(PLUG_REL + "/"):
            continue
        dst = sb + "/" + key
        if os.path.exists(dst):
            os.remove(dst)


PATHRX = re.compile(r'((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;')
PATHATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCRX = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')


def stub_missing(tree):
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


def prepare(sb, real, mode="full"):
    names = {libn(k) for k in real}
    plug = sb + "/" + PLUG_REL
    owners = {}
    for k, root in real.items():
        sroot = sb + "/" + os.path.relpath(root, REPO)
        for f, parent, n in T.tree(sroot):
            owners.setdefault(f, k)
    for base, dirs, files in os.walk(plug):
        dirs[:] = [d for d in dirs if d != "node_modules"]
        for f in files:
            if f.endswith(".rs"):
                p = os.path.join(base, f)
                if p not in owners:
                    rewrite_foreign(p, names)
    shims = {}
    MANIFEST_DIR.clear()
    for k in real:
        MANIFEST_DIR[libn(k)] = sb + "/" + CR[k]["dir"] + "/" + PKGDIR
        os.makedirs(MANIFEST_DIR[libn(k)], exist_ok=True)
    for p, k in owners.items():
        rewrite_owned(p, libn(k), names)
    for k, root in real.items():
        sroot = sb + "/" + os.path.relpath(root, REPO)
        macros = []
        for f, parent, n in T.tree(sroot):
            txt = read(f)
            for m in re.finditer(r"#\[macro_export\][^\n]*\n(?:\s*#\[[^\n]*\n)*\s*macro_rules!\s+([A-Za-z_0-9]+)", txt):
                macros.append(m.group(1))
        if macros:
            with open(sroot, "a", encoding="utf8") as h:
                h.write("\n" + "".join(f"pub use crate::{s};\n" for s in sorted(set(macros))))
    pk = sb + "/" + PK_REL + "/🦀️.rs"
    mounts = "".join(f'#[path = "{os.path.relpath(sb + "/" + os.path.relpath(root, REPO), os.path.dirname(pk))}"]\npub mod {libn(k)};\n' for k, root in sorted(real.items()))
    if mode == "crates":
        write(pk, "#![allow(unused)]\n#![feature(associated_type_defaults)]\n" + mounts)
    else:
        with open(pk, "a", encoding="utf8") as h:
            h.write("\n" + mounts)


def run(sb, label, extra, tag):
    pk = sb + "/" + PK_REL
    env = dict(os.environ, OUT_DIR=sb + "/outdir", CARGO_PKG_VERSION="0.1.0", CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME="semio-framework-plugin")
    started = time.time()
    if os.environ.get("PI2_WASM"):
        extra = [*extra, "--target", "wasm32-unknown-unknown", "--cfg", 'target_env="p2"', "-A", "explicit_builtin_cfgs_in_flags"]
    p = subprocess.run(["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "semio_framework_plugin", "--error-format=json", "--emit=metadata", "-o", "/dev/null", *extra, pk + "/🦀️.rs"], capture_output=True, text=True, env=env, cwd=pk)
    write(f"{GEN}/pi2-oracle-{tag}-{label}.jsonl", p.stderr)
    return p.stderr, time.time() - started


def parse(err, sb):
    c = collections.Counter()
    err = err.replace(sb + "/", "")
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


def norm(key):
    code, msg, text, label, file = key
    f = lambda s: re.sub(r"crate::" + PFX + r"[a-z_0-9]+::", "crate::", s).replace("crate::" + PFX, "")
    return (code, f(msg), f(text), "")


def noise_rx(real):
    allk = [k for k, v in CR.items() if v.get("kind") in ("lower", "instance", "support")]
    e = r"(?:store|protocol|dsl|vcs|pack|serde|serde_json|wit_bindgen|replication|base64_codec|semio_framework(?!_plugin_(?:" + "|".join(re.escape(c.replace("-", "_")) for c in allk) + r")\b)[A-Za-z0-9_]*)"
    return [
        re.compile(r"cannot find module or crate `" + e + r"` in this scope"),
        re.compile(r"cannot find [a-z ,]*`[A-Za-z_0-9]+` in (?:module )?`" + e + r"`"),
        re.compile(r"cannot find [a-z ]*`[A-Za-z_0-9]+` in the crate root"),
        re.compile(r"unresolved imports? `" + e + r"(?:::[^`]*)?`(?:, `" + e + r"(?:::[^`]*)?`)*$"),
        re.compile(r"unresolved imports? `" + e + r"::\{[^`]*\}`$"),
        re.compile(r"^cannot find (?:attribute|derive macro|macro)"),
        re.compile(r"^failed to resolve: use of unresolved module or unlinked crate `" + e + r"`"),
    ]


def is_noise(key, rxs):
    return any(r.search(key[1]) for r in rxs)


def apply_overlay(sb, overlay):
    for relp, text in overlay.items():
        dst = sb + "/" + relp
        if text is None:
            if os.path.exists(dst):
                os.remove(dst)
        else:
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            write(dst, text)


def build(tag, side, real, mode, restore, seeds=(), overlay=None):
    sb = f"{SBROOT}/sb-{tag}/{side}"
    copy_tree(sb)
    if restore:
        restore_touched(sb)
    if overlay:
        apply_overlay(sb, overlay)
    prepare(sb, {k: v for k, v in real.items()}, mode)
    for s in seeds:
        rel_, old, new = s.split("|||")
        path = sb + "/" + PLUG_REL + "/" + rel_
        txt = read(path)
        assert txt.count(old) >= 1, (rel_, old)
        write(path, txt.replace(old, new, 1))
        print("seeded", rel_, flush=True)
    made = stub_missing(sb + "/" + PLUG_REL)
    if made:
        print("stubbed missing mount targets:", made[:6], len(made), flush=True)
    neutralize(sb + "/" + PLUG_REL)
    os.makedirs(sb + "/outdir", exist_ok=True)
    return sb


def keys_of(counter, rxs):
    c = collections.Counter()
    for k, v in counter.items():
        n = norm(k)
        if is_noise(n, rxs):
            continue
        c[n] += v
    return c


def main_crates(args):
    tag = args[0]
    configs = ["lib", "test"]
    seeds = []
    vs = None
    only = None
    for i, a in enumerate(args):
        if a == "--configs":
            configs = args[i + 1].split(",")
        if a == "--seed":
            seeds.append(args[i + 1])
        if a == "--vs":
            vs = args[i + 1]
    overlay, extra = None, []
    for i, a in enumerate(args):
        if a == "--overlay":
            overlay = json.load(open(args[i + 1]))
        if a == "--extra-real":
            extra = args[i + 1].split(",")
    realA = real_crates()
    for k in extra:
        realA[k] = REPO + "/" + CR[k]["dir"] + "/🦀️.rs"
    A = build(tag, "A", realA, "crates", False, seeds, overlay)
    rxs = noise_rx(set(realA))
    for name in configs:
        ea, ta = run(A, "A-" + name, CFGS[name], tag)
        a = parse(ea, A)
        sig = keys_of(a, rxs)
        where = {}
        for k, v in a.items():
            where.setdefault(norm(k), collections.Counter())[short(k[4])] += v
        write(f"{GEN}/pi2-oracle-{tag}-{name}.keys.json", json.dumps([[list(k), v] for k, v in sig.items()], ensure_ascii=False))
        base = collections.Counter()
        if vs and os.path.exists(f"{GEN}/pi2-oracle-{vs}-{name}.keys.json"):
            base = collections.Counter({tuple(k): v for k, v in json.load(open(f"{GEN}/pi2-oracle-{vs}-{name}.keys.json"))})
        new, old = sig - base, base - sig
        print(f"== {name}: errors={sum(a.values())} signal={sum(sig.values())} baseline={sum(base.values())} new={sum(new.values())} gone={sum(old.values())} sec={round(ta)}", flush=True)
        for k, v in sorted(new.items(), key=lambda kv: (-kv[1], kv[0]))[:150]:
            print("  +", v, k[0], k[1][:160], "|", k[2][:44], "|", dict(where[k].most_common(2)))
        for k, v in sorted(old.items(), key=lambda kv: (-kv[1], kv[0]))[:20]:
            print("  -", v, k[0], k[1][:120], "|", k[2][:40])


def main_full_vs(args):
    """full monolith, A only (live + overlay), compared with the stored signal keys of an earlier full run (--vs-full <tag>)"""
    tag = args[0]
    configs = ["lib", "test"]
    seeds, vs, overlay, extra = [], None, None, []
    for i, a in enumerate(args):
        if a == "--configs":
            configs = args[i + 1].split(",")
        if a == "--vs-full":
            vs = args[i + 1]
        if a == "--overlay":
            overlay = json.load(open(args[i + 1]))
        if a == "--extra-real":
            extra = args[i + 1].split(",")
        if a == "--seed":
            seeds.append(args[i + 1])
    realA = real_crates()
    for k in extra:
        realA[k] = REPO + "/" + CR[k]["dir"] + "/🦀️.rs"
    A = build(tag, "A", realA, "full", False, seeds, overlay)
    rxs = noise_rx(set(realA))
    for name in configs:
        ea, ta = run(A, "A-" + name, CFGS[name], tag)
        a = parse(ea, A)
        sig = keys_of(a, rxs)
        where = {}
        for k, v in a.items():
            where.setdefault(norm(k), collections.Counter())[short(k[4])] += v
        write(f"{GEN}/pi2-oracle-{tag}-full-{name}.keys.json", json.dumps([[list(k), v] for k, v in sig.items()], ensure_ascii=False))
        base = collections.Counter()
        pth = f"{GEN}/pi2-oracle-{vs}-full-{name}.keys.json"
        if vs and os.path.exists(pth):
            base = collections.Counter({tuple(k): v for k, v in json.load(open(pth))})
        new, old = sig - base, base - sig
        print(f"== {name}: errors={sum(a.values())} signal={sum(sig.values())} baseline={sum(base.values())} new={sum(new.values())} gone={sum(old.values())} sec={round(ta)}", flush=True)
        for k, v in sorted(new.items(), key=lambda kv: (-kv[1], kv[0]))[:150]:
            print("  +", v, k[0], k[1][:160], "|", k[2][:44], "|", dict(where[k].most_common(2)))
        for k, v in sorted(old.items(), key=lambda kv: (-kv[1], kv[0]))[:60]:
            print("  -", v, k[0], k[1][:120], "|", k[2][:40])


def main():
    args = sys.argv[1:]
    if "--mode" in args and args[args.index("--mode") + 1] == "crates":
        return main_crates(args)
    if "--vs-full" in args or "--baseline-keys" in args:
        return main_full_vs(args)
    tag = args[0]
    configs = ["lib", "test", "wasip2", "lib-features", "test-features"]
    seeds = []
    mode = "full"
    for i, a in enumerate(args):
        if a == "--configs":
            configs = args[i + 1].split(",")
        if a == "--seed":
            seeds.append(args[i + 1])
        if a == "--mode":
            mode = args[i + 1]
    overlay, extra = None, []
    for i, a in enumerate(args):
        if a == "--overlay":
            overlay = json.load(open(args[i + 1]))
        if a == "--extra-real":
            extra = args[i + 1].split(",")
    realA = real_crates()
    mine = flipped_by_me()
    realB = {k: v for k, v in realA.items() if k not in mine}
    for k in extra:
        realA[k] = REPO + "/" + CR[k]["dir"] + "/🦀️.rs"
    A = build(tag, "A", realA, mode, False, seeds, overlay)
    B = build(tag, "B", realB, mode, restore=bool(mine or not overlay), seeds=())
    rxs = noise_rx(set(realA))
    summary = {}
    for name in configs:
        ea, ta = run(A, "A-" + name, CFGS[name], tag)
        eb, tb = run(B, "B-" + name, CFGS[name], tag)
        a, b = parse(ea, A), parse(eb, B)
        fa, fb = collections.Counter(), collections.Counter()
        where = {}
        for k, v in a.items():
            fa[norm(k)] += v
            where.setdefault(norm(k), collections.Counter())[short(k[4])] += v
        for k, v in b.items():
            fb[norm(k)] += v
        fa = collections.Counter({k: v for k, v in fa.items() if not is_noise(k, rxs)})
        fb = collections.Counter({k: v for k, v in fb.items() if not is_noise(k, rxs)})
        new, old = fa - fb, fb - fa
        write(f"{GEN}/pi2-oracle-{tag}-full-{name}.keys.json", json.dumps([[list(k), v] for k, v in fa.items()], ensure_ascii=False))
        write(f"{GEN}/pi2-oracle-{tag}-fullB-{name}.keys.json", json.dumps([[list(k), v] for k, v in fb.items()], ensure_ascii=False))
        summary[name] = dict(a=sum(a.values()), b=sum(b.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()))
        print(f"== {name}: A errors={sum(a.values())} B errors={sum(b.values())} signal A={sum(fa.values())} B={sum(fb.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}", flush=True)
        for k, v in sorted(new.items(), key=lambda kv: (-kv[1], kv[0]))[:80]:
            print("  +new", v, k[0], k[1][:150], "|", k[2][:40], "|", dict(where[k].most_common(2)))
        for k, v in sorted(old.items(), key=lambda kv: (-kv[1], kv[0]))[:30]:
            print("  -old", v, k[0], k[1][:150], "|", k[2][:40])
    write(f"{GEN}/pi2-oracle-{tag}-summary.json", json.dumps(summary, indent=1))

if __name__ == "__main__":
    main()
