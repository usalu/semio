# 📓️ PX1 — peer framework diffs landed

Slice PX1 of ticket 26/09/18 OS-HUB-COLLABORATION-AI-END-TO-END.
Four framework defects measured by the peer session `26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP`
(`📓️cad-content.md` §11/§16, `📓️engineering.md` §9, `📓️media.md` §8, `📓️raster.md` §3), landed as root
fixes with laws in `semio-framework-plugin` / the sequence guest.

Started 2026-09-22 21:52 at load average **44.77**; source work first, one cargo at a time, private
`CARGO_TARGET_DIR=…/target-px1`, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=4` (load rule of the brief).

## 1. Retained window-config decoder — `Shape::Tuple` only (cad-content §11)

**Verified on disk, unchanged by me.** The cad agent's fix is present in
`/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs`
(mtime 2026-09-22 16:01), all three parts:

| part | line | state |
|---|---|---|
| `fn shape_is_tuple_valued()` — `Tuple \| Coord \| Dir \| Dim \| Range`, mirroring the whole-pack decoder's `is_tuple_shape` | 261 | present, with the provenance in its doc comment |
| `end_container` uses it for the tuple-vs-list decision | 460 | present (`… \|\| Self::shape_is_tuple_valued(self.expected()?.shape.as_ref())`) |
| `child_element` gives a coord/dir/dim/range's elements `Shape::Float` | 272–286 | present |

**cad's number, quoted from their report, not re-measured by me:** `📓️cad-content.md` §14 `run20`
(2026-09-22 21:38, log `.🧬semio/🦑️repo/⚡️cache/play-fleet/cad-content/run20.txt`) —
`semio-s-artifact-cad-cad --lib` **432 passed / 0 failed / 1 ignored**, plus `semio-s-plugin-cad` 5/0,
`aec-building` 8/0, `aec-building-energy` 2/0, `aec-building-structure` 2/0, `spatial-shape` 2/0.
I did not re-run the cad suite (fleet load); see §6.

**The law I added** — the defect was reachable only through a CAD-owned state, so the framework crate
had no law for it. `…/🔌️plugin/🪟️window/🎚️config/🧪️tests/📥️retained-pack-load/🦀️.rs`:
- `RetainedLoadCameraConfig` gains `#[dsl(coord)] pub eye: [f64; 3]` — the exact `Shape::Coord(3)`
  shape CAD's `CadCamera::position`/`::target` carry, so ALL THREE existing retained-load laws
  (baseline reopen, saved-camera round trip, bound refusals) now drive a coordinate through the
  decoder, not only the new one;
- new law `window_config_retained_pack_load_reloads_a_coordinate_valued_window_state`: saves a state
  whose coordinate differs from `Default`, closes the registry, reloads the Pack in a fresh registry,
  and asserts (a) the load succeeds at all, (b) the reopened coordinate is exactly the saved one,
  (c) no other field drifts, (d) the Pack+SPR bytes are byte-identical across the reopen;
- fixture `…/🎚️config/🧫️fixtures/📥️retained-pack-load/🔣️.json` gains `savedCamera.eye`
  `[18.75, -4.5, 33.25]` and a 13th `requiredScenarios` row `coordinate-valued-state`
  (the scenario-count assertion moved 12 → 13).

Without the decoder fix this law fails at the LOAD with `WindowConfigPackLoadDiagnostic::TypedState`
(`window-config.typed-state`), which is exactly the fault CAD measured.

## 2. `ChildRestoreProjection::from_snapshot` bypasses the app's own projection (engineering §9)

