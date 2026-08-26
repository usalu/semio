"""🐍️ VDI 3805's contribution to the norm reference implementation — the four things that are
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
from semio_norm_vocabulary import Subset, build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind this subset's committed catalog declares, in catalog order.
KINDS = [
    "update-manufacturer-file",
    "change-correction-as-of",
    "change-strict-mode",
    "update-limits",
    "change-edition-profile",
    "remove-edition-profile",
    "create-product",
    "delete-product",
    "rename-product",
    "replace-product-configuration",
    "create-geometry",
    "delete-geometry",
    "resize-geometry",
    "add-geometry-connection",
    "remove-geometry-connection",
    "replace-geometry-parameters",
    "create-curve",
    "delete-curve",
    "replace-curve-points",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "update-manufacturer-file": ("🏕️update-manufacturer-file", "renames-the-header-manufacturer-to-acme"),
    "change-correction-as-of": ("🏜️change-correction-as-of", "advances-the-correction-cut-off-to-2025-03"),
    "change-strict-mode": ("🦋change-strict-mode", "turns-strict-mode-on"),
    "update-limits": ("🦈update-limits", "tightens-every-untrusted-input-limit"),
    "change-edition-profile": ("🐝change-edition-profile", "switches-sheet-8-from-legacy-to-current"),
    "remove-edition-profile": ("⛰️remove-edition-profile", "clears-the-sheet-8-legacy-override"),
    "create-product": ("🪵create-product", "appends-vlv-80-002-and-its-index-entry"),
    "delete-product": ("🐳delete-product", "removes-vlv-50-001-and-its-index-entry"),
    "rename-product": ("🏖️rename-product", "retitles-vlv-50-001-and-resyncs-its-index-tags"),
    "replace-product-configuration": ("🗻replace-product-configuration", "reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn"),
    "create-geometry": ("🦭create-geometry", "adds-the-geom-valve-80-definition"),
    "delete-geometry": ("🐬delete-geometry", "removes-the-geom-valve-50-definition"),
    "resize-geometry": ("🏟️resize-geometry", "doubles-the-geom-valve-50-bounding-box"),
    "add-geometry-connection": ("🐞add-geometry-connection", "attaches-the-drain-connection-to-geom-valve-50"),
    "remove-geometry-connection": ("🏔️remove-geometry-connection", "detaches-the-out-connection-from-geom-valve-50"),
    "replace-geometry-parameters": ("🐌replace-geometry-parameters", "rescales-geom-valve-50-to-half-and-adds-clearance"),
    "create-curve": ("🏝️create-curve", "adds-the-curve-dp-pressure-drop-curve"),
    "delete-curve": ("🐢delete-curve", "removes-the-curve-kvs-flow-curve"),
    "replace-curve-points": ("🏞️replace-curve-points", "resamples-curve-kvs-onto-three-points"),
}

#: 🗣️ The real committed VDI 3805 document, read where the domain already keeps it.
DSL_ASSET = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.vdi3805.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("VDI 3805", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
