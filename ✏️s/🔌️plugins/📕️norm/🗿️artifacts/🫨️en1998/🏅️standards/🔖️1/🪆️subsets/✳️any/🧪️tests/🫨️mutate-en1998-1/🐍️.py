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
import json
from importlib import import_module
from pathlib import Path

_vocabulary = import_module("🐍️")
Subset = _vocabulary.Subset
build_adapter = _vocabulary.build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 📇️ This subset's committed `en1998-1-any` catalog, read where the subset keeps it.
CATALOG = next(entry for entry in json.loads((Path(__file__).resolve().parents[2] / "🔮️oracles" / "🔣️.json").read_text(encoding="utf-8"))["mutationCatalogs"] if entry["id"] == "en1998-1-any")

#: 🏷️ Every kind the catalog declares, in catalog order.
KINDS = CATALOG["kinds"]

#: 🧫️ The committed specification vectors: each kind's `apply` vector as (triad directory, fixture), and each further row
#: `<kind>-<slug>` as (kind, triad directory, fixture).
VECTORS = {
    row["id"] if row["id"] != "apply" else vector["mutationId"]: (vector["mutationDirectoryName"], row["directoryName"]) if row["id"] == "apply" else (vector["mutationId"], vector["mutationDirectoryName"], row["directoryName"])
    for vector in CATALOG["vectors"]
    for row in vector["scenarios"]
}

#: 🗣️ The real committed EN 1998 document, read where the domain already keeps it.
DSL_ASSET = "asset://🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio"

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
