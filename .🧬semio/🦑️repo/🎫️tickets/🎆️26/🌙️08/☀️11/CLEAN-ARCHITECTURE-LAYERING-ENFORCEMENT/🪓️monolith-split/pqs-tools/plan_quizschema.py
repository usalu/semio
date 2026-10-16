"""quizschema: lift the document, sheet and result, lifecycle, view and presence regions of 🧬️schema/🦀️.rs into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from splitter import *

C = lambda **k: k

SPEC = dict(
    target="quizschema",
    imports=(14, 15),
    root_drop=["Deserialize"],
    skip_import=[],
    root_cfg={"Serialize": ['#[cfg(any(test, feature = "testkit"))]'], "BTreeMap": ["#[cfg(test)]"]},
    children=[
        C(kind="cluster", extra_uses=["use serde::{Deserialize, Serialize};"], mod="documents", folder="📚️quiz-documents", ranges=[(17, 389)],
          doc=["//! 📚️ The authored quiz documents: scalars and limits, quizzes with their tasks, and the catalog with its badges."]),
        C(kind="cluster", extra_uses=["use serde::{Deserialize, Serialize};"], mod="sheets_and_results", folder="🃏️sheets-and-results", ranges=[(391, 681)],
          doc=["//! 🃏️ What a run is made of: the sheet drawn for a learner, the answers given to it and the results scored from them."]),
        C(kind="cluster", extra_uses=["use serde::{Deserialize, Serialize};"], mod="commands_and_events", folder="🔁️commands-and-events", ranges=[(683, 878)],
          doc=["//! 🔁️ The learner lifecycle on the wire: identities and limits, commands, rejections and events."]),
        C(kind="cluster", extra_uses=["use serde::{Deserialize, Serialize};"], mod="read_views", super_drop=["Catalog", "Category", "Profile"], folder="👁️read-views", ranges=[(880, 1265)],
          doc=["//! 👁️ The read-model views and queries: catalog, run and learner views, hints, leaderboards, handles and the crowd."]),
        C(kind="cluster", extra_uses=["use serde::{Deserialize, Serialize};"], mod="presence_wire", super_drop=["Introduction", "Leaderboard", "Quiz"], folder="👥️presence-wire", ranges=[(1267, 1369)],
          doc=["//! 👥️ The presence wire: screens, places, cursors, drags and the thinking states shared live."]),
    ],
)


def main():
    plan = Plan(SPEC).build()
    stage(plan)
    gone, added = multiset_report(plan)
    print("files:", {k.split("/")[-2] if k.endswith("🦀️.rs") and "/" in k else k: len(lines_of(v)) for k, v in plan.out.items()})
    print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:3000])
    print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:6000])
    for m, r in plan.report["children"].items():
        print(m, "widened", len(r["widened"]), r["widened"][:120])
        print("   imports", r["imports"])
        print("   super", r["super_imports"])
        print("   pub", r["reexport_pub"])
        print("   priv", r["reexport_priv"])


if __name__ == "__main__":
    main()
