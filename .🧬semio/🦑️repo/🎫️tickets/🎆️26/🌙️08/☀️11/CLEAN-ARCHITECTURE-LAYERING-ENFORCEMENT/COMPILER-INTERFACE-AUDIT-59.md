# Compiler Interface Audit 59

Read-only current-source inspection on 2026-10-13. Reports 52, 55, 56 and 58 were read as context, then the implementation and concrete callers were independently inspected. No source, GUI, fixtures, Git state or lifecycle was changed. Repo MCP is unavailable; no lifecycle receipt is fabricated. This is a source snapshot under concurrent edits, not runtime acceptance. No tests or native builds were run.

## Actionable Findings

### P1: Native stages cannot observe original numeric expiry and driver drains them indefinitely

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`:81, 265–268, 318–323, 359, 485, 497.

The deadline timer records a stopped error and terminates only children in its own set. Stage context exposes the original signal but no remaining deadline or stop check. The timer intentionally does not abort that original signal. An already-running stage therefore remains unaware of numeric expiry. The driver awaits its pending work before checking expiry, then drains it again in finally. Finalizers skip the pre-run check entirely. Actual Specific driver stages forward only context.signal into descriptor/transpile/optimize/publish operations (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`:32–56), so this is a production receiving gap, not merely an unused callback type. Pass original remaining/publish/continuation ports through stage work and implement finite interruptible admission for stage/finalizer waits without replacing the original signal. Prove an un-aborted incoming signal with a finite deadline while a stage is pending.

### P1: Native provenance independently renews deadline instead of consuming caller remainder

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`:839–841 and `🧩️component-pass/🟦️.ts`:283.

Both actual production physical provenance controls omit remainingMs. `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts`:22–31 then grants a newly started default physical deadline (scaled in the driver), with original cancellation as its only incoming authority. In capture this occurs in finally after compilation/staging, when a finite caller deadline may already be exhausted. Supply the original intersected admission remainder and check it across awaited provenance work. Existing genuine signal identity alone does not satisfy numeric expiry.

The relay is a related narrower gap: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🌊️relay/🟦️.ts`:28 uses operation.invocation.control.remainingMilliseconds rather than its already-created requested compiler control. Its provenance can therefore outlive policy.budgetMs when the parent has a longer grant. Consume control.remainingMilliseconds for this same relay phase.

### P2: Native generation materializes paths before path admission

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`:109–115, 262.

Generation creation accepts an unrestricted cacheRoot and shared stores, performs mkdir and mkdtemp, then creates report/log/jobserver directories. This function contains no resolved 256-unit materialized-path guard and runs before compiler-capacity admission. It can create forbidden paths on hosts permitting them, or leave partial materialization before platform rejection. Validate resolved roots and every fixed/generated suffix budget before any generation write; validate shared target/build and stage-cache paths too. This finding is outside the correctly guarded compiler-state helper and should not be generalized to that helper.

## Descriptor Cleanup Review

The extracted reader validates independent byte/code-unit limits, rejects the first overflowing chunk, flushes the decoder, awaits individual progress publication, removes original-signal listeners and releases the reader lock in finally. No concrete regression in those ordinary terminal paths was found by source inspection. Its catch at `🧰️framework/🔨️modules/🎠️kernel/📇️descriptor/📥️text/🟦️.ts`:68 still awaits reader.cancel(error) unconditionally: a legal underlying stream cancel promise which never settles can postpone releaseLock and rejection after original cancellation or overflow. This is an untested bounded-settlement frontier, not a runtime-reproduced regression. Add an actual Response/ReadableStream witness with pending underlying cancel before asserting total cleanup acceptance.

## Moved Repo Go Integration Review

The actual moved law at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️executable-owners/🟦️.ts`:13–26 still compiles all three fixture owners in every fixture mode, passes one original received control to every canonical build, preserves 120000 ms requests and CLI/MCP/coordinator assertions, and gives the actual services the original signal. No move-specific production regression was identified. The synchronous Go metadata call has no timeout and health fetch has no original signal; these are fixture coverage/settlement limitations, not evidence of a regression introduced by relocation. The report's historical executable receipt was not rerun.

## Qualified Positive Checks And Limits

The Script compiler projection has exactly three arguments and preserves invocation.control.signal. Its requested budget is captured once and intersected with original remaining time; finite fractions are refused before conversion to child zero. Compiler-state resolves its directory and each declared leaf before mkdir or leases, validates lease paths, checks original authority around awaited operation, and releases capacity/state leases. Capacity publication and continuation are awaited and checked afterward. These inspected source properties do not establish complete caller coverage or runtime success.

The scope did not exhaust every resolved/materialized native/Go/WASM path, arbitrary stage implementation, launch row, full TypeScript closure or host platform. No broad green, absence-of-alternate-authority assertion, or test-pass claim follows from literal scans or prior receipts. Root owns lifecycle and any receiving changes.
