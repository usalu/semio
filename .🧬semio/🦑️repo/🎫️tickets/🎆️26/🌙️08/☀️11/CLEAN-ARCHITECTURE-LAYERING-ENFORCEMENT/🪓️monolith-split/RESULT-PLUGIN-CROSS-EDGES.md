# Result: Plugin-to-Plugin Cross Edges (R2/R3)

Scope: `✏️s/🔌️plugins`, cruiser rule family `no-cross-plugin-*`, plus Cargo and Rust include scans.
Raw logs and throwaway scripts: `🪓️monolith-split/🗑️generated/xplugin-*` (kept, per executor rules).

## 1. Headline numbers

| Measure | Before | After |
|---|---|---|
| Cruiser `no-cross-plugin-*` on `✏️s` (re-run, 10039 / 10089 modules) | **32** | **18** |
| Cargo cross-plugin path dependencies (`✏️s/🔌️plugins/**/Cargo.toml`) | 433 | 366 |
| Unused cross-plugin `[workspace.dependencies]` entries | 67 | 0 |
| Rust `include_str!`/`#[path]` into a sibling plugin | 4 | 3 |

The total cruiser violation count moved 10275 to 10480 only because peers changed other areas at the same time. Diffing the per-rule counts shows the removed rules are exactly mine: sourcing 4, dag 3, energy to cad 2, forms to stdio 2, block 1, tests to stdio 1, tests to energy 1, plus `plugin-no-extension-or-artifact-🧪️tests` 1.

## 2. Fixed edges (14 of 32)

### 2.1 Child-dialect checks (sourcing 4, dag 3, forms 2, block 1 = 10 edges)

Symbols that crossed: `parseSemioChild` (from stdio `✉️base/🧬️schema/🪆️child/🟦️.ts`), `semioKitDialectParts` (stdio Kit snapshot), and the stdio child JSON schema (test imports).

Decision: the symbol is a generic dialect check, not stdio domain. A plugin refers to another plugin's artifact kind by declared id (`@child kind=s.stdio.semio`, `$ref` by `$id`), never by import.

- New framework helper `requireArtifactDialect(actual, expected | expected[], at)` in `🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts`.
- Each consumer declares its expected dialect as data and composes it with the existing framework `parseArtifactChild`:
  - sourcing `parseCurationCatalogChild` (kit), used by the schema and the diff.
  - dag `parseDagContentChild` (graph), used by the schema and the diff.
  - forms `parseFormsChild` (value, table).
  - block `validateBlock3dCatalogDialect` accepts `kit` or `*`.
- stdio `parseSemioChild` now delegates to the same helper, so no duplicate logic remains.
- Consumer tests no longer import stdio's `child/🔣️.json`. New `declaredChildDialectSchema(id, childSchemaId, dialects)` in `🧬️schema/🔮️oracles/🪪️document/🟦️.ts` builds the `$defs` stand-in the plugin schemas `$ref`.
- Drift guard: new stdio test `✉️base/🧬️schema/🪆️child/🧪️tests/🟦️.ts`, registered as a suite in `🧿️semio/📦️packages/🟦️typescript/📜️script.ts`. It asserts the generated `$defs` deep-equal the authored `child/🔣️.json`, and that `parseSemioChild` agrees with an independent Ajv oracle for every subset.

Files changed:
- Framework: `artifact-reference/🟦️.ts`, `artifact-reference/🧪️tests/🟦️.ts`, `artifact-reference/🧪️tests/🧫️fixtures/🪪️identity/🔣️.json` (new `dialectExpectations`), `🔮️oracles/🪪️document/🟦️.ts`.
- stdio: `child/🟦️.ts`, `child/🧪️tests/🟦️.ts` (new), `📦️packages/🟦️typescript/📜️script.ts`.
- sourcing: `🧬️schema/🟦️.ts`, `🧬️schema/🔺️diff/🟦️.ts`, `🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`, `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`.
- dag: `🧬️schema/🟦️.ts`, `🔺️diff/🟦️.ts`, `🧪️tests/🪪️document-contract/🟦️.ts`.
- forms: `🧬️schema/🟦️.ts`, `🧬️schema/🧪️tests/🪪️document/🟦️.ts`.
- block: `3d/…/🧬️schema/📸️snapshot/🟦️.ts`.

