# Intrinsic Media output work and allocation settlement

The mounted Trinity MediaOut test reached the Pack intrinsic encoder and failed on the first cursor advance. Branch probes against a local copy of the native test executable identified the remaining marker as a canceled allocation outcome from that advance. Consuming the completed canceled outcome exposed the cursor's zero-progress refusal.

The Pack cursor's structural advance spends one original work unit before a funded page reservation. The paged list separately quotes the work needed to select and reserve the page. The native effect wrapper accepted a maximum-work argument but opened every settlement with one unit, so the first frame reservation returned no progress and canceled its unused allocation quote. Cursor close also needs an additional unit when it releases an emptied page.

Intrinsic Media now uses the cursor and paged list work demands for advance, and includes the additional release unit on close. The native closure effect opens and validates the original work settlement against the supplied maximum, posts the actual performed receipt, and leaves genuine refusal and unsettled state with the original owner. Media resets its completed marker after consuming a settled accepted or canceled allocation outcome.

The native test passed in a diagnostic copy of the prior executable with the two cursor work ceilings raised. The probe observed no further reset refusal and the mounted output passed the test assertions. The source fix uses the paged list’s published Work demand, while the diagnostic copy used a fixed ceiling to avoid rebuilding the test executable.

The final native build passed the mounted Trinity output test using a populated rule. It decoded the actual Pack response and matched the authored rule and parameter bindings.

## Native encoding laws

The four integration test targets now use the current ownership boundaries. Recorded source setup supplies an authenticated witness, typed close lends a genuine producer invocation with the bounded retirement body, and Work settlement receives the quoted maximum. The action fixture verifies an owned Work refusal after the physical recipient accepts its receipt. The recorded-source payout fixture verifies retained physical refusals and now asserts the callback's innermost result.

The serial native Cargo fallback ran all four targets. Action work passed 9 tests, effect settlement passed 5, source effects passed 8, and typed retirement passed 5. The existing ignored scoped-pause case still requires its recorded allocation fixture to drive source preparation. The typed retirement suite passed the multi-unit Work quote and tested actual debit, callback unwind recovery, and physical progress.
