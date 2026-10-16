# Result: Plugin Assets, Splits and Counter Defects (PLUGIN-ASSETS-SPLITS)

Executor: Sonnet, foreground, no sub-agents. Scope: (A) move embedded payloads of at least 262,144 bytes out of the compiled closures of the 12 asset-blocked plugin packages, (B) the two S5 file splits (`mathematical-equation` `cas-internals`, `sequence` editor), (C) the S8 counter defects (pdf, space-home, bim-model, puzzle-5d, sourcing-curation).
Scratch tools and logs: `🗑️generated/pas-*` (not deleted). Library root used below: `L = 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## 1. Outcome in one table

| package (crate) | embedded payload at least 262,144 B before | after | how |
| --- | --- | --- | --- |
| stdio-epw | test: 397,789 JSON contract fixture | none (test 849,123 -> 451,564 B) | runtime read |
| stdio-gif | lib: 4,433,936 dancing.gif; test: 14,053,614 pack, 21,033,376 dsl, (+75,523 oracle stays) | none (lib 5,047,093 -> 615,127 B; test 40,432,410 -> 914,998 B) | package asset port (gif), runtime read (tests) |
| stdio-mp4 | test: 2,719,031 `.mp4` fixture | none (test 3,401,278 -> 683,162 B) | runtime read |
| stdio-jpg | test: 483,496 JPEG fixture | none (test 1,133,748 -> 650,386 B) | runtime read |
| stdio-avi | test: 766,968 AVI fixture | none (test 1,120,789 -> 354,046 B) | runtime read |
| architect-program | lib: 375,218 snapshot JSON Schema, 317,901 artifact JSON Schema (+204,801 diff JSON, 146,269 and 83,879 TypeScript leaves) | none (lib 2,828,975 -> 1,598,344 B; test 3,732,392 -> 2,501,761 B) | served schema documents + package assets + guest reader |
| stdio-gltf-editor | lib: 284,436 `.glb` | none (lib 358,157 -> 76,825 B) | deferred example + digest sidecar + package asset port |
| stdio-gltf-model | test: 284,436 `.glb` | none (test 1,017,098 -> 732,745 B) | runtime read |
| stdio-gltf-inferences | none at 262,144 or more | unchanged | n/a (the survey counted the `.glb` through the editor crate) |
| stdio-semio | test: 14 fixtures, largest 2,828,738 | none (test 13,537,730 -> 1,676,498 B) | runtime read |
| stdio-semio-presentation / -mesh / -brep | test: 315,532 / 288,536 / 282,543 oracle JSON | none (test 801,115 -> 485,671; 1,009,645 -> 721,197; 1,121,278 -> 838,823 B) | runtime read |
| stdio-pdf (in scope as S4/S8) | lib: 6,346,331 PDF, 295,342 TypeScript leaf; test: second 6.3 MB copy; test closure refused | none (lib 10,539,688 -> 3,737,829 B; test closure now measurable, 4,737,216 B) | package asset port + served TypeScript leaves + runtime read |
| puzzle-5d (in scope as S4/S8) | lib: 3,058,536 DSL, 860,636 golden JSON | none (lib 5,901,121 -> 1,984,750 B; test 7,689,779 -> 3,774,015 B) | deferred example + digest sidecar + package asset + hub declaration |

Census run: `🗑️generated/pas-emb.ts` (the canonical `readRustSourceClosureV1`, host cfg plus `debug_assertions`, with and without `test`); before logs `pas-emb-1.log`, `pas-emb-2.log`, after log `pas-emb-after.log`. Re-measured with `MIN=262144`: no embedded member at or above the limit remains in any of the 14 packages above, lib or test.

None of the 12 plans clean with the generator yet (section 6): every one of them still has the structural blockers that survey v3 listed next to the asset (parent-child, coherence, over-budget schema slice). Nothing was carved, by design: the brief says to carve only what plans clean.

## 2. (A) Payload moves: three mechanisms

### 2.1 Test-only payloads: read at test run time

Pattern (the same one the repo already uses in `runtime_fixture_text`, `bcf`, `html`, `las`): `std::fs::read[_to_string](concat!(env!("CARGO_MANIFEST_DIR"), "/<manifest-relative path>"))`, with the literal directly behind `env!` so the carve engine's `rebaseManifestDirPaths` rewrites it when a file lands in another crate. A `const X: &[u8] = include_bytes!(..)` became `static X: LazyLock<&'static [u8]>` over a leaked read, and each use became `*X` (T1 shape). Tooling: `pas-lib.ts` (`rewriteRuntime`, `constToRuntime`), driver `pas-auto.ts` (converts only sites under a `🧪️tests` directory and reports production sites untouched).

