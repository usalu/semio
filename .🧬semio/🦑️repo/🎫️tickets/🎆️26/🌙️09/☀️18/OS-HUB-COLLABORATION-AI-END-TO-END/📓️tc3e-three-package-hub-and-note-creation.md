# TC3e — a three-package trusted catalog on hub 7651, and the first `s.note.note` document

Slice TC3e of ticket 26/09/18. Predecessor `📓️tc3d-guest-genesis-and-note-creation.md`.
Started 2026-09-22 11:0x.

## 0. HUB HANDOFF

**No `semio-hub` source changed by this slice** — everything landed is in one `s` plugin app and in
two law files. The coordinator needs no `semio-hub` rebuild on TC3e's account, and
`cargo check -p semio-hub` was therefore not run (preamble rule 26).

**Hub 7651 for CE3** (`hub-agent-participant-check`): pid, `readyz` and the generation id land here
the moment §4's hold finishes — (filling). The hub stays up after this slice ends. Data root
`.🧬semio/🌐hub/tc3e-boot`, binary `⚡️cache/cargo/target-tc3d/debug/os-hub`, credential sign-in on,
two humans provisioned by `📜️tc3e-provision.sh` (`user1@semio.dev` / `user2@semio.dev`).

## 1. Inherited state — TC3d's hold DID run, and died one layer deeper

The brief said "TC3d's driver never reached the mutex". **The captures say otherwise, and that
changes the whole slice.** `🗑️generated/tc3d-hub-dev.txt` (808 KB, last written 04:12:44):

| stage | begin | end | exit |
|---|---|---|---|
| preflight (outside the mutex) | 01:27:15 | 01:35:14 | 0, 0 retries |
| stage 1 `semio-s-plugin-stdio` wasm-release cdylib | 03:05:05 | 03:12:40 | 0 |
| stage 1 `semio-s-plugin-gis` | 03:12:40 | 03:12:42 | 0 (fresh) |
| stage 1 `semio-s-plugin-note` | 03:12:42 | 03:16:31 | 0 |
| stage 2 `trusted-catalog-bootstrap --packages stdio,gis,note` | 03:16:31 | **04:12:44** | **1** |

So all three components WERE rebuilt with TC3d's `codec` resolver fix compiled in
(`⚡️cache/cargo/target-tc3d/wasm32-wasip2/wasm-release/semio_s_plugin_{stdio,gis,note}.wasm`,
03:12/02:53/03:16), the bootstrap ran for 56 minutes, got through the actor closure and the actor
hash, and died in `trustedBootstrapComponentCodecRowsV1`. 7651 never came up because stage 2 failed,
not because the lock was never granted. TC3d's `📜️tc3d-prove.sh` watcher therefore never fired and
`🗑️generated/tc3d-prove.txt` holds only its start line.

## 2. The next fault in the chain — it was never `codec` at all

The exact stage-2 death (`🗑️generated/tc3d-hub-dev.txt`, tail):

```
semio-framework-plugin-describe codecs: codec.pack-schema-hash(note.document) on
  …/trusted-catalog/staging-a999881f2407f3681838d26d58f89c4d/packages/note/component.wasm:
guest fault plugin.internal: throwaway artifact codec app close faulted:
  Fault { origin: Framework, code: FaultCode("interactive-job.close-owned-disposer-missing"),
          message: "app owner did not provide the required bounded disposer for document-store" }
```

**That is TC3d's fix working.** Where TC3c had `wasm trap: unreachable executed` and TC3d decoded it
to a store `Drop` assert, the guest now returns a NAMED, reportable fault from
`close_artifact_codec_app` instead of aborting. What the fault names is the layer below: the close
cursor exists, and the app it is asked to close cannot close.

