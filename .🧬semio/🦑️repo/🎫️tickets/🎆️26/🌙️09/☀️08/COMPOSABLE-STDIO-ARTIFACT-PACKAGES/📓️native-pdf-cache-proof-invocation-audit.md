# PDF Native Cache-Proof Invocation Audit

Captured 2026-09-09T14:33:10+02:00. This is a read-only audit of the root-owned invocation bodies in [📓️native-pdf-proof-invocation-review.md](📓️native-pdf-proof-invocation-review.md), the accepted run 26 receipts, and current receipt presence. It does not execute an invocation or establish a cache result for active runs 27 or 28.

## Hardened Invocation Recheck — 2026-09-09T14:42:45+02:00

The future source-probe and consumer bodies correct the earlier audit findings. They take receipt freshness from the invocation start timestamp and changed receipt stat before `capture`; source insertion holds a sidecar lease and cleanup requires the original inode and exact `original + marker` bytes before truncating only the owned tail; and the consumer accepts only `/usr/lib/` or `/System/Library/` runtime libraries. [🗑️generated/pdf-proof-helper-recovery-validation.json](🗑️generated/pdf-proof-helper-recovery-validation.json) records three passing synthetic recovery cases: absent output restored exactly, an existing output preserved, and changed backup rejected. These are helper checks only; no cache proof or source probe has run.

Two concrete issues remain in the documented invocation. First, `move_owned` writes its move receipt only after `output.rename(backup)`. A crash or receipt-write failure in that interval leaves the output absent with no receipt, and the terminal recovery code therefore concludes that no output was moved. Persist and fsync a `prepared` receipt containing the owner, nonce, backup path, and baseline before the rename, then mark it moved afterward; recovery can verify the backup inventory and decide safely whether a rename occurred. This applies to stage 29 and stage 32.

Second, the review document currently repeats the obsolete pre-hardening invocation bodies after the hardened ones (headings at lines 516, 615, and 737). Because the document calls itself the exact invocation input, an operator could execute the stale bodies, which lack the new recovery, freshness, source-cleanup, and runtime-linkage guards. Remove the duplicate legacy blocks before executing any proof.

## Final Mechanics Recheck — 2026-09-09T14:46:39+02:00

The five-block runbook now hashes to `d464d967214e6be242af85c8cfc33b4f3fb1a597f08ee476387d4ef1560f225a` and contains no duplicate legacy blocks. Pending moves now persist and fsync one prepared receipt before the rename, retain that phase intentionally, and restore the verified backup under the already-held lease on a post-rename exception. This resolves the earlier post-rename receipt-write gap for future stage 32. The three recovery fixtures were rerun, and [🗑️generated/pdf-proof-helper-marker-validation.json](🗑️generated/pdf-proof-helper-marker-validation.json) records the three source-cleanup cases. These remain synthetic helper evidence only.

The original in-memory stage 29 still has one specific journal-recovery edge case. Its old receipt was written after the rename without fsync. The terminal journal fallback reconstructs a receipt only when that path is absent; a crash can leave a present but truncated or malformed JSON file, causing `json.loads` to abort before the unique-prefix backup can be inspected and restored. Treat an unreadable or invalid original receipt the same as an absent one, then require the journal's one candidate, 32-hex nonce, exact owner, and exact successful-inventory match before creating the reconstructed receipt. This exception path applies only to the already-running historical loop, not the prepared future helper.

Apart from that original-loop malformed-receipt case, the prepared-move ownership, fresh-receipt, source-tail cleanup, and system-only consumer-linkage mechanics have no further concrete defect in this read-only review. No runtime cache, source-isolation, or consumer acceptance is implied.

## Accepted Baseline And Local-Hit Criterion

Run 26 is the only current accepted publication baseline. [🗑️generated/pdf-native-cache-26-owner-bundle-recovery-local-run.json](🗑️generated/pdf-native-cache-26-owner-bundle-recovery-local-run.json) records four status-zero tasks. Its PDF task is `@semio-tech/stdio-pdf-rs:build`, hash `4030302576088414454`, and `cache-miss`. [🗑️generated/pdf-native-baseline-26-owner-bundle-recovery-local-inventory.json](🗑️generated/pdf-native-baseline-26-owner-bundle-recovery-local-inventory.json) records the declared PDF owner, 200 files, and 576,961,088 bytes. [🗑️generated/pdf-native-26-format-closure.json](🗑️generated/pdf-native-26-format-closure.json) limits its observed stdio closure to PDF, Binary, Deflate, and the shared contract with complete `rlib`/`rmeta` pairs.

The stable loop is correctly shaped to make a local hit meaningful: it requires exactly one successful PDF task and no failing task, then, after an observed `local-cache-hit`, moves the whole publication aside, reruns with the same isolated Nx state, requires the same task hash and a second `local-cache-hit`, and requires full inventory equality. The inventory compares every regular-file relative path, size, and SHA-256, including the owner marker; it also verifies that marker's declared file set and rejects symlinks. The prior invocation-13 inventory is only the pre-loop guard: each successful loop stage replaces the baseline, so run 27 and 28 compare against run 26.

`run(stage)` should also bind its consumed `nx-cache-root-pdf-native-proof/run.json` to the subprocess it launches. At present it opens a fresh stage log but neither removes nor snapshots the fixed run receipt before `Popen`, and `capture` does not require a new inode or modification time after launch. Thus a status-zero receipt could theoretically be stale if the router exits zero without refreshing that file. A future guarded invocation should remove the receipt or retain prior stat data, record `started_ns`, and require a newly written receipt whose modification time is after launch before accepting its task fields. The active loop cannot be retrofitted in memory, so this needs a separate current verifier before any later acceptance claim.

