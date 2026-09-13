# 🎚️ Fix Forward — `WindowConfigOwner::State: DslField`, Puzzle 3D and the Whole Remaining Tree (2026-09-13)

Ticket: `26/09/09/PROCEDURAL-3D-END-TO-END`.
Lane: unblock `bunx nx run @semio-tech/framework-renderer-wgpu:wasm`, which had failed since 05:33
(`📓️wgpu-retained-hit-registry-2026-09-13.md` §5) on

```
error[E0277]: the trait bound `window::component::Puzzle3dWindowConfig: DslField` is not satisfied
   --> ✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🪟️window/🦀️.rs:194:18
```

All logs under `🗑️generated/fix-forward-puzzle/`. Retained input script:
`🐍️window-config-dslfield-forward.py` (ticket root).

## 1. Headline

| deliverable | result |
| --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown` | **0 errors**, 30 warnings (`check-wgpu-3.txt`) |
| `bunx nx run @semio-tech/framework-renderer-wgpu:wasm` | **WASM-EXIT=0**, `dist/wasm-dev` 79 522 302 B + 177 202 B, 07:40 (`wasm-build-final.txt`) |
| `bun …/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts generate-frame-worker` | **WORKER-EXIT=0**, `generated 🎞️frame-worker.js` (`frame-worker-final.txt`) |
| `cargo check --workspace --keep-going` | **0** remaining `DslField` bounds anywhere (`check-workspace-2.txt`) |

The brief's frame-worker path (`…/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`) does not exist — that was the
`WORKER-EXIT=1 Module not found` in the earlier `wgpu-rebuild/build.txt`. The registered script is
`…/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts` (`📋️project.json:319`). No census throw this run.

## 2. The peer's bound, and how it was completed

`🔌️plugin/🪟️window/🎚️config/🦀️.rs:21` (uncommitted, 05:28) added `store::mounted_pack_rt::DslField`
to `WindowConfigOwner::State`. The new retained loader beside it —
`📥️retained/🦀️.rs`, an **untracked** peer file, together with the equally untracked
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/` — reads
`<O::State as store::ArtifactPack>::record_spec()` (`:197`) and
`<O::State as …::DslField>::from_value` (`:359`).

The completion applied at every owning site is exactly §4 of `📓️fix-forward-viewport2d-2026-09-13.md`:

1. `#[derive(dsl::DslArtifact)]` on the config struct with `id`/`extension` stated **explicitly**, so
   the derived `__DSL_ENVELOPE_ID`/`__DSL_EXTENSION` reproduce the envelope identity the hand-written
   `ArtifactDsl` already carried — no envelope changes anywhere;
2. every field type made to satisfy `DslField`;
3. the hand-written `ArtifactDsl`/`ArtifactPack` bodies replaced with the record-backed pair
   (`dsl::parse`/`dsl::print`, `store::pack_rt::encode_document`/`decode_document`) and
   `record_spec() -> Some(Self::__dsl_spec())` — `None` there fails every retained load of that kind
   with `WindowConfigPackLoadDiagnostic::TypedState`.

### 2.1 The framework records, derived once instead of mirrored

