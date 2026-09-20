# HT3b — the socket / revocation / checkpoint reds (10 of the hub suite's last 18)

Slice HT3b, 2026-09-20 ~15:40–17:1x, against the coordinator rerun 15:20 (`321 — 303 / 18`,
`🗑️generated/coordinator-hub-nextest-full.txt`). Sibling HT3a owns `inference/` (the other 8).
Spec read: `📓️ht1-hub-suite-to-green.md` §H1–H18 (HT2's three batches, KD2, `SOCKET_RENDEZVOUS_HANG_GUARD`).

**Constraint that shaped this slice**: preamble rule 26 — no `cargo test/nextest/build -p semio-hub` from a
worker. Every root below is derived from source and proven only by `cargo check -p semio-hub --all-targets`.
Nothing here is claimed as "runs green"; the coordinator's rerun is the oracle.

## 0. The ten, with the panic they died on (rerun 15:20)

| # | law | site | panic |
|---|---|---|---|
| 1 | `socket_admin_user_gate_rejects_a_late_same_user_grant_after_batch_revoke` | `🔬️bin-unit:6994` | `the admin revoke gate was never admitted: Elapsed(())` |
| 2 | `scoped_directory_socket_removal_and_delivery_have_one_total_membership_order` | `:6837` | `the removal-wins sender gate was never admitted: Elapsed(())` |
| 3 | `socket_grant_revoke_before_command_admission_has_no_storage_effect` | `:2121` | `the revocation close never arrived: the server closed no socket for this revoke` |
| 4 | `scoped_directory_socket_route_rejects_scope_substitution_and_rest_removal_closes_without_event` | `:2085` | `no directory message before 5s deadline` |
| 5 | `admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` | `:2068` | `stream ended before server frame` |
| 6 | `document_open_plan_admin_revocation_invalidates_session_and_share_bindings` | `:3198` | `left "cancelled" == right "succeeded"` |
| 7 | `directory_event_page_v1_route_revalidates_session_generation_after_read_before_response` | `:5759` | `assertion failed: control.cancelled.load(Acquire)` |
| 8 | `retained_short_admin_request_drop_duplicate_cancel_and_secret_lifecycle_is_exact` | `:6705` | `left "cancelled" == right "succeeded"` |
| 9 | `checkpoint_publication_route_is_author_owned_actor_fenced_idempotent_and_cancellation_safe` | `:6761` | `author checkpoint publication: left 409 == right 200` |
| 10 | `checkpoint_publication_route_rejects_stale_or_cross_scope_inputs_before_publication` | `:6897` | `left 1 == right 0` |

## 1. The shared root the brief asked for — and it is not the one the brief guessed

The brief expected "revocation must close live sockets with a typed frame and fence new admission". That is
**not** what the socket group is dying of. Three of the four socket laws never get far enough for a
revocation to matter: **their socket task is parked in a `TestLiveGate` pause seam the law never names.**

`TestLiveGate` (`🏗️bootstrap/🦀️.rs:703`) has two kinds of pause seam:

- **opt-in** — `socket_scoped_send_mode`, `socket_membership_remove_enabled`, `directory_command_pause_user`,
  `admin_effect_pause_enabled`, `directory_event_page_fence_enabled`, `checkpoint_publication_pause_enabled`.
  A law that does not set the flag never stops there.
- **unconditional** — two of them, which pause **every** socket of that kind as soon as *any* law installs a
  gate at all, for any unrelated reason:

| seam | site (before) | who consumes it | who it trapped |
|---|---|---|---|
| `socket_directory_admitted` / `socket_directory_release` | `🏗️bootstrap/🦀️.rs:6985` in `handle_directory_ws_v1`, immediately after binding validity and **before replay** | 2 laws (`🔬️bin-unit:4011`, `:4086`) | **law 2** — its two scoped sockets block before a single frame is considered, so `socket_scoped_send_admitted` can never be signalled |
| `socket_broadcast_received` / `socket_broadcast_release` | `🏗️bootstrap/🦀️.rs:4924` in the document socket's `broadcast_rx.recv()` arm | 1 law (`:3372`) | **law 3** — `handle_client_frame` relays an accepted `Commands` batch through `fanout.send(...)` (`:4520`) and the **sending socket is itself a subscriber** (`:4822`), so its own accepted batch 90 comes straight back and parks the task forever; the revocation close it is waiting for can never be written |

**Landed**: both seams are now opt-in, the same way their six siblings already are —
`socket_directory_pause_enabled` and `socket_broadcast_pause_enabled` (`🏗️bootstrap/🦀️.rs:720`, `:724`, with
`Default` at `:769`/`:773` and the two guarded sites). The three laws that genuinely use the seams set the
flag (`🔬️bin-unit:3356`, `:3997`, `:4080`). No seam was deleted and no law's assertions changed: the laws
that name a seam still rendezvous at it exactly as before.

A second, independent defect in law 3's rendezvous: both `socket_command_received.acquire()` calls dropped
their permit instead of `forget()`ing it, so the permit was returned to the semaphore and the *second*
acquire could be satisfied by the *first* command's permit — the law's "the revoke wins before command
admission" ordering was not actually established. **Landed**: both now `.forget()` (`:6544`, `:6553`),
matching every other `.forget()`ing rendezvous in the file.

## 2. The second shared root — a repeat `AnnounceDocument` emits no event (laws 2 and 4)

Both laws produce the scoped event they wait for by announcing an **already announced** document a second
time. `decide` refuses to invent an event for that:

```
🌎️hub/📇️directory/🦀️.rs:2110   Some(existing) if existing == *descriptor => Ok(Decision { events: Vec::new(), result: None }),
```

and `announce_document_for_test` always builds the identical `document_descriptor_for_test`. This is
long-standing (`git show HEAD:🌎️hub/📇️directory/🦀️.rs` has it at `:1928`) and it is *correct* — a document
descriptor is immutable, so a re-announce is a no-op. The laws cannot be repaired by announcing earlier
either: the scoped socket only delivers messages matching its own `space_id`+`document_id`
(`directory_message_matches_scope`, `🏗️bootstrap/🦀️.rs:6731`), and the scoped socket-grant route refuses a
document that has no descriptor yet (`issue_scoped_directory_socket_grant`, `:3208` → `404`), so the
document must already exist before the socket can be opened at all.

**Landed**: both laws now produce the scoped frame the only repeatable way the wire offers —
`ArtifactCheckpointPublished`, which `decide` emits as exactly `single(...)` per publication
(`📇️directory/🦀️.rs:2203`), so there is no second event to race the assertion.

- law 2 (`:6795`+): `announce_document_for_test` → `seed_genesis_document_for_test(…, "scoped-order-document")`
  (creation-owned id, its own author, the member stays a Spectator); each trigger →
  `publish_checkpoint_for_test(&state, STUDIO, &order_document)`; the delivery-wins assertion now pins the
  **exact** `checkpoint_id` returned by that publication and prints the delivered message on failure.
- law 4 (`:3465`+): the unaffected user's document is genesis-seeded through its own owner session
  (`seed_genesis_for_document_for_test`), and "another user's exact scoped subscription remains live" is now
  proven by publishing a checkpoint on it and matching scope **and** `checkpoint_id`.

Strictly more is asserted than before (identity of the delivered checkpoint, not just the event kind).

## 3. Law 6 — the law hand-builds an administrator the product no longer accepts

`document_open_plan_admin_revocation_invalidates_session_and_share_bindings` called `execute_admin_intent`
directly with a synthetic `AdminPrincipalV1 { user_id: "open-plan-admin", auth_session_id:
"open-plan-admin-session", identity_subject_digest: [7; 32], … }`. Since `execute_admin_intent` acquires its
own authority (`🏗️bootstrap/🦀️.rs:8627` → `acquire_admin_intent_authority`, `:8573`), that principal is
refused twice over — it is in no `state.admin_subjects`, and `socket_session_binding` finds no such durable
session — and `admin_directory_authority_refusal` journals phase **`"cancelled"`**. That is the whole `left
"cancelled" == right "succeeded"`; nothing about revocation was ever reached.

**Landed** (`🔬️bin-unit:3163`): the law now registers a real administrator (`authorize_test_admin`) and
resolves the principal through the production seam (`authenticate_admin_principal`), keeping its own
correlation id. The revocation assertions are untouched.

## 4. Law 1 — the seam the law waits on genuinely did not exist; it does now

Confirmed HT2 §H15 row 1 independently: `socket_admin_revoke_admitted` / `socket_admin_revoke_release` were
declared (`:722`) and constructed (`:771`) with **no producer anywhere in the tree**.

The ordering the law asserts is real and is implemented: `admin_intent_bindings` for
`RevokeUserSessions` includes `SocketBindingKeyV1::User(user_id)` (`:8563`), `acquire_admin_intent_authority`
takes that gate, and a socket-grant mint for the same user takes the same key
(`SocketSubjectV1::admission_bindings`, `:917` → `socket_record_bindings`, `:993` → `issue_socket_grant`'s
`acquire_record`, `:2698`). So a same-user grant genuinely queues behind a batch revoke — there was simply no
place for a law to observe the window.

**Landed**: the missing seam, opt-in like its siblings — `socket_admin_revoke_pause_enabled` plus the pause in
`execute_admin_intent` immediately **after** the authority guards are held and before `admit_effect`
(`🏗️bootstrap/🦀️.rs:8634`), firing only for `AdminIntentV1::RevokeUserSessions`. Law 1 sets the flag
(`🔬️bin-unit:6978`). Three other laws issue `RevokeUserSessions` (`:3783`, `:3934`, and law 6) and are
unaffected because they do not set it.

## 5. Laws 9 and 10 — the counts predate the fixture's creation genesis

`checkpoint_publication_fixture` publishes a real creation genesis before the law starts
(`🔬️bin-unit:1009`, `publish_genesis_checkpoint_for_test`), and `artifact_checkpoint_count` counts it. Law 10
dies on exactly that: `left: 1, right: 0` where the law means "no publication happened".

**Landed**: the six count assertions in the two laws are rebased on the seeded genesis with messages that
say so — `1` where the law means "genesis only, no publication" (`:6899`, `:6919`, `:6948`, `:6973`) and `2`
where it means "genesis plus exactly one publication" (`:6766`, `:6771`). The cross-scope count stays `0`
(that scope is announced, never genesis-seeded).

**Law 9 is not closed by this.** Its first failure is earlier — `POST …/checkpoint-publications` answers
**409 with an empty body** where the law expects 200 — and I could not settle which conflict without running
it. The two candidates, both consequences of the genesis the fixture now seeds, are (a) the ordinary
publication re-reserves artifact-CAS ownership for the *same* pack/SPR bytes the genesis already staged (the
fixture publishes the genesis and then the ordinary checkpoint from one `pack`/`spr` pair), and (b) a lineage
fence between the genesis baseline and `snapshot.frontier`. The first thing the next rerun should print is
the refusal reason, which the route currently discards.

## 6. Laws 5, 7, 8 — narrowed, not closed, and why

**Law 5** (`stream ended before server frame`). The message is a **reporting artefact**, not the fact:
`next_server_frame` treated a `WsMessage::Close` as "keep looping" (`Ok(Some(Ok(_))) => continue`), so a
server close frame was swallowed and the following EOF was reported as "stream ended". The server is closing
the document socket before `Welcome` and the law never saw the code or reason. **Landed**: `next_server_frame`
and `next_directory_message` now panic with the close frame itself
(`the server closed before its next frame: {frame:?}`), so the next rerun names the actual refusal. On
source, the pre-`Welcome` close paths are `document_plan_socket_validity(&state, &socket_grant,
Some(&surface))` (`🏗️bootstrap/🦀️.rs:4712`) and `register_live` (`:4702`) — consistent with HT2 §H9's reading
that the genesis brings the plan/surface validity into the path. H3's `404` fix stands and must not be
reverted.

**Law 7** (`control.cancelled` false). Traced to the one contradiction I could not resolve from source:
`DirectoryEventPageHttpControl.active` is cleared by exactly two paths — `Drop for
DirectoryEventPageHttpRequest` (`:6483`), which sets `cancelled` **first**, and `finish_response_owned`
(`:6479`), which does not. The law observes `active == false` with `cancelled == false`, so the handler
future **ran to completion** rather than being dropped by the client abort. But the handler is parked in the
`directory_event_page_read_admitted` fence (`:6535`, one per request) which the law only releases *after*
the assertion, so on source it cannot complete. Either hyper does not drop the in-flight h1 handler on read
EOF (in which case the law's premise "aborting the client cancels the server request" is wrong for this
route as wired, and the `Drop` guard is unreachable for a disconnect), or the control the law reads from
`gate.directory_event_page_control` is not the second request's. Both are one print away; I did not guess.

**Law 8** (`first_rows[1].fact.phase` = `"cancelled"`). The law's claim — "the HTTP waiter expires without
cancelling its admitted writer" — is a real product claim and the retained writer is past `admit_effect()`
(`:8453`) when its waiter gives up, so none of `execute_admin_intent`'s three `"cancelled"` returns should be
reachable. The remaining `"cancelled"` producer is the retained task's re-authentication arm
(`:8838`, `_ => AdminIntentExecution { phase: "cancelled", … code: "admin-authority-changed" }`), which would
mean `authenticate_admin_principal`/`same_authority` fails for the retained task once its HTTP waiter is
gone. The terminal row's **outcome code** discriminates the two immediately and the law does not currently
print it — worth adding before the next attempt.

## 7. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht3b cargo check -p semio-hub --all-targets` (after §1–§5) | **EXIT 0, 0 errors** | `🗑️generated/ht3b-check-1.txt` |
| same, after §6's helper change and law 3's `forget()` | **EXIT 0, 0 errors, 308 warnings** (warnings prove the expansion type-checked) | `🗑️generated/ht3b-check-2.txt` |

No hub test/nextest/binary build was run from this slice (preamble rule 26). **Nothing in §1–§6 is verified
at runtime.**

## 8. Files changed

- `🌎️hub/🏗️bootstrap/🦀️.rs` — `TestLiveGate` gains `socket_broadcast_pause_enabled`,
  `socket_directory_pause_enabled`, `socket_admin_revoke_pause_enabled` (declaration + `Default`); the
  document-socket broadcast pause (`:4924`) and the directory-socket admission pause (`:6985`) become opt-in;
  `execute_admin_intent` gains the admin-revoke admission seam (`:8634`).
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — the three seam-owning laws set their flag; law 1 sets the revoke flag;
  laws 2 and 4 seed a genesis and trigger `ArtifactCheckpointPublished` instead of a no-op re-announce; law 3
  `forget()`s its command-boundary permits; law 6 uses a verified administrator principal; the six checkpoint
  counts in laws 9/10 are rebased on the seeded genesis; `next_server_frame` / `next_directory_message` report
  a close frame instead of swallowing it.

## 9. Batch 2 — measured (coordinator rerun 16:16: **321 — 304 / 17**) and answered

Batch 1 flipped **law 1** (`socket_admin_user_gate…`) and **law 4**
(`scoped_directory_socket_route_rejects_scope_substitution…`) to green, and moved four more to a new,
sharper panic. One new red appeared. All of it below.

### 9.1 The new red is NOT the seam change — it is a wall clock on an open

`tests::directory_invite_redemption_admitted_fence_precedes_archive` died at `🔬️bin-unit:5362:97`,
`invite order state deadline: Elapsed(())` — that is the law's **first line**,
`timeout(5 s, test_state())`, before a `TestLiveGate` exists. The law opens no socket, dispatches no
admin intent and never touches `socket_directory_pause_enabled` / `socket_broadcast_pause_enabled` /
`socket_admin_revoke_pause_enabled` (checked by grep over its body). Opening a hub state simply took
longer than 5 s under the rerun's load.

Seven laws carried that same ad-hoc 5 s bound. **Landed**, following HT2 §H10/§H15's rule (one named
bound, never a per-call duration, and never deleted): `TEST_STATE_OPEN_HANG_GUARD = 60 s`
(`🔬️bin-unit:490`), used by all seven. A genuinely wedged open still fails fast and locally with its own
site; fleet scheduling latency no longer does.

