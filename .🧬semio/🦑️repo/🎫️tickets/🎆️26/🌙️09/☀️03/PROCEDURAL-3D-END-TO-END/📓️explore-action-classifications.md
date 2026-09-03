# Action Classification Audit: Procedural Generation3d vs. Reference App (Lowpoly)

**Audit Date:** 2026-09-03  
**Scope:** `generation3d` app in `procedural` plugin, with comparison to reference app  
**Framework Gate:** `validate_ui_dispatch_classification` (framework/products/os/modules/plugin/🦀️.rs ~line 11915)

---

## Executive Summary

| App | Migrated | BatchOnlyPendingRewrite | ForbiddenFromUi | Total |
|---|---:|---:|---:|---:|
| **generation3d** | 23 | 6 | 0 | 29 |
| **lowpoly** (reference) | 47 | 0 | 0 | 47 |
| **generation2d** (colocated) | 7 | 13 | 1 | 21 |

**Key Blockers for generation3d UI Dispatch:**
1. **nodeGraphEdit** (BatchOnlyPendingRewrite) - primary flow editing action
2. **Generation lifecycle actions** (6 total: addGeneration, removeGeneration, renameGeneration, updateGenerationValues, selectGeneration, flowEvalTick)

---

## Part 1: Generation3d Complete Action Audit

**Plugin Path:** `./✏️s/🔌️plugins/🌀️procedural`  
**App Artifact:** `🗿️artifacts/🧊️generation3d`  
**Editor Source:** `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (lines 599–627)

### Full Action Table

| # | Action Name | Line | Classification | Category |
|---|---|---|---|---|
| 1 | setActiveExample | 599 | **Migrated** | Example/Fixture Loading |
| 2 | nodeGraphEdit | 600 | **BatchOnlyPendingRewrite** | Flow Editing |
| 3 | deleteSelection | 601 | Migrated | Graph Edit |
| 4 | removeWidget | 602 | Migrated | Graph Edit |
| 5 | moveMediaNode | 603 | Migrated | Graph Edit |
| 6 | addWidget | 604 | Migrated | Graph Edit |
| 7 | patchFlowWidgets | 605 | Migrated | Graph Edit |
| 8 | reorganize | 606 | Migrated | Graph Layout |
| 9 | translateSelection | 607 | Migrated | Transform |
| 10 | rotateSelection | 608 | Migrated | Transform |
| 11 | scaleSelection | 609 | Migrated | Transform |
| 12 | addGeneration | 610 | **BatchOnlyPendingRewrite** | Generation Lifecycle |
| 13 | removeGeneration | 611 | **BatchOnlyPendingRewrite** | Generation Lifecycle |
| 14 | renameGeneration | 612 | **BatchOnlyPendingRewrite** | Generation Lifecycle |
| 15 | updateGenerationValues | 613 | **BatchOnlyPendingRewrite** | Generation Lifecycle |
| 16 | nodeGraphViewport | 614 | **Migrated** | Viewport/Camera |
| 17 | worldPointerDown | 615 | **Migrated** | Viewport/Camera |
| 18 | graphPointerDown | 616 | **Migrated** | Viewport/Camera |
| 19 | setLodMode | 617 | Migrated | Rendering |
| 20 | setShowMode | 618 | Migrated | Rendering |
| 21 | toggleSun | 619 | Migrated | Lighting |
| 22 | setSunAzimuth | 620 | Migrated | Lighting |
| 23 | setSunElevation | 621 | Migrated | Lighting |
| 24 | setSunIntensity | 622 | Migrated | Lighting |
| 25 | setCamera | 623 | **Migrated** | Viewport/Camera |
| 26 | selectGeneration | 624 | **BatchOnlyPendingRewrite** | Generation Lifecycle |
| 27 | setActiveUtility | 625 | Migrated | UI State |
| 28 | setLocale | 626 | Migrated | Localization |
| 29 | flowEvalTick | 627 | Migrated | Engine Tick |

### Classification Totals

```
✓ Migrated:                    23 (79.3%)
⚠ BatchOnlyPendingRewrite:     6  (20.7%)
✗ ForbiddenFromUi:             0  (0%)
？ Unclassified:               0  (0%)
────────────────────────────────────
  TOTAL:                        29
