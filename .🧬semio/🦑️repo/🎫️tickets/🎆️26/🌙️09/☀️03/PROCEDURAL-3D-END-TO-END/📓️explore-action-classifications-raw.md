# Generation3d Action Classification Audit

**Plugin:** procedural  
**App:** generation3d  
**Source File:** `./✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

## Complete Action Table

| Action Name | File:Line | Classification |
|---|---|---|
| setActiveExample | 🦀️.rs:599 | Migrated |
| nodeGraphEdit | 🦀️.rs:600 | **BatchOnlyPendingRewrite** |
| deleteSelection | 🦀️.rs:601 | Migrated |
| removeWidget | 🦀️.rs:602 | Migrated |
| moveMediaNode | 🦀️.rs:603 | Migrated |
| addWidget | 🦀️.rs:604 | Migrated |
| patchFlowWidgets | 🦀️.rs:605 | Migrated |
| reorganize | 🦀️.rs:606 | Migrated |
| translateSelection | 🦀️.rs:607 | Migrated |
| rotateSelection | 🦀️.rs:608 | Migrated |
| scaleSelection | 🦀️.rs:609 | Migrated |
| addGeneration | 🦀️.rs:610 | **BatchOnlyPendingRewrite** |
| removeGeneration | 🦀️.rs:611 | **BatchOnlyPendingRewrite** |
| renameGeneration | 🦀️.rs:612 | **BatchOnlyPendingRewrite** |
| updateGenerationValues | 🦀️.rs:613 | **BatchOnlyPendingRewrite** |
| nodeGraphViewport | 🦀️.rs:614 | Migrated |
| worldPointerDown | 🦀️.rs:615 | Migrated |
| graphPointerDown | 🦀️.rs:616 | Migrated |
| setLodMode | 🦀️.rs:617 | Migrated |
| setShowMode | 🦀️.rs:618 | Migrated |
| toggleSun | 🦀️.rs:619 | Migrated |
| setSunAzimuth | 🦀️.rs:620 | Migrated |
| setSunElevation | 🦀️.rs:621 | Migrated |
| setSunIntensity | 🦀️.rs:622 | Migrated |
| setCamera | 🦀️.rs:623 | Migrated |
| selectGeneration | 🦀️.rs:624 | **BatchOnlyPendingRewrite** |
| setActiveUtility | 🦀️.rs:625 | Migrated |
| setLocale | 🦀️.rs:626 | Migrated |
| flowEvalTick | 🦀️.rs:627 | Migrated |

## Classification Counts

- **Migrated:** 23
- **BatchOnlyPendingRewrite:** 6
- **Unclassified:** 0
- **ForbiddenFromUi:** 0
- **Total:** 29

## Verification (Second Method)

Second-pass grep count to verify:
```bash
grep "action_interactive_job" ./✏️.rs | wc -l        # 29 total
grep "action_interactive_job" ./✏️.rs | grep "Migrated" | wc -l  # 23 Migrated
grep "action_interactive_job" ./✏️.rs | grep "BatchOnlyPendingRewrite" | wc -l  # 6 BatchOnlyPendingRewrite
```

## Key Findings: Critical Action Classifications

### Example/Fixture Loading
- **setActiveExample** (line 599): **Migrated** ✓

### Node Graph / Flow Editing
- **nodeGraphEdit** (line 600): **BatchOnlyPendingRewrite** ⚠️  
  *Flow graph editing is NOT fully migrated to UI dispatch*
- **addWidget** (line 604): Migrated ✓  
  *Adding nodes to graph is migrated*
- **moveMediaNode** (line 603): Migrated ✓  
  *Moving nodes is migrated*
- **deleteSelection** (line 601): Migrated ✓
- **patchFlowWidgets** (line 605): Migrated ✓

### Camera/Viewport Interaction (3D Preview)
- **nodeGraphViewport** (line 614): Migrated ✓  
  *Node graph viewport camera is migrated*
- **worldPointerDown** (line 615): Migrated ✓  
  *3D world pointer interaction is migrated*
- **setCamera** (line 623): Migrated ✓  
  *Direct camera setting is migrated*
- **graphPointerDown** (line 616): Migrated ✓  
  *Graph pointer interaction is migrated*

---

## Notes

1. The only blocker for generation3d UI dispatch is the **nodeGraphEdit** action (line 600), which remains in **BatchOnlyPendingRewrite** state.
2. All other critical operations (examples loading, node addition/movement, camera control) are properly classified as **Migrated**.
3. Generation-related actions (addGeneration, removeGeneration, etc.) are categorized separately and remain in **BatchOnlyPendingRewrite**.
