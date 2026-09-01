# Entwerfen mit Bestand Demonstrator — Complete End-to-End Analysis

## 1. Landing Page Render/Boot Path (3×2 Grid Layout)

### Layout Geometry
- **Grid dimensions**: 3 columns × 2 rows = 6 panes total
- **Grid CSS**: `gridTemplateColumns: repeat(3, 100vw)`, `gridTemplateRows: repeat(2, 100vh)`
- **Total canvas size**: 300vw × 200vh (scrollable)
- **Grid element**: Desktop mode uses CSS Grid at `/📦️index.tsx:866-887`

### Desktop Grid Mode (Non-Touch)
**📦️index.tsx:863–935**
- Root wrapper: `<div className="relative h-full w-full overflow-hidden bg-background text-foreground">`
- Grid container: `<div className="grid">` with transform-based scroll
- **Transform property** (animated): `translate(-${scrollOffset.x}vw, -${scrollOffset.y}vh)` at line 872
- **Scroll glide**: cubic ease-in-out over 500ms (`DEMONSTRATOR_SCROLL_GLIDE_MS`) for focus/hover transitions at line 76
- **Free-pan follow**: exponential lerp (`DEMONSTRATOR_SCROLL_FOLLOW_LERP = 0.12`) when cursor freely pans at line 79
- **Settle epsilon**: 0.01vw/vh (line 82) for free-pan convergence
- **rAF loop**: Single `requestAnimationFrame` loop drives the transform; starts lazily and stops when settled at lines 700–754

### Mobile/Touch List Mode
**📦️index.tsx:813–860**
- Activated when: `touchListMode` = true (line 493: `UI_MOBILE_MEDIA_QUERY and (hover: none) and (pointer: coarse)`)
- Layout: Vertical snap-list with `height: 100dvh` sections
- Scroll: `snap-y snap-mandatory overflow-y-auto` (line 822)
- List container ref: `listScrollRef` at line 527
- Scroll lock: `listScrollLocked` prevents auto-scroll to next pane when user manually scrolling (line 530, 603, 590)

### Pane Focus/Zoom Path
1. **Hash-based pane focus**: `paneIdFromLocationHash()` reads `window.location.hash` to auto-focus a pane on page load (line 64–67)
2. **Click focus**: `focusPane(id)` at line 579–597
   - Stops introduction overlay: `setShowIntroduction(false)`
   - Marks pane as not suspended: `resumePane(id)`
   - **Desktop**: `applyPaneScroll(paneIndex)` triggers 500ms glide to center pane
   - **Mobile**: `scrollListToPaneIndex(paneIndex)` instantly snaps list
   - Updates URL: `window.history.replaceState(null, "", \`#${id}\`)`
3. **Return to overview**: `returnToOverview()` at line 599–610 clears focus, resets URL to pathname only

### Pane Reveal Rect (Hover Cutout)
**Desktop only** — unused in touch mode
- Hovering a pane card calls `onMouseEnter` → `refreshRevealRect(pane.id, scrollCurrentRef.current)` at line 909–914
- `demonstratorPaneRevealRect(paneIndex, scrollOffset)` at line 128–139 computes visible on-screen bounds of the grid cell
- `demonstratorTintSegmentsPx(revealRect)` at line 142–157 generates up to 4 veiling div segments with rectangular cutout
- Each segment is rendered as `<div className="ui-veil absolute">` at line 893

---

## 2. Per-Pane Configuration: Brands, Variants, Runtime, and Plugin IDs

### General Pane Boot Pattern

Each pane's `DemonstratorPane` component (line 374–437) receives:
- **`pane`**: `DemonstratorPaneSpec` — id, variant, brand, label, tagline, icon
- **`bootVariants`** (computed): `demonstratorPaneBootVariants(pane.variant)` → `{ runtime, manifest }`
  - **`runtime`**: Which plugin crate to load (e.g., "procedural3d" for Generator)
  - **`manifest`**: Canonical app id in the variant's branded row (e.g., "generator")
- **`runtimeBoot`**: `resolvePlaygroundBoot(PLUGIN_CATALOG, bootVariants.runtime)` → plugins array + defaultAppId
- **`manifestBoot`**: `resolvePlaygroundBoot(PLUGIN_CATALOG, bootVariants.manifest)` → plugins array + defaultAppId
- **Props to `FrameworkOsShell`** (line 412–423):
  - `pluginFilter`: `bootVariants.runtime`
  - `appId`: `manifestBoot.defaultAppId` (from manifest row)
  - `brand`: `pane.brand`
  - `shellId`: `pane.id`
  - `storageNamespace`: `pane.id`
  - `suppressAutoIntroduction`: true unless pane is focused

