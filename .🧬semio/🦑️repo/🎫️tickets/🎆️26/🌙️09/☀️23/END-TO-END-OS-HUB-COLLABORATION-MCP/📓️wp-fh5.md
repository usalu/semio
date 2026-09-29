# WP-FH5: Fault Class Review for Family A (Framework Catalog) and Family H (11 Plugins)

Slice FH5 (session 15) helps S20 (spec: `📓️fault-localization-api.md` §7). Work lists:
`.🧬semio/🌐hub/s14-s20-sets/class/family-A.json` (617 codes, 171 `default`) and `family-H.json` (122 codes, 20 `default`).
I edited only the `class` field. A line diff with the class value masked is empty for both files, so owner, code, rule and en are
byte-identical. I did not edit any source file or overlay, did not build, and ran no git or ticket tools.

Tools in `wp-fh5/`:
- `fh5-digest.py`: shows each entry with its census raise sites, read from the live tree, or from the overlay at the census line
  when the live tree still has the old sentence form.
- `fh5-grep.py`: an indexed source grep. The repo-wide grep stalls.
- `fh5-decisions.tsv`: every changed code, as owner, code, from, to and one-line reason. This is the full record.
- `fh5-apply.py`: writes the TSV into the lists. It is idempotent: a re-run changes 0 lines.

I applied the coordinator's cross-family rules from `📓️wp-fh6.md` unchanged. Nothing here is an exception to them.

## Session 15

### Counts

| family | changed | input-invalid | precondition-failed | conflict | permission-denied | unavailable | cancelled | internal |
|---|---|---|---|---|---|---|---|---|
| A (framework) | 314 of 617 (default 159 of 171, other 155) | 88→70 | 233→34 | 12→21 | 2→2 | 44→79 | 16→13 | 222→398 |
| H (plugins) | 24 of 122 (default 13 of 20, other 11) | 35→42 | 49→36 | 2→0 | 0→0 | 1→0 | 0→0 | 35→44 |

Family A transitions:

| from → to | count |
|---|---|
| precondition-failed → internal | 138 |
| input-invalid → internal | 39 |
| precondition-failed → input-invalid | 35 |
| precondition-failed → unavailable | 26 |
| unavailable → internal | 17 |
| input-invalid → unavailable | 16 |
| internal → unavailable | 11 |
| precondition-failed → conflict | 9 |
| internal → input-invalid | 5 |
| input-invalid → precondition-failed | 5 |
| cancelled → unavailable | 3 |
| unavailable → input-invalid | 2 |
| internal → precondition-failed | 2 |
| unavailable → precondition-failed | 2 |
| cancelled → input-invalid | 1 |
| precondition-failed → cancelled | 1 |
| conflict → precondition-failed | 1 |
| input-invalid → conflict | 1 |

Family H transitions:

| from → to | count |
|---|---|
| precondition-failed → input-invalid | 12 |
| input-invalid → internal | 4 |
| precondition-failed → internal | 4 |
| conflict → precondition-failed | 2 |
| input-invalid → precondition-failed | 1 |
| unavailable → internal | 1 |

### Judgement calls (the rule, then the codes it covers)

- **Broken framework bookkeeping → internal.** An owner, outcome, session, ticket or resume authority was lost; something was
  "Complete without its terminal-empty witness"; a generation was exhausted. Covers `*-missing`, `*-resume`, `*-false-terminal`,
  `*-terminal-not-empty`, `*-authority` and `*-generation` across artifact-envelope, artifact-store, interactive-job, job.infer and
  window-config/transient.
- **`*-over-budget` / "exceeded its exact grant" → internal, not unavailable.** A disposer broke its single-owner grant. That is a
  broken contract, and retrying does not help.
- **Busy or full, retry later → unavailable.** Covers fixed slots that are saturated or collided, a failed `try_reserve`, a busy
  or contended lock or lease, and worker admission capacity: `*-capacity`, `*-allocation`, `*-busy`, `cancellation-busy/capacity`,
  `admission-capacity`, `reactor-turn-deadline`, `deadline-exceeded`, `http-error`, `worker-lost`, `storage-error`, `stream-error`.
- **Undecodable bytes that our own code built → internal.** Covers paged command pages, varints, UTF-8 and padding; job inputs
  (`job.*.decode`); `plugin.checkpoint.*`; `retained-command-checkpoint-*-invalid`; `os.app-frame.decode`, `os.fault.decode` and
  `extension.evaluate`; bundled or registered examples (`app.example.unreadable` and the four H `*-example-unparsable` codes);
  and our persisted window-config (`pack`, `history`, `replay`, `typed-state`, `inner-identity`, `pack-envelope`).
