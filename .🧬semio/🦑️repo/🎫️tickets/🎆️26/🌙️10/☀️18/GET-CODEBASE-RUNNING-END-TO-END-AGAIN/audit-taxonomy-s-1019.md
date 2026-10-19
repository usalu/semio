# 🧭️ Audit: taxonomy findings in the `✏️s` tree (aud-tax, 2026-10-19)

Read-only audit of the 22,393 `✏️s` findings that remain after `result-tax.md`. No source file was edited. Nothing was committed, staged or stashed.

Abbreviation: every path below starts with `✏️s/🔌️plugins/` unless shown otherwise. `…` marks elided middle segments; the leaf is exact.

## 1. How the numbers were produced

- Command: `bun C/tax-tools/inventory.ts <scope> <out.json>` (the per-scope inventory `tax` used, `inventoryTaxonomy`), run once per `✏️s` sub-scope, each under `perl -e 'alarm 540'`. Driver: `C/🗑️generated/aud-tax-run.sh`. Scope list: `C/🗑️generated/aud-tax-s/scopes.txt` (69 scopes: 39 plugins, 4 modules, 4 test scopes, 29 dev scopes, plus `🔌️plugins/🏙️bim` re-run).
- Slowest sub-scope: `🗄️stdio` 153 s (49,511 entries). Every sub-scope finished with rc=0.
- Sum over sub-scopes: **22,393 violations** (`result-tax` reports 22,395 for the whole `✏️s` scope in one run; a 2-finding difference, not reconciled).
- Outputs: `C/🗑️generated/aud-tax-s/inv-*.json|log` (per scope), `aud-tax-all-violations.json` (flat list), analysis scripts `aud-tax-analyze.mjs`, `aud-tax-patterns.mjs`, `aud-tax-lengths.mjs`, `aud-tax-breakdown.mjs`, `aud-tax-unpaired.mjs`, `aud-tax-unpaired-impl.mjs`, `aud-tax-unrealized.mjs`, `aud-tax-examples.mjs`.
- Package-form run for the other top-level scopes (framework 111 s, hub 52 s, mit-bestand 34 s, semio-tech 32 s, teaching 54 s, leutwiler, fixtures, tests, schema): `C/🗑️generated/aud-tax-pkg/`. Analysis: `aud-tax-pkgforms.mjs`.

## 2. Findings by validator code (`✏️s`, 22,393)

| code | count | root-cause class | mechanical fix? |
| --- | ---: | --- | --- |
| `path-too-long` | 15,742 | policy constant (unit and limit) amplified by naming depth | policy: yes (one constant); naming: no |
| `mutation-fixture-unpaired` | 3,148 | mixed: missing or empty implementation cases, orphan fixtures | no (per case), except orphan and empty-dir deletes |
| `mutation-pair-path-budget` | 1,163 | policy constant (same 240-byte budget) plus `pathBudgetReserve` | same as `path-too-long` |
| `projection-catalog-unrealized` | 821 | generated registry out of sync with physical fixtures | yes, once the generator is rerun and its source of truth is chosen |
| `mutation-payload-schema-authority-invalid` | 550 | real per-owner defect (missing or wrong descriptor file) | semi: needs a descriptor template per owner |
| `fixed-source-disposition-unresolved` | 229 | real code defect (root `📜️script.ts` shape vs the router law) | no |
| `projection-catalog-coverage` | 223 | generated registry out of sync (inverse direction) | yes, with the same generator rerun |
| `mutation-fixture-invalid` | 122 | real per-case defect (scenario root absent, node count) | no |
| `projection-member-unresolved` | 109 | registry naming: member identity missing or not unique | yes (registry rows) |
| `semantic-stem-unresolved` | 96 | taxonomy: JSON data stems not registered as kinds | yes, once the kind is chosen |
| `projection-authority-invalid` | 35 | real defect in the Draw editor command bundles | no |
| `package-implementation-file` | 50 | Rust and TS package-form (see section 4) | no |
| `taxonomy/kind-only-basename` | 33 | naming: `🟦️.test.ts`, `🗄️.ts`, `🟨️.d.ts` leaves | yes (rename plus import re-point) |
| `mutation-implementation-invalid` | 18 | real per-case defect (bundle node count) | no |
| `file-kind-unresolved` | 15 | taxonomy: no file kind for `.epJSON`, `.digest`, `.icc`, `.scan`, `.DS_Store` | yes (kind rows) for most; `.DS_Store` is a real defect |
| `package-role-unresolved` | 13 | grammar gap (Rust) | no (grammar work) |
| `opaque-reference-target` | 10 | real defect: fixtures reference excluded `compose` and `mit-bestand` paths | no |
| `path-emoji-multiple` | 6 | naming (two emoji on one file) | yes (rename) |
| `directory-kind-unresolved` | 5 | taxonomy (`🗟️artifacts`, `▭️profile`) | yes (kind rows) or rename |
| `package-implementation-destination-unresolved` | 2 | naming or code (no owner for package file) | no |
| `path-emoji-presentation` | 2 | naming (emoji presentation) | yes (rename) |
| `projection-catalog-invalid` | 1 | generated registry | with the generator rerun |

