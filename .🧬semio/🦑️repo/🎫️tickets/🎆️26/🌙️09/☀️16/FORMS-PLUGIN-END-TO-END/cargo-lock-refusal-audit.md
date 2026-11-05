# Cargo Lock Refusal Audit

The retained descriptor owner (session 19405) reached its real native emitter prerequisite, `@semio-tech/os-plugin-describe-rs:build:prototype`, but refused root Cargo.lock freshness before descriptor emission. It remains live in the repository's automatic retry. No lockfile, cache, receipt, grant, pointer, or freshness guard was modified for this audit.

## Exact Observations

The affected file is `/Users/ueli/Documents/semio/Cargo.lock`. Five independent reads at 2026-11-05T03:40:15.658Z–03:40:16.670Z each observed 662021 bytes and SHA256 `8b229bd7451341d797d8d4d5994753d93700c0b7d9f3cce9760eb46d9cbe173b`. Device 16777229, inode 641369838, mtime and ctime 1793849587501477589 ns were unchanged before and after each read. The retained observations are in generated/cargo-lock-stability.json. This proves stability during that sample only.

The historical expected digest/byte count and the differing observed digest/byte count are **not present in the retained refusal output**. The current digest cannot honestly be substituted for either historical value. The failure identifies the bytes category, not a byte-extent calculation or a missing map record.

The emitter manifest is `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/Cargo.toml`; its package.workspace resolves to the repository root. Root Cargo.lock is therefore a relevant input.

## Exact Retry Boundary

In generated/integration-describe-cold-stable.log:

- Lines 54009–54019: this invocation reports root Cargo.lock behind its manifests, refreshes it, then reports it current again.
- After artifact-plan capture reaches 5097780 bytes / 391 units, lines 66269–66270 report provenance reads of 15244920 and 338 bytes.
- Lines 66271, 66336, and 66401: root Cargo.lock bytes changed; snapshot attempts 2/4, 3/4, and 4/4, with 2000/8000/30000 ms backoff.
- Line 66472: the outer command schedules a new snapshot (1/6) after 250 ms.
- Line 66506: “Restarting with the current implementation (1 of 6).”
- The same retained handle subsequently verifies a new 2043-closure graph. This is the owned automatic retry following a concrete emitter prerequisite failure, not an agent restart.

Four preceding declared prerequisites succeeded. Components itself did not emit descriptors in the failed attempt.

## Source Ordering

Repository compiler inputs await ensureRepositoryCargoLocksV1 before entering configuration views. Neutral captureCargoCompilationPlanV1 also awaits lock(false) before locked/offline metadata and retries lock(true) only on a diagnosed stale-lock failure. Therefore the simple hypothesis that initial planning always pins Cargo.lock before any owned lock refresh is contradicted by source order.

The native artifact plan captures its compilation graph and Source admission before cargoArtifactTargetRecipeV1 creates a fresh CurrentPhysicalOwnerV1 and hashes manifest/ancestor Cargo.toml, Cargo.lock, Cargo configuration, and toolchain paths. Each plan recheck hashes the recipe first, then recaptures the graph. Actual compiler capture also ensures the lock immediately before its locked Cargo child.

The lock service holds a per-workspace try-lease. Its probe is cargo update --workspace --locked --offline. A needed refresh uses update --workspace; receipt recording then runs metadata --no-deps --offline without --locked and hashes the resulting lockfile. This is a possible write-capable call, but the observed output does not establish that it changed the lock after the relevant physical claim.

CurrentPhysicalFilesV1.recheck compares fresh SHA256 and byteLength against its retained claim, then throws a generic bytes concurrent-edit error. That diagnostic does not include either comparison operand. Cargo target recipes retain an input SHA256 only once the exact target lease is acquired; there is no evidence yet that this attempt reached that point. No private custody was changed to recover unavailable evidence.

## Assessment

The output establishes a real freshness disagreement and automatic retry. It does not identify the writer or prove continuous peer churn. A stale retained claim, a later owned metadata write, and an actual peer write remain distinguishable hypotheses; none is proven by current evidence. No validator or retry-policy change is justified by the stable later sample alone. Preserve the existing owner and observe its next native boundary.

## Refined Observed Ordering

A further bounded extraction establishes that **recipe observation completed before the invocation's logged refresh**: line 54004 reports `[cargo-target-recipe] reading the target recipe inputs: 492 file operation(s), 3872086 of 3872086 bytes`; lines 54009–54019 then refresh root Cargo.lock; lines 54020–54021 start unit metadata/graph. Thus an initial early ensure does not rule out a subsequent owned write after observation. A recipe recheck performs its physical check before a fresh planning capture, and that capture can refresh a newly stale lock. Retaining the earlier recipe across such a refresh is a concrete potential self-invalidation boundary. It is not yet proof that the final refusal originated from that particular custody: the 4-attempt snapshot retry is in completed provenance capture, which creates fresh sessions. Historical comparison values remain necessary to distinguish the custody precisely.

This ordering replaces the earlier broad inference that source ordering alone excluded a late owned refresh. It does not establish continuous peer writes or justify relaxing any comparison.

## Recovered Exact Emitter Recipe

A bounded read of the two known compiler lanes found the exact emitter recipe at `.r/i/m/u/l/qda3ae2df7a6/t/83e508234b3407d9790391d36be0b87a/recipe.json`, mtime **2026-11-05T03:22:58.521Z**. Its manifest is the selected OS plugin descriptor emitter, and its root Cargo.lock input SHA256 is **`fbf9b1f7a2433bc68ac7bbdb2a0ec40c14149c51302f68c679b1ed845e9dc9c5`**. This differs from the stable current SHA above. The recipe retains no byteLength.

The matching current root-lock service receipt `.r/i/m/cargo/lock/25476eb0bd495fc96f5098b38fdc1bed14250529832fff67747b67f24dfab979.lock.json` was written at **03:33:14.751Z**, pins the current `8b229bd7…` digest, and lists 1942 manifests. Its observed lock mtime is **03:33:07.501Z**. This proves that the retained compiler recipe preceded the later owned refresh. The selected non-secret recipe fields are retained in generated/cargo-lock-recipe-evidence.json.

These are exact recipe expected/current digests; they must not be mislabeled as the unprinted final provenance comparison pair. The provenance retries still create fresh custody. No recipe or lock receipt was altered.