- **The request is too big → input-invalid.** Covers `plugin.command-byte-cap`, `plugin.command-page-count`,
  `plugin.command-field-cap`, `plugin.transaction-prepared-ops`, `interactive-job.raw-wire-limit`,
  `interactive-job.host-configuration-output`, `interactive-job.child-root-retirement-span`, the `request-registry` body and answer
  caps, `artifact-inference.budget`/`job.infer.result` (the request's own budget) and H `*.retained.extent`.
- **The current document is too big → precondition-failed.** `interactive-job.media-output-limit`,
  `interactive-job.segmented-output-limit` and `animate.video.export.program-too-large`.
- **Unknown action, or the right action on the wrong channel → input-invalid.** `framework.document-transfer.shell-owned`,
  `interactive-job.framework-route`, `interactive-job.agent-lane-uncarried`, `plugin.command.not-app-owned`, `opening.*` argument
  contradictions, and the H `*.unhandled-action` codes.
- **Missing or empty argument → input-invalid.** `writer.camera.missing`, `animate.import-media.input-missing`,
  `agent.target-revision-unresolved` and the `artifact-inference` request validation codes.
- **Stale handle, generation or checkpoint context → conflict.** `artifact-envelope.ingress-handle` (matches
  `load-stale-handle`), `retained-command-checkpoint-context/workspace-mismatch` and `window-transient.document-generation`.
  Post-merge reconcile warnings are also conflict: `workflow/edge-*`, `workflow/parameter-binding-invalid`, `externalDivergence` and
  `genesisMismatch`.
- **Window no longer open → precondition-failed** (the FH6 rule): `remodeling-window-stale` and `sourcing-grid-window-stale`.
  So is a target that is gone: `job.unknown`, `interactive-job.unknown-segmented-download` and `interactive-job.peer-roster-owner`.
- **Feature not built yet → internal; runtime cannot do it by design → precondition-failed.** Internal: `not-wired` and energy's
  `mutation.kind-unavailable`. Precondition-failed: `host-async.poll-backed`, `plugin.host.direct-unavailable`,
  `surface.unknown-dialect` (as `artifact-inference.not-registered`) and `surface.conflict`, which is an installation clash, not a
  revision conflict.
- **Sync status codes.** `reconnecting` and `link-expired` → unavailable. `artifactBootstrap` and `artifactBootstrapLocalReplay`
  → unavailable, because the client reconnects automatically. `capability-revoked` → cancelled, since every site is "cancelled
  before dispatch" or "host closed". `hubDocumentSeed` → internal.
- **Malformed or oversized data from peers → input-invalid.** `interactive-job.peer-presence-actor`/`-duplicate`,
  `peer-roster-presence-pack-cap`, `plugin.command-presence-*`, and damaged archives (`plugin.document-archive-members`, `-ordinal`,
  `-utf8`).
- **Other internal cases.** Job `*.suspended`/`*.terminal`/`job.stalled`/`job.unknown-kind`. Wrong route or step:
  `remodeling.retained.route`, `energy.model.3d.viewer.retained-route-required`, `window-config/transient.address`, `*-type`.
- **`mutation.rejected` → precondition-failed** (was internal), the same as `transaction.member-rejected`. The merge policy
  refuses the change against the current state.

### Ambiguous → S20

**The en text contradicts the class:**
- `artifact-inference.unavailable` (internal): every site is a poisoned registry lock, but en says "try again".
- `artifact-envelope.ingress-credits` (input-invalid): the dominant cause is a document over the fixed decode maximum, but en says
  "wait a moment". An allocation failure at the same site would be unavailable.
- `remodeling.retained.route` (internal): en says "wait for the running step".

**Codes that should not carry a fault class:**
- `linked` (kept precondition-failed): a sync status, not a fault. It should leave the catalog or be treated as a non-fault.
- `app.notice` (kept precondition-failed): used only as a cause code under `app.command.no-effect`.

**The same code is raised for different reasons (not split):**
- `*-retained-command-tool-mismatch-or-capacity` (playbook, imperative, block2d/3d/5d): kept internal. The capacity half is
  input-invalid.
- `interactive-job.dispatch` (input-invalid): an `UnknownController` is internal.
- `interactive-job.reserved-admission` (internal): the decoded-items cap is input.
- `job.infer.admission` (unavailable): the "lost the rejection" site is internal.
- `plugin.command-batch-cap` (unavailable): the empty-batch site is internal.
- `plugin.presence.local-read` and `plugin.ephemeral.apply-rejected` (unavailable): a store that is closing is a precondition,
  and an app mutation refusal is internal.
- `animate.video.export.program` (internal): `VideoRenderProgramError` mixes generator defects (schema, index, paint) with
  document limits (`TooLong`, `Dimensions`).
- `job.host-fault` (internal): `TurnFault` includes deadline, fuel and cancel.
- `artifact-inference.cancellation` (input-invalid): an unknown id at cancel time could be precondition-failed (already finished).

**Generic stand-ins that drop the inner fault's class:**
- `transaction.commit-failed`, `interactive-job.fault` and `interactive-job.reserved-spawn-failed` are set internal. They should
  forward the inner code.

**Close calls:**
- Stale or unknown handles (`artifact-envelope.ingress-handle`, `load-stale-handle`, `stale-handle`, `plugin.command-driver-stale`)
  are set conflict. They are internal if hosts must never reuse a handle after a cancel.
- `mutation.rejected` and `transaction.member-rejected` (precondition-failed) become conflict when the rejection comes from a
  concurrent change.
- `genesisMismatch` (conflict) could be precondition-failed.
- `module.pack` (kept precondition-failed): `PackError` covers both corrupt user data and our own encoder.
- `artifact-inference.cache-mode` (input-invalid, following en): it is internal if a service must always honor the requested mode.
- `interactive-job.step-overrun` (unavailable: the 8 ms wall ceiling depends on load) and `interactive-job.preview-budget`
  (precondition-failed: the turn count is deterministic, but the wall clock is not).
- `file-import.*` (input-invalid) assumes a caller may supply the chunk stream (os-mcp). They are internal if only the host ever
  chunks.
- `router-error`, `storage-error` and `artifactBootstrap` (unavailable) can also be permanent errors or protocol defects.
- `interactive-job.segmented-download-total-over-cap` (internal): I did not check the caller of `admit_maximum`.
- `remodeling.reconstruction.provisional-count` (internal): a u32 overflow of the provisional list, but en offers a state remedy.
