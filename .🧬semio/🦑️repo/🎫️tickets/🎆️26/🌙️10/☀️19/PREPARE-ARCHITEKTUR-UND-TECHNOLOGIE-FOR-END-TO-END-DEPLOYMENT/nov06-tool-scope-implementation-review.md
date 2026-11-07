# Native Tool Scope Implementation Review

## Actionable cancellation defect

`🏗️build/🧰️tool-registry/⌨️command/🟦️.ts:43–51` catches SIGINT and SIGTERM in a local AbortController, but passes the unmodified issued control to `withRepositoryNativeToolchainV1`. The controller is combined with the actual control only inside the factory body. The parent Original and issued control can still cancel acquisition through their own signal. The specific gap is that this command’s local OS-signal controller is not itself forwarded to the factory: if the upstream process does not independently propagate that OS signal into the issued control, acquisition continues and the local signal reaches only the subprocess if acquisition eventually finishes. Upstream OS-signal propagation was not verified in this finite review. Link cancellation to an issued derived control for the entire factory lifetime, preserving authenticated Original ownership.

At lines 46 and 53–56 the abort listener registered on `context.signal` is not removed after execution, and an already-aborted signal is not forwarded by adding a listener. Remove the listener in the finally block and account for prior cancellation.

## Traced route

- `🏗️build/📜️script.ts:119–134` allows native classified commands through the capacity door and creates or receives the Script Original after admission.
- `🏗️build/⌨️command/📥️entry/🟦️.ts:12` receives the authenticated Original and constructs issued control; `⌨️command/⚙️runtime/🟦️.ts:19–25` validates the pair and dispatches native commands before the diagnostic and dependency-store branches at lines 28 and 50–66.
- `🧰️tool-registry/⌨️command/🟦️.ts:16–17,39–51` resolves registry owner and normalizes commands, identifies cc and cargo-owned rustc, forwards the argument tail to `runToolV1`, and passes `admission:false` to reuse the outer grant. The factory holds custody for the complete runner promise and performs pre/post rechecks.
- `🏗️build/⌨️command/🏃️invocation/🟦️.ts:9` uses the existing control when re-entering capacity; source capture and the captured runtime entry still precede the branch. This inspection did not execute a cold dependency-free command and cannot establish whether source closure capture requires any installed package in that situation.

Read-only source review; no expensive tests or source edits performed.
