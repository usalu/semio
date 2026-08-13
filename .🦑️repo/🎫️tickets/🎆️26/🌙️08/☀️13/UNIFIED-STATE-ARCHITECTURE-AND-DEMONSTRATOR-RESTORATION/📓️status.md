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

## Blocked — on other sessions, not on us

- **Demonstrator fresh build + boot proof (D5) cannot run.** `semio-s-plugin-procedural` does not
  compile: a peer session's mutation-module refactor is actively in flight (measured 94 → 16 → 116
  errors over this session). Per the coordinate-vs-fix rule, we wait. `semio-framework-os` was
  blocked by DKM's brep dissolution earlier today and is **now clear (0 errors)**.
- Consequence: the three migrated demonstrator bundle tests are compile-verified but not run.

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