`WorldSunConfig` and `WorldProjectionConfig` live in `semio-framework-plugin`
(`🔌️plugin/🦀️.rs`, `world3d_host`). CAD had worked around that with hand-mirrored local copies
(`CadSunConfig`, `CadProjectionDsl`, "foreign, out-of-scope crate — cannot gain a `dsl` derive
there"). That premise is false: the derive only needs the name `dsl` bound at the crate root, and the
crate already depends on `semio-framework-os-kernel`. So the plugin crate gained

```rust
extern crate semio_framework_os_kernel as dsl;
```

plus `dsl::DslRecord` on both records. `cargo check -p semio-framework-plugin --target wasm32-wasip2
--profile wasm-dev` → **0 errors, 26 warnings** (`check-plugin.txt`). Every plugin embedding a
`world-3d` sun or projection now gets `DslField` for free instead of a mirror (Puzzle 3D and Puzzle 5D
both needed it); CAD's mirrors were left alone — they are that lane's, and removing them is a separate
packet.

### 2.2 Puzzle 3D

* `Puzzle3dCamera` and `Puzzle3dSelectableKinds` (`✏️editor/🎚️config/🦀️.rs`) gained `dsl::DslRecord`;
  `position`/`target`/`up` carry `#[dsl(coord)]`, `projection` `#[dsl(block)]`.
* `panel_pages: HashMap<String, u32>` → `BTreeMap<String, u32>`: `🗣️dsl` implements `DslField` for
  `BTreeMap<String, T>` and not for `HashMap`, and a persisted record's map field has to be ordered to
  encode deterministically at all. 11 signatures plus every construction site, across
  `✏️editor/🦀️.rs`, `🎚️config/🦀️.rs`, `📌️panels/🔍️inspection/🦀️.rs`, `📌️panels/🗿️artifact/🦀️.rs`
  (+ its unit test) and `🪟️window/🦀️.rs`.
* `Puzzle3dWindowConfig` left the local `json_store!` shorthand for the record-backed codecs, keeping
  `s.puzzle.puzzle3d.windowconfig` / `puzzle3dwindowcfg`. The transient stays on `json_store!` —
  `WindowTransientOwner` has no `DslField` bound.

`cargo check -p semio-s-artifact-puzzle-3d --target wasm32-wasip2 --profile wasm-dev --features
component-app-assembly` → **0 errors, 97 warnings** (`check-puzzle3d-2.txt`).

### 2.3 Puzzle 2D and Puzzle 5D — surfaced by the same check once 3D compiled

`Puzzle2dWindowConfig` (all primitives, shared by the overview/detail/selection panes) and
`Puzzle5dBoardWindowConfig`/`Puzzle5dWorldWindowConfig` got the same treatment;
`Puzzle5dCamera2d`/`Puzzle5dCamera3d` gained `dsl::DslRecord`, and 5D's two codec pairs became one
local `record_store!` macro so the shared body is written once.

### 2.4 The renderer's own wasm break, found behind the E0277

With the three puzzle crates green the renderer failed on its own:

```
error[E0609]: no field `identity` on type `&mut ShellState`
   --> 🧰️framework/…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3827:56
```

`ShellState::identity` is inside the `#[cfg(not(target_arch = "wasm32"))]` `🔖️Identity` region. A peer's
uncommitted `session_identity` plumbing (file mtime 05:54, last commit 03:15) cfg-gated the write at
`:3715` and inside `start_directory_home_publication`, but wrote the third construction site
un-gated — invisible to any native build. Fixed forward with one cfg'd accessor,
`ShellState::session_identity_view()`, used at both open-coded sites, rather than a third copy of the
cfg pair.

## 3. The rest of the tree — 12 more configs

The `-p semio-framework-os-renderer-wgpu` check only reaches the puzzle crates. A full
`cargo check --workspace --keep-going` reported 12 further `*WindowConfig: DslField` bounds. All were
forwarded by the retained script `🐍️window-config-dslfield-forward.py`:

| crate | configs |
| --- | --- |
| `semio-s-artifact-sequence-sequence` | `SequenceMainWindowConfig` |
| `semio-s-artifact-forms-forms` | `FormsTryWindowConfig` |
| `semio-s-artifact-layout-layout` | `LayoutWindowConfig` |
| `semio-s-artifact-note-note` | `NoteCompositeWindowConfig` |
| `semio-s-artifact-draw-drawing` | `DrawingCanvasWindowConfig` |
| `semio-s-artifact-remodel-remodeling` | `RemodelingFramesWindowConfig`, `RemodelingReportWindowConfig`, `RemodelingModelWindowConfig` |
| `semio-s-artifact-architect-program` | `ArchitectAdjacencyWindowConfig`, `ArchitectRegisterWindowConfig`, `ArchitectReportWindowConfig`, `ArchitectGraphWindowConfig` |

Nested types that needed nothing: `SequenceCamera`, `LayoutCamera`, `NoteCamera` and
architect's `AdjacencyKind` already derive `dsl::DslRecord`/`DslScalar`; `EntityId` already carries a
hand-written `DslField`; `store::Viewport2d`/`Viewport3dOrbit` are bound in
`🗣️dsl/🪟️viewport/🦀️.rs`. Added: `dsl::DslRecord` on `RemodelingFrameCursor` and
`RemodelingLayerVisibility`. Two single-line struct bodies (remodel frames/report) were expanded to
multi-line so the field attributes have somewhere to sit.

Workspace check before → after, crates that fail to compile:

```
before: os-mcp, os-run, server, hub, architect-program, cad-cad, draw-drawing, forms-forms,
        layout-layout, mathematical-equation, note-note, remodel-remodeling, sequence-sequence,
        space-home, playbook-procedural, trinity-jack-shell     (16)
after:  os-mcp, os-run, server, hub, cad-cad, draw-drawing, mathematical-equation,
        space-home, playbook-procedural, trinity-jack-shell     (10)
```

A strict subset: six crates went green, none regressed, and `DslField` appears in **zero** errors.

## 4. Verification — what was actually run

Every command below ran in the foreground from the repo root with
`RUST_MIN_STACK=33554432 NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false` and the shared cargo
build dir. Nothing here is inferred.

### 4.1 New laws (written this lane, all green)

| law | crate | result |
| --- | --- | --- |
| `window_config_pack_round_trips_every_persisted_option` | puzzle-3d | ok |
| `window_config_pack_round_trips_every_persisted_option` | puzzle-2d | ok |
| `window_configs_pack_round_trip_every_persisted_option` | puzzle-5d | ok |

Each builds a fixture that differs from `Default` in **every** field, asserts that, then asserts one
`ArtifactPack::encode_pack`/`decode_pack` round trip and one `ArtifactDsl::print_dsl`/`parse_dsl` round
trip both reproduce it exactly (`test-roundtrip.txt`, `test-puzzle-2d.txt`, `test-puzzle-5d.txt`).

### 4.2 Crate suites (counts as reported by cargo)

| crate | result |
| --- | --- |
| `semio-s-artifact-puzzle-3d --lib --features component-app-assembly` | **742 passed; 19 failed** (364.80s) |
| `semio-s-artifact-puzzle-2d --lib --features component-app-assembly` | **518 passed; 194 failed** |
| `semio-s-artifact-puzzle-5d --lib --features component-app-assembly` | **219 passed; 108 failed** |
| `semio-s-artifact-sequence-sequence --lib window` | 7 passed; 4 failed (187 filtered) |
| `semio-s-artifact-forms-forms --lib window` | 11 passed; 6 failed (173 filtered) |
| `semio-s-artifact-layout-layout --lib window` | 9 passed; 7 failed (332 filtered) |
| `semio-s-artifact-note-note --lib window` | 10 passed; 4 failed (373 filtered) |
| `semio-s-artifact-remodel-remodeling --lib window` | 17 passed; 4 failed (1266 filtered) |
| `semio-s-artifact-architect-program --lib window` | 10 of 24 ran, then SIGABRT (§5.3) |

Architect's own codec law is in that green set and is the strongest single check of this migration:

```
test …register::config::window_ownership_tests::architect_window_ownership_matches_the_neutral_fixture_and_codecs
[DEBUG] Architect Register, Adjacency, Graph, and Report configs matched the neutral fixture,
        inverse, DSL, Pack, text-op, and binary-op laws
ok
```

## 5. What is still red, and whose it is

### 5.1 The retained window-config **reload** is broken framework-side, not by these codecs

`puzzle3d …unit_tests::window_options_are_local_to_the_window_instance_not_shared_across_split_panes`
fails at `🔬️unit/🦀️.rs:3518`, the re-render after `load_window_config_pack`: the reopened app renders
`"gridFactor":10.0` (the default) where the source app renders the dispatched `2.5`. `load()` itself
returned `Ok` — nothing was rejected.

It is **not** this lane's encoding, on four independent measurements:

1. the §4.1 round-trip law proves `encode_pack`/`decode_pack` and `print_dsl`/`parse_dsl` are lossless
   for a fully non-default `Puzzle3dWindowConfig`;
2. `WindowConfigOwnerEntry::packs()` (`🎚️config/🦀️.rs:459`) encodes
   `envelope.vcs.initial_snapshot.encode_pack()` — the **initial** snapshot, i.e. `Default`, identical
   under the old and new encodings. The dispatched `2.5` is restored only by the `.spr` op-log replay
   (`RetainedHistoryDecode` → `RetainedConfigStoreHydration`), and this lane changed neither the
   mutation's `OpText`/`OpBinary` (still JSON) nor `Mutation::diff`/`inverse`;
3. `semio-s-artifact-procedural-generation2d`, migrated to the same record-backed shape **before** this
   lane, fails its own
   `generation2d_window_camera_ownership_runtime_isolates_routes_renders_and_reopens` with
   `registered fixture typed operation did not retire within 30 seconds` (`test-gen2d.txt`);
   `semio-s-artifact-flow-flow` fails
   `flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows` with
   `document store close awaits a retained reader or owner` (`test-flow.txt`);
4. the loader carrying it — `🔌️plugin/🪟️window/🎚️config/📥️retained/` and
   `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/` — is **untracked** (`git status`: `??`),
   i.e. new, uncommitted, in-flight peer work. The committed `load()` it replaced was a different
   implementation.

The same shape accounts for the window-ownership law failures in sequence, forms and remodel.
**Owner: whoever holds the uncommitted `🔌️plugin/🪟️window/🎚️config/` packet** (that tree's
`🦀️.rs` last written 05:28, `🧪️tests/📥️retained-pack-load/🦀️.rs` modified, `📥️retained/` untracked).

### 5.2 Framework-level failures unrelated to window configs

Dominating the puzzle-2d/5d counts and present across crates:

* `tool factory key 's.puzzle.puzzle2d@1/*#editor/import-media' is already registered`
  (`interactive-job.owner-registration`) — 29 in 5D, 24 in 2D;
* `apply: ValidationFailed("edit history insertion requires its exact mutation retirement factory")`;
* fixture-projection diffs, `mounted fill pump fault`, `IconPaintCache … terminal-empty`.

Puzzle 3D's 19 are the same family (mesh announcements, brush-preview lanes, catalogue paging, the
document-independent publication ladder). They are **not stable**: two full runs of the identical tree
produced overlapping-but-different failure sets (3 dropped out, 4 appeared), so that lane is flaky as
well as red. `📌️panels/🛍️catalogue/`, `🗣️terminology/`, `🎮️commands/📥️import-fixture/` and
`🎮️commands/🧵️retained/` in the puzzle tree are all peer-modified in the working tree and were not
touched here.

