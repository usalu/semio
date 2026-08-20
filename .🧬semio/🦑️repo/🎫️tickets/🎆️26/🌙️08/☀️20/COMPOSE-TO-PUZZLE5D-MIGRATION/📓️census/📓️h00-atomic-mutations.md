# Puzzle5d Mutation Census — All 28 Atomic Mutations

**Report Date**: 2026-08-21
**Scope**: Read-only analysis of 28 mutation definitions, diffs, and inverses
**Source**: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`

---

## Mutation Details

### 1. ⚓change-part-anchor

**Files**: 
- Mutation: `⚓change-part-anchor/🦠️mutation/🦀️component.rs:12-30`
- Diff: `⚓change-part-anchor/🔺️diff/🦀️component.rs:5-20`
- Inverse: `⚓change-part-anchor/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**: 
- `id: String`
- `new_anchor: crate::artifacts::puzzle5d::Puzzle5dPartAnchor`

**DSL Keyword**: `"change-part-anchor"` (no block fields)

**SEMANTICS**: verb=`"change"`, entity=`"part"`, kind=`"change-part-anchor"`, record=`"ChangedPartAnchor"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**: 
- `"mutation.target-missing"` — part id not found in base

**No-op Behavior**: `accepted-empty-diff` — if new anchor equals current anchor, returns warning `"mutation.no-op"` with empty diff

**Inverse**: Returns 1 mutation (`change_part_anchor` with base anchor value). Reads base to get original anchor. If target missing, returns `Vec::new()`.

**Cascade**: None — modifies only the addressed part.

**Round-trip Safe**: Yes

---

### 2. ✂️disconnect-grips

**Files**:
- Mutation: `✂️disconnect-grips/🦠️mutation/🦀️component.rs:12-31`
- Diff: `✂️disconnect-grips/🔺️diff/🦀️component.rs:5-14`
- Inverse: `✂️disconnect-grips/↩️inverse/🦀️component.rs:5-13`

**Payload Fields**:
- `id: String` (fastener id)

**DSL Keyword**: `"disconnect-grips"` (no block fields)

**SEMANTICS**: verb=`"disconnect"`, entity=`"grips"`, kind=`"disconnect-grips"`, record=`"DisconnectedGrips"`

**Diff Shape**: `fasteners` (removed: list)

**Rejection Conditions**:
- `"mutation.target-missing"` — fastener id not found in base

**No-op Behavior**: `rejected` — returns error if fastener not found

**Inverse**: Returns 1 mutation (`connect_grips` with captured fastener parameters). Reads base to get fastener details. If target missing, returns `Vec::new()`.

**Cascade**: None — removes only the addressed fastener.

**Round-trip Safe**: Yes

---

### 3. ✏️edit-part2d-text

**Files**:
- Mutation: `✏️edit-part2d-text/🦠️mutation/🦀️component.rs:12-31`
- Diff: `✏️edit-part2d-text/🔺️diff/🦀️component.rs:5-20`
- Inverse: `✏️edit-part2d-text/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_text: Option<String>`

**DSL Keyword**: `"edit-part2d-text"` (no block fields)

**SEMANTICS**: verb=`"edit"`, entity=`"part"`, kind=`"edit-part2d-text"`, record=`"EditedPart2dText"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found in base

**No-op Behavior**: `accepted-empty-diff` — if text unchanged, returns warning with empty diff

**Inverse**: Returns 1 mutation (`edit_part_2d_text` with base text value). Reads base. If target missing, returns `Vec::new()`.

**Cascade**: None — modifies only addressed part's text.

**Round-trip Safe**: Yes

---

### 4. ➕add-part-grip

**Files**:
- Mutation: `➕add-part-grip/🦠️mutation/🦀️component.rs:12-36`
- Diff: `➕add-part-grip/🔺️diff/🦀️component.rs:5-27`
- Inverse: `➕add-part-grip/↩️inverse/🦀️component.rs:5-9`

**Payload Fields**:
- `part_id: String`
- `grip: Puzzle5dGrip` (**block field**)
- `index: Option<usize>`

**DSL Keyword**: `"add-part-grip"` | block: `grip`

**SEMANTICS**: verb=`"add"`, entity=`"part-grip"`, kind=`"add-part-grip"`, record=`"AddedPartGrip"`

**Diff Shape**: `parts` (patched: replacement), `reordered` if index specified

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found in base
- `"mutation.duplicate-id"` — grip id already exists in part's grip list