**Landed.** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`,
the `AwaitingMembers` arm of the store-replacement maintenance step (was `:23585`, now `:23590`):

```rust
-let projection = store::ChildRestoreProjection::from_snapshot(candidate.snapshot_ref()).map_err(|_| plugin_sdk_fault("candidate parent child projection is invalid"))?;
+let projection = A::child_restore_projection(candidate.snapshot_ref())?;
```

with the provenance in an inline note. `A` is the app of the enclosing
`impl<A: ArtifactApp, M: SpaceMember + MemberFactory> VcsArtifactApp<A, M>`; the sibling genesis path
in the same impl already reads `A::genesis_child_pack(snapshot, …)` off the same `candidate.snapshot_ref()`.
This is the one call site that erased its `ChildRestoreProjectionError`; the other four
(`:7150`, `:22994`, `:23779`, `:23830`) already format the error into their message.

**The law I added** — `…/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`:
`retained_composed_replacement_candidate_projection_is_the_apps_own_answer`. The fixture's candidate
parent is the very snapshot the sibling (green) replacement laws drive to publication, so the
STRUCTURAL projection accepts it; `ComposedParentApp::child_restore_projection` refuses only the
sentinel revision `COMPOSED_PARENT_PROJECTION_REFUSAL_REVISION = -7` with its own code
`test.parent-projection` and its own message. The law drives the replacement to `AwaitingMembers`,
takes one maintenance step, and asserts the step returns that app-owned code AND message — a pump
that still called the structural projection would answer progress here instead of a fault, so the
law fails closed on a regression. It then cancels, drains to `Complete`, asserts the live parent was
never displaced, acknowledges and closes.
Supporting refactor in the same file: `retained_composed_replacement_fixture` is now a thin wrapper
over `retained_composed_replacement_fixture_at_revision`, whose candidate half is split out as
`composed_replacement_candidate` (no member ingress, so a law that never reaches member admission
leaves no `OwnedDocumentMemberIngress` owner to hand off — its `Drop` asserts exact handoff).

## 3. Sequence guest: no resumable reserved builder for `steps:in` (media §8, S10-E)

**Landed, and it needed a framework change first.** The peer's diagnosis was right but incomplete:
`VcsArtifactApp::dispatch_import_media` (`🔌️plugin/🦀️.rs:29379`) builds its emit EXCLUSIVELY from the
resumable job — `A::import_media` is never reached on a mounted app — and
`ArtifactReservedToolJobRequest` carried `snapshot`/`config`/`history` but **not the composed
children**. Sequence's live scene is NOT its own snapshot: `import_media` reads it through
`sequence_host_snapshot_from_children(doc.snapshot, &doc.children)` off the `content` Flow child. So a
composed app structurally could not write a resumable importer at all.

Framework (root fix), `…/🔌️plugin/🦀️.rs`:
- `ArtifactReservedToolJobRequest` gains `pub children: ChildContentView` (`:15084`), documented with
  the failure it fixes. `ChildContentView` is `#[derive(Clone, Default)] { Option<Arc<ChildContentRoot>> }`
  — a cheap clone with no `Drop` witness;
- both construction sites populate it from the live root: `build_artifact_reserved_action_job`
  (`:26861`) and `build_artifact_reserved_media_job` (`:26892`), via
  `ChildContentView::clone(&self.child_content_root)` — the same clone `media_fingerprint` already
  makes to build its `ArtifactView`.

Guest, `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:
- new `//#region 🎞️ReservedImport` (before `#region 🔖️SequencePlayApp`): `SEQUENCE_IMPORT_TOOL_ID`
  (`"import-media"`), `SEQUENCE_IMPORT_PORT` (`"steps:in"`), `struct SequenceImportJob` +
  `impl semio_framework_job::InteractiveJob` + `impl semio_framework_plugin::ArtifactReservedJob`,
  following `generation2d`'s `Generation2dImportJob` line for line (two bounded steps: decode →
  publish through the completion authority; `sequence_job_payload` / `sequence_job_fault` helpers);
- `ArtifactEditor::build_reserved_tool_job` on `SequencePlayApp`: answers `Ok(None)` for any other
  `tool_id`, refuses a non-empty `raw_wire`, destructures `ArtifactReservedToolInput::Media`, and
  hands back `ArtifactReservedToolJob::new(SequenceImportJob::new(request, port, media))`.
- The decode is ONE step and reproduces `import_media`'s body exactly (object payload verbatim, bare
  scalar wrapped under `"value"`, `step-<n>` minted off `max_serial_in_snapshot(…).max(100) + 1`,
  `kind = "computation.import"`, x placed 280 to the right of the rightmost step) and releases the
  children view the moment it has decoded.
- Close ladder retires, in order: a rejected completion's emit → the decoded emit's child emits
  (`Emit::close_child_one`) → children view → media json → port → snapshot (`Blocked` while
  `Arc::strong_count == 1`, mirroring generation2d) → completion (`Blocked` without a mounted
  consumer). `terminal_is_empty` requires every one of them empty.

## 4. `ImageWindowKit::render` puts a `data:` URL in a 512-byte `UiText` (raster §3)

**Landed.** `…/🔌️plugin/🦀️.rs` `#region 🔖️ImageWindowKit` (`:33054`+):
- two new constants — `IMAGE_WINDOW_PIXELS_LANE_KEY = "framework.window.image.pixels"` and
  `IMAGE_WINDOW_PAGED_NODE_KEY = "framework.window.image.view"`;
