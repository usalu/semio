"""EN 1999 v4 reference for the fatigue spectrum: independent numpy implementation of the bilinear S-N curve and the Palmgren-Miner sum, printing the numbers asserted in the Rust unit tests of `fatigue-math`. Usage: /home/deck/semio/.venv/bin/python -I miner_ref.py"""
import numpy as np

NC, ND, NL = 2e6, 5e6, 1e8


def endurance(ds, dc, m1, m2):
    dd = dc * (NC / ND) ** (1 / m1)
    dl = dd * (ND / NL) ** (1 / m2)
    ds = np.asarray(ds, float)
    return np.where(ds >= dd, NC * (dc / ds) ** m1, np.where(ds < dl, np.inf, ND * (dd / ds) ** m2))


def spectrum(blocks, dc, m1, m2):
    b = np.asarray(blocks, float)
    d = b[:, 1] / endurance(b[:, 0], dc, m1, m2)
    return d, float(d.sum()), float((np.sum(b[:, 1] * b[:, 0] ** m1) / b[:, 1].sum()) ** (1 / m1))


if __name__ == "__main__":
    blocks = [(60.0, 2e5), (40.0, 1e6), (25.0, 5e6), (12.0, 2e7)]
    d, total, eq = spectrum(blocks, 71.0, 3.4, 5.4)
    assert abs(total - 0.09509099032433865) < 1e-12 and abs(d[0] - 0.05642036165499026) < 1e-12 and d[2] == 0.0
    assert abs(eq - 21.430183581707396) < 1e-9
    dd = 71.0 * (NC / ND) ** (1 / 3.4)
    assert abs(dd - 54.2271986648574) < 1e-9 and abs(dd * (ND / NL) ** (1 / 5.4) - 31.13763969961171) < 1e-9
    print("miner reference ok", d.tolist(), total, eq)
