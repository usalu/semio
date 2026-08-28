# Runtime Regression R48 Routing Failure

Attempted canonical `test exhaustive --lib --no-fail-fast`. Actual exit1 before native test execution: `error: unexpected argument '--no-fail-fast' found`.

Read-only inspection shows the shared runner partitions execution flags, then invokes `cargo nextest list` with build arguments. This flag was not partitioned out, so it reached the list phase. No helper mutation or production change was made. The next canonical exhaustive run omits the unsupported routing flag and will report actual fail-fast counts honestly.

Raw: `🧪️member-runtime-canonical-regression-r48-native-2026-08-27.txt`. Source oracle completed40 checks before the routing failure. No native test PASS credit.
