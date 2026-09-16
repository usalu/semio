# 📓️ Day 5 run — 2026-09-15 (late)

Resumes [📓️day4-run.md](📓️day4-run.md) **Open 1**: example switch / persisted reload of a curation faults
`Incomplete` at archive closure validation. Repo MCP did not connect; ticket managed on disk.

## Why the load is rejected

`CurationSnapshot.catalog` is a `#[child(kind = "s.stdio.semio")]` kit handle whose id is content-addressed
from the stock (`catalog_child_handle`). The closure walk (`🏪️store/🧩️composition/🌳️closure/🦀️.rs:147`)
projects the root's declared child rows and requires a member per row; the browser host answers
`Effect::LoadDocument { pack, spr }` with `loadDocumentArchive({ parent_pack, parent_spr, members: [] })`
(`🏛️ShellHost/🟦️.tsx:2655`), so the roster is empty and `walk` returns `Incomplete`. The same shape
would bite the initial document too: the editor ran `VcsArtifactApp<…, NoMembers>`, never created the
catalog member, so `readAppDocumentArchive` exported zero members and any reload of a saved curation
would have hit the same wall.

## Options weighed

| | change | verdict |
| --- | --- | --- |
| A | `Effect::LoadDocument` carries a full archive (kernel + WIT + reactor + both hosts + TS + ~70 plugin sites) | honest wire contract, but a 30-crate blast radius through peers' in-flight crates for a member whose content is a pure function of the parent |
| B | Guest completes the closure: an app hook supplies the initial pack of a **derivable** child slot the archive omits; the wrapper mints the member's envelope and feeds it through the existing bounded roster | contained (plugin crate + store helper + sourcing), strict for non-derivable slots (`None` ⇒ still `Incomplete`), and the same hook seeds the child at boot so the live app is complete from the first frame |

B chosen. The door's contract becomes "archive members ∪ app-derivable genesis = complete closure"; nothing
that is not derivable from the parent is ever invented.

## Landed

### Store
- `genesis_member_envelope_pack(schema, dialect, owner, initial_pack)` — a brand-new member's full envelope
  pack (pack + edit-free SPR stamped with id/schema/dialect/owner via the same `stamp_document_spr_identity`
  path the parent uses), minted **without a live store**, so nothing needs the bounded close protocol.

### Plugin framework
- `ArtifactApp::genesis_child_pack(snapshot, slot, child_id) -> Option<Vec<u8>>` (default `None`), mirrored on
  `ArtifactEditor`/`ArtifactViewer` and forwarded by `EditorApp`/`ViewerApp`.
- `VcsArtifactApp::with_registry_on_bus` seeds every projected child row the app can derive through
  `open_child` (the restore path), so `children`, the composition graph and the child content root are
  populated at boot.
- Archive load: a `GenesisMembers` step (before the roster is reserved) appends one
  `OwnedDocumentMemberPackEntry` per projected row the archive lacks and the app derives, owner-stamped to the
  candidate root; ordinals continue after the archived ones; the fixed member/byte authorities are re-checked.
  The member schema is resolved from `M::OPEN_DECLARATIONS` — an app deriving a child of a dialect its member
  roster does not declare is a fault, not a silent skip.

### Second wall: no `s.stdio.semio` subset but `flow` could be opened retained
With the genesis member on the roster the load still faulted — `member open rejected Decode … in phase
Snapshot`. `semio_snapshot_open!` (`🧿️semio/🦀️.rs`) binds `flow` to its streaming `SemioFlowSnapshotDecode`
and **every other subset to `UnsupportedMemberSnapshotOpen`**, which rejects unconditionally: the archive door
could never open a kit (or mesh, cad, …) member at all, whatever carried it.
- Store: `PackMemberSnapshotOpen<P>` — a retained member snapshot open over the member's own `ArtifactPack`
  codec: framed bytes copied in 4 KiB chunks into an exactly reserved buffer, one `P::decode_pack`, the same
  whole-pack decode the parent's `RetainedPersistedDocumentHydration` performs; bounded close retires the
  snapshot through `owned_retirement`. Same shape as the plugin crate's `RecursiveBranchSnapshotOpen` fixture.
- stdio-semio: the macro's default arm now names it, so all seventeen non-flow subsets open.

