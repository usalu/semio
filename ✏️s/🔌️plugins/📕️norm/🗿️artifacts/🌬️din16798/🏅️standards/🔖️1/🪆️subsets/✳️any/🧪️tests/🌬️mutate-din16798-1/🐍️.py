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
import json
from importlib import import_module
from pathlib import Path

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

#: 🧫️ The committed specification vector each kind is measured on, as (leaf directory, scenario directory), and each
#: further refusal, no-op or clamp vector, keyed by its `<kind>-<slug>` row as (kind, leaf directory, scenario directory).
VECTORS = {
    "change-annex": ("🌍️change-annex", "✅apply"),
    "change-theta-rm": ("🔄️change-theta-rm", "✅apply"),
    "change-outdoor-co2": ("🌫️change-outdoor-co2", "✅apply"),
    "change-envelope-n50": ("🏠️change-envelope-n50", "✅apply"),
    "change-envelope-volume": ("📦️change-envelope-volume", "✅apply"),
    "change-cellar-area": ("🏚️change-cellar-area", "✅apply"),
    "change-cellar-ventilation": ("🌀change-cellar-ventilation", "✅apply"),
    "change-night-setback": ("🌙️change-night-setback", "✅apply"),
    "insert-zone": ("➕️insert-zone", "✅apply"),
    "remove-zone": ("➖️remove-zone", "✅apply"),
    "remove-zone-middle-row": ("remove-zone", "➖️remove-zone", "🔬️middle-row"),
    "change-zone-usage-type": ("🏢️change-zone-usage-type", "✅apply"),
    "change-zone-floor-area": ("📐️change-zone-floor-area", "✅apply"),
    "change-zone-occupants": ("👥️change-zone-occupants", "✅apply"),
    "change-zone-comfort-category": ("🛋️change-zone-comfort-category", "✅apply"),
    "change-zone-pollution-class": ("🏭️change-zone-pollution-class", "✅apply"),
    "change-zone-comfort-model": ("🧭️change-zone-comfort-model", "✅apply"),
    "change-zone-t-op-winter": ("❄️change-zone-t-op-winter", "✅apply"),
    "change-zone-t-op-summer": ("☀️change-zone-t-op-summer", "✅apply"),
    "change-zone-air-speed": ("💨change-zone-air-speed", "✅apply"),
    "change-zone-clothing": ("👔change-zone-clothing", "✅apply"),
    "change-zone-metabolic-rate": ("🏃️change-zone-metabolic-rate", "✅apply"),
    "change-zone-rh": ("💧️change-zone-rh", "✅apply"),
    "change-zone-outdoor-air": ("💨️change-zone-outdoor-air", "✅apply"),
    "change-zone-co2": ("🫧change-zone-co2", "✅apply"),
    "change-zone-illuminance": ("💡change-zone-illuminance", "✅apply"),
    "change-zone-noise": ("🔊️change-zone-noise", "✅apply"),
    "change-zone-vent-system-id": ("🔗change-zone-vent-system-id", "✅apply"),
    "change-zone-turbulence": ("💨change-zone-turbulence", "✅apply"),
    "change-zone-vent-method": ("📐️change-zone-vent-method", "✅apply"),
    "insert-vent-system": ("🆕️insert-vent-system", "✅apply"),
    "remove-vent-system": ("🗑️remove-vent-system", "✅apply"),
    "remove-vent-system-middle-row": ("remove-vent-system", "🗑️remove-vent-system", "🔬️middle-row"),
    "change-vent-system-type": ("⚙️change-vent-system-type", "✅apply"),
    "change-vent-sfp": ("🌀️change-vent-sfp", "✅apply"),
    "change-vent-sfp-class": ("🎓️change-vent-sfp-class", "✅apply"),
    "change-vent-heat-recovery": ("♻️change-vent-heat-recovery", "✅apply"),
    "change-vent-oda-class": ("🏞️change-vent-oda-class", "✅apply"),
    "change-vent-filter-sup": ("🧽change-vent-filter-sup", "✅apply"),
    "change-vent-inspection": ("📅️change-vent-inspection", "✅apply"),
    "change-vent-duct-class": ("🧱change-vent-duct-class", "✅apply"),
    "change-vent-duct-leakage": ("🕳️change-vent-duct-leakage", "✅apply"),
    "change-vent-design-airflow": ("🌬️change-vent-design-airflow", "✅apply"),
    "insert-zone-dupe": ("insert-zone", "➕️insert-zone", "⛔dupe"),
    "insert-zone-clamp": ("insert-zone", "➕️insert-zone", "📏clamp"),
    "remove-zone-gone": ("remove-zone", "➖️remove-zone", "❓gone"),
    "insert-vent-system-dupe": ("insert-vent-system", "🆕️insert-vent-system", "⛔dupe"),
    "insert-vent-system-clamp": ("insert-vent-system", "🆕️insert-vent-system", "📏clamp"),
    "remove-vent-system-gone": ("remove-vent-system", "🗑️remove-vent-system", "❓gone"),
}

#: 📐️ Each kind's committed leaf payload schema, read where the subset keeps it; its stated bounds are the payload's.
SCHEMAS = {kind: json.loads((Path(__file__).resolve().parents[2] / "🧬️schema" / "🧬️mutations" / VECTORS[kind][0] / "🧬️schema" / "🔣️.json").read_text(encoding="utf-8")) for kind in KINDS}

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
    return build_adapter(Subset("DIN EN 16798", KINDS, VECTORS, DSL_ASSET, ENVELOPE, schemas=SCHEMAS))
# endregion 🔖️Registration
