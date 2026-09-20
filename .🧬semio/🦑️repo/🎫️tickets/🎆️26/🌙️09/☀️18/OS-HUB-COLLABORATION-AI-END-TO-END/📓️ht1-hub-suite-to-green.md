# HT1 — the hub suite to green (`semio-hub`, 318 → 321 tests)

Slice: outcome 2's gate. Entry state, coordinator rerun **12:06 with full failure output**
(`🗑️generated/coordinator-hub-nextest-full.txt`): **321 tests run: 263 passed / 58 failed / 0 skipped**
(31.3 s). The failing SET is byte-identical to the 09:17 run (`coordinator-hub-nextest-latest.txt`) —
diffed both ways, `0` laws went green and `0` newly red — so H1b's §27 cluster-B fix moved its family to a
*different* disjunct rather than closing it (§3 below), and the +3 tests are peers' new laws, all green.

Rule 26 binds: this slice never ran `cargo test`/`nextest`/`build` on `-p semio-hub`. Everything below is
either measured from the coordinator's capture or proven by `cargo check -p semio-hub --all-targets` with
rule 25's private `CARGO_TARGET_DIR` (§6). Captures: `🗑️generated/ht1-*.txt`.

## 1. Headline — the largest cluster was mis-attributed, and it is not a product defect

H1b §26 attributed **cluster A (19 `artifact_authority::trusted_catalog` laws)** to a GIS-plugin peer
regression, from the ONE message the 09:17 capture held
(`Catalog("trusted document-open target is invalid, unbound, or duplicated")`), and routed it to DS1.

With all 58 panics captured, that is **wrong**. Those 19 laws never reach the catalog loader. They panic in
their fixture setup, on a missing environment variable:

```
🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs:398   ticket-owned catalog fixture root          × 14
🔏️trusted-catalog/🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs:20    ticket-owned artifact root                 ×  5
🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs:509   ticket-owned exact-law artifact directory  ×  2
📇️directory/🧪️tests/🔬️unit/🦀️.rs:{896,1176}                       ticket generated artifact root             ×  2
📇️directory/🧪️tests/🔬️unit/🦀️.rs:1476                             ticket artifacts: NotPresent               ×  1
📇️directory/🪶️sqlite/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs:183 ticket generated artifact root             ×  1
```