### Pane Grid Order
Array order at **🟦️brand.ts:789–796** = row-major (indices 0–2 top row, 3–5 bottom row)

| Index | Pane | Grid | Column | Row |
|-------|------|------|--------|-----|
| 0 | generator | Top-left | 0 | 0 |
| 1 | koordinator | Top-center | 1 | 0 |
| 2 | aggregator | Top-right | 2 | 0 |
| 3 | aussuchen | Bottom-left | 0 | 1 |
| 4 | bearbeiten | Bottom-center | 1 | 1 |
| 5 | verfolgen | Bottom-right | 2 | 1 |

---

### Pane 0: Generator
**🟦️brand.ts:504–549** | **PLAYGROUND_BUILD_TARGETS line 52**

| Property | Value | Notes |
|----------|-------|-------|
| **id** | `"generator"` | |
| **variant** | `"generator"` | Branded variant name |
| **runtime variant** | `"procedural3d"` | `demonstratorPaneRuntimeVariant("generator")` returns "procedural3d" at 🟦️brand.ts:779 |
| **manifest variant** | `"generator"` | Stays as "generator" for branded app id |
| **plugin id** | `"demonstrator"` | All 6 panes share the demonstrator crate |
| **brand id** | `"entwerfen-mit-bestand-generator"` | |
| **app id** | `"s.procedural.procedural3d@1/*#editor"` | PLAYGROUND line 52 |
| **exampleId** | `"hexagonal-mushroom-column"` | Default loaded example |
| **label** | `"Generator"` | |
| **tagline** | `"Parametrische Abläufe"` | |
| **icon** | `"workflow"` | |
| **introduction title** | `"Willkommen beim Generator"` | Steps: viewport (ablauf-editor), panels (catalogue) |

---

### Pane 1: Koordinator
**🟦️brand.ts:553–598** | **PLAYGROUND_BUILD_TARGETS line 57**

| Property | Value | Notes |
|----------|-------|-------|
| **id** | `"koordinator"` | |
| **variant** | `"koordinator"` | |
| **runtime variant** | `"koordinator"` | No transformation (line 779) |
| **manifest variant** | `"koordinator"` | |
| **plugin id** | `"demonstrator"` | |
| **brand id** | `"entwerfen-mit-bestand-koordinator"` | |
| **app id** | `"s.cad.cad@1/*#editor"` | PLAYGROUND line 57 |
| **exampleId** | `"hexagonal-cut-concrete-forest-left"` | |
| **label** | `"Koordinator"` | |
| **tagline** | `"Modelle koordinieren"` | |
| **icon** | `"cad-shape"` | |
| **introduction title** | `"Willkommen beim Koordinator"` | Steps: viewport (Modellansichten), panels |

---

### Pane 2: Aggregator
**🟦️brand.ts:346–497** | **PLAYGROUND_BUILD_TARGETS line 22**

| Property | Value | Notes |
|----------|-------|-------|
| **id** | `"aggregator"` | |
| **variant** | `"aggregator"` | |
| **runtime variant** | `"aggregator"` | |
| **manifest variant** | `"aggregator"` | |
| **plugin id** | `"demonstrator"` | |
| **brand id** | `"entwerfen-mit-bestand-aggregator"` | |
| **app id** | `"s.puzzle.puzzle3d@1/*#editor"` | PLAYGROUND line 22 |
| **exampleId** | `"concrete-forest"` | |
| **label** | `"Aggregator"` | |
| **tagline** | `"Bestand zusammensetzen"` | |
| **icon** | `"puzzle"` | |
| **introduction title** | `"Willkommen beim Aggregator"` | 8 steps: viewport, panels, catalogue-objects, add-object, transform-utility, verbindungspunkte, suggest-objects, fill-tool, fill-distribution |
| **tutorial** | `ENTWERFEN_MIT_BESTAND_TUTORIAL` | 4-minute voiced, seekable recorded tour (🟦️brand.ts:130–341). **Note**: document track is empty (intentional; see line 125–128) |

---

### Pane 3: Aussuchen
**🟦️brand.ts:606–644** | **PLAYGROUND_BUILD_TARGETS line 25**

