"""🐍️ EN 1991's contribution to the norm reference implementation — the four things that are
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

_vocabulary = import_module("🗣️vocabulary")
Subset = _vocabulary.Subset
build_adapter = _vocabulary.build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind this subset's committed catalog declares, in catalog order.
KINDS = [
    "change-area-m2",
    "change-category",
    "change-annex",
    "change-self-weight-material",
    "change-self-weight-thickness-m",
    "change-assumed-gk-kn-m2",
    "change-fire-curve",
    "change-fire-resistance-min",
    "change-fire-member-capacity-c",
    "change-snow-zone",
    "change-snow-altitude-m",
    "change-en-sk-kn-m2",
    "change-wind-zone",
    "change-en-vbms",
    "change-delta-tk",
    "change-construction-activity",
    "change-accidental-mass-t",
    "change-accidental-speed-km-h",
    "change-bridge-lane",
    "change-bridge-span-m",
    "change-bridge-lane-width-m",
    "change-bridge-moment-resistance-knm",
    "change-crane-class",
    "change-hoist-class",
    "change-hoisting-speed-ms",
    "change-silo-bulk-density-kn-m3",
    "change-silo-height-m",
    "change-silo-hydraulic-radius-m",
    "change-silo-mu",
    "change-silo-k",
    "change-cs",
    "change-cd",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-area-m2": ("📐️change-area-m2", "📐️enlarges-loaded-area-to-360-m2"),
    "change-category": ("🗂️change-category", "🗂️reclassifies-imposed-load-to-category-d"),
    "change-annex": ("🌍️change-annex", "🌍️switches-national-annex-to-en"),
    "change-self-weight-material": ("🧱️change-self-weight-material", "🧱️switches-self-weight-material-to-structural-steel"),
    "change-self-weight-thickness-m": ("📏️change-self-weight-thickness-m", "📏️thickens-self-weight-layer-to-0-375-m"),
    "change-assumed-gk-kn-m2": ("⚖️change-assumed-gk-kn-m2", "⚖️raises-assumed-gk-to-7-5-kn-m2"),
    "change-fire-curve": ("🔥️change-fire-curve", "🔥️switches-fire-curve-to-hydrocarbon"),
    "change-fire-resistance-min": ("⏱️change-fire-resistance-min", "⏱️extends-fire-resistance-to-120-min"),
    "change-fire-member-capacity-c": ("🛡️change-fire-member-capacity-c", "🛡️raises-fire-member-capacity-to-700-c"),
    "change-snow-zone": ("🗺️change-snow-zone", "🗺️moves-site-to-snow-zone-3"),
    "change-snow-altitude-m": ("🏔️change-snow-altitude-m", "🏔️lifts-snow-altitude-to-780-m"),
    "change-en-sk-kn-m2": ("❄️change-en-sk-kn-m2", "❄️raises-en-characteristic-snow-load-to-1-25-kn-m2"),
    "change-wind-zone": ("🪁️change-wind-zone", "🪁️moves-site-to-wind-zone-4"),
    "change-en-vbms": ("🌬️change-en-vbms", "🌬️raises-en-basic-wind-speed-to-30-m-s"),
    "change-delta-tk": ("🌡️change-delta-tk", "🌡️raises-thermal-delta-tk-to-45-k"),
    "change-construction-activity": ("🚧️change-construction-activity", "🚧️switches-construction-activity-to-concreting"),
    "change-accidental-mass-t": ("🚚️change-accidental-mass-t", "🚚️lightens-impact-vehicle-to-12-5-t"),
    "change-accidental-speed-km-h": ("🚗️change-accidental-speed-km-h", "🚗️lowers-impact-speed-to-50-km-h"),
    "change-bridge-lane": ("🛣️change-bridge-lane", "🛣️widens-carriageway-to-3-notional-lanes"),
    "change-bridge-span-m": ("🌉️change-bridge-span-m", "🌉️lengthens-bridge-span-to-36-m"),
    "change-bridge-lane-width-m": ("↔️change-bridge-lane-width-m", "↔️widens-notional-lane-to-3-5-m"),
    "change-bridge-moment-resistance-knm": ("💪️change-bridge-moment-resistance-knm", "💪️raises-bridge-moment-resistance-to-4500-knm"),
    "change-crane-class": ("🏗️change-crane-class", "🏗️upgrades-crane-to-class-hc3"),
    "change-hoist-class": ("🏷️change-hoist-class", "🏷️upgrades-hoist-to-class-hc4"),
    "change-hoisting-speed-ms": ("🪝️change-hoisting-speed-ms", "🪝️speeds-hoisting-to-1-25-m-s"),
    "change-silo-bulk-density-kn-m3": ("🌾️change-silo-bulk-density-kn-m3", "🌾️raises-silo-bulk-density-to-10-5-kn-m3"),
    "change-silo-height-m": ("🗼️change-silo-height-m", "🗼️raises-silo-to-18-m"),
    "change-silo-hydraulic-radius-m": ("⭕️change-silo-hydraulic-radius-m", "⭕️widens-silo-hydraulic-radius-to-2-25-m"),
    "change-silo-mu": ("🧲️change-silo-mu", "🧲️raises-silo-wall-friction-mu-to-0-625"),
    "change-silo-k": ("⚙️change-silo-k", "⚙️raises-silo-lateral-pressure-ratio-k-to-0-625"),
    "change-cs": ("🔎️change-cs", "🔎️raises-size-factor-cs-to-1-125"),
    "change-cd": ("🌀️change-cd", "🌀️lowers-dynamic-factor-cd-to-0-875"),
}

#: 🗣️ The real committed EN 1991 document, read where the domain already keeps it.
DSL_ASSET = "asset://📚️examples/🔥️retail-hydrocarbon-fire/🖼️assets/🔥️retail-hydrocarbon-fire/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1991.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1991", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