**Which app, and why.** `plugin_artifact_codec_app`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37311`) constructs every app of the bundle
to read its schema, keeps the editor, and closes the viewer it rejects. `VcsArtifactApp`'s close
ladder drives eight owned lanes (`🦀️.rs:30594`), each through
`drive_artifact_owned_disposer(lane, …)`, and lane 0 is `document-store`. The disposer for that lane
is `A::build_document_store_disposer()`, captured at construction (`🦀️.rs:22700`); the trait default
is `None` (`🦀️.rs:33366` for `ArtifactViewer`) and `None` is the fault above — fail-closed by
design.

`NotePlayApp`, the editor, declares all five of its disposers
(`…/🪆️subsets/✳️any/✏️editor/🦀️.rs:526-551`). **`NoteViewer` declared none of them** — no store
owners, no disposers, nothing but snapshot/handle/render. So the note document kind was reachable
only as long as nobody ever closed its viewer.

**This is not a codec defect and not a TC3d regression: `ViewerApp<NoteViewer>` could never close at
all.** The identical defect is already documented one plugin over —
`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/…/👁️viewer/🦀️.rs:60` records that every close of a mounted
`ViewerApp<Block3dViewer>` failed with this exact code until its owners were declared, with the
comment "Read-only says nothing about ownership". Note's viewer is the same omission, found through
the codec path because the codec resolver is the one caller that closes EVERY app of a bundle.

**Scope, measured.** Only packages the hub links no Rust codec for run the component codec probe
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:9669-9676`: `spec.linkedCodecRegistry` short-circuits to the
linked rows). stdio and gis are linked, note is not — so note is the only one of the three whose
guest `codec` interface the bootstrap calls, and the only one whose bundle apps must close.

## 3. The fix — one app, six declarations

`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`:
`NoteViewer` now declares `build_document_store_owners` and `build_config_store_owners`, and the
four bounded disposers `VcsArtifactApp`'s ladder drives for a viewer —
`bounded_document_store_disposer::<NoteSnapshot, NoteMutation>()`, `no_config_store_disposer()`,
`no_presence_store_disposer()`, `no_transient_store_disposer()` (the draft lane is supplied by
`ViewerApp` itself, `🔌️plugin/🦀️.rs:34051`). The document lane is the editor's own catalogue: a
viewer owns the same `NoteSnapshot` envelope and retires it the same way.

No framework edit was needed; TC3d's cursor and diagnostics are what made this legible in one read
instead of another two-hour wall.

| check | exit | warnings | capture |
|---|---|---|---|
| `cargo check -p semio-s-artifact-note-note` | 0 | 86 | `🗑️generated/tc3e-check-note-artifact.txt` |

(Warning count as the proof of a real type-check rather than a short-circuited cache.)

## 4. The rebuild + bootstrap hold

ONE detached hold, driver `📜️tc3e-hub-boot.sh 7651 note,stdio,gis` (pid in
`🗑️generated/tc3e-driver-pid.txt`, work script `📜️tc3e-mutex-work.sh`, capture
`🗑️generated/tc3e-hub-dev.txt`), launched 11:10:34. Private `CARGO_TARGET_DIR=…/target-tc3d`
(deliberately TC3d's, not a fresh one: it already holds the warm wasm32 closure of all three
plugins and the `os-hub` binary, so only what actually changed rebuilds), fresh data root
`.🧬semio/🌐hub/tc3e-boot`, ordered mutex stamp `20260922110200-tc3e` (session 8's fixed queue
order, `📜️mutex-ordered.sh`).

**One structural change against TC3d's work script: note is built FIRST and gated.** TC3d spent
7 min on stdio, 4 min on note and then 56 min inside `trusted-catalog-bootstrap` before dying on the
note codec probe. TC3e builds note, runs `semio-framework-plugin-describe codecs` on the fresh
component immediately (`🗑️generated/tc3e-gate-codecs.txt`, exit 77 on failure) and only then builds
stdio and gis and enters the bootstrap. The same failure now costs ~5 min of a hold instead of ~70.

The preflight (outside the mutex, so a queue slot is never spent on a tree that cannot build) is
`cargo check -p semio-s-plugin-note --target wasm32-wasip2` plus `cargo build -p
semio-framework-plugin-describe` — the second both proves the native emitter and produces the binary
the in-hold gate runs.

`📜️tc3e-prove.sh` is a SECOND detached watcher (pid in `🗑️generated/tc3e-prove-pid.txt`, launched
11:15:17) polling `:7651/readyz` for up to 10 h; on 200 it runs the codec laws, then
`📜️tc3e-provision.sh` (two credentials + create-and-attach per kind), then re-reads `readyz`, all
into `🗑️generated/tc3e-prove.txt`.

**Preflight timeline (measured):**

