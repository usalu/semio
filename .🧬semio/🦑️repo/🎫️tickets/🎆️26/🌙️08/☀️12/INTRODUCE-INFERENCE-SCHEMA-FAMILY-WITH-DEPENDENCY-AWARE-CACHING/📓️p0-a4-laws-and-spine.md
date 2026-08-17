# Part 1 Law-Test Coverage Audit

## Coverage Summary

Across all 72 inference families (documented at `/Users/ueli/Documents/semio/✏️s/🔌️plugins/*/🗿️artifacts/*/🏅️standards/*/🪆️subsets/✳️any/🧬️schema/💡️inferences`):

| Law | Count | Status |
|-----|-------|--------|
| `inference_determinism_law` | 72/72 | COMPLETE |
| `inference_default_law` | 72/72 | COMPLETE |
| `inference_cache_transparency_law` | 0/72 | **MISSING** |
| `inference_incrementality_law` | 0/72 | **MISSING** (required for ~70+ DAG-shaped families) |

**Families with all 3 mandatory laws (DET+DEF+CACHE): 0/72**

**DAG-shaped families (with parent/child relationships, requiring INCR): ~70 families** — includes all stdlib artifacts (stdio), most domain plugins (mathematical, procedural2d/3d, flow, norm chain, gis, etc.).

---

## Part 2 Law Quality Audit

### Test Exemplar Pattern

Reference implementation: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs` (lines 117-137)

**Laws' intended shape:**
- `inference_determinism_law`: Takes a real fixture (`chain_snapshot()`), calls `infer()` twice, asserts equality. Tests stable computation.
- `inference_default_law`: Asserts `infer(&Snapshot::default()) == Inference::default()`. Tests the law, but snapshot is empty.
- Additional custom assertion (line 129-136): Tests specific output properties (e.g., `assert_eq!(inferred.flat_positions.get(id), Some(pose))`).

### Vacuous Test Sample (10 families examined)

| Family | Tests | Determinism Fixture | Default Test | Custom Test | Verdict |
|--------|-------|---------------------|--------------|-------------|---------|
| ✒️writer | 2 | **Empty** `WriterSnapshot::default()` | Empty only | None | **VACUOUS** |
| ➗️mathematical | 3 | Empty `MathematicalSnapshot::default()` | Empty only | Real (diamond graph) | MIXED (custom saves it) |
| 🌀️procedural2d | 3 | Real `sample_snapshot()` (3 widgets + 2 synapses) | Empty only | Real (chain topology) | GOOD |
| 🌊️flow | 3 | Real fixture | Empty only | Real | GOOD |
| 🧩️puzzle3d | 3 | Real `chain_snapshot()` (3-object chain) | Empty only | Real (matches flatten) | GOOD |
| 🗄️stdio/svg | 2 | **Empty** `SvgSnapshot::default()` | Empty only | None | **VACUOUS** |
| 🗄️stdio/md | 2 | **Empty** `MdSnapshot::default()` | Empty only | None | **VACUOUS** |
| 🔱️trinity/jack | 2 | Empty default | Empty only | None | **VACUOUS** |
| 🧱️block/3d | 2 | Empty default | Empty only | None | **VACUOUS** |
| 🪵️sourcing/curate | 2 | Empty default | Empty only | None | **VACUOUS** |

**Vacuous test count: At least 6 families** (writers, svg, md, jack, block/3d, curate) where both law tests use empty defaults with zero meaningful assertions. The determinism law in these cases asserts that `infer(empty) == infer(empty)` — always true, tests nothing.

---

## Part 2 Framework Spine Reachability

### Spine Component Status

#### 1. SPR Protocol Traits (spr/command/component.rs, re-exported through spr/component.rs)

| Symbol | Location | Status |
|--------|----------|--------|
| `Inference<P>` trait | spr/command.rs:128 | ✓ FOUND |
| `DiffRegions` trait | spr/command.rs:175 | ✓ FOUND |
| `TouchedPaths` struct | spr/command.rs:137 | ✓ FOUND |
| `InferenceFieldSpec` struct | spr/command.rs:182 | ✓ FOUND |
| `InferenceSpec<P>` trait | spr/command.rs:190 | ✓ FOUND |
| **Re-export in spr/component.rs** | Line 30 | ✓ FOUND: `DiffRegions, Inference, InferenceFieldSpec, InferenceSpec, TouchedPaths` |

#### 2. OS Inference Module (mounted in kernel glue)

| Symbol | Location | Status |
|--------|----------|--------|
| **Module mount** | glue.rs:263-274 | ✓ FOUND: `pub mod os_inference` |
| `DepHash` | inference/component.rs:19 | ✓ FOUND |
| `InferredField<P>` trait | inference/component.rs:78 | ✓ FOUND |
| `InferenceCache` struct | inference/component.rs | ✓ FOUND |
| `InferenceSession` struct | inference/component.rs | ✓ FOUND |
| `infer_field()` fn | inference/component.rs | ✓ FOUND |
| `infer_field_after_diff()` fn | inference/component.rs | ✓ FOUND |
| `InferenceCacheConfig` struct | inference/component.rs | ✓ FOUND |
| `InferencePersistence` enum | inference/component.rs | ✓ FOUND |
| **Crate-root re-export** | glue.rs:274 | ✓ FOUND: `pub use crate::os_inference::*;` |

#### 3. Plugin Module (ArtifactInferrer)

| Symbol | Location | Status |
|--------|----------|--------|
| `ArtifactInferrer` trait | plugin/component.rs:807 | ✓ FOUND |
| **Curated re-export** | plugin/component.rs:10002 | ✓ FOUND: `ArtifactInferrer` in explicit name list |

#### 4. Schema Module (Descriptors + StateClass)

| Symbol | Location | Status |
|--------|----------|--------|
| `ArtifactInferenceDescriptor` struct | schema/component.rs:324 | ✓ FOUND |
| `ArtifactInferenceRegistry` struct | schema/component.rs:338 | ✓ FOUND |
| `artifact_inference_graphql_sdl()` fn | schema/component.rs | ✓ FOUND |
| `StateClass::Inferred` variant | schema/component.rs | ✓ FOUND |
| `parse_state_class_kebab()` fn | schema/component.rs:592 | ✓ FOUND: handles "inferred" |
| `state_class_kebab()` fn | schema/component.rs:605 | ✓ FOUND: returns "inferred" |
| **TypeScript twin** | schema/component.ts | ✓ FOUND: INFERRED in preamble (line 35) |
| **GRAPHQL_STATE_PREAMBLE** | schema/component.rs | ✓ FOUND: includes `INFERRED` in enum (line 80) |

#### 5. Derive Dual-Copy (critical integrity check)

**Schema derive:**
```
diff /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️component.rs \
     /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📦️glue.rs
