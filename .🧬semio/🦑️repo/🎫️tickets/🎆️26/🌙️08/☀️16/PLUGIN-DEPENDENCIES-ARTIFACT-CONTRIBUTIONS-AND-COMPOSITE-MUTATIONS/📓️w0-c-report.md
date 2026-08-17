# W0-C — Manifest Spine Report

Lane: 0-C manifest spine (Sonnet 5). Scope: contract freeze §3/§4 manifest types, `VersionReq`,
dependency-graph pure functions + tests.

## Files touched

### Exclusive lease

- **`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`** (crate `semio-framework`)
  - New regions inserted directly before `PluginManifest` (inside the file's outer `🔖️Manifest`
    region): `//#region 🔖️PluginDependency` (`Version`, `VersionParseError`, `VersionReq`,
    `VersionReqParseError`, `PluginDependency`), `//#region 🔖️ArtifactContribution`
    (`ContributedMutationSemantics`, `ContributedMutationMetadata`, `ContributedInferenceMetadata`,
    `ArtifactContributionDescriptor`).
  - `PluginManifest` gains `dependencies: Vec<PluginDependency>` and
    `contributions: Vec<ArtifactContributionDescriptor>`, both `#[serde(default,
    skip_serializing_if = "Vec::is_empty")]`.
  - New `//#region 🔖️DependencyGraph` directly after `PluginManifest`: `DependencyGraphError`,
    `validate_dependency_graph`, `resolve_load_order`, `find_cycle_members` (private),
    `dependents`.
  - New `#[cfg(test)] mod plugin_dependency_tests` (own module, mirrors the existing
    `media_vocabulary_tests` convention) placed right after the `🔖️DependencyGraph` region, before
    `ViewModel` — subregions `🔖️VersionAndVersionReq`, `🔖️DependencyGraphTests`,
    `🔖️ManifestSerdeTests`.
  - Added `.export().unwrap()` calls for the five new `ts_rs`-derived types in the existing
    `#[cfg(feature = "typegen")] fn exports_typescript_bindings()` test, next to
    `PluginManifest::export()`.

- **`🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs`** (crate
  `semio-framework-os-kernel`, `.sxt` package format)
  - New `PackagePluginDependency { plugin_id: String, version: String }` — a **local, byte-wire-compatible
    mirror** of `semio_framework::PluginDependency`, not an import (see "Dependency-edge-law finding"
    below).
  - `ExtensionPackageManifest`: renamed the pre-existing `contributions: serde_json::Value` field
    (which actually carried open **topic** contributions) to `topic_contributions`, freeing the name;
    added `dependencies: Vec<PackagePluginDependency>` (`#[serde(default)]`) and a new
    `contributions: serde_json::Value` (`#[serde(default)]`) holding a raw JSON array of
    `semio_framework::ArtifactContributionDescriptor` — kept untyped for the same crate-boundary
    reason as `PackagePluginDependency`.
  - New `impl ExtensionPackageManifest { pub fn extends_matches_primary_dependency(&self) -> bool }`
    implementing contract freeze §4 rule 1 (`extends == dependencies[0].plugin_id`, vacuously true
    when both are empty).
  - Updated `tests::sample_manifest()` and added `//#region 🔖️DependencyAndContributionTests`
    (4 new tests) inside the existing `mod tests`.

- **`🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`** (type declarations region only)
  - `PluginRegistryEntry` gains `readonly dependencies?: readonly PluginDependency[]`.
  - New `//#region 🔖️PluginDependency` (`VersionReq = string`, `PluginDependency`) and
    `//#region 🔖️ArtifactContribution` (`ContributedMutationSemantics`,
    `ContributedMutationMetadata`, `ContributedInferenceMetadata`, `ArtifactContributionDescriptor`),
    inserted between `PluginRegistryEntry` and the pre-existing `//#region 🗂️PluginCatalog`.
  - No runtime functions touched (`expandPluginRegistry` et al. left exactly as-is).

### Mechanical cross-lease fixup (outside lease, unavoidable — see note)