`note …neutral_window_schema_round_trips_match_the_serde_json_oracle` fails on its **first** line —
`serde_json::from_str(include_str!(…🧫️fixtures/🔣️.json))` →
`invalid type: map, expected a sequence`. A fixture/struct mismatch that never reaches any codec.

### 5.3 Architect SIGABRT

`architect_window_ownership_runtime_isolates_renders_reloads_and_closes` overflows the stack of its own
deliberately 2 MiB `architect-window-ownership-law` thread right after
`[DEBUG] … constructed the first registered app`, aborting the binary and cutting the run at 10 of 24
tests. The three sibling laws in the same module — including the codec law quoted in §4.2 — pass
first. Not diagnosed further; it is framework app-construction growth, not a codec path.

### 5.4 `semio-s-artifact-draw-drawing`

Still red, **8× `E0433: cannot find module or crate serde_json`**, all in
`…/🖼️canvas/🫧️transient/🦀️.rs` — the window **transient**, unmodified from HEAD, while
`serde_json.workspace = true` sits in that crate's manifest. Present in the baseline workspace check
(as 17 errors, of which one was the `DrawingCanvasWindowConfig` bound this lane removed). Pre-existing,
committed-state, not this lane's.

### 5.5 `semio-s-artifact-puzzle-5d` test target, fixed in passing

