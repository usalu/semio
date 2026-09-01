# 📐️ CAD End to End — Baseline Survey

Start commit: `bb06c41f73f0122fbed315b7487428b976f99921` (2026-08-28)
Survey date: 2026-08-29

## What already exists (much more than expected)

The cad plugin is **not** a stub. It is a large, mostly-authored system:

| Area | State |
| --- | --- |
| Rust crate `semio-s-plugin-cad` | ~20.8k LOC, WASM component (cdylib+rlib), 50+ public types, **zero** `todo!()`/`unimplemented!()` |
| Model definitions (data) | **9** model definitions under `🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/` |
| Typologies | 38 (`🗂️typologies/{name}/🔣️typology.json`) |
| Actions | 137 (`🎬️actions/`) |
| Interactions | 49 (`🎬️interactions/`) |
| Transformations | 10 (`🔀️transformations/`), incl. `spatial.shape → aec.building.energy` |
| Interaction spec runtime | `🎬️interaction-spec/🦀️component.rs` — state-machine AST + expression language, 40+ interaction assets parse & validate |
| Artifact standard `🔖️1` | CadArtifact with 39 required members |
| TS engine | renderer (R3F, 7.6k lines), stately (XState), runtime, actions, artifact, registry, typology, schema/io/mutations |
| Extensions | 4 (spatial-shape, aec-building, aec-building-structure, aec-building-energy) — real stat/property computers |
| Host registration | Registered in playground registry as `s.cad.cad@1/*#editor`; launch configs exist (react @6020, wgpu @6120) |

Model definitions are **data-first**: JSON documents interpreted by the runtime, matching `AGENTS.md` ("model definitions do not ship executable code").

## The actual blocker

**The CAD TypeScript does not compile: 371 `tsc` errors.**

Baseline captured in `🗑️generated/tsc-baseline.txt` via:
`npx tsc -p "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json" --noEmit`

### By file

| Errors | File |
| ---: | --- |
| 89 | `…/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx` |
| 79 | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts` |
| 59 | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts` |
| 39 | `…/✏️editor/⚙️engine/📄️artifact/🟦️component.ts` |
| 37 | `…/🧬️schema/💡️inferences/🟦️component.ts` |
| 23 | `…/✏️editor/⚙️engine/🎬️actions/🟦️component.ts` |
| 27 | `🧩️extensions/*/🟦️component.ts` (all four) |
| 8 | `…/✏️editor/⚙️engine/🎰️stately/🟦️component.ts` |
| 5 | `…/✏️editor/⚙️engine/🏃️runtime/🟦️component.ts` |
| 2 | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts` |
| 2 | `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/📦️index.ts` |
| 1 | `…/✏️editor/⚙️engine/📔️registry/🟦️component.ts` |

### By cause

- **143 × TS2304 "Cannot find name"** — cross-file references left dangling by a file split. e.g. `🗺️spatial` uses `AnchorRecord`, `VertexRecord`, `EdgeRecord`, `WireRecord`, `FaceRecord`, `ShellRecord`, `SolidRecord`, `AnchorRef`, `AnchorAttachment` but imports only `EdgeRef, FaceRef, Model, ShellRef, SolidRef, VertexRef, WireRef` from `📐️geometry`.
- **46 × TS2503 "Cannot find namespace"** — 25 × `THREE` (renderer never imports `three`), 21 × `core`.
  - The `core` failure is structural: `📦️index.ts` does `export const core = {...geometry, ...spatial, ...registry}` — a **value**. Extensions then write `type FaceRef = core.FaceRef`, which needs `core` to be a *namespace*. Fix: add a barrel module re-exporting the three engines and `export * as core` from it.
- 56 × TS2322 / 26 × TS2345 / 23 × TS2339 — genuine type mismatches downstream of the above.
- 16 × TS7006 implicit `any` params, 11 × TS2540 assign-to-readonly, 11 × TS18046 `unknown`.

These files were last touched 2026-08-19 → 2026-08-22, i.e. **pre-existing breakage, not another session's in-flight refactor** (verified with `git log --date=iso`).

## Build & test commands

```
bun nx run @semio-tech/cad-plugin:test          # rust
bun nx run @semio-tech/cad-js:test              # typescript (vitest)
bun nx run @semio-tech/cad-js:generate
bun ./📜️script.ts dev cad                       # react renderer, port 6020
```
Launch configs: `🛠️dev📐️cad⚛️react` (6020), `🛠️dev📐️cad🧊️wgpu🌐️wasm` (6120).

Artifact test: `mutate-cad-1` — 20 typed mutation kinds × 100 handcrafted vectors, validated against an **independent Python oracle** re-implementing `CadSnapshot` from the schema. Two metamorphic laws (payload-kind match, footprint completeness).

## Plan

1. **Wave 1 — make TS compile.** Six parallel slices (kernel, actions-group, artifact, renderer, inferences, index+extensions). No `any`-casting to silence errors.
2. **Wave 2 — make tests green.** Rust `cad-plugin:test`, TS `cad-js:test`, artifact `mutate-cad-1` incl. Python oracle parity.
3. **Wave 3 — runtime proof.** Boot `dev cad`, exercise a create interaction → construct action → object, and a model-space transformation, with console evidence.
