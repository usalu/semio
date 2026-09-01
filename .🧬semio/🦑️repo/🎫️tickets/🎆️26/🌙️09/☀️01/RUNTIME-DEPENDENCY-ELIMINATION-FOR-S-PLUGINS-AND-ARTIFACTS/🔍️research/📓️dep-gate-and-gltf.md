# Dependency gate honesty + gltf runtime removal

Slice: measurement layer (`dependencyClassifyOracleEntry` in root `📜️script.ts`) plus the
`rust:gltf` production violation in `🧰️framework/📦️packages/🦀️rust` and
`🧰️framework/🔨️modules/🔺️mesh-engine`.

## Baseline (commit aad3d81959, before this slice)

```
target=0, current=163, oracle-conflicts=18, toolchain-owner-conflicts=0, toolchain-failures=0
```

## Classifier changes (slice a) — 📜️script.ts

Three separate, independently-justified fixes to `dependencyClassifyOracleEntry` /
`dependencyParseCargoToml`, all near line 17750-18415:

1. **Widened test-domain path regex.** Old: `/(?:^|\/)🧪️(?:oracle|test)\//u`. New
   `DEPENDENCY_TEST_DOMAIN_PATH_RE`: adds `🔬️probes`, `🏭️generator`, `🧫️fixtures` as recognised
   test-domain path segments, alongside the pre-existing `🧪️oracle`/`🧪️test`. Justification: these
   are established repo conventions (confirmed by reading manifests, not assumed) — `🏭️generator`
   crates exist specifically to build fixtures with the registered reference crate ("never with
   this repository's own codec", per e.g.
   `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🏭️generator/🦀️engine/Cargo.toml`),
   `🔬️probes` crates are oracle probes with their own standalone `[workspace]` (never a workspace
   member, never a path-dependency target of any production crate — verified by grep across every
   `Cargo.toml` referencing these three directory names: zero production `path =` references
   found), and `🧫️fixtures` crates are `role = "testkit"` fixture-only crates (verified via
   `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️scale/📦️packages/🦀️rust/Cargo.toml`'s own
   `[package.metadata.semio] role = "testkit"`). Abuse check: the regex requires an exact
   `/segment/` path match (bounded by `/` both sides, same shape as the pre-existing pattern), so a
   production crate cannot dodge the gate by embedding e.g. `🏭️generatorish/` in its path, and no
   production manifest anywhere references a `🏭️generator`/`🔬️probes`/`🧫️fixtures` crate as a
   `path =` dependency (confirmed by grep — the only 🧫️fixtures path-dependency found is
   `semio-framework-os-scale-fixture`, itself `role = "testkit"`, wired only as a workspace member).

2. **Kind-aware, not just path-aware.** Old: any user whose manifest path isn't test-domain counts
   as a "product user", regardless of which `Cargo.toml` section declared the dependency. New:
   `dependencyClassifyOracleEntry` now inspects `entry.declarations` (per-manifest, per-section
   kind) and only counts a declaration as a real conflict when it is BOTH outside the test-domain
   path AND kind `production-runtime`/`production-build`. Justification: the master ticket's own
   "Definition of done" states a third-party dependency is compliant when kept in
   `[dev-dependencies]`/`devDependencies` from *any* location, not only from a test-domain
   directory — the old path-only check would have kept flagging `gltf` in
   `semio-framework-mesh-engine`'s `[dev-dependencies]` as a conflict purely because that crate's
   own root isn't under a `🧪️`/`🔬️`/`🏭️`/`🧫️` segment, even after correctly moving it there. This
   is not hypothetical: it was **already miscounting today** — `manifold-3d` is declared only in
   root `package.json`'s `devDependencies` (repository-tooling kind) yet was flagged as an
   oracle-conflict before this fix, purely on path. Abuse check: this can't be used to smuggle a
   production dependency past the gate — a genuine `[dependencies]`/production entry still carries
   kind `production-runtime`/`production-build` regardless of which directory it lives in, so it is
   still caught.

