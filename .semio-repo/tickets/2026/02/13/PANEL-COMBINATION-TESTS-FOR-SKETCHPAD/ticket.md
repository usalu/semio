---
goal: test/js
---

# Ticket: Panel Combination Tests for Sketchpad

## Summary

Added comprehensive panel combination tests to `semio/js/sketchpad.test.ts` as a new `test("Panels")` within the existing `test.describe("sketchpad")` block. Tests cover all-open/all-closed/pairwise panel combinations across Home, Kit, Design, and Type apps; cross-app panel persistence; toolbar group+panel coexistence; resize handles; and keyboard shortcuts. All tests pass with no regressions.

## Changes

- `semio/js/sketchpad.test.ts`: Added `test("Panels")` (~350 lines) covering 8 panel combination scenarios across 4 apps + cross-app navigation + resize + keyboard shortcuts

## Log

- Researched all 6 app files + Sketchpad.tsx + shared.ts
- Mapped all `getPanels`, `addSection`, toolbar groups, and panel section IDs

## Todos

- [x] Research Home.tsx panels
- [x] Research Kit.tsx panels
- [x] Research Design.tsx panels
- [x] Research Type.tsx panels
- [x] Research Docs.tsx panels
- [x] Research Feedback.tsx panels
- [x] Research Sketchpad.tsx panel composition
- [x] Research shared.ts panel types

## Plan

Research only.

## Research Results

### Infrastructure (shared.ts + Sketchpad.tsx)

#### PanelKind enum
- `WORKBENCH`, `TOOLS`, `TOOLBAR`, `HUD`, `STATS`, `DETAILS`, `CHAT`, `SETTINGS`, `PARAMS`, `CONSOLE`

#### PanelKey type
- `"details"`, `"workbench"`, `"tools"`, `"hud"`, `"stats"`, `"console"`, `"chat"`, `"settings"`, `"toolbar"`, `"leftSidePanel"`, `"rightSidePanel"`, `"hudPanel"`

#### PanelPosition enum
- `LEFT`, `RIGHT`, `MIDDLE`, `BOTTOM`

#### Panel Position Mapping (panelKindConfigs)
| PanelKind | Position | Group | Hotkey |
|-----------|----------|-------|--------|
| WORKBENCH | LEFT | workbench | ctrl+j |
| TOOLS | LEFT | workbench | ctrl+j |
| TOOLBAR | BOTTOM | — | — |
| HUD | MIDDLE | hud | ctrl+k |
| STATS | MIDDLE | hud | ctrl+k |
| DETAILS | RIGHT | right | ctrl+l |
| CHAT | RIGHT | right | ctrl+l |
| SETTINGS | RIGHT | right | ctrl+l |
| PARAMS | RIGHT | right | ctrl+l |
| CONSOLE | BOTTOM | — | ctrl+k |

#### PanelSections keys
- `details`, `workbench`, `tools`, `hud`, `stats`, `console`, `chat`, `settings`, `toolbar`, `leftSidePanel`, `rightSidePanel`, `hudPanel`

#### PanelVisibility keys
- `toolbar?`, `leftSidePanel?`, `rightSidePanel?`, `hudPanel?`, `workbench?`, `tools?`, `hud?`, `stats?`, `details?`, `chat?`, `settings?`, `params?`, `console?`

#### Default PanelVisibility
- All false: `{ toolbar: false, workbench: false, details: false, chat: false, settings: false }`

#### Panel Composition (Sketchpad.tsx ~L15355)
- Panels from `getPanels()` are mapped by `PanelPosition`:
  - `LEFT` → registered as left side panel tab via `addSidePanelTab("left", tab)`
  - `RIGHT` → registered as right side panel tab via `addSidePanelTab("right", tab)`
  - `MIDDLE` → registered as hud panel tab via `addHudPanelTab(tab)`
  - `BOTTOM` → handled separately (toolbar)

---

### Home App (`Home.tsx`)

#### Config
- **id**: `"home"`
- **order**: 0

#### getPanels
| PanelKind | Panel Toggle ID |
|-----------|----------------|
| TOOLBAR | `semio.sketchpad.navbar.panelToggle.toolbar.show` |
| DETAILS | `semio.sketchpad.navbar.panelToggle.details.show` |
| CHAT | `semio.sketchpad.navbar.panelToggle.chat.show` |
| SETTINGS | `semio.sketchpad.navbar.panelToggle.settings.show` |

#### Panel Sections (addSection calls)

**`"details"` panel:**
| Section ID | Specificity | Order | Condition |
|-----------|-------------|-------|-----------|
| `semio.sketchpad.app.kit.properties` | 0 | 0 | Single kit selected |
| `semio.sketchpad.app.home.kits.multiple` | 0 | 0 | Multiple kits selected |

