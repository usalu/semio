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

## CAD Example Loading Regression

CAD's production `ArtifactApp` now converts its declared host actions into the closed typed
`CadCommand` vocabulary. The conversion had previously existed only in `#[cfg(test)]`, so production
rejected `setActiveExample` and left all four CAD 3D windows empty. The CAD suite passes 141/141 with
the new production-bridge regression test. Full diagnosis, repair, and the post-repair WASM build
contention boundary are recorded in `📓️cad-example-action-bridge-regression.md`.

## Sourcing Action Bridge — the same defect as CAD, one pane over

The Aussuchen pane could not load its `demo-stock` example: `🪵️sourcing`'s curate app declared 15
command rows and **zero** `command_from_action` implementations, so it inherited the trait default
that rejects every app-owned action. Fixed with a production `sourcing_curate_command_from_action`
joining the manifest's camelCase arg names to the payloads' snake_case fields, verified against what
the renderer actually dispatches (`{value}` / `{delta}` / `{pressed}`, and drag payloads spread from
the row's own `{objectId}`). `semio-s-plugin-sourcing --lib` **80 passed / 0 failed**, including the
framework's own `assert_declared_actions_bridge_to_commands` harness.

The same census across `✏️s` found **34 more apps with 442 declared command rows and no production
bridge** — none of them a demonstrator pane, but all latent UI-dead-on-arrival. Full diagnosis, the
reason this must NOT be generated inside `app_commands!`, and the per-app remedy (lead with the
framework law, which *measures* instead of assuming) are in `📓️action-bridge-defect-class.md`.

### Proven in the browser on freshly built WASM ✅

`.core.wasm` rebuilt today 11:44, dev server on :6029, boot with **250 resources and zero 404s**.
The **Aussuchen pane works end to end**:

- the `demo-stock` example loads — the Pool window lists all ten stock components with module,
  typology, availability and curate steppers, and the title bar's example selector reads
  *Beispielbestand*. This is the exact surface that was dead.
- clicking `+` on *Glulam GL24h 200×400* took its curated count **0 → 1**, the **Kuratiert** window
  picked the row up, and the **Raster** window renders real 3D beam geometry. That is a full round
  trip — chrome action → the new bridge → typed command → WASM handler → mutation → snapshot →
  re-render — with `delta` supplied by the host and merged onto the row's own `objectId`, which is
  precisely the host-filled-args path the bridge was written to read defensively.
- the old `action 'setActiveExample' is not a framework-reserved action` fault is **gone** from the
  console.

The `engagementPointerDown` alias deletion is likewise live (cad 137/137).

### Koordinator (cad) — geometry still absent, OPEN and NOT diagnosed

On a **clean** dev server with fresh WASM (250 resources, zero 404s at boot), all four CAD windows
(Form, Energie, Gebäude, Tragwerk Klassisch) render their grid and axis gizmo, each with a live
628×326 canvas, and the title bar's example selector reads *Sechseckig geschnittener Betonwald links* —
but **no geometry is drawn**. This reproduces after a clean restart, so it is a real defect and not
the degraded-server state that produced the earlier 404 noise.

It is explicitly **not** the action bridge: cad's bridge is present, its suite is 137/137 including the
framework harness, and the example reaches the selector.

**Root cause found — an empty `Vec` vanishes from the wire while TypeScript promises an array.**

Two earlier suspicions were both eliminated first: it is not the D2 IO move (cad's example is a
built-in document, so drawing it needs no file IO at all), and it is not the peer's
`world3d_scene_extended` signature change (cad's call site was already updated at 11:14, before the
11:44 build).

The real defect is in the manifest SSOT. `AppDefinition`'s `Vec` fields carried
`#[serde(default, skip_serializing_if = "Vec::is_empty")]`, so **an app that declares no commands
emits no `commands` key at all** — while the generated TypeScript declares
`commands: Array<CommandDefinition>` as *required*. `appOwnsCommand` then evaluates
`app.commands.some(…)` on `undefined`, which is precisely the observed
`TypeError: Cannot read properties of undefined (reading 'some')`. The console named the trigger:
`setContributions command failed demonstrator unknown cad action 'setContributions'` — the
demonstrator pushing a host command at cad, an app with an empty `commands` vec.

The codebase's own convention exposes the inconsistency: every skipped `Option` field carries
`#[cfg_attr(feature = "typegen", ts(optional))]` (`default_layout`, `introduction`), but **none of the
skipped `Vec` fields do** — so the emitted JSON and the generated type disagree for eleven fields
(`utilities`, `tools`, `commands`, `interactions`, `named_layouts`, `terminologies`,
`terminology_breadcrumbs`, `tutorials`, `dialogs`, `media_inputs`, `media_outputs`, `artifact_kinds`).

**Fixed at the schema, not at the call sites:** the `skip_serializing_if` was removed so the wire
always carries the arrays the existing generated type already promises. No typegen regeneration is
needed (removing the skip does not change the TS type), and `#[serde(default)]` is retained so absent
input still deserializes. The alternative — marking them `ts(optional)` and adding null guards in JS —
was rejected: it pushes the obligation onto every consumer in perpetuity and invites the same class
back, which is how `appBreadcrumb` took down the whole shell earlier in this ticket.

**The whole class was fixed, not just `AppDefinition`.** The mismatch proved systematic rather than
incidental: across the manifest SSOT, **0 of 31** `Vec`-skipping fields carried `ts(optional)`, while
**61** `Option`-skipping fields did — the discipline had been applied to `Option` and never to `Vec`.
All 31 now emit their arrays (verified: exactly 31 lines changed, the only untouched serde attributes
being the unrelated `transparent` and the `NonEmptyVec` `try_from`/`into` converter). `semio-framework`
compiles clean and `git status` on `🤖️generated/🟦️manifest.ts` is empty, confirming the wire moved to
match the type rather than the type being loosened.

Worth noting for whoever picks this up: this same class has now bitten three times in one ticket —
`appBreadcrumb` (killed the entire shell), `appOwnsCommand` (this one), and the two `render failed`
sites. A policy rule that flags a `skip_serializing_if` without a matching `ts(optional)` would make
it structurally impossible; that is the durable follow-up.

**A second sub-class, found because the fix broke a test.** `introduction_gesture_…` asserted that a
defaulted `button` and an empty `modifiers` were OMITTED. Checking the generated type before touching
the test showed the opposite: `button: IntroductionPointerButton` and
`modifiers: Array<IntroductionKeyModifier>` are both **required** there — so the test had been pinning
the defect. These fields skip via *default-valued* predicates (`is_left`/`is_right`/
`introduction_orbit_modifiers_is_default`) rather than `Vec::is_empty`, so the first sweep missed them.
Those skips are gone too: defaults are still *inferred on the way in*, but always *written on the way
out*, and the test now asserts that.

New law: `empty_collections_serialize_as_arrays_rather_than_vanishing_from_the_manifest` pins all
eleven `AppDefinition` collections as `[]`, the round trip, and the `#[serde(default)]` tolerance for
absent keys — provable without a browser or a working stdio.

Verification of the serde change:

```
semio-framework        --lib  106 passed / 0 failed
semio-framework-os-kernel --lib  862 passed / 1 failed  (the documented pre-existing fixture sweep)
semio-framework-plugin --lib  167 passed / 1 failed  (NOT caused by this change — see below)
```

`view_action_emitting_ops_is_rejected` fails because the kind-discipline check resolves a verb through
`registry.get_command`, which reads only `app_commands`/`mode_commands`, while the fixture declares
`badView` via `.view_action(..)` — so no kind resolves and nothing is rejected. That is the
action-versus-command vocabulary migration a sibling session is mid-way through (the same migration
that added a `CommandDefinition` import to sourcing's app file); this change is serialization-only and
touches no in-memory registry path, and `🔌️plugin/🦀️component.rs` has not been modified since 12:02,
before the manifest edits began. Left for its owner.

**Status: fix applied and compiling, NOT yet proven to restore CAD's geometry.** Proving it needs a
demonstrator rebuild, which is blocked by a peer mid-refactor adding a `source` field across stdio's
snapshot types (`Mp4Snapshot`/`SvgSnapshot`/`PdfSnapshot`, five initializers still lagging, last
touched 12:45). Left alone deliberately.

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
