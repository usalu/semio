# Renderer Parity Acceptance Matrix

The React renderer is the behavioral and visual reference. A previous source patch or an old green test report does not establish current parity. Each row needs current implementation evidence and the relevant automated/runtime check. Shared schema contradictions must be resolved consistently in both implementations, rather than adding a wgpu compatibility layer.

| Area | Required behavior | Current acceptance status | Owner / next evidence |
| --- | --- | --- | --- |
| Build and contracts | UI engine and OS renderer compile natively and for browser wasm; relevant language-neutral and renderer suites pass | React quick suite 5/5 reported; native tests running | Sol baseline |
| Window lifecycle | Close, reopen, split, tab, focus, maximize, resize and cancel; an empty dock can receive a new window | Fresh audit identifies empty-dock reopen and stale closed-world authority defects | Sol windows; neutral fixture plus shell/dock laws |
| Display template drag | Drag a window/projection template into each tab/split zone or the empty root; show matching preview and seed its configuration | Current wgpu substitutes fixed-right insertion | Sol windows |
| Retained Select | Popup bounds, scroll chevrons, keyboard selection, hit geometry, clipping and foreground text match React | Production retained painter still differs from the shared Select renderer | Sol Select/overlay |
| Modal and popup compositing | Images, text, vectors and controls appear above their own surface with correct clipping; outside content remains behind | Overlay raster routing defect confirmed by current source audit | Sol Select/overlay |
| Authored layout | Stack, grid, overlay, absolute and scroll semantics agree with the owned contract and browser layout | React ContainerView overrides declared positioning; decision pending bounded paired tests | Sol baseline follow-on |
| Shell geometry and styles | Navbar cluster alignment, cap heights, active/hover/focus styling, panes, all panel anchors, footer and utility controls match at equal viewport/density | Existing wasm baseline visibly differs; fresh build needed before scoring | Next Terra visual/geometry audit |
| Scene output | Every supported scene paints; camera framing, grid, materials, references, selection and picking agree | Prior waves implemented substantial coverage; current rebuilt runtime not yet verified | Fresh scene audit and representative app boots |
| Scene input | Pointer modifiers, dragging, keyboard, wheel, context menus and cancellation produce matching actions | Prior Wave 15 code exists; current suite/runtime verification pending | Baseline suite and interaction journey |
| Accessibility | Current controls expose names, states, shortcuts and keyboard reachability | Existing wasm's mirror is empty; stale build cannot establish a current-source defect | Fresh browser DOM plus keyboard checks |
| Locale and appearance | Equal explicitly selected locale, terminology, density, appearance and theme yield matching chrome | Old comparison artifacts use default state only | Controlled en/de and light/dark comparison |
| Host and shell integrations | File I/O, configured layouts, shell commands, history/conflicts and connection lifecycle match supported React flows | Several prior reports leave runtime checks and browser backend gaps; peers are changing shared infrastructure | Reconcile current source and integration tests after main build |
| Representative apps | Window and renderer behavior holds beyond puzzle3d: graph, list/tree, map, 2D canvas, 3D, document surfaces | Previous multi-app boot wave incomplete | Fresh activation and scene-family smoke matrix |

## Current Runtime Evidence Limits

- The initial in-app browser comparison used existing wasm artifacts. It establishes that a baseline can paint puzzle3d, not that the current checkout works.
- The first two-step scripted comparison is invalid as a parity gate: React lost its development server connection during boot. Its two reported differences are not scored as renderer defects.
- Restart the owned React listener without HMR for the comparison and activate fresh inputs. Do not restart, terminate or rewrite other agents' listeners or build artifacts as cleanup.
- Reserve full interaction/pixel acceptance until both renderers finish booting from the intended builds and no activation occurs during the measured journey.