- `render` now formats the data URI into a `String` first. **If it fits one `UiText`
  (`UI_TEXT_MAX_BYTES` = 512) the node is byte-identical to what it always produced.** If it does not,
  the window body becomes a `column()` keyed `framework.window.image` with exactly two children: the
  image node (src = the lane key, alt unchanged) and `paged_text_carrier(IMAGE_WINDOW_PIXELS_LANE_KEY,
  &source)` — the same out-of-doc lane shape `scene_surface` (`:536`) uses for a world scene, and the
  shape `artifact_app_laws::built_carrier_text` / the React Interpreter's `surfaceSceneLaneText`
  already reassemble. A wrapping container is required because `image()` builds on `NodeBase::leaf()`
  and cannot own children.

**The law I added** — `…/🔌️plugin/🧪️tests/🔬️app-window-kits/🦀️.rs`:
`image_kit_pages_a_composite_larger_than_one_ui_text_out_of_the_doc`. A 2 048-byte base64 composite
(4 × `UI_TEXT_MAX_BYTES`): asserts the render SUCCEEDS (it used to fail
`ui.fixed-capacity … image-window.source`), the body key is unchanged, there are exactly two children,
the image `src` names the lane, the alt survives, the carrier really pages (more than one child), and
`built_carrier_text(carrier)` reassembles the EXACT data URI. The existing small-image law
`image_kit_renders_data_uri_from_base64` is untouched and still exercises the inline path.

## 5. Verification runs

Every run: repo root, foreground, ONE cargo at a time, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=4`,
private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-px1`, shared build dir untouched.
The machine was NOT quiet: load average was 44.77 at 21:52, 35.5 at 22:05, 33.2 at 22:16 and 43.1 at
22:26, so the load rule's "source work first" window was used for all four items and the builds ran at
load 39–46 with the peer fleet's 6 rustc live. Captures in `🗑️generated/px1-*.txt`.

| run | command | result |
|---|---|---|
| 22:27–22:29 (2m00s) | `cargo check -p semio-framework-plugin --tests` | **`Finished`, 0 errors**, 376 warnings |
| 22:29–22:30 | `cargo test -p semio-framework-plugin --lib -- --test-threads=1` | **830 passed / 1 failed** — my own new image law, see below |
| 22:30–22:34 (21.28s run) | same, after the fixture correction | **831 passed / 0 failed / 0 ignored**, 394 warnings |
| 22:35 (1.22s run) | `cargo test -p semio-s-artifact-sequence-sequence --lib` | **207 passed / 0 failed / 0 ignored**, 187 warnings |