**No-op Behavior**: `rejected` if duplicate grip id

**Inverse**: Returns 1 mutation (`remove_part_grip` with same part_id and grip.id). Does NOT read base (`_base` parameter).

**Cascade**: None — adds grip only to specified part.

**Round-trip Safe**: Yes

---

### 5. ➖remove-part-grip

**Files**:
- Mutation: `➖remove-part-grip/🦠️mutation/🦀️component.rs:12-31`
- Diff: `➖remove-part-grip/🔺️diff/🦀️component.rs:5-33`
- Inverse: `➖remove-part-grip/↩️inverse/🦀️component.rs:5-19`

**Payload Fields**:
- `part_id: String`
- `grip_id: String`

**DSL Keyword**: `"remove-part-grip"` (no block fields)

**SEMANTICS**: verb=`"remove"`, entity=`"part-grip"`, kind=`"remove-part-grip"`, record=`"RemovedPartGrip"`

**Diff Shape**: `parts` (patched: replacement), `fasteners` (removed: list if any fasteners touched this grip)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found in base

**No-op Behavior**: `rejected` if grip_id not found in part's grips

**Inverse**: Returns 1 + N mutations (1x `add_part_grip` + Nx `connect_grips` for each severed fastener). Reads base for grip details and to find affected fasteners.

**Cascade**: Removes all fasteners that touch this grip (cascade repair via inverse).

**Round-trip Safe**: Yes

---

### 6. 🌐change-domain

**Files**:
- Mutation: `🌐change-domain/🦠️mutation/🦀️component.rs:12-27`
- Diff: `🌐change-domain/🔺️diff/🦀️component.rs:5-11`
- Inverse: `🌐change-domain/↩️inverse/🦀️component.rs:5-9`

**Payload Fields**:
- `new_domain: String`

**DSL Keyword**: `"change-domain"` (no block fields)

**SEMANTICS**: verb=`"change"`, entity=`"domain"`, kind=`"change-domain"`, record=`"ChangedDomain"`

**Diff Shape**: metadata or top-level field (UNKNOWN — no visible `Puzzle5dDiff` sub-delta)

**Rejection Conditions**: NONE — total function, always succeeds

**No-op Behavior**: `accepted-empty-diff` if domain unchanged (no explicit check, but diff returns success)

**Inverse**: Returns 1 mutation (`change_domain` with base domain value). Reads base.

**Cascade**: None — metadata-only change.

**Round-trip Safe**: Yes

---

### 7. 🌱create-part

**Files**:
- Mutation: `🌱create-part/🦠️mutation/🦀️component.rs:12-34`
- Diff: `🌱create-part/🔺️diff/🦀️component.rs:5-24`
- Inverse: `🌱create-part/↩️inverse/🦀️component.rs:5-9`

**Payload Fields**:
- `part: Puzzle5dPart` (**block field**)
- `index: Option<usize>`

**DSL Keyword**: `"create-part"` | block: `part`

**SEMANTICS**: verb=`"create"`, entity=`"part"`, kind=`"create-part"`, record=`"CreatedPart"`

**Diff Shape**: `parts` (added: list), `reordered` if index specified

**Rejection Conditions**:
- `"mutation.duplicate-id"` — part id already exists in base.parts

**No-op Behavior**: `rejected` if id exists

**Inverse**: Returns 1 mutation (`delete_part` with part.id). Does NOT read base.

**Cascade**: None — creates only the part; grips and fasteners are separate.

**Round-trip Safe**: Yes

---

### 8. 🎨change-part2d-icon

**Files**:
- Mutation: `🎨change-part2d-icon/🦠️mutation/🦀️component.rs:12-31`
- Diff: `🎨change-part2d-icon/🔺️diff/🦀️component.rs:5-20`
- Inverse: `🎨change-part2d-icon/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_icon_kind: Option<String>`

**DSL Keyword**: `"change-part2d-icon"` (no block fields)