Converted sites: epw receiving test; mp4 (editor unit, snapshot/diff/mutations unit, codec; 7 sites); jpg derived-analysis; avi codec; gif 89a `fixture_honesty_law` (21 MB and 14 MB fixtures) and the dancing example tests; gltf-model sqlite snapshot test; semio (10 `↩️inverts` tests, the base oracle manifest check, 2 sqlite snapshot test files incl. `🎛️complete-semantic.json` and graph `hex-float`); presentation/mesh/brep oracle checks; bim sqlite test (788,893 B); pdf `THESIS` in the 1.4 io unit and the 1.7 export unit; 5d golden poses (860,636 B).

Byte identity for these is by construction (same file, read instead of embedded); their assertions are untouched.

### 2.2 Production payloads that have no registered consumer yet: gif dancing, pdf bachelor-thesis

Both example leaves are mounted but no editor registers their `source()` (the shipped `stdio` plugin assembles html, md, csv, tsv, txt, json and xml only). They keep their API (`decoded_snapshot()`, `source()`, `demo_gif_snapshot()` need no caller change) and read the bytes through the guest asset port:

- constants `DANCING_GIF_ASSET_NAME` / `_SIZE_BYTES` / `_SHA256` (and `BACHELOR_THESIS_ASSET_*`): the repository-relative path as package asset name, exact size and SHA-256;
- `dancing_gif_bytes()` / `document_bytes()`: `semio_framework_plugin::reactor::package_asset(NAME)`; under `cfg(test)` a host-less fallback reads the repository file once per process (found through `nx.json` above `CARGO_MANIFEST_DIR`); outside tests a missing delivery panics with the asset name;
- no hub declares them: nothing in a shipped plugin reads them, and a declared package asset is preloaded into every instance open. When a plugin registers these examples it declares `.asset(..)` from the same constants.

Generator: `pas-lib.ts` `assetPortBlock`.

### 2.3 Registered deferred examples: gltf metabolism, puzzle-5d capsule-dream

`ExampleSource::deferred` takes the authored bytes as `&'static [u8]`, which is the embedding. The `plugin` crate (peer-owned, being split right now; `builder/🧩️app` has an in-flight `📦️packages`) is not touched. Instead the leaf embeds a three-line **digest sidecar** `🔏️.digest` (`name`, `size`, `sha256`) next to the payload and hands that to `deferred` as its `source`:

- the descriptor declares the example by `example_body_asset_name(dialect, id, ".glb.digest" | ".dsl.semio.digest")`, which satisfies `auditNavbarExampleArtifactPayload` (`<prefix>.` rule) and needs neither a parse nor the payload;
- the producer reads the delivered file (`package_asset`, same test fallback as 2.2) and does the decode or parse only when something asks for the document, as before;
- puzzle-5d additionally exposes `dsl_asset_declaration()` (name = repository path, exact size and SHA-256, media type text document) and `🌎️hub/🧩️compositions/🧩️puzzle/🦀️.rs` declares it with `.asset(..)`, so the existing hosts (`package_file_assets`, the browser kernel `📦️package-assets`) deliver it with `Event::InstanceOpen.assets`;
- gltf declares nothing in a hub: the gltf editor is not in the shipped `stdio` plugin yet; `base_glb_bytes_digest()` has the triple for whoever registers it.

Removed from the 5d lib as dead embedded data (no consumer in the repo, checked with `git grep`): `DSL_TEXT`, `OP_TEXT`, `PACK_BYTES`, `SPR_BYTES`, `GOLDEN_POSES_JSON` of the capsule-dream leaf and `PUZZLE5D_CAPSULE_DREAM_EXAMPLE_TEXT` of the io snapshot (its one test now reads `capsule_dream::dsl_text()`); the leaf tests read the DSL through the leaf accessor and the golden JSON from disk. The `every-example` law in 5d now takes the capsule-dream DSL from `dsl_text()` (its `source` is the sidecar). Greenfield, so no alias was left.

