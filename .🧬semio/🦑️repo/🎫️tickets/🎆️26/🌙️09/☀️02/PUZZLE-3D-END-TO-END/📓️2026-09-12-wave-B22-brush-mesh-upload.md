# 🥽️ Wave B22 — the brush-mesh upload stops owning the actor's command queue (2026-09-12)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Follows B19 §3.1 (`📓️2026-09-12-wave-B19-mutation-lane-regression.md`),
W-M2 (`📓️2026-09-09-wave-M2-paged-brush-mesh-upload.md`), W-H (`📓️2026-09-10-wave-H-brush-mesh-reannounce.md`),
the upload audit (`📓️2026-09-10-brush-mesh-upload-audit.md`) and B3 (`📓️2026-09-11-wave-B3-chunked-owner-pages.md`).

No git write, ticket not opened/closed, nothing deleted under `🗑️generated`, every command foreground,
**no wasm build run** — so every browser number below is on wasm **#47**, i.e. the HOST half of this wave
only. The guest half needs #48 before it can be measured in the browser; it is proved by cargo laws here.

---

## 1 Measured — what one mesh upload actually costs

Probe: `🔍️b22-mesh-probe.ts` (new, this ticket). It needs **no source tap**: it timestamps the
`[DEBUG] command ingress lane` / `… settled` / `… continuation` lines the runtime already prints, pairs
enqueue with settle (the ingress mutex serialises, so settle order is enqueue order), counts actor round
trips by wrapping `Worker.prototype.postMessage` through `addInitScript`, and fires one canvas click
mid-run to measure what a user action waits behind. Outputs in `🗑️generated/b22-mesh-<stamp>.{json,ndjson}`.

### 1.1 Baseline, default example (Concrete Forest) — `b22-mesh-2026-09-11T17-50-19.json`

```
meshCommands 18   firstMeshAt 3.841s   lastMeshAt 7.930s   lane Background
meshCommandSeconds { min 0.021, p50 0.643, p90 1.137, max 1.216, total 11.482 }
```

**0.64 s of the actor's only command queue per 5.7 KB page**, 18 pages, 11.5 s of boot spent on nothing
but mesh transfer. Wire per page is 1 024 values as two base64 payloads ≈ 5 464 payload characters inside
a ~261-byte envelope ≈ **5 725 raw bytes against `PUZZLE_COMMAND_RAW_BYTES = 8 192`** (W-M2 §2, unchanged).

### 1.2 Baseline, document scale — `b22-mesh-2026-09-11T17-54-55.json`

```
meshCommands 202 enqueued between 289.7s and 311.1s (21.3s)
settled       40 of them over the following 420s
meshCommandSeconds { min 0.017, p50 7.718, p90 36.485, max 87.911, total 328.233 }
userAction    interactionHover, queuedAhead 15, clicked 292.2s → settled 336.6s  (44.3 s)
settle gaps   0.2 … 8.0 … 12.3 … 17.9 … 87.9 s (monotonically worsening)
```

This reproduces B19's "5.6–14.7 s EACH" independently and worse. **202 commands went into the queue in
21 s and the guest drained 40 of them in the next 420 s.** One ordinary click landed behind 15 pages and
took **44.3 seconds**. (This run shared `:6013` with another agent's `browser-probe`; the shape, not the
absolute constant, is what carries — §1.1 is uncontended and shows the same 0.64 s/page floor.)

### 1.3 Where the time goes — and where it does NOT

| candidate | verdict |
| --- | --- |
| host encode (`puzzle3dBrushMeshPages`) | **not it.** Pure `btoa` over 4 KB slices; the whole 202-page run encodes in one macrotask burst — `firstMeshAt`→`lastMeshAt` is the DISPATCH spread, not encode. |
| guest base64 decode | **not it.** `decode_brush_mesh_page_values` is one `base64_standard_decode` of 5 464 chars plus an `extend_from_slice`; `stage_brush_mesh_page` is O(page), and the digest is computed **once, on close** (`⏳️precompute/🦀️.rs:1310`), not per page. Nothing quadratic. |
| guest re-validating the whole mesh per page | **not it**, but it re-validates the whole PAGE twice: `Puzzle3dPrecomputeCommandWork` walks the payload in 512-character slices (`✏️editor/🦀️.rs:6747 scan_mesh_page`, `:6760 PUZZLE3D_MESH_PAGE_SCAN_CHARS`) claiming `ceil(5464/512)*2 + 1 ≈ 23` work items, and then the reducer decodes the same string again. |
| **per-command settle round trips** | **this is it.** `runQueuedTurn` (`🔌️PluginRuntime/🟦️.tsx:2190`) takes the per-actor command-ingress mutex, submits one turn per ingress page, then loops continuation turns until `command-complete`, then `settleAcknowledgedPluginTurns`. Measured with the `postMessage` counter on the uncontended run: **84.2 worker round trips per `registerBrushMesh` command** (`workerTurnsPerMeshCommand`, 842 posts over 10 consecutive mesh-only settles). |
| **queue occupancy** | **this is the user-visible half.** `handleRegisterBrushMesh` drained one page per `setTimeout(drain, 0)` and never waited for anything, so all 202 pages were in the actor's queue within 21 s regardless of how slowly the guest drained them. |

