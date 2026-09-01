# Action-Bus Short-Close Conservation

## Scope

The full common framework R1 gate actually ran 265 tests: 263 passed and two failed. Its registered retained-number fixture incorrectly expected an eight-byte input to release zero bytes under a seven-byte close grant. Production `RetainedToolWireInput::close_step` already releases the exact seven initialized bytes, retains the final byte, and releases empty backing separately.

No production close implementation, grant, capacity or timing threshold changed. The strict neutral fixture now pins zero-item refusal, zero-byte refusal, seven-plus-one logical release, unchanged physical backing through partial close, and a separate one-item/zero-logical-byte backing release. The existing registered fixture and a new direct physical-backing law consume the same fixture. Node Buffer independently derives the bytes and short-close frontiers; Ajv rejects inconsistent backing accounting.

## Source Gate

Actual exit 0: five ownership cases, five hostile fixtures, four short-close frontiers. Native behavior remains pending.

```sh
set -o pipefail
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-rs:test-wire-retirement-source --skip-nx-cache 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-actionbus-short-close-source-r2-2026-08-28.md'
```

## Complete Captured Tool Output

```text

> nx run @semio-tech/framework-rs:test-wire-retirement-source

> bun ./📜️script.ts test-wire-retirement-source

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] raw wire retirement source: 5 ownership cases, 5 hostile fixtures, 4 short-close frontiers; native grant/terminal behavior is separate



 NX   Successfully ran target test-wire-retirement-source for project @semio-tech/framework-rs



```

