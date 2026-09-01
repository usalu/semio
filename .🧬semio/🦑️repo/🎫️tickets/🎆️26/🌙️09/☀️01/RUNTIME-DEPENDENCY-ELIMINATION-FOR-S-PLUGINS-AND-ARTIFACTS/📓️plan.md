# 🧹️ Runtime Dependency Elimination For S Plugins And Artifacts

## Definition of done

Every manifest under `✏️s/` that is **production** (i.e. not under `🧪️oracle/`, `🔬️probes/`,
`🏭️generator/`, `🧫️fixtures/`) has **zero third-party entries** in `[dependencies]` /
`"dependencies"`. Third-party libraries survive only:

- in `[dev-dependencies]` / `devDependencies`, or
- behind an `optional = true` feature that no production target enables (the
  `semio-s-plugin-stdio-test-oracle` pattern), or
- inside `🧰️framework/**`, which is the platform layer the plugins are allowed to consume.

Everything an s plugin needs at runtime arrives through a `path`/`workspace` dependency on a
`🧰️framework/🔨️modules/**` crate or a `@semio-tech/*` package.

## Baseline (measured 2026-09-01, commit aad3d81959)

Scanned all 109 `Cargo.toml` + 42 `package.json` under `✏️s/`.

| bucket | entries | manifests |
|---|---|---|
| third-party in `[dependencies]`, total | 188 | 92 |
| — oracle / generator / probe (COMPLIANT by design) | 69 | 41 |
| — **production (VIOLATIONS)** | **119** | **51** |
| JS third-party in `"dependencies"` (production) | 11 pkgs | 3 |

The oracle side is already correct and must not be touched: `🧪️oracle` gates every reference
crate behind an `oracles` feature in its own workspace, and `🏭️generator/🦀️*-engine` crates
exist precisely to produce fixtures with an independent implementation. That is the
"gltf as oracle, never at runtime" contract, already honoured.

### Production violations by crate

| count | crate | replacement surface |
|---|---|---|
| 36 | `serde_json` | `🧰️framework/🔨️modules/🌱️value` + `🎒️pack` |
| 25 | `wasm-bindgen` | `🧰️framework/🔨️modules/🌉️abi` + `🛍️products/💻️os/🔨️modules/🔌️plugin` |
| 23 | `serde` | `🧰️framework/🔨️modules/🧬️schema` + `🌱️value/🔀️serde` |
| 7 | `base64` | `🧰️framework/🔨️modules/🚪️io` |
| 6 | `js-sys` | `🧰️framework/🔨️modules/🌉️abi` |
| 3 | `web-sys` | `🧰️framework/🔨️modules/🖥️platform` |
| 2 | `png` | `🧰️framework/🔨️modules/🖼️assets` |
| 2 | `image` | `🧰️framework/🔨️modules/🖼️assets` |
| 1 each | `getrandom` `kurbo` `usvg` `typst` `typst-svg` `typst-assets` `vello` `wgpu` `parley` `swash` `blake3` `parry3d` `syn` `quote` `proc-macro2` | see waves |

### Production violations by plugin

```
🖍️draw        base64 image js-sys proc-macro2 quote serde serde_json syn wasm-bindgen
🎞️animate     image kurbo typst typst-assets typst-svg usvg vello wgpu
🔱️trinity     js-sys serde serde_json wasm-bindgen web-sys
📐️cad         base64 getrandom serde serde_json wasm-bindgen
📏️layout      js-sys parley swash wasm-bindgen web-sys
🏭️process     base64 serde serde_json wasm-bindgen
🧩️puzzle      blake3 js-sys parry3d web-sys
🪐️space       base64 serde serde_json
📸️remodel     base64 png
🌊️flow        serde serde_json
🗄️stdio       serde serde_json
🌀️procedural  serde_json wasm-bindgen
🌍️gis         js-sys wasm-bindgen
📜️imperative  serde_json wasm-bindgen
🪵️sourcing    serde_json wasm-bindgen
➗️mathematical serde serde_json
📖️playbook    serde serde_json
💠️lowpoly     base64 png
🔋️energy      serde serde_json
🖨️raster      base64
✒️writer      js-sys
🎥️shooting    wasm-bindgen
🏗️fem         serde_json
```

JS (production `dependencies`):
```
📐️cad     react react-dom three @react-three/fiber @react-three/drei brepjs brepjs-opencascade chevrotain xstate
🧩️puzzle  react react-dom three @react-three/fiber @react-three/drei brepjs brepjs-opencascade chevrotain xstate
🎞️animate react react-dom pdfjs-dist reveal.js
```

## Key architectural finding

The framework already carries the whole replacement surface as `🔨️modules`:
`🌱️value` `🎒️pack` `🌉️abi` `🔢️hash` `🚪️io` `🧬️schema` `🔄️machine` `📐️geometry` `🧮️math`
`🔢️number` `🔺️mesh` `🧊️3d` `◻2d` `🖼️assets` `🖱️ui/🖼️render` `🖥️platform` `🗣️dsl`.

`semio-framework-hash` already wraps `blake3`; `🖱️ui/🖼️render` already wraps `parley`/`swash`.
So most of this work is **rewiring s plugins onto existing framework interfaces**, not writing
new engines from scratch. Where a gap exists, the implementation lands in the framework module
(the platform), never in the plugin.

This also satisfies `CLAUDE.md`: *"You MUST use all external libraries behind an interface"* and
*"You MUST NOT export api that directly or indirectly requires an interface/class/type outside of
this codebase."*

## Waves

