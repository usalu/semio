#!/usr/bin/env python3
"""🎚️ Wave A2 one-shot manifest rewrite: every Cargo workspace that declares wasm component profiles ends with exactly the
shipped `wasm-release` profile (one definition, cgu 16, no package overrides) and the local-only `wasm-iterate` profile;
`wasm-dev` and every `wasm-release` package override disappear. Usage: `🎚️profiles.py [--write]` from the repo root."""
import re
import subprocess
import sys

ROOT_MANIFESTS = {"./Cargo.toml", "./🌎️hub/Cargo.toml"}

RELEASE_RATIONALE = """# 🪶️ The ONE shipped component profile (ticket 26/09/23 wave A2): wasm32-wasip2 plugin and extension components and
# the wasm-bindgen browser modules. `describe`, provenance, receipts, module materialization and the trusted catalog all
# act on these exact bytes; nothing else is ever described for commit or shipped. Inherits ship-oriented
# `[profile.release]` above, then overrides for wasm size/runtime:
#
# - opt-level "s" (not "z"): "z" measurably slows hot numeric loops in geometry/FEM solvers for a
#   few extra percent of size; the wasm-opt -Oz post-pass of module materialization gets the "z"-class
#   shrink without paying that runtime cost in every plugin.
# - lto "thin" (not "fat"): fat LTO re-links every plugin cdylib individually — 2-4x slower per plugin
#   for a low single-digit percent size gain over thin. Thin still gets full cross-crate inlining.
# - codegen-units = 16: measured on the raster hub (ticket 26/09/23 BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT,
#   2026-10-13): 317 s / 8.2 GB peak vs 612 s / 10.6 GB at 1, for +11% raw but only +2.4% brotli bytes on the wire
#   (5.13 vs 5.01 MB); the ElemSection::writeBody crash no longer reproduces with the current toolchain.
# - strip = "symbols": drops the wasm `name` custom section, which is pure debug/dev-tooling weight
#   (measured ~14MB on the largest single plugin, ~87MB across the built fleet) never used at runtime.
# - incremental = false: deterministic output; release units are rebuilt whole anyway, so incremental
#   session state would only cost disk in the shared build-dir (`.cargo/config.toml` `build.build-dir`).
# - trim-paths = "object": strips absolute build-host cargo-registry paths retained in panic
#   `Location` strings (wasm32-wasip2 already aborts on panic, so `panic` is not overridden).
"""

RELEASE_REFERENCE = """# 🪶️ The ONE shipped component profile (ticket 26/09/23 wave A2); its rationale and measurements live beside the same
# definition in the repository root `Cargo.toml`.
"""

RELEASE_BODY = """[profile.wasm-release]
inherits = "release"
opt-level = "s"
lto = "thin"
codegen-units = 16
strip = "symbols"
incremental = false
trim-paths = "object"
"""

ITERATE_RATIONALE = """# 🔁️ The single-plugin browser iteration loop only (`bun nx run workspace:components iterate <plugin-id>`): never in a
# release graph, never described for commit, never shipped. Inherits the shipped profile's target settings and trades
# size and speed for compile latency — incremental, no LTO, 256 codegen units, assertions on, symbols kept. Its bytes
# reach no receipt but a local `wasm-iterate` one and stay under the local-only `ITERATE_COMPONENT_MAX_BYTES` (512 MiB).
"""

ITERATE_BODY = """[profile.wasm-iterate]
inherits = "wasm-release"
opt-level = 1
lto = false
codegen-units = 256
incremental = true
debug-assertions = true
overflow-checks = true
strip = "none"
"""

HOT_PACKAGES = [
    ("semio-framework-os-flow", "# 🎚️ An iterated plugin still meets the framework's 8 ms interactive-step contract: below `opt-level = 2` a lowpoly\n# render turn tessellates its seeded mesh in 10-11.5 ms and the runtime traps it with `plugin.internal.interactive-ceiling`.\n"),
    ("semio-framework-os-kernel-neural-engine", "# 🎚️ `Evaluator::evaluate_channels_budgeted` — the topological dag walk itself, run once per hop.\n"),
    ("semio-framework-replication", "# 🎚️ `DslValue`/`OrderedMap` — the per-neuron `Dictionary` every walk clones, merges and hashes.\n"),
    ("semio-framework-pack", "# 🎚️ `pack::json` — the codec that serializes each node's input/output payload on the same hop.\n"),
]

