# Independent Numeric Reference Admission R3

Canonical numeric-index tests passed independently: **12 semantic, 165 lifecycle/cancellation prefixes, 2 ordinal, 3,072 Immer+Map differential, 6 saturation and 5 invalid-ID cases**, with strict TypeScript zero diagnostics and unchanged 256/4,096-byte grants. Exit 0.

The coordinator read the complete 612-line implementation, including cumulative reservation collection, atomic recheck/commit, at-most-three-node allocation plans and cancellation queue ordering. A subsequent source finding remains assigned: capture calls its publicly shadowable assertCaptureCapacity method before separate retains. Per-instance shadowing can bypass the composite check and leave the first root retained if the second root is saturated. A private helper or equivalent exact non-shadowable admission plus a hostile private-probe law is required before closing this boundary. This is a source finding, not an independently executed hostile failure yet.

The executor's earlier canonical RED failed in the long-data-URL test loader; its preserved-module VM reproduction is supplemental behavioral evidence, not relabeled as the canonical RED. Core index tests do not establish full retained rendering or outer callback timing.

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/value-numeric-index:test --skip-nx-cache
```

```text
$ bun ./📜️script.ts nx run @semio-tech/value-numeric-index:test --skip-nx-cache

> nx run @semio-tech/value-numeric-index:test

> bun ./📜️script.ts test

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] Numeric-index laws=12 lifecycle=165 ordinals=2 stress=3072 references=6 invalidIds=5 oracle=Immer+Map grants=256,4096 strictTS=0



 NX   Successfully ran target test for project @semio-tech/value-numeric-index



```

