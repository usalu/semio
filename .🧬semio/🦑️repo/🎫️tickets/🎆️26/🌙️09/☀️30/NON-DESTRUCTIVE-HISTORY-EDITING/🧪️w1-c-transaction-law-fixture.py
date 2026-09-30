"""🧫️ W1-C: writes the hand-authored transaction-law expectations as canonical JSON (indent 2). Usage: `bun 🧪️w1-c-transaction-ids.ts > ids.jsonl && python3 🧪️w1-c-transaction-law-fixture.py ids.jsonl <owner>/🧫️fixtures/🧫️transaction-law/🔣️.json`."""
import json, sys

TOOL = "demo#drag"
T2 = {"id": "tx-e4f8055f280d5342", "tool": TOOL}
T5 = {"id": "tx-8d464e628edab68b", "tool": TOOL}
T7 = {"id": "tx-82d57c9700d82af7", "tool": TOOL}
CLOSED, UNCLOSED = "toolTransaction.closed", "toolTransaction.unclosed"

def up(key, n): return {"kind": "upsert", "key": key, "mutation": {"n": n}}
def rt(key): return {"kind": "retract", "key": key}
COMMIT, ABORT = {"kind": "commit"}, {"kind": "abort"}
def e(key, n): return {"key": key, "mutation": {"n": n}}
def case(name, yields, state, entries, refused=()):
    return {"name": name, "transaction": T2, "yields": yields, "expect": {"state": state, "entries": entries, "refused": list(refused)}}

def tr(dx, dy): return {"kind": "translate", "dx": dx, "dy": dy}
CONNECT = {"kind": "connect", "x": 100, "y": 0}
def ent(key, m): return {"key": key, "mutation": m}
def clock(n): return {"actor": 7, "physical_ms": 1000 + n, "logical": 0}
def down(x, y): return {"type": "pointerDown", "x": x, "y": y}
def move(x, y): return {"type": "pointerMove", "x": x, "y": y}
UP, ESC = {"type": "pointerUp"}, {"type": "escape"}
IDLE = {"kind": "idle"}
def opened(t, entries): return {"kind": "open", "transaction": t, "entries": entries}
def committed(t, mutations): return {"kind": "committed", "transaction": t, "mutations": mutations}
def empty(t): return {"kind": "empty", "transaction": t}
def aborted(t, reason): return {"kind": "aborted", "transaction": t, "reason": reason}
def abort(reason): return {"host": "abort", "reason": reason}
RESET = {"host": "reset"}
RESUME = {"host": "resume"}
def refused(code): return {"refusal": code}
def row(index, ev, st, sp):
    if "host" in ev and ev["host"] == "abort": return {"host": "abort", "reason": ev["reason"], "state": st, "step": sp}
    if "host" in ev and ev["host"] == "resume": return {"host": "resume", "state": st}
    if "host" in ev: return {"host": "reset", "state": st, "dropped": sp}
    outcome = sp if "refusal" in sp else {"step": sp}
    return {"event": ev, "clock": clock(index + 1), "state": st, **outcome}
def scenario(name, rows):
    return {"name": name, "steps": [row(i, ev, st, sp) for i, (ev, st, sp) in enumerate(rows)]}
def transitions(release):
    return [
        {"from": "idle", "event": "pointerDown", "to": "pressed", "action": "press"},
        {"from": "pressed", "event": "pointerMove", "to": "dragging", "action": "drag"},
        {"from": "pressed", "event": "pointerUp", "to": "idle", "action": None},
        {"from": "pressed", "event": "escape", "to": "idle", "action": None},
        {"from": "dragging", "event": "pointerMove", "to": "dragging", "action": "drag"},
        {"from": "dragging", "event": "pointerUp", "to": "idle", "action": release},
        {"from": "dragging", "event": "escape", "to": "idle", "action": "cancel"},
    ]
def chart(id, release): return {"id": id, "initial": "idle", "states": ["idle", "pressed", "dragging"], "transitions": transitions(release)}

