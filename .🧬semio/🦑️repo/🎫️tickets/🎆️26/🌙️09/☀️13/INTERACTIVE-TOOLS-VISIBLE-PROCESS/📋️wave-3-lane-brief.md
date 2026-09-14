# 🧭️ Wave 3 lane brief — convert one plugin's tools to ToolRun

Read, in order: `T/📋️lane-rules.md` (binding), `T/📋️tool-run-contract.md` (§2, §3.2, §3.6, §3.7, §4, §5 wave 3),
the landed reports `T/📓️wave-W0-A.md`, `T/📓️wave-W0-B.md`, `T/📓️wave-W0-D.md`, `T/📓️wave-W0-E.md`,
`T/📓️wave-W0-H.md` (runtime API as landed), `T/📓️wave-W1-A.md` (a complete run job example), `T/📓️wave-W1-D.md`
(policy predicates + required-tool table), `T/📓️audit-p4-tool-inventory.md` (your plugin's section and §8 row),
and `/Users/ueli/Documents/semio/CLAUDE.md`.

For every tool of your plugin classified `algorithmic-mutating` or `algorithmic-readonly` (re-verify the
classification in code first; correct the inventory in your report where it is wrong):

1. Declare `ToolRunDefinition` on the tool/utility in the manifest: stages, counters, reasons with verdicts and
   EN + DE templates, trace subject kind, `mutating`, `rebase`, `reconfigure` (justify each policy).
2. Implement the run job as an `InteractiveJob`. Intermediate algorithm state must be VISIBLE:
   - trace records with verdicts for every attempt, candidate, element or iteration the algorithm evaluates;
   - progress counters and steps;
   - `consume_fuel(1)` per algorithm unit, so `step` is one meaningful unit;
   - `CheckpointReady` where resumable; bounded steps under the 8 ms interactive ceiling (2 ms target).
3. Mutating tools emit provisional `appendOps` + entities and implement `build_tool_run_job` + the finalize
   preparation factory. Nothing is written to the document before finalize (no `Emit::amend`, no per-step emits).
   Add a revalidate job when finalize must re-check against a moved head.
4. Delete the plugin's own run verbs, lifecycle enums, progress/cancel measures and local run chords (no compat).
5. Synchronous "solve inside render()" or "run to completion in one reducer" code moves into the job.

Tests first:
- a language-agnostic fixture run (seeded input → verdict/trace prefix, op counts);
- a third-party oracle for the algorithm's results where a mature library exists (name it and add it as a dev-dependency);
- start → complete → finalize = one undo entry;
- abort leaves the document byte-identical;
- worst `drive_step` under the ceiling on the plugin's largest example.

Make W1-D's predicates green for your plugin. Verify: native + wasm32-wasip2 checks with the crate's real features,
crate lib tests. Report `T/📓️wave-<lane>.md`.