So the wire format is not the problem — 202 × 5.7 KB is 1.15 MB, nothing. **The problem is that the mesh
is 202 separate top-level COMMANDS, each a fully serialised ~84-round-trip ingress transaction, all
queued at once ahead of every user action.**

---

## 2 The design, and why this one

### 2.1 What the wire actually allows (read, not assumed)

- (a) **guest fetches the mesh by URL itself.** `Effect::HttpRequest` exists (`🎠️kernel/🦀️.rs:568`) and the
  guest SDK can await it (`⚛️reactor/📮️requests/🦀️.rs:249 request`). But it is **not mapped in the browser
  host at all** — `wireEffectToFriendly` (`🔌️PluginRuntime/🟦️.tsx:896-959`) has no `http-request` case and
  drops it with the `[DEBUG] unmapped effect` warning; and the awaiting form needs a guest `AsyncTask`,
  while the puzzle command arm is synchronous. Additionally the host, not the guest, is what flattens the
  GLB scene graph (`extractGlbCollisionMesh` bakes world matrices, `🌐️World3dHost/🟦️.tsx:1960-1999`), so
  moving the fetch guest-side also moves the flattening and changes what the digest is over.
- (b) **a bulk binary lane.** One exists and is production: `Effect::InvokeExtension` mints a
  `RequestRegistry::request_continuation` slot (`⚛️reactor/🦀️.rs:718`) — no parked future, no async task —
  and the host answers it with `guestAnswerPages` (`⏱️trace/🧮️memory/🟦️.ts:26`): `http-chunk` prologue
  pages of ≤ `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` plus a terminal `completed`, **all in ONE turn**
  (`🔌️PluginRuntime/🟦️.tsx:2481-2485`), up to `GUEST_HOST_ANSWER_CEILING_BYTES = 8 388 608`.
  **But its landing site is command-bounded.** The answer is redispatched as an ordinary app command
  (`⚛️reactor/🔄️turn/🦀️.rs:565-587` → `plugin_dispatch_response_action`), and that dispatch goes through the
  app's own action bus, whose declared raw-wire bound is the same 8 KiB. The procedural plugin already hit
  exactly this and chunks its answer at 4 KiB — see `🌊️flow/📐️brep-geometry/🦀️.rs:923-934`:
  *"the response action is an ordinary interactive retained route whose declared wire bound is 8 KiB … a
  single-chunk mesh therefore arrived as 38 770 raw bytes and the bus rejected it before decoding."*
  So (b) buys nothing: it re-lands on N commands of the same size.
- `Effect::SpawnJob` is not a host-fetch lane either — the browser host only *steps the guest's own*
  `BoundedJob` (`🔌️PluginRuntime/🟦️.tsx:2040-2077`); the bytes never originate host-side.

**Conclusion: every host→guest delivery path that lands anywhere the puzzle app can consume it is capped
at the retained command's 8 KiB.** The only uncapped one is the parked-future `RequestRegistry::request`,
which needs a guest `AsyncTask` plus an `http-request` mapping the browser host does not have. That is
real follow-up work (§6), not a one-wave change, and nothing in it is verifiable before #48.

### 2.2 So the wave attacks the two things that are actually wrong