### 9.2 Law 2 — phase 1 now passes; the failure moved to a permit that was never consumed

New panic `:6855` `removal waits while an admitted scoped send owns the membership gate`. §1 and §2 both
worked: the socket is no longer parked, and `publish_checkpoint_for_test` delivers a real scoped event, so
the whole removal-wins phase and the delivery-wins admission now pass. What remains is the same class of
bug as law 3's: `gate.socket_scoped_send_admitted.acquire()` **dropped** its permit instead of
`forget()`ing it, so the returned phase-1 permit satisfied the phase-2 acquire instantly — the test moved
on while no scoped send yet held the membership gate, and the removal it spawned was free to complete.

**Landed**: `.forget()` on the removal-admission, removal-wins and delivery-wins acquires (`:6833`,
`:6837`, `:6849`) and on law 1's admin-revoke acquire, so each rendezvous consumes exactly the permit it
waited for.

### 9.3 Law 3 — the self-relay is legitimate, and the law must read it

New panic `:2119` `authority-bearing binary frame crossed revocation`. This is a direct consequence of
§1: with the broadcast seam opt-in, the socket no longer parks on its own relayed batch, so
`fanout.send(commands_frame)` (`🏗️bootstrap:4520`) reaches the very socket that sent it (`:4822`) — an
**authorized, pre-revoke** frame that the law's `next_close_without_authority` was reading as a leak.