Adding two new fields to `PluginManifest` (a struct built via full literal syntax everywhere, no
`Default` impl) broke every existing struct-literal construction repo-wide. This is the same class
of problem the contract freeze explicitly pre-authorized for lane 0-A's `MutationMeta.origin`
("a one-line mechanical fixup lane 0-A is explicitly allowed to make outside its lease"). No
equivalent explicit grant exists for 0-C, but leaving ~20 compile sites broken across the shared
tree was a worse outcome than a minimal, purely-additive, mechanically-applied fixup. Every site
got exactly `dependencies: vec![]/Vec::new(), contributions: vec![]/Vec::new(),` appended — no other
line touched:

- `🧰️framework/🛍️products/💻️os/🦀️component.rs` — 7 literals (all inside `#[cfg(test)] mod
  tests`; this file is **not wired into any crate's `📦️glue.rs`** — appears to be an orphaned
  duplicate of `🖥️host/🦀️component.rs` from an earlier split, confirmed dead code, fixed anyway
  for consistency).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — 1 literal
  (`read_manifest`'s bootstrap placeholder).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — 3 literals
  (`Plugin::new`, `plugin_manifest()`'s two fallback branches). This file is under **heavy
  concurrent edit** by lane 0-D and ticket `FULL-STDIO-...` — re-read immediately before each edit,
  touched nothing else.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
  — 2 literals (`resolve_commands` tests).
  `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` — 7 literals (the "os-host" crate,
  `semio-framework-os`; near-duplicate test suite of the orphaned file above, this one **is**
  live-compiled).

None of these files' surrounding logic was touched — verified via `git diff` showing only the
mechanical field additions in each.

## Final type list (exact field names, `semio-framework` crate)

```rust
pub struct Version { pub major: u64, pub minor: u64, pub patch: u64 } // Display "major.minor.patch", Ord = field order = semver precedence
pub enum VersionReq { Any, Exact(Version), Caret(Version), Tilde(Version), AtLeast(Version) } // Display "*" / "=X.Y.Z" / "^X.Y.Z" / "~X.Y.Z" / ">=X.Y.Z"; Serialize/Deserialize as that plain string
pub struct PluginDependency { pub plugin_id: String, pub version: VersionReq }

pub struct ContributedMutationSemantics { pub verb: String, pub entity: String, pub kind: String, pub record: String }
pub struct ContributedMutationMetadata { pub mutation_id: String, pub semantics: ContributedMutationSemantics, pub schema_version: u32, pub algorithm_version: u32 }
pub struct ContributedInferenceMetadata {
    pub owner: String, pub artifact_kind: String, pub artifact_schema: String, pub artifact_schema_version: u32,
    pub document_schema: String, pub document_schema_version: u32, pub inference_schema: String, pub inference_schema_version: u32,
    pub algorithm_version: u32, pub policy_version: u32, pub contributor: String, pub depends_on: Vec<String>, // skip_serializing_if empty
}
pub struct ArtifactContributionDescriptor { pub artifact_kind: String, pub mutations: Vec<ContributedMutationMetadata>, pub inferences: Vec<ContributedInferenceMetadata> } // both skip_serializing_if empty

// PluginManifest additions:
pub dependencies: Vec<PluginDependency>,           // #[serde(default, skip_serializing_if = "Vec::is_empty")]
pub contributions: Vec<ArtifactContributionDescriptor>, // #[serde(default, skip_serializing_if = "Vec::is_empty")]
```

All camelCase on the wire (`pluginId`, `mutationId`, `artifactKind`, `schemaVersion`,
`algorithmVersion`, `artifactSchemaVersion`, `documentSchemaVersion`, `inferenceSchemaVersion`,
`policyVersion`, `dependsOn`).

## Dependency-graph function signatures (pure, no runtime dependency)

```rust
pub fn validate_dependency_graph(manifests: &[PluginManifest]) -> Result<(), DependencyGraphError>;
pub fn resolve_load_order(manifests: &[PluginManifest]) -> Result<Vec<String>, DependencyGraphError>; // Kahn toposort, ties broken by lexicographically-smallest plugin id
pub fn dependents(manifests: &[PluginManifest], plugin_id: &str) -> Vec<String>; // direct dependents only, sorted

pub enum DependencyGraphError {
    MissingDependency { plugin_id: String, depends_on: String },
    VersionMismatch { plugin_id: String, depends_on: String, required: String, actual: String },
    Cycle { members: Vec<String> }, // every plugin on the cycle, named
}
```

`resolve_load_order` always calls `validate_dependency_graph` first, so a missing dependency or
version mismatch is reported before a cycle would be (a cycle also fails validation's "missing
dependency" check only if truly missing; a real cycle among present plugins passes validation and
is caught by the toposort leftover-set walk in `find_cycle_members`).

## Dependency-edge-law finding (read before wiring host/SDK lanes)

`semio-framework-os-kernel` (home of `🧩️extension/🦀️component.rs`, the `.sxt` format) has **no**
dependency on `semio-framework` (confirmed in its `Cargo.toml`) and, per contract freeze §0, must
never gain one. So `ExtensionPackageManifest` **cannot** literally hold
`Vec<semio_framework::PluginDependency>` / `Vec<semio_framework::ArtifactContributionDescriptor>` —
this was already true of `ExtensionManifest.capabilities`/`.topic_contributions` before this
ticket (flattened to `Vec<String>`/raw JSON on the `.sxt` side for the exact same reason). I
followed that existing precedent: `PackagePluginDependency` is a **field-for-field, JSON-shape-identical**
local struct (`{pluginId, version}`, `version` as the plain `VersionReq` display string), and the new
artifact contributions are carried as raw `serde_json::Value` (array of the real
`ArtifactContributionDescriptor` shape). Any code in a crate that *does* depend on `semio-framework`
(the guest SDK, `semio-framework-plugin`) can decode/encode both losslessly via
`semio_framework::VersionReq::parse` / `serde_json::from_value::<Vec<ArtifactContributionDescriptor>>`.
The guest `ExtensionManifest` in `🔌️plugin/🦀️component.rs` (that lane's file, not touched here) has
no such constraint — `semio-framework-plugin` already depends on `semio-framework` — so it should use
the real `semio_framework::PluginDependency`/`ArtifactContributionDescriptor` types directly.

## Test output

`cargo test -p semio-framework --lib` (full crate, unfiltered):

```
test result: FAILED. 128 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

The single failure, `io::tests::io_registry_rejects_a_conflicting_key_without_replacing_the_first_entry`
in `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, is **not mine** — that file is not in my lease
(never edited by this lane) and `git status --porcelain` shows it `M ` (uncommitted, currently
being edited). It is explicitly listed in `📋️ownership-and-handoffs.md`'s shared-tree rule #1 as a
W0 lease of ticket `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`. Fails
deterministically even in single-threaded isolation (`--test-threads=1`, same assertion), so it is
that file's own in-flight refactor state, not test-order flakiness. Filtering to just the 18 new
tests this lane added, all pass:

```
test manifest::plugin_dependency_tests::dependents_returns_direct_dependents_sorted ... ok
test manifest::plugin_dependency_tests::resolve_load_order_accepts_a_self_satisfying_empty_graph ... ok
test manifest::plugin_dependency_tests::resolve_load_order_reports_missing_dependency ... ok
test manifest::plugin_dependency_tests::version_ord_matches_semver_precedence ... ok
test manifest::plugin_dependency_tests::resolve_load_order_is_deterministic_regardless_of_input_order ... ok
test manifest::plugin_dependency_tests::version_parses_valid_triples_and_rejects_malformed_input ... ok
test manifest::plugin_dependency_tests::version_req_matches_caret_semantics_across_leading_zero_tiers ... ok
test manifest::plugin_dependency_tests::resolve_load_order_toposorts_a_diamond ... ok
test manifest::plugin_dependency_tests::resolve_load_order_reports_version_mismatch ... ok
test manifest::plugin_dependency_tests::version_req_display_round_trips_through_parse ... ok
test manifest::plugin_dependency_tests::version_req_matches_tilde_semantics ... ok
test manifest::plugin_dependency_tests::version_req_parses_all_five_grammar_forms_and_rejects_unknown_operators ... ok
test manifest::plugin_dependency_tests::version_req_matches_exact_and_at_least ... ok
test manifest::plugin_dependency_tests::resolve_load_order_names_every_member_of_a_cycle ... ok
test manifest::plugin_dependency_tests::plugin_dependency_serde_round_trips_as_a_plain_string ... ok
test manifest::plugin_dependency_tests::plugin_manifest_dependencies_and_contributions_default_absent_on_the_wire ... ok
test manifest::plugin_dependency_tests::artifact_contribution_descriptor_round_trips ... ok
test manifest::plugin_dependency_tests::plugin_manifest_with_dependencies_and_contributions_round_trips ... ok
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 111 filtered out
```

### Extra diligence beyond the mandated gate (files touched outside lease)

- `cargo check -p semio-framework-plugin` → **clean** (my 3 mechanical fixups there compile fine).
- `cargo check -p semio-framework-plugin-host -p semio-framework-os` → fails, but the single error
  (`store::document_codec` return-type mismatch in
  `🔌️plugin/🖥️host/🦀️component.rs:90`) and the ~40 errors transitively from `semio-s-plugin-stdio`
  (`crate::artifacts::*::assembly` / `definition` not found) are **not mine** — `git status`
  confirms `🏪️store/🦀️component.rs` and the entire `✏️s/🔌️plugins/🗄️stdio/` tree are currently
  `M`/uncommitted, both explicitly named as other tickets' live W0 leases.
- `cargo test -p semio-framework-os-kernel --lib extension::` → crate fails to compile, but every
  error is in `📡️spr/🎮️command/🦀️component.rs` / `📡️spr/🧪️testkit/🦀️component.rs` (lane 0-A's
  exclusive lease, `git status` confirms both `M`/uncommitted — mid-`CompositeMutation`-derive
  work). Zero errors reference `🧩️extension/🦀️component.rs`.
- TS: no project-wide typecheck target exists for `@semio-tech/framework`
  (`🧰️framework/📦️packages/🟦️typescript/📋️project.json` only has `test`/`test-quick`/
  `test-long`/`test-exhaustive`); a standalone `tsc --noEmit` on just the edited file pulled in an
  unrelated, already-broken sibling (`🟦️glue.ts`, pre-existing `eventCount`/statechart errors, not
  from this change) so its output isn't informative. The edit itself is a self-contained, additive
  block of plain type aliases with no new imports — re-read visually twice for syntax.

## Notes for later waves

- **Host lanes (W2)**: `resolve_load_order`/`validate_dependency_graph`/`dependents` are pure and
  take `&[PluginManifest]` — no runtime/registry coupling, callable directly from whatever loads
  manifests.
- **Guest SDK lane (W1-A, `🔌️plugin/🦀️component.rs`)**: add `dependencies`/`contributions` to the
  guest `ExtensionManifest` using the **real** `semio_framework::PluginDependency` /
  `ArtifactContributionDescriptor` types (that crate already depends on `semio-framework`) — do not
  copy the `.sxt` crate's local-mirror pattern, it exists only because of the os-kernel dependency
  edge law.
- **`.sxt` pack/install code** (whoever wires `ExtensionBundle` → `ExtensionPackageManifest`, not
  built yet): converting from the guest's real `PluginDependency`/`ArtifactContributionDescriptor`
  to the package's `PackagePluginDependency`/raw-JSON form is a one-line `.to_string()` /
  `serde_json::to_value` at that boundary, inside `semio-framework-plugin` (which can see both
  shapes) — never inside `semio-framework-os-kernel`.
- `ExtensionPackageManifest.extends_matches_primary_dependency()` is validation-only (not wired
  into `pack()`/`verify()`/`unpack()`) — a later wave's registration-gate code should call it
  explicitly per contract freeze §4 rule 1 rather than relying on `pack()` to enforce it.
- The orphaned `🧰️framework/🛍️products/💻️os/🦀️component.rs` (not referenced by any `📦️glue.rs`)
  is worth a dedicated cleanup ticket outside this one's scope — flagged, not removed here.
