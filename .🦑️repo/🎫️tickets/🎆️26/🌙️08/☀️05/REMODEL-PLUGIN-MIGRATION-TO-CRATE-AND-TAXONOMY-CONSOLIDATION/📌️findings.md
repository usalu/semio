# 📌️ Findings — 📸️remodel migration

Things the next agent (or TEMPLATE.md) should know. Ordered by how much time each would cost to
rediscover.

## 0. 🚨 The registry's `validateTaxonomyTree` is still V1-shaped and will mis-report every V2 plugin

`🔣️taxonomy.json` now carries `entryLocation: "packages"`, but
`🧰️framework/…/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/📜️script.ts` still does:

```ts
const libRsPath = join(pluginRoot, "📦️lib.rs");          // line ~845 — V1 location
…
const declaredAbs = new Set(declaredPaths.map((p) => join(pluginRoot, p)));   // V1 base for #[path]
…
findings.push(`${pluginId}: missing 📦️lib.rs at plugin root`);
```

Against a V2 plugin this produces a false `missing 📦️lib.rs at plugin root` plus one
`… is not declared by any #[path]` per component file (44 of them for remodel), because the entry file
moved into `📦️packages/🦀️rust/` and its `#[path]` strings now resolve relative to **that** directory,
not the owner root. Two-line fix: read the entry file from `join(pluginRoot, "📦️packages", "🦀️rust")`
and use that same dir as the `#[path]` resolution base. The audit is warn-only today so it does not
fail `check`, but it will the moment W4 promotes it to an error. **Not remodel's file to edit** — the
registry script is on the registrar's never-touch list (TEMPLATE §10).

A V2-aware standalone mirror lives in this ticket as `🔍️taxonomy-audit.ts` (also adds the tree-purity
sweep the V1 version had no concept of); it reports `taxonomy tree clean: 📸️remodel`.

## 1. 🚨 `📦️packages/🦀️rust/📜️script.ts` has one `../` too many in 30 of 32 migrated plugins

TEMPLATE.md §1's copy-paste snippet uses

```ts
from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️implementations/🟦️typescript/📦️index.ts"
```

but from `✏️s/🔌️plugins/<p>/📦️packages/🦀️rust/` the repo root is **five** levels up
(`🦀️rust` → `📦️packages` → `<p>` → `🔌️plugins` → `✏️s` → root), not six. Verified empirically:

```
$ cd ✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust && bun build ./📜️script.ts --target=bun
error: Could not resolve: "../../../../../../🧰️framework/…/📦️index.ts"
```

…while the same command in remodel's package (5 `../`) resolves. Every migrated plugin except
🕸️dag and 📸️remodel copied the six-level form, so `bun nx run @semio-tech/<p>-plugin:test` — the
`test`/`test-quick`/`test-long`/`test-exhaustive` targets `verify gate`'s `checkLeveledTestTargets`
insists on — currently cannot start for any of them. Count by plugin root:

```
5  📸️remodel, 🕸️dag
6  ✒️writer ➗️mathematical 🌀️procedural 🌊️flow 🌍️gis 🌿️vcs 🎞️animate 🎥️shooting 🎬️sequence 🏗️fem
   🏛️architect 🏭️process 💠️lowpoly 💡️reasoning 📋️forms 📏️layout 📐️cad 📕️norm 📖️playbook 📜️imperative
   🔋️energy 🔱️trinity 🖍️draw 🖨️raster 🗒️note 🧩️puzzle 🧱️block 🪐️space 🪵️sourcing
```

(🖍️draw/🔄️fsm and its ✨️macros sit one/two levels deeper, so their 6/7 are probably correct — check
per-crate, not by pattern.) **This is a one-character fix per plugin but it is not remodel's to make.**

## 2. 🩹 `DocumentApp::command_from_action` was never implemented for remodel — the whole manifest was dead from the host's side

The framework's default `command_from_action` returns
`"action '…' is not a framework-reserved action"`, and `VcsDocumentApp::dispatch_action` routes every
non-reserved action id through it. The pre-migration `remodel_ui` implemented `app_id`,
`document_schema`, `initial_projection`, `io`, `export_media`, `import_media`, `command_id`, `handle`,
`render` and `window_measures` — **but not `command_from_action`**. So the media drop zone, the layer
toggles and all ~40 command-palette entries could only ever be reached by a direct `dispatch_typed`
from a test; from the real host they all errored.