### Third wall: the load worked in the browser but took 97 s
Rebuilt wasm (`activate-sourcing-react-dev`, 01:07), served on :6081. The boot-time example load
(`[DEBUG] loadDocument pack 979`) settled `loaded: true` — after **96 768 ms**. Measured in the unit test: a
demo-sized archive needs **22 729** bounded maintenance steps (decode, hydrate, member open, closure,
candidate views, commit), and the browser only advances them through the runtime's live-cleanup job — one
family per pump tick across a 25-stage rotation — while the host's `PollDocumentArchiveLoad` loop merely
read status. Every example switch therefore rendered one interaction late, which is what day 4 saw as
"the load never applies".
- Plugin framework: `VcsArtifactApp::poll_document_archive_load` now drives the same maintenance rotation
  for up to `DOCUMENT_ARCHIVE_POLL_WALL_US` (= `BACKGROUND_LANE_WALL_US`, 4 ms) before reporting, so
  progress scales with how eagerly the host polls and no single poll holds the actor longer than the pump's
  own worst step. The unit test's load settles in **5** polls; the browser boot load in **317 ms**.

### Sourcing
- Editor and viewer run with `semio_s_artifact_stdio_semio::SemioMembers` (plugin fleet, subset declaration,
  test harness).
- `genesis_catalog_pack` (artifact root): re-derives `catalog_child_handle` from `stock_of(snapshot)` and only
  answers when the derived id equals the declared one — a snapshot whose handle disagrees with its stock is not
  repaired, it is rejected as `Incomplete`.
- `example_load_settles_through_the_host_document_archive_door` un-ignored.

## Evidence

- `cargo test -p semio-s-artifact-sourcing-curation --lib`: **127 passed, 0 failed, 1 ignored** (the manual
  fixture export). `example_load_settles_through_the_host_document_archive_door` reaches `Ready` and the loaded
  snapshot's stock equals `📦️expected-stock.json`; `retained_example_load_publishes_authored_stock_and_closes_exact_owners`
  still passes with the app booting over `SemioMembers` (boot genesis opens the catalog child).
- `cargo test -p semio-framework-os-kernel --lib -- genesis_member_envelope_pack_opens typed_child_store_factory_round_trips`:
  2 passed — the store-less genesis envelope opens through `open_member_store` with the stamped owner, rejects a
  mismatched owner and an empty initial pack.
- `cargo check -p semio-framework-plugin --features artifact-app-testing --tests`: clean.

### Peer-state failures, not this ticket's
Run once each, messages recorded so nobody chases them here:
- `semio-framework-plugin` (child/composition filter): 14 failures — `typed command 'compositeEdit' has no exact
  controller/owner/factory/tool/schema proof` (`interactive-job.missing-factory`), `size_of::<OwnedDocumentMemberIngressRegistry>() < 1024`,
  and Drop witnesses that follow those first panics. The test files were changed by peers at 20:01 today; none of
  the assertions touch `genesis_child_pack`, `seed_genesis_children` or `complete_document_archive_genesis`.
- `semio-framework-os-kernel -- member_open composition`: 1 failure, `composition_pins_rederive_checkpoint_identity_without_partial_mutation`
  — `edit history insertion requires its exact mutation retirement factory`.
- `semio-s-artifact-stdio-semio --lib`: 2476 passed / 47 failed — kit JSON default-slot and property-id
  fixture assertions, brep fixture canonical JSON, and `a_semio_member_mints_and_reopens_a_real_child_envelope`,
  which never closes the two stores it creates and so trips the `ArtifactStore` Drop witness on its own.

### Browser, :6081, wasm 01:38 (React)
Console hooked in-page (the dev console is a firehose; the tab buffer drops ~100k lines/min). No `error`,
`fault`, `trap` or unhandled rejection across the whole pass.
- Boot: Pool (10 kinds), Curated, Preview, Grid — `loadDocument settled {loaded:true, ms:317}`.
- Selector → **No example**: `settled ms:262`; Pool, Curated and Grid read "No data"/empty within 0.5 s.
- Selector → **Demo**: `settled ms:225`; the ten kinds and the 3D grid are back within 0.5 s.
- `+` on KVH C24 twice on the reloaded document → Curated row with count 2 (edits land on the loaded store).
- Saved-archive reload is proven at the guest level by
  `a_saved_curation_archive_carries_its_catalog_member_and_reloads_with_its_edits`: `document_archive()` of a
  booted, edited curation carries exactly the `catalog` kit member (boot genesis), feeds back through the
  door with that member present, settles `Ready`, keeps the curated edit and the reopened member is live.

### Machine notes
- Peers regenerate `🔌️plugin/📇️registry/🤖️generated/*.ts` every few minutes; each time vite on :6081 logs
  "restarting server..." and never rebinds. `serve-sourcing-supervised.sh` (this folder) recycles the serve
  whenever the port stops answering for 30 s — `pkill -f serve-sourcing-supervised` stops it.
- `SEMIO_RUNTIME_DIAGNOSTICS=1` in localStorage turns the console into 377k lines per boot; useless with
  the tab buffer, and the boot under it landed on "Plugin Recovery" once. Left off.
- `cargo test -p semio-s-artifact-sourcing-curation --lib`: **128 passed, 0 failed, 1 ignored** (final).
