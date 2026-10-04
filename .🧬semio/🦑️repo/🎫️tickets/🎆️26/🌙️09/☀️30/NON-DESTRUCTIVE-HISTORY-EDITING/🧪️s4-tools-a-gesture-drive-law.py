"""🌊️ Independent author of the streamed-gesture drive law (`🛠️tool-machine/🧫️fixtures/🧫️gesture-drive-law`).

A second implementation of the shared streamed-gesture runner (`drive_gesture` in `🛠️tool-machine/🦀️.rs`, `driveGesture`
in its TS twin) over a counting tool, written from the contract (design §5, audit item F8/F21 of ticket
26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) rather than from either twin. It writes the expected outcome of every row,
validates the fixture against `$defs/GestureDriveLawFixture` of the module's schema of record with `jsonschema` (draft 7)
and rejects hostile mutations, and with `--check` only verifies the committed fixture is current.

Usage: .venv/bin/python 🧪️s4-tools-a-gesture-drive-law.py [--check]
"""

import copy
import json
import pathlib
import sys

from jsonschema import Draft7Validator
from referencing import Registry, Resource

REPO = pathlib.Path(__file__).resolve().parents[7]
MODULE = REPO / "🧰️framework/🔨️modules/🛠️tool-machine"
FIXTURE = MODULE / "🧫️fixtures/🧫️gesture-drive-law/🔣️.json"
SCHEMA = MODULE / "🧬️schema/🔣️.json"

REASONS = ("tool", "blur", "captureLost", "baseMoved", "frozen", "retired")
CLOSED = "toolTransaction.closed"
UNCLOSED = "toolTransaction.unclosed"
TOOL = {"refuseStartVerb": "refuse-start", "startRefusal": CLOSED, "resumeRefusal": CLOSED, "sendRefusal": UNCLOSED}


class Refused(Exception):
    """🚫️ A counting-tool refusal carrying its fault code."""

    def __init__(self, code):
        super().__init__(code)
        self.code = code


def parse_phase(args):
    """🔡️ The wire `phase`/`reason` pair: absent phase = one-shot, absent reason = `tool`, unknown words = None."""
    phase, reason = args.get("phase"), args.get("reason")
    if phase is None:
        return {"kind": "once"}
    if phase in ("stream", "commit"):
        return {"kind": phase}
    if phase == "abort":
        reason = "tool" if reason is None else reason
        return {"kind": "abort", "reason": reason} if reason in REASONS else None
    return None


class Counter:
    """🧮️ The counting tool: a stream tick accumulates, a one-shot or commit folds its tick in and commits the ticks."""

    def __init__(self, verb, base, ticks):
        self.verb, self.base, self.ticks = verb, base, ticks

    @staticmethod
    def start(verb, base):
        if verb == TOOL["refuseStartVerb"]:
            raise Refused(TOOL["startRefusal"])
        return Counter(verb, base, [])

    @staticmethod
    def resume(gesture):
        if gesture.get("corrupt"):
            raise Refused(TOOL["resumeRefusal"])
        return Counter(gesture["verb"], gesture["base"], list(gesture["ticks"]))

    def send(self, kind, tick):
        if tick is not None and tick < 0:
            raise Refused(TOOL["sendRefusal"])
        self.ticks += [] if tick is None else [tick]
        if kind == "stream":
            return None
        committed, self.ticks = self.ticks, []
        return committed or None

    def persist(self):
        return {"verb": self.verb, "base": self.base, "ticks": self.ticks} if self.ticks else None


def outcome(aborted=None, committed=None, next="unchanged", refused=None):
    return {"aborted": aborted, "committed": committed, "next": next, "refused": refused}


def drive(persisted, dispatch):
    """🚂️ One dispatch from the window's persisted gesture. A gesture its tool cannot restore is dropped with zero trace
    and the dispatch runs from rest; a moved base drops the gesture (a one-shot then commits fresh) and wins over another
    verb or a one-shot interrupting it (`captureLost`); a refused start or tick faults the dispatch with no effect at all."""
    kind = parse_phase(dispatch)
    assert kind is not None, dispatch
    resumed = None
    if persisted is not None:
        try:
            resumed = Counter.resume(persisted)
        except Refused:
            resumed = None
    dropped = "unchanged" if persisted is None else "cleared"
    if kind["kind"] == "abort":
        return outcome(aborted=kind["reason"] if resumed else None, next=dropped)
    interrupted, tool = None, None
    if resumed is not None and resumed.base != dispatch["base"]:
        if kind["kind"] != "once":
            return outcome(aborted="baseMoved", next="cleared")
        interrupted = "baseMoved"
    elif resumed is not None and (resumed.verb != dispatch["verb"] or kind["kind"] == "once"):
        interrupted = "captureLost"
    else:
        tool = resumed
    try:
        tool = tool or Counter.start(dispatch["verb"], dispatch["base"])
        committed = tool.send(kind["kind"], dispatch["tick"])
    except Refused as refusal:
        return outcome(refused=refusal.code)
    gesture = tool.persist()
    return outcome(aborted=interrupted, committed=committed, next="unchanged" if gesture == persisted else gesture or "cleared")


