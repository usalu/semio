# Current Rust Enforcement Audit — 2026-10-01

Read-only source/report audit in the existing ticket. No compiler, test, or production mutation was run by this auditor. Counts below are the supplied completed gate report, not an independently rerun claim. Applicable root, products, Repo, s, and relevant module/plugin AGENTS were consulted where present; no AGENTS changes.

## Completed Gate Evidence

The report inventories **24,077 Rust files and 61,102 authored references**, with **86 strict direction violations and 49 census/resolution problems**. The gate is RED. Passing synthetic owner vectors cannot establish repository cleanliness.

| Cluster | Count | Interpretation |
|---|---:|---|
| stdio plugin root oracle aggregator → artifact subset oracle sources | 72 | Genuine matches of current physical rule; policy-versus-test-composition decision required |
| framework modules → OS product | 7 | One production mount and six product-source/fixture test imports |
| puzzle command tests → artifact editor/fixtures | 4 | Integration ownership in an inner command test |
| space core → draw/writer artifact demos | 2 | Product fixture registration owned too low |
| norm registry contract → artifacts definition | 1 | Artifact descriptor authority owned too high for neutral registry |
| Missing compile input | 37 | 36 stale draw fixture paths plus one absent hub actor fixture |
| Unresolved nested module | 9 | All `#[path = "."] mod shared` wrappers in generated-host adapters |
| Unsupported compile expression | 3 | Proc-macro emitted includes; two finite macro fixture templates |

## Smallest Legitimate Next Cuts

1. **Fix stale draw fixture references as one bounded four-file cut (36 failures).** Style, transform, structure, metadata adapters beneath `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests` still compile fixtures from `<subset>/🧬️schema/🧬️mutations/<kind>/🧪️tests/<case>`. Physical fixtures are now `<subset>/🧫️fixtures/🧬️mutations/<kind>/<case>`. Example transform `🔍️update-layer/🔍️sharpens` and structure `➕️create-layer/➕️appends` before/mutation/after files are present at the latter addresses. Change authored include literals to the real existing fixture paths; preserve vector content. Do not recreate stale directories or exempt census checks. Exact adapter positions appear below.
2. **Move cooperative product-host wiring assertion into the OS plugin owner (one edge).** `🧰️framework/🔨️modules/⏳️async/🤝️cooperative/🧪️tests/🔬️standalone/🦀️.rs:44` reads actual OS plugin source to verify its pump binding. That assertion belongs with its product host; the neutral cooperative tests can retain their runtime behavior laws. This is genuine boundary inversion despite being a test.
3. **Separate neutral fixture authority from OS integration fixtures (five edges).** Replication mutation metadata test `:129`, UI raster test `:14`, UI component test `:168`, and surface paint test `:54,:438` consume product fixtures. Put generic contract vectors with their lower contract and keep host-specific vectors/assertions with OS renderer/derive. A byte-for-byte copy into lower ownership without separating product semantics is not enough.
4. **Remove 2D’s OS engine facade mount (one production edge).** `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/🦀️.rs:13-15` mounts/reexports product `Engine`, `EngineCache`, `EngineFault`, `EngineHandle`, `EngineKey`. Decide which are domain-neutral engine contracts and lower them, or make product consumers import their own owner. Also review its line 7 `pub use semio_framework_os_kernel::os_spr`: this source-path gate does not inspect Cargo/use dependencies, so the one physical mount is not the whole package-level cut.
5. **Move space demo registration above plugin core (two edges).** `✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs:60-61` embeds draw/writer demo assets. A composed demo/studio owner can supply registrations to core through owned interfaces. Avoid replacing physical includes with opaque runtime paths to hide the same ownership.
6. **Resolve norm descriptor ownership (one edge).** `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🦀️.rs:42` mounts the plugin artifact definition directly. Place package descriptor vocabulary in a shared plugin contract if independent artifact packages consume it; actual artifact inventory/composition remains above registry contract.
7. **Move retained editor integration tests outward (four edges).** Puzzle retained command test embeds 3D/5D editor source at `:91`, 2D editor source at `:97`, and 2D retained-job fixture at `:232`. Keep command behavior tests with commands; verify artifact editor binding from the artifact integration owner.
8. **Hub fixture must be found/reowned, not waived.** `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs:76` resolves to `🌎️hub/🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json`, but the `🌎️hub/🔨️modules/🎭️actor` directory is absent. Determine the actual actor lifecycle contract owner, then point the test at its fixture.

