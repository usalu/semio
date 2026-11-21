"""🧥 Round 5 garment forms of `humans/courier-with-hand-truck`, stated by hand once and shared by every scale so that all four scales keep the same proportions (1.82 m male, DIN 33402-2 / ANSUR II girths plus the ease of jacket and work trousers). Each table row is one cross-section: (u along the bone, centre v sagittal, centre w lateral, half width, half depth, superellipse exponent); a coarser scale picks fewer rows of the same table (`pick`). Torso frame: u up from the lumbosacral end of the pelvis (world y 1.060), front is -v. Limb frames: u down the limb, front of the hanging limb +v, w lateral (left +).
@see ./r3-humans-courier-with-hand-truck.md"""

JACKET = [
    (-0.130, 0.004, 0, 0.194, 0.133, 2.4),
    (-0.075, 0.003, 0, 0.189, 0.129, 2.4),
    (-0.010, 0.000, 0, 0.176, 0.122, 2.3),
    (0.060, -0.002, 0, 0.164, 0.119, 2.3),
    (0.150, -0.006, 0, 0.170, 0.125, 2.3),
    (0.250, -0.010, 0, 0.180, 0.134, 2.25),
    (0.320, -0.006, 0, 0.186, 0.133, 2.2),
    (0.380, 0.002, 0, 0.196, 0.124, 2.2),
    (0.425, 0.008, 0, 0.200, 0.110, 2.15),
    (0.452, 0.011, 0, 0.192, 0.097, 2.1),
    (0.474, 0.012, 0, 0.158, 0.085, 2.0),
    (0.492, 0.011, 0, 0.108, 0.075, 2.0),
    (0.506, 0.009, 0, 0.078, 0.069, 2.0),
]

SLEEVE = [
    (-0.042, -0.004, -0.018, 0.010, 0.014, 2.0),
    (-0.034, -0.004, -0.013, 0.032, 0.038, 2.0),
    (-0.018, -0.002, -0.005, 0.049, 0.052, 2.1),
    (0.004, 0.000, 0.002, 0.059, 0.059, 2.2),
    (0.040, 0.002, 0.008, 0.063, 0.062, 2.2),
    (0.110, 0.003, 0.007, 0.058, 0.060, 2.2),
    (0.190, 0.004, 0.003, 0.052, 0.057, 2.2),
    (0.270, 0.002, 0.000, 0.049, 0.053, 2.2),
    (0.3366, -0.002, 0.000, 0.047, 0.050, 2.2),
    (0.372, -0.002, 0.000, 0.046, 0.048, 2.2),
]

FOREARM = [
    (-0.032, -0.002, 0, 0.044, 0.046, 2.2),
    (0.040, 0.000, 0, 0.047, 0.044, 2.2),
    (0.110, 0.000, 0, 0.044, 0.040, 2.25),
    (0.180, 0.000, 0, 0.040, 0.035, 2.3),
    (0.238, 0.000, 0, 0.037, 0.032, 2.4),
    (0.250, 0.000, 0, 0.036, 0.031, 2.4),
]

THIGH = [
    (-0.030, -0.004, -0.008, 0.084, 0.098, 2.3),
    (0.050, 0.000, -0.004, 0.091, 0.101, 2.3),
    (0.150, 0.004, -0.006, 0.087, 0.094, 2.3),
    (0.260, 0.006, -0.008, 0.080, 0.085, 2.25),
    (0.360, 0.005, -0.008, 0.070, 0.074, 2.2),
    (0.4437, 0.004, -0.006, 0.0625, 0.067, 2.2),
    (0.474, 0.004, -0.006, 0.061, 0.0655, 2.2),
]

SHANK = [
    (-0.030, 0.004, -0.006, 0.058, 0.062, 2.2),
    (0.0303, 0.004, -0.006, 0.061, 0.0655, 2.2),
    (0.130, -0.008, -0.002, 0.064, 0.071, 2.2),
    (0.220, -0.006, 0.000, 0.058, 0.064, 2.2),
    (0.320, -0.002, 0.000, 0.053, 0.058, 2.3),
    (0.375, 0.004, 0.000, 0.054, 0.063, 2.4),
]

SEAT = [
    (-0.105, 0.000, 0, 0.075, 0.085, 2.3),
    (-0.050, 0.008, 0, 0.160, 0.115, 2.3),
    (0.000, 0.010, 0, 0.170, 0.119, 2.3),
    (0.070, 0.004, 0, 0.165, 0.116, 2.3),
    (0.130, 0.000, 0, 0.158, 0.113, 2.3),
]

POCKET = [
    (0.200, 0.004, 0.066, 0.020, 0.056, 2.8),
    (0.215, 0.004, 0.068, 0.022, 0.060, 2.8),
    (0.340, 0.006, 0.060, 0.022, 0.060, 2.8),
    (0.355, 0.006, 0.058, 0.020, 0.056, 2.8),
]


def pick(table, rows):
    """The rows of a table with the given indices (a coarser scale states fewer sections of the same form)."""
    return [table[i] for i in rows]


def at(table, u):
    """The section of a table at u, linearly between the stated rows (for bands and seams that must lie on the garment)."""
    for a, b in zip(table, table[1:]):
        if a[0] <= u <= b[0]:
            t = (u - a[0]) / (b[0] - a[0])
            return tuple([u] + [a[i] + t * (b[i] - a[i]) for i in range(1, 6)])
    raise ValueError(u)


def sections(S, table, grow=0.0, ease=None):
    """Kit sections of the table rows, every half extent grown by `grow` (a band over the jacket) or overridden exponent `ease`."""
    return [S(u, cv, cw, rw + grow, rv + grow, ease or e) for u, cv, cw, rw, rv, e in table]


def band(S, u0, u1, grow=0.002):
    """A 50 mm reflective band lying `grow` proud of the jacket between u0 and u1."""
    return sections(S, [at(JACKET, u0), at(JACKET, (u0 + u1) / 2), at(JACKET, u1)], grow)
