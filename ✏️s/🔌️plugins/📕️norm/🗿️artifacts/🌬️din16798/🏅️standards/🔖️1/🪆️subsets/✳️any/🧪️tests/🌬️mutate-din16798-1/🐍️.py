"""🐍️ DIN EN 16798's contribution to the norm reference implementation — the four things that are
genuinely per-standard, and nothing else.

The second producer this case's differential comparison needs is `semio_norm_vocabulary`, the ONE
independent Python implementation of the norm mutation vocabulary, imported here rather than copied. Its
module docstring carries the survey that established no third-party library reads or writes `s.norm.*`
and the honest boundary on the `.dsl.semio` carrier. This file adds no verb, no addressing rule and no
carrier rule: everything below is DATA read off this subset's own committed catalog, its own committed
specification vectors and its own committed example document.
"""

from __future__ import annotations

# region 🔖️Imports
from importlib import import_module

_vocabulary = import_module("🐍️")
Subset = _vocabulary.Subset
build_adapter = _vocabulary.build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind of `Din16798Mutation`, in the committed catalog's (declaration) order.
KINDS = [
    "change-annex",
    "change-theta-rm",
    "change-outdoor-co2",
    "change-envelope-n50",
    "change-envelope-volume",
    "change-cellar-area",
    "change-cellar-ventilation",
    "change-night-setback",
    "insert-zone",
    "remove-zone",
    "change-zone-usage-type",
    "change-zone-floor-area",
    "change-zone-occupants",
    "change-zone-comfort-category",
    "change-zone-pollution-class",
    "change-zone-comfort-model",
    "change-zone-t-op-winter",
    "change-zone-t-op-summer",
    "change-zone-air-speed",
    "change-zone-clothing",
    "change-zone-metabolic-rate",
    "change-zone-rh",
    "change-zone-outdoor-air",
    "change-zone-co2",
    "change-zone-illuminance",
    "change-zone-noise",
    "change-zone-vent-system-id",
    "change-zone-turbulence",
    "change-zone-vent-method",
    "insert-vent-system",
    "remove-vent-system",
    "change-vent-system-type",
    "change-vent-sfp",
    "change-vent-sfp-class",
    "change-vent-heat-recovery",
    "change-vent-oda-class",
    "change-vent-filter-sup",
    "change-vent-inspection",
    "change-vent-duct-class",
    "change-vent-duct-leakage",
    "change-vent-design-airflow",
]

#: 🧫️ The committed specification vector each kind is measured on, as (leaf directory, scenario directory).
VECTORS = {
    "change-annex": ("🌍️change-annex", "🌍️change-annex"),
    "change-theta-rm": ("🔄️change-theta-rm", "🔄️change-theta-rm"),
    "change-outdoor-co2": ("🌫️change-outdoor-co2", "🌫️change-outdoor-co2"),
    "change-envelope-n50": ("🏠️change-envelope-n50", "🏠️change-envelope-n50"),
    "change-envelope-volume": ("📦️change-envelope-volume", "📦️change-envelope-volume"),
    "change-cellar-area": ("🏚️change-cellar-area", "🏚️change-cellar-area"),
    "change-cellar-ventilation": ("🌀change-cellar-ventilation", "🌀change-cellar-ventilation"),
    "change-night-setback": ("🌙️change-night-setback", "🌙️change-night-setback"),
    "insert-zone": ("➕️insert-zone", "➕️insert-zone"),
    "remove-zone": ("➖️remove-zone", "➖️remove-zone"),
    "change-zone-usage-type": ("🏢️change-zone-usage-type", "🏢️change-zone-usage-type"),
    "change-zone-floor-area": ("📐️change-zone-floor-area", "📐️change-zone-floor-area"),
    "change-zone-occupants": ("👥️change-zone-occupants", "👥️change-zone-occupants"),
    "change-zone-comfort-category": ("🛋️change-zone-comfort-category", "🛋️change-zone-comfort-category"),
    "change-zone-pollution-class": ("🏭️change-zone-pollution-class", "🏭️change-zone-pollution-class"),
    "change-zone-comfort-model": ("🧭️change-zone-comfort-model", "🧭️change-zone-comfort-model"),
    "change-zone-t-op-winter": ("❄️change-zone-t-op-winter", "❄️change-zone-t-op-winter"),
    "change-zone-t-op-summer": ("☀️change-zone-t-op-summer", "☀️change-zone-t-op-summer"),
    "change-zone-air-speed": ("💨change-zone-air-speed", "💨change-zone-air-speed"),
    "change-zone-clothing": ("👔change-zone-clothing", "👔change-zone-clothing"),
    "change-zone-metabolic-rate": ("🏃️change-zone-metabolic-rate", "🏃️change-zone-metabolic-rate"),
    "change-zone-rh": ("💧️change-zone-rh", "💧️change-zone-rh"),
    "change-zone-outdoor-air": ("💨️change-zone-outdoor-air", "💨️change-zone-outdoor-air"),
    "change-zone-co2": ("🫧change-zone-co2", "🫧change-zone-co2"),
    "change-zone-illuminance": ("💡change-zone-illuminance", "💡change-zone-illuminance"),
    "change-zone-noise": ("🔊️change-zone-noise", "🔊️change-zone-noise"),
    "change-zone-vent-system-id": ("🔗change-zone-vent-system-id", "🔗change-zone-vent-system-id"),
    "change-zone-turbulence": ("💨change-zone-turbulence", "💨change-zone-turbulence"),
    "change-zone-vent-method": ("📐️change-zone-vent-method", "📐️change-zone-vent-method"),
    "insert-vent-system": ("🆕️insert-vent-system", "🆕️insert-vent-system"),
    "remove-vent-system": ("🗑️remove-vent-system", "🗑️remove-vent-system"),
    "change-vent-system-type": ("⚙️change-vent-system-type", "⚙️change-vent-system-type"),
    "change-vent-sfp": ("🌀️change-vent-sfp", "🌀️change-vent-sfp"),
    "change-vent-sfp-class": ("🎓️change-vent-sfp-class", "🎓️change-vent-sfp-class"),
    "change-vent-heat-recovery": ("♻️change-vent-heat-recovery", "♻️change-vent-heat-recovery"),
    "change-vent-oda-class": ("🏞️change-vent-oda-class", "🏞️change-vent-oda-class"),
    "change-vent-filter-sup": ("🧽change-vent-filter-sup", "🧽change-vent-filter-sup"),
    "change-vent-inspection": ("📅️change-vent-inspection", "📅️change-vent-inspection"),
    "change-vent-duct-class": ("🧱change-vent-duct-class", "🧱change-vent-duct-class"),
    "change-vent-duct-leakage": ("🕳️change-vent-duct-leakage", "🕳️change-vent-duct-leakage"),
    "change-vent-design-airflow": ("🌬️change-vent-design-airflow", "🌬️change-vent-design-airflow"),
}

#: 🗣️ The real committed DIN EN 16798 document, read where the domain already keeps it.
DSL_ASSET = "asset://🎬️demo/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.din16798.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("DIN EN 16798", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
