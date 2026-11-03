"""🥫️ Independent oracle of the enhanced yield strength of cold forming (EN 1993-1-3 3.2.2): f_ya = f_yb + (f_u - f_yb) k n t^2 / A_g limited to (f_u + f_yb) / 2 with k = 7 for cold roll forming and 5 for other forming (registry rows, unverified), evaluated closed form and with `numpy.minimum`. Run with `python -I`; it prints the JSON that the Rust unit tests embed."""

import json

import numpy as np


def average_yield(fyb, fu, k, n, t, ag):
    return float(np.minimum(fyb + (fu - fyb) * k * n * t * t / ag, (fu + fyb) / 2))


def main():
    out = {
        "showcase": average_yield(350e6, 420e6, 7.0, 4, 0.003, 0.0006),
        "other_forming": average_yield(350e6, 420e6, 5.0, 4, 0.003, 0.0006),
        "capped": average_yield(350e6, 420e6, 7.0, 40, 0.003, 0.0006),
        "no_bends": average_yield(350e6, 420e6, 7.0, 0, 0.003, 0.0006),
        "cap": (420e6 + 350e6) / 2,
    }
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