```

### Verification (Second Method)

Second-pass grep using pattern count:

```bash
$ grep "action_interactive_job" ./✏️editor/🦀️.rs | wc -l
29

$ grep "action_interactive_job" ./✏️editor/🦀️.rs | grep -c "Migrated"
23

$ grep "action_interactive_job" ./✏️editor/🦀️.rs | grep -c "BatchOnlyPendingRewrite"
6

$ grep "action_interactive_job" ./✏️editor/🦀️.rs | grep -c "ForbiddenFromUi"
0
```

**Result:** ✓ Verified (23 Migrated, 6 BatchOnlyPendingRewrite, 0 ForbiddenFromUi)

---

## Part 2: Critical Action Classifications for generation3d

### Example/Fixture Loading

| Action | Status | Notes |
|---|---|---|
| **setActiveExample** (line 599) | ✓ **Migrated** | Switching active example is ready for UI dispatch |

### Node Graph / Flow Editing

| Action | Status | Notes |
|---|---|---|
| **nodeGraphEdit** (line 600) | ⚠️ **BatchOnlyPendingRewrite** | **PRIMARY BLOCKER** — flow graph parameter/node changes cannot be dispatched from UI; must use batch interface |
| addWidget (line 604) | ✓ Migrated | Adding new nodes to graph is migrated |
| moveMediaNode (line 603) | ✓ Migrated | Moving nodes within graph is migrated |
| deleteSelection (line 601) | ✓ Migrated | Deleting selected nodes is migrated |
| patchFlowWidgets (line 605) | ✓ Migrated | Widget patching is migrated |
| reorganize (line 606) | ✓ Migrated | Graph layout reorganization is migrated |

**Critical Gap:** The core `nodeGraphEdit` action that handles structured flow changes remains non-interactive. Individual node operations (add, move, delete) are migrated, but the unified editing action is not.

### Camera/Viewport Interaction (3D Preview)

| Action | Status | Notes |
|---|---|---|
| **nodeGraphViewport** (line 614) | ✓ **Migrated** | Flow graph viewport navigation (pan/zoom) is fully migrated |
| **worldPointerDown** (line 615) | ✓ **Migrated** | 3D world picking and interaction is ready for UI |
| **setCamera** (line 623) | ✓ **Migrated** | Direct camera state setting is migrated |
| graphPointerDown (line 616) | ✓ Migrated | Flow graph pointer picking is migrated |

**Status:** ✓ All viewport/camera operations fully migrated.

---

## Part 3: Generation2d (Colocated Reference)

**Source:** `./✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (lines 738–758)

### Classification Summary

```
✓ Migrated:                     7  (33.3%)
⚠ BatchOnlyPendingRewrite:    13  (61.9%)
✗ ForbiddenFromUi:             1  (4.8%)
────────────────────────────────────
  TOTAL:                       21
```

### Actions (All 21)

| Action | Classification | Line |
|---|---|---|
| nodeGraphEdit | BatchOnlyPendingRewrite | 738 |
| moveMediaNode | BatchOnlyPendingRewrite | 739 |
| addWidget | BatchOnlyPendingRewrite | 740 |
| removeWidget | BatchOnlyPendingRewrite | 741 |
| connectMediaPorts | BatchOnlyPendingRewrite | 742 |
| reorganize | BatchOnlyPendingRewrite | 743 |
| addGeneration | BatchOnlyPendingRewrite | 744 |
| removeGeneration | BatchOnlyPendingRewrite | 745 |
| renameGeneration | BatchOnlyPendingRewrite | 746 |
| updateGenerationValues | BatchOnlyPendingRewrite | 747 |
| nodeGraphViewport | Migrated | 748 |
| setShowMode | Migrated | 749 |
| generate | Migrated | 750 |
| setEvalOutputs | BatchOnlyPendingRewrite | 751 |
| canvasPointerDown | Migrated | 752 |
| canvasPointerMove | Migrated | 753 |
| canvasPointerUp | Migrated | 754 |
| canvasWheel | Migrated | 755 |
| selectGeneration | BatchOnlyPendingRewrite | 756 |
| flowEvalTick | BatchOnlyPendingRewrite | 757 |
| setLocale | ForbiddenFromUi | 758 |