```
**Result: IDENTICAL** ✓

**DSL derive:**
```
diff /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs \
     /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs
```
**Result: IDENTICAL** ✓

**`#[state(inferred)]` support:**
- Schema derive (both copies), line 54: `"expected #[state(persistent|shared_ui|local_ui|preview|effect|inferred)]"`
- Schema derive (both copies), line 63: `"inferred" => "Inferred"`
- ✓ Macro supports the new attribute across both copies

---

## Gaps

1. **Missing CACHE law (0/72):** No family implements `inference_cache_transparency_law` — the key verification that stale cache is impossible.
2. **Missing INCR law (~70 families):** DAG-shaped families owe `inference_incrementality_law` but have zero implementations.
3. **Vacuous tests (at least 6 families):** Tests using empty defaults with no real fixtures or assertions provide no meaningful coverage.
4. **No fixture coverage:** Families like `✒️writer`, `🗄️stdio/svg`, `🗄️stdio/md` test only against `::default()` snapshots, never against real authored content.

---

## Concurrent-Churn Observations

**None.** This is a read-only audit. All spine symbols are in place, derive dual-copies are byte-identical, and no active edits were detected during the scan. Framework is stable and reachable.

---

## Conclusion

**Framework spine: INTACT and FULLY REACHABLE.** All 15+ required symbols present, properly exported, and derive dual-copy constraint satisfied.

**Law coverage: CRITICAL GAP.** 72/72 families have the two baseline laws (determinism + default), but **zero have the two newer laws** (cache transparency + incrementality). The ticket's "dependency-aware merkle caching" thesis requires both to be proven. Additionally, at least 6 families have VACUOUS tests (empty snapshots, no real fixtures or assertions), rendering their coverage illusory.
