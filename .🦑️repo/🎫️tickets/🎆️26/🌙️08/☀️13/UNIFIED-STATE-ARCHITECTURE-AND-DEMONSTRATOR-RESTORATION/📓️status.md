# Status — Unified State Architecture and Demonstrator Restoration

Plan: `/Users/ueli/.claude/plans/the-following-architecture-must-happy-possum.md`.
Reports: `📓️cw1-report.md` (composition), `📓️a0-report.md` (vocabulary SSOT),
`📓️a1-report.md` (presence/transient lanes), `📓️d2-d3-report.md` (demonstrator IO + panes).

## Verification as of this writing

```
semio-framework-os-kernel --lib   835 passed / 1 failed   (pre-existing fixture-sweep, sibling churn)
semio-framework-plugin    --lib   155 passed / 0 failed
semio-s-plugin-stdio      --lib  2442 passed / 5 failed   (the 5 documented pre-existing)
semio-s-plugin-cad                140 passed / 0 failed
```

## Landed

### Composition is real (CW1 + CW2 partial)
Children are now genuinely their own envelopes with their own version history, end to end:
production `TypedChildStoreFactory`, the object-safe `SpaceMember` read surface
(`document_pack_bytes`/`envelope_pack_bytes`/`pack_at_checkpoint`), the first production
`LinkResolver` (`MemberLinkResolver`, with `LinkPin::Checkpoint` resolving real history), child
persistence over the channel (`LoadChildren`/`ReadChildren`/`Children`, `CHANNEL_VERSION` 6), the
checkpoint pin cascade with a pending-pin queue, and `ArtifactView.children` — a read seam that
cannot go stale by construction, replacing the `thread_local!` caches that could.

**Two real defects found by testing the documented behaviour rather than reading the code:**
1. The durable `.spr` form persisted **no** composition facts (`owner`, `dialect`,
   `composition_pins` were all dropped on parse). A reloaded child forgot it was owned and could not
   be typed. Fixed with a non-critical `REC_COMPOSITION` extension record.
2. `DerivedArtifactComposer::reads()` memoized in a `static` inside a **generic function** — which
   Rust does not monomorphize. One artifact kind's answer was served to every other kind, by call
   order. Fixed with a `TypeId`-keyed table; verified stable across repeated runs.

stdio now registers a real child-store factory for the `semio` artifact, dispatching across all 18
composable subsets (one factory, because `ArtifactKindId` is strictly three segments — the subset
lives in the dialect, and `open` recovers it from the newly-persisted overlay).

### Four state mechanisms exist and are typed (A0 + A1)
`StateClass` is strictly `{ Artifact, Config, Presence, Transient }` (2,396 files swept across all
five schema formats); `Inferred` left the enum for its own orthogonal `#[derived]` axis; `Effect`
deleted. Taxonomy gained `🫧️transient`, `modeChildDirs`, and window config/transient, with 238
window + 179 mode dirs scaffolded and a new `policyModeCompletenessBreaches`.
`PresenceStore` (LWW roster) and `TransientStore` are wired through `EphemeralEmit` /
`ArtifactApp::ephemeral`, proven not to leak into history or be rewound by undo.

### Demonstrator (D2 + D3)
The 12 IO registrations the demonstrator made for kinds it does not own moved to 📐️cad and 🌍️gis
(which also fixes standalone `cad-play` having had no solid/mesh IO at all). The six panes dissolved
into `🎛️apps/🦀️component.rs`; `🎪️panes/` deleted; policy ratcheted from downgrade to ban.

## Demonstrator — BOOTS AGAIN ✅ (see `📓️d5-demonstrator-boot-report.md`)

Verified in-browser on :6029 — six panes, **0 crashed**, 5 live canvases, Generator opens to a live
node graph driving a 3D preview, guided tour runs. Three stacked causes, none of them the one the
earlier waves predicted:

1. **The disk was full** (257 MB free of 926 GB). That killed the dev server's esbuild service →
   `📦️index.tsx` 500 → blank page, AND made `cargo` fail with `No space left on device` — which is
   why procedural's error count appeared to swing 94 → 16 → 116. Freed **202 GB** from regenerable
   caches only (`target/debug/incremental` 80 GB + the one closed ticket's `🎯️target`). The four
   open peer tickets' `🎯️target` dirs (283 GB) were left alone deliberately.
2. **`appBreadcrumb(breadcrumb.join(…))` on an optional field.** `AppDefinition.breadcrumb` is
   declared optional; every consumer dereferenced it unguarded, inside `FrameworkOsShellInner`'s
   render — so one app without a breadcrumb killed the whole shell and every pane with it. Fixed +
   regression test.
3. **A 5-day-old dev server.** Restarted.

**Procedural + demonstrator crates repaired: 105 → 0 and 14 → 0** (see
`📓️procedural-repair-report.md`). Nine defect classes, all with exactly one correct answer —
namespace collision, stale slot names, missing path segments, `payload`→`self`, mis-scoped helper
imports, a missing `SynapseSpec` import, an absent `generation_mutation_to_procedural2d` bridge, and
the deleted `SetWidget`/`Generation` vocabularies migrated to their semantic replacements. Plus
stdio's `CsvSnapshot` reshape carried into playground's CSV io (which had been silently dropping the
schema on every round trip).

`semio-s-plugin-demonstrator` **19/19**, including the three migrated bundle tests that had never
been run-verified — one of which caught a **real bug**: `puzzle3d-play` published an empty manifest
`io` because `create_puzzle3d_app()` never called `.io(..)` on the builder, so no host could route a
document to that surface. Fixed by sharing one `puzzle3d_io()` between the trait method and builder.

Forced fresh WASM build (`FORCE_PLUGIN_BUILD=1`) now unblocked and running.

## Flagged for the user — a 4,400-line unmounted ghost

`🧰️framework/🛍️products/💻️os/🦀️component.rs` holds ~110 `semio_framework::` references and a full
copy of the escape-hatch registries (`register_solid_exporter`/`register_solid_importer`/
`register_dwg_import_handler`) that were **deleted today**. The DKM session reports **nothing mounts
it** — verified by realpath, not grep, so `#[path]` aliasing is accounted for.

It is squarely in this ticket's lane: dead code carrying a duplicate of a removed mechanism is
exactly what the state-architecture work exists to eliminate. **I did not delete it**, for two
reasons: "unmounted" is a claim about the *current* mount graph, and three sessions rewrote `#[path]`
mounts today (one file physically moved between trees mid-build); and deleting 4,400 lines is a
call for the user, not an inference. Recorded here as a documented candidate — **needs a decision**.

## Not started

- **A2/A3 presence/transient PRODUCERS** — the lanes are real and tested, but no app emits into them
  yet, and the host-side cursor/viewport heartbeat is unwritten.
- **A4** `OsShellConfig` — the four `🖥️platform` localStorage stores and wgpu prefs still bypass the
  config lane.
- **A5** mode/window scoped facets have dirs but no content; `window_measures` still reads flat app
  config.
- **A7 is now LANDED** (see `📓️a7-report.md`): `policyStateLaneExhaustivenessBreaches` measures every
  route around the four mechanisms — **117 breaches** (110 `ephemeralBox*`, 7 storage-outside-the-
  config-lane), each with a per-site solution, plus corrected `plugin-purity` guidance that routes a
  `thread_local!` to the lane matching what the state actually IS. Report-mode by design; the full
  policy run is unchanged at 23,866 high-priority across 30 rules. Flipping either sub-kind to
  gating is the closing move **after** A4 burns the count down to 0.
- **CW2 remainder** — the composed plugins still carry `thread_local!` child caches. The seam they
  were waiting for now exists, but deleting them is **blocked on a design decision, not effort**:
  `ArtifactDsl::print_dsl(&self)` and `ArtifactPack::encode_pack_with(&self, …)` take no context, so
  a codec structurally cannot resolve child content — which is the actual reason the caches exist.
  The question ("should a parent's serialized form carry its child's resolved content, or only the
  handle?") and the recommended answer are written up in `📓️cw2-child-cache-finding.md`. I stopped
  rather than half-migrating, because moving app read sites while codecs still expect resolved text
  would leave the cache in place AND add a second read path.
- **CW5** — puzzle, procedural, sourcing (three demonstrator panes) are still unmigrated to
  composition.
- The **TypeScript channel twin** is at `APP_CHANNEL_VERSION = 4` vs Rust 6 — pre-existing drift
  (it never implemented v5 either), deliberately not papered over.
