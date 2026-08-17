# P0-A1 Audit: stdio & norm Inference Schema Families

**Audit Date:** 2026-08-12
**Total Families Audited:** 34 (15 norm + 19 stdio)
**Fully Clean Families:** 0
**Families with Failures:** 34

## Audit Criteria Reference

| Criterion | Description |
|-----------|-------------|
| C1 | Family-root 5 leaves present (component.rs, component.ts, component.graphql, component.json, component.proto) |
| C2 | text/ directory contains 8 expected leaves (derived from snapshot/text/) |
| C3 | binary/ directory contains 6 expected leaves (derived from snapshot/binary/) |
| C4 | ≥1 slug dir present (text/, binary/, outline/) each with both component.rs and component.ts |
| C5 | Slug component.ts is REAL (not export {} stub, >20 bytes) |
| C6 | Slug component.rs contains impl … InferredField< |
| C7 | Family-root component.rs contains: Inference struct, impl Inference<, impl InferenceSpec, impl ArtifactInferrer, *_artifact_inference_descriptor() with 5+ include_str! |
| C8 | Grammar honesty: no placeholder/stub text (lorem, TODO, FIXME, placeholder, XXX, Example) |

## Audit Results Table

| Plugin | Artifact | Version | C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 | Gaps |
|--------|----------|---------|----|----|----|----|----|----|----|----|------|
| norm | 📓️iso16757 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📔️vdi3805 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📕️din4108 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📗️din16798 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📘️en1990 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📘️en1991 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📘️en1992 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📘️en1993 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📘️en1994 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📘️en1995 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📘️en1996 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📘️en1997 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📘️en1998 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📘️en1999 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| norm | 📙️din18599 | 1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 🌐️html | 5 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 🎞️gif | 87a | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 🎞️gif | 89a | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 🎞️pptx | ecma-376 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 🎨️svg | 1.1 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📄txt | utf-8 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📄️pdf | 1.4 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📄️pdf | 1.7 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📊️csv | rfc4180 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📑️tsv | iana | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📕️xlsx | ecma-376 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📜️docx | ecma-376 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📝️md | commonmark | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📰xml | 1.0 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📷️jpg | jfif-1.01 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 📷️png | 1.2 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 🔣️json | rfc8259 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 🖼️bmp | v3 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |
| stdio | 🖼️tiff | 6.0 | PASS | PASS | PASS | PASS | PASS | **FAIL** | PASS | PASS | 1 |

## Gaps

### C6-FAIL: Missing InferredField impl (34/34 families)

**Root cause:** Criterion 6 fails universally across all 34 families. Each family has three slug directories (text/, binary/, outline/) that contain stub implementations. None of them have the required `impl InferredField<` implementation.

Affected paths (listing first slug per family, pattern repeats 3× per family):

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs`
- ... (pattern repeats for all 34 families, total 102 files)

**Action:** Each slug's component.rs must include:
```rust
impl InferredField<[ArtifactType]> for [InferenceType] {
    fn dep_input(&self) -> impl Iterator<Item = &dyn ArtifactRef> { /* ... */ }
    fn compute(&mut self, deps: &DependencyMap) { /* ... */ }
}
```

## Duplicate-Grammar Suspicions

**Result:** No duplicate grammar files detected.

All 476 grammar files across stdio (238 files) and norm (238 files) have unique content (no md5 hash collisions). Grammar honesty check passed for all families—no placeholder text detected.

## Concurrent-Churn Observations

**Date/Time:** Audit run 2026-08-12 09:15 UTC
**Live Sessions (per important.md):**
- SMO (SEMANTIC-MUTATIONS-OVERHAUL #2545) — owns trinity, app component, per-plugin glue.rs
- UCAS (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM #2548) — owns stdio (transiently red), framework, kernel inference module
- APA (ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE #2549) — owns taxonomy.json, script.ts write order, puzzle surface

**No churn detected in stdio or norm inference families during audit.** All families show stable timestamps (Aug 12 11:46–11:47). No locked files, no partial writes. UCAS owns stdio transiently; no concurrent edits observed.

## Summary

- **Total families:** 34 (100%)
- **Fully passing (0 gaps):** 0 (0%)
- **Failing C6 only:** 34 (100%)
- **Most common gap:** C6-NoInferredField (34 families, 102 slug files)
- **Secondary gaps:** 0 (no other failures)
- **Grammar duplicates:** 0
- **Concurrent churn:** None

**Remediation Priority:** CRITICAL — All 34 families must have real `impl InferredField<` in each of their 3 slug files (text/, binary/, outline/). Total: 102 files requiring implementation.

