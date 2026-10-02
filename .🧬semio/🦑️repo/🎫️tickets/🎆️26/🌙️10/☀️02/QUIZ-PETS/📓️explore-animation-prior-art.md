# Quiz Pets: prior art in the repo (read-only exploration, 2026-10-02)

Scope: what already exists that rigged, procedurally animated, cursor-aware "pets" for the quiz UI (TypeScript + Rust twins, schema-first, no external runtime libraries) must reuse, and what does not exist yet. Everything below was read in the working tree on 2026-10-02 (other agents are editing the quiz react target and the architecture site concurrently, so line numbers can drift by a few lines). Nothing was built or run; statements about behaviour come from reading source, not from execution. All paths are relative to `C:\git\semio`.

## 0. Decision-relevant summary

1. Nothing for the pets themselves exists. No skeleton, bone, rig, IK/FK, FABRIK/CCD, sprite, particle, steering, boids, behaviour tree, spring-damper, verlet, pet, mascot or creature code exists in non-ticket source (section 2). The only "skeleton/bone" hits are glTF interchange data, a loading-skeleton UI element and a causal-graph term.
2. Rust has a clean, dependency-free 2D geometry crate to build on (`semio-framework-geometry`, `Point`/`Vec2`/`Affine`/`Rect`/`BezPath`, plus `Rng` xoshiro256**), but it has no TypeScript twin, no schema, no shared fixtures, and lacks basic rig operations (`Vec2` normalize/angle/lerp, `Affine` inverse, `Rect` contains). TypeScript has no vector/matrix module at all (section 4).
3. The ANIMATE goal code is a Rust-only offline Manim-style renderer folded into one plugin crate (`✏️s/🔌️plugins/🎞️animate`). It owns the best easing catalog (`⏱️rate`) and imperative `Scene`/`Sobject`/`Updater` abstractions, but they are not reachable from a framework crate, not event-sourced, not real-time, not twinned in TS, and only endpoint-tested (section 3).
4. The quiz product already contains everything needed for the "standing on UI elements" and "state lanes" parts: the `data-presence-anchor` anchor system with scale-aware `placePeers`, the pointer feed `usePresencePointer`, a four-lane state taxonomy (`artifact`/`config`/`presence`/`transient`), the `animateIcons` preference plus `prefers-reduced-motion` gating, `{en,de}` text, and the Protocol v2 language-agnostic test layout with oracle registry (sections 6, 8).
5. Seeded RNG: bit-exact MT19937 + `uniformIndex` + `shuffle` + FNV-1a are exported by `@semio-tech/quiz` and crate `quiz` (numpy/CPython/C++ oracles), but have no float API. The Rust-only `geometry::random::Rng` (xoshiro256**, SplitMix64, distributions) has no TS twin and no reference vectors (section 5).
6. No fixed-timestep, game-loop or generic frame-scheduler abstraction exists. Closest reusable parts: `createTutorialClock` (rAF external store, not in the slim `/chrome` barrel), `ContinuationPorts` + `createVirtualContinuationHost` (injectable clock, virtual time), `🔄️machine` `Host`/`TestHost` (injectable clock for statecharts), and the quiz-wide `now?: () => number` injection convention (section 6).
7. SVG: the repo only has monochrome 24x24 stroke icons (`currentColor`, forced `stroke-width="2"` by the normalizer) and a few multi-colour logo SVGs. There is no convention for multi-part coloured artwork; the quiz renderer inlines two tiny SVGs only (section 7).
8. Decorative motion that assistive technology must ignore and reduced-motion users must not get already has two precedents: `.quiz-glyph` (aria-hidden, CSS-gated) and `PresenceOverlay` (`aria-hidden`, `pointer-events-none fixed inset-0 z-40`).

---

## 1. Repo map

### 1.1 Top-level

| Path | Purpose |
|---|---|
| `AGENTS.md` | Project rules (the rules given to all agents). |
| `README.md`, `CHANGELOG.md`, `CITATION.cff`, `LICENSE.md` | Project docs. |
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `rustfmt.toml` | Rust workspace; members include `Cargo.toml:147` (geometry), `:155` (quiz), `:157` (machine), `:163` (animate plugin), `:244` (animate presentation artifact). |
| `package.json`, `bun.lock`, `bunfig.toml`, `nx.json`, `tsconfig.json` | Bun workspaces + Nx task runner (quiz packages at `package.json:94`, `:111`, `:112`). |
| `go.work`, `CMakeLists.txt`, `CMakePresets.json`, `Monorepo.sln`, `pyproject.toml`, `uv.lock` | Go, C++, .NET, Python workspace roots. |
| `📜️script.ts`, `📋️project.json`, `🔒️dependencies.json`, `🚚️migration.json` | Root script router (the only permitted script file) and Nx project. |
| `🧰️framework/` | Domain-neutral framework: `🔨️modules/` (building blocks), `🛍️products/` (product frameworks), `📦️packages/` (TS barrel `@semio-tech/framework`, Rust facade `semio-framework`), `🛒️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits` (the only content), `🧪️tests/`. |
| `✏️s/` | "s" (semio os): `🔌️plugins/` (about 40 design-domain plugins such as `🎞️animate`, `🗄️stdio` (artifact codecs incl. `🎨️svg`, `🧊️gltf`), `🧩️puzzle`, `🌊️flow`), `🔨️modules/`, `🧑‍💻dev/`. |
| `🌎️hub/` | `os-hub` collaboration server (Rust binary): directory, auth, stores, presence and collaboration sockets. |
| `🎓️teaching/` | Teaching area: `🛂️proctor/` (Rust server), `🏛️architecture/` (site `quizzes.architektur-und-technologie.de` plus topic quizzes `⚡️energy/{🧲️physics,🔥️heating,❄️cooling,📊️demand}/❓️quiz/🔣️.json`). The pets ship on this site. |
| `🏢️semio-tech/🎡️play` | The CDN card-grid site of every app. |
| `♻️mit-bestand/` | "Mit Bestand" project deliverables: `🎤️präsentation`, `📋️bericht`, `🖼️asset`, `🧺️demonstrator`. |
| `👴️leutwiler/` | Lean proofs (`💤️realparts-of-powers-z-n`). |
| `🧪️tests/`, `🧫️fixtures/` | Repo-wide test configuration/runners (`🎚️config`, `🗿️artifact-runner`, `🦀️rust-warnings`) and Rust-warning fixtures. |
| `.🧬semio/` | Repo infrastructure: `🦑️repo/{🎫️tickets,🎯️goals,💬️prompts,...}` (this ticket is `🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS`). |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | Developer entry points; quiz entries at `🧩️launch.seed.jsonc:1074`, `:1094`, `:1197` (`🧪️test❓️quiz🟦️`), `:1208` (`🧪️test❓️quiz⚛️react`), `:1219` (`🧪️test❓️quiz🦀️`), `:1241`, `:1252`, `:2640-2676`. |
| `.github`, `.claude`, `.cursor`, `.codex`, `.agents`, `.devcontainer`, `.storybook`, `.nx`, `.cargo`, `.config`, `.repo`, `.venv` | Tooling configuration. |
| `node_modules`, `storybook-static`, `temp`, `test-results`, `__pycache__`, `activation.log` | Generated or scratch, not source. |

### 1.2 `🧰️framework/🔨️modules/` (domain-neutral building blocks)