**Landed**: the law now reads it, immediately after the batch-90 Ack —
`assert!(matches!(next_server_frame(…), ServerFrame::Commands { .. }), "the authorized batch's own relay
is delivered before any revoke")`. The law asserts strictly more than before: the accepted batch's relay
is ordered before the revoke, and nothing authority-bearing follows it.

### 9.4 Law 6 — the revoke half is green; the share half needed the fixture's own space

New panic `:3196:114` `share issue: NotFound("document descriptor raum:ä/plan:東京")` — §3 worked
(`session_revoke.phase == "succeeded"` and the plan is `Stale`), and the law now fails one step later:
`issue_share_token(&fixture.valid_plan.scope, …)` on a scope whose space and descriptor were never
created, because `test_state()` seeds neither `raum:ä` nor `plan:東京`.

**Landed**: the law creates the fixture's own space through the admin seam with its verified principal
(`execute_create_space_with_id_and_admin_effect`) and announces `fixture.descriptor` before issuing the
share. Nothing in the share-revocation assertions changed.

### 9.5 Law 5 — the close frame is now visible: **4401 `unauthorized`, before `Welcome`**

§6's helper change paid off in one rerun: `the server closed before its next frame: Some(CloseFrame {
code: Library(4401), reason: "unauthorized" })`. The member's plan-exchanged document socket is refused
before the welcome. On source there are exactly three pre-`Welcome` 4401 sites, in order:
`state.socket_grants.register_live` (`🏗️bootstrap:4702`), `socket_grant.subject.revalidate` (`:4711`), and
`document_plan_socket_validity(&state, &socket_grant, Some(&surface))` (`:4712`). The third is the
likely one and has six collapsed refusal reasons (`:4184`–`:4213`): plan `validate()`, subject/actor/
audience/surface mismatch, missing descriptor, `descriptor != authority.descriptor`, catalog generation,
and the `resolve_document_open` selection tuple. Since H3 the law's descriptor is genesis-written
(`bootstrap_snapshot_hash` = sha256(pack)) while the plan and the openable catalog are both built from
the descriptor read back afterwards, so a descriptor mismatch should be excluded — which points at the
catalog selection tuple. **Not closed**: discriminating those six needs one run, which this slice cannot
do (rule 26). The next owner should split that function's `Unauthorized` returns into distinct close
reasons; it is one edit and it ends the guessing.

### 9.6 Laws 9 and 10 — one root, and it is inside `checkpoint_publication_response`

Law 10 moved past the counts (§5 worked) to `:6912` `publication fence admission deadline: Elapsed(())`:
its queued publication never reaches the `checkpoint_publication_admitted` pause because it is refused
first — the same refusal law 9 sees as `409` on the very first author publication. The claim ledger is
excluded as the cause: authorization (`403`) precedes `claim_or_read_checkpoint_publication`
(`🏗️bootstrap:4090` vs `:4101`), so the law's earlier spectator attempt cannot poison the correlation id,
and the claim is fresh. The refusal therefore comes from `checkpoint_publication_response` (`:4133`),
whose `409` carries an **empty body** by contract (a green assertion in law 9 pins
`conflict.body.is_empty()`), so it is undiagnosable from the wire. Ranked suspects, both consequences of
the genesis the fixture now seeds: the ordinary publication re-reserves artifact-CAS ownership for the
*same* pack/SPR bytes the genesis already staged, or a lineage fence between the genesis baseline and
`snapshot.frontier`. **Not closed** — needs a span or a one-off print inside that function.

### 9.7 Laws 7 and 8 — unchanged, roots as stated in §6

Both failed identically to the 15:20 run. §6's analysis stands; each is one print away.

## 10. Verification (batch 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht3b cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors** | `🗑️generated/ht3b-check-3.txt` |

No hub test/nextest/binary build from this slice. **Nothing in §9 is verified at runtime.**

## 11. Files changed (batch 2)

`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` only — `TEST_STATE_OPEN_HANG_GUARD` and its seven call sites;
`.forget()` on four gate rendezvous (laws 1 and 2); law 3 reads its own authorized relay; law 6 creates
the fixture's space and announces its descriptor.

## 12. Batch 3 — typed refusals, as the coordinator asked (rerun 16:34: **321 — 306 / 15**)

Batch 2 flipped **law 6** (`document_open_plan_admin_revocation…`) green and returned
`directory_invite_redemption_admitted_fence_precedes_archive` to green (§9.1's guard held). Laws 2 and 3
came back with identical panics, so batch 3 stops guessing at them and makes every remaining refusal
**name itself**.

### 12.1 Two typed refusal vocabularies, recorded process-locally, wire unchanged

The blocker on four laws was the same shape twice: a real refusal collapsed into one opaque code with an
empty body. Landed in `🏗️bootstrap/🦀️.rs`, `#[cfg(test)]` only:

| vocabulary | refusals it discriminates | recorded at |
|---|---|---|
| `CheckpointPublicationRefusalV1` | `DescriptorDigestDiffers`, `DocumentSnapshotDiffers`, `ExpectedCurrentDiffers`, `BaseFrontierUnavailable`, `PackBlob(_)`, `SprBlob(_)`, `Materialize(_)`, `Publish(_)` | all eight refusal sites of `checkpoint_publication_response` |
| `DocumentPlanRefusalV1` | `PlanInvalid`, `BindingMismatch`, `DescriptorMissing`, `DescriptorDiffers`, `CatalogSelectionDiffers`, `DirectoryRevisionDiffers`, `CheckpointDiffers` | the seven `Unauthorized` returns of `document_plan_socket_validity` |

**Why process-local statics and not `TestLiveGate`**: a law cannot install a gate merely to read a
diagnostic — the document socket's pre-`Welcome` seams (`socket_before_welcome`, `socket_after_welcome`,
`document_subscribed`, …) are still unconditional, so giving law 5 a gate would park its own socket and
replace one red with a hang. nextest runs **one process per test**, so
`LAST_CHECKPOINT_PUBLICATION_REFUSAL` / `LAST_DOCUMENT_PLAN_REFUSAL` are per-law by construction. The
`Unauthorized` value and the `409`/`503` status are returned exactly as before under `cfg(not(test))`;
no route, close code or body changed.

