import importlib
import sys

import numpy as np

sys.path.insert(0, "C:/git/semio")
ex = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
pk = (ex.H_T_W_K + ex.H_V_W_K) * ex.D_THETA_DESIGN_K / 1000
print(pk, ex.Q_LOSS_KWH_PER_A, ex.Q_SOL_KWH_PER_A + ex.Q_INT_KWH_PER_A)


def load(g, n=8760, seed=5):
    rng = np.random.default_rng(seed)
    t = np.linspace(0, 1, n)
    w = 240
    noise = np.convolve(rng.normal(0, 1, n + 2 * w), np.ones(w) / w, "same")[w:w + n]
    th = 9 - 9.5 * np.cos(2 * np.pi * t) + 3 * noise / noise.std()
    x = np.maximum(0, g - th)
    return x / x.max()


for g in (11, 11.5, 11.7, 12):
    print(g, load(g).sum() * pk)
