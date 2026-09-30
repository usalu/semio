"""🐍️ DIN V 18599's contribution to the norm reference implementation — the four things that are
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
#: 🏷️ Every kind of `Din18599Mutation`, in the committed catalog's (declaration) order.
KINDS = [
    "change-building-category",
    "change-attachment",
    "change-use-class",
    "change-method",
    "change-net-floor-area-m2",
    "change-heated-volume-m3",
    "change-geg-qp-factor",
    "change-delta-u-wb",
    "change-automation-class",
    "specify-heating-system",
    "specify-dhw-system",
    "update-ventilation",
    "update-cooling",
    "update-lighting",
    "update-renewables",
    "replace-zones",
    "replace-elements",
    "change-element-u",
    "update-climate",
]

#: 🧫️ The committed specification vector each kind is measured on, as (leaf directory, scenario directory).
VECTORS = {
    "change-building-category": ("🏠️change-building-category", "🎯️applies-building-category"),
    "change-attachment": ("🧱change-attachment", "🎯️applies-attachment"),
    "change-use-class": ("🏷️change-use-class", "🏢️reclassifies-the-building-as-an-office"),
    "change-method": ("🧮change-method", "🎯️applies-method"),
    "change-net-floor-area-m2": ("📐️change-net-floor-area-m2", "📏️extends-net-floor-area-to-160-m2"),
    "change-heated-volume-m3": ("📦change-heated-volume-m3", "🎯️applies-heated-volume-m3"),
    "change-geg-qp-factor": ("⚖️change-geg-qp-factor", "🎯️applies-geg-qp-factor"),
    "change-delta-u-wb": ("🌉change-delta-u-wb", "🎯️applies-delta-u-wb"),
    "change-automation-class": ("🎛️change-automation-class", "🎯️applies-automation-class"),
    "specify-heating-system": ("🔥specify-heating-system", "🎯️applies-specify-heating-system"),
    "specify-dhw-system": ("🚿specify-dhw-system", "🎯️applies-specify-dhw-system"),
    "update-ventilation": ("🌬️update-ventilation", "🎯️applies-update-ventilation"),
    "update-cooling": ("❄️update-cooling", "🎯️applies-update-cooling"),
    "update-lighting": ("💡update-lighting", "🎯️applies-update-lighting"),
    "update-renewables": ("☀️update-renewables", "🎯️applies-update-renewables"),
    "replace-zones": ("🗺️replace-zones", "🎯️applies-replace-zones"),
    "replace-elements": ("🧩replace-elements", "🎯️applies-replace-elements"),
    "change-element-u": ("🌡️change-element-u", "🎯️applies-change-element-u"),
    "update-climate": ("🌦️update-climate", "🌧️refuses-a-negative-january-irradiance"),
}

#: 🗣️ The real committed DIN V 18599 document, read where the domain already keeps it.
DSL_ASSET = "asset://🎬️demo/🗣️.dsl.semio"

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
