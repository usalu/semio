"""grammar: lift the model, lexer, parser, writer, recognizer and walker clusters of 🗣️dsl/📖️grammar/🦀️.rs into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from splitter import *

C = lambda **k: k

SPEC = dict(
    target="grammar",
    imports=(16, 16),
    root_drop=[],
    skip_import=["Cursor"],
    root_cfg={},
    children=[
        C(kind="cluster", mod="model", folder="🧱️model", ranges=[(22, 273)],
          doc=["//! 🧱️ The `.grammar` / `.protocol` data model: dialects, productions, symbols, framing, blocks, primitives and the walk trace."]),
        C(kind="cluster", mod="lexer", folder="🔡️lexer", ranges=[(275, 459)],
          doc=["//! 🔡️ The grammar-format pre-scan lexer: `?` and `|` here, every other run delegated to the shared dsl lexer."]),
        C(kind="cluster", mod="parser", folder="🌳️parser", ranges=[(461, 1431)],
          doc=["//! 🌳️ Recursive-descent parsers for `.grammar.semio` and `.protocol.semio` sources over one shared token cursor."]),
        C(kind="cluster", mod="writer", folder="🖨️writer", ranges=[(1433, 1935)],
          doc=["//! 🖨️ Canonical printers for grammar and protocol files, and the idempotent `canonicalize`."]),
        C(kind="cluster", mod="recognizer", folder="🧭️recognizer", ranges=[(1937, 2223)],
          doc=["//! 🧭️ Token-stream `Recognizer` with explicit terminal predicates, family fragment merge and macro matchers."]),
        C(kind="cluster", mod="walker", folder="🧮️walker", ranges=[(2225, 2773)],
          doc=["//! 🧮️ The protocol byte walker: deep-walks bytes against a `ProtocolFile` with walk-wide state and exact mismatch offsets."]),
    ],
)


def main():
    plan = Plan(SPEC).build()
    stage(plan)
    gone, added = multiset_report(plan)
    print("files:", {k.split("/")[-2] if k.endswith("🦀️.rs") and "/" in k else k: len(lines_of(v)) for k, v in plan.out.items()})
    print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:1500])
    print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:3500])
    for m, r in plan.report["children"].items():
        print(m, "widened", len(r["widened"]), r["widened"][:80])
        print("   imports", r["imports"])
        print("   super", r["super_imports"])
        print("   pub", r["reexport_pub"])
        print("   priv", r["reexport_priv"])


if __name__ == "__main__":
    main()