def gesture(verb, ticks, base="r1", corrupt=False):
    return {"verb": verb, "base": base, "ticks": ticks, **({"corrupt": True} if corrupt else {})}


def dispatch(verb, tick, phase=None, reason=None, base="r1"):
    return {"verb": verb, **({"phase": phase} if phase else {}), **({"reason": reason} if reason else {}), "tick": tick, "base": base}


ROWS = [
    ("a-one-shot-at-rest-commits-its-tick", None, dispatch("drag", 1)),
    ("a-one-shot-without-a-tick-commits-nothing", None, dispatch("drag", None)),
    ("the-first-stream-tick-opens-the-gesture", None, dispatch("drag", 1, "stream")),
    ("a-stream-tick-accumulates-into-the-open-gesture", gesture("drag", [1]), dispatch("drag", 2, "stream")),
    ("the-commit-folds-its-tick-in-and-commits-the-gesture", gesture("drag", [1, 2]), dispatch("drag", 3, "commit")),
    ("a-tickless-commit-commits-the-open-gesture", gesture("drag", [1]), dispatch("drag", None, "commit")),
    ("a-commit-at-rest-commits-its-tick", None, dispatch("drag", 4, "commit")),
    ("a-blur-aborts-the-open-gesture", gesture("drag", [1, 2]), dispatch("drag", None, "abort", "blur")),
    ("an-abort-at-rest-changes-nothing", None, dispatch("drag", None, "abort", "blur")),
    ("an-abort-without-a-reason-is-the-tools", gesture("drag", [1]), dispatch("drag", None, "abort")),
    ("a-retired-window-aborts-the-open-gesture", gesture("drag", [1]), dispatch("drag", None, "abort", "retired")),
    ("a-stream-tick-on-a-moved-base-drops-the-gesture", gesture("drag", [1]), dispatch("drag", 2, "stream", base="r2")),
    ("a-commit-on-a-moved-base-drops-the-gesture", gesture("drag", [1, 2]), dispatch("drag", 3, "commit", base="r2")),
    ("a-one-shot-on-a-moved-base-commits-fresh", gesture("drag", [1]), dispatch("drag", 9, base="r2")),
    ("another-verbs-stream-tick-interrupts-the-gesture", gesture("drag", [1]), dispatch("turn", 5, "stream")),
    ("another-verbs-commit-interrupts-and-commits-fresh", gesture("drag", [1]), dispatch("turn", 5, "commit")),
    ("a-one-shot-interrupts-the-same-verbs-gesture", gesture("drag", [1, 2]), dispatch("drag", 7)),
    ("a-tickless-stream-resumes-and-keeps-the-gesture", gesture("drag", [1]), dispatch("drag", None, "stream")),
    ("a-stream-tick-drops-an-unrestorable-gesture-and-opens-fresh", gesture("drag", [1], corrupt=True), dispatch("drag", 2, "stream")),
    ("a-commit-drops-an-unrestorable-gesture-and-commits-fresh", gesture("drag", [1], corrupt=True), dispatch("drag", 2, "commit")),
    ("a-bare-stream-tick-drops-an-unrestorable-gesture", gesture("drag", [1], corrupt=True), dispatch("drag", None, "stream")),
    ("an-abort-drops-an-unrestorable-gesture-without-trace", gesture("drag", [1], corrupt=True), dispatch("drag", None, "abort", "blur")),
    ("a-one-shot-replaces-an-unrestorable-gesture", gesture("drag", [1], corrupt=True), dispatch("drag", 7)),
    ("a-refused-start-faults-the-dispatch", None, dispatch("refuse-start", 1, "stream")),
    ("a-refused-first-tick-faults-the-dispatch", None, dispatch("drag", -1, "stream")),
    ("a-refused-commit-keeps-the-open-gesture", gesture("drag", [1]), dispatch("drag", -3, "commit")),
    ("a-bare-stream-tick-at-rest-opens-nothing", None, dispatch("drag", None, "stream")),
    ("a-moved-base-wins-over-a-verb-switch", gesture("drag", [1]), dispatch("turn", 5, "stream", base="r2")),
    ("a-refused-start-after-a-verb-switch-aborts-nothing", gesture("drag", [1]), dispatch("refuse-start", 1, "stream")),
    ("a-refused-tick-after-a-one-shot-interruption-aborts-nothing", gesture("drag", [1]), dispatch("drag", -1)),
    ("a-refused-tick-keeps-an-unrestorable-gesture", gesture("drag", [1], corrupt=True), dispatch("drag", -1, "stream")),
]

