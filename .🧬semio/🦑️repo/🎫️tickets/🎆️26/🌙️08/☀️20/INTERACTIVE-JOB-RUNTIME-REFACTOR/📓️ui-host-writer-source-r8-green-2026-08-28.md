# Input Writer Source R8 GREEN

Actual registered @semio-tech/ui-host-rs:test-source exit0. Added 13 incremental UTF-8 vectors checked with Node TextDecoder and Buffer (including BOM, NUL, overlong encodings, surrogate, maximum scalar, invalid continuation and truncation). Existing 22 admission, 6 root arithmetic, 9 copy-frontier, 3 close-frontier and 8 watchdog vectors remain source-only models. Four byte-buffer native tests are now mounted against absent private input_writer API; original root5 compile RED remains visible. No live queue or funding is implemented.

```text
> nx run @semio-tech/ui-host-rs:test-source

> bun ./📜️script.ts test source

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] input admission oracle: 22 neutral cases, 7 schema hostiles, 3 logical-close frontiers over retained 64-byte backing; native ownership and actual Watchdog execution remain separate
[DEBUG] input root oracle: 6 arithmetic vectors, 6 schema hostiles; native CAS, concurrency, queue ownership and allocation remain separate
[DEBUG] input writer oracle: 9 byte-copy frontiers, 3 retained-backing frontiers, 13 incremental UTF-8 vectors, 6 schema hostiles; native writer, admission, unwind and allocation remain unexecuted
[DEBUG] watchdog tail oracle: 8 same-window vectors, 5 schema hostiles; actual WGPU publication and native guard execution remain separate



 NX   Successfully ran target test-source for project @semio-tech/ui-host-rs
```

