# Coordinator Review of Layout Enforcement

Issues sent to the policy worker for correction and regression coverage:

- C/Rust and Python lexical masks used code-point arrays with UTF-16 source offsets. Emoji in comments or strings could shift masks and source line references.
- Rust lifetime apostrophes must not be interpreted as the start of character literals and hide later test declarations.
- Logical canonical import resolution must use platform-independent POSIX paths, rather than comparing Windows native paths to a fixed `/repo/` prefix.
- The dominant Rust `semio_framework_async_macros::async_test` attribute must be detected.
- Valid case files may contain invalid nested module wiring: canonical file ownership suppresses inline-body findings, not path validation.
- Rust nested inline-module `#[path]` rules differ from simple source-directory resolution and must use the extraction worker's proven rules.
- Scan memory and cancellation must remain bounded through parsing, not only during filesystem reads. Handle concurrent file disappearance without hiding genuine I/O failures.

These are review findings during active implementation; the final worker report must record their disposition and executed tests.

## Nx Input Review

`library/🟨️.mjs` still contains legacy suffix exclusions in generated project inputs. Target-first delivery trees derive their source owner only from `📦️packages`, which can omit newly semantic-scoped test cases above `🎯️targets`. Shared library exclusions may also suppress the library's own canonical tests from its test target cache key. The policy worker was asked to remove legacy suffix handling and verify canonical cases participate in owning test inputs while production inputs exclude tests.

Additional parser classification cases: config-only imports such as Playwright `defineConfig` are not executable tests; Storybook `play` callbacks with assertions are. Rust cfg predicates must distinguish test-only conditions from `cfg(not(test))` and mixed production/test conditions.

## Root Follow-up: Case Identity and Lexical Bindings

The feature contract validator previously applied the bare slug regex to the full emoji-prefixed case name. The new regression failed before the shared canonical case helper; after the fix, feature contracts and implementation paths accept the same one-emoji plus kebab-case names and reject delivery-owned cases (`📦️packages`, `🎯️targets`, `⚡️implementations`).

The JavaScript scanner previously treated any call named `test`, including local utilities, parameters, loops, catches, and shadowed imports, as a registration. New language-neutral cases reproduced these false positives and also found missed modifier/require-alias calls. The scanner now resolves value bindings through lexical scopes before classifying harness calls. Named, aliased, namespace, require, global, and modified registrations remain visible; object-property Storybook play assertions are also recognized. No authored source paths were excluded to obtain this result.

Runtime proof: public `bun nx exec --projects=workspace -- bun test <canonical-layout-case>` ran **13 tests, 45 assertions, 0 failures** after the changes. Ajv validated the language-neutral fixture schema; minimatch independently checked legacy suffixes; the TypeScript checker independently checked local and imported binding resolution. Red and green console logs remain temporarily in the root-layout generated lane until final cleanup.

## Build-output Discovery

The fresh repository scan encountered two compiled Vite bundles in `📤️dist`. The repository discovery module already treats both `dist` and `📤️dist` as build output, while the test scanner previously recognized only the plain spelling. Added the existing canonical output spelling to the scanner skip set and aligned in-memory file-set inspection with filesystem discovery. A language-neutral regression keeps the adjacent authored source visible while ignoring both output spellings. It failed before the fix and passed afterward.

The complete focused suite now passes **14 tests, 46 assertions, 0 failures** via the public `bun nx exec --projects=workspace -- bun test` path (5.67 seconds in the test process). Authored scratch/source directories remain in scope.