| Module | One-line purpose (from its barrel or Cargo description) | Langs |
|---|---|---|
| `⏪️time-travel` | Non-destructive history-edit session: pure reducer, stages, replay report. | TS+Rust+schema |
| `⏯️tool-run` | Tool run lifecycle contract, step ring, trace pages. | TS+Rust+schema |
| `⏱️trace` | Browser hop-trace spans (TS) and Rust span/clock authority (`⏱️clock`, `🧮️memory`). | TS+Rust |
| `⏳️async` | Async interface layer: `CancelToken`, `OperationContext`, worker pool, `🪃️continuation` scheduler, `🥇️latest-wins`, `🔁️jittered-backoff`, `🎟️lease-pool`; `⏱️clock` installs browser `performance.now` as clock authority. | TS+Rust |
| `◻️2d` | 2D drawing scene contracts, canvas raster, SVG export (TS); planar booleans, bitmap trace, `PathSegment` (Rust, depends on `os-kernel`). | TS+Rust |
| `⚠️diagnostic` | Text errors, spans, parse limits. | Rust |
| `✍️editor` | Canvas-agnostic editor state machine (selection, transforms, undo). | Rust |
| `🌉️abi` | Owned byte/message ABI. | Rust+schema |
| `🌱️value` | `DslValue` dynamic value, ordered/list/paged containers, codec. | Rust+schema |
| `🎒️pack` | Binary document format family. | Rust+schema |
| `🎠️kernel` | Plugin runtime, leases, invocation responses, playground boot; `ephemeralMap` (`🟦️.ts:96`). | TS+Rust |
| `🎭️actor` | Pooled actor/plugin runtime kernel (scheduler, mailbox, failure ladder). | TS+Rust |
| `🎯️action-bus` | Utility/tool derivation helpers. | TS+Rust |
| `🏗️mesh-engine` | Mesh data, primitives, Obj/Glb/Stl codecs. | Rust |
| `📏️intrinsic-size` | PNG/JPEG/GIF/WebP/SVG intrinsic-dimension reader. | Rust |
| `📐️geometry` | First-party 2D geometry + `Vec3`/`Mat4` + seeded `Rng` (Cargo description: "`kurbo` is a differential-test oracle only"). | Rust only |
| `📚️compiler` | Typst replacement: math/text typesetting to SVG. | Rust |
| `📡️replication` | Replication wire contract, peer overlay (`👕️peer-overlay`, ephemeral shared), codecs. | TS+Rust+schema |
| `🔀️dispatch` | `dyn_enum` macros (drop dyn dispatch). | Rust |
| `🔄️machine` | Statechart kernel (XState-parity), actor runtime, hosts. | TS+Rust |
| `🔏️hash` | First-party BLAKE3. | TS+Rust |
| `🔢️number` | Bigint, rational, interval arithmetic, algebra traits. | Rust |
| `🔤️typeset` | Typst to SVG and SVG (`usvg`) to `BezPath`, behind a first-party interface. | Rust |
| `🔲️pixels` | Owned PNG/zlib codecs. | Rust |
| `🕸️graph` | Graph vocabulary, algorithms, force-directed layout, Jack DSL. | Rust |
| `🕹️interaction` | Hover/selection state machine; `👆️gesture` pointer/pinch math. | TS+Rust+schema |
| `🖌️raster` | Headless vello rasterization of `BezPath` scenes; first-party video tier. | Rust |
| `🖥️platform` | Element ids, presence, dock/pane persistence, inspector helpers. | TS+Rust |
| `🖱️ui` | Design system: React/wgpu/tui targets, tokens (`🎨️styling`), i18n, `🎬️scene`, `🔨️modules/🥞️layered-overview-geometry`, `🔨️modules/👥️presence-presentation`. | TS+Rust |
| `🖼️assets` | Icons, logos, fonts, cursors, lists (CC BY-ND 4.0 `LICENSE.md`). | TS+gen |
| `🗜️deflate` | Raw DEFLATE. | Rust |
| `🗺️surface` | Paint, board-2d, terrain, node-graph, tiled-map sessions. | Rust+schema |
| `🚪️io` | Dialect vocabulary, base64, fetch timeout, stream mux. | TS+Rust |
| `🛂️manifest` | Plugin manifests, app definitions, declarative UI contract. | TS+Rust |
| `🛠️tool-machine` | Tool transaction machines over `🔄️machine`. | TS+Rust |
| `🧊️3d` | B-Rep, half-edge mesh, BVH, `🌀️rigid` (f32 isometries), `🧿️collision`. | TS+Rust |
| `🧩️action-argument-resolution` | Action argument resolution. | TS |
| `🧬️schema` | Artifact schema descriptors, state lanes (`StateClass`). | TS+Rust |
| `🧮️math` | **Not general math**: an LLM token-sampling/diffusion engine, "misfiled here" per its own `Cargo.toml` description. | Rust |
| `🧵️job` | Resumable `InteractiveJob` protocol (bounded steps, cancel, progress). | Rust |

### 1.3 `🧰️framework/🛍️products/`

| Product | Purpose |
|---|---|
| `❓️quiz` | Render-independent quiz model (catalog, quiz, tasks, seeded sheets, scoring, badges, learner lifecycle, presence), TS core `@semio-tech/quiz` + Rust core `semio-framework-quiz` bit-for-bit, React target `@semio-tech/quiz-react` (`README.md:1-8`). |
| `🎤️presentation` | Declarative presentation model + React/reveal.js renderer. |
| `💻️os` | Cooperative pseudo operating system (plugins, artifacts, renderers, kernels, db, shell). |
| `📓️print` | LaTeX stack + grammar-of-graphics visualization library (`📊️viz-kernel`, TS, d3-parity). |
| `🖥️server` | Authoritative server product (commands/queries/events, policy, presence sockets `semio.presence.v1`). |
| `🦑️repo` | Repo infrastructure: library, test platform (Protocol v2 at `🔨️modules/🧪️test`), goals, tickets, MCP. |

---

## 2. Searches by family (source only; tickets and `node_modules` excluded)