**`"chat"` panel:**
| Section ID | Specificity | Order |
|-----------|-------------|-------|
| `semio.sketchpad.app.home.chat` | 0 | 0 |

**`"settings"` panel:**
| Section ID | Specificity | Order |
|-----------|-------------|-------|
| `semio.sketchpad.app.home.settings` | 20 | 0 |
| `semio.sketchpad.settings` | 0 | 0 |

**`"toolbar"` panel:**
| Section ID | Specificity | Order | Toolbar Group |
|-----------|-------------|-------|---------------|
| `semio.sketchpad.app.home.toolbar.filters` | 20 | 0 | `{ id: "filter", labelId: "semio.sketchpad.toolbar.parent.filter", order: 20 }` |
| `semio.sketchpad.app.home.toolbar.create` | 20 | 0 | `{ id: "create", labelId: "semio.sketchpad.toolbar.parent.create", order: 30 }` |

---

### Kit App (`Kit.tsx`)

#### Config
- **id**: `"kit"`
- **order**: 10

#### getPanels
| PanelKind | Panel Toggle ID |
|-----------|----------------|
| TOOLBAR | `semio.sketchpad.navbar.panelToggle.toolbar.show` |
| DETAILS | `semio.sketchpad.navbar.panelToggle.details.show` |
| CHAT | `semio.sketchpad.navbar.panelToggle.chat.show` |
| SETTINGS | `semio.sketchpad.navbar.panelToggle.settings.show` |

#### Panel Sections (addSection calls)

**`"details"` panel:**
| Section ID | Specificity | Order | Condition |
|-----------|-------------|-------|-----------|
| `semio.sketchpad.app.kit.artifacts.multiple` | 30 | 0 | totalSelectedKinds > 1 |
| `semio.sketchpad.app.design.properties` | 30 | 10 | single design selected |
| `semio.sketchpad.app.kit.designs.multipleTitle` | 30 | 10 | multiple designs selected |
| `semio.sketchpad.app.type.properties` | 30 | 20 | single type selected |
| `semio.sketchpad.app.kit.types.multipleTitle` | 30 | 20 | multiple types selected |
| `semio.sketchpad.app.kit.port.properties` | 30 | 25 | single port selected |
| `semio.sketchpad.app.kit.ports.multipleTitle` | 30 | 25 | multiple ports selected |
| `semio.sketchpad.app.kit.tag.properties` | 30 | 26 | single tag selected |
| `semio.sketchpad.app.kit.tags.multipleTitle` | 30 | 26 | multiple tags selected |
| `semio.sketchpad.app.kit.concept.properties` | 30 | 27 | single concept selected |
| `semio.sketchpad.app.kit.concepts.multipleTitle` | 30 | 27 | multiple concepts selected |
| `semio.sketchpad.app.kit.file.properties` | 30 | 30 | single file selected |
| `semio.sketchpad.app.kit.files.multipleTitle` | 30 | 30 | multiple files selected |
| `semio.sketchpad.app.kit.folder.properties` | 30 | 40 | single folder selected |
| `semio.sketchpad.app.kit.folders.multipleTitle` | 30 | 40 | multiple folders selected |
| `semio.sketchpad.app.kit.properties` | 10 | 100 | always |

**`"settings"` panel:**
| Section ID | Specificity | Order |
|-----------|-------------|-------|
| `semio.sketchpad.app.kit.settings` | 10 | 0 |
| `semio.sketchpad.settings` | 0 | 0 |

**`"toolbar"` panel:**
| Section ID | Specificity | Order | Toolbar Group |
|-----------|-------------|-------|---------------|
| `semio.sketchpad.app.kit.toolbar.selection` | 20 | 10 | `{ id: "selection", labelId: "semio.sketchpad.toolbar.parent.selection", order: 10, subToolId: "select", subToolLabelId: "semio.sketchpad.toolbar.subtool.select" }` |
| `semio.sketchpad.app.kit.toolbar.filters` | 20 | 20 | `{ id: "filter", labelId: "semio.sketchpad.toolbar.parent.filter", order: 20 }` |
| `semio.sketchpad.app.kit.toolbar.create` | 20 | 30 | `{ id: "create", labelId: "semio.sketchpad.toolbar.parent.create", order: 30 }` |

---

### Design App (`Design.tsx`)

#### Config
- **id**: `"design"`
- **order**: 20

