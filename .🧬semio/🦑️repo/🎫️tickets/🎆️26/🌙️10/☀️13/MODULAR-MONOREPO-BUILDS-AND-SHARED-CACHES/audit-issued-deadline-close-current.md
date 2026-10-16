# Issued Deadline And Interactive Close Independent Audit

Read-only Source audit on 2026-10-16. Only this ticket Markdown was written. No runtime, test, compiler, Native provider, cache, network, process-launch qualification, Git mutation or worktree operation ran. Existing retained logs were read and hashed. This audit does not requalify the separately coedited physical cleanup implementation.

## Findings And Disposition

No new actionable defect established in the bounded ordinary Scope/settle/controlFor/scheduleControlDeadline and owned interactive close paths. The prior blocked-stdin close finding in `audit-interactive-graph-current.md` is corrected in the inspected Source. No Source change is requested by this audit.

## Ordinary Deadline Ownership

The private ControlScope remains the only identity recorded by the control WeakMap. The frozen public control cannot replace its signal or deadline owner. Finite requests create a private controller combined with the actual incoming and optional additional signal; no public authority factory, new Original or separate funding appears. Requested zero without additional cancellation returns the exact incoming signal; with additional cancellation it combines actual signals while retaining ancestor restrictions.

`remaining` authenticates the live invocation and every actual parentControl, checks each actual signal, and computes the minimum original remaining time, inherited absolute deadline and monotonic requested duration. `scheduleControlDeadline` rechecks this same function after each wakeup. Ceil and the maximum timer delay can defer a wakeup but cannot renew the deadline. The actual error thrown by that function becomes the private controller abort reason. Actual additional/parent cancellation propagates through AbortSignal.any without reconstructing its reason.

The timer registers its idempotent stop function in its own and all ordinary scope ancestors. Abort removes the listener, clears the pending timeout and removes every retirement registration. Scoped settlement drains all accepted work before marking the scope dead and invoking retirement stops, on success and failure. A synchronous control beneath an invocation alias or an intervening scoped control therefore loses its timer when that actual ancestor retires. Descendant control admission still traverses private ancestry and cannot revive a retired parent.

`track` registers accepted promises on every ancestor before their deferred callback starts. Rejection closes admission and records a first failure using array presence, preserving thrown undefined. `settle` retains body failure precedence and performs post-drain validation before retirement. The timer remains armed while accepted descendants drain, so successful body return alone does not cancel deadline delivery. There is no asynchronous gap between creation/registration and the next admission step in the inspected issuance paths.

## Interactive Close And Failure Drain

The corrected private close first rejects new operations and resolves queued readers, then ends stdin and initiates termination of the actual child before awaiting accepted operations and terminal close. This removes the prior wait-for-write-before-stop cycle. Repeated close shares one retained promise. A blocked write retains its real stdin write callback/error/close completion; buffer wiping occurs only after awaitOriginal has also joined that actual publishing promise. A close-induced EPIPE/input-closed error remains fatal in the real accepted operation and Original rather than being converted to successful publication.

Actual abort, output refusal, callback failure and spawn error all route through the same stop function and preserve the first captured reason, including undefined through a separate failed boolean. Capture joins the terminal, accepted channel operations, pending input publication and progress publication before returning or throwing. Token retirement occurs only after channel close joins. Close and cached diagnostics consume the already accepted live owner rather than admitting fresh ordinary work after cancellation.

The blocked-close test uses an ordinary Node peer that pauses stdin, emits PID/projected envelope DATA and stays live; the surrounding Bun child receives the Original. It accepts an actual 4 MiB write, yields once, closes, then requires the actual input failure and child disappearance. It does not represent that Node peer as a genuine Original receiver. This specifically exercises the corrected control-flow dependency.

## Evidence And Limits

All 37 cited entries across the two successor reports (including repeated paths) matched their recorded SHA-256 at read and final report generation. The read green3 prototype/release expiry logs each report 1 pass, 0 fail and 73 assertions; the read green12 prototype/release interactive logs each report 1 pass, 0 fail and 85 assertions. These are the authors' retained execution evidence, not tests run by this auditor. Empty strict typecheck logs were hash-checked but cannot independently establish exit status. The standalone canonical proof's changed signal identity expectation remains explicitly unexecuted by this narrow successor selector; execute its existing selector when broader Root qualification is authorized. Native, platform, cache, compiler and activation claims remain outside this review.

## Exact Source Epoch

The following exact files define the inspected epoch. The shared canonical file contains peer-owned physical-cleanup work outside this review's scope. Later file changes require separate epoch accounting.

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` — SHA-256 `e824abd12449f6c63fc13819f4ea8f8bae85cf604158c535d4985897ce40a90f`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧬️schema/🔣️.json` — SHA-256 `d24a78f2d3f6a24a4df1c4d9987d2e988c0c7a3a042c9419644c2bec3b28577b`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/🔣️.json` — SHA-256 `d722885673ddbcbb54638e2e809b35c21b204aed4d8ced8a313cdfa18ac4e190`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/🟦️.ts` — SHA-256 `a1261c1fd3003aaac6d38c315fb3ce4062605af4a0581b09e5a260e15e15a0a2`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📥️canonical/🟦️.ts` — SHA-256 `f5e6563f9cca37d628100dc4d0483f37e47b8d9be3aeba3e4b912c407253e175`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` — SHA-256 `f1a695410760d17358b0d2c3b571ec35c91591f82d069d9f3b105e14851c6c18`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts` — SHA-256 `bba161d21ef9fcb95a516d442498b83281f30bb0ffa7c0e619d8e1b3fab6863f`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts` — SHA-256 `d9bc31c31d77fbf774ef8c57020ca534c8243c706d64fb0394c6b7dbfa3f8899`
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` — SHA-256 `9b6c3fce27ae99477e1785c9e3acd91e9cd198480b57199d173f56d5266a38b8`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📡️channel/🧬️schema/🔣️.json` — SHA-256 `e5145df81fa9defd05f64808bc5688ad8a632323cd540c75fd8b3b6c23122f03`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📡️channel/🧪️tests/🔣️.json` — SHA-256 `e3e3a928b140b62a1615339e100aa6ea83cca6b21338c8da011a539dd84f7cb1`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📡️channel/🧪️tests/🟦️.ts` — SHA-256 `bf689ecc055160c817944ca5ea6781e6987ef42b9d01a1ab4617a94152eadfad`
- `🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts` — SHA-256 `3df81b9a89a5c5e56bb37ecf579c0684dca29beacc50035fa6bfcfe75cb9c7ff`
