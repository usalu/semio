# TimeTravel Replay Report Ownership — October 9

The fresh original BIM native receipt `🗑️generated/bim-n-before.log` stops at two E0277 errors: `ReplayReport: RetireOwned` is absent at the TimeTravelSession report field. This is compilation evidence; no Nextest law ran.

Current readback has the concurrent domain provider at `🧰️framework/🔨️modules/📡️replication/⚔️conflict/♻️report/🦀️.rs`, mounted by conflict. It declares field-wise controlled retirement for both ReplayReport and MutationReplayOutcome. The report owns its original outcomes Vec; outcomes own identities and messages; diagnostics retain their original text, targets and original unused backing. I preserved this provider without adding duplicate methods or aliases.

The existing language-neutral TimeTravel retirement fixture now carries the actual populated report rather than null: Unicode/NUL mutation identity, edit identity, error severity, diagnostic code/text and an empty target. The existing original native law asserts the complete ToValue report against this fixture before ownership transfer. The independent source law validates the fixture with Ajv and records ordered report outcomes and diagnostic values through SQLite. Diagnostic UTF8 bytes are kept explicitly as byte columns so embedded NUL survives the independent oracle.

Original native selector `test(time_travel_session_retirement)` exercises 18 combinations: original capacity 0/8192/65536, copy baselines 1/17/4096 and cancellation handoff cuts 0/3. Every demand query is observed allocation-free. Each five-currency shortfall retains the original owner without allocation/release. The original law issues copy capacity as max(baseline, exact next work demand), not a strict one-byte copy ceiling; other currencies use their exact observed demands. Each admitted turn compares the reported birth/release with the actual allocator event. Final physical conservation requires original bytes plus funded births equals all releases, with zero terminal drop.

## Genuine Original Receipts

- Native session65805 terminated0. Original Nextest executed 1/1 law,17 skipped,0.043s, all18 physical combinations. The owning router also passed its preexisting23 conformance tests,3408 expectations. Native log `🗑️generated/tt-native.log`; command/environment `📥️inputs/tt-native.json`.
- Every original case measured funded retirement births6980 bytes. Capacity0 retained1233 bytes and released8213; capacity8192 retained2326528 and released2333508; capacity65536 retained3014656 and released3021636. Every copy-baseline1/17/4096 and cancel0/3 combination conserved exactly, and every final owner drop measured0 allocation/0 release. Demand observers and individual five-currency refusal turns measured0/0.
- Independent source session93782 terminated0,1 law passed,16 expectations,313.11ms test runtime: Ajv validates the populated neutral report and SQLite preserves the original outcome order, Unicode identities, diagnostic UTF8 bytes and empty target. Log `🗑️generated/tt-source2.log`. First attempt71340 terminated1 because its relative path resolved inside Nx project cwd; no test ran. The corrected command uses an absolute path.
- Launch rows206.1922/1923 are registered in the seed. Registry generation12632 terminated130 at repo:generator-inputs: existing `Publication protocol digest mismatch: /nativeCodecs/9/protocolSourceSha256`. Log `🗑️generated/tt-launch.log`. Registry generation did not publish launch outputs; no generated-publication claim.

New touched paths and output roots were checked under the 256 codepoint and UTF16 code-unit limits. No checked-in source was copied to the ticket; no permanent script or runtime dependency was added. Shared Cargo target is preserved. Current free disk is17GiB.

## Remaining Universal Scope

These receipts can qualify the concrete TimeTravel prerequisite only. SnapshotClone still has an explicit unsupported publication-assembly return in build_sealer; funded factory birth and complete original request closure/publication receipts remain independent work. This ticket and its goal remain open.
