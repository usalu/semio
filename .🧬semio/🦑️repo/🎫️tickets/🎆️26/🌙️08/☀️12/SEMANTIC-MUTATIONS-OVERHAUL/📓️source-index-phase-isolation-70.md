# Source Index Phase Isolation 70

## Evidence Boundary

The sole full before/after pair timed out without its final marker or a retained snapshot boundary. The separately retained import-only diagnostic completed in 135 ms. Neither fact establishes the cause of the pair timeout. Their terminal distinctions remain in [Source Index Pair — Sole Bounded Attempt 68](./source-index-pair-terminal-68.md) and [Source Index Import Isolation 69](./source-index-import-isolation-69.md).

The pair child received exactly `subject <repoRoot> <cancelRelative> <runRelative>`. It validated arguments, dynamically imported root and discovery, invoked the first SourceIndex call, built/wrote first snapshot artifacts and `before-manifest.json`, then repeated the call/snapshot, wrote drift and final manifest, and only then emitted its single stdout marker. Its progress callback only accumulated in-memory counters. Therefore that pair's empty stdout excludes only the final all-phases-complete marker; it is not evidence that import or first invocation did not start. Absence of `before-manifest.json` means the first snapshot helper did not finish its artifact/manifest boundary; it does not prove the first SourceIndex call did not return.

## Proposed Single-Call Phase Diagnostic

[This controller](../🧪️source-index-phase-isolation-70/📜️script.ts) has one owned child and a fixed 30-second total budget: 25 seconds before it writes the cancellation file, then five seconds terminal grace before it signals only that child. It dynamically imports current root and normalization modules, records synchronous import markers, calls only canonical `inventoryTaxonomySources({ repoRoot, cancelFile, progress })`, then calls current root `mutationTaxonomySourceIndex(repoRoot, { cancelFile, progress }, admission)` exactly once with that returned admission injected. It supplies no explicit ticket directory, alternate roots, policy list, scope, or source file list.

The N call is canonical source admission, not a replacement collector. Injection deliberately prevents SourceIndex from running admission a second time: it isolates SourceIndex's post-admission schema capture, mutation-root projection, file-fact selection, per-file snapshot loop, and roster/digest construction. Both calls receive the same cancellation-file path. The child synchronously writes boundary markers, each distinct N/SourceIndex callback phase, and measured begun/completed call counts. If admission is rejected, it emits the exact status/diagnostics and does not invoke SourceIndex.

The parent concurrently drains pipes and captures six selected, nonexhaustive inputs before and after: controller, root S entry, N normalization, D discovery, taxonomy schema, and mutation-descriptor schema. Captures check lexical Compose rejection, no-follow root/ancestor/leaf identity before and after descriptor reads, including root and initial leaf joins. Raw child stdout/stderr and terminal observation are written before event parsing or post-capture; a post-capture failure is retained as an explicit receipt error, never substituted with an empty successful capture. A grace-period exit is recorded only as `cancel-requested-exit`, not as a claimed cooperative response.

## Interpreting a Future Terminal Receipt

- `after-root-import` absent: root module import/evaluation remains a possible boundary.
- `before-admission` present but no admission-progress/after-admission marker: admission entered but no callback was observed; this does not name a specific internal N stage.
- `after-admission:complete` present but no source-index progress/after marker: the injected post-admission SourceIndex region is implicated, not a repeated admission. An `after-admission:rejected` marker records admission outcome rather than a successful handoff.
- `source-index-progress:source-index` present but no after marker: SourceIndex reached the selected-file snapshot loop; no semantic claim follows.
- `after-injected-source-index` present: this single admission plus one injected SourceIndex execution completed within the bounded observation. It does not prove a full paired census or semantic completeness.

No diagnostic run has been started. The controller adds no source authority and does not edit N, S, D, taxonomy, or production sources.

## Root Preparation Release

Root fully read the revised controller and the actual root SourceAdmission/SourceIndex declarations. The direct N call uses the same repoRoot/cancelFile/default-source-authority operands as the private root wrapper, but the later call deliberately injects that returned admission. It is a phase experiment, not a byte-identical replay of the prior pair's control flow.

Root corrected the preliminary controller's ENOENT workspace lookup/capture weaknesses before any run. The current selected-input comparison checks exact source leaf hash/bytes/identity and root/ancestor device+inode; directory mtimes/sizes are retained as observations, not treated as evidence that unrelated child creation changed a selected source. Raw terminal/log files are retained before fallible endpoint capture. Start/cancel/kill timestamps and child PID are recorded. A hard terminal kill is not a cooperative-cancellation success.

Current controller is16207bytes, SHAd1b34c159570aae1203bb37c3c852607c7036ebf3995a82b16763c047582c3db. Earlier a5b775…/a03bc5… preparation hashes remain historical. Root's current non-executing Nx/Bun plan (session10917) exited0. Independent TypeScript source parsing returned zero syntax diagnostics; this is not a strict type-check or runtime proof. The subject has not run at this release.

The exact event is now `phase: after-admission` with a separate `status` field, not a colon-combined phase. Only status=complete proceeds to the injected SourceIndex; rejected diagnostics remain actual observations. Six selected inputs are captured, not the complete transitive import or Git/worktree membership authority. No physical snapshot files, semantic inventory or pair-stability claim are produced by this experiment.

Proposed exact launch row (pending canonical collision/nofollow/pure-preview join; no source hold requested):

```json
{
  "name": "⚖️gate🧬️mutations🧾️phase-isolation",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-phase-isolation-70/📜️script.ts\" run",
  "cwd": "${workspaceFolder}",
  "presentation": {
    "group": "4_gate",
    "order": 410.529
  }
}
```
