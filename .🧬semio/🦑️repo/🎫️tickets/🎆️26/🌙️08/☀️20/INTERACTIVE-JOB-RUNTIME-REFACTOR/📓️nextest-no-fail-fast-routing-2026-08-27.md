# Nextest No-Fail-Fast Routing

The existing argument partition incorrectly forwarded `--no-fail-fast` to warm build/list. Two schema-backed vectors now require the flag to reach execution only and preserve separator ownership. The existing independent Node parseArgs oracle validates semantic options. Production correction is one additional recognized execution flag; no profile, timeout, grant, source-filter or coverage change.

Canonical repo-lib test RED: 0 passed, 1 failed, 318 filtered, 2 assertions, 1.299 seconds. GREEN `nextest execution filters`: 2 passed, 317 filtered, 78 assertions, 1.088 seconds. The second test is the pre-existing artifact-location regression. Full runtime diagnostic can now use the existing runner without excluding failures.

## RED Actual Output

```text

> nx run @semio-tech/repo-lib:test --args=-t "preserves language-neutral build and execution vectors"

> bun ./📜️script.ts test -t "preserves language-neutral build and execution vectors"

bun test v1.3.14 (0d9b296a)

🧪️index.test.ts:
Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

1949 |       const options = {
1950 |         lib: { type: "boolean" }, release: { type: "boolean" }, "no-default-features": { type: "boolean" }, package: { type: "string", short: "p", multiple: true }, features: { type: "string", short: "F", multiple: true }, target: { type: "string" }, "target-dir": { type: "string" }, "build-jobs": { type: "string" }, "cargo-profile": { type: "string" }, "cargo-message-format": { type: "string", multiple: true }, timings: { type: row.input.some((arg) => arg.startsWith("--timings=")) ? "string" : "boolean" }, config: { type: "string", multiple: true }, Z: { type: "string", short: "Z", multiple: true }, test: { type: "string", multiple: true }, bin: { type: "string", multiple: true }, bench: { type: "string", multiple: true }, example: { type: "string", multiple: true },
1951 |         "filter-expr": { type: "string", short: "E", multiple: true }, partition: { type: "string" }, "run-ignored": { type: "string" }, "ignore-default-filter": { type: "boolean" }, "no-fail-fast": { type: "boolean" },
1952 |       } as const;
1953 |       const actual = partitionNextestExecutionFilters(row.input);
1954 |       expect(actual, row.name).toEqual({ buildArgs: row.build, executionArgs: row.execution, libtestArgs: row.libtest });
                                      ^
error: no-fail-fast-is-execution-only

  {
    "buildArgs": [
      "--lib",
+     "--no-fail-fast",
      "--features",
      "no-fail-fast",
    ],
-   "executionArgs": [
-     "--no-fail-fast",
-   ],
+   "executionArgs": [],
    "libtestArgs": [
      "--nocapture",
    ],
  }

- Expected  - 3
+ Received  + 2

      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:1954:32)
✗ nextest execution filters > preserves language-neutral build and execution vectors with an independent Node parser [70.06ms]

 0 pass
 318 filtered out
 1 fail
 2 expect() calls
Ran 1 test across 1 file. [1299.00ms]
Warning: command "bun ./📜️script.ts test -t "preserves language-neutral build and execution vectors"" exited with non-zero status code


 NX   Running target test for project @semio-tech/repo-lib failed

Failed tasks:

- @semio-tech/repo-lib:test

Hint: run the command with --verbose for more details.


```

## GREEN Actual Output

```text

> nx run @semio-tech/repo-lib:test --args=-t "nextest execution filters"

> bun ./📜️script.ts test -t "nextest execution filters"

bun test v1.3.14 (0d9b296a)

🧪️index.test.ts:
Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

✓ nextest execution filters > retains explicit task artifacts and preserves the default temporary location [62.92ms]
✓ nextest execution filters > preserves language-neutral build and execution vectors with an independent Node parser [25.01ms]

 2 pass
 317 filtered out
 0 fail
 78 expect() calls
Ran 2 tests across 1 file. [1088.00ms]



 NX   Successfully ran target test for project @semio-tech/repo-lib



```