Verification (real results):
- `bun test` artifact-reference tests: 14 pass, 0 fail (5 new dialect rows, each checked against an independent Ajv const oracle).
- `bun test` stdio child guard: 9 pass, 0 fail.
- Block sqlite snapshot suite (exercises `validateBlock3dCatalogDialect`): 65 pass.
- stdio semio sqlite suites kit, graph, table, value, drawing, image, base: 72 tests, 71 pass. The one failure was a 5 s timeout in the Base Native test while the cruiser ran. Re-run alone: 42 pass, 0 fail.
- Document-contract oracles:
  - Curation passes after the change, as before.
  - Forms and dag fail at their last assertions, identically to HEAD. I proved this by running HEAD copies of the files in a temporary sibling `✳️any-baseline` tree, since removed. Forms fails on a reference-identity assertion after `applyFormsDiff`. Dag fails because an `invalidIdentityDocuments[0]` (childId differs from artifactId) is accepted, so the parser does not enforce that law. These two are pre-existing defects, not caused by this work.
- The sourcing sqlite test file has 46 of 49 failures (missing `curationSnapshotToSqliteDatabase`); HEAD gives the identical 3 pass / 46 fail.
- `tsc --noEmit --strict` on all 18 touched TS files: 35 errors, all in two untouched regions (forms `🔣️json/🟦️.ts` 3, sourcing sqlite test 32), none on edited lines.

Caveat: the error text of the mismatch changed from "Semio child dialect mismatch" to "<at>: artifact dialect mismatch". No test or fixture referenced the old text (git grep). A Rust twin of `requireArtifactDialect` was not added (see section 5).

### 2.2 `tests` plugin (2 edges)

`🧪️tests/📁fixture-placement/🟦️.ts` imported `runWeatherChecks` (energy) and `runOriginalCopyCloseChecks` (stdio contract) to re-run other plugins' oracles. Both are already run by their owners (energy model Rust package script, stdio contract Rust package script). Dead edge: the two redundant tests and imports are deleted.
Verification: the file runs 35 pass / 4 fail. The 4 failures concern procedural and puzzle paths (`🧪️transform-tool`, procedural editor tests) and none touches the deleted tests or my files. They are not from this change.

### 2.3 energy to cad (2 edges)

`🔋️energy/…/🧬️mutations/🧪️tests/🪪️owned-target/🟦️.test.ts` tested CAD's `parseCadChildPayload` with a CAD fixture. That is a CAD test in the wrong plugin. Moved into `📐️cad/…/🧬️mutations/🧪️tests/🪪️owned-target/🟦️.test.ts`; the energy file keeps only its own test. Registered as `verify owned-target` in the cad Rust package `📜️script.ts` and `📋️project.json` (`verify-owned-target`, workspaceCommand `cad-owned-target`).
Verification: `bun test` energy 1 pass (10 expects), cad 1 pass (8 expects); the original file had 2 tests with 18 expects.
`bun repo run @semio-tech/cad-cad-rs:verify-owned-target` could not run: the runner stops at `Canonical native dependency owner required: …cad/…/🧬️schema/🔺️diff/📦️packages/🦀️rust`, a pre-existing cad ownership fault independent of this change.

### 2.4 Cargo dead edges (67 entries, 35 manifests)

Static analysis: a cross-plugin `[workspace.dependencies.X]` path entry that no member crate inherits (`X = { workspace = true }`, `X.workspace = true`, or `[dependencies.X] workspace = true`) is inert. Removed all 67, including the two `norm to fem` and one `writer to stdio-docx` edges.
Verification: all 1357 plugin manifests parse (`Bun.TOML`), and all 14167 `workspace = true` references resolve against the nearest workspace root (0 problems). Re-scan: 0 unused cross-plugin entries remain.
Cargo itself: **blocked**. `bun repo cargo … fetch --locked` fails at baseline for dag, forms, cad and block (stale `Cargo.lock`; cad also "no matching package named `semio-framework-3d-brep-operations`"). Tried once on cad, did not loop. Compile is reported as blocked by the repo's cargo state and the source-boundary gate.

### 2.5 Rust include (1 edge)

