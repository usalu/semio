# H36/H38/H39 Census — Validation, Filter-Kit, Find-Replaceable-Types

## A. VALIDATION / DIAGNOSTICS

### 1. Validation Fixture Coverage

**`compose/fixture/validation.compose.json`** encodes 10 distinct invalid conditions (one per problem):
- Duplicate entity ID (id-unique): Connection 019ab664-3333-3333-3333-333333333333
- Duplicate type name (type-name-unique): Two types named "Duplicate Type Name"
- Duplicate design name (design-name-unique): Two designs named "Duplicate Design Name"
- Duplicate piece name within design (piece-name-unique): Two pieces in same design
- Duplicate connector name within type (connector-name-unique): Two connectors in same type
- Duplicate representation name within type (representation-name-unique): Two representations in same type
- Duplicate quality name across kit (quality-name-unique): Two qualities
- Duplicate file name across kit (file-name-unique): Two files
- Duplicate folder name among siblings (folder-name-unique): Two folders with same parent
- Duplicate layer path within design (layer-path-unique): Two layers in same design

**`compose/fixture/invalid.kit.compose.json`** demonstrates the schema carrying these issues — all listed above are present in structured form.

### 2. Diff-Validation Differences

**`compose/fixture/validate-kit-diff.cases.compose.json`** (6 cases) shows diff-validation differs from plain validation:

- **empty-diff-ok**: Empty diff passes validation (no errors/warnings)
- **update-missing-design**: Error `kitdiff.update.missing-target` when modifying a design that doesn't exist
- **remove-missing-type-warns**: Warning `kitdiff.remove.missing-target` when removing a non-existent type (does NOT error)
- **add-piece-bad-type**: Error `kitdiff.ref.piece-type-missing` when adding a piece referencing missing type
- **no_operation-remove-add-type-warns**: Warning `kitdiff.cycle.no-operation-restore` when removing and re-adding the same entity
- Plain validation only checks static invariants; diff-validation checks reference validity within diffs and detects cyclic operations

### 3. Rust Implementation Symbols

**`compose/client/lib/rs/lib.rs`** — UNKNOWN — workspace broken by peer refactor; `cargo build` not runnable. Line numbers for validation functions unavailable. Grep found:
- Line 19080: `pub fn kit_store_validate_comprehensive_fixture(fixture: &serde_json::Value)` — likely test helper

### 4. THE ISSUE MODEL — CRITICAL

**Problem struct (Python: `compose/client/lib/py/main.py:13441`)**
```python
@dataclasses.dataclass
class Problem:
    """🔒️A validation problem with a constraint identifier and message."""
    constraintId: str  # e.g., "id-unique", "type-name-unique"
    message: str      # Human-readable message with context
    entityKind: str   # e.g., "Connection", "Type", "Design", "Piece"
    entityId: str     # UUID of offending entity
    fixes: list[ValidationFix] = dataclasses.field(default_factory=list)
```

**ValidationFix struct (implicit in dict serialization)**:
```python
class ValidationFix:
    title: str        # e.g., 'Rename "Duplicate Type Name"'
    diff: dict        # Kit diff structure for one-click remediation
```

**Issue ORDER is DETERMINISTIC** (line 13470-13473):
```python
def toDict(self) -> dict:
    sortedProblems = sorted(
        self.problems, key=lambda i: (i.constraintId, i.entityId)
    )
```
Sorted by `(constraintId, entityId)` tuple lexicographically.

**Each Problem carries:**
- ✓ **Stable machine code**: `constraintId` (e.g., "id-unique")
- ✓ **Path/pointer**: `entityKind` + `entityId` (identifies offending entity)
- ✗ **Severity level**: NOT present (all treated equally)
- ✓ **Human message**: `message` field

### 5. Validation Rules — Exhaustive List

| Rule ID | Constraint | Check | Scope | Line (Py) |
|---------|-----------|-------|-------|-----------|
| id-unique | All entity IDs must be unique across kit | UUID collision detection | Global across all entity kinds | 13554 |
| type-name-unique | Type names unique among siblings | Group by parent, detect duplicate names | Within parent hierarchy | 13592 |
| design-name-unique | Design names unique among siblings | Group by parent, detect duplicate names | Within parent hierarchy | 13621 |
| piece-name-unique | Piece names unique within design | Dict by name per design | Per design | 13650 |
| connector-name-unique | Connector names unique within type | Dict by name per type | Per type | 13674 |
| representation-name-unique | Representation names unique within type | Dict by name per type | Per type | 13698 |
| quality-name-unique | Quality names unique across kit | Dict by name across all qualities | Global | 13722 |
| file-name-unique | File names unique across kit | Dict by name across all files | Global | 13744 |
| folder-name-unique | Folder names unique among siblings | Group by parent, detect duplicate names | Within parent hierarchy | 13766 |
| layer-path-unique | Layer paths unique within design | Dict by path per design | Per design | 13795 |

