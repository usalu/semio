# Browser Control Workload Robustness

Only retained ticket inputs changed. Step 101 now prepares the control workload through the existing public action forms and history witnesses, before step 102 starts Fill. Bitmap uses Resize Output in the output window at 64×64; grid2d uses Resize Grid at64×64; grid3d uses Resize Grid at16×16×16. Each graph editor adds256 slots through its public Create Slot form, with explicit ids, positions and extents. Both locales receive unique slot ids. The schema requires the workload and strict control coverage contract; step102 refuses to proceed if public setup did not complete.

Setup is cancellable between each public form and bounded to900000ms per preparation phase. Each mutation still has its existing30000ms history witness bound; the first unseated form or failed witness stops setup. Solve observation remains bounded to120000ms, with explicit pause and abort observation waits. No guest internals, hidden state injection, resource policy change, clock override or browser execution was used.

Pause coverage requires an observed Paused state, an unchanged progress value held for1200ms, and a successful Resume control. Completion before or during pause is separately classified as a terminal race, and the required pause/resume verdict fails rather than claiming coverage. Active cancellation requires a nonterminal, non-ready sample followed by Aborted; aborting a ready completed result has its own verdict and does not satisfy active cancellation coverage. Progress visibility and advancement are separate required verdicts. Both control runs must leave authored history unchanged.

Bitmap Keep now runs after the cancellation case. This prevents fully pinned output from trivializing the cancellation workload and additionally checks that cancelling a new run preserves the prior rendered preview. Keep still must create exactly one authoring operation.

The independent workload check constructs nonempty complete payloads over the planned extents, the longest declared scenario tile id,512 retained plugin trace events,128 legal framework step records across tick/progress,9 counters and the largest JSON-safe integer metadata. Ajv validates each actual plugin payload schema. The real public TypeScript encodeToolRunTick/decodeToolRunTick and JSON/hex mirror codecs both round-trip the payload. The262144byte limit applies to the binary ToolRunTick; outer base64, JSON/hex and escaped command representations are separately measured and fit the declared retained command envelopes.

| Variant | Plugin Payload | Full Binary Tick | Escaped Payload Argument | Base64 Tick | JSON/Hex Tick |
|---|---:|---:|---:|---:|---:|
| bitmap64² |32415|42567|35523|56756|96561|
| grid2d64² |196845|206997|234759|275996|425421|
| graph2d256 new slots |49658|59810|55838|79748|131047|
| grid3d16³ |222288|232440|269420|309920|476307|
| graph3d256 new slots |49658|59810|55838|79748|131047|

The16³ grid3d case therefore fits the normative binary tick carrier with29704bytes remaining even under this enlarged framework metadata case; no smaller replacement was necessary. Its quoted payload and JSON/hex outer representation also fit the540672byte retained raw envelope. Grid2d uses its declared1MiB raw envelope. These are carrier bounds, not a claim that a particular solver output was produced.

The retained inputs wfc-browser-cases.json, wfc-browser-schema.json and wfc-browser-scenario.ts now describe this journey. wfc-browser-check.ts passed Ajv validation, all five scenario imports and both-locale public workload expansion checks. wfc-browser-workload-check.ts passed all five normative payload schemas and actual codec round-trips. Browser execution and positive control coverage remain pending with the parent execution owner.
