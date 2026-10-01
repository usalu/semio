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
import json
from importlib import import_module
from pathlib import Path

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

#: 🧫️ The committed specification vector each kind is measured on, as (leaf directory, scenario directory), and each
#: further refusal, no-op or clamp vector, keyed by its `<kind>-<slug>` row as (kind, leaf directory, scenario directory).
VECTORS = {
    "change-building-category": ("🏠️change-building-category", "✅apply"),
    "change-attachment": ("🧱change-attachment", "✅apply"),
    "change-use-class": ("🏷️change-use-class", "✅apply"),
    "change-method": ("🧮change-method", "✅apply"),
    "change-net-floor-area-m2": ("📐️change-net-floor-area-m2", "✅apply"),
    "change-heated-volume-m3": ("📦change-heated-volume-m3", "✅apply"),
    "change-geg-qp-factor": ("⚖️change-geg-qp-factor", "✅apply"),
    "change-delta-u-wb": ("🌉change-delta-u-wb", "✅apply"),
    "change-automation-class": ("🎛️change-automation-class", "✅apply"),
    "specify-heating-system": ("🔥specify-heating-system", "✅apply"),
    "specify-dhw-system": ("🚿specify-dhw-system", "✅apply"),
    "update-ventilation": ("🌬️update-ventilation", "✅apply"),
    "update-cooling": ("❄️update-cooling", "✅apply"),
    "update-lighting": ("💡update-lighting", "✅apply"),
    "update-renewables": ("☀️update-renewables", "✅apply"),
    "replace-zones": ("🗺️replace-zones", "✅apply"),
    "replace-elements": ("🧩replace-elements", "✅apply"),
    "change-element-u": ("🌡️change-element-u", "✅apply"),
    "update-climate": ("🌦️update-climate", "🚫rule"),
    "change-net-floor-area-m2-rule": ("change-net-floor-area-m2", "📐️change-net-floor-area-m2", "🚫rule"),
    "change-heated-volume-m3-rule": ("change-heated-volume-m3", "📦change-heated-volume-m3", "🚫rule"),
    "change-geg-qp-factor-rule": ("change-geg-qp-factor", "⚖️change-geg-qp-factor", "🚫rule"),
    "change-delta-u-wb-rule": ("change-delta-u-wb", "🌉change-delta-u-wb", "🚫rule"),
}

#: 📐️ Each kind's committed leaf payload schema, read where the subset keeps it; its stated bounds are the payload's.
SCHEMAS = {kind: json.loads((Path(__file__).resolve().parents[2] / "🧬️schema" / "🧬️mutations" / VECTORS[kind][0] / "🧬️schema" / "🔣️.json").read_text(encoding="utf-8")) for kind in KINDS}

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
    return build_adapter(Subset("DIN V 18599", KINDS, VECTORS, DSL_ASSET, ENVELOPE, schemas=SCHEMAS))
# endregion 🔖️Registration