## Policy False Positives and Fail-Closed Limits

The **72 stdio edges** all originate `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🦀️.rs`, starting at line 63. Its explicit purpose is a nonproduction test oracle crate composing per-artifact oracles under the `oracles` feature. These are genuine forbidden physical edges under the stated universal rule, but they are not 72 production implementations importing outward. Choose a clean composition boundary: each artifact supplies its oracle package/registration, and a higher test composition owner aggregates them. A blanket oracle/test exemption would also permit lower neutral tests to import product fixtures and is not a legitimate solution. If policy deliberately admits composition roots, define exact schema-governed authority and hostile vectors first.

The **nine `shared` provenance problems** are not demonstrated missing files. Example procedural generation2d adapter line 40 declares an inline `#[path = "."] mod shared`, then mounts neutral stdio law helpers beneath it. Target resolver currently requires a graph context for inline path attributes, even though these adapters are compiled by generated test hosts omitted from authored inventory. The source direction scanner can prove this inline base lexically from the source without inventing Cargo provenance; implement explicit lexical inline context construction and hostile ownership vectors. Do not use workspace root Cargo as substitute context. Verify whether the eventual cross-plugin neutral helper placement itself is correct after resolving targets.

The **three unsupported cases are RED by design, not silent success**. OS derive source line 595 contains `include_str!(#taxonomy_dependency)` inside a `quote!` token template, plus descriptor/payload includes: these are emitted at the derive consumer, not evaluated at the proc-macro definition. Existing mutation-authority evidence should govern the emitted dependency at consumer scope; blindly ignoring every quoted include would allow hostile output dependencies. PDF lopdf-vector macro line 99 uses a literal `$directory` substituted from finite invocations; home transient case line 19 uses `$name`, also finite invocation literals. Support exact template-to-invocation provenance or expand these authored finite fixture declarations explicitly. The scanner rejects the entire source on first unsupported expression, so the count reports three source-level problems rather than every underlying include.

## Root Context and Symlink Review

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:72-79` inventories each present taxonomy area plus top-level `.rs` and Cargo files. Missing taxonomy areas remain permitted, nonphysical area roots rejected. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:8382-8397` creates Cargo contexts only when an inventoried declared target exists. A virtual root `[workspace]` without `[package]` does not create named crate/manifest authority: root Cargo is inventoried for real package context, but there is no universal root fallback. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:94-99` nearest-manifest fallback excludes `.` and therefore cannot grant virtual workspace root provenance. Preserve this property.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:62-69` uses lstat and refuses `.rs`, Cargo, or directory symlinks. Directory detection uses followed stat but does not recurse or read through the target. An irrelevant dangling non-Rust link will currently throw ENOENT while detecting its target type; this is conservative but produces no typed report. Taxonomy and policy bootstrap inputs at `:41-43` are loaded before inventory using readFile/createRequire, without the no-follow ancestry helper; their symlink authority should be reviewed separately. The root path itself/ancestors are not lstat-validated by this scanner, so literal root-symlink refusal is not guaranteed. Standard no-follow helpers already exist at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts:32-86`; reuse owned authority instead of adding another scanner-specific partial safeguard.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:84,91` drops all successfully parseable references in a source when a later unsupported expression throws. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts:44` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:100-102` similarly stop source processing on the first unresolved target, potentially suppressing other real violations in that file. These are **diagnostic completeness blind spots, not green escapes**, because a typed problem keeps verification red. A complete typed report should collect per-reference problems while retaining other valid references. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:8382-8387` records invalid Cargo manifests but current executor does not request strictManifests or report invalidManifests; malformed real-package declarations should not confer owner authority. Keep virtual roots deliberately nonauthoritative rather than reporting every workspace-only manifest invalid.

## Exact Report Source Clusters

### Violations

- `🧰️framework/🔨️modules/⏳️async/🤝️cooperative/🧪️tests/🔬️standalone/🦀️.rs` — 1; 44
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/🦀️.rs` — 1; 13
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🔬️mutation-leaf-metadata/🦀️.rs` — 1; 129
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs` — 1; 14
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️component-unit/🦀️.rs` — 1; 168
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs` — 2; 54, 438
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🦀️.rs` — 1; 42
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🦀️.rs` — 72; 63, 81, 99, 117, 135, 153, 171, 179, 187, 205, 217, 235, 253, 271, 283, 301, 319, 335, 343, 349, 355, 361, 373, 391, 397, 415, 433, 451, 469, 487, 505, 523, 531, 539, 551, 559, 567, 575, 583, 591, 599, 617, 635, 653, 661, 669, 687, 705, 713, 719, 725, 731, 737, 743, 749, 767, 785, 791, 797, 815, 821, 839, 857, 875, 893, 901, 909, 927, 933, 951, 957, 989
- `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs` — 4; 91, 97, 232
- `✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs` — 2; 60, 61