| Property | Value | Notes |
|----------|-------|-------|
| **id** | `"aussuchen"` | |
| **variant** | `"aussuchen"` | |
| **runtime variant** | `"aussuchen"` | |
| **manifest variant** | `"aussuchen"` | |
| **plugin id** | `"demonstrator"` | |
| **brand id** | `"entwerfen-mit-bestand-aussuchen"` | |
| **app id** | `"s.sourcing.curate@1/*#editor"` | PLAYGROUND line 25 |
| **exampleId** | `"demo-stock"` | |
| **label** | `"Aussuchen"` | |
| **tagline** | `"Bestand sichten"` | |
| **icon** | `"library"` | |
| **introduction title** | `"Willkommen bei Aussuchen"` | Steps: viewport (Bestandspool), panels |

---

### Pane 4: Bearbeiten
**🟦️brand.ts:649–693** | **PLAYGROUND_BUILD_TARGETS line 26**

| Property | Value | Notes |
|----------|-------|-------|
| **id** | `"bearbeiten"` | |
| **variant** | `"bearbeiten"` | |
| **runtime variant** | `"bearbeiten"` | |
| **manifest variant** | `"bearbeiten"` | |
| **plugin id** | `"demonstrator"` | |
| **brand id** | `"entwerfen-mit-bestand-bearbeiten"` | |
| **app id** | `"s.process.process3d@1/*#editor"` | PLAYGROUND line 26 |
| **exampleId** | `"timber-beam-joinery"` | |
| **label** | `"Bearbeiten"` | |
| **tagline** | `"Bauteile anpassen"` | |
| **icon** | `"hammer"` | |
| **introduction title** | `"Willkommen bei Bearbeiten"` | Steps: viewport (Werkstück), panels |

---

### Pane 5: Verfolgen
**🟦️brand.ts:698–741** | **PLAYGROUND_BUILD_TARGETS line 80**

| Property | Value | Notes |
|----------|-------|-------|
| **id** | `"verfolgen"` | |
| **variant** | `"verfolgen"` | |
| **runtime variant** | `"verfolgen"` | |
| **manifest variant** | `"verfolgen"` | |
| **plugin id** | `"demonstrator"` | |
| **brand id** | `"entwerfen-mit-bestand-verfolgen"` | |
| **app id** | `"s.gis.gismap@1/*#editor"` | PLAYGROUND line 80 |
| **exampleId** | `"reuse-map"` | |
| **label** | `"Verfolgen"` | |
| **tagline** | `"Herkunft verfolgen"` | |
| **icon** | `"gis2d"` | |
| **introduction title** | `"Willkommen bei Verfolgen"` | Steps: viewport (Karte), panels |
| **engines** | `["./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"]` | Only verfolgen loads tiled-map engine (PLAYGROUND line 80) |

---

## 3. Lazy/Idle Boot Mechanism

### Sequential Pane Boot (`useSequentialPaneBoot`)
**📦️index.tsx:172–200**

**Objective**: Boot panes one at a time rather than all six simultaneously to avoid janky first paint.

**Initial focus priority**:
- If URL hash targets a pane (e.g., `#aggregator`), that pane boots first
- Otherwise, queue all panes for warm boot

**Timing**:
- **First boot delay**: 1,500ms (line 194)
- **Subsequent boots**: 35,000ms (`DEMONSTRATOR_PANE_BOOT_INTERVAL_MS`) between each (line 167)
- **Scheduling function**: `scheduleDemonstratorIdle(callback, delayMs, window)` at line 754–763
  - Enforces minimum delay via `setTimeout()`
  - Then yields to `requestIdleCallback()` with 1-second timeout if available, else calls immediately
  - Returns cancellation function (cleanup)

**Conditional boot skipping**:
- **Touch list mode** (`touchListMode`): Skips idle queue, boots on-demand as user scrolls (line 509)
- **Focused pane** (`focusedId != null`): Skips idle queue when a pane is already focused

### Pane Suspension (`usePaneSuspension`)
**📦️index.tsx:262–343**

**Policy**: Release booted pristine panes (never interacted with) to free memory when idle or offscreen.

**Key constraint**: Only PRISTINE panes are suspended (line 209). If user has touched a pane (even clicked once), it's marked dirty and never suspended. **No document round-trip exists** (readAppDocument/loadAppDocument unimplemented) so suspending a modified pane would lose work.

