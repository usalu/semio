# Fable Explore — VCS Provider Frontier

Read-only current-source audit, 2026-09-05. No build, test, or generator was run. All statements
below are static-source findings against the working tree as of this read; every claim is marked
**verified** (I read the exact bytes/lines cited) or **inferred** (reasoned from adjacent evidence,
not directly confirmed).

## Headline: the blueprints undersell the current frontier

The two governing blueprints (`📓️terra-all-plugin-provider-expansion-blueprint.md`,
`📓️terra-multi-provider-verified-catalog-blueprint.md`, both dated 2026-09-04) describe a
"selected-provider resolver" and a "package-id join" as future work that **does not yet exist**.
Current source (read 2026-09-05) already has both, landed by the Sol lane
(`📓️sol-trusted-stdio-gis-bundle.md`, `📓️sol-native-catalog-selection-foundation.md`):

- `PackageDescriptor.package_id: String` is a real required field
  (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4906`), joined against `BundlePackage.package_id` in
  `validate_descriptor` (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:938-941`).
- `TrustedCatalogLoader::load(bundle_path, profile_id, providers: &NativeCodecProviderSetV1,
  context)` (`trusted-catalog/🦀️.rs:367`) requests a binding **per selected package** via
  `providers.preview(record, &descriptor, context)` (`:424`), not one eager global vector. The
  "unconsumed-binding law" is real: `if consumed_bindings.len() != binding_map.len()` fails the
  package (`:455-457`).
- `NativeCodecProviderSetV1::linked()` (`🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:24-32`)
  is a fixed compile-time table of `(plugin_id, package_id, preview_fn)` rows, currently two rows:
  stdio (26 receipts) and gis (2 receipts, `NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS = 28` total,
  `:11`). Its own doc-comment at `:26` already says: *"VCS remains absent until its private receipt
  is verified."*

So the task's framing ("the trusted loader change from eager binding slice to selected-closure/
provider-set resolution") is **already done**. What remains for VCS is narrower than either
blueprint states: (1) a VCS-owned native-codecs receipt module, (2) one new entry in
`NativeCodecProviderSetV1::linked()`, (3) a hub `Cargo.toml` dependency line, (4) a regenerated
static descriptor pair, (5) extending the hard-coded stdio+gis bundle producer/gates to a third
package. Sections 1-6 below detail exactly these gaps, verified against current bytes.

---

## 1. Current VCS state

