"""🐍️ EN 1998's contribution to the norm reference implementation — the four things that are
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
    "change-seismic-zone",
    "change-ground-type",
    "change-importance-class",
    "change-structural-system",
    "change-t1-s",
    "change-mass-t",
    "change-v-rd-kn",
    "change-drift-mm",
    "change-height-m",
    "change-multiple-resisting-systems",
    "change-annex",
    "change-en-a-gr",
    "change-en-ground-type",
    "change-en-spectrum-type",
    "change-period-ratio",
    "change-bridge-v-rd-kn",
    "change-bearing-d-ed-mm",
    "change-bearing-d-rd-mm",
    "change-retrofit-knowledge-level",
    "change-retrofit-limit-state",
    "change-retrofit-ed-kn",
    "change-retrofit-rk-kn",
    "change-retrofit-gamma-el",
    "change-silo-height-m",
    "change-silo-radius-m",
    "change-silo-n-rd-kn",
    "change-silo-v-ed-kn",
    "change-silo-v-rd-kn",
    "change-silo-q-nominal",
    "change-tank-height-m",
    "change-tank-radius-m",
    "change-tank-mass-t",
    "change-tank-v-rd-kn",
    "change-tower-m-ed-knm",
    "change-tower-m-rd-knm",
    "change-tower-is-chimney",
    "change-tower-q-nominal",
    "change-tower-mass-t",
    "change-foundation-area-m2",
    "change-foundation-p-rd-kpa",
    "change-foundation-h-ed-kn",
    "change-foundation-h-rd-kn",
    "change-k-foundation",
    "change-k-soil",
    "change-wall-height-m",
    "change-wall-phi-deg",
    "change-wall-soil-gamma-kn-m3",
    "change-wall-r",
    "change-wall-h-rd-kn",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-seismic-zone": ("🫨️change-seismic-zone", "🫨️raises-seismic-zone-to-4"),
    "change-ground-type": ("🪨️change-ground-type", "🪨️switches-ground-type-to-c"),
    "change-importance-class": ("🏛️change-importance-class", "🏛️switches-importance-class-to-cc3"),
    "change-structural-system": ("🏗️change-structural-system", "🏗️switches-structural-system-to-wall-dcm"),
    "change-t1-s": ("🕐️change-t1-s", "🕐️raises-t1-s-to-0-75"),
    "change-mass-t": ("⚖️change-mass-t", "⚖️raises-mass-t-to-812-5"),
    "change-v-rd-kn": ("🛡️change-v-rd-kn", "🛡️raises-v-rd-kn-to-925-0"),
    "change-drift-mm": ("↔️change-drift-mm", "↔️raises-drift-mm-to-33-5"),
    "change-height-m": ("↕️change-height-m", "↕️raises-height-m-to-18-75"),
    "change-multiple-resisting-systems": ("🕸️change-multiple-resisting-systems", "🕸️turns-multiple-resisting-systems-off"),
    "change-annex": ("🌍️change-annex", "🌍️switches-annex-to-en"),
    "change-en-a-gr": ("🏎️change-en-a-gr", "🏎️raises-en-a-gr-to-0-25"),
    "change-en-ground-type": ("🗺️change-en-ground-type", "🗺️switches-en-ground-type-to-e"),
    "change-en-spectrum-type": ("🌈️change-en-spectrum-type", "🌈️switches-en-spectrum-type-to-type2"),
    "change-period-ratio": ("⏱️change-period-ratio", "⏱️raises-period-ratio-to-3-5"),
    "change-bridge-v-rd-kn": ("🌉️change-bridge-v-rd-kn", "🌉️raises-bridge-v-rd-kn-to-725-0"),
    "change-bearing-d-ed-mm": ("🎯️change-bearing-d-ed-mm", "🎯️raises-bearing-d-ed-mm-to-165-5"),
    "change-bearing-d-rd-mm": ("🛑️change-bearing-d-rd-mm", "🛑️raises-bearing-d-rd-mm-to-312-5"),
    "change-retrofit-knowledge-level": ("🎓️change-retrofit-knowledge-level", "🎓️switches-retrofit-knowledge-level-to-kl3"),
    "change-retrofit-limit-state": ("🚦️change-retrofit-limit-state", "🚦️switches-retrofit-limit-state-to-near-collapse"),
    "change-retrofit-ed-kn": ("📥️change-retrofit-ed-kn", "📥️raises-retrofit-e-d-kn-to-337-5"),
    "change-retrofit-rk-kn": ("💪️change-retrofit-rk-kn", "💪️raises-retrofit-r-k-kn-to-512-5"),
    "change-retrofit-gamma-el": ("✖️change-retrofit-gamma-el", "✖️raises-retrofit-gamma-el-to-1-25"),
    "change-silo-height-m": ("🌾️change-silo-height-m", "🌾️raises-silo-height-m-to-14-5"),
    "change-silo-radius-m": ("⭕️change-silo-radius-m", "⭕️raises-silo-radius-m-to-6-25"),
    "change-silo-n-rd-kn": ("🗜️change-silo-n-rd-kn", "🗜️raises-silo-n-rd-kn-to-640-0"),
    "change-silo-v-ed-kn": ("📉️change-silo-v-ed-kn", "📉️raises-silo-v-ed-kn-to-225-5"),
    "change-silo-v-rd-kn": ("🚧️change-silo-v-rd-kn", "🚧️raises-silo-v-rd-kn-to-412-5"),
    "change-silo-q-nominal": ("📊️change-silo-q-nominal", "📊️raises-silo-q-nominal-to-2-75"),
    "change-tank-height-m": ("🛢️change-tank-height-m", "🛢️raises-tank-height-m-to-11-5"),
    "change-tank-radius-m": ("🥁️change-tank-radius-m", "🥁️raises-tank-radius-m-to-5-75"),
    "change-tank-mass-t": ("⚓️change-tank-mass-t", "⚓️raises-tank-mass-t-to-425-0"),
    "change-tank-v-rd-kn": ("🔰️change-tank-v-rd-kn", "🔰️raises-tank-v-rd-kn-to-537-5"),
    "change-tower-m-ed-knm": ("↪️change-tower-m-ed-knm", "↪️raises-tower-m-ed-knm-to-1562-5"),
    "change-tower-m-rd-knm": ("🦾️change-tower-m-rd-knm", "🦾️raises-tower-m-rd-knm-to-2812-5"),
    "change-tower-is-chimney": ("🏭️change-tower-is-chimney", "🏭️turns-tower-is-chimney-off"),
    "change-tower-q-nominal": ("💨️change-tower-q-nominal", "💨️raises-tower-q-nominal-to-3-25"),
    "change-tower-mass-t": ("🗼️change-tower-mass-t", "🗼️raises-tower-mass-t-to-112-5"),
    "change-foundation-area-m2": ("🔲️change-foundation-area-m2", "🔲️raises-foundation-area-m2-to-144-0"),
    "change-foundation-p-rd-kpa": ("👇️change-foundation-p-rd-kpa", "👇️raises-foundation-p-rd-kpa-to-625-0"),
    "change-foundation-h-ed-kn": ("➡️change-foundation-h-ed-kn", "➡️raises-foundation-h-ed-kn-to-212-5"),
    "change-foundation-h-rd-kn": ("🧲️change-foundation-h-rd-kn", "🧲️raises-foundation-h-rd-kn-to-475-0"),
    "change-k-foundation": ("🌀️change-k-foundation", "🌀️raises-k-foundation-to-640000-0"),
    "change-k-soil": ("🌱️change-k-soil", "🌱️raises-k-soil-to-262500-0"),
    "change-wall-height-m": ("🧱️change-wall-height-m", "🧱️raises-wall-height-m-to-5-5"),
    "change-wall-phi-deg": ("📐️change-wall-phi-deg", "📐️raises-wall-phi-deg-to-37-5"),
    "change-wall-soil-gamma-kn-m3": ("🧂️change-wall-soil-gamma-kn-m3", "🧂️raises-wall-soil-gamma-kn-m3-to-20-5"),
    "change-wall-r": ("🔢️change-wall-r", "🔢️raises-wall-r-to-2-25"),
    "change-wall-h-rd-kn": ("🏋️change-wall-h-rd-kn", "🏋️raises-wall-h-rd-kn-to-187-5"),
}

#: 🗣️ The real committed EN 1998 document, read where the domain already keeps it.
DSL_ASSET = "asset://📚️examples/🏢️seismic-rc-frame/🖼️assets/🏢️seismic-rc-frame/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1998.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1998", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