- **W1 — small utilities** (13 entries): `base64`×7, `blake3`×1, `getrandom`×1, `png`×2, `image`×2.
  Route through `🚪️io`, `🔢️hash`, `🖼️assets`. Fill framework gaps where the module lacks the API.
- **W2 — proc-macro trio** (3 entries): `syn`/`quote`/`proc-macro2` in `🖍️draw`'s statechart macro.
  Verify `proc-macro = true`; these are build-time, so either reclassify or fold into
  `🧰️framework/🔨️modules/🔀️dispatch` macros.
- **W3 — text/layout** (2 entries): `parley`/`swash` in `📏️layout` → `semio-framework-ui-render`,
  which already wraps both.
- **W4 — geometry** (2 entries): `parry3d` (+ transitive `nalgebra`) in `🧩️puzzle`, `kurbo` in
  `🎞️animate`. Both already sit behind thin local adapter structs
  (`Vec3d`/`Point3d`/`Rotation3d`/`Pose3d`/`CollisionShape`), so the adapters re-point at
  `📐️geometry`/`🧮️math`/`◻2d`; gaps = quaternion algebra, isometry, trimesh intersection,
  Bézier arc-length.
- **W5 — serde/serde_json** (59 entries, ~40 manifests): the bulk. Framework provides the
  in-house `Value` + schema derive; plugins stop deriving `Serialize`/`Deserialize` directly.
  Needs the framework surface confirmed/extended before the mass rewire.
- **W6 — wasm glue** (34 entries): `wasm-bindgen`/`js-sys`/`web-sys` → `🌉️abi` +
  `🔌️plugin` component-guest (`wit-bindgen` already lives in the framework).
- **W7 — heavy render/typeset** (6 entries): `typst*`, `usvg`, `vello`, `wgpu` in `🎞️animate`.
  Largest single-plugin lift; these are unconditionally linked today (no feature gate).
- **W8 — JS** (11 packages, 3 manifests): `react`/`react-dom` → `@semio-tech/ui-react`;
  `three`/`@react-three/*` → `@semio-tech/s-3d-js` + `infinite-world-r3f`;
  `xstate` → in-house statechart kernel; `chevrotain` → framework `🗣️dsl` grammar;
  `brepjs*` → spatial kernel; `pdfjs-dist`/`reveal.js` → framework equivalents.
- **W9 — enforcement**: extend `bun ./📜️script.ts dependency` so a production manifest under
  `✏️s/` with any third-party `[dependencies]` entry fails the check; regenerate
  `🔒️dependencies.json`.

## Rules for every wave

1. Test-driven: a language-agnostic test per replaced capability, plus a differential test
   against the third-party crate kept as a **dev-dependency oracle**.
2. No migration shims, no adapters-for-compat, no deprecations. Handcraft the final shape.
3. Never move a third-party crate from a plugin into another plugin — it goes to the framework
   or to a dev-dependency, nowhere else.
4. Concurrent devs are editing these files. Do not stop on unrelated churn; rebase mentally and
   keep going.

## ⚠️ Scope correction — transitive inheritance through the framework

Measured directly from the framework manifests on the plugin link path:

| framework crate | its own third-party `[dependencies]` |
|---|---|
| `semio-framework-schema` | *(none)* |
| `semio-framework` | `gltf` |
| `semio-framework-hash` | `blake3` |
| `semio-framework-pack` | `blake3`, `ureq` |
| `semio-framework-plugin` | `serde`, `serde_json`, `wit-bindgen` |
| `semio-framework-geometry` | `kurbo`, `serde` |
| `semio-framework-replication` | `blake3`, `miniz_oxide`, `serde`, `serde_json`, `wasm-bindgen` |
| `semio-framework-os-kernel` | `base64`, `blake3`, `miniz_oxide`, `serde`, `serde_json`, `tokio`, `ureq`, `zip`, `js-sys`, `wasm-bindgen`, `web-sys`, `futures`, `tokio-tungstenite` |

Every s plugin depends on `semio-framework-plugin`, `semio-framework-os-kernel` and
`semio-framework`. So **a plugin with a spotless manifest still links serde, blake3, base64 and
gltf transitively.** There are two readings of the goal:

1. **Manifest-level** — no third-party in an s plugin's own `[dependencies]`. Reachable with the
   waves above alone.
2. **Link-level** — the compiled `wasm32-wasip2` component links no third-party code at all.
   Requires the framework crates on the plugin path to be first-party too.

The goal says *"dependency free **at runtime**"* and names `gltf`, which lives in
`semio-framework` — not in any s plugin. That is reading **2**, and it is the one being pursued.
The repo's own gate agrees: `verify dependencies literal-external` counts the whole repo, not just
`✏️s/`.

Reading 2 does not contradict `CLAUDE.md`'s *"use only system libraries provided by the
frameworks"* — that sanctions the framework being the single wrapping layer, not the framework
being a permanent third-party carrier. The interface stays; the implementation behind it becomes
first-party.

Consequence: each wave lands its implementation in a framework module, and that module must itself
be first-party. Added wave:

- **W10 — framework link path**: first-party BLAKE3 (digests are persisted and content-addressed —
  byte-exact parity required) and first-party DEFLATE/zlib replacing `miniz_oxide`. `base64` is
  already first-party in `📡️replication/⚙️codec` and only needs re-homing. `gltf` is W9's.
  `tokio`/`ureq`/`futures`/`tokio-tungstenite`/`zip` need a reachability check first — if they are
  native-host-only and never on the `wasm32-wasip2` plugin path, they are out of scope for
  "plugins dep-free at runtime" and should be recorded as such rather than removed.
