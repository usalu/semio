"""catalog: lift the record types, phase cursors, symbol table and catalog cursor of 🗂️catalog/🦀️.rs into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from splitter import *

C = lambda **k: k

SPEC = dict(
    target="catalog",
    imports=(4, 17),
    root_drop=["size_of", "PagedListAllocationError", "PagedListError", "PagedListRefusalKind", "RawManifest", "Superblock", "RetirementDemand", "ValueError", "ValueRefusalKind", "retained_page_close_demands"],
    root_keep=["PackSink"],
    skip_import=[],
    root_cfg={n: ["#[cfg(test)]"] for n in ["ByteRange", "PackLimits", "RetainedPackCatalogCause", "RetainedPackCloseStep", "RetainedPackSegmentEvent", "RetainedPackSegmentHeader", "RetainedVarintCursor", "RetainedVarintStep"]},
    children=[
        C(kind="cluster", mod="catalog_records", folder="📋️catalog-records", ranges=[(26, 99)],
          doc=["//! 📋️ The public catalog records: chunk entries, symbol spans, the manifest, allocation steps, progress and events."]),
        C(kind="cluster", mod="phase_cursors", folder="🔤️phase-cursors", ranges=[(101, 368)],
          doc=["//! 🔤️ Byte-driven phase cursors: the manifest varint cursor, the UTF-8 scalar cursor, the symbol-table cursor and the chunk-table cursor."]),
        C(kind="cluster", mod="symbol_table", folder="🗃️symbol-table", ranges=[(370, 730)],
          doc=["//! 🗃️ The retained symbol table: paged symbol spans and scalars with checked allocation, handback and controlled close."]),
        C(kind="cluster", mod="catalog_cursor", folder="🧭️catalog-cursor", ranges=[(732, 1349)],
          doc=["//! 🧭️ The retained catalog cursor: segment events in, manifest, symbols and chunk table out, bounded and cancellable."]),
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
