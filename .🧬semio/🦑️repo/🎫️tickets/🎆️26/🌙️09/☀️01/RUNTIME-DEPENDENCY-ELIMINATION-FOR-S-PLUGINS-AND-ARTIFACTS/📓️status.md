# 🧭️ Status — Runtime Dependency Elimination

## Authoritative measurement

```bash
bun ./📜️script.ts verify dependencies literal-external
```

Baseline captured 2026-09-01 at commit `aad3d81959`:

```
rust   77 raw  77 third-party  77 literal-external  66 production-reachable
js     73 raw  73 third-party  70 literal-external  31 production-reachable
python 16 raw  16 third-party  16 literal-external   0 production-reachable
total 168 raw 166 third-party 163 literal-external  97 production-reachable
zero-target=0 literal-external=163 meets-target=false
oracle-conflicts=18
```

`oracle-conflicts` is the sharpest signal for this goal: a crate registered as a **test oracle**
that a **production** manifest also declares — the literal "gltf as oracle but linked at runtime"
failure the goal names. The 18: `js:brepjs`, `js:manifold-3d`, `js:three`, `rust:csv`, `rust:dxf`,
`rust:gif`, `rust:gltf`, `rust:image`, `rust:las`, `rust:lopdf`, `rust:png`, `rust:quick-xml`,
`rust:riff`, `rust:ruststep`, `rust:serde_json`, `rust:tiff`, `rust:tobj`, `rust:zip`.

Ratchet gate (must stay green): `bun ./📜️script.ts verify dependencies`.
Baseline regeneration (`… write-baseline`) is reserved to the coordinating session — it rewrites
the shared `🔒️dependencies.json` the whole team ratchets against.

## Fleet in flight

| slice | scope | agent |
|---|---|---|
| base64 ×7 plugins | `🏭️process 💠️lowpoly 📐️cad 📸️remodel 🖍️draw 🖨️raster 🪐️space` | running |
| png ×2 + image ×2 | `💠️lowpoly 📸️remodel 🎞️animate 🖍️draw` | running |
| blake3 + getrandom + parry3d | `🧩️puzzle 📐️cad` | running |
| parley + swash + kurbo | `📏️layout 🎞️animate` | running |
| wasm-bindgen/js-sys/web-sys ×34 | repo-wide under `✏️s/` | running |
| serde/serde_json PILOT + playbook | framework value/schema + 1 plugin | running |
| JS deps ×11 | `📐️cad 🧩️puzzle 🎞️animate` TypeScript | running |
| dep classifier truth + gltf | `📜️script.ts`, `🧰️framework` mesh-engine | running |
| typst/usvg/vello/wgpu | `🎞️animate` | running |

## Contended files — expected, not a problem

`✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` is owned by three slices at once
(image, kurbo, typst/vello/wgpu); `🧩️puzzle`'s and `📐️cad`'s by two each. Every agent is scoped to
named dependency lines only and instructed to re-read before each edit and never revert a peer.
Independent developers are also live in this tree.

## Deliberately NOT in scope

`**/🧪️oracle/**`, `**/🔬️probes/**`, `**/🏭️generator/**`, `**/🧫️fixtures/**` keep their
third-party crates — that is the oracle contract working as designed. `semio-s-plugin-stdio-test-oracle`
is the reference shape: own workspace, every reference crate `optional = true` behind an `oracles`
feature that no production target enables.