| Family | Hits (path:line) | What it is | Lang |
|---|---|---|---|
| skeleton / bone / rig / joint / armature / IK / FK / FABRIK / CCD | none as runtime code. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔺️diff/🦀️.rs:2570-2585` (`skeleton`, `joints`, `inverse_bind_matrices` fields of a glTF `skin`), `.../🏭️generator/📜️script.ts:94-108` (builds a `THREE.Bone`/`Skeleton` fixture), `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🟦️.tsx` (loading skeleton placeholders), `✏️s/.../📊️table/🧬️schema/🧭️causal-internals/🦀️.rs:223,258,342` (causal "skeleton"). "rig" appears only as "orbit rig" (`🕹️interaction/👆️gesture/🟦️.ts:6`). | interchange data / UI placeholder / causal term | Rust, TS |
| keyframe / easing / tween / lerp | `✏️s/🔌️plugins/🎞️animate/.../⚙️engine/⏱️rate/🦀️.rs:13-300` (full easing catalog), `:309` `map_child_alpha`; private lerps `.../🎬️scene/🦀️.rs:722` `lerp_f64`, `:833` `lerp_affine`, `:763` `interpolate_path_sets`, `.../🎞️animation/🦀️.rs:1077` `lerp_point`. TS: `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🥞️layered-overview-geometry/🟦️.ts:139` `easeInOutCubic`, `:144` `lerpOffset`, `:149` `glideOffset`, `:155` `followStep`, `:133` `LAYERED_FOLLOW_LERP = 0.12` (per-frame, not dt-based), `:247` `lerpRect`, `:252` `glideRect`. Duplicates: `🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:1358` `easeInOutCubic`, `:1362` `lerpVec3`; `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:3062` `introductionDemoLerp`. Colour blend: `🖱️ui/🎨️styling/🌓️theme/🟦️.ts:196`. Build-time keyframes: `🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts:132-214` (SMIL `⚡️animated.svg` from 6 keyframe SVGs). Quiz CSS keyframes `quiz-icon-{bounce,pulse,spin,sway,float,flip}` at `❓️quiz/🎯️targets/⚛️react/🎨️.css:74-160`. | easing, interpolation | Rust (catalog), TS (one cubic), CSS |
| spring / verlet / physics / integrators | none as reusable runtime. Force layout with Hooke/Fruchterman-Reingold springs and velocity damping (`velocity_damping = 0.88`): `🕸️graph/🖊️drawing/🦀️.rs:76-116`, `🕸️graph/⏯️layout-run/🦀️.rs:400-468,1207` (graph-specific, Rust). Test-only velocity-Verlet and RK4 reference: `📓️print/🧪️tests/🏹️physics-projectile-rk4/🟦️.ts:25,113-123`. 3D rigid algebra and collision: `🧊️3d/🌀️rigid/🦀️.rs:13-260`, `🧊️3d/🧿️collision/🦀️.rs` (BVH triangle meshes, f32, 3D). | n/a | Rust |
| steering / boids / flocking / behaviour tree | none (only the word "steering" for plugin dispatch options, `🎠️kernel/🟦️.ts:280`). | n/a | n/a |
| state machine | `🔄️machine/🟦️.ts` (1156 lines) and `🦀️.rs` (1783 lines): see 6.4. `🕹️interaction/🟦️.ts:202,267` (`nextSelection`, `nextHover`). `🛠️tool-machine`, `⏪️time-travel`. Quiz: `evolveQuizState` `❓️quiz/🎯️targets/⚛️react/🔨️modules/🧭️session/🟦️.ts:113`. (`xstate ^5.25.0` is a dependency of `@semio-tech/ui-react`; external, not for runtime reuse.) | statechart kernel, reducers | TS+Rust |
| sprite | only three.js sprites in 3D hosts (`💻️os/.../🌐️World3dHost/🟦️.tsx:3809` point sprites, `🎨️r3f/🟦️.tsx:1556`) and a sprite-grid `background-position` helper in `🎤️presentation/.../🟦️.tsx:1404`. No 2D sprite/atlas system. | 3D / CSS helper | TS |
| character / mascot / avatar / pet / creature / companion | none. "avatar" is a user picture field (`📡️replication/🔗️causal/🔀️transition/🦀️.rs:26`). Quiz task glyphs are emoji (`TaskGlyph`, `❓️quiz/🎯️targets/⚛️react/🟦️.tsx`; contract `🧬️schema/🔣️.json:53-57`). Fonts `🖼️assets/🔤️fonts/😀️noto-emoji`; `🖼️assets/📃️list/🦊️animals.json` is a word list. | n/a | n/a |
| particle | none (GPU point-sprite layers of World3d only). | n/a | n/a |
| collision / AABB / quadtree | 2D: `📐️geometry/⚙️engine/🦀️.rs:1256-1440` (`geom_sel::WorldBox`, `world_boxes_overlap`, `world_box_contains_point`, `point_in_polygon :1304`, `segment_intersects_world_box :1358`), `:1135` `segment_intersection`, `:1151` `circle_line_intersections`, `:1179` `convex_hull`, `:1211` `polygon_area`. 3D: `🧊️3d/🟦️.ts:211` `Aabb`, `🧿️collision`. Quadtree appears only as the d3-quadtree oracle test `📓️print/🧪️tests/🌳️spatial-quadtree/🟦️.ts`. TS spatial (data-viz): `📓️print/🔨️modules/📊️viz-kernel/📍spatial/🟦️.ts:32-167` (Delaunay, hull, Voronoi, polygon area). | geometry queries | Rust (2D), TS (viz) |
| frame loop | section 6. | | |

Third-party libraries already installed that are usable as differential-test oracles (never as runtime dependencies): `three@0.182.0` incl. `three/examples/jsm/animation/CCDIKSolver.js` and `three-stdlib/animation/CCDIKSolver.js`, `gl-matrix@3.4.3` (declared in `📓️print/📦️packages/🟦️typescript/package.json:49`), `d3-ease@3.0.1` (declared in the ui-react package `:64`, already used as oracle in `🥞️layered-overview-geometry/🧪️tests/🔬️unit/🟦️.ts:158`), `d3-interpolate@3.0.1`, `d3-force`, `d3-quadtree`, `d3-path` (print package), `seedrandom@3.0.5`, `fast-check@3.23.2`, `xstate@5.32.5`, `motion@12.40.0`, `mathjs@14.0.0` and `jstat@1.9.6` (quiz package devDependencies), `kurbo@0.13.1` (geometry Rust dev-dependency), numpy/scipy/jsonschema (Python oracles of the quiz). Declaring them in a new package's `devDependencies`/`[dev-dependencies]` is still required.

---

## 3. The ANIMATE goal

Goal: `.🧬semio/🦑️repo/🎯️goals/ANIMATE/🎯️goal.json` ("Manim-class Rust animation compiler: Sobject scene graph, imperative Scene timeline, headless video and present engines"; open, due 2026-12-31, milestone 80).

### 3.1 Where the code lives

`✏️s/🔌️plugins/🎞️animate/` is a plugin (WASM component + rlib):

- Plugin crate `semio-s-plugin-animate` at `📦️packages/🦀️rust` (`Cargo.toml:489`), artifact crate `semio-s-artifact-animate-presentation` at `🗿️artifacts/🎬️presentation/📦️packages/🦀️rust` (`Cargo.toml:244,311`), TS package `@semio-tech/animate-js` at `📦️packages/🟦️typescript` (React + reveal.js + pdfjs presentation renderer, not an engine).
- The "animate/core, animate/video, animate/present" crates named in `AGENTS.md` (`✏️s/🔌️plugins/🎞️animate/AGENTS.md`) no longer exist as crates: they were folded into one plugin as engine topic files (`⚙️engine/🦀️.rs:1-14` module doc). Engine root: `.../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/` with `⏱️rate` (440 lines), `🎛️config` (352), `🎞️animation` (2424), `📷️camera` (276), `🎬️scene` (1239), `📐️geometry` (798), `🔤️text` (366), `🎥️video` (1224), `🦀️.rs` (475), 7594 lines in total. Modules are addressed as `crate::editor::animate::engine::<topic>::<topic>`, so they are only reachable from inside the plugin crate.
- Tickets: closed `ANIMATE-CORE-RUST-LIBRARY`, `ANIMATE-VIDEO-HEADLESS-ENGINE`, `ANIMATE-ABSOLUTE-MANIM-FEATURE-COMPLETENESS`, `ANIMATE-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`; open `EXTEND-ANIMATE-CORE-MOBJECT-CATALOGS`, `RUST-CLEAN-REFACTOR-WAVE-12-ANIMATE-CORE-VIDEO-PRESENT-PLUGIN` (all under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️18|19` and `🌙️08/☀️05`).
- Tests: 12 unit directories (`⏱️rate/🧪️tests/{🔬️rate-unit,🔬️updater-unit}`, `🎞️animation`, `🎬️scene`, `📐️geometry`, `🧪️tests/{🔬️compiler-unit,🔬️slide-unit}`). `🔬️rate-unit/🦀️.rs` has 18 `fn`s and checks endpoints (`f(0)=0`, `f(1)=1`) and `map_child_alpha`; no oracle (no d3-ease or Manim vectors).

### 3.2 Public API of the most reusable pieces (Rust)

```rust
// ⏱️rate/🦀️.rs
pub type RateFunc = fn(f64) -> f64;                                  // :13
pub fn linear|smooth|rush_into|rush_from|slow_into|double_smooth|there_and_back|running_start|lingering(t: f64) -> f64
pub fn there_and_back_with_pause(t, pause_ratio) -> f64              // :57
pub fn wiggle(t, num_wiggles) -> f64                                 // :75
pub fn exponential_decay(t, half_life) -> f64                        // :85
pub fn ease_{in,out,in_out}_{sine,quad,cubic,quart,quint,exp,circ,back,elastic,bounce}(t) -> f64   // :90-300
pub fn map_child_alpha(parent_alpha, start, end) -> f64              // :309
pub struct ValueTracker; pub struct Updater;                         // :332, :357
pub fn add_updater|always|f_always|always_redraw|run_updaters        // :381-430
// 🎬️scene/🦀️.rs
pub trait Scene { fn construct(&mut self); fn add(&mut self, Sobjects); fn remove(&mut self, u64);
                  fn play(&mut self, Animations); fn wait(&mut self, f64); fn sample_frame(&mut self, dt: f64); ... }   // :19-109
pub fn preview_scene_loop<S: Scene>(scene: &mut S, max_frames: u64, on_frame: impl FnMut(&SceneFrame))             // :121
#[dyn_enum] pub trait Sobject: Send { fn transform(&self) -> Affine; fn shift(&mut self, Vec2); fn scale(&mut self, f64);
                  fn rotate(&mut self, f64); fn bounds(&self) -> Bounds; fn paths(&self) -> Vec<BezPath>;
                  fn updaters(&self) -> &[Updater]; fn save_state/restore/generate_target/apply_target ... }         // :565-631
pub struct VSobject { paths: Vec<BezPath>, style: Style, transform: Affine, ... }  // :635 (+ interpolate_snapshots :697)
pub fn trim_path_at_ratio(path: &BezPath, ratio: f64) -> BezPath     // :727
// 🎞️animation/🦀️.rs
pub trait Animation: Send { ... }                                    // :27
Create, FadeIn, FadeOut, Transform, Rotate, MoveAlongPath(CubicBez), Shift, AnimationGroup, Succession, LaggedStart, Blink, Wiggle, Flash, ... // :74-2242
```

### 3.3 Verdict on reusability for pets

