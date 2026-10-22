# audit graph-proof2 (wave 1023, auditor) - live proof of graph reuse

Ticket: `GET-CODEBASE-RUNNING-END-TO-END-AGAIN`. Source: `C/result-graph-reuse.md` section 4 ("exact proof commands for later"). Tools: `C/graph-proof2-tools/` (`show.sh`, `newline.py`). Outputs: `C/🗑️generated/graph-proof2-*`.
Verdict: the live proof did NOT pass. The reuse path was not observed. The first post-fix run did a full refresh, and the two concurrent readers never reached their result (queued for exclusive repository-graph-facts behind peers, stopped by me at 01:18).

## Result table

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | Before start, no exclusive `repository-graph-facts` holder | PASS | `C/🗑️generated/graph-proof2-leases-pre.txt` (00:03): no graph-facts line; 0 waiting |
| 2 | Baseline `nice -n 10 bun repo show targets --json` exits 0, valid JSON | PASS | rc=0, wall 1822 s (00:04:04 to 00:34:26); `graph-proof2-base.json` parses |
| 3 | Baseline is a delta, not a full refresh (may refresh once) | FAIL | `graph: refreshing 3480 projects (generation 27)`; ends `generation 27 published, 3744 projects refreshed in 1670 s`. Only about 140 stale inputs were listed (peer edits: receipt `.ts`, renderer-wgpu `.ts`, `Cargo.toml`, `project.json`), yet every project was re-inferred |
| 4 | Newline edit applied with sha check before | PASS | `🟨️.mjs` sha 33ce24a4... (6044 B) confirmed, then 6045 B (81ac8c24...) |
| 5 | Two concurrent `show targets --json` finish, graph "generation N verified", no refresh | FAIL (not reached) | Both started 00:34:53 (wrappers 94397, 94426). Blocked first on dependency receipt: `installer input changed ... nx@23.2.0.patch` (peer edit, mtime 00:31). Then both queued exclusive `repository-graph-facts:/Users/ueli/Documents/semio:shard:7` and `build-graph-state` behind peer holder 31259 (held 1289 s at 01:18) and peers 99069, 94030, 44060 |
| 6 | Readers take no exclusive lease when the generation is verified | NOT OBSERVED (FAIL in practice) | Both readers requested EXCLUSIVE graph-facts and build-graph-state. That is consistent with the stale dependency receipt and the peer refresh, not with a clean reader path |
| 7 | Inference counts | baseline: 3480 inferred, 3744 refreshed; c1/c2: none reached | `Inferring projects: 3480 of 3480` (base) |
| 8 | Stale reasons | baseline: source analysis / input file changed (peer edits); c1/c2: dependency generation receipt stale (`nx@23.2.0.patch`) | `graph-proof2-base.err`, `graph-proof2-c1.err` |
| 9 | `show targets --json` stdout stays pure JSON under contention | FAIL | c1/c2 `.json` start with "Another process is building the dependencies (10 s)..." and are invalid JSON. Progress text goes to stdout |
| 10 | Newline restored byte-identical | PASS | `newline.py restore`; sha 33ce24a402abb44b1ecec83e850e4a450cbca3058939983b17cfa36f63b029a0; `cmp` equal to `graph-proof2-newline-backup.mjs` |
| 11 | Stray image `🧰️framework/.🧬semio/🦑️repo/⚡️cache/rust/fixed` holder | NO HOLDER NOW; holder pid not recovered | 00:04 `hdiutil info` listed it mounted (disk43s1 on its `m`). 00:14 gone from `mount` and `hdiutil info`. `lsof -- fixed.dmg` and `lsof +D <m>` empty. Its holder pid at 00:04 was in a truncated block and cannot be read back. Not detached by me. The root-path `.🧬semio/🦑️repo/⚡️cache/rust/fixed` (disk17) is still mounted; its lease `repo:compiler-writer` is WAITING (not mine) |
| 12 | My processes all stopped | PASS | Queued runs stopped child-first at 01:18 (96459, 98676, 94406, 94430, 94405, 94429, 94397, 94426). `kill -0` gone for all eight. Baseline 41447/41450/42192 exited on completion |

## Timeline
- 00:03 leases pre-check clean (no graph-facts holder).
- 00:04 baseline launched (`graph-proof2-tools/show.sh base`, detached via `chain-tools/detach.py`).
- 00:14 baseline still inferring (716 then 2197 of 3480); stray framework image already unmounted.
- 00:31 peer changed `🩹️patches/nx@23.2.0.patch` (mtime).
- 00:34 baseline done (rc 0). Newline added. c1/c2 launched.
- 00:35-01:00 c1/c2 wait on dependency build (peer 94030 exclusive `javascript-dependency-build`) and dependency receipt.
- 01:09-01:18 c1/c2 queued exclusive behind peer holder 31259. Stopped by me. Newline restored, sha verified.

## Open / for the next owner
- The proof needs a quiet dependency generation (no peer edit of `nx@23.2.0.patch`) and no queue of peer full refreshes ahead. Re-run after the peers finish. Then judge rows 3, 5, 6 and 9.
- Row 3 is the real signal: the first post-fix run re-inferred all 3480 projects for about 140 stale inputs. Check whether `sourceAnalysisFactHoldsV1` or the stale-plan logic still marks every owner stale. The result file's "fix LIVE" claim is not confirmed by this run.
- Row 9 is a separate defect: `show targets --json` writes waiting progress to stdout. It should go to stderr.
