# Block plugin — TS side + non-editor Rust surfaces (viewer / io / schema / oracle)

Scope: `✏️s/🔌️plugins/🧱️block` — the 232(+2) `.ts` files (root, `📦️packages/🟦️typescript`, per-subset `🗿️artifacts/**`), plus `👁️viewer`, `🚪️io`, `🧬️schema`, `🔮️oracle` for `◻️2d`, `🧊️3d`, `🖐️5d` under `🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

All paths below are relative to the repo root `/Users/ueli/Documents/semio` unless given absolute.

## 1. What the TS files implement, who imports them, test-runner wiring

**Two kinds of TS files:**

- **Schema mirrors (majority, ~190 files)**: plain `interface`/`type`/`const` declarations that mirror Rust structs 1:1 — artifact document shape (`🧬️schema/🟦️.ts`), snapshot/diff/mutation payloads (one file per mutation under `🧬️schema/🧬️mutations/<slug>/🟦️.ts`), and the shared cross-dimension identity/attribute/author/compatibility/representation/camera types in the plugin root `✏️s/🔌️plugins/🧱️block/🟦️.ts:7-65`. No runtime logic anywhere in these — they are compile-time-only "typed twins" of the Rust types, explicitly documented as such (e.g. `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🟦️.ts:1-3`).
- **Viewer typed twins**: read-only view-model interfaces mirroring each subset's Rust `render()` boundary — e.g. `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📋️board/🟦️.ts:5-27` and `.../🧊️3d/.../🌐️world/🟦️.ts:7-13`.
- **IO barrels**: empty stubs, `export {};`, e.g. `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️.ts:1-2` ("IO facet barrel — WASM facades land in W7").

**Importers.** `📦️packages/🟦️typescript/🟦️.ts` (the `@semio-tech/block-js` package entry point) re-exports `schema`, `snapshot`, `diff`, `mutations` and `io` for all three subsets (lines 2-34), but **never re-exports `viewer` or `editor`** — those TS files have zero importers anywhere in the repo. Confirmed by repo-wide grep: no file outside `✏️s/🔌️plugins/🧱️block` imports any `🔌️plugins/🧱️block` path, and `grep -rl "block-js"` only turns up the package's own `project.json`/`package.json` and taxonomy registry entries for the unrelated JCO WASM-component outputs (`dev-plugin-component-block-js` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:11917-11919`) — not a TS import. **The block-js package is not consumed by any app, window, or mode in TypeScript.**

**Test-runner wiring.** The only nx target for the TS package is `@semio-tech/block-js:test` (`📦️packages/🟦️typescript/📋️project.json`), which runs `bun ./📜️script.ts test`. That script (`📦️packages/🟦️typescript/📜️script.ts`) does **not** exercise any of the 232 schema/io/viewer files at all — it validates an unrelated "publication authority" law (Ajv-validates a fixture, then regex-greps a Rust source string for anchor strings) that governs plugin-lane wiring, not block's own domain schema. Root `📜️script.ts` (35k lines) has no block-specific bun test glob; the actual block2d/3d/5d mutation *behaviour* is tested only via the repo-wide oracle/parity runner `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` (`discover|contract|oracle|subject|parity|run` subcommands) driving Rust cucumber tests + a Python second implementation — no TS is involved in that path either.

**`bun test` run (executed).** Ran `bun ./📜️script.ts test` in `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript`. It failed immediately (cheap, <1s):
```
ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🔣️publication-authority.json'
```
The script expects a flat file `✏️s/🔌️plugins/🧱️block/🔣️publication-authority.json` (`📦️packages/🟦️typescript/📜️script.ts:24`), but the actual fixture lives at `✏️s/🔌️plugins/🧱️block/🧪️publication-authority/🔣️.json` (and `🧬️.schema.json` next to it) — a directory, not a flat filename. **The block-js package's only test target is currently broken** (path mismatch, not a logic bug). See item 6.

## 2. `🚪️io` per subset: serializers/deserializers and Rust/TS mirror status