| Question | Answer | Evidence |
| --- | --- | --- |
| `.package_id("semio:vcs")` declared after label/version? | **Yes — done.** `.label("VCS")` (line 30) → `.version("0.1.0")` (31) → `.package_id("semio:vcs")` (32), in that order. | `✏️s/🔌️plugins/🌿️vcs/🦀️.rs:30-32` |
| `s.vcs.vcs` vs `vcs.document` factory split still present? | **No — repaired.** `VCS_DOCUMENT_SCHEMA = "vcs.vcs"` (line 5); `artifact_kind()` returns `ArtifactKindSpec { id: VCS_DOCUMENT_SCHEMA.into(), ... }` (lines 22-32), i.e. `"vcs.vcs"`, not `"vcs.document"`. No `vcs.document` string remains in any `.rs` source file under the VCS plugin — confirmed by `grep -rn "vcs\.document"` returning zero hits outside checked-in JSON/fixture/guard files. | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🦀️.rs:5,22-32`; repo-wide grep |
| Strict JSON descriptor with `packageId` exists? | **No — still stale.** The checked-in `🔣️.json` has zero occurrences of `"packageId"` and still contains two literal `"vcs.document"` values (`ArtifactKindSpec.id` at line 3875, `kind` at line 4922). This is exactly the "stale JSON derivative, not authority" state the blueprint predicted; the *source* is already fixed, only the generated pack/JSON pair has not been re-emitted. | `✏️s/🔌️plugins/🌿️vcs/🔣️.json:3875,4922` (verified via grep); regeneration path is the existing `describe` target (below) |
| Editor/viewer open target and real `ArtifactCodec::of` Io? | **Yes, both real.** Editor root `✏️editor/🦀️.rs`, viewer root `👁️viewer/🦀️.rs` both exist under `🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/`, wired into `VcsApps` in the plugin root (`🦀️.rs:9-15`, `VcsArtifactApp<EditorApp<VcsPlayApp>>` / `VcsArtifactApp<ViewerApp<VcsViewer>>`). The typed Io codec is real: `store::ArtifactCodec::of::<VcsSnapshot, VcsDemoMutation>(VCS_DOCUMENT_SCHEMA.to_string())` at `io/🦀️.rs:52`, inside `IoDeclaration { native: NativeCodecs { codec: ..., .. }, entries: entries() }`. | `✏️s/🔌️plugins/🌿️vcs/🦀️.rs:9-15`; `…/🚪️io/🦀️.rs:52` |

**Additional verified state not asked for but load-bearing:**

- VCS has **no** `📇️native-codecs` (or `📇️registry`) directory at all — `find` over the VCS plugin
  root shows only `🗿️artifacts/`, `🧪️fixtures/`, `🎮️commands/`, `📦️packages/`. There is no receipt
  module today, matching both blueprints.
- VCS's Cargo dependency on plugin crates is exactly one:
  `semio-s-plugin-stdio` (`✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/Cargo.toml:25`) — confirms the
  "smallest topological choice" reasoning in `terra-multi-provider-verified-catalog-blueprint.md`.
- A real, running identity proof already exists one layer below a receipt: an integration test
  (`✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/🧪️tests/🪪️native-openable-identity/🦀️.rs`) constructs the
  real plugin bundle, calls the real guest `describe_plugin`, decodes the real
  `PackageDescriptor`, and asserts `descriptor.package_id == "semio:vcs"`,
  `descriptor.manifest.apps[...].dialect.artifact_kind == "s.vcs.vcs"`, and
  `!format!("{descriptor:?}").contains("vcs.document")`. Per `📓️sol-vcs-native-openable-provider-v1.md`
  this specific law is registered (`@semio-tech/vcs-plugin:native-openable-identity-check`) but its
  Rust execution had not yet produced a green terminal as of that report; I did not run it, so I
  cannot upgrade that to "passing" — only "written and wired."
- The neutral fixture `🧪️fixtures/🪪️native-openable-identity/🧬️v1/🔣️.json` already models the exact
  target identity (`pluginId: vcs`, `packageId: semio:vcs`, `artifactKind: vcs.vcs`, `dialectKind:
  s.vcs.vcs`, viewer/editor app ids, `windowKindId: framework.window.tree`, `rendererTarget: wasm`,
  `execution: isolated`) plus 11 hostile cases (missing/foreign package, legacy kind, foreign
  schema/dialect/plugin, viewer/editor role or standard substitution, in-process execution, foreign
  window/renderer). This is identity-level only — it has no `factoryId`, `packSchemaHash`, or
  capability-list fields, i.e. it does not yet model a receipt.

---

## 2. What a VCS `native-codecs` receipt module must contain, and what should move out of stdio

### Reference pattern (GIS, verified line-by-line)

`✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs` (106 lines) is the exact model to follow. Its shape:

```text
enum GisCodecV1 { Map, Terrain }                      // one variant per artifact this plugin exposes
struct NativeGisCodecIdentityV1 {                     // plain data, Clone+Debug+PartialEq+Eq
    plugin_id, package_id, package_version: &'static str,
    factory_id, artifact_kind, schema, extension, capability: &'static str,
    pack_schema_hash: [u8; 32],
}
struct NativeGisCodecReceiptV1 { artifact: GisCodecV1 }  // inert until into_codec() is called
impl NativeGisCodecReceiptV1 {
    fn identity(&self) -> NativeGisCodecIdentityV1        // static match, include_bytes! protocol hash
    fn validate(&self) -> Result<(), PluginAssemblyError>  // rechecks declaration().definition() vs identity
    fn into_codec(self) -> Result<store::ArtifactCodec, _> // validate() then ArtifactCodec::of::<Snapshot,Mutation>
}
pub fn native_codec_factory_receipts() -> Result<[NativeGisCodecReceiptV1; 2], _>  // validates every receipt eagerly
```

A `vcs/📇️native-codecs/🦀️.rs` following this exactly would be a **one-variant enum** (`enum
VcsCodecV1 { Vcs }`), one receipt row, `pack_schema_hash` from
`include_bytes!("../🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/📡️.protocol.semio")`
(the exact file the VCS identity fixture already pins as `snapshotProtocolSha256`), `artifact_kind
= "s.vcs.vcs"`, `schema = "vcs.vcs"`, `extension = "vcs"` (matches the codec-extension claim already
declared in `artifacts/🌿️vcs/🦀️.rs`'s `.capability(... ArtifactCapabilityKind::codec() ...
claim(codec_extension("vcs.vcs", "vcs")) ...)`), and `into_codec()` calling
`store::ArtifactCodec::of::<VcsSnapshot, VcsDemoMutation>("vcs.vcs")` — the exact call the io
module already makes. This is a small, mechanical module (roughly 60-80 lines): one artifact, no
`Terrain`-style second variant, no `childOnly` visibility distinction to model yet.

### Shared receipt types that should move out of stdio (verified gap, not yet done)

The blueprint's call to "move the receipt contract out of `semio-s-plugin-stdio::registry` into a
narrow framework/plugin/catalog-contract module" has **not happened**. Verified:

- `NativeCodecFactoryReceipt` (the stdio type) lives at
  `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:852-864` with fields `plugin_id, package_id,
  package_version, factory_id, descriptor_codec_id, runtime_capability_id, artifact_kind, schema,
  pack_schema_hash, extension, factory: fn() -> store::ArtifactCodec`.
- GIS did **not** reuse this type. It hand-rolled its own `NativeGisCodecIdentityV1` /
  `NativeGisCodecReceiptV1` with an overlapping-but-not-identical field set (no
  `descriptor_codec_id`/`runtime_capability_id`, has `capability` singular instead) and its own
  `validate()`/`into_codec()` logic, independently duplicating `instantiate()`'s job.
- If VCS copies the GIS pattern verbatim (as this task's context instructs), the repo will have
  **three** independently-typed, structurally-similar receipt shapes (stdio, GIS, VCS) with no
  shared trait or struct. This is exactly the "no reusable selected-provider resolver" concern the
  blueprint raised, just one layer down (at the receipt-identity level, not the loader level, which
  *is* already shared via `NativeCodecProviderSourceV1`/`NativeCodecProviderSetV1`).
- **Recommendation for a future implementer** (not verified as planned by anyone): extract a
  neutral `struct NativeCodecReceiptIdentityV1 { plugin_id, package_id, package_version, factory_id,
  artifact_kind, schema, extension, pack_schema_hash: [u8;32] }` plus a
  `trait NativeCodecReceiptV1 { fn identity(&self) -> NativeCodecReceiptIdentityV1; fn
  into_codec(self) -> Result<store::ArtifactCodec, PluginAssemblyError>; }` into a framework crate
  (candidate location: alongside `semio_framework_plugin`, since both stdio and GIS already depend
  on it and it owns `PluginAssemblyError`/`ArtifactKindSpec`). stdio's 26-row macro-generated table
  and GIS's 2-row match would both adapt into this trait without behavior change; VCS would
  implement it fresh rather than adding a fourth duplicate. This is scope beyond "add VCS" — it is
  a separate refactor packet the blueprint already named but nobody has picked up.

---

## 3. Trusted loader: already generalized — what "adding VCS" actually touches

As shown in the headline, the eager-vector → selected-closure change is **done**. Concretely, wiring
VCS into that already-generic mechanism needs exactly:

1. **`🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs`**: add a third
   `NativeCodecProviderEntryV1 { plugin_id: "vcs", package_id: "semio:vcs", preview:
   preview_vcs_bindings }` to the `entries: &[...]` array at lines 27-31, plus a
   `fn preview_vcs_bindings(version: &str, context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError>`
   modeled on `preview_gis_bindings` (lines 53-70) — same shape: call the VCS
   `native_codec_factory_receipts()`, check `plugin_id/package_id/version/pack_schema_hash != 0`,
   insert into `factories`/`artifacts` `BTreeSet`s for duplicate detection, call `into_codec()`, and
   push one `NativeCodecBinding::new(...)`. Because VCS has exactly one codec, this function is
   simpler than GIS's (no need for the `factories.len()`/`artifacts.len()` post-loop count check
   GIS doesn't even do — actually GIS also skips a post-loop count check; stdio's
   `from_receipts` does have one at `NativeOpenableCatalogProviderV1::from_receipts` lines
   ~114-118). Update the constant `NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS` if it is meant to track
   the sum (currently `28`; would become `29`), and update `NATIVE_OPENABLE_PROVIDER_SET_V1_ID`
   (`"stdio+gis/native-codecs/v1"`, line 9) if a new named set is intended — or, more likely,
   **do not touch `linked()`'s existing ID/constant at all** and instead add a **second** const
   provider-set function (e.g. `NativeCodecProviderSetV1::linked_with_vcs()` or a version-suffixed
   `V2`) so that the existing stdio+gis production path is not silently changed. This is an
   open design choice, not something current source decides for you.
2. **No change needed** to `TrustedCatalogLoader::load`, `load_selected`, `validate_bundle`,
   `validate_descriptor`, or `validate_native_bindings` for a *single* new plugin/package pair —
   those are already generic over `N` packages (see §6 caveat about the hard-coded profile-id
   branch, which is a *bundle producer* concern, not a loader-generality concern).
3. **Real collision risk (verified):** `validate_bundle` in `trusted-catalog/🦀️.rs:856-884`
   contains a hard-coded `if profile.id == "local-stdio-gis-open-v1" { ... requires bundle.packages.len()
   == 2, gis.native_codecs.len() == 2, stdio.native_codecs.len() == 26 ... }` block — a
   **profile-literal special case embedded in otherwise-generic validation code**. This is squarely
   inside the file the Sol lane is actively iterating on (`📓️sol-trusted-stdio-gis-bundle.md`'s
   newest evidence entry is timestamped the same day as this read). Any edit to this file for VCS
   risks a merge collision with Sol's next commit. A `native-stdio-vcs-v1` or `native-stdio-gis-vcs-v1`
   profile would either need its own analogous `if profile.id == "..." { ... }` block (extending the
   duplication) or — better — this ad hoc block should be generalized/removed in favor of the
   generic per-codec/per-target checks the rest of the function already does. I did not verify
   whether Sol's own remaining-acceptance items (native/process gates) plan to touch this block
   further; treat it as **actively live code**.

---

## 4. Producer/profile change for a stdio+vcs (or stdio+gis+vcs) bundle, and gates to extend

### Current producer is hard-coded to exactly two packages (verified, major finding)

`🌎️hub/📦️packages/🦀️rust/📜️script.ts` implements the entire stdio+gis bundle lifecycle
(`materializeTrustedStdioGisBundle` at line 3993, `materializeTrustedStdioGisRotation` at line
4121, `trustedBootstrapSourceCodecs` at line 3983, `trustedBootstrapProfileEncoding` at line 3844,
`trustedBootstrapClosureEncoding` at line 3884) using **TypeScript literal unions `"gis" |
"stdio"`** throughout, plus hard cardinality assertions:

- `trustedBootstrapSourceCodecs` returns `Record<"gis" | "stdio", ...>` (line 3983) reading two
  fixed files: stdio's `📇️registry/🧬️schema/📜️native-codec-factories.json` and GIS's
  `📇️native-codecs/🔣️.json` (lines 3984-3985). **VCS has neither file** — no
  `native-codec-factories.json`, and the receipt module from §2 doesn't exist yet, so this function
  cannot even discover a VCS codec today.
- `if (codecs.stdio.length !== 26 || codecs.gis.length !== 2)` (line 4032) — hard 26/2 assertion.
- `bundle.packages?.length !== 2` (line 4126, in the rotation reader) and `profile.packages.length
  !== 2` (the source-oracle equivalent at line 3911) — hard 2-package assertion in **two places**.
- `for (const [plugin, receipt] of [["gis", gis], ["stdio", stdio]] as const)` (lines 4090, and the
  analogous pattern in the source oracle) — fixed 2-tuple iteration, not a loop over an arbitrary
  package list.
- The bundle producer's own `requests` array (line 4015-4018) is a literal 2-element array of
  `{ pluginId, cargoPackage, componentPackageId, outputName, componentProfile, rootCdylib }`.

**None of this is parameterized by profile.** Adding VCS as a third package requires either (a) a
parallel `materializeTrustedStdioGisVcsBundle` function (duplicating ~150 lines with `"vcs"` spliced
into every union/tuple/count), or (b) generalizing these functions to iterate an arbitrary
`readonly PackageRequest[]` — the latter is the clean long-term fix per CLAUDE.md's "aim for clean
long term solution" rule, but it is a non-trivial rewrite of code the Sol lane owns and is actively
extending (rotation, cancellation, candidate-hub proof, receipt-exchange ordering all live in this
same function family, per `📓️sol-trusted-stdio-gis-bundle.md`'s "Remaining acceptance" section,
items 1-4, none of which have run yet). **This is the single largest collision-risk file for this
lane.**

### Gates to extend (verified registrations)

| Gate | Registration | Current scope (verified) | VCS extension needed |
| --- | --- | --- | --- |
| `os-hub:trusted-stdio-gis-bundle-check` | `🌎️hub/📦️packages/🦀️rust/📋️project.json:287-295`, class `TrustedStdioGisBundleCheckScript` registered at `📜️script.ts:5722` | `--source` (oracle+source), `--native` (builds both components), `--process` (rotation A→B) — all literally "stdio+gis" | Needs a sibling gate (new class/target, e.g. `trusted-stdio-vcs-bundle-check` or a generalized `trusted-catalog-bundle-check --profile <id>`) rather than an in-place rename, to avoid breaking the existing stdio+gis acceptance history |
| `os-hub:native-openable-catalog-provider-check` | `📋️project.json:239-246` | Stdio-only 26-receipt bijection/hostile-substitution laws (per `📓️sol-native-openable-catalog-provider-v1.md`) | Needs an analogous VCS-only law set (bijection + missing/extra/duplicate/hash/factory-substitution), or folding VCS into a generalized "provider-set" gate alongside stdio and gis |
| `os-hub:native-catalog-selection-check` | `📋️project.json:231-238`, `NativeCatalogSelectionCheckScript` in `🧰️framework/…/📇️registry/📜️script.ts:2896` | A **planning-only** Bun algorithm (`planNativeCatalogSelectionV1`) with a neutral fixture that already **models** `native-stdio-vcs-v1` as test data (26 stdio + 1 VCS declared receipt count) per `📓️sol-native-catalog-selection-foundation.md` — but this is data-only; it does not call any real Rust provider | No source change strictly required to add real VCS — the fixture already anticipates it; when a real receipt exists, the planning model and the actual provider-set outcome should be cross-checked (nobody currently does this) |
| `@semio-tech/vcs-plugin:native-openable-identity-check` | `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📋️project.json:6-13` | Already registered and covers identity-only (11 hostile cases), **not** a receipt | Extend this gate — or add a sibling `native-codecs-check` — once §2's receipt module exists |
| `@semio-tech/vcs-plugin:describe` | `📋️project.json:37-43`, calls shared `describePluginComponent` | Builds VCS's own `wasm32-wasip2` cdylib and re-emits `🛂️.descriptor.semio` + `🔣️.json` | This is the exact mechanism to fix the stale `vcs.document`/missing-`packageId` JSON from §1 — running it should regenerate both files from the now-correct Rust source. **Not run by this audit; whether VCS's own cdylib build hits the "Registry" row's wasm-component-ld million-function-count blocker (`✅️acceptance-matrix.md:27`) is inferred-unknown** — that blocker is specifically tied to stdio's own 176-variant `StdioApps` `dyn_enum_close!` closure inside stdio's *own* component; VCS's cdylib only depends on stdio as a Rust library (not stdio's `PluginApp` surface), so it is plausible VCS's own component build is unaffected, but I did not build it and cannot confirm. |

### What a `native-stdio-vcs-v1` bundle producer needs, additionally

Per `📓️terra-multi-provider-verified-catalog-blueprint.md` §"P2", the dependency list in the bundle
must come from VCS's **emitted descriptor**, not copied from `Cargo.toml` — `BundlePackage.dependencies`
(trusted-catalog/🦀️.rs:129) is a `Vec<BundleIdentity>` the loader cross-checks at lines 811-818
(`validate_bundle`) against each package's own `dependencies` field, so the producer must read the
real fresh VCS descriptor's dependency list (once one exists) rather than hand-writing `[{pluginId:
"stdio", ...}]`. Nothing in current source does this derivation generically yet — the stdio+gis
producer has zero cross-package dependencies to model (GIS's `dependencies: []` at line 4064,
stdio's likewise), so this exact code path (a non-empty `dependencies` array with real hash-checked
identity) is **untested by current source**, another concrete "first" for a VCS packet.

---

## 5. Test packet and nonclaims

### Neutral fixture / oracle (new, VCS-specific)

Model directly on the two existing patterns already in the repo:

- **Identity fixture** (already exists, extend or add a sibling):
  `✏️s/🔌️plugins/🌿️vcs/🧪️fixtures/🪪️native-openable-identity/🧬️v1/🔣️.json` — add `factoryId`,
  `packSchemaHash` fields once the receipt exists, or create a new
  `🧪️fixtures/🪪️native-codec-receipt/🧬️v1/🔣️.json` modeled on
  `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🌍️gis-v1/🔣️.json` (8 cases:
  exact package, cross-package owner, cross-plugin owner, wrong version, unknown provider,
  cancelled-before-preview, deadline-at-preview, deadline-before-preview — this exact 8-case shape
  is schema `semio.hub.gis-native-provider-selection/v1` and is trivially portable to VCS by
  substituting `gis`/`semio:gis` for `vcs`/`semio:vcs`).
- **Bijection/hostile fixture** (new, model on stdio's `catalog-root`/`native-codec-factories.json`
  pattern and GIS's own 2-receipt fixture at `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🔣️.json`): one
  positive VCS receipt row, plus hostile substitutions for factory id, artifact kind, schema,
  extension, pack-schema hash (zero and substituted), and package id/version.
- **AJV oracle**: reuse the exact pattern in `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📜️script.ts`'s
  `NativeOpenableIdentityCheckScript` (lines 22-50) — strict-mode AJV, `structuredClone` +
  single-field mutation per hostile case, `crypto.subtle.digest` + Node `createHash` cross-check on
  the pinned protocol file. This is a proven, reusable in-repo idiom, not a new design.

### Rust receipt laws (new)

Model exactly on GIS's `📇️native-codecs/🧪️tests/🦀️.rs` (not read in full by this audit, but its
sibling module signature is visible from `native-codecs/🦀️.rs`'s public surface): construct the
receipt, call `into_codec()`, assert schema/extension/pack-hash equality, then hostile-mutate each
identity field via the same technique stdio's own tests use (`native_openable_provider::tests`,
`🦀️.rs:141-183`: missing/extra/duplicate receipts, wrong version/plugin, zero hash, wrong schema,
substituted factory — all via cloning a `Vec` and mutating one field before asserting `is_err()`).

### Hub link law (new)

Model on `native_openable_provider::tests::native_openable_provider_rejects_identity_hash_schema_and_factory_substitution`
(`🦀️.rs:167-183`) and the GIS-specific tests implied by `preview_gis_bindings`'s checks (lines
57-68) — no such VCS-specific test module exists yet since `preview_vcs_bindings` doesn't exist.

### Process law (new)

Model on `📓️sol-trusted-stdio-gis-bundle.md`'s registered `--process` gate description: publish
generation A (stdio+gis, or stdio+vcs), require a wrong/missing-profile candidate to fail without
disturbing current A, then reissue generation B and require A's authenticated plan to fail while
B's succeeds. **This entire process law family is itself unrun for stdio+gis** ("Remaining
acceptance" items 1-4 in that report) — a VCS process law is therefore blocked on the *same*
unresolved stdio+gis process proof landing first, since it is presumably built by extending the
identical harness function.

### Honest nonclaims (explicit, per CLAUDE.md's "must not assume" rule)

- No VCS receipt, provider entry, or hub Cargo dependency exists in current source. `grep` for
  `semio-s-plugin-vcs` in `🌎️hub/📦️packages/🦀️rust/Cargo.toml` returns nothing (only
  `semio-s-plugin-stdio` and `semio-s-plugin-gis` are linked, lines 39-40).
- The checked-in VCS descriptor JSON (`🔣️.json`) is stale (missing `packageId`, still has
  `vcs.document` at two locations) — regenerating it via the already-registered `describe` target
  has not been run by this audit or, per the ticket reports read, by anyone yet with the corrected
  source.
- The VCS guest-descriptor integration test exists in source but its own report
  (`📓️sol-vcs-native-openable-provider-v1.md`) says the native Rust law itself had not produced a
  passing terminal as of its last entry — I did not re-run it, so its current pass/fail state is
  **unknown to this audit**, only its existence and shape are verified.
- The stdio+gis bundle producer/process gates this VCS work would extend are themselves not fully
  accepted yet (`📓️sol-trusted-stdio-gis-bundle.md` "Remaining acceptance" 1-4 all unresolved) —
  any VCS packet inherits that dependency.
- The "Registry" acceptance-matrix row is BLOCKED on a `wasm-component-ld` function-count ceiling
  for stdio's own monolithic component (`✅️acceptance-matrix.md:27`). Whether this blocks a real
  (non-source-only) VCS fresh-build proof is **inferred, not verified** either way.
- No browser/WGPU/MCP rendering, no client document-open, no "all-plugin" claim is made or implied
  anywhere in this report.

---

## 6. Dependency-ordered file list, change-size estimates, and collision risks

| Order | File | Change | Est. size | Collision risk |
| --- | --- | --- | --- | --- |
| 1 | `✏️s/🔌️plugins/🌿️vcs/🔣️.json` + `🛂️.descriptor.semio` | Regenerate via existing `describe` target — no hand edit | 0 lines hand-written (generated) | Low — mechanical, but must run after any further source change to `🦀️.rs`/`artifacts/🌿️vcs/🦀️.rs` |
| 2 | `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🦀️.rs` (new file) | New module modeled on GIS's `📇️native-codecs/🦀️.rs` (106 lines) | ~60-90 lines | Low — new file, no existing owner |
| 2b | `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🧪️tests/🦀️.rs` (new) + `🔣️.json`/`🧬️.schema.json` fixtures (new) | Model on GIS's sibling files | ~40-60 lines Rust + ~30 lines JSON/schema | Low |
| 3 | `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/Cargo.toml` | Add `[[test]]` entry for new native-codecs test, if not inlined into lib | ~3-5 lines | Low |
| 4 | `🌎️hub/📦️packages/🦀️rust/Cargo.toml` | Add `semio-s-plugin-vcs = { path = "...", default-features = false }` after line 40 | 1 line | **Medium** — this file is a natural point of contention if any other lane is also wiring a new provider dependency concurrently; check `git status`/recent log before editing |
| 5 | `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs` | Add `preview_vcs_bindings` fn (~15-20 lines, modeled on `preview_gis_bindings` lines 53-70) + one new entry in `linked()`'s array (line 27-31) + decide on constant naming (see §3.1) | ~25-30 lines | **Medium** — small file (183 lines), single well-scoped edit, but is the exact file `sol-native-openable-catalog-provider-v1`/`sol-vcs-native-openable-provider-v1` lanes are also targeting next per their own "next VCS packet" language |
| 6 | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` | **Only if** a hard-coded closed-profile check (like the `local-stdio-gis-open-v1` block at lines 856-884) is wanted for a new VCS profile id | 0 lines (no change needed for loader generality) or ~25-30 lines (if following the existing ad hoc pattern) | **High** — 1882-line file under active, very recent edits by the Sol lane (same-day evidence in `📓️sol-trusted-stdio-gis-bundle.md`); do not touch without reconfirming HEAD |
| 7 | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | Extend/parallel the `"gis" \| "stdio"` literal-union bundle producer (§4) to include `"vcs"`, or add a wholly separate `materializeTrustedStdioVcsBundle` | 150-300 lines (parallel function) or 200-400 lines (generalizing all touched functions to N packages) | **Highest** — 5730-line file, the single most actively-changing file in this frontier per the ticket's own evidence trail (11+ registered-gate commits listed in `📓️sol-trusted-stdio-gis-bundle.md`'s evidence table, most within the last two days) |
| 8 | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | New Nx targets (`trusted-stdio-vcs-bundle-check` or similar, `--native`/`--process` variants) | ~15-25 lines | Medium — additive only, low structural risk, but must follow existing target-ordering convention CLAUDE.md requires |
| 9 | Registry launch generator (`🧰️framework/…/📇️registry/📜️script.ts` `generateLaunchJson`) | No direct edit — running `@semio-tech/plugin-registry:generate` picks up new project.json targets automatically, per existing convention (`⚖️gate🪪️native-openable-identity🌿️vcs` was already auto-generated this way) | 0 hand-written lines | Low, but **must be re-run** after any project.json change or `check-generated` will fail |