#### getPanels
| PanelKind | Panel Toggle ID |
|-----------|----------------|
| WORKBENCH | `semio.sketchpad.navbar.panelToggle.workbench.show` |
| TOOLS | `semio.sketchpad.navbar.panelToggle.tools.show` |
| TOOLBAR | `semio.sketchpad.navbar.panelToggle.toolbar.show` |
| HUD | `semio.sketchpad.navbar.panelToggle.hud.show` |
| STATS | `semio.sketchpad.navbar.panelToggle.stats.show` |
| DETAILS | `semio.sketchpad.navbar.panelToggle.details.show` |
| CHAT | `semio.sketchpad.navbar.panelToggle.chat.show` |
| SETTINGS | `semio.sketchpad.navbar.panelToggle.settings.show` |

#### Panel Sections (addSection calls)

**`"details"` panel:**
| Section ID | Specificity | Order | Condition |
|-----------|-------------|-------|-----------|
| `semio.sketchpad.app.design.properties` | 20 | 50 | no selection / always as fallback |
| `semio.sketchpad.app.type.connector.properties` | 30 | 0 | port/connector selected |
| `semio.sketchpad.app.design.panel.details.section.piece.properties` | 30 | 0 | single piece selected |
| `semio.sketchpad.app.design.panel.details.section.piece.multipleTitle` | 30 | 0 | multiple pieces selected |
| `semio.sketchpad.app.design.panel.details.section.connection.properties` | 30 | 10 | single connection selected |
| `semio.sketchpad.app.design.panel.details.section.connection.multipleTitle` | 30 | 10 | multiple connections selected |
| `semio.sketchpad.app.design.panel.details.section.selection.multipleTitle` | 30 | 20 | pieces AND connections selected |
| `semio.sketchpad.app.kit.properties` | 10 | 100 | always |

**`"workbench"` panel:**
| Section ID | Specificity | Order |
|-----------|-------------|-------|
| `semio.sketchpad.app.kit.pieces` | 20 | 0 |
| `semio.sketchpad.app.design.windows` | 20 | 1 |

**`"hud"` panel:**
| Section ID | Specificity | Order |
|-----------|-------------|-------|
| `semio.sketchpad.app.design.hud.pieces` | 20 | 0 |

**`"settings"` panel:**
| Section ID | Specificity | Order |
|-----------|-------------|-------|
| `semio.sketchpad.app.design.settings` | 30 | 0 |
| `semio.sketchpad.app.kit.settings` | 10 | 0 |
| `semio.sketchpad.settings` | 0 | 0 |

**`"toolbar"` panel (from DesignApp component):**
| Section ID | Specificity | Order | Toolbar Group |
|-----------|-------------|-------|---------------|
| `semio.sketchpad.app.design.tools.select` | 20 | 0 | `{ id: "selection", labelId: "semio.sketchpad.toolbar.parent.selection", order: 10 }` |

---

### Type App (`Type.tsx`)

#### Config
- **id**: `"type"` (inferred from pattern)
- **order**: 30

#### getPanels
| PanelKind | Panel Toggle ID |
|-----------|----------------|
| TOOLBAR | `semio.sketchpad.navbar.panelToggle.toolbar.show` |
| HUD | `semio.sketchpad.navbar.panelToggle.hud.show` |
| STATS | `semio.sketchpad.navbar.panelToggle.stats.show` |
| DETAILS | `semio.sketchpad.navbar.panelToggle.details.show` |
| CHAT | `semio.sketchpad.navbar.panelToggle.chat.show` |
| SETTINGS | `semio.sketchpad.navbar.panelToggle.settings.show` |

#### Panel Sections (addSection calls)

**`"details"` panel:**
| Section ID | Specificity | Order | Condition |
|-----------|-------------|-------|-----------|
| `semio.sketchpad.app.type.connector.properties` | 30 | 0 | single connector selected |
| `semio.sketchpad.app.type.panel.details.section.connectors.multipleTitle` | 30 | 0 | multiple connectors selected |
| `semio.sketchpad.app.type.properties` | 20 | 50 | always |
| `semio.sketchpad.app.kit.properties` | 10 | 100 | always |

**`"settings"` panel:**
| Section ID | Specificity | Order |
|-----------|-------------|-------|
| `semio.sketchpad.app.type.settings` | 30 | 0 |
| `semio.sketchpad.app.kit.settings` | 10 | 0 |
| `semio.sketchpad.settings` | 0 | 0 |

**`"toolbar"` panel:**
| Section ID | Specificity | Order | Toolbar Group |
|-----------|-------------|-------|---------------|
| `semio.sketchpad.app.type.tools.selection` | 20 | 0 | `{ id: "selection", labelId: "semio.sketchpad.toolbar.parent.selection", order: 10, subToolId: ToolKind.SELECTION_NORMAL, subToolLabelId: "semio.sketchpad.toolbar.subtool.select" }` |
| `semio.sketchpad.app.type.tools.connector` | 20 | 10 | `{ id: "create", labelId: "semio.sketchpad.toolbar.parent.create", order: 10, subToolId: ToolKind.CONNECTOR, subToolLabelId: "semio.sketchpad.toolbar.subtool.connector" }` |