Each subset (`◻️2d`, `🧊️3d`, `🖐️5d`) has the identical format set under `🚪️io/📤️export/🧵️serializers/🗿️artifacts/<fmt>/<ver>/✳️any/` and `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/<fmt>/<ver>/✳️any/`: `🎒️zip/🔖️2.0`, `📷️png/🔖️1.2`, `🔣️json/🔖️rfc8259`, `🔤️txt/🔖️utf-8`, `🔺️stl/🔖️ascii`, `🧊️obj/🔖️3.0` — 12 leaf directories × 3 subsets, each with both a `🟦️.ts` and a `🦀️.rs` file (36 pairs total). No pack/spr/op/dsl-named leaf formats exist in this plugin (the DSL `.semio` snapshot/mutation grammar lives instead under `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/📝️text/📖️.grammar.semio`, not under `🚪️io`).

**Rust/TS mirror status — NOT symmetric:**
- Every one of the 36 leaf `🟦️.ts` files is a 1-line stub, `export {};` (verified by `wc -l` on all of them — every file is 1 line except the two `🚪️io/🟦️.ts` root barrels which are 2 lines).
- The corresponding `🦀️.rs` files are real (15-20 lines), with actual `register()`/`serialize_bytes()`/`deserialize_bytes()` functions.
- Only **JSON** is genuinely implemented on the Rust side, e.g. `.../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs` parses real bytes through `dsl::FromValue`.
- **STL/OBJ/ZIP/PNG** import stubs ignore the bytes and return `Ok(Block2dSnapshot::default())` (e.g. `.../🔺️stl/🔖️ascii/✳️any/🦀️.rs`), i.e. compile but are not functionally implemented.
- **TXT** is the one explicitly-documented gap on both import and export sides: `.../🔤️txt/🔖️utf-8/✳️any/🦀️.rs` (both import and export) carries the doc comment: *"Pre-migration content here referenced `crate::artifacts::json`/`crate::artifacts::txt`, types that don't exist in this crate (dead code, never mounted by the old glue, never compiled) — likely a copy-paste of stdio's own internal json↔txt bridge into the wrong plugin's txt target folder. Left as an honest stub... pending a real txt import/export implementation."* Both return `Err("txt import/export not yet implemented")` — identical wording across all three subsets.
- The registry (`🚪️io/🦀️.rs::io_registry::entries()`, e.g. `◻️2d/.../🚪️io/🦀️.rs:199-212`) wires ZIP/PNG/JSON/STL/OBJ export composer entries but **omits TXT** — consistent with TXT not having a real `serialize_bytes` function.
- Since the `.ts` files are inert exports and the `.rs` files are the sole functional side, "mirror" here is asymmetric by design intent (WASM facade "lands in W7" — a future ticket), not currently broken, but it means the CLAUDE.md multi-implementation rule (schema-first, same output validated by 2+ implementations) is **not yet satisfied for TS** on the IO layer at all.

**2d/3d/5d io registration itself is inconsistent.** `◻️2d/.../🚪️io/🦀️.rs:1-8` carries a header marking the whole file as an "OLD channel... still live at runtime but no longer wired into `crate::artifacts::block2d::artifact()`... a documented, real gap", whereas `🧊️3d/.../🚪️io/🦀️.rs:1-2` and `🖐️5d/.../🚪️io/🦀️.rs:1-2` instead say "registration now flows through `🎹️composer::register`... not per-leaf `register()`" — i.e. block2d's IO composition doc is stale/inconsistent with block3d/block5d's (both describe a different wiring state for structurally identical code).

**Rename audit (📄️txt → 🔤️txt, 🟪️stl → 🔺️stl).** `git status` shows all 24 affected files (both `.ts` and `.rs`, import+export, all 3 subsets) staged as renames (`R`). Verified: `grep -rn "📄️txt\|🟪️stl"` across the whole plugin returns **zero** hits — no stale references remain. All `#[path = "..."]` attributes in the aggregator `📦️packages/🦀️rust/🦀️.rs` (e.g. lines 470, 506, 551, 587, 1314, 1350, 1395, 1431, 2106, 2142, 2187, 2223) already point at the new `🔤️txt`/`🔺️stl` segments, and every one of those paths was confirmed to resolve on disk.

