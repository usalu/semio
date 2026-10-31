"""🖌️ Preview-only drawing of the shared Nakagin visuals: the plain painter's sort and tower capsules without small hardware.

The exact occlusion pipeline (`_opaque_visible`, `_section_order` with BIM drafting fragments) needs minutes per tower
and gigabytes per process, and a whole tower carries about 345 000 facets (hinges, bolts, gaskets, pipes) that cost
about 22 s per 480p frame while invisible at tower scale. Inside the preview process only, this swaps the occlusion
for the classic tier-then-depth sort and keeps the capsule envelope and window parts of the 140 tower slots.
Single-capsule drawings keep every part. The shared module on disk is untouched.
"""
import importlib

TOWER_ROLES = ("shell", "shell_back", "window_rim", "window_frame", "glazing")
HIDDEN_CORE_ROLES = ("stair_tread", "stair_rail", "stair_stringer", "landing")


def _painter(visuals, items, view):
    """🎨 Front-facing items, far to near within each tier."""
    towards = visuals._basis(view)[1]
    front = [item for item in items if item.tier < 0 or item.normal is None or item.normal @ towards > 1e-6]
    return sorted(front, key=lambda item: (item.tier, visuals._order(view, item.centre)))


def install():
    """🔁 Patch the imported Nakagin visuals module for quick previews."""
    visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
    slot_parts = visuals._slot_parts
    visuals._opaque_visible = lambda items, view, padding=0.0: list(items)
    visuals._section_order = lambda items, view, progress=None, cancelled=None: _painter(visuals, items, view)
    core_parts = visuals._core_parts
    visuals._slot_parts = lambda slot, segments=None: tuple(row for row in slot_parts(slot, segments) if row[0].role in TOWER_ROLES)
    visuals._core_parts = lambda *args: tuple(row for row in core_parts(*args) if row[0].role not in HIDDEN_CORE_ROLES)
    return visuals