`📐️cad/…/✏️editor/🧪️tests/📦️member-authority/🦀️.rs` included a 72-byte OBJ from stdio's fixtures. Copied byte-identical (`cmp`) to `…/📦️member-authority/🧫️fixtures/⬅️before.obj` and the `include_str!` now uses the local path. The fixture-placement rule accepts it (checked). Not compiled (blocked, above).

## 3. Remaining edges (18) and why

All remaining edges are one structural fact: **stdio hosts the native Semio standards and the file-format codecs, and consumers use them as real data types and codecs**. A symbol-level fix would only duplicate them.

| Edges | Consumer | What it needs from stdio / forms | Decision and blocker |
|---|---|---|---|
| 6 | layout to forms | `FormDictionary` schema, `native-json` codec, schema JSON | Genuine shared type (ordered unique id to `IntrinsicValue`). Clean fix: move it to the framework value module as a neutral dictionary. Touches forms (TS, Rust, graphql, proto, json, canonical, native-json) and about 45 layout files in all languages, including `FormDictionary` paths in layout Rust, sqlite DDL and mutations. Needs its own ticket. Not started. |
| 4 | layout to stdio | Semio drawing snapshot type/parser, drawing sqlite DDL and codec, drawing JSON schema (test) | Native Semio drawing standard, see section 4. The Rust side also `include_str!`s the drawing `🗄️.sql` (the one remaining runtime Rust include). |
| 2 | raster to stdio | `SemioImageSnapshot` type (type-only import) and image JSON schema (decoded-image oracle test) | Native Semio image standard, see section 4. Raster already carries a local copy of the parser, so only the type crosses; a local copy would be a duplicate of a shape that also exists in Rust. |
| 2 | trinity to stdio | Semio graph snapshot and graph sqlite codec, used in a 493-line integration test of the working-graph child | Test-only. It is a real cross-plugin integration test; it belongs in an s-level test project outside `🔌️plugins` (the `🧪️tests` plugin is itself checked). Creating one needs registrations in `.vscode/launch.json` and `📋️build-owners.json`, which are shared and churning, so I did not. |
| 4 | procedural to stdio | glTF/OBJ/PLY snapshot types and parsers inside `generation3d/🚪️io/🟦️.ts` (conversion between the framework polygon mesh and the formats) | Codec logic living in the wrong plugin. Clean fix: the polygon-mesh to format conversions move into the stdio format artifacts and procedural reaches them through a framework registration port. Sizeable (the file is 43 KB) and ties to the Rust `🔺️mesh-bridge`. Not started. |

Rust side (not visible to the cruiser) after the dead-edge cleanup: 366 cross-plugin Cargo path dependencies. 361 point into stdio (36 distinct stdio artifacts, 25 distinct Semio sub-crates: base 25, drawing 24, mesh 18, value 17, image 10, table 10, brep 6, text 6, flow 5 and others), plus writer to trinity-jack 1, layout to forms 1, lowpoly to cad 1, playbook to flow 2. Remaining Rust includes into a sibling: flow to note action-cohort fixture, flow-brep to procedural `evaluate-budget` fixture (both are deliberate two-sided contract fixtures; the right fix is to move each to its contract owner rather than copy it) and layout to stdio drawing DDL.

## 4. Decision needed from the parent

433 Cargo and about 30 TS edges point at one target, so per-edge TS patches cannot reach zero. Options, in my order of preference:

1. **Relocate the native Semio standard (`🗄️stdio/🗿️artifacts/🧿️semio`) and the format artifacts into the framework as a formats module** (one scripted, resolver-based path codemod: every `…/🗄️stdio/🗿️artifacts/X` reference is recomputed from absolute paths; about 366 Cargo lines, 18 TS imports, taxonomy and registry entries). It clears all 18 TS edges and about 360 Cargo edges at once and also removes the framework's `s.stdio.*` id leaks if combined with work package 1 (neutral ids). Cost: the semio tree is 7459 files; moving it needs a green Rust toolchain, which is blocked today (stale locks, source-boundary gate).
2. Declare stdio a library target in the policy generator (allowed target of `no-cross-plugin`). Cheapest, but it permanently exempts the largest coupling.
3. Piecemeal: dictionary to framework value, polygon-mesh conversions into stdio with a port, trinity integration test into an s-level project.

## 5. Not done / open

