# Compose Diff System Census — H34 Legacy

**FINDING: NO legacy diff-inversion function exists in Rust. `metabolism.kit.diff.inverted.compose.json` is hand-authored.**

---

## 1. `design-with-diff.cases.compose.json` — Case Contract

| Property | Value |
|----------|-------|
| **File** | `compose/fixture/design-with-diff.cases.compose.json` |
| **Case count** | 1 |
| **Case names** | `nakagin_capsule_tower` |

**Case schema** (single entry in `cases[]`):
- `name: string` — test identifier
- `kit: string` — path to baseline kit (e.g., `kit/dev/metabolism/wip/initialKit/kit.compose.json`)
- `designName: string` — design entity name being modified
- `diff: string` — fixture filename of diff to apply (e.g., `nakagin-capsule-tower.diff.design.compose.json`)
- `expected: string` — fixture filename of expected state after diff applied
- `expectedPieceCounts: object`
  - `unchanged: int`
  - `modified: int`
  - `removed: int`
  - `added: int`
- `expectedConnectionCounts: object` — same structure for connections
- `designFamilies: array` — (empty in this case)

---

## 2. Fixture Relationships & Diffs

### Nakagin Capsule Tower Example

**Files:**
- `/compose/fixture/nakagin-capsule-tower.diff.design.compose.json` — **the diff**
- `/compose/fixture/nakagin-capsule-tower.with-diff.design.compose.json` — **expected after apply**
- Also related: `/compose/fixture/nakagin-capsule-tower.paste.design.diff.compose.json`, `/compose/fixture/nakagin-capsule-tower.deleted.selection.compose.json`, etc. (different operations)

**Diff structure** (line 2-316 of `nakagin-capsule-tower.diff.design.compose.json`):
```json
{
  "pieces": {
    "removed": [ { "id": "cb832a2c-e9db-..." }, ... ],
    "updated": [ { "piece": { "id": "..." }, "diff": { "description": "..." } }, ... ],
    "added": [ { "id": "...", "name": "...", "type": { "id": "..." }, "description": "...", "pose": { ... } }, ... ]
  },
  "connections": {
    "removed": [ { "id": "..." }, ... ],
    "updated": [ { "connection": { "id": "..." }, "diff": { "parent": { "piece": { "id": "..." }, "connector": { "id": "..." } } } } ],
    "added": [ { "id": "...", "parent": { "piece": { "id": "..." }, "connector": { "id": "..." } }, "child": { ... }, "gap": 0, "shift": 0, ... } ]
  }
}
```

**Applied result** (nakagin-capsule-tower.with-diff.design.compose.json) includes `compose.diffStatus` attributes on each piece:
- `value: "unchanged"` — unmodified
- `value: "modified"` — changed scalar fields
- `value: "removed"` — deleted
- `value: "added"` — created (added pieces marked with these attributes post-apply)

### Metabolism Kit Example

**Files:**
- `/compose/fixture/metabolism.kit.diff.compose.json` — **the diff** (195 lines)
- `/compose/fixture/metabolism.kit.diff.inverted.compose.json` — **hand-authored inverse** (167 lines)

**Inverted structure comparison:**
- Original `tags.added[]` entries have `{ id, owner_id, attribute_ids, tag: TagInput { name, description, icon } }`
- Inverted `tags.removed[]` entries have only `{ id }`
- Original `tags.removed[]` entries have only `{ id }`
- Inverted `tags.added[]` entries have `{ id, name }` (simplified!)

This confirms: **inversion is hand-authored, not algorithmically complete.**

---

## 3. Diff Data Model — Complete Field Inventory

**Type:** `CanonicalKitDiff` (Rust `/compose/client/lib/rs/lib.rs:9308-9327`)

