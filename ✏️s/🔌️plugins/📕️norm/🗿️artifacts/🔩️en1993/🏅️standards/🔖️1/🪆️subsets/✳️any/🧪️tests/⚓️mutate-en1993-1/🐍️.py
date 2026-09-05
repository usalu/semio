"""🐍️ EN 1993's contribution to the norm reference implementation — the four things that are
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
    "change-annex",
    "update-member-properties",
    "update-fire-inputs",
    "update-cold-formed-inputs",
    "update-stainless-inputs",
    "update-plated-inputs",
    "update-silo-shell-inputs",
    "update-bolt-inputs",
    "update-weld-inputs",
    "update-fatigue-inputs",
    "update-through-thickness-inputs",
    "update-tension-component-inputs",
    "update-hss-inputs",
    "update-bridge-inputs",
    "update-tower-inputs",
    "update-pile-inputs",
    "update-crane-inputs",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🌍️change-annex", "switches-the-national-annex-from-de-to-en"),
    "update-member-properties": ("📊️update-member-properties", "re-grades-the-base-member-to-s460-under-a-heavier-load"),
    "update-fire-inputs": ("🔥️update-fire-inputs", "raises-the-fire-protection-to-r90"),
    "update-cold-formed-inputs": ("🥶️update-cold-formed-inputs", "thickens-the-cold-formed-flange-and-reverses-its-stress-gradient"),
    "update-stainless-inputs": ("✨️update-stainless-inputs", "upsizes-the-stainless-section-to-a-duplex-grade"),
    "update-plated-inputs": ("🧱️update-plated-inputs", "makes-the-plate-panel-more-slender-and-more-stressed"),
    "update-silo-shell-inputs": ("🛢️update-silo-shell-inputs", "deepens-the-silo-and-thickens-its-shell"),
    "update-bolt-inputs": ("🔩️update-bolt-inputs", "moves-the-connection-to-four-m24-grade-10-9-bolts"),
    "update-weld-inputs": ("🧲️update-weld-inputs", "lengthens-the-fillet-weld-and-re-grades-it-to-s460"),
    "update-fatigue-inputs": ("🔁️update-fatigue-inputs", "drops-to-detail-category-56-under-a-safe-life-assessment"),
    "update-through-thickness-inputs": ("↕️update-through-thickness-inputs", "upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c"),
    "update-tension-component-inputs": ("🪢️update-tension-component-inputs", "derates-the-tension-rod-to-a-400-kn-characteristic-strength"),
    "update-hss-inputs": ("⬜️update-hss-inputs", "reclassifies-the-hollow-section-to-class-3-in-s355"),
    "update-bridge-inputs": ("🌉️update-bridge-inputs", "raises-the-bridge-damage-equivalence-and-dynamic-factors"),
    "update-tower-inputs": ("🗼️update-tower-inputs", "raises-the-tower-wind-factor-and-leg-force"),
    "update-pile-inputs": ("🪵️update-pile-inputs", "derates-the-driven-pile-for-hard-driving"),
    "update-crane-inputs": ("🏗️update-crane-inputs", "widens-the-crane-wheel-contact-patch-under-a-heavier-wheel"),
}

#: 🗣️ The real committed EN 1993 document, read where the domain already keeps it.
DSL_ASSET = "asset://📚️examples/📕️high-strength-connection/🖼️assets/🧪️high-strength-connection/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1993.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1993", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
