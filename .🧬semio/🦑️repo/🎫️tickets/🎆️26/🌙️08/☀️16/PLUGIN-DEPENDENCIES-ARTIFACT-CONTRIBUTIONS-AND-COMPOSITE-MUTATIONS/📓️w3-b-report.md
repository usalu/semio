# W3-B Report — CAD `aec-building` extension: pilot P2 (extension contributes onto another plugin's artifact)

Lane: **W3-B** (Sonnet 5). Contract: `📋️contract-freeze.md` §1/§3/§4, `📓️w1-a-report.md` (the guest-SDK
API this lane codes directly against). Start commit `7ad8955884`.

## Exclusive lease (files touched)

All changes are inside `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/`, per the lease:

- `📦️packages/🦀️rust/Cargo.toml` — added two dependencies (`semio-s-plugin-cad`,
  `semio-framework-os-kernel`), both `default-features` handled per convention (see below).
- `📦️packages/🦀️rust/📦️glue.rs` — added `extern crate semio_framework_os_kernel as protocol;` /
  `as store;`, the same local crate-alias convention every plugin/extension crate in the repo repeats
  at its own root (cad's own `📦️glue.rs` does the identical thing).
- `🦀️component.rs` — added the composite mutation, the contributed inference, the `.depends_on(...)`
  declaration, the `.contributes(...)` wiring, and 5 new tests. This extension has no `🗿️artifacts`
  dir of its own (it is a pure topic/contribution-contributor onto `cad-play`/cad's artifact, not an
  artifact owner), so per the brief's item 5 the composite payload stays in the extension's own
  component tree — no `🧬️mutations/<kind>/{🦠️mutation,🧩️plan}` tree was created.

No other files were touched.

## 1. Cargo dependency on `semio-s-plugin-cad`

```toml
semio-s-plugin-cad = { path = "../../../../../📐️cad/📦️packages/🦀️rust", package = "semio-s-plugin-cad", default-features = false }
semio-framework-os-kernel = { path = "../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
```

`default-features = false` on the cad dependency mirrors `💠️lowpoly`'s own `cad_plugin` dependency
(cad's `default = ["plugin-entry"]` feature is what emits its own WASM component entry point; this
extension owns its own `extension_exports!(bundle)` entry point, so cad's must stay off). The second
dependency (`semio-framework-os-kernel`, default features, same as cad's own use of it) was not named
in the task brief but is load-bearing: `ArtifactContribution::mutation::<Snapshot, Op, K>()`'s bounds
(`Op: protocol::Mutation<Snapshot> + protocol::OpBinary`, `K: protocol::CompositeMutationKind<Snapshot,
Op>`) and `store::ArtifactPack` are reached through the same `protocol`/`store` crate-alias convention
cad's own leaf mutations use — `semio-framework-plugin` does not re-export these names at its own
crate root (`ArtifactContribution` itself is only reachable as `semio_framework_plugin::app::
ArtifactContribution`, not flattened either — confirmed by reading its `pub use app::{...}` list),
so every plugin/extension that writes a `CompositeMutationKind` impl needs this dependency directly,
exactly as cad/lowpoly/every other leaf-mutation-writing crate already does.

## 2. `.depends_on("cad", …)` on the `ExtensionBundle`

```rust
fn bundle() -> ExtensionBundle {
    ExtensionBundle::new(EXTENSION_ID, "CAD AEC Building", "0.1.0")
        .extends("cad")
        .depends_on("cad", semio_framework::VersionReq::parse("^0.1.0").expect("valid version req"))
        .contributes_topic(/* unchanged cad.computer topic contribution */)
        .contributes(building_storey_contribution())
}
```

`extends("cad")` is called before `depends_on("cad", …)`; both funnel through
`ExtensionBundle::assert_extends_matches_primary_dependency`, so `extends == dependencies[0].plugin_id`
holds by construction — `bundle_declares_the_cad_dependency_and_registers_the_building_storey_
contribution` asserts `manifest.dependencies[0].plugin_id == "cad"` directly. Version req `^0.1.0`
matches cad's actual declared version (`Plugin::builder("cad").version("0.1.0")`, workspace version
`0.1.0`).

## 3. The composite mutation — `CreateBuildingStorey`

Contributed onto cad's canonical artifact kind `"s.cad.cad"` (NOT the pre-migration app-manifest id
`"3d.cad"` cad's own `artifact_kind()` still carries — the dependency/contribution-target gates both
parse `ArtifactKindId::parse(kind).plugin()`, which only the canonical `s.<plugin>.<artifact>` grammar
satisfies). Planned from two of cad's own existing leaf mutations
(`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{➕create-node,🎯change-active-model-definition}`)
through `protocol::Planner::call` — a real building-domain workflow step (creating a named "storey"
node and switching the document's active model definition to the building pane) that a bare CAD tool
has no notion of:

```rust
impl protocol::CompositeMutationKind<CadSnapshot, CadMutation> for CreateBuildingStorey {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "building-storey", kind: "create-building-storey", record: "CreatedBuildingStorey" };
    fn plan(&self, _base: &CadSnapshot, planner: &mut protocol::Planner<CadSnapshot, CadMutation>) -> Result<(), protocol::PlanError> {
        planner.call(CadMutation::CreateNode(CreateNode { node: CadNode { id: self.storey_id.clone(), label: self.storey_label(), kind: "building-storey".into() } }))?;
        planner.call(CadMutation::ChangeActiveModelDefinition(ChangeActiveModelDefinition { new_model_definition_id: "aec.building".into() }))
    }
    ...
}
```

Registered via `ArtifactContribution::builder("s.cad.cad").mutation::<CadSnapshot, CadMutation,
CreateBuildingStorey>(CAD_DOCUMENT_SCHEMA, 1, 1)`, where `CAD_DOCUMENT_SCHEMA = "cad.scene"` is cad's
own `ArtifactApp::DOCUMENT_SCHEMA` (`CadPlayApp::DOCUMENT_SCHEMA`), per the builder's own doc comment
— **not** the `"cad.cad"` string cad's `#[mutations(... schema = "cad.cad")]` derive attribute uses
internally for its own owner-mutation `SchemaId` (a separate, pre-existing namespace in cad's own
code, unrelated to `ArtifactApp::DOCUMENT_SCHEMA` and not something this lane touches or needs to
reconcile).

**Landed id**: `"cad.scene#cad-extension-aec-building:create-building-storey"`, assembled by
`ArtifactContribution::resolve` per the frozen grammar
`"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"` — never hand-formatted in this
extension's own code.

## 4. The contributed inference — `building-structure-summary`

```rust
ArtifactInferenceServiceMetadata {
    owner: EXTENSION_ID,              // "cad-extension-aec-building"
    artifact_kind: CAD_ARTIFACT_KIND, // "s.cad.cad"
    artifact_schema: CAD_ARTIFACT_KIND,
    document_schema: CAD_DOCUMENT_SCHEMA, // "cad.scene"
    inference_schema: AEC_BUILDING_INFERENCE_SCHEMA, // "s.cad-extension-aec-building.building-structure-summary"
    ...
}
```

Computes whether the document has a building model slot attached and how many of the extension's own
`create-building-storey` nodes exist — a summary a generic CAD plugin has no reason to own. `owner`
equals this extension's own plugin id (contract §4 rule 4); `inference_schema` starts with
`s.cad-extension-aec-building.` (contract §3).

## 5. Composite taxonomy

This extension has no `🗿️artifacts` dir of its own — it never owned an artifact before this ticket and
still doesn't (it contributes onto cad's), so there is no `🧬️mutations/<kind>/{🦠️mutation,🧩️plan}`
tree to place. Per the brief's item 5, the payload stays in the extension's own component tree
(`🦀️component.rs`, `//#region 🔖️Composite`).

## Tests written and run (co-located, `🦀️component.rs` `mod tests`)

| Requirement | Test |
|---|---|
| Contribution registers successfully against the declared dependency | `bundle_declares_the_cad_dependency_and_registers_the_building_storey_contribution` (plus the mere fact `bundle()` — called by every other test — never panics; `.contributes()` panics synchronously on any gate rejection) |
| Mismatching/missing the dependency fails with the typed gate error | `contribution_onto_cad_requires_a_declared_dependency` — builds a variant `ExtensionBundle` with `.extends("cad")` but **no** `.depends_on("cad", …)`, then `.contributes(...)` the same contribution; asserted via `catch_unwind` (mirrors the framework's own `extension_bundle_dependency_tests`, since `ExtensionBundle::contributes` is `panic!`-on-gate-violation, not `Result`) |
| Contributed mutation id does not collide with any cad owner kind | `contributed_mutation_id_structurally_cannot_collide_with_any_cad_owner_kind` — checks the landed id carries the `:` segment, then sweeps every `CadMutation::kinds()` entry and confirms none can ever equal it (owner ids never carry `:`) |
| Plan folds to the same snapshot as applying cad's leaf mutations by hand | `plan_folds_to_the_same_snapshot_as_applying_cads_leaf_mutations_by_hand` — `protocol::fold_plan_diff(&kind, &base).apply(&base)` compared byte-for-byte against sequentially applying `CreateNode` then `ChangeActiveModelDefinition` directly |
| Inference's metadata passes the ownership gate | Covered two ways: (a) `bundle_declares_the_cad_dependency_and_registers_the_building_storey_contribution` asserts the landed `owner`/`contributor`/`artifact_kind` on the resolved manifest row (which could only be present because `register_contributions` accepted it — `.contributes()` would have panicked otherwise); (b) `contributed_inference_computes_a_real_building_summary` additionally drives the inference function directly and checks `building_structure_summary_service().metadata()` |

Plus the pre-existing `bundle_contributes_building_import_profile` test, unchanged.

**I could not run these tests to a passing result** — see Gates below. I manually cross-checked every
type, trait bound, field name and method signature used against their real definitions (`CadSnapshot`/
`CadMutation`/`CadNode`/`CreateNode`/`ChangeActiveModelDefinition` in
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/...`, `ArtifactContribution`/`CompositeMutationKind`/`Planner`/
`ArtifactInferenceService*` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`,
`Mutation`/`MutationDiff`/`SemanticMutation`/`fold_plan_diff` in
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`) — every reference resolves to
a real, currently-existing item with a matching signature. This is a careful manual review, not a
confirmed-passing test run; do not read the tests-written table above as "tests passing."

## Gates — real output, all currently blocked by unrelated concurrent churn

### `cargo check -p semio-s-plugin-cad-aec-building` / `cargo test -p semio-s-plugin-cad-aec-building --lib`

**Blocked.** Ran the check 3 times and the test once, spaced minutes apart (including an automated
8-attempt background poll, ~4 minutes, 30s apart). Every single run fails to compile
`semio-framework-plugin` itself (a dependency of every plugin/extension in the repo, including mine) —
**never** my own crate, and **zero** of the error messages mention `aec-building` (checked with `grep
-c aec-building` on the full output each time: `0`). The specific errors change from run to run:

- Run 1: `E0432 unresolved imports HostMediaHandlerDeclaration, LinkedFlowExtensionInstallerDeclaration`, `E0599 no associated function 'foreign' for DocumentCodecSpec`, `E0599 no method 'preflight_foreign'`, `E0061 wrong arg count` (4 errors)
- Run 2 (identical to run 1, retried immediately)
- Background poll, final attempt: `E0425/E0433 cannot find type BTreeSet`, `E0425/E0433 cannot find type HostMediaHandlerRegistry`, `E0425/E0433 cannot find type FlowExtensionRegistry`, `E0624 method 'preflight' is private` (10 errors)
- Final `cargo test` run: `E0425/E0433 cannot find type BTreeSet`, `E0624 method 'preflight' is private` (4 errors)

**Attribution** (per the brief's instruction, `git log --date=iso` against start commit `7ad8955884`):
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` is dirty, uncommitted, 65
insertions/3 deletions vs HEAD, adding `HostMediaHandlerDeclaration`/`LinkedFlowExtensionInstallerDeclaration`/
`DocumentCodecSpec::foreign`/`.preflight_foreign(...)` call sites that don't yet exist on the
(currently clean) `🔌️plugin/🦀️component.rs` — a different, live session mid-edit on exactly the file
W1-A's own report already flagged as a hot spot ("Concurrent churn since... hit a different external
compile error on each attempt"). Confirmed **not specific to my new dependency edges**: `cargo check -p
semio-s-plugin-cad` (cad itself, untouched by this lane, no relation to my Cargo.toml edits) fails
with the **identical** `semio-framework-plugin` errors at the same moment. This is a repo-wide,
transient, external blocker, consistent with the ticket brief's own warning about the shared
`component.rs` tree and my own prior-session note on concurrent cargo-workspace churn (30–90+ minute
windows). Recommend the coordinator re-run both gates once that lane's `🏗️builder`/`🦀️component.rs`
pair lands consistently.

I am not claiming these gates passed. They did not run to completion in this session.

**Update after further polling** (ran an automated retry loop, 30s apart, over several more minutes):
the `semio-framework-plugin` errors above **cleared** — that crate now compiles clean, confirming the
`🏗️builder`/`🦀️component.rs` pair did land consistently while this lane was working, exactly as
recommended above. The blocker moved one level further down cad's dependency chain, to
`semio-s-plugin-stdio` (cad → stdio):

```
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
error: couldn't read ".../🧊️gltf/.../💡️inferences/💾️binary/🦀️component.rs": No such file or directory (os error 2)
```

— a **different** concurrent session mid-refactor of stdio's `gltf` artifact's inference facets
(splitting one `💡️inferences/🦀️component.rs` into per-measure subdirectories: `🪞️symmetry/`,
`🌊️roughness/`, `↔️clearance/`, …). Confirmed via `git status --porcelain` on
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf`: **400** changed path entries, uncommitted, against start
commit `7ad8955884` — a large, still-moving restructure, not a stable error to chase (the exact missing
file differed on every one of the 6 retries: `🔗️component.graphql`, `🛰️component.proto`,
`🌊️roughness/deviation-from-ideal/🦀️component.rs`, `🚪️io/💡️inferences/📝️text/🦀️component.rs`,
`🚪️io/💡️inferences/💾️binary/🦀️component.rs`, …). This is, again, zero occurrences of `aec-building` in
any of the output, and `cargo check -p semio-s-plugin-cad` (untouched by this lane) hits the identical
`semio-s-plugin-stdio` failure at the same moment — confirming once more this is a repo-wide transitive
blocker on cad's own dependency chain, not anything in this extension's diff. Recommend the coordinator
re-run both gates once the stdio `gltf` inference restructure settles.

### `bun ./📜️script.ts policy` (repo root)

Ran to completion (exit 1, as it always does — ~24.8k pre-existing high-priority breaches across the
whole repo, unrelated to this ticket). Queried the full breach cache
(`.🦑️repo/⚡️cache/breaches/compose.json`) directly for the two gates this task names:

- **`plugin-dependency/contribution-target`: 0 rows, repo-wide** — my contribution targets `"s.cad.cad"`,
  a declared dependency (`.depends_on("cad", …)`), so it is correctly not flagged.
- **`plugin-dependency/parity`: 0 rows scoped to `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building`** —
  both directions now agree (Cargo dependency on `semio-s-plugin-cad` ⇔ `.depends_on("cad", …)`), so
  this extension is gone from the list, as required.

**Side effect discovered and flagged, not fixed (out of lease):** `policyDependencyOwnerRoots`
(`📜️script.ts` ~7799) lists `✏️s/🔌️plugins/<plugin>` and each of its `🧩️extensions/<ext>` as *separate*
roots, but `policyWalkRelFiles` (~7209) is a plain recursive walk with no exclusion for a root's own
nested-extension subdirectories. Since `🧩️extensions/🏢️aec-building` physically lives inside
`✏️s/🔌️plugins/📐️cad`, my new `.depends_on("cad", …)` call gets picked up **twice**: once correctly
under the extension's own root, and once spuriously under the *parent* cad plugin's root — producing a
new HIGH-priority breach attributed to the wrong scope:

```
plugin-dependency/parity  ✏️s/🔌️plugins/📐️cad  "✏️s/🔌️plugins/📐️cad" declares .depends_on("cad") with no Cargo dependency on semio-s-plugin-cad
```

This is a pre-existing latent bug in shared `📜️script.ts` (not in my lease, and that file is
explicitly under concurrent edit by another session per `📓️w0-i-report.md`) that will hit every other
extension lane in this ticket the moment each lands its own real `.depends_on(...)` call (process's
metal/robotic/concrete/wood, sourcing's slabs/windows/beams, flow's brep, draw's draw-fsm — none of
which have landed one yet, which is why I appear to be the first to trip it). Flagged via
`spawn_task` (`task_7072615b`, "Fix policy owner-root walk double-counting nested extensions") with a
full repro rather than touched directly.

## Summary of what landed vs. what could not be verified

Landed and manually verified correct against real definitions: the Cargo dependency (both directions,
`default-features = false` on cad), the `protocol`/`store` crate-alias convention, `.depends_on("cad",
…)` consistent with `.extends("cad")`, one composite mutation planned from two of cad's real leaf
mutations, one contributed inference with a correct ownership triple, and 5 new tests covering every
item in the task's test list. Not verified by an actual passing build/test run in this session, purely
due to unrelated, confirmed, repo-wide concurrent breakage in `semio-framework-plugin` that blocks
`semio-s-plugin-cad` itself, not just this extension.
