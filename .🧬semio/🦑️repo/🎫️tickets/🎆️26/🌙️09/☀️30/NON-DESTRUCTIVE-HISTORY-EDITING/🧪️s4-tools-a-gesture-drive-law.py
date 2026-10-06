"""🌊️ Independent author of the streamed-gesture drive law (`🛠️tool-machine/🧫️fixtures/🧫️gesture-drive-law`).

A second implementation of the shared streamed-gesture runner (`drive_gesture` in `🛠️tool-machine/🦀️.rs`, `driveGesture`
in its TS twin) over a counting tool, written from the contract (design §5, audit item F8/F21 of ticket
26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) rather than from either twin. It writes the expected outcome of every row,
validates the fixture against `$defs/GestureDriveLawFixture` of the module's schema of record with `jsonschema` (draft 7)
and rejects hostile mutations, and with `--check` only verifies the committed fixture is current.

Session 5 (design §22.10) adds the framework-owned window slot: `hostEvents` — the reason each host fact ends a window's
open gesture with (a moved base ends only a gesture pinned to a base revision) — and `slots` — step sequences over the
per-window ledger (`GestureLedger`): a gesture persisted by one dispatch and resumed by the next, host facts that end it
with zero trace (a history edit freezing every window), windows that never share a slot, retired windows.

Press identity (coordinator decision, 2026-10-05): a dispatch may name the host press it belongs to. The slot owns that
identity: its gesture belongs to the press that opened it, and the ledger remembers the press each window last closed — a
dispatch of that press is dropped with zero trace (a late release after a blur commits nothing), a dispatch of another
press interrupts the open gesture first, and a refused dispatch changes neither.

Usage: .venv/bin/python 🧪️s4-tools-a-gesture-drive-law.py [--check] [--module <staged 🛠️tool-machine dir>]
"""

import copy
import json
import pathlib
import sys

from jsonschema import Draft7Validator
from referencing import Registry, Resource

REPO = pathlib.Path(__file__).resolve().parents[7]
MODULE = pathlib.Path(sys.argv[sys.argv.index("--module") + 1]).resolve() if "--module" in sys.argv else REPO / "🧰️framework/🔨️modules/🛠️tool-machine"
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

HOST_EVENTS = ("blur", "captureLost", "utilityChanged", "retiring", "timeTravelFrozen", "baseMoved")


def host_reason(event, base_bound):
    """📡️ The reason a host fact ends an open gesture with; a moved base ends only a gesture pinned to a base revision."""
    if event == "baseMoved":
        return "baseMoved" if base_bound else None
    return {"blur": "blur", "captureLost": "captureLost", "utilityChanged": "retired", "retiring": "retired", "timeTravelFrozen": "frozen"}[event]


