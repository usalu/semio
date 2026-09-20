# CE1 — `client-e2e` determinism, the puzzle bound, and the rest of the descriptor sweep

Slice owner: CE1, session 5e (2026-09-20 22:16 →). Spec: `📓️a3b-descriptor-sweep.md` (whole),
`📓️wr4-typed-command-dispatch-and-gates.md` §5.3/§5.4, `📓️ex1-extension-exports-and-describe-cliff.md` §3.

Three items:
1. `client-e2e` determinism — pin every dispatch step to an explicit artifact kind + verb, stage
   the component the gate needs, and REFUSE up front by name when it is missing or stale.
2. `🧩️puzzle` under the 4 MiB bound — lazy example sources in the framework.
3. The remaining non-current descriptors, cheapest first, with census + catalog diagnostics after.

Machine at 22:16: load 43.8, 1 `rustc`, fleet wasm mutex free, 44 GiB free.

---

## 0. Headline — measured only

(filling)

## 1. `client-e2e` — the four reds, named

Baseline re-run at 22:17 (`bun ./📜️script.ts client-e2e` from `🌉️mcp/📦️packages/🟦️typescript`,
capture `🗑️generated/ce1-client-e2e-baseline.txt`): **13/17**, identical to A3b §6. The four:

| # | step | what it actually said |
| --- | --- | --- |
| 1 | `os: capability catalog health` | `4 diagnostic(s)` — `skipping plugin `puzzle`: … 🔣️.json did not decode as a PackageDescriptor` and `stdio: NotFound`, each counted twice (catalog-load + routing). |
| 2 | `os: artifact_create (a real plugin artifact kind)` | `kind=s.energy.model: NOT_FOUND — plugin `energy`'s compiled wasm is missing`. |
| 3 | `os: artifact_export` | `PLUGIN_UNAVAILABLE` on `mcp-client-e2e-…` — a CONSEQUENCE of red 2: the export step fell back to the gateway's own `os.agent.probe/v1` artifact, which by construction belongs to none of the 33 registered plugins. It was never a measurement of anything. |
| 4 | `os: action_prepare` | `energy.s.energy.model@1/*#editor.set-cell input={}` → the same `NOT_FOUND`. The journey `return`s here, which is why the denominator is 17 and not ~32. |

Reds 2–4 all descend from one fact: the journey took **hit 0 of a fuzzy
`capabilities_search({query:"set", kind:["mutation"]})`**. `capabilities_describe` on that hit
PASSED — the catalog types `energy` correctly — so nothing about the dispatch lane was measured.
Between WR4's 18:00 run and A3b's 22:0x run the same query's hit 0 moved `animate` → `energy` purely
because 20 more descriptors became decodable. The gate measured the BM25 weather.

## 2. The pin: explicit kind + verb, staged component, up-front refusal

All in `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts`.

