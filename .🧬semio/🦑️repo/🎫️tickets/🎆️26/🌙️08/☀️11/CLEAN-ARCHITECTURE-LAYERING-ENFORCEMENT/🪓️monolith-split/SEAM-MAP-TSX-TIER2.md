# Seam Map: TSX tier 2 (seven monoliths)

Read-only audit of seven TypeScript/TSX monoliths. No source file was edited; no git write command, no build, no `tsc`, `nx`, or test run. Scratch scripts and JSON are in `🗑️generated/tier2/` next to this file (`ast.cjs`, `deps.cjs`, `scc.py`, `verify2.py`, `auto.py`, `final.py`, `build.py`, `*.json`). Date: 2026-10-15. Format follows `SEAM-MAP-SHELLHOST-TSX.md`; taxonomy findings follow `RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md`.

Method. Top-level declarations, imports, exports and `#region` markers come from a TypeScript-parser pass (`ts.createSourceFile`, parse only, no program, no checker). Dependencies between top-level units are identifier references, so shadowed locals can create false edges; treat every child-cycle result as an identifier-level result until the TypeScript checker or the repo cycle gate confirms it. Consumers are `git grep` over tracked files (fast, read-only); the plain `grep -r` over the whole tree exceeded the 120 s limit and was stopped.

## 0. Summary

| # | File (short name) | Lines (`wc -l`) | Sections | Exports | Code consumers | Hook/closure hazard | Child cycles after plan | Largest child | Verdict |
|---|---|---:|---:|---|---|---|---|---|---|
| A | `📐️cad` renderer (`✏️editor/⚙️engine/📺️renderer`) | 6,859 | 22 | 182 decls + 1 list | 1 story (namespace import) | `InteractionRepl` 1,674 lines, 136 hook calls; 3 hook-after-early-return bugs | none | `🎛️repl` 1,938 | Split ready after moving 3 hook bugs out |
| B | `🎞️animate` react engine (`✏️editor/📺️renderer/⚛️react`) | 5,559 | 21 | 141 decls + 4 lists | none (own test config only) | `InteractiveDisposition` 530 lines/17 hooks; conditional hook L2542 | none | `🧰️presentation-core` 2,030 | Duplicate of C: decide consolidation first |
| C | `🎤️presentation` react target (`📦️packages/🎯️targets/⚛️react`) | 5,550 | 20 | 147 decls + 4 lists | 7 code files via `@semio-tech/presentation-react` | same as B, conditional hook L2540 | none | `🧰️presentation-core` 2,032 | Split ready; canonical copy |
| D | `💻️os` root barrel (`💻️os/🟦️.ts`) | 5,775 | 40 | 221 decls + 21 lists | 71 code files via `@semio-tech/framework-os`; 4 source-text readers | no components | none after the cut below | `🧾️app-channel` 1,904 | Split ready after cutting the browser-bundle back-import |
| E | `🔌️PluginRuntime` element | 4,582 | 16 | 70 decls + 5 lists | ShellHost 24 lines, Shell 6 (type-only), ShellHelpers 2, tests 38, wgpu bridge 21 | no components; 24 mutable module-level variables | 1 residual cycle (`🗃️core` ↔ `📬️typed-result`) | `🧩️module-loader` 1,387 (one 1,145-line function) | Blocked on one design decision |
| F | `🕸️NodeGraph` element | 4,005 | 27 | 86 decls + 0 lists | Interpreter, TextEditor, React target, 7 tests (5 read the source as text) | `FlowGraphCanvasHost` 1,126 lines/89 hooks; `WasmGraphSurface` 439/32 | none internally; runtime cycle with `Interpreter` | `🧰️graph-core` 1,171 | Split ready after the ShellHost/Interpreter seam cut |
| G | `🎨️r3f` world engine (`♾️infinite/🌍️world/🎨️r3f`) | 4,601 | 20 | 239 decls + 1 list | 20 code files via `@semio-tech/infinite-world-r3f` | none (`WorldOrbitGated` early return verified: no hook after it) | none | `🧷️r3f-core` 919 | Split ready; no seam cuts needed |

Child sizes are measured from unit spans (imports excluded), so the parent barrel is a few hundred lines on top. Every proposed child name was checked for emoji uniqueness among its siblings and against existing siblings, for a trailing U+FE0F, and for path length (maximum 220 bytes, file B). Registration targets are listed per file.

## 1. Cross-cutting findings

1. **B and C are near-duplicate engines.** Same section layout, 5,559 vs 5,550 lines, and 77 changed lines in 32 diff hunks. The copied modules drifted: `🔌️pdf-canvas-port` 122 vs 115 lines, `📝️markdown-html-compiler` 366 vs 359 lines. B's only code consumers are its own test config and its deck story, and that story imports C through `@semio-tech/presentation-react`. Splitting B would split dead code. Recommendation: make C canonical and have B consume `@semio-tech/presentation-react`, or delete B; this is a product decision for the owner.
2. **Real runtime cycle through the OS root (D).** `🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🎛️command/🟦️.ts:3`, `…/📤️publication/🟦️.ts:4` and `…/🎯️action-handoff/🟦️.ts:207` import the root barrel `💻️os/🟦️.ts` by relative path, while the root re-exports `🔌️plugin/🌐️browser-bundle` (D lines 49-53). The plugin depends on the framework root, which depends on the plugin: inverted direction. Cut: move `encodeAppCommand`, `encodePackValue`, `decodePackValue`, `decodeAppFrame`, `encodeAppFrame`, `faultDisplayMessage` into the codec leaves (`🔨️modules/🪢️codec-core`, `🔨️modules/🎁️packs`, `🔨️modules/🧾️app-channel`) and point browser-bundle at those.
3. **Element-to-shell runtime cycle (F, Interpreter).** `NodeGraph` imports `useAppKeybindingsByActionId` and `useMapContextMenuSpecs` from `ShellHost` (F line 83) and `openSurfaceContextMenu`, `parseSceneJsonField`, `useAppCatalogue`, `useShellContextMenuFallback` from `Interpreter` (F line 80). `Interpreter` imports `NodeGraphHost` (Interpreter line 174) and `ShellHost` (lines 21 and 171). `ShellHost` imports `Interpreter` (ShellHost line 41). Runtime cycle `F → ShellHost → Interpreter → F`. Cut with the seam map's Phase 0 leaves (`shell-model`, `shell-backend`); the `NodeGraph` side needs no code change beyond the import specifiers.
4. **Type-only cycle (E, Shell).** `Shell` line 77 `import { type PluginWasmHandle } from "../🔌️PluginRuntime/🟦️.tsx"`; `PluginRuntime` line 118 `import { type PluginManifest, type ViewModel } from "../🐚️Shell/🟦️.tsx"`. Both are erased at runtime, so the risk is structural, not a load-order problem. Move the four types into a leaf.
5. **External runtime libraries in B and C.** `reveal.js` (JavaScript and `reveal.js/dist/reveal.css`) is imported by value in both files; `pdfjs-dist` is loaded through `new URL("pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url)` (B line 987, C line 983). AGENTS.md says runtime libraries must sit behind an interface. A split does not change this; flag it.
6. **Rules-of-hooks violations (A, pre-existing).** Each is a hook call after an early return, so the hook count changes between renders: `PreviewItem` (L1546 `if (!p) return null;`, hook at L1549); `SpatialPickTargetNode` (L2646 vertex `return`, `useMemo` at L2655); `TessellatedCommitMesh` (L3324 `if (!showFaces && !showEdges) return null;`, `useMemo` at L3325). Fix by hoisting the hooks above the returns. This changes behaviour, so do it as its own change, before the move.
7. **Conditional hook (B and C).** The hook-call scanner flags a hook inside a conditional expression in `FigureCropMorphView`: B line 2542, C line 2540. Read the source before the move; not verified by running.
8. **Mojibake section label (A).** `// #region ­ƒº¬Tests` at A line 6759 is a double-encoded `🧪️Tests`. Rename while moving the in-source test block.
9. **Registration needed (taxonomy).** Enforcement is confirmed by the reference doc (`verify taxonomy report`). Registration targets: children of `🧱️elements/<Element>` (E, F) go into `members-of-members-of-elements` (22 names today). Children of `📺️renderer` (A) go into `members-of-members-of-engine` (13 names). Children of `⚛️react` (B) go into `members-of-members-of-members-of-editor` (1 name). Modules under `🔨️modules/` (C, D) go into `members-of-modules` (184 names, which includes `📇️directory`). Children of `🎨️r3f` (G) go into `members-of-members-of-members-of-members-of-modules` (13 names). Name collision to avoid: `📇️directory-client` would clash with the existing `📇️directory` module (emoji 📇), so D uses `📞️directory-client`, or the directory code should go into the existing `📇️directory/🔌️client`.
10. **Source-text readers.** A split of any of these files breaks tests and scripts that read the file as text (grep-like checks). Listed per file in sections 2.A-2.G. Each reader must be re-pointed in the same change.
11. **In-source tests** (`if (import.meta.vitest)` and `registerTests…`) are at: A L6856-6858 and L6759-6761; B L134-136, L5423-5425, L5555-5557 (plus a nested check at L5190); C L5546-5548; D L371, 2582, 4702, 4804, 4903, 4929, 4957, 5044, 5051, 5058, 5065, 5735; E L4578; G L4597. F has none. Keep each block with the parent barrel unless it has a single owner.
12. **Module-level state.** Summarised per file in the sections below. Each mutable value must have exactly one owner module after the split; two modules that both define one would fork state silently.