**SEMANTICS**: verb=`"change"`, entity=`"part"`, kind=`"change-part2d-icon"`, record=`"ChangedPart2dIcon"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if icon unchanged

**Inverse**: Returns 1 mutation (`change_part_2d_icon` with base icon value). Reads base. If missing, returns `Vec::new()`.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 9. 🎯change-fastener-kind

**Files**:
- Mutation: `🎯change-fastener-kind/🦠️mutation/🦀️component.rs:12-31`
- Diff: `🎯change-fastener-kind/🔺️diff/🦀️component.rs:5-20`
- Inverse: `🎯change-fastener-kind/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_fastener_kind: Option<String>`

**DSL Keyword**: `"change-fastener-kind"` (no block fields)

**SEMANTICS**: verb=`"change"`, entity=`"fastener"`, kind=`"change-fastener-kind"`, record=`"ChangedFastenerKind"`

**Diff Shape**: `fasteners` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — fastener id not found

**No-op Behavior**: `accepted-empty-diff` if kind unchanged

**Inverse**: Returns 1 mutation (`change_fastener_kind` with base kind value). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 10. 🏗change-part-kind

**Files**:
- Mutation: `🏗change-part-kind/🦠️mutation/🦀️component.rs:12-31`
- Diff: `🏗change-part-kind/🔺️diff/🦀️component.rs:5-20`
- Inverse: `🏗change-part-kind/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_part_kind: Option<String>`

**DSL Keyword**: `"change-part-kind"` (no block fields)

**SEMANTICS**: verb=`"change"`, entity=`"part"`, kind=`"change-part-kind"`, record=`"ChangedPartKind"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if kind unchanged

**Inverse**: Returns 1 mutation (`change_part_kind` with base kind value). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 11. 🏷rename-puzzle5d

**Files**:
- Mutation: `🏷rename-puzzle5d/🦠️mutation/🦀️component.rs:12-27`
- Diff: `🏷rename-puzzle5d/🔺️diff/🦀️component.rs:5-11`
- Inverse: `🏷rename-puzzle5d/↩️inverse/🦀️component.rs:5-9`

**Payload Fields**:
- `new_name: String`

**DSL Keyword**: `"rename-puzzle5d"` (no block fields)

**SEMANTICS**: verb=`"rename"`, entity=`"puzzle5d"`, kind=`"rename-puzzle5d"`, record=`"RenamedPuzzle5d"`

**Diff Shape**: metadata/top-level (UNKNOWN)

**Rejection Conditions**: NONE — total function

**No-op Behavior**: `accepted-empty-diff` if name unchanged

**Inverse**: Returns 1 mutation (`rename_puzzle5d` with base name). Reads base.

**Cascade**: None — metadata change.

**Round-trip Safe**: Yes

---

### 12. 💔disconnect-kind-compatibility

**Files**:
- Mutation: `💔disconnect-kind-compatibility/🦠️mutation/🦀️component.rs:12-32`
- Diff: `💔disconnect-kind-compatibility/🔺️diff/🦀️component.rs:5-15`
- Inverse: `💔disconnect-kind-compatibility/↩️inverse/🦀️component.rs:5-13`

**Payload Fields**:
- `source: String`
- `target: String`

**DSL Keyword**: `"disconnect-kind-compatibility"` (no block fields)

**SEMANTICS**: verb=`"disconnect"`, entity=`"kind-compatibility"`, kind=`"disconnect-kind-compatibility"`, record=`"DisconnectedKindCompatibility"`

