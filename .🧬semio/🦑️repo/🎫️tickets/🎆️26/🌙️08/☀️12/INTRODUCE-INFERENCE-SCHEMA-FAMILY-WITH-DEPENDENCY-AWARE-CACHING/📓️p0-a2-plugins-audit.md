# Inference Schema Family Audit

## Reference Exemplar Structure

Audit baseline: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/`

**Family-root (5 leaves):** `🦀️component.rs`, `🟦️component.ts`, `🔗️component.graphql`, `🔣️component.json`, `🛰️component.proto`

**📝️text/ (8 leaves):**
- 🅰️component.g4
- 📖️component.grammar.semio
- 🔗️component.graphql
- 🔣️component.json
- 🔤️component.ebnf
- 🛰️component.proto
- 🟦️component.ts
- 🦀️component.rs

**💾️binary/ (6 leaves):**
- 🌶️component.spicy
- 📡️component.protocol.semio
- 🔠️component.abnf
- 🟦️component.ts
- 🥋️component.ksy
- 🦀️component.rs

**Root 🦀️component.rs must contain:** assembly struct with `Inference` name, `impl Inference<`, `impl InferenceSpec`, `impl ArtifactInferrer`, `*_artifact_inference_descriptor()` fn with 5 `include_str!` calls

**Each slug dir:** BARE emoji prefix (no U+FE0F), unique within family, kebab-stem as NOUN phrase, ≥1 with both `🦀️component.rs` (real `impl InferredField<`) and `🟦️component.ts` (real, not stub/empty)

## Summary

| Metric | Count |
|--------|-------|
| Total families audited | 38 |
| Clean families | 17 |
| Families with gaps | 21 |

## Audit Table

| Plugin/Artifact | Root 5 | Text 8 | Binary 6 | Slugs | TS Real | RS impl | Root Asm | Status |
|-----------------|--------|--------|----------|--------|---------|---------|---------|--------|
| ✒️writer/✒️writer | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| ➗️mathematical/➗️mathematical | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🌀️procedural/🌀️procedural2d | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🌀️procedural/🧊️procedural3d | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🌊️flow/🌊️flow | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🌍️gis/🏔️gisterrain | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🌍️gis/🗺️gismap | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🌿️vcs/🌿️vcs | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🎞️animate/🎬️present | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🎥️shooting/🎥️shooting | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🎪️demonstrator/🎪️playground | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🎬️sequence/🎬️sequence | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🏗️fem/◻2d | ✓ | ✗ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🏗️fem/🧊️3d | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🏛️architect/🏛️program | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🏭️process/🧊️process3d | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 💠️lowpoly/💠️lowpoly | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 💡️reasoning/🔌️wires | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 📋️forms/📋️forms | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 📏️layout/📏️layout | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 📐️cad/📐️cad | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 📖️playbook/📖️playbook | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 📜️imperative/📜️imperative | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 📸️remodel/📸️remodel | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🔋️energy/🔋️model | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🔱️trinity/♻️rewrite | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🔱️trinity/🔌️jack | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🕸️dag/🕸️dag | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🖍️draw/🖍️draw | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🖨️raster/🖨️raster | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🗒️note/🗒️note | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🧩️puzzle/🖐️5d | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🧩️puzzle/🧊️3d | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🧱️block/◻2d | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🧱️block/🖐️5d | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🧱️block/🧊️3d | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ❌ FAIL |
| 🪐️space/🏠️home | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |
| 🪵️sourcing/🗂️curate | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ CLEAN |

## Gaps

### Summary by category:

**Missing slug RS InferredField impl:** 21 families

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`

**Missing text/ leaves:** 1 families

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences` — missing: text/🛰️component.proto

## Clean Families (Fully Compliant)

These families pass all 9 audit criteria:

- 🌊️flow/🌊️flow
- 🌍️gis/🏔️gisterrain
- 🌍️gis/🗺️gismap
- 🌿️vcs/🌿️vcs
- 🏛️architect/🏛️program
- 🏭️process/🧊️process3d
- 💠️lowpoly/💠️lowpoly
- 💡️reasoning/🔌️wires
- 📐️cad/📐️cad
- 📜️imperative/📜️imperative
- 📸️remodel/📸️remodel
- 🔱️trinity/♻️rewrite
- 🔱️trinity/🔌️jack
- 🕸️dag/🕸️dag
- 🧩️puzzle/🧊️3d
- 🪐️space/🏠️home
- 🪵️sourcing/🗂️curate

## Duplicate-Grammar Suspicions

- **✒️writer/✒️writer**: 1 files with placeholders
  - `📡️component.protocol.semio`: Contains 'foo'
- **➗️mathematical/➗️mathematical**: 1 files with placeholders
  - `📡️component.protocol.semio`: Contains 'foo'
- **🌀️procedural/🌀️procedural2d**: 1 files with placeholders
  - `📡️component.protocol.semio`: Contains 'foo'
- **🌀️procedural/🧊️procedural3d**: 1 files with placeholders
  - `📡️component.protocol.semio`: Contains 'foo'
- **🌊️flow/🌊️flow**: 1 files with placeholders
  - `📡️component.protocol.semio`: Contains 'foo'
- **🌍️gis/🏔️gisterrain**: 1 files with placeholders
  - `📡️component.protocol.semio`: Contains 'foo'
- **🌍️gis/🗺️gismap**: 1 files with placeholders
  - `📡️component.protocol.semio`: Contains 'foo'
- **🌿️vcs/🌿️vcs**: 1 files with placeholders
  - `📡️component.protocol.semio`: Contains 'foo'
- **🎞️animate/🎬️present**: 1 files with placeholders
  - `📡️component.protocol.semio`: Contains 'foo'
- **🎥️shooting/🎥️shooting**: 1 files with placeholders
  - `📡️component.protocol.semio`: Contains 'foo'

## Concurrent-Churn Observations

Recent git activity on audited families (last 3 commits per family):

- **✒️writer/✒️writer**: 1 recent commits
  - `16619a9699 🐙️ueli🎆️26🌙️06☀️04🚩️490`
- **➗️mathematical/➗️mathematical**: 1 recent commits
  - `a46ac1f883 🐙️ueli🎆️26🌙️06☀️04🚩️491`
- **🌀️procedural/🌀️procedural2d**: 1 recent commits
  - `a46ac1f883 🐙️ueli🎆️26🌙️06☀️04🚩️491`
- **🌀️procedural/🧊️procedural3d**: 1 recent commits
  - `a46ac1f883 🐙️ueli🎆️26🌙️06☀️04🚩️491`
- **🎞️animate/🎬️present**: 1 recent commits
  - `16619a9699 🐙️ueli🎆️26🌙️06☀️04🚩️490`
- **🎥️shooting/🎥️shooting**: 1 recent commits
  - `16619a9699 🐙️ueli🎆️26🌙️06☀️04🚩️490`
- **🎪️demonstrator/🎪️playground**: 1 recent commits
  - `16619a9699 🐙️ueli🎆️26🌙️06☀️04🚩️490`
- **🎬️sequence/🎬️sequence**: 1 recent commits
  - `16619a9699 🐙️ueli🎆️26🌙️06☀️04🚩️490`
- **🏗️fem/◻2d**: 1 recent commits
  - `16619a9699 🐙️ueli🎆️26🌙️06☀️04🚩️490`
- **🏗️fem/🧊️3d**: 1 recent commits
  - `16619a9699 🐙️ueli🎆️26🌙️06☀️04🚩️490`