Readers: law 9's `author checkpoint publication` assertion and law 10's fence-admission deadline now
print `refused as {:?}`; law 5 reads its first frame inline so the close names
`refused as {:?}` instead of dying inside the helper.

### 12.2 Laws 2 and 3 — the two facts I still cannot see, now printed

- **law 3** (`:2126` `authority-bearing binary frame crossed revocation`): `next_close_without_authority`
  discarded the frame it rejected, so "which frame crossed" was unknowable. It now decodes and prints it
  (`Ack`? the batch-90 `Commands` relay a second time? a `Presence` fanout?). §9.3's read of the relay did
  not close it, so the crossing frame is a *different* one — most likely the `ServerFrame::Presence`
  fanout published on socket open (`🏗️bootstrap:1939`), which the opt-in broadcast seam now also lets
  through. One rerun names it.
- **law 2** (`:6886`): established from source that the removal *does* take
  `SocketBindingKeyV1::Membership { user_id, space_id }` (`acquire_directory_command_fence`, `:5929`) and
  that a paused scoped send *does* hold it (`directory_message_bindings` == `record.bindings()` for a
  scoped audience, pinned by the green law at `:3979`). So the failing assertion means the removal was
  never actually blocked — i.e. the mode-2 admission the law waited for came from a send that is no longer
  paused. The assertion now prints the removal's own outcome
  (`Ok(Ok(Ok(n)))` = applied `n` events vs `Ok(Ok(Err(Unavailable)))` = fence expired), which separates
  "the gate was never held" from "the gate was held and the 2 s fence lost".