Per-plugin concentration: `🏙️bim` 5,892, `🔋️energy` 4,270, `🏛️architect` 4,111 (together 64% of the total). `🗄️stdio` 1,749, `📕️norm` 1,386, `🏗️fem` 831, `🧩️puzzle` 830. The 29 `✏️s/🧑‍💻dev/*` scopes, the 4 `✏️s/🧪️tests/*` scopes and the 4 `✏️s/🔨️modules/*` scopes carry 0 to 1 finding each.

## 3. Top 10 patterns in `✏️s`

Each row is one root cause. Count is within `✏️s`.

### 3.1 `path-too-long`, 15,742: policy unit plus naming depth

- Root cause: `collisionPolicy.maxPathBytes = 240` in `🔣️taxonomy.json` (line 30650), measured in UTF-8 bytes by `pathPolicyViolations` (`🧹️normalization/👥️mutation-projection/🟦️.ts:31`). AGENTS.md says 256 characters. The emoji-heavy names cost 3 to 4 bytes per glyph, so the byte budget bites while the character count stays low.
- Measured on the 15,742 findings: UTF-8 bytes 241 to 314 (p50 249). UTF-16 units 165 to 226. Code points 150 to 207. None exceeds 256 characters.
- Remaining after a change of budget: at 256 bytes 4,021 remain; at 280 bytes 281; at 300 bytes 47. At 256 UTF-16 units, 0 remain.
- Naming amplifier: 11,378 of the 15,742 sit in the three model plugins (`🏙️bim` 4,297, `🔋️energy` 3,643, `🏛️architect` 3,438). They share the depth `🗿️artifacts/<model>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/<mut>/<case>/📸️snapshot/⬅️before/🔣️.json` (13 path segments below the plugin directory).
- Examples:
  1. `🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔠️remove-family-parameter/✅️removes-an-unused-parameter/📸️snapshot/⬅️before/🔣️.json`: "Path exceeds 240 UTF-8 bytes"
  2. `🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧪️tests/🧫️fixtures/🧬️mutations/💿️buffer/🗑️delete/🔬️middle-row/📸️snapshot/⬅️before/🔣️.json`: "Path exceeds 240 UTF-8 bytes"
  3. `🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📋️register-row/🗑️delete/🏁️benchmark-record-absent-a/🦠️mutation`: "Path exceeds 240 UTF-8 bytes"
- Single fix: policy. Measure in UTF-16 units against 256 (the AGENTS.md rule) and state the checkout-root allowance explicitly. A 226-unit repo-relative maximum plus a long Windows checkout prefix can still exceed 260 for absolute paths, so the budget must be derived, not guessed. Naming (dropping the `🪆️subsets/✳️any` level) would also help but is not needed if the policy changes.
- Mechanical: policy change yes (constant, default argument at `👥️mutation-projection/🟦️.ts:19`, reserve values in `🔣️taxonomy.json`). Naming no (thousands of renames through the normalization transaction).

### 3.2 `mutation-fixture-unpaired`, 3,148: four sub-causes

Pairing rule (`👥️mutation-projection/🟦️.ts:570`): every fixture bundle `…/🧪️tests/🧫️fixtures/🧬️mutations/<mut>/<case>` needs an implementation case that `projectionSourceAt` accepts. Split of the 3,148 by what exists on disk:

