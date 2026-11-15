# Undefined Stale Probe Flood

Date: 2026-11-13, worker `g1-m07`.

## Cause

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs` line 1139 (`// [DEBUG] hitcost`) appended one JSON line per stale closure entry to `process.env.PROBE_OUT + ".stale"`. Without `PROBE_OUT` that is `undefined.stale` in the current directory, so every graph inference on any machine grew the root file (37 MB at 23:16) and the replica stored each rewrite as a new blob.

## Is The Probe Still Needed

No. `hitcost.md` in ticket `26/11/10/GET-NORM-PLUGIN-WORKING-END-TO-END` ends with a result section, says "probes removed" (09:10) and `coordination-plan.md` records "09:30 hitcost ended". The successor `farmhits.md` also ended with its probes removed. `PROBE_OUT` is only read by ticket drivers (`hitcost-driver.ts`, `farmhits-driver.ts`, `graphmem3-part`, `graphmem4-part`), which write their own files and only read `globalThis.__hc` for a log line (spreading `undefined` is harmless).

## What Was Done

- Removed all hitcost probes with `remove-hitcost-probes.py` (exact-match replacements, asserts that no `__hc`, `hitcost`, `PROBE_OUT` remain):
  - `🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`: 9 probes (the `.stale` writer, counters, timing wrappers around `derive`, `closureEntryHoldsAsyncV1`, `answer`, `settle`).
  - `🏗️build/🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs`: 1 counter.
  - `🏗️build/👁️observation/🟨️.mjs`: 3 counters.
  - Diff: 3 files, 7 insertions, 12 deletions, probes only. `node --check` passes for all three.
- No source outside ticket folders mentions `PROBE_OUT` any more.
- Deleted root `undefined.stale` at 23:31:17.

## Verification

- File absent at 23:33, 23:37 and 23:38 (over 7 minutes, polled every 30 s for 4 of them); it was last written 23:16, so no process was appending at deletion time.
- `repo ./📜️script.ts verify --lang en` in `🔁️replica` (23:33, 273097 paths): `undefined.stale` not reported. Exit code is still 1 because of 4 unrelated paths that were being edited by others at that moment: `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` (content) and three files under `🏗️build/🌐️remote-cache/🛰️fleet/` (state).
- Tests (`repo tools exec bun -- test`): closure-facts 22 pass, layer-stream 4, durable-entries 4, late-closures 4, shared-prime 2.

## Findings For Others (Not Fixed Here)

1. The farmhits fix is missing from the shared worktree. `farmhits.md` (13:40) describes `inPassRowsV1` in `script-inputs` and `factTooLargeV1` in `observation`; neither identifier exists anywhere in the build library, also not in the version staged at join time. The test `🧪️tests/🧮️pass-verdicts` is present and fails exactly as that report predicts for the old code (Expected 40, Received 240). Together with the hitcost probes that were reported as removed but were back, this means both library files were overwritten by an older copy (state of about 09:00 on 2026-11-13) — work was lost by last-writer-wins or by seeding the replica from an older tree. The 13:40 fix has to be recovered from the machine that wrote it or redone.
2. `🧪️tests/♻️owner-reuse` fails here with `library-test-checkout-required` (environment, unrelated to this change).
3. Processes on other machines that loaded the old module before the edit replicated keep the probe in memory until they restart; none wrote during the watch window.
