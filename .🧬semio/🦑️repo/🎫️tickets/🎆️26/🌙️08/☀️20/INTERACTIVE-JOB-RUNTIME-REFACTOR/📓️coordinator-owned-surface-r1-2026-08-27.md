# Owned Surface Independent Runtime Review

## Actual Targeted Run

The coordinator independently ran the canonical Bun/Nx target on 2026-08-27 at16:14:24. Exit0:5passed,537skipped,542discovered across5files; duration6.12s. These are concrete OwnedUiSurface/React subscription tests, not live UiNodeView or browser/Wasm proof. The captured output did not display the prefix-count DEBUG line despite the passed --silent=false option; no count is inferred from it.

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedSurface --silent=false'
```

## Source Review

The coordinator read the surface owner, subscription hook, publication, notification and close paths. The patch now retains a staged-cell frontier. Before an uncommitted patch can become terminal, it drains the cancelled read captures owned by those exact cells; detached cells are handed to the surface's bounded maintenance queue and drained. A committed late-close continues notification delivery and blocks on its exact publication acknowledgement. Notification exceptions retain the exact subscription and explicit retry authority instead of silently losing it.

The five tests cover hash/publication/invalid-root behavior and active-reader close rejection; actual React byte-view lifetime across replacement and unmount; cancellation at every enumerated program prefix; reentrant subscriptions and callback failure/retry; and invalid quota rejection with preserved operation ownership. The prefix fixture uses3-byte Surface data, not a maximum-envelope performance certificate.

Still open: incremental scene decoding/prepared projection, live UiNodeView adoption, per-instance aggregate lifetime, every scene host's asynchronous byte owner, all-app browser and hard timing. No source-only or test-harness result is being treated as those gates.

## Full Captured Output

```text

> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t OwnedSurface --silent=false

> bun ./📜️script.ts test long --run -t OwnedSurface --silent=false

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 Test Files  1 passed | 4 skipped (5)
      Tests  5 passed | 537 skipped (542)
   Start at  16:14:24
   Duration  6.12s (transform 8.39s, setup 0ms, import 12.80s, tests 2.44s, environment 2.71s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react



```

