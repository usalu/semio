# 📓️ M9 — an MCP agent EDITS a hub document and a human sees it

Slice M9 (session 7b, 2026-09-21). Finishes what M8 opened: M8 made the agent a hub *reader*
(`artifact_open` + `artifact_snapshot` of a real `gis.map` hub document on 7621) and named D2 —
`action_prepare` answers `repo root not found` — as the whole remaining distance. This slice's bar
is a `Commands` frame in the hub ledger authored by an agent, and a human browser seeing it.

Status legend: ✅ landed and executed live · 🚧 landed and compiled, not executed · ❌ not done.

## 0. Headline

**D2 — the refusal M8 named as "the whole remaining distance" — is closed: a hub-bound MCP gateway
now fetches, hash-verifies and compiles the plugin component the HUB authorized, and
`action_prepare` no longer answers `repo root not found`.** Measured live on C5's hub 7621, not
reasoned about (§2.3). It then stops one step further on, at `wasmtime` instantiate, against a
component-world export (`semio:framework/codec@1.0.0`) that peer **TC3b added to `world actor` in
the working tree today** and that every component published before it — including 7621's, published
at 05:49 — therefore lacks. The same failure now blocks `client-e2e` and `live-agent-loop-check`
in the FOLDER lane too, so it is a tree-wide catalog/world boundary and not this chain's defect
(§2.4). **No agent edit crossed, so no human saw one, and no screenshot is offered.**

Landed with laws, in order of certainty:

- ✅ **The hub execution-target component chain** (§2) — a new `DirectoryClient` route method, a
  budget derived from the hub's own ceiling instead of a JSON-page number, verified bytes, and
  `PluginComponentSource::{Repo, Hub}` in place of an `Option<PathBuf>` that meant "no plugins at
  all" for every hub session. Live-proven as far as instantiate.
- ✅ **The kind-spelling finding was a mislabel, not drift** (§5.1) — `s.gis.gismap` is the artifact
  KIND, `gis.map` the pack schema, and `DocumentIndexEntryV1::validate`'s grammar settles it. The
  gateway published only the schema and called it `kind`. Fixed; row 5b of the probe went red → green
  live, so an agent can now match a verb to a hub document.
- ✅ **A retryable binding state no longer kills the gateway at startup** (§5.2) — bounded typed
  retry, with a law that pins both the bound and that non-retryable failures are never retried.
- ✅ **The hub no longer ABORTS on its 30 s trusted-catalog budget** (§5.3) — it reports a closed
  `artifactAuthority` gate with a reason of its own and still binds. Hub Rust: `cargo check` only.
  **Needs coordinator hub rerun.**
- ❌ **`client-e2e`'s "the snapshot shows the mutation"** (§4) — NOT fixed and not attempted blind:
  the row is downstream of `artifact_create`, which §2.4's boundary now fails, so the defect could
  not be reproduced, let alone a fix verified. §4 records what narrowing was possible statically.
- ❌ **The gateway's presence beat** (§3) — the kind is hub-derived, not client-sent, and already
  correct; what is missing is the gateway opening a document socket at all, which is downstream of
  §2.4.

## 1. Inherited state

