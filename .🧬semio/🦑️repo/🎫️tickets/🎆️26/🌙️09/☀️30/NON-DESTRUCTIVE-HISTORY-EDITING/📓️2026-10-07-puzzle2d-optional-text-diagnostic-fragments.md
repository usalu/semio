# Puzzle2d Optional Text Diagnostic Fragments

## Exact Current Leaf Authority

Read the physical `ChangeNodeIcon` and `EditTargetRegionLabel` diff leaves at12:49. Both accept arbitrary optional UTF8 payload text (schema string/null), choose the first matching native ID, and produce no diagnostic for a changed field.

| Plan | Severity | Code | Exact body fragments | Targets |
| --- | --- | --- | --- | --- |
| Missing icon node | Error | mutation.target-missing | static `node "` + borrowed original payload `id` PagedUtf8 + static `" not found` | one original payload ID |
| Missing label region | Error | mutation.target-missing | static `target region "` + borrowed original payload `id` PagedUtf8 + static `" not found` | one original payload ID |
| Present unchanged optional text | Warning | mutation.no-op | static `no changes to apply` | one original payload ID |
| Present changed optional text | none | none | none | none |

The target/body authority is the original payload ID, even when another retained native snapshot ID has equal semantic text under different chunk partition. Quoted body text includes original Unicode/NUL without escape reinterpretation. A final-clamped builder may copy only the visible admitted UTF8 prefix, omit an unfitting target by its known byte length, and retain full severity/code semantics. No whole String, Vec<String>, or contiguous original ID must be reconstructed in a hot producer.

## Capability Boundary

The actual optional text preparation plans, inverses and changed candidates are staged native cursors; no diagnostic builder is installed here and no Store factory is registered. Root owns the borrowed paged diagnostic builder seam; it has not been adopted or certified by this family. Current new native domain selection2710 is live, unexecuted; current source receipts are preparation1/1, inverse1/1 and candidate1/1 only.