- The plugin lib went **827/0 (FP13's baseline) → 831/0**. All three new laws pass by name in the
  rerun log: `window_config_retained_pack_load_reloads_a_coordinate_valued_window_state ... ok`,
  `retained_composed_replacement_candidate_projection_is_the_apps_own_answer ... ok`,
  `image_kit_pages_a_composite_larger_than_one_ui_text_out_of_the_doc ... ok`.
- **The one red was mine and is closed.** My first image law asserted `carrier.children.len() > 1` on
  a 2 KiB composite, but `section_text_chunks` packs `UI_TEXT_MAX_BYTES * (1 + UI_FIXED_LIST_ITEMS)`
  = 512 × 33 = 16 896 bytes into ONE leaf (value + 32 ascending `data_attributes`), so 2 KiB is a
  single node. The fixture is now `128 * UI_TEXT_MAX_BYTES` = 65 536 base64 bytes, which really does
  need several packed leaves. Nothing in the production fix changed.
- Sequence is **exactly the 207/0 the brief predicted** (media §8's 205/2 + the two S10-E tests). Both
  named reds pass: `import_media_steps_in_inserts_a_new_step_from_an_object_payload ... ok` and
  `import_media_steps_in_wraps_a_bare_scalar_payload ... ok`; `import_media_rejects_unknown_port` is
  still ok, so the builder did not widen the port surface.
- **Correction to the brief:** `semio-s-artifact-sequence-sequence` has NO `component-app-assembly`
  feature — `cargo test … --features component-app-assembly` fails with
  `the package 'semio-s-artifact-sequence-sequence' does not contain this feature`. Plain `--lib` is
  the right invocation and is what produced 207/0.

| 22:46–22:49 (3m13s) | `cargo check -p semio-s-artifact-raster-raster --all-targets` | **EXIT=0, `Finished`**, 121 warnings |
| 22:49–22:53 (3m28s) | `cargo check -p semio-s-artifact-cad-cad --all-targets` | **EXIT=0, `Finished`**, 104 warnings |
| queued 22:36 | `cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest`, through `📜️wasm-build-mutex.sh px1` | **not run — the wasm mutex has been held by the peer `play` session since 22:17:48** (see §6) |

- **Second correction to the brief:** neither `semio-s-artifact-raster-raster` nor
  `semio-s-artifact-cad-cad` has a `component-app-assembly` feature either (they have no `[features]`
  section at all — that flag lives on the `semio-s-plugin-*` crates). Both were checked with
  `--all-targets` instead, which also type-checks their test code. The warning counts are the proof
  the tree really was compiled, per the fleet's "warnings as proof of type-check" rule.

## 6. Honest gaps

1. **The wasm32 guest check did not run.** `📜️wasm-build-mutex.sh px1 -- cargo check -p
   semio-framework-plugin --target wasm32-wasip2 --features component-guest` was launched at 22:36 and
   is still queued: `/tmp/semio-wasm-build.lock/owner` reads `play 22:17:48` — the peer session has
   held the mutex for over 35 minutes. No cargo of mine is running for it (only the wrapper waiting on
   the flock), so nothing of mine is parked in `prebuild_lock_exclusive`. The four changed framework
   regions are all target-independent (`#[cfg(wasm32)]` appears in none of them), but that is an
   argument, not a measurement: **whoever picks this up should rerun that one command.**
2. **None of the four fixes is verified in a browser.** Everything here is native laws only:
   - the retained-decoder fix still needs a describe + `activate-dev` before a served guest can reopen
     a window config pack (cad-content §16 item 1 already touched `🗑️generated/activate.request/demonstrator`);
   - the sequence importer likewise only reaches a pane after a re-activation.
3. **The image lane has no host-side reader yet.** `ImageWindowKit::render` now publishes an oversized
   composite as `framework.window.image.pixels` pages, and the Rust twin `built_carrier_text`
   reassembles it (the law proves that), but the React Interpreter renders `Component::Image` from its
   `src` directly — it does not yet resolve a lane key for an image node the way
   `surfaceSceneLaneText` resolves a surface's lanes. **So an oversized raster composite now ASSEMBLES
   instead of refusing the whole window, but will not paint until `📺️renderer` learns the lane.**
   That file belongs to another slice; the framework half is what raster §3 asked for and what the
   law pins. Small composites are byte-identical to before, so nothing regressed.
4. **The TypeScript twin is not updated.** `…/🔌️plugin/🪟️window-kits/🖼️image/🟦️.ts` `renderImage` still
   inlines the data URI unconditionally. Its own header already declares a deliberate divergence from
   the Rust twin and says no production code calls it yet; there is no TS `pagedTextCarrier` to mirror,
   so bringing it across is a separate piece of work, not a one-line edit. Recorded, not done.
5. **cad's 432/0 is quoted, not re-measured.** I verified the decoder fix byte-for-byte on disk and
   added the framework-side law; re-running `semio-s-artifact-cad-cad --lib` under a load of 40–70 with
   the peer fleet live would have cost the slice its remaining budget. `cargo check -p
   semio-s-artifact-cad-cad --all-targets` is green, so nothing I changed broke cad's compile.
6. **`ArtifactReservedToolJobRequest` gained a field.** Every in-repo construction site is inside
   `🔌️plugin/🦀️.rs` (two) and both are updated, and `cargo check -p semio-framework-plugin --tests`
   plus the raster/cad/sequence checks are green — but a guest crate outside those four that
   constructs the request literally would now fail to compile. A repo-wide grep found no such site.

## 7. Files changed

Production:
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
  — `A::child_restore_projection` at the replacement pump's `AwaitingMembers` arm (§2);
    `children: ChildContentView` on `ArtifactReservedToolJobRequest` + both construction sites (§3);
    `IMAGE_WINDOW_PIXELS_LANE_KEY` / `IMAGE_WINDOW_PAGED_NODE_KEY` + the paged `ImageWindowKit::render` (§4).
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  — `#region 🎞️ReservedImport` (`SequenceImportJob`, its `InteractiveJob` + `ArtifactReservedJob` impls,
    the two payload helpers) and `ArtifactEditor::build_reserved_tool_job` (§3).
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs`
  — **not changed by me**; verified on disk as the cad agent left it (§1).

Laws and fixtures:
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/📥️retained-pack-load/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧫️fixtures/📥️retained-pack-load/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-window-kits/🦀️.rs`

Captures (`…/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/`): `px1-check-plugin-tests.txt`,
`px1-test-plugin-lib.txt`, `px1-test-sequence.txt`, `px1-check-raster.txt`, `px1-check-cad.txt`,
`px1-check-wasm-guest.txt` (queued).

No git-modifying command was run. No file outside my slice's four items was touched.
