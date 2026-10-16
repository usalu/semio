"""session: lift the helpers, errors, tessellation types, media export, generic invoke and session core of the spatial-kernel session into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from plan_common import *

SPEC = dict(
    target="session",
    glob_super=True,
    gate_members=True,
    imports=(2, 12),
    root_drop=[],
    skip_import=[],
    root_cfg={},
    children=[
        C(kind="cluster", mod="helpers", folder="🧰️helpers", ranges=[(24, 500)],
          doc=["//! 🧰️ Operator helpers: dictionaries, channel specs, schemas, registration and the `BrepDeconstruct` operator."]),
        C(kind="cluster", mod="errors", folder="⚠️errors", ranges=[(501, 552)],
          doc=["//! ⚠️ The brep module error type of the media import and export bridges."]),
        C(kind="cluster", mod="tessellation", folder="🧊️tessellation", ranges=[(553, 634), (644, 695), (697, 702)],
          doc=["//! 🧊️ Tessellation: the retained job registry, mesh cache and authority claims, and the failed-envelope codec."]),
        C(kind="cluster", mod="media_export", folder="📤️media-export", ranges=[(696, 696), (703, 731)],
          doc=["//! 📤️ Media export: GLB export and import bridged through tessellation."]),
        C(kind="cluster", mod="generic_invoke", folder="🌉️generic-invoke", ranges=[(732, 828), (837, 838)],
          doc=["//! 🌉️ Generic invoke: the `brep_invoke` argument readers and result writers."]),
        C(kind="cluster", mod="session_core", folder="🧠️session-core", ranges=[(829, 836), (839, 1605)],
          doc=["//! 🧠️ Session core: the shell allocator, `Session`, its capture and retirement state, and the geometry port."]),
    ],
)

if __name__ == "__main__":
    run(SPEC)