**Total estimated net-new/changed lines for the smallest complete VCS receipt+link packet (items
1-5, 8-9, excluding the bundle-producer generalization in item 7):** roughly 150-250 lines across 7
files/dirs, most of it mechanical (modeled 1:1 on GIS). **Item 7 (the bundle producer) is
disproportionately the largest and riskiest piece** — it alone could be 150-400 lines depending on
whether a parallel function or a genuine N-package generalization is chosen, and it directly
overlaps the file the Sol lane is iterating on right now.

## Explicit collision summary (for the coordinator)

- **`trusted-catalog/🦀️.rs`** and **hub `📜️script.ts`** are both live-edit targets of the Sol
  "trusted-stdio-gis-bundle" lane as of this same read. Any VCS work touching either file should
  re-diff against HEAD immediately before editing, not rely on this audit's line numbers staying
  exact.
- **`native-openable-provider/🦀️.rs`** is explicitly called out in its own source comment (line 26)
  as the intended VCS insertion point — low ambiguity, but confirm no one has already added a VCS
  entry between this read and any future edit.
- The **Fable execution-target lease** (mentioned in this task's own briefing as adding a
  selection-bound asset accessor to the trusted catalog) was not located or read by this audit —
  I found no file matching that description under `🌎️hub/🗿️artifact-authority/`. If that lane is
  real and active, it is a second concurrent editor of `trusted-catalog/🦀️.rs`/`🔏️trusted-catalog`
  beyond Sol; I could not verify its existence or scope from source alone and flag this as an
  **open question for the coordinator**, not a verified risk.
