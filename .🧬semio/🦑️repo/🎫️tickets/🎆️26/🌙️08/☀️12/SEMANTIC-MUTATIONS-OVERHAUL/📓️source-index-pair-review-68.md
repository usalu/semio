# Source Index Pair Review 68

Root fully read the prepared [controller](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-before-after-65/📜️script.ts) at SHA256 `cec8aff6305b3c63dc92992897d2cd0f3e9216729f7232b40adfb04e471397ca` and its [proposal](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️source-index-before-after-65.md). It has not executed a real source-index call.

The initial controller is not approved for execution. It calls SourceIndex once, so critical-file hashes before/after do not establish a source roster pair. Its 10MiB summary cap is also unsuitable for complete large rosters; stdout/stderr are drained only after child exit; the unused deadline timer remains live after early completion; and critical captures omit per-read/file/ancestor identities. These are preparation review findings, not observed runtime failures.

The revised bounded packet must:

- Call only the existing `mutationTaxonomySourceIndex(repoRoot, {cancelFile, progress})` twice serially, with no output-derived `explicitTicketDir`, additional roots, or alternate collector.
- Use one 120-second cooperative cancellation window plus a 15-second terminal grace for the whole pair. No retry, larger deadline, foreign process action, or cache cleanup.
- Retain complete exact source paths, source roster, and complete admission observations (including generatorOutputs/indexEntries/gitlink boundary) as separately hashed JSONL artifacts under the fresh ticket run.
- Keep a small summary containing exact counts/digests and added/removed/changed membership/content, not truncated rows. A changed pair is drift evidence, not stable census.
- Drain both child pipes concurrently; clear deadline timers at terminal; preserve all produced logs and failure evidence.
- Validate lexical exclusion before probing, and retain no-follow ancestor/leaf plus open/read identity checks before/after each critical input.
- Explicitly join index schema hashes to critical captured schema endpoints. Root S/N/D/controller/schema stability is selected-source evidence, never all-transitive-source stability.
- State that source membership/content observation does not establish semantic mutation or off-facet declaration completeness.

No production S/N/D/schema source changed during this review. The earlier 15-test/277-assertion source-boundary result remains separate actual fixture evidence.

## Revised Controller Release And Exact Launch Request

Root fully read the revised 303-line `923e57ac…` controller and made only the parent-side evidence changes: persist the selected baseline before spawning; persist terminal PID/exit/log counts before post-validation; and do not let the missing-result artifact-set check suppress a failure receipt. The child still calls the actual SourceIndex twice serially under the unchanged single120s+15s window. No source-index call has executed at this release.

Current controller:25327bytes, SHA256 `2badfd4e7b88f7fdbb57310b9f6873ac33f4084b26a28c1b9341580e454f9b53`.

The initial root preparation-only command omitted Nx's explicit workspace project. It exited1 with `Module not found` in a selected project's working directory before the controller ran. This is an invocation boundary, not evidence of source loss or a failed SourceIndex call. The corrected command uses `exec --projects=workspace --skipNxCache`; its actual terminal result will be recorded below. No real pair has been retried.

Read-only launch text search found410.522–.526, with no410.527 row in either current seed or output. This is a proposed row pending the launch owner's exact numeric/name/no-follow/pure-producer admission; no local launch mutation is authorized here.

```json
{
  "name": "⚖️gate🧬️mutations🧾️source-index-pair",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-before-after-65/📜️script.ts\" run",
  "cwd": "${workspaceFolder}",
  "presentation": {
    "group": "4_gate",
    "order": 410.527
  }
}
```

The decoded command above contains ordinary U+0022 path delimiters, no literal backslash. Registration does not execute the pair. Exact child artifact hashes, selected source identities, observed membership/content drift and source-index schema joins will be reported only after the sole real attempt. Semantic completeness remains explicitly false.

The corrected preparation command actually completed with exit0 and only `[DEBUG] source-index before-after controller prepared; root review is required before run.` (session58443). It did not import the root SourceIndex subject or make an inventory call. The earlier invocation failure remains recorded above.

## Independent Final Review Correction

The independent read-only review identified an omitted parent join: the marker count was checked but the authenticated manifest's measured count was not. Root corrected only that parent validation and its owned type declarations. The parent now requires exactly one marker, exactly one captured manifest payload, schemaVersion2, the exact pair kind, manifest apiCallCount2 equal to the marker count, both admission statuses complete, and semanticCompleteness false. It also records markerCount and manifestJoined in the final receipt.

Superseding current controller:26051bytes, SHA256 `d9cd27afb325fa28abf5d4d0f14897fbf1892855435459757e1e610eb9fcef46`. The launch command/name/order are unchanged. Previous hashes/results are historical preparation observations; no real SourceIndex pair ran at this correction. The launch owner was notified before its final availability capture.

The d9cd controller preparation command also actually exited0 (session50704) with the same single prepared message and no SourceIndex invocation. A separate read-only TypeScript createProgram diagnostic first lacked the requested Bun ambient library; the installed Node ambient library then resolved the Node/Buffer imports. That second check had zero syntax diagnostics but seven semantic diagnostics: four Bun ImportMeta dir/path declarations, one absent Bun global, and two child-exit callback types left implicit because Bun's declaration was absent. Neither `bun-types` nor `@types/bun` resolved in this workspace probe; no package was installed, no ambient stub introduced, and no strict-typecheck pass is claimed. Actual Bun execution of preparation is the observed runtime boundary. The failed additional probe for a node_modules/.bun directory is an absent package-manager-layout observation, not source loss; Node's package actually resolves through node_modules/@types/node.