## 3. `🔮️oracle/🔣️.json` per subset

Structure is identical across subsets (`◻️2d/.../🔮️oracle/🔣️.json`, `🧊️3d/...`, `🖐️5d/...`): a `schemaVersion: 2` document with:
- `oracles[]` — one `verified-native-second-implementation` entry per subset (`block-2d-python-independent`, `block-3d-python-independent`, `block-5d-python-independent`), each pointing at a from-scratch Python re-implementation at `../../../../../🧪️tests/mutate-block-<dim>-1/🐍️.py`, written directly from the schema/mutation JSON + `.grammar.semio` files, declaring `noThirdPartySurvey` (KiCad/Modelica/IFC considered and declined — no library models a kind-level handle-compatibility relation), `fixtureCoverage.vectors: 26/37/41`.
- `mutationCatalogs[]` — the full per-mutation vector list (mutation id, source dir, scenario id/dir) — 26 for `◻️2d`, 37 for `🧊️3d`, 41 for `🖐️5d`.
- `mutationManifests[]` — per-mutation `oracleRequirements` demanding `qualifyingKind: verified-native-second-implementation` for capability `block-<dim>-1-mutate`.

**Consumer.** Verified counts match exactly: mutation-directory count under each subset's `🧬️schema/🧬️mutations/` (26/37/41) equals the oracle's `mutationCatalogs[].vectors` length. The actual consumer is the Rust cucumber test crate at `🧪️tests/🧩️mutate-block-<dim>-1/{🥒️.feature, 🦀️.rs, 🐍️.py, 🧫️fixtures}` (present for `◻️2d`; `🧊️3d`/`🖐️5d` have the `.feature`/`.rs`/`.py` but no separately-listed `🧫️fixtures` dir), driven through the repo-wide `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` oracle/parity harness (`discover|contract|oracle|subject|parity|run|report` subcommands) — **not** the root `📜️script.ts`, and **not** any TS test.

## 4. `👁️viewer` per subset — what it renders, SurfaceKind, wiring