**Suspension thresholds** (DEMONSTRATOR_SUSPENSION_POLICY):
- **`offscreenSuspendDelayMs: 30_000`** (line 215): Booted pane fully offscreen (another pane focused)
- **`overviewIdleSuspendMs: 300_000`** (5 min, line 217): Booted pane on overview grid, nothing focused, no recent input
- **`hiddenTabSuspendMs: 60_000`** (line 219): Tab backgrounded (document.hidden), release aggressively
- **`sweepIntervalMs: 5_000`** (line 221): Sweep runs every 5 seconds

**Lifecycle**:
1. Pane booted → added to `unfocusedSinceRef` map with boot timestamp
2. Focus changes → timestamp updated
3. On sweep: Compare `Date.now() - unfocusedSinceRef.get(id)` against threshold
4. If threshold exceeded → call `suspendPane(id)`:
   - Capture current 2D canvas composite via `capturePanePoster(container)` (line 289–291)
   - Add id to `suspendedIds` set
   - FrameworkOsShell unmounts (teardown via React), releasing plugin worker + WASM
5. On hover/focus → call `resumePane(id)` to remove from suspendedIds, re-render live shell

**Poster capture** (line 229–256):
- Queries all `<canvas>` elements in pane container
- Composites each onto offscreen canvas via `ctx.drawImage()`
- Returns data URL or null if blank/tainted
- Falls back to "wird vorbereitet" skeleton if poster unavailable

### Skeleton/Loading States
**📦️index.tsx:428–433** (DemonstratorPane render)

Pane displays three possible views:
```
if (live) { /* FrameworkOsShell + error boundary */ }
else if (booted && suspended && posterDataUrl) { /* <img src={posterDataUrl} /> */ }
else { /* Skeleton: Logo + CanvasSkeleton */ }
```

**Skeleton always shows**:
- Large faint Entwerfen-mit-Bestand logo (40% opacity)
- `CanvasSkeleton` component with label `"${pane.label} wird vorbereitet"`
- `loadingBorderClass` animation
- `role="status" aria-busy="true"` accessibility

**Permanent skeleton risk**: If a pane's boot fails (e.g., plugin load timeout), it remains in skeleton state indefinitely. **No retry or error recovery mechanism exists** — it will show "wird vorbereitet" forever. Error boundary catches React-phase throws and shows "konnte nicht geladen werden", but plugin bootstrap failures (before React render) fall through.

---

## 4. TODOs, FIXMEs, Placeholders, and Stubs

### In `/📦️index.tsx`

1. **Line 125–128** (Aggregator tutorial docstring):
   - **Context**: Hand-authored tutorial skeleton for Aggregator's recorded tour
   - **Note**: Document track (`tracks.document`) is **intentionally empty**
   - **Rationale**: Real document mutations (addObjectKind, setVortexShow, etc.) must be captured from a live run; inventing them would be "silently wrong"
   - **Status**: Awaiting "recording pass" to merge live-captured ops into document track
   - **Impact**: Tutorial narrates and animates camera/gestures correctly, but document state changes won't materialize during playback

2. **Line 204–212** (DEMONSTRATOR_SUSPENSION_POLICY docstring):
   - **"REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT"** comment
   - **Gap identified**: `readAppDocument`/`loadAppDocument` are **unimplemented, documented Wave-1 gaps in the framework core**
   - **Consequence**: Suspension only safe for pristine panes; cannot save/restore user edits
   - **Use case**: Kiosk/booth screen with no user interaction

3. **Line 228** (capturePanePoster docstring):
   - **"wird vorbereitet" placeholder visual** — fallback when canvas capture fails
   - **Conditions**: No canvases yet, or every canvas samples blank, or data URL throws
   - **Caller fallback**: Existing skeleton placeholder still works

4. **Line 247** (capturePanePoster catch):
   - Comment: `/* tainted canvas or a lost GPU context — skip it; other canvases (or the placeholder fallback) still work */`
   - **Behavior**: Silently skips tainted/lost-context canvases; no logging

### In `/🟦️brand.ts`

5. **Line 53** (ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION body):
   - **Prototype disclaimer**: "Dieser Demonstrator befindet sich in aktiver Entwicklung. Viele Funktionen sind noch unvollständig oder nur als Platzhalter vorhanden"
   - **User-facing**: Shown as step 2 of general introduction overlay (line 63–72)

---

## 5. Introduction/Onboarding Overlay

### General Introduction (`ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION`)
**🟦️brand.ts:47–91** | **Rendered at 📦️index.tsx:769–777**

**Component**: `UIIntroduction` (imported from `@semio-tech/ui-react`)

**Condition**: Shown when `showIntroduction && !focusedId` (landing page focus, no app selected)