| Field | Type | Description |
|-------|------|-------------|
| `name` | `Option<String>` | Kit name change |
| `version` | `Option<String>` | Kit version change |
| `description` | `Option<String>` | Kit description change |
| `icon` | `Option<String>` | Kit icon resource path |
| `image` | `Option<String>` | Kit preview image path |
| `remote` | `Option<String>` | Kit remote URL (tarball/archive) |
| `homepage` | `Option<String>` | Kit homepage URL |
| `license` | `Option<String>` | Kit license identifier |
| `preview` | `Option<String>` | Kit preview resource path |
| `types` | `Option<TypesCollectionDiff>` | Type definitions (removed/modified/added) |
| `designs` | `Option<DesignsCollectionDiff>` | Designs (removed/modified/added) |
| `tags` | `Option<TagsCollectionDiff>` | Tags (removed/modified/added) |
| `concepts` | `Option<ConceptsCollectionDiff>` | Concepts (removed/modified/added) |
| `qualities` | `Option<QualitiesCollectionDiff>` | Qualities (removed/modified/added) |
| `files` | `Option<FilesCollectionDiff>` | Files (removed/modified/added) |
| `folders` | `Option<FoldersCollectionDiff>` | Folders (removed/modified/added) |
| `families` | `Option<FamiliesCollectionDiff>` | Families (removed/modified/added) |
| `authors` | `KitAuxSubtree` | Tri-state: `None` (omitted), `Some(false)` (empty subtree), `Some(true)` (unsupported non-empty) |

### Collection Diffs (Per-collection Schema)

Each collection (`types`, `designs`, `tags`, etc.) follows this pattern:

```rust
pub struct TypesCollectionDiff {
    pub removed: Vec<IdRef>,              // { id: Id }
    pub modified: Vec<TypeModified>,      // { type_ref: IdRef, diff: TypeScalarDiff }
    pub added: Vec<TypeAdded>,            // { owner_id, id, name, description?, icon?, image?, unit? }
}
```

**Removed entries:** Only `{ id }` (minimum destructive record)

**Modified entries:** 
- Piece: `{ piece: IdRef, diff: PiecePatch }`
- Type: `{ type_ref: IdRef, diff: TypeScalarDiff }`
- Design: `{ design: IdRef, diff: DesignDiff }`
- Tag: `{ tag: IdRef, diff: TagPatch }`
- etc.

**Added entries:** Full entity structure with owner context:
- Type: `{ owner_id, id, name, description?, icon?, image?, unit? }`
- Design: `{ owner_id, id, name, description?, icon?, image?, unit? }`
- Piece (in design): `{ id, blueprint_id, name?, description?, scale, pose }`
- Tag: `{ owner_id, id, attribute_ids, tag: TagInput }`
- etc.

**Design-specific detail** (`DesignDiff`):
```rust
pub struct DesignDiff {
    pub scalars: DesignScalarDiff,    // name?, description?, icon?, image?, folder_id?
    pub pieces: Option<PiecesCollectionDiff>,  // nested pieces changes
}
```

### DesignDiff Nesting

When a design is modified, its pieces can be simultaneously added/removed/modified:
- Apply logic recurses: `apply_designs_collection_diff()` → `apply_design_piece_patch()`

---

## 4. Rust Implementation — Functions & Signatures

| Function | Location | Signature | Purpose |
|----------|----------|-----------|---------|
| `apply_diff` | `lib.rs:5817` | `pub async fn apply_diff(self: &Arc<Self>, diff: &crate::operation::KitDiff) -> Result<(), ComposeError>` | Apply a complete kit diff (mutation) |
| `apply_types_collection_diff` | `lib.rs:5886` | `async fn apply_types_collection_diff(self: &Arc<Self>, t: &TypesCollectionDiff) -> Result<(), ComposeError>` | Apply type removals/modifications/additions |
| `apply_designs_collection_diff` | `lib.rs:5981` | `async fn apply_designs_collection_diff(self: &Arc<Self>, d: &DesignsCollectionDiff) -> Result<(), ComposeError>` | Apply design changes (recurses into pieces) |
| `apply_design_piece_patch` | `lib.rs:6049` | `async fn apply_design_piece_patch(self: &Arc<Self>, design_id: &Id, piece_id: &Id, pdiff: &PiecePatch) -> Result<(), ComposeError>` | Patch single piece (name, description, pose, etc.) |
| `apply_tags_collection_diff` | `lib.rs:6088` | `async fn apply_tags_collection_diff(self: &Arc<Self>, t: &TagsCollectionDiff) -> Result<(), ComposeError>` | Apply tag changes |
| `apply_concepts_collection_diff` | `lib.rs:6110` | `async fn apply_concepts_collection_diff(self: &Arc<Self>, c: &ConceptsCollectionDiff) -> Result<(), ComposeError>` | Apply concept changes |
| `apply_qualities_collection_diff` | `lib.rs:6132` | `async fn apply_qualities_collection_diff(self: &Arc<Self>, q: &QualitiesCollectionDiff) -> Result<(), ComposeError>` | Apply quality changes |
| `to_diff` (Operation) | `lib.rs:9659` | `pub async fn to_diff(&self, kit: &Arc<Kit>) -> Result<KitDiff, ComposeError>` | Generate diff from an Operation against a Kit state |
| `absorb` (KitDiff) | `lib.rs:9334` | `pub fn absorb(&mut self, other: CanonicalKitDiff)` | Shallow-merge: last-one-wins scalar/collection fields |
| `to_backwards` (Operation) | `lib.rs:10165` | `pub async fn to_backwards(&self, kit: &Arc<Kit>) -> Result<Vec<Operation>, ComposeError>` | **Operation-level** inversion (NOT diff-level) |

