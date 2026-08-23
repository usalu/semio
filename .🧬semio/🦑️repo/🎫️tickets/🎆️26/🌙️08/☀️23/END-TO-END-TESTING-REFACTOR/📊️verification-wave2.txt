🧪️ Verification record — wave 2 (Phase 6 owner migration + §13 metrics)
Every line below is a command that was actually executed and its actual output.

────────────────────────────────────────────────────────────────────────────────
1. Discovery — 6 cases across 4 owners
   $ bun ./📜️script.ts test discover
   …classnamecomposition-77034c-flatten-class-name-inputs      [typescript]
   …classnamecomposition-77034c-merge-conflicting-utilities    [typescript]
   …stylevariants-bcdf79-compile-style-variants                [typescript]
   …repo-modules-test-4e9a61-host-protocol-parity              [rust,typescript,go,python,dotnet]
   …stdio-artifacts-pdf-fc3e39-create-minimal-pdf              [rust]
   …stdio-artifacts-pdf-fc3e39-edit-existing-pdf               [rust]
   [discover] 6 test case(s)

2. Full quick-level run
   $ bun ./📜️script.ts parity quick
   [test] level=quick cases=6 executed=32 passed=32 failed=0 errored=0 parity=37/37

3. Owner migration 1 — 🏷️class-name-composition, oracle `clsx`
   $ bun ./📜️script.ts parity quick --owner 🏷️class-name-composition
   [test] level=quick cases=2 executed=11 passed=11 failed=0 errored=0 parity=4/4
   • flatten-class-name-inputs   → 4 @mode-differential scenarios against clsx 2.1.1
   • merge-conflicting-utilities → 3 @mode-conformance scenarios against specification
                                    vectors, under the recorded no-oracle decision
                                    `repository-owned-utility-groups` (tailwind-merge is
                                    NOT a valid oracle for repository-owned families)
   Legacy 🧪️component.test.ts deleted, its ⚛️react vitest include entry removed,
   baseline 🧰️framework count lowered 37 → 36.

4. Owner migration 2 — 🏷️style-variants, oracle `class-variance-authority`
   $ bun ./📜️script.ts parity quick --owner 🏷️style-variants
   [test] level=quick cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3
   styleVariants() matches the published cva contract across base-only, the full
   single-variant matrix including the explicit null opt-out, boolean choices and
   compound conjunctions with array conditions.
   Legacy 🧪️component.test.ts deleted, include entry removed, baseline 36 → 35.

5. §13 non-aggregate metrics
   $ bun ./📜️script.ts test metrics
   [metrics] scenario coverage      13/18 (72.2%)
   [metrics] oracle coverage        6/6 (100.0%)
   [metrics] dependency-clean       54/208 (26.0%)
   [metrics] implementation dotnet      3/3 scenarios
   [metrics] implementation go          3/3 scenarios
   [metrics] implementation python      3/3 scenarios
   [metrics] implementation rust        3/8 scenarios
   [metrics] implementation typescript  13/13 scenarios
   [metrics] parity         dotnet      12/12 · go 12/12 · python 12/12 · rust 12/12 · typescript 19/19

   $ bun ./📜️script.ts test metrics --enforce      → exit 1, naming exactly:
     scenario coverage 13/18: the five PDF scenarios
     implementation coverage rust 3/8 — a claimed implementation did not run
   Both gaps are the peer-blocked PDF subject phase. The gate is doing its job.

6. Dependency gates after two new registered oracles
   $ bun ./📜️script.ts verify dependencies       → clean, 208 entries
   $ bun ./📜️script.ts dependency
   [dependency] ecosystems=4 entries=208 production-reachable=154 test-oracle=4
   [dependency] test-oracle js:class-variance-authority@0.7.1 (cva)
   [dependency] test-oracle js:clsx@2.1.1 (clsx)
   [dependency] test-oracle rust:lopdf@0.44 (lopdf)
   [dependency] test-oracle rust:pdf-writer@0.15 (pdf-writer)
   production-reachable held at 154 across both additions — the ratchet only admitted
   them because the oracle registry claims them.

7. Platform self-tests
   $ bun test ./🧪️index.test.ts       39 pass  0 fail  554 expect() calls

8. Generated Nx case target for a migrated owner
   $ bun nx run test-…-stylevariants-bcdf79-compile-style-variants:test-quick --skip-nx-cache
   [test] level=quick cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3
   NX   Successfully ran target test-quick

────────────────────────────────────────────────────────────────────────────────
BUGS FOUND AND FIXED IN THIS WAVE

a) Root `test <phase> <level>` silently ran the WRONG level. `TestScript` resolved the
   level from `segments[0]`, which is the PHASE word, so `test parity quick` ran at
   `fundamental` — 5 of 7 scenarios, reported as a pass. Now the level is resolved over
   the segments that FOLLOW the phase, and both `test quick parity` and
   `test parity quick` produce the identical 7/7 run. Caught by the new metrics gate.

b) The oracle-purity scan produced FALSE POSITIVES on any narrowed run. Its exclusion
   set was derived from the CALLER's selected cases, so `contract --case X` — which is
   exactly what every generated Nx case target runs — reported every OTHER case's
   adapter as "production source importing a registered oracle". The first Nx run of the
   style-variants target failed for this reason. The exclusion is now derived from a full
   repository discovery inside the scan itself, and both the positive and the narrowed-scope
   regression are covered by self-tests.

c) The no-oracle rule was too blunt. It demanded ≥2 implementations for EVERY recorded
   no-oracle case, including conformance cases whose decision rests on specification
   vectors — a rule the decision never claimed to meet. Now the requirement follows the
   decision's own declared `substitutes`, and a `@mode-differential` scenario with
   neither an oracle nor a second implementation is caught at CONTRACT time.

STILL BLOCKED, NOT BY THIS WORK
   `semio-framework-os-kernel` still fails (179 errors): 162 `semio_framework_job::`
   references in 🏪️store/🦀️component.rs with no such dependency in the os-kernel
   manifest, introduced by commit 9d7cabfd9c from a concurrent session. The PDF pilot's
   oracle AND subject phases both link that crate transitively and therefore cannot run.