**25 of the 58 reds are `std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect(…)` failing.** The nx verb
`os-hub:test` exports that variable (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `hubTestArtifactRoot` →
`🌎️hub/📦️packages/🦀️rust/🗑️generated/test-artifacts`, and its own docstring says the laws "panic without
it"). The coordinator's `cargo nextest run -p semio-hub` exports nothing, so a quarter of the suite fails
in setup and reports as a product regression. Only **3** laws in the entire suite actually carry the
`trusted document-open target is invalid, unbound, or duplicated` message, and all three live in
`🔬️bin-unit` (§5), not in cluster A.

**Root fix landed** — a suite that only one runner can execute is the defect. New shared helper
`🌎️hub/🧪️tests/🗂️artifact-root/🦀️.rs`:

```rust
pub fn test_artifact_root() -> PathBuf {
    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR")
        .map_or_else(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/🗑️generated/test-artifacts")), PathBuf::from);
    std::fs::create_dir_all(&root).expect("hub test artifact root");
    root
}
```

The default is the *same* directory the verb computes, derived from the crate manifest, so `nx run
os-hub:test`, `cargo test`, `cargo nextest` and an IDE run all observe one law. Declared `#[cfg(test)]` in
both crate roots (`📦️packages/🦀️rust/🦀️.rs` for the lib, `🏗️bootstrap/🦀️.rs` for the bin) via `#[path]`,
so nothing is added to any production surface. All **11** Rust sites now call it, including the four that
had silently diverged onto `std::env::temp_dir()`.

## 2. The 58, re-clustered by panic message

Machine-generated from the full capture into `🗑️generated/ht1-red-clusters.txt` (every law, its panic site
and its message). Summary, with the owner after this slice's work:

| # | panic | laws | verdict | state |
|---|---|---|---|---|
| 1 | `SEMIO_TEST_ARTIFACT_DIR` absent (5 distinct `expect` strings) | **25** | runner gap, not a defect | **fixed** §1 |
| 2 | `claim publication genesis: Conflict("artifact creation acceptance is outside its live server deadline")` | 4 | stale fixture (epoch clock) | **fixed** §3 |
| 3 | `publish verified checkpoint: Conflict("ordinary artifact publication requires a committed genesis parent")` | 3 | stale fixture (R3) | **fixed** §4 |
| 4 | `socket test: Any { .. }` — the `run_socket_test` join, 7 inner panics behind it | 7 | 3 × open-plan 404, 4 × distinct | **2 fixed** §4, 5 open |
| 5 | `Catalog("trusted document-open target is invalid, unbound, or duplicated")` | 3 | trusted-catalog fixture | open — §5 |
| 6 | `inference::{wal,runtime,sqlite}` — 7 distinct assertions | 7 | H1b R1/R6 | open |
| 7 | `missing or duplicate test declaration: scoped_directory_socket_removal…` | 1 | oracle blind to level tiers | **fixed** §4 |
| 8 | `document_open_plan_{exchange,admin_revocation}` | 2 | cascade of #2 | expect green |
| 9 | `directory_event_page_v1_*`, `presence_normalization_matches_neutral…`, `invite_archive…`, two `Elapsed(())` admin deadlines, `gis_map_abandoned…` | 6 | distinct, unread | open |

The corrected ownership line: **no red is M6's, P4's or OB1r's** — that survives the full capture, and is
now a positive result rather than an inference from an incomplete one. **Cluster A is not DS1's** either;
DS1's lane (`📓️ds1-stdio-descriptor-bound.md` §10) is the *live* trusted catalog for a booted hub — it has
no verdict on these laws and never claimed one.

## 3. Cluster B — H1b's fix was right and landed on the next disjunct

`publish_genesis_checkpoint_for_test` accepted its creation intent at `accepted_at_ms = 1` (epoch), with
`published_at_ms: 1`. Every backend's `claim_artifact_creation` gates on the **live** clock
(`📇️directory/🪶️sqlite/🦀️.rs:1093`, and the same line in the postgres/neo4j drivers):

```rust
if intent.accepted_at_ms > observed_now || observed_now >= intent.deadline_ms { … }
```

`deadline_ms = accepted_at_ms + ARTIFACT_CREATION_DEADLINE_MS` is pinned by
`ArtifactCreationIntentV1::validate`, so an epoch-1 acceptance is *always* expired. The product code is
right — a creation claim outside its live window must be refused. **Stale test**: the fixture now accepts on
`now_ms()` and publishes at the same instant (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`, `publish_genesis_checkpoint_for_test`).
`ArtifactCreationPreparedV1::validate` (`🌱️creation/🧬️schema/🦀️.rs:317-318`) additionally requires
`accepted_at_ms <= published_at_ms < deadline_ms`, which that satisfies by construction.

## 4. Cluster C / R3 — the scoped fixture redesign

`publish_checkpoint_for_test` published an **ordinary** checkpoint with `parent_checkpoint_id: None` into a
document that only had an `AnnounceDocument`. `decide_verified_checkpoint`
(`📇️directory/🦀️.rs:2196`) refuses that: a document's first checkpoint is a creation genesis and only
`publish_document_genesis` mints one. Session 4 §13.1 scoped the redesign but could not build it, because
the helper had no session token.

It needed no new seam — the bin suite already mints real sessions (`issue_test_session` →
`state.directory.issue_auth_session`, the same authority AU1/AU3 drive). **No test-only back door was added
to product code.** Landed in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`:

- `seed_genesis_for_document_for_test(state, token, space_id, document_id, label)` — authenticates a real
  session capability, builds `ArtifactCreationActorV1` from it, and publishes the genesis through
  `publish_genesis_checkpoint_for_test`. The document id must be creation-owned (`artifact-<32 hex>`)
  because `ArtifactCreationIntentV1`'s request id *is* that suffix and
  `SpaceArtifactCreationStatusV1::validate` bounds it to 32 lowercase hex.
- `seed_genesis_document_for_test(state, author_email, space_id, label)` — the same, for a law with no
  session yet: mints one, upserts that user as a space **Author** (`creation_authority`,
  `📇️directory/🪶️sqlite/🦀️.rs:604`, requires exactly that), mints the id, delegates.
- `publish_checkpoint_for_test` now **chains**: it reads `get_active_artifact_checkpoint`, sets
  `parent_checkpoint_id: Some(head)` and advances `head_edit_ordinal`/`last_commit_seq` by one so
  `frontier_strictly_advances` (`📇️directory/🦀️.rs:1698`) holds. Its docstring records why a parentless
  ordinary publication cannot exist.

Six callers moved off `announce_document_for_test` onto the seed (the two cannot be combined —
`append_document_genesis` refuses a scope whose descriptor is already publicly occupied): the two
`canonical_pair_route_*` laws (4 documents across 3 spaces) and
`artifact_cas_maintenance_checkpoint_reaches_tail_after_sixteen_requests` (5 spaces).

Also in this section, two more:

- **3 of the 7 `socket test: Any { .. }` are `issue document open plan: 404 Not Found NotFound`** — the same
  root cause seen through the open-plan route, which correctly refuses a document with no committed
  checkpoint (the law `document_open_and_execution_target_refuse_descriptor_or_index_without_genesis`
  asserts exactly that and passes). Two of the three seed a genesis now
  (`presence_lease_reconnect_rejects_old_live_refresh_and_close`,
  `presence_normalization_socket_overwrites_identity_and_rejects_without_refresh`). The third,
  `admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen`, pins its `documentId`
  in a committed fixture (`🧫️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json` →
  `"admin-presence-recovery"`), which is not creation-owned; closing it means changing that fixture and its
  expectations, left open and named rather than guessed.
- **`socket_grant_oracle::quick::hub_socket_grant_fixture_serde_parity`** — a genuine oracle defect, not a
  fixture drift. `test_declarations` walked only `syn::File::items`, so when H1b's level tiering moved
  `scoped_directory_socket_removal_and_delivery_have_one_total_membership_order` into `mod quick`, the stage
  contract could no longer find its declaration. It now recurses into inline modules
  (`📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🔮️oracles/🦀️.rs`); the file's own hostile-input
  assertions are unchanged and still hold.

## 5. `Catalog("trusted document-open target is invalid, unbound, or duplicated")` — 3 laws, left to DS1

Only three laws carry it, all in `🔬️bin-unit`:
`checkpoint_publication_route_is_author_owned_actor_fenced_idempotent_and_cancellation_safe`,
`checkpoint_publication_route_rejects_stale_or_cross_scope_inputs_before_publication`
(`load stdio publication catalog: …`) and
`native_openable_stdio_provider_is_the_only_atomic_readiness_transition` (`verified stdio authority: …`).
They load a real stdio publication catalog through `NativeCodecProviderSetV1::linked()`, which is the seam
H1b named and the one DS1's descriptor work moves. Untouched here by instruction; the correction to record
is that this is **3 laws, not 19**.

## 6. OB1r's open thread — why two laws hang in-process and pass under nextest

`an_authenticated_session_read_reports_one_span_with_its_principal` and
`the_observability_route_answers_an_admin_with_the_events_its_own_requests_produced` never answer under
`cargo test` but pass under nextest and on a live hub. OB1r bounded them with a 20 s timeout
(`bounded_http_request`, `🔬️bin-unit/🦀️.rs:486`) — a band-aid, and the instruction was to find the cause.

**Found, and it is structural.** `hub_worker_pool()` (`🌎️hub/🏗️bootstrap/🦀️.rs`) returned
`semio_framework_async::process_worker_pool(config)`, which is a `OnceLock` singleton
(`🧰️framework/🔨️modules/⏳️async/🦀️.rs:2456-2459`): **one pool per process**, sized to the machine's core
count, and every `db::Database` in the process is constructed on it
(`db::Database::open_at(hub_worker_pool(), …)`). That contract is right for a hub — a hub is one process.
The bin test binary is not one hub: `cargo test` hosts ~180 independent `HubState`s in a single process, so
they all shared ten workers and queued each other's blocking storage work behind laws that hold a worker
for their whole body. Nextest gives one process per law, so the contention cannot arise there; a live hub
is a single hub, so it cannot arise there either. That is the exact shape of "hangs in-process, passes
under nextest and live".

**Fixed at the root, not at the timeout**: under `#[cfg(test)]` `hub_worker_pool()` now returns a fresh
`WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2))` per hub, restoring the isolation
nextest gets for free; production keeps the process singleton unchanged. The change is a near no-op under
nextest (one or two pools per process either way), so it cannot move the coordinator's numbers in either
direction — which is also why it is safe to land without being able to run the suite. The 20 s bound is
left in place as an assertion (a law that stops answering should fail, not hang); it is no longer load-bearing.

## 7. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht1 cargo check -p semio-hub --all-targets` (after §1, §3, §4) | **0 errors**, 305 warnings (all peers'), 41.6 s | `ht1-check-1.txt` |
| same, after §4's oracle fix and §6's pool fix | **0 errors**, 61 warnings replayed for `bin "os-hub" test`, `Checking semio-hub` present | `ht1-check-2.txt` |

Honest limit: **nothing in this report was executed as a test.** Rule 26 gives hub test/bin builds to the
coordinator. Every claim above is either read off the coordinator's capture or compile-proven. The
red-count-after column of §2 is a prediction until the rerun lands, and is written as such.

## 8. Files changed

| file | change |
|---|---|
| `🌎️hub/🧪️tests/🗂️artifact-root/🦀️.rs` | **new** — `test_artifact_root()`, the runner-independent fixture root |
| `🌎️hub/📦️packages/🦀️rust/🦀️.rs` | `#[cfg(test)] pub(crate) mod test_artifact_root` |
| `🌎️hub/🏗️bootstrap/🦀️.rs` | `#[cfg(test)] mod test_artifact_root`; `hub_worker_pool()` per-hub pool under `cfg(test)` + its docstring |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs` | 2 fixture roots |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs` | 1 fixture root |
| `🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs` | 3 fixture roots |
| `🌎️hub/📇️directory/🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs` | 1 fixture root (was `temp_dir`) |
| `🌎️hub/📇️directory/🪶️sqlite/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs` | 1 fixture root |
| `🌎️hub/🧪️tests/🔏️trusted-catalog-profile/🦀️.rs` | 1 fixture root (was `temp_dir`) |
| `🌎️hub/📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🔮️oracles/🦀️.rs` | `test_declarations` recurses into inline level-tier modules |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` | 2 fixture roots; genesis fixture on the live clock; `publish_checkpoint_for_test` chains on the active head; `seed_genesis_document_for_test` + `seed_genesis_for_document_for_test`; 8 caller sites reseeded |

Ticket artefacts: `🗑️generated/ht1-red-clusters.txt` (all 58 reds by panic), `ht1-check-1.txt`, `ht1-check-2.txt`.

## 9. Open after this batch, in the order worth taking

1. `inference::{wal ×4, runtime ×2, sqlite ×1}` — 7 laws, 7 distinct assertions (H1b R1/R6; R6's discarded
   assembly terminal, §13.3, is still the prerequisite).
2. The 4 remaining socket inner panics: `removal-wins sender deadline: Elapsed(())`, `no revocation close
   before 5s deadline`, and two assertions — load-shaped on their face, unproven.
3. `admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` — needs the committed
   fixture's `documentId` made creation-owned (§4).
4. The 6 singles of §2 row 9, each needing its own read.
5. The 3 trusted-catalog laws of §5 — DS1's seam.

**needs hub rerun** — with `--status-level all --failure-output immediate` again, so the next batch can be
clustered the same way.

---

# HT1 — batch 2 (coordinator rerun 13:1x)

**321 tests — 290 passed / 31 failed** (was 263/58). Batch 1 closed **27** reds. Every prediction of §2
held: the 25 fixture-root laws, the 4 deadline laws, the 3 genesis-parent laws, the oracle law and the two
`document_open_plan_*` cascades are all PASS; nothing regressed. Re-clustered into
`🗑️generated/ht1-red-clusters-2.txt` (32 panics for 31 laws — `retained_short_admin_request…` panics twice,
in its HTTP task and in the join that observes it).

## 10. M6 / M6b, as asked

From `coordinator-hub-nextest-latest.txt`: **M6's 11 `auth::agent::tests::*` laws are all PASS**
(ids 52–63) and **2 bin integration laws are PASS** —
`an_agent_delegation_mints_a_session_that_works_until_it_is_revoked` (214/321) and
`an_agent_can_never_widen_its_own_audience_or_probe_another_humans_delegation` (211/321). Searching the
whole verdict list for `agent|delegation` returns exactly those 13 rows, so **M6b's third integration law
is not in this binary** — either not landed at the 13:1x build, or named without `agent`/`delegation`.
M6's laws were not "missing" earlier: the 09:17 capture was a `tail -400` excerpt.

## 11. Batch 2 — what was fixed

**1. My own regression, caught by the rerun: the genesis commit clock (7 laws).**
`publish_genesis_checkpoint_for_test` passed `accepted_at_ms` as the `now_ms` of
`publish_document_genesis`. With the fixture on the live clock (§3) the backend had already stamped the
**Prepared** fact at its own `observed_now` (`decide_artifact_creation_fact_append_v1`,
`🌱️creation/🧬️schema/🦀️.rs:211` — the backend stamps, the caller does not), which is strictly later, so the
Committed fact went in *behind* it and `ArtifactCreationOperationV1::fold` refused the history at
`fact.recorded_at_ms < timestamp` → `Conflict("artifact creation fact identity or order differs")`. The
commit now takes a fresh `now_ms()`. This is exactly the defect the epoch clock had been hiding.

**2. One request id per (user, space) — `canonical_pair_route_is_exact_member_or_share…`.**
`claim_artifact_creation` keys creation facts on `(actor_user_id, request_id)` **without the space**
(`📇️directory/🪶️sqlite/🦀️.rs:1098`), and the request id *is* the document id's `artifact-` suffix. The law
deliberately publishes the same document id into three spaces, so one author seeding all three collided:
`Conflict("artifact creation request is already bound to another intent")`. The product rule is right — a
request id is an author's, not a space's. The law now seeds each space with its own author, keeping the
document id identical, which is what it is actually testing.

**3. `document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use`.** Announce-only,
so `document_open_plan_authority_for_session` found no active checkpoint (`committed document checkpoint`).
Both of its documents are now genesis-seeded through `seed_genesis_for_document_for_test`.

**4. The trusted-catalog refusal no longer hides which predicate fired (product, observability).**
`trusted document-open target is invalid, unbound, or duplicated` is ONE message behind a ten-term
disjunction (`🔏️trusted-catalog/🦀️.rs:1016-1026`) and is the only thing the three remaining stdio laws say.
H1b read it as a GIS/descriptor regression, DS1 read it as not-descriptor-caused, and static reading of the
fixture cannot separate the terms: the synthetic bundle looks self-consistent — the open target takes its
kind/schema/pack hash from the same `stdio.native.json.v1` receipt that is in `nativeCodecs`, its parent
dialect's kind is the same `"s.stdio.json"` the viewer asserts, and its grant matches its viewer role. So
the disjunction is now six discriminated refusals, each a fixed static string (the
`…have_bounded_diagnostics` law still holds — bounded, not uninformative): empty/padded identity, parent
dialect kind, role-fixed grant, zero pack hash, **bound to no native codec of its own package**, duplicated
key. No caller asserted the old text (grepped repo-wide, 0 hits). The next rerun names the cause instead of
the family, and these 3 close in one hop.

## 12. Still open after batch 2, with what is known

| law(s) | panic | read |
|---|---|---|
| `inference::wal` ×4, `runtime` ×3, `sqlite` ×1 | 7 distinct assertions (`retained WAL`, `close_steps() > 0`, `"absent" vs "verified"`, `Busy { retained_uses: 1 }`, `Some(Denied)`, `Elapsed`) | H1b R1/R6; R6's discarded assembly terminal (§13.3) is the prerequisite for the runtime pair |
| `space_public_boundary…` | `left: 400, right: 202` on `POST /directory/commands` `RemoveMember` | unread |
| one socket law | `left: 500, right: 200` on `GET /directory/spaces/{public}` — a **500** on a public detail route is a product defect on its face | unread; hypothesis worth testing first: `create_space_for_test` mints spaces owned by a user id that need not exist (its own docstring), and a detail route that now resolves the owner would 500 on the dangling id |
| `scoped_directory_socket_route_rejects_scope_substitution…` | `no revocation close before 5s deadline` | unread |
| `scoped_directory_socket_removal_and_delivery…` | `removal-wins sender deadline: Elapsed(())` | load-shaped, unproven |
| `admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` | `issue document open plan: 404` | known: needs a genesis, but its `documentId` is pinned by `🧫️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json` to `"admin-presence-recovery"`, which is not creation-owned — closing it means changing that fixture and its `expected` block |
| `retained_short_admin_request…`, `socket_admin_user_gate…` | two `Elapsed(())` deadlines | load-shaped, unproven |
| `directory_event_page_v1_*` ×2 | `left: 5, right: 3` PostgreSQL admit seams; `control.cancelled` never set | unread |
| `document_open_plan_admin_revocation…` | `"cancelled"` vs `"succeeded"` | unread |
| `presence_normalization_matches_neutral_authority…` | frozen-wire byte vector mismatch | unread |
| 3 stdio catalog laws | see §11.4 — discriminated now, cause named on the next run | |

## 13. Verification (batch 2)

`CARGO_TARGET_DIR=…/target-ht1 cargo check -p semio-hub --all-targets` → **0 errors**, `Checking semio-hub`
present, 3 m 55 s (`ht1-check-3.txt`). Nothing in this batch was executed as a test — rule 26.

Files changed in batch 2: `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` (genesis commit clock; per-space authors in
`canonical_pair_route_is_exact_member_or_share…`; genesis seed in
`document_open_plan_exchange_route…`), `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`
(six discriminated open-target refusals).

**needs hub rerun** — same flags.

---

# HT1 — batch 3 (coordinator rerun 13:32)

**321 tests — 299 passed / 22 failed** (58 → 31 → 22). Batch 2 closed 9, nothing regressed. Re-clustered into
`🗑️generated/ht1-red-clusters-3.txt` (24 panics for 22 laws).

## 14. The discriminated refusal paid off in one hop — and named a real outcome-2 defect

All three stdio laws now say **`trusted document-open target parent dialect names a different artifact kind`**,
so the failing term is `🔏️trusted-catalog/🦀️.rs`'s `target.parent_dialect.artifact_kind != target.artifact_kind`.

**The validator is wrong, and it is the hub's own defect, not stdio's and not a fixture drift.** An open
target carries two artifact-kind ids from two spaces the product keeps deliberately distinct, and
`validate_descriptor_open_target` (same file) pins each to a different one:

- `target.artifact_kind` → a manifest `ArtifactKindSpec::id` (`discoverable` requires a manifest kind with
  exactly that id and schema). For stdio that is **`stdio.json`** — `ArtifactKindSpec { id: "stdio.json", … }`
  at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🦀️.rs:58`, and `artifact_kind: "stdio.<x>"` for **every** artifact
  in the committed projection `✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json`.
- `target.parent_dialect` → the owning app's `Dialect` (`app.dialect != target.parent_dialect` ⇒ refuse).
  For stdio that is **`s.stdio.json`** — every `Viewer::builder`/`Editor::builder` dialect in the plugin is
  `s.stdio.<x>` (`BCF_ANY_DIALECT`, `MP4_DIALECT`, `STL_ANY_DIALECT`, …).

So the disjunct demanded `"s.stdio.json" == "stdio.json"` and **no real stdio bundle could ever load**. That is
the live chain DS1 measured from the other end: no trusted catalog ⇒ `artifactAuthority` never ready ⇒
`/readyz` 503 ⇒ no open plan, no MCP workspace, no inference. The hub's own synthetic fixtures never caught it
because they use one spelling for both fields (`s.fixture.document`, `s.gis.gismap`).

**Fixed**: the cross-space comparison is deleted, with a comment recording both spaces and both spellings.
No authority is lost — the target is still bound to a native codec of its own package by
(kind, schema, pack hash) here, and to the descriptor's own app and manifest artifact kind in
`validate_descriptor_open_target`. No law asserted that refusal (the `for field in ["artifactKind", …]` loop in
the catalog unit tests only asserts the *generation digest* changes, which is unaffected).

## 15. M6b's third law — nothing to mount

`only_an_author_of_the_space_can_delegate_to_an_agent` is present, correctly attributed
(`#[tokio::test]`, `🔬️bin-unit/🦀️.rs:6203`, in the same `mod tests` as its two siblings) and **PASSES** in the
13:32 run (269/321). The `agent|delegation` rows went 13 → 14. It was simply landed after the 13:1x binary was
built; there was no mounting defect. M6/M6b are **14/14 green**.

## 16. The presence frozen wire — measured, and it is M6's wire change to finish

`presence_normalization_matches_neutral_authority_and_no_effect_rejections` compares the hub's normalized
peer bytes against `🌎️hub/🧫️fixtures/🪪️presence-normalization-v1/🔣️.json`. Decoding both sides of the failure:

```
expected (fixture) …  0c "server-actor" ff 07 e8 07 03 "Ada" …
actual   (hub)     …  0c "server-actor" ff 17 e8 07 03 "Ada" …
                              ^^ field bitmap 0x07 → 0x17
```

One extra field is present on the wire — M6's `PresencePrincipalKind`, stamped server-side. The fixture is
**pre-M6**: `git log` gives `2026-09-14 01:26:52`, six days before M6 landed. The coordinator's hypothesis is
confirmed exactly.

**Deliberately not "fixed" by copying the hub's output into the expectation.** This fixture is the *neutral
authority* named in the law: `provePresenceNormalizationFixture` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:15295`)
re-derives it independently with AJV and a third-party LEB128 decoder. Rewriting the 19 vectors from the bytes
the hub currently emits would delete the only independence the law has — it would assert that the hub agrees
with itself. Closing it correctly means teaching the neutral decoder the `principalKind` field and re-deriving
all 19 vectors, which is the second half of M6's wire change (M6 regenerated the replication twin's 31 shared
vectors and this hub fixture was missed). Owner: M6/M6b, with the exact delta above.

## 17. `500` on `GET /directory/spaces/{public}` — narrowed, needs one runtime bisect

`space_public_boundary_real_routes_emit_discriminated_public_member_author_and_private_404` gets `500` where
it asserts `200`, on the **anonymous public** projection only (every authenticated
`space_administration_page_v1_route_*` law is green). Narrowed by reading
`build_directory_space_administration_page_v1` (`🏗️bootstrap/🦀️.rs:6261`): the only two `500`s reachable on
that path are the `u64::try_from(document_offset + documents)` cursor conversion (unreachable here — one
document, no `document_more`) and `page.validate().map_err(|_| INTERNAL_SERVER_ERROR)`. So it is
`DirectorySpaceAdministrationPageV1::validate` returning `Invalid` or `ReceiptMismatch` for the `Public`
variant, and the route throws the discriminant away — the same anti-pattern that hid §14 for three sessions.
I did not guess a fix: the remaining candidates (`anonymous = generation == 0 && binding all-zero`,
`valid_document_open_hash` on the all-zero binding, `receipt_matches` over the `Public` arm) are one run apart
and zero reads apart. **Cheapest next step for whoever has a hub test lane: run that one law with the
`DirectorySpaceAdministrationPageErrorV1` variant printed** — it names the cause immediately.

## 18. Remaining 22, and who they belong to

| laws | state |
|---|---|
| 3 stdio catalog | **fixed** §14 — expect green |
| `inference::wal` ×4, `runtime` ×3, `sqlite` ×1 (+ a `semio-pool-worker-1` panic, `🏪️store/🦀️.rs:18399` `artifact store reached Drop without its exact terminal-empty shallow-shell witness`, which is the store-drop witness behind `gis_map_terminal_close…`'s `Busy { retained_uses: 1 }`) | open — H1b R1/R6; the drop-witness panic is the memory playbook's known bucket |
| presence frozen wire ×1 | **measured, owner named** §16 |
| `space_public_boundary…` 500 ×1 | **narrowed to one assertion** §17 |
| `admin_removal…` (404, fixture-pinned `documentId`) | open — needs `🧫️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json` made creation-owned |
| `scoped_directory_socket_route_rejects_scope_substitution…` (`no revocation close before 5s`), `scoped_directory_socket_removal_and_delivery…` (`removal-wins sender deadline`), `socket_admin_user_gate…`, `retained_short_admin_request…` (two `Elapsed(())`) | open — load-shaped, unproven |
| `directory_event_page_v1_*` ×2, `document_open_plan_admin_revocation…` ×1, `RemoveMember 400≠202` ×1 | open, unread |

## 19. Verification (batch 3)

`CARGO_TARGET_DIR=…/target-ht1 cargo check -p semio-hub --all-targets` → **0 errors**, `Checking semio-hub`
present, 1 m 02 s (`ht1-check-4.txt`). Rule 26: nothing executed as a test.

Files changed in batch 3: `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` only (the cross-space
artifact-kind disjunct removed, both id spaces documented at the loop).

**needs hub rerun** — same flags. Worth watching beyond the count: whether `artifactAuthority` now reaches
ready on a booted hub (§14 is DS1's `/readyz` blocker), which is outcome 2's gate, not just a test.

---

# HT1 — batch 4 (coordinator rerun 13:46)

**321 — 300 passed / 21 failed.** Only +1, but the +1 is the one that mattered:
`native_openable_stdio_provider_is_the_only_atomic_readiness_transition` **PASSES** — §14's fix loads a real
stdio catalog for the first time. Clusters: `🗑️generated/ht1-red-clusters-4.txt`.

## 20. The same category error, one layer up (the 2 remaining catalog laws)

Both `checkpoint_publication_route_*` moved to a NEXT term at the creation boundary:
`exact prepared publication genesis: Conflict("artifact creation accepted identity is invalid")` —
H1b session 4's R4 item 2, "one of the eleven disjuncts, this session did not isolate which". It is
`self.ready().validate()`, and inside it, at
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs:187`:

```rust
&& self.parent_dialect.artifact_kind == self.kind_id
```

**Character-for-character the same defect as §14, one layer up.** `kind_id` (with `artifact_schema`) names a
manifest `ArtifactKindSpec` — `stdio.json`; `parent_dialect` is the owning app's `Dialect` — `s.stdio.json`.
`VerifiedDocumentOpenSelectionV1` carries them as two separate fields for exactly that reason. So creation
against any real plugin catalog was refused, while the hub's synthetic fixtures (`s.fixture.document`,
`s.gis.gismap`) spell both fields the same and never noticed. Replaced by an `identity()` bound on the
dialect kind, with the two spaces documented. Nothing else in the repo asserts that equality (grepped).

## 21. The presence frozen wire — closed through its own producer, and VERIFIED

Done the way the coordinator directed: the fixture was re-derived from the **wire's own stated rule**, not
from hub output. `📡️replication/📡️wire/🦀️.rs:1643` says it exactly — "Bit 11 carries `principal_kind` as one
declaration-order tag byte (`0` human, `1` agent)", appended after every other optional field. So the
transform of a pre-agent normalized peer is mechanical: set bit 11 in the LEB128 flag word after the actor
text, append the human tag. `🐍️ht1-presence-principal-kind.py` applies it — **10 of 19 vectors** carry a
normalized peer, 9 are rejections; the diff is 10 lines and reformats nothing.

Then the neutral oracle was taught the field so it still re-derives the answer **independently**
(`📜️script.ts`): `independentEncode`'s field list gains `peer.principalKind` at index 11 with its tag-byte
branch, the reconstructed `output` carries `admitted.principalKind ?? "human"`, `PRESENCE_PRINCIPAL_KINDS`
is imported from the replication twin M6 landed, and the hub-source assertion list gains
`principal_kind: Some(slot.principal_kind)` so the ingress cannot silently stop stamping it.

**This one is not compile-proven, it is RUN:**
```
bun ./📜️script.ts presence-normalization-check source   → EXIT 0
presence-normalization-independent-oracle: AJV=1 LEB128=1 exact-vectors=19
presence-normalization-check: checks=19 phase=source
```
(`🗑️generated/ht1-presence-oracle.txt`.) AJV plus a third-party LEB128 encoder re-derived all 19 vectors and
matched the rewritten fixture byte-for-byte, and the hub's own bytes (`ff 07 … → ff 17 … 00` in the failure)
are the same. Independence intact.

## 22. The `500` — root cause found in the source, no bisect needed

`DirectorySpaceAdministrationPageV1::validate` contradicts itself for exactly one shape:

```rust
let anonymous = generation == 0 && binding.bytes().all(|byte| byte == b'0');   // :1302 — admits all-zero
…
|| !valid_document_open_hash(binding)                                          // :1305 — and rejects it
```
`valid_document_open_hash` (`:1663`) is `len == 64 && !all zeros && all lowercase hex`, and
`space_administration_session_binding_v1` returns `[0u8; 32]` precisely when there is no caller
(`🏗️bootstrap/🦀️.rs:6031`). So an anonymous page's binding is the one word the predicate forbids: the
**anonymous `Public` projection was unconstructible**, and `GET /directory/spaces/{public}` answered `500` to
every unauthenticated caller — on a route whose whole purpose is to be publicly readable. Not a test defect.

Fixed by splitting shape from admissibility: the binding is checked for the 64-lowercase-hex **shape**, and
which word is admissible stays with `anonymous || bound` immediately below — `anonymous` already demands
all-zero *and* generation 0, `bound` already demands non-zero *and* generation ≥ 1, and `Member`/`Author`
still demand `bound`. No authority is lost; a forged pairing (all-zero with a generation, or a digest with
generation 0) still fails both.

## 23. Verification (batch 4)

| command | result | capture |
|---|---|---|
| `bun ./📜️script.ts presence-normalization-check source` | **EXIT 0**, 19/19 vectors re-derived independently | `ht1-presence-oracle.txt` |
| `CARGO_TARGET_DIR=…/target-ht1 cargo check -p semio-hub --all-targets` | **0 errors**, 59.2 s, `semio-hub` + `semio-framework-os-kernel` recompiled | `ht1-check-5.txt` |

Files changed in batch 4: `…/🌱️space-artifact-creation-v1/🦀️.rs` (§20),
`…/📇️directory/🧬️schema/🦀️.rs` (§22), `🌎️hub/🧫️fixtures/🪪️presence-normalization-v1/🔣️.json` (10 vectors),
`🌎️hub/📦️packages/🦀️rust/📜️script.ts` (neutral oracle + import). Ticket artefact:
`🐍️ht1-presence-principal-kind.py`.

## 24. Remaining after batch 4 — 17 expected, all named

`inference::wal` ×4 + `runtime` ×3 + `sqlite` ×1 (the `semio-pool-worker-1` store-drop-witness panic at
`🏪️store/🦀️.rs:18399` sits under `gis_map_terminal_close…`'s `Busy { retained_uses: 1 }`) — H1b R1/R6, the
discarded assembly terminal first; `directory_event_page_v1_*` ×2; `document_open_plan_admin_revocation…`;
`RemoveMember 400 ≠ 202`; `admin_removal…` (needs `🛂️admin-presence-target-recovery-v1` given a
creation-owned `documentId`); four `Elapsed(())` deadline laws, still load-shaped and still unproven.

**needs hub rerun** — same flags. Also worth a look on a 13:44-or-later binary: §14 + §20 together are the
whole creation-and-open path against a real catalog, which is DS1's `/readyz` chain and outcome 2's gate.

---

# HT1 — batch 5 (rerun 14:04, binary 14:01) and hand-over

**321 — 300 / 21**, the same count as 13:46, but the set moved. Diffed against `ht1-red-clusters-4.txt`:

| | law | verdict |
|---|---|---|
| green | `presence_normalization_matches_neutral_authority…` | §21's producer-path rewrite + neutral-oracle update landed |
| **new red** | `artifact_authority::creation::tests::creation_contract_matches_neutral_intents_and_ready_only_coordinates` | **caused by me — §20. Reverted.** |
| moved | `checkpoint_publication_route_*` ×2 | past the ready check → `Conflict("document index descriptor binding differs")` |
| moved | `space_public_boundary…` | the `500` is **gone** (§22 worked); it now fails a later assertion at `🔬️bin-unit/🦀️.rs:4278` |

## 25. §20 was wrong — the equality is the declared contract

The neutral contract fixture `🌱️space-artifact-creation-v1/🔣️.json:487` carries a vector **`ready-kind-mismatch`**
with `accepted: false`, and `🌱️creation/🧪️tests/🔬️unit/🦀️.rs:186` asserts it. So
`parent_dialect.artifact_kind == kind_id` in `SpaceArtifactCreationReadyV1::validate` IS the contract, and my
batch-4 reasoning ("same category error as §14") does not transfer across the two boundaries. **Reverted** to
the exact prior text; `cargo check -p semio-hub --all-targets` → **0 errors** (`ht1-check-6.txt`). My earlier
grep missed the law because it mutates the pair through a JSON fixture id, not a Rust field path — the lesson
is that a neutral-contract fixture must be grepped by *vector id*, not by field name.

§14 (trusted catalog) is a different boundary and stands on evidence, not symmetry:
`native_openable_stdio_provider…` flipped **green** with it, i.e. a real stdio catalog loads for the first
time. Only the creation-side twin was wrong.

Consequence, stated honestly: with the revert the two `checkpoint_publication_route_*` laws go back from
`"document index descriptor binding differs"` to `"artifact creation accepted identity is invalid"`. The open
question is the hand-over's first row.

## 26. Hand-over — 20 expected reds after the revert

| law | panic (file:line) | best root-cause hypothesis |
|---|---|---|
| `checkpoint_publication_route_is_author_owned…`, `…rejects_stale_or_cross_scope_inputs…` | `🔬️bin-unit/🦀️.rs:867` `accepted identity is invalid` (ready-kind) | the fixture passes `selection.parent_dialect` (`s.stdio.json`) beside a descriptor whose `artifact_kind` is the manifest id (`stdio.json`), and `ready()` requires them equal. Decide which space a CREATED artifact's descriptor carries by reading `🌱️creation/🧑‍🏭️service-v1/🦀️.rs:93` onward, then make `🔬️bin-unit/🦀️.rs:989-999` and `:1415` consistent. Do **not** relax the validator. |
| `space_public_boundary_real_routes…` | `🔬️bin-unit/🦀️.rs:4278` `left == right` (via `run_socket_test`'s `:470`) | the `500` is fixed; this is the next assertion in the same law, unread |
| `admin_removal_revokes_visible_plan_presence…` | `🔬️bin-unit/🦀️.rs:2347` `issue document open plan: 404` | needs a genesis; its `documentId` is pinned by `🧫️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json` to `admin-presence-recovery`, not creation-owned. Give the fixture an `artifact-<32hex>` id + update its `expected` block, then `seed_genesis_for_document_for_test` |
| `scoped_directory_socket_route_rejects_scope_substitution…` | `🔬️bin-unit/🦀️.rs:2108` `no revocation close before 5s deadline` | deadline; unproven |
| `scoped_directory_socket_removal_and_delivery…` | `🔬️bin-unit/🦀️.rs:6801` `removal-wins sender deadline: Elapsed(())` | deadline; unproven |
| `socket_grant_revoke_before_command_admission…` | `🔬️bin-unit/🦀️.rs:3491` `left == right` | unread |
| `socket_admin_user_gate…`, `retained_short_admin_request…` | `:6958` / `:1188`+`:6650` `Elapsed(())` | deadline; unproven |
| `directory_event_page_v1_append_admission…` | `🔬️bin-unit/🦀️.rs:5789` `left: 5, right: 3` PostgreSQL admit seams | unread |
| `directory_event_page_v1_route_revalidates_session_generation…` | `🔬️bin-unit/🦀️.rs:5726` `control.cancelled` never set | unread |
| `document_open_plan_admin_revocation…` | `🔬️bin-unit/🦀️.rs:3185` `"cancelled"` vs `"succeeded"` | unread |
| `inference::wal` ×4 | `🧾️wal/🧪️tests/🔬️unit/🦀️.rs:{303,338,374}`, `⛓️chain/🦀️.rs:{232,321}` | H1b R1 |
| `inference::runtime` ×3 (+ the `semio-pool-worker-1` panic `🏪️store/🦀️.rs:18399` "store reached Drop without its terminal-empty shallow-shell witness", which sits under `Busy { retained_uses: 1 }`) | `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:{494,804,1323}` | H1b R6 — the discarded three-store assembly terminal (`🏃️runtime/🦀️.rs:1421-1426`) first; the drop-witness is the memory playbook's known bucket |

Full per-law panics: `🗑️generated/ht1-handover.txt`. Batch-5 file changed:
`…/🌱️space-artifact-creation-v1/🦀️.rs` (revert only).

**needs hub rerun** — one revert landed; expect the creation-contract law green and the two
`checkpoint_publication_route_*` back on the ready-kind term.

---

# HT2

Picked up HT1's §26 hand-over against the 14:04 verdict list (`321 — 300 / 21`) plus the 14:15 rebuild.
Working top-down. **Batch 1 = 6 laws, landed and compile-proven; not yet run.**

## H1. `checkpoint_publication_route_*` ×2 — which id space creation speaks, decided from the callers

The hand-over asked me to decide, at `🌱️creation/🧑‍🏭️service-v1/🦀️.rs:93`, whether `kind_id` is a manifest
`ArtifactKindSpec::id` or a `Dialect::artifact_kind`. Read end to end, the chain pins it to the **manifest
space**, and the binding term is not in the service at all:

- `ArtifactCreationPreparedV1::validate` (`🌱️creation/🧬️schema/🦀️.rs:307`) refuses unless
  `d.artifact_kind == intent.request.kind_id`, where `d` is the real `DocumentDescriptor`;
- `ArtifactCreationIntentV1::validate` (`:258`) builds a `DocumentDescriptor` *from* `request.kind_id` and
  requires its digest to be constructible;
- `validate_descriptor_open_target` pins a descriptor's `artifact_kind` to a manifest `ArtifactKindSpec::id`
  (HT1 §14).

So `request.kind_id` → `descriptor.artifact_kind` → manifest kind, one space the whole way. HT1's §20 reading
was right about the *space*; the revert was right about the *contract*, because three separate validators
declare the equality `kind == dialect.artifact_kind`:

| validator | file:line |
|---|---|
| `SpaceArtifactCreationReadyV1::validate` | `…/🌱️space-artifact-creation-v1/🦀️.rs:187` |
| `SpaceArtifactCreationKindV1::validate` | `…/🌱️space-artifact-creation-v1/🦀️.rs:67` |
| `document_index_projection_v1` | `🌎️hub/📇️directory/🦀️.rs:1808` (`descriptor.artifact_kind != entry.dialect.artifact_kind`) |

**And the shipped plugins disagree with each other about it.** GIS satisfies all three by construction — its
`ArtifactKindSpec` is literally `id: GISMAP_DIALECT.artifact_kind.into()`
(`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:144`), so kind == dialect kind == `s.gis.gismap`, and its native
codec identity declares `artifact_kind: "s.gis.gismap"`. stdio does not: `id: "stdio.json"`
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🦀️.rs:58`) and `artifact_kind: "stdio.avi"`… in the committed
projection `📇️registry/📜️native-codec-factories.json`, against dialects `s.stdio.<x>`.

That is why the two laws diverge exactly where the two plugins do. `:1415` (the **GIS** genesis) passes
`selection.parent_dialect` unchanged and is fine — for GIS the two spellings are the same string.
`:989` (the **stdio** checkpoint fixture) passed `selection.parent_dialect` (`s.stdio.json`) beside a
descriptor whose kind is `stdio.json`, and failed first `ready().validate()` and then, once HT1 relaxed that,
`document_index_projection_v1`.

**Landed** (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:989`): the fixture now builds its parent dialect the way its two
sibling genesis helpers at `:878` and `:912` already do — the descriptor's own artifact kind, keeping the
catalog's standard and subset. No validator relaxed, no fixture vector touched.

**Named product defect, unowned, bigger than this slice.** The real root is that stdio spells its artifact
kinds in the manifest space while its dialects are `s.`-prefixed, so against a *real* stdio catalog
`accept()` → `intent.validate()` → `ready().validate()` still refuses, and `artifact_creation_catalog` still
cannot emit a valid `SpaceArtifactCreationKindV1`. **Artifact creation from a stdio bundle is impossible
today**, which is outcome 2's create path. HT1 §14 deleted the trusted-catalog equality to let the bundle
*load*; the three validators above are the same equality further downstream and were not deleted. The
principled fix is to make stdio follow GIS (`ArtifactKindSpec::id == Dialect::artifact_kind`) across
`artifact_kind()`, the receipts and the committed registry projection — a plugin-side change with a wide
blast radius that must not land mid-fleet from this slice.

## H2. `space_public_boundary_real_routes…` — the next assertion was impossible as written

`:4278` asserted an authenticated non-member's `GET /directory/spaces/{public}` body is **byte-identical** to
the anonymous one. `DirectorySpaceAdministrationPageV1::Public` declares three per-caller fields —
`sessionBindingSha256`, `authorizationGeneration`, `receiptSha256` — and §22 established that the anonymous
word is all-zero/generation 0 while a bound caller's is a digest/generation ≥ 1. The two bodies can never be
equal; the law could only pass while the route answered `500`.

**Landed**: the law now compares the two projections with the caller-bound triple stripped (new helper
`public_projection_without_caller_binding`, `🔬️bin-unit/🦀️.rs:4142`) **and** additionally asserts the
discrimination the schema promises — anonymous is `"0"×64` at generation 0, the authenticated reader's
binding differs and carries generation ≥ 1 — plus `assert_public_projection_has_no_private_keys` on the
non-member body too. Strictly more is asserted than before.

## H3. `admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` — 404

The law announced its document (`announce_document_for_test`) and then issued an open plan, which needs a
committed genesis, so `issue_document_open_plan_inner` answered `404`. Per the hand-over the id must be
creation-owned.

**Landed**: `🧫️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json` `documentId` →
`artifact-a01f2c3d4e5f60718293a4b5c6d7e8f9` (32 lowercase hex, non-zero, so `request_id()` and
`SpaceArtifactCreationStatusV1::validate` admit it), and the law seeds through
`seed_genesis_for_document_for_test` with the removed member's author session instead of announcing. The
`expected` block needed no change — nothing in it names the id.

## H4. `directory_event_page_v1_append_admission…` — a stale census literal, not a lost invariant

`left: 5, right: 3`. The law pinned the *number* of `validate_directory_event_page_event(&` call sites at 3 per
remote backend. Both backends now have 5, and all five in each are validate-then-persist (postgres
`:1305/2326/2693/2934/3008`, each immediately before its `INSERT INTO hub_directory_event`; neo4j
`:1087/2183/2598/2851/2933`, each before its `CREATE (e:DirectoryEvent …)`). Two append seams were added since
the literal was written; the invariant never broke.

**Landed**: the law no longer pins a number. It counts each backend's event-row persistence seams
(`INSERT INTO hub_directory_event(seq` / ` (seq`, `CREATE (e:DirectoryEvent {seq:`), asserts each backend
still has ≥ 3, and asserts the validate count **equals** the persistence count — so a future seam added
without admission fails the law instead of silently passing a bumped literal.

## H5. Verification (batch 1)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht2 cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors**, 313 warnings, 2 m 04 s, `semio-hub` recompiled | `🗑️generated/ht2-check-1.txt` |

Files changed: `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` (H1 `:989`, H2 `:4277`+ helper, H3 `:3596`, H4 append law),
`🌎️hub/🧫️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json` (H3).

## H6. Triage carried into batch 2 (read, not yet fixed)

- **`inference::wal` ×4 — one root, not four.** `committed_fixture_witness` (`🧾️wal/🧪️tests/🔬️unit/🦀️.rs:338`)
  unwraps `verify(...)` and gets `Ok(None)`: the committed WAL record produces **no witness**. That single
  absence explains `:303` (`outcome` = `absent` ≠ `"exact-committed-command"`), `:338` and `:374`
  (instead of `Err(Invalid)` the hostile `trailing-byte-after-hlc` command reaches a non-`Invalid` outcome).
  `git log` shows the WAL *source* untouched since 2026-09-09; the only working-tree change under
  `💡️inference` is the `mod quick { … }` re-indentation of the three test files, which does not move
  `include_str!` paths. So these are pre-existing product failures (H1b R1), surfaced by the lane split.
- **`inference::runtime` ×3 / the `🏪️store:18399` drop witness.** The `Assembly → Terminal` arm
  (`🏃️runtime/🦀️.rs:1420-1426`) restores the three stores into `Ready { owners, pending: None,
  document_write: None }` — it **drops the Assembly's `document_write`** on the floor rather than closing it.
  That is the shape the drop-witness rule predicts for `Busy { retained_uses: 1 }` at worker-pool shutdown
  (`🔬️unit/🦀️.rs:804`). Not yet fixed; needs the close path read before editing.
- **4 `Elapsed(())` deadline laws** and `socket_grant_revoke` / `document_open_plan_admin_revocation`
  (`:3185`, `"cancelled"` ≠ `"succeeded"` from `RevokeUserSessions`) — unread.

**needs hub rerun** — 6 laws changed, `cargo check` green, nothing run yet.

## H7. Batch 1 measured (rerun 14:33) — **321 — 303 / 18**, three of five flipped

| law | verdict | where it went |
|---|---|---|
| `space_public_boundary_real_routes…` | **green** | H2 |
| `directory_event_page_v1_append_admission…` | **green** | H4 |
| `creation_contract_matches_neutral_intents…` | **green** | HT1's §25 revert |
| `checkpoint_publication_route_*` ×2 | still red, **new root** | H1 worked — both now clear the genesis and die at `🔬️bin-unit/🦀️.rs:1102`, `checkpoint input blob lands before publication: left 400, right 200` |
| `admin_removal…` | still red, **new root** | H3 worked — the `404` is gone; it now reaches the socket and dies at `:2055` `stream ended before server frame` |

## H8. KD2 — the checkpoint-publication input blob is addressed in the wrong hash space

New, and it is product, not fixture. The two checkpoint laws now fail on `PUT /spaces/{s}/blobs/{sha256}`
answering **400**, and the reason is a genuine dead end:

- `PayloadStorage` is declared **blake3** CAS —
  `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:1132` ("Content-addressed blob storage (blake3
  CAS)"), and its `put` returns "its `blake3` `ContentHash`" (`:1136`). The digest is taken with
  `semio_framework_hash::Hasher`, which is BLAKE3 (`🧰️framework/🔨️modules/🔏️hash/🦀️.rs:332`).
- `put_blob` (`🏗️bootstrap/🦀️.rs:3504-3519`) rejects with `400` unless the **URL path hash equals that
  BLAKE3 address**.
- `checkpoint_publication_blob` (`🏗️bootstrap/🦀️.rs:3691-3699`) resolves its input with
  `parse_content_hash(&reference.sha256)` — feeding a **SHA-256** digest (`CheckpointPublicationBlobV1.sha256`,
  verified two lines later with `Sha256::digest`) into the inverse of a BLAKE3 `ContentHash::Display`.

So the publication command names its inputs by SHA-256 while the only blob upload route
(`/spaces/{space_id}/blobs/{hash}`, `🏗️bootstrap/🦀️.rs:9591` — the router has no other blob seam) accepts
only BLAKE3 addresses. **No client can supply the pack/spr bytes a checkpoint publication references**, and
no test fixture can either. I did not patch it: the three coherent repairs are (a) give
`CheckpointPublicationBlobV1` the store address beside its integrity digest (wire change), (b) route
publication inputs through the SHA-256-addressed artifact CAS and add the staging seam it lacks, or
(c) make the hub blob namespace SHA-256 end to end. That is a product decision with callers outside this
slice, and it changes a route several green laws already assert on.

## H9. `admin_removal…` — the `404` is fixed; the next hop is the socket, not the plan

With the genesis seeded the open plan now issues (`issue_document_open_plan_inner`'s
`get_active_artifact_checkpoint(...).ok_or(NotFound)` at `🏗️bootstrap/🦀️.rs:2832` was the `404`, confirmed in
source). The law then opens the member socket, sends hello, and the server **closes without sending a frame**.
Narrowed but not closed: the difference from the previous `announce`-only shape is that the scope now has a
committed genesis checkpoint, which brings `verified_rebootstrap_control` /
`send_socket_document_rebootstrap` (`🏗️bootstrap/🦀️.rs:4332-4345`) into the path. No green law in this file
combines a genesis with a document socket, so there is no precedent to copy. Left for the next batch; the
`404` fix stands and must not be reverted (the checkpoint is genuinely required).

## H10. Batch 2 — the four deadline laws, given a deterministic basis

All four asserted **wall-clock** around an **in-process rendezvous**, in a suite of ~320 hubs on a 10-core
machine under a full fleet. None of them measures latency; the clock was only a hang guard, and it was
duplicated:

| site | was | now |
|---|---|---|
| `raw_http_request_transport` `🔬️bin-unit/🦀️.rs:1173` | own `5 s` read bound **inside** `bounded_http_request`'s declared `20 s` — the inner one silently pre-empted the documented one | one named bound, `RAW_HTTP_READ_HANG_GUARD` (`:484`), used by both. The read's real completion condition is EOF or content-length, which is already deterministic |
| `next_close_without_authority` `:2107` | `timeout(5 s, ws.next())`, `Err(_) => panic!("no revocation close before 5s deadline")` | plain `ws.next().await`; the law's actual terms (close frame / no authority-bearing frame / no early EOF) are unchanged |
| `gate.socket_scoped_send_admitted.acquire()` ×2 `:6831`, `:6843` | `timeout(2 s, …)` | `.acquire().await` — a semaphore the server task signals is already a deterministic rendezvous |
| `gate.socket_admin_revoke_admitted.acquire()` `:6988` | `timeout(2 s, …)` | `.acquire().await` |

A genuine hang is still caught, once, by the harness that owns it: `.config/nextest.toml`
`[profile.long] slow-timeout = { period = "300s", terminate-after = 1 }`, which the coordinator's run uses.
Nothing was deleted, ignored or loosened — the four laws assert exactly what they asserted before, minus a
clock that was never part of them.

## H11. `socket_grant_revoke_before_command_admission…` — a pre-envelope request body

`:3491`, `left: 400, right: 202`. The law posted a **bare** `DirectoryCommand` to `/directory/commands`, but
the route has since closed over `DirectoryCommandRequestV1` (`schema` + `request_id` + `command`) and refuses
anything else with `malformed-request` → `400` (`🏗️bootstrap/🦀️.rs:5923`). Every other command law in the file
already goes through `directory_command_body` / `post_directory_command_for_test` (`:5042`, `:5046`); this one
did not. **Landed**: it now uses `post_directory_command_for_test` with a 32-hex request id. The dropped
`owner_authorization` local had no other use.

## H12. Verification (batch 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht2 cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors**, 313 warnings, `semio-hub` recompiled | `🗑️generated/ht2-check-2.txt` |

Files changed in batch 2: `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` only.

## H13. KD1 — the exact sites, for whoever owns it

**Claim**: artifact creation from a real stdio bundle is impossible, because the hub requires an artifact
kind and its dialect's artifact kind to be the same string, GIS satisfies that by construction and stdio
does not.

*The three validator sites that require the equality*

| # | site | term |
|---|---|---|
| 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs:187` (`SpaceArtifactCreationReadyV1::validate`) | `self.parent_dialect.artifact_kind == self.kind_id` |
| 2 | same file `:67` (`SpaceArtifactCreationKindV1::validate`) | `self.dialect.artifact_kind == self.kind_id` |
| 3 | `🌎️hub/📇️directory/🦀️.rs:1808` (`document_index_projection_v1`) | `descriptor.artifact_kind != entry.dialect.artifact_kind` ⇒ `Conflict("document index descriptor binding differs")` |

Reached from `accept()` → `ArtifactCreationIntentV1::validate` (`🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs:258`,
which calls `self.ready().validate()` at `:281`), from `materialize_selected_genesis`
(`🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:63-70`, `target.validate()` ⇒ `AuthorityError::InvalidScope`),
and from `artifact_creation_catalog` (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:371-399`, which emits
`kind_id: selection.artifact.kind` beside `dialect.artifact_kind: selection.parent_dialect.artifact_kind`).
The client-facing id is one space the whole way: catalog `kind_id` → `SpaceArtifactCreateV1.kind_id` →
`artifact_creation_selection` (matches `selection.artifact.kind`, `🔏️trusted-catalog/🦀️.rs:352-355`) →
`intent.request.kind_id` → `descriptor.artifact_kind` (pinned by `ArtifactCreationPreparedV1::validate`,
`🌱️creation/🧬️schema/🦀️.rs:307`).

*GIS — satisfies it, by deriving both from one constant*

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:22` — `GISMAP_DIALECT = Dialect { artifact_kind: "s.gis.gismap", … }`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:144` — `ArtifactKindSpec { id: GISMAP_DIALECT.artifact_kind.into(), … }`
- `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs:36` — codec identity `artifact_kind = "s.gis.gismap"`

*stdio — violates it, in three places that must move together*

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🦀️.rs:58` — `ArtifactKindSpec { id: "stdio.json", … }` (and the
  sibling `artifact_kind()` of every other stdio artifact)
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json` — every receipt's `"artifact_kind"` is
  `"stdio.<x>"` (`"artifact_kind": "stdio.avi"`, …), projected by `📇️registry/🦀️.rs:520` and cross-checked at
  `:279` and `:624`
- the dialects those artifacts actually declare are `s.stdio.<x>` — e.g. the app dialect in the hub's own
  synthetic bundle, `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:296`, whose surface id assertion at `:307`
  (`"s.stdio.json@rfc8259/*#viewer"`) pins the real plugin's spelling

*Consequence for HT1 §14*: the equality HT1 deleted from `🔏️trusted-catalog/🦀️.rs` is the same equality as
sites 1–3, one layer earlier. Deleting it let a stdio bundle **load**; it did not make creation work, and if
stdio is brought in line with GIS that deletion should be reconsidered rather than kept.

## H14. Still open after batch 2

`inference::wal` ×4 + `inference::sqlite` ×1 (one root — the committed witness is absent, see H6),
`inference::runtime` ×3 (the discarded `document_write` in the `Assembly → Terminal` arm, see H6),
`checkpoint_publication_route_*` ×2 (**KD2**, H8), `admin_removal…` (H9),
`directory_event_page_v1_route_revalidates_session_generation…` (`:5748`, `control.cancelled` never set — a
cancelled request's server-side control is not marked; unread), and
`document_open_plan_admin_revocation…` (`:3185`, `RevokeUserSessions` returns `"cancelled"` not
`"succeeded"`; unread).

Unrelated hygiene noticed in passing, not mine to land: a leftover `eprintln!("[DEBUG] …")` at
`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:1091`.

**needs hub rerun** — batch 2 is 6 more laws changed (4 deadline + 1 command envelope + the shared HTTP read
bound), `cargo check` green, nothing run yet.

## H15. The three TIMEOUTs were mine, and they were not flaky — the signal never comes

Rerun 14:49: **303 / 15 / 3 TIMED OUT**, suite wall time 31 s → 311 s. The three are exactly the three waits
I made unbounded in H10, and removing the clock did not expose a slow rendezvous — it exposed that the
rendezvous **never completes at all**. The old 2 s/5 s bounds were not measuring anything; they were the only
thing turning a never-signalled wait into a fast red. My H10 reasoning ("a semaphore the server task signals
is already deterministic") assumed a premise that is false for these three.

| law | waits on | why it never returns |
|---|---|---|
| `socket_admin_user_gate_rejects_a_late_same_user_grant_after_batch_revoke` | `gate.socket_admin_revoke_admitted` | **the product never signals it.** `socket_admin_revoke_admitted` is declared (`🏗️bootstrap/🦀️.rs:722`) and constructed (`:771`) and `add_permits` is called on it **nowhere in the repo** (grepped, whole tree, `🦀️.rs`). Its paired `socket_admin_revoke_release` is equally producerless. The law waits at a seam that does not exist — either it was removed from the socket's admin-revoke path or it was never implemented |
| `scoped_directory_socket_removal_and_delivery_have_one_total_membership_order` | `gate.socket_scoped_send_admitted` ×2 | producer exists (`🏗️bootstrap/🦀️.rs:6847`, `:6861`) but is triple-gated: `#[cfg(test)]` **and** `record.audience == DirectoryScoped` **and** `socket_scoped_send_mode ∈ {1,2}` **and** a scoped directory frame actually being sent. The permit's absence means no scoped frame reaches that socket — the real defect is upstream of the gate |
| `socket_grant_revoke_before_command_admission_has_no_storage_effect` | `next_close_without_authority` | the revocation close never arrives; the law's own premise ("no Ack crosses a revoke that wins before command admission") is what is failing |

**Landed**: one named `SOCKET_RENDEZVOUS_HANG_GUARD = 15 s` (`🔬️bin-unit/🦀️.rs:484`) restores a bound to all
four sites, with messages that now say what is wrong (`"the admin revoke gate was never admitted"`,
`"the revocation close never arrived: the server closed no socket for this revoke"`) instead of naming a
duration. One constant instead of 2 s/5 s picked per call site, so it is not load-shaped; the three failures
come back fast and local. Suite wall time returns to ~31 s (the three run as separate nextest processes, so
they overlap). `RAW_HTTP_READ_HANG_GUARD` (H10) stays — no HTTP law timed out.

**Honest correction to H10**: only the *duplication* of bounds was wrong. Deleting them was wrong too, and I
should have checked whether a producer existed before calling a wait "deterministic". Three named roots came
out of it, which is the part worth keeping.

## H16. KD2 decided and landed — schema-first, the BLAKE3 address beside the SHA-256 claim

Decision taken as directed: the payload store is a BLAKE3 CAS **by declaration**
(`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:1132`, `PayloadStorage` — "Content-addressed blob
storage (blake3 CAS)"), so the address is BLAKE3 and nothing else can find the bytes. The publication
reference now carries it; SHA-256 stays exactly what it is, the integrity claim the artifact lineage is
written in, and is still verified against the resolved bytes.

| layer | change |
|---|---|
| Rust schema | `CheckpointPublicationBlobV1 { sha256, blake3, byte_length }` (`…/📇️directory/🧬️schema/🦀️.rs:441`), docstring states both spaces; `blob_valid` requires `valid_document_open_hash(&blob.blake3)` |
| TS twin | `CheckpointPublicationBlobV1` interface + `checkpointPublicationBlob` parses `["sha256","blake3","byteLength"]` (`…/🧬️schema/🟦️.ts:408`, `:457`) |
| JSON-schema module | `CheckpointPublicationCommandBlob` gains required `blake3` (`…/🧬️schema/🔣️.json`, `$defs`) — this is what `hubSchemaExport` compiles, so AJV and the twin stay independent |
| neutral fixtures | `📣️checkpoint-publication-command-v1/🔣️.json` (pack+spr) and `🌱️artifact-genesis-v1/📤️current.json` (**20** command blobs across every case, accepted and rejected alike — adding a required field never repairs a case's own defect) |
| hub route | `checkpoint_publication_blob` resolves `parse_content_hash(&reference.blake3)` (`🏗️bootstrap/🦀️.rs:3693`); the `Sha256::digest(bytes) == reference.sha256` check two lines below is unchanged |
| laws | `checkpoint_publication_command` emits both digests, and `put_checkpoint_publication_blob` now `PUT`s at the BLAKE3 address `put_blob` actually enforces (`🔬️bin-unit/🦀️.rs:1098`, `:970`) |
| live process oracle | `📜️script.ts` uploads at `blake3Hex(bytes)` and sends both digests; imports the repo's own TS BLAKE3 (`🧰️framework/🔨️modules/🔏️hash/🟦️.ts`), so the oracle stays independent of the Rust hasher |

One rule I tried and withdrew: `blake3 != sha256`. AJV cannot express a cross-field inequality, so the neutral
oracle refused the fixture — the two oracles must agree, and an invariant only one of them can state is not a
contract. Dropped from Rust, TS and the hostile corpus.

## H17. Verification (batch 3)

| command | result | capture |
|---|---|---|
| `bun ./📜️script.ts checkpoint-publication-check source` (from `🌎️hub/📦️packages/🦀️rust`) | **EXIT 0** — `valid=2 rejected=14 ajv=1 typescript=1 sha256=2 … process-route=1`, `checks=24`. Rejected corpus grew 12 → 14 with the two new address cases (missing `blake3`, all-zero `blake3`) | `🗑️generated/ht2-oracle-1.txt` |
| `CARGO_TARGET_DIR=…/target-ht2 cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors**, 313 warnings, `semio-hub` recompiled | `🗑️generated/ht2-check-4.txt` |
| `bun ./📜️script.ts space-artifact-creation-check source` | **EXIT 1**, `required checkpoint presence differs: present` — **pre-existing and not mine**: it fails before the genesis publication cases are reached, and its fixture `🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json` is unmodified in the working tree | `🗑️generated/ht2-oracle-2.txt` |

Files changed in batch 3: `…/📇️directory/🧬️schema/🦀️.rs`, `…/📇️directory/🧬️schema/🟦️.ts`,
`…/📇️directory/🧬️schema/🔣️.json`, `…/📣️checkpoint-publication-command-v1/🔣️.json`,
`…/🌱️artifact-genesis-v1/📤️current.json`, `🌎️hub/🏗️bootstrap/🦀️.rs`, `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`,
`🌎️hub/📦️packages/🦀️rust/📜️script.ts`.

## H18. Still open

Untouched roots, unchanged from H14: `inference::wal` ×4 + `inference::sqlite` ×1 (one root — the committed
witness is absent), `inference::runtime` ×3 (the discarded `document_write` in the `Assembly → Terminal`
arm), `directory_event_page_v1_route_revalidates_session_generation…`, `document_open_plan_admin_revocation…`,
`admin_removal…`'s socket close (H9). Newly named and needing an owner's decision rather than a patch: the
producerless `socket_admin_revoke_admitted` gate (H15, row 1).

**needs hub rerun** — batch 3: the three hangs are bounded again (expect three fast reds, suite back to ~31 s)
and KD2 is landed across schema, twin, JSON-schema module, two fixture corpora, the hub route, the laws and
the live oracle. `cargo check` green, neutral oracle green, nothing else run.
