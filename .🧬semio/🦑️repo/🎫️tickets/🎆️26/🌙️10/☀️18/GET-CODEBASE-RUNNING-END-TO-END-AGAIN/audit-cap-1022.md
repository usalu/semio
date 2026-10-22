# 🔍️ Capacity and triage audit (cap-audit, 2026-10-22)

Read-only audit of `status-1022.md` O15 and `audit-live1021.md` #7 and #10. No file was edited except this report; no builds, no git writes, no sub-agents.

Path key: `L/` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`. `B/` = `L/🏗️build/`.

## Verdict summary

| Item | Verdict | Why (short) |
|---|---|---|
| A. `Inference reference text capacity exceeded` on the play (3,737) and demo (1,783) slice path | **SAFE** (static, not yet proven by a green e2e, see O6) | every slice call is byte-bounded to 16 MiB, the capacity is 64 MiB per call |
| A2. Same message on unchunked whole-graph calls | **CAN TRIP** only under `SEMIO_BUILD_GRAPH_VERIFY=1` or `baseline` (not the default slice run) | three whole-graph `portableInputs` / capture calls remain |
| A3. `Inference reference traversal capacity exceeded` (1,048,576 values per call) | **UNKNOWN** | not the reported message; trips only if JSON averages under 16 bytes per value inside a 16 MiB chunk; density not measured |
| B. Triage preview (`audit-live1021` #7) drops the refusal reason | **CAN TRIP** (diagnostic loss) | reason lost at `inventory-planning/🟦️.ts:316` and `taxonomy-authority/🟦️.ts:410`; `Error.cause` is never printed |
| B2. Triage preview still runs bare `bun` | **CAN TRIP** (host dependency, zero-touch and pinned-version rule) | `owned-file-approvals/🟦️.ts:331` returns `command: "bun"`; `inventory-planning/🟦️.ts:313` spawns it directly |

Correction to the premise: the live check is **64 MiB of UTF-8 bytes per call** (`policy.maximumTextBytes`), not "16 MB UTF-16 per call". The 16 MiB figure is the chunk budget (a quarter of the limit). The check measures `Buffer.byteLength`, which is UTF-8 bytes, not UTF-16 units. The `.length` (UTF-16) variant only exists in the staged tool `C/conc-graph-tools/expand-bytes.py` (NEW block, `JSON.stringify(...).length`), not in the landed code.

## A. Capacity: how the limit and the chunking are wired

- Limit constants, `B/👁️observation/📥️reuse/🧬️schema/🔣️.json`:
  - line 11: `"maximumTextBytes": 67108864` (64 MiB)
  - line 9: `"maximumValues": 1048576`
- Per-call text check, `B/👁️observation/📥️reuse/🟨️.mjs:19`: `charge` adds `Buffer.byteLength(value)` for every own key and every string, and throws `Inference reference text capacity exceeded` when the per-call total exceeds `policy.maximumTextBytes`. The counter is local to each call (`let values=0,text=0`), so the limit is per call.
- Byte-bounded chunker, `B/🕰️generations/🔌️load/🟨️.mjs` (path is `B/🕸️graph/🕰️generations/🔌️load/🟨️.mjs`):
  - line 144-154 `expandInChunksV1`: groups projects while `group.length < maximumProjects` and `bytes + size <= maximumBytes`, where `size = Buffer.byteLength(JSON.stringify(projects[name]))` (line 151).
  - line 160: `limits = host.expansionLimits ?? {maximumProjects:32, maximumBytes:16777216}`.
  - line 174: every generation load (slice, closure and all) passes assembled projects through `expandInChunksV1`.
- Host limits, `B/🕸️graph/🟦️.ts:330`: `maximumProjects:32`, `maximumBytes: floor(maximumTextBytes/4) = 16777216`.
- Each chunk reaches `portableInputs` (`B/🕸️graph/🟦️.ts:187`), whose first step is `captureInferenceInventoryReferencesV1(value)` on that chunk (`🟦️.ts:194`).

### Bound

For a chunk, text(chunk) <= sum of UTF-8 bytes of `JSON.stringify(project)` + UTF-8 bytes of the project names. The text counts only keys and strings, while the JSON size also counts punctuation and escapes, so text is never larger than the JSON bytes. The names are negligible (32 short keys). So:

- a normal chunk has text <= 16,777,216 + small, against a limit of 67,108,864: **4.0x headroom**;
- a chunk can exceed the limit only if it holds a single project whose JSON is above about 64 MiB (a singleton chunk is allowed to exceed `maximumBytes`, see `expandInChunksV1` line 151 check `group.length &&`).

### Arithmetic for the two slices

Input measurement from the ticket, `result-conc-graph.md:123` (§13): compact generation nodes average 715 KB (2.86 GB for 4000 projects), measured offline on the real generation on 10/21.

- Play, 3,737 projects: 3,737 x 715 KB = about 2.67 GB; at about 23.5 nodes per 16 MiB chunk, about 159 chunks (the 32-project cap is not binding for these sizes).
- Demo, 1,783 projects: about 76 chunks.
- Every chunk: <= 16.8 MB text, so the 67.1 MB capacity is not reachable.
- Trip threshold for the slice path: one project above about 64 MiB JSON, about 90x the 715 KB average. The largest recorded slice (`result-conc-graph.md:123`) is `proctor:build`, 109 projects, 87 MiB, an average of about 0.8 MB per project. No project above 64 MiB is recorded. Max single-project size is UNKNOWN but has a large margin.
- Internal inconsistency in the ticket: "4000 nodes in 148 chunks" (`result-conc-graph.md:123`) implies about 27 nodes per chunk, i.e. about 0.62 MB average under the 16.8 MB bound, not 0.715 MB. This is about 15% and does not change the verdict (the trip needs about 4x).

### Runtime evidence

- 14:49 failure (run `c`): `report-e2e-orch-e2e-1021c.md:156-158` and `e2e-map-1022.md:101,106,116`. That run used the pre-fix code (64-project chunks, no byte bound; `coordination.md:655`).
- Fix recorded at 16:28, `coordination.md:691`: byte-bounded chunks.
- After the fix, the play cold run (e2e-1021e) reached "passed portable inputs 155/155" and failed later with OOM, not with a capacity error (`coordination.md:698`). Demo failed with OOM too (`coordination.md:699`). So the capacity error did not recur on the slice path after 16:28.
- Caveat: the 18:52 artifact change (`coordination.md:713`) touched the publication path, not the chunker; the chunker is unchanged in the code read here. Green cold/e2e runs since the fix: none (O6).

### Test coverage

- `B/🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts:236-247` uses the real `maximumTextBytes`, asserts that a single over-limit call is refused, that chunks of `limit/4` each cost one capture call, and that one oversized project is refused. This covers audit #10's "assert it in a test" request.

## A2. Unchunked whole-graph calls (CAN TRIP, not on the default slice path)

Callers of `portableInputs` or `captureInferenceInventoryReferencesV1` with the whole graph, not a chunk:

1. `B/🕸️graph/🟦️.ts:487`: `readBuildGraphV1(await portableInputs(root, compact, ...))` on the full-inference publish path. The whole graph is one call.
2. `B/🕸️graph/🟦️.ts:449`: `image(source)`, which calls `portableInputs(root, decoded, ...)` on the whole sealed graph JSON.
3. `B/👁️observation/...` via `B/🕸️graph/📥️inference/🚦️invocation/🟨️.mjs:40`: the non-snapshot `nodes` observation passes the whole workspace node value to `inferenceInventoryReferencesHoldV1` as the `reusable` check on a cache hit. The slice path avoids it because `discoverBuildGraphPartV1` sets `snapshot:true` (`B/🕸️graph/🟦️.ts:364`).

Reachability:

- Slice runs (`loadBuildGraphSliceV1`, `🟦️.ts:512-516`) take `loadGenerationGraphV1`, which is chunked. Not reached.
- `selection.kind === "all"` non-baseline (`🟦️.ts:545-549`) is chunked. Not reached.
- Items 1-3 are reached only through `loadBuildGraphUnpinnedV1` with `baseline:true`, or through the oracle `SEMIO_BUILD_GRAPH_VERIFY=1` (`🟦️.ts:504`, `🟦️.ts:548`).
- Environment search (repo, including hidden dirs): `SEMIO_BUILD_GRAPH_VERIFY` appears only in `🟦️.ts` and in conc-graph tools; the one harness use is `conc-graph-tools/measure-after-go.sh:26` (`--verify`, a measurement step). The e2e orchestrator was not found to set it; this search was not exhaustive.

Arithmetic: a whole 3,737-project compact graph is about 2.67 GB of JSON. Text is most of that, so it exceeds 64 MiB by more than an order of magnitude. Any VERIFY or baseline run on play or demo would trip with the reported message. CAN TRIP under that env.

Minimal root fix: route items 1 and 2 through the same byte-bounded chunker, i.e. `readBuildGraphV1({...compact, projects: await expandInChunksV1(neutral, chunk => portableInputs(root, chunk, invocation, phaseControl), limits)})`, with `limits` lifted from the generation host literal (`🟦️.ts:330`) into a module constant. For item 3, use `snapshot:true` on the VERIFY path or split the reuse check per project. No change to the slice path is needed.

## A3. Values cap (UNKNOWN)

`maximumValues = 1,048,576` per call (`🔣️.json:9`). A 16 MiB chunk trips it only if the average JSON value is under 16 bytes (16,777,216 / 1,048,576 = 16). The ticket has no measured value density, only "3,737 projects exceed it" (`result-conc-graph.md:116`), which bounds the whole graph only. Verdict UNKNOWN; not the reported message. Measure values per project on one real node before calling it safe.

## Doc drift to fix

- `result-conc-graph.md:123` says "a chunk that still exceeds a capacity is halved". The landed `expandInChunksV1` (`load/🟨️.mjs:144-154`) has no halving retry; the halving only exists in the staged `expand-bytes.py` NEW block. Either drop the sentence or restore the retry; the code does not need it for the slice path.
- `status-1022.md:27` (O15) and `audit-live1021.md:63` say "16 MB UTF-16 per call". Correct to "64 MiB UTF-8 bytes per call; 16 MiB per chunk".

## B. Triage preview (audit-live1021 #7)

Path: `L/🔍️discovery/✅️owned-file-approvals/🟦️.ts` (preview spec), `L/🧹️normalization/🔍️inventory-planning/🟦️.ts` (spawn), `L/🔍️discovery/🔐️taxonomy-authority/🟦️.ts` (validation).

### B1. Refusal reason dropped: CAN TRIP (diagnosis)

- `owned-file-approvals/🟦️.ts:320,330` throw the generic message with no cause. The validator text from `generator-commands/🟦️.ts:33-37` (for example "previewScript must be an owner-relative path...") is only reachable via `cause` at lines 322 and 328, and the message itself does not contain it. Partly fixed: `{cause}` is attached (the audit's "rethrow with cause" is done), but the message is still generic.
- `inventory-planning/🟦️.ts:316`: when the spawn fails (`result.error`, for example ENOENT when `bun` is absent), the thrown message carries only `status=-1` and sha256 hashes of empty stdout and stderr. `result.error` is never read. The reason is dropped here, in the spawn path, not at line 322.
- `taxonomy-authority/🟦️.ts:409-410`: `catch { problems.push("...must route exactly...") }` discards the error entirely.
- No code in `L/` reads `Error.cause` (the only `.cause` hits are the `collect-errors` field name), so `cause` is never displayed.

Minimal fix: at `inventory-planning/🟦️.ts:316` append `result.error?.message`; at `owned-file-approvals/🟦️.ts:322,328` interpolate `cause.message` into the message; at `taxonomy-authority/🟦️.ts:410` keep the cause text in the problem.

### B2. Still runs bare `bun`: CAN TRIP (environment and rule)

- `owned-file-approvals/🟦️.ts:331`: `return { command: "bun", args, cwd: options.cwd }` for the owner-cwd case. `args` is `direct` (line 326), so the spawn is `bun ./<owner>/📜️script.ts ...` with no repo wrapper.
- `inventory-planning/🟦️.ts:313`: `spawnSync(execution.command, execution.args, ...)`, so this is a bare process spawn.
- The `.` and framework cases (lines 323, 325) do run `bun` inside a repo wrapper (`native owner-command` / `command --config`). Only the owner case is bare.
- Today it works on this host (bun 1.3.14 on PATH). It fails with ENOENT on any zero-touch host without `bun` on PATH (AGENTS: repo binary for all commands), and it bypasses the pinned bun 1.4.2 (O10, `package.json:145`).

Minimal fix: route the owner-cwd case through the same repo script wrapper as lines 323 and 325 (`./🔨️modules/🏃️process/📜️script.ts command ...` with `--cwd owner`), so the spawn is `./🔨️modules/🏃️process/📜️script.ts` and the wrapper provisions bun. UNKNOWN: the wrapper's required `--manifest` for a non-cargo owner needs a check before the edit.

The triage taxonomy fix itself is covered by fixture rows (`owned-generator-preview-inventory`, +4 `ownerExecutionRoutes`, `audit-live1021.md:49`), so no new test is needed for the reason text beyond an assertion on `cause.message` or on the `result.error` text.

## Remaining gap

- No green cold or e2e run since the fixes (O6), so A and B are static verdicts. The proof is one run of play and demo to the slice expansion step, checking for the absence of the capacity error, plus one VERIFY run on a small project to confirm A2 is the only unchunked path.
- The measurements (715 KB per node, 148 chunks) come from the 10/21 offline measure in `conc-graph-tools/expand-bytes.py` and `slice-closure.py`, and the graph cache is empty right now (`.cache/b/g/m` has 7 files, no generation records), so they were not re-measured here.
