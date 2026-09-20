# React Reference Watcher Race

The fresh React reference server at port 6313 exited when a concurrent atomic write removed a temporary source file between `existsSync` and `statSync` in `semioSourceWatchVitePlugin`. The captured server log identifies the source-watch callback and an `ENOENT` for the transient file. This prevented live React/WGPU comparison despite a completed React activation.

The watch-policy fixture now includes a vanished rename event with a stale positive existence observation. The regression injects only the watcher transport and previous existence result; Node filesystem metadata still determines that the file is gone. Before the fix, the focused Nx test failed with the same `ENOENT`. The production callback now uses one nonthrowing-for-missing stat result, emitting `unlink` and invalidating moved sibling modules when absent. Directory and existing-file branches retain their events. Other filesystem errors remain errors.

Validation: the deterministic red run completed with one expected failure. The combined watcher-policy/config-graph run passed all 24 watch-policy cases and two graph cases, but failed two separate graph contracts (58 reachable modules exceed 40; Bun sourcemap tree shaking differs from esbuild by five modules). No production dependency was added by this fix; those graph contracts are left unchanged. A focused watch-policy rerun passed. The owned React listener was restarted with HMR disabled at port 6313 and Vite reports ready; its browser is loading the plugin. Generated logs are in `🗑️generated/astra-runtime/watcher-race-{red,green}.log`.

Changed files: dev Vite plugins TypeScript, config regression tests, and the language-neutral watch-policy fixture. No new commands or runtime dependencies.
