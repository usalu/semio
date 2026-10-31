# Independent Compiler Maintenance Boundary Audit

Read-only source review, 2026-10-29. No preview, retirement, build, test, cache mutation, process cancellation or live storage operation was executed. Only the report was written. Canonical target naming changes belong to the Rust lane and were not edited.

## Conclusion

No additional concrete authority bypass or deletion defect was established in the reviewed declared maintenance path. The closed canonical preview route is suitable for the owner-authorized preview with the limitations below. Retirement remains unreleased. A successful preview is diagnostic DATA, not a deletion permit and not proof that descriptor-qualified retirement will succeed.

## Genuine Entry and Early Recovery

Library-relative paths below start at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

- `🏗️build/🚀️bootstrap/⌨️cli/🟦️.ts:15–21` selects a fixed project/target pair, accepts only authored options and bounded parallel capacity, reads the bounded exact owner manifest, and validates its authored command/dependency/source shape.
- The actual entry validates its owned invocation before parsing (`🏗️build/⌨️command/⚙️runtime/🟦️.ts:18`). Recovery detection takes the special branch before generic command diagnostics (`29–31`), and recovery dispatch returns before ordinary graph execution (`33–34`).
- The recovery executor requires the actual incoming Original/control association (`bootstrap/⌨️cli:33–40`). Its injected composition root loads only the selected maintenance owner (`bootstrap/⌨️cli/🔌️ports/🟦️.ts:4–8`).
- The maintenance owner again verifies invocation/control, narrows the same deadline, and derives build mode from Original capabilities (`⚡️caching/🧹️pruning/🏪️budget/📜️script.ts:22–26,38`). No caller path, arbitrary partition, byte limit, or forwarded selector is accepted by the declared two targets.

This establishes source-level independence from generic graph/Dependencies admission. It does not establish that every transitive source import or Native opening can execute under the current host storage state. No runtime success is claimed.

## Storage and Exclusion

The owner opens the fixed rust partition and obtains a genuine compiler workspace reservation before entering exclusive compiler writer custody. Its layout supplies the build and target directories (`budget/📜️script.ts:28–29`). Nested execution authenticates that reservation, its kind and exact mode-derived paths (`budget/⚙️execution/🟦️.ts:41–46`).

Compiler writer custody uses real image rechecks, an exclusive resource lease by default, and retained operation lifetime (`⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts:37–40,71–72`). Shared compiles retain shared custody; standalone maintenance waits for exclusive custody. Maintenance additionally acquires the cache-prune EX plus every normalized build/target Cargo store EX (`⚡️caching/🧹️pruning/🟦️.ts:50–54`).

Retirement requires complete process-observation exit, physical Cargo lock custody, mounted scope checks on macOS, Native descriptor witnesses, selected-descriptor-name checks, and a recensus under custody (`budget/⚙️execution:50–59`). Changed size/recency or working ownership refuses. Windows takes the defining conditional backend branch (`53`); its full implementation was not independently audited in this pass.

## Preview Is Diagnostic, Not Globally Read-Only

The preview flag skips descriptor retirement planning and returns immediately after writing the plan (`budget/⚙️execution:46,52`). No `deleteUnit` or Windows retirement call is reached. It reports completed units/logicalBytes as zero and separately reports planned counts/bytes/unmet bytes. Physical available bytes come from the current Native filesystem interface.

It still opens/reserves the rust image, acquires custody, creates diagnostics directories, captures process output and writes plan.json. Without an explicitly provided artifact directory, output is placed under the diagnostics store (`budget/📜️script.ts:31–34`). These are real owned metadata effects.

On macOS the common opener can also inspect and settle an inactive publication through its owned reset callback (`⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts:135–137`). The preview flag does not flow into this opener. Therefore “without deleting units” in the manifest description is accurate; “no possible filesystem repair/reset effects” would be inaccurate. If the current partition is already normally openable, this recovery callback is not selected. This audit did not observe current readiness or authorize a reset.

Preview's plan is calculated without the later descriptor census and mount guards. Its plannedUnits/plannedLogicalBytes can exceed what the actual guarded retirement would admit. They must remain labeled planned diagnostics. The read-only process list is explicitly context only and grants no custody (`budget/👁️observation/🟦️.ts` and schema Observation.grants=[]).

## Fixed Limits and Protection

The authored policy retains 4 GiB, targets 4 GiB free, caps one retirement at 4 GiB and narrows runtime to 600000 ms. Planning skips lockHeld units and units that exceed the remaining deletion budget (`budget/⚙️execution:32–35`). Current execution verifies exact buildDirectory and the one exact targetDirectory against the compiler reservation (`46`).

Actual deletion receives only scanned build/incremental units. `deleteUnit` refuses absolute/empty/dot/dotdot/backslash/NUL paths, checks physical nonsymlink ancestors and same-device descent, and removes symlinks as entries rather than traversing them (`⚡️caching/🧹️pruning/🟦️.ts:314–337`). It does not expose arbitrary caller paths through the declared owner.

## Limits

This is source review, not a current preview result. Writer14137 ENOSPC and the reported full 47.8 GiB image are owning-agent observations, not remeasured facts. Naming fixes, tests and current preview belong to the owning lanes. This report does not release retirement, imply reclaimed bytes, or authorize manual cache modification.