## 13. Verification (batch 3)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht3b cargo check -p semio-hub --all-targets` (diagnostics) | **EXIT 0, 0 errors** | `🗑️generated/ht3b-check-4.txt` |
| same, after the law-2 / law-3 printers | **EXIT 0, 0 errors** | `🗑️generated/ht3b-check-5.txt` |

No hub test/nextest/binary build from this slice. **Nothing in §12 is verified at runtime.**

## 14. Files changed (batch 3)

- `🌎️hub/🏗️bootstrap/🦀️.rs` — `CheckpointPublicationRefusalV1`, `DocumentPlanRefusalV1`, their two
  process-local slots and recorders; the eight `checkpoint_publication_response` refusal sites and the
  seven `document_plan_socket_validity` `Unauthorized` sites record before returning.
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — laws 5, 9, 10 read the typed refusal; law 3's crossing frame is
  decoded and printed; law 2's early-removal outcome is printed.

## 15. Batch 4 — the one root behind laws 2, 3 and 7, and it is NOT an authorization defect

**Law 3's `Ack { batch_id: 91, … Persisted, Applied }` is real but its cause is the harness, and the
product's fence is intact.** Proven from the law's own step order and from the seam:

1. The law sends batch 91, then waits on `socket_command_received`, then revokes, then releases. So the
   law's sequencing is correct — it does intend the revoke to win.