**(a) Three constants, one plugin.** `CLIENT_E2E_PINNED_PLUGIN_ID = "note"`,
`CLIENT_E2E_PINNED_ARTIFACT_KIND = "s.note.note"`,
`CLIENT_E2E_PINNED_CAPABILITY_ID = "note.s.note.note@1/*#editor.addBlock"` — the exact guest and verb
WR4 §5.1 proved the two-phase prepare/apply law against, and the smallest of the rebuilt components
(64 MB vs `🌍️gis`'s 202 MB, the one that wedged an earlier run behind the 240 s wall). One plugin
per journey is also one cold component compile per journey, which is what WR4 §5.4's revert was
about.

**(b) `verifyStagedPluginComponent(repoRoot, pluginId)`** — a new exported function, and a new step
placed immediately after the channel pin, i.e. before the gate spends one request on a guest. It
reads the generated plugin registry for the plugin's `cratePath`/`wasmOut`, resolves the component
under `.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/{wasm-dev,wasm-release}` (the TypeScript
twin of the gateway's own `PLUGIN_WASM_TARGET_DIR`/`PLUGIN_WASM_PROFILE_DIRS`), and compares its
SHA-256 against the committed descriptor's `hashes.wasmSha256`. Three named refusals, each carrying
the command that fixes it; the journey returns on any of them.

It found a real defect on its first run — `note`'s staged component is **not** the build its
committed descriptor describes:

```
note: the staged wasm-dev/semio_s_plugin_note.wasm (64459968 B, sha256 46e12c0dfb01…) is NOT the
build its committed descriptor describes (63b37c41401a…) — the catalog would type this verb from a
descriptor that no longer describes the guest that runs it; re-describe it: …
energy: no compiled component is staged (tried …/wasm-dev/semio_s_plugin_energy.wasm, …)
animate: wasm-dev/semio_s_plugin_animate.wasm 104416649 B, sha256 ac2a2ed33b37… matches
```

i.e. WR4's 17:38 rebuild of `note`/`draw` left both components **newer than their descriptors**, and
nothing in the tree said so. `animate` matches because A3b re-described it at 20:33.

**(c) The dispatch rows.** `capabilities_describe`, `artifact_create (a real plugin artifact kind)`,
`artifact_export`, `action_prepare`, the saga member and all four `headRevision` re-reads now name
the pinned capability. `capabilities_describe` additionally asserts the catalog types it against the
pinned artifact kind, so a descriptor that retyped the verb is a named red rather than a silent
retarget. The `artifact_export` fallback to the gateway's own probe artifact is **removed** — a
failed typed create now returns instead of manufacturing red 3.


**(d) `🗒️note` was re-described** (`📜️a3-describe.sh 🗒️note`, 22:25 → 22:34, **546 s, rc=0**,
`🔣️.json` 197 877 → 197 877 B, pack 55 508 → **55 501** B) so the descriptor and the staged
component are one build again. The preflight then reads
`note: wasm-dev/semio_s_plugin_note.wasm 64 528 261 B, sha256 7cc1e0ed1125… matches`.

### 2b. What the pin then exposed — and two product defects it made visible

First pinned run (22:34 → 22:38, `🗑️generated/ce1-client-e2e-pinned1.txt`): **30/35**. The
denominator went 17 → 35 because the journey no longer returns at `action_prepare`: subscribe,
invoke, the resource-updated push, unsubscribe, snapshot, live head, undo, redo, transaction
begin/rollback, inference and the job rows all RAN for the first time. Three of the four original
reds are gone; two new ones appeared, both real:

1. **`artifact_snapshot` — `no such artifact: note`.** The step followed
   `revisionAfter.artifactId`, and in the headless lane that field is the **plugin id**:
   `🌉️mcp/🏠️workspace/🦀️.rs:1793` stamps `self.entry.plugin_id`. `action_prepare`/`action_invoke`
   drive the plugin's own live session document, while `artifact_create` persists a *separate*
   folder artifact seeded from it — so the step was snapshotting an id that names neither. Fixed in
   the gate by snapshotting the artifact the journey created, by the id it created it with, and the
   "the mutation landed" statement is left where it belongs, on `os: live head advanced` (a fresh
   `ReadHistory` against the document the mutation actually went to). The identity split itself is
   §8 gap 1 — it is the headless twin of WR4 §4 and bigger than this slice.
2. **`history_undo` did not move the revision stamp** — and this one is a product defect, fixed
   here. `HistoryPatch.upserts` is ordered newest-first and an undo does **not** remove its row, it
   clears `HistoryEntry.applied`; the stamp read `upserts.first()`, so a real `addBlock`, its undo
   and its redo all stamped `note@transaction:txn_…/1`. `expectedRevision` is the ONLY revision
   oracle an agent has for a plugin-owned artifact (the gateway's own `headRevision` docstring says
   so, and the shipped `history-undo` prompt tells agents to "confirm the result rather than
   assuming it"), so an optimistic-concurrency client would have committed over somebody's undo
   without ever seeing a `REVISION_CONFLICT`. **Fix:** `head_edit_id` is now the newest **applied**
   entry (`🏠️workspace/🦀️.rs:1793`). Measured after a rebuild of the binary (100 s, rc=0):

   ```
   PASS os: history_undo (mutation reverted) — members=1 warnings=none head note@/1 (baseline note@/0)
   PASS os: history_redo (mutation restored) — members=1 warnings=none head note@transaction:txn_5d4…/1
   ```

   The gate also stopped reading `members` as a success signal: `ActionAdapter::fan_out` returns
   `undo.members.len()` whether or not a member failed — per-member failures travel in `warnings`,
   and only an all-member failure is a tool error — so a fan-out where every member warned used to
   pass. Both rows now assert `warnings` is empty, and print it.

## 3. `capabilities_search` ranking, as its own catalog-only step

`capabilities_search` is no longer what picks the dispatch target. It has a step of its own, and it
is a **ranking** step read over the compiled catalog alone — no guest, no component, no artifact.
`capabilitySearchRankingVerdict(hits, mustReach, filteredArtifactKind)` (exported, next to the
preflight) asserts four properties of one reply to
`{query:"add a block to the note", kind:["mutation"], artifactKind:"s.note.note"}`:

| property | why a ranking owes it |
| --- | --- |
| scores monotonically non-increasing | a ranking that is not ordered is not a ranking |
| every `capabilityId` unique | two catalog rows carrying one id is the duplicate-id defect the catalog health line counts |
| every hit `audience=agent` | the catalog is an agent PROJECTION; a hit outside it is a leak |
| every hit's `artifactKind` equals the filter | the filter is honored rather than decorative |
| the pinned verb is present | the verb a client would look for is reachable by search at all |

Measured (both pinned runs, identical):

```
PASS os: capabilities_search ranking properties — 18 mutation hit(s) of `s.note.note`,
scores 6.824600030530695…0.05329356174636807 monotonically non-increasing, ids unique,
all audience=agent, `note.s.note.note@1/*#editor.addBlock` at rank 0
```

The journey's earlier free-text `capabilities_search` step (`query:"edit the document"`, no filter)
is untouched and still asserts only that the catalog answers a natural-language query at all.

### 3b. The inference target was the same defect, one tool along

`inference_list` returns the UNION of every installed plugin's declared roster and the journey took
`declared[0]` — which is `🌍️gis`, whose guest answered
`inference instantiate: wasmtime: failed to convert function to given type` (22:38). Only three
packages in the tree declare an inference at all (`gis` 1, `🀄️wfc` 5, `cad-extension-aec-building`
1 contributed), so the pin is `wfc`/`s.wfc.bitmap`/`s.wfc.bitmap.solve`; `wfc`'s component is
already staged and current (123 021 115 B, 09-18) and is verified by the same preflight, which now
checks **both** pinned plugins. `inference_list` asserts the roster CONTAINS the pinned service
instead of reading its first row.

## 4. `🧩️puzzle` — lazy example sources

(filling)

## 5. The descriptor sweep — remaining owners

(filling)

## 6. Census + catalog diagnostics

(filling)

## 7. Files changed

(filling)

## 8. Honest gaps

(filling)
