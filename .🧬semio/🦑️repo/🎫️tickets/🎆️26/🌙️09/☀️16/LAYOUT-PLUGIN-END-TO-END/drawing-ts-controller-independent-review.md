# Drawing TypeScript Controller and Behavioral RED Review

The proposed mandatory controller lifecycle is coherent enough for production implementation under the stated contracts. No design blocker remains in the document itself. This is design/source clearance only: the eight abstract controller cases are not an executed implementation, and complete lifecycle acceptance remains required. This auditor changed no Source and executed no runtime.

## Actual Behavioral Counterexample

The actual focused normal run 94026 ended exit1, with 0 passes, 1 failure and 4 assertions. Independent log verification matches 16,932 bytes and SHA-256 `19ac90471fd0c228c214d1873039c2f42ef3d35f4b2c90cd2489bf53cc130c09`. The assertion is genuinely `progress.work` expected0, received1 under an explicitly all-zero FontWorkLedger. It is reached before any proposed observation API. This is a behavioral counterexample to zero-authority handling, not an import/API-absence RED.

The current test first creates the old actual DocumentTextJob, then checks progress before consulting new methods. Its finally block cancels, verifies no output, closes the real owner and verifies the source node archive is empty. The first failed expectation plus those three successful cleanup assertions account for the four assertions; no cleanup error masks the observed work1. Its temporary numeric/funded close discrimination supports this pre-repair regression cleanup only; production must use the new funded close contract without a numeric bridge. All eight captured input hashes and byte sizes independently match the terminal receipt and current files.

## Controller Contract

`drawing-text-ts-funding-boundary.md` requires actual caller-supplied finite policy and no default controller, no demand-derived credits and no byte grant derived from item quantum/maxWork. The borrowed turn is controller/operation/original-grant bound, live only during callback, and nonreentrant. Validation precedes mutable custody. The same remaining-grant ledger spans descendants; retained effects and CPU are separate, folded exactly once, including failures, before recheck/leave. Demand is scheduling data. Blocked work returns and awaits the caller scheduler; wait itself supplies no credit. These requirements resolve the earlier missing TS authority seam without claiming an existing product host issues it.

The funded close contract must remain equally mandatory. Cancellation suppresses semantic publication; close authority comes explicitly from close mode. Refusal or expiry returns a first-party failure carrying the actual close owner, rather than dropping it or exposing pending output. Successful output stays private until close and controller recheck succeed. Provider failure, abort while waiting and callback/accounting exceptions must all converge on this retained-owner path.

Implementation review and regression checks must specifically verify that a thrown `consumeRetained` cannot skip CPU folding or owner preservation; both actual effects and original child error must survive. Also verify that checking a retained borrowed turn after callback exit fails and that changing operation identity cannot reuse a valid controller token. These are required by the document but are not covered by the eight abstract rows alone. Do not weaken them to satisfy only those rows.

## Schema Scope and Required Follow-Through

The controller schema fixes a closed lifecycle, exact rules and eight cases: unfunded, paid, child-error-after-effects, overrun-recorded-before-refusal, close-release-denied, close-paid, expired-turn and foreign-controller. The fixture repeats these declarations exactly. Current SHA-256 values are schema `19aad4a1c61877d304a0a4f68c2283bd8042918b5758504462504e8cebfb8e29` and cases `86d2bea883bdd3fda12590ffffc3660c84f7bc9ddcc9c9c3f35f71380d46d231`. They were outside the immutable RED eight inputs, so this review does not attribute their semantics to run94026.

Actual lifecycle laws must additionally exercise same-operation/original-grant refusal, callback reentry/expired borrow, blocked wait wake/cancel, font-provider rejection and failed/refused close owner recovery. Error-after-effects must be a positive final failing-turn witness, not totals accumulated on earlier successful turns. The full Rust/TS owning verification, existing geometry parity and Layout19 remain required. The actual zero-authority RED proves only its concrete old-code bug; it does not prove the newly proposed controller or funded close implementation.
