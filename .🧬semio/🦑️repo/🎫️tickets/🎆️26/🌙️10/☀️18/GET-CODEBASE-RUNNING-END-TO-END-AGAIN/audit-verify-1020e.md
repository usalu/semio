# audit-verify-1020e: live state of the sets applied today vs their result files

Auditor `aud-verify-1020e`. Read-only except this file. Checks: file reads, `apply.py` dry runs, sha256 of live files, and five single-file `nice -n 10 bun test` runs (T1-T5 below, all green). No `bun repo`, builds, git-modifying commands, or repo-wide greps.

## Side effect to know about
My dry run of `input-precision-tools/apply.py` (set 1) calls `gen-rules.py`, which rewrote `input-precision-tools/staged2/rules.json` (default output path). The rewritten file matched the live `rules.json` when checked afterwards. Nothing under the live tree was written.

## Per set

| set | verdict | 3 claims checked (evidence) |
| --- | --- | --- |
| round-2 groups (`result-integrate2.md`, ledger `🗑️generated/round2-apply/ledger.json`) | VERIFIED for content, CONTRADICTED on its "not applied" list | (1) 223 of 255 last-writer files hash equal to the ledger `after`. 32 differ: 10 explained by later sets (cli-zt 7, bzl-hermetic 2, testfix-real 1), 7 by documented staged2/E1 edits, 15 unexplained (mem-gov 3, boot-sweep 2, testfix-defect 1, parity 1, fixup-rbe-probes 1, bzl-default README 1, sync-incr 1, testfix-law 4, pg-chain project-assembly 1). (2) `⚡️caching/🔣️policy.json`: none of the 4 `driver-owned` names remain (fixup-policy-driver). (3) E1 edit in `📥️caller/🧪️tests/📥️birth/🟦️.ts` present (AsyncLocalStorage, retainedCompilerStores). Minor: the cli suite names `runAdmittedBuildInvocationV1` but not `runAdmittedBuildCliV1` as the result text says. Ledger "NOT applied" list (cli-zt, bzl-hermetic, graph-speed) is contradicted: all three are live-applied. |
| cli-zt (`result-cli-zt.md`) | VERIFIED, with 2 drifted files | (1) `apply.py` dry run: 287 already, 0 create/replace/edit. 2 files match neither planned pre- nor post-hash, so a peer changed them after apply: `🏃️process/🦀️cargo/🧪️testing/📦️executable/🟦️.ts`, `🏗️build/⚙️operations/🔣️catalog.json`. (2) `commands/🗂️registry/🔣️.json`: 25 commands, 14 `none`, 11 `dependencies`, as claimed. (3) T1 `registry/🧪️tests/🟦️.ts`: 42 pass, 0 fail. |
| graph-speed (`result-graph-speed.md`) | VERIFIED (ledger contradicted) | (1) `graph-speed-tools/apply.py check`: ops 38, pending 0, new files 25. (2) `⚡️caching/🗃️records/🟦️.ts` has `RECORD_STORE_DESCRIPTOR_LIMIT = 32`. (3) T2 `🧭️file-owner/🧪️tests/🟦️.ts`: 6 pass. The result header says "APPLIED live" while its body still says "apply only after GO", and integrate2 lists it as NOT applied. |
| input-precision staged2 (`result-input-precision.md` follow-up, `input-precision-tools/staged2/`) | VERIFIED, with a count discrepancy | (1) live `🏗️build/📥️inputs/🎯️precision/` `🟦️.ts`, `🧪️tests/🟦️.ts`, `🧬️schema/🔣️.json`, `🔣️.json` are sha-identical to staged2 `precision.ts`, `test.ts`, `schema.json`, `rules.json`. (2) storage schema admits `(?:np|ip)/[a-f0-9]{24}\.json`. (3) T4 precision suite: 11 pass, 1 skip, 0 fail (the result says 12 pass). Set 1 `apply.py` now refuses 4 files ("exists with other content"): expected, staged2 superseded them. |
| bzl-hermetic (`result-bzl-hermetic.md`) | VERIFIED, with two flags | (1) `bases.json` has 16 entries; `🏃️execution/🟦️.ts` contains the new `export{BAZEL_HERMETIC_DEPENDENCIES,BAZEL_HERMETIC_RUNNER}` line. (2) `🏃️execution/🔒️hermetic/` holds `🔣️.json`, `🚪️entry`, `📦️dependencies`, `🔌️providers`, `🧪️tests`, `🟦️.ts`. (3) T5 `hermetic/🧪️tests/🟦️.ts`: 5 pass. UNVERIFIABLE: the real-Bazel class law and real-target probes (not run). Flags: `apply.py` is not idempotent (crashes with "anchor found 0 times" instead of "already applied"); integrate2 lists the set as NOT applied. |
| testfix-real (`result-testfix-real.md`) | VERIFIED, PARTIAL on the two lock files | `apply.py` dry run: 35 ops, 33 already applied, 2 drifted, both root `Cargo.lock` and `🧰️framework/Cargo.lock` anchors. (1) New crate `🧳️retained/…/📦️packages/🦀️rust/Cargo.toml` names `semio-framework-replication-mutation-retained-field`. (2) Both lock files contain that name 3 times, so the crate is registered, but their anchors no longer match the planned text. (3) `🧱️row/🦀️.rs` is deleted. Flag: the 2 lock edits are not in the applied state. |
| conc-tests (`result-conc-tests.md`) | VERIFIED live, CONTRADICTED ledger | The result says READY/NOT APPLIED (29 files). Live dry run: "0 file(s) change", 19 new files declared, edits "already applied", so the set is live. (1) `🧪️testing/🏖️sandbox/` has `🔣️.json`, `🧬️schema/🔣️.json`, `🔌️ports`, `📁️root`, `🛡️guard`, `🧪️tests/`. (2) cleanup `🎛️command/🧪️tests/🟦️.ts` uses the sandbox API and no longer contains `MODULAR-MONOREPO`. (3) T3 sandbox `🧪️tests/🟦️.ts`: 16 pass, 0 fail. No record of who applied it: flag. |
| cargo-fix static (`result-cargo-fix.md`) | VERIFIED, with 1 drifted file | Dry run: 77 ops, 76 already applied, 1 drifted: `…/🍎️shell-tests-input-1/📦️packages/🦀️rust/Cargo.toml` matches neither the staged old nor new hash, so it was changed after apply. (1) `🧊️generation3d/🦀️.rs` = staged new hash. (2) `🔌️plugin/🦀️.rs` = staged new hash (facade re-exports). (3) UNVERIFIABLE: compile claims (no cargo run). |

## Ledger mismatches to flag
- integrate2 "NOT applied" list (cli-zt, bzl-hermetic, graph-speed) is wrong: all three are live-applied.
- conc-tests is live-applied while its result file says READY/NOT APPLIED.
- Drifted after apply, changed by a peer or unrecorded writer: cli-zt 2 files, cargo-fix 1 file, testfix-real 2 lock files, integrate2 15 files.
- staged2 superseded set-1 input-precision files (documented, expected).

## Test runs (single file each, `nice -n 10 bun test`)
- T1 cli-zt `commands/🗂️registry/🧪️tests/🟦️.ts`: 42 pass.
- T2 graph-speed `🧭️file-owner/🧪️tests/🟦️.ts`: 6 pass.
- T3 conc-tests `🏃️process/🧪️testing/🏖️sandbox/🧪️tests/🟦️.ts`: 16 pass.
- T4 input-precision `🎯️precision/🧪️tests/🟦️.ts`: 11 pass, 1 skip, 0 fail.
- T5 bzl-hermetic `🔒️hermetic/🧪️tests/🟦️.ts`: 5 pass.
