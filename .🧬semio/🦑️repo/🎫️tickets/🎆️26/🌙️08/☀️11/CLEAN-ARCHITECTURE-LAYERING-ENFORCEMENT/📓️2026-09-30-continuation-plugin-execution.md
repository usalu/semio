# Continuation Plugin Execution

## Scope

Neutral caller-authored Stdio contribution assembly and format-independent simulation weather, with concrete catalog and EPW mapping owned by outward composition consumers. Root plugin fleet/package deployment remains outside this increment; moving selection does not prove package-level removal.

## Implemented Behavior

`ContributionRegistry` lives in the neutral artifact contract and accepts exactly the caller-authored roster, including an empty roster. Registration and removal validate atomically; removal of an owner with surviving declared dependents fails without changing the roster. Schema, identity, directory, MIME, extension, dialect, runtime capability, definition/assembly owner, and native receipt authorization/uniqueness checks remain enforced. Unknown format owners fail without a global fallback. The neutral implementation imports no concrete artifact and includes no fixed catalog asset.

The concrete `🔌️plugin/📇️catalog` owner retains shipped selection, native codec factories, PluginBuilder assembly and its serialized projection. `artifact_catalog_contribution_for` builds definitions and receipts from a supplied registered selection, rejects foreign assembly owners, and supports successive removal through an empty catalog. Concrete serialized receipt IDs must exactly match the authored shipped factories; selected receipts must match every serialized ownership field. Shipped catalog count assertions remain in regression tests only. All direct consumers and the trusted Stdio projection locator use the concrete owner; the removed registry facade is not reexported.

The general simulation service exports `WeatherData` and `WeatherRecord` without EPW type/import references. The specific `🧩️extensions/🌦️epw` consumer owns stdio decoding and preserves all numeric field mapping, including hour-ending conversion and humidity division. The general BESTEST runner consumes WeatherData; EPW parsing plus run convenience belongs to that consumer. Existing parser functions are retained there. Portable WeatherData/WeatherRecord schemas and EPW vectors separate neutral outputs from specific input syntax.

## Verification

- Registered `@semio-tech/stdio-artifact-contract-rs:test -- --lib contribution_tests`: passed, 3 native tests, 62 unrelated tests filtered, 3m16s (Nextest run c4998e16-49f8-4ef4-b268-eda8dc22c0c5).
- Registered `@semio-tech/stdio-artifact-contract-rs:canonical-architecture`: passed, 21 checks, 3.5s; final source rerun passed 21 checks in 1.3s. Ajv validates portable removal/registration/receipt vectors and variable-cardinality native commitment; serde_json independently evaluates dependencies, claimed MIME/extension/directory uniqueness, and unauthorized receipt closure.
- Registered `@semio-tech/energy-model-rs:canonical-architecture`: passed, 6 checks, 5.3s; final source rerun passed 6 checks in 2.0s. Ajv validates specific EPW vectors against format-independent WeatherData/WeatherRecord schemas; source check rejects EPW mentions/imports in the general weather service. After also covering the general BESTEST service, the final registered rerun passed all 7 checks in 520ms.
- Registered `@semio-tech/stdio-plugin:test -- --lib catalog::tests`: passed, 6 native tests, 5 skipped, 27m4s including 10 dependency tasks (Nextest run fd2bd853-e529-4aa7-8988-9564591a7309). The exact resulting binary then ran `catalog:: --nocapture`: 9 passed, 1 ignored measurement, 1 filtered; this additionally executed component package identity and both projection budget laws.
- First full Energy EPW native route reached the actual test crate after 27m42s and failed on two remaining string-weather BESTEST call sites. The general report API now consumes WeatherData; the specific EPW report convenience owns parsing, and the EnergyPlus subject calls that consumer. The registered retry passed 4 native tests, 6,298 skipped, 2m28s including 10 dependency tasks (Nextest run 82dc48e1-d991-4527-9999-b616b0f088ae). The exact resulting native binary reproduced all 4 laws with console evidence. A diagnostic `cargo test -p semio-s-artifact-energy-model --lib --no-run --message-format=short` also compiled the corrected test crate successfully; no duplicate diagnostic builds remain running.
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

Temporary registry DEBUG logging was removed after this evidence. The exact newly built Stdio native binary ran the complete catalog filter, exit 0, including executable instantiation of every surviving reduced receipt:

```text
[DEBUG] selected catalog definitions=3 codecs=2
[DEBUG] selected catalog definitions=2 codecs=1
[DEBUG] selected catalog definitions=1 codecs=0
[DEBUG] selected catalog definitions=0 codecs=0
```

Temporary concrete catalog DEBUG logging was removed after this evidence. The final registered Stdio route after removing logging ran the complete `catalog::` filter: 9 passed, 2 skipped, 5m21s including dependencies (Nextest run 245805a1-c134-431f-8858-243577d9ebd9).

The exact resulting Energy native binary ran `epw::tests --nocapture`, exit 0:

```text
[DEBUG] weather="hour-ending-one" admitted=true records=1
[DEBUG] weather="hour-ending-twenty-four" admitted=true records=1
[DEBUG] weather="short-record" admitted=false records=0
[DEBUG] weather="invalid-number" admitted=false records=0
```

Temporary weather DEBUG logging was removed afterward; this final change removes a test print only. The same built Energy binary passed the `site::tests` filter (22 laws, including 2 weather/site laws and 20 matching site mutation laws), `synthetic_annual_weather_decodes_to_a_full_year` (1 law), and `annual_run_is_deterministic` (1 law, 15.32s). The latter executes two full-year simulations using the moved specific decoder and the neutral BESTEST input API. No full EnergyPlus conformance or scenario-host run is claimed.


## Exact File Attribution

Cargo overlap: only the Energy model manifest’s new dev-only `csv` dependency belongs to this execution; metadata role canonicalization belongs to root. Launch entries belong to root. Shared consumer files claim only the direct Stdio API/projection paths described above.

### Updated

```text
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️simulate-bestest-energyplus/🦀️.rs
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

Root final strict Cargo regression passed 6 tests/299 assertions. Root’s final live source inventory remains red: 301 strict edges and nine extension owner-role mismatches across 3,172 declarations/278 packages. Those are not advertised as resolved by this bounded registry/consumer extraction.

## Completion and Cleanup

40 production/test paths updated, 21 created, and 4 removed, expanded above; this retained report is an additional created ticket path. All owned temporary DEBUG prints were removed. Scoped `git diff --check` passed. All seven new/moved Rust include/path owners resolve to existing targets. No legacy Stdio registry facade or `EpwWeather` references remain in the changed production consumers. No `🗑️generated/continuation-plugin` subtree exists to remove; no permanent inputs/reports were deleted, and shared caches/other agents’ generated output were preserved. No owned build or test sessions remain running.