- 2,077: schema mutation directory exists, no `🧪️tests/<case>` implementation. Unpaired findings by plugin, all sub-causes: `🏙️bim` 1,576, `📕️norm` 648, `🏛️architect` 329.
- 747: the same-named implementation case exists on disk but is an empty directory. Git does not track empty directories, so the pairing never sees it. These are scaffolding leftovers.
- 126: the implementation case exists and holds only one `🦀️.rs` stub. The pairing contract does not accept a lone Rust file as an implementation case.
- 198: no schema mutation directory at all. Orphan fixtures (e.g. `🔋️energy/…/🧬️mutations/☔️change-humidistat/⛔️refuses`).
- Examples:
  1. `🏙️bim/…/🧪️tests/🧫️fixtures/🧬️mutations/🔠️remove-family-parameter/❌️missing`: "Fixture bundle has no canonical implementation case" (the schema side `🧬️schema/🧬️mutations/🔠️remove-family-parameter/` has no `🧪️tests/`)
  2. `➗️mathematical/…/🧪️tests/🧫️fixtures/🧬️mutations/🎚️change/🧪️raises`: same message (implementation case `🧬️schema/🧬️mutations/🎚️change/🧪️tests/🧪️raises` holds only `🦀️.rs`)
  3. `➗️mathematical/…/🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds`: same message (implementation case directory is empty)
- Single fix: none. Per case, either author the implementation case or delete the fixture. Orphans (198) and empty scaffolding (747) are the only candidates for deletion without a decision, and the empty-directory half can be removed mechanically once the owner confirms.
- Mechanical: no for 2,077 and 126; partly for 198 and 747.

### 3.3 `mutation-pair-path-budget`, 1,163: same policy constant

- Root cause: the same 240-byte budget, applied to the canonical pair plus a reserved descendant suffix (`pathBudgetReserve`, e.g. 133 bytes for cad). Largest leaves: `⛔️refuses` 286, `✅️applies` 151.
- Plugins: `🔋️energy` 580, `🏗️fem` 121, `📸️remodel` 109, `🧩️puzzle` 51.
- Examples:
  1. `🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🧪️tests/✂️severs-edge-ab`: "Canonical mutation pair plus reserved descendant suffix exceeds 240 bytes"
  2. `🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌱️change-ground-shallow/🧪️tests/⛔️refuses`: same
  3. `📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/🔎️switches`: same
- Single fix: the same policy change as 3.1. Not recomputed here; re-run the scoped inventory after the change.
- Mechanical: yes, with 3.1.

### 3.4 `projection-catalog-unrealized`, 821: registry ahead of disk

- Root cause: the oracle catalog `🔮️oracles/🔣️.json` registers vectors whose physical bundles do not exist. Checked: 819 of 821 registered scenario IDs have no directory of that name anywhere in their plugin (the other 2 exist). The path length is not the cause (no hit exceeds 240 bytes).
- Plugins: `🏛️architect` 329, `🏗️fem` 168, `🗄️stdio` 105, `🧩️puzzle` 69.
- Examples:
  1. `🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json`: "Registered vector change-seed/🎲️change-seed/reseeds-the-solve-from-7-to-99 has no physical bundle"
  2. `🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json`: "Registered vector delete-register-row/🗑️delete/change-record-deletes-middle has no physical bundle"
  3. `📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json`: "Registered vector remove-zone/➖️remove-zone/apply has no physical bundle"
- Single fix: regenerate the oracle catalog from the physical fixtures, or materialize the missing bundles. This is a generator decision, not a text edit.
- Mechanical: yes, once the source of truth is chosen and the generator is rerun. Not run in this audit.

### 3.5 `mutation-payload-schema-authority-invalid`, 550: missing descriptor per owner

- Root cause: 505 "Mutation owner requires one canonical regular-file descriptor": the mutation owner directory has no single canonical descriptor file. Example: `…/🚪️io/📝️text/🧬️mutations/🔀️move-page` holds only `🦀️.rs`. 42 more: "Descriptor version and semantic identity must belong to the exact source owner" (version naming, e.g. standard `4️⃣1.4`).
- Plugins: `🗄️stdio` 391 (pdf and text mutation families), `📕️norm` 34, `🔱️trinity` 25.
- Examples:
  1. `✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/✂️splice-text`: "Mutation owner requires one canonical regular-file descriptor"
  2. `🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🔀️move-page`: same
  3. `🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/📥️insert-page`: same
- Single fix: code (add the descriptor per owner, one template) plus naming (align the version segment with the descriptor identity).
- Mechanical: semi, after a template and the canonical descriptor name are chosen.

### 3.6 `fixed-source-disposition-unresolved`, 229: root scripts vs the router law

- Root cause: `📜️script.ts` bodies rejected by `ecmaCommandRouterModule` (one routing terminal, registered classes only). Result-tax open item 1 counts 495 repo-wide. Here 229 in `✏️s` (`🗄️stdio` 101, `🏗️fem` 10, `🀄️wfc` 7).
- Examples:
  1. `✒️writer/🏭️bridge/📜️script.ts`: "Fixed source contract root-script rejected the body with its declared semantic validator"
  2. `🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state/📜️script.ts`: same
  3. `🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/📜️script.ts`: same