PHASES = [
    {},
    {"reason": "blur"},
    {"phase": "stream"},
    {"phase": "stream", "reason": "blur"},
    {"phase": "commit"},
    {"phase": "abort"},
    *({"phase": "abort", "reason": reason} for reason in REASONS),
    {"phase": "abort", "reason": "sideways"},
    {"phase": "once"},
    {"phase": ""},
    {"phase": "Stream"},
]

NOTE = (
    "🌊️ The shared streamed-gesture runner (design §5 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): one dispatch of a "
    "window's streamed tool (a gumball drag, a paint stroke) from the gesture its window persisted. `phases` pins the wire "
    "`phase`/`reason` arguments (absent phase = one-shot, absent reason = `tool`, unknown words refused). Every row drives the "
    "counting `tool` from `persisted` through `dispatch` in Rust (`drive_gesture`) and TypeScript (`driveGesture`) and must "
    "report `expected`: the reason the open gesture was aborted with, the ticks committed as ONE transaction, the window's next "
    "gesture and the refusal that faulted the dispatch. A gesture its tool cannot restore is dropped with zero trace and the "
    "dispatch runs from rest; a refused start or tick has no effect at all. Authored by "
    "`🧪️s4-tools-a-gesture-drive-law.py`, an independent Python model."
)


def fixture():
    rows = [{"name": name, "persisted": persisted, "dispatch": wire, "expected": drive(persisted, wire)} for name, persisted, wire in ROWS]
    for row in rows:
        expected = row["expected"]
        if expected["refused"] and (expected["aborted"] or expected["committed"] or expected["next"] != "unchanged"):
            raise SystemExit(f"{row['name']}: a refused dispatch has an effect")
    return {"schema": "semio.framework.tool-machine.gesture-drive-law.v1", "note": NOTE, "tool": TOOL, "phases": [{"args": args, "phase": parse_phase(args)} for args in PHASES], "rows": rows}


def validate(document):
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    registry = Registry().with_resource(schema["$id"], Resource.from_contents(schema))
    validator = Draft7Validator({"$ref": f"{schema['$id']}#/$defs/GestureDriveLawFixture"}, registry=registry)
    errors = list(validator.iter_errors(document))
    if errors:
        raise SystemExit("\n".join(f"{list(error.absolute_path)}: {error.message}" for error in errors))
    hostile = {
        "a one-shot phase word": lambda doc: doc["rows"][0]["dispatch"].__setitem__("phase", "once"),
        "an unknown abort reason": lambda doc: doc["rows"][7]["dispatch"].__setitem__("reason", "sideways"),
        "an empty persisted gesture": lambda doc: doc["rows"][3]["persisted"].__setitem__("ticks", []),
        "a text tick": lambda doc: doc["rows"][0]["dispatch"].__setitem__("tick", "1"),
        "an unknown next": lambda doc: doc["rows"][0]["expected"].__setitem__("next", "kept"),
        "a missing refusal": lambda doc: doc["rows"][0]["expected"].pop("refused"),
        "an empty commit": lambda doc: doc["rows"][0]["expected"].__setitem__("committed", []),
        "a falsy corrupt flag": lambda doc: doc["rows"][3]["persisted"].__setitem__("corrupt", False),
        "an undeclared field": lambda doc: doc["rows"][0].__setitem__("seed", "s"),
    }
    for name, mutate in hostile.items():
        mutated = copy.deepcopy(document)
        mutate(mutated)
        if not list(validator.iter_errors(mutated)):
            raise SystemExit(f"the schema accepts {name}")


def render(document):
    return json.dumps(document, ensure_ascii=False, indent=2) + "\n"


def main():
    document = fixture()
    validate(document)
    text = render(document)
    refused = sum(1 for row in document["rows"] if row["expected"]["refused"])
    committed = sum(1 for row in document["rows"] if row["expected"]["committed"])
    summary = f"{len(document['rows'])} rows ({committed} commit, {refused} refused), {len(document['phases'])} phase words"
    if "--check" in sys.argv:
        current = FIXTURE.read_text(encoding="utf-8") if FIXTURE.exists() else ""
        if current != text:
            raise SystemExit(f"{FIXTURE} is stale: re-run without --check")
        print(f"fixture current: {summary}")
        return
    FIXTURE.parent.mkdir(parents=True, exist_ok=True)
    FIXTURE.write_text(text, encoding="utf-8")
    print(f"wrote {FIXTURE.relative_to(REPO)}: {summary}")


if __name__ == "__main__":
    main()
