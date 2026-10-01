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

_vocabulary = import_module("🐍️")
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
    "insert-material",
    "remove-material",
    "insert-section",
    "remove-section",
    "insert-member",
    "remove-member",
    "insert-load-case",
    "remove-load-case",
    "insert-member-action",
    "remove-member-action",
    "insert-joint",
    "remove-joint",
    "insert-fatigue-detail",
    "remove-fatigue-detail",
    "insert-fire-exposure",
    "remove-fire-exposure",
    "insert-cold-formed-member",
    "remove-cold-formed-member",
    "insert-plated-panel",
    "remove-plated-panel",
    "insert-silo-shell",
    "remove-silo-shell",
    "insert-tension-component",
    "remove-tension-component",
    "insert-bridge-fatigue",
    "remove-bridge-fatigue",
    "insert-tower-leg",
    "remove-tower-leg",
    "insert-pile",
    "remove-pile",
    "insert-crane-runway",
    "remove-crane-runway",
]

#: 🧫️ The committed specification vectors: each kind's `✅apply` vector as (triad directory, fixture), and each refusal row
#: `<kind>-<slug>` as (kind, triad directory, fixture).
VECTORS = {
    "change-annex": ("🌍️change-annex", "✅apply"),
    "update-member-properties": ("📊️update-member-properties", "✅apply"),
    "update-fire-inputs": ("🔥️update-fire-inputs", "✅apply"),
    "update-cold-formed-inputs": ("🥶️update-cold-formed-inputs", "✅apply"),
    "update-stainless-inputs": ("✨️update-stainless-inputs", "✅apply"),
    "update-plated-inputs": ("🧱️update-plated-inputs", "✅apply"),
    "update-silo-shell-inputs": ("🛢️update-silo-shell-inputs", "✅apply"),
    "update-bolt-inputs": ("🔩️update-bolt-inputs", "✅apply"),
    "update-weld-inputs": ("🧲️update-weld-inputs", "✅apply"),
    "update-fatigue-inputs": ("🔁️update-fatigue-inputs", "✅apply"),
    "update-through-thickness-inputs": ("↕️update-through-thickness-inputs", "✅apply"),
    "update-tension-component-inputs": ("🪢️update-tension-component-inputs", "✅apply"),
    "update-hss-inputs": ("⬜️update-hss-inputs", "✅apply"),
    "update-bridge-inputs": ("🌉️update-bridge-inputs", "✅apply"),
    "update-tower-inputs": ("🗼️update-tower-inputs", "✅apply"),
    "update-pile-inputs": ("🪵️update-pile-inputs", "✅apply"),
    "update-crane-inputs": ("🏗️update-crane-inputs", "✅apply"),
    "insert-material": ("➕️insert-material", "✅apply"),
    "insert-material-dupe": ("insert-material", "➕️insert-material", "⛔dupe"),
    "remove-material": ("➖️remove-material", "✅apply"),
    "insert-section": ("➕️insert-section", "✅apply"),
    "insert-section-dupe": ("insert-section", "➕️insert-section", "⛔dupe"),
    "remove-section": ("➖️remove-section", "✅apply"),
    "insert-member": ("➕️insert-member", "✅apply"),
    "insert-member-dupe": ("insert-member", "➕️insert-member", "⛔dupe"),
    "remove-member": ("➖️remove-member", "✅apply"),
    "insert-load-case": ("➕️insert-load-case", "✅apply"),
    "insert-load-case-dupe": ("insert-load-case", "➕️insert-load-case", "⛔dupe"),
    "remove-load-case": ("➖️remove-load-case", "✅apply"),
    "insert-member-action": ("➕️insert-member-action", "✅apply"),
    "insert-member-action-dupe": ("insert-member-action", "➕️insert-member-action", "⛔dupe"),
    "remove-member-action": ("➖️remove-member-action", "✅apply"),
    "insert-joint": ("➕️insert-joint", "✅apply"),
    "insert-joint-dupe": ("insert-joint", "➕️insert-joint", "⛔dupe"),
    "remove-joint": ("➖️remove-joint", "✅apply"),
    "insert-fatigue-detail": ("➕️insert-fatigue-detail", "✅apply"),
    "insert-fatigue-detail-dupe": ("insert-fatigue-detail", "➕️insert-fatigue-detail", "⛔dupe"),
    "remove-fatigue-detail": ("➖️remove-fatigue-detail", "✅apply"),
    "insert-fire-exposure": ("➕️insert-fire-exposure", "✅apply"),
    "insert-fire-exposure-dupe": ("insert-fire-exposure", "➕️insert-fire-exposure", "⛔dupe"),
    "remove-fire-exposure": ("➖️remove-fire-exposure", "✅apply"),
    "insert-cold-formed-member": ("➕️insert-cold-formed-member", "✅apply"),
    "insert-cold-formed-member-dupe": ("insert-cold-formed-member", "➕️insert-cold-formed-member", "⛔dupe"),
    "remove-cold-formed-member": ("➖️remove-cold-formed-member", "✅apply"),
    "insert-plated-panel": ("➕️insert-plated-panel", "✅apply"),
    "insert-plated-panel-dupe": ("insert-plated-panel", "➕️insert-plated-panel", "⛔dupe"),
    "remove-plated-panel": ("➖️remove-plated-panel", "✅apply"),
    "insert-silo-shell": ("➕️insert-silo-shell", "✅apply"),
    "insert-silo-shell-dupe": ("insert-silo-shell", "➕️insert-silo-shell", "⛔dupe"),
    "remove-silo-shell": ("➖️remove-silo-shell", "✅apply"),
    "insert-tension-component": ("➕️insert-tension-component", "✅apply"),
    "insert-tension-component-dupe": ("insert-tension-component", "➕️insert-tension-component", "⛔dupe"),
    "remove-tension-component": ("➖️remove-tension-component", "✅apply"),
    "insert-bridge-fatigue": ("➕️insert-bridge-fatigue", "✅apply"),
    "insert-bridge-fatigue-dupe": ("insert-bridge-fatigue", "➕️insert-bridge-fatigue", "⛔dupe"),
    "remove-bridge-fatigue": ("➖️remove-bridge-fatigue", "✅apply"),
    "insert-tower-leg": ("➕️insert-tower-leg", "✅apply"),
    "insert-tower-leg-dupe": ("insert-tower-leg", "➕️insert-tower-leg", "⛔dupe"),
    "remove-tower-leg": ("➖️remove-tower-leg", "✅apply"),
    "insert-pile": ("➕️insert-pile", "✅apply"),
    "insert-pile-dupe": ("insert-pile", "➕️insert-pile", "⛔dupe"),
    "remove-pile": ("➖️remove-pile", "✅apply"),
    "insert-crane-runway": ("➕️insert-crane-runway", "✅apply"),
    "insert-crane-runway-dupe": ("insert-crane-runway", "➕️insert-crane-runway", "⛔dupe"),
    "remove-crane-runway": ("➖️remove-crane-runway", "✅apply"),
}

#: 🗣️ The real committed EN 1993 document, read where the domain already keeps it.
DSL_ASSET = "asset://🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio"

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
