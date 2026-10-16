#!/usr/bin/env python3
# 🔬️ Union oracle of the RENDERER-INTEGRATION-1 pass (generalised from rwp1-tools/uo.py).
# usage: uo.py <tag> [--mode crates|full] [--configs lib,test,wasm,wasm-test] [--crates a,b|real] [--baseline snap0] [--seed]
# A (crates mode) = the named crates mounted as modules of ONE crate (crate-name and `crate::` paths rewritten); signal errors grouped.
# A (full mode)   = the live monolith (hubs' `extern crate <crate> as <module>;` rewritten to `use`) + the crates as modules.
# B (full mode)   = the live tree with the files named in ri1-tools/touched.txt restored from the pre-edit snapshot (or deleted when new).
import collections, importlib.util, json, os, re, shutil, subprocess, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import modtree
from rs import blank

spec = importlib.util.spec_from_file_location("rwp0oracle", SPLIT + "/rwp0-tools/oracle.py")
O = importlib.util.module_from_spec(spec)
spec.loader.exec_module(O)
CFGS = dict(O.CFGS)
CFGS['lib-tk'] = ['--cfg', 'feature="testkit"']
CFGS['wasm-tk'] = ['--target', 'wasm32-unknown-unknown', '--cfg', 'feature="testkit"']


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
R_EXTERN = re.compile(r"(pub\s+)?extern\s+crate\s+(" + PREFIX + r"[A-Za-z0-9_]+)\s+as\s+([A-Za-z0-9_]+)\s*;")
R_EXTERN_BARE = re.compile(r"(pub\s+)?extern\s+crate\s+(" + PREFIX + r"[A-Za-z0-9_]+)\s*;")


def rewrite_owned(path, libname, names):
    text = open(path, encoding="utf8").read()
    new = sub_outside_strings(text, R_CRATE, f"crate::{libname}::")
    new = sub_outside_strings(new, R_PUBCRATE, f"pub(in crate::{libname})")
    new = sub_outside_strings(new, R_EXT, lambda m: "crate::" + PREFIX + m.group(1) if PREFIX + m.group(1) in names else m.group(0))
    new = sub_outside_strings(new, R_EXTERN, lambda m: f"{m.group(1) or ''}use crate::{m.group(2)} as {m.group(3)};" if m.group(2) in names else m.group(0))
    new = sub_outside_strings(new, R_EXTERN_BARE, lambda m: f"{m.group(1) or ''}use crate::{m.group(2)};" if m.group(2) in names else m.group(0))
    if new != text:
        open(path, "w", encoding="utf8").write(new)


def rewrite_extern_only(path, names):
    text = open(path, encoding="utf8").read()
    new = sub_outside_strings(text, R_EXT, lambda m: "crate::" + PREFIX + m.group(1) if PREFIX + m.group(1) in names else m.group(0))
    new = sub_outside_strings(new, R_EXTERN, lambda m: f"{m.group(1) or ''}use crate::{m.group(2)} as {m.group(3)};" if m.group(2) in names else m.group(0))
    if new != text:
        open(path, "w", encoding="utf8").write(new)


def prepare(sb, crates, table, mode):
    names = {lib_name(c) for c in crates}
    eng = sb + "/" + ENG_REL
    owners = {}
    for c in crates:
        root = sb + "/" + T_REL + "/" + table[c] + "/🦀️.rs"
        for f in modtree.tree(root):
            owners.setdefault(f, c)
    owned_set = set(owners)
    if mode == "full":
        for base, dirs, files in os.walk(eng):
            dirs[:] = [d for d in dirs if d != "node_modules"]
            for f in files:
                if f.endswith(".rs"):
                    p = os.path.join(base, f)
                    if p not in owned_set:
                        rewrite_extern_only(p, names)
    for p, c in owners.items():
        rewrite_owned(p, lib_name(c), names)
    lib = sb + "/" + LIB_REL
    mounts = "".join(('#[cfg(not(target_arch = "wasm32"))]\n' if c in NATIVE_ONLY else '#[cfg(target_arch = "wasm32")]\n' if c in WASM_ONLY else "") + f'#[path = "{os.path.relpath(sb + "/" + T_REL + "/" + table[c] + "/🦀️.rs", os.path.dirname(lib))}"]\npub mod {lib_name(c)};\n' for c in crates)
    if mode == "crates":
        open(lib, "w", encoding="utf8").write(mounts)
    else:
        text = open(lib, encoding="utf8").read()
        open(lib, "w", encoding="utf8").write(text + "\n" + mounts)


