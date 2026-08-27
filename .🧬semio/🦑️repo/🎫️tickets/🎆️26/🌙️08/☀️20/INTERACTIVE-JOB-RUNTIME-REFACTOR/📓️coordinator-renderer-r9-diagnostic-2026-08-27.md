# Renderer R9 Bounded Diagnostic

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--reporter=verbose --testTimeout=3000 --hookTimeout=3000 --teardownTimeout=3000'`

Exit code: 1. **No tests executed**: Vitest rejected the requested stricter timeout because the task router already supplies that option. The actual diagnostic error is a duplicate single-valued option, not a test failure. Overall process and production watchdogs were unchanged. R10 instead uses the unmodified quick profile with a verbose reporter.

```text

> nx run @semio-tech/framework-renderer-react:test-long --args=--reporter=verbose --testTimeout=3000 --hookTimeout=3000 --teardownTimeout=3000

> bun ./📜️script.ts test long --reporter=verbose --testTimeout=3000 --hookTimeout=3000 --teardownTimeout=3000

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

2188 | 	let command = option.shorthand ? `-${option.shorthand}, --${commandName}` : `--${commandName}`;
2189 | 	if ("argument" in option) command += ` ${option.argument}`;
2190 | 	function transform(value) {
2191 | 		if (!option.array && Array.isArray(value)) {
2192 | 			const received = value.map((s) => typeof s === "string" ? `"${s}"` : s).join(", ");
2193 | 			throw new Error(`Expected a single value for option "${command}", received [${received}]`);
                    ^
error: Expected a single value for option "--testTimeout <timeout>", received [300000, 3000]
      at transform (/Users/ueli/Documents/semio/node_modules/vitest/dist/chunks/cac.DdICfEr1.js:2193:14)
      at setDotProp (/Users/ueli/Documents/semio/node_modules/vitest/dist/chunks/cac.DdICfEr1.js:209:22)
      at mri (/Users/ueli/Documents/semio/node_modules/vitest/dist/chunks/cac.DdICfEr1.js:593:9)
      at parse (/Users/ueli/Documents/semio/node_modules/vitest/dist/chunks/cac.DdICfEr1.js:510:27)
      at /Users/ueli/Documents/semio/node_modules/vitest/dist/cli.js:11:13

Bun v1.3.14 (macOS arm64)
Warning: command "bun ./📜️script.ts test long --reporter=verbose --testTimeout=3000 --hookTimeout=3000 --teardownTimeout=3000" exited with non-zero status code


 NX   Running target test-long for project @semio-tech/framework-renderer-react failed

Failed tasks:

- @semio-tech/framework-renderer-react:test-long

Hint: run the command with --verbose for more details.


```