Forward-fixed here (`🎛️apps/📸️remodel/🦀️component.rs`, `🔖️ActionBridge` region) with an arm per row,
reading each arg leniently and falling back to the manifest's own declared default. Same class of gap
🧱️block found for its single `setCamera` action — but remodel's was total, not partial.
`testkit::assert_declared_actions_bridge_to_commands` is the test that catches it; **TEMPLATE.md §7
recommends it but does not say it will fail outright on a plugin that never wrote the bridge.** Worth
running against every already-migrated plugin — several likely have the same hole.

Two arg-shape subtleties the bridge has to absorb, both real:
* select-typed args arrive as **strings**, so a select whose option ids are numeric (`textureSize`,
  options `"1024"/"2048"/"4096"`) must parse into its `u32` field;
* the world-3d surface reports its camera as flat `{position,target,fov}` while `RemodelWorldCamera`
  itself serializes to `{camera:{…}}` — accept both.

## 3. 💥 The systemic broken-`Ok(` corruption (TEMPLATE §12.3) hit remodel's `🖱️ui` crate

`semio-s-app-remodel-ui` did not compile at HEAD: 32 `mismatched closing delimiter` errors, the exact
`Ok(Emit::operations(…)` / `Ok(Emit::default()` / `Ok(Emit::effect(…)` missing-paren pattern, plus a
missing `Fault` import. Instance **9** of the pattern repo-wide (after vcs, cad, shooting, sourcing,
animate, gis, puzzle-2d-ui, draw, norm). Forward-fixed as a natural side effect of porting `handle`
into `app_commands!` handlers — every arm now returns a properly closed `Result`.

Consequence for the baseline: remodel's ui crate contributed **0** tests to the pre-migration count
because it could not be compiled at all; its ~19 tests were recovered and now run.

## 4. 🐛 One genuinely-failing pre-existing test, unrelated to the migration

`⚙️engine/🦀️reconstruction.rs`'s `tests::long::video_in_yields_watertight_mesh_out` (a ~11-minute
end-to-end video→mesh contract test) fails at HEAD, before and after the migration, identically:

```
panicked at …: need >= 3 registered cameras to fit a Sim3 gauge alignment, got 2
[long] reached terminal status after 118 advance() calls: Done
[long] mesh vertices=32729 triangles=42924
```

The pipeline runs to `Done` and produces a mesh; only the gauge-alignment assertion at the end trips,
because the synthesized fixture registers 2 cameras where the Sim3 fit needs 3. That is a real
photogrammetry bug (or a too-weak fixture) in the engine's own test, **not** migration fallout — the
file was moved verbatim. Left as-is: guessing at the intended camera count would be fabricating a fix.
See `🧪️baseline-tests.txt` (pre) and `🧪️test-full.txt` (post) for the identical panic.

## 5. 🧭️ Engine topic-file split for a 10-crate subsystem stack

remodel's ten plugin-level module crates were a real DAG, not a flat pile. Folding them into
`🗿️artifacts/📸️remodel/⚙️engine/` as one topic file per pre-merge crate kept every internal path
byte-identical: each topic file gets a two-line header aliasing its siblings back to their old crate
names, e.g.

```rust
use crate::artifacts::remodel::engine::{camera as remodel_camera, feature as remodel_feature, images as remodel_image};
```

