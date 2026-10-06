# 🪆 S5-NESTED — composed members, recursive composition (design §21.9)

Owner: S5-NESTED (session 5, launched 02:38, cut 02:40, resumed 07:28 on 2026-10-05). No predecessor report exists (new WP).
Scratch: `🗑️generated/s5-nested/`. Input scripts: `🧪️s5-nested-*.py` in the ticket root.

## §21.9 design (07:50, read from the tree after commit 670; gist sent to `main` before any §21.9 code)

**What is already recursive (peer, commit 670 — verified by reading, laws in `🧪️tests/🧩️composition/🦀️.rs`):** the archive entry
(`owner {parent ref, slot, child_id}`), the `.spr` owner stamp, the closure validator (BFS over every member's own projection;
LAW: artifact ids are unique per document closure → `DuplicateMember`), the replacement/archive-load ladder (opens members of any
depth into ONE flat registry + ONE flat content root + the artifact-id-keyed composition graph), `document_archive` export.
**What is root-only / colliding:** the flat registry and content root are keyed `(owner.slot, owner.child_id)` whatever the owner
(a grandchild collides with a root child of the same step); boot genesis, derived follow, archive/replacement genesis and
`open_child` read the ROOT projection and `A::genesis_child_pack` only; follow is gated by the ROOT generation alone; members
have no derivation authority; lanes, emits and history address a member by `(slot, child_id)`.

1. **Owner-path type** (plugin runtime, beside `ChildMemberRegistry`): `MemberStep { slot, child_id }`, `MemberPath(Vec<MemberStep>)`
   (non-empty; a root child is one step). `MemberPath::child(slot, id)`, `.join(slot, id)`, `.owner() -> Option<MemberPath>`,
   `.step()`, canonical text `slot/childId[/slot/childId]*` with `%25`/`%2F` escapes (ONE codec: `Display` + `parse`; depth-1
   text of every id without `/` or `%` is byte-identical to today's `<slot>/<childId>`).
2. **Keys.** `ChildMemberRegistry` (`locate/admit/get/get_mut/remove`), `ChildContentRoot`/`ChildContentView` and the per-member
   lanes are keyed by `MemberPath`; entries keep `owner: OwnerRef` (parent REF) and gain `path`. `ChildContentView` carries a
   scope: `typed_read(slot, id)` resolves `scope.join(slot, id)`; `view.member(slot, id)` is the same root scoped to that
   member — a member artifact's readers compose on read with unchanged calls whether it is the document or a member.
3. **Derivation authority.** `MemberFactory::genesis_child_pack(&self, slot, child_id)` (live member, typed snapshot) and
   `MemberFactory::genesis_children(dialect, initial_pack)` (a freshly minted member, decoded from its own genesis pack);
   `space_members!` generates both per variant from an optional derivation fn of the member artifact's app
   (`Variant(kind, standard, subset, schema) => (Snapshot, Mutation, <FormsApp as ArtifactApp>::genesis_child_pack)`; absent = derives nothing).
4. **The five paths recurse with the member as `owner.parent`:** (a) boot genesis — worklist over root + every opened member;
   (b) follow — per owner (root + every member): mint declared-unheld derivable children, retire held ones whose slot names
   another derived child (a retired member retires its subtree, leaves first); armed by the root generation AND by every
   child-lane publication (`publish_child_content_member`, composite emits, member undo/redo/checkout, inbound lanes, member
   time-travel finalize, member tool-run finalize); (c) archive-load genesis and (d) replacement genesis — every MINTED member's
   derivable children are minted from its genesis pack (`genesis_children`), recursively, before the roster seals (an archived
   member with history ships its own children: a complete archive, else `Incomplete` as today); (e) `open_child_at(owner path,
   …)` — declared reference and restore validation read the OWNER's projection (root snapshot or the owner member's
   `child_restore_projection()`), owner ref = that member's reference.
5. **History / time travel / tool run on a nested member** need no new mechanism: rows, ledgers and visitors iterate the flat
   registry; the `store` id is the path text; `TimeTravelMemberSubject` and the member tool-run target hold a `MemberPath`
   (8 mechanical sites in S5-RUNTIME's `⏪️time-travel` and S5-GRAPHS-WIRES' `⏯️tool-run`).