### 2.4 Schemas: served documents and a guest reader

- `L/🏗️authoring/📦️crate-tree/🧾️facet-leaves` (the S3 codemod) gained `options.json` (CLI `bun ./📜️script.ts new leaves <rust-file> [--dry-run] [--json-schema]`): the JSON Schema leaf is served too. Unit test added to `L/🧪️tests/📦️crate-tree`.
- schema registry (`🧰️framework/🔨️modules/🧬️schema/📇️registry/🦀️.rs`, region `🔖️SchemaLeafDocuments`): `PackageAssetSchemaLeaves` (reads a document from the package assets a host delivered, UTF-8 checked once, cached) and `install_default_schema_leaf_reader(package_asset, manifest_directory)`: on `wasm32` the package-asset reader, elsewhere `FileSystemSchemaLeaves` rooted at the checkout above the manifest. Exported in the package root; unit test `package_asset_leaves_answer_delivered_documents_and_refuse_everything_else`.
- architect-program: codemod applied with `--json-schema` (16 leaves of `s.architect.program` now empty bodies plus `register_schema_leaf_documents`); the descriptor function installs the default reader (`wasm32` passes an empty manifest directory so no build path is baked into the guest); `🧬️schema/🔏️.digest` lists the four JSON documents (name, size, SHA-256) and `schema_leaf_asset_declarations()` turns it into `AssetDeclaration`s; the architect hub `plugin()` declares them with `.asset(..)`. The TypeScript, GraphQL and proto leaves are served but deliberately not declared: no guest consumes them, so shipping them to every instance would be waste. Cargo: `semio-framework` path dependency added to the architect-program manifest and the matching row to its `Cargo.lock`.
- pdf 1.7: codemod applied (TypeScript, GraphQL, proto leaves of four facets, 12 documents) and the same default reader installed. Nothing in a guest reads those formats.

## 3. (B) S5 file splits

- `➗️equation` `🧬️schema/💡️inferences/🌿️cas-internals/🦀️.rs`: 5,091 lines -> root of 143 lines (mounts and the explicit `pub use` list) plus 20 child modules, one per former inline `mod` (`🏷️fnkind`, `🧮️expr`, `📏️canon`, `🧷️assume`, `🧭️visit`, `🖨️fmt`, `🧩️pattern`, `🌉️polybridge`, `🧹️simplify`, `📐️trig`, `📈️diff`, `🎞️series`, `🎯️limits`, `🌱️rootof`, `🧠️solve`, `🔢️matrix`, `➗️integrate`, `➕️sums`, `🌊️ode`, `🔄️transforms`), 64 to 489 lines each. Module paths (`crate::cas::..`) are unchanged because each child is mounted with `#[path]` under the same name; the cfg(test) `include!` of `🧪️tests/🔬️<name>-unit` was re-pointed to `../🧪️tests/..`; every `//#region` marker moved with its block; the first doc line of each child starts with the folder emoji. Tool: `pas-split-cas.ts` (asserts every inline module ends at column 0 and the line multiset is preserved).
  Effect (planner run `pw0-plan.ts`): the `cas` slice is now `semio-s-artifact-mathematical-equation-cas` with lib 11 files / 106,188 B / 2,853 lines and test 21 / 131,300 / 3,496, instead of an unplaceable 5,091-line file. The equation plan still has the survey's structural problems (mutations crate 70 files / 3,593 lines, two mirror roots over the 150-line wiring budget).