2. The server's pause seam was `let _ = live_gate.socket_command_release.acquire().await;`
   (`🏗️bootstrap:5003`). `acquire()` yields a `SemaphorePermit` and `let _ =` **drops it immediately,
   returning the permit to the semaphore.** So the permit the law added to release *batch 90* was handed
   straight back, and batch 91 never paused at all: the server ran `socket_live_authority` **before**
   `delete_session_me` was even called, found the grant live (it was), and persisted. Nothing crossed a
   revocation — the revocation had not happened yet.
3. Confirmation from law 2's new printer in the same rerun: `it instead returned Ok(Ok(Ok(1)))` — the
   removal **applied its event**, i.e. it was never fenced, because `socket_scoped_send_release`
   (`:6967`, `:6981`) returns its permit the same way, so the second scoped send never paused and never
   held `Membership{user,space}`. The fence itself is correctly taken (`acquire_directory_command_fence`,
   `:5929`) and correctly held by a paused send (`directory_message_bindings` == `record.bindings()`).

All **twelve** pause seams in `🏗️bootstrap/🦀️.rs` were written this way, while every seam in
`🔬️standalone/🦀️.rs` (`pause_admin_effect_started`, `pause_directory_command_authority`,
`pause_global_directory_send_for_test`) already used `.expect(…).forget()`. **Landed**: all twelve now
consume their permit — `checkpoint_publication_release`, `socket_welcome_release`,
`socket_bootstrap_release`, `document_release`, `socket_command_release`, `socket_broadcast_release`,
`socket_lag_release`, `directory_event_page_read_release`, `socket_scoped_send_release` ×2,
`socket_directory_release`, `socket_admin_revoke_release`.

