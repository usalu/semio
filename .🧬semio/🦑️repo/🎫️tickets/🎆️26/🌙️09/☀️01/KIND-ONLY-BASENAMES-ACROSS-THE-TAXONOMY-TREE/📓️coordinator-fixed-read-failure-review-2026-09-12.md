# Fixed-Source Read-Failure Regression Review

Date: 2026-09-12

The coordinator reviewed the durable follow-up to 📓️terra-fixed-script-acceptance-audit-2026-09-12.md. The new TaxonomyCapturedSourceRead interface is a first-party function from an absolute source path to Uint8Array. inventoryTaxonomyWithCapturedSourceRead calls the existing inventory implementation, keeping its ordinary source admission, physical path checks, symlink handling and parent pruning. Only the admitted regular leaf byte read is injected; the normal entry uses readFileSync.

The permanent portable case creates a real fixed script inside a unique ticket-generated control and throws from the reader only for that exact leaf. Its assertions require the root-script fixed contract, null file kind, not-package role and both path-read-failed and fixed-source-content-unreadable findings. It removes only the allocated control directory in finally. This closes the audit's durable integration coverage gap without relying on Unix mode permissions or requiring a live source to remain unresolved.

Root ran the exact new test independently: 1 passed, 120 filtered out, 0 failed, 4 assertions, 3.59 seconds. The first command omitted Bun's explicit ./ path prefix and matched no test file; it is not a test failure or acceptance run. The corrected explicit-path command produced the passing result. Raw logs are coordinator/fixed-read-failure-review.log and fixed-read-failure-review-explicit-path.log.

Sol's updated retained report separately records the full direct and registered suite at 121 passed/530 assertions and strict taxonomy at zero diagnostics. Root did not repeat those full suites after the focused acceptance. No product source changed in this review. The two source/test files, portable schema/fixture and Sol report retain their exact attribution in 📓️sol-fixed-script-semantic-enforcement-2026-09-12.md.