- No Rust twin of `requireArtifactDialect` (the Rust side still calls stdio's `validate_semio_child_identity`); needed once option 1 moves the standard, and avoids rebuilding the widely used `artifact-reference` crate now.
- Dag `🧪️tests/🪪️document/🟦️.ts` is referenced by its Rust script but the directory is actually `🪪️document-contract`; pre-existing, unchanged.
- Compile checks: none possible (cargo blocked), so Rust edits are verified statically only.
- No debug logs were added to source; scratch scripts and logs are only under `🗑️generated/xplugin-*`.

---

# Wave 2: coordinator decision "option 3 now" (policy exemption rejected)

Cruiser `no-cross-plugin-*` on `✏️s` after wave 2: **4** (was 32 at the start, 18 after wave 1). Raw: `🗑️generated/xplugin-after3.txt` (10096 modules). The 4 left are all `layout to stdio` for the Semio drawing standard (section W2.5). The dictionary and test moves re-target edges at the framework, so they now count under the already existing `plugins-framework-sdk-only` and `no-cross-technology-✏️s-to-🧰️framework` families (about 1750 before); the SDK-barrel work packages 7 and 8 own those.

## W2.1 (a) FormDictionary moves down into the framework value module — done

Decision: the type is "ordered unique question identity to intrinsic value", which the framework already models (`questionId` appears in the manifest and playbook). Home: `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/`, a sibling of the value module's other leaves, with its own Rust crate `semio-framework-value-dictionary`.

- Symbols renamed everywhere, wire unchanged (`entries[{questionId,value}]`, same limb form, so no fixture or hash changes): `FormDictionary`/`FormDictionaryEntry` to `QuestionDictionary`/`QuestionDictionaryEntry` (Rust, TS, GraphQL, proto), `parseFormDictionary` to `parseQuestionDictionary`, `formDictionary*NativeJson` to `questionDictionary*NativeJson`, layout's `projectFormDictionary`/`reconstructFormDictionary` to the `Question` spelling. Layout's own SQL table `layout_form_dictionary` is layout-owned persistence and stays.
- Moved with `mv` (nothing left in forms, no alias): schema `🟦️.ts`, `🔣️.json` (new `$id` `…/framework/value/question-dictionary.json`), `🔗️.graphql`, `🛰️.proto` (package `semio.framework.value.dictionary`), Rust `🦀️.rs` plus `🌳️canonical` and `🪪️native-json`, and the TS native-json codec.
- New framework crate files: `📦️packages/🦀️rust/Cargo.toml` and `📋️project.json`; member and workspace dependency added to `🧰️framework/Cargo.toml`; build-owners entry `@semio-tech/value-dictionary-rs`; six build/check/test launch entries.
- New tests (there were none at the framework level): `🧪️tests/🟦️.ts` with Ajv as the independent oracle over a language-agnostic fixture (`🧪️tests/🧫️fixtures/🔣️.json`: 2 valid, 10 structurally invalid, 1 duplicate-identity row) and `🧪️tests/🦀️.rs` driving the same fixture through serde. Registered as `value-ts:test-dictionary` (script, project target, launch entry).
- Forms: `schema/🦀️.rs` drops `pub mod dictionary`, `configured_dictionary` returns the framework type; forms editor test and `Cargo.toml` updated.
- Layout: all uses in 39 files updated in every language (TS, Rust, JSON `$ref`s, GraphQL, proto, g4). Five of seven manifests swap `semio-s-artifact-forms-forms` for `semio-framework-value-dictionary`; the editor crate keeps forms only as a `[dev-dependencies]` entry for the one live-VCS law (below).

Verification (real results):
- `bun test` framework dictionary: **13 pass, 0 fail**.
- Layout sqlite snapshot suite: 18 pass, 1 fail. The failure ("Layout dictionary changes distinguish unchanged…") is `$.dataFields: unknown field` inside `parseLayoutDataFieldsDelta` against a fixture whose shape the parser does not accept; only identifier renames touch that code, so it is not caused by the move (not re-run at HEAD).
- Layout document oracle `testLayoutDocumentContractOracle()`: passes.
- `tsc --strict` on the 16 touched or moved TS files: one error, `TS1005` on `typeof a.b[0].row` in an unchanged line of `layout/…/🪪️document/🟦️.ts`; Bun accepts it.
- Rust, static: `rustfmt --edition 2021 --check` parses the whole dictionary module tree with no `error`. `rustc +nightly-2026-07-20 -Zunpretty=expanded` differential, error sets normalised for the rename:
  - dictionary module HEAD (in forms) vs new crate: 25 errors each, identical sets (only unresolved extern crates).
  - layout, 7 crate roots (model, diff, mutations, mutations-content, editor, commands, export): identical, except one extra `unresolved import` of the new crate in the model root, from the explicit `use semio_framework_value_dictionary::QuestionDictionary;` I added to the sqlite dictionary module (it previously relied on an unresolved glob).
  - forms root: 363 vs 360 errors; the difference is exactly the moved module's own extern imports (−4) and the new dependency import (+1).
- Manifests: all 1357 plugin manifests parse and all 14168 `workspace = true` references resolve (0 problems). New-crate relative paths exist (checked).
- Cargo itself: **blocked** (stale lockfiles and the source-boundary gate; the framework, forms and layout `Cargo.lock` files still need the new package entry).

Remaining Rust edge from this change: `layout/…/✏️editor/📦️packages/🦀️rust` keeps `semio-s-artifact-forms-forms` as a **dev**-dependency because `✏️editor/🧪️tests/🔬️unit/🎞️intrinsic/🦀️.rs` runs a live Forms app and feeds its `dictionary:out` into a live Layout app. That is a genuine two-plugin law; its home is a Rust integration crate under the s-level composition project, which cannot be created and verified without a compiler (queued, W2.6).

## W2.2 (d) s-level composition project — done

`✏️s/🧪️tests/🏘️plugin-composition/` (project `@semio-tech/s-plugin-composition`, target `test`), modelled on the existing `✏️s/🧪️tests/🗂️native-ownership`: the policy generator only forbids plugin-to-plugin pairs (every plugin, including `🧪️tests`), so an `✏️s/🧪️tests` project may depend on several plugins. Registered in `📋️build-owners.json` and with a launch entry; the script uses the modern `ScriptRouter` + `receiveScriptProcessInvocation` pattern.

- `🔱️trinity-graph-child/🟦️.ts`: six tests moved out of trinity's 493-line sqlite test (those importing stdio's graph codec), with their helper lines; relative fixture URLs resolve against the old directory. Result sets, old trinity file plus new file, were **identical to the baseline** (same 16 failing names before and after).
  Then I repaired two stale bits in the moved code: graph JSON functions now come from `🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json` (they moved), and a shadowed constant was renamed. The moved file now has 3 of 6 passing (editor initial, full child lease frame, composed graph with rich typed ownership), 3 failing for pre-existing reasons: two need the artifact-reference schema registered in their Ajv instances, one reads a fixture path that no longer exists in stdio.