class Slots:
    """🗂️ The per-window gesture ledger: at most one persisted gesture per window, driven by dispatches, ended by host facts."""

    def __init__(self):
        self.open = {}
        self.closed = {}

    def snapshot(self):
        return {window: copy.deepcopy(self.open[window]) for window in sorted(self.open)}

    def end(self, window, reason):
        press = self.open.pop(window).get("press")
        if press:
            self.closed[window] = press
        return {"window": window, "reason": reason}

    def drive(self, window, press, wire):
        """🪪️ One dispatch of `window`, optionally naming its host press: the ticks committed and the refusal."""
        held = self.open.get(window)
        if press and self.closed.get(window) == press:
            return None, None
        interrupts = bool(press and held and held.get("press") and held["press"] != press)
        bare = None if held is None or interrupts else {key: value for key, value in held.items() if key != "press"}
        result = drive(bare, wire)
        if result["refused"]:
            return None, result["refused"]
        continued = bare is not None and not bare.get("corrupt") and result["aborted"] is None
        if result["next"] == "unchanged":
            final = None if interrupts else held
        elif result["next"] == "cleared":
            final = None
        else:
            owner = held.get("press") if continued else press
            final = {**result["next"], **({"press": owner} if owner else {})}
        ended = held.get("press") if held else None
        if ended and (final is None or final.get("press") != ended):
            self.closed[window] = ended
        if press and parse_phase(wire)["kind"] != "stream":
            self.closed[window] = press
        if final is None:
            self.open.pop(window, None)
        else:
            self.open[window] = final
        return result["committed"], None

    def step(self, step):
        """👣️ One step and what it reports: the ticks committed, the refusal, the gestures host facts ended."""
        committed, refused, aborted = None, None, []
        if "drive" in step:
            window, press = step["drive"]["window"], step["drive"].get("press")
            committed, refused = self.drive(window, press, {key: value for key, value in step["drive"].items() if key not in ("window", "press")})
        elif "host" in step:
            window, event = step["host"]["window"], step["host"]["event"]
            reason = host_reason(event, bool(self.open[window]["base"])) if window in self.open else None
            aborted = [self.end(window, reason)] if reason else []
        elif "hostAll" in step:
            for window in sorted(self.open):
                reason = host_reason(step["hostAll"]["event"], bool(self.open[window]["base"]))
                aborted += [self.end(window, reason)] if reason else []
        elif "retain" in step:
            aborted = [self.end(window, "retired") for window in sorted(self.open) if window not in step["retain"]]
            self.closed = {window: press for window, press in self.closed.items() if window in step["retain"]}
        else:
            self.open[step["corrupt"]]["corrupt"] = True
        return {"committed": committed, "refused": refused, "aborted": aborted, "open": self.snapshot(), "closed": {window: self.closed[window] for window in sorted(self.closed)}}


def at(window, verb, tick, phase=None, base="r1", press=None):
    return {"drive": {"window": window, **({"press": press} if press else {}), **dispatch(verb, tick, phase, base=base)}}


def host(window, event):
    return {"host": {"window": window, "event": event}}