- Single fix: code. Move the logic into modules and keep the root script a thin router.
- Mechanical: no.

### 3.7 `projection-catalog-coverage`, 223: physical mutation without a registry row

- Root cause: the inverse of 3.4. Physical mutation directories such as `🗑️delete-load-case` or `✍️paint-input-stroke` have no exact vector row. `🏗️fem` 117, `🗄️stdio` 42, `📏️layout` 27.
- Six paths carry a test-looking segment `🚨️no-such-case-ef1fde` (findings under `path-too-long`, `mutation-pair-path-budget` and this code). They look like leftovers from a test run. Not verified.
- Examples:
  1. `🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✍️paint-input-stroke/🧪️tests/⚠️clips`: "Physical mutation ✍️paint-input-stroke does not have an exact one-to-one vector registry"
  2. `🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🚨️no-such-case-ef1fde`: same message
  3. `📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️scale-frames/🧪️tests/⏸️keeps-unit-factors`: same
- Single fix: the same generator rerun as 3.4, plus deleting the six `no-such-case` leftovers after confirmation.
- Mechanical: yes with 3.4.

### 3.8 `mutation-fixture-invalid`, 122: scenario root absent

- Root cause: 119 fixture bundles have no scenario root for their case (e.g. `🚚️swaps`, `🔀️flips`, `🔗️binds`). Three more have 16 nodes where 12 are expected. Per-case defect.
- Plugins: `🗄️stdio` 63, `🧩️puzzle` 52.
- Examples:
  1. `🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧲️adjacency/🧲️connect/🧲️reception`: "scenario root is absent"
  2. `🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧪️tests/🧫️fixtures/🧬️mutations/💎️material/🚚️move/🚚️swaps`: same
  3. `🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️selection/🔄️rotate-selection/⛓️orbits`: same
- Single fix: code (restore or remove each case).
- Mechanical: no.

### 3.9 `projection-member-unresolved`, 109: registry identity

- Root cause: mutation member names (`✂️disconnect`, `🖋️create-character-style`, `📣️set-declaration`, `🧩️change-extensions`) have no unique canonical registry identity. Registry rows are missing or duplicated under emoji-fold.
- Plugins: `🗄️stdio` 38, `📏️layout` 21, `🎥️shooting` 8.
- Examples:
  1. `🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect`: "Mutation member ✂️disconnect has no unique canonical registry identity"
  2. `📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️create-character-style`: same
  3. `🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📣️set-declaration`: same
- Single fix: registry rows (one per member, unique under fold).
- Mechanical: yes.

### 3.10 `semantic-stem-unresolved`, 96: JSON data stems not registered

- Root cause: data files named `🔣️typology.json`, `🔣️modelDefinition.json`, `🔣️collector.json` under `🏗️modelDefinitions` and `🗂️typologies` have no registered directory kind. `📐️cad` 72, `🗄️stdio` 8, `🀄️wfc` 5. Result-tax open item 5 counts 306 more repo-wide.
- Examples:
  1. `📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building/🎬️actions/a01.json`: "Semantic stem has no registered directory kind"
  2. `📐️cad/…/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🧱️Slab/🔣️typology.json`: same
  3. `🀄️wfc/🗿️artifacts/◻️2d/🏭️bridge/🔣️collector.json`: same
- Single fix: policy (register these stems as a data kind in the taxonomy).
- Mechanical: yes once the kind is chosen.

### 3.11 Rest of the list (not top 10)

`projection-authority-invalid` 35 (Draw editor command bundles: "Draw command source must match exactly one owner vector", real defect); `taxonomy/kind-only-basename` 33 (`🟦️.test.ts` 16, `🗄️.ts` 11, `🟨️.d.ts` 1; rename to `🟦️.ts` after import re-point; the 16 test files also need the vitest include change); `mutation-implementation-invalid` 18 (bundle node counts, real per-case defect); `file-kind-unresolved` 15 (`⚡️model.epJSON` 4, `🔏️.digest` 3, `.DS_Store` 2); `package-role-unresolved` 13; `opaque-reference-target` 10; `path-emoji-multiple` 6; `directory-kind-unresolved` 5; `path-emoji-presentation` 2; `package-implementation-destination-unresolved` 2; `package-implementation-file` 50; `projection-catalog-invalid` 1.