- `🎬️sequence` `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: 3,739 lines -> 123-line root plus six region children: `🧱️base` (constants, io, camera, errors; 245 lines), `🖥️host` (579), `🧵️retained` (commands, caps, artifact/config/example retained routes; 848), `🔁️persistent` (reorganize, node graph, run order; 959), `🎞️import` (442), `🎹️app` (proof catalogs, `ArtifactEditor` impl, manifest; 589). The unit-test mount stays in the root. Tool `pas-split-flat.ts` (parses the top-level item skeleton with the crate-tree parser, assigns whole top-level regions, rewrites the root to `#[path] mod x; pub use self::x::*;`, gives each child `use super::*;`, and widens to `pub(super)` exactly the private items, inherent methods and fields whose name another part still uses: 107 widenings from the dry run plus 4 found afterwards by checking the identifiers of the descendant unit-test module, which sees the original module's private items (`rebuild_dag`, `build_dag_host_snapshot`, `step_to_dag_node`, `sequence_context_menu_items`); `splitFlat` now takes such descendant files as an extra argument. An inherent method name such as `new` or `empty` widens everywhere it occurs, which is harmless). `pub use` is deliberate: the original `pub` items stay reachable at the same path from the grandparent glob. The module was named `base`, not `core`, because a child called `core` would shadow the `core::` crate paths of every descendant.
  Effect: the editor no longer is one 3.7k-line unit. The planner now reports the `snapshot` slice (which contains the editor) as the over-budget crate (lib 31 files / 318,632 B / 5,491 lines), a facade-weaving blocker, not a file-size one. Residual risk of this split: E0616/E0624 (field and method privacy) are typeck-phase and not covered by the differential.

## 4. (C) S8 / counter-input defects

| crate | defect | fix | state now |
| --- | --- | --- | --- |
| pdf | `⚖️lopdf-vectors` generated input (an `include_bytes!(concat!(.., $directory, ..))` inside `macro_rules!`); the path also lacked the `/` between `🧫️fixtures` and the directory, so the file could not have compiled | fixtures read at run time (`LOPDF_FIXTURES` + `fixture()`), slash restored | test closure measurable: 1,240 files / 4,737,216 B / 65,064 lines |
| space-home | generated input in `✏️editor/🫧️transient/🧪️tests/🔬️unit` (`vector!` macro with `include_str!(concat!(..))`) | `vector(name)` reads the five committed JSON documents at run time | test closure measurable: 155 / 581,453 / 8,251 |
| bim-model | "unsupported expression include_str": `fixtures!` macro in `🕸️model-graph/🎯️dirty/🧪️tests/🔬️unit` with a path one level too deep (`../../../../../../🧫️fixtures` does not exist, the fixtures are under `🧪️tests/🧫️fixtures`) | `fixtures()` reads them at run time from the real location; the 788,893 B sqlite fixture also moved | test closure measurable: 11,107 files / 27.9 MB (a separate finding, section 7) |
| puzzle-5d | "missing source input" in the survey | already measurable at the start of this task (a peer fixed it); the S4 payloads were the remaining problem | lib and test measurable |
| sourcing-curation | `include_str!("../../../🧫️fixtures/🚚️retained-table/..")` missing the `🧪️tests` segment | path corrected | test closure measurable: 183 / 756,435 / 10,081 |

Three of the five (pdf, bim, curation) were real compile errors at HEAD, not only counter refusals: the include path could not be resolved. They are fixed, not worked around.

## 5. Tests added or extended

