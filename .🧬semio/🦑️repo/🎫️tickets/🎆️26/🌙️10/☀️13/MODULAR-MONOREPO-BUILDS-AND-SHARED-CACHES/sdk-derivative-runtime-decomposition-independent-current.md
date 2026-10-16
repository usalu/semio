# SDK Derivative Runtime Decomposition Independent Audit

2026-10-15. Source-only bounded review after reading `sdk-derivative-runtime-decomposition-current.md`. Inspected the actual derivative capacity test, tiny derivative copy test, ticket prerequisite dispatcher, private headroom unwrap and shared test Original receiver. No runtime, copies, tests or fresh SDK work was started. Only this report was written.

## Findings

No actionable source findings in the bounded decomposition scope.

The prerequisite validates `runtime` and `drain` as distinct commands, declares the selected command and its 120000 ms policy/capabilities before `createScriptProcessEnvelope`, and dispatches each to its own test-name selection. Both selected tests retain explicit 120000 ms test deadlines. The child capture forwards the actual inherited Original and uses its remaining time; it does not create another budget after setup. The shared test helper receives one actual process invocation and does not author a replacement root. These are independent invocations when separately launched, not a renewed continuation of the failed combined operation.

Both full-copy branches unwrap the actual `publish` result with `requireToolingPublicationValueV1`. Inspection of that helper confirms private headroom refusal DATA throws. Each branch also increments `completedCopies` only after the accepted copy and its branch-specific checks; the final exact-one assertion is outside the store/receiver callback. A skipped publisher therefore cannot pass silently.

Runtime retains the counterfeit seal/index integrity check, unadmitted and copied projection rejection, foreign quote/store/control rejection, captured input and one-shot checks, competing clone single-use check, independent SQLite exclusion, complete Python allocation census and actual Node module/shared-inode/read-only oracle. Drain starts the accepted clone without awaiting it in the publisher body, then requires the accepted promise to exist and its success continuation to have already set `settled` at publication return, awaits that promise and checks an actual output file before incrementing the completion count. Escaped projection, quote and input capabilities are checked after the projection scope closes.

The tiny real fixture retains its first complete SQLite/SHA/inode/permission and bounded concurrency checks. Its second copy is nested inside the authentic outer projection; it verifies the inner material, authenticates the outer before/after inner settlement, unwraps publication refusal, requires detached completion before publication return and checks an output file. Inner projection/quote/input reuse is rejected after closure. Skipping that nested publisher cannot satisfy the settled/output checks; skipping the whole nested scope leaves undefined escaped capabilities which cannot satisfy the expected released-capability errors.

## Limits

This audit establishes source structure and refusal/completion guard placement only. It does not establish successful full runtime/drain execution, Source-to-frozen continuity, fresh SDK publication, all-platform behavior or final physical recensus. Root owns the live runtime and its outcomes. Earlier combined timeout failures remain failures; no runtime claim is inferred from focused gate results.