## 2. Per-file plans

### 2.A `✏️s/🔌️plugins/📐️cad/…/✏️editor/⚙️engine/📺️renderer/🟦️.tsx` (6,859 lines, 320,072 bytes)

- **Consumers.** One code importer: `…/✏️editor/📖️stories/🎭️renderer/🧪️.story.tsx:22` (`import * as renderer from "../../⚙️engine/📺️renderer/🟦️"`, namespace import, so every export is reachable). No source-text reader (the nearby `🧪️repluserfacingsuggestiondetail` test reads JSON only). Relative imports point into the same cad plugin (`📔️registry`, `🧬️typology`, `🎬️actions`, `🗿️artifact`) and into `✏️s/🔨️modules/🌐️spatial-kernel` (shared `✏️s` module, allowed); package imports `@semio-tech/framework`, `ui-react`, `ui-styling`, `infinite-world-r3f` point framework-ward (plugin to framework, allowed).
- **Wrong-direction imports.** None found. The `ts.createSourceFile` import list has no framework-to-plugin edge.
- **Module-level state and side effects.** `r3fPreviewKernel` (L246) and `scenePreviewKernelRef` (L248) are mutable module singletons, and `THREE.Object3D.DEFAULT_UP.set(0, 0, 1)` at L83 mutates a global at import time. Keep the side effect and both singletons in the `🔌️ports` child, which imports first.
- **Closures needing hook decomposition.** `InteractionRepl` (L4808-6481, 1,674 lines, 136 top-level hook calls) is the largest closure in the set; split it into custom hooks (chrome state, selection attributes, engagement and search specs) before or during the move. `InteractionSpatialView` (L3784-4044, 11 hooks) is acceptable as is.
- **Hook order hazards** (pre-existing, see cross-cutting 6): L1546/1549 `PreviewItem`, L2646/2655 `SpatialPickTargetNode`, L3324/3325 `TessellatedCommitMesh`.
- **Cycles.** The 21-region graph has one 11-region strongly connected cluster, but that is an artifact of coarse sections: the unit-level graph has no cycle. The plan below was checked to be acyclic at child level (identifier graph). The only two-cycle is with a test file, `…/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx`, which imports A.
- **Section table**

| Lines | Section (`#region` label) | Start-end | Top-level decls | Exported | Largest decl |
|---:|---|---|---:|---:|---|
| 4 | (preamble) | 1-4 | 0 | 0 |  |
| 4 | 🧲️Header | 5-8 | 0 | 0 |  |
| 70 | 🔌️Adapters | 9-78 | 0 | 0 |  |
| 155 | 🔌️PortWiring | 79-233 | 17 | 0 | THREE (134) |
| 27 | ⚡️R3FPreviewKernel | 234-260 | 5 | 3 | R3FPreviewKernel (11) |
| 191 | 🎬️WorkerClient | 261-451 | 9 | 8 | useDocumentMeshes (41) |
| 79 | 🪩️ArchivedFootprints | 452-530 | 10 | 6 | mergeDisplayWithArchivedBoxes (12) |
| 117 | 📐️Layout | 531-647 | 12 | 6 | bboxWireSegments (30) |
| 735 | 🧲️GeometryTargets | 648-1382 | 80 | 49 | appendPrimitiveSpatialPickTargets (63) |
| 224 | 🖼️DisplayPrimitives | 1383-1606 | 7 | 0 | PreviewItem (165) |
| 156 | 🔵️CircleArcPreview | 1607-1762 | 5 | 0 | defaultDisplayItemNode (27) |
| 23 | 🎨️HostCustomization | 1763-1785 | 5 | 3 | renderDisplayItem (8) |
| 159 | 🔖️TransformGumball | 1786-1944 | 8 | 6 | SpatialTransformGumball (82) |
| 154 | 🎨️SpatialSceneColors | 1945-2098 | 10 | 4 | spatialSceneColors (47) |
| 179 | 🖱️Interaction | 2099-2277 | 13 | 5 | SpatialConstrainedPointerBridge (44) |
| 465 | 🧲️GeometryInteraction | 2278-2742 | 33 | 11 | SpatialPickTargetNode (89) |
| 148 | 🧲️SpatialPickGeometryLayer | 2743-2890 | 2 | 1 | SpatialPickGeometryLayer (86) |
| 93 | 🎨️TypologyPatternMaterial | 2891-2983 | 7 | 1 | createTypologyStyledMaterial (40) |
| 472 | 🧊️CommittedMesh | 2984-3455 | 25 | 17 | TessellatedCommitMesh (96) |
| 49 | 🪝️Hooks | 3456-3504 | 4 | 4 | useHostState (28) |
| 543 | 🪩️Canvas | 3505-4047 | 18 | 14 | InteractionSpatialView (265) |
| 2712 | 🪩️Repl | 4048-6759 | 67 | 43 | InteractionRepl (1676) |
| 102 | ­ƒº¬Tests (MOJIBAKE label) | 6760-6861 | 3 | 1 | RendererTestDependencies (97) |

- **Proposed children** (names checked for uniqueness; path max 190 bytes; register all nine in `members-of-members-of-engine`: `🔌️ports 🎨️style 🧲️geometry-targets 🎯️pick-layer 🖼️display 🔖️interaction 🪩️canvas 🧾️repl-types 🎛️repl`)

