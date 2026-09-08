# Descriptor Emission Budget Propagation

The registered registry regression after required-root ownership passed22 tests but failed its existing emission-abort test (session11530): expected the deliberate stop at emit, but got “descriptor emission deadline of 0ms exceeded at emit.” The repository's current buildBudgetMs default is0, explicitly meaning unlimited. The emitter instead compared elapsed time against0 and clamped child build/emitter budgets to at least1ms. This is a real producer contract regression, not a reason to loosen the test.

A closed language-neutral five-row fixture under the descriptor owner's 🧪️fixtures/⏱️emission-budget covers unlimited, before-limit, at-limit, after-limit, and already-exhausted budgets. The registry's actual Vitest gate validates AJV, derives independent interval/remaining outputs using Decimal, controls Date.now deterministically, and always aborts before native emission. Both prior owner files must remain unchanged. The old child-budget math was extracted behavior-neutrally into remainingDescriptorEmissionBudgetMs before repair.

The deterministic test-first run was RED (session68338): the unlimited row expected the intentional pre-emission abort but encountered a zero-ms expiry. The other23 registry tests passed in that run; no Cargo/emitter child was launched.

Production now preserves0 through the stage guard and both child-process budgets. Positive deadlines retain their existing strict greater-than boundary and minimum1ms remaining child value; exhausted negative input still fails before emission. The complete four-file24-test registry gate is rerunning as session74822. Scoped whitespace checks passed. No fresh descriptor or GIS asset is claimed by these source/light-runtime tests.


Final registered registry regression session74822 is GREEN: four files,24 tests, including all five deterministic budget rows. AJV1/Decimal5, exact prior owner-pair preservation, and zero native-emitter starts are confirmed by the test DEBUG output. The emitter guard and child-budget helper are qualified at this boundary; actual fresh Wasm emission remains a separate native requirement.