SLOTS = [
    ("a-gesture-persists-between-dispatches-and-commits-once", [at("w1", "drag", 1, "stream"), at("w1", "drag", 2, "stream"), at("w1", "drag", 3, "commit")]),
    ("a-history-edit-freezes-every-open-gesture-with-zero-trace", [at("w1", "drag", 1, "stream"), at("w2", "turn", 5, "stream"), {"hostAll": {"event": "timeTravelFrozen"}}, at("w1", "drag", None, "commit")]),
    ("a-frozen-window-ends-only-its-own-gesture", [at("w1", "drag", 1, "stream"), at("w2", "drag", 7, "stream"), host("w1", "timeTravelFrozen"), at("w2", "drag", 8, "commit")]),
    ("a-blur-ends-the-windows-gesture", [at("w1", "drag", 1, "stream"), host("w1", "blur"), at("w1", "drag", None, "stream")]),
    ("a-lost-pointer-capture-ends-the-windows-gesture", [at("w1", "drag", 1, "stream"), host("w1", "captureLost")]),
    ("a-utility-switch-retires-the-windows-gesture", [at("w1", "drag", 1, "stream"), host("w1", "utilityChanged")]),
    ("a-closing-window-retires-its-gesture", [at("w1", "drag", 1, "stream"), host("w1", "retiring")]),
    ("a-moved-base-ends-a-gesture-pinned-to-its-revision", [at("w1", "drag", 1, "stream"), host("w1", "baseMoved")]),
    ("a-moved-base-keeps-a-gesture-pinned-to-no-revision", [at("w1", "paint", 1, "stream", base=""), host("w1", "baseMoved"), at("w1", "paint", 2, "commit", base="")]),
    ("a-host-fact-at-rest-changes-nothing", [host("w1", "blur"), {"hostAll": {"event": "timeTravelFrozen"}}, {"retain": []}]),
    ("windows-never-share-a-slot", [at("w1", "drag", 1, "stream"), at("w2", "drag", 5, "stream"), at("w1", "drag", 2, "commit"), at("w2", "drag", 6, "stream")]),
    ("a-window-that-left-the-roster-retires-its-gesture", [at("w1", "drag", 1, "stream"), at("w2", "drag", 5, "stream"), at("w3", "paint", 9, "stream", base=""), {"retain": ["w2"]}]),
    ("another-verb-takes-the-windows-slot", [at("w1", "drag", 1, "stream"), at("w1", "turn", 5, "stream"), at("w1", "turn", 6, "commit")]),
    ("a-refused-tick-keeps-the-windows-slot", [at("w1", "drag", 1, "stream"), at("w1", "drag", -1, "stream"), at("w1", "drag", 2, "commit")]),
    ("an-unrestorable-slot-is-dropped-by-the-next-dispatch", [at("w1", "drag", 1, "stream"), {"corrupt": "w1"}, at("w1", "drag", None, "stream")]),
    ("a-host-fact-ends-an-unrestorable-slot-too", [at("w1", "drag", 1, "stream"), {"corrupt": "w1"}, host("w1", "timeTravelFrozen")]),
    ("a-moved-base-under-a-dispatch-drops-the-slot", [at("w1", "drag", 1, "stream"), at("w1", "drag", 2, "stream", base="r2"), at("w1", "drag", 3, "stream", base="r2")]),
    ("a-late-tick-of-a-committed-press-is-dropped", [at("w1", "drag", 1, "stream", press="p1"), at("w1", "drag", 2, "commit", press="p1"), at("w1", "drag", 3, "stream", press="p1"), at("w1", "drag", 4, "commit", press="p1")]),
    ("a-late-release-after-a-blur-commits-nothing", [at("w1", "drag", 1, "stream", press="p1"), host("w1", "blur"), at("w1", "drag", 9, "commit", press="p1"), at("w1", "drag", 2, "stream", press="p2")]),
    ("a-frozen-press-stays-ended", [at("w1", "drag", 1, "stream", press="p1"), {"hostAll": {"event": "timeTravelFrozen"}}, at("w1", "drag", 2, "stream", press="p1"), at("w1", "drag", 3, "commit", press="p1")]),
    ("another-press-interrupts-the-open-one", [at("w1", "drag", 1, "stream", press="p1"), at("w1", "drag", 5, "stream", press="p2"), at("w1", "drag", 9, "commit", press="p1"), at("w1", "drag", 6, "commit", press="p2")]),
    ("a-refused-tick-of-another-press-keeps-the-open-one", [at("w1", "drag", 1, "stream", press="p1"), at("w1", "drag", -1, "stream", press="p2"), at("w1", "drag", 2, "commit", press="p1")]),
    ("a-press-that-opened-nothing-is-not-closed", [at("w1", "drag", None, "stream", press="p1"), at("w1", "drag", 1, "stream", press="p1"), at("w1", "drag", 2, "commit", press="p1")]),
    ("a-one-shot-closes-its-press", [at("w1", "drag", 7, press="p1"), at("w1", "drag", 1, "stream", press="p1")]),
    ("a-moved-base-ends-a-named-press-for-good", [at("w1", "drag", 1, "stream", press="p1"), at("w1", "drag", 2, "stream", base="r2", press="p1"), at("w1", "drag", 3, "stream", base="r2", press="p1")]),
    ("an-unnamed-dispatch-rides-and-ends-a-named-gesture", [at("w1", "drag", 1, "stream", press="p1"), at("w1", "drag", 2, "stream"), at("w1", "drag", 3, "commit"), at("w1", "drag", 4, "stream", press="p1")]),
    ("a-named-dispatch-rides-an-unnamed-gesture", [at("w1", "drag", 1, "stream"), at("w1", "drag", 2, "stream", press="p1"), at("w1", "drag", 3, "commit", press="p1")]),
    ("another-verb-of-the-same-press-reopens-under-it", [at("w1", "drag", 1, "stream", press="p1"), at("w1", "turn", 5, "stream", press="p1"), at("w1", "turn", 6, "commit", press="p1")]),
    ("a-retired-window-forgets-its-closed-press", [at("w1", "drag", 1, "stream", press="p1"), at("w1", "drag", 2, "commit", press="p1"), {"retain": []}, at("w1", "drag", 3, "stream", press="p1")]),
    ("an-unrestorable-named-slot-reopens-under-the-dispatching-press", [at("w1", "drag", 1, "stream", press="p1"), {"corrupt": "w1"}, at("w1", "drag", 2, "stream"), at("w1", "drag", 3, "commit", press="p1")]),
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
    "dispatch runs from rest; a refused start or tick has no effect at all. `hostEvents` pins the reason each host fact ends "
    "a window's open gesture with (`null`: the gesture stays; a moved base ends only a gesture pinned to a base revision). "
    "Every `slots` scenario runs its steps over the per-window ledger (`GestureLedger`) — a dispatch of a window, a host fact "
    "of one window or of all, the window roster, a tampered slot — and must report after each step the ticks committed, the "
    "refusal, the gestures host facts ended, every window's persisted gesture and the press each window last closed. A "
    "dispatch may name its host `press`: the gesture belongs to the press that opened it, a dispatch of the press a window "
    "last closed is dropped with zero trace, a dispatch of another press interrupts the open gesture first. Authored by "
    "`🧪️s4-tools-a-gesture-drive-law.py`, an independent Python model."
)


