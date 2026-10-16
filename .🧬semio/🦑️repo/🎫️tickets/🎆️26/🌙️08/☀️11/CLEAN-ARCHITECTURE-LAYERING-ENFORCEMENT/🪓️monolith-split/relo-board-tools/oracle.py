"""🔬️ Slice D full-resolution differential: patched rustc metadata analysis of baseline (S0) vs current tree per crate and configuration."""
import collections
import os
import re
import shutil
import subprocess
import sys
from importlib import util as _util

_spec = _util.spec_from_file_location("relo_baseline", os.path.join(os.path.dirname(os.path.abspath(__file__)), "baseline.py"))
baseline = _util.module_from_spec(_spec)
_spec.loader.exec_module(baseline)

R = "/Users/ueli/Documents/semio"
SCR = sys.argv[1]
TREES = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite",
    "✏️s/🔌️plugins/🧩️puzzle",
    "🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners",
    "🧰️framework/🔨️modules/🖱️ui/🎬️scene",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer",
]
CRATES = {
    "infinite": ("semio_framework_os_infinite", TREES[0] + "/📦️packages/🦀️rust/🦀️.rs"),
    "puzzle2d": ("semio_s_artifact_puzzle_2d", TREES[1] + "/🗿️artifacts/◻️2d/🦀️.rs"),
    "puzzle5d": ("semio_s_artifact_puzzle_5d", TREES[1] + "/🗿️artifacts/🖐️5d/🦀️.rs"),
    "plane": ("semio_framework_ui_scene_plane", TREES[3] + "/🎬️scenes/🖼️plane/🦀️.rs"),
    "renderer": ("semio_framework_os_renderer_wgpu", TREES[4] + "/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/🦀️.rs"),
}
CONFIGS = {
    "lib": [],
    "test": ["--test"],
    "web": ["--target", "wasm32-unknown-unknown"],
    "testweb": ["--test", "--target", "wasm32-unknown-unknown"],
    "p2": ["--target", "wasm32-wasip2"],
    "testp2": ["--test", "--target", "wasm32-wasip2"],
}
KEEP_EXTERN = {"self", "core", "alloc", "std", "proc_macro", "test"}


def copy_trees(src_root, dst_root):
    shutil.rmtree(dst_root, ignore_errors=True)
    for tree in TREES:
        src = os.path.join(src_root, tree)
        if os.path.isdir(src):
            shutil.copytree(src, os.path.join(dst_root, tree), symlinks=True, ignore=shutil.ignore_patterns("target", "node_modules"))


def restore_pristine(root):
    pristine = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/relo-board-baseline"
    count = 0
    for name in os.listdir(pristine):
        rel = name.replace("__", "/")
        if rel.endswith(".rs") and any(rel.startswith(tree) for tree in TREES):
            shutil.copyfile(os.path.join(pristine, name), os.path.join(root, rel))
            count += 1
    return count