**Observation:** generation2d has significantly lower migration rate (33.3%). Most graph editing actions remain in BatchOnlyPendingRewrite, and setLocale is explicitly forbidden from UI.

---

## Part 4: Lowpoly Reference App (Success Baseline)

**Plugin Path:** `./✏️s/🔌️plugins/💠️lowpoly`  
**Artifact:** `🗿️artifacts/💠️lowpoly`  
**Editor Source:** `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (lines 2012–2058)

### Classification Summary

```
✓ Migrated:                    47  (100%)
⚠ BatchOnlyPendingRewrite:     0  (0%)
✗ ForbiddenFromUi:             0  (0%)
────────────────────────────────────
  TOTAL:                       47
```

### All Actions Migrated (Sample)

- addPrimitive, patchObject, extrude, inset, bevel, loopCut, subdivide, triangulate, mirror, decimate, flipFaces, merge, dissolve, snap, toggleSmooth, unwrapActive, markUvSeam, clearSeam, translateSelection, rotateSelection, scaleSelection, addPaintLayer, paintStrokeEnd, paintFill, fillBucket, transformEnd, importSnapshotJson, setFixtureJson, engagementSubmit, setActiveObject, setActivePaintLayer, setUtilityParam, engagementInput, toggleShowEdges, toggleSun, setSunAzimuth, setSunElevation, setSunIntensity, setCamera, paintStrokeBegin, paintSample, paintStroke, paintAt, canvasPointerDown, canvasPointerMove, transformBegin, setActiveUtility

### Verification

```bash
$ grep "action_interactive_job" ./✏️editor/🦀️.rs | wc -l
47

$ grep "action_interactive_job" ./✏️editor/🦀️.rs | grep -c "Migrated"
47

$ grep "action_interactive_job" ./✏️editor/🦀️.rs | grep -v "Migrated" | wc -l
0
```

**Result:** ✓ Verified (47/47 Migrated, 100% readiness)

---

## Part 5: Comparison & Gap Analysis

### Migration Readiness

```
lowpoly (reference):    47/47  = 100%   ✓ Ready for full UI dispatch
generation3d:           23/29  = 79.3%  ⚠️  Blocked by 6 actions
generation2d:            7/21  = 33.3%  ✗ Heavily restricted
```

### generation3d Blockers vs. Lowpoly

| Issue | generation3d | lowpoly |
|---|---|---|
| Primary editing action | nodeGraphEdit (BatchOnly) | All editing actions (Migrated) |
| Generation lifecycle | 6 actions in BatchOnly | N/A (different domain) |
| Viewport/Camera | ✓ All Migrated | ✓ All Migrated |
| Forbidden actions | 0 | 0 |

---

## Conclusions & Recommendations

### Current Status

1. **generation3d is 79% ready** for UI dispatch, with strong viewport/camera support.
2. **Two class of blockers** prevent 100% readiness:
   - **nodeGraphEdit** (1 action): Unified flow graph editing interface
   - **Generation lifecycle** (5 actions): add/remove/rename/select generation, updateGenerationValues

### Required for Full UI Dispatch

To match lowpoly's 100% readiness, generation3d must migrate:

1. `nodeGraphEdit` from `BatchOnlyPendingRewrite` to `Migrated`
2. All 5 generation-lifecycle actions to `Migrated`
3. Ensure no actions are classified as `ForbiddenFromUi` (currently 0 ✓)

### Migration Priority

**High:** nodeGraphEdit (line 600) — used in core flow-editing workflows  
**Medium:** Generation lifecycle actions — used in generate panel  
**Low:** None; all critical viewport/camera actions are ready

---

**Audit Complete**
EOF
cat "./.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️03/PROCEDURAL-3D-END-TO-END/📓️explore-action-classifications.md"
