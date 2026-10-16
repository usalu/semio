# 🔍️ Audit: Refused Crates, Plugin Source Gate, Non-Root Workspaces

Read-only audit. No source edited, no git writes, no cargo builds, no `🗑️generated` folder deleted. All probes ran in memory or on scratch copies under `🗑️generated/audit/`. Machine outputs: `🗑️generated/plugin-census.csv`, `🗑️generated/workspace-census.csv`, scripts in `🗑️generated/audit/` and `🗑️generated/{census,plugin-census,workspace-census}.ts`.

## Headline

- **`semio-framework-os-renderer-wgpu` is refused for a true generated input.** `include!(concat!(env!("OUT_DIR"), "/🧩️icons.rs"))` in the IconRenderHost wgpu target. The counter throws on every `OUT_DIR` base regardless of owner, so no owner can clear it. With that one include overlaid in memory, the closure is 145 files / 5,859,758 bytes / 102,284 lines (about 25x the line limit).
- **`semio-framework-os-kernel-db` is refused by a counter bug, not by generated code.** `selectRustSourceV1` removes `#[cfg(...)]` items at the last position of a block (struct-literal field or tail expression with no `,` or `;`) and scans past the enclosing `}`. This unbalances the storage root file and fails with `Unsupported Rust compile module body`. Patching four sites in memory removes the refusal, but the crate is still oversized: 29 files / 2,614,229 bytes / 52,301 lines.
- **Plugins are gated when built through the WASM/component path.** Plugin units have `workspace` provenance and the admission path applies `min(policy, declared)`. 79 of 1,357 plugin manifests exceed the policy; none declare a boundary; 0 are refused by the counter.
- **Hub:** 2 oversized members (`semio-hub`, `semio-hub-space`), and all 92 hub members are outside the root census.

## 1. Refused crates

### 1a. `semio-framework-os-renderer-wgpu`

- Manifest: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml`, lib `📚️library/🦀️.rs`.
- Counter message (census run, `ONLY=…`): `Generated Rust input requires compiler ownership: …/🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs`.
- Triggering construct: `IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs:16`, `include!(concat!(env!("OUT_DIR"), "/🧩️icons.rs"));`.
- Producer: `🧊️wgpu/🏗️builder/🦀️.rs` (the `build.rs` include chain). It copies SVGs into `OUT_DIR/🔣️icons/` and writes `OUT_DIR/🧩️icons.rs` with `include_str!` of those copies. The generated file is not checked in.

**How the counter decides "compiler owns them"** (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`, lines 15–31):

- `RustSourceOwnerV1` carries `manifestDirectory` and a `generated(file, output)` observer. The observer only receives token output from proc-macro `quote!` (syntax module, `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts` lines 211, 270).
- Line 31: `if (reference.expansion || reference.base === "generated" || reference.base === "manifest" && !owner) throw`.
- `base === "generated"` is what `env!("OUT_DIR")` produces (syntax module line 282). It throws unconditionally. `expansion` (local `macro_rules!` expansion) also throws unconditionally. No owner object can lift either refusal.
- `base === "manifest"` (`CARGO_MANIFEST_DIR`) is admitted when an owner is supplied. The census and the admission path both pass `{manifestDirectory, generated: () => undefined}`.

**What other crates do:** in the root census (224 nodes), the plugin census (1,357 manifests) and the workspace census, this wgpu crate and `os-kernel-db` are the only ones that reach an `OUT_DIR` include or expansion. Everything else uses manifest-relative `include_str!`/`include_bytes!` (counted as `embedded`) and `#[path]` mounts.

**Measured size behind the refusal** (`🗑️generated/audit/probe-wgpu.ts`, in-memory overlay removing only the `OUT_DIR` include line): 145 files, 5,859,758 bytes, 102,284 active lines, 10 embedded. The census row is "refused" and hides this.

**What would make it admissible:**
1. Replace the build-time codegen with a schema-first, checked-in output. Commit the generated `🧩️icons.rs` and the SVG copies under the crate (or a shared `🔨️modules/🖼️assets` mount) and reference them with manifest-relative paths, which the counter admits. Add a drift check in the existing builder instead of writing to `OUT_DIR`.
2. Or move the atlas into a compiler-owned embedded asset (`include_bytes!` of a committed atlas file).
3. Either way the crate remains oversized: 145 files and 102k lines against 32 files and 4,000 lines. Its `IconRenderHost`, `Scenes`, `NodeGraph` and shell sources must be carved as well. Option 1 only removes the refusal.

