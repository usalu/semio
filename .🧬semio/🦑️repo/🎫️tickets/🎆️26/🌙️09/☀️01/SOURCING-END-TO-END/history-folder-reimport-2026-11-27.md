# History journey: folder helpers, alternatives list, Open File (2026-11-27)

## Failures and fixes
1. `attachFolder is not defined` (and `folderFile`): helpers did not exist. Added shared `attachFolder(s, folder)` to `journey-kit.ts` (forgets a remembered folder via `#s-folder-forget`, opens the sync menu only as needed, handles the collapsed form after Detach) and `folderFile` to `journey-history.ts`. Now PASS (en): offline spell, detach offered, detach/resume.
   - Second-context step: the old expectation (peer sees the first context's rows) contradicts the design (a folder is persisted-local-only, see 🕰️history-io-features.md row 134; live sharing needs the hub). Step rewritten to assert the design: peer keeps its own document, its edit appends to the shared archive; PASS (en).
2. Alternatives "only the main line before creation got 0": product hides the Alternatives section while there is <=1 alternative (`history.alternatives.len() <= 1`, `⏪️time-travel/🪟️panel/🛬️lifted/🎮️command/🦀️.rs`). Journey expectation was wrong: now expects 0 rows before and 2 (main + new) after Create Alternative. PASS (en).
3. Open File yields []: root cause 1 (fixed, TS): the sourcing dev host used the stdio-generated format catalog, which routes `.json` to `s.stdio.json@rfc8259/*` (editor not loaded) -> "no surface registered". Added `formatCatalogOwnedByV1` in `✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🟦️.ts` and used it in `✏️s/🧑‍💻dev/🎭️variants/🪵️sourcing/🚀️entry/🟦️.ts` to route the JSON kind to `s.sourcing.curation@1/*`. Routing now works.
   Root cause 2 (open): after routing, the guest natural-file import (`media-in`) never completes: host warns "continuation N/32 ... guest status more-work" with N growing ~5000/s (a yield livelock in `NaturalFileImportJob`, `🔌️plugin/📜️declaration/📥️natural-file/🦀️.rs`; suspect Decode stage `units > fuel_remaining -> yield` for the 4.6 KB input, or Publish wait). Added a temporary `[DEBUG] natural-import step` print in `step_original`; rebuild `semio-hub-sourcing-component` queued via rebuild-sourcing-retry.sh (output `🗑️generated/hf-rebuild.out`), waiting on another loop's build lock at report time. The DEBUG print MUST be removed once the cause is read.

## Files changed
- journey-kit.ts, journey-history.ts (ticket)
- ✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🟦️.ts, ✏️s/🧑‍💻dev/🎭️variants/🪵️sourcing/🚀️entry/🟦️.ts
- 🧰️framework/.../📥️natural-file/🦀️.rs (temporary [DEBUG] only)
- probe-open-file.ts (scratch probe, ticket)

## Final lines (en, targeted runs)
PASS folder sync offline; PASS detach offered; PASS detach/resume; PASS second context; PASS create alternative; FAIL Open File (import livelocks in guest). de not rerun separately (same code paths).

## Update: native reproduction attempt (coordinator request)
- A faithful Rust unit test of `NaturalFileImportJob` is not feasible with existing precedent: the job is built from an `ArtifactReservedToolJobRequest` carrying an `ArtifactToolCompletion` with a mounted consumer plus a `StepContext`; no test in the repo constructs one (the natural-file lifecycle tests only check codec metadata), and a hand-built fake would not prove the wasm behaviour.
- Static analysis of `NaturalFileImportWork::step_request` (natural-file `🦀️.rs`): the only silent no-progress (`Ok(None)` = guest "more-work") paths are (a) job creation when `should_yield`/`fuel==0`/`depth==0` or `RetainedCloneProgress{size_of::<NaturalFileImportJob<E>>()+...}` does not fit the retained grant, and (b) `E::admit_natural_file_decode_owner` returning `Ok(false)` (`should_yield`, zero grant, or `maximum_copy_bytes < size_of::<NaturalFileDecodeOwner<Snapshot>>()`). Note also that the sourcing editor leaves `SINGLE_TURN_NATURAL_DECODE=false` (only xlsx/pdf set it), so the default `admit_natural_file_decode_owner` should refuse, not spin; which of these the wasm hits is not determinable statically.
- Therefore I added a second temporary print (`[DEBUG] natural-request ...` at the top of `step_request`: job/decoder presence, fuel, yield, grant, `size_of` job, single_turn flag). Both DEBUG prints (`natural-import step`, `natural-request`) are in the natural-file `🦀️.rs` and MUST be removed after the queued rebuild (hf-rebuild.out) shows the cause in the browser console (run `probe-open-file.ts`: `JOURNEY=probe FILE=<saved json> bun probe-open-file.ts en`, filter console for `[DEBUG] natural-`).
- The build is still queued behind other loops' builds on `build.lock`; the root-cause fix is therefore still open.
