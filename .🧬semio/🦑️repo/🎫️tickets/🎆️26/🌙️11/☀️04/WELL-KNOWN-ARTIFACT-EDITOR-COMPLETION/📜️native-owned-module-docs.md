# Native Owned Module Documentation Placement

Actual isolated semantic check reached Rust and failed E0753 in eight native owned modules: field, original, sealed-shared receiver, tree, tuple, array, map, set. Imports preceded their inner module docs in that admitted snapshot. On fresh inspection all eight docs were already restored to the head by concurrent changes, so root made no source edits. A fresh semantic check is required; no native pass is claimed.