### 1b. `semio-framework-os-kernel-db`

- Manifest: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml`, lib `🦀️.rs` (facade of `#[path]` mounts into `🛢️db/`).
- Counter message: `Unsupported Rust compile module body`. Thrown at syntax module line 535 when a `mod x {` has no matching `}` in the cfg-selected text.
- Per-file scan (`🗑️generated/audit/diag-tree.ts`, all 69 files under `🛢️db/`): exactly one file is unbalanced: `🛢️db/🗄️storage/🦀️.rs` (8,975 lines). Nothing in the file is generated and no `OUT_DIR` or `include!` is involved.

**Root cause (counter bug):**

- `selectRustSourceV1` (source module, lines 95–140) removes a cfg-false item by calling `rustFindTopLevel(tokens, pairs, attrs.next, tokens.length, {";", ",", "{"})` (syntax module, line 640).
- `rustFindTopLevel` does not stop at an unpaired closer (`}`, `)`, `]`) of the enclosing scope. When the attributed item is the last item in its block (no trailing `,` or `;`), the boundary is the first `;` after the enclosing statement. The removal then deletes the enclosing `}`.
- Four sites in `🗄️storage/🦀️.rs` (found by `🗑️generated/audit/cfg-overrun.ts`): lines 130, 235 and 258 (`#[cfg(test)] owner: …` as the last field of a struct literal, followed by `};`), and 7844–7846 (`#[cfg(not(target_vendor = "apple"))]` on the tail expression `file.sync_all().map_err(io_err)` before `}`).

**Verification in memory** (`🗑️generated/audit/patch-storage.ts`, `probe-closure.ts`):

- Original: `Unsupported Rust compile module body`.
- With the four sites patched (trailing comma after the three struct fields; tail expression wrapped in a block): closure OK, 29 files, 2,614,229 bytes, 52,301 active lines, 0 embedded. The crate is still oversized (bytes about 10x, lines about 13x).

**What would make it admissible:**

- Compiler side: stop `rustFindTopLevel` at an unpaired closer inside `selectRustSourceV1`. Not run here, because that is a source edit.
- Source side: the four-site rewrite above, which is verified in memory.
- Either way the crate needs a carve (29 files, 52k lines). Its `os-kernel` dependency is itself oversized in the root census (180 files, 54,848 lines, tier T2).

## 2. Plugin crates: is the source gate applied?

**Code path (read, not run):**

- Each plugin declares its own `[workspace]` (e.g. `✏️s/🔌️plugins/🌀️procedural/🫀️core/Cargo.toml`). `✏️s/Cargo.toml` excludes the 224 `🔌️plugins` entries, so each plugin is a standalone workspace root.
- `admitCargoSourceBoundariesV1` (`📥️source/🎟️admission/🟦️.ts` line 19) skips only units whose provenance is not `workspace`. A plugin manifest lies inside the repo root, so its units are `workspace` and gated. Limits are `min(policy, unit.boundary)`.
- The WASM build (`🕸️wasm-build/🟦️.ts` line 36) calls `admitCargoSelectionV1` with `workspaceRoot: options.policy.workspaceRoot` and `cwd` = the plugin component dir. The component pass (`🧩️component-pass/🟦️.ts` line 342) does the same.
- The launcher/catalog caller that supplies the policy for plugin builds was not located under `🏃️process` (`/usr/bin/grep` for `🔌️plugins` there returned nothing). The `wasm32-unknown-unknown` target is what the TS WASM build paths pass; `wasip2` appears in the native admission Rust file (`🎟️admission/🦀️.rs`) and in fixtures, not in the TS launcher path I read. So I could confirm the gate applies to plugin units when that path runs, but not that the live plugin catalog build runs it. This needs a runtime check, not run here (no builds allowed).

**Census** (`🗑️generated/audit` → `🗑️generated/plugin-census.ts`, `ONLY` not used; output `🗑️generated/plugin-census.csv`):