**Diff Shape**: `kind_compatibility` (full list replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — (source, target) pair not found in kind compatibility list

**No-op Behavior**: `rejected` if pair not found

**Inverse**: Returns 1 mutation (`connect_kind_compatibility` with captured specificity fields). Reads base to get compat entry details.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 13. 📍move-part2d

**Files**:
- Mutation: `📍move-part2d/🦠️mutation/🦀️component.rs:12-30`
- Diff: `📍move-part2d/🔺️diff/🦀️component.rs:5-20`
- Inverse: `📍move-part2d/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_position: [f64; 2]`

**DSL Keyword**: `"move-part2d"` (no block fields)

**SEMANTICS**: verb=`"move"`, entity=`"part"`, kind=`"move-part2d"`, record=`"MovedPart2d"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if position unchanged

**Inverse**: Returns 1 mutation (`move_part_2d` with base position). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 14. 📏scale-part3d

**Files**:
- Mutation: `📏scale-part3d/🦠️mutation/🦀️component.rs:12-30`
- Diff: `📏scale-part3d/🔺️diff/🦀️component.rs:5-20`
- Inverse: `📏scale-part3d/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_scale: Option<crate::artifacts::puzzle5d::Puzzle5dScale>`

**DSL Keyword**: `"scale-part3d"` (no block fields)

**SEMANTICS**: verb=`"scale"`, entity=`"part"`, kind=`"scale-part3d"`, record=`"ScaledPart3d"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if scale unchanged

**Inverse**: Returns 1 mutation (`scale_part_3d` with base scale). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 15. 📚replace-kind-catalogs

**Files**:
- Mutation: `📚replace-kind-catalogs/🦠️mutation/🦀️component.rs:12-28`
- Diff: `📚replace-kind-catalogs/🔺️diff/🦀️component.rs:5-11`
- Inverse: `📚replace-kind-catalogs/↩️inverse/🦀️component.rs:5-9`

**Payload Fields**:
- `new_catalogs: Option<Puzzle5dKindCatalogs>`

**DSL Keyword**: `"replace-kind-catalogs"` (no block fields)

**SEMANTICS**: verb=`"replace"`, entity=`"kind-catalogs"`, kind=`"replace-kind-catalogs"`, record=`"ReplacedKindCatalogs"`

**Diff Shape**: metadata/top-level (UNKNOWN)

**Rejection Conditions**: NONE — total function

**No-op Behavior**: `accepted-empty-diff` if catalogs unchanged

**Inverse**: Returns 1 mutation (`replace_kind_catalogs` with base catalogs). Reads base.

**Cascade**: None — metadata change.

**Round-trip Safe**: Yes

---

### 16. 📝change-description

**Files**:
- Mutation: `📝change-description/🦠️mutation/🦀️component.rs:12-27`
- Diff: `📝change-description/🔺️diff/🦀️component.rs:5-11`
- Inverse: `📝change-description/↩️inverse/🦀️component.rs:5-9`

**Payload Fields**:
- `new_description: String`

**DSL Keyword**: `"change-description"` (no block fields)

**SEMANTICS**: verb=`"change"`, entity=`"description"`, kind=`"change-description"`, record=`"ChangedDescription"`

**Diff Shape**: metadata/top-level (UNKNOWN)

**Rejection Conditions**: NONE — total function

**No-op Behavior**: `accepted-empty-diff` if description unchanged

**Inverse**: Returns 1 mutation (`change_description` with base description). Reads base.

**Cascade**: None — metadata change.

**Round-trip Safe**: Yes

---

### 17. 🔃rotate-part3d

**Files**:
- Mutation: `🔃rotate-part3d/🦠️mutation/🦀️component.rs:12-30`
- Diff: `🔃rotate-part3d/🔺️diff/🦀️component.rs:5-20`
- Inverse: `🔃rotate-part3d/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_orientation: Option<[f64; 4]>` (quaternion)

**DSL Keyword**: `"rotate-part3d"` (no block fields)

**SEMANTICS**: verb=`"rotate"`, entity=`"part"`, kind=`"rotate-part3d"`, record=`"RotatedPart3d"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if orientation unchanged

**Inverse**: Returns 1 mutation (`rotate_part_3d` with base orientation). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 18. 🔌replace-part-grip

**Files**:
- Mutation: `🔌replace-part-grip/🦠️mutation/🦀️component.rs:12-35`
- Diff: `🔌replace-part-grip/🔺️diff/🦀️component.rs:5-27`
- Inverse: `🔌replace-part-grip/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `part_id: String`
- `grip_id: String`
- `new_grip: Puzzle5dGrip` (**block field**)

**DSL Keyword**: `"replace-part-grip"` | block: `new_grip`

**SEMANTICS**: verb=`"replace"`, entity=`"part-grip"`, kind=`"replace-part-grip"`, record=`"ReplacedPartGrip"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found or grip_id not in part

**No-op Behavior**: `accepted-empty-diff` if new_grip equals old grip

**Inverse**: Returns 1 mutation (`replace_part_grip` with base grip). Reads base.

**Cascade**: None — modifies only the grip.

**Round-trip Safe**: Yes (but note: if fasteners were attached to old grip, they are NOT touched; replacement is atomic at grip level)

---

### 19. 🔒change-part2d-locked

**Files**:
- Mutation: `🔒change-part2d-locked/🦠️mutation/🦀️component.rs:12-31`
- Diff: `🔒change-part2d-locked/🔺️diff/🦀️component.rs:5-20`
- Inverse: `🔒change-part2d-locked/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_locked: Option<bool>`

**DSL Keyword**: `"change-part2d-locked"` (no block fields)

**SEMANTICS**: verb=`"change"`, entity=`"part"`, kind=`"change-part2d-locked"`, record=`"ChangedPart2dLocked"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if locked state unchanged

**Inverse**: Returns 1 mutation (`change_part_2d_locked` with base locked state). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 20. 🔗connect-grips

**Files**:
- Mutation: `🔗connect-grips/🦠️mutation/🦀️component.rs:12-43`
- Diff: `🔗connect-grips/🔺️diff/🦀️component.rs:5-21`
- Inverse: `🔗connect-grips/↩️inverse/🦀️component.rs:5-9`

**Payload Fields**:
- `id: String` (fastener id)
- `source: String` (full grip id, `part_id:grip_id`)
- `target: String` (full grip id)
- `fastener_kind: Option<String>`
- `gap: f64`
- `shift: f64`
- `rise: f64`
- `rotation: f64`
- `turn: f64`
- `tilt: f64`
- `x: f64`
- `y: f64`

**DSL Keyword**: `"connect-grips"` (no block fields)

**SEMANTICS**: verb=`"connect"`, entity=`"grips"`, kind=`"connect-grips"`, record=`"ConnectedGrips"`

**Diff Shape**: `fasteners` (added: list)

**Rejection Conditions**: NONE — total function (duplicate fastener id returns no-op warning, not error)

**No-op Behavior**: `accepted-empty-diff` if fastener id already exists (warning `"already connected"`)

**Inverse**: Returns 1 mutation (`disconnect_grips` with fastener.id). Does NOT read base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 21. 🖋️edit-part3d-label

**Files**:
- Mutation: `🖋️edit-part3d-label/🦠️mutation/🦀️component.rs:12-31`
- Diff: `🖋️edit-part3d-label/🔺️diff/🦀️component.rs:5-20`
- Inverse: `🖋️edit-part3d-label/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_label: Option<String>`

**DSL Keyword**: `"edit-part3d-label"` (no block fields)

**SEMANTICS**: verb=`"edit"`, entity=`"part"`, kind=`"edit-part3d-label"`, record=`"EditedPart3dLabel"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if label unchanged

**Inverse**: Returns 1 mutation (`edit_part_3d_label` with base label). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 22. 🗑delete-part

**Files**:
- Mutation: `🗑delete-part/🦠️mutation/🦀️component.rs:12-32`
- Diff: `🗑delete-part/🔺️diff/🦀️component.rs:5-26`
- Inverse: `🗑delete-part/↩️inverse/🦀️component.rs:5-21`

**Payload Fields**:
- `id: String`

**DSL Keyword**: `"delete-part"` (no block fields)

**SEMANTICS**: verb=`"delete"`, entity=`"part"`, kind=`"delete-part"`, record=`"DeletedPart"`

**Diff Shape**: `parts` (removed: list), `fasteners` (removed: list if any fasteners touched this part's grips)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `rejected` if part not found

**Inverse**: Returns 1 + N mutations (1x `create_part` + Nx `connect_grips` for each severed fastener). Reads base for part details and affected fasteners. If target missing, returns `Vec::new()`.

**Cascade**: Removes all fasteners that touch any of this part's grips (cascade repair via inverse).

**Round-trip Safe**: Yes

---

### 23. 🙈change-part2d-hidden

**Files**:
- Mutation: `🙈change-part2d-hidden/🦠️mutation/🦀️component.rs:12-31`
- Diff: `🙈change-part2d-hidden/🔺️diff/🦀️component.rs:5-20`
- Inverse: `🙈change-part2d-hidden/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_hidden: Option<bool>`

**DSL Keyword**: `"change-part2d-hidden"` (no block fields)

**SEMANTICS**: verb=`"change"`, entity=`"part"`, kind=`"change-part2d-hidden"`, record=`"ChangedPart2dHidden"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if hidden state unchanged

**Inverse**: Returns 1 mutation (`change_part_2d_hidden` with base hidden state). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 24. 🚀move-part3d

**Files**:
- Mutation: `🚀move-part3d/🦠️mutation/🦀️component.rs:12-30`
- Diff: `🚀move-part3d/🔺️diff/🦀️component.rs:5-20`
- Inverse: `🚀move-part3d/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_origin: [f64; 3]`

**DSL Keyword**: `"move-part3d"` (no block fields)

**SEMANTICS**: verb=`"move"`, entity=`"part"`, kind=`"move-part3d"`, record=`"MovedPart3d"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if origin unchanged

**Inverse**: Returns 1 mutation (`move_part_3d` with base origin). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 25. 🤝connect-kind-compatibility

**Files**:
- Mutation: `🤝connect-kind-compatibility/🦠️mutation/🦀️component.rs:12-36`
- Diff: `🤝connect-kind-compatibility/🔺️diff/🦀️component.rs:5-21`
- Inverse: `🤝connect-kind-compatibility/↩️inverse/🦀️component.rs:5-9`

**Payload Fields**:
- `source: String`
- `target: String`
- `bidirectional: bool`
- `important: bool`
- `specificity: Puzzle5dCompatSpecificity`

**DSL Keyword**: `"connect-kind-compatibility"` (no block fields)

**SEMANTICS**: verb=`"connect"`, entity=`"kind-compatibility"`, kind=`"connect-kind-compatibility"`, record=`"ConnectedKindCompatibility"`

**Diff Shape**: `kind_compatibility` (full list replacement)

**Rejection Conditions**: NONE — total function (duplicate (source, target) pair returns no-op warning, not error)

**No-op Behavior**: `accepted-empty-diff` if (source, target) pair already exists (warning `"already connected"`)

**Inverse**: Returns 1 mutation (`disconnect_kind_compatibility` with source/target). Does NOT read base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 26. 🧊replace-part2d-geometry

**Files**:
- Mutation: `🧊replace-part2d-geometry/🦠️mutation/🦀️component.rs:12-34`
- Diff: `🧊replace-part2d-geometry/🔺️diff/🦀️component.rs:5-25`
- Inverse: `🧊replace-part2d-geometry/↩️inverse/🦀️component.rs:5-16`

**Payload Fields**:
- `id: String`
- `new_shape: Option<String>`
- `new_radius: Option<f64>`
- `new_width: Option<f64>`
- `new_height: Option<f64>`

**DSL Keyword**: `"replace-part2d-geometry"` (no block fields)

**SEMANTICS**: verb=`"replace"`, entity=`"part"`, kind=`"replace-part2d-geometry"`, record=`"ReplacedPart2dGeometry"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if geometry unchanged

**Inverse**: Returns 1 mutation (`replace_part_2d_geometry` with base geometry fields). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 27. 🧮replace-fastener-geometry

**Files**:
- Mutation: `🧮replace-fastener-geometry/🦠️mutation/🦀️component.rs:12-36`
- Diff: `🧮replace-fastener-geometry/🔺️diff/🦀️component.rs:5-26`
- Inverse: `🧮replace-fastener-geometry/↩️inverse/🦀️component.rs:5-18`

**Payload Fields**:
- `id: String`
- `new_gap: f64`
- `new_shift: f64`
- `new_rise: f64`
- `new_rotation: f64`
- `new_turn: f64`
- `new_tilt: f64`
- `new_x: f64`
- `new_y: f64`

**DSL Keyword**: `"replace-fastener-geometry"` (no block fields)

**SEMANTICS**: verb=`"replace"`, entity=`"fastener"`, kind=`"replace-fastener-geometry"`, record=`"ReplacedFastenerGeometry"`

**Diff Shape**: `fasteners` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — fastener id not found

**No-op Behavior**: `accepted-empty-diff` if geometry unchanged

**Inverse**: Returns 1 mutation (`replace_fastener_geometry` with base geometry values). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

### 28. 🧱change-part3d-mesh

**Files**:
- Mutation: `🧱change-part3d-mesh/🦠️mutation/🦀️component.rs:12-31`
- Diff: `🧱change-part3d-mesh/🔺️diff/🦀️component.rs:5-20`
- Inverse: `🧱change-part3d-mesh/↩️inverse/🦀️component.rs:5-12`

**Payload Fields**:
- `id: String`
- `new_mesh_url: Option<String>`

**DSL Keyword**: `"change-part3d-mesh"` (no block fields)

**SEMANTICS**: verb=`"change"`, entity=`"part"`, kind=`"change-part3d-mesh"`, record=`"ChangedPart3dMesh"`

**Diff Shape**: `parts` (patched: replacement)

**Rejection Conditions**:
- `"mutation.target-missing"` — part id not found

**No-op Behavior**: `accepted-empty-diff` if mesh_url unchanged

**Inverse**: Returns 1 mutation (`change_part_3d_mesh` with base mesh_url). Reads base.

**Cascade**: None.

**Round-trip Safe**: Yes

---

## Summary Tables

### Table A: Error Codes Inventory

| Mutation | Can Reject? | Error Codes |
|----------|------------|------------|
| ⚓change-part-anchor | Yes | `mutation.target-missing` |
| ✂️disconnect-grips | Yes | `mutation.target-missing` |
| ✏️edit-part2d-text | Yes | `mutation.target-missing` |
| ➕add-part-grip | Yes | `mutation.target-missing`, `mutation.duplicate-id` |
| ➖remove-part-grip | Yes | `mutation.target-missing` |
| 🌐change-domain | No | — |
| 🌱create-part | Yes | `mutation.duplicate-id` |
| 🎨change-part2d-icon | Yes | `mutation.target-missing` |
| 🎯change-fastener-kind | Yes | `mutation.target-missing` |
| 🏗change-part-kind | Yes | `mutation.target-missing` |
| 🏷rename-puzzle5d | No | — |
| 💔disconnect-kind-compatibility | Yes | `mutation.target-missing` |
| 📍move-part2d | Yes | `mutation.target-missing` |
| 📏scale-part3d | Yes | `mutation.target-missing` |
| 📚replace-kind-catalogs | No | — |
| 📝change-description | No | — |
| 🔃rotate-part3d | Yes | `mutation.target-missing` |
| 🔌replace-part-grip | Yes | `mutation.target-missing` |
| 🔒change-part2d-locked | Yes | `mutation.target-missing` |
| 🔗connect-grips | No | — |
| 🖋️edit-part3d-label | Yes | `mutation.target-missing` |
| 🗑delete-part | Yes | `mutation.target-missing` |
| 🙈change-part2d-hidden | Yes | `mutation.target-missing` |
| 🚀move-part3d | Yes | `mutation.target-missing` |
| 🤝connect-kind-compatibility | No | — |
| 🧊replace-part2d-geometry | Yes | `mutation.target-missing` |
| 🧮replace-fastener-geometry | Yes | `mutation.target-missing` |
| 🧱change-part3d-mesh | Yes | `mutation.target-missing` |

**Distinct Error Codes**: 2 total
- `mutation.target-missing` — entity id not found in base (used by 22 mutations)
- `mutation.duplicate-id` — id/pair already exists; used by create-part and add-part-grip

---

### Table B: Inverse Return Behavior

| Mutation | Inverse Returns | Reads Base? |
|----------|-----------------|------------|
| ⚓change-part-anchor | 1 mutation | Yes |
| ✂️disconnect-grips | 1 mutation | Yes |
| ✏️edit-part2d-text | 1 mutation | Yes |
| ➕add-part-grip | 1 mutation | No |
| ➖remove-part-grip | 1 + N mutations | Yes |
| 🌐change-domain | 1 mutation | Yes |
| 🌱create-part | 1 mutation | No |
| 🎨change-part2d-icon | 1 mutation | Yes |
| 🎯change-fastener-kind | 1 mutation | Yes |
| 🏗change-part-kind | 1 mutation | Yes |
| 🏷rename-puzzle5d | 1 mutation | Yes |
| 💔disconnect-kind-compatibility | 1 mutation | Yes |
| 📍move-part2d | 1 mutation | Yes |
| 📏scale-part3d | 1 mutation | Yes |
| 📚replace-kind-catalogs | 1 mutation | Yes |
| 📝change-description | 1 mutation | Yes |
| 🔃rotate-part3d | 1 mutation | Yes |
| 🔌replace-part-grip | 1 mutation | Yes |
| 🔒change-part2d-locked | 1 mutation | Yes |
| 🔗connect-grips | 1 mutation | No |
| 🖋️edit-part3d-label | 1 mutation | Yes |
| 🗑delete-part | 1 + N mutations | Yes |
| 🙈change-part2d-hidden | 1 mutation | Yes |
| 🚀move-part3d | 1 mutation | Yes |
| 🤝connect-kind-compatibility | 1 mutation | No |
| 🧊replace-part2d-geometry | 1 mutation | Yes |
| 🧮replace-fastener-geometry | 1 mutation | Yes |
| 🧱change-part3d-mesh | 1 mutation | Yes |

**Summary**:
- **Returns 1 mutation**: 24 mutations
- **Returns 1 + N mutations**: 2 mutations (➖remove-part-grip, 🗑delete-part — both have cascade cleanup)
- **Never reads base**: 3 mutations (➕add-part-grip, 🌱create-part, 🔗connect-grips, 🤝connect-kind-compatibility)

---

### Table C: No-op Behavior Classification

| Mutation | Classification | Behavior |
|----------|-----------------|----------|
| ⚓change-part-anchor | `accepted-empty-diff` | No change → warning, empty diff accepted |
| ✂️disconnect-grips | `rejected` | Target not found → error |
| ✏️edit-part2d-text | `accepted-empty-diff` | No change → warning, empty diff accepted |
| ➕add-part-grip | `rejected` | Duplicate grip_id → error |
| ➖remove-part-grip | `rejected` | Grip not found → error |
| 🌐change-domain | `accepted-empty-diff` | No explicit check, diff succeeds |
| 🌱create-part | `rejected` | Duplicate part id → fatal error |
| 🎨change-part2d-icon | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🎯change-fastener-kind | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🏗change-part-kind | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🏷rename-puzzle5d | `accepted-empty-diff` | No explicit check, diff succeeds |
| 💔disconnect-kind-compatibility | `rejected` | Pair not found → error |
| 📍move-part2d | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 📏scale-part3d | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 📚replace-kind-catalogs | `accepted-empty-diff` | No explicit check, diff succeeds |
| 📝change-description | `accepted-empty-diff` | No explicit check, diff succeeds |
| 🔃rotate-part3d | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🔌replace-part-grip | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🔒change-part2d-locked | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🔗connect-grips | `accepted-empty-diff` | Duplicate fastener id → warning, empty diff accepted |
| 🖋️edit-part3d-label | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🗑delete-part | `rejected` | Target not found → error |
| 🙈change-part2d-hidden | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🚀move-part3d | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🤝connect-kind-compatibility | `accepted-empty-diff` | Duplicate pair → warning, empty diff accepted |
| 🧊replace-part2d-geometry | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🧮replace-fastener-geometry | `accepted-empty-diff` | No change → warning, empty diff accepted |
| 🧱change-part3d-mesh | `accepted-empty-diff` | No change → warning, empty diff accepted |

**Summary**:
- **accepted-empty-diff**: 21 mutations — idempotent: multiple applications are harmless
- **rejected**: 7 mutations — attempting a no-op state fails (e.g., duplicate-id, target-missing)

---

### Table D: Round-Trip Safety Analysis

**Finding**: All 28 mutations are round-trip safe.

Reasoning:
1. **Simple one-to-one inverses** (24 mutations) — return exactly one inverse mutation that restores the prior value by reading from base. Example: `ChangePartAnchor` → inverse reads base.anchor and returns `ChangePartAnchor` with that value.
2. **Cascading mutations with multi-return inverses** (2 mutations):
   - **➖remove-part-grip**: Inverse returns `add_part_grip` + one `connect_grips` per severed fastener. Applying the forward mutation removes the grip and severs fasteners; applying the inverse adds the grip back AND re-connects every fastener. Round-trip restores exact before-state.
   - **🗑delete-part**: Inverse returns `create_part` + one `connect_grips` per severed fastener. Applying the forward mutation removes the part and severs fasteners; applying the inverse recreates the part AND re-connects every fastener. Round-trip restores exact before-state.
3. **Append-only create mutations** (2 mutations):
   - **🌱create-part**: Inverse returns `delete_part`. Order is captured via `reordered` field in diff and via `index` in CreatePart payload, so round-trip is safe.
   - **🔗connect-grips**: Inverse returns `disconnect_grips`. Appending a fastener and then disconnecting it restores prior state.

**Conclusion**: No exceptions. All 28 mutations are exactly round-trip safe — applying mutation then inverse restores the exact before-state.

---

## Notes

### Cascade Behavior Summary
Only **2 mutations** perform cascade-aware removals:
1. **➖remove-part-grip** — removes all fasteners that touch the removed grip; inverse re-connects them.
2. **🗑delete-part** — removes all fasteners that touch any of the deleted part's grips; inverse re-connects them.

### Block Fields
Only **3 mutations** use `#[dsl(block)]` for multi-line payload blocks:
1. **🌱create-part**: `part` field (the full `Puzzle5dPart` definition)
2. **➕add-part-grip**: `grip` field (the `Puzzle5dGrip` definition)
3. **🔌replace-part-grip**: `new_grip` field (the replacement `Puzzle5dGrip`)

### Total Functions (Never Reject)
**4 mutations** have diff builders that always succeed:
1. **🌐change-domain** — metadata change
2. **🏷rename-puzzle5d** — metadata change
3. **📚replace-kind-catalogs** — metadata change
4. **📝change-description** — metadata change

Plus **2 create mutations** that warn on no-op but never reject:
- **🔗connect-grips** — warns if duplicate, but still succeeds
- **🤝connect-kind-compatibility** — warns if duplicate, but still succeeds