6. **S5-STORE:** nothing persisted changes (`.spr` owner stamp, archive, pins, graph stay keyed by owner ref / artifact id under
   the closure law). Wire: see 7. Recorded limit that follows from the closure law: derived child ids must be unique per
   document — two sibling members that content-address equal content collide as `DuplicateMember` (an app salts the address).
7. **Wire (NOT persisted) — three records address a member by `(slot, child_id)` and need the owner path for a nested member:**
   `BackboneMessage::Member` (lane identity; Rust only: store, sync, wgpu shell, mcp), `ChildEmit` (`TransactionPrepare.
   prepared_child_ops` value pack; a root app editing a grandchild lane), `ChildPackEntry`/`ChildHeadPackEntry` (frames 14/15/42).
   Each gains ONE string `owner` = path text of the owning member (`""` = the document root). → channel 23 (S5-CHANNEL).
   String-typed addresses (`HistoryMutationEntry.store`, `historyEditBegin.store`, `nextProblem.store`, tool-run `member`,
   `child:<…>` inference keys) keep their layout: the grammar extends, hosts pass them through opaquely.
8. **Landing:** wave N1 (no wire change: 1–5, fixture, laws; a nested member whose lane has events and cannot be addressed is
   refused BY NAME at the announce, never dropped) → wave N2 with the bump (7). Laws: two-level fixture (root → branch member
   deriving a leaf) through boot, child-lane edit + re-mint, save → fresh load, member history edit + time travel after reload.

## Session 5 — 2026-10-05

### P1 — composed fixture red (`child member identity is not declared by the current parent`)

**Verdict (read from code + `git log -S`): the registration rule is right; the fixture must declare the child first.**

- The rule entered with commit 670 (`git log -S'child member identity is not declared by the current parent'` → one commit,
  `5c7f51ee643`, 10-04 19:39). Same commit: `declared_child_reference` (`PLG` ≈26952) became the ONE identity authority of
  `admit_child_member` and `open_child`. Before it the runtime assumed `artifact_id == child_id`
  (`expected = ArtifactRef { artifact_id: child_id.clone(), dialect }`); since 670 the member's artifact id is read from the
  parent's declaration (`fields.artifact_id`), the composition graph is seeded with that id
  (`admit_owns(&parent_id, &slot, &expected.artifact_id)`) and the dialect is checked against the declared target. A member the
  parent does not declare has no identity to admit — and would fail closure validation on the next save anyway.
- Commit 670 converted the fixtures it touched: `retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires`
  (builder contract ≈1925, "the actual parent declares the exact member before registration"), `🧪️tool-run-member` `member_app`
  (≈:113), `ComposedParentApp::initial_snapshot` (declares `child-1` at genesis). The ~20 `contract_composed_app[_raw]()` sites
  were not converted; two of them declared AFTER the composite edit (the old order).
- Production never calls `register_child` (repo grep: tests only) — live members arrive through `open_child` (boot genesis,
  derived follow, `load_child_pack`) and the archive/replacement ladder, all of which read the parent's declaration.
- `🧪️tests/🔬️child-read-capture/🦀️.rs` (commit 670, peer) is NOT mounted by any `include!`/`#[path]` (repo grep) and cannot
  pass as written (slot `workingGraph` on `TestApp`, whose only slot is `slot`; child id with `!` does not survive
  `ArtifactRef::parse_uri`; `install_test_snapshot_retirement` hard-codes `"slot"`). Not compiled, not touched — reported.

**Fix (test side, one helper pair):** `declare_test_child(app, child_id)` appends the member's `ArtifactRef` uri to the parent's
`slot` (`SetSlotChildren`, beside whatever is declared already) and `register_test_child(app, child_id)` = declare + fresh
`new_test_child` + `register_child`. Script: `🧪️s5-nested-p1-fixture.py` (exact-string, counted, fail-closed).

_State: staged, not landed (see log below)._

### Design correction after reading the candidate ladder (09:45) — owner EDGE is the key, owner PATH is the address