**Diff-Specific Rules** (not in plain validation, validated in `validate_kit_diff_dict` at line 11627):
| Rule ID | Constraint |
|---------|-----------|
| kitdiff.update.missing-target | Target entity to update must exist in kit before applying diff |
| kitdiff.remove.missing-target | Warns (does not error) if removing non-existent entity |
| kitdiff.ref.piece-type-missing | Piece added in diff must reference existing or added type |
| kitdiff.cycle.no-operation-restore | Warns if same entity removed then re-added in one diff |

---

## B. FILTER-KIT

### 6. Filter-Kit Test Cases

**`compose/fixture/filter-kit.cases.compose.json`** contains:

**Primary Cases (1 case)**:
- **nakagin_capsule_tower**: Input kit from `kit/dev/metabolism/wip/initialKit/kit.compose.json`, design `"Nakagin Capsule Tower"`, expected output `nakagin-capsule-tower.filtered.kit.compose.json`

**Glob Cases (5 cases)**:
- **type_include_capsule**: Include types matching `Capsule*` pattern
- **type_exclude_capsule**: Exclude types matching `Capsule*` pattern
- **design_include_nakagin**: Include designs matching `Nakagin*` pattern
- **empty_filter**: No filters applied (returns full kit)
- **combined_design_and_type_exclude**: Design-scoped filter + exclude pattern for types

**Glob Match Test Cases (19 cases)**:
- Wildcard matching: `Nakagin*`, `*Tower`, `*Capsule*`, exact match, `Other*` (false)
- Single-char wildcards: `W?ll`, `W??l`, `W????`
- Case-insensitive matching: `wall` matches `Wall`, etc.

**Expected Shape**: Each case has:
- Input kit path
- Optional design filter (by name)
- Optional include/exclude glob patterns per entity kind
- Expected output kit or result assertions

### 7. Nakagin Filtered Output Analysis

**`nakagin-capsule-tower.filtered.kit.compose.json`** (13,969 lines) encodes a **transitive design-scoped subset**:

**What was filtered out**:
- **Types not used by pieces in "Nakagin Capsule Tower"**: Removed all unused type definitions
- **Designs not the target or referenced by pieces**: Kept only the named design + designs used as piece type specifications
- **Ports not referenced by connectors of kept types**: Dropped orphaned ports
- **Files not referenced by selected representations**: Removed unused media assets
- **Tags/concepts/authors only if referenced**: Pruned unused metadata
- **One representation per type selected by tag matching**: Used Jaccard similarity to pick best match

**Filter criterion**: Start from design, collect pieces → collect piece types → collect ancestor types → transitively include dependencies (ports, files, tags) → output minimal self-contained kit for that design

### 8. Filter-Kit Rust Implementation

**`compose/client/lib/rs/lib.rs`** — UNKNOWN — workspace broken. Could not extract line numbers.

**Python Implementation** (`compose/client/lib/py/main.py`):
- Line 6062: `def filter_kit(self: "Kit", filter_spec: dict) -> "Kit"` — main entry point
- Line 6015: `_select_best_representation_filter` — Jaccard similarity for tag-based selection
- Line 6046: `_matches_glob_filter` — fnmatch include/exclude logic
- Line 6151: `_filter_kit_by_design` — transitive design-scoped extraction

**Filter INPUTS**:
- `filter_spec` dict with optional keys:
  - `design_id`: UUID of design to filter around
  - `representation_tags`: List of tag names/IDs to select best representation
  - `types`, `designs`, `ports`, `files`, `tags`, `concepts`, `qualities`, `authors`, `folders`: glob filter dicts `{"include": [...], "exclude": [...]}`

**Is filtering transitive**: **YES**
- Keep a type ⇒ keep its connectors ⇒ keep ports referenced by connectors
- Keep a type ⇒ keep ancestors in type hierarchy
- Keep a type ⇒ keep one representation ⇒ keep its file
- Keep a design ⇒ keep pieces ⇒ keep piece types ⇒ keep type chain

---

## C. FIND-REPLACEABLE-TYPES

### 9. Find-Replaceable-Types Test Cases

**`compose/fixture/find-replaceable-types.cases.compose.json`** contains:

**Primary Cases (6 cases)**:
- **selection_asset_returns_compatible_ids**: Load selection from JSON, find compatible type IDs (expected 3 design IDs, 0 type IDs)
- **parent_piece_yields_only_exact_design_matches**: Parent pieces (with designs) return only design matches
- **isolated_piece**: Isolated piece (no connections) in nested design yields non-empty types
- **capital_piece**: Look up "Capital" type, forbid Capital/Capsule in results
- **multiple_selected_pieces**: Selection of multiple pieces yields design matches, 0 types
- **empty_selection**: No pieces selected; expects types with no connectors

**Boundary Cases (5 complex scenarios)**:
- Single capsule piece
- Two capsule pieces with specific family names
- Four capsule pieces (cross-floor arrangement)
- Eight capsule pieces
- Tambour piece with forbidden families

