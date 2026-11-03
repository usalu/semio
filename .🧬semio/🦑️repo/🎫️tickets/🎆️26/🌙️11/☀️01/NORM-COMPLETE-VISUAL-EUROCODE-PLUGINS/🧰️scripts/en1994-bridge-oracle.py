"""Independent derivation of the EN 1993-1-5 Table 3.1 effective-width factors used by the EN 1994 bridge module (hand formulas, no shared code)."""
for k in (0.01, 0.02, 0.1, 0.5, 0.7, 1.0):
    b1 = 1.0 if k <= 0.02 else (1 / (1 + 6.4 * k * k) if k <= 0.7 else 1 / (5.9 * k))
    b2 = 1.0 if k <= 0.02 else (1 / (1 + 6.0 * (k - 1 / (2500 * k)) + 1.6 * k * k) if k <= 0.7 else 1 / (8.6 * k))
    b0 = min((0.55 + 0.025 / k) * b1, b1)
    print(f'kappa={k}: beta1={b1:.5f} beta2={b2:.5f} beta0={b0:.5f}')