3. **Proc-macro crates' `[dependencies]` reclassified `production-build`, not
   `production-runtime`** (flagged mid-task by the coordinator). New
   `dependencyCargoTomlIsProcMacro(content)` detects `[lib] proc-macro = true` in the SAME
   manifest; `dependencyParseCargoToml`'s `kindFor` then routes that crate's `[dependencies]`
   section to `production-build` instead of `production-runtime` — a proc-macro crate's deps are
   compiler plugins, linked into the compiler at build time, never into the target binary.
   Verified 6 manifests repo-wide declare `[lib] proc-macro = true` (grepped every `Cargo.toml` for
   `proc-macro = true`): 5 framework derive/macro crates
   (`🔀️dispatch`, `🔄️machine/✨️derive`, `🧬️schema/✨️derive`, `⏳️async/✨️macros`,
   `🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive`) and one s-plugin macro crate
   (`✏️s/🔌️plugins/🖍️draw/…/🔄️fsm/✨️macros`, the one the coordinator named). All six declare `syn`,
   `quote`, `proc-macro2` in `[dependencies]`; the `dsl/✨️derive` crate also declares `serde_json`
   there. All were the ONLY declarers of `syn`/`quote`/`proc-macro2` repo-wide (grep confirms no
   other manifest touches those three names), so this reclassification is total for those three
   entries. Abuse check: only a manifest whose OWN `[lib]` table sets `proc-macro = true` gets the
   reclassification — a normal crate cannot claim it without actually becoming a proc-macro crate
   (which would then fail to compile as anything else).

## Effect on the numbers, isolated per change

| stage | current= | oracle-conflicts= | rust production-reachable |
|---|---|---|---|
| baseline (aad3d81959) | 163 | 18 | 97 |
| + regex widening + proc-macro kind fix (2a) | 163 | 8 | 87 |
| + kind-aware oracle-conflict filter + gltf moved to dev-only (2a refinement + slice b) | 163 | 6 | 85 |

**`current=163` did not move at all**, at any stage, and this is expected, not a bug: that number
counts every distinct `${ecosystem}:${name}` third-party identity anywhere in the repo (production,
dev, build, tooling — kind-blind), per `dependencyTruthReportFromEntries`. A name only drops out of
that count when EVERY declaration of it anywhere in the whole repo disappears — reclassifying
declarations, or moving one crate's dependency from `[dependencies]` to `[dev-dependencies]`,
changes `oracle-conflicts`/`kind-census`/`production-reachable`, never `current=`. Do not read a
future `current=163→N` drop as proof of a *reclassification* fix — only an actual full removal of a
package name from every manifest in the repo moves that number.

**`oracle-conflicts` 18→8 is entirely change 1+3** (regex widening reclassified 10 entries —
`csv`, `dxf`, `gif`, `las`, `lopdf`, `quick-xml`, `riff`, `ruststep`, `tiff`, `tobj` — from
mixed-conflict to fully `test-oracle`; the proc-macro fix moved `syn`/`quote`/`proc-macro2` off
`production-runtime` but those three were never oracle-registered so they don't appear in the
oracle-conflict list at all — their effect is visible only in kind-census, see below).

**`oracle-conflicts` 8→6 is change 2 (kind-aware filter, a genuine RECLASSIFICATION) removing
`manifold-3d`, combined with slice b (a genuine REMOVAL) removing `gltf`.** These are NOT the same
kind of fix and must not be conflated: `manifold-3d` was never a real production leak (it was
always `devDependencies`-only; the old classifier was simply wrong about it). `gltf` WAS a real
production leak in two framework crates before this slice, and is now actually gone from
`[dependencies]` everywhere — see below.

