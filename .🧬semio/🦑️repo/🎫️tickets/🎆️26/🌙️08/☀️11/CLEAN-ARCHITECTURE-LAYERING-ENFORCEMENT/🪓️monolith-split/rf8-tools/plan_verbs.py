"""verbs: lift the verb handlers of 🚦️verbs into one child per aggregate family."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from plan_common import *

SPEC = dict(
    target="verbs",
    gate_members=True,
    imports=(3, 12),
    root_drop=[],
    skip_import=[],
    root_cfg={},
    children=[
        C(kind="cluster", mod="verb_support", folder="🧰️verb-support", ranges=[(18, 124)],
          doc=["//! 🧰️ Verb support: the operand, flag and vocabulary readers every verb handler shares."]),
        C(kind="cluster", mod="gateway_verbs", folder="🔌️gateway-verbs", ranges=[(125, 187), (905, 967)],
          doc=["//! 🔌️ Gateway verbs: `mcp`, `graphql`, `test` and `hook`, the doors to the other front ends."]),
        C(kind="cluster", mod="ticket_verbs", folder="🎫️ticket-verbs", ranges=[(188, 395)],
          doc=["//! 🎫️ Ticket verbs: open, close, reopen, change, purge, list, show and files."]),
        C(kind="cluster", mod="planning_verbs", folder="🎯️planning-verbs", ranges=[(396, 574)],
          doc=["//! 🎯️ Planning verbs: todos, goals and contributors."]),
        C(kind="cluster", mod="artifact_verbs", folder="📁️artifact-verbs", ranges=[(575, 790)],
          doc=["//! 📁️ Artifact verbs: folders, files, sections, move, integrate, extract, rename and sync."]),
        C(kind="cluster", mod="query_verbs", folder="🔎️query-verbs", ranges=[(791, 904)],
          doc=["//! 🔎️ Query verbs: search, list, query, the trees and export."]),
        C(kind="cluster", mod="code_metric_verbs", folder="📊️code-metric-verbs", ranges=[(968, 1146), (1215, 1253)],
          doc=["//! 📊️ Code metric verbs: the mermaid charts, `loc`, technologies, bundles and `benchmark`."]),
        C(kind="cluster", mod="statute_verbs", folder="⚖️statute-verbs", ranges=[(1147, 1171), (1347, 1385)],
          doc=["//! ⚖️ Statute verbs: analyze, autofix and the statute catalog listings."]),
        C(kind="cluster", mod="system_verbs", folder="🧩️system-verbs", ranges=[(1172, 1214), (1254, 1346)],
          doc=["//! 🧩️ System verbs: entity emojis, configure, micro commit, update and the auth status verbs."]),
        C(kind="cluster", mod="history_verbs", folder="📜️history-verbs", ranges=[(1386, 1478)],
          doc=["//! 📜️ History verbs: checkpoints, interactions, drafts and definitions."]),
    ],
)

if __name__ == "__main__":
    run(SPEC)