| Child | Lines | Contents (regions and units) |
|---|---:|---|
| `🔌️ports` | 232 | Header, Adapters, PortWiring (L78), R3FPreviewKernel (L233) with `r3fPreviewKernel`, `scenePreviewKernelRef`, and the L83 side effect |
| `🎨️style` | 128 | Palette only: `SpatialSceneColorPalette`, `SPATIAL_SCENE_COLOR_FALLBACK`, `spatialSceneColors`, its cache (L1944-2084). Pure leaf, no dependencies on other children |
| `🧲️geometry-targets` | 1,516 | WorkerClient (L260), ArchivedFootprints (L451), Layout (L530), GeometryTargets (L647), TypologyPatternMaterial (L2890), CommittedMesh (L2983); plus units `typologyStyleToMaterialProps` (L2437), `targetStyle` (L2450), `pinnedPickTargetKeys`, `spatialHoverKeyAliases`, `spatialHoverKeysMatch` (L2481-2503) |
| `🖼️display` | 390 | DisplayPrimitives (L1382), CircleArcPreview (L1606), HostCustomization (L1762-1784, the display-kind registry), `InteractionDisplay` (L2087-2095) |
| `🔖️interaction` | 300 | TransformGumball (L1785-1943), Interaction (L2098-2276) |
| `🎯️pick-layer` | 509 | GeometryInteraction (L2277-2741, minus the moved helpers) and SpatialPickGeometryLayer (L2742-2889) |
| `🪩️canvas` | 552 | Hooks (L3455-3503) and Canvas (L3504-4046, `InteractionSpatialView`) |
| `🧾️repl-types` | 664 | Repl types and default chrome state (L4047-4807) |
| `🎛️repl` | 1,938 | `InteractionRepl` (L4808-6481) and its panes and viewport (L6482-6758); the component needs hook decomposition |
| parent `🟦️.tsx` | - | Imports, barrel, the in-source test block (L6759-6860, keep here: it reaches every child) |

- **Extraction order (dependencies first):** `🔌️ports` → `🎨️style` → `🧲️geometry-targets` → `🔖️interaction` → `🖼️display` → `🎯️pick-layer` → `🪩️canvas` → `🧾️repl-types` → `🎛️repl` → barrel. Hook-order fixes (cross-cutting 6) first, as a separate change.

### 2.B `✏️s/🔌️plugins/🎞️animate/…/✏️editor/📺️renderer/⚛️react/🟦️.tsx` (5,559 lines, 229,234 bytes)

