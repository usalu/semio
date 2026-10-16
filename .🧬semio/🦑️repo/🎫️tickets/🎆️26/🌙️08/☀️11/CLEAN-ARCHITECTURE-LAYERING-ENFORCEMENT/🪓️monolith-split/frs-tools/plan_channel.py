"""channel: lift wire entries, archive load, presence, paged command decode, AppCommand, AppFrame and the binary codec of 📡️spr/🧵️channel/🦀️.rs into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from splitter import *

C = lambda **k: k

SPEC = dict(
    target="channel",
    imports=(17, 17),
    root_drop=["COMMAND_MAXIMUM_BYTES", "COMMAND_MAXIMUM_PAGES", "PagedCommandReader"],
    skip_import=[],
    root_cfg={n: ["#[cfg(test)]"] for n in ["CommandPageSet", "FixedCommandPage", "COMMAND_PAGE_MAXIMUM_BYTES", "INVOCATION_RESULT_PACK_MAXIMUM_BYTES", "PagedCommand"]},
    children=[
        C(kind="cluster", mod="wire_entries", folder="🪪️wire-entries", ranges=[(48, 59), (61, 99), (101, 133), (187, 195)],
          doc=["//! 🪪️ Wire entry records: the document identity, media-export handle and state, child pack entries and window-config pack entry."]),
        C(kind="cluster", mod="archive_load", folder="🚚️archive-load", ranges=[(143, 184), (1948, 2110)],
          doc=["//! 🚚️ Document-archive load: the load state and status wire types, and the `DocumentArchiveLoadHost` step machine with its outcomes."]),
        C(kind="cluster", mod="presence", folder="👥️presence-roster", ranges=[(199, 353)],
          doc=["//! 👥️ Presence: the bounded roster wire and the paged presence-command cursor."]),
        C(kind="cluster", mod="command_decode", folder="🧩️command-decode", ranges=[(355, 1458)],
          doc=["//! 🧩️ Paged decoders for document archives, route fields, media-export handles and `AppCommand`s over bounded command pages."]),
        C(kind="cluster", mod="app_command", folder="📨️app-command", ranges=[(1460, 1711)],
          doc=["//! 📨️ `AppCommand`: every UI interaction forwarded to the headless engine."]),
        C(kind="cluster", mod="app_frame", folder="🎞️app-frame", ranges=[(1713, 1946)],
          doc=["//! 🎬️ `AppFrame`: every engine reaction returned to the client."]),
        C(kind="cluster", mod="codec", folder="🗜️codec", ranges=[(2112, 3311)],
          doc=["//! 🔣️ Binary codec: option and vec combinators, pack readers and writers, and the `AppCommand` and `AppFrame` tag dispatch."]),
    ],
)


def main():
    plan = Plan(SPEC).build()
    stage(plan)
    gone, added = multiset_report(plan)
    print("files:", {k.split("/")[-2] if k.endswith("🦀️.rs") and "/" in k else k: len(lines_of(v)) for k, v in plan.out.items()})
    print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:2500])
    print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:4500])
    for m, r in plan.report["children"].items():
        print(m, "widened", len(r["widened"]), r["widened"][:120])
        print("   imports", r["imports"])
        print("   super", r["super_imports"])
        print("   pub", r["reexport_pub"])
        print("   priv", r["reexport_priv"])


if __name__ == "__main__":
    main()