**No diff-level inversion function exists.**

---

## 5. Inversion: Analysis

### Operation-Level Inversion: `to_backwards()`

Located at `lib.rs:10165`. Used in VCS trait:
```rust
fn inverse(&self, projection: &KitSnapshot) -> Vec<Self> {
    operation.to_backwards(&kit).await
        .into_iter()
        .map(|row| ComposeWireOperation::from_operation(&row))
        .collect()
}
```

**Scope:** Produces the **inverse operations** that would undo the forward operation. Example:
- `CreateDesign { ... }` → inverse is `[DeleteDesign { ... }]`
- `CreateTag { ... }` → inverse is `[DeleteTag { ... }]`
- Compound operations may invert to multiple steps

**NOT diff-level inversion:** This operates on operations (e.g., `CreateTag`), not on diffs.

### Diff-Level Inversion: ABSENT

No function produces an inverted `KitDiff` or `DesignDiff` that would:
- Swap `added ↔ removed`
- Negate scalar diffs (e.g., `name: "new" → name: "old"`)
- Recurse through nested structures

The fixture `metabolism.kit.diff.inverted.compose.json` is **hand-authored** (verified by structure comparison above).

---

## 6. Absorb & Compose: Shallow Merge Only

### `KitDiff::absorb()`

**Line:** `lib.rs:9334`

```rust
pub fn absorb(&mut self, other: CanonicalKitDiff) {
    let a = &mut self.0;
    let b = other;
    // For each field (name, version, types, designs, tags, ...):
    // if b.field.is_some() { a.field = b.field; }
}
```

**Semantics:**
- Last-one-wins for scalar fields (name, version, etc.)
- Last-one-wins for collection fields (types, designs, etc.)
- **NOT associative:** `(A.absorb(B)).absorb(C)` ≠ `A.absorb((B.absorb(C)))`

**Example:** If diff A has `types.added = [T1]` and diff B has `types.added = [T2]`, then `A.absorb(B)` results in `types.added = [T2]` (B overwrites A).

### No Proper Compose

There is **no function that merges two diffs while preserving their combined effect** on a kit:
- No per-collection append (would need `types.added ← types.added + other.types.added`)
- No deduplication or rebase logic
- No associativity property

The only reference to "compose" is operational (e.g., `compose_report`, `ComposeError`), not diff composition.

---

## 7. Validation: `validate-kit-diff.cases.compose.json`

**File:** `/compose/fixture/validate-kit-diff.cases.compose.json` (272 lines)

**Test kit:** `tinyKit` with 1 type, 1 design, 1 piece, etc.

| Case ID | Diff Input | Expected | Error Codes | Warning Codes |
|---------|-----------|----------|-------------|---------------|
| `empty-diff-ok` | `{}` | ✓ OK | — | — |
| `update-missing-design` | Update design ID `99999999-9999-...` (non-existent) | ✗ ERROR | `kitdiff.update.missing-target` | — |
| `remove-missing-type-warns` | Remove type ID `ffffffff-ffff-...` (non-existent) | ✓ OK | — | `kitdiff.remove.missing-target` |
| `add-piece-bad-type` | Add piece to design D1 with type `eeeeeeee-...` (non-existent) | ✗ ERROR | `kitdiff.ref.piece-type-missing` | — |
| `no_operation-remove-add-type-warns` | Remove type T1, then add T1 with same ID | ✓ OK | — | `kitdiff.cycle.no-operation-restore` |

**Validation behavior:**
- Updates to missing targets: **error** (reference violation)
- Removals of missing targets: **warning only** (idempotent)
- References to missing types/designs: **error** (integrity constraint)
- No-op cycles (remove+add same ID): **warning** (inefficiency)

**Applying an invalid diff:** Depends on which target is missing:
- Missing design/type target → error returned, kit **not modified**
- Missing reference (e.g., piece type) → error returned, kit **not modified**

---