- **`◻️2d`**: single window, `📋️board`. Rust `render()` (`.../👁️viewer/🎭️modes/👁️view/🪟️windows/📋️board/🦀️.rs:36-45`) builds a `UiNode` text tree (node-kind label, handle-kind catalog with color, per-handle angle/radius) from `Block2dSnapshot` — `SurfaceKind::Board2d`. TS twin mirrors the same fields (`Block2dViewHandleKind`, `Block2dViewHandle`) with `surfaceId: "block2d.view.board2d/board"`.
- **`🧊️3d`**: single window, `🌐️world`, `windowKindId: "framework.window.mesh"` (the framework's shared `MeshWindowKit`, not app-minted) — a 3D mesh/instance/camera scene. TS twin (`.../👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🟦️.ts:7-13`) declares `surfaceId: "block3d.view.world"` in addition to windowKindId/bodyKey.
- **`🖐️5d`**: same `framework.window.mesh` pattern, but its TS twin (`.../🌐️world/🟦️.ts:9-13`) declares only `windowKindId`/`bodyKey` — **no `surfaceId` field/const**, unlike the 3d twin. Minor 3d/5d asymmetry in the typed twin, not necessarily a bug (5d's own doc comment says it uses the "frozen `MeshWindowKit`... contract §2.6", so a surface id may be framework-owned rather than block-owned here).

**Wired or dead?** Rust-side, each subset's own root `🦀️.rs` wires `viewer: viewer_surface::<viewer::Block<Dim>Viewer, crate::BlockApps>(viewer::create_block<dim>_viewer())` (`◻️2d/.../🦀️.rs:66`, `🧊️3d/.../🦀️.rs:53`, `🖐️5d/.../🦀️.rs:53`) — genuinely mounted, not dead, and each `create_block<dim>_viewer` has a Rust unit test. TS-side, the viewer typed twins are **not re-exported by the block-js package barrel** (`📦️packages/🟦️typescript/🟦️.ts` exports schema/snapshot/diff/mutations/io only) and have **zero importers anywhere in the repo** — they exist purely as documentation-grade "typed twin" declarations, cross-referenced only in a doc comment from the editor (`🧊️3d/.../✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🟦️.ts:5`), never actually imported.

## 5. `🧬️schema` per subset — document types, example ids, asset parity

**Document schema.** Root artifact type per subset (e.g. `Block2dArtifact` at `◻️2d/.../🧬️schema/🟦️.ts:6-27`) composes the plugin-shared cross-dimension types from `✏️s/🔌️plugins/🧱️block/🟦️.ts` (`BlockKindIdentity`, `BlockAttribute`, `BlockAuthor`, `BlockCompatibilityRule`, `BlockCamera2d/3d`, `BlockMeta`) with per-dimension nouns from the artifact-level `🗿️artifacts/<dim>/🟦️.ts` (e.g. `Block2dPresentation`, `Block2dHandleKind`, `Block2dHandleTemplate` — note the doc comment "mirrors `Puzzle2dNode`'s shape fields", an intentional documented lineage from the puzzle plugin, not a copy-paste accident).

**Field-name spot check (TS ↔ Rust ↔ JSON schema), `add-attribute`:**
- TS: `AddAttribute { attribute: BlockAttribute }` (`🧬️mutations/🧩️add-attribute/🟦️.ts`)
- Rust: `pub struct AddAttribute { pub attribute: BlockAttribute }` (`🧬️mutations/🧩️add-attribute/🦀️.rs:16-19`)
- `.schema.json`: `required: ["attribute"]`, `attribute.required: ["key","value"]`, optional `definition` (`🧬️mutations/🧩️add-attribute/🧬️.schema.json`)
All three agree exactly.

**Mutation count / schema.json coverage.** `◻️2d` has 26 mutation directories and 26 `🧬️.schema.json` files (1:1); `🧊️3d` has 37; `🖐️5d` has 41 — all matching their respective oracle `mutationCatalogs[].vectors` counts exactly (cross-checked in item 3).

**DSL grammar assets.** `.grammar.semio` files exist per subset for `📸️snapshot/📝️text`, `🔺️diff/📝️text`, `🧬️mutations/📝️text`, and `💡️inferences/📝️text` — present for all three subsets, giving the DSL text-encoding contract the schema oracles cite as their specification source.

**Example-id list.** Not schema-exported as a TS union; it lives as a Rust `OnceLock<Vec<ExampleSource>>` per subset (e.g. `◻️2d/.../🦀️.rs:34-35`: `art_2d_hexagonal_cut_concrete_forest_left`/`_right`). Matches the two `📚️examples/*` directories present for `◻️2d` (`🎬️hexagonal-cut-concrete-forest-left`, `➡️hexagonal-cut-concrete-forest-right`) and the two each for `🧊️3d`/`🖐️5d` (`🎬️hexagonal-cut-concrete-forest-left`, `🏢️nakagin-capsule`). Each example dir's `🟦️.ts` is just `{id, label:{en,de}, icon}` metadata, not a document schema.

## 6. Defects found

1. **Broken TS test target (confirmed by running it).** `bun ./📜️script.ts test` in `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript` fails with `ENOENT` — it looks for `✏️s/🔌️plugins/🧱️block/🔣️publication-authority.json` (a flat file at the plugin root), but the actual fixture is at `✏️s/🔌️plugins/🧱️block/🧪️publication-authority/🔣️.json` (and `🧬️.schema.json`). `📦️packages/🟦️typescript/📜️script.ts:24` needs its path updated. This is the *only* nx test target for the entire TS package, so `@semio-tech/block-js:test` is currently 100% broken.

2. **`package.json` is an uncustomized copy of the CAD plugin's.** `📦️packages/🟦️typescript/package.json`: `description` reads *"📐️ CAD plugin TS: spatial factory runtime/model graph (core), R3F renderer, brepjs kernel..."* (verbatim CAD-plugin wording, diffing only by one clause vs. `📐️cad/📦️packages/🟦️typescript/package.json`'s description); `scripts.test`/`scripts.generate`/`scripts.fixture` all say `bun nx run @semio-tech/cad-js:test` (should be `block-js`); `dependencies` lists `@semio-tech/cad-js-module-spatial-shape`, `@semio-tech/cad-js-module-aec-building(-energy|-structure)` — none of which relate to Block's domain (2D board/3D-5D vortex-mesh node kinds) and are near-certainly a wholesale copy-paste from `📐️cad`'s package.json with only the `name` field changed. None of this affects `bun test` today because nx's own `project.json` target (which actually runs) bypasses `package.json`'s `scripts.test`, but it is dead-weight/misleading metadata and an unused, incorrect dependency graph.

3. **Copy-paste "wrong plugin" leftover, txt io (both directions, all 3 subsets).** Confirmed via the Rust files' own doc comments: `🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs` for `◻️2d`/`🧊️3d`/`🖐️5d` all self-report that their pre-migration content referenced nonexistent `crate::artifacts::json`/`crate::artifacts::txt` types — "likely a copy-paste of stdio's own internal json↔txt bridge into the wrong plugin's txt target folder" — dead code that never compiled, now replaced with an honest `Err(...)` stub. Not exploitable (never wired into the export registry), but confirms a real historical copy/paste defect, self-documented rather than hidden.

4. **Stray function in an export/serializer file.** `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs` (all 3 subsets) defines a `deserialize_bytes` function inside the *export/serializers* tree — a leftover from whatever was copy-pasted in, since export leaves are only supposed to expose `serialize`/`serialize_bytes`. Dead code (never called; `io_registry::entries()` doesn't reference it and TXT isn't in the export registry at all).

5. **All 36 IO leaf `.ts` files (12 formats × 3 subsets) are inert `export {};` stubs** — see item 2. Not a bug per se (documented "WASM facades land in W7"), but means none of the CLAUDE.md-mandated "second implementation" parity exists yet on the TypeScript side for any IO format, including JSON (which *is* implemented in Rust).

6. **STL/OBJ/ZIP/PNG import deserializers are non-functional stubs that compile silently.** They accept `bytes: &[u8]` and unconditionally return `Ok(Snapshot::default())`, discarding the input (e.g. `.../🔺️stl/🔖️ascii/✳️any/🦀️.rs`). Unlike the TXT stub (which fails loudly with `Err`), these succeed with a meaningless empty snapshot — a caller round-tripping through these formats gets silent data loss instead of an error. Worth the same "not yet implemented" `Err` treatment TXT already got, for consistency and to avoid masking the gap.

7. **`◻️2d`'s `🚪️io/🦀️.rs` doc header is stale relative to `🧊️3d`/`🖐️5d`'s.** 2d's says the file is an "OLD channel... no longer wired into `artifact()`... a documented, real gap"; 3d/5d's (structurally identical code) instead say "registration now flows through `🎹️composer::register`". Either 2d's registration really is stale (real functional gap) or the comment simply wasn't updated when 2d was migrated alongside 3d/5d — worth resolving one way or the other.

8. **Viewer TS typed twins are orphaned.** Not re-exported by the block-js package barrel and not imported anywhere — see item 4. Likely intentional (compile-time documentation only) but worth confirming that's the intended end state rather than an oversight, since the `📦️packages/🟦️typescript/🟦️.ts` barrel comment ("heavy plugin facet WASM facades") suggests viewer/editor facades are simply not built yet, same "W7" marker as IO.

No `TODO`/`unimplemented!()` literals were found in the scoped viewer/io/schema/oracle Rust or TS files themselves (the "not yet implemented" gaps are expressed as `Err("... not yet implemented")` strings, already covered above).