The 5D lib test target did not compile at HEAD: `🧪️tests/🔬️unit/🦀️.rs` called `context::meta`,
`context::new_app`, `context::new_app_with_registry` and `context::project_and_retire_fixture_tree`
**from inside its own `pub(crate) mod context`** — 4× `E0433`, the signature of a botched unqualify
sweep. Repointed at `semio_framework_plugin::artifact_app_laws::…`, matching the 2D and 3D siblings
verbatim. Without it none of 5D's 327 tests could run.

## 6. Files changed

Framework (both are live peer files; the edits are additive and nothing of theirs was reverted):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `extern crate … as dsl`, `dsl::DslRecord` on `WorldSunConfig` + `WorldProjectionConfig`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `session_identity_view()` cfg pair

Puzzle 3D (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`):

- `✏️editor/🪟️window/🦀️.rs`, `✏️editor/🎚️config/🦀️.rs`, `✏️editor/🦀️.rs`,
  `✏️editor/📌️panels/🔍️inspection/🦀️.rs`, `✏️editor/📌️panels/🗿️artifact/🦀️.rs`,
  `✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`, `✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs`

Puzzle 2D / 5D:

- `…/◻️2d/…/✏️editor/🪟️window/🦀️.rs`, `…/◻️2d/…/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs`
- `…/🖐️5d/…/✏️editor/🪟️window/🦀️.rs`, `…/🖐️5d/…/✏️editor/🎚️config/🦀️.rs`,
  `…/🖐️5d/…/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs`, `…/🖐️5d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`

Sweep (one `🎚️config/🦀️.rs` each unless noted): sequence `📽️main`, forms `▶️try`, layout
`📐️blueprint`, note `✏️editor/🪟️window/🦀️.rs`, draw `🖼️canvas`, remodel `🖼️frames`/`📊️report`/`🧊️model`,
architect `↔️adjacency`/`📋️register`/`📓️report`/`🕸️graph`.

Ticket:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🐍️window-config-dslfield-forward.py` — **new**, retained input script
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️fix-forward-puzzle3d-dslfield-2026-09-13.md` — this report

Not touched: `🔌️plugin/🪟️window/🎚️config/🦀️.rs`, its `📥️retained/`, and
`🏪️store/🎚️config/` — the peer's live packet, read only. CAD's `CadSunConfig`/`CadProjectionDsl`
mirrors, now redundant, were left in place for their own lane.