- Manifests scanned: 1,357. Targets: lib, bin and build script per manifest, through the canonical counter with the census cfg (host triple, `debug_assertions`, default features). 
- Over limit: 79 (binding: files 70, bytes 7, lines 2). Refused by the counter: 0. Declaring `rust-source-boundary`: 0. Manifests with no compile target: 133 (workspace-only roots).

Top 40 by worst ratio (policy limits 32 files / 262,144 bytes / 4,000 lines; "emb" = embedded files in the worst target):

| # | Crate | Manifest (under `✏️s/🔌️plugins/`) | Files | Bytes | Active lines | Ratio | Binding | Emb |
|---|---|---|---:|---:|---:|---:|---|---:|
| 1 | semio-s-artifact-bim-model | 🏙️bim/🗿️artifacts/🏢️model | 1288 | 6,280,428 | 88,458 | 40.25 | files | 41 |
| 2 | semio-s-artifact-stdio-pdf | 🗄️stdio/🗿️artifacts/📖️pdf | 721 | 10,539,688 | 52,628 | 40.21 | bytes | 62 |
| 3 | semio-s-artifact-puzzle-5d | 🧩️puzzle/🗿️artifacts/🖐️5d | 323 | 5,900,521 | 26,646 | 22.51 | bytes | 51 |
| 4 | semio-s-artifact-stdio-gif | 🗄️stdio/🗿️artifacts/🎞️gif | 158 | 5,047,093 | 9,264 | 19.25 | bytes | 61 |
| 5 | semio-s-artifact-stdio-gltf-inferences | 🗄️stdio/🗿️artifacts/🧊️gltf/…/💡️inferences | 362 | 522,511 | 6,626 | 11.31 | files | 270 |
| 6 | semio-s-artifact-architect-program | 🏛️architect/🗿️artifacts/🏛️program | 176 | 2,828,975 | 23,387 | 10.79 | bytes | 40 |
| 7 | semio-s-artifact-puzzle-2d | 🧩️puzzle/🗿️artifacts/◻️2d | 343 | 2,681,789 | 25,838 | 10.72 | files | 75 |
| 8 | semio-s-artifact-note-note | 🗒️note/🗿️artifacts/🗒️note | 248 | 819,779 | 12,501 | 7.75 | files | 36 |
| 9 | semio-s-artifact-shooting-shooting | 🎥️shooting/🗿️artifacts/🎥️shooting | 219 | 662,055 | 9,695 | 6.84 | files | 41 |
| 10 | semio-s-artifact-forms-forms | 📋️forms/🗿️artifacts/📋️forms | 195 | 717,250 | 10,410 | 6.09 | files | 38 |
| 11 | semio-s-artifact-gis-gismap | 🌍️gis/🗿️artifacts/🗺️gismap | 188 | 1,023,225 | 11,642 | 5.88 | files | 43 |
| 12 | semio-s-artifact-trinity-rewriting | 🔱️trinity/🗿️artifacts/♻️rewriting | 161 | 499,048 | 7,068 | 5.03 | files | 31 |
| 13 | semio-s-artifact-writer-writer | ✒️writer/🗿️artifacts/✒️writer | 155 | 428,042 | 7,483 | 4.84 | files | 33 |
| 14 | semio-s-artifact-animate-presentation | 🎞️animate/🗿️artifacts/🎬️presentation | 152 | 734,115 | 14,018 | 4.75 | files | 39 |
| 15 | semio-s-artifact-stdio-semio | 🗄️stdio/🗿️artifacts/🧿️semio | 150 | 513,940 | 4,880 | 4.69 | files | 62 |
| 16 | semio-s-artifact-stdio-svg | 🗄️stdio/🗿️artifacts/🎨️svg | 145 | 644,677 | 9,703 | 4.53 | files | 31 |
| 17 | semio-s-artifact-mathematical-equation | ➗️mathematical/🗿️artifacts/➗️equation | 143 | 865,774 | 14,763 | 4.47 | files | 31 |
| 18 | semio-s-artifact-wfc-3d | 🀄️wfc/🗿️artifacts/🧊️3d | 139 | 537,663 | 8,800 | 4.34 | files | 29 |
| 19 | semio-s-artifact-wfc-2d | 🀄️wfc/🗿️artifacts/◻️2d | 139 | 573,682 | 8,719 | 4.34 | files | 28 |
| 20 | semio-s-artifact-trinity-jack | 🔱️trinity/🗿️artifacts/🔌️jack | 132 | 716,017 | 12,082 | 4.13 | files | 30 |
| 21 | semio-s-artifact-flow-flow | 🌊️flow/🗿️artifacts/🌊️flow | 131 | 530,302 | 8,480 | 4.09 | files | 29 |
| 22 | semio-s-artifact-stdio-gltf-mutations | 🗄️stdio/🗿️artifacts/🧊️gltf/…/🧬️mutations | 131 | 732,261 | 12,910 | 4.09 | files | 2 |
| 23 | semio-s-artifact-stdio-semio-graph | 🗄️stdio/🗿️artifacts/🧿️semio/…/🕸️graph | 127 | 579,170 | 8,750 | 3.97 | files | 30 |
| 24 | semio-s-artifact-vcs-vcs | 🌿️vcs/🗿️artifacts/🌿️vcs | 123 | 315,978 | 4,770 | 3.84 | files | 40 |
| 25 | semio-s-artifact-sourcing-curation | 🪵️sourcing/🗿️artifacts/🗂️curation | 123 | 486,506 | 6,907 | 3.84 | files | 40 |
| 26 | semio-s-artifact-stdio-xml | 🗄️stdio/🗿️artifacts/📰️xml | 122 | 634,759 | 9,359 | 3.81 | files | 33 |
| 27 | semio-s-artifact-stdio-semio-drawing | 🗄️stdio/🗿️artifacts/🧿️semio/…/🖊️drawing | 120 | 516,806 | 6,757 | 3.75 | files | 29 |
| 28 | semio-s-artifact-stdio-semio-mesh | 🗄️stdio/🗿️artifacts/🧿️semio/…/🔺️mesh | 118 | 448,535 | 6,306 | 3.69 | files | 30 |
| 29 | semio-s-artifact-stdio-json | 🗄️stdio/🗿️artifacts/🧾️json | 116 | 563,173 | 8,135 | 3.63 | files | 32 |
| 30 | semio-s-artifact-wfc-bitmap | 🀄️wfc/🗿️artifacts/🖼️bitmap | 115 | 527,670 | 8,689 | 3.59 | files | 28 |
| 31 | semio-s-artifact-stdio-semio-document | 🗄️stdio/🗿️artifacts/🧿️semio/…/📑️document | 114 | 459,717 | 6,703 | 3.56 | files | 30 |
| 32 | semio-s-artifact-stdio-tiff | 🗄️stdio/🗿️artifacts/🖼️tiff | 114 | 484,672 | 5,804 | 3.56 | files | 32 |
| 33 | semio-s-artifact-dag-dag | 🕸️dag/🗿️artifacts/🕸️dag | 112 | 316,526 | 5,041 | 3.50 | files | 38 |
| 34 | semio-s-artifact-space-home | 🪐️space/🗿️artifacts/🏠️home | 112 | 368,287 | 5,471 | 3.50 | files | 39 |
| 35 | semio-s-artifact-stdio-semio-kit | 🗄️stdio/🗿️artifacts/🧿️semio/…/🧰️kit | 111 | 380,214 | 4,892 | 3.47 | files | 30 |
| 36 | semio-s-artifact-wfc-grid2d | 🀄️wfc/🗿️artifacts/🔲️grid2d | 111 | 518,771 | 8,443 | 3.47 | files | 26 |
| 37 | semio-s-artifact-wfc-grid3d | 🀄️wfc/🗿️artifacts/🧱️grid3d | 110 | 499,849 | 7,841 | 3.44 | files | 28 |
| 38 | semio-s-artifact-stdio-semio-brep | 🗄️stdio/🗿️artifacts/🧿️semio/…/🧊️brep | 109 | 549,460 | 7,143 | 3.41 | files | 30 |
| 39 | semio-s-artifact-stdio-semio-cad | 🗄️stdio/🗿️artifacts/🧿️semio/…/📐️cad | 108 | 353,233 | 4,786 | 3.38 | files | 30 |
| 40 | semio-s-artifact-imperative-procedure | 📜️imperative/🗿️artifacts/📜️procedure | 108 | 319,542 | 5,080 | 3.38 | files | 37 |

