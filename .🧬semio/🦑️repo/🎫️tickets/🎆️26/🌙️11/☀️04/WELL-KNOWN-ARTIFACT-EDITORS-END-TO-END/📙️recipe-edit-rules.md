# 📙️ Recipe: Edit-Rule Gaps for Weak Editors (T16c, static)

A = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`. Rule types are defined in `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧭️rules/🦀️.rs`.

**Corrections to the earlier audit:**
- svg base, basic and tiny already edit element children, attributes and text through `special → tree_edit`.
- gif 87a is complete.
- jpg baseline inherits the document subset's rules.
- tiff baseline reaches its two kinds through `special` (by design).

| Editor | Gap | Pointer shapes | Template | Owner |
| --- | --- | --- | --- | --- |
| svg basic/tiny | No element rename. `set-transform` and `set-view-box` exist but are not reachable as typed edits. `strip/restore-non-tiny` and `stamp-base-profile` are whole-document actions. | `/doc/root/children/<n>/name`, `…/transform`, viewBox attribute value | base `mutation-support/🦀️.rs:158-222`; add `set_element_name` to the kit (`:72-74`) | D |
| bmp | No pixel/palette Details edits. `replace-samples` has no editor path. | `/image/pixels/indices/<n>`, `/image/pixels/samples/<n>`, `/image/palette/<n>` | gif 87a `EntityRule("/images/*/indices",…)` (`edit-rules:21`); dxf Insert/Remove rules | C |
| dwg | No kinds at all for layers, objects or geometry entities | `/drawing/layers/<n>` (set/insert/remove, keyed by name), `/drawing/objects/<n>/geometry` | dxf layer rules (`🖋️dxf/…/edit-rules/🦀️.rs:9,18,26`) | D |
| mp3 | No per-frame kinds (`/frames` is replaced whole) | `/frames/<n>`, `/frames` `-`, `/frames/<n>/header/…` (new kinds `set-frame`, `insert-frame`, `remove-frame`) | gif 89a frame rules (`9️⃣89a/…/edit-rules:22-30`) | G |
| ifc 2x3 cobie/sav/cv20 | Domain kinds exist (`set-facility-name`, `set-view-definition`, `set-floor-elevation`, `set-space`, `set-type-assignment`) but nothing emits them (grep finding) | per-instance-type pointers under `/document/instances/<n>/…` | base special (`🧱️base/…/edit-rules:37-49`); ifc 4 table | F |
| step cc1–cc6 | `set-product-identity`, `set-shape-representation`, `demote-shape-representation`/`remove-shape-representation` cannot be reached; only `/header/fileSchema/schemas` plus `ladder::edit_restore_rows` | `/entities/<row>` product and shape fields | cc1 special plus `ladder` | E |
