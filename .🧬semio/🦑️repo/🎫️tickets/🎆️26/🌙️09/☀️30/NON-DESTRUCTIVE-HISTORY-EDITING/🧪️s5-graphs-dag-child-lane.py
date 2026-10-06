"""🚦️ S5-GRAPHS-WIRES — wave `dag-child-lane` (fault of the every-editor acceptance law, `📓️s5-every-editor-faults.md` dag row).

`addNode` faulted at publication in the product and in the law: `plugin.internal` "typed-operation emitted a store lane
absent from its exact factory publication contract". Cause: since the §20.15 conversion every graph verb of the dag editor
publishes leaves of the composed `content` child (`dag_child_emit`, `Emit::node_drag_child`) and the parent vocabulary is
uninhabited, but the nine verbs' publication contracts still declared the `Artifact` lane (and `nodeGraphEdit` a `Config`
lane it never writes). The typed-operation route checks every emitted lane against the contract before it publishes.

The wave declares `Child` for the nine graph verbs and adds the dag-local law that pins it; the law that EXECUTES
`addNode` through the typed-operation route is the existing `composed_child_history_law!("dag", …, addNode)`.

    python3 🧪️s5-graphs-dag-child-lane.py --check | --apply | --restore

Fails closed: two explicit files, every anchor exactly once, `--restore` refuses a file that is no longer what `--apply` wrote.
"""

import pathlib
import sys

TICKET = pathlib.Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
EDITOR = ROOT / "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
ROOT_FILE = EDITOR / "🦀️.rs"
LAWS = EDITOR / "🧪️tests/🔬️unit/🦀️.rs"
BACKUP = TICKET / "🗑️generated/s5-graphs-wires/dag-child-lane/before"
NAMES = {ROOT_FILE: "editor.rs", LAWS: "laws.rs"}
GRAPH_VERBS = ["addNode", "removeNode", "deleteSelection", "nodeGraphEdit", "connectMediaPorts", "disconnect", "moveMediaNode", "renameDagNode", "patchDagNodes"]


def contract(tool_id: str, lanes: str) -> str:
    return "    ArtifactToolPublicationContract { tool_id: \"%s\", lanes: &[%s] },\n" % (tool_id, lanes)


ARTIFACT, CONFIG, CHILD = ("ArtifactToolPublicationLane::" + lane for lane in ("Artifact", "Config", "Child"))
EDITS = {
    ROOT_FILE: [
        (
            "/// 🚦️ The config verb's lane in front of the document verbs' lanes, in `DAG_RETAINED_TOOL_IDS` order.\n",
            "/// 🚦️ The config verb's lane in front of the document verbs' lanes, in `DAG_RETAINED_TOOL_IDS` order. Every verb that\n"
            "/// edits the graph publishes leaves of the composed `content` child (design §20.15: the parent vocabulary is\n"
            "/// uninhabited), so its lane is `Child`; an `Artifact` lane could never carry anything.\n",
        )
    ]
    + [(contract(verb, ARTIFACT + ", " + CONFIG if verb == "nodeGraphEdit" else ARTIFACT), contract(verb, CHILD)) for verb in GRAPH_VERBS],
    LAWS: [
        (
            """            assert_eq!(action.semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "{} is retained but not Migrated", action.id);
        }
    }
}
""",
            """            assert_eq!(action.semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "{} is retained but not Migrated", action.id);
        }
    }
}

/// ⚖️ LAW (design §20.15): the dag parent vocabulary is uninhabited, so no retained verb declares the `Artifact` lane, and
/// every verb that edits the graph declares exactly the `Child` lane its `content` leaves are published on — the lane the
/// typed-operation route holds an emission against before it publishes (`addNode` faulted there while it said `Artifact`).
#[test]
fn every_graph_verb_publishes_on_the_child_lane_and_none_on_the_artifact_lane() {
    use semio_framework_plugin::ArtifactToolPublicationLane as Lane;
    for contract in DAG_RETAINED_PUBLICATION_CONTRACTS {
        assert!(!contract.lanes.contains(&Lane::Artifact), "{} declares the Artifact lane of an uninhabited vocabulary", contract.tool_id);
    }
    for tool_id in [%s] {
        let contract = DAG_RETAINED_PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == tool_id).expect("a graph verb has a publication contract");
        assert_eq!(contract.lanes, &[Lane::Child], "{tool_id} publishes its content leaves on the Child lane");
    }
}
"""
            % ", ".join('"%s"' % verb for verb in GRAPH_VERBS),
        )
    ],
}


def convert(path: pathlib.Path, source: str, refusals: list) -> str:
    text = source
    for before, after in EDITS[path]:
        if text.count(before) != 1:
            refusals.append("%s: anchor %r occurs %d times, expected 1" % (NAMES[path], before.strip()[:70], text.count(before)))
            continue
        text = text.replace(before, after)
    return text


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) == 2 else ""
    if mode not in ("--check", "--apply", "--restore"):
        raise SystemExit("usage: 🧪️s5-graphs-dag-child-lane.py --check | --apply | --restore")
    if not EDITOR.is_dir() or any(not path.is_file() for path in EDITS):
        raise SystemExit("[dag-child-lane] REFUSED: the dag editor tree or one of its two files is missing")
    refusals = []
    if mode == "--restore":
        saved = {path: BACKUP / NAMES[path] for path in EDITS}
        if any(not backup.is_file() for backup in saved.values()):
            raise SystemExit("[dag-child-lane] REFUSED: no backup")
        moved = [NAMES[path] for path in EDITS if path.read_text() != convert(path, saved[path].read_text(), refusals)]
        if moved or refusals:
            raise SystemExit("[dag-child-lane] REFUSED: %s changed since --apply; restore by hand from %s" % (", ".join(moved or refusals), BACKUP))
        for path, backup in saved.items():
            path.write_text(backup.read_text())
        print("[dag-child-lane] restored 2 files")
        return
    converted = {path: convert(path, path.read_text(), refusals) for path in EDITS}
    if refusals or ARTIFACT in converted[ROOT_FILE]:
        raise SystemExit("[dag-child-lane] REFUSED, nothing written:\n  " + "\n  ".join(refusals or ["an Artifact lane survives in the editor root"]))
    if mode == "--apply":
        BACKUP.mkdir(parents=True, exist_ok=True)
        for path in EDITS:
            (BACKUP / NAMES[path]).write_text(path.read_text())
        for path, text in converted.items():
            path.write_text(text)
    print("[dag-child-lane] %s: %d edits in 2 files (%d graph verbs on the Child lane)" % ("applied" if mode == "--apply" else "would apply", sum(len(rows) for rows in EDITS.values()), len(GRAPH_VERBS)))


main()
