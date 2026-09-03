# Generation3d vs Lowpoly App Infrastructure Analysis

## Overview
Comparison of infrastructure patterns between the fully-migrated lowpoly reference app and the partially-migrated generation3d app.

---

## 1. Preparation Factories

### Lowpoly App Infrastructure
- **ArtifactStorePreparationFactory**: `LowpolyArtifactStorePreparationFactory`
  - Defined: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1244`
  - Registered: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1599-1601` (method `build_artifact_store_one_item_preparation_factory()`)

- **ConfigStorePreparationFactory**: `LowpolyConfigStorePreparationFactory`
  - Defined: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1385`
  - Registered: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1603-1605` (method `build_config_store_one_item_preparation_factory()`)

### Generation3d App Infrastructure
**Status**: ABSENT

- No `Generation3dArtifactStorePreparationFactory` defined
- No `Generation3dConfigStorePreparationFactory` defined
- No `build_artifact_store_one_item_preparation_factory()` override in `Generation3dPlayApp`
- No `build_config_store_one_item_preparation_factory()` override in `Generation3dPlayApp`

---

## 2. Command Job Factory

### Lowpoly App Infrastructure
- **Type**: `LowpolyCommandJobFactory`
  - Defined: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:979-1040`
  - Implements: `semio_framework::ToolJobFactory` + `semio_framework_plugin::ArtifactOwnedToolJobFactory`
  - Registered: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1607-1641` (via `bounded_first_step_tool_proofs!` macro with `factory: "LowpolyCommandJobFactory"`)

### Generation3d App Infrastructure
- **Type**: Framework-provided `BoundedFirstStepCommandJobFactory` (NOT app-owned)
  - Declared: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:210-244` (via `bounded_first_step_tool_proofs!` macro with `factory: "BoundedFirstStepCommandJobFactory"`)
  - No custom implementation

---

## 3. Transient Type

### Lowpoly App Infrastructure
- **Type**: `LowpolyTransient`
  - Imported: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:20`
  - Defined in: `crate::editor::lowpoly::session::LowpolyTransient`
  - Used by app: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1591` (as `type Transient = LowpolyTransient;`)
  - Purpose: Holds mid-gesture scratch state (paint runs, open bytes tracking) — see line 975

### Generation3d App Infrastructure
- **Type**: `semio_framework_plugin::NoTransient` (framework default, not app-owned)
  - Declared: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:167`
  - No custom transient state; uses framework's empty transient

---

## 4. ArtifactStore Construction Path

### Lowpoly App Infrastructure
**Verdict**: Construction path UNCONFIRMED
- No explicit `from_new` or `from_initialized_runtime_with_owners` call found in lowpoly's artifact schema files
- Likely uses default framework path (to be verified in framework plugin opening logic)
- **Status of snapshot_retirement_factory**: UNKNOWN (depends on construction path)

### Generation3d App Infrastructure
**Verdict**: Uses `from_initialized_runtime_with_owners` ✓ CONFIRMED
- **Call site**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3328`
- **Associated factory provider**: `generation3d_document_store_owners()` — `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:820`
- **Status of snapshot_retirement_factory**: INSTALLED ✓
  - `generation3d_document_store_owners()` returns `store::MemberStoreOwners::new(snapshot_retirement_factory, initial_snapshot_retirement_factory, mutation_retirement_factory, disposer)`
  - These factories are passed to `ArtifactStore::from_initialized_runtime_with_owners()` which assigns them

---

## 5. Handler Implementation Status for Six Pending Rewrite Actions

All six actions have command handlers already implemented:

| Action | Handler Location | Status |
|--------|------------------|--------|
| `addGeneration` | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️add-generation/🦀️.rs:57` | ✓ Handler exists |
| `removeGeneration` | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️remove-generation/🦀️.rs:59` | ✓ Handler exists |
| `renameGeneration` | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️rename-generation/🦀️.rs:60` | ✓ Handler exists |
| `updateGenerationValues` | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️update-generation-values/🦀️.rs:61` | ✓ Handler exists |
| `selectGeneration` | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️select-generation/🦀️.rs:18` | ✓ Handler exists |
| `nodeGraphEdit` | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs:58` (handle) + line 66 (apply) | ✓ Handler exists |

All six actions already have their `handle()` functions implemented; they are only missing the classification/wiring to be marked as non-`BatchOnlyPendingRewrite`.

---

## Key Findings Summary

| Aspect | Lowpoly | Generation3d |
|--------|---------|--------------|
| **Custom ArtifactStorePreparationFactory** | ✓ Yes (LowpolyArtifactStorePreparationFactory) | ✗ No |
| **Custom ConfigStorePreparationFactory** | ✓ Yes (LowpolyConfigStorePreparationFactory) | ✗ No |
| **Custom CommandJobFactory** | ✓ Yes (LowpolyCommandJobFactory, app-owned) | ✗ No (uses BoundedFirstStepCommandJobFactory) |
| **Custom Transient Type** | ✓ Yes (LowpolyTransient with scratch state) | ✗ No (uses NoTransient) |
| **ArtifactStore Construction** | ? Unknown (unconfirmed path) | ✓ from_initialized_runtime_with_owners |
| **snapshot_retirement_factory Installed** | ? Depends on construction path | ✓ Yes |
| **Handler Logic Implemented** | N/A (lowpoly is fully migrated) | ✓ All 6 pending actions have handlers |
| **Handlers Only Need Wiring** | N/A | ✓ Yes (classification/registration only) |

---

## Conclusions

1. **Generation3d lacks app-owned infrastructure** compared to lowpoly:
   - No custom preparation factories (uses framework defaults)
   - No custom transient type (uses NoTransient)
   - Uses framework's BoundedFirstStepCommandJobFactory instead of app-owned factory

2. **Generation3d's snapshot_retirement_factory is installed** via `from_initialized_runtime_with_owners`, so the known runtime fault ("snapshot read retirement factory is not installed") should NOT occur for generation3d.

3. **All six pending rewrite actions have handlers already implemented**; they are only awaiting classification wiring (`BatchOnlyPendingRewrite` → migrated classification).

4. **Lowpoly's construction path needs clarification** — it should be traced through the framework plugin opening logic to confirm whether it takes the `from_new` path (which would leave snapshot_retirement_factory as None).

