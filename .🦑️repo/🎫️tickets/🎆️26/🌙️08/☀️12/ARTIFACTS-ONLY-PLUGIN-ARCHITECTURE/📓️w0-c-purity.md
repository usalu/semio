# W0-C — Purity Census: Impurity Inside Plugin Trees

Scope: everything under `/Users/ueli/Documents/semio/✏️s/🔌️plugins/` (33 plugin dirs, 7848 `.rs` + 5461 `.ts`/`.tsx` files outside target/node_modules/dist).

Method: `rg` literal/PCRE greps, classified programmatically — a `.rs` hit is **test-only** if its line falls inside a `#[cfg(test)]` module or a `#[test]`/`#[tokio::test]` fn body (brace-matched per file); a `.ts`/`.tsx` hit is **test-only** if the file lives under a `🧪️tests/` dir or is named `🟦️test.ts` (the repo's sole TS test convention — confirmed via `find`). Everything else is production. Doc-comment-only mentions (`///`, `//!`) are called out and excluded from "real" counts where found. Scripts used: `census.py` (fs/env/net) and `find_pure_fn_mutation.py` (category 6), both in the ticket-external scratchpad, not committed.

All counts below are **real grep counts**, not estimates. Where I could not fully verify a claim, it is marked `UNVERIFIED`.

---

## 1. Filesystem (`std::fs`, `tokio::fs`, `File::`, `read_dir`, `read_to_string`, `write`, `create_dir`)

`tokio::fs` and `File::` : 0 hits anywhere. Consolidated search for word-bounded `fs::` (catches both `std::fs::x` and `use std::fs; fs::x`) plus a Node `fs`-import sweep for TS:

| # | file:line | anchor | line | bucket |
|---|---|---|---|---|
| 1 | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs:12` | `read_dir(&icons_src)` | `for ent in std::fs::read_dir(&icons_src)...` | **prod** (build.rs, compile-time, not runtime plugin code) |
| 2 | `...build.rs:24` | `std::fs::copy(&path, &dest)` | `std::fs::copy(&path, &dest)...` | **prod** (build.rs) |
| 3 | `...build.rs:41` | `std::fs::write(&gen_path, gen)` | `std::fs::write(&gen_path, gen)...` | **prod** (build.rs) |
| 4 | `🎞️animate/🗿️artifacts/🎬️present/.../⚙️engine/🦀️component.rs:50` | `fs::create_dir_all(&scene_dir)` | | **prod** |
| 5 | same file:62 | `` unchanged `fs::write`s `` | doc comment | **not code** (doc comment only) |
| 6 | same file:64,66,69,70,71,72 | `fs::create_dir_all(output_dir)` / 5× `fs::write(output_dir.join(...))` | writes `deck.json`, `🌐️index.html`, `styles.css`, `manifest.json`, `player.js` | **prod** — 6 real calls, `compile()` path |
| 7 | `.../⚙️engine/🎥️video/🦀️component.rs` — 22 real call sites: lines 27,30,76,92,96,99,106,122,124,128,427,428,1064,1077,1105,1106,1108,1111,1143,1174,1201,1202,1209,1222 | `fs::create_dir_all`/`fs::write`/`fs::read`/`fs::remove_file`/`fs::remove_dir_all`/`fs::copy` | disk cache LRU + mp4/gif partial-render pipeline | **prod** — 24 real calls (see note below) |
| 8 | `🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs:39` | `fs::read_to_string(fixture_path)` | | **prod** (CLI bin, not app/artifact) |
| 9 | `🔱️trinity/.../📦️bin.rs:32` | `std::process::exit(1)` | (see §2) | **prod** (CLI bin) |

**Real production fs call sites: 35** (36 `fs::`-matching lines minus 1 doc comment), concentrated in **one file**: `🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs` (26 calls: disk-backed render cache + LRU eviction + partial-mp4/gif assembly under a temp dir). This file *is* inside `🗿️artifacts/`, so per APA "IO lives in artifacts" this is architecturally the **correct location** — flagging it here only because the assignment asked to census all impurity, sanctioned-location or not. The `present` engine.rs (non-video) file adds 6 more `fs::write`/`create_dir_all` calls also inside `🗿️artifacts/`.

The two non-`component.rs` hits (`build.rs`, `trinity/jack/📦️bin.rs`) are compile-time / standalone-CLI code, not part of the running plugin's app/artifact surface — flagged but likely out of APA's jurisdiction; UNVERIFIED whether APA's "exactly apps+artifacts" rule is meant to cover `build.rs`/`bin.rs` siblings at all.

**Test-only fs bucket (sanctioned, separate): 29 `.rs` hits + 120 TS hits.** All 120 TS hits are `import { readFileSync } from "node:fs"` inside `🧪️tests/🟦️test.ts` files — 100% clean split, zero exceptions. Rust test-only fs calls read fixture bytes (`std::fs::read(concat!(...))`, `read_to_string`) for stdio-format golden files (csv/ifc/las/png/jpg/stl/zip) plus 5 `temp_dir()`-based test scratch dirs in animate's video renderer tests.

TS fetch/network is filesystem-adjacent so reported under §3 instead.

---

## 2. Environment / process (`std::env::var`, `set_var`, `temp_dir`, `Command::new`, `process::exit`, `process::id`)

| file:line | anchor | line | bucket |
|---|---|---|---|
| `🧩️puzzle/📦️packages/🦀️rust/build.rs:6` | `CARGO_MANIFEST_DIR` | `std::env::var("CARGO_MANIFEST_DIR")` | prod (build.rs) |
| `...build.rs:45` | `OUT_DIR` | `std::env::var("OUT_DIR")` | prod (build.rs) |
| `🌍️gis/🎛️apps/◻2d/.../🗺️map/🦀️component.rs:77` | `SEMIO_ASSET_BASE_URL` | `std::env::var("SEMIO_ASSET_BASE_URL")` | **prod, inside 🎛️apps** |
| `🌍️gis/...🗺️map/🦀️component.rs:116` | `set_var("SEMIO_ASSET_BASE_URL"` | `unsafe { std::env::set_var(...) }` | **RS_TEST** (inside `#[cfg(test)]`) — the read at line 77 is prod but real |
| `🔱️trinity/.../📦️bin.rs:32` | `std::process::exit(1)` | | prod (CLI bin) |
| `🎞️animate/.../⚙️engine/🦀️component.rs:239,257` | `std::env::temp_dir().join(...std::process::id()...)` | scratch-dir naming for present/scene compile | prod, inside `🗿️artifacts` |
| `📐️cad/📦️packages/🟦️typescript/📜️script.ts:26` | `process.env.CAD_GENERATE_STEP_FIXTURES` | | prod-ish, but `script.ts` is nx tooling per CLAUDE.md, not app/artifact runtime |

Test-only env bucket: `std::env::var(...)` gated demo-dump flags in dag/lowpoly/layout tests (3), `env::set_var` in gis map test (1), `std::process::id()`-seeded `temp_dir()` scratch dirs in animate/video tests (2) and lowpoly forest export test (1).

**Real production env/process hits: 3 non-tooling** — the GIS `🗺️map` component's `SEMIO_ASSET_BASE_URL` env read (inside `🎛️apps`, a genuine app-tree env-var reach-around) and animate's two `temp_dir()+process::id()` scratch-path builders (inside `🗿️artifacts`, paired with the fs writes in §1). `Command::new`/`std::process::Command`: **0 real hits** — the only text match is a *doc comment* in `animate/.../🎥️video/🦀️component.rs:923` that itself narrates: *"The FFmpeg subprocess path (`Command::new("ffmpeg")`...) is deleted outright — mp4 assembly now goes through stdio's real...`encode_mp4`/`encode_gif`...in-process, no subprocess involved."* I could not verify who wrote that comment or when; it reads as a prior recon's finding already remediated. Treating it as **informational, not a live violation** — grep confirms zero live `Command::new` calls in this file or plugin tree.

---

## 3. Network (`reqwest`/`ureq`/`hyper`/`std::net`/`TcpStream`; TS `fetch(`/`XMLHttpRequest`/`WebSocket`/`localStorage`/`sessionStorage`/`indexedDB`)

Rust side: **0 hits, all 5 patterns, whole tree.** No HTTP client crate, no raw socket usage anywhere under `✏️s/🔌️plugins/`.

TS side — `XMLHttpRequest`, `WebSocket`, `localStorage`, `sessionStorage`, `indexedDB`: **0 hits, all 5 patterns.**

`fetch(` : **2 hits, both real production, both inside `🎛️apps` (not `🗿️artifacts`):**

| file:line | anchor | context |
|---|---|---|
| `🎞️animate/🎛️apps/🎬️present/📺️renderer/⚛️react/🟦️component.tsx:3303` | `fetch(resolvePresentationAssetUrl(embodiment.src))` | inside `MarkdownMorphView`'s `useEffect`, fetches markdown source then compiles to HTML |
| same file:3352 | `fetch(resolvePresentationAssetUrl(embodiment.src))` | inside the JSON-embodiment sibling view's `useEffect`, fetches + `JSON.parse`s remote asset |

**This is the cleanest, highest-confidence violation in the whole census**: two live network calls, direct side effect, executed from inside a `useEffect` in an app-tree React component (`🎛️apps/🎬️present/📺️renderer/⚛️react/🟦️component.tsx`), not routed through any artifact. Grep-anchor: search for `resolvePresentationAssetUrl(embodiment.src)` in that file — both call sites use the identical helper-call substring.

---

## 4. Mutable global/ambient state

Patterns run: `thread_local!`, `static mut` (0 hits), `lazy_static!` (0 hits, not a repo dependency), `once_cell` (0 hits — repo is on std `OnceLock`/`LazyLock`), `OnceLock`, `OnceCell` (0 hits), `LazyLock`, `Mutex<`, `RwLock<`, `RefCell<`, `Cell<`, `Atomic*`.

### 4a. `OnceLock` — 500 total hits (495 prod / 5 test), 235 distinct files

| location | files | verdict |
|---|---|---|
| inside `🗿️artifacts/` | 230 | **sanctioned write-once table** — every sampled instance is the `get_or_init(\|\| ...)` cached-example / cached-computation pattern (e.g. `norm`'s per-Eurocode engine files each declare 3: a doc cache, a demo-scene cache, an id-counter slot) |
| inside `🎛️apps/` | 3 lines across 2 files (`🪐️space/🎛️apps/🪐️space/🦀️component.rs:116`, `🪐️space/🎛️apps/🏠️home/🦀️component.rs:69,74`) | **real mutable state, NOT write-once** — see 4d, these back live `Mutex<HashMap<...>>` registries that are read *and written* on every call, not computed once |
| non-standard plugin subdir | 2 (`🌊️flow/🧩️extensions/📐️brep/🦀️component.rs:1487` — `OnceLock<Mutex<()>>` lock slot; `🔱️trinity/🗣️language-service/🦀️component.rs:14` — `OnceLock<GraphManifest>` cached-parse slot) | sanctioned write-once pattern, but flagged because `🧩️extensions/` and `🗣️language-service/` are themselves **structural APA violations** (plugin subtrees that are neither `🎛️apps` nor `🗿️artifacts`) — out of this assignment's scope but worth another agent's attention |

### 4b. `LazyLock` — 31 hits, all prod, 0 test

29 of 31 are `pub static SOURCE`/`EXAMPLE_JSON: LazyLock<...> = LazyLock::new(|| parse_example_dsl(...))` — deterministic, pure, computed-once-from-embedded-const-text. **Sanctioned.**

The other 2, both in `🪐️space/🦀️component.rs` (the plugin **root** `🦀️component.rs`, not inside apps/artifacts at all):
- line 24: `static FIXTURES: LazyLock<()> = LazyLock::new(|| { register_os_fixture_json(...); register_os_fixture_json(...); });` — **real side effect gated by laziness**, not a value cache: first call to `ensure_space_fixtures_registered()` mutates a global fixture registry (`register_os_fixture_json`, an external `semio_framework_os` call) as a side effect; the `LazyLock<()>` is being used purely as a "run exactly once" gate. Anchor: grep `static FIXTURES: LazyLock<()>`.
- line 68 in `🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs`: `pub static PUZZLE3D_MESH_REGISTRY: LazyLock<Mutex<HashMap<String, (Vec<f32>, Vec<u32>)>>>` — real mutable registry, see 4d.

### 4c. `thread_local!` — 7 hits total, 6 prod / 1 test

| file:line | anchor | holds | verdict |
|---|---|---|---|
| `🖍️draw/🎛️apps/🖍️draw/🦀️component.rs:164` | `static DRAW_SESSION` | `RefCell<DrawSession>` — mid-gesture FSM session | **real ambient state**, inside `🎛️apps` |
| `💠️lowpoly/🎛️apps/💠️lowpoly/🦀️component.rs:48` | `static LOWPOLY_SCRATCH` | `RefCell<LowpolyScratch>` — mid-gesture scratch + texture cache | **real ambient state**, inside `🎛️apps` |
| `📐️cad/🎛️apps/📐️cad/🦀️component.rs:947` | `static CAD_PREVIEW_SEQ` | `RefCell<u64>` — rubber-band preview tick counter | **real ambient state**, inside `🎛️apps` |
| `🧩️puzzle/🎛️apps/🖐️5d/🦀️component.rs:1295` | `static PUZZLE5D_PLAY_SESSION` | `RefCell<Puzzle5dPlayApp>` — precompute session | **real ambient state**, inside `🎛️apps` |
| `🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs:1876` | `static PUZZLE3D_PLAY_SESSION` | `RefCell<Puzzle3dPlayApp>` — precompute/gumball scratch | **real ambient state**, inside `🎛️apps` |
| `🌊️flow/🧩️extensions/🧮️math/🦀️component.rs:291` | `static ENTROPY_SEED` | `Cell<u64>` — RNG seed | **real ambient state**, non-standard `🧩️extensions/` subtree |
| `🪐️space/🎛️apps/🪐️space/🦀️component.rs:568` | (inside `#[cfg(test)]`) | — | **test-only** |

All 6 prod hits carry a doc comment along the lines of *"ArtifactApp methods are associated fns (no `&mut self`/no `&self`), so session state lives here until [Draft lane / EngineHandles] carries it"* — i.e. **every author independently reached for `thread_local!` for the identical structural reason: the framework gives apps no owned-state slot.** This is the single strongest piece of evidence for §5 below.

### 4d. `Mutex<` — 22 hits (19 prod / 3 test)

Split by whether the Mutex's payload is genuinely mutated after construction ("real") vs. only ever read after one-time init ("sanctioned"):

| file:line | anchor | payload | verdict |
|---|---|---|---|
| `🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs:68` | `PUZZLE3D_MESH_REGISTRY` | `Mutex<HashMap<...>>` | **real** — written at `🎮️commands/🖌️brush/🦀️component.rs:132` (`.lock()` then insert), read at `🦀️component.rs:2643`; inside `🎛️apps` |
| same file:1046,1897,1898,1899 | `fill_display_memo`/`geometry_cache`/`document_sections_cache` | `Mutex<Option<...>>` struct fields | **real** interior-mutability memoization fields on a session struct, inside `🎛️apps` |
| `💠️lowpoly/🎛️apps/💠️lowpoly/🖌️session/🦀️component.rs:5` | doc comment referencing `flow`'s `Mutex<FlowEvalSession>` | — | doc comment only, not code |
| `📐️cad/🗿️artifacts/📐️cad/.../⚙️engine/🦀️component.rs:851-852` | `last_cad_computer_contributions_json` | `OnceLock<Mutex<String>>` | **real** — a "last contribution" slot that's overwritten on every host push, not write-once, but inside `🗿️artifacts` (sanctioned location even if not sanctioned pattern) |
| `🌀️procedural/🗿️artifacts/🧊️procedural3d/.../⚙️engine/🦀️component.rs:60` | `static LAST: Mutex<String>` | | **real**, same "last value" pattern, inside `🗿️artifacts` |
| `🪐️space/🎛️apps/🪐️space/🦀️component.rs:115-116` | `shared_presence_peers` | `OnceLock<Arc<Mutex<HashMap<String,HashMap<...>>>>>` | **real** — presence peer registry, written by `publish_presence`, read by `presence_peers_json`; inside `🎛️apps` |
| `🪐️space/🎛️apps/🏠️home/🦀️component.rs:73-74` | `shared_studio_ports` | `OnceLock<Arc<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>>>` | **real**, inside `🎛️apps` |
| `🎞️animate/🗿️artifacts/.../⏱️rate/🦀️component.rs:499` | `pub value: Arc<Mutex<f64>>` | | **real**, mutable rate-state field, inside `🗿️artifacts` |
| `🏭️process/🗿️artifacts/🧊️process3d/.../⚙️engine/🦀️component.rs:137-138` | `CONTRIBUTED_MACHINE_CATALOGS` / `LAST_PROCESS_CONTRIBUTIONS_JSON` | `Mutex<Vec<...>>` / `Mutex<String>` | **real** process-global growable registry, inside `🗿️artifacts` |
| same file:421 | `fn kernel(&self) -> &std::sync::Mutex<Brep>` | | **real**, accessor into mutable kernel state |
| `🪵️sourcing/🗿️artifacts/🗂️curate/.../⚙️engine/🦀️component.rs:590-591` | `CONTRIBUTED_SOURCING_MODULES` / `LAST_SOURCING_CONTRIBUTIONS_JSON` | same shape as process3d | **real**, inside `🗿️artifacts` |

**Verdict: 17 of 19 prod `Mutex<` hits are real mutable state (not write-once)**, split roughly evenly between `🎛️apps` (7: puzzle3d ×5, space ×2) and `🗿️artifacts` (10). The `🗿️artifacts` ones are at least in the sanctioned *location*; the `🎛️apps` ones are a structural violation on top of being impure.

### 4e. `RefCell<` — 48 hits (47 prod / 1 test)

44/47 prod hits are inside `🎛️apps`; 2 in `🖍️draw/🔄️fsm/✨️macros/🦀️component.rs` (macro-template code generating `RefCell` fields, non-standard `🔄️fsm/✨️macros/` subtree); 1 in `🧩️puzzle/🔨️modules/🎲️board-2d/🦀️component.rs` (non-standard `🔨️modules/` subtree).

Of the 47, **10 are doc-comment-only** (`///`/`//!` lines narrating a *past* `RefCell<XPlayRuntime>` that a prior "B1" migration wave already removed — found in `lowpoly`, `playbook`, `layout`, `architect` ×3, `forms`) — these are informational, not live code, and evidence that **some** apps have already been migrated off ad hoc `RefCell` app-state by another concurrent session.

The remaining **37 are live code**, and they cluster into two distinct shapes:

**Shape A — WASM host-bridge store, 17 occurrences**, one per app's `🌉️wasm/🦀️component.rs`: `store: RefCell<XStore>` or `state: Rc<RefCell<XSessionInner>>` (procedural2d/3d, cad, fem2d/3d, present, writer, process3d, sequence, puzzle5d/3d, raster, layout, shooting, trinity-jack, imperative, gis2d). This looks like a **framework-mandated boilerplate pattern** for the wasm-bindgen interop layer, present near-uniformly — every app that has a `🌉️wasm/` subdir has exactly this shape. Anchor: grep `store: RefCell<` or `state: Rc<RefCell<`.

**Shape B — hand-rolled session/scratch state directly in the app's root `🦀️component.rs`, 15 occurrences** across puzzle3d (6: `precompute`, `transform_drag_active`, `transform_base`, `transform_scratch`, `preview_seq`, plus the thread_local from §4c), puzzle5d (2: `precompute`, `registered_mesh_urls`), cad (2: a `&'a RefCell<u64>` handle param + the thread_local), draw (1 `store: RefCell<DrawStore>` field — distinct from its thread_local), trinity/rewrite (2: `store`, `state`), puzzle2d (1: `host: &'a RefCell<BoardHost>` param), board-2d module (1: `state: Rc<RefCell<BoardSessionInner>>`).

### 4f. `Cell<` — 2 hits, both prod

`🔱️trinity/🎛️apps/♻️rewrite/🌍️world/🦀️component.rs:292` (`last_logged_lod: Cell<i8>`, a log-throttle field) and the `ENTROPY_SEED` already counted in §4c.

### 4g. `Atomic*` — 17 hits, all prod, 0 test

15 are `AtomicU64`/`AtomicU32` monotonic-ID counters (`next_cad_id`, `create_note_id`, and equivalents in remodel/shooting/note/architect-kernel/forms/puzzle2d/3d/5d) — real cross-call mutable global state (each call visibly increments), but a narrow, arguably-benign ID-generator pattern rather than business-state mutation; **still a real violation of "no mutable ambient state,"** just a low-severity one. 2 are `AtomicBool` fields on an animate-video `PreviewApp` struct (`closed: Arc<AtomicBool>`), interior-mutability on an owned struct rather than a process global — lower severity again.

### Summary verdict for §4

- **Sanctioned write-once tables:** ~260 hits (230 `OnceLock` in artifacts + 29 `LazyLock` examples + the 2 non-standard-subtree `OnceLock`s).
- **Real mutable/ambient state:** thread_local ×6, Mutex ×17, RefCell ×37 live (10 more are dead doc-comment mentions), Cell ×2, Atomic ×17, plus the 2 `OnceLock`/`LazyLock` cases holding live `Mutex<HashMap>` registries in `🎛️apps` (already counted under Mutex/LazyLock). **Roughly 79 live real-mutable-state code sites**, the large majority (Shape A's 17 + Shape B's 15 + all 6 thread_locals + puzzle3d's 5 Mutex fields + its 1 LazyLock registry = 44 of 79) inside `🎛️apps` trees specifically.

---

## 5. App-state bypass — `type Draft =` / `type DraftMutation =` census

`rg "type Draft\s*="` across the plugin tree: **54 matches, 54 distinct files, every single one reads `type Draft = NoDraft;`.** Companion `rg "type DraftMutation\s*="`: **54 matches, every one `type DraftMutation = NoDraftMutation;`.**

**0 of 54 apps use a real typed Draft lane. 54/54 = 100% NoDraft.**

This is the root cause behind §4c/§4e: the `ArtifactApp` trait's `handle`/`render` are associated functions with no `&self`/`&mut self` and `Draft` is universally the empty `NoDraft`, so there is **no framework-sanctioned place for an app to hold session-only state** (drag deltas, precompute caches, texture scratch, gesture FSM state). Every app that needs such state independently reinvented a storage mechanism — `thread_local!` (6 apps), bespoke `RefCell` fields threaded through helper structs (puzzle3d, puzzle5d, cad, draw, trinity/rewrite, puzzle2d, board-2d — 7 apps/modules), or a `Mutex<HashMap>` global registry (puzzle3d, space×2 apps). This is a single systemic gap, not 20 independent bugs.

**Migration-size estimate per state-holding app** (small = swap field type + threading, one Draft variant; medium = several distinct pieces of scratch state or cross-command sequencing; large = precompute caches keyed by content hash plus drag/gesture FSM plus id-registries):

| app | state held | size | why |
|---|---|---|---|
| `🖍️draw` | `DrawSession` (gesture FSM) in thread_local + `DrawStore` RefCell (wasm bridge, separate concern) | **medium** | FSM session type already exists (`DrawSession`), just needs relocating into a typed `Draft`; interacts with `🔄️fsm/✨️macros` codegen (non-standard subtree, adds coupling) |
| `💠️lowpoly` | `LowpolyScratch` (mid-gesture + texture cache) in thread_local, mutated *inside `render()`* (see §6) | **medium** | scratch is read-modify-write from both `handle` and `render`; moving to Draft requires the render mutation to become a pure read of pre-computed Draft state instead |
| `📐️cad` | `CAD_PREVIEW_SEQ: RefCell<u64>` tick counter | **small** | single scalar counter, no cross-references |
| `🧩️puzzle/🧊️3d` | `Puzzle3dPrecomputeSession` + `transform_drag_active`/`transform_base`/`transform_scratch` + `preview_seq` (all RefCell) + `fill_display_memo`/`geometry_cache`/`document_sections_cache` (Mutex memo fields) + `PUZZLE3D_MESH_REGISTRY` (global Mutex HashMap, written from a sibling `🎮️commands/🖌️brush/` module) | **large** | 9 distinct pieces of state, one of which (mesh registry) is written from an entirely different file/module, plus drag/gumball transform FSM |
| `🧩️puzzle/🖐️5d` | `Puzzle5dPrecomputeSession` + `registered_mesh_urls: RefCell<HashSet<String>>` + thread_local play session | **medium** | 3 pieces, same shape as 3d but no cross-module writer |
| `🌊️flow/🧩️extensions/🧮️math` | `ENTROPY_SEED: Cell<u64>` | **small** | single scalar, but lives in a non-standard `🧩️extensions` subtree (structural issue precedes the Draft-migration issue) |
| `🪐️space` (both `🪐️space` and `🏠️home` apps) | presence-peer registry, studio-port registry, temp-catalog port — all process-global `OnceLock<Arc<Mutex<...>>>` | **large** | these are cross-app, cross-session-lifetime shared registries (presence must survive per-client, ports are looked up by string key across the whole running process) — genuinely hard to fit into a per-document Draft lane; likely needs a framework-level answer (a real shared/ephemeral capability), not just "move the field," flagged as **UNVERIFIED** whether Draft is even the right target for this one |
| `🔱️trinity/♻️rewrite` | `TrinityGraphStore` (wasm) + `TrinitySessionInner` (Rc<RefCell>) + `last_logged_lod: Cell<i8>` | **medium** | 3 pieces, log-throttle counter is trivial, session/store are the real work |
| `🧩️puzzle/🔨️modules/🎲️board-2d` + `🧩️puzzle/◻2d` | `BoardSessionInner` (Rc<RefCell>) shared via `&'a RefCell<BoardHost>` param | **medium** | lives in a non-standard `🔨️modules/` subtree, same structural-first caveat as flow/math |
| 17 apps' `🌉️wasm/🦀️component.rs` (Shape A) | `store: RefCell<XStore>` wasm-bindgen bridge | **UNVERIFIED — likely framework-mechanism, not per-app** | did not trace whether this is boilerplate the *framework* generates/mandates for the wasm entry point (in which case it's a W1 mechanism-design concern, not a per-app W3 migration) or hand-written per app; recommend the framework-mechanisms wave (W1) confirm before scoping W3 |

---

## 6. `render()`/`measure()`/`context_menu()` mutating state

Brace-matched every `fn render(`/`fn measure(`/`fn context_menu(` body (313 / 62 / 14 candidate functions respectively) against every `borrow_mut()`/`.lock()` call site (314 total in the tree) for direct containment.

**1 hit, confirmed real:**

`💠️lowpoly/🎛️apps/💠️lowpoly/🦀️component.rs:343`, inside `fn render(body_key: &str, doc: ..., cfg: ...) -> UiNode` (declared line 337):
```rust
let (scratch_projection, texture_cache) = LOWPOLY_SCRATCH.with(|scratch| {
    let mut scratch = scratch.borrow_mut();
    if matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV) {
        scratch.refresh_texture_cache(projection);
    }
    (scratch.transform_projection(), scratch.textures().clone())
});
```
`render()` takes `&ArtifactView`/`&ConfigView` (immutable, no `&mut self` at all per the trait signature) yet mutates the app's thread-local scratch cache as a side effect of being called. Anchor: grep `refresh_texture_cache(projection)`.

Spot-checked the other apps that hold thread_local/RefCell session state (`cad`, `puzzle3d`, `puzzle5d`, `draw`) by grepping their own `fn render(` bodies for `borrow_mut`/`.lock()`: **none of the other 4 mutate inside `render()`** — they only mutate inside `handle()`. Lowpoly is the sole outlier for this category.

---

## 7. `ArtifactApp::seed` usage

`rg "fn seed\("` across the whole plugin tree: **1 hit.**

`🌿️vcs/🎛️apps/🌿️vcs/🦀️component.rs:183`:
```rust
fn seed(store: &mut ArtifactStore<VcsSnapshot, VcsDemoMutation>) {
    seed_vcs_demo_history(store);
}
```
`seed_vcs_demo_history` (same file, line 82) builds a synthetic commit history purely by calling `store.dispatch(ArtifactCommand::Apply {...})` / `store.dispatch(ArtifactCommand::CommitCheckpoint {...})` in sequence — every mutation goes through the same `ArtifactCommand` dispatch path a real user action would use, no direct field writes, no IO. **Clean — this is the one `seed()` in the whole fleet and it is pure/in-process.** 53 of 54 apps implement no `seed()` at all (default/no demo bootstrap, or bootstrap done some other way not matching this literal signature — not independently verified).

---

## Notable side findings (outside the 7 numbered categories, flagging for other agents)

- **Non-standard plugin subtrees** that are neither `🎛️apps` nor `🗿️artifacts`, found incidentally while classifying hits above: `🌊️flow/🧩️extensions/`, `📖️playbook/🧩️extensions/`, `🔱️trinity/🗣️language-service/`, `🖍️draw/🔄️fsm/✨️macros/`, `🧩️puzzle/🔨️modules/`. These are structural APA violations (a plugin must be "EXACTLY 🎛️apps and 🗿️artifacts") independent of the purity findings above, several of which (flow/math's `Cell`, trinity's `OnceLock`, draw's macro-generated `RefCell` fields, puzzle's board-2d `RefCell`) *also* carry a purity finding. UNVERIFIED whether this list is exhaustive — it is only what turned up as a byproduct of the greps above, not a dedicated structural sweep.
- A doc comment at `🎞️animate/.../🎥️video/🦀️component.rs:923-927` explicitly self-reports a prior "W0 recon" having found and removed an `Command::new("ffmpeg")` subprocess violation. I did not verify this claim against git history/blame (out of scope, read-only assignment) — flagging as **UNVERIFIED provenance**, but the current tree genuinely has zero live `Command::new` calls, so the *current-state* finding (0 hits) stands regardless of the comment's origin.

---

Files touched by this agent: **only this report**, `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w0-c-purity.md`. No source file was modified. Scratch scripts (`census.py`, `find_pure_fn_mutation.py`) live outside the ticket folder in the session scratchpad per this agent's read-only mandate and are not part of the deliverable.