**Expected Output Shape**: Each case has:
- Input kit path
- Input selection (piece IDs or design name)
- `expectedTypeIdCount` / `expectedDesignIds`: List of UUIDs or counts
- `forbiddenTypeNames`: Types to exclude from results
- `designFamilies`: (stub; always empty array)

**Synthetic Cases (4 cases, fixture: `synthetic-find-replaceable.kit.compose.json`)**:
- **double_left_selection**: Piece with double-L port → expects candidate-ll type, not candidate-g
- **left_and_gable_selection**: Two pieces with L+G ports → expects candidate-lg type + free design candidate-design-free-lg
- **isolated_selection**: Isolated piece with L+G → expects candidate-lg but not candidate-l
- Tests pass through compatibility check and design-consumption rules

### 10. Find-Replaceable-Types Implementation

**Go Implementation** (`compose/client/lib/go/main.go:12150`):
```go
func FindReplaceableTypesInDesignsForPiecesInDesign(
    design Design,
    designs []Design,
    types []Type,
    ports []Port,
    selectionPieces []string
) (typeIds []string, designIds []string)
```

**Python Implementation** (`compose/client/lib/py/main.py`):
- Line 5888: `find_replaceable_types_for_piece_in_design` — single piece
- Line 5947: `find_replaceable_types_for_pieces_in_design` — multiple pieces

**Symbols**:
- `checkPortCompatibility(candidatePortId, requiredPortId)` (Go line 12171): Checks equality OR bidirectional `CompatiblePorts` membership
- `getBoundaryRequirementPortIds()` (Go line 12235): Ports from external connections
- `canSatisfyRequirements()` (Go line 12275): Bipartite matching of required to available ports using backtracking

### 11. THE COMPATIBILITY RULE — CRITICAL

**Compatibility Predicate** (Go `compose/client/lib/go/main.go:12171–12194`):

Type B's connector port is compatible with Type A's connector port if:

1. Port IDs are **equal** (identical port reference), **OR**
2. Type B's port is in Type A's port's `CompatiblePorts` list, **OR**
3. Type A's port is in Type B's port's `CompatiblePorts` list (bidirectional check)

**Code**:
```go
checkPortCompatibility := func(candidatePortId, requiredPortId string) bool {
    if candidatePortId == "" || requiredPortId == "" {
        return false  // Empty ports never compatible
    }
    if candidatePortId == requiredPortId {
        return true   // Direct match
    }
    candidatePort, okCandidate := portMap[candidatePortId]
    requiredPort, okRequired := portMap[requiredPortId]
    if !okCandidate || !okRequired {
        return false  // Port not found
    }
    // Check candidatePort's compatible list for requiredPortId
    for _, compatiblePort := range candidatePort.CompatiblePorts {
        if compatiblePort.Id == requiredPortId {
            return true
        }
    }
    // Check requiredPort's compatible list for candidatePortId (symmetric)
    for _, compatiblePort := range requiredPort.CompatiblePorts {
        if compatiblePort.Id == candidatePortId {
            return true
        }
    }
    return false
}
```

**Replacement Criteria** (Go line 12275–12323):

Type B can replace pieces of Type A if:

1. **No external connections**: Any non-abstract type is replaceable
2. **With external connections**: Candidate type must have connectors satisfying ALL external port requirements via **bipartite matching** (backtracking algorithm)
   - External ports = connections crossing selection boundary
   - Required ports = ports from neighboring pieces
   - Available ports = candidate type's connector ports
   - Algorithm: Sort requirements by fewest-options-first, then recursively try to assign each requirement to an available port

**Result carries**: **List of type IDs ONLY** — no reason/explanation per candidate

### 12. Replace Operation (State-Changing)

**No state-changing replace operation found**. The codebase is **READ-ONLY**:
- Go: `FindReplaceable...` returns lists; no mutations
- Python: `find_replaceable_types_for_*` returns lists; side-effect free

Operations involving replacements would be separate mutations at higher application layer (not in this library).

### 13. Other Language Implementations

**Python** (`compose/client/lib/py/main.py`):
- Lines 5888–6008: Find-replaceable implementations with same logic as Go
- Lines 6062–6149: Filter-kit with tag selection

**Go** (`compose/client/lib/go/main.go`):
- Lines 12144–12400+: Complete FindReplaceableTypesInDesignsForPiecesInDesign impl

**Tests**:
- `compose/client/lib/go/main_test.go` — contains filter-kit and replaceable-types test cases
- `compose/fixture/` — all fixture files listed above

---

## Summary

| Item | Model/Shape | Determinism | Carries Code | Carries Path | Notes |
|------|---------|-------------|--------------|--------------|-------|
| Problem | Struct w/ constraintId, message, entityKind, entityId, fixes | Sorted by (constraintId, entityId) | ✓ Yes | ✓ Yes (kind+id) | No severity; all equal |
| Filter | Design-scoped + glob spec | Deterministic but depends on tag selection | N/A | N/A | Transitive dependency closure |
| Replaceable | Port compatibility + bipartite matching | Deterministic (backtrack order) | N/A | N/A | Read-only; no state changes |

