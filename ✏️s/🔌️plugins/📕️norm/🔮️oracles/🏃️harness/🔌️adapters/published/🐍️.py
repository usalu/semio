"""📚️ Published-table adapter: values printed in the standards or textbooks, keyed independently of the Rust tables; exact within the printed rounding (use a per-case tolerance of half the last printed digit)."""

LIBRARY = "published"

ISO_834 = {5: 576.0, 10: 678.0, 15: 739.0, 30: 842.0, 60: 945.0, 90: 1006.0, 120: 1049.0, 180: 1110.0, 240: 1153.0}
KN_UNKNOWN = {3: 3.37, 4: 2.63, 5: 2.33, 6: 2.18, 8: 2.00, 10: 1.92, 20: 1.76, 30: 1.73}
KN_KNOWN = {3: 1.89, 4: 1.83, 5: 1.80, 6: 1.77, 8: 1.74, 10: 1.72, 20: 1.68, 30: 1.67}


def iso834(i):
    """🔥️ ISO 834-1 and EN 1991-1-2 3.2.1 standard fire: gas temperature at t minutes as tabulated in fire-engineering handbooks (rounded to 1 K)."""
    return {"theta_g": ISO_834[int(i["t_min"])]}


def annex_d_kn(i):
    """🧪️ EN 1990 Annex D Table D.1 k_n at the 5 % fractile for known and unknown coefficient of variation."""
    return {"k_n_unknown": KN_UNKNOWN[int(i["n"])], "k_n_known": KN_KNOWN[int(i["n"])]}


CHECKS = {"en1991.fire.iso834": iso834, "en1990.D.7.characteristic-from-tests": annex_d_kn}