Nothing of my own: `ls 🗑️generated | grep -i m9` empty, no `📓️m9-*.md`, no `🐍️m9-*`. Context from
M8 (§5.3 gaps, the 3 gates, the live-only findings), C3 §2 (the browser attach chain), AP1, LB1,
PR1. Hub 7621 (C5's, `jc1-boot`, jco 1.34) answered `/readyz` `status: ready` at slice start.

## 2. Item 1 — the gateway resolves its execution target from the HUB (D2)

### 2.1 The measurement that sized it

`POST /spaces/{s}/documents/{d}/execution-target/{manifest,component,descriptor}` on 7621 as
`user1@semio.dev` (`🗑️generated/m9-exec-target-sizes.txt`):

| asset | status | bytes |
|---|---|---|
| manifest | 200 | 3 038 |
| **component** | 200 | **47 466 541** |
| descriptor | 200 | 477 852 |

and the manifest names, in one document, BOTH vocabularies §5.1 is about:
`artifact.kind = s.gis.gismap`, `artifact.schema = gis.map`, plus
`surface.surfaceId = s.gis.gismap@1/*#editor`, `windowKindId = gis2d-main`,
`package.pluginId = gis`, catalog generation `8086b61f336e08b5…`.

### 2.2 What was missing, and what landed

| # | gap | landed |
|---|---|---|
| 1 | the Rust `DirectoryClient` has `document_execution_target_manifest` and `…_descriptor` and **no `…/execution-target/component` method at all** | `DirectoryClient::document_execution_target_component` (`📇️directory/🔌️client/🦀️.rs`), the exact sibling of the descriptor method, bounded by the hub's own `DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES` |
| 2 | the MCP hub transport's per-package byte bucket is a flat **16 MiB/min** — a JSON-page budget. A 47 MB component aborts mid-body with `ByteBudgetExhausted` (`🛎️services/🦀️.rs:1625` charges every real chunk) | the budget is now DERIVED: `DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES + 16 MiB` (`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`, `NativeHubBindingDriver::connect`) |
| 3 | the 10 s directory operation budget is a JSON budget too | `HUB_EXECUTION_TARGET_COMPONENT_TIMEOUT_MS = 120_000`, used only by the component fetch |
| 4 | `NativeHubBindingDriver` kept no client, so nothing could fetch on demand | it now holds the SAME authenticated `DirectoryClient` the binding refreshes through, and `fetch_execution_target_component(scope, expected, client_instance_id)` verifies **exact byte length and exact SHA-256** against the manifest lease before any bytes reach a runtime |
| 5 | `open_plugin_artifact_channel(repo_root: Option<&Path>, …)` — `None` for every hub session, hence `repo root not found` | `PluginComponentSource::{Repo(PathBuf), Hub(Arc<HubPluginComponents>)}`. A hub workspace binds the `Hub` arm; there is no repo fallback, because a hub-bound agent must run the code the hub authorized or none |
| 6 | `PluginArtifactChannel::new` could only read a `.wasm` off a repo path, and held a whole `PluginRegistryEntry` to use ONE field of it | `PluginArtifactChannel::from_component(bytes, plugin_id, descriptor, app_ref, actor_label)` is now the shared core; `new` reads the file and delegates. The struct holds `plugin_id: String`, not a registry row |

`HubPluginComponents::resolve(plugin_id)` reads the descriptor from the catalog snapshot the binding
already verified (manifest identity, canonical projection, digest — `refresh_catalog` does all of
it), fetches only the component, and caches it under `(catalog generation_id, component sha256)` so
a republished catalog is a different entry and never a stale hit.

**Compile gate:** `cargo check -p semio-framework-os-mcp --all-targets` → **0 errors**, 15 lib
warnings / 76 lib-test warnings (`🗑️generated/m9-mcp-check-1.txt`);
`cargo check -p semio-framework-os-kernel --lib` → 0 errors; gateway `build` exit 0, staged
(`🗑️generated/m9-mcp-build-1.txt`).

### 2.3 Live on hub 7621 — D2 is open, and the next boundary is named

`🐍️m9-agent-edit-probe.ts` (new; M8's probe plus an `artifactKind` match and a
before/after snapshot comparison), delegation `agent:01a0c336-b6b4-7861-8c0c-55e8abb36da3` into
C5's space `01a0c314-…` on document `artifact-0954e2d10d8fff9605f101b0dba34f3b`. Capture
`🗑️generated/m9-edit-run1.txt`.

| # | step | M8 | M9 |
|---|---|---|---|
| 1 | `initialize` over the delegated hub session | PASS | **PASS** |
| 2 | `context_resolve` names the agent | PASS | **PASS** `agent:01a0c336-…` |
| 3 / 3b | the agent sees the space's documents | PASS | **PASS** |
| 4 | `artifact_open` | PASS | **PASS** `kind=gis.map` |
| 4b | `artifact_open` names the artifact KIND | — (new) | **PASS** `artifactKind=s.gis.gismap` |
| 5 | `capabilities_search` over the hub catalog | PASS | **PASS** 1 hit |
| 5b | a verb typed against the opened document's kind | **FAIL** (§5.1) | **PASS** `gis.s.gis.gismap@1/*#editor.addFeature`, `verbKind == openedKind == s.gis.gismap` |
| 6 | `action_prepare` | **FAIL** `repo root not found — cannot locate the plugin registry or compiled wasm` | **FAIL, new reason** — `INTERNAL: instantiate: wasmtime: no exported instance named \`semio:framework/codec@1.0.0\`` |
| 6b | `action_invoke` | FAIL | FAIL (no handle) |
| 7 | `artifact_snapshot` | PASS | **PASS** `packBytes=80801 sprBytes=237` |
| 7b | the snapshot shows the mutation | — (new) | **FAIL** — identical, because 6 never committed |
| 8 | the hub's own document status | PASS | **PASS** `head_seq 0` |

**`repo root not found` is gone.** To reach `instantiate` at all the gateway must have: resolved the
plugin from the hub catalog, POSTed `…/execution-target/component`, received 47 466 541 bytes
through a pool budget that now admits them, matched them against the manifest's declared byte length
AND SHA-256 (a mismatch is a `PreconditionFailed` before any runtime sees a byte), and compiled them
through `shared_compiled_component`. Every one of those is new in this slice and every one is
proven by the fact that the failure is now downstream of all of them.

### 2.4 The boundary the last row names — a peer's in-flight WIT widening, not this chain

`strings` on the component the hub served (`🗑️generated/m9-hub-component-interfaces.txt`): it
exports `capabilities`, `checkpoint`, `describe`, `effects`, `events`, `instance-lifetime`, `jobs`,
`pure`, `reactor`, `types`, `ui` — and **no `codec`**. In the working tree right now, peer **TC3b**
has added `export codec;` to `world actor`
(`🔌️plugin/🧬️schema/📜️.wit`, uncommitted, plus `OwnedSemioExport::ALL` 9 → 13 in
`🧠️interpreter/🦀️.rs` and the four `codec_*` host calls in `🖥️host/🦀️.rs`). A wasmtime component
whose world lacks an exported instance the host's bindings require cannot be instantiated, so
**every** component built before TC3b lands — including the one C5 published to `jc1-boot` at 05:49
today — is now uninstantiable by a host built from this tree, in the folder lane exactly as in the
hub lane. This is preamble rule 28's jco/catalog boundary repeating one generation later, on the
component world instead of the codegen policy, and it clears for the hub lane only when the trusted
catalog is republished from a post-TC3b tree. It is not this chain's defect and this slice did not
paper over it: the row is left red naming it.

## 3. Item 2 — the gateway's presence beat carries kind `agent`

**Measured, and the premise is already satisfied on the side that decides it.** The presence
principal kind is NOT a client-sent field: the hub derives it from the authenticated session and
normalizes it onto every outbound roster row —
`if session_kind.is_agent() { PresencePrincipalKind::Agent } else { Human }`
(`🌎️hub/🏗️bootstrap/🦀️.rs:4839`), with `PresenceLeaseSlot.principal_kind`'s own doc stating the
reason: "never one a client claimed … so a human session can never impersonate an agent and an agent
can never hide as a human". The wire carries it at flags bit 11 in both codecs
(`📡️replication/📡️wire/🦀️.rs:1683`, `📡️replication/🟦️.ts:433`) — PR1/M6b's work, not mine — and the
gateway's session IS `AuthSessionKind::Agent`, minted at `POST /auth/agent-sessions`.

So there is nothing for a client to send, and **nothing in this item is a code gap**. What is missing
is that the gateway opens **no document socket yet**: the socket-grant leg (`…/socket-grants` → the
binary document socket as the store's backbone) is step 4 of M8 §5.3 and is gated behind a guest
that can be instantiated (§2.4). The beat will carry `agent` the moment that socket opens, because
the hub stamps it; **that is an argument, not a measurement, and it is labelled as one.**

## 4. Item 3 — `client-e2e` "the snapshot shows the mutation" ❌ not fixed

**The lane no longer reaches the row.** `client-e2e` measured this slice
(`🗑️generated/m9-client-e2e-1.txt`) is **15/17**, not M8's 35/38, and the run ABORTS at

```
FAIL  os: artifact_create (a real plugin artifact kind) — kind=s.note.note:
  `note` refused ReadArtifact (channel.not-wired):
  instantiate: wasmtime: no exported instance named `semio:framework/codec@1.0.0`
```

i.e. the folder lane hits §2.4's boundary on the shipped `note` component exactly as the hub lane
hits it on `gis`. Every row after `artifact_create` — including the one this item is about — never
executes, so the 38-row shape does not exist on this tree at all. The other red,
`os: capability catalog health`, is PZ1's (CE1 gap 1) and is counted separately.

I did not write a speculative fix for a defect I could not reproduce. What narrowing was possible
without running, for the next owner:

- The read is not a frozen row and not a wrong instance. `artifact_snapshot` →
  `read_live_session_artifact_bytes(plugin_id)` → `ActionAdapter::read_session_artifact(instance)` →
  `exchange_one(instance, AppCommand::ReadArtifact)`, and the `instance` is
  `plugin_instance_slot(&catalog, plugin_id)` — the SAME function `prepare_action` derives the
  mutation's slot from, so read and mutation address one slot.
- `PluginArtifactChannel::ensure_instance` caches exactly one `GuestInstance` per slot, so read and
  mutation reach one guest.
- `AppCommand::ReadArtifact` becomes `store::AppCommand::ReadDocument { seq: 0 }`
  (`🌉️mcp/🏠️workspace/🦀️.rs:1943`), which the guest answers from
  `plugin_document_pack` → `instance.app.document_pack()`
  (`🔌️plugin/🦀️.rs:36883`).
- **Therefore the remaining suspect is inside the guest**: `PluginApp::document_pack()` re-encodes
  from a snapshot that a committed transaction did not move, while the history/head that
  `ReadHistory` and undo/redo read demonstrably did. The next measurement is a single guest-side
  one — `document_pack()` immediately before and after a `TransactionCommit` on one instance — and
  it needs a `note` component rebuilt against the post-TC3b world.

## 5. Item 4 — the live-only findings M8 named

### 5.1 Kind-spelling drift `gis.map` vs `s.gis.gismap` ✅ — it was never drift, it was a mislabel

Measured on 7621 (§2.1): one execution-target manifest carries **both**, as two named fields of one
`artifact` object — `kind: "s.gis.gismap"`, `schema: "gis.map"` — and the hub's `DocumentDescriptor`
carries the same pair (`artifact_kind`, `artifact_schema`). They are two vocabularies of one
document, and `DocumentIndexEntryV1::validate`'s grammar settles which is which:
`canonical_dialect_artifact_kind` admits `s.` + one or two lowercase-kebab segments, i.e. exactly
`s.gis.gismap`. `gis.map` could never be an artifact KIND under that grammar.

So the canonical spelling is `s.gis.gismap` for the kind and `gis.map` for the pack schema, and the
defect was in the GATEWAY: `semio://artifact/{id}/schema`'s hub arm published only
`artifact_schema`, and `artifact_open`'s structured result put that value in a field it calls
`kind` — so an agent matching `capabilities_search`'s `artifactKind` (`s.gis.gismap`) against
`artifact_open`'s `kind` (`gis.map`) could never match, for any hub document.

| file | change |
|---|---|
| `🌉️mcp/🏠️workspace/🦀️.rs` (hub arm of `read_artifact_resource`, `Some("schema")`) | publishes `artifactKind` beside `schema` — both, neither collapsed into the other |
| `🌉️mcp/🗿️artifact/🦀️.rs` | `resolve_artifact_kind_id` beside `resolve_artifact_schema_id`, both over one `artifact_schema_resource_field` helper; `artifact_open` answers `artifactKind` |
| `🌉️mcp/🧬️schema/🦀️.rs` | `artifact_open_output_shape` declares `artifactKind` |

### 5.2 "descriptor index is refreshing" must not kill the gateway at startup ✅

`HeadlessWorkspace::open_hub` was called ONCE at `🌉️mcp/🦀️.rs:825`, so any retryable binding state —
a descriptor index mid-refresh, a directory stream that has not re-dialled — exited the process
before `initialize`. `open_hub_workspace_with_retry` (`🌉️mcp/🦀️.rs`) re-attempts **only** errors the
hub itself marks `retryable`, bounded at `HUB_OPEN_RETRY_ATTEMPTS = 6` with a linear
`HUB_OPEN_RETRY_STEP_MS = 500` step (10.5 s total), and returns a non-retryable error — bad
credential, non-member space, version boundary — on its first occurrence untouched. The bound is
what makes it a retry rather than a hang.

### 5.3 The hub must not ABORT on its 30 s trusted-catalog budget 🚧 (compiled; needs coordinator hub rerun)

M8 measured this twice: `configured_artifact_authority` gives the whole trusted-catalog load a fixed
30 000 ms `OperationContext`, and the `?` on it propagated `AuthorityError::DeadlineExceeded` out of
the boot function, so the hub **exited** — the opposite of every other gate, which names a reason in
`blocked_by` and still binds. A machine that was merely busy therefore looked exactly like a corrupt
data root.

`configured_artifact_authority` now answers a three-way `StartupArtifactAuthority`
(`🌎️hub/🏗️bootstrap/🦀️.rs`) — `Configured` / `Absent` / `BudgetExceeded` — because "the load ran out
of budget" and "this data root has no catalog" are different facts an operator is owed the
difference between. Only `DeadlineExceeded`/`Cancelled` become `BudgetExceeded`; **a structural
refusal (corrupt catalog, missing provider, signature mismatch) is still an `Err` and still aborts
the boot**, because that one IS a statement about the data root. The budget is
`TRUSTED_CATALOG_STARTUP_BUDGET_MS`, a named constant with its measurement in its own doc, instead of
a literal. `artifact_authority_closed_reason(native, budget_exceeded, pointer_present)` is the reason
predicate, extracted so it can be pinned, and it publishes a fourth, distinct reason
`trusted-catalog-load-exceeded-its-startup-budget` — deliberately NOT
`trusted-catalog-pointer-present-but-not-loadable`, which would read as if something had been found
wrong with the catalog.

Law: `a_trusted_catalog_budget_overrun_closes_the_gate_and_never_claims_the_catalog_is_unloadable`
(`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`) — the outcome carries no authority, all four reason corners, the
reasons are pairwise distinct, and `blocked_by` carries `artifactAuthority` with the new reason while
`status` is `not-ready` (i.e. the hub still binds and still reports).

**Verification:** `cargo check -p semio-hub --all-targets` → **0 errors**
(`🗑️generated/m9-hub-check-2.txt`). Rule 26 forbids me the hub binary, so the law's verdict and the
served `/readyz` have NOT been observed. **Needs coordinator hub rerun.**

## 6. Gates, all executed this slice

| gate | M8 | M9 | note |
|---|---|---|---|
| `hub-agent-participant-check` | 14/17 | **14/17** (`m9-gate-participant-1.txt`) | same count, two of the three reds have MOVED: rows 11/12 no longer fail on `repo root not found` but on §2.4's world boundary. Row 0c (`features.mcpWorkspace is true`) still needs the coordinator's hub rerun of M8 §4 |
| `client-e2e` | 35/38 | **15/17** (`m9-client-e2e-1.txt`) | the run ABORTS at `artifact_create` on §2.4, so 21 rows never execute — the 38-row shape does not exist on this tree. Reds: `os: artifact_create` (§2.4) and `os: capability catalog health` (**PZ1's**, counted separately) |
| `live-agent-loop-check` | 20/20 | **8/20** (`m9-gate-live-agent-1.txt`) | run against C5's shell on 6190, read-only. Row (e3) fails with the identical `no exported instance named semio:framework/codec@1.0.0`; rows (f1)–(f8) fail on `plugin note … has no open instance in the attached shell (the shell reports 3 open instance(s))`, i.e. 6190 is another slice's shell with a different instance set. **Not a regression from this slice, and I do not claim it is clean either** — see §8 gap 3 for exactly what is and is not established |

`hub-agent-participant-check`'s three reds, verbatim:

```
FAIL  0c features.mcpWorkspace is true … — mcpWorkspace=false openPlan=true
FAIL  11 action_prepare reaches a guest the HUB authorized — gis.s.gis.gismap@1/*#editor.addFeature:
      {"code":"INTERNAL","message":"instantiate: wasmtime: no exported instance named `semio:framework/codec@1.0.0`"}
FAIL  12 action_invoke commits the agent's edit — action_prepare minted no handle
```

Unit law executed: `a_hub_open_retries_only_what_the_hub_marks_retryable_and_always_terminates`
→ **ok** (`m9-mcp-law-1.txt`).

## 7. Live proof on hub 7621 — and what was NOT proven

Hub **7621** (C5's, data root `jc1-boot`, catalog generation `8086b61f336e08b5…`) answered
`/readyz` `"status":"ready"`, `runId 57728fd47c980fd40c77fc0de4a6a31b`, at slice start and
throughout. **It was never restarted by me** and its catalog was never republished by me.
Space `01a0c314-e41f-780d-a980-3adda40ca9f7`, document `artifact-0954e2d10d8fff9605f101b0dba34f3b`.
A fresh delegation was minted for this slice — `agent:01a0c336-b6b4-7861-8c0c-55e8abb36da3`,
audience `edit`, credential `0600` — and revoked by the gate at the end (`HTTP 204`).

**Proven live** (§2.3): the agent authenticates as its own principal, sees the space's documents,
opens one, reads its bytes off the verified canonical pair, reads its artifact KIND, finds a
mutation verb typed against exactly that kind, and drives the gateway to fetch 47 466 541 authorized
component bytes from the hub, verify their SHA-256 against the manifest, and compile them.

**NOT proven, and not claimed** (§2.4): no `Commands` frame reached the hub, so the hub log shows
none and the ledger has none (`head_seq 0`, `commit_seq 0`, unchanged). Therefore:

- **the browser half was not run.** A human browser context attached to the same document could only
  have shown an unchanged document and a roster without an agent row. Taking that screenshot and
  presenting it would have been theatre, so no playwright run against 6191 was made and no
  screenshot is offered.
- **the second half — a second agent invoke while the human edits, convergence with an agent
  writer — was not run**, for the same reason: it is strictly downstream of one agent edit crossing.

## 8. Honest gaps

1. **The agent still has not EDITED a hub document.** `action_prepare` from a hub-bound gateway now
   gets all the way to `wasmtime` instantiate and fails there on §2.4's world boundary. The
   remaining work after that boundary clears is M8 §5.3 steps 3 and 4, both untouched here:
   `AppCommand::LoadDocument` of the canonical pair so the guest's session document IS the hub
   document rather than the plugin's genesis, and `…/socket-grants` + the binary document socket as
   the store's backbone so the committed ops leave as a `Commands` frame. Step 4 still carries M8's
   named unknown: the hub binding pins `PersistenceBinding::Hub { surface: Some(PROBE_SURFACE_ID) }`
   (`🌉️mcp/🏠️workspace/🦀️.rs:501`) where the document's real surface is
   `s.gis.gismap@1/*#editor`, and `ArtifactHost::open` needs a registered native codec for
   `gis.map`, which this process does not have.
2. **§2.4 blocks the whole tree and is a peer's in-flight change, so it is not mine to land.**
   TC3b's `export codec;` in `world actor` is uncommitted in the working tree together with its
   guest-side export macro (`🔌️plugin/🦀️.rs:39278`, `pack_schema_hash`/`genesis`/`print_mirror`/
   `apply_ops`). It clears when every plugin component is rebuilt from a post-TC3b tree AND, for the
   hub lane, the trusted catalog is republished — **7621's catalog is C5's and I did not touch it**.
   I deliberately did not rebuild plugin wasm: TC3b is still editing the world it would be built
   against.
3. **`live-agent-loop-check` was NOT shown unchanged.** It measured 8/20 (§6) and I cannot honestly
   call that a clean regression check. What IS established: rows (f1)–(f8) fail on the attached
   shell's open-instance set, on C5's 6190 which has three other instances open and no `note`, and
   row (e3) fails on §2.4; both causes are independent of this slice. What is NOT established: a
   20/20 run, which is unreachable on this tree because the gateway's native lane instantiates
   `note` locally and hits §2.4. I did not start a private `s` serve on 6196 because it would not
   have changed (e3).
   The argument that the folder lane is behaviourally unchanged by this slice is structural and
   checkable by reading: `PluginComponentSource::Repo`'s arm performs registry → descriptor →
   `resolve_plugin_wasm_path` → `std::fs::read` → `from_component`, the same operations in the same
   order the old `PluginArtifactChannel::new` did, and `find_repo_root().ok().map(Repo)` has exactly
   the availability the old `find_repo_root().ok()` had.
4. **`client-e2e`'s "the snapshot shows the mutation" is untouched** (§4) — the defect could not be
   reproduced this session. §4 narrows it to `PluginApp::document_pack()` inside the guest and names
   the one measurement that would settle it.
5. **Two hub changes are compiled and never observed live** — M8 §4's derived `features.mcpWorkspace`
   (still `false` on 7621, gate row 0c) and this slice's §5.3. Rule 26 forbids me the hub binary.
   **Needs coordinator hub rerun**, and a rerun of `cargo test -p semio-hub`, where both laws' verdicts
   live.
6. **No unit law covers `HubPluginComponents::resolve` or
   `NativeHubBindingDriver::fetch_execution_target_component`.** Both need a hub transport to
   exercise; they are proven by the live rows in §2.3 (the fetch, the hash check and the compile all
   demonstrably ran) and by nothing else. This is the same gap M8 §8.8 records for
   `read_hub_canonical_pair`.
7. **The MCP hub transport's byte budget is now 80 MiB/min per package**, up from 16 MiB. It is
   derived from the hub's own component ceiling, so it cannot drift from what one authorized
   component costs — but it is a real widening of a real quota and a reviewer should see it as one.
8. **`.mcp.json` still cannot express a hub-bound agent** (M8 §8.6, unchanged): it hard-codes
   `--folder .`, and `--folder`/`--hub` are mutually exclusive.

## 9. Files changed

**Modified — directory client**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs` —
  `DirectoryClient::document_execution_target_component` + the
  `DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES` import

**Modified — os-mcp**
- `🌉️mcp/🦀️.rs` — `open_hub_workspace_with_retry`, `hub_open_retry_backoff_ms`,
  `HUB_OPEN_RETRY_ATTEMPTS`/`HUB_OPEN_RETRY_STEP_MS`; the `open_hub` call site
- `🌉️mcp/🏠️workspace/🦀️.rs` — `PluginComponentSource`, `HubPluginComponents`,
  `HeadlessWorkspace::plugin_components` (+ the field, set to the `Hub` arm in `open_hub`),
  `open_plugin_artifact_channel` now takes a source, `PluginArtifactChannel::from_component` and the
  `entry: PluginRegistryEntry` → `plugin_id: String` field, `RoutingArtifactChannel`'s
  `repo_root` → `components`, `hub_driver` behind an `Arc`, and the hub `schema` resource publishing
  `artifactKind`
- `🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` — `NativeHubBindingDriver` holds the `DirectoryClient`,
  `fetch_execution_target_component`, `HUB_EXECUTION_TARGET_COMPONENT_TIMEOUT_MS`, and the derived
  per-package byte budget
- `🌉️mcp/🗿️artifact/🦀️.rs` — `resolve_artifact_kind_id` + `artifact_schema_resource_field`;
  `artifact_open` answers `artifactKind`
- `🌉️mcp/🧬️schema/🦀️.rs` — `artifact_open_output_shape` declares `artifactKind`
- `🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs` — one widened `RoutingArtifactChannel::new` call site
- `🌉️mcp/🧪️tests/🔬️quick/🦀️.rs` — the hub-open retry law

**Modified — hub** (all unverified live; needs coordinator hub rerun)
- `🌎️hub/🏗️bootstrap/🦀️.rs` — `TRUSTED_CATALOG_STARTUP_BUDGET_MS`, `StartupArtifactAuthority` (+
  `configured`/`is_none`), `configured_artifact_authority`'s new return,
  `artifact_authority_closed_reason`, and the boot call site
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — the budget-overrun law + 2 widened call sites

**New — ticket artefacts**
- `🐍️m9-agent-edit-probe.ts` — the live agent-edit probe (artifactKind match + before/after snapshot)
- `🗑️generated/m9-*` — `exec-target-sizes`, `hub-component-interfaces`, `delegate`,
  `agent-credential.json`, `edit-run1`, `mcp-check-1`, `mcp-build-1`, `mcp-law-1`, `hub-check-1`,
  `hub-check-2`, `client-e2e-1`, `gate-participant-1`, `gate-live-agent-1`

**No `.vscode/launch.json` row** — this slice registered no new runnable command.

**Compile gates:** `cargo check -p semio-framework-os-mcp --all-targets` **0 errors**;
`cargo check -p semio-hub --all-targets` **0 errors**; `cargo check -p semio-framework-os-kernel
--lib` **0 errors**; the gateway `build` exit 0 and staged. All under the private
`CARGO_TARGET_DIR=…/target-m9`.