**(i) The bytes cross once per GEOMETRY, not once per mesh id.** Every `dist/mesh/*.glb` in this repo is
the same 771 728-byte capsule (W-M2 §1). The transfer was keyed by url — `brush_mesh_store().handles:
HashMap<url, EngineHandle>` — so a scene placing several object kinds over byte-identical geometry paged
the same 294 912 bytes once per id, 72 commands each. The collision engine's own `meshes` map keys by url
deliberately (two ids stay two identities, `encode_brush_mesh_request`'s note), so **only the TRANSFER is
content-addressed**: a second id adopts the first id's derived page and the wire carries the announcement
alone.

**(ii) The run is a back-pressured stream, not a burst.** `onAction` settles on the dispatched command's
own typed-operation completion (`🔺️mesh/🟦️.ts:777-784`) — that is exactly the "this page is in" signal the
fire-and-forget shape threw away. Awaiting it before queueing the next page bounds what a user action can
ever wait behind at **one page**, whatever the run's length.

Plus two correctness repairs the measurement exposed: a **retransmitted page** used to destroy the whole
run (so one retried page cost the client all 72), and an **aliased announcement the guest refuses** had no
way to fall back to the bytes.

---

## 3 Changes

| file | change |
| --- | --- |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs` | digest index on the process store; `adopt_brush_mesh_by_digest`; digest fallback in `Puzzle3dCollision::adopt_shared_mesh`; idempotent re-receipt in `stage_brush_mesh_page`; page-0 short circuit in `Puzzle3dPrecomputeSession::stage_mesh_page` |
| `…/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` | two new laws, one rewritten to the new contract, one seeded off the shared cube |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes` seeded off the shared cube |
| `…/🧬️schema/🧪️tests/🔬️testkit/🦀️.rs` | `seeded_cube_mesh_buffers(seed)` |
| `🧰️framework/…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `Puzzle3dBrushMeshRegistry` entries carry `paged`; `holdsDigest` / `alias` / `mayAlias`; new `puzzle3dBrushMeshQueueStep` + `drainPuzzle3dBrushMeshQueue` |
| `🧰️framework/…/🧱️elements/🌐️World3dHost/🟦️.tsx` | `dispatchBrushMesh` (awaitable), back-pressured `drainBrushMeshQueue`, digest-alias fast path, unmount via a live flag instead of a timer |
| `🧰️framework/…/🎯️targets/⚛️react/🟦️.tsx` | re-exports the two new helpers and the step type |
| `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts` | five new laws |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️b22-mesh-probe.ts` | the measurement probe (new; `🔍️browser-probe.ts` untouched) |

### 3.1 Guest — content-addressed transfer

`⏳️precompute/🦀️.rs`:

- `Puzzle3dBrushMeshStore` gains `digests: HashMap<String, store::EngineHandle>`; `derive_brush_mesh`
  records the digest alongside the url handle.
- `adopt_brush_mesh_by_digest(url, digest)` — reads the handle the digest index carries, **aliases it onto
  `url`**, bumps `BRUSH_MESH_INSTALLS` (a genuinely new resident identity, which is what `meshResidency`
  means to the client) and hands back the geometry.
- `Puzzle3dCollision::adopt_shared_mesh` now tries `shared_brush_mesh(url)` (digest-verified as before)
  and falls through to the digest index. A digest this process never derived still adopts nothing.
- `Puzzle3dPrecomputeSession::stage_mesh_page` short-circuits at **page 0 only** when the identity is
  already adoptable: a run over resident geometry closes on its first page and the remaining
  `pageCount - 1` commands are never needed. Page 0 only, so a live run costs no store lookup per page.

### 3.2 Guest — a retransmission is not a gap

`stage_brush_mesh_page` gained one arm before the `Gap` arm: a page with `0 < page < next_page` and a
matching `page_count` for a matching `(url, digest)` run is **acknowledged at the cursor the run actually
stands on**, with no second append. Before, any duplicate `swap_remove`d the slot and returned `Gap`, so
the client had to re-open at page 0 — one retried page cost all 72 of them, and any lane that retries at
all was quadratic. A page for a run that is *gone* is still a `Gap`.

### 3.3 Host — the registry knows how each claim was justified

`Puzzle3dBrushMeshRegistry.#entries` is now `Map<url, {digest, paged}>`:

- `confirm(url, digest)` — a run's LAST page went out ⇒ `paged: true`, and clears any alias refusal.
- `alias(url, digest)` — carried by a sibling's bytes ⇒ `paged: false`.
- `holdsDigest(digest)` — **only a `paged` entry answers**, so a chain of aliases can never stand in for
  bytes nobody sent.
- `mayAlias(url)` / `claimReupload` — a guest that refuses an alias (it publishes the identity on
  `interactionJson.meshReuploadUrls`, W-H) marks that url, and its next announcement **pages the bytes**.
  Without this the recovery loop is non-terminating: the guest asks for bytes, the host answers with the
  same alias, forever, and that identity keeps no collision body at all. A guest restart clears it.

### 3.4 Host — the drain

`drainPuzzle3dBrushMeshQueue` (`🛠️ShellHelpers`, pure, testable) — **exactly one command outstanding**:
take the next page; if its digest is `paged`-resident and the url may still alias, drop the rest of that
url's run and send the announcement instead; otherwise `await dispatch(page)`; confirm on the last page.
`World3dHost` owns only the queue, the refs and the awaitable `dispatchBrushMesh`; the unmount effect
retires the drain through a live flag (there is no timer any more).

---

## 4 Laws, and their output

### 4.1 Rust — foreground, quoted tails

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
  → Finished `dev` profile [unoptimized] target(s) in 2.82s        (0 errors; 88 pre-existing warnings)

cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
  → Finished `dev` profile [unoptimized] target(s) in 5.20s        (0 errors)

RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib \
  -- --test-threads=1 <the 15 B22 + B3 + W-M2 + W-H filters>
  → running 15 tests
    …a_run_over_resident_geometry_closes_on_its_first_page ... ok
    …a_retransmitted_page_is_acknowledged_without_dropping_the_run ... ok
    …an_uploaded_mesh_is_adopted_by_url_and_digest ... ok
    …an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes ... ok
    …one_brush_mesh_page_validates_inside_one_command_work_budget ... ok
    …the_paged_upload_contract_matches_the_language_neutral_fixture ... ok
    …a_document_scale_mesh_uploads_in_pages_and_registers ... ok
    …every_fixed_owner_sub_page_request_stays_under_the_guest_contiguous_ceiling ... ok
    …an_owner_wider_than_one_sub_page_reads_writes_and_retires_across_its_boundaries ... ok
    …nakagin_scale_fill_places_an_object_under_a_fragmented_guest_reservation_ceiling ... ok
  → test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 698 filtered out; finished in 0.65s

RUST_MIN_STACK=134217728 cargo test … --lib -- --test-threads=1 precompute
  → test result: ok. 156 passed; 0 failed; 0 ignored; 0 measured; 557 filtered out; finished in 4.05s
```

New Rust laws:

| law | what it pins |
| --- | --- |
| `a_run_over_resident_geometry_closes_on_its_first_page` | a second id announcing an 8-page run over resident geometry returns `Ok(None)` on page 0, installs live collision geometry, and holds no staging slot — **the other seven pages are never needed** |
| `a_retransmitted_page_is_acknowledged_without_dropping_the_run` | a duplicate page is `Staged` at the real cursor, the run still closes on exactly the announced bytes, and a page for a run that is gone is still `Gap` |
| `an_uploaded_mesh_is_adopted_by_url_and_digest` (rewritten tail) | the alias installs, is resident for later sessions, a stale digest still adopts nothing, and geometry this process never derived adopts nothing **by id or by digest** |

The store is process-wide, so two W-M2/W-H laws that asserted "this process holds nothing for that
geometry" became order-dependent the moment the transfer went content-addressed; both now use
`seeded_cube_mesh_buffers(seed)` — geometry no other law in the binary derives — and the helper's
docstring says why. That is the one real trap this change sets for future laws.

### 4.2 Full puzzle-3d lib suite

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1
  → test result: FAILED. 705 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out; finished in 159.32s
```

The 8 are **not this wave's** and not mesh-related:
`every_advertised_engagement_verb_is_implemented`, `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin`,
`selected_object_inspector_renders_that_object_field_group`, `settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on`,
`the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind`, `two_instances_converge_disjoint_object_edits_via_backbone`,
`kinds_tree_object_drag_data_carries_object_kind_and_mesh_url`, `the_catalogue_pages_an_over_wide_kind_catalog_without_exceeding_the_fixed_page`.
They live in `✏️editor/🦀️.rs` / `📌️panels/🛍️catalogue/` — files this wave never touched, and `✏️editor/🦀️.rs`
was rewritten by a live peer at 19:36 during this wave. `kinds_tree_object_drag_data_…` **passes in
isolation**, so that one is order/peer coupling, not a logic break.

### 4.3 TypeScript

```
cd …/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bun x vitest run "engine-contract" -t "puzzle3d brush mesh paged upload"
  → Test Files  1 passed (1) / Tests  10 passed | 528 skipped (538)

cd …/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bun x vitest run "engine-contract"
  → Test Files  1 failed (1) / Tests  1 failed | 537 passed (538)
```

The single failure is `noteShellCommand > buildNoteShellCommandAction …` (a `shell.windowClose` inverse
descriptor) — pre-existing, a peer's chrome/history lane, nothing to do with meshes. It fails identically
before and after this wave's edits.

New vitest laws (`🔬️engine-contract/🟦️.ts`, region `🥽️Puzzle3dBrushMeshUpload`):

| law | what it pins |
| --- | --- |
| *knows a digest this guest holds under another id…* | `holdsDigest` answers only for PAGED entries; an alias is itself holdable; forgetting the paged id withdraws the digest |
| *never re-aliases an identity the guest refused, and never chains one alias off another* | an alias never satisfies `holdsDigest`; `claimReupload` on an alias sets `mayAlias → false`; a real `confirm` restores it; a guest restart clears it |
| *collapses a queued run whose geometry a sibling id already put into the guest* | `puzzle3dBrushMeshQueueStep` drops the **rest of that url's run** and yields one `adopt` |
| *keeps exactly one mesh command outstanding and confirms only on a run's last page* | a >4-page run drains with **peak in-flight = 1**, dispatches every page exactly once, and confirms exactly once, with the right digest |
| *retires the drain without dispatching once the surface it belongs to is gone* | `live() === false` dispatches nothing and leaves the queue intact for the unmount sweep |

```
cd …/🎯️targets/⚛️react && bun x tsc --noEmit
  → 1085 errors repo-wide, byte-identical to the count before this wave's edits.
    ZERO inside 🛠️ShellHelpers; in 🌐️World3dHost and 🔬️engine-contract every reported line
    (1303 / 3302 / 4160 / 4497 and 4900 / 4901 / 8057) is outside every region this wave wrote.
```

---

## 5 Before / after in the browser (both on wasm #47 — HOST half only)

| | before `…T17-54-55` | after `…T18-51-16` |
| --- | --- | --- |
| mesh commands put in the queue | **202**, all inside 21.3 s | **42**, paced across 419 s |
| settled vs enqueued in the window | 40 of 202 — **162 outstanding** | 65 of 67 — **≈2 outstanding** |
| per-command p50 / p90 / max | 7.72 / 36.49 / **87.91** s | 6.97 / 13.05 / **18.08** s |
| user click: pages queued ahead | **15** | **2** |
| user click: dispatch → settle | **44.27 s** | **10.73 s** |
| `puzzle3d-register-mesh-*` notices | none | **none** |

Default example, uncontended: **18 → 12** mesh commands (`…T17-50-19` vs `…T18-47-27`), total mesh time
11.48 s → 9.38 s, and the mid-run click settled in **0.17 s** behind an empty queue.

**Read these honestly.** With back pressure the enqueue count now tracks the settle count, so "42" is not
"the run got 5× shorter" — it is "the host stopped pre-loading the queue". The numbers that are real
improvements are the ones that measure the defect: **outstanding queue depth 162 → 2**, **user-action wait
44.3 s → 10.7 s**, and the tail **87.9 s → 18.1 s**. The command-count reduction from the content-addressed
transfer cannot be measured until #48 — #47's guest has no digest index, so on #47 an alias is refused and
re-driven (bounded to one extra round trip by `mayAlias`; the run above recorded **no** refusal notices).

---

## 6 Residual — named, not hidden

1. **One page still costs ~84 worker round trips and ~0.64 s on an idle actor** (§1.3). Nothing in this
   wave touches that; it is the per-command ingress transaction in `runQueuedTurn`, and it is why a user
   action still waits ~7 s behind one page at Nakagin scale. This is the next ceiling.
2. **The page scan is cursorized for no reason.** `Puzzle3dPrecomputeCommandWork` claims ~23 work items per
   page to walk a 5 464-character base64 string 512 characters at a time (`✏️editor/🦀️.rs:6747-6794`), and
   the reducer then decodes the same string again. A base64 admissibility pass over 5.5 KB is microseconds
   against a 7 500 µs step budget. Deliberately NOT changed here: `✏️editor/🦀️.rs` was being rewritten by a
   live peer throughout this wave, and whether a work *step* costs a reactor *turn* needs the guest-side
   instrumentation only #48 can carry.
3. **The real fix is the pull lane, and its exact blocker is now known.** `Effect::HttpRequest` + the
   parked-future `RequestRegistry::request` is the one path with no 8 KiB cap. It needs (a) an
   `http-request` case in `wireEffectToFriendly` and a host answerer built on the `guestAnswerPages` +
   `completed` pair `captureExtensionCompletion` already uses, and (b) a guest `AsyncTask` to await it —
   the synchronous command arm cannot. Both are framework-altitude, both are unverifiable before a wasm
   build, and (b) overlaps B21's scheduling lane; this wave deliberately did not start it.
4. **`:6013` was contended.** Up to five peer probes shared the serve; the `…T17-54-55` baseline ran
   alongside another `browser-probe`, and the serve stopped answering for ~9 minutes mid-wave (never
   killed, polled back). §1.1's uncontended run is the clean per-page floor.
5. **8 pre-existing puzzle-3d lib failures and 1 vitest failure** (§4.2, §4.3) are peer-owned and
   unrelated; they are listed so #48's gate reader does not attribute them here.