**rust production-reachable 97→87→85** (`kinds.some(production-runtime|production-build)`,
summed per-ecosystem in the `verify dependencies literal-external` table's last column) is the
metric that actually tracks "is this genuinely still linked into something built as production" —
97→87 came from the 10 regex-reclassified oracle entries plus `syn`/`quote`/`proc-macro2` no longer
counting as reachable in that sense; 87→85 is `gltf` (flips fully to `test-oracle` once its only
declarers are the probe and mesh-engine's now-dev-only entry) plus general noise — **NOTE:** other
agents on this same ticket were live-editing `Cargo.toml`/`package.json` files concurrently for
their own waves (W1/W3/W4/W8) throughout this session (confirmed: `js:brepjs`'s and `js:three`'s and
`rust:image`'s and `rust:png`'s oracle-conflict USER LISTS visibly shrank between my two
measurement runs, dropping `✏️s/🔌️plugins/🧩️puzzle`, `✏️s/🔌️plugins/📐️cad`, `✏️s/🔌️plugins/🎞️animate`,
`✏️s/🔌️plugins/🖍️draw`, `✏️s/🔌️plugins/💠️lowpoly`, `✏️s/🔌️plugins/📸️remodel` production entries —
none of which I touched). So the 87→85 two-point drop cannot be cleanly attributed 100% to gltf
alone without a diff of the full JSON at both timestamps; gltf's own entry is confirmed to flip
`productionReachable: true→false` (one point), the second point is most plausibly a concurrent
sibling agent's unrelated fix landing in the same window, not mine.

## Per-case verdict, all 18 baseline oracle-conflicts

| package | baseline conflicting users (non-test-domain) | verdict | now |
|---|---|---|---|
| `js:brepjs` | `📐️cad`, `🧩️puzzle` package.json (production `dependencies`) | genuinely production (W8, JS→`@semio-tech/*`) — NOT mine to fix | `🧩️puzzle` cleared by a concurrent agent; `📐️cad` still open |
| `js:manifold-3d` | root `package.json` (but `devDependencies`!) | **classifier bug** — was never production | cleared (kind-aware fix) |
| `js:three` | `📐️cad`, `🧩️puzzle`, + 3 framework production | plugin entries genuinely production (W8); framework entries are framework's own choice (goal only requires zero under `✏️s/`) | plugin entries cleared by a concurrent agent; framework entries remain (expected — not the goal's target) |
| `rust:csv` | probe + generator only | **test-domain, reclassified** | cleared |
| `rust:dxf` | generator + 3 probes | **test-domain, reclassified** | cleared |
| `rust:gif` | 2 probes + generator | **test-domain, reclassified** | cleared |
| `rust:gltf` | probe + `🧰️framework/📦️packages/🦀️rust` + `🔺️mesh-engine` (both PRODUCTION `[dependencies]`) | **genuinely production — THE slice-b target** | **fixed**: both moved to a first-party codec; `gltf` survives only as `🔺️mesh-engine`'s `[dev-dependencies]` oracle |
| `rust:image` | `🎞️animate`, `🖍️draw` (plugin production) + 2 generators + 3 framework production | plugin entries genuinely production (W1, not mine); framework entries are framework's own | plugin entries cleared by a concurrent agent; framework entries remain |
| `rust:las` | generator only | **test-domain, reclassified** | cleared |
| `rust:lopdf` | many generators + 1 probe | **test-domain, reclassified** | cleared |
| `rust:png` | `💠️lowpoly`, `📸️remodel` (plugin production) + generator + framework | plugin entries genuinely production (W1, not mine); framework entry is framework's own | plugin entries cleared by a concurrent agent; 1 framework entry remains |
| `rust:quick-xml` | 2 probes + 4 generators | **test-domain, reclassified** | cleared |
| `rust:riff` | generator only | **test-domain, reclassified** | cleared |
| `rust:ruststep` | probe only | **test-domain, reclassified** | cleared |
| `rust:serde_json` | ~5 generators + ~80 genuinely-production plugin/framework manifests | generator entries reclassified; the rest is genuinely production (W5, the bulk wave — not mine) | generator entries cleared from the conflict list; the ~80 production entries remain (expected, unowned by this slice) |
| `rust:tiff` | generator + probe | **test-domain, reclassified** | cleared |
| `rust:tobj` | 2 generators | **test-domain, reclassified** | cleared |
| `rust:zip` | `🧰️framework/🛍️products/💻️os` + `🖥️host` (both framework production) | genuinely production, but framework-only (outside this ticket's `✏️s/` scope) AND a registry-data question: is `zip` really test-only, or does the framework legitimately need it at runtime, in which case it shouldn't be *registered* as an oracle at all? **Flagging, not fixing — out of scope for this slice.** | unchanged |

No `🏭️generator` crate turned out to be production-reachable after all — every one of the ten
regex-reclassified entries was verified by opening the flagged manifest and confirming the
declaring crate is a standalone `[workspace]`-rooted probe/generator binary, never referenced by a
`path =` dependency from any production manifest.

## Slice (b): `rust:gltf` at runtime — fixed

### What existed already (checked before writing anything new)

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
(1060 lines) is a complete, already-correct first-party glTF 2.0 byte/container codec: base64
data-uri codec, `.gltf` JSON parse/serialize, `.glb` binary container encode/decode
(`decode_glb`/`encode_glb`), accessor decode with sparse-accessor overlay and normalization
(`decode_accessor`/`read_elements`/`normalize_components`), and buffer resolution
(`resolve_document_buffers`, which deliberately leaves external file-uri buffers unresolved — "this
artifact has no filesystem/network access"). It is NOT usable verbatim in the framework as-is: its
`GltfDocument`/`GltfAccessor`/etc. types are `serde`-derived and its JSON parse goes through
`serde_json::from_str`, and the ticket instructed the framework's own copy to use
`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` (`pack::json::Value`) instead, since
`serde_json` itself is a separate, actively-being-removed violation elsewhere in this ticket (W5).

### What changed

- `🧰️framework/📦️packages/🦀️rust/Cargo.toml`: `gltf` dependency **deleted outright** — it was
  vestigial. Verified via grep: this crate's only source file (`📦️glue.rs`) never references
  `gltf::` anything; it re-exports `mesh_from_glb`/`mesh_to_glb`/etc. from
  `semio-framework-mesh-engine`, none of which are `gltf`-crate types.
- `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/Cargo.toml`: `gltf` moved out of
  `[dependencies]` into `[dev-dependencies]` (kept ONLY as the differential-test oracle). Added
  first-party path dependencies: `pack` (package `semio-framework-pack`, for `pack::json::Value`)
  and `semio-framework-io-base64` (for `base64_standard_decode`, replacing the hand-rolled base64
  the s-plugin codec carries — that crate already exists specifically to be the shared base64
  codec for exactly this purpose, per its own Cargo.toml description).
- `🧰️framework/🔨️modules/🔺️mesh-engine/🦀️.rs`: the four `gltf`-crate-based decode functions
  (`glb_triangle_indices`, `append_glb_primitive`, `append_glb_mesh`, `append_glb_node`) and
  `mesh_from_glb` were replaced with a first-party glTF 2.0 decode pipeline in a new
  `//#region 🔖️GltfCodec`: `gltf_split_container` (GLB magic/chunk walk or bare `.gltf` JSON,
  byte-for-byte mirroring the stdio codec's `decode_glb`), `gltf_decode_data_uri` /
  `gltf_resolve_buffers` (via `semio_framework_io_base64`), `gltf_decode_accessor` +
  `gltf_read_elements` + `gltf_normalize_components` + `gltf_read_bufferview_elements` (ported
  algorithm, same component-type/accessor-type semantics, same sparse-accessor overlay, working
  against `pack::json::Value` instead of `serde`-typed structs), and node/mesh/primitive traversal
  (`gltf_append_node`/`gltf_append_mesh`/`gltf_append_primitive`) including a first-party
  TRS→matrix quaternion composition (`gltf_trs_matrix`, `gltf_node_local_matrix`) reimplementing
  what the `gltf` crate's `node.transform().matrix()` used to do. `mesh_to_glb` (the encoder) was
  ALREADY gltf-crate-free (hand-rolled JSON `format!` string) and is untouched.
  `mesh_from_glb`'s public signature is unchanged.
- `pack::json` is used exactly as instructed and NOT modified — read-only. Its `Value::get`/
  `as_array`/`as_u64`/`as_f64`/`as_str`/`as_bool` cover everything needed for manual glTF field
  extraction; nothing was missing.

### Differential-test design

`🧰️framework/🔨️modules/🔺️mesh-engine/🦀️.rs`, new `#[cfg(test)] mod gltf_oracle_differential`: the
exact deleted `gltf`-crate-based decode pipeline was MOVED (not rewritten) into this test module,
renamed `oracle_*`, and kept as the `gltf` `[dev-dependencies]` reference. Three tests decode the
SAME bytes through both `mesh_from_glb` (first-party) and `oracle_mesh_from_glb` (third-party) and
assert structural equality (`assert_structurally_equal`: indices exact-equal, positions within
1e-4, normals within 1e-3 — per repo convention, glTF-adjacent exporters are not
byte-deterministic, so this compares DECODED STRUCTURE, never raw bytes):
1. `differential_embedded_bin_chunk_matches_gltf_crate_oracle` — the new committed
   `single-triangle-embedded.glb` fixture.
2. `differential_generated_uv_sphere_glb_matches_gltf_crate_oracle` — a mesh round-tripped through
   `mesh_to_glb`.
3. `differential_puzzle_fixture_glb_matches_gltf_crate_oracle` — the pre-existing real-world
   `../🖼️assets/🌱️metabolism/🎨️representation/🧊️capsule_J.glb` fixture (1472 verts / 1750 tris,
   already used by an existing non-differential regression test,
   `glb_import_collects_triangle_primitives_after_guides`, which exercises non-triangle "guide"
   nodes before the real mesh nodes in the scene graph).

### Language-agnostic fixture suite

New: `🧰️framework/🔨️modules/🔺️mesh-engine/🧪️tests/🧊️gltf-codec/🧫️fixtures/`:
- `single-triangle-embedded.gltf` — `.gltf` JSON, embedded base64 `data:` buffer.
- `single-triangle-embedded.glb` — binary GLB, embedded BIN chunk. Same triangle, byte-exact
  layout, hand-computed (not generated via this crate's own encoder, to avoid a self-fulfilling
  round trip).
- `external-buffer.gltf` + `external-buffer.bin` — the external-buffer case. Per the SAME
  deliberate contract the stdio artifact's own codec already documents ("this artifact has no
  filesystem/network access"), this engine does not read the referenced `.bin` off disk either;
  the test (`mesh_from_gltf_reports_a_clear_error_for_unresolved_external_buffer`) asserts a clear
  typed error, not fabricated geometry or a panic — this is a deliberate, pre-existing repo
  convention, not a gap introduced here.
- `expected-single-triangle.json` — decoded positions/normals/indices as data, so any future
  non-Rust implementation can validate against the same fixture pair without touching Rust.
  Rust's own tests load it through `pack::json::parse` rather than re-hardcoding the numbers.

## VERIFY BY RUNNING

`bun ./📜️script.ts verify dependencies literal-external` (verbatim tail, this slice's final
state):
```
total	168	166	2	0	3	163	85
zero-target=0 literal-external=163 meets-target=false
audited-toolchain bun=bun@1.3.14 engines.bun=>=1.2.0 nx=@nx/devkit,@nx/js,nx authorized-rows=3 unauthorized-rows=0 lock-owned=3/3
oracle-conflicts=6 toolchain-owner-conflicts=0
  oracle-conflict js:brepjs declared by ✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/package.json
  oracle-conflict js:three declared by 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json, 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/package.json, 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json
  oracle-conflict rust:image declared by 🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml, 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml, 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml
  oracle-conflict rust:png declared by 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml
  oracle-conflict rust:serde_json declared by … (unchanged bulk W5 list, ~80 entries, none mine)
  oracle-conflict rust:zip declared by 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml, 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml
error: [verify dependencies literal-external] target=0, current=163, oracle-conflicts=6, toolchain-owner-conflicts=0, toolchain-failures=0.
```

`gltf` no longer appears anywhere in the oracle-conflict list — confirmed fixed.

`cargo build -p semio-framework-mesh-engine` (isolated `CARGO_TARGET_DIR`, `RUSTC_WRAPPER=""` —
see note below): `Finished \`dev\` profile [unoptimized] target(s) in ...` — clean, no errors.

`cargo test -p semio-framework-mesh-engine` (same env), verbatim tail:
```
running 26 tests
test tests::box_has_triangles ... ok
test tests::mesh_from_glb_decodes_embedded_bin_chunk ... ok
test tests::mesh_exporter_and_importer_use_short_format_kind_ids_not_media_format ... ok
test tests::glb_round_trip_preserves_positions_and_indices ... ok
test tests::glb_round_trip ... ok
test tests::mesh_from_glb_rejects_bytes_without_valid_glb_container ... ok
test tests::mesh_from_indexed_with_face_groups_empty_groups_leaves_face_ids_empty ... ok
test tests::mesh_from_gltf_reports_a_clear_error_for_unresolved_external_buffer ... ok
test tests::mesh_from_gltf_decodes_embedded_base64_buffer ... ok
test tests::mesh_data_aabb_and_merge ... ok
... (16 more pre-existing tests, all ok)
test gltf_oracle_differential::differential_embedded_bin_chunk_matches_gltf_crate_oracle ... ok
test gltf_oracle_differential::differential_generated_uv_sphere_glb_matches_gltf_crate_oracle ... ok
test gltf_oracle_differential::differential_puzzle_fixture_glb_matches_gltf_crate_oracle ... ok
test tests::glb_import_collects_triangle_primitives_after_guides ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```
All three differential-oracle tests pass — the first-party decode pipeline agrees with the `gltf`
crate on the embedded-BIN-chunk fixture, a generated UV-sphere GLB, AND the pre-existing real-world
1472-vertex Puzzle fixture. The two new fixture tests and the external-buffer error-path test also
pass.

`cargo build -p semio-framework` (top-level crate whose only change was DELETING the vestigial
`gltf` line): **FAILS — 75 pre-existing errors, ALL unrelated to this slice.** Every error is in
`semio-framework-os-kernel`'s mutation/store code
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/…`): `SpaceHistoryMutation` doesn't satisfy
`protocol::ToValue`/`FromValue`/`serde::Serialize`/`Deserialize` bounds required by
`dsl::Mutations`/`ArtifactStore::dispatch`. Verified this is not mine: grepped the full error
output for `gltf`, `mesh_engine`/`mesh-engine`, `pack::json`, `io_base64`/`io-base64` — zero
matches. This is `os-kernel`'s own VCS/store/mutation-schema code, a completely different
subsystem from the framework kernel's `gltf`/mesh re-exports I touched, and matches documented
session precedent ("Concurrent Cargo Workspace Churn": repo-wide build failures are often another
session's in-progress refactor elsewhere in the shared tree — this ticket has other agents actively
reworking mutation/schema code concurrently, per the coordinator's own note that "another agent is
actively working on that JSON stack"). `cargo build -p semio-framework-mesh-engine` on its own
(the crate I actually rewrote) is clean with zero errors, confirming my change is not the cause.

`bun ./📜️script.ts verify dependencies` (the ratchet gate against `🔒️dependencies.json`) — ALREADY
FAILING before and unrelated to this slice's changes: 6 new third-party entries not in the frozen
baseline, none of which trace to this slice's edits — `js:lodash`, `js:markdown-it`,
`js:minimatch` (root `package.json` devDependencies, unrelated), `python:mercantile`
(`pyproject.toml`, unrelated), `rust:sha2` (a PRE-EXISTING `[dev-dependencies]` entry in
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml`, not touched by
either of my two script.ts fixes — dev-dependencies always map to `test`/`repository-tooling`
intent regardless of the new `isProcMacro` branch, which only changes the `[dependencies]`
section), and `rust:tiff` (a `🏭️generator`/`🔬️probes` test-oracle addition, also not mine). None of
my new path dependencies (`pack`, `semio-framework-io-base64`) appear — both are internal (`path =`
present), correctly filtered before third-party classification. `gltf` itself does not appear as
"new" either (it was already a known baseline identity via the probe; moving its declaring section
in two manifests doesn't change the `${ecosystem}:${name}` identity the ratchet compares by). This
confirms my changes did not break the ratchet gate any further than it already was — but the gate
was already red before I started (from other agents' concurrent, unrelated work), and I did not
regenerate `🔒️dependencies.json` (explicitly forbidden by this slice's constraints; the parent
session owns that regeneration).

**Build/test environment note, for whoever re-runs this**: a plain `cargo build -p
semio-framework-mesh-engine` at repo default settings hung indefinitely (0% CPU, 30+ minutes, zero
progress) even after moving `CARGO_TARGET_DIR` to an isolated scratchpad directory — this matches
prior session memory ("sccache Serializes Concurrent Builds"): with 60-100+ concurrent `rustc`
processes across sibling agents/sessions all wrapped through the same `sccache` server, every
compilation unit queues behind the others regardless of target-dir isolation. Setting
`RUSTC_WRAPPER=""` (bypassing sccache) alongside the isolated `CARGO_TARGET_DIR` fixed it —
the mesh-engine build then completed in ~4 minutes for `build` and ~6 minutes for `test` (compiling
`gltf`/`gltf-json`/`gltf-derive` and the whole `pack`/`replication`/`async` dependency chain from a
cold isolated cache).