def restore_touched(sb):
    touched = [l.rstrip("\n") for l in open(os.path.dirname(os.path.abspath(__file__)) + "/touched.txt", encoding="utf8") if l.strip() and not l.startswith("#")]
    for rel in touched:
        dst = sb + "/" + ENG_REL + "/" + rel
        src = SNAP + "/" + rel
        if os.path.exists(src):
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copy(src, dst)
        elif os.path.exists(dst):
            os.remove(dst)


def run(sb, label, extra, tag):
    pk = sb + "/" + PKG_REL
    env = dict(os.environ, OUT_DIR=sb + "/outdir", CARGO_PKG_VERSION="0.1.0", CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME="semio-framework-os-renderer-wgpu")
    started = time.time()
    p = subprocess.run(["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "semio_framework_os_renderer_wgpu", "--error-format=json", "--emit=metadata", "-o", "/dev/null", *extra, pk + "/📚️library/🦀️.rs"], capture_output=True, text=True, env=env, cwd=pk)
    os.makedirs(GEN, exist_ok=True)
    open(f"{GEN}/ri1-oracle-{tag}-{label}.jsonl", "w").write(p.stderr)
    return p.stderr, time.time() - started


def parse(err, sb):
    c = collections.Counter()
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
        c[(code, msg, text, label, file)] += 1
    return c


EXT_NOISE = re.compile(r"cannot find [a-z ,]*`[A-Za-z_0-9]+` in (?:module )?`(?:protocol|store_sync|store|dsl|dsl_core|infinite_world)`|unresolved imports? `[^`]*`(?:, `[^`]*`)*$|unresolved module or unlinked crate `(?!semio_framework_os_renderer_wgpu_)[A-Za-z0-9_]+`|^cannot find attribute|^cannot find derive macro|^cannot find macro `(?:serde_json|json|wasm_bindgen|thread_local)")
RES = {"E0412", "E0425", "E0422", "E0423", "E0432", "E0433", "E0603", "E0616", "E0624", "E0252", "E0405", "E0407", "E0408", "E0424", "E0426", "E0428", "E0430", "E0431", "E0434", "E0435", "E0437", "E0438", "E0449", "E0451", "E0530", "E0531", "E0532", "E0574", "E0575", "E0576", "E0583", "E0658", "E0659", "E0255", "E0254"}


def noise(key):
    code, msg, text, label, file = key
    if "--all" not in sys.argv and code not in RES and not msg.startswith(("couldn't read", "couldn't find file")):
        return True
    if code == "E0432" and "crate::" + PREFIX in msg:
        return False
    return bool(EXT_NOISE.search(label)) or bool(EXT_NOISE.search(msg))


def short(file):
    return file.replace(ENG_REL + "/", "E/").replace("🎯️targets/🧊️wgpu/", "T/").replace("🧱️elements/", "el/")


def chain_snap(sb):
    real, box = REPO, sb
    parts = ENG_REL.split("/")
    shutil.rmtree(sb, ignore_errors=True)
    os.makedirs(sb)
    for part in parts:
        for e in os.listdir(real):
            if e == part:
                continue
            os.symlink(real + "/" + e, box + "/" + e)
        real, box = real + "/" + part, box + "/" + part
        os.makedirs(box, exist_ok=True)
    subprocess.check_call(["rsync", "-a", "--exclude", "node_modules", "--exclude", "target", SNAP + "/", box + "/"])


def main():
    args = sys.argv[1:]
    tag = args[0]
    mode, configs, crate_arg = "crates", ["lib", "test", "wasm", "wasm-test"], "real"
    for i, a in enumerate(args):
        if a == "--mode":
            mode = args[i + 1]
        if a == "--configs":
            configs = args[i + 1].split(",")
        if a == "--crates":
            crate_arg = args[i + 1]
    table = crate_table()
    real = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/real.json"))
    crates = real if crate_arg == "real" else crate_arg.split(",")
    assert all(c in table for c in crates), [c for c in crates if c not in table]
    base = f"{RS}/sb-{tag}-{mode}"
    A, B = base + "/A", base + "/B"
    if "--from-snap" in args:
        chain_snap(A)
    else:
        O.chain(A)
    prepare(A, crates, table, mode)
    seeds = [a.split(":", 1)[1] for a in args if a.startswith("seed:")]
    SEEDS = {
        "pub": (T_REL + "/🧊️renderer/🗂️asset-authority/🦀️.rs", "pub fn reserve_renderer_asset_request", "fn reserve_renderer_asset_request"),
        "import": (T_REL + "/👆️ui-engine-focus/🦀️.rs", "use ui_wgpu::wgpu::{ActionDescriptor, NodeId, Rect, UiNode};", ""),
        "path": (T_REL + "/🔧️host-plumbing/🦀️.rs", "⏰️deadlines/🦀️.rs", "📛️absent/🦀️.rs"),
    }
    for name in seeds:
        rel, old, new = SEEDS[name]
        sp = A + "/" + rel
        s = open(sp, encoding="utf8").read()
        assert s.count(old) >= 1, (name, old)
        open(sp, "w", encoding="utf8").write(s.replace(old, new, 1))
        print("seeded", name, flush=True)
    O.neutralize(A + "/" + ENG_REL)
    os.makedirs(A + "/outdir", exist_ok=True)
    if mode == "full":
        O.chain(B)
        restore_touched(B)
        prepare(B, [c for c in crates if c not in json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/new_in_a.json"))], table, mode)
        O.neutralize(B + "/" + ENG_REL)
        os.makedirs(B + "/outdir", exist_ok=True)
    for name in configs:
        ea, ta = run(A, "A-" + name, CFGS[name], tag)
        a = parse(ea, A)
        if mode == "crates":
            print(f"== {name}: errors={sum(a.values())} sec={round(ta)}", flush=True)
            kept = {k: v for k, v in a.items() if not noise(k)}
            print(f"   (noise suppressed: {sum(a.values()) - sum(kept.values())})")
            for k, v in sorted(kept.items(), key=lambda kv: (kv[0][4], kv[0][0], kv[0][1])):
                print("  ", v, k[0], k[1][:160], "|", k[2][:50], "|", short(k[4]))
            continue
        eb, tb = run(B, "B-" + name, CFGS[name], tag)
        b = parse(eb, B)
        fa, fb = collections.Counter(), collections.Counter()
        where = {}
        for k, v in a.items():
            fa[k[:4]] += v
            where.setdefault(k[:4], collections.Counter())[short(k[4])] += v
        for k, v in b.items():
            fb[k[:4]] += v
        new, old = fa - fb, fb - fa
        print(f"== {name}: A errors={sum(a.values())} B errors={sum(b.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}", flush=True)
        sig = {k: v for k, v in new.items() if not noise(k + ("",))}
        print(f"   (new rows that are noise: {sum(new.values()) - sum(sig.values())}; signal rows: {len(sig)})")
        for k, v in sorted(sig.items(), key=lambda kv: (-kv[1], kv[0]))[:40]:
            print("  +SIGNAL", v, k[0], k[1][:120], "|", k[2][:40], "|", dict(where[k].most_common(2)))
        for k, v in sorted(old.items(), key=lambda kv: (-kv[1], kv[0]))[:15]:
            print("  -old", v, k[0], k[1][:120], "|", k[2][:40])


if __name__ == "__main__":
    main()
