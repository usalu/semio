"""🐍️ DIN 4108's contribution to the norm reference implementation — the four things that are
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
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-climate-zone": ('🌦️change-climate-zone', '🗺️moves-to-zone-3'),
    "change-usage": ('🗂️change-usage', '🏢️sets-nonresidential'),
    "change-t-int-c": ('🌡️change-t-int-c', '🌡️sets-t-int-to-21-point-5'),
    "change-rh-int": ('💧️change-rh-int', '💧️raises-rh-to-0-point-55'),
    "change-airtightness-n50": ('💨️change-airtightness-n50', '💨️tightens-n50-to-1-point-0'),
    "change-has-mechanical-ventilation": ('🌬️change-has-mechanical-ventilation', '🌬️disables-mechanical-ventilation'),
    "change-bb2-details-conform": ('✅️change-bb2-details-conform', '❌️declares-bb2-non-conforming'),
    "insert-zone": ('➕️insert-zone', '➕️appends-extra-zone'),
    "remove-zone": ('➖️remove-zone', '🚫️removes-first-zone'),
    "change-zone-floor-area": ('📐️change-zone-floor-area', '📐️sets-floor-area-to-90'),
    "change-zone-heaviness": ('🧱change-zone-heaviness', '🧱sets-heaviness-light'),
    "change-zone-night-ventilation": ('🌙change-zone-night-ventilation', '🌙sets-night-ventilation-high'),
    "insert-zone-window": ('🪟insert-zone-window', '🪟appends-extra-window'),
    "remove-zone-window": ('🚫️remove-zone-window', '🚫️removes-east-window'),
    "change-zone-window-area": ('📏change-zone-window-area', '📏grows-south-window'),
    "change-zone-window-g-value": ('☀️change-zone-window-g-value', '☀️sets-g-value-0-point-6'),
    "change-zone-window-shading-fc": ('⛱️change-zone-window-shading-fc', '⛱️tightens-shading-fc'),
    "insert-element": ('🏠️insert-element', '🏠️appends-extra-wall'),
    "remove-element": ('🚫️remove-element', '🚫️removes-first-element'),
    "change-element-area": ('📐️change-element-area', '📐️grows-wall-area'),
    "change-element-adjacent": ('↔️change-element-adjacent', '↔️sets-adjacent-unheated'),
    "change-element-kind": ('🏷️change-element-kind', '🏷️retags-as-wall'),
    "insert-layer": ('➕️insert-layer', '➕️inserts-layer-into-wall'),
    "remove-layer": ('➖️remove-layer', '➖️removes-eps-layer'),
    "reorder-layers": ('🔀️reorder-layers', '🧭️swaps-first-two-layers'),
    "change-layer-thickness": ('📏️change-layer-thickness', '📏️thickens-eps-to-0-point-2'),
    "change-layer-lambda": ('🌡change-layer-lambda', '🌡sets-eps-lambda'),
    "change-layer-mu": ('💧change-layer-mu', '💧raises-eps-mu'),
    "change-layer-material-id": ('🧽️change-layer-material-id', '🧽️retags-eps-material'),
    "insert-thermal-bridge": ('🌉️insert-thermal-bridge', '🌉️appends-extra-bridge'),
    "remove-thermal-bridge": ('🧊remove-thermal-bridge', '🧊removes-first-bridge'),
    "change-thermal-bridge-psi": ('🔘change-thermal-bridge-psi', '🔘lowers-psi'),
    "change-thermal-bridge-length": ('↔️change-thermal-bridge-length', '↔️shortens-bridge'),
}

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
    return build_adapter(Subset("DIN 4108", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