**This also resolves §6's law 7 contradiction.** `directory_event_page_read_release` (`:6647`) returned
its permit too, so the second request never parked at the fence: it ran to completion,
`finish_response_owned()` cleared `active` **without** `cancel()`, and the law read exactly that. That is
why `active == false` and `cancelled == false` could coexist — no hyper behaviour was involved.

A pause that only pauses once is worse than no pause: it makes an ordering law assert the opposite of
what it reads. Nothing was deleted or loosened; the twelve seams now do what their laws already assume.

## 16. Verification (batch 4)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht3b cargo check -p semio-hub --all-targets` | **EXIT 101 — one error, and it is not mine**: `can't call method checked_add on ambiguous numeric type` at `💡️inference/🧾️wal/🦀️.rs:371`, sibling HT3a's in-flight edit. No error in `🏗️bootstrap/🦀️.rs` or `🔬️bin-unit/🦀️.rs`; the previous five checks of my files were EXIT 0 | `🗑️generated/ht3b-check-6.txt` |

Per preamble rule 3 I did not wait on the peer. The rerun needs HT3a's `wal` line to compile.

## 17. Hand-over — every remaining red, what it printed, and where to look

| law | printed | root hypothesis | site |
|---|---|---|---|
| `socket_grant_revoke_before_command_admission…` | `Ack{91, Persisted, Applied}` | **fixed** — the release permit was returned, so batch 91 never paused and the revoke had not yet run. NOT an authorization defect | `🏗️bootstrap/🦀️.rs:5003` |
| `scoped_directory_socket_removal_and_delivery…` | `it instead returned Ok(Ok(Ok(1)))` | **fixed** — same root; the scoped send never paused, so it never held the membership fence | `🏗️bootstrap/🦀️.rs:6967`, `:6981` |
| `directory_event_page_v1_route_revalidates…` | `control.cancelled` false | **fixed** — same root; the second request never parked, so `finish_response_owned` cleared `active` without `cancel` | `🏗️bootstrap/🦀️.rs:6647` |
| `checkpoint_publication_route_*` ×2 | `refused as Some(DocumentSnapshotDiffers)` | the route's fresh `checkpoint_publication_snapshot()` disagrees with the command the fixture built; the diagnostic now carries **both** frontiers + authority generation, which names the differing field in one rerun | `🏗️bootstrap/🦀️.rs:3723` (predicate), `🔬️bin-unit/🦀️.rs:1009` (fixture) |
| `admin_removal…` | `server closed before its next frame: 4401` | a 4401 **after** the welcome, so `socket_live_authority` → `document_plan_socket_validity(…, None)`; the close panic now prints the typed variant. My ranking: `DirectoryRevisionDiffers` (the plan pins `head_seq`, the law appends events after issuing plan B) | `🏗️bootstrap/🦀️.rs:4196`+ |
| `retained_short_admin_request…` | `left "cancelled" == right "succeeded"` | only reachable `"cancelled"` is the retained task's re-auth arm; the terminal row's **outcome code** discriminates it and the law does not print it yet | `🏗️bootstrap/🦀️.rs:8838`, law at `🔬️bin-unit/🦀️.rs:6736` |

Unrelated hygiene, not mine: `eprintln!("[DEBUG] …")` at `🔬️bin-unit:1091`, `:3986`, `:4130`.

**needs hub rerun** — batch 4: twelve pause seams now consume their release permit (one root under laws
2, 3 and 7), plus the snapshot diagnostic carrying both frontiers and the plan refusal in the close
panic. My files check clean; the tree currently fails on HT3a's `💡️inference/🧾️wal/🦀️.rs:371`.
