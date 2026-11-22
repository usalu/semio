# Stale deletion audit (merged and already-recovered repairs)

Scope: the 28 rows with action `merged` (16) or `already-recovered` (12) in `🗑️generated/final.json`. For each path, the fleet head blob (short sha in the `head` field, full sha resolved from `/Volumes/semio-store/v1/blobs/<2hex>/<sha256>` and checked against its name) was compared with the current worktree file.

Identifiers checked per file type: Rust `fn`, `struct`, `enum`, `trait`, `type`, `const`, `static`, `mod`, every identifier in `use` statements, and enum variants. TypeScript exported symbols (plus top-level declarations and import names). JSON keys (all levels). Identifiers present in the head but not declared or imported in the current file were listed. Reference search: `grep -rnw --include=*.rs --include=*.ts --include=*.tsx` over the repo, excluding `.git`, `node_modules`, `target`, `.cache` and the `.🧬semio` ticket tree.

Result: 24 of 28 paths have no missing identifiers. 4 paths have 7 missing identifiers in total. 3 paths are byte-identical to the fleet head (`paint-editor-and-composite/🦀️.rs`, `packet-image-and-blend/🦀️.rs`, `shell-actions/…/layouts/🦀️.rs`). 

## Missing identifiers

| path | missing identifier | references found | classification |
|---|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs` | `ATTACHED_SURFACES` (import, head line 40) | Defined as `static` in the sibling `📎️registry-and-node-graph/🦀️.rs` (lines 89, 93, 103). Zero uses in the current file. | (a) still referenced elsewhere. The current file no longer uses it, so the import removal is consistent locally, but the static is live in a sibling and the removal needs review. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📋️receipts-drop-and-clipboard/🦀️.rs` | `DslValue` (`use semio_framework::DslValue;` at head line 141, function-local) | Used in 20 k+ lines across the tree (`semio_framework::DslValue` in many crates). Current file uses only fully qualified `semio_framework::DslValue::…` paths, so it compiles either way. | (a) referenced elsewhere. Benign here: the restored use line would be cosmetic. |
| same file | `semio_framework` (head used it as a path root for `DslValue`) | Path root used across the tree and in the current file itself (qualified paths). | (a) benign, same as above. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs` | `card_path_segments_use_the_declared_wire_tag_and_id` (test fn, head line 34) | No reference anywhere in the tree (`card_path_segments` matches nothing outside the head copy). No renamed similar test found. | (b) not referenced, probably intentionally removed. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🎛️acceptance-policy-pump-and-drive/🦀️.rs` | `std` / `collections` / `BTreeMap` (`use std::collections::BTreeMap;`, head line 4) | `BTreeMap` used twice in head (line 286 and the `refusals` map, function `refusals` at 286-289). The current file has 0 uses of `BTreeMap`, 0 of `refusals`. Standard library type, not a fleet feature. | (b) not referenced locally. The code that used it was removed in the current file as well, so the import is consistent. |

## Paths with no missing identifiers

Identical to fleet head (no deletion possible): `paint-editor-and-composite/🦀️.rs`, `packet-image-and-blend/🦀️.rs`, `shell-actions/…/layouts/🦀️.rs`.

All other paths differ from the fleet head, but every head identifier is still declared or imported. This includes `hub/…/🔣️.json` (187 keys checked) and `load/🟦️.ts`.

## Limits

- Struct fields, method call sites, match arms and config keys are not covered. A stale deletion of one of those would not show up here. Only the identifier classes listed above were checked, as requested.
- Name-based matching: an identifier that is still declared somewhere in the file, even at a different place, counts as present.
- `grep -w` on short common identifiers over the whole tree is noisy. Only the 7 identifiers above needed a reference search.

## Count

(a) items: 3 (`ATTACHED_SURFACES`, `DslValue`, `semio_framework`), in 2 paths.
(b) items: 3 (`card_path_segments_use_the_declared_wire_tag_and_id`, `std`/`collections`/`BTreeMap`, all in 2 paths).

Paths to restore or review (a): 
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📋️receipts-drop-and-clipboard/🦀️.rs` (the `DslValue`/`semio_framework` import lines only; functionally the file already compiles with qualified paths)

Scratch files (extraction script, head and current copies, grep output) are in `/private/tmp/claude-501/-Users-ueli-Documents-semio/1b9281d2-630e-44c8-9645-7ae50361d950/scratchpad/audit/`. No source file was edited.