- `L/🧪️tests/📦️package-asset-constants/🟦️.ts` (new, registered as `test-package-asset-constants`, launch order 206.17006 in both launch files; `📜️script.ts`, `📋️project.json`, `package.json`): language-agnostic byte-identity law. For the gif and pdf constants and for the three digest sidecars (gltf, puzzle-5d capsule-dream, architect's four JSON documents) it recomputes size and SHA-256 with `node:crypto` and compares them with the Rust constants / sidecar rows; a corrupted digest is refused. Run directly: 6 pass, 28 expect calls.
- `L/🧪️tests/📦️crate-tree/🟦️.ts`: new test for `--json-schema`; 18 pass (run directly).
- Rust (not run, see section 6): registry `package_asset_leaves_answer_delivered_documents_and_refuse_everything_else`; gif `the_declared_package_asset_matches_the_fixture_bytes`; pdf `the_declared_package_asset_matches_the_fixture_bytes`; gltf `the_digest_sidecar_declares_the_fixture_it_stands_for`; 5d `the_digest_sidecar_declares_the_dsl_it_stands_for`.

## 6. Verification actually run

| check | command / driver | result |
| --- | --- | --- |
| census of embedded members, lib and test cfg, before and after | `bun pas-emb.ts <pkg..>` (`MIN=262144`, `MIN=65536`) | table in section 1; no member at or above 262,144 B left |
| rustfmt parse of every touched or new Rust file | `rustfmt --edition 2021 --emit stdout <file>` over 54 files plus the later edits | exit 0 for all |
| registry crate type check (the only crate I changed with no dependencies) | `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib -Zno-codegen --out-dir <scratchpad>` for lib, `--test` and `--target wasm32-wasip2` | no diagnostics; the output directory was deleted afterwards (`/dev` is not writable in this sandbox, so `-o /dev/null` was not possible) |
| name-resolution differential, HEAD tree against working tree, with the oracle sandbox patches (`pas-oracle.py`) | sequence (lib), equation, puzzle-5d, pdf, gif, gltf root, architect-program | sequence: 1,611 errors in both (rerun after the 4 test-driven widenings), identical multisets, no E0603/E0583; equation: 1,789 vs 1,789, identical; gltf root: 242 vs 242, identical; 5d: 5,427 vs 5,434, the 3 new distinct lines are unresolved external crates (`semio_framework`, `semio_framework_plugin`) that the new code names, no new internal path error; pdf: 13,448 vs 13,477 and architect: 5,503 vs 5,544, the new lines are only unresolved external crates plus `CARGO_MANIFEST_DIR` not defined (the sandbox has no cargo environment); gif: 2,016 vs 2,017, one more unresolved `semio_framework_plugin` |
| losslessness of the two splits | line multiset in the tools | equation: 54 line shapes differ, all of them the module wrapper brace and header, the doc re-emojis and the include prefix; sequence: 66, all `pub(super)` insertions |
| planner on all touched packages | `pw0-plan.ts` (`pas-plan-all.log`) | section 7 |
| new law test | `bun test ./🟦️.ts` in `L/🧪️tests/📦️package-asset-constants` | 6 pass |
| crate-tree tests | `bun test ./🟦️.ts` in `L/🧪️tests/📦️crate-tree` | 18 pass |
| JSON parse | `.vscode/launch.json` (JSON), `.vscode/🧩️launch.seed.jsonc` (`Bun.JSONC`), `📋️project.json`, `package.json` of the TS package | all parse |

### Blocked or not run

- Real compile and tests: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-s-artifact-stdio-mp4 --tests` failed at the start with `Finite original maintenance authority required` (the known launcher defect). No Rust test was run, including the five added Rust tests; every Rust edit is statically verified only.
- `bun repo run @semio-tech/repo-lib:test-package-asset-constants`: the owned Nx invocation exits 1 (`Owned Nx invocation failed`) with no further reason; `bun ./📜️script.ts test package-asset-constants` needs the parent process envelope. The test was therefore run with `bun test` directly.
- Plugin registry `generate` / `check-generated`, `verify taxonomy report`: not run (same build-owner and infra state as in `RESULT-PLUGIN-WAVE0-GENERATORS.md` section 4).
- Typeck-phase privacy (E0616, E0624) is not covered by the differential; the sequence split avoids the question by widening by name.

## 7. Planner outcome after the changes and carve decision

Log: `🗑️generated/pas-plan-all.log` (default planner mode `auto`, tests rewrites on).

| package | plan result |
| --- | --- |
| epw | `schema` crate over budget: lib 57 files / 244,942 B / 2,910 lines, test 66 / 337,348 / 3,187 |
| gif | 8 crates, 18 `ambiguous coherence` problems (GifSnapshot/GifMutation/GifDiff defined in two slices across 87a and 89a) |
| mp4 | `schema` over budget: lib 50 files / 308,782 B / 4,230 lines |
| jpg | `schema` over budget: lib 53 / 254,490 / 3,251, test 62 / 342,854 / 4,004 |
| avi | `schema` over budget: lib 56 / 225,973 / 3,195, test 64 / 273,608 / 3,637 |
| gltf editor | plans clean as its own package: 1 crate, lib 16 / 76,838 / 1,330, test 23 / 84,959 / 1,542 (the editor package has no problems; it is already inside the budget, so there is nothing to carve) |
| gltf schema (model) | `schema` over budget (35 files / 288,071 B) and `diff` (8 files, 5,157 lines) |
| gltf inferences | `schema` crate 358 files / 492,943 B |
| semio | `schema` 92 files / 267,268 B, `schema-2`, a 46-file sqlite test crate (614,896 B test) |
| semio presentation | `schema-2` 45 files |
| semio mesh | `schema-2` 58 files |
| semio brep | `io` 89 files / 492,803 B / 6,161 lines |
| architect-program | `registers` 7 files / 288,630 B / 6,756 lines and `schema` 103 files / 1,027,405 B / 12,334 lines |
| pdf | 20 crates; `schema-3` (the 1.7 base standard) lib 364 files / 2,912,220 B / 38,425 lines and `mutations` (1.4) test 52 files; 20 `ambiguous coherence` problems (PdfSnapshot / PdfMutation / PdfDiff defined in two slices) |
| puzzle-5d | `schema` lib 152 files / 760,926 B / 7,005 lines (test 227 / 1,442,074 / 15,976) and `editor-2` lib 80 files / 896,437 B / 15,208 lines |
| bim-model | planner did not finish within its 1,500 s cap (11,107-file test closure); no result |
| space-home | 6 crates, one problem: `editor` test closure 30 files / 304,593 B / 4,503 lines over the hard test limits (lib 26 / 182,513 / 2,725 fits). The 564-line shared editor unit-test module (48,485 B) and the transient and explore-window tests account for it; splitting that module is the one step from a clean plan |
| sourcing-curation | planner error `Missing or ambiguous Rust module: component`, now located: the planner's measuring mirror (`📦️packages/🦀️carve-measure/🦀️.rs`) mounts the inline wrapper `#[path = "🏅️standards/.../🪟️windows/🔢️grid"] pub mod grid { #[path = "🦀️.rs"] mod component; .. }` with a base that already contains `✏️editor/🎭️modes/✏️edit`, so the nested file resolves to `<carve-measure>/🏅️standards/../✏️edit/🏅️standards/../🔢️grid/🦀️.rs`. The real crate measures fine (lib 123 files, test 183 files); this is a mirror-rendering defect for inline modules that carry a directory `#[path]` (`rewriteMountPaths` throws for those, the wrapper-node path does not), found with a Bun load plugin that printed the candidate list (scratchpad only) |

No package that exceeds the limits plans with `problems: []`, so nothing was carved and no `--verify --oracle --sandbox` run was started. The asset gate is no longer the binding constraint for any of the 12; what remains is the structural class (facade weave, coherence splitting, test-closure slicing) that the other agent is implementing, plus two engine findings for its owner: the curation mirror defect above and the home editor test slice. The gltf editor package is the only one that already fits as a whole (1 crate, lib 16 files / 76,838 B / 1,330 lines), so there is nothing to carve there.

## 8. Descriptor and host work pending the compile window

Regenerate (nothing was regenerated here; no compile is possible):

1. `puzzle` plugin descriptor: the `capsule-dream` example row is now declared as `📚️examples/s.puzzle.5d.1.-/capsule-dream.dsl.semio.digest` (was the authored DSL) and a package asset named by the DSL's repository path is declared by the hub. Hosts deliver it with `package_file_assets` (native) and the browser kernel's `📦️package-assets`.
2. `architect` plugin descriptor: four JSON Schema documents as package assets; the `plugin()` of `🌎️hub/🧩️compositions/🏛️architect` changed shape (builder variable plus loop).
3. The wasm components of both, so the guest installs the package-asset schema reader (architect) and reads the DSL asset (puzzle).
4. No other shipped descriptor changed: gif, pdf, gltf, mp4, epw, jpg, avi, semio are not in the shipped `stdio` plugin (only html, md, csv, tsv, txt, json, xml are).

Order matters on the first regeneration: `describe` does not run deferred producers, so it can write the new `assets` list; the descriptor probe that runs every deferred producer needs those assets delivered, so run it after the descriptor was written (the 5d `every-example` note says the probe runs the producers).

## 9. Residual risks and follow-ups

1. Nothing was compiled or run (section 6). The Rust edits are checked by rustfmt, the resolution differential and reading. The highest-risk ones, in order: the sequence split (107 name-based visibility widenings), the registry reader and its `OnceLock` statics (type-checked), the 5d/gltf `deferred` ports, the architect `Cargo.lock` row.
2. Guest-side resolution timing: JSON Schema leaves of architect resolve through `PackageAssetSchemaLeaves` only after the open event delivered the documents. Every call site of `registered_input_schema_document` and `structural_validator_in` I found is request-time (stdio patch editing, time-travel panel); a registration-time call would see the documents absent. Unverified at runtime.
3. Dependents that request the capsule-dream body without a host (a cargo test of a crate that depends on `semio-s-artifact-puzzle-5d`) now panic with the asset name, because the test fallback exists only for the artifact's own tests. Such a test must call `semio_framework_plugin::reactor::admit_package_assets_for_testing` with the declared asset. I did not find one; the hub `🧪️tests` were not run.
4. `SchemaExportEntries` / `schema_export_catalog_entries` now list TypeScript, GraphQL and proto leaves of the pdf and architect scopes only after a reader is installed; the descriptor functions install it, and there is no production dumper of those entries today (only registry tests).
5. A clean long-term design for registered deferred examples would be a `packaged` example source in the plugin crate (identity and delivery declared once, no sidecar). I did not add it because `plugin/builder/🧩️app` and `🔌️program` are being split by peers today; the sidecar needs no change there. If they stabilize, the sidecar can be replaced mechanically (three leaves).
6. bim-model's test closure is 11,107 files / 27.9 MB: something in its test cfg mounts thousands of files (not investigated); it will dominate any bim carve.
7. The two `bachelor-thesis.pdf` copies (1.4 and 1.7 standard, identical SHA-256) are still both in the repository; dedupe needs the `asset://` resolver of the feature files and was left alone.
8. `module_path!()` strings change for the 20 equation and 6 sequence children (inline module -> out-of-line module of the same name keeps the path, so only the `sequence` children add `base`/`host`/.. segments); the 953 existing uses repo-wide were not reviewed per file.
9. Machine load was 24 to 33 throughout; everything ran niced and sequentially; no `[DEBUG]` code was added.

## 10. Files changed (by area)

- Test-only runtime reads: epw `🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests`; mp4 (5 test files); jpg; avi; gif 89a io unit and dancing example tests; gltf-model sqlite snapshot test; semio (12 mutation-test files, 2 sqlite snapshot test files, 3 subset oracle checks: presentation, mesh, brep); bim sqlite test; pdf 1.4 io unit and 1.7 export unit.
- Asset ports: gif `📚️examples/💃️dancing/🦀️.rs` + unit test; pdf `📚️examples/🎓️bachelor-thesis/🦀️.rs` + example test; gltf `📚️examples/🌱️metabolism/🦀️.rs`, its unit and example tests, new `🖼️assets/🌱️metabolism/🏙️base/🔏️.digest`; 5d `📚️examples/🌙️capsule-dream/🦀️.rs`, its example test, new `🖼️assets/🌙️dream/🔏️.digest`, io snapshot `🦀️.rs` and its unit test, `🧪️tests/🧪️every-example`; hub `🧩️puzzle/🦀️.rs`.
- Schemas: registry `🦀️.rs`, `📦️packages/🦀️rust/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`; architect-program `🧬️schema/🦀️.rs`, new `🧬️schema/🔏️.digest`, `Cargo.toml`, `Cargo.lock`; hub `🏛️architect/🦀️.rs`; pdf 1.7 `🧬️schema/🦀️.rs`; library `🧾️facet-leaves/🟦️.ts`, `🎮️command/🟦️.ts`, `🧪️tests/📦️crate-tree/🟦️.ts`.
- Splits: equation `🌿️cas-internals/🦀️.rs` + 20 new child folders; sequence editor `🦀️.rs` + 6 new child folders.
- Counter defects: pdf `⚖️lopdf-vectors/🦀️.rs`; space-home `🫧️transient/🧪️tests/🔬️unit/🦀️.rs`; bim `🎯️dirty/🧪️tests/🔬️unit/🦀️.rs`; curation `✏️editor/🧪️tests/🔬️unit/🦀️.rs`.
- Law test and registration: `L/🧪️tests/📦️package-asset-constants/🟦️.ts`, `L/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json,package.json}`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`.
- Scratch (kept under `🗑️generated`, prefix `pas-`): `pas-lib.ts`, `pas-emb.ts`, `pas-sites.ts`, `pas-auto.ts`, `pas-rel.ts`, `pas-oracle.py`, `pas-split-cas.ts`, `pas-split-flat.ts`, `pas-split-sequence.ts`, `pas-do-*.ts`, logs `pas-emb-*.log`, `pas-plan-all.log`.