- **Recommendation first.** B duplicates C (cross-cutting 1). Decide which copy survives before splitting. The plan below is for the case where B is kept.
- **Consumers.** No code importer other than its own coverage/alias config (`…/🎞️animate/🧪️tests/🎚️config/🟦️.ts` lines 35 and 37) and its own tests (`🧪️compilemarkdowntohtml/🟦️.tsx`). The deck story `…/🎞️animate/📖️stories/🎭️presentation-deck/🧪️.story.tsx` imports `@semio-tech/presentation-react` (C).
- **Source-text readers.** The `🧪️compilemarkdowntohtml` test reads the sibling `🎨️.css` (not B's TSX, so a split does not break it).
- **Wrong-direction.** A plugin (`🎞️animate`) carries framework-grade copies of `pdf-canvas-port` and `markdown-html-compiler` (same modules as C). Inverted ownership; the fix is cross-cutting 1.
- **Module-level state.** `defaultMarkdownHtmlCompiler` (L118), `DEFAULT_SURFACE_CHROME` (L238), `defaultPdfCanvasPort` (L994, singleton), `DISPOSITION_RESIZE_HANDLES` (L3145), `SLIDE_INTERACTIVE_ENLARGE_FRAME` (L3315), `defaultJsonTreeRenderer` (L5535), and nine React contexts (L949, 952, 955, 958, 961, 964, 1773, 3843, 4111). The contexts are referenced by several sections, so they go to the core child.
- **Closures and hooks.** `InteractiveDisposition` (L4196-4725, 530 lines, 17 hooks) is the largest component; `FigureCropMorphView` has the conditional hook at L2542 (cross-cutting 7). `FigureScrollViewport` (L1944-2108, 14 hooks) and `PdfMorphView` (L2847-2968, 18 hooks) are acceptable.
- **Cycles.** The region graph has a four-region cluster (`PdfCanvasPort`, `FigureScroll`, `Interaction`, `ArrangementSettled`). Unit-level there is no cycle, but section-level groupings are cyclic. The cure is to promote the shared, non-component units (contexts, constants, types, helpers) into `🧰️presentation-core`. Child-level result (identifier graph): acyclic, with 8 children.
- **Import.meta and worker URL.** `new URL("pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url)` (L987), `import.meta.env.BASE_URL` (L1372), in-source tests at L134, L5423, L5555 and a nested check at L5190.
- **Section table**

| Lines | Section (`#region` label) | Start-end | Top-level decls | Exported | Largest decl |
|---:|---|---|---:|---:|---|
| 1 | (preamble) | 1-1 | 0 | 0 |  |
| 4 | 🧲️Header | 2-5 | 0 | 0 |  |
| 107 | 🔌️Adapters | 6-112 | 0 | 0 |  |
| 91 | 🔖️Markdown | 113-203 | 5 | 3 | compileMarkdownToHtml (75) |
| 41 | 🔖️MountOptions | 204-244 | 4 | 3 | syncPresentationSlideUrl (15) |
| 29 | 🔖️RevealChrome | 245-273 | 3 | 3 | syncRevealBackgroundKind (12) |
| 204 | 🔖️RevealMorph | 274-477 | 12 | 4 | centerRevealResolvedArrangement (35) |
| 413 | 🔖️ArrangementSettled | 478-890 | 32 | 24 | presentationAutoAnimateMatcher (60) |
| 39 | 🔖️HiddenPreflight | 891-929 | 1 | 1 | relaxHiddenPreflight (28) |
| 19 | 🔖️MorphView | 930-948 | 5 | 0 | morphAnchorClass (5) |
| 27 | 🔖️SlideEpoch | 949-975 | 7 | 1 | parsePresentationSlideCssSize (10) |
| 542 | 🔌️PdfCanvasPort | 976-1517 | 41 | 18 | usePdfContainerSize (54) |
| 1573 | 🔖️FigureScroll | 1518-3090 | 54 | 26 | FigureScrollViewport (167) |
| 1857 | 🔖️Interaction | 3091-4947 | 81 | 52 | InteractiveDisposition (531) |
| 127 | 🔖️ArrangementSection | 4948-5074 | 4 | 0 | ArrangementSection (71) |
| 57 | 🔖️PresentationInteractionProvider | 5075-5131 | 2 | 0 | PresentationSlideInteractionBridge (47) |
| 265 | 🔖️PresentationDeck | 5132-5396 | 1 | 1 | PresentationDeck (264) |
| 26 | 🔖️Mount | 5397-5422 | 4 | 2 | unmountPresentation (19) |
| 7 | 🧪️Tests | 5423-5429 | 0 | 0 |  |
| 2 | 🔖️json | 5430-5431 | 0 | 0 |  |
| 123 | 🔖️Renderer | 5432-5554 | 10 | 3 | JsonBranch (35) |
| 7 | 🧪️Tests | 5555-5561 | 0 | 0 |  |

- **Proposed children** (path max 220 bytes, registration `members-of-modules` if placed under `🔨️modules/`):

| Child (folder) | Lines | Units | Order |
|---|---:|---:|---:|
| `🧰️presentation-core` | 2030 | 167 | 1 |
| `🎞️deck` | 280 | 5 | 8 |
| `📑️pdf-canvas-view` | 118 | 8 | 2 |
| `🫳️interaction` | 938 | 34 | 6 |
| `🖼️figure-scroll` | 1366 | 45 | 5 |
| `🧭️arrangement-section` | 167 | 5 | 7 |
| `🧩️leaves` | 68 | 5 | 3 |
| `🪟️renderer` | 3 | 1 | 4 |

  - `🧰️presentation-core` (2,030 lines, 167 units): the promoted shared units (contexts, dependency-free constants, interfaces such as `PresentationInteractionState` and `SlideDispositionRegistry`, the morph/disposition/measurement helpers referenced across children). No component over 40 lines is promoted (checked). It is 30 lines over the 2,000-line budget; if that matters, split it into morph helpers and slide helpers and re-check.
  - `🧩️leaves` (68 lines), `📑️pdf-canvas-view` (118), `🖼️figure-scroll` (1,366), `🫳️interaction` (938), `🧭️arrangement-section` (167), `🎞️deck` (280), `🪟️renderer` (3; nearly all its units are promoted to the core).
- **Extraction order:** `🧰️presentation-core` → `📑️pdf-canvas-view` → `🧩️leaves` → `🪟️renderer` → `🖼️figure-scroll` → `🫳️interaction` → `🧭️arrangement-section` → `🎞️deck` → barrel.

### 2.C `🧰️framework/…/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` (5,550 lines, 228,805 bytes)

- **Recommendation.** Canonical copy (cross-cutting 1). Its structure matches B; the same child plan applies, with the modules placed under `🔨️modules/` next to the existing `🔌️pdf-canvas-port` and `📝️markdown-html-compiler` modules (which must be deduplicated against B's copies).
- **Consumers.** Seven code files import `@semio-tech/presentation-react`: `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🏗️builder/🌐️vite/🟦️.ts` (alias), `…/📦️packages/🟦️typescript/📦️index.ts` (the package entry), `…/🧪️tests/🎚️config/🟦️.ts`, `…/🧪️tests/🎞️react-deck/🟦️.tsx`, the animate deck story and coverage config (B), and a tracked config at `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, which sits outside `📦️packages` (check whether it is stale).
- **Source-text readers.** None found for this file's path tail. The package alias in the vite and test configs resolves to this file, so a split keeps the public entry (`📦️index.ts`) unchanged.
- **Module-level state and side effects.** Same as B: `defaultMarkdownHtmlCompiler` (L118), `DEFAULT_SURFACE_CHROME` (L234), `defaultPdfCanvasPort` (L990), `DISPOSITION_RESIZE_HANDLES` (L3143), `SLIDE_INTERACTIVE_ENLARGE_FRAME` (L3313), `defaultJsonTreeRenderer` (L5526), nine contexts (L945-960, 1771, 3841, 4109).
- **Closures and hooks.** Same as B: `InteractiveDisposition` (L4194-4723, 17 hooks), conditional hook at L2540.
- **Import.meta and worker URL.** `new URL("pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url)` (L983), `import.meta.env.BASE_URL` (L1370), in-source test at L5546-5548.
- **Cycles.** Child-level acyclic (identifier graph), 8 children.
- **Section table**

| Lines | Section (`#region` label) | Start-end | Top-level decls | Exported | Largest decl |
|---:|---|---|---:|---:|---|
| 1 | (preamble) | 1-1 | 0 | 0 |  |
| 4 | 🧲️Header | 2-5 | 0 | 0 |  |
| 107 | 🔌️Adapters | 6-112 | 0 | 0 |  |
| 87 | 🔖️Markdown | 113-199 | 5 | 3 | compileMarkdownToHtml (71) |
| 41 | 🔖️MountOptions | 200-240 | 4 | 3 | syncPresentationSlideUrl (15) |
| 29 | 🔖️RevealChrome | 241-269 | 3 | 3 | syncRevealBackgroundKind (12) |
| 204 | 🔖️RevealMorph | 270-473 | 12 | 4 | centerRevealResolvedArrangement (35) |
| 413 | 🔖️ArrangementSettled | 474-886 | 32 | 28 | presentationAutoAnimateMatcher (60) |
| 39 | 🔖️HiddenPreflight | 887-925 | 1 | 1 | relaxHiddenPreflight (28) |
| 19 | 🔖️MorphView | 926-944 | 5 | 0 | morphAnchorClass (5) |
| 27 | 🔖️SlideEpoch | 945-971 | 7 | 1 | parsePresentationSlideCssSize (10) |
| 544 | 🔌️PdfCanvasPort | 972-1515 | 41 | 18 | usePdfContainerSize (56) |
| 1573 | 🔖️FigureScroll | 1516-3088 | 54 | 26 | FigureScrollViewport (167) |
| 1857 | 🔖️Interaction | 3089-4945 | 81 | 53 | InteractiveDisposition (531) |
| 127 | 🔖️ArrangementSection | 4946-5072 | 4 | 0 | ArrangementSection (71) |
| 57 | 🔖️PresentationInteractionProvider | 5073-5129 | 2 | 0 | PresentationSlideInteractionBridge (47) |
| 264 | 🔖️PresentationDeck | 5130-5393 | 1 | 1 | PresentationDeck (263) |
| 27 | 🔖️Mount | 5394-5420 | 4 | 2 | mountPresentation (13) |
| 2 | 🔖️json | 5421-5422 | 0 | 0 |  |
| 123 | 🔖️Renderer | 5423-5545 | 10 | 4 | JsonBranch (35) |
| 7 | 🧪️Tests | 5546-5552 | 0 | 0 |  |

- **Proposed children** (registration: `members-of-modules`):

| Child (folder) | Lines | Units | Order |
|---|---:|---:|---:|
| `🧰️presentation-core` | 2032 | 167 | 1 |
| `🎞️deck` | 279 | 5 | 8 |
| `📑️pdf-canvas-view` | 118 | 8 | 2 |
| `🫳️interaction` | 938 | 34 | 6 |
| `🖼️figure-scroll` | 1366 | 45 | 5 |
| `🧭️arrangement-section` | 167 | 5 | 7 |
| `🧩️leaves` | 64 | 4 | 3 |
| `🪟️renderer` | 3 | 1 | 4 |

- **Extraction order:** `🧰️presentation-core` → `📑️pdf-canvas-view` → `🧩️leaves` → `🪟️renderer` → `🖼️figure-scroll` → `🫳️interaction` → `🧭️arrangement-section` → `🎞️deck` → barrel. Re-point `📦️index.ts` only if the barrel path changes (it should not).

### 2.D `🧰️framework/🛍️products/💻️os/🟦️.ts` (5,775 lines, 353,940 bytes)

- **Consumers.** 71 code files import `@semio-tech/framework-os`, so the root barrel is the public API of the OS framework; all 221 exported declarations and 21 export lists must stay reachable from the root. Source-text readers of this file: `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:599` (`osTs`), `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts:156` (`protocol`), `…/💻️os/📦️packages/🦀️rust/📜️script.ts:2336` (`typescript`), and a match at `…/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts:387` (verify). Those scripts read this file as text; re-check their assertions whenever a declaration moves.
- **Wrong-direction.** Plugin `browser-bundle` imports the root (cross-cutting 2). Also the root has 3 test-only back-edges (`🧪️tests/🧪️backbone-envelope-io`, `🧪️tests/🧊️mesh-pack-decode`, `browser-bundle/…/action-handoff`).
- **Module-level state.** Constants `replicationPackCodec` (L55), `BACKBONE_MESSAGE_ORDINALS` (L385), `DESKTOP_WINDOW_CONTROL_CHANNELS` (L541), `COMMAND_REJECTION_CODES_V1` (L716), `packIntegerMint` (WeakSet, L1727), `MESH_PACK_FIELD` (L2427), `APP_COMMAND_TAGS` and `APP_FRAME_TAGS` (L2928, L2939), plus in-source tests at L371, L2582, L4702, L4804, L4903, L4929, L4957, L5044, L5051, L5058, L5065, L5735.
- **Closures and hooks.** No React components; no hook hazards. Large functions to watch: `decodeBackboneWorkerResponse` (112 lines), `encodeAppCommand` (195-line span in the Codec region), `DirectoryClient` class (380 lines inside the ConcurrentWrite region).
- **Cycles.** Region graph: two cycles (`PublicApi`↔`Header`, and a three-section cycle `ConcurrentWrite`/`CommandRejection`/`BackboneWorkerProtocol`). Unit-level graph: none. Pinning the six backbone worker codec units (`encodeBackboneWorkerRequest`, `decodeBackboneWorkerRequest`, `encodeBackboneWorkerResponse`, `decodeBackboneWorkerResponse`, `parseDirectoryAdministrationWorkerRequestV1`, `parseSpaceArtifactCreationWorkerRequestV1`) into the backbone child breaks the remaining child-level cycle. Child-level result: acyclic, 5 children plus barrel.
- **Section table**

| Lines | Section (`#region` label) | Start-end | Top-level decls | Exported | Largest decl |
|---:|---|---|---:|---:|---|
| 14 | (preamble) | 1-14 | 0 | 0 |  |
| 43 | Header | 15-57 | 1 | 0 | replicationPackCodec (7) |
| 93 | 🔖️ArtifactOpeningRelay | 58-150 | 5 | 2 | resolveArtifactOpeningRelay (59) |
| 103 | 🔖️Backbone | 151-253 | 19 | 17 | decodeDocumentPackBytes (11) |
| 124 | 🌐️BackboneEnvelopeIo | 254-377 | 9 | 3 | readBackboneEnvelope (42) |
| 162 | 🔀️BackboneMessage | 378-539 | 12 | 11 | decodeBackboneMessage (59) |
| 45 | 🔖️DesktopWindowChrome | 540-584 | 4 | 4 | registerDesktopWindowControlHandlers (19) |
| 9 | 🔖️Blob | 585-593 | 1 | 1 | BLOB_ENDPOINT_PATH (14) |
| 923 | 🔖️BackboneWorkerProtocol | 594-1516 | 65 | 48 | decodeBackboneWorkerResponse (113) |
| 116 | 🔖️WorkflowPlanner | 1517-1632 | 8 | 7 | planWorkflow (52) |
| 18 | 🔖️PackValueCodec | 1633-1650 | 0 | 0 |  |
| 60 | 🔖️PackContainerPrimitives | 1651-1710 | 15 | 1 | packEncodedByteCompare (19) |
| 102 | 🔖️PackInteger | 1711-1812 | 17 | 7 | packWriteVarintBigInt (15) |
| 167 | 🔖️JsonValueTags | 1813-1979 | 7 | 0 | packDecodeValue (59) |
| 316 | 🔖️PublicApi | 1980-2295 | 31 | 20 | encodePackValue (42) |
| 125 | 🔖️ScenePackCodec | 2296-2420 | 21 | 1 | decodeScenePackItem (68) |
| 168 | 🔖️MeshPackCodec | 2421-2588 | 12 | 3 | decodeMeshPackBody (59) |
| 17 | 🔖️AppChannelCodec | 2589-2605 | 0 | 0 |  |
| 167 | 🔖️Types | 2606-2772 | 17 | 17 | AppCommandValue (69) |
| 126 | 🔖️Combinators | 2773-2898 | 19 | 3 | writeDocumentArchive (20) |
| 773 | 🔖️Codec | 2899-3671 | 11 | 4 | encodeAppCommand (195) |
| 96 | 🔖️AppChannelClient | 3672-3767 | 10 | 10 | faultDisplayMessage (14) |
| 75 | 🏠️LocalQueryOwnership | 3768-3842 | 7 | 1 | AppChannelRequestSequence (31) |
| 421 | 🗃️DocumentArchiveLoadHost | 3843-4263 | 12 | 9 | AppChannelClient (677) |
| 356 | 🏠️LocalInteractionQuery | 4264-4619 | 0 | 0 |  |
| 22 | 🔖️Merge | 4620-4641 | 0 | 0 |  |
| 11 | 🔖️Presence | 4642-4652 | 0 | 0 |  |
| 49 | 🔖️Transaction | 4653-4701 | 0 | 0 |  |
| 9 | 🧪️Tests | 4702-4710 | 0 | 0 |  |
| 29 | StdioFormatKinds | 4711-4739 | 2 | 2 | mediaAcceptFilterKinds (89) |
| 70 | 🔖️Directory | 4740-4809 | 0 | 0 |  |
| 21 | 🔖️HubBinding | 4810-4830 | 3 | 3 | CanonicalDirectorySpaceAdministrationPageV1 (8) |
| 79 | 🔌️DocumentLinkShortage | 4831-4909 | 14 | 14 | documentLinkExpiresAtMs (16) |
| 26 | 🔁️DocumentEchoSuppression | 4910-4935 | 2 | 2 | noteAuthoredEnvelopeIds (15) |
| 28 | ⏳️TransientApplyRefusal | 4936-4963 | 2 | 2 | hubTransientApplyRefusalV1 (20) |
| 87 | 🚫️CommandRejection | 4964-5050 | 9 | 4 | commandBatchIdOfValueV1 (47) |
| 7 | 🔑️DirectoryAccessChanged | 5051-5057 | 0 | 0 |  |
| 7 | 📤️OutboundAnnouncement | 5058-5064 | 0 | 0 |  |
| 678 | ⚔️ConcurrentWrite | 5065-5742 | 42 | 25 | DirectoryClient (380) |
| 26 | 🔖️HubSignIn | 5743-5768 | 0 | 0 |  |
| 9 | 🔖️HubFirstRun | 5769-5777 | 0 | 0 |  |

- **Proposed children** (registration `members-of-modules`; the `📇️directory` module already exists, so `📞️directory-client` is only needed if the directory client does not move under `📇️directory/🔌️client`):

| Child (folder) | Lines | Units | Order |
|---|---:|---:|---:|
| `🔨️modules/🪢️codec-core` | 510 | 74 | 1 |
| `🔨️modules/🎁️packs` | 419 | 61 | 2 |
| `🔨️modules/📞️directory-client` | 794 | 87 | 5 |
| `🔨️modules/🧵️backbone` | 1126 | 111 | 3 |
| `🔨️modules/🧾️app-channel` | 1904 | 68 | 4 |

  - `🪢️codec-core` (510): shared pack and wire codecs referenced by two or more children, promoted automatically (for example `packDecodeValue`, `packEncodeValue`, `commandAckOutcomeOfValueV1`).
  - `🎁️packs` (419): PackValueCodec (L1632), PackContainerPrimitives (L1650), PackInteger (L1710), JsonValueTags (L1812), PublicApi (L1979), ScenePackCodec (L2295), MeshPackCodec (L2420).
  - `🧵️backbone` (1,126): ArtifactOpeningRelay (L57), Backbone (L150), BackboneEnvelopeIo (L253), BackboneMessage (L377), DesktopWindowChrome (L539), Blob (L584), BackboneWorkerProtocol (L593-1515) with the pinned codecs, WorkflowPlanner (L1516).
  - `🧾️app-channel` (1,904, close to the budget): AppChannelCodec (L2588), Types (L2605), Combinators (L2772), Codec (L2898-3670), AppChannelClient (L3671), LocalQueryOwnership (L3767), DocumentArchiveLoadHost (L3842-4262).
  - `📞️directory-client` (794): LocalInteractionQuery (L4263) through ConcurrentWrite (L5064-5741, `DirectoryClient`) and HubSignIn (L5742), HubFirstRun (L5768).
- **Extraction order:** `🪢️codec-core` → `🎁️packs` → `🧵️backbone` → `🧾️app-channel` → `📞️directory-client` → root barrel (keep explicit named re-exports; the reference doc's `export *` ban applies).

### 2.E `…/🧱️elements/🔌️PluginRuntime/🟦️.tsx` (4,582 lines, 303,438 bytes)

- **Consumers.** Literal-path grep: ShellHost (24 lines), Shell (6 lines, one of them the type-only import at L77), ShellHelpers (2, including the re-export at ShellHelpers L4249), the `🧪️tests/🔌️plugin-runtime` test (38 lines), the `🧪️tests/🔬️engine-contract` test (9), and the wgpu target `🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` (21).
- **Source-text readers.** `🌎️hub/📦️packages/🦀️rust/📜️script.ts:14512` (`bundleModule(join(repoRoot, …PluginRuntime/🟦️.tsx))`, a build input), the React target package script at line 185 (`const runtime = readFile…`), and the target's test config at line 217. Re-point all three.
- **Wrong-direction.** Imports `🐚️Shell` (type-only, cross-cutting 4), `📃️UiDocumentStore` (runtime, element to element), `📤️SegmentedDownload` (runtime marker constant), `🫀️load-progress` and `💼️job-ledger` (sibling children, fine). The Shell edge is the only back-edge.
- **Module-level state (24 mutable values).** `let` singletons: `isolatedJobDriveDepth` (L335), `isolatedJobUiPollDue` (L336), `sharedShardClient` (L555), `sharedActivationRegistry` (L586), `nextGlobalInstanceId` (L1260), `sharedThunkScheduler` (L1305), `sharedPluginTurnScheduler` (L1532), `currentPluginRuntimeActor` (L3817). Maps and Sets: `isolatedJobDriveListeners` (L337), `instanceRecoveryByActor` (L443), `pluginInstanceLostSubscribers` (L445), `typedOperationRoutersByActor` (L774), `pendingTurnEffects` (L1136), `pendingCompletionEffects` (L1165), `completionFanouts` (L1175), `pendingCoalescedTurns` (L1482), `pendingLifecycleTurns` (L1483), `tearingDownPluginActors` (L1484), `crossingCensusV1` (L1490), `typedOpAckCensusV1` (L1492), `retainedWindowByActor` (L2059), `retiredPluginInstances` (L2356), `guestIngressGenerationByInstance` (L3685).
- **Side effects.** `Object.defineProperty(globalThis, "__semioCrossingCensus", …)` (L1529) and `"__semioTypedOpAckCensus"` (L1530). These getters must move with their Maps (both belong to the core child), and the globalThis property names are part of the runtime contract, so keep them unchanged.
- **Closures and hooks.** No React components. The large functions are the problem: `loadPluginModule` is a 1,145-line function (L2398-3542), and `adaptPluginHandle` 242 lines (L3883). Splitting `loadPluginModule` is a separate, manual refactor; the plan keeps it whole in `🧩️module-loader`.
- **Cycles.** Child-level: one residual two-cycle, `🗃️core` ↔ `📬️typed-result` (core→typed 9 edges, typed→core 3 edges). At the last run, the three typed→core edges come from `withTypedOperationCall` and `consumeTypedOperationEffects` (into core state and codec helpers), and the nine core→typed edges include `settlePluginTurn` and `settleAcknowledgedPluginTurns` (into `TypedOperationCall` and `consumeTypedOperationEffects`) and the `TYPED_OPERATION_*` constants. Resolution needs a manual decision: move the typed-operation router state and the `settle*` callers into the typed-result child, or introduce an interface between them. Not resolved here.
- **Section table**

| Lines | Section (`#region` label) | Start-end | Top-level decls | Exported | Largest decl |
|---:|---|---|---:|---:|---|
| 4 | (preamble) | 1-4 | 0 | 0 |  |
| 22 | 🧲️Header | 5-26 | 0 | 0 |  |
| 98 | 🔌️Adapters | 27-124 | 0 | 0 |  |
| 130 | 🔖️plugin-runtime | 125-254 | 4 | 4 | PluginWasmHandle (166) |
| 39 | 🔖️Merge | 255-293 | 0 | 0 |  |
| 379 | 🔖️ActorAdapter | 294-672 | 40 | 18 | buildShardClientOptions (36) |
| 186 | 📬️TypedOperationResult | 673-858 | 15 | 0 | consumeTypedOperationEffects (78) |
| 434 | 🔖️RetainedUiPatch | 859-1292 | 23 | 4 | wireEffectToFriendly (114) |
| 140 | 🔖️GenericThunkQueue | 1293-1432 | 14 | 7 | serializePerActor (29) |
| 893 | 🔖️PluginTurnScheduler | 1433-2325 | 71 | 11 | settlePluginTurn (63) |
| 1300 | 🪦️InstanceRetirement | 2326-3625 | 14 | 9 | loadPluginModule (1149) |
| 182 | 🔖️ChannelAdapter | 3626-3807 | 10 | 3 | invocationFromFrames (52) |
| 19 | 🔖️ActorIdentity | 3808-3826 | 2 | 1 | setPluginRuntimeActor (15) |
| 200 | 🔖️HistorySnapshotRetry | 3827-4026 | 5 | 1 | adaptPluginHandle (247) |
| 101 | 🔖️Merge | 4027-4127 | 0 | 0 |  |
| 313 | 🔖️Transaction | 4128-4440 | 15 | 9 | TransactionCoordinator (198) |
| 144 | 🔖️DependencyOrderedBoot | 4441-4584 | 6 | 3 | loadPluginModulesInDependencyOrder (45) |

- **Proposed children** (registration `members-of-members-of-elements`; all names are sibling-unique, longest path 167 bytes):

| Child (folder) | Lines | Units | Order |
|---|---:|---:|---:|
| `🗃️core` | 1171 | 173 |  |
| `📬️typed-result` | 152 | 15 |  |
| `🔁️history-retry` | 329 | 19 |  |
| `🧲️actor-adapter` | 29 | 11 |  |
| `🧩️module-loader` | 1387 | 2 |  |
| `🪝️wire-codec` | 22 | 4 | 1 |
| `🧵️turn-scheduler` | 2 | 2 |  |
| `🧷️plugin-runtime-core` | 137 | 1 |  |
| `🪟️retained-ui-patch` | 108 | 1 | 2 |

  - `🗃️core` (1,171): shared state and helpers referenced by more than one child (pending-effect maps, census maps, retained-window state, InstanceRetirement's shared helpers). Module state lives here.
  - `🧩️module-loader` (1,387): `loadPluginModule` (L2398) and `adaptPluginHandle` (L3883).
  - `🔁️history-retry` (329), `🧷️plugin-runtime-core` (137, PluginWasmHandle L153-289 and its types), `📬️typed-result` (152), `🧲️actor-adapter` (29), `🪟️retained-ui-patch` (108), `🪝️wire-codec` (22, WireVariant and `decodeWirePack`, `coerceWireBytes`, `DIRECTORY_PROJECTION_RECEIPT_SCHEMA`), `🧵️turn-scheduler` (2).
- **Extraction order** (once the residual cycle is resolved): `🪝️wire-codec` → `🪟️retained-ui-patch` → `🧷️plugin-runtime-core` → `🧲️actor-adapter` → `📬️typed-result` → `🗃️core` → `🧩️module-loader` → `🔁️history-retry` → `🧵️turn-scheduler` → barrel. The order in `final.json` is incomplete because of the residual cycle.

### 2.F `…/🧱️elements/🕸️NodeGraph/🟦️.tsx` (4,005 lines, 199,310 bytes)

- **Consumers.** The React target (`…/🎯️targets/⚛️react/🟦️.tsx:1101`, `NodeGraphHost`), `Interpreter` (L174), `TextEditor` (L18 `useClient`, and `…/✏️TextEditor/🧪️tests/📝️explicit-draft/🟦️.tsx:8`), and the tests `🎥️tutorial-bridge` (L3, `dagWorldToScreen`), `🕸️node-graph-domain-interaction` (L11), `🧪️node-graph-edit-rows` (L11), `🛑️scene-pointer-cancellation` (L6, `?raw` import).
- **Source-text readers (break if declarations move).** `🔗️node-graph-wire-edit/🟦️.ts:29` (`nodeGraphSource`), `🔬️engine-contract/🟦️.ts:13940-13955` (readFileSync plus dynamic import of `nodeGraphSurfaceSelectionDomV1`), `🧪️node-graph-edit-rows/🟦️.ts:54`, `🫳️node-graph-gestures/🟦️.ts:158, 177, 185`, and `🛑️scene-pointer-cancellation/🟦️.ts:6` (`?raw`). These tests match text patterns in this file; each needs a re-point, and at least the `?raw` one must follow the new file.
- **Wrong-direction.** Cross-cutting 3 (ShellHost, Interpreter, World3dHost, ShellSearch, WasmSessionLoader imports). NodeGraph is an element that pulls in shell hosts: move the keybinding and context-menu hooks it needs into the Phase 0 leaves.
- **Module-level state.** `activeFlowTasks` (WeakMap, L2384), `flowSessionDeliveredPayloads` (L2466), `flowSessionSendingPayloads` (L2467), `flowSharedPayloadDigests` (Set, L2498), `flowSessionSharedCoalescers` (L2561), `flowSessionSharedIssues` (L2562). Plus constants `DEFAULT_NODE_GRAPH_VIEWPORT` (L258), `workflowNodeTypes` (L738), `GRAPH_WHEEL_ZOOM_NOTCH` (L746), `ALIGN_MODES` (L2339). The six flow maps belong with `FlowGraphCanvasHost`, which is the only writer.
- **Closures and hooks.** `FlowGraphCanvasHost` (L2877-4002, 1,126 lines, 89 hooks) is the largest component of the seven. It needs hook decomposition before or during the move: this is the single biggest risk in F. `WasmGraphSurface` (L765-1203, 439 lines, 32 hooks) and `DiagramGraphFallback` (179 lines, 14 hooks) also need it. `NodeGraphHost` (L1450-1538, 21 hooks) has an early return at L1498, with no hook after it.
- **Cycles.** Internal: none (the 22-region graph is a DAG). Runtime, cross-file: `NodeGraph` → `Interpreter` → `NodeGraph` and `NodeGraph` → `ShellHost` → `Interpreter` → `NodeGraph`.
- **Section table**

| Lines | Section (`#region` label) | Start-end | Top-level decls | Exported | Largest decl |
|---:|---|---|---:|---:|---|
| 1 | (preamble) | 1-1 | 0 | 0 |  |
| 7 | 🧲️Header | 2-8 | 0 | 0 |  |
| 81 | 🔌️Adapters | 9-89 | 0 | 0 |  |
| 1 | 🔖️NodeGraphHost | 90-90 | 0 | 0 |  |
| 45 | Types | 91-135 | 3 | 0 | FrameworkGraphSession (36) |
| 73 | 🎯️GraphPickContract | 136-208 | 8 | 5 | usePortTypeLabels (13) |
| 49 | Viewport | 209-257 | 7 | 4 | publishNodeGraphHover (12) |
| 38 | Parsing | 258-295 | 5 | 4 | resolveHostSnapshotWidgetInstanceId (12) |
| 335 | FlowCatalogueSpotlight | 296-630 | 17 | 9 | FlowSpotlight (153) |
| 71 | Keyboard | 631-701 | 3 | 0 | handleGraphKeyboard (37) |
| 40 | DiagramNode | 702-741 | 2 | 0 | WorkflowDiagramNode (36) |
| 465 | WasmGraphSurface | 742-1206 | 4 | 3 | WasmGraphSurface (442) |
| 182 | DiagramFallback | 1207-1388 | 1 | 0 | DiagramGraphFallback (185) |
| 1 | NodeGraphHost | 1389-1389 | 0 | 0 |  |
| 60 | Helpers | 1390-1449 | 4 | 3 | PresencePeersOverlay (27) |
| 93 | Component | 1450-1542 | 1 | 1 | NodeGraphHost (95) |
| 2 | 🔖️graph-canvas-overlays | 1543-1544 | 0 | 0 |  |
| 83 | DagOverlayTypes | 1545-1627 | 9 | 8 | DagSliderOverlayRow (19) |
| 25 | DagOverlayGeometry | 1628-1652 | 3 | 3 | dagScreenToWorld (11) |
| 93 | 📷️ContentFraming | 1653-1745 | 11 | 9 | dagContentCoverage (19) |
| 453 | DagOverlayPaint | 1746-2198 | 28 | 19 | dagIntroductionResolver (73) |
| 56 | 🎚️SliderGestureLanes | 2199-2254 | 1 | 0 | useGraphSliderLanes (37) |
| 121 | DagDomOverlays | 2255-2375 | 4 | 3 | GraphSliderOverlays (84) |
| 2 | 🔖️flow-graph-canvas-host | 2376-2377 | 0 | 0 |  |
| 101 | Sync | 2378-2478 | 9 | 0 | observeFlowTask (31) |
| 259 | 📦️SharedFlowPayloads | 2479-2737 | 24 | 3 | createFlowSharedPayloadCoalescer (28) |
| 5 | FlowGraphCanvasHost | 2738-2742 | 1 | 1 | flowSurfaceRenderAllowed (6) |
| 1265 | 🫱️InteractionPublication | 2743-4007 | 12 | 11 | FlowGraphCanvasHost (1130) |

- **Proposed children** (registration `members-of-members-of-elements`):

| Child (folder) | Lines | Units | Order |
|---|---:|---:|---:|
| `🧰️graph-core` | 1171 | 130 | 1 |
| `🔷️diagram-node` | 215 | 3 | 3 |
| `🎚️node-graph-host` | 117 | 5 | 8 |
| `🖼️dag-overlays` | 92 | 4 | 4 |
| `🗂️types-and-parsing` | 1 | 1 | 2 |
| `🧭️wasm-graph-surface` | 439 | 1 | 6 |
| `🪄️catalogue-spotlight` | 204 | 5 | 5 |
| `🫱️interaction-publication` | 1167 | 8 | 7 |

  - `🧰️graph-core` (1,171): shared types, pick contract, viewport and parsing helpers, keyboard and sync helpers, promoted automatically because other children reference them.
  - `🫱️interaction-publication` (1,167): `FlowGraphCanvasHost` component (1,126 lines) plus `InteractionPublication` (L2742).
  - `🧭️wasm-graph-surface` (439), `🪄️catalogue-spotlight` (204), `🔷️diagram-node` (215), `🎚️node-graph-host` (117), `🖼️dag-overlays` (92), `🗂️types-and-parsing` (1, almost all promoted into the core).
- **Extraction order:** `🧰️graph-core` → `🗂️types-and-parsing` → `🔷️diagram-node` → `🖼️dag-overlays` → `🪄️catalogue-spotlight` → `🧭️wasm-graph-surface` → `🫱️interaction-publication` → `🎚️node-graph-host` → barrel.

### 2.G `…/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` (4,601 lines, 215,975 bytes)

- **Consumers.** 20 code files import `@semio-tech/infinite-world-r3f`, among them the cad plugin (A), two vite builder configs, the OS React target and its config, a story, World3dHost, ShellHost, ChromePanels, and framework tests (`⌨️window-scope`, `📐️projection-render-parity`, `🔬️engine-contract`, `🖱️world3d-interaction`, World3dHost tests). Through the package name, not through this file's path.
- **Source-text readers.** None found for this file's path tail. The reference hits are package-path aliases and an editor plan file.
- **Wrong-direction.** None. The file has no relative imports (dynamic imports in its in-source test only).
- **Module-level state.** `worldMeshBorderColorCache` (`let`, L523). Scratch objects `_mCadToThree` (L252) and `_collisionHit`, `_collisionBoxA`, `_collisionBoxB`, `_collisionInterBox`, `_collisionSize`, `_collisionPoint`, `_collisionLocal`, `_collisionPartLocal`, `_collisionMatA`, `_collisionMatB`, `_collisionAtoB`, `_collisionInvA`, `_collisionInvB`, `_collisionRay`, `_collisionVa`, `_collisionVb`, `_collisionVc`, `_collisionNormal`, `_collisionToPoint` (L336-354) are reused mutable scratch objects; they appear to be used only by the Collision section, so they belong with `🧱️collision` (verify). Constants: `WORLD_NAVIGATION_THRESHOLDS_DEFAULT` (L129), `DEFAULT_LOD_RANGE` (L783), `DEFAULT_GRID_PLANE_ANCHOR_CAD` (L785), the orbit-camera tables (L1148-1242), `WORLD_PROJECTION_MORPH_FALLBACK_REF` (L1383).
- **Closures and hooks.** Largest components: `WorldProjectionGizmoViewport` (62 lines, 6 hooks), `WorldProjectionSnapDriver` (99 lines, 8 hooks), `WorldOrbitControlsBridge` (92, 9), `WorldOrbitGated` (77, 9), `WorldCanvas` (99, 8). None needs decomposition for size. `WorldOrbitGated` has an early return at L3639; checked, no hook follows it (the hook scanner's hit was a comment).
- **Cycles.** Child-level acyclic; 11 children.
- **Section table**

| Lines | Section (`#region` label) | Start-end | Top-level decls | Exported | Largest decl |
|---:|---|---|---:|---:|---|
| 1 | (preamble) | 1-1 | 0 | 0 |  |
| 4 | 🧲️Header | 2-5 | 0 | 0 |  |
| 84 | 🔌️Adapters | 6-89 | 20 | 0 | {destructure} (31) |
| 143 | 🔖️Types | 90-232 | 22 | 19 | classifyWorldNavigationGestures (25) |
| 19 | 🔖️EventBinding | 233-251 | 1 | 1 | WorldEventBindingController (18) |
| 71 | 🧭️Precision | 252-322 | 12 | 9 | cadObjectLocalDirectionToThreeGroupLocal (13) |
| 194 | Collision | 323-516 | 29 | 7 | collisionBodyFromObject (50) |
| 49 | 🎨️MeshBorder | 517-565 | 7 | 6 | applyWorldMeshEdgeBorders (17) |
| 113 | 🧱️Chunking | 566-678 | 8 | 7 | useVisibleChunkKeys (28) |
| 13 | 👁️ViewRadius | 679-691 | 2 | 2 | ViewRadiusLayerProps (7) |
| 91 | 🏊️Pool | 692-782 | 3 | 3 | createTemplatePool (54) |
| 334 | 📶️Lod | 783-1116 | 36 | 32 | WorldLodBridge (57) |
| 25 | 🗂️LayerStack | 1117-1141 | 4 | 3 | WorldLayer (10) |
| 334 | 📷️OrbitCameraView | 1142-1475 | 46 | 19 | computeOrbitCameraViewState (35) |
| 829 | 🧭️WorldOrbitViewGizmoViewport | 1476-2304 | 43 | 30 | WorldProjectionSnapDriver (100) |
| 972 | 📐️WorldProjection | 2305-3276 | 72 | 59 | WorldCurvilinearPass (71) |
| 140 | 🖱️OrbitMouseBindings | 3277-3416 | 8 | 8 | useWorldOrbitRightMouseBindings (67) |
| 482 | 🎬️WorldCanvas | 3417-3898 | 16 | 7 | WorldCanvas (105) |
| 362 | 🖼️Reference | 3899-4260 | 22 | 18 | WorldReferencePlaneItem (126) |
| 234 | 🧊️Volume | 4261-4494 | 16 | 8 | WorldVolumeBoxItem (71) |
| 109 | 🧪️Tests | 4495-4603 | 1 | 1 | WorldR3fTestDependencies (106) |

- **Proposed children** (registration `members-of-members-of-members-of-members-of-modules` for the children of `🎨️r3f`):

| Child (folder) | Lines | Units | Order |
|---|---:|---:|---:|
| `🧷️r3f-core` | 919 | 122 | 1 |
| `🎬️canvas` | 413 | 17 | 2 |
| `📐️projection` | 435 | 31 | 3 |
| `📷️camera-view` | 33 | 7 | 4 |
| `🔭️scene-types` | 41 | 10 | 5 |
| `🖼️reference` | 328 | 22 | 6 |
| `🗂️layers` | 13 | 2 | 7 |
| `🧊️volume` | 214 | 16 | 8 |
| `🧭️gizmo-viewport` | 708 | 38 | 9 |
| `🧮️chunking-lod` | 478 | 47 | 10 |
| `🧱️collision` | 209 | 36 | 11 |

  - `🧷️r3f-core` (919): shared types and event bindings, precision and gesture helpers, promoted automatically.
  - `🧱️collision` (209), `🧮️chunking-lod` (478: Chunking L565, ViewRadius L678, Pool L691, Lod L782), `🧭️gizmo-viewport` (708, WorldOrbitViewGizmoViewport L1475), `📐️projection` (435, WorldProjection L2304), `🎬️canvas` (413, WorldCanvas L3416), `🖼️reference` (328), `🧊️volume` (214), `📷️camera-view` (33), `🔭️scene-types` (41), `🗂️layers` (13), parent keeps adapters (L5-88) and the in-source test (L4494-4602).
- **Extraction order:** `🧷️r3f-core` → `🔭️scene-types` → `🧱️collision` → `🧮️chunking-lod` → `📷️camera-view` → `📐️projection` → `🧭️gizmo-viewport` → `🎬️canvas` → `🖼️reference` → `🗂️layers` → `🧊️volume` → barrel.

## 3. Global extraction sequence

Phase 0 (cross-file seams, no body moves): (a) move the ShellHost hooks that NodeGraph and Interpreter use into the seam map's `shell-model` leaf (F lines 80, 83; Interpreter lines 21, 171); (b) move the four types exchanged between PluginRuntime and Shell into a leaf (E line 118, Shell line 77); (c) cut the browser-bundle import of the OS root (D, cross-cutting 2); (d) decide B vs C (cross-cutting 1).

Then, independent children: G (acyclic, no seams), A (after the three hook-order fixes), F (after Phase 0 seam a; the 1,126-line component needs hook decomposition first or in parallel), D (after Phase 0 seam c), C (canonical copy; B only if kept, after deduplication). E last, because its residual cycle needs a manual decision.

## 4. Open questions and unverified points

- Identifier-level child graphs and cycle results come from name references; shadowed locals can add false edges and hide real ones. Re-run the cycle check with the TypeScript checker or the repo gate before every extraction.
- No checker, build, test or taxonomy validator was run. Taxonomy registration is specified from the taxonomy JSON; the emoji and path checks were performed by script.
- Hook placements come from the TypeScript parser and the source lines quoted; they are not runtime-verified.
- Whether B or C is canonical is a product decision; this report does not assume it.
- The seven tracked `#region` labels are not uniform (A has the mojibake label; F, D and E mix emoji and plain-word labels). Cosmetic, but it affects grep-based gates that search by label.
- The `🧪️tests` folder under `📺️renderer` is an existing sibling of A's new children; no child name collides with it.
- E's `loadPluginModule` (1,145 lines, one function) and F's `FlowGraphCanvasHost` (1,126 lines, one component with 89 hooks) are each a single unit of that size. Every child stays under 2,000 lines, but these two bodies need their own decomposition work items.

