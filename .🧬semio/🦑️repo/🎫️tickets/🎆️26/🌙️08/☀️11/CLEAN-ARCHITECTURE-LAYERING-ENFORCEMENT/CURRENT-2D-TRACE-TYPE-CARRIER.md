# Current 2D Trace Type Carrier

The actual full framework typecheck first failed with two readonly-vector diagnostics in the newly added trace test helper. Root changed only three type annotations: imported the public Vec2 type, used Vec2[][] for collected rings, and accepted readonly Vec2[] in the area helper. All production code, test inputs, assertions, expected geometry, masks and timers remain byte-for-byte retained by the exact inverse.

The actual uncached Nx framework:typecheck then passed in 14.2 seconds, session 25368. The existing uncached full s-2d-js:test route passed all 91 current tests in five files, zero failures; Vitest took 6.96 seconds and Nx took 11.7 seconds, session 8733. These are current full-route results, not the historical 67-test roster.

Complete original and repaired source bytes remain in framework-script-inputs/trace-type-carrier-repairs.json. All three inverse replacements are unique and reconstruct the exact original; the current source matches the repaired capture. Terminal receipts and both complete logs remain under 🗑️generated/goal-root/sqlite-refusal. No geometry production feature or package rename is attributed to this annotation repair.