def neutralize(root):
    files, mods = [], set()
    for dp, _, fn in os.walk(root):
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                files.append(p)
                for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", open(p, encoding="utf-8", errors="ignore").read(), re.M):
                    mods.add(m.group(1))
    keep = {"crate", "super", "self", "infinite"} | mods
    glob_rx = re.compile(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$", re.M)
    extern_rx = re.compile(r"^[ \t]*(?:#\[[^\]]*\][ \t]*)?(?:pub(?:\([^)]*\))?\s+)?extern crate\s+(\w+)[^;]*;[ \t]*$", re.M)
    count = [0, 0]
    for p in files:
        t = open(p, encoding="utf-8", errors="ignore").read()

        def glob_sub(m):
            if m.group(2) in keep:
                return m.group(0)
            count[0] += 1
            return m.group(1) + "const _: () = ();"

        def extern_sub(m):
            if m.group(1) in KEEP_EXTERN:
                return m.group(0)
            count[1] += 1
            return ""

        t2 = extern_rx.sub(extern_sub, glob_rx.sub(glob_sub, t))
        if t2 != t:
            open(p, "w", encoding="utf-8").write(t2)
    return count


def parse(err, root):
    lines = err.split("\n")
    out = collections.Counter()
    for i, line in enumerate(lines):
        m = re.match(r"^(error(\[E\d+\])?|warning)(\[[^\]]*\])?: (.*)", line)
        if not m:
            continue
        loc = ""
        for j in range(i + 1, min(i + 4, len(lines))):
            mm = re.match(r"^\s*--> (.*?):(\d+):(\d+)", lines[j])
            if mm:
                loc = mm.group(1)
                break
        loc = loc.replace(root + "/", "")
        loc = re.sub(r"(\.\./|\./)+", "", loc)
        out[(m.group(1), m.group(4).replace(root + "/", ""), loc)] += 1
    return out


def fill_static(roots, rounds=40):
    """🧭️ Copies, from the repo into every root, each `#[path]`/`include!` target the sandbox lacks, until the closure is stable."""
    rx = re.compile(r'#\[path\s*=\s*"([^"]+)"\]|\binclude!\(\s*"([^"]+)"\s*\)')
    copied, scanned = [], set()
    for _ in range(rounds):
        fresh = []
        for dp, _, fn in os.walk(roots[-1]):
            for f in fn:
                if not f.endswith(".rs"):
                    continue
                path = os.path.join(dp, f)
                if path in scanned:
                    continue
                scanned.add(path)
                for m in rx.finditer(open(path, encoding="utf-8", errors="ignore").read()):
                    target = os.path.normpath(os.path.join(dp, m.group(1) or m.group(2)))
                    rel = os.path.relpath(target, roots[-1])
                    if rel.startswith("..") or os.path.exists(target) or not os.path.exists(os.path.join(R, rel)):
                        continue
                    unit = os.path.dirname(rel) if os.path.basename(rel) == "🦀️.rs" else rel
                    for root in roots:
                        if os.path.isdir(os.path.join(R, unit)):
                            shutil.copytree(os.path.join(R, unit), os.path.join(root, unit), symlinks=True, dirs_exist_ok=True, ignore=shutil.ignore_patterns("target", "node_modules"))
                        else:
                            os.makedirs(os.path.dirname(os.path.join(root, unit)), exist_ok=True)
                            shutil.copyfile(os.path.join(R, unit), os.path.join(root, unit))
                    copied.append(unit)
                    fresh.append(unit)
        if not fresh:
            break
    return copied


def fill_missing(roots, crate, extra, rounds=60):
    """📥️ Copies every `#[path]` target outside the sandbox trees (directory of a missing `🦀️.rs`, else the file) from the repo into all roots until the crate no longer aborts on a missing file."""
    copied = []
    for _ in range(rounds):
        name, lib = CRATES[crate]
        p = subprocess.run(["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", name, "--emit=metadata", "-o", "/dev/null", *extra, os.path.join(roots[-1], lib)], capture_output=True, text=True)
        missing = [m for m in re.findall(r"couldn't find file `([^`]+)`", p.stderr)]
        fresh = []
        for path in missing:
            rel = os.path.relpath(os.path.normpath(path), roots[-1])
            source = os.path.join(R, rel)
            if rel.startswith("..") or not os.path.exists(source):
                continue
            target_rel = os.path.dirname(rel) if os.path.basename(rel) == "🦀️.rs" else rel
            if target_rel in copied:
                continue
            for root in roots:
                target = os.path.join(root, target_rel)
                if os.path.isdir(os.path.join(R, target_rel)):
                    shutil.copytree(os.path.join(R, target_rel), target, symlinks=True, dirs_exist_ok=True, ignore=shutil.ignore_patterns("target", "node_modules"))
                else:
                    os.makedirs(os.path.dirname(target), exist_ok=True)
                    shutil.copyfile(os.path.join(R, target_rel), target)
            copied.append(target_rel)
            fresh.append(target_rel)
        if not fresh:
            return copied
    return copied


def run(root, crate, extra, tag):
    name, lib = CRATES[crate]
    p = subprocess.run(["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", name, "--emit=metadata", "-o", "/dev/null", *extra, os.path.join(root, lib)], capture_output=True, text=True)
    open(f"{SCR}/oracle-{crate}-{tag}.err", "w").write(p.stderr)
    return parse(p.stderr, root)


if __name__ == "__main__":
    base, cur = SCR + "/S0n", SCR + "/S1n"
    copy_trees(R, base)
    restored = len(baseline.build(base))
    copy_trees(R, cur)
    print("baseline = current tree with", restored, "pre-slice-D files rebuilt")
    print("neutralized baseline (globs, externs):", neutralize(base), "current:", neutralize(cur))
    crates = sys.argv[2].split(",") if len(sys.argv) > 2 else list(CRATES)
    configs = sys.argv[3].split(",") if len(sys.argv) > 3 else list(CONFIGS)
    for crate in crates:
        if crate == "renderer":
            extra_trees = fill_static([base, cur])
            extra_trees += fill_missing([base, cur], crate, [], rounds=4)
            print("filled outside-tree mounts:", len(extra_trees), extra_trees[:12])
            print("re-neutralized:", neutralize(base), neutralize(cur))
        for tag in configs:
            a = run(base, crate, CONFIGS[tag], tag + "-0")
            b = run(cur, crate, CONFIGS[tag], tag + "-1")
            ea = sum(v for k, v in a.items() if k[0].startswith("error"))
            eb = sum(v for k, v in b.items() if k[0].startswith("error"))
            only_cur, only_base = b - a, a - b
            print(f"== {crate} {tag}: baseline errors={ea} current errors={eb}; only-current={sum(only_cur.values())} only-baseline={sum(only_base.values())}")
            for (kind, msg, loc), c in sorted(only_cur.items())[:80]:
                print(f"  +cur {c}x {kind}: {msg[:160]} @ {loc[-120:]}")
            for (kind, msg, loc), c in sorted(only_base.items())[:80]:
                print(f"  -cur {c}x {kind}: {msg[:160]} @ {loc[-120:]}")
