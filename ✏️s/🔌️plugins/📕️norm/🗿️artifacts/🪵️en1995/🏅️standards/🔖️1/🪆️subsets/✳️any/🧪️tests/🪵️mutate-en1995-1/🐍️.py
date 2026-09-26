"""🐍️ EN 1995's contribution to the norm reference implementation — the four things that are
genuinely per-standard, and nothing else.

The second producer this case's differential comparison needs is
`semio_norm_vocabulary`, the ONE independent Python implementation of the norm mutation vocabulary,
imported here rather than copied. Its module docstring carries the survey that established no
third-party library reads or writes `s.norm.*`, the two committed documents it was written from, and
the honest boundary on the `.dsl.semio` carrier. This file adds no verb, no addressing rule and no
carrier rule: everything below is DATA read off this subset's own committed catalog, its own
committed specification vectors and its own committed example document.

Stating it this way is the point. The fifteen norm adapters used to hold fifteen byte-identical
copies of that engine, which made the reference surface read as fifteen independent implementations
when it was one. One import says what fifteen copies concealed — a shared bug here agrees with itself
in all fifteen cases, and that is now visible instead of pretended.
"""

from __future__ import annotations

# region 🔖️Imports
from importlib import import_module

_vocabulary = import_module("🐍️")
Subset = _vocabulary.Subset
build_adapter = _vocabulary.build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind this subset's committed catalog declares, in catalog order.
KINDS = [
    "change-annex",
    "insert-member",
    "remove-member",
    "change-member-label-en",
    "change-member-label-de",
    "change-member-role",
    "change-member-strength-class",
    "change-member-service-class",
    "change-member-support",
    "change-member-b",
    "change-member-h",
    "change-member-span",
    "change-member-support-length",
    "change-member-bearing-length",
    "change-member-buckling-y",
    "change-member-buckling-z",
    "change-member-lateral-restraint",
    "change-member-notch-depth",
    "change-member-notch-distance",
    "change-member-m-crit",
    "change-member-mass-per-m",
    "change-member-mass-per-m2",
    "change-member-damping",
    "change-member-fire-duration",
    "change-member-bridge-n-obs",
    "change-member-bridge-tl-years",
    "change-member-bridge-beta",
    "change-member-bridge-a",
    "change-member-bridge-b",
    "change-member-bridge-crowd",
    "insert-member-action",
    "remove-member-action",
    "change-member-action-kind",
    "change-member-action-category",
    "change-member-action-load-duration",
    "change-member-action-q-line",
    "change-member-action-f-point",
    "change-member-action-mk",
    "change-member-action-vk",
    "change-member-action-nk",
    "change-member-action-ntk",
    "change-member-action-fc90-k",
    "insert-connection",
    "remove-connection",
    "change-connection-label-en",
    "change-connection-label-de",
    "change-connection-fastener-type",
    "change-connection-strength-class",
    "change-connection-service-class",
    "change-connection-diameter",
    "change-connection-number",
    "change-connection-rows",
    "change-connection-spacing",
    "change-connection-edge-distance",
    "change-connection-end-distance",
    "change-connection-t1",
    "change-connection-t2",
    "change-connection-steel-plate",
    "change-connection-steel-plate-thickness",
    "change-connection-shear-planes",
    "change-connection-fuk",
    "insert-connection-action",
    "remove-connection-action",
    "change-connection-action-kind",
    "change-connection-action-load-duration",
    "change-connection-action-fk",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🌍️change-annex", "🌍️switches-from-the-german-na-to-the-recommended-en-annex"),
    "insert-member": ("➕️insert-member", "➕️inserts-a-member-at-end"),
    "remove-member": ("➖️remove-member", "➖️removes-the-first-member"),
    "change-member-label-en": ("🏷️change-member-label-en", "✏️sets-labelEn"),
    "change-member-label-de": ("🏷️change-member-label-de", "✏️sets-labelDe"),
    "change-member-role": ("🎯️change-member-role", "✏️sets-role"),
    "change-member-strength-class": ("🛡️change-member-strength-class", "✏️sets-strengthClass"),
    "change-member-service-class": ("🌧️change-member-service-class", "✏️sets-serviceClass"),
    "change-member-support": ("📍️change-member-support", "✏️sets-support"),
    "change-member-b": ("↔️change-member-b", "✏️sets-bM"),
    "change-member-h": ("↕️change-member-h", "✏️sets-hM"),
    "change-member-span": ("↔️change-member-span", "✏️sets-spanM"),
    "change-member-support-length": ("↔️change-member-support-length", "✏️sets-supportLengthM"),
    "change-member-bearing-length": ("↔️change-member-bearing-length", "✏️sets-bearingLengthM"),
    "change-member-buckling-y": ("↔️change-member-buckling-y", "✏️sets-bucklingLengthYM"),
    "change-member-buckling-z": ("↔️change-member-buckling-z", "✏️sets-bucklingLengthZM"),
    "change-member-lateral-restraint": ("↔️change-member-lateral-restraint", "✏️sets-lateralRestraintSpacingM"),
    "change-member-notch-depth": ("↔️change-member-notch-depth", "✏️sets-notchDepthM"),
    "change-member-notch-distance": ("↔️change-member-notch-distance", "✏️sets-notchDistanceM"),
    "change-member-m-crit": ("⚠️change-member-m-crit", "✏️sets-mCritNm"),
    "change-member-mass-per-m": ("⚖️change-member-mass-per-m", "✏️sets-massKgPerM"),
    "change-member-mass-per-m2": ("⚖️change-member-mass-per-m2", "✏️sets-massKgPerM2"),
    "change-member-damping": ("🌊️change-member-damping", "✏️sets-dampingXi"),
    "change-member-fire-duration": ("🔥️change-member-fire-duration", "✏️sets-fireDurationS"),
    "change-member-bridge-n-obs": ("🌉️change-member-bridge-n-obs", "✏️sets-bridgeNObs"),
    "change-member-bridge-tl-years": ("🌉️change-member-bridge-tl-years", "✏️sets-bridgeTLYears"),
    "change-member-bridge-beta": ("🌉️change-member-bridge-beta", "✏️sets-bridgeBeta"),
    "change-member-bridge-a": ("🌉️change-member-bridge-a", "✏️sets-bridgeA"),
    "change-member-bridge-b": ("🌉️change-member-bridge-b", "✏️sets-bridgeB"),
    "change-member-bridge-crowd": ("🚶️change-member-bridge-crowd", "✏️sets-bridgeCrowdPerM2"),
    "insert-member-action": ("➕️insert-member-action", "➕️inserts-an-action-at-end-of-member"),
    "remove-member-action": ("➖️remove-member-action", "➖️removes-the-first-action-of-member"),
    "change-member-action-kind": ("⚖️change-member-action-kind", "✏️sets-kind"),
    "change-member-action-category": ("🏢️change-member-action-category", "✏️sets-category"),
    "change-member-action-load-duration": ("⏳️change-member-action-load-duration", "✏️sets-loadDuration"),
    "change-member-action-q-line": ("⬇️change-member-action-q-line", "✏️sets-qLineNPerM"),
    "change-member-action-f-point": ("⬇️change-member-action-f-point", "✏️sets-fPointN"),
    "change-member-action-mk": ("⤴️change-member-action-mk", "✏️sets-mKNm"),
    "change-member-action-vk": ("↕️change-member-action-vk", "✏️sets-vKN"),
    "change-member-action-nk": ("🏋️change-member-action-nk", "✏️sets-nKN"),
    "change-member-action-ntk": ("🏋️change-member-action-ntk", "✏️sets-nTKN"),
    "change-member-action-fc90-k": ("🏋️change-member-action-fc90-k", "✏️sets-fC90KN"),
    "insert-connection": ("➕️insert-connection", "➕️inserts-a-connection-at-end"),
    "remove-connection": ("➖️remove-connection", "➖️removes-the-first-connection"),
    "change-connection-label-en": ("🏷️change-connection-label-en", "✏️sets-labelEn"),
    "change-connection-label-de": ("🏷️change-connection-label-de", "✏️sets-labelDe"),
    "change-connection-fastener-type": ("🔩️change-connection-fastener-type", "✏️sets-fastenerType"),
    "change-connection-strength-class": ("🛡️change-connection-strength-class", "✏️sets-strengthClass"),
    "change-connection-service-class": ("🌧️change-connection-service-class", "✏️sets-serviceClass"),
    "change-connection-diameter": ("↔️change-connection-diameter", "✏️sets-diameterM"),
    "change-connection-number": ("🔢️change-connection-number", "✏️sets-number"),
    "change-connection-rows": ("🔢️change-connection-rows", "✏️sets-rows"),
    "change-connection-spacing": ("↔️change-connection-spacing", "✏️sets-spacingM"),
    "change-connection-edge-distance": ("↔️change-connection-edge-distance", "✏️sets-edgeDistanceM"),
    "change-connection-end-distance": ("↔️change-connection-end-distance", "✏️sets-endDistanceM"),
    "change-connection-t1": ("↔️change-connection-t1", "✏️sets-t1M"),
    "change-connection-t2": ("↔️change-connection-t2", "✏️sets-t2M"),
    "change-connection-steel-plate": ("🔩️change-connection-steel-plate", "✏️sets-steelPlate"),
    "change-connection-steel-plate-thickness": ("↔️change-connection-steel-plate-thickness", "✏️sets-steelPlateThicknessM"),
    "change-connection-shear-planes": ("🔢️change-connection-shear-planes", "✏️sets-shearPlanes"),
    "change-connection-fuk": ("🛡️change-connection-fuk", "✏️sets-fUK"),
    "insert-connection-action": ("➕️insert-connection-action", "➕️inserts-an-action-at-end-of-connection"),
    "remove-connection-action": ("➖️remove-connection-action", "➖️removes-the-first-action-of-connection"),
    "change-connection-action-kind": ("⚖️change-connection-action-kind", "✏️sets-kind"),
    "change-connection-action-load-duration": ("⏳️change-connection-action-load-duration", "✏️sets-loadDuration"),
    "change-connection-action-fk": ("🔩️change-connection-action-fk", "✏️sets-fKN"),
}

#: 🗣️ The real committed EN 1995 document, read where the domain already keeps it.
DSL_ASSET = "asset://🏠️glulam-floor-beam/🏠️glulam-floor-beam/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1995.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1995", KINDS, VECTORS, DSL_ASSET, ENVELOPE, vector_root="shared://🧬️mutations"))
# endregion 🔖️Registration
