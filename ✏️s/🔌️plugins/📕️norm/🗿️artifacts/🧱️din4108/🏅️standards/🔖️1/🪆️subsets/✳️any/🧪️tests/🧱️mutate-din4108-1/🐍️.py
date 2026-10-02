"""🐍️ DIN 4108's contribution to the norm reference implementation — the four things that are
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
#: 🏷️ Every kind of `Din4108Mutation`, in the committed catalog's (declaration) order.
KINDS = [
    "change-climate-zone",
    "change-usage",
    "change-t-int-c",
    "change-rh-int",
    "change-airtightness-n50",
    "change-has-mechanical-ventilation",
    "change-bb2-details-conform",
    "insert-zone",
    "remove-zone",
    "change-zone-floor-area",
    "change-zone-heaviness",
    "change-zone-night-ventilation",
    "insert-zone-window",
    "remove-zone-window",
    "change-zone-window-area",
    "change-zone-window-g-value",
    "change-zone-window-shading-fc",
    "insert-element",
    "remove-element",
    "change-element-area",
    "change-element-adjacent",
    "change-element-kind",
    "insert-layer",
    "remove-layer",
    "reorder-layers",
    "change-layer-thickness",
    "change-layer-lambda",
    "change-layer-mu",
    "change-layer-material-id",
    "insert-thermal-bridge",
    "remove-thermal-bridge",
    "change-thermal-bridge-psi",
    "change-thermal-bridge-length",
    "change-element-orientation-deg",
    "change-element-inclination-deg",
    "change-element-delta-ug",
    "change-element-delta-uf",
    "change-element-delta-ur",
    "change-thermal-bridge-bb2-type",
    "change-zone-window-orientation",
    "change-zone-window-inclination-deg",
    "change-layer-application-type",
    "change-layer-compressive-class",
]

#: 🧫️ The committed specification vector each kind is measured on, as (leaf directory, scenario directory), and each
#: further refusal, no-op or clamp vector, keyed by its `<kind>-<slug>` row as (kind, leaf directory, scenario directory).
VECTORS = {
    "change-climate-zone": ("🌦️change-climate-zone", "✅apply"),
    "change-usage": ("🗂️change-usage", "✅apply"),
    "change-t-int-c": ("🌡️change-t-int-c", "✅apply"),
    "change-rh-int": ("💧️change-rh-int", "✅apply"),
    "change-airtightness-n50": ("💨️change-airtightness-n50", "✅apply"),
    "change-has-mechanical-ventilation": ("💨change-has-mechanical-ventilation", "✅apply"),
    "change-bb2-details-conform": ("✅️change-bb2-details-conform", "✅apply"),
    "insert-zone": ("➕️insert-zone", "✅apply"),
    "remove-zone": ("➖️remove-zone", "✅apply"),
    "change-zone-floor-area": ("📐️change-zone-floor-area", "✅apply"),
    "change-zone-heaviness": ("🧱change-zone-heaviness", "✅apply"),
    "change-zone-night-ventilation": ("🌙change-zone-night-ventilation", "✅apply"),
    "insert-zone-window": ("🪟insert-zone-window", "✅apply"),
    "remove-zone-window": ("🚫️remove-zone-window", "✅apply"),
    "change-zone-window-area": ("📏change-zone-window-area", "✅apply"),
    "change-zone-window-g-value": ("☀️change-zone-window-g-value", "✅apply"),
    "change-zone-window-shading-fc": ("⛱️change-zone-window-shading-fc", "✅apply"),
    "insert-element": ("🏠️insert-element", "✅apply"),
    "remove-element": ("🚫️remove-element", "✅apply"),
    "change-element-area": ("📐️change-element-area", "✅apply"),
    "change-element-adjacent": ("↔️change-element-adjacent", "✅apply"),
    "change-element-kind": ("🏷️change-element-kind", "✅apply"),
    "insert-layer": ("➕️insert-layer", "✅apply"),
    "remove-layer": ("➖️remove-layer", "✅apply"),
    "reorder-layers": ("🔀️reorder-layers", "✅apply"),
    "change-layer-thickness": ("📏️change-layer-thickness", "✅apply"),
    "change-layer-lambda": ("🌡change-layer-lambda", "✅apply"),
    "change-layer-mu": ("💧change-layer-mu", "✅apply"),
    "change-layer-material-id": ("🧽️change-layer-material-id", "✅apply"),
    "insert-thermal-bridge": ("🌉️insert-thermal-bridge", "✅apply"),
    "remove-thermal-bridge": ("🧊remove-thermal-bridge", "✅apply"),
    "change-thermal-bridge-psi": ("🔘change-thermal-bridge-psi", "✅apply"),
    "change-thermal-bridge-length": ("↔️change-thermal-bridge-length", "✅apply"),
    "change-element-orientation-deg": ("🧭change-element-orientation-deg", "✅apply"),
    "change-element-inclination-deg": ("📐change-element-inclination-deg", "✅apply"),
    "change-element-delta-ug": ("📈️change-element-delta-ug", "✅apply"),
    "change-element-delta-uf": ("📈️change-element-delta-uf", "✅apply"),
    "change-element-delta-ur": ("📈️change-element-delta-ur", "✅apply"),
    "change-thermal-bridge-bb2-type": ("🏷change-thermal-bridge-bb2-type", "✅apply"),
    "change-zone-window-orientation": ("🧭change-zone-window-orientation", "✅apply"),
    "change-zone-window-inclination-deg": ("📐change-zone-window-inclination-deg", "✅apply"),
    "change-layer-application-type": ("🏷️change-layer-application-type", "✅apply"),
    "change-layer-compressive-class": ("🏷️change-layer-compressive-class", "✅apply"),
    "change-climate-zone-noop": ("change-climate-zone", "🌦️change-climate-zone", "🟰noop"),
    "change-usage-noop": ("change-usage", "🗂️change-usage", "🟰noop"),
    "change-t-int-c-rule": ("change-t-int-c", "🌡️change-t-int-c", "🚫rule"),
    "change-t-int-c-noop": ("change-t-int-c", "🌡️change-t-int-c", "🟰noop"),
    "change-rh-int-rule": ("change-rh-int", "💧️change-rh-int", "🚫rule"),
    "change-rh-int-noop": ("change-rh-int", "💧️change-rh-int", "🟰noop"),
    "change-airtightness-n50-rule": ("change-airtightness-n50", "💨️change-airtightness-n50", "🚫rule"),
    "change-airtightness-n50-noop": ("change-airtightness-n50", "💨️change-airtightness-n50", "🟰noop"),
    "change-has-mechanical-ventilation-noop": ("change-has-mechanical-ventilation", "💨change-has-mechanical-ventilation", "🟰noop"),
    "change-bb2-details-conform-noop": ("change-bb2-details-conform", "✅️change-bb2-details-conform", "🟰noop"),
    "insert-zone-dupe": ("insert-zone", "➕️insert-zone", "⛔dupe"),
    "insert-zone-clamp": ("insert-zone", "➕️insert-zone", "📏clamp"),
    "insert-element-dupe": ("insert-element", "🏠️insert-element", "⛔dupe"),
    "insert-element-clamp": ("insert-element", "🏠️insert-element", "📏clamp"),
    "insert-thermal-bridge-dupe": ("insert-thermal-bridge", "🌉️insert-thermal-bridge", "⛔dupe"),
    "insert-thermal-bridge-clamp": ("insert-thermal-bridge", "🌉️insert-thermal-bridge", "📏clamp"),
}

#: 📐️ Each kind's committed leaf payload schema, read where the subset keeps it; its stated bounds are the payload's.
SCHEMAS = {kind: json.loads((Path(__file__).resolve().parents[2] / "🧬️schema" / "🧬️mutations" / VECTORS[kind][0] / "🧬️schema" / "🔣️.json").read_text(encoding="utf-8")) for kind in KINDS}

#: 🗣️ The real committed DIN 4108 document, read where the domain already keeps it.
DSL_ASSET = "asset://🎬️demo/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.din4108.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("DIN 4108", KINDS, VECTORS, DSL_ASSET, ENVELOPE, schemas=SCHEMAS))
# endregion 🔖️Registration
