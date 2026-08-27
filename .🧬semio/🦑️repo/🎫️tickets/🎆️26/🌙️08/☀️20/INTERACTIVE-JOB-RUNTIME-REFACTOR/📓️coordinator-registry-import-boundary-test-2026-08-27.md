# Registry Import Boundary Verification

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/repo-lib:test --args='-t "registryCompilerImports"' --skip-nx-cache`

Exit code: 0. Direct coordinator execution of the existing canonical library target. No dependency was installed. This gate verifies the owned runtime boundary and parser oracle, not browser or native application behavior.

```text

> nx run @semio-tech/repo-lib:test --args=-t "registryCompilerImports"

> bun ./📜️script.ts test -t "registryCompilerImports"

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

✓ registryCompilerImports > validates the language-neutral vectors against Bun and TypeScript [141.71ms]
✓ registryCompilerImports > rejects malformed runtime compiler capabilities without trusting declarations [0.43ms]

 2 pass
 291 filtered out
 0 fail
 21 expect() calls
Ran 2 tests across 1 file. [466.00ms]



 NX   Successfully ran target test for project @semio-tech/repo-lib



```