- `🖨️raster-decoded-image/🟦️.ts`: the stdio image-schema conformance loop of raster's decoded-image test (3 of its 5 tests). Raster keeps the other 2.
- `🌀️procedural-formats/`: see W2.3.
- Runner check: `bun repo run @semio-tech/s-plugin-composition:test` loads the project, runs bun over the files, and exits 1 only because of those 3 pre-existing trinity failures.

## W2.3 (c) procedural to stdio (glTF/OBJ/PLY) — done by relocation, no port needed

`mesh-engine` only has Rust importer/exporter traits; no TS glTF/OBJ/PLY parser exists in the framework to reuse. The TS code in `generation3d/🚪️io/🟦️.ts` (conversion between the framework polygon mesh and the three format snapshots, plus three stdio parse functions) was used by exactly one consumer: the TS "third-party twin" of the artifact-IO surface test, which by design reads each `s.stdio.<format>` definition from disk. That is an integration oracle, so both files moved to `🌀️procedural-formats/` (library `🟦️.ts`, test `🗿️artifact-surface/🟦️.ts`, bun wrapper `🧪️laws/🟦️.ts`):
- Procedural's package script lost the TS half of `verify generation3d-document-io` (the Rust `artifact_surface` and, with `native`, `document_io` cargo tests remain) and the `testGeneration3dIoInputContracts` line of `canonical-io`; the selection unit test lost its two calls (both now run in the composition project). Rust doc comment updated to the new twin path.
- Fixed four stale fixture paths in the moved twin; results: baseline 2 of 6 functions threw `ENOENT` (document-io surface, io input contracts), now **all 7 bun tests pass** (capacity 7, mesh surface 6, polygon import 4, io input contracts 12 checks).