Design points 1–2 above said "keyed by `MemberPath`". The replacement/archive ladder admits members UNORDERED (law
`retained_window_input_recursive_replacement…`, vector `recursive-unordered-success`): when a member enters the candidate
registry its owner may not be there yet, so a path cannot be computed at insertion. The key is therefore the OWNER EDGE —
`MemberKey { owner: artifact id of the owning MEMBER | "" = the document, slot, child_id }`, exactly the persisted
`store::OwnerRef` — and `MemberPath` is the public address (`ChildMemberRegistry::resolve(path)` walks down one lookup per step,
`path_of(key)` walks up). Never `(slot, child_id)` alone; nothing persisted changes. Root-ness is one bit per registry slot
(`nested`), set at admission, so no new owned string enters a member entry (their bounded retirement ladders are untouched).
A retiring member is identified by its artifact id (unique per document by the closure law), so `ChildContentEntry` gains
`owner` + `artifact_id` and `ChildMemberRetirement::retires(artifact_id)`. Hash and `identity_digest` of every root-owned entry
are byte-identical to before (the owner prefix is only folded in for a nested entry).

### Wave N1 — STAGED 09:57, dry-run clean on the live tree (`python3 🧪️s5-nested-n1-keys.py check`)

`🧪️s5-nested-n1-keys.py check|land|restore` (pre-images → `🗑️generated/s5-nested/pre-n1/`; lands P1 through the P1 script).
| File | What |
|---|---|
| `🔌️plugin/🦀️.rs` (40 counted replacements + 5 regions) | `MemberKey`, `MemberKeyRef`, `MemberStep`, `MemberPath` (+ text codec); `ChildMemberRegistry` keyed by owner edge (`nested` bitset, `key_at`, `member`/`member_mut`, `keyed_entries`, `resolve`, `path_of`, `addressed_entries`; `get`/`get_mut`/`admit` stay the document's own `(slot, child_id)` step); `ChildContentView` keyed by owner edge (`typed_read_at`, `dialect_at`, `keys`; `slots()` = the document's own steps; `with_member_read(key, …)`); content retirement finds its disposer by key, a retiring one by artifact id; candidate ladder admits `owner.parent == candidate root ? "" : parent id`; `publish_member_content(generation, key)`; `send_member_lane` (a member of a member has no lane before N2: refused, never dropped); checkpoint cascade, checkout pins and group undo/redo re-look members up by key (they used `(owner.slot, owner.child_id)` and would have missed or mis-hit a nested member) |
| `🔌️plugin/⏪️time-travel/🦀️.rs` (16 sites, approved to ride N1) | `TimeTravelMemberSubject { key, path, … }`; `store` id = path text; rows/ledger tails/stamps iterate `addressed_entries()`; `historyEditBegin.store` → `MemberPath::parse` → `resolve` (a nested member is editable as soon as it is live) |
| `🔌️plugin/⏯️tool-run/🦀️.rs` (4 sites, approved) | member runs stay on the document's own members, exactly: the slot census filters root-owned entries, the two `entries().find(slot, child)` scans become the root lookup |
| `🔌️plugin/🧪️tests/🔬️app-child-member-registry/🦀️.rs` | one `hash` call |
| `🔌️plugin/🧪️tests/🧩️composition/🦀️.rs` | the two recursive laws read the leaf through `typed_read_at(MemberKeyRef { owner: "child-1", … })`; `keys().len() == 2`, `slots()` = the branch alone |
| `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | P1 (22 replacements) |
| `🏪️store/🦀️.rs` | ONE `space_members!` arm: `merge_persisted_envelope` delegates to the variant (S5-STORE's item (b)) |

**N1 + P1 LANDED 10:06:19** (rule 67 train: apply-only hold 10:06:05 → 10:06:19, `check` clean against the live tree under the
lock, train line appended; `restore` is only safe until the next wave lands on the same files — after that: fix forward).
Verification = the coordinator's train (`🗑️generated/coord/train.status`), then my targeted plugin test build. Results below.

**Train verdict (read 10:22):** `FRAMEWORK GREEN 10:18:38 through: 10:09:16 … (covers STORE AA, UI rows, PUZZLE 10:04–10:08,
AGNOSTIC P1, NESTED N1, WGPU 5a, RUNTIME H+I, TOOLS press + press-bound)` — kernel + plugin + wgpu renderer + ui `--lib`
compile with N1, and RUNTIME's waves H+I landed on top of my 16 time-travel sites without a re-base.

**Targeted test run 1 (started 10:23 through gate v6 `3 25`, shared build dir per rule 66, output `🗑️generated/s5-nested/test-n1-1.txt`):**
`RUST_MIN_STACK=268435456 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=3 cargo test -p semio-framework-plugin --lib --features artifact-app-testing --message-format=short -- a_child_survives an_agent_transaction_carries_owned_child composite_gesture_produces_one_undo_group a_composed_document_crosses a_composed_documents_child_heads member_factory_closed_dialect member_factory_parent_snapshot_restore a_composed_checkpoint_commit a_checkpoint_pins_its_children child_content_publication_path child_snapshot_retirement_rejection child_root_maintenance child_publications_at_the_maximum_rate maximum_child_public_dispatch the_child_content_view_never_goes_stale group_undo_skips_a_foreign_tail created_children_survive_absorb retained_child_group_publishes retained_window_input_recursive retained_composed_replacement a_whole_document_media_import fixed_child_member_registry stale_child_member_admission incomplete_child_member_registry owned_document_ingress member_run_ member_backfill_answers a_history_edit_is_its_own_row an_overwrite_head_equals_a_fresh_fold every_fixture_scenario_reaches`
Result: **NOT RUN — the gate never opened.** `🚦️gate.sh 3 25` waited from 10:23: free disk fell 32 → 16 GiB (read 10:34,
below the 25 GiB test floor of rule 65) with 4 cargos on the shared build dir, so no cargo was started and no output file
exists. **OWED (rule 65 disk floor): the exact command above, prefixed `zsh T/🚦️gate.sh 3 25 &&`** — it is what turns
"N1 compiles" into "COMPOSED FIXTURE GREEN" (P1's ~20 laws + S5-LOAD's two composed load laws + registry + recursive laws).

**§22.15 nested withdraw law — not writable yet:** `history.unit-spans-documents` exists (`🌿️vcs/🦀️.rs` `UnitSpansDocuments`,
kernel notice row), but `CompositionCoordinator::withdraw_group` (composed members of one instance) is not on disk (repo grep:
0 definitions) — S5-STORE's §22.15 half. The nested law for it follows that landing; nothing in N1–N3 special-cases it.

### Wave N2 — STAGED 09:57 as the LAYOUT half, dry-run clean on N1's planned result (`python3 🧪️s5-nested-n2-wire.py check`)

Channel 23 ("wave C", approved). `🧪️s5-nested-n2-wire.py check|land|restore [--without-frames]` (needs N1 on disk to land).
| File | What |
|---|---|
| `🏪️store/🦀️.rs` (8) | `BackboneMessage::Member { owner, slot, child_id, envelopes }` (owner FIRST), `send_member_mutations(owner, …)`, `take_member_inbound() -> (owner, slot, child_id, envelopes)`, bounded retirement arm takes `owner` too |
| `🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` (1) | the one literal |
| `🔌️plugin/🦀️.rs` (13) | `ChildEmit.owner` (first field; `open` writes "", both close ladders walk it); `MemberPath::owner_text`/`under`; `send_member_lane` sends the owner-path text (exact for any depth); `fold_member_inbound` resolves `(owner, slot, child_id)` through `MemberPath` and keeps the "declared by its owner but not held → fault" law per owner; the two emission gates refuse a non-empty `owner` (nested emits = N3); `child_packs`/`child_head_packs` fill `owner`; `LoadChildren` refuses an entry with an owner (a member's own children load with the archive) |
| `🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` (1) | the one `ChildEmit` literal |
CHANNEL's half (field list sent 09:55): trailing `owner: String` on `ChildPackEntry` (after `envelope_pack`) and `ChildHeadPackEntry`
(after `head_pack`), codec + TS twin + vectors; CHANNEL also owns the other literals: `📡️spr/🧵️channel/🦀️.rs:2611/2621`,
`📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs:460/461/480/481`, `🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs:949/950`.
No TS twin encodes `BackboneMessage` (repo grep: hosts carry backbone bytes opaquely) and `ChildEmit`'s value pack has no
pinned vector (the emission fixture pins refusal behaviour, not layout).
NOT in N2 (wave N3, no further bump): nested emits end to end, nested `load_child_pack`, the five recursive paths, member
derivation authority, the two-level fixture and laws.

### Wave N3 — STAGED 10:18 (the §21.9 recursion), dry-run clean on N2's planned result (`python3 🧪️s5-nested-n3-recursion.py check`)

`🧪️s5-nested-n3-recursion.py check|land|restore` (needs N1 + N2 on disk to land; WRITTEN, NOT COMPILED — no private build dir).
| File | What |
|---|---|
| `🏪️store/🦀️.rs` (5) | `MemberGenesisChild`, `MemberDerivation<P>`, `no_member_derivation`, `member_genesis_children`; `MemberFactory::genesis_child_pack(&self, slot, child_id)` (live member) + `genesis_children(dialect, initial_pack)` (freshly minted member), both defaulting to "derives nothing"; `space_members!` takes an optional third tuple entry per variant — the member artifact's derivation fn (`=> (Snapshot, Mutation, <App as ArtifactApp>::genesis_child_pack)`) — and generates both; every existing roster compiles unchanged |
| `🔌️plugin/🦀️.rs` (7 + 4 regions) | ONE follow pass `follow_member_owners` (breadth first over the document and every live member, the ones it opens included) used by boot genesis and by the live follow; `member_follow_plan(owner)` (unheld-derivable → open, held-undeclared with a replacement → retire with its subtree leaves first, declared-retiring → deferred); the follow gate is `(parent generation, child-content generation)` so EVERY member-lane publication arms it, plus an explicit re-arm after inbound member lanes; `open_member(owner, …)` / `admit_child_member(owner, …)` / `declared_child_reference(owner, …)` / `validate_parent_child_restore(owner, …)` read the OWNER's projection, the owner member's reference becomes `owner.parent`; `derived_member_entries` (shared by archive-load and replacement genesis): every MINTED member's derivable children are minted from its genesis pack, breadth first, owner-stamped, numbered after the archived ones, the member and byte authorities re-checked |
| `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` (3) | the fixture document and `TestMembers` share one derivation (`derived-branch-<n>` → declares + derives `derived-leaf-<n>`); mounts the law file |
| `🧪️tests/🧪️nested-composition/🦀️.rs` (new, from `🧪️s5-nested-n3-law-nested-composition.rs`) | LAW `a_member_composes_and_derives_children_like_a_document_at_every_depth`: one declaration opens two levels; owner edge / owner path / `owner.parent` / graph; member-lane edit re-mints the leaf; a re-pointed member leaves with its subtree; archive names every owner; save → fresh load; load WITHOUT members derives the closure again |
| `🧪️tests/🧪️time-travel/🦀️.rs` (1) + `🧪️tests/🧪️time-travel-nested-member/🦀️.rs` (new, from `🧪️s5-nested-n3-law-time-travel-nested-member.rs`) | LAW `a_mutation_of_a_member_of_a_member_is_edited_in_history_under_its_owner_path`: begin by owner path → input → accept → replay → finalize → overwrite on a leaf that a member derives; the leaf head is the fresh fold, its owner and the document are untouched, the composed read follows |
Recorded cost: a follow pass visits every member (one bounded projection each) per member-lane publication — O(members), not
O(change); the subtree-scoped pass (follow only below the published member) is the follow-up if a census shows it.

**10:35 P1 doc-order follow-up LANDED** (apply-only hold, train line appended): the P1 landing had inserted the two fixture
helpers between `new_test_child`'s docstring and its signature; `🧪️s5-nested-p1-doc-order.py` moved the 16 lines above that
docstring (test-only, no behaviour, idempotent).

**10:35 the queued test run was stopped by me before any cargo started** (disk 14 GiB and falling; I killed my own waiting
`gate.sh` so nothing could start after the turn). No test of this WP has run today: every "passes" below is OWED.

### State at 10:36 and the resume order (for me on resume, or a successor)

| Wave | State | Next action |
|---|---|---|
| P1 fixture + N1 keys + `merge_persisted_envelope` arm | LANDED 10:06:19, train `FRAMEWORK GREEN 10:18:38` | run the OWED targeted test command (needs ≥ 25 GiB free) → "COMPOSED FIXTURE GREEN" with counts; a red in my files = fix forward |
| N2 layout (channel 23) | STAGED, `check` clean on the live tree 10:22 | on "WAVE C GO", after S5-STORE's §22.28 half: `acquire landing` → `python3 T/🧪️s5-nested-n2-wire.py check` → `land` → `release` → train line `… S5-NESTED N2 4 files restore: python3 T/🧪️s5-nested-n2-wire.py restore`; compiles only together with S5-CHANNEL's two frame fields (one train check after CHANNEL's write) |
| N3 recursion + laws | STAGED 10:18, `check` clean on N2's planned result, NOT compiled | after wave C is green: apply-only hold → `python3 T/🧪️s5-nested-n3-recursion.py check` → `land` → train line (6 files incl. 2 new law files; `restore` removes them) → then `zsh T/🚦️gate.sh 3 25 && RUST_MIN_STACK=268435456 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=3 cargo test -p semio-framework-plugin --lib --features artifact-app-testing --message-format=short -- a_member_composes_and_derives a_mutation_of_a_member_of_a_member` plus the N1 filter list (the roster and the fixture app gain a derivation) |
| P2 composed child history on today's tree | OWED, never run today (S5-AGNOSTIC's TSV: puzzle 2d/3d only) | one crate at a time through the single-flight runner: `zsh T/🧪️s5-agnostic-run-acceptance.sh semio-s-artifact-sequence-sequence --force` (then `semio-s-artifact-dag-dag`, `semio-s-artifact-reasoning-wires`, `semio-s-artifact-flow-flow`); column `child` of `🗑️generated/s5-agnostic/acceptance-results.tsv` is the verdict |
| Nested emits end to end (`ChildEmit.owner` non-empty), nested `LoadChildren` | refused by name in N2 | after N3: `child_emit_key` at the 7 dispatch sites listed in this report's N1 census, `ChildEmitPreparation` builder for an owner path, `load_child_pack(owner, …)` → `open_member` |
| §22.15 nested withdraw law | blocked on S5-STORE (`CompositionCoordinator::withdraw_group` absent) | write it beside the nested time-travel law once that lands |
| Playbook F2 (S5-TOOLS) | unblocked by N3 | roster entry `Form("s.forms.forms", …) => (FormsSnapshot, FormsMutation, <FormsApp as ArtifactApp>::genesis_child_pack)`; note the closure law: derived child ids are unique per document |

Peer files seen and left alone: `🔌️plugin/🪆️child/👁️capture/🦀️.rs`, `🔌️plugin/🪆️child/🧵️document/**` and
`🔌️plugin/🧪️tests/🔬️child-read-capture/🦀️.rs` (commit 670) are mounted by no `#[path]`/`include!`; the capture impl reads a
`reference` field `ChildContentEntry` never had. They do not compile into any target and N1 did not touch them; whoever mounts
them owes the owner-edge key (`ChildContentEntry { owner, slot, child_id, artifact_id, … }`).

### 11:03 resume — wave C on disk, the kernel test red, the notice row

**Wave C (channel 23) is on disk since 10:58:37** — S5-CHANNEL applied my N2 with the frame hunks; `FRAMEWORK GREEN 11:02:30`
through it. Verified at 11:09: `🧪️s5-nested-n2-wire.py check` → 4 × "already landed", `CHANNEL_VERSION = 23`,
`BackboneMessage::Member { owner, … }`, `ChildPackEntry.owner` / `ChildHeadPackEntry.owner` present; `🧪️s5-nested-n3-recursion.py
check` is clean on the LIVE tree (7 + 4 regions, 5, 3, 1).

**Kernel `--lib` test red `🏪️store/🧪️tests/🔬️unit/🦀️.rs:9712` `ChildDispatch::borrowed` (E0599) — NOT this WP's, not adaptable.**
Evidence: the law `dispatch_group_borrowed_child_keeps_exact_sources_on_policy_refusal` is uncommitted (`git log -S` → no
commit; `git diff HEAD` holds the line), its fixture `🏪️store/🧫️fixtures/🫳️child-dispatch/{🔣️.json,🧬️schema.json}` was created
at 10:26:12 and the test file last written 10:32:25 — no train line and no fleet report names either, and none of my scripts
touches that file (N1: one macro arm in `🏪️store/🦀️.rs`; N2: `BackboneMessage`, the sync test literal). On disk `ChildDispatch`
is the OWNED struct (`ops: Vec<Vec<u8>>`, `op_schema: SchemaId`, `labels: Vec<_>`), in HEAD and in the tree, with no `borrowed`
constructor, and `dispatch_group` takes `&mut [(&mut Mc, ChildDispatch)]`. The law asserts pointer identity of BORROWED sources
(`std::ptr::eq(dispatch.op_schema, &schema)`, `dispatch.ops.as_ptr() == cells`): its subject is an API that is not on disk
(the peer's borrowed child-dispatch wave, test + fixture ahead of the type). No one-line adaptation exists — it is not a
renamed call. Unblock = the peer lands `ChildDispatch<'a>`/`borrowed`, or its owner gates that one law out until then.

**Notice row LANDED 11:07:50 (landing + serve, 2 s hold, train line appended).** `🧪️s5-nested-notice-child-emission.py`:
the SDK fault `framework.child-emission.retirement-refusal` sat outside the notice schema's closed namespace pattern, so it
became `interactive-job.child-emission-retirement-refused` (the namespace of every sibling bounded child-ownership refusal) with
the row "A change to a part could not be released cleanly — try again." / "Eine Änderung an einem Teil konnte nicht sauber
freigegeben werden — erneut versuchen." in the kernel table (86 → 87), the fixture of record and the TS twin, same position;
plugin site + its law renamed (repo grep: 0 references to the old code).
RAN: carrier cross-check (python: fixture 87 = Rust 87 = declared 87 = TS 87, every code matches the schema pattern, unique);
`bun test ./🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️framework-notices/🟦️.ts` → **4 pass / 0 fail**. OWED: the Rust twin
`cargo test -p semio-framework --lib the_framework_notices_mirror` (kernel test build, after the activation).
Train: `FRAMEWORK GREEN 11:10:31 through: 11:07:50 S5-NESTED notice-child-emission` (kernel + plugin + wgpu + ui `--lib`).

**Owed test run, on S5-RUNTIME's prebuilt plugin test binary (no build of mine; path arrives through `main`):**
`RUST_MIN_STACK=268435456 <binary> --test-threads=4 a_child_survives an_agent_transaction_carries_owned_child composite_gesture_produces_one_undo_group a_composed_document_crosses a_composed_documents_child_heads member_factory_closed_dialect member_factory_parent_snapshot_restore a_composed_checkpoint_commit a_checkpoint_pins_its_children child_content_publication_path child_snapshot_retirement_rejection child_root_maintenance child_publications_at_the_maximum_rate maximum_child_public_dispatch the_child_content_view_never_goes_stale group_undo_skips_a_foreign_tail created_children_survive_absorb retained_child_group_publishes retained_window_input_recursive retained_composed_replacement a_whole_document_media_import fixed_child_member_registry stale_child_member_admission incomplete_child_member_registry owned_document_ingress member_run_ member_backfill_answers a_history_edit_is_its_own_row an_overwrite_head_equals_a_fresh_fold every_fixture_scenario_reaches owned_child_emission`
(the last filter covers the renamed emission refusal law; the binary must be built at or after 11:07:50 to hold that rename —
an older one fails only the one assertion on the code string).
N3 stays staged until "N3 GO" (after B2 is live); the nested time-travel law now waits on `status().is_none()` like the harness.

### 16:46 resume — build B2 live (N1 + N2, channel 23); the shared law run and my three reds

Shared run on S5-RUNTIME's binary (`📓️s5-plugin-law-run.md`, 16:38): my list **39 passed / 8 failed**. Five `member_run_*`
belong to the tool-run family (four non-member `tool_run_*` laws fail identically: `has_pending_work()` never clears) →
S5-GRAPHS-WIRES. N1's four tool-run sites are not the cause: each runs only for `entry.member = Some(_)` and resolves, for a
root member, the same registry entry the old `(slot, child_id)` scan did.

My three, all TEST-side (landed 16:54:33, `🧪️s5-nested-composed-laws.py`, train line, invisible to `--lib`):
1. `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` — the law loaded the parent pack alone and the
   children afterwards (`load_document` + `load_child_pack`). Since commit 670 every candidate parent passes the ownership
   closure, so a parent that declares a member never loads without it (`closure-rejected`, Incomplete). That is the product
   rule, not a B2 fault: NO host sends `LoadChildren` / `ReadChildren` (repo grep over `🧰️framework`, `🌎️hub`, `✏️s/🧑‍💻dev`:
   codec arms and TS types only); `🏃️run`, MCP and the shells persist and reload through `ReadDocumentArchive` /
   `LoadDocumentArchive`, and `a_composed_document_crosses_a_media_edge_with_its_children` (archive route) passed in the same
   run. The law now persists the archive, reloads it into a fresh app (child at its history's value, owned by the reloaded
   document) and pins the refusal of the same archive without its members.
   Follow-up for S5-CHANNEL / S5-LOAD (needs a bump, not done): frames 14/15 `LoadChildren` / `ReadChildren` and
   `PluginApp::load_child_pack` / `child_packs` have no sender and can no longer restore a declared member — candidates for
   deletion; `ReadChildHeads` (42) is live (MCP inference dependencies).
2. `created_children_survive_absorb_into_the_child_store_map` — absorbed `genesis-child` in `genesisSlot`, which the parent
   neither declares nor has; `absorb_created_children` refuses (`created child is not declared by the published parent`,
   commit 670) and the `?` dropped the live member → store Drop witness panic. The law declares the child in the parent's real
   slot first. RESIDUE (recorded, not fixed): `absorb_created_children` still drops the refused live member on its error path;
   unreachable in products today (no code constructs a `ChildGenesis`: repo grep), it needs the abort-retirement route of
   `open_member` before `ChildGenesis` gets a producer.
3. `retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires` — after the group undo the PARENT still
   reads 9 (the assertion that fails is the parent's, line 2032). Re-run alone on the 16:48 binary with
   `SEMIO_RUNTIME_DIAGNOSTICS=1`: the undo reaches `commit_framework_history_route` (`[TRACE] history route action=undo`) and
   no chrome route. Not explained by N1 (the group route's lookups are by reference; the same route passes in
   `group_undo_skips_a_foreign_tail…` and `the_child_content_view_never_goes_stale…` on the non-runtime fixture). The law now
   asserts that the undo skipped no member and prints the skip diagnostics and the parent's tail group, so the rebuild names
   the cause.

One plugin test rebuild (gate v6 `2 18`, `CARGO_BUILD_JOBS=4`, shared dirs, `--no-run`) started 16:55 →
`🗑️generated/s5-nested/build-composed-1.txt`; results below.

**Rebuild + run (17:10).** `zsh T/🚦️gate.sh 2 18 480 && RUST_MIN_STACK=268435456 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 cargo test -p
semio-framework-plugin --lib --features artifact-app-testing --no-run --message-format=short` → exit 0 (gate open 16:55:25,
binary 17:09, same path `…/semio-framework-plugin/b80951f2dbb7024a/out/semio_framework_plugin-b80951f2dbb7024a`). My list on
it (`🗑️generated/s5-nested/test-composed-2.txt`, `--test-threads=4`): **44 passed / 3 failed** (was 39 / 8).
- GREEN now: `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` (archive route + the refusal of the
  halved archive), `created_children_survive_absorb_into_the_child_store_map`, and every other composed builder-contract law
  of P1, the registry laws, the recursive archive / replacement laws, S5-LOAD's two composed load laws, 3 of 5 `member_run_*`.
- RED 1 — `retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires`, cause named by the law itself now:
  `the group undo skips no member of its own gesture: ["undo skipped member s.test.keyed@1/*#editor (nothing to undo)"]`.
  The law acts as `fixture`. `🏪️store/🦀️.rs:22182` (`replay_mutations`, every plain / group `Apply`) authors an operation as
  `mutation.author_id()` else the LITERAL `"local"`, never the store's acting actor; the undo route binds the acting actor
  (`fixture`) and `undo` takes the newest edit that `edit_is_local` — none is. Same cause as S5-RUNTIME's expected red
  `every_route_authors_as_its_acting_actor`; design §22.34 (b) (S5-STORE, "own wave after B2") is the fix. Not N1 / N2 (the
  group route resolves members by reference; the same route is green for every law that acts as `local`). The law is right as
  written and turns green with §22.34. PRODUCT reading: until §22.34 an instance opened as anyone but `local` cannot undo its
  own gesture — composed or not.
- RED 2, 3 — tool-run family (S5-GRAPHS-WIRES): `member_run_finalize_is_one_member_edit…` (row label `"Set count to 1 (+9)"`
  instead of the declared `"Toy member fill"`), `member_run_holds_at_most_the_member_ceiling…` (abort never settles; its own
  probe names `child-content admission refusal Some("interactive-job.child-root-retirement-saturated")`). N1 changed how a
  retired child root FINDS its disposer (owner edge, a retiring member by artifact id), not when a root retires; if GRAPHS-WIRES
  suspects the ring, the A/B is `🗑️generated/s5-nested/pre-n1/` vs the tree on `ChildContentRetirement::close_step`.