fixture = {
    "schema": "semio.framework.tool-machine.transaction-law.v1",
    "states": ["open", "committed", "aborted"],
    "yieldKinds": ["upsert", "retract", "commit", "abort"],
    "refusals": [CLOSED, UNCLOSED],
    "abortReasons": ["tool", "blur", "captureLost", "baseMoved", "frozen", "retired"],
    "matrix": [
        {"from": "open", "yield": "upsert", "to": "open"},
        {"from": "open", "yield": "retract", "to": "open"},
        {"from": "open", "yield": "commit", "to": "committed"},
        {"from": "open", "yield": "abort", "to": "aborted"},
    ] + [{"from": s, "yield": k, "refusal": CLOSED} for s in ("committed", "aborted") for k in ("upsert", "retract", "commit", "abort")],
    "cases": [
        case("upsert appends new keys in order", [up("a", 1), up("b", 2)], "open", [e("a", 1), e("b", 2)]),
        case("upsert replaces by key keeping the first-insertion slot", [up("a", 1), up("b", 2), up("a", 3)], "open", [e("a", 3), e("b", 2)]),
        case("retract removes the key", [up("a", 1), up("b", 2), rt("a")], "open", [e("b", 2)]),
        case("retract of an unknown key changes nothing", [up("a", 1), rt("z")], "open", [e("a", 1)]),
        case("retract then upsert re-inserts at the end", [up("a", 1), up("b", 2), rt("a"), up("a", 3)], "open", [e("b", 2), e("a", 3)]),
        case("commit keeps the entries in first-insertion order", [up("a", 1), up("b", 2), up("a", 3), COMMIT], "committed", [e("a", 3), e("b", 2)]),
        case("commit of an emptied transaction is no edit", [up("a", 1), rt("a"), COMMIT], "committed", []),
        case("commit without yields is no edit", [COMMIT], "committed", []),
        case("abort leaves zero trace", [up("a", 1), up("b", 2), ABORT], "aborted", []),
        case("every yield after commit is refused", [up("a", 1), COMMIT, up("b", 2), ABORT, rt("a"), COMMIT], "committed", [e("a", 1)], [2, 3, 4, 5]),
        case("every yield after abort is refused", [up("a", 1), ABORT, up("b", 2), COMMIT], "aborted", [], [2, 3]),
    ],
    "ids": [json.loads(line) for line in open(sys.argv[1])],
    "gesture": {
        "tool": TOOL,
        "actor": "actor-1",
        "input": {"magnet": {"x": 100, "y": 0}, "radius": 10},
        "resume": [{"state": state, "transaction": transaction, "expect": expect} for state, transaction, expect in [
            ("idle", "none", "ok"),
            ("idle", "open", UNCLOSED),
            ("idle", "committed", CLOSED),
            ("idle", "aborted", CLOSED),
            ("pressed", "none", "ok"),
            ("pressed", "open", "ok"),
            ("dragging", "none", "ok"),
            ("dragging", "open", "ok"),
            ("dragging", "committed", CLOSED),
            ("dragging", "aborted", CLOSED),
        ]],
        "variants": [{
            "chart": chart("drag_gesture", "release"),
            "scenarios": [
            scenario("press, drag and release commit one transaction", [
                (down(0, 0), "pressed", IDLE),
                (move(5, 0), "dragging", opened(T2, [ent("translate", tr(5, 0))])),
                (move(10, 4), "dragging", opened(T2, [ent("translate", tr(10, 4))])),
                (UP, "idle", committed(T2, [tr(10, 4)])),
            ]),
            scenario("escape aborts with zero trace", [
                (down(0, 0), "pressed", IDLE),
                (move(3, 4), "dragging", opened(T2, [ent("translate", tr(3, 4))])),
                (ESC, "idle", aborted(T2, "tool")),
            ]),
            scenario("a second gesture is a second transaction", [
                (down(0, 0), "pressed", IDLE),
                (move(2, 0), "dragging", opened(T2, [ent("translate", tr(2, 0))])),
                (UP, "idle", committed(T2, [tr(2, 0)])),
                (down(10, 10), "pressed", IDLE),
                (move(10, 13), "dragging", opened(T5, [ent("translate", tr(0, 3))])),
                (UP, "idle", committed(T5, [tr(0, 3)])),
            ]),
            scenario("a click without a drag publishes nothing", [
                (down(1, 1), "pressed", IDLE),
                (UP, "idle", IDLE),
            ]),
            scenario("dragging back to the origin commits no edit", [
                (down(0, 0), "pressed", IDLE),
                (move(3, 3), "dragging", opened(T2, [ent("translate", tr(3, 3))])),
                (move(0, 0), "dragging", opened(T2, [])),
                (UP, "idle", empty(T2)),
            ]),
            scenario("magnet proximity upserts and retracts the connection", [
                (down(80, 0), "pressed", IDLE),
                (move(95, 0), "dragging", opened(T2, [ent("translate", tr(15, 0)), ent("connect", CONNECT)])),
                (move(70, 0), "dragging", opened(T2, [ent("translate", tr(-10, 0))])),
                (move(92, 1), "dragging", opened(T2, [ent("translate", tr(12, 1)), ent("connect", CONNECT)])),
                (UP, "idle", committed(T2, [tr(12, 1), CONNECT])),
            ]),
            scenario("a drag starting on the magnet keeps the connection first", [
                (down(100, 0), "pressed", IDLE),
                (move(100, 0), "dragging", opened(T2, [ent("connect", CONNECT)])),
                (move(104, 3), "dragging", opened(T2, [ent("connect", CONNECT), ent("translate", tr(4, 3))])),
                (UP, "idle", committed(T2, [CONNECT, tr(4, 3)])),
            ]),
            scenario("escape while pressed changes nothing", [
                (down(0, 0), "pressed", IDLE),
                (ESC, "idle", IDLE),
            ]),
            scenario("pointer events before a press are ignored", [
                (move(5, 5), "idle", IDLE),
                (UP, "idle", IDLE),
                (ESC, "idle", IDLE),
            ]),
            scenario("a host abort while dragging drops the transaction and rests the tool", [
                (down(0, 0), "pressed", IDLE),
                (move(5, 5), "dragging", opened(T2, [ent("translate", tr(5, 5))])),
                (abort("blur"), "idle", aborted(T2, "blur")),
                (move(9, 9), "idle", IDLE),
                (UP, "idle", IDLE),
                (down(0, 0), "pressed", IDLE),
                (move(1, 0), "dragging", opened(T7, [ent("translate", tr(1, 0))])),
                (UP, "idle", committed(T7, [tr(1, 0)])),
            ]),
            scenario("a host abort without an open transaction reports idle", [
                (down(0, 0), "pressed", IDLE),
                (abort("captureLost"), "idle", IDLE),
                (abort("frozen"), "idle", IDLE),
                (UP, "idle", IDLE),
            ]),
            scenario("a gesture persisted and resumed between events is still one transaction", [
                (down(0, 0), "pressed", IDLE),
                (move(5, 0), "dragging", opened(T2, [ent("translate", tr(5, 0))])),
                (RESUME, "dragging", None),
                (move(10, 4), "dragging", opened(T2, [ent("translate", tr(10, 4))])),
                (RESUME, "dragging", None),
                (UP, "idle", committed(T2, [tr(10, 4)])),
                (RESUME, "idle", None),
            ]),
            scenario("a host reset drops the transaction silently", [
                (down(0, 0), "pressed", IDLE),
                (move(2, 2), "dragging", opened(T2, [ent("translate", tr(2, 2))])),
                (RESET, "idle", T2),
                (UP, "idle", IDLE),
                (RESET, "idle", None),
            ]),
        ]}, {
            "chart": chart("leaky_gesture", None),
            "scenarios": [
                scenario("a release that forgets to commit is refused and the next gesture opens a new transaction", [
                    (down(0, 0), "pressed", IDLE),
                    (move(4, 0), "dragging", opened(T2, [ent("translate", tr(4, 0))])),
                    (UP, "idle", refused(UNCLOSED)),
                    (down(0, 0), "pressed", IDLE),
                    (move(0, 6), "dragging", opened(T5, [ent("translate", tr(0, 6))])),
                    (ESC, "idle", aborted(T5, "tool")),
                ]),
                scenario("an escape still aborts the leaky tool", [
                    (down(0, 0), "pressed", IDLE),
                    (move(3, 0), "dragging", opened(T2, [ent("translate", tr(3, 0))])),
                    (ESC, "idle", aborted(T2, "tool")),
                    (abort("retired"), "idle", IDLE),
                ]),
            ],
        }],
    },
    "invariants": [
        {"id": "upsertReplacesInPlace", "statement": "an upsert of a present key replaces its mutation in its first-insertion slot; a new key is appended"},
        {"id": "retractRemoves", "statement": "a retract removes its key; a retract of an absent key changes nothing; a later upsert of that key appends it"},
        {"id": "emptyCommitIsNoEdit", "statement": "a transaction that commits with no entries publishes no edit"},
        {"id": "abortLeavesZeroTrace", "statement": "an aborted transaction keeps no entries and publishes nothing"},
        {"id": "closedRefusesEveryYield", "statement": "a committed or aborted transaction refuses every yield with toolTransaction.closed and stays unchanged"},
        {"id": "oneTransactionOneEdit", "statement": "one committed transaction is one edit, one undo step and one history row; every op carries its TransactionRef"},
        {"id": "opensOnFirstUpsert", "statement": "a runner opens a transaction at the first upsert while none is open; retract, commit and abort without an open transaction change nothing"},
        {"id": "idFromOpeningEvent", "statement": "the transaction id is minted from the actor, the clock of the opening event and the tool"},
        {"id": "oneClosePerEvent", "statement": "one event closes at most one transaction; a yield after the close within the same event is refused (toolTransaction.closed) and the event publishes nothing"},
        {"id": "restClosesTransaction", "statement": "an event that leaves the tool at rest (the root's initial state active) with its transaction open is refused (toolTransaction.unclosed); the transaction is dropped, so the next gesture opens a new one"},
        {"id": "resumeContinuesTheGesture", "statement": "persisting a runner (snapshot and open transaction) and resuming it between any two events yields the same steps and ids as the uninterrupted runner; resume refuses a closed transaction (toolTransaction.closed) and a resting snapshot holding an open transaction (toolTransaction.unclosed)"},
        {"id": "hostCancelLeavesZeroTrace", "statement": "a host abort or reset drops the open transaction without an edit, cancels the timers and invokes of the configuration it leaves and returns the statechart to its initial configuration; abort reports aborted with its reason, or idle when nothing was open"},
        {"id": "toolStateIsNotHistory", "statement": "the statechart configuration and context are ephemeral tool state; only committed mutations and their TransactionRef are durable"},
    ],
}
json.dump(fixture, open(sys.argv[2], "w", encoding="utf-8"), indent=2, ensure_ascii=False)
open(sys.argv[2], "a").write("\n")