| Aspect | State |
|---|---|
| Vector math, transforms, bezier | Not owned here: the engine imports `semio_framework_geometry::{BezPath, Affine, Point, Vec2, ...}` (`🎬️scene/🦀️.rs:516`) and adds private extension traits (`PointVec2::to_vec2` `:1227`). |
| Easing / rate functions | Complete catalog, Rust only, `fn(f64)->f64`, no TS twin, plugin-private path, endpoint tests only. |
| Interpolation | `lerp_f64`, `lerp_affine` (componentwise on the six coefficients, not decomposed), `interpolate_path_sets` (resamples to 32 points) are private functions. |
| SVG path output | None in the animate engine (video path rasterizes through `🖌️raster`/vello; text goes Typst to SVG to `usvg` to `BezPath` in `🔤️text/🦀️.rs:294-360`). SVG `d` serialization lives elsewhere: TS `◻️2d/🟦️.ts:278` `pathSegmentsToSvgD`, Rust only in the stdio SVG artifact (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:418` `parse_path_data`, `:526` `path_data_to_string`, `:325` `parse_transform_list`, `:227` `Matrix2D`). |
| Execution model | Offline, imperative: `Scene::play` loops `ceil(duration*fps)` frames synchronously (`🎬️scene/🦀️.rs:55-74`); `HashMap<u64, Sobjects>` scene graph; `async_fn_in_trait` + `dyn_enum` conventions of the plugin crate; not event-sourced, no cancellation or progress, no real-time clock. |
| Observation | `next_id()` (`🎬️scene/🦀️.rs:519-521`) hashes `concat!(file!(), line!())` evaluated inside its own body, so it appears to return the same id on every call unless ids are set explicitly. |
| Framework reach | A framework crate may not depend on a plugin (the `📐️geometry` Cargo description states framework crates "may name without reaching a plugin"). Reusing `⏱️rate` requires relocating it to a framework module, as was done for `🖌️raster` ("Relocated from `🎞️animate`'s `⚙️engine/🎥️video`", `🖌️raster/🦀️.rs:6`). |

---

## 4. Shared math and geometry

### 4.1 Rust: `semio-framework-geometry` (the only domain-neutral candidate)

Files: `🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️.rs` (1666 lines), `🎲️random/🦀️.rs` (323), package glue `📦️packages/🦀️rust/🦀️.rs` (10 lines, `#[path]` wiring), `Cargo.toml` (lib `semio_framework_geometry`, `[dependencies]` empty, `kurbo = "0.13.1"` dev-dependency oracle). Types are plain values on every target including `wasm32-wasip2` (`⚙️engine/🦀️.rs:4`). Pure functions are sync.

API (all `f64` unless noted; `⚙️engine/🦀️.rs` line numbers):

```rust
pub struct Point { pub x: f64, pub y: f64 }                // :75  ZERO, new, distance, x(), y(); Point+Vec2, Point-Vec2, Point-Point -> Vec2
pub struct Vec2  { pub x: f64, pub y: f64 }                // :120 ZERO, new, hypot, dot, x(), y(); +,+=,-,-=,*f64,f64*,/f64,*=,neg, From<(f64,f64)>
pub struct Affine { pub(crate) coeffs: [f64; 6] }          // :221 IDENTITY, new([a,b,c,d,e,f]), translate(Vec2), scale(f64), rotate(angle), as_coeffs(); Affine*Affine (:255), Affine*Point (:273)
pub struct Rect  {x0,y0,x1,y1}                             // :282 new, from_points, inflate, x0()/y0()/x1()/y1(), width(), height(), path_elements
pub struct RoundedRect / RoundedRectRadii / Circle / Line / Arc / CubicBez   // :330-511  (eval(t), path_elements(tol))
pub enum PathEl { MoveTo, LineTo, QuadTo, CurveTo, ClosePath } // :517
pub struct BezPath                                         // :526 new, move_to, line_to, quad_to, curve_to, close_path, push, elements, bounding_box (tight), path_segments, apply_affine :631
pub enum PathSeg                                           // :692 start, end, eval(t), subdivide_at, subsegment, as_path_el, arclen(accuracy), tight_bounds
pub fn clamp_f64 :987, distance_between :992, normalize_or_zero(Vec2) :997, ray_from_origin_to_axis_aligned_rectangle_edge :1006,
       distance_point_to_polyline :1028, distance_point_to_cubic_bezier :1055, cubic_point_at :1084, cubic_split :1093, cubic_arc_length :1105,
       cubic_nearest_t :1119, segment_intersection :1135, circle_line_intersections :1151, convex_hull :1179, polygon_area :1211, polygon_centroid :1225, bounding_box :1250
pub mod geom_sel { WorldBox {min_x,min_y,max_x,max_y}, inflate_world_box, world_boxes_overlap, world_box_contains_point/_box, point_in_polygon :1304,
                   segment_intersects_world_box :1358, polygon_*_world_box, cubic_bezier_axis_bounds :1426 }   // :1256
pub struct Vec3 (f32) :1464  new, from_array, to_array, add, sub, scale, dot, cross, length, normalize
pub struct Mat4 (f32) :1523  identity, perspective, look_at, mul, transform_point, transform_direction, inverse, translation, scale_vec, from_quat, to_cols_array
```

Gaps a rig needs (verified absent by search): `Vec2` length alias/normalize/atan2/angle/perp/cross/lerp, `Point::lerp`/`to_vec2`, `Affine` inverse/determinant/rotate-about/`Mul<Vec2>`, `Rect` contains/intersects/center/union, 2D rotation/angle-wrap helpers. `🖱️ui/🎬️scene/📐️math/🦀️.rs` re-exports `Mat4`/`Vec3` and adds `Vec3Math`/`Mat4Math` (3D only); `🧊️3d/🌀️rigid/🦀️.rs` has f32 `Vector3`/`Point3`/`Quaternion`/`UnitQuaternion`/`Isometry3` but derives `value_derive::{ToValue, FromValue}` against `::protocol::value` (heavy for a 2D pets crate).

Tests/oracle: `📐️geometry/🧪️tests/📐️first-party-geometry/🦀️.rs` (5 tests, differential against `kurbo` with a local LCG), `⚙️engine/🧪️tests/{🔬️algebra,🔬️first-party-shape,🔬️path-seg,🔬️unit}` (15+12+11+12 tests). No `🧬️schema`, no `🧫️fixtures`, no TS twin, no `🥒️.feature` case.

### 4.2 TypeScript: nothing equivalent

- `◻️2d/🟦️.ts:11` `type Vec2 = readonly [number, number]`; `:22` affine as a 6-tuple (`SceneNode.transform`); `:40-54` `PathSegment` union (`move|line|quad|cubic|arc|close`); `:119` `applyTransform`; `:125` `nodePath`; `:205` `paintDrawingScene(ctx, scene, {clear})` (canvas); `:278` `pathSegmentsToSvgD`; `:292` `drawingSceneToSvgMarkup`. This is the only TS "scene to SVG/canvas" code and is a contract file, not a math library (no vector operations).
- `🧊️3d/🟦️.ts:8` `Vec3`, `:211` `Aabb` (types only).
- `🕹️interaction/👆️gesture/🟦️.ts`: pinch math with `shortestAngleDelta :123`, `pinchStep :137`, `zoomAboutPoint :299` (pure, ephemeral-local, documented as never crossing a wire, `:11-17`).
- `🖱️ui/🔨️modules/🥞️layered-overview-geometry/🟦️.ts`: `LayeredRect`, `lerpRect`, `glideRect`, polygon veil clipping (TS only, schema `🧬️schema/🥞️layered-overview/🔣️.json`, fixtures, d3-ease and polygon-clipping oracles in its test).
- Quiz radar: `radarAngle`, `radarPoint`, `radarPolygon`, `radarLayout` (`❓️quiz/🎯️targets/⚛️react/🔨️modules/🕸️radar/🟦️.tsx`, re-exported at `🟦️.tsx` reexports block).
- `📓️print/🔨️modules/📊️viz-kernel/📍spatial/🟦️.ts`: Delaunay/Voronoi/hull/polygon area for data viz; `🧭coordinate`, `🧮transform` (statistics), not animation math.
- `colord`, `d3-*`, `gl-matrix`, `three` exist only as dev/test or non-quiz dependencies.

### 4.3 Is there a domain-neutral geometry/math module both cores can depend on?

Rust: yes, `📐️geometry` (framework tier, zero dependencies, wasm-safe, RNG included), but TS has no twin, so it cannot be "the shared module" yet; a TS twin plus a schema + shared fixtures (the pattern of `🕹️interaction`, `⏪️time-travel`, `🔄️machine`) would have to be created. `🧮️math` is not it (LLM sampling). `◻️2d` (Rust) is not a good dependency because its crate pulls `semio-framework-os-kernel` and `semio-framework-hash` (`◻️2d/📦️packages/🦀️rust/Cargo.toml`).

---

## 5. Seeded RNG: all implementations

| Implementation | Where | API | Exported / reusable? |
|---|---|---|---|
| **MT19937 + FNV-1a** (quiz) TS | `❓️quiz/🔨️modules/🎲️randomness/🟦️.ts` | `fnv1a32(text):number :19`, `runSeed(run):number :26`, `interface RandomSource { next():number } :31`, `class Mt19937(seed) implements RandomSource :36` (`next()` tempered u32), `uniformIndex(random, n):number :70` (rejection sampling, no draw for `n<=1`), `shuffle(random, items):T[] :80` (Fisher-Yates from the end, copy) | Yes: barrel `export *` in `❓️quiz/📦️packages/🟦️typescript/🟦️.ts` (`@semio-tech/quiz`). No float API, no range, no snapshot/restore of state. |
| **MT19937 + FNV-1a** (quiz) Rust | `❓️quiz/🔨️modules/🎲️randomness/🦀️.rs` | `fnv1a32(&str)->u32 :17`, `run_seed :22`, `Mt19937::new(u32) :35`, `next_u32 :46`, `uniform_index(&mut Mt19937, usize)->usize :70`, `shuffle<T: Clone>(&mut Mt19937, &[T])->Vec<T> :85` | Yes: `pub use crate::randomness::*` in `❓️quiz/🦀️.rs`, crate `quiz`. `uniform_index`/`shuffle` take the concrete `Mt19937` (no trait, unlike TS). |
| Oracle evidence | `❓️quiz/🧪️tests/🎲️seeded-randomness/{🥒️.feature,🟦️.ts,🦀️.rs,🐍️.py}`, `🧫️fixtures/🎲️seeded-randomness/🔣️.json`, `🧪️tests/🌀️mt19937-generator/🟦️.ts` (numpy 2.4.3 vectors, C++ `[rand.predef]` check value 4123659995), oracle ids `quiz-numpy-mt19937` and `quiz-python-reference` in `❓️quiz/🔮️oracles/🔣️.json` | Vectors generated by `.🧬semio/.../QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py` (`mt19937-numpy-vectors.py`). | |
| **xoshiro256\*\*, SplitMix64, AliasTable, distributions** | `🧰️framework/🔨️modules/📐️geometry/🎲️random/🦀️.rs` | `SplitMix64::new/next_u64 :6-15`, `Rng::from_seed(u64) :42`, `next_u64 :52`, `next_f64 :65` (53-bit), `next_range(lo,hi) :73` (rejection), `next_bool(p) :89`, `shuffle :94`, `choose :103`, `state/from_state :112/:119` (snapshot/restore), `sample_without_replacement :127`, `AliasTable :146/:156/:203`, `normal :218`, `geometric :227`, `poisson :239`, `powerlaw_sequence :256`, `zipf :269`, `discrete_sequence :290`, `cumulative_distribution :301` | Yes, `semio_framework_geometry::random` (and used by `🧮️math/🎯️sampling` via `XoshiroSource`). **Rust only; no TS twin; 30 tests are determinism/property tests with no reference vectors** (`🎲️random/🧪️tests/🔬️unit/🦀️.rs`). |
| `RandomSource` trait (async), `CounterRng` (Philox-lite), `XoshiroSource` | `🧮️math/🎯️sampling/🦀️.rs:1118,1163,1205` | LLM token sampling | Domain-specific, do not reuse. |
| TS d3 LCG | `📓️print/🔨️modules/📊️viz-kernel/🌳hierarchy/🟦️.ts:11` `vizLcg(seed=1)` | `s ← (1664525·s + 1013904223) mod 2³²` | Exported from print viz-kernel for d3 parity only. |
| Private TS PRNG copies | `💻️os/🧫️fixtures/⚖️scale/📽️projection/🟦️.ts:82` mulberry32; `💻️os/.../🧪️tests/🎯️input-ledger/🟦️.ts:246` mulberry32; `💻️os/🧪️tests/🔤️pack-key-order/🟦️.ts:23` xorshift32; `...🌐️World3dHost/⏯️tool-run-trace/🧪️tests/🧩️component/🟦️.ts:58` lcg | test fixtures | Private. |
| Private Rust PRNG copies | many test-local `Lcg`/`SplitMix64` (e.g. `📐️geometry/🧪️tests/📐️first-party-geometry/🦀️.rs:3`, `🚪️io/🔤️base64/🧪️tests/🔬️unit/🦀️.rs:37`, `🖌️raster/🧪️tests/🔬️unit/🦀️.rs:5`), `✏️s/🔌️plugins/🀄️wfc/⚙️engine/💼️job/🦀️.rs:134` `JobRng`, `✏️s/.../📊️table/🧬️schema/🌀️entropy-internals/🦀️.rs:588` `Xorshift64`, `🧵️job/🦀️.rs:3440` | | Private. |
| Python | numpy MT19937 in the quiz oracles; `seedrandom@3.0.5` is installed for JS | | Test-only. |

The quiz client already derives per-run randomness from `runSeed(runId)` (README "Sheet"), so a pet "personality/seed" can come from the same FNV-1a seed or from `learnerTag`/`presence` ids without any new primitive.

---

## 6. Frame loop, clocks, scheduling, state lanes, event sourcing

### 6.1 No generic game loop

Search for accumulator / fixed-step / delta-time / frame-budget found no shared abstraction (hits are unrelated byte accumulators). Frame-driven code is ad hoc:

| Where | What | Notes |
|---|---|---|
| `❓️quiz/🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx:971-998` `usePlacing` | rAF-coalesced re-layout of overlay marks on `resize`, capture `scroll`, `focusin`, plus a 500 ms (250 ms for panes) `setInterval` | Overlay positions peers on anchors; `placePeers :918` writes `transform: translate(...)`. |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:4013-4078` `createTutorialClock(durationMs)` | rAF external store: `play/pause/seek/setRate/dispose`, `TutorialClockPort {getTimeMs, subscribe}` `:3996`, `useTutorialClock :4081` (via `useSyncExternalStore`) | In the full `@semio-tech/ui-react` barrel only, not in `/chrome` (`🪟️chrome/🟦️.ts`). Not dt-clamped. |
| same file `:3078` `IntroductionDemonstrationOverlay` | imperative rAF ghost-cursor loop, "no per-frame React state", hides on real pointer movement, honours `prefers-reduced-motion` (`:3097`) | Closest UI analogue to a pet overlay. `useIntroductionPointerIdle :2914`, `INTRODUCTION_DEMO_IDLE_THRESHOLD_MS = 1600 :2894`. |
| `🖱️ui/🧱️elements/🎬️Scene/🟦️.tsx:1834-2007` | camera glide loops via rAF | 3D. |
| `🥞️layered-overview-geometry/🟦️.ts:128-165` | pure time-based `glideOffset(from,to,elapsedMs,durationMs)` and per-frame `followStep` (exponential smoothing constant, not dt-correct) | Pure, unit-testable, TS only. |

### 6.2 Clock injection and virtual time (the testing seam)

- Quiz convention: `now?: () => number` defaulting to `Date.now` (`🧭️session/🟦️.ts:275-281,369`, `📮️outbox/🟦️.ts:143,290`, `🛂️proctor/🟦️.ts:342`). Presence uses raw `Date.now()` (`👥️presence/🟦️.tsx:226,348,362`). `QuizOptions` (`❓️quiz/🎯️targets/⚛️react/🟦️.tsx:173-183`) injects `transport`, `presence`, `storage`, `languages`, `timing` but no clock or scheduler yet.
- `⏳️async/🪃️continuation/🟦️.ts`: `ContinuationPorts {postMacrotask, setTimer, clearTimer, now}` `:42-51`; `defaultContinuationPorts` `:92` (`performance.now` with `Date.now` fallback); `ContinuationScheduler` `:104`; `createVirtualContinuationHost() -> {ports, nowMs, drain(untilMs?, stepBudget?), idle}` `:265-330` (deterministic virtual clock for tests). Module doc `:24-29` states `requestAnimationFrame` does not fire in hidden tabs and must not carry evaluation progress; only paint may stop. Not re-exported from the `@semio-tech/framework` barrel (`🧰️framework/📦️packages/🟦️typescript/🟦️.ts` exports `createLeasePool`, `retryWithJitteredBackoff`, `latestWins`, `waitForEvent`, `fetchWithTimeout`, `🔄️machine`, ...).
- `🥞️layered-overview-geometry/🟦️.ts:342-360` `LayeredIdleScheduler` / `scheduleIdle(callback, delayMs, scheduler)` (injectable `setTimeout`/`requestIdleCallback`), re-exported through `@semio-tech/ui-react/chrome`.
- Rust: `⏳️async/⏱️clock/🦀️.rs` installs `performance.now` as the `semio_framework_trace` clock authority (`install_clock`); `🧵️job/⏱️budget` binds a testable clock (`bind_clock`, `⏱️budget/🧪️tests/⏱️budget/🦀️.rs:17`).
- `🔄️machine`: `Host.nowMs()` / `Host::now_ms` and delayed transitions (`after`), `TestHost.advance(ms)` (`🟦️.ts:625,708-773`; Rust `Host :15`, `NativeHost :38` (Instant), `TestHost :122`).

### 6.3 State lanes: ephemeral local-only / ephemeral shared / persisted

Repository-wide taxonomy of exactly four lanes:

- `🧰️framework/🔨️modules/🧬️schema/🟦️.ts:44-48`: `STATE_CLASSES = ["artifact","config","presence","transient"]` with `artifact` = persisted shared, `config` = persisted local-only, `presence` = ephemeral shared, `transient` = ephemeral local-only; `x-semio-state` JSON-schema key; GraphQL `@state` `:38-42`. Rust `StateClass` (re-exported `🧬️schema/⚛️component/🦀️.rs:7`, kebab helpers `:775-798`, validator `:755`).
- Quiz table `❓️quiz/README.md:312` ("State classes"): persisted shared = proctor event store; persisted local-only = browser (learner id, locale, theme, outbox, cached views); ephemeral shared = leaderboard poll, presence, cursors, drafts; ephemeral local-only = renderer (drag state, focus, current step).
- Persisted local-only in code: `QuizPreferences {locale, theme, textSize, showCursors, showAnswers, animateIcons}` (`🎯️targets/⚛️react/🔨️modules/🎛️preferences/🟦️.tsx:42-49`, `readPreferences :52`, `writePreferences :66`), stored through `LocalStore` slices `"introduced" | "learner" | "catalog" | "learner-view" | "preferences"` and collections `"outbox" | "runs"` (`💾️persistence/🟦️.ts:110-113`), cross-tab merge via `StorageArea.watch`.
- Ephemeral local-only: pure fold `evolveQuizState(state, event)` (`🧭️session/🟦️.ts:113`) over `QuizClientEvent` (`:79-93`) with `QuizSession.subscribe/getSnapshot` read by `useSyncExternalStore` (`🟦️.tsx:499`); `🧭️session/🟦️.ts:1-17` states the lifetimes explicitly. `🕹️interaction/👆️gesture/🟦️.ts:11-17`: per-frame gesture state is ephemeral local and "belongs in window TRANSIENT state", never in the schema, only the end result commits.
- Ephemeral shared: presence sockets `semio.presence.v1` (`🖥️server/🟦️.ts:204-239`: frames `welcome|state|batch|refused|watch|watched`, `PresenceEntry {session, colour, surface, state: unknown}`); quiz core owns rooms/admission (`❓️quiz/🔨️modules/👥️presence/🟦️.ts:38-69,74`), `PRESENCE_FRAME_HZ = 15 :57`, thinking frames at most 2 Hz (`:63`), watch interval 250 ms (`:66`), max 16 watched rooms (`:69`), flap backoff `rejoinDelay :78`. Schema is closed: `CursorState`/`PresenceState` (`❓️quiz/🧬️schema/🔣️.json:1009,1022`, `additionalProperties:false`); `Anchor` pattern `^[a-z0-9]+(?:[:-][a-z0-9]+)*$`, max 64 (`:991`). Sharing pet state with other learners therefore needs schema changes in both twins plus `cursorIssues`/`presenceIssues` validation (`✅️validation`). The OS-level ephemeral-shared overlay derivation is `📡️replication/👕️peer-overlay` (`UiPeerMark`, TS+Rust twin).

### 6.4 Event sourcing / CQRS primitives

- Persisted shared, quiz: pure `decideHandle/decideLearner(state, command, ctx) -> {events}|{rejection}` and `evolveHandle/evolveLearner` (`❓️quiz/🔨️modules/🧾️lifecycle/🟦️.ts|🦀️.rs`, README "Run lifecycle"); framework server command/query envelopes, event stream, idempotency key (`🖥️server/🟦️.ts`); `📡️replication` wire.
- Reducers: `⏪️time-travel`, `🛠️tool-machine`, `🎯️action-bus`.
- **Statechart kernel `🔄️machine`** (TS `🟦️.ts`, Rust `🦀️.rs`, derive crate `✨️derive`): `MachineDefinition {id, nodes, transitions, guards, actions, fingerprint, manifestJson}` `:145`, `init :593`, `macrostep :611`, `timerElapsed :617`, `Snapshot :239`, `persist :801`/`restore :817` (stable string ids, migrations), `Inspector/TraceInspector :299-315`, `explore :1079` (coverage), `checkInvariants :1116`, `runConformance :1134`, `ActorSystem :869`, `Host/NativeHost/TestHost`. Rust authoring through the `statechart!` macro; TS authoring through the compiled-table interface (`MachineDefinition`). Conformance harness `🧪️tests/🧪️semio-tech-machine/🟦️.ts`.
- There is no framework primitive that event-sources purely ephemeral local-only state; the repo pattern is a pure `evolve*(state, event)` reducer plus an external store, never persisted (`🧭️session`, `createTutorialClock`).

### 6.5 Presence, anchors, pointer feed (what "walking on UI elements" can reuse)

- Anchor attribute: `data-presence-anchor` on cards, items, categories (`↕️sorting/🟦️.tsx:208`, `🃏️matching/🟦️.tsx:105`, `🗂️classification/🟦️.tsx:51,91`, `🪟️chrome/🟦️.tsx:51`); constants `PRESENCE_ANCHORS` (`👥️presence/🟦️.tsx:725`). Drag/drop attributes `data-quiz-drag|drop|item|grip` (`🤏️drag/🟦️.ts`).
- Pointer feed: `usePresencePointer(presence)` (`:803-850`) attaches passive document listeners (`pointermove`, `pointerdown`, `pointerup/cancel`, `pointerleave`, `blur`, `keydown`, `focusin/out`); `cursorAt(anchor,x,y)` (`:783`) normalizes to anchor box 0..1 rounded to 1e-4.
- Placement: `placePeers(layer, within?)` (`:918-941`) finds anchors via `anchorSelector :884` (`CSS.escape`), skips anchors inside `[inert]` pages, supports scaled panes through `within` (divides by `frame.width / within.offsetWidth`), sets `transform: translate(...)`; `focusedBox :894` and `PEER_LABEL_CLEARANCE_PX = 24` keep labels clear of the focused control (WCAG 2.4.11).
- Overlay shell: `PresenceOverlay` (`:1003`) renders `<div aria-hidden="true" data-presence-layer class="pointer-events-none fixed inset-0 z-40 overflow-hidden">`; `PanePeers :1019` does the same inside a pane (`absolute inset-0 z-10`). Colours: `presenceColor/presencePaint` (`🖱️ui/🔨️modules/👥️presence-presentation/🟦️.ts:23,48`, 12 hues, TS and Rust twins) and CSS `--quiz-peer`, `--quiz-peer-ink-*` (`🎨️.css:317-395`).
- Home page is a `LayeredOverview` strip of scaled panes (`🖱️ui/🧱️elements/🥞️LayeredOverview`, `reducedMotion: "always"|"never"` prop, tests `🧱️elements/🥞️LayeredOverview/🧪️tests/🧩️component/🟦️.tsx`), so walkable surfaces move and scale.

### 6.6 Accessibility and reduced-motion precedent

- `.quiz-glyph[data-motion]` animations only inside `@media (prefers-reduced-motion: no-preference)` and `.quiz-app:not([data-icon-motion="off"])` (`🎨️.css:41-72`); `data-icon-motion` set at `🟦️.tsx:451` from `preferences.animateIcons`; transitions collapse under `prefers-reduced-motion: reduce` (`🎨️.css:410-416`); `forced-colors: active` block (`:420-466`). Test: `❓️quiz/🧪️tests/🖼️task-icons/🟦️.tsx` (glyph `aria-hidden`, keyframes exist for every `MOTIONS` entry, preference persisted).
- JS precedent for the media query: `window.matchMedia?.("(prefers-reduced-motion: reduce)")?.matches` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:3097`).
- i18n: no default language, English first then German; learner-facing strings in `QUIZ_BUNDLE_EN/DE` (`🌐️i18n/🟦️.ts:56,392`, `QuizLabelKey :728`, `quizText(locale) :761`), content strings as `{en, de}` `Text` objects; completeness test `❓️quiz/🧪️tests/🗣️translation-completeness`.

