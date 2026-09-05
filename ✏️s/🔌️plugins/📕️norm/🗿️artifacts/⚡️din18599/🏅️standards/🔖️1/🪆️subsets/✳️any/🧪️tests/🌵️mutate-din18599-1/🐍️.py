"""🐍️ DIN V 18599's contribution to the norm reference implementation — the four things that are
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
    "change-use-class",
    "change-heated-area-m2",
    "change-occupants",
    "change-ht",
    "change-hv",
    "change-internal-gains-wm2",
    "change-solar-gains-kwh",
    "change-system-losses-kwh",
    "change-renewable-kwh",
    "change-annual-limit-kwh",
    "change-energy-carrier",
    "change-reference-qp-kwh",
    "update-climate",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-use-class": ("🏷️change-use-class", "🏢️reclassifies-the-building-as-an-office"),
    "change-heated-area-m2": ("📐️change-heated-area-m2", "📏️extends-the-heated-area-to-160-m2"),
    "change-occupants": ("👥️change-occupants", "👥️raises-the-occupancy-to-six-people"),
    "change-ht": ("🧱️change-ht", "🧱️raises-the-transmission-loss-coefficient-to-118-w-per-k"),
    "change-hv": ("🌬️change-hv", "🌬️raises-the-ventilation-loss-coefficient-to-52-25-w-per-k"),
    "change-internal-gains-wm2": ("🔥️change-internal-gains-wm2", "🌡️raises-the-internal-gains-to-5-w-per-m2"),
    "change-solar-gains-kwh": ("☀️change-solar-gains-kwh", "🌞️raises-the-annual-solar-gains-to-132-kwh"),
    "change-system-losses-kwh": ("📉️change-system-losses-kwh", "🛠️cuts-the-system-losses-to-450-kwh"),
    "change-renewable-kwh": ("♻️change-renewable-kwh", "🔆️raises-the-on-site-renewable-yield-to-2250-kwh"),
    "change-annual-limit-kwh": ("🚦️change-annual-limit-kwh", "🎯️tightens-the-annual-primary-energy-limit-to-6000-kwh"),
    "change-energy-carrier": ("🔋️change-energy-carrier", "⚡️switches-the-energy-carrier-to-an-electric-heat-pump"),
    "change-reference-qp-kwh": ("🏢️change-reference-qp-kwh", "📉️lowers-the-reference-building-primary-energy-to-8750-kwh"),
    "update-climate": ("🌦️update-climate", "🌧️refuses-a-negative-january-irradiance"),
}

#: 🗣️ The real committed DIN V 18599 document, read where the domain already keeps it.
DSL_ASSET = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.din18599.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("DIN V 18599", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