Full rows (all 1,357) are in `🗑️generated/plugin-census.csv`.

## 3. Oversized crates in non-root workspaces

Census: `🗑️generated/workspace-census.ts` → `🗑️generated/workspace-census.csv`. Members of each explicit workspace root, same counter and policy.

| Workspace | Members | Not in root census | Over limit | Refused |
|---|---:|---:|---:|---:|
| `🌎️hub` (standalone) | 92 | 92 | 2 | 0 |
| `🧰️framework` | 209 | 90 | 11 | 0 |
| `✏️s` (plugins excluded) | 20 | 20 | 0 | 0 |
| `🎓️teaching` | 1 | 1 | 1 | 0 |

Oversized (ratio, binding):

- `🌎️hub`: `semio-hub` (`🌎️hub/📦️packages/🦀️rust`): 62 files, 2,088,285 bytes, 33,835 lines, 8.46, lines. Not in root census. `semio-hub-space` (`🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust`): 66 files, 345,590 bytes, 4,976 lines, 2.06, files. Not in root census.
- `🧰️framework` members also in the root census: `semio-framework-replication` (4.12 lines), `semio-framework` (3.79 lines), `semio-framework-ui-contract` (3.74 lines), `semio-framework-pack` (3.07 bytes), `semio-framework-ui-scene` (2.55 lines), `semio-framework-ui-render` (1.91 lines), `semio-framework-canvas` (1.66 lines), `semio-framework-surface` (1.50 lines), `semio-framework-ui-runtime` (1.28 lines). These confirm the root-census entries.
- `🧰️framework` members not in the root census: `semio-framework-3d` (20 files, 519,227 bytes, 6,926 lines; 1.98 bytes) and `semio-framework-3d-brep-engine` (8 files, 272,023 bytes, 3,934 lines; 1.04 bytes).
- `🎓️teaching`: `teaching-proctor` (11 files, 238,019 bytes, 4,457 lines; 1.11 lines), not in root census.
- `✏️s` root: 20 members, all within limits. The workspace excludes 236 entries, 224 of them under `🔌️plugins` (covered by section 2). The other excluded entries were not census-ed here.