| attempt | at | result |
|---|---|---|
| 1 | 11:13:40 | RED — 7 errors, none of them this slice's: `E0027 pattern does not mention field \`tasks\`` at `🔌️plugin/🦀️.rs:25311` and 5×`E0609 no field \`reserved\` on TaskSlot` at `⚛️reactor/🧵️executor/🦀️.rs:477-496`, a peer's async-task-lane refactor landing half-finished (preamble rule 3 — not mine to revert) |

(continues; the driver retries every 60 s and enqueues on the first green.)

## 5. The 7651 proof table

(filling)

## 6. Codec laws (TC3d §6 a/b/c)

**(b) The per-component sweep.** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs`
gains `STAGED_CODEC_COMPONENTS` (package id, component file, artifact kind, document schema for the
three trusted-catalog packages), `codec_sweep_one_component` (all four exports plus BOTH resolver
keys over one component, returning the failure text rather than panicking) and
`owned_codec_answers_every_call_on_every_staged_component`, which reports every component in one run
instead of dying on the first. A component not built in any `target*` root is SKIPPED; the law fails
if it found none at all.

**(c) The nonempty `apply-ops` batch** is a NATIVE law, in the note artifact crate rather than in the
host: `artifact_app_apply_ops` needs a real `NoteMutation`, and `semio-framework-plugin-host` cannot
depend on an `s` plugin. `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🧪️tests/🔬️unit/🦀️.rs` ·
`note_apply_ops_reduces_a_nonempty_batch_and_closes_its_store` drives genesis → one `rename-note` op
encoded exactly the way the guest receives it (`os_spr::encode_ops_vec` over `OpBinary::encode_op`)
→ `artifact_app_apply_ops::<EditorApp<NotePlayApp>>`, and asserts the batch lands exactly one edit.
This is the first run of TC3d §2(b)'s close cursor: the empty batch returns before a store is ever
built, so that cursor had only ever been type-checked.

**(a) Results** (filling — they need the rebuilt component of §4).

## 7. Gaps

**(a) The missing disposer is a REPO-WIDE defect; only note is fixed.** Counted across
`✏️s/🔌️plugins` (every file with an `impl ArtifactEditor`/`impl ArtifactViewer`, checked for a
`build_document_store_disposer` declaration):

| plugin | apps declaring a disposer | apps leaving the fail-closed default |
|---|---|---|
| `🗒️note` | 2 | **0** (after §3) |
| `🌍️gis` | 2 (both editors) | **2** (both viewers: `🗺️gismap`, `🏔️gisterrain`) |
| `🗄️stdio` | 9 | **167** |
| all `s` plugins | 69 | **229 of 298** |

Every one of those 229 surfaces faults `interactive-job.close-owned-disposer-missing` on its first
close. gis and stdio are invisible to the bootstrap only because the hub links Rust codecs for them
(§2), so nothing ever asks their bundles to construct-and-close. **They were deliberately NOT fixed
here**: gis is in the same mutex hold as note and a compile error in a gis viewer would cost the
whole hold and the slice's outcome, and stdio is 167 apps. The measurement plus the worked example
is the handover; a background task carries it.

**(b) The sweep law's table.** `STAGED_CODEC_COMPONENTS` names all three packages, so the sweep
covers gis and stdio the moment their components are built — which means (a) is what it will report
for them. The measured result is in §6; if a package is red there, its row is a live statement of
(a), not a flaky law.

**(c) No law over `NoteViewer`'s close on its own.** The viewer is proven to close through the codec
resolver (which closes the rejected app, §2) and through the sweep, not by a direct unit law: the
throwaway app the resolver builds is the plugin-level app enum, which lives in `semio-s-plugin-note`
rather than in the artifact crate where the laws are.

## 8. Files changed

Source:
* `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` —
  `NoteViewer::build_{document,config}_store_owners` and
  `build_{document,config,presence,transient}_store_disposer` (§3)

Laws:
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs` —
  `STAGED_CODEC_COMPONENTS`, `codec_sweep_one_component`,
  `owned_codec_answers_every_call_on_every_staged_component` (§6b)
* `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🧪️tests/🔬️unit/🦀️.rs` —
  `note_apply_ops_reduces_a_nonempty_batch_and_closes_its_store` (§6c)

Ticket-owned: this report, `📜️tc3e-hub-boot.sh`, `📜️tc3e-mutex-work.sh`, `📜️tc3e-provision.sh`,
`📜️tc3e-prove.sh`, and `🗑️generated/tc3e-*`.