def slots():
    scenarios = []
    for name, steps in SLOTS:
        ledger = Slots()
        scenarios.append({"name": name, "steps": [{**step, "expected": ledger.step(step)} for step in steps]})
    return scenarios


def fixture():
    rows = [{"name": name, "persisted": persisted, "dispatch": wire, "expected": drive(persisted, wire)} for name, persisted, wire in ROWS]
    for row in rows:
        expected = row["expected"]
        if expected["refused"] and (expected["aborted"] or expected["committed"] or expected["next"] != "unchanged"):
            raise SystemExit(f"{row['name']}: a refused dispatch has an effect")
    host_events = [{"event": event, "baseBound": bound, "reason": host_reason(event, bound)} for event in HOST_EVENTS for bound in (True, False)]
    return {
        "schema": "semio.framework.tool-machine.gesture-drive-law.v1",
        "note": NOTE,
        "tool": TOOL,
        "phases": [{"args": args, "phase": parse_phase(args)} for args in PHASES],
        "rows": rows,
        "hostEvents": host_events,
        "slots": slots(),
    }


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
        "an unknown host fact": lambda doc: doc["hostEvents"][0].__setitem__("event", "scroll"),
        "a host reason that is no abort reason": lambda doc: doc["hostEvents"][0].__setitem__("reason", "timeTravelFrozen"),
        "a slot step doing two things": lambda doc: doc["slots"][0]["steps"][0].__setitem__("retain", []),
        "a slot step without its outcome": lambda doc: doc["slots"][0]["steps"][0].pop("expected"),
        "a host fact without its window": lambda doc: doc["slots"][3]["steps"][1]["host"].pop("window"),
        "an ended gesture without its reason": lambda doc: doc["slots"][3]["steps"][1]["expected"]["aborted"][0].pop("reason"),
        "an open slot that is no gesture": lambda doc: doc["slots"][0]["steps"][0]["expected"]["open"].__setitem__("w1", "open"),
        "a step without its closed presses": lambda doc: doc["slots"][0]["steps"][0]["expected"].pop("closed"),
        "an empty press": lambda doc: doc["slots"][17]["steps"][0]["drive"].__setitem__("press", ""),
        "a closed press that is no text": lambda doc: doc["slots"][17]["steps"][1]["expected"]["closed"].__setitem__("w1", 1),
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
    steps = sum(len(scenario["steps"]) for scenario in document["slots"])
    summary = f"{len(document['rows'])} rows ({committed} commit, {refused} refused), {len(document['phases'])} phase words, {len(document['hostEvents'])} host facts, {len(document['slots'])} slot scenarios ({steps} steps)"
    if "--check" in sys.argv:
        current = FIXTURE.read_text(encoding="utf-8") if FIXTURE.exists() else ""
        if current != text:
            raise SystemExit(f"{FIXTURE} is stale: re-run without --check")
        print(f"fixture current: {summary}")
        return
    FIXTURE.parent.mkdir(parents=True, exist_ok=True)
    FIXTURE.write_text(text, encoding="utf-8")
    print(f"wrote {FIXTURE}: {summary}")


if __name__ == "__main__":
    main()