HEADER = re.compile(r"^\s*\[\[?([^\]]+)\]\]?\s*(#.*)?$")


def sections(lines):
    """🧩️ Splits a manifest into `(name, span_start, header)` blocks: a block spans from its leading comment run (the
    contiguous comment lines directly above its header) to the leading comment run of the next block."""
    starts = [index for index, line in enumerate(lines) if HEADER.match(line)]
    blocks = []
    for start in starts:
        lead = start
        while lead > 0 and lines[lead - 1].lstrip().startswith("#"):
            lead -= 1
        blocks.append((HEADER.match(lines[start]).group(1).strip(), lead))
    return [(name, lead, blocks[position + 1][1] if position + 1 < len(blocks) else len(lines)) for position, (name, lead) in enumerate(blocks)]


def rewrite(path, text):
    lines = text.splitlines(keepends=True)
    drop, release_lead = set(), None
    for name, lead, end in sections(lines):
        if name.startswith("profile.wasm-dev") or name.startswith("profile.wasm-release.package.") or name == "profile.wasm-release":
            if name == "profile.wasm-release":
                release_lead = lead
            drop.update(range(lead, end))
    if release_lead is None:
        raise SystemExit(f"{path}: no [profile.wasm-release]")
    rationale = RELEASE_RATIONALE if path in ROOT_MANIFESTS else RELEASE_REFERENCE
    hot = "".join(f"\n{comment}[profile.wasm-iterate.package.{name}]\nopt-level = 2\n" for name, comment in HOT_PACKAGES)
    block = f"{rationale}{RELEASE_BODY}\n{ITERATE_RATIONALE}{ITERATE_BODY}{hot}\n"
    out = []
    for index, line in enumerate(lines):
        if index == release_lead:
            out.append(block)
        if index not in drop:
            out.append(line)
    return re.sub(r"\n{3,}", "\n\n", "".join(out)).rstrip("\n") + "\n"


def verify(paths):
    script = "const e=(a,b,m)=>{if(JSON.stringify(a)!==JSON.stringify(b))throw Error(m+': '+JSON.stringify(a))};const hot=" + repr([name for name, _ in HOT_PACKAGES]).replace("'", '"') + ";for(const path of process.argv.slice(1)){const p=Bun.TOML.parse(await Bun.file(path).text()).profile??{};if(p['wasm-dev'])throw Error(path+': wasm-dev');e(p['wasm-release'],{inherits:'release','opt-level':'s',lto:'thin','codegen-units':16,strip:'symbols',incremental:false,'trim-paths':'object'},path);const {package:pk,...it}=p['wasm-iterate'];e(it,{inherits:'wasm-release','opt-level':1,lto:false,'codegen-units':256,incremental:true,'debug-assertions':true,'overflow-checks':true,strip:'none'},path);e(Object.keys(pk).sort(),[...hot].sort(),path);for(const k of hot)e(pk[k],{'opt-level':2},path)}console.log('toml-verified',process.argv.length-1)"
    subprocess.run(["bun", "-e", script, *paths], check=True)


def main():
    write = "--write" in sys.argv[1:]
    files = subprocess.run(["rg", "-l", "--hidden", "-g", "!.git", "-g", "!**/🎫️tickets/**", "-g", "!**/node_modules/**", "-g", "!**/dist/**", "-g", "!**/⚡️cache/**", "-g", "Cargo.toml", r"^\[profile\.wasm-", "."], capture_output=True, text=True, check=True).stdout.split()
    for path in sorted(files):
        text = open(path, encoding="utf-8").read()
        rendered = rewrite(path, text)
        if write:
            open(path, "w", encoding="utf-8").write(rendered)
    if write:
        verify(sorted(files))
    print(f"{'rewrote' if write else 'rendered'} {len(files)} manifests")


if __name__ == "__main__":
    main()