Neither [🗑️generated/pdf-native-local-restoration-proof.json](🗑️generated/pdf-native-local-restoration-proof.json) nor [🗑️generated/pdf-native-source-isolation-proof.json](🗑️generated/pdf-native-source-isolation-proof.json) exists. Therefore no local-hit, restoration, or source-probe result is accepted yet.

## Ownership And Restoration

`move_owned` is otherwise appropriately bounded: it obtains an exclusive sibling lease, re-inventories the exact owner output before moving it, atomically renames only that output directory into the ticket's generated directory, records the move receipt, and refuses symlinked ancestors, entries, or an unexpected owner marker. It neither moves a Cargo target tree nor a source file.

There is one actionable recovery gap before source probes. In both restoration stage 29 and sibling-source stage 32, the backup is removed only after a successful rerun and assertions. If `run`, `capture`, or a subsequent assertion raises after the rename, `dist/build` remains absent and the retained baseline backup is not restored. The minimal repair is a guarded failure path around each post-move sequence: if the output is still absent and the backup still exists, rename the backup back, re-inventory it against the recorded baseline, then re-raise. It must never overwrite a newly published output. This keeps the backup receipt while preserving the usable publication after an unsuccessful proof attempt.

## Own And Sibling Source Probes

The planned checks are semantically sufficient once executed:

- A unique appended PDF marker must produce a different hash and `cache-miss` (stage 30).
- Removing that marker must recover the baseline hash, `local-cache-hit`, and byte-identical inventory (stage 31).
- A unique appended JPG marker must leave the PDF hash unchanged, produce `local-cache-hit`, and restore the moved PDF inventory exactly (stage 32).

The probes use the actual PDF and JPG crate-root files and remove only the generated marker. There is a small shared-workspace race: `remove_probe` reads the source, checks marker multiplicity, and writes the replacement without a compare-and-swap or cooperative source lease. A concurrent writer between the read and write can be lost. The current marker count protects against a pre-existing duplicate but does not make the later write exclusive. At minimum the commands should detect an unexpected source-content change immediately before cleanup and preserve the marker for manual recovery rather than overwrite it; a cooperative adjacent lease, analogous to the output lease, would make the intended ownership protocol explicit.

## Restored Publication Consumer

The consumer algorithm uses only the ticket fixture source and the restored PDF publication for artifact inputs. It derives a sorted `serde_json` pair from the verified inventory, asserts the selected PDF and serde `rlib`/`rmeta` entries are present, and invokes `rustc` with only `-L dependency=<publication>` and `-L dependency=<publication>/deps` plus explicit `--extern` paths under that publication. It runs the ticket-generated binary and finally re-inventories the publication, so the consumer cannot silently mutate deliverables.

The current `otool -L` check only rejects a runtime path containing `cargo`. That proves no Cargo-target runtime linkage, but does not rule out every other unexpected non-system absolute dependency. If the claim is strictly that the restored external consumer links only publication artifacts plus the platform runtime, tighten the assertion to the expected macOS system loader/library set (the prior consumer record indicates only `libSystem.B.dylib`) or explicitly reject all non-system absolute paths. This is a proof-strength improvement; the controlled `rustc` search and `--extern` inputs already constrain the artifact libraries to the publication.

## Result

No defect blocks interpreting run 26 as a successful publication baseline. The two proposed pre-probe changes are failure restoration for moved output and safe source-marker cleanup; neither active run 27 nor 28 has been counted as a local cache hit. The restored-consumer linkage assertion can be tightened before claiming an absence of all non-publication runtime dependencies.

## Coordinator Guarded Helper Validation

Executed three output-recovery fixtures: restore an absent publication from an exact owned backup; preserve an existing publication and backup; reject a tampered backup without publication mutation. All three passed. Executed three source-marker fixtures: remove only the exact appended marker from an unchanged inode; preserve concurrent content and require scoped recovery; preserve a replacement inode and require scoped recovery. All three passed. Python syntax validation accepted all four guarded invocations. Receipts are `🗑️generated/pdf-proof-helper-recovery-validation.json`, `🗑️generated/pdf-proof-helper-marker-validation.json`, and `🗑️generated/pdf-proof-guarded-invocation-syntax.json`. These checks validate proof mechanics; native local-hit/restoration/source-isolation/consumer acceptance remains pending.

## Original Loop Receipt Reconstruction

The terminal handler now treats absent, unreadable, malformed and structurally invalid original stage29 move receipts as candidates for journal reconstruction. It requires the durable pre-move journal, at most one stage-specific nonce backup, exact owner/inventory equality with a successful accepted stage, and preserves a damaged receipt under its own unique evidence name before installing the reconstructed receipt. Missing output without a uniquely recoverable backup fails explicitly. This handles the in-memory original loop; future probes persist the prepared receipt before renaming. Current terminal handler syntax was validated; actual native restoration is still pending.

## Original Malformed Receipt Runtime Check

The original-loop terminal handler executed against a temporary malformed JSON move receipt with absent publication and a uniquely owned backup. It reconstructed the receipt using the durable journal and an exact successful inventory, preserved the damaged receipt, and restored the publication byte-exactly. The one-case runtime check passed: `🗑️generated/pdf-proof-original-receipt-reconstruction-validation.json`. The pending consumer also now requires its complete starting inventory to equal the accepted stage32 restored inventory before compilation, closing the gap between the restoration and consumer invocations. Actual native PDF cache acceptance remains pending.
