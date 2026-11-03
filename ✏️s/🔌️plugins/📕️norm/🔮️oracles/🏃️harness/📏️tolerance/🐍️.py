"""📏️ The five Eurocode oracle tolerance profiles, mirrored in the root `🔣️.json` `comparisonProfiles`.

See `📐️specs/oracle-strategy.md` section 5.4 of the ticket NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS.
"""

PROFILES = {
    "closed-form-v1": {"relative": 1e-9, "absolute": 1e-12},
    "eurocode-quantities-v1": {"relative": 1e-4, "absolute": 1e-9},
    "numerical-v1": {"relative": 5e-3, "absolute": 1e-9},
    "method-spread-v1": {"relative": 3e-2, "absolute": 1e-9},
    "table-exact-v1": {"relative": 0.0, "absolute": 0.0},
}

UTILIZATION_TOLERANCE = 1e-9


def within_limit(utilization):
    """🎯️ The shared limit rule, mirrored from `norm_compliance::within_limit`: a utilisation within the relative `UTILIZATION_TOLERANCE` of 1 is within its limit."""
    return utilization <= 1.0 + UTILIZATION_TOLERANCE


def limit_status(utilization):
    """🎯️ The shared status of a utilisation against its limit: `pass` within `within_limit`, otherwise `fail`."""
    return "pass" if within_limit(utilization) else "fail"


def tolerance(profile, override=None):
    """⚖️ Relative tolerance of `profile`, or the explicit per-case `override`."""
    return override if override is not None else PROFILES[profile]["relative"]


def within(profile, expected, actual, override=None):
    """✅️ True when `actual` equals `expected` under the profile's `max(absolute, relative * scale)` rule."""
    rule = PROFILES[profile]
    allowed = max(rule["absolute"], tolerance(profile, override) * max(abs(expected), abs(actual)))
    return abs(expected - actual) <= allowed


def relative_error(expected, actual):
    """📉️ Scale-relative difference used for reports."""
    scale = max(abs(expected), abs(actual))
    return 0.0 if scale == 0.0 else abs(expected - actual) / scale
