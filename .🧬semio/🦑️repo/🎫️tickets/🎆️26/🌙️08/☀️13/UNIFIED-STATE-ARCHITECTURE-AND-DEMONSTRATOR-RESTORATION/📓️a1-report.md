# A1 — Presence and transient lanes wired end-to-end

Status: **landed and verified**. `semio-framework-plugin` **155/155**;
`semio-framework-os-kernel --lib` **835 passed / 1 failed** (the 1 is the pre-existing
`fixture_sweep::…all_non_stdio_grammars_reject_each_others_shipped_fixtures`, caused by sibling
sessions restructuring plugin `📚️examples`).

## What now exists

The four state mechanisms are all real, typed, and reachable only through the lane API:

| Mechanism | Lifetime | Scope | Store |
|---|---|---|---|
| **artifact** | persisted | shared | `ArtifactStore` (event-sourced, undoable, checkpointed) |
| **config** | persisted | local-only | `ConfigStore` (= `ArtifactStore`) |
| **presence** | ephemeral | shared | `PresenceStore` — **new** |
| **transient** | ephemeral | local-only | `TransientStore` — **new** |

`draft` remains an artifact-side concept (ephemeral *artifact content*, i.e. a `DraftStore` alias of
`ArtifactStore`), per the user's ruling that "draft are transient artifacts; transient is about
ephemeral local UI state". The two are distinguished by **what the state is**, not by how long it
lives.

### Stores (`🏪️store/🦀️component.rs`, `//#region 🔖️EphemeralLanes`)

- `PresenceStore<P, M>` — a **last-writer-wins roster**, deliberately NOT an `ArtifactStore` alias
  like config/draft. Presence has no history, no undo, no checkpoints and no merge: each peer is the
  sole author of its own value, and the wire already ships whole per-peer snapshots
  (`ClientFrame::Presence` / `ServerFrame::Presence`), so a later frame simply supersedes the
  earlier one. Modelling it as an op log would mint an unbounded history of cursor positions nobody
  could ever undo. Carries `local`, `peers: HashMap<actor, (P, received_at_ms)>`, and a `generation`
  that bumps only on LOCAL change — the exact signal a broadcast coalescer throttles on.
  `expire_peers` drops silent collaborators so a disconnected cursor cannot linger.
- `TransientStore<P, M>` — presence minus the roster. Never shared, persisted, packed, checkpointed
  or undone.

### Lane surface (`🔌️plugin/🦀️component.rs`)

- `ArtifactApp::{Transient, TransientMutation}` associated types + `NoTransient`/`NoTransientMutation`
  (twins of the existing `NoPresence` pair), fanned out across all **55** app impls.
- `PresenceView` (local + peers, sorted for stable render order) and `TransientView`.
- `EphemeralEmit<A>` + `ArtifactApp::ephemeral(command, doc, cfg, presence, transient)`, defaulting
  to emitting nothing.
- `VcsArtifactApp` gained `presence_store` / `transient_store`, applied in the typed-command
  dispatch path.

## The main design decision: ephemeral lanes do NOT ride on `Emit`

I first extended `Emit` to five type parameters (`…, PresenceMutation, TransientMutation`) and
**backed it out**. Two reasons, one practical and one principled:

1. **Practical:** there are **1092** `Emit<…>` signatures across the repo. With 53 of 55 apps
   declaring real presence types, `Emit<M, C, D>` would no longer be the app's own emit type, so
   nearly all of them would need editing — a huge sweep against a tree several other sessions are
   actively editing.
2. **Principled, and the real reason:** the document lanes (artifact/config/draft) each have an op
   log, an edit id, an undo group and a failure mode. Presence and transient have **none** of those.
   They cannot fail, cannot be undone, never enter a checkpoint, and never appear in the command
   log. Putting them in `Emit` would attach five type parameters to a thousand signatures in order
   to express something that shares none of `Emit`'s machinery — and would force every app that
   emits no presence to name its presence type anyway.

So they get their own emission (`EphemeralEmit`) via their own trait method with a default. An app
with no shareable or UI-local state writes **no code at all**; an app that wants presence overrides
one method. `ephemeral` is computed BEFORE `handle` (so it sees pre-command state) and applied
unconditionally (a command that fails still moved the cursor that provoked it).

## Proven, not assumed

`a_command_reaches_both_ephemeral_lanes_without_touching_history` asserts the whole path:
both generations bump on dispatch; the document's edit log gains exactly ONE edit (no ephemeral lane
leaked into history); and `undo` rewinds the document while leaving both ephemeral generations
untouched — they were never part of the undoable gesture.
`a_command_that_emits_nothing_ephemeral_leaves_both_lanes_untouched` pins the negative case.

## Regression I caused and fixed

CW1's `CHANNEL_VERSION` 5 → 6 bump broke two golden-hex corpus tests
(`app_command_fixture_corpus_matches_golden_hex_and_round_trips` and its frame twin) — the version
byte is literally in the pinned `Hello`/`Welcome` hex. Re-pinned by running the encoder (never
hand-computed, per the corpus's own documented provenance rule), and added corpus entries + goldens
for the three new CW1 variants (`LoadChildren`, `ReadChildren`, `Children`). Flagged to me by the A0
agent's measurement, which is exactly what the shared baselines are for.

## Handed on, deliberately not faked

**The TypeScript channel twin is stale and I did not touch it.** `APP_CHANNEL_VERSION = 4` in
`🧰️framework/🛍️products/💻️os/🟦️component.ts` was already two versions behind Rust BEFORE this
ticket (Rust was at 5; the TS side never implemented v5's `PureCommand`). Bumping the constant to 6
without implementing the frames would assert a compatibility that does not exist. This belongs to
whoever owns the TS channel; it is a pre-existing gap, now one version wider.

## Remaining A-wave work

- **A2/A3 producers:** nothing yet EMITS presence in a real app — the host-side generic tier
  (cursor/viewport heartbeat at ~10 Hz, coalesced ≥100 ms) and the per-app typed tier are unwritten.
  The lane is real and tested; the content is not yet authored.
- **A4:** `OsShellConfig` — the four `🖥️platform` localStorage stores and the wgpu prefs still
  bypass the config lane.
- **A5:** mode/window scoped facets now have their taxonomy dirs (A0 scaffolded 238 window + 179
  mode dirs) but no real content; `window_measures` still reads flat app config.
- **A7:** the policy that makes the four mechanisms *exhaustive* (ban localStorage outside the
  config adapter, ban `thread_local!` state, ban new ad-hoc stores) is not yet written — today the
  lanes are available but nothing forbids the alternatives.