---

## 7. SVG authoring conventions and assets

| Asset family | Convention | Path |
|---|---|---|
| UI chrome icons | Lucide-derived monochrome stroke icons: `viewBox="0 0 24 24"`, `fill="none"`, `stroke="currentColor"`, `stroke-width="2"`, round caps/joins. 249 sources, one file per icon named `<one handpicked emoji><kebab-id>.svg` inside a `<emoji><subject>` folder (`🔷️shapes/⚪️circle.svg`, `🎬️media/🪄️animate.svg`). The public icon id is the kebab name, independent of the folder. | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/` |
| Build/bundling | `bun nx run @semio-tech/assets:build`; `normalizeCatalogSvg` strips comments, classes, width/height, forces `stroke-width="2"` and `currentColor`; `catalogSvgSources` enforces the emoji-prefixed path pattern and unique ids; outputs under `🔣️icons/🤖️generated/` (`🖼️icons/{🟦️.ts,🐍️.py,🔷️.cs}` holding the `ICONS` record of inline SVG strings and `IconName`, plus `🦀️icon_name.rs`, `🖼️icon_svgs`, `🔤️shortcodes`). The barrel `🖼️assets/🔣️icons/🟦️.ts` re-exports `ICONS`, `ICON_NAMES`, `isIconName`, `resolveCatalogIconSvgFromTheme`. | `🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts:11-80`, `🖼️assets/🔣️icons/🟦️.ts`, test `🖼️assets/🧪️tests/🧪️metabolism-icon-codegen` |
| Not suitable for pets | The normalizer forces monochrome strokes and `stroke-width="2"`; there is no multi-part, multi-colour or pose-addressable SVG convention. | |
| Logos | Multi-colour hand-authored vector sets: `<emoji><name>/{☀️light,🌙️dark,⚪️round,🌘️dark-round}/{🖋️vector.svg,🖼️raster.png,✏️source.svg,📏️size-24.png,🔖️icon.ico}`, hex colours, `<g id="...">` groups; animated variant `🎞️animation/⚡️animated.svg` generated from six keyframe SVGs by a TS builder using `jsdom` to read group `transform`/path attributes and emit SMIL `<animateTransform type="matrix" keyTimes values>` (`🪧️logos/🏗️builder/🎞️animation/🟦️.ts`). Precedent for bone-like named groups with matrix transforms and 6-decimal rounding (`normalizeLogoNumber :31`). | `🧰️framework/🔨️modules/🖼️assets/🪧️logos/` |
| Site usage | The architecture site imports a logo with Vite `?raw` and passes the string to `mountQuiz(root, {logo})` (`🎓️teaching/🏛️architecture/❓️quiz/🟦️.ts:11,26`). | |
| Inline SVG in the quiz renderer | Only two: the peer pointer arrow `viewBox="0 0 16 16"` (`👥️presence/🟦️.tsx:958`, filled with `var(--quiz-peer)` via `.quiz-peer-arrow` `🎨️.css:333`) and the radar chart (`🕸️radar/🟦️.tsx:463`, `role="img"` with `aria-labelledby`/`aria-describedby`). Kind icons come from the catalog via `Icon` (`@semio-tech/ui-react/chrome`). | |
| Emoji as artwork | Task icons are single emoji glyphs plus one of six CSS microanimations (`🧬️schema/🔣️.json:53-57`; note `.🧬semio/.../QUIZ-PRODUCT-AND-TEACHING-PROCTOR/📓️task-icons.md`). | |
| SVG tooling in repo | TS: `◻️2d/🟦️.ts:278,292`; Rust: first-party typed SVG model, path/transform/viewBox/points parsers and writer in the stdio artifact (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/.../📸️snapshot/🦀️.rs:28 parse_svg_xml, :41 write_svg_xml, :186 ViewBox, :227 Matrix2D, :325 parse_transform_list, :401 PathCommand, :418 parse_path_data, :798 SvgElement`) - plugin-level, so usable as a test oracle/validator but not as a framework dependency; `🔤️typeset` `svg_outline_paths` (`usvg`, native only). | |
| Licensing | Assets module `LICENSE.md` is CC BY-ND 4.0; package licences LGPL-3.0-or-later (framework) / AGPL-3.0-or-later (architecture site package). | |