### Problems

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` — 1; Unsupported Rust compile expression at line 595: include_str
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/⚖️lopdf-vectors/🦀️.rs` — 1; Unsupported Rust compile expression at line 99: include_bytes
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🦀️.rs` — 1; Unsupported Rust compile expression at line 19: include_str
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌀️mutate-procedural-2d-1/🦀️.rs` — 1; Rust source dependency requires module provenance: ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌀️mutate-procedural-2d-1/🦀️.rs:40; module=shared
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️mutate-procedural-3d-1/🦀️.rs` — 1; Rust source dependency requires module provenance: ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️mutate-procedural-3d-1/🦀️.rs:40; module=shared
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/💠️mutate-lowpoly-1/🦀️.rs` — 1; Rust source dependency requires module provenance: ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/💠️mutate-lowpoly-1/🦀️.rs:40; module=shared
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📐️mutate-cad-1/🦀️.rs` — 1; Rust source dependency requires module provenance: ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📐️mutate-cad-1/🦀️.rs:40; module=shared
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🎨️mutate-drawing-1-any-style/🦀️.rs` — 12; 38, 39, 40, 43, 44, 45, 48, 49, 50, 53, 54, 55
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔀️mutate-drawing-1-any-transform/🦀️.rs` — 9; 38, 39, 40, 43, 44, 45, 48, 49, 50
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-drawing-1-any-structure/🦀️.rs` — 12; 38, 39, 40, 43, 44, 45, 48, 49, 50, 53, 54, 55
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪪️mutate-drawing-1-any-metadata/🦀️.rs` — 3; 38, 39, 40
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/◻️mutate-puzzle-2d-1/🦀️.rs` — 1; Rust source dependency requires module provenance: ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/◻️mutate-puzzle-2d-1/🦀️.rs:44; module=shared
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🖐️mutate-puzzle-5d-1/🦀️.rs` — 1; Rust source dependency requires module provenance: ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🖐️mutate-puzzle-5d-1/🦀️.rs:40; module=shared
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️mutate-puzzle-3d-1/🦀️.rs` — 1; Rust source dependency requires module provenance: ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️mutate-puzzle-3d-1/🦀️.rs:40; module=shared
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-block-5d-1/🦀️.rs` — 1; Rust source dependency requires module provenance: ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-block-5d-1/🦀️.rs:40; module=shared
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-block-3d-1/🦀️.rs` — 1; Rust source dependency requires module provenance: ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-block-3d-1/🦀️.rs:40; module=shared
- `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs` — 1; 76

