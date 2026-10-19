# VHD Data Audit — 2026-10-19

## Purpose and Ownership

The VHDs implement the repository's physically bounded Native build/cache storage on Windows. They are separate storage domains, not one arbitrary application disk. The authoritative map is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json:5`:

| Domain | Directory | Consequences of discarding its image |
|---|---|---|
| Rust | `.🧬semio/🦑️repo/⚡️cache/rust/fixed` | Compiler workspaces/state and build artifacts must be rebuilt. |
| Tools | `.cache/b/t` | Provisioned tooling/cache must be restored or downloaded again. |
| Graph | `.cache/b/g` | Build graph/pool data must be recomputed. |
| Dependencies | `.cache/b/d` | Materialized dependencies must be restored or installed again. |
| Products | `.cache/b/p` | Artifacts can be rebuilt, but this partition also contains explicitly non-recomputable development state. Blanket deletion can lose that state. |
| Diagnostics | `.cache/b/o` | Temporary diagnostic outputs, logs, and test scratch can be lost. Historical observations cannot necessarily be regenerated exactly. |
| Nx | `.cache/b/n` | Nx cached results/state must be regenerated. |
| Leases | `.cache/b/l` | Cross-process lock/queue databases and synchronization identity are lost. Removing them while participants run breaks coordination safety. |

Evidence for the lease image: budget policy lines 42–45 assigns `.cache/b/l` a 67,108,864-byte capacity and 66,060,288-byte logical envelope. `.../📦️budget/📥️opening/🟦️.ts:30` resolves its `m/data` as the lease directory; lines 42–55 open the Native lease image first, then retain it while opening other cache partitions. Line 41 explicitly permits idle-lease retirement only when no other invocation holds the image. `.../⚡️caching/💾️storage/🔒️enforcement/🟦️.ts:14` likewise fixes the shared lease directory to `.cache/b/l/m/data`.

## Non-Recomputable Product Data

Do not describe every VHD as disposable cache. `🧰️framework/🔨️modules/🏃️process/💾️state/📋️layout/🟦️.ts:7` explicitly calls its envelope non-recomputable development state. Lines 12–15 locate the development namespace and exclude it and its binding/intent records from recomputable publication/restore targets. The state schema `.../💾️state/🧬️schema/🔣️.json:11–20` reserves a 1 GiB filesystem at `persistent/development`, with `data` and parent binding/intent records. `.../📦️budget/📥️opening/🏪️store/🟦️.ts:3,40` deducts that persistent charge from the products artifact budget. `.../📦️budget/📥️opening/🟦️.ts:81` exposes the dedicated development-state partition callback.

These declarations prove capacity and intended ownership, not that any particular user's development records currently exist. I did not mount or enumerate product images, so the amount of actual user data is unverified.

## Deleting the Existing Lease VHD vs Removing the Backend

The originally observed `.cache/b/l/fixed.vhd` was a 66,060,800-byte incomplete birth, inactive according to Windows `Get-DiskImage`, with an empty underlying mount directory and no `image.json`. It is specifically the lease domain; it is not the products data image. The empty underlying mount directory alone did not prove the VHD's internal bytes contained no files, because an inactive volume's content is not visible there. No filesystem inspection was performed.

On the latest read-only check, `.cache/b/l` no longer exists and Windows reports its image path absent. This audit did not remove it. Prior measurements describe historical state and must not be presented as the current disk inventory.

Deleting a physical image leaves the backend feature configured: the next owned cache opening still attempts to create/mount it. Removing the backend properly is a code/schema/policy redesign of physical bounds, publication/receipt identity, image lifecycle, cross-process leases/queue custody, and persistent-state protection. It needs replacement storage ownership and capacity guarantees, plus an explicit preservation decision for any non-recomputable data. Merely deleting files neither disables this feature nor repairs its guarantees.

Current fixed storage recovery/removal is custody checked: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:148–159` requires original receipt, image identity, mapping, exclusive lease, and closure before detaching/removing; `:187` refuses an existing directory without its valid receipt. Those checks explain why an interrupted receipt-less birth needs its own canonical recovery rather than fabricated metadata.

No code, image, cache, or lease database was modified. No assertion that all VHD contents are reconstructible is supported by this audit.