Colour tokens: design tokens are generated from one source `🖱️ui/🎨️styling/🔣️.json` into TS/Rust/Python/.NET/Tailwind (`🎨️styling/🤖️generated/*`, `🌓️theme`, `🎨️palette`, presence palette `🚦️palette-presence`); pets should take colours from CSS custom properties (`var(--base)`, `var(--active-base)`, `--quiz-peer`) so dark mode and forced-colors work. This is the closest existing schema-first multi-language generation precedent in the repo.

---

## 8. Conventions the pets work must follow (found while exploring)

- Protocol v2 test layout (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md`): `<owner>/🧪️tests/<emoji><kebab>/{🥒️.feature, 🟦️.ts, 🦀️.rs, 🐍️.py, 🧫️fixtures}`, shared vectors at `<owner>/🧫️fixtures/<case>/🔣️.json` read via `shared://<case>/🔣️.json`, oracle registry `<owner>/🔮️oracles/🔣️.json` (quiz: 15 oracles, third-party libraries and a Python reference, `❓️quiz/🔮️oracles/🔣️.json`; entry fields `id, kind, ecosystem, package, version, capabilities, comparisonProfiles, testOnly, productionReachable`), feature tags `@capability-… @oracle-… @comparison-ordered-json-v1`, scenarios `@id-… @level-fundamental @mode-differential`. Adapters: TS `defineTestAdapter({implementation, scenarios})` (`🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`), Rust `semio_repo_test_host::Adapter`. Worked example `❓️quiz/🧪️tests/🎲️seeded-randomness/`. Prior exploration: `.🧬semio/.../QUIZ-PRODUCT-AND-TEACHING-PROCTOR/📓️explore-test-conventions.md` and `📓️explore-ui-web-patterns.md`.
- Schema-first twins: quiz `🧬️schema/🔣️.json` (JSON Schema draft-07, `x-semio-formats: ["🔣️jsonschema","🦀️rust","🟦️typescript"]`, `additionalProperties:false`) with hand-written `🧬️schema/🟦️.ts` and `🦀️.rs` twins, one type per `$defs` entry; `🕹️interaction`, `⏪️time-travel`, `🔄️machine` show the module shape (TS at module root or package, `📦️packages/🦀️rust` glue, `🧬️schema`, `🧫️fixtures`, `🧪️tests`).
- Package shape: each package has `package.json`, `📋️project.json` (Nx targets calling `bun ./📜️script.ts test [level]`), `📜️script.ts` (`BundleScript`/`ScriptRouter`, `runVitest`); Rust glue `📦️packages/🦀️rust/🦀️.rs` with `#[path]` modules; the quiz Rust crate is plain sync pure functions with `serde` as its only runtime dependency (`❓️quiz/📦️packages/🦀️rust/Cargo.toml`); quiz TS has "zero runtime imports" (README). Registering a new product/package touches `Cargo.toml` (members + workspace dependency), `package.json` workspaces, and `.vscode/🧩️launch.seed.jsonc` (names like `🧪️test❓️quiz🟦️`).
- Directory emoji uniqueness holds among siblings only (ticket `ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY`); `🐾` currently names only `✏️s/🔌️plugins/🀄️wfc/⚙️engine/🐾️trail`, `🦴` names `🖱️ui/🧱️elements/🦴️Skeletons`.
- Hot files being edited by others at the time of reading: the whole quiz react target (`🎨️.css`, `🟦️.tsx`, `🎛️preferences`, `🌐️i18n`, `👥️presence`, `▶️run`, ...), `🎓️teaching/🏛️architecture/❓️quiz/*`, `🎓️teaching/🛂️proctor/*`, `package.json`, `.vscode/launch.json`.

---

## 9. Conclusion: need to existing thing vs. must be built new

| Need | Existing thing to reuse (path) | Must be built new |
|---|---|---|
| Seeded deterministic RNG, bit-exact TS=Rust | Quiz `Mt19937`, `fnv1a32`, `runSeed`, `uniformIndex`, `shuffle` (`❓️quiz/🔨️modules/🎲️randomness/🟦️.ts|🦀️.rs`; numpy/CPython oracles); or Rust `geometry::random::Rng` xoshiro256** (`📐️geometry/🎲️random/🦀️.rs`). | A float/range API on top of MT19937 (or a TS xoshiro256** twin with reference vectors); RNG state snapshot for replay. |
| 2D vector / affine / rect math | Rust `semio-framework-geometry` (`Point`, `Vec2`, `Affine`, `Rect`, `BezPath`, `PathSeg`, `geom_sel`); kurbo oracle already wired. | TS twin of the used subset; schema + shared fixtures; missing ops (`Vec2` normalize/length/angle/lerp/perp, `Affine` inverse/rotate-about, `Rect` contains/center/union, angle wrap); a dependency decision (extend `📐️geometry` vs. a new neutral module). |
| Bezier / path to SVG | TS `◻️2d` `PathSegment` + `pathSegmentsToSvgD` + `paintDrawingScene`; Rust `BezPath`, `PathSeg::eval/subdivide/arclen`; stdio SVG codec as validator/oracle. | Rust `BezPath` to SVG `d` serializer in a framework crate; pose-to-SVG-attribute writer. |
| Easing / interpolation | TS `easeInOutCubic`, `lerpOffset`, `glideOffset`, `lerpRect` (exported via `/chrome`); Rust catalog `⏱️rate` (to be relocated); `d3-ease` as oracle. | One shared neutral easing module (TS+Rust, schema'd, vectors from d3-ease); dt-correct smoothing and critically-damped spring (none exists); angle-aware interpolation. |
| Skeleton / bones / FK / IK / procedural animation | nothing | Everything: bone tree schema, pose, FK, 2-bone/FABRIK/CCD, secondary motion, blink/fidget generators. Oracles: `three/examples/jsm/animation/CCDIKSolver.js` (installed), `gl-matrix` (matrices), Python reference, closed-form two-bone IK. |
| Behaviour / state machine | `🔄️machine` (TS+Rust statechart kernel, `TestHost.advance`, `runConformance`, `persist/restore`, `explore`). | The pet statecharts (idle, fidget, walk, sit, interact), their guards/actions, and a TS authoring convention parallel to Rust `statechart!`. |
| Frame loop / scheduler | `createTutorialClock` (rAF store; full barrel only), `usePlacing` (rAF coalescing), `ContinuationPorts` + `createVirtualContinuationHost` (virtual time), `now` injection convention. | A pure `step(state, dt, input)` core with fixed-step accumulator and dt clamp; a rAF host adapter that pauses on hidden tab / reduced motion; an injectable clock in `QuizOptions`. |
| Cursor tracking (eyes) | `usePresencePointer` listener pattern, `cursorAt`, `useIntroductionPointerIdle` (idle detection). | A local-only pointer tracker for pets (the presence feed is for sharing, not for local simulation); look-at math; idle policy. |
| Walking on UI elements | `data-presence-anchor` system, `anchorSelector`, `placePeers` (scale-aware, `[inert]`-aware), `focusedBox`, overlay shell (`aria-hidden`, `pointer-events-none fixed inset-0 z-40`), `LayeredOverview` scaling. | Walkable-surface extraction (top edges of anchored/marked elements), scroll/resize/layout-shift tracking, ground contact/collision, falling and landing. |
| Social interactions between pets | `QuizPresence` rooms, `CursorState`/`PresenceState` schema + validation, `PRESENCE_FRAME_HZ`, `thinking` room pattern. | Local interaction rules (affection, disputes) as pure events; if cross-learner, a new closed `presence` schema in both twins and admission checks. |
| Event-driven / CQRS for pets | `evolveQuizState`/`QuizClientEvent` pattern; `🔄️machine`; four state lanes. | Pet events + reducer (ephemeral local-only `transient` lane); `config` slice for the on/off toggle and chosen pets; no event store needed. |
| Persistence of settings | `QuizPreferences` + `LocalStore` slice `"preferences"` + `animateIcons`/`data-icon-motion` gating. | A `pets` preference (default and migration-free), its checkbox and labels. |
| i18n of pet names/labels | `QUIZ_BUNDLE_EN/DE`, `Text {en,de}`, completeness test. | Strings for pets (names per topic, on/off label, accessible description). |
| Accessibility / reduced motion | `.quiz-glyph` and presence overlay patterns, `matchMedia` precedent, forced-colors block, `aria-hidden`. | Pet-specific gating CSS/JS (static pose under reduced motion), keyboard-focus avoidance, no pointer capture. |
| SVG artwork | Logo conventions (`🖋️vector.svg`, themed folders, named groups), icon naming (`<emoji><kebab>.svg`), Vite `?raw` import precedent, design tokens. | The multi-part, token-coloured pet SVGs (sun, cloud, house, solar panel, radiator, heat pump, window, wall, battery, ...), a named-part (bone-slot) id convention, a build/validation step, and a place in the taxonomy. |
| Tests | Protocol v2 layout, `🥒️.feature` + adapters + shared vectors + oracle registry, `defineTestAdapter`, `semio_repo_test_host`, vitest for React. | The pets cases, vectors generated by a Python/third-party reference, oracle entries (three CCD IK, d3-ease, gl-matrix, numpy/scipy). |
| Registration / launch | `Cargo.toml`, `package.json`, `📋️project.json`, `📜️script.ts`, `.vscode/🧩️launch.seed.jsonc` patterns. | New package(s) wiring. |

## 10. Risks and gotchas

1. The full `@semio-tech/ui-react` barrel pulls three, xyflow, pdfjs, i18next, xstate; the quiz imports only `@semio-tech/ui-react/chrome`, so pets must not import from the barrel (`createTutorialClock` would need to be re-exported through `/chrome` or re-implemented).
2. `@semio-tech/quiz` is a product package named for quizzes. A domain-neutral pets core should not import it just to get `Mt19937`; either the RNG moves to a neutral module or the quiz binding passes a `RandomSource`.
3. `🧮️math` is mislabelled (LLM sampling); do not treat it as the math module.
4. `◻️2d` (Rust) and the animate engine drag in `os-kernel` and plugin-only paths; neither is a safe dependency for a framework pets core.
5. The geometry crate's visible API has no inverse transform or normalization; plan to extend it (and test against kurbo, which already supplies `Affine::inverse`, `Vec2::normalize`, `Point::lerp`).
6. `LAYERED_FOLLOW_LERP` and `followStep` are frame-rate dependent (constant factor per call), so they are not a model for dt-correct smoothing.
7. Presence state schemas are closed (`additionalProperties:false`) and validated in both cores: any shared pet state is a schema change with validation, vectors and oracle updates.
8. rAF does not fire in hidden tabs (`⏳️async/🪃️continuation/🟦️.ts:24-29`), so pet simulation time must be derived from a clamped dt, not from frame counts.
9. The quiz react target is the hottest area of the working tree right now; coordinate edits to `🎨️.css`, `🟦️.tsx`, `🎛️preferences`, `🌐️i18n` and `👥️presence`.
10. No process could be run during this exploration, so nothing above (e.g. that the animate crate currently compiles, or that listed tests pass) was verified by execution.
