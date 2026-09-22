# S12 — 30/35 inside `s`, and the hub-document journey carried to the hub itself

**Headline, all measured.** The 35-kind sweep goes **26 → 30/35** (`space`, `lowpoly`, `shooting`,
`flow`), and `space`'s "flake" turns out to be a probe pressing a toggle before the spawned app's
actions existed. The hub-document journey gains **four root fixes in a row** and now reaches the hub:
the shell MOUNTS the space index instead of spawning it, every one of the route's ten silent gates
raises a localized notice, the index binds to its hub scope for the first time in any hub root, and
the shell stops throwing away every artifact-creation catalog the hub can send — after which
`createArtifact` is **accepted by the hub** and fails only in the hub's own genesis materialization
(HC1's fault, reached from a human gesture for the first time). S11 §3.3b's **95 exposed editors**
are cured by one generic framework default, S10 §4.2's regression law is finally **written in a shape
that runs**, and 🎬️sequence is **207/0**.

Slice S12, session 8, 2026-09-22.

Slice S12, session 8, 2026-09-22. Inherits `📓️s11-thirty-five-kinds-inside-s.md` (26/35, §3.3b the
91 exposed editors, §5 the silent `replayShellCommand` gate, §7 the `space` rail flake, §8 sequence
205/2), `📓️s10-…` §2.9/§4.2/§7, `📓️tc3e-…` §0 (hub **7651**, stdio/gis/note catalog).

Status legend: **measured** = this slice ran it and captured the output; **unverified** = read only.

## 0. Infrastructure this slice inherits (measured at 21:14)

| what | value | state |
|---|---|---|
| serve | `s` react dev on **6071** (bun pid `96947`) → hub `7641` | `200` — inherited from S11, not restarted |
| hub | **7641**, data root `🌐hub/s10-boot`, `os-hub` pid `96468` | `503` (`artifactAuthority: trusted-catalog-never-published`, S11 §0.1 — not a fault) |
| hub | **7651**, TC3e's three-package stdio/gis/note catalog, `os-hub` pid `68878` | `200` — **never restarted by this slice** |
| load at start | `21:13 up 5 days, load averages: 13.54 23.88 21.24` | |

## 1. The nine open kinds — **26 → 29/35 measured**, three more named at their root

S11 left nine red: `energy`, `flow`, `lowpoly`, `shooting`, `sourcing`, `space`, `stdio`, `trinity`,
`writer`. All re-measured on S11's own pair (serve **6071** → hub **7641**), with the sweep's rail
fix of §5.

| kind | S11 | S12 | evidence |
|---|---|---|---|
| `lowpoly` | FAIL `[0,1,1,1]` | **PASS** | `addPrimitive` `edits [0,1,0,1]`, `applied [0,1,3,5]`, `…entry.1:Add Primitive↶`, 0 refusals — S11 §3.4b's `applied` fix has reached the guest |
| `shooting` | FAIL `[0,1,1,1]` | **PASS** | `addShot` `edits [0,1,0,1]`, `applied [1,2,4,6]`, `…entry.2:Add Shot↶`, 0 refusals |
| `space` | not scored | **PASS** | §5 — `touchArtifact` `edits [1,2,1,2]`, rail 24 rows |

(`🗑️generated/s6-sweep-s12{a,c}.txt`.) And on the same pair, with no change of this slice's:

| kind | S11 | S12 | evidence |
|---|---|---|---|
| `flow` | FAIL `[0,0,0,0]` (S11 §3.5, "a regression in files this slice may not touch") | **PASS** | `addWidget` `edits [0,1,0,1]`, `applied [1,3,6,8]`, rail 27, 0 refusals (`🗑️generated/s6-sweep-s12d.txt`). The peer session's own rebuild cured it; **nothing of `✏️s/🔌️plugins/🌊️flow/**` was touched by this slice** |

### 1.1 **30 / 35 measured.** The five that remain, each named at its declaration

| kind | measured this slice | reading |
|---|---|---|
| `energy` | `create-zone` **offered**, dispatched, `filled []`, **no refusal**, `applied [5,5,5,5]`, `mutated false` | Sharper than S11 §3.4, and a NEW measurement: `rename-zone` reports `filled: []` because the seeded model has no zone id to harvest, so this slice re-pinned the sweep to `create-zone` — the same editor's `Artifact`-lane verb (`🔋️model/…/✏️editor/🦀️.rs:1854`) whose four fields ALL have declared defaults (`…:303`, `name`/`volumeM3`/`multiplier`/`conditioned`). It still journals nothing and still refuses nothing. **An `Artifact`-lane verb that needs no argument and produces no edit is a product question, not a probe gap** — named with its file:line, not fixed |
| `trinity` | rail's 23 rows dumped; the pinned `setParameter` is **not offered at all** (`knownVerbOffered: false`); `patchNodes`, `runQuery`, `loadExampleQuery`, `paste` all `mutated true`, `edits [0,0,0,0]` | The pin is stale. `textEdit` is on the **`WindowConfig`** lane (S11 §3.4, `🔌️jack/…/✏️editor/🦀️.rs:384`), which `#s-checkin` is right never to count; `patchNodes` is `Artifact` but carries a patch payload no probe can synthesize. **Left red honestly** rather than re-pinned to a verb that would pass for the wrong reason |
| `sourcing` | rail 16 rows; only `stockFromCatalogue` is a candidate and the pinned `curationSetCount` is not offered | Unchanged from S10 §2.5 group 6: **`HostOnly` by declaration**, so it publishes no store lane and no `#s-checkin` count can move |
| `stdio` | rail **0** rows this run (11 in S11's) | S11 §3.6a settled the declaration side: `grep` for `ActionDefinition::new`/`.mutation(` across **all 35** `🗄️stdio` artifacts returns nothing — no stdio artifact declares an action of its own, so there is no rail-reachable document verb to pass with, whatever the rail count |
| `writer` | rail 16 rows; `paste`, `formatDocument`, `lintDocument` all `edits [0,0,0,0]` | S11 §3.6b: every real document operation (`textEdit`, `setText`, `setSnapshot`, `commitRename`) is minted by `writer_hidden_operation`, i.e. `in_palette: false`, so none of them reaches this rail |

**So the honest number this slice leaves is 30/35**, +4 over S11, with the five survivors split into
**three declaration cases** (`stdio`, `writer`, `sourcing` — the kinds have no rail-reachable document
verb, which is a product decision to take, not a bug to fix) and **two open questions** (`energy`'s
argument-free `create-zone` that journals nothing, `trinity`'s missing `setParameter`).


## 2. The hub-document journey inside `s` — **four roots removed, the chain now reaches the hub**

All measured on serve **6072** (this slice's own, `S_HUB_URL=http://127.0.0.1:7651`, wrapper `8543`,
vite `8555`) against TC3e's three-package hub **7651**, signed in as `user1@semio.dev`.
Probes: `🐍️s12-hub-document.mjs`, `🐍️s12-kind-chooser-diagnose.mjs` (both new, permanent).

### 2.0 The headline, in one line

`create artifact` now travels **rail → guest → host route → backbone worker → hub**, the hub
**accepts** it, and the only thing that fails is the hub's own genesis materialization:

```
[creation:accepted] "Creating artifact S12 Hub Document 7873 — The creation request was accepted."
[creation:failed]   "Creating artifact S12 Hub Document 7873 — Artifact creation failed."

🗑️generated/tc3e-hub-7651.txt (hub 7651's own log, same second):
{"level":"error","event":"server.artifact.creation","outcome":"failed",
 "detail":"creation 01a0ca76-a13b-7c68-82ec-1e136686be57 49f9599027dd2620adc78951738a1048:
           genesis materialization failed: trusted artifact codec Input failed: epoch deadline exceeded"}
```

That is **HC1's fault, reached from the shell for the first time** (C8 saw it only from its own
bootstrap, on gis; this is the `s.note.note` kind, driven by a human gesture). Every link in front of
it is this slice's, and each was a separate root.

### 2.1 The single-origin gate — the shell must MOUNT the index, and the refusal is now visible

S11 §5.4 left `[os-shell] replayShellCommand: space artifact creation requires one mounted Space
index` as the last link, refused through `console.warn` alone.

**(a) The journey's own lane.** The shell mounts the space index on a bare `/spaces/{id}` route
(`🏛️ShellHost/🟦️.tsx:6602`), and the gesture that navigates there is the space row in the hub
workspace's Space Browser (`🏘️SpaceBrowser/🟦️.tsx:267`, `li[data-space-id] button`). Driving THAT
instead of a palette spawn, the gate is silent on every run since:

```
open space → 01a0ca76-a13b-7c68-82ec-1e136686be57 ok
uri /spaces/01a0ca76-a13b-7c68-82ec-1e136686be57   rail (24) createArtifact, deleteArtifact, …
shellLines []            ← the sentence S11 measured never appears again
```

**(b) All TEN gates on that route are now user-visible, en + de.** New unit
`🏛️ShellHost/📣️replay-refusal/🟦️.ts` (no shell i18n import — the `🎯️input-ledger` rule): a closed
seven-reason set with an `{en, de}` label each and a `shell.replayShellCommand.<reason>` code, and
one `refuseReplay(reason, line, …rest)` closure in the route that keeps the console line AND raises
`showTransientNoticeRef.current(…, "warning", code)`. Ten `console.warn`-only gates → ten notices
(`🐍️s12-replay-refusal-patch.py`, anchored + idempotent, 12 edits).

### 2.2 The space index had never bound to a hub — in ANY hub root, ever (root-fixed)

With the index mounted, the browser's network log reads

```
404 POST /_semio/hub/spaces/{id}/documents/index/open-plan   (×5, retried)
     (no GET …/artifact-creations at all)
```

`runDocumentOpeningAttemptV1` runs `socket()` **before** `attach()`
(`🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts:123-126`), so the 404 rejected the opening before
`attach` could send `directory-scope-open` **or** `space-artifact-creation-catalog-open`. The 404 is
`DocumentOpenPlanErrorCodeV1::NotFound` from `get_document_descriptor`
(`🌎️hub/🏗️bootstrap/🦀️.rs:3091`) — and the reason is structural, measured across every hub root on
this machine:

```
for d in .🧬semio/🌐hub/*/: strings $d/directory.db | grep -c '"s.space"'
c8-boot 0 · db2-pg 0 · db3-neo4j 0 · db3-pg-nae 0 · db3-pg 0 · db4-fs 0 · db4-neo4j 0 · db4-pg 0
gm1-boot 0 · hc1-boot 0 · hs1-boot 0 · jc1-boot 0 · rb1-prod 0 · s10-boot 0 · tc3c-boot 0
tc3e-boot 0 · tc3e-hub 0
```

**Seventeen roots, zero space-index documents.** The hub has no space-index concept at all (`grep -rn
'"index"' 🌎️hub --include='*.rs'` → one unrelated hit): the index is the SHELL's projection of a
space's directory, with its own schema `s.space`
(`🪐️space/🗿️artifacts/🪐️space/🦀️.rs:24`), not an artifact the hub stores. Demanding a hub document
socket for it could therefore never succeed.

**Fixed** at `🏛️ShellHost/🟦️.tsx` (`🐍️s12-space-index-binding-patch.py`): the index keeps its hub
BINDING — `scope`, the hub runtime key and the catalog/presence lanes are all keyed on it — and only
the worker's own document `open` drops it (`workerBindings`, and `expectsSocketActor` reads that).
Measured effect, immediately:

```
200 GET /_semio/hub/spaces/{id}/artifact-creations      ← first ever
[data-semio-artifact-creation-catalog="ready"]  "The available artifact kinds are current."
```

### 2.3 The shell dropped EVERY creation catalog the hub can send — a `*` the wire grammar forbids

The catalog was `200`/`ready` and the `Kind` chooser was still `<button id="kindChoice" role="combobox" … disabled>`
with zero options. `renderStagedArgControl` disables a select whose options are empty
(`🛠️ShellHelpers/🟦️.tsx:4119`), and the options come from the projection, which was empty because
`captureSpaceArtifactCreationCatalogAuthorityV1` had no catalog to capture.

The hub's answer, read directly with a minted session
(`🗑️generated/s12-creation-catalog.json`):

```json
{"schema":"semio.hub.space-artifact-creation-catalog/v1","kinds":[
 {"kindId":"s.gis.gismap","schema":"gis.map","dialect":{"artifactKind":"s.gis.gismap","standard":"1","subset":"*"},…},
 {"kindId":"s.note.note","schema":"note.document","dialect":{"artifactKind":"s.note.note","standard":"1","subset":"*"},…}]}
```

`parseSpaceArtifactCreationCatalogV1` validated **every** dialect component with
`workerWireCreationIdentityV1` (`🛍️products/💻️os/🟦️.ts:904`),
`/^[A-Za-z0-9][A-Za-z0-9._:\/-]*$/` — which rejects `"*"`. But `"*"` is `SubsetId::ANY`
(`🚪️io/🧬️schema/🦀️.rs:41`), *the* unconstrained base subset every standard carries and the subset of
every kind this repository ships. So the main thread **threw on decode of every catalog message the
hub can ever produce**, the shell kept `null`, and the chooser was disabled on every space of every
hub — silently, because a decode throw is not a refusal.

**Fixed** with `workerWireDialectComponentV1` (new, one line: `value === "*" ? value : identity`),
applied to `dialect.standard`/`dialect.subset` in the catalog parser **and** to
`ready.parentDialect`'s two components in `parseSpaceArtifactCreationStatusV1`, which is the very
next link and had the identical defect (`🐍️s12-wildcard-subset-patch.py`). Measured:

| | chooser |
|---|---|
| before | `name=ok kind=no-option options=[]` (`🗑️generated/s12-hub-document-b.txt`) |
| **after** | **`name=ok kind=ok options=["Editor","Editor"]`** (`…-c.txt`) — the hub's own two kinds |

**Named, not fixed (hub-side, out of this slice's scope):** the hub labels *both* kinds `"Editor"` —
it sends the app ROLE label, not the kind's own name, so the chooser is unreadable to a human. The
probe addresses options by the catalog's own ascending-`kindId` ORDER, which
`parseSpaceArtifactCreationCatalogV1` enforces.

### 2.4 What is now proven end to end, and the two links that are not this slice's

| link | state |
|---|---|
| sign in on 7651 | **ok** |
| space browser lists the user's spaces | **ok** (5 rows, incl. one this slice created through the shell) |
| open a space ⇒ the shell MOUNTS the index as its base session | **ok** — `uri /spaces/{id}`, rail 24 rows |
| the hub's artifact-creation catalog reaches the shell | **ok** — §2.2 + §2.3, first ever |
| `createArtifact` stages both args and the execute row is enabled | **ok** — `…createArtifact.execute\|button\|disabled=false` |
| the guest emits `ReplayShellCommand`, the host's four gates pass, the worker POSTs | **ok** — `creation:accepted` |
| **the hub materializes the genesis** | **FAILS** — `genesis materialization failed: trusted artifact codec Input failed: epoch deadline exceeded` → **HC1's** |
| the index LISTS the space's documents | **FAILS** — `404 POST /_semio/hub/directory/spaces/{id}/documents/index/socket-grants`: `issue_scoped_directory_socket_grant` demands a document descriptor for the scope (`🌎️hub/🏗️bootstrap/🦀️.rs:3481-3485`), and the space index has none (§2.2). **Hub-side; `🌎️hub/**` is outside this slice's brief.** This is why the table has rendered its header and zero rows on every hub in this ticket |

Both remaining links are in `🌎️hub/**`. Nothing here is claimed as working that was not measured.


## 3. The 95-editor presence exposure — **one framework root, fixed, checked green**

S11 §3.3b counted it and deliberately did not attempt it: **230 of 298 app/editor/viewer files
declare a presence type and no retirement owner**; 134 are viewers, which the viewer wrapper already
defaults (`🔌️plugin/🦀️.rs:8003`, `…or_else(|| Some(no_presence_local_root_retirement_factory()))`),
leaving **95 editors exposed**. Four of them were found one at a time by four different slices in two
days (🏠️home S10 §2.9a, 📖️playbook S10 §2.8c, ⚙️playbook-module-procedural and 🪐️space-index S11
§3.2/§3.3). `PresenceStore::local_read` fails closed without a live owner
(`🏪️store/🦀️.rs:4716`), so only an app whose commands actually read local presence refuses — which is
why the other 91 are quiet rather than correct.

**S11's blocker was real but avoidable.** A `Presence`-implemented default would need a new bound on
`ArtifactApp::Presence` and a marker impl on all **26** distinct real presence types in the repo
(counted: `grep -rh "type Presence = " ✏️s 🧰️framework | sort -u`). It is not needed: the framework
already owns a fully generic bounded root retirement for the sibling lane —
`bounded_transient_root_retirement_factory<P>() where P: ArtifactDsl + Send + Sync + 'static`
(`🫧️transient/🧵️publication/🦀️.rs:224`) — and the presence lane has the identical shape (one
displaced local root, retired in grant-sized steps, priced by the payload's own DSL print).

**Fixed** with a presence twin and two changed trait defaults:

| file | change |
|---|---|
| `🔌️plugin/👥️presence/♻️retirement/🦀️.rs` | new `bounded_presence_root_retirement_factory<P>()` + `BoundedPresenceRootRetirement{,Factory}<P>`, generic over any presence payload |
| `🔌️plugin/🦀️.rs` | `ArtifactApp::build_presence_local_root_retirement_factory` and `…peer_retirement_factory` default to `Some(bounded_presence_root_retirement_factory::<Self::Presence>())` instead of `None`; the export list carries the new owner |

`build_presence_store_disposer` is deliberately **left at `None`**: the refusal that gates the 95 is
`local_read`'s missing FACTORY, and the disposer is the close path, where a wrong generic terminal
would be a worse bug than the one being fixed. The `Presence` publication LANE is not silently
opened either — it additionally requires `build_presence_store_one_item_preparation_factory`, whose
default stays `None` (`🔌️plugin/🦀️.rs:22766`).

**Verified**

```
cargo check -p semio-framework-plugin --lib          exit 0, 59 warnings   (22:23:27, 7.9 s)
cargo test  -p semio-framework-plugin --lib          829 passed; 2 failed  (private target-s12)
```

Both failures are attributed, neither is this slice's: `image_kit_pages_a_composite_larger_than_one_ui_text_out_of_the_doc`
is a peer's **brand-new law in a file carrying 30 uncommitted insertions**
(`git diff 🧪️tests/🔬️app-window-kits/🦀️.rs` → `+ async fn image_kit_pages_a_composite…`) and is red
in both the parallel and the serial run; the other differs between the two runs
(`a_conflicting_declaration_leaves_zero_rows_behind` in parallel, the 2 ms
`tool_run_overlay_append_per_tick` timing law serially at load 36–44) — a parallel-registry flake and
a timing law under fleet load, the class `feedback-worker-timing-laws-jitter-under-peer-load` names.

**Honest gap:** the 95 editors are cured **in source and by compilation**, not at runtime. Every
guest links `semio-framework-plugin` statically, and the activate verb's staleness view is keyed on
`✏️s/🔌️plugins/**`, so nothing marks stale — reaching them needs all 60 components rebuilt, which
this slice did not take the mutex for (§7).


## 4. S10 §4.2's regression law — **written in the third shape, and it RUNS**

S11 §3.4b fixed the predicate and could not write its law: `compositeEdit` logs a row with
`edit_id: None` (so it is not a proxy for a parent-lane row), and a purpose-built `MultiLaneEdit`
verb is refused before it reaches the store (`interactive-job.missing-factory`) because a multi-lane
verb needs a retained factory whose `PUBLICATION_CONTRACTS` name both lanes, and the suite's proof
roster is pinned by a fixture count. Both attempts were reverted.

**The third shape is to name the predicate.** `build_history_view`'s one-line expression becomes a
pure function beside the impl (`🔌️plugin/🦀️.rs`, in `pub mod app`):

```rust
pub fn history_row_applied_v1(has_parent_edit: bool, document_applied: bool, config_applied: bool, child_applied: bool) -> bool {
    if has_parent_edit { document_applied } else { config_applied || child_applied }
}
```

so the law drives **all eight lane combinations** with no app, no retained factory and no fixture
roster at all — `a_multi_lane_row_follows_its_parent_document_lane_after_an_undo`, in the same
contract suite, next to S10's own row.

```
cargo test -p semio-framework-plugin --lib -- lane_row --test-threads=1      (private target-s12)
test …::a_multi_lane_row_follows_its_parent_document_lane_after_an_undo ... ok
test …::config_lane_row_reports_itself_applied_so_the_host_can_count_it ... ok
test result: ok. 2 passed; 0 failed; 829 filtered out
```

The law pins both halves: a parent-lane row with a live CONFIG edit and a retracted document edit is
**not** applied (the 💠️lowpoly / 🎥️shooting regression), and a Config-only or Child-only row **is**
(S10 §4.2's cure, kept intact). Its runtime half is §1's two PASSes.

**One transcription trap, recorded because it costs a whole build.** The function first landed
between `#[cfg(test)] #[path = "…"]` and `mod interaction_selection_laws;`, so both attributes
attached to the FUNCTION: `E0583 file not found for module interaction_selection_laws` and
`E0425 cannot find function history_row_applied_v1` — one edit, two unrelated-looking errors.


## 5. The `space` "rail flake" is NOT a race — the probe pressed the chip too early (root-fixed, PASS)

S11 §7 measured the space index's Actions rail rendering **zero rows in 4 of 6 consecutive runs on
one build** and left it as a race. It is deterministic, and both halves are measured.

**(a) The chip is a clean toggle, and the rail has 26 rows.** `🐍️s12-space-rail-diagnose.mjs` (new)
spawns the index and presses the ONE engagement chip four times, dumping the pane's own
`data-folded` each round:

```
round 0: rows 0  → 26   chip data-folded="true"
round 1: rows 26 → 0    chip data-folded=absent
round 2: rows 0  → 26   chip data-folded="true"
round 3: rows 26 → 0    chip data-folded=absent
```

Perfect alternation — so `railRows: 0` was never "the app has no rail".

**(b) The press was landing before the spawned program's actions existed.** A spawned window mounts
its engagement chip before `activeSpawnedApp` resolves, and `windowActionPaneNode` returns
`undefined` for an app whose `resolveWindowActions(...).filter(inPalette)` is still empty
(`🛠️ShellHelpers/🟦️.tsx:4490`). The pane then renders with the Engagement and NO action rows — and
because the chip is now *un*folded, every later round of the probe's own loop had nothing left to
press. Reproduced twice at 22:07 and 22:1x (`🗑️generated/s6-sweep-s12{a,b}.txt`, `railToggles: 1`,
`railRows: 0`, 25 s of polling) against the diagnose's 26 rows after a 12 s settle on the same build.

**Fixed in the shared sweep** (`🐍️s6-all-kinds-sweep.mjs`), two rules, both of which the History
panel already had and the rail did not:

1. the chip is a TOGGLE — press only a chip whose pane reads `data-folded="true"`
   (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10163`), never every chip blindly, with the old
   "press what has not been pressed" rule kept as the fallback for a build that publishes no
   `data-folded`;
2. wait for the chip to EXIST and then let the spawned app definition land (2.5 s) before the first
   press.

**Measured, same build, immediately after** (`🗑️generated/s6-sweep-s12c.txt`):

```
space  PASS  railRows 24  verb touchArtifact  edits [1,2,1,2]  applied [6,7,9,10]
       lastLedgerRow "framework.history.entry.7:Touch Artifact↶"   refusals []   faultCount 0
```

The same fix is in `🐍️s12-hub-document.mjs`. **`space` is scored PASS for the first time in this
ticket.**


## 6. 🎬️sequence — **207 / 0**, and the `steps:in` builder is a PEER's in-flight work

S11 §8 left `semio-s-artifact-sequence-sequence` at **205 passed / 2 failed**, both
`interactive-job.missing-reserved-builder: media port 'steps:in' is registered but has no concrete
resumable importer`.

It is fixed, and **not by this slice**: `git diff` on
`🎬️sequence/🗿️artifacts/🎬️sequence/…/✏️editor/🦀️.rs` shows **236 uncommitted insertions** carrying
`SEQUENCE_IMPORT_TOOL_ID`, `SEQUENCE_IMPORT_PORT`, `struct SequenceImportJob` and
`fn build_reserved_tool_job` — the concrete resumable importer the refusal named, added after S11's
20:58 measurement. `VcsArtifactApp::dispatch_import_media` builds its emit exclusively from that job
(`A::import_media` is never reached on a mounted app), which is why the synchronous importer was
correct all along and every delivery still died.

**Measured by this slice** (private `target-s12`, 22:32–22:33):

```
cargo test -p semio-s-artifact-sequence-sequence --lib -- import_media
  import_media_steps_in_inserts_a_new_step_from_an_object_payload ... ok
  import_media_steps_in_wraps_a_bare_scalar_payload ... ok
  import_media_rejects_unknown_port ... ok

cargo test -p semio-s-artifact-sequence-sequence --lib
  test result: ok. 207 passed; 0 failed; 0 ignored; 0 filtered out
  grep -c "Dictionary ownership"  →  0
```

| | passed | failed | `final Dictionary ownership` panics |
|---|---|---|---|
| play session, before S11 | 200 | 7 | 5 |
| S11, after its `StepParams` cold boundary | 205 | 2 | 0 |
| **S12, with the peer's importer** | **207** | **0** | **0** |


## 7. Honest gaps

1. **The hub-document journey ends in TWO hub-side faults, both outside this slice's brief.**
   (a) `genesis materialization failed: trusted artifact codec Input failed: epoch deadline exceeded`
   — HC1's, now reproduced from a human gesture in the shell on the `s.note.note` kind rather than
   from a bootstrap on gis. (b) `404 POST /_semio/hub/directory/spaces/{id}/documents/index/socket-grants`
   — `issue_scoped_directory_socket_grant` demands a document descriptor for its scope
   (`🌎️hub/🏗️bootstrap/🦀️.rs:3481-3485`), and the space index has none in any hub root, so the
   scoped directory stream never opens and the artifact table lists nothing. **Until (b) is fixed no
   space index can show a document and no `openArtifact` can resolve one** — its guest handler needs
   the row (`🗿️open-artifact/🦀️.rs:22`, `indexed_artifacts.iter().find(...)`), so "a hub document
   OPENED inside `s`" is **not** achieved by this slice and no other lane exists (`os.open-artifact`
   is a replay action id, not a palette command). Stated as a gap, not as a result.
2. **The 95-editor presence cure is unproven at runtime** (§3). It compiles and the plugin lib suite
   is green, but every guest links the crate statically and the activate verb's staleness view is
   keyed on `✏️s/🔌️plugins/**`, so reaching the 95 needs all 60 components rebuilt. **No wasm mutex
   hold was taken by this slice** — the machine carried the peer play fleet's native test lanes and
   HC1 throughout, at load 30–44, and a 60-component rebuild on top of that would have starved them.
3. **`energy`'s `create-zone` is a NEW open question, not a closed one** (§1.1). An `Artifact`-lane
   verb whose four fields all have declared defaults dispatches with no refusal and journals nothing.
   Named with its file:line; not diagnosed further.
4. **The react renderer's own vitest project was not run to completion.** `@semio-tech/framework-os`
   is green (**363/363**, 22:34) and `typecheck` is **rc 0 with zero output** after every TS change
   here, but the renderer suite exceeded its own 300 s budget at load ~40 under
   `📜️script.ts test`, and a direct `vitest run` of the engine-contract file was still spinning after
   22 min and was killed **by pid** (wrapper `49429`, children `49432`/`49437`; the two `ppid 1`
   vitest forks from 21:59 and 22:12 predate every run of this slice and were left alone). No law in
   that suite names any line this slice changed (`grep` for `replayShellCommand`, `workerBindings`,
   `expectsSocketActor` and the gate sentences returns one unrelated hit), but that is reading, not
   running.
5. **`stdio`'s rail read 0 rows in the s12d chunk** where S11 read 11. The kind fails for a
   declaration reason either way (§1.1), but the count itself is not stable and was not chased.
6. **What was NOT touched.** `✏️s/🔌️plugins/🌊️flow/**` — read only; its PASS in §1 is a peer's
   rebuild, not this slice's change. No `🌎️hub` Rust was edited; both hub findings are named with
   file:line for their owners. `🎬️sequence`'s importer is a peer's uncommitted work, run and
   reported, not modified. No `git commit`/`stash`/`checkout` was run, `📌️important.md` and
   `🎫️ticket.json` were not touched, and no `🗑️generated` file this slice did not create was removed.
   Hubs **7641** and **7651** were left on their inherited pids and never restarted.

### 7.1 Infrastructure this slice started (kill by pid)

| what | pid | note |
|---|---|---|
| serve **6072** → hub **7651** (`📜️s12-serve.sh`, new) | wrapper `8543`, vite `8555` | the only lane on which the hub-document journey can be re-run; log `🗑️generated/s12-serve-6072.txt` |

Serve 6071 (bun `96947`), hub 7641 (`96468`) and hub 7651 (`68878`) are S11's and TC3e's and were
**not** restarted.


## 8. Files changed

**Product source (4 files, 5 root fixes):**

| file | change | § |
|---|---|---|
| `🧰️framework/…/🧱️elements/🏛️ShellHost/📣️replay-refusal/🟦️.ts` (**new**) | the closed seven-reason set behind `replayShellCommand`, each with `{en, de}` text and a `shell.replayShellCommand.<reason>` code | §2.1 |
| `🧰️framework/…/🧱️elements/🏛️ShellHost/🟦️.tsx` | (a) all **ten** fail-closed gates on that route raise a localized transient notice beside their console line; (b) the space INDEX keeps its hub binding but opens WITHOUT a hub document socket, which is what lets the scoped directory lane and the artifact-creation catalog open at all | §2.1, §2.2 |
| `🧰️framework/🛍️products/💻️os/🟦️.ts` | `workerWireDialectComponentV1` — a dialect's `standard`/`subset` on the creation-catalog wire admit the canonical wildcard `*`, in the catalog parser AND in the ready-status parser. Without it the shell threw on decode of **every** catalog any hub can send | §2.3 |
| `🧰️framework/…/🔌️plugin/👥️presence/♻️retirement/🦀️.rs` | `bounded_presence_root_retirement_factory<P>()` — a generic bounded presence-root retirement, the twin of the transient lane's own | §3 |
| `🧰️framework/…/🔌️plugin/🦀️.rs` | the two `ArtifactApp` presence retirement defaults become `Some(bounded_presence_root_retirement_factory::<Self::Presence>())` — one framework fix for the **95** exposed editors; and `build_history_view`'s `applied` predicate becomes the named `history_row_applied_v1` | §3, §4 |

**Laws (1 added, both green):**

| file | law |
|---|---|
| `🧰️framework/…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | `a_multi_lane_row_follows_its_parent_document_lane_after_an_undo` — all eight lane combinations; **2 passed** with S10's own row |

**Ticket folder (permanent probes and scripts, all new except the shared sweep):**
`🐍️s12-hub-document.mjs` (the journey, by navigation), `🐍️s12-kind-chooser-diagnose.mjs`,
`🐍️s12-space-rail-diagnose.mjs`, `📜️s12-serve.sh`, and the five anchored idempotent patch scripts
`🐍️s12-{replay-refusal,space-index-binding,wildcard-subset,presence-default,applied-law}-patch.py`.
`🐍️s6-all-kinds-sweep.mjs` (shared) gained the engagement-chip **toggle-parity** rule and its
pre-press settle (§5 — the single change that turned `space` from a coin flip into a PASS) and the
`energy` verb/args re-pin.

**Captures:** `🗑️generated/s12-*` and `🗑️generated/s6-sweep-s12{a,b,c,d,e}.txt`.
