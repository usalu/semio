# Vite Optimizer Rejection Propagation

The actual admitted transform/warmup closure now delegates operation settlement, optimizer snapshot drainage and real compiler lease release to `library/⚡️caching/🌐️vite/🏗️compiler/💧️drain/🟦️.ts`. The earlier implementation awaited `Promise.allSettled` but discarded every rejected optimizer result.

The focused helper joins every promise in the existing snapshot before capacity release. It preserves an earlier operation rejection exactly (including thrown undefined); otherwise the first rejected optimizer entry becomes the task failure. A snapshot/release exception fails an otherwise successful operation. Real task failure still enters the existing pending-set observer, closes admission and interrupts the service. The maximum32 accepted-transform policy, issued phase, actual capacity acquisition, private store/input scopes and server shutdown remain unchanged. No new runtime parallelism/cap or execution authority was added. The helper covers the finite snapshot already selected by the existing caller; it does not claim continuously growing optimizer state is globally quiescent.

## Evidence

Closed language-neutral outcome corpus is independently validated by Ajv, with SQLite selecting the expected first failure in operation/snapshot order. Deferred promises prove no early capacity release, including a rejected undefined entry while another dependency remains pending. Error identity, primary operation failure, snapshot failure and standalone release failure are checked. TypeScript AST confirms the actual admitted closure invokes this helper with its carried phase and exact `lease.release()` callback. Tests use observable delegates, not fabricated controls or physical capacity grants.

- Genuine missing-owner RED: actual19114 exit1, 96 pass/1 fail/1 loader error; `🗑️generated/vite-optimizer-drain-red.log`.
- First successor34410 exit0:99 tests/1,366assertions.
- Final prototype46790 exit0:99/1,366,5.04s; `vite-optimizer-vite-environment-prototype-final.log`.
- Final release27632 exit0:99/1,366,4.87s; `vite-optimizer-vite-environment-release-final.log`.
- Expanded strict/noUnchecked ES2022 prototype92215 and release2366 both collected exit0. Logs `vite-optimizer-vite-environment-types-{prototype,release}-final.log`.

These totals include the existing declaration/environment laws; three laws are optimizer-specific. Existing editor-registered `sdk-derivative/📜️script.ts vite-environment|vite-environment-types <mode>` roots were extended; no new command or script was introduced. Only the actual admitted closure and its import were changed in generic Vite. All own handles are terminal. No Native image/listener/Vite compiler/SDK/maintenance execution occurred.
