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

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🌍️change-annex", "🌐️switches-the-national"),
    "update-member-properties": ("📊️update-member-properties", "🏋️re-grades"),
    "update-fire-inputs": ("🔥️update-fire-inputs", "🧯️raises-the-fire"),
    "update-cold-formed-inputs": ("🥶️update-cold-formed-inputs", "✏️sets"),
    "update-stainless-inputs": ("✨️update-stainless-inputs", "✏️sets-inputs"),
    "update-plated-inputs": ("🧱️update-plated-inputs", "📈️makes-plate"),
    "update-silo-shell-inputs": ("🛢️update-silo-shell-inputs", "✏️sets"),
    "update-bolt-inputs": ("🔩️update-bolt-inputs", "✏️sets-inputs"),
    "update-weld-inputs": ("🧲️update-weld-inputs", "✏️sets-inputs"),
    "update-fatigue-inputs": ("🔁️update-fatigue-inputs", "🔁️drops-detail"),
    "update-through-thickness-inputs": ("↕️update-through-thickness-inputs", "✏️sets"),
    "update-tension-component-inputs": ("🪢️update-tension-component-inputs", "✏️new"),
    "update-hss-inputs": ("⬜️update-hss-inputs", "✏️sets-inputs"),
    "update-bridge-inputs": ("🌉️update-bridge-inputs", "🌉️raises-bridge"),
    "update-tower-inputs": ("🗼️update-tower-inputs", "✏️sets-inputs"),
    "update-pile-inputs": ("🪵️update-pile-inputs", "✏️sets-inputs"),
    "update-crane-inputs": ("🏗️update-crane-inputs", "🏋️widens-crane"),
    "insert-material": ("➕️insert-material", "➕️inserts-material"),
    "remove-material": ("➖️remove-material", "➖️removes-material"),
    "insert-section": ("➕️insert-section", "➕️inserts-section"),
    "remove-section": ("➖️remove-section", "➖️removes-section"),
    "insert-member": ("➕️insert-member", "➕️inserts-member"),
    "remove-member": ("➖️remove-member", "➖️removes-member"),
    "insert-load-case": ("➕️insert-load-case", "➕️inserts-case"),
    "remove-load-case": ("➖️remove-load-case", "➖️removes-case"),
    "insert-member-action": ("➕️insert-member-action", "➕️inserts-action"),
    "remove-member-action": ("➖️remove-member-action", "➖️removes-action"),
    "insert-joint": ("➕️insert-joint", "➕️inserts-joint"),
    "remove-joint": ("➖️remove-joint", "➖️removes-joint"),
    "insert-fatigue-detail": ("➕️insert-fatigue-detail", "➕️inserts-detail"),
    "remove-fatigue-detail": ("➖️remove-fatigue-detail", "➖️removes-detail"),
    "insert-fire-exposure": ("➕️insert-fire-exposure", "➕️inserts"),
    "remove-fire-exposure": ("➖️remove-fire-exposure", "➖️removes"),
    "insert-cold-formed-member": ("➕️insert-cold-formed-member", "➕️inserts"),
    "remove-cold-formed-member": ("➖️remove-cold-formed-member", "➖️removes"),
    "insert-plated-panel": ("➕️insert-plated-panel", "➕️inserts-panel"),
    "remove-plated-panel": ("➖️remove-plated-panel", "➖️removes-panel"),
    "insert-silo-shell": ("➕️insert-silo-shell", "➕️inserts-shell"),
    "remove-silo-shell": ("➖️remove-silo-shell", "➖️removes-shell"),
    "insert-tension-component": ("➕️insert-tension-component", "➕️inserts"),
    "remove-tension-component": ("➖️remove-tension-component", "➖️removes"),
    "insert-bridge-fatigue": ("➕️insert-bridge-fatigue", "➕️inserts"),
    "remove-bridge-fatigue": ("➖️remove-bridge-fatigue", "➖️removes"),
    "insert-tower-leg": ("➕️insert-tower-leg", "➕️inserts-leg"),
    "remove-tower-leg": ("➖️remove-tower-leg", "➖️removes-leg"),
    "insert-pile": ("➕️insert-pile", "➕️inserts-pile"),
    "remove-pile": ("➖️remove-pile", "➖️removes-pile"),
    "insert-crane-runway": ("➕️insert-crane-runway", "➕️inserts-runway"),
    "remove-crane-runway": ("➖️remove-crane-runway", "➖️removes-runway"),
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
