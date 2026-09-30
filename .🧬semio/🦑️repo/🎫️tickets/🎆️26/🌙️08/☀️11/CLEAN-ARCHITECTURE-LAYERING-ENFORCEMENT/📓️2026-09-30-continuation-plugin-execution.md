# Continuation Plugin Execution

## Scope

Neutral caller-authored Stdio contribution assembly and format-independent simulation weather, with concrete catalog and EPW mapping owned by outward composition consumers. Root plugin fleet/package deployment remains outside this increment; moving selection does not prove package-level removal.

## Implemented Behavior

`ContributionRegistry` lives in the neutral artifact contract and accepts exactly the caller-authored roster, including an empty roster. Registration and removal validate atomically; removal of an owner with surviving declared dependents fails without changing the roster. Schema, identity, directory, MIME, extension, dialect, runtime capability, definition/assembly owner, and native receipt authorization/uniqueness checks remain enforced. Unknown format owners fail without a global fallback. The neutral implementation imports no concrete artifact and includes no fixed catalog asset.

The concrete `🔌️plugin/📇️catalog` owner retains shipped selection, native codec factories, PluginBuilder assembly and its serialized projection. `artifact_catalog_contribution_for` builds definitions and receipts from a supplied registered selection, rejects foreign assembly owners, and supports successive removal through an empty catalog. Concrete serialized receipt IDs must exactly match the authored shipped factories; selected receipts must match every serialized ownership field. Shipped catalog count assertions remain in regression tests only. All direct consumers and the trusted Stdio projection locator use the concrete owner; the removed registry facade is not reexported.

The general simulation service exports `WeatherData` and `WeatherRecord` without EPW type/import references. The specific `🧩️extensions/🌦️epw` consumer owns stdio decoding and preserves all numeric field mapping, including hour-ending conversion and humidity division. The general BESTEST runner consumes WeatherData; EPW parsing plus run convenience belongs to that consumer. Existing parser functions are retained there. Portable WeatherData/WeatherRecord schemas and EPW vectors separate neutral outputs from specific input syntax.

## Verification

- Registered `@semio-tech/stdio-artifact-contract-rs:test -- --lib contribution_tests`: passed, 3 native tests, 62 unrelated tests filtered, 3m16s (Nextest run c4998e16-49f8-4ef4-b268-eda8dc22c0c5).
- Registered `@semio-tech/stdio-artifact-contract-rs:canonical-architecture`: passed, 21 checks, 3.5s. Ajv validates portable removal/registration/receipt vectors and variable-cardinality native commitment; serde_json independently evaluates dependencies, claimed MIME/extension/directory uniqueness, and unauthorized receipt closure.
- Registered `@semio-tech/energy-model-rs:canonical-architecture`: passed, 6 checks, 5.3s. Ajv validates specific EPW vectors against format-independent WeatherData/WeatherRecord schemas; source check rejects EPW mentions/imports in the general weather service.
- Actual Stdio catalog and Energy EPW native routes remain running.
- Initial native dependency build failed because framework-schema did not resolve its async dependency; retry compiled once the existing manifest dependency appeared. No source change to that dependency was needed here.
- First authored native removal run failed on incorrect neutral fixture `is_alpha`/`is_bravo`/`is_charlie` keys; corrected to `is_binary`. Both native and oracle tests then passed.
- Initial new Nx source target was absent from a cached graph; uncached project graph discovery resolved the target and both source routes passed.
- Initial Energy native route failed during plugin-registry generation because its receipt projection locator searched the old registry owner. Updated that existing Stdio-specific locator to the concrete catalog owner, with no compatibility fallback.

## Runtime Console Evidence

The exact newly built native contract test binary ran `contribution_tests --nocapture`, executing all three selected laws, exit 0:

```text
[DEBUG] removal="independent-removal" accepted=true remaining=["alpha"]
[DEBUG] removal="empty-catalog" accepted=true remaining=[]
[DEBUG] removal="absent-removal" accepted=true remaining=["alpha"]
[DEBUG] removal="dependent-removal-refused" accepted=false remaining=["alpha", "charlie"]
[DEBUG] removal="dependent-before-owner" accepted=true remaining=[]
```

Temporary registry DEBUG logging was removed after this evidence. Native weather and concrete catalog evidence remains pending.


## Exact File Attribution

Cargo overlap: only the Energy model manifest’s new dev-only `csv` dependency belongs to this execution; metadata role canonicalization belongs to root. Launch entries belong to root. Shared consumer files claim only the direct Stdio API/projection paths described above.

### Updated

```text
✏️s/🔌️plugins/🌍️gis/🦀️.rs
✏️s/🔌️plugins/🌿️vcs/🦀️.rs
✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs
✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/🦀️.rs
✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/🧪️tests/🔬️unit/🦀️.rs
✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📍️site/🦀️.rs
✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📍️site/🧪️tests/🔬️unit/🦀️.rs
✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧪️tests/🔬️unit/🦀️.rs
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/📋️project.json
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/📜️script.ts
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧫️fixtures/📇️native-catalog-surface/🧪️imports.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/📋️project.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/📜️script.ts
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🔣️.json
✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📋️project.json
✏️s/🔌️plugins/🗄️stdio/🔌️plugin/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/🛂️manifest/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/🧪️tests/📇️native-openable-provider/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs
🌎️hub/📦️packages/🦀️rust/📜️script.ts
🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs
🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️tests/🔬️unit/🦀️.rs
🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🪪️v1/🔣️.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🛫️catalog-selection-preflight/🔣️.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json
🌎️hub/🧪️tests/⛓️linked-codec-ownership/🟦️.ts
🌎️hub/🧪️tests/🔏️trusted-catalog-profile/🦀️.rs
🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️trusted-stdio-catalog/🟦️.ts
```

### Created

```text
✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📍️site/🧬️schema/🔣️.json
✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/🦀️.rs
✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/🧪️tests/🔬️unit/🟦️.ts
✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/🧪️tests/🔬️unit/🦀️.rs
✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/🧫️fixtures/🌦️weather/🔣️.json
✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/🧬️schema/🔣️.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🟦️.ts
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧫️fixtures/📇️contributions/alpha.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧫️fixtures/📇️contributions/bravo-directory.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧫️fixtures/📇️contributions/bravo-extension.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧫️fixtures/📇️contributions/bravo-mime.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧫️fixtures/📇️contributions/bravo.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧫️fixtures/📇️contributions/charlie.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧫️fixtures/📇️contributions/🔣️.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/📇️contributions/🔣️.json
✏️s/🔌️plugins/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json
✏️s/🔌️plugins/🗄️stdio/🔌️plugin/📇️catalog/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/🔌️plugin/📇️catalog/🧪️tests/🔬️catalog-projection-budget/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/🔌️plugin/📇️catalog/🧪️tests/🔬️component-package-id/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/🔌️plugin/📇️catalog/🧪️tests/🔬️unit/🦀️.rs
```

### Removed

```text
✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️tests/🔬️catalog-projection-budget/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️tests/🔬️component-package-id/🦀️.rs
✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️tests/🔬️unit/🦀️.rs
```


## Limits and Ownership

The root Stdio plugin still statically imports its concrete catalog and selected artifact fleet. This increment proves authored selection and removal at the contribution assembly seam; it does not prove complete package/fleet removal or new WASM deployment behavior. The nine Stdio family extension components still have PluginBuilder worlds and descriptors and retain their plugin metadata; root’s stricter physical taxonomy exposes that existing mismatch. The general Energy engine remains physically mounted by the Energy model artifact; this work removes its specific format types, without relabeling its ownership. The existing framework trusted Stdio locator remains Stdio-specific tooling; its projection path correction is required for the actual registered native route after the concrete asset move. Global Cargo direction failures and full physical extraction remain root’s separately reported inventory.