## 4. Other findings and caveats

- **The root census doc is stale.** The census doc (`CRATE-SOURCE-GATE-CENSUS.md`) reports 210 nodes. The re-run found 224. The `os-kernel-db` and `os-renderer-wgpu` rows are unchanged.
- **Refusals mask size.** The census doc says the two refusals are "independent of size". Measured: the db closure is 2.6 MB / 52k lines after its parse bug is removed, and the wgpu closure is 5.9 MB / 102k lines. Both are much larger than the refusal message suggests.
- **Counter bug is in the shared counter.** `selectRustSourceV1` and `rustFindTopLevel` are used by the admission path, so the same false refusal would hit any crate that has a trailing cfg-false item. Only the db crate hits it today.
- **Method:** host cfg plus `debug_assertions`, default features only. Feature-unified units may differ. The gate's per-unit cfg was not captured (needs `cargo`/`rustc` unit graph, not run). Lines are `selected.split("\n").length` as in the canonical counter.
- **In-memory overlays** (`probe-closure.ts`, `probe-wgpu.ts`, `patch-storage.ts`) never wrote to repo files. The source tree is unchanged.
- **Gap:** which launcher path builds the live plugin catalog and whether it passes the boundary policy is not confirmed (section 2).
- **Process note:** a broad recursive `grep` over `🧰️framework` and the repo was started and killed; no output was used from those runs.

## Files

- Plugin CSV: `🗑️generated/plugin-census.csv` (1,357 rows).
- Workspace CSV: `🗑️generated/workspace-census.csv`.
- Scripts: `🗑️generated/plugin-census.ts`, `🗑️generated/workspace-census.ts`, `🗑️generated/audit/{diag-tree,dump-sel,unmatched,cfg-overrun,patch-storage,probe-closure,probe-inspect,probe-patch,probe-wgpu}.ts`.
