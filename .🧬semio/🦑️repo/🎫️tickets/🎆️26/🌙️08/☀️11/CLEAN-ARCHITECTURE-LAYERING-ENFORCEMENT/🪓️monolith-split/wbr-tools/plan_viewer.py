"""viewer: lift the view command with its binary codec and the camera gesture bridge of the energy model viewer root into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from splitter import *

C = lambda **k: k

SPEC = dict(
    target="viewer",
    imports=(20, 57),
    root_drop=["FromValueDerive", "ToValueDerive"],
    skip_import=[],
    root_cfg={},
    children=[
        C(kind="cluster", mod="view_command", folder="🔖️view-command", ranges=[(74, 145)],
          extra_uses=["use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};"],
          doc=["//! 🔖️ The viewer's typed command channel: the one `setCamera` row and its binary op codec."]),
        C(kind="cluster", mod="camera_gesture", folder="🎥️camera-gesture", ranges=[(147, 192)],
          doc=["//! 🎥️ The camera gesture bridge: the dispatched action's args become the typed command, and the command becomes the addressed window-config write."]),
    ],
)


def anchors():
    ls = lines_of(read(snap_dir("viewer") + "/" + TARGETS["viewer"]["file"]))
    for ln, text in [(74, "//#region 🔖️Command"), (145, "//#endregion 🔖️Command"), (147, "//#region 🎥️Camera"), (192, "//#endregion 🎥️Camera")]:
        assert ls[ln - 1].startswith(text), (ln, ls[ln - 1])


def main():
    anchors()
    plan = Plan(SPEC).build()
    stage(plan)
    gone, added = multiset_report(plan)
    print("files:", {k.split("/")[-2] if k.endswith("🦀️.rs") and "/" in k else k: len(lines_of(v)) for k, v in plan.out.items()})
    print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:2500])
    print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:4500])
    for m, r in plan.report["children"].items():
        print(m, "widened", len(r["widened"]), r["widened"][:200])
        print("   imports", r["imports"])
        print("   super", r["super_imports"])
        print("   pub", r["reexport_pub"])
        print("   priv", r["reexport_priv"])


if __name__ == "__main__":
    main()
