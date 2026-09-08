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

## Nx Case Discovery Integration

The actual Nx plugin rejected canonical emoji-prefixed cases by applying the raw kebab-case regex to the complete directory name. A new regression reuses the language-neutral layout vectors against the real plugin hook, covering case names, nested test roots and delivery owners. RED: one case rejected (0 pass, 1 fail). The plugin now validates the leading presented emoji separately and applies the taxonomy slug pattern to its name; convenience discovery applies the same rule. GREEN through public Bun/Nx in the private minimal workspace: 15 tests, 52 assertions, 0 failures (12.26-second test process). The full repository Nx graph is still a separate unresolved setup check.

The earlier full-repository Nx invocation eventually completed graph construction and executed the first framework runtime round (four passes, eight pre-fix fixture failures). Its process returned status 1 from those assertions, rather than a graph failure. A fresh full-graph case-project inventory is now checking the subsequent Nx plugin change. Focused follow-up tests remain separately attributed to the private fixture.

## Full Repository Nx Discovery

The real public `bun nx show projects --with-target test-contract --json` completed with exit 0 after the case-name plugin correction. It returned 250 projects, including 248 generated canonical test-case projects. This used the complete repository graph, with plugin isolation explicitly enabled and worker timeouts disabled for this invocation. The focused plugin/vector regression is separate evidence.

## Rust Inline Module Path Bases

Added four language-neutral vectors for explicit dot, named, default, and invalid module bases. Before the correction, the focused public Bun/Nx fixture run reported 1 pass and 3 failures. The scanner now uses each enclosing inline module’s explicit `#[path]` directory when present. It no longer assumes every Rust namespace name is a filesystem directory.

The full layout suite then passed 20 tests with 66 assertions in 6.53 seconds. Its independent `rustc --test` oracle compiled and ran the three valid path trees (one test per binary) and rejected the invalid tree. Compiler files and logs were confined to this ticket’s generated directory. The full repository scan is a separate pending check.

A fifth compiler-backed vector put fake path attributes inside comments. It failed before the lexical fix (0 pass, 1 fail). Path extraction now identifies attributes in masked Rust syntax before reading their literal values. The combined full layout and source-evidence suites passed 23 tests with 79 assertions in 14.70 seconds through public Bun/Nx using the private minimal graph. This includes five path-tree compiler checks and the production/test evidence-isolation compiler check.

## Policy File Attribution

Read-only comparison with the pre-goal base confirmed the policy worker’s seven taxonomy fields and their validation in the discovery library, the matching taxonomy vocabulary, the test-domain JSON protocol, the async contract/run scanner integration, the canonical package runner paths, and the Ajv/minimatch development-oracle declarations. Their exact files, including the matching workspace lock entry, are included in the coordinator manifest. Other concurrent changes in those same files are not attributed to this work. The existing VS Code contract launch command remains available; no additional public command was introduced for this gate.

## Test-Gated Domain Modules

The fresh repository scan inspected 29,246 sources and reported 59 findings: 58 Rust wiring entries and one remaining TypeScript self-test declaration. Thirty-four Rust entries were ordinary WFC algorithm modules enabled under `cfg(test)`; their own executable tests already live in canonical cases. The scanner must distinguish module gating from executable test ownership. A language-neutral domain-support vector and a counterexample with an actual misplaced test body were added; the pre-fix run failed both (0 pass, 2 fail). The correction allows an existing ordinary support target while still checking named test modules, canonical/legacy test path markers, and every discovered file’s executable bodies.

Post-fix evidence: the complete layout and source-evidence suites passed 25 tests and 84 assertions in 8.36 seconds through public Bun/Nx. The test-gated support tree compiled and executed with rustc; the misplaced-body counterexample still reports its body violation.

## Complete Repository Layout Scan

The final public Bun/Nx scan traversed the real repository and inspected 29,224 authored source files. It completed with zero findings. This uses the full production scanner, not a filename-only survey or the private Nx fixture’s own sources. The private graph only dispatches the scan; `SEMIO_LAYOUT_REPO_ROOT` explicitly points to the real repository. Later package-caller and expected-literal repairs change references and assertions inside existing canonical cases without changing test locations.