#### Initial PanelVisibility (from useTypeAppInitialize)
- `{ toolbar: true, workbench: false, details: false, chat: false, settings: false }`

---

### Docs App (`Docs.tsx`)

#### Config
- **id**: `"docs"`
- **order**: 5

#### getPanels
| PanelKind | Panel Toggle ID | Hotkey | Tooltip |
|-----------|----------------|--------|---------|
| WORKBENCH | `semio.sketchpad.navbar.panelToggle.workbench.show` | from getHotkeyFn | `{ labelKey: "...", manualPath: "/docs/manuals/sketchpad#workbench" }` |
| DETAILS | `semio.sketchpad.navbar.panelToggle.details.show` | from getHotkeyFn | `{ labelKey: "...", manualPath: "/docs/manuals/sketchpad#details" }` |
| SETTINGS | `semio.sketchpad.navbar.panelToggle.settings.show` | from getHotkeyFn | `{ labelKey: "...", manualPath: "/docs/manuals/sketchpad#settings" }` |

#### Panel Sections (addSection calls)

**`"workbench"` panel:**
| Section ID | Specificity | Order |
|-----------|-------------|-------|
| `semio.sketchpad.app.docs.docs` | 20 | 1 |
| `semio.sketchpad.app.docs.overview` | 20 | 2 |

**`"details"` panel:**
| Section ID | Specificity | Order |
|-----------|-------------|-------|
| `semio.sketchpad.app.docs.page` | 20 | 1 |

**`"settings"` panel:**
| Section ID | Specificity | Order |
|-----------|-------------|-------|
| `semio.sketchpad.app.docs.settings` | 20 | 1 |

**`"toolbar"` panel:**
| Section ID | Specificity | Order | Notes |
|-----------|-------------|-------|-------|
| `semio.sketchpad.app.docs.toolbar.empty` | 20 | 0 | `toolbarPlaceholder: true` |

---

### Feedback App (`Feedback.tsx`)

#### Config
- **id**: `"feedback"`
- **order**: 10

#### getPanels
| PanelKind | Panel Toggle ID |
|-----------|----------------|
| TOOLBAR | `semio.sketchpad.navbar.panelToggle.toolbar.show` |

#### Panel Sections (addSection calls)

**`"toolbar"` panel:**
| Section ID | Specificity | Order | Toolbar Group |
|-----------|-------------|-------|---------------|
| `semio.sketchpad.app.feedback.toolbar.send` | 20 | 0 | `{ id: "actions", labelId: "semio.sketchpad.toolbar.parent.actions", order: 50 }` |

---

### Summary of All Toolbar Groups Across Apps

| App | Group ID | Group labelId | Group Order |
|-----|----------|---------------|-------------|
| Home | `filter` | `semio.sketchpad.toolbar.parent.filter` | 20 |
| Home | `create` | `semio.sketchpad.toolbar.parent.create` | 30 |
| Kit | `selection` | `semio.sketchpad.toolbar.parent.selection` | 10 |
| Kit | `filter` | `semio.sketchpad.toolbar.parent.filter` | 20 |
| Kit | `create` | `semio.sketchpad.toolbar.parent.create` | 30 |
| Design | `selection` | `semio.sketchpad.toolbar.parent.selection` | 10 |
| Type | `selection` | `semio.sketchpad.toolbar.parent.selection` | 10 |
| Type | `create` | `semio.sketchpad.toolbar.parent.create` | 10 |
| Feedback | `actions` | `semio.sketchpad.toolbar.parent.actions` | 50 |
| Docs | — (placeholder) | — | — |

### Summary of All Panel Toggle IDs (navbar buttons)

| ID | Used By Apps |
|----|-------------|
| `semio.sketchpad.navbar.panelToggle.toolbar.show` | Home, Kit, Design, Type, Feedback |
| `semio.sketchpad.navbar.panelToggle.details.show` | Home, Kit, Design, Type, Docs |
| `semio.sketchpad.navbar.panelToggle.chat.show` | Home, Kit, Design, Type |
| `semio.sketchpad.navbar.panelToggle.settings.show` | Home, Kit, Design, Type, Docs |
| `semio.sketchpad.navbar.panelToggle.workbench.show` | Design, Docs |
| `semio.sketchpad.navbar.panelToggle.tools.show` | Design |
| `semio.sketchpad.navbar.panelToggle.hud.show` | Design, Type |
| `semio.sketchpad.navbar.panelToggle.stats.show` | Design, Type |
