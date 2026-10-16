"""events: lift the kinds, payloads, emit, sha256, store, export and server-client regions of 📡️events into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from plan_common import *

SPEC = dict(
    target="events",
    gate_members=True,
    no_super_import=["serde_json"],
    imports=(13, 19),
    root_drop=[],
    skip_import=[],
    root_force=["Write"],
    root_cfg={n: ["#[cfg(test)]"] for n in ["Write", "fs", "Path", "PathBuf", "Deserialize"]},
    reexport_cfg={n: ["#[cfg(test)]"] for n in ["STAGE_SCHEMA", "Stage"]},
    children=[
        C(kind="cluster", mod="event_kinds", folder="📋️event-kinds", ranges=[(26, 216)],
          doc=["//! 📋️ Event kinds and the envelope: the kind constants and catalog, the event record and its builders."]),
        C(kind="cluster", mod="payloads", folder="🌨️payloads", ranges=[(217, 576)],
          doc=["//! 🌨️ Payloads: one typed payload record per event kind."]),
        C(kind="cluster", mod="emit", folder="📤️emit", ranges=[(577, 679)],
          doc=["//! 📤️ Emit: the coordinator emit path that turns an interaction into an event."]),
        C(kind="cluster", mod="sha256", folder="🔏️sha256", ranges=[(680, 827)],
          doc=["//! 🔏️ SHA-256: the hand-rolled digest the store and the server client use."]),
        C(kind="cluster", mod="store", folder="🗄️store", ranges=[(828, 1302)],
          doc=["//! 🗄️ The append-only event store."]),
        C(kind="cluster", mod="export", folder="📦️export", ranges=[(1303, 1383)],
          doc=["//! 📦️ Export: the entity and event export records."]),
        C(kind="cluster", mod="server_client", folder="🔐️server-client", ranges=[(1384, 1457)],
          doc=["//! 🔐️ Server client: the hand-rolled HTTP/1.1 request to the coordinator."]),
    ],
)

if __name__ == "__main__":
    run(SPEC)