**Stepper props**:
- `introduction`: `ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION`
- `stepIndex`: `introductionStep` state (line 506)
- `completedInteractionIndices`: `[]` (empty; no interactions required)
- `onStepIndexChange`: `setIntroductionStep`
- `onDismiss`: `dismissIntroduction` → sets `showIntroduction = false`

**Three steps** (line 49–90):
1. **"welcome"** (id, title, body)
   - Title: "Willkommen bei Entwerfen mit Bestand"
   - Body: Project description (Leibniz Universität Hannover, Universität der Künste Berlin)
   - **Placement**: "center"

2. **"prototype"** (id, title, body)
   - Title: "Früher Prototyp"
   - Body: "Dieser Demonstrator befindet sich in aktiver Entwicklung…"
   - **Note**: Warns about incomplete features
   - **Placement**: "center"

3. **"funding"** (id, title, body)
   - Title: "Förderhinweis"
   - Body: Funding attribution (BBSR, BMWSB, Zukunft Bau)
   - **Logos**: Three dark/light variants (BMWSB, BBSR, Zukunft Bau) with hrefs to official pages
   - **Placement**: "center"

**Each step structure**:
```typescript
{
  id: string,
  title: string,
  body: string,
  introduce: null,          // No specific element focused
  show: [],                 // No elements highlighted
  placement: "center",
  interactions: [],         // No required interactions
  ordered: false,
  logos: [...],            // Only funding step has logos
  demonstrations: [],
}
```

### App-Specific Introductions

Each pane's brand has its own `introduction` (not the general overlay).

**Aggregator** (line 356–495):
- **Title**: "Willkommen beim Aggregator"
- **8 steps**: viewport, panels, catalogue-objects, add-object, transform-utility, verbindungspunkte, suggest-objects, fill-tool, fill-distribution
- Each step has `introduce` (element id), `show` (elements to highlight), `interactions` (gestures to demonstrate), `placement` (auto/right/top)

**Generator** (line 513–547):
- **Title**: "Willkommen beim Generator"
- **2 steps**: viewport (ablauf-editor), panels (catalogue)

**Koordinator** (line 562–596):
- **Title**: "Willkommen beim Koordinator"
- **2 steps**: viewport (Modellansichten), panels

**Aussuchen** (line 615–642):
- **Title**: "Willkommen bei Aussuchen"
- **2 steps**: viewport (Bestandspool), panels

**Bearbeiten** (line 658–691):
- **Title**: "Willkommen bei Bearbeiten"
- **2 steps**: viewport (Werkstück), panels

**Verfolgen** (line 707–739):
- **Title**: "Willkommen bei Verfolgen"
- **2 steps**: viewport (Karte), panels

### Binding to Panes

- App introductions render **inside** `FrameworkOsShell` (framework's own UI layer)
- Triggered via brand's `introduction` property passed to shell
- **`suppressAutoIntroduction` prop** (📦️index.tsx:422): Set to true for unfocused panes, so their intros don't interfere with overview
- When pane focused (`focusedId === pane.id`), shell renders its introduction

### Persistence

- General introduction shown by default (`showIntroduction` initialized as `!initialFocusId` at line 507)
- **Dismissed by**: User clicking "Done" button on last step or clicking "Skip"
- **Also dismissed by**: `focusPane(id)` call (line 583) — entering an app hides the general overlay
- **No localStorage persistence** — intro replays on every page load

---

## Summary Table

| Aspect | Detail |
|--------|--------|
| **Grid geometry** | 3 cols × 2 rows; CSS Grid with transform scroll (desktop) or snap-list (mobile) |
| **Desktop scroll driver** | Single rAF loop, glide + follow modes, 500ms cubic ease-in-out, 0.12 exponential lerp |
| **Boot strategy** | Sequential warm-boot with 1.5s → 35s intervals; skipped in touch mode or when focused |
| **Suspension policy** | Pristine panes only; 30s offscreen, 5min idle, 60s hidden-tab thresholds |
| **Poster capture** | 2D canvas composite on suspend; fallback to skeleton if blank/tainted |
| **Skeleton fallback** | No retry; permanent "wird vorbereitet" if plugin boot fails |
| **Plugin sharing** | All 6 panes use "demonstrator" plugin; Generator loads procedural3d runtime module |
| **Intro overlay** | 3-step general + app-specific intros (2–8 steps each); shown on landing, suppressed in apps |
| **Known gaps** | Document save/restore unimplemented; tutorial has empty document track; no error recovery for failed pane boots |

