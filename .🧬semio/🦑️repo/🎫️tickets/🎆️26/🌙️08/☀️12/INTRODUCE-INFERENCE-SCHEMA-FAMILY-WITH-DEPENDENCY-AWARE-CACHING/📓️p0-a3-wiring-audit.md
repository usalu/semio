# Wiring Audit Report: Inference Schema Family

Date: 2026-08-12
**Scope:** 72 inference families across 33 plugins (session 1 addition: 17 mounts; remaining: 55 mounts pending)

## A Glue mounts

### Status Summary

- **Total inference families found:** 72 across 33 plugins
- **Currently mounted in glue.rs:** 17 families  
- **Missing mounts:** 55 families

### Mounted families (17):
✒️writer, ➗️mathematical, 🌊️flow, 🌿️vcs, 🎥️shooting, 🎬️sequence, 💠️lowpoly, 📋️forms, 📏️layout, 📐️cad, 📖️playbook, 📜️imperative, 📸️remodel, 🕸️dag, 🖍️draw, 🖨️raster, 🗒️note

### Broken #[path] strings

No broken paths detected in the 17 mounted families. All paths resolve to real files on disk.

### Missing mounts (55 families)

The following artifacts have inference family directories but ARE NOT mounted in their plugin glue.rs files:

**Procedural plugin (2):**
- 🌀️procedural2d
- 🧊️procedural3d

**GIS plugin (2):**
- 🏔️gisterrain
- 🗺️gismap

**Animate plugin (1):**
- 🎞️pptx

**Animate plugin (1):**
- 🎬️present

**Demonstrator plugin (1):**
- 🎪️playground

**Fem plugin (2):**
- ◻2d (puzzle2d)
- 🧊️3d (puzzle3d, not puzzle 3d)

**Architect plugin (1):**
- 🏛️program

**Process plugin (1):**
- 🧊️process3d

**Reasoning plugin (1):**
- 🔌️wires

**Norm plugin (10 standards):**
- 📓️iso16757, 📔️vdi3805, 📕️din4108, 📗️din16798, 📘️en1990-1999 (10 standards), 📙️din18599

**Energy plugin (1):**
- 🔋️model

**Trinity plugin (2):**
- ♻️rewrite
- 🔌️jack

**Stdio plugin (15 file formats):**
- 🌐️html, 🎞️gif (2 standards), 🎨️svg, 📄txt, 📄️pdf (2 standards), 📊️csv, 📑️tsv, 📕️xlsx, 📜️docx, 📝️md, 📰xml, 📷️jpg, 📷️png, 🔣️json, 🖼️bmp, 🖼️tiff

**Block plugin (3):**
- ◻2d, 🧊️3d, 🖐️5d

**Space plugin (1):**
- 🏠️home

**Sourcing plugin (1):**
- 🗂️curate

## B Registration

**Status:** Only the 17 mounted families have been checked for registration calls.

All 17 mounted families have `register_artifact_inferences` calls in their artifact engine components. The 55 unmounted families cannot be registered until their glue.rs mounts are added.

## C Descriptor fns

**Status:** Only the 17 mounted families have been checked for descriptor functions.

All 17 mounted families have `_artifact_inference_descriptor` functions defined. The 55 unmounted families cannot have descriptors verified until their infrastructure is complete.

## D TS index export verdict

### Evidence: TS index exports for sibling families

**🧩️puzzle/index.ts (mutations ARE exported):**
```typescript
export * as puzzle2d_mutations from "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🟦️component.ts";
export * as puzzle5d_mutations from "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🟦️component.ts";
export * as puzzle5d_snapshot from "../../🗿️artifacts/🖐️5d/🧬️schema/📸️snapshot/🟦️component.ts";
```

**🌀️procedural/index.ts (mutations ARE exported):**
```typescript
export * as procedural2d_mutations from "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts";
export * as procedural3d_mutations from "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts";
```

**Inferences currently exported from TS index files:** None (zero across all plugins).

### Policy evidence from script.ts

The repo's TypeScript facade policy defines `POLICY_TS_FACADE_CONSTITUTIONAL_FACETS`:
```typescript
const POLICY_TS_FACADE_CONSTITUTIONAL_FACETS = new Set<string>([
  "🗣️dsl",
  "🔧️op",
  "🔺️diff",
  "🎒️pack",
  "📡️spr",
  "🧬️mutations",
  "⚙️engine",
]);
```
(Source: `/Users/ueli/Documents/semio/📜️script.ts`, lines 1880–1886)

**Key finding:** `🧬️mutations` IS in the constitutional facets list. `💡️inferences` is NOT.

### VERDICT: REQUIRED

**Evidence basis:**
1. **Mutations ARE exported:** puzzle2d_mutations, procedural2d_mutations, procedural3d_mutations, etc. appear in index.ts files (9+ mutation exports in puzzle alone).
2. **Inferences mirror mutations:** The ticket thesis states "💡️inferences (plural, slug-dir shape mirroring 🧬️mutations)".
3. **Constitutional policy:** The script.ts policy explicitly lists `🧬️mutations` as a constitutional TS facade facet. Since inferences must mirror mutations' architecture, they should receive identical treatment.
4. **No explicit ban:** The absence of inferences from the constitutional list appears to be a forward-looking omission (inferences are new in this session), not an intentional exclusion.

**Recommendation:** Inferences must be exported from TS index.ts files to maintain architectural symmetry with mutations.

## Gaps

### Critical blockers (must be fixed before this ticket closes):

1. **Add 55 missing glue.rs mounts** — 55 artifacts have inference family directories but no Rust module mounts in their plugin glue.rs files. Mount patterns must follow the constitutional facet shape (see puzzle2d inferences mount at `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs`, lines 592–606 and 1126–1134 for reference).

   **Affected plugins (must add mounts for):**
   - 🌀️procedural, 🌍️gis, 🎞️animate, 🎪️demonstrator, 🏗️fem, 🏛️architect, 🏭️process, 💡️reasoning, 📕️norm, 🔋️energy, 🔱️trinity, 🗄️stdio, 🧱️block, 🪐️space, 🪵️sourcing

2. **Add TS index exports** — After mounts are complete, all 72 families must be exported from their plugin `📦️packages/🟦️typescript/📦️index.ts` files (e.g., `export * as <artifact>_inferences from "..."`).

3. **Add `💡️inferences` to constitutional facets policy** — Update `POLICY_TS_FACADE_CONSTITUTIONAL_FACETS` in `/Users/ueli/Documents/semio/📜️script.ts` to include `💡️inferences` (line 1885, add to the set).

## Concurrent-churn observations

- **Trinity:** Currently owned by peer session SMO (semantic-mutations-overhaul #2545). Two inference families (♻️rewrite, 🔌️jack) missing mounts.
- **Puzzle:** Currently owned by peer session APA (artifacts-only-plugin-architecture #2549). Three inference families (◻2d, 🧊️3d from fem/block confusion, 🖐️5d from block) missing mounts. Note: puzzle3d and puzzle5d DO have inferences mounted, but puzzle2d does not.
- **Stdio:** Currently owned by peer session UCAS (unified-composable-artifact-system #2548). All 15 stdio artifact formats missing inference mounts.
- **Norm:** Single-plugin with 10 standard artifacts — all missing inference mounts.

Recommend coordinating mount additions across concurrent sessions to avoid ping-ponging edits to glue.rs files.