## W2.4 (b) raster to stdio (Semio image) — TS edges cleared, Rust parity not moved

- The raster TS schema now declares the four interfaces its own parser already builds (`SemioImageSnapshot` and parts), replacing the stdio type import and re-export. The independent conformance check against the stored stdio schema is the relocated composition test above (`bun test`: 2 raster + 3 composition = the original 5, all passing). `tsc` reports one `TS2769` on `expect(image).toEqual(row.image)`; the identical expression with stdio's own type gives the same error in a scratch file, so it is a pre-existing typing mismatch.
- Not done, with evidence: moving the **Rust** image document to the framework. `SemioImageSnapshot` is an `ArtifactSchema` document (`id = "s.stdio.semio.image"`, `#[state(artifact)]` fields, four derive macros) referenced by 178 files (118 in stdio, 24 in raster, wfc 4, remodel 3, gis 1, shooting 1). A neutral framework type would have to split `schema` from the pixel fields (a `flatten` across ToValue, FromValue, CanonicalJsonTree, RetainedClone, ArtifactSchema, all unverifiable without a compiler), or the framework would host an `s.stdio` document id, contradicting work package 1. The existing framework primitive (`RasterImage`, single-frame RGBA8) is not a substitute. The image subset crates also depend on stdio `base` and `contract`, so moving the subset is option 1.

## W2.5 (b) layout to stdio (Semio drawing) — not done

Layout's TS and Rust use the drawing standard as executable code, not only as types: `parseSemioDrawingSnapshot`, `DrawNode`, `DrawStyle`, `SemioDrawingFormat`, the drawing sqlite DDL (`include_str!`d into layout's schema) and sqlite/native-json codecs, plus `encode_drawing`/`decode_drawing`. The drawing subset is 470 files in 6 crates that depend on stdio `base` geometry and value types. A "generic primitive" cut would have to lift base geometry with it. Splitting only the layout test (1 of the 4 edges) would unhook layout's `verify layout-document-contract` command without clearing the production edges, so I left all four.

## W2.6 Queued follow-ups

1. **Option 1 (queued, deferred until a compiler can verify it):** move the native Semio standard (`🗄️stdio/🗿️artifacts/🧿️semio`, with `base`, `drawing`, `image`, `graph`, `kit`, `mesh`, `value`, `table`, …) and the format artifacts into the framework as a formats module. One resolver-based path codemod rewrites about 366 Cargo paths and the 4 remaining TS imports and the raster/layout Rust uses; it must be combined with work package 1 (neutral ids) so the framework does not host `s.stdio.*`. It also lets the `SemioImageSnapshot` and drawing types be used without any plugin boundary.
2. Rust integration crate under `✏️s/🧪️tests/🏘️plugin-composition` for the live Forms-to-Layout `dictionary:out` law, then drop forms from layout's editor dev-dependencies.
3. Register the artifact-reference schema in the 3 failing moved trinity tests and repoint the stale stdio fixture path; then the composition target is green.
4. Refresh `Cargo.lock` for the framework, forms and layout workspaces (new package `semio-framework-value-dictionary`) once cargo is usable.
5. Optional: split the two flow contract fixtures (note action-cohort, procedural evaluate-budget) to their contract owners.

## W2.7 Files changed in wave 2 (beyond wave 1)

New: `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/**` (15 files including the crate manifest and project file), `✏️s/🧪️tests/🏘️plugin-composition/**` (8 files).
Edited: framework `Cargo.toml`, `🌱️value/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}`, `📋️build-owners.json`, `.vscode/launch.json` (generated file edited by hand: composition project, `value-dictionary-rs` ×6, `value-ts:test-dictionary`, cad `verify-owned-target`), forms (`🧬️schema/🦀️.rs`, editor unit test, crate `Cargo.toml`), layout (39 migrated files, 7 `Cargo.toml`, intrinsic test, sqlite dictionary module), raster (`🧬️schema/🟦️.ts`, decoded-image test), trinity (sqlite test), procedural (package script, selection test, artifact-surface Rust doc), and the deletion of procedural's two TS files (moved).
Scratch and logs only under `🗑️generated/xplugin-*`.