Rust 2018 uniform paths make a later `use remodel_image::X;` inside the same file resolve against that
alias, so **not one body line had to change**. Strongly recommended for any remaining heavy plugin
(📕️norm's 107 crates especially) — it turns a 28k-line merge into a `cp` plus one header per file.

The alias header also documents the DAG: `images` → `video`/`feature`/`dense`/… → `reconstruction`.
`motion` is deliberately NOT aliased into `reconstruction` — the pre-merge engine crate declared the
dependency in its `Cargo.toml` and never used it (`EngineParams::motion_enabled` is accepted but never
drives the motion code), which only became visible as an `unused_imports` warning once the crates
merged. Documented in place rather than silently kept.

## 6. ⏱️ Concurrent-session churn cost ~40 minutes

Two independent, unrelated breakages from the other session's in-flight framework/ui work blocked
every `cargo check` in the repo while they lasted:

* `semio-framework-ui-styling`'s `[lib] path` pointed at a `🤖️generated.rs` that did not exist;
* `semio-framework-ui-wgpu`'s `pub mod widgets` lost its `#[cfg(feature = "engine")]` guard mid-edit,
  producing 7 `unresolved import crate::{chrome,draw,input,layout,text,select}` errors.

Both are dependencies of `semio-framework-core`, i.e. of everything. Neither was diagnosable as "my
bug" without checking the file's mtime. The right move (per the master doc's own advice) was to arm a
poll loop and keep doing non-cargo work; both cleared on their own. Also note the root workspace was
red the whole time for a *third* reason — 📜️imperative's old crates are deleted while its member lines
are still listed — so `cargo check -p <anything>` was unusable and the TEMPLATE §3 `[workspace]`
overlay was the only way to build at all.

## 7. 🔀️ Shape V2 retrofit — what actually moved

The directive landed after remodel was already V1-green (check/clippy/tests/wire-diff all passing), so
this was the §14 mechanical retrofit rather than a from-scratch V2 build. 13 files moved, zero logic
lines changed:

```
📦️lib.rs                                      → 📦️packages/🦀️rust/📦️lib.rs
🎛️apps/📸️remodel/🦀️config.rs                  → 🎛️apps/📸️remodel/🎚️config/🦀️component.rs
🎛️apps/📸️remodel/🦀️terminology.rs             → 🎛️apps/📸️remodel/🗣️terminology/🦀️component.rs
🗿️artifacts/📸️remodel/⚙️engine/🦀️images.rs         → …/⚙️engine/🖼️images/🦀️component.rs
🗿️artifacts/📸️remodel/⚙️engine/🦀️video.rs          → …/⚙️engine/🎥️video/🦀️component.rs
🗿️artifacts/📸️remodel/⚙️engine/🦀️camera.rs         → …/⚙️engine/📷️camera/🦀️component.rs
🗿️artifacts/📸️remodel/⚙️engine/🦀️feature.rs        → …/⚙️engine/🌟️feature/🦀️component.rs
🗿️artifacts/📸️remodel/⚙️engine/🦀️sfm.rs            → …/⚙️engine/📸️sfm/🦀️component.rs
🗿️artifacts/📸️remodel/⚙️engine/🦀️dense.rs          → …/⚙️engine/🌫️dense/🦀️component.rs
🗿️artifacts/📸️remodel/⚙️engine/🦀️mesh.rs           → …/⚙️engine/🥽️mesh/🦀️component.rs
🗿️artifacts/📸️remodel/⚙️engine/🦀️motion.rs         → …/⚙️engine/🏃️motion/🦀️component.rs
🗿️artifacts/📸️remodel/⚙️engine/🦀️geo.rs            → …/⚙️engine/🗺️geo/🦀️component.rs
🗿️artifacts/📸️remodel/⚙️engine/🦀️reconstruction.rs → …/⚙️engine/🏭️reconstruction/🦀️component.rs
```

Notes for the next retrofitter:

* **Folder emojis were carried over from the pre-merge module dirs** (`🖼️images`, `🎥️video`, `📷️camera`,
  `🌟️feature`, `📸️sfm`, `🌫️dense`, `🥽️mesh`, `🏃️motion`, `🗺️geo`) so the mapping back to the deleted
  crates stays obvious. Only `reconstruction` needed a new one: its pre-merge dir was `⚙️engine`, which
  would have collided with its own parent folder — `🏭️reconstruction` instead.
* **Rust module idents did not change** (`config`, `terminology`, `images`, …), so not one `use crate::…`
  site anywhere in the crate needed touching. Only the `#[path]` targets in the entry file moved.
* **`#[path = "."]` on grouping modules stays exactly as-is** — it resets the base to the entry file's own
  directory, which is now `📦️packages/🦀️rust/`. That is precisely why all 41 leaf paths need the
  `../../` prefix and the 17 grouping resets need nothing.
* remodel had **no** `📚️examples`/`🧫️fixtures`/`🤖️generated`/`🛂️manifest.json`/`AGENTS.md`/`README.md`
  anywhere in its tree, so V2 point 3 and point 4 were both no-ops — **no AGENTS.md/README.md collision
  to flag.**
* Test count is the correctness bar and it held exactly: 376 before the retrofit (375 pass + the one
  pre-existing long-tier failure), 376 after (366 pass + 10 `long`-filtered).