## 4. Rust, Go and TypeScript package-form findings

Repo-wide (framework, hub, mit-bestand, semio-tech, teaching, fixtures, tests, schema, leutwiler, plus `✏️s`): **712** package-form findings, which matches result-tax's "666 warnings and 33 role-unresolved" (616 + 50 implementation-file, 20 + 13 role-unresolved, 11 + 2 destination-unresolved).

| pattern | count (repo) | root cause | mechanical? |
| --- | ---: | --- | --- |
| P1. Go platform sub-package `📦️packages/🐹️go/<platform>/🐹️.go` must move to the parent `🐹️.go` | 268 | policy: the glue wants one `🐹️.go` per member directory, but Go platform packages (`🐚️unix-durability`, `🪟️windows-durability`) are legitimate sub-packages under `📦️packages/🐹️go/go.mod`. Both platform files map to the same destination `🛡️durability/🐹️.go`, so a move cannot succeed. | no |
| P2. Rust crate `📦️packages/🦀️rust/<crate>/🦀️.rs` (crate has `Cargo.toml`) must move to the owner `🦀️.rs` | 218 (43 in `✏️s`) | policy: the crate roots are real crates; the glue treats them as misplaced. Examples in `✏️s`: `🀄️wfc/…/🧬️schema/🔺️diff/📦️packages/🦀️rust/🪞️mirror/🦀️.rs` → `🔺️diff/🦀️.rs`. | no |
| P3. TypeScript package file `📦️packages/🟦️typescript/…/🟦️.ts` must move to the owner | 143 (6 in `✏️s`) | policy and placement: mostly test harnesses (`🧪️tests/🪶️sqlite/🟦️.ts`) inside packages. Example: `🌍️gis/…/🏔️gisterrain/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts`. | no |
| P4. Non-code assets or state inside package directories | 37 (`.css` 15, `.json` 10, `.html` 3, `.tsx` 2, `.wit` 2, `.svg` 1, `.mts` 1, `.bin` 1, `.db` 1, extensionless 1) | mixed: assets belong at the owner (mechanical move with reference re-point). Two real defects: tracked runtime state `🌎️hub/📦️packages/🦀️rust/.semio/hub/directory.db` and `…/db/catalog/root.bin` (result-tax item 7). | assets: yes with re-point; runtime state: no (untrack, decide) |
| P5. Role unresolved: Rust 18, Go 10, TS 2, `.csproj` 2, `.cjs` 1 | 33 | grammar gap, not invalid source. Rust: the classifier returns "unsupported Rust package form" after stripping `#[…]`, `use`, `mod x;`; the sampled files are the generated `🪞️mirror` trees (`#[path]` inline module nests). Go: "unbalanced lexical structure" on `🗂️files-plan/🐹️.go`, which `gofmt -e` parses with exit 0. `.csproj` is outside the grammar. | no (grammar work with tests) |
| P6. Package file with no deterministic owner (`📋️executors.json`, `packages.lock.json`, `regenerate_example_assets.rs`, `next.config.mjs`, `command-text-transfer/input`) | 13 | naming or code: bespoke tooling files in package dirs. | no |

Single decision for P1 to P3: the glue grammar must accept the package layouts that already exist (Go sub-packages with `go.mod`, Rust crates with `Cargo.toml`, TypeScript test harnesses), or the code must leave `📦️packages`. Moving files by rule breaks builds (P1 collides) and is not a mechanical fix.

## 5. Generated-file classification

- The fixture leaves `📸️snapshot/⬅️before/🔣️.json`, `➡️after/🔣️.json`, `🦠️mutation`, `🔺️diff`, `🎯️outcome` make up most of the path budget. They are derived from the mutation runs. If they are generated outputs, the budget should not apply to them (not verified; no generator was run).
- The oracle catalog `🔮️oracles/🔣️.json` and the Rust `🪞️mirror` tree appear to be generated and checked in (not verified). Their findings (3.4, 3.7, P5) are drift between a generator and the tree.

## 6. Not run / open

- The generator that rebuilds the oracle catalog (3.4, 3.7) was not run. The mutation pairing (3.2) was checked by directory listing, not by executing `projectionSourceAt`.
- `path-too-long` was not re-measured under a changed budget through the validator; the length numbers are from the findings' own paths.
- The 6 `no-such-case` leftovers were not confirmed as test artifacts.
- Nothing edited, committed or deleted in the repo. No `bun test`, cargo build or activation was run.
