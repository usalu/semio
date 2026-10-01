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
    "change-member-buckling-length-y",
    "change-member-buckling-length-z",
    "change-member-restraint-spacing",
    "change-member-notch-depth",
    "change-member-notch-distance",
    "change-member-m-crit",
    "change-member-mass-kg-per-m",
    "change-member-mass-kg-per-m2",
    "change-member-damping-xi",
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
    "change-member-load-duration",
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
    "change-connection-plate-thickness",
    "change-connection-shear-planes",
    "change-connection-fuk",
    "insert-connection-action",
    "remove-connection-action",
    "change-connection-action-kind",
    "change-connection-load-duration",
    "change-connection-action-fk",
]

#: 🧫️ The committed specification vectors: each kind's `✅apply` vector as (triad directory, fixture), and each refusal row
#: `<kind>-<slug>` as (kind, triad directory, fixture).
VECTORS = {
    "change-annex": ("🌍️change-annex", "✅apply"),
    "insert-member": ("➕️insert-member", "✅apply"),
    "insert-member-dupe": ("insert-member", "➕️insert-member", "⛔dupe"),
    "remove-member": ("➖️remove-member", "✅apply"),
    "change-member-label-en": ("🏷️change-member-label-en", "✅apply"),
    "change-member-label-de": ("🏷️change-member-label-de", "✅apply"),
    "change-member-role": ("🎯️change-member-role", "✅apply"),
    "change-member-strength-class": ("🛡️change-member-strength-class", "✅apply"),
    "change-member-service-class": ("🌧️change-member-service-class", "✅apply"),
    "change-member-support": ("📍️change-member-support", "✅apply"),
    "change-member-b": ("↔️change-member-b", "✅apply"),
    "change-member-h": ("↕️change-member-h", "✅apply"),
    "change-member-span": ("↔️change-member-span", "✅apply"),
    "change-member-support-length": ("↔️change-member-support-length", "✅apply"),
    "change-member-bearing-length": ("↔️change-member-bearing-length", "✅apply"),
    "change-member-buckling-length-y": ("↔️change-member-buckling-length-y", "✅apply"),
    "change-member-buckling-length-z": ("↔️change-member-buckling-length-z", "✅apply"),
    "change-member-restraint-spacing": ("↔️change-member-restraint-spacing", "✅apply"),
    "change-member-notch-depth": ("↔️change-member-notch-depth", "✅apply"),
    "change-member-notch-distance": ("↔️change-member-notch-distance", "✅apply"),
    "change-member-m-crit": ("⚠️change-member-m-crit", "✅apply"),
    "change-member-mass-kg-per-m": ("⚖️change-member-mass-kg-per-m", "✅apply"),
    "change-member-mass-kg-per-m2": ("⚖️change-member-mass-kg-per-m2", "✅apply"),
    "change-member-damping-xi": ("🌊️change-member-damping-xi", "✅apply"),
    "change-member-fire-duration": ("🔥️change-member-fire-duration", "✅apply"),
    "change-member-bridge-n-obs": ("🌉️change-member-bridge-n-obs", "✅apply"),
    "change-member-bridge-tl-years": ("🌉️change-member-bridge-tl-years", "✅apply"),
    "change-member-bridge-beta": ("🌉️change-member-bridge-beta", "✅apply"),
    "change-member-bridge-a": ("🌉️change-member-bridge-a", "✅apply"),
    "change-member-bridge-b": ("🌉️change-member-bridge-b", "✅apply"),
    "change-member-bridge-crowd": ("🚶️change-member-bridge-crowd", "✅apply"),
    "insert-member-action": ("➕️insert-member-action", "✅apply"),
    "remove-member-action": ("➖️remove-member-action", "✅apply"),
    "change-member-action-kind": ("⚖️change-member-action-kind", "✅apply"),
    "change-member-action-category": ("🏢️change-member-action-category", "✅apply"),
    "change-member-load-duration": ("⏳️change-member-load-duration", "✅apply"),
    "change-member-action-q-line": ("⬇️change-member-action-q-line", "✅apply"),
    "change-member-action-f-point": ("⬇️change-member-action-f-point", "✅apply"),
    "change-member-action-mk": ("⤴️change-member-action-mk", "✅apply"),
    "change-member-action-vk": ("↕️change-member-action-vk", "✅apply"),
    "change-member-action-nk": ("🏋️change-member-action-nk", "✅apply"),
    "change-member-action-ntk": ("🏋️change-member-action-ntk", "✅apply"),
    "change-member-action-fc90-k": ("🏋️change-member-action-fc90-k", "✅apply"),
    "insert-connection": ("➕️insert-connection", "✅apply"),
    "insert-connection-dupe": ("insert-connection", "➕️insert-connection", "⛔dupe"),
    "remove-connection": ("➖️remove-connection", "✅apply"),
    "change-connection-label-en": ("🏷️change-connection-label-en", "✅apply"),
    "change-connection-label-de": ("🏷️change-connection-label-de", "✅apply"),
    "change-connection-fastener-type": ("🔩️change-connection-fastener-type", "✅apply"),
    "change-connection-strength-class": ("🛡️change-connection-strength-class", "✅apply"),
    "change-connection-service-class": ("🌧️change-connection-service-class", "✅apply"),
    "change-connection-diameter": ("↔️change-connection-diameter", "✅apply"),
    "change-connection-number": ("🔢️change-connection-number", "✅apply"),
    "change-connection-rows": ("🔢️change-connection-rows", "✅apply"),
    "change-connection-spacing": ("↔️change-connection-spacing", "✅apply"),
    "change-connection-edge-distance": ("↔️change-connection-edge-distance", "✅apply"),
    "change-connection-end-distance": ("↔️change-connection-end-distance", "✅apply"),
    "change-connection-t1": ("↔️change-connection-t1", "✅apply"),
    "change-connection-t2": ("↔️change-connection-t2", "✅apply"),
    "change-connection-steel-plate": ("🔩️change-connection-steel-plate", "✅apply"),
    "change-connection-plate-thickness": ("↔️change-connection-plate-thickness", "✅apply"),
    "change-connection-shear-planes": ("🔢️change-connection-shear-planes", "✅apply"),
    "change-connection-fuk": ("🛡️change-connection-fuk", "✅apply"),
    "insert-connection-action": ("➕️insert-connection-action", "✅apply"),
    "remove-connection-action": ("➖️remove-connection-action", "✅apply"),
    "change-connection-action-kind": ("⚖️change-connection-action-kind", "✅apply"),
    "change-connection-load-duration": ("⏳️change-connection-load-duration", "✅apply"),
    "change-connection-action-fk": ("🔩️change-connection-action-fk", "✅apply"),
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
    return build_adapter(Subset("EN 1995", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