## 8. Other Language Implementations

### Go (`/compose/client/lib/go/main.go`)

**Functions present:**
- `func (d *DevKit) Apply(diff *KitDiff)` — line 2049
- `func (l *LocalKit) Apply(diff *KitDiff)` — line 2085
- `func (r *RemoteKit) Apply(diff *KitDiff)` — line 2121
- `func ApplyKitDiff(kit *Kit, diff *KitDiff)` — line 9907
- `func ApplyDesignDiff(design *Design, diff *DesignDiff)` — line 14471

**Types defined:** `KitDiff`, `DesignDiff`, `TypeDiff`, `TagDiff`, `PieceDiff`, etc. (lines 497–1582 show diff types).

**Status:** Partial — apply functions exist; inversion/absorb/compose status unknown (requires deeper audit).

### JavaScript (`/compose/client/lib/js/index.ts`)

**Search:** No `diff`, `apply`, `absorb`, `invert` functions found in index.ts.

**Status:** Not implemented in JS layer (may rely on Rust wasm bindings or GraphQL mutations).

### .NET (`/compose/client/lib/net/Compose/cs/Compose.cs`)

**Search:** No `ApplyDiff`, `Absorb`, `Invert` methods found.

**Status:** Not implemented in .NET layer.

---

## 9. Tests Exercising Diff/Apply

### Rust Tests

| Test Name | Location | Purpose |
|-----------|----------|---------|
| `normalized_kit_operation_create_tag_diff_and_backwards_use_scoped_ids` | `lib.rs:21316` | Verify `CreateTag.to_diff()` produces correct `TagAdded`; verify `apply_diff()` can materialize it |
| `normalized_kit_operation_create_design_diff_backwards_and_json_roundtrip` | `lib.rs:21369` | Verify `CreateDesign.to_diff()` and `apply_diff()` round-trip |
| `normalized_kit_operation_create_type_diff_backwards_and_json_roundtrip` | `lib.rs:21426` | Verify `CreateType.to_diff()` and `apply_diff()` round-trip |
| `canonical_kit_diff_metabolism_fixture_has_contract_keys` | `lib.rs:21460` | Validate `metabolism.kit.diff.compose.json` structure (name, types, designs present with added entries) |

**Note:** No tests for:
- `absorb()` behavior
- diff composition/merge
- diff inversion
- applying diffs to design (design-with-diff test cases at query layer; see below)

### Query / E2E Tests

- **File:** `/compose/client/lib/query/rs/lib.rs`
- **Test suite:** `architect_cases_e2e_suite` (line 1289) runs against Nakagin design and other fixtures
- **Fixture:** `design-with-diff.cases.compose.json` is loaded and executed in E2E tests (specific line not yet located, but test harness loads `cases_for_tier(&doc, "e2e")`)

---

## Summary Table: Diff Model Completeness

| Aspect | Status | Details |
|--------|--------|---------|
| **Data model** | ✓ Complete | `CanonicalKitDiff` covers all Kit + Design nested scalars & collections |
| **Apply (forward)** | ✓ Complete | Rust `Kit::apply_diff()` + collection-specific apply methods |
| **Diff generation** | ✓ Complete | `Operation::to_diff()` produces diffs from operations |
| **Inversion (diff-level)** | ✗ Missing | No function; `metabolism.kit.diff.inverted.compose.json` is hand-authored |
| **Inversion (operation-level)** | ✓ Complete | `Operation::to_backwards()` inverts operations (not diffs) |
| **Composition (merge)** | ⚠️ Partial | `KitDiff::absorb()` does shallow last-one-wins merge; no per-collection append |
| **Associativity (absorb)** | ✗ No | Absorb is not associative; later diffs completely overwrite fields |
| **Validation** | ✓ Complete | Reference integrity checks (missing targets, missing entity types) |
| **Go implementation** | ✓ Partial | Apply functions exist; inversion/absorb unclear |
| **JS/NET** | ✗ Not found | No diff operations in JS or .NET layers |

---

## Key Risks for Migration

1. **No diff-level inversion:** Migration must implement `invert(diff) → inverted_diff` if needed for undo/branching.
2. **Shallow absorb:** If migration needs semantic merge of diffs, shallow last-one-wins won't suffice.
3. **Hand-authored inverted fixture:** The `metabolism.kit.diff.inverted.compose.json` is incomplete; can't be used as an oracle for inversion semantics.
4. **Missing tests:** No tests validate absorb behavior or diff composition.

