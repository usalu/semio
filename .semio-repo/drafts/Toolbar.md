# Global Bottom Toolbar Redesign Specification (Single-Band, Center-Split)

## 1. Scope

This specification defines the canonical global toolbar for all Sketchpad applications.

The build target is a **fixed, bottom-center, single flat horizontal band** with two synchronized sections on the same surface:

- **Tool Zone** (center-origin, grows left)
- **Tool Setting Bar** (center-origin, grows right)

This document is normative and implementation-ready.

## 2. Non-Negotiable Outcomes

1. The toolbar stays fixed at the bottom center exactly like the current system.
2. The toolbar is one flat band, not floating, not detached, not stacked.
3. Tool hierarchy is tree-based but rendered as **upward-only dropdown layers**.
4. No motion is allowed (no animation, transition, easing, fade, or slide).
5. One active path is enforced at all times:
   - one active tool
   - zero or one active sub-tool
6. The Tool Setting Bar always reflects the exact active path and never mixes unrelated settings.

## 3. Layout Contract

### 3.1 Global Placement

- Position: fixed
- Horizontal anchor: viewport center
- Vertical anchor: viewport bottom
- Toolbar is rendered above canvas content and below modal overlays.

### 3.2 Single-Band Surface

- Tool Zone and Tool Setting Bar share:
  - one background level
  - one height
  - one border/surface treatment
- Internal separators are allowed for structure, but they must not create detached panels.

### 3.3 Center-Seam Split

Define a logical seam on the viewport center line.

- **Tool Zone** starts at seam and grows left as tool categories are added.
- **Tool Setting Bar** starts immediately right of seam and grows right.
- Both sections stay on one baseline and remain vertically aligned.

### 3.4 Tool Zone Geometry

- Tool buttons are uniform rectangular controls.
- Buttons are equal width and equal height.
- Buttons are horizontally aligned in a single row.
- Visual order inside Tool Zone reads left-to-right, while the zone itself grows leftward from center.
- Only one tool button is highlighted as active.

### 3.5 Tool Setting Bar Geometry

- Tool Setting Bar starts near center and extends toward the right viewport edge.
- Tool Setting Bar **must not move** from the center seam when active tool changes.
- Only Tool Setting Bar content changes by active context.
- Overflow handling is local to setting content, not by resizing the bar container.
- The toolbar shell is content-sized from the center seam and must not span full viewport width.

## 4. Information Architecture

### 4.1 Tree Model

Toolbar data model:

- `tool` (root category)
- `subTool` (child category/action)

Sub-sub-tool depth is not rendered in this build; any deeper nodes must be represented as sub-tool items in upward flow.

### 4.2 Root Category Inventory

The following roots are required for this build:

1. **Selection** (existing)
2. **Filter** (existing)
3. **Create** (placeholder root)
4. **View** (placeholder root)
5. **Actions** (placeholder root)

### 4.3 Required Branch Content

#### Selection (existing branch, required)

Required sub-tools (app-dependent profile):

- **Kit profile**: Selection Tool, Selection Mode
- **Design/Type/Home/Feedback profile**: Selection Mode, Additive, Subtractive, Intersect

#### Filter (existing branch, required)

Required sub-tools (app-dependent profile):

- **Home profile**: Filter Type, Filter Name, Filter Version, Reset Filters
- **Other profiles**: Filter Design, Filter Type, Filter Status, Reset Filters

#### Placeholder Branches (required)

Each placeholder root must include placeholder sub-tools:

- **Create**
  - Add Type (placeholder)
  - Add Design (placeholder)
  - Add Variant (placeholder)
- **View**
  - View Layers (placeholder)
  - View Density (placeholder)
  - View Connections (placeholder)
- **Actions**
  - Action Placeholder 1
  - Action Placeholder 2
  - Action Placeholder 3

Placeholder labels may change later, but branch presence and structure are required now.

## 5. Dropdown Behavior

### 5.1 Directionality

- Dropdowns open upward only.
- Opening downward is forbidden.
- Horizontal branch expansion is forbidden in this build.

### 5.2 Anchoring

- Every dropdown is visually anchored to its source tool button.
- Dropdown origin stays attached to the triggering button while open.

### 5.3 Visibility Rules

- Open/close is instantaneous.
- No animation or transition effects are allowed.
- Activating a new tool collapses all unrelated open dropdowns immediately.

## 6. State and Synchronization

### 6.1 Single Source of Truth

Global toolbar state per app:

- `activeToolId` (required)
- `activeSubToolId` (optional)

No parallel active branches are allowed.

### 6.2 Atomic State Updates

Selecting tool/sub-tool updates state atomically in one commit.

### 6.3 Path Cleanup

When active tool changes:

- Previous sub-tool selection is cleared if not valid under new tool.
- Old dropdown state is collapsed.
- Tool Setting Bar context is replaced immediately.

### 6.4 Context Resolution Precedence

Tool Setting Bar content resolution order:

1. active sub-tool settings
2. active tool settings

No global fallback UI may appear while a valid active path exists.

## 7. Tool Setting Bar Rules

1. Show only settings for the active path.
2. Never show settings from unrelated tools.
3. Never mix settings from multiple categories.
4. Switching active path replaces settings content immediately.
5. If a placeholder sub-tool is active, show placeholder settings scoped to that sub-tool only.
6. Setting content renders as direct named toggles/buttons only; static heading rows are not shown inside the settings bar.

## 8. Interaction Contract

### 8.1 Pointer

- Click tool button: set active tool, open upward dropdown for that tool.
- Click sub-tool: set active sub-tool, update settings content immediately.
- Click outside toolbar/dropdown: close dropdown UI only; keep current active path.

### 8.2 Keyboard

- Tab order: tool buttons, then Tool Setting Bar controls.
- Arrow navigation inside dropdown: vertical only.
- Enter/Space: activate focused tool/sub-tool.
- Escape: close open dropdown and return focus to source tool button.

### 8.3 Visual Hierarchy Without Motion

Hierarchy must be communicated only by:

- spacing
- alignment
- grouping
- contrast
- separators

Motion is not permitted for hierarchy signaling.

## 9. Cross-App Application

- All apps use the same toolbar shell and state model.
- Apps may provide different concrete settings content per tool/sub-tool.
- Empty apps must still preserve shell integrity without collapsing band geometry.

## 10. Accessibility Contract

- Toolbar root exposes toolbar semantics.
- Tool toggles expose expanded and selected state.
- Dropdown items expose selected state.
- Tool Setting Bar has stable label and region semantics.
- Keyboard-only operation supports full tool/sub-tool selection workflow.

## 11. Compliance Checklist

Implementation is conformant only when all checks pass:

1. Bottom-center fixed placement is preserved.
2. Toolbar renders as one flat single band.
3. Tool Zone grows left from center seam.
4. Tool Setting Bar grows right from center seam and does not resize/move on selection changes.
5. Dropdowns open upward only.
6. Horizontal branch expansion does not exist.
7. Only one active tool is possible.
8. Only one optional active sub-tool is possible.
9. Tool Setting Bar reflects exact active path only.
10. Selection + Filter existing branches are present.
11. Create + View + Actions placeholder branches are present with placeholder sub-tools.
