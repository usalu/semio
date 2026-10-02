"""⏪️ Writes `🧰️framework/🔨️modules/⏪️time-travel/🧫️fixtures/🧫️lifecycle-law/🔣️.json`.

Independent of the Rust and TS reducers: the law table, the concrete cases and the scenarios below are
transcribed by hand from `📋️design.md` §4 plus the approved W1-B interpretations (`📓️w1-b-report.md`).
Run from the repo root: `python3 <this file>`.
"""

import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[7]
OUT = ROOT / "🧰️framework/🔨️modules/⏪️time-travel/🧫️fixtures/🧫️lifecycle-law/🔣️.json"

SCHEMA = "s.demo.op"
G = 5
ID = 3
B1 = {"storeGeneration": 10, "contentRevision": "11" * 32}
B2 = {"storeGeneration": 11, "contentRevision": "22" * 32}
POS = {"a": 2, "b": 5, "c": 9}
MOVED = {"a": 3, "b": 6, "c": 10}


def inp(payload):
    return {"kind": "input", "schema": SCHEMA, "payload": list(bytes.fromhex(payload))}


WITHDRAWN = {"kind": "withdrawn"}
ORIG = {"a": inp("0a01"), "b": inp("0b01"), "c": inp("0c01")}
DRAFT = {"a": inp("0a02"), "b": inp("0b02"), "c": inp("0c02")}


def tgt(mutation, position):
    return {"mutation": mutation, "position": position}


def draft(mutation, replacement=None, position=None):
    return {"target": tgt(mutation, POS[mutation] if position is None else position), "replacement": DRAFT[mutation] if replacement is None else replacement}


def pend(mutation, replacement, return_stage, position=None):
    return {"target": tgt(mutation, POS[mutation] if position is None else position), "original": ORIG[mutation], "replacement": replacement, "returnStage": return_stage}


def outcome(mutation, position, worst, messages, superseded=False, withdrawn=False):
    return {"mutationId": mutation, "editId": f"edit-{mutation}", "opIndex": position, "worst": worst, "messages": messages, "superseded": superseded, "withdrawn": withdrawn}


PARTIAL = {"level": "warning", "code": "mutation.partial", "message": "1 of 2 targets is missing", "target": ["node", "7"], "opIndex": 0}
MISSING = {"level": "error", "code": "mutation.target-missing", "message": "every target is missing", "opIndex": 0}
CLEAN = {"fromPosition": 2, "outcomes": [outcome("a", 0, None, [], superseded=True), outcome("b", 0, "warning", [PARTIAL]), outcome("c", 0, None, [])], "worst": "warning"}
BLOCKING = {"fromPosition": 2, "outcomes": [outcome("a", 0, None, [], superseded=True), outcome("b", 0, "error", [MISSING]), outcome("c", 0, None, [])], "worst": "error"}


def session(stage, accepted=(), pending=None, report=None, progress=None, fault=None, base=B1, generation=G, id=ID):
    return {
        "id": id,
        "generation": generation,
        "base": base,
        "stage": stage,
        "accepted": [entry if isinstance(entry, dict) else draft(entry) for entry in accepted],
        "pending": pending,
        "report": report,
        "progress": progress,
        "fault": fault,
    }


STAGES = ["inactive", "editing", "replaying", "reviewing", "choosing", "finalizing"]
EVENT_KEYS = ["begin", "draft", "withdraw", "accept", "discard", "replayProgressed", "replayCompleted", "replayCancelled", "replayFaulted", "rerun", "requestFinalize", "chooseOverwrite", "chooseAlternative", "back", "finalized", "finalizeFaulted", "baseMoved", "exit"]
EFFECT_KINDS = ["showPreview", "startReplay", "cancelReplay", "openFinalizePrompt", "commitOverwrite", "commitAlternative", "close"]
REFUSALS = ["timeTravel.illegal", "timeTravel.stale", "timeTravel.blocked", "timeTravel.empty"]
GUARDS = [
    "always",
    "newBase",
    "newBaseEmpty",
    "sameBase",
    "pendingUnchanged",
    "pendingChanged",
    "unchangedReturnInactive",
    "unchangedReturnReviewing",
    "unchangedReturnReplaying",
    "changed",
    "changedToEmpty",
    "returnInactive",
    "returnReviewing",
    "returnReplaying",
    "ready",
    "acceptedEmpty",
    "reportMissing",
    "reportBlocking",
    "needsReplay",
    "reportCurrent",
    "validCode",
    "invalidCode",
    "validName",
    "invalidName",
]
REVIEW_KINDS = ["noChanges", "needsReplay", "blocked", "ready"]
CANCELLED = "timeTravel.cancelled"
TEXT_MAX_BYTES = 256

CONTEXTS = {
    "inactive": session("inactive"),
    "inactive.atNextBase": session("inactive", base=B2),
    "editing.fromInactive.unchanged": session("editing", pending=pend("a", ORIG["a"], "inactive")),
    "editing.fromInactive.changed": session("editing", pending=pend("a", DRAFT["a"], "inactive")),
    "editing.fromReviewing.unchanged": session("editing", accepted=["b"], pending=pend("a", ORIG["a"], "reviewing"), report=CLEAN),
    "editing.fromReviewing.changed": session("editing", accepted=["b"], pending=pend("a", DRAFT["a"], "reviewing"), report=CLEAN),
    "editing.fromReviewing.unchanged.reportLost": session("editing", accepted=["b"], pending=pend("a", ORIG["a"], "reviewing")),
    "editing.fromReviewing.revertsOnlyDraft": session("editing", accepted=["a"], pending=pend("a", ORIG["a"], "reviewing"), report=CLEAN),
    "editing.atNextBase": session("editing", pending=pend("a", DRAFT["a"], "inactive"), base=B2),
    "replaying": session("replaying", accepted=["a", "b"], progress={"done": 1, "total": 3}),
    "replaying.atNextBase": session("replaying", accepted=["a", "b"], base=B2),
    "reviewing.ready": session("reviewing", accepted=["a", "b"], report=CLEAN),
    "reviewing.blocking": session("reviewing", accepted=["a", "b"], report=BLOCKING),
    "reviewing.noReport": session("reviewing", accepted=["a", "b"], fault="replay.targetMissing"),
    "reviewing.empty": session("reviewing"),
    "reviewing.atNextBase": session("reviewing", accepted=["a", "b"], report=CLEAN, base=B2),
    "reviewing.cancelled": session("reviewing", accepted=["a", "b"], fault=CANCELLED),
    "reviewing.faultAfterReport": session("reviewing", accepted=["a", "b"], report=CLEAN, fault="vcs.rejected"),
    "choosing": session("choosing", accepted=["a", "b"], report=CLEAN),
    "choosing.atNextBase": session("choosing", accepted=["a", "b"], report=CLEAN, base=B2),
    "finalizing": session("finalizing", accepted=["a", "b"], report=CLEAN),
    "finalizing.atNextBase": session("finalizing", accepted=["a", "b"], report=CLEAN, base=B2),
}


def gen_event(kind, generation=G, **fields):
    return {"type": kind, "generation": generation, **fields}


EVENTS = {
    "begin": {"type": "begin", "target": tgt("c", POS["c"]), "original": ORIG["c"]},
    "draft": gen_event("draft", replacement=inp("0a0f")),
    "withdraw": gen_event("withdraw"),
    "accept": gen_event("accept"),
    "discard": gen_event("discard"),
    "replayProgressed": gen_event("replayProgressed", done=2, total=3),
    "replayCompleted": gen_event("replayCompleted", report=CLEAN),
    "replayCancelled": gen_event("replayCancelled"),
    "replayFaulted": gen_event("replayFaulted", code="replay.targetMissing"),
    "rerun": gen_event("rerun"),
    "requestFinalize": gen_event("requestFinalize"),
    "chooseOverwrite": gen_event("choose", choice={"kind": "overwrite"}),
    "chooseAlternative": gen_event("choose", choice={"kind": "alternative", "name": "Edited history"}),
    "back": gen_event("back"),
    "finalized": gen_event("finalized"),
    "finalizeFaulted": gen_event("finalizeFaulted", code="vcs.rejected"),
    "baseMoved": {"type": "baseMoved", "base": B2, "positions": [tgt(key, value) for key, value in MOVED.items()]},
    "exit": {"type": "exit"},
}


def ok(to, effects=(), generation="keep", session_id="keep"):
    return {"to": to, "effects": list(effects), "generation": generation, "session": session_id}


def no(code):
    return {"rejection": f"timeTravel.{code}"}


REPLAY = ok("replaying", ["startReplay"], "increment")
CLOSE = ok("inactive", ["close"], "increment")

LAW = {
    "inactive": {
        "begin": [("always", "inactive", ok("editing", ["showPreview"], "increment", "next"))],
        "baseMoved": [("newBase", "inactive", ok("inactive")), ("sameBase", "inactive.atNextBase", no("stale"))],
        "exit": [("always", "inactive", no("illegal"))],
    },
    "editing": {
        "begin": [("pendingUnchanged", "editing.fromReviewing.unchanged", ok("editing", ["showPreview"], "increment")), ("pendingChanged", "editing.fromReviewing.changed", no("blocked"))],
        "draft": [("always", "editing.fromInactive.unchanged", ok("editing", ["showPreview"]))],
        "withdraw": [("always", "editing.fromInactive.unchanged", ok("editing", ["showPreview"]))],
        "accept": [
            ("unchangedReturnInactive", "editing.fromInactive.unchanged", CLOSE),
            ("unchangedReturnReviewing", "editing.fromReviewing.unchanged", ok("reviewing")),
            ("unchangedReturnReplaying", "editing.fromReviewing.unchanged.reportLost", REPLAY),
            ("changed", "editing.fromReviewing.changed", REPLAY),
            ("changedToEmpty", "editing.fromReviewing.revertsOnlyDraft", ok("reviewing")),
        ],
        "discard": [
            ("returnInactive", "editing.fromInactive.changed", CLOSE),
            ("returnReviewing", "editing.fromReviewing.changed", ok("reviewing")),
            ("returnReplaying", "editing.fromReviewing.unchanged.reportLost", REPLAY),
        ],
        "baseMoved": [("newBase", "editing.fromInactive.changed", ok("editing", ["showPreview"])), ("sameBase", "editing.atNextBase", no("stale"))],
        "exit": [("always", "editing.fromReviewing.changed", CLOSE)],
    },
    "replaying": {
        "replayProgressed": [("always", "replaying", ok("replaying"))],
        "replayCompleted": [("always", "replaying", ok("reviewing"))],
        "replayCancelled": [("always", "replaying", ok("reviewing"))],
        "replayFaulted": [("validCode", "replaying", ok("reviewing")), ("invalidCode", "replaying", no("illegal"), gen_event("replayFaulted", code=""))],
        "baseMoved": [("newBase", "replaying", REPLAY), ("sameBase", "replaying.atNextBase", no("stale"))],
        "exit": [("always", "replaying", ok("inactive", ["cancelReplay", "close"], "increment"))],
    },
    "reviewing": {
        "begin": [("always", "reviewing.ready", ok("editing", ["showPreview"], "increment"))],
        "requestFinalize": [
            ("ready", "reviewing.ready", ok("choosing", ["openFinalizePrompt"])),
            ("acceptedEmpty", "reviewing.empty", no("empty")),
            ("reportMissing", "reviewing.noReport", no("blocked")),
            ("reportBlocking", "reviewing.blocking", no("blocked")),
        ],
        "rerun": [("needsReplay", "reviewing.noReport", REPLAY), ("acceptedEmpty", "reviewing.empty", no("empty")), ("reportCurrent", "reviewing.ready", no("illegal"))],
        "baseMoved": [("newBase", "reviewing.ready", REPLAY), ("newBaseEmpty", "reviewing.empty", ok("reviewing")), ("sameBase", "reviewing.atNextBase", no("stale"))],
        "exit": [("always", "reviewing.ready", CLOSE)],
    },
    "choosing": {
        "chooseOverwrite": [("always", "choosing", ok("finalizing", ["commitOverwrite"], "increment"))],
        "chooseAlternative": [
            ("validName", "choosing", ok("finalizing", ["commitAlternative"], "increment")),
            ("invalidName", "choosing", no("illegal"), gen_event("choose", choice={"kind": "alternative", "name": " \u00a0"})),
        ],
        "back": [("always", "choosing", ok("reviewing"))],
        "baseMoved": [("newBase", "choosing", REPLAY), ("sameBase", "choosing.atNextBase", no("stale"))],
        "exit": [("always", "choosing", CLOSE)],
    },
    "finalizing": {
        "finalized": [("always", "finalizing", CLOSE)],
        "finalizeFaulted": [("validCode", "finalizing", REPLAY), ("invalidCode", "finalizing", no("illegal"), gen_event("finalizeFaulted", code="vcs rejected"))],
        "baseMoved": [("newBase", "finalizing", ok("finalizing")), ("sameBase", "finalizing.atNextBase", no("stale"))],
        "exit": [("always", "finalizing", no("blocked"))],
    },
}

DEFAULT_CONTEXT = {"inactive": "inactive", "editing": "editing.fromReviewing.changed", "replaying": "replaying", "reviewing": "reviewing.ready", "choosing": "choosing", "finalizing": "finalizing"}


def matrix():
    rows = []
    for stage in STAGES:
        for key in EVENT_KEYS:
            branches = LAW[stage].get(key, [("always", DEFAULT_CONTEXT[stage], no("illegal"))])
            for when, context, outcome_row, *override in branches:
                assert CONTEXTS[context]["stage"] == stage, (stage, key, context)
                rows.append({"from": stage, "event": key, "when": when, "context": context, **({"input": override[0]} if override else {}), **outcome_row})
    return rows


def cleared(generation, base=B1, id=ID):
    return session("inactive", generation=generation, base=base, id=id)


def show(mutation, replacement):
    return {"type": "showPreview", "target": mutation, "replacement": replacement}


def start(drafts, frm):
    return {"type": "startReplay", "drafts": [{"target": mutation, "replacement": replacement} for mutation, replacement in drafts], "from": frm}


INPUTS_AB = [{"target": "a", "replacement": DRAFT["a"]}, {"target": "b", "replacement": DRAFT["b"]}]


def transition(next_session, effects):
    return {"transition": {"session": next_session, "effects": effects}}


CASES = [
    {
        "name": "begin from inactive opens the next session with the original input as the draft",
        "session": CONTEXTS["inactive"],
        "event": EVENTS["begin"],
        "expect": transition(session("editing", pending=pend("c", ORIG["c"], "inactive"), generation=G + 1, id=ID + 1), [show("c", ORIG["c"])]),
    },
    {
        "name": "begin on an accepted target starts from its accepted draft and keeps the report",
        "session": CONTEXTS["reviewing.ready"],
        "event": {"type": "begin", "target": tgt("a", 2), "original": ORIG["a"]},
        "expect": transition(session("editing", accepted=["a", "b"], pending=pend("a", DRAFT["a"], "reviewing"), report=CLEAN, generation=G + 1), [show("a", DRAFT["a"])]),
    },
    {
        "name": "switching target while the draft is unchanged keeps the session and its return stage",
        "session": CONTEXTS["editing.fromInactive.unchanged"],
        "event": EVENTS["begin"],
        "expect": transition(session("editing", pending=pend("c", ORIG["c"], "inactive"), generation=G + 1), [show("c", ORIG["c"])]),
    },
    {
        "name": "switching target with a changed draft is blocked",
        "session": CONTEXTS["editing.fromInactive.changed"],
        "event": EVENTS["begin"],
        "expect": {"rejection": "timeTravel.blocked"},
    },
    {
        "name": "a draft replaces the pending replacement and previews it",
        "session": CONTEXTS["editing.fromInactive.unchanged"],
        "event": EVENTS["draft"],
        "expect": transition(session("editing", pending=pend("a", inp("0a0f"), "inactive")), [show("a", inp("0a0f"))]),
    },
    {
        "name": "withdraw drafts a withdrawal",
        "session": CONTEXTS["editing.fromInactive.unchanged"],
        "event": EVENTS["withdraw"],
        "expect": transition(session("editing", pending=pend("a", WITHDRAWN, "inactive")), [show("a", WITHDRAWN)]),
    },
    {
        "name": "accept inserts the draft in history order and replays from the earliest accepted target",
        "session": CONTEXTS["editing.fromReviewing.changed"],
        "event": EVENTS["accept"],
        "expect": transition(session("replaying", accepted=["a", "b"], generation=G + 1), [start([("a", DRAFT["a"]), ("b", DRAFT["b"])], "a")]),
    },
    {
        "name": "an accepted withdrawal is replayed like any draft",
        "session": session("editing", pending=pend("a", WITHDRAWN, "inactive")),
        "event": EVENTS["accept"],
        "expect": transition(session("replaying", accepted=[draft("a", WITHDRAWN)], generation=G + 1), [start([("a", WITHDRAWN)], "a")]),
    },
    {
        "name": "accepting the original removes the accepted draft; nothing left means no replay",
        "session": CONTEXTS["editing.fromReviewing.revertsOnlyDraft"],
        "event": EVENTS["accept"],
        "expect": transition(session("reviewing"), []),
    },
    {
        "name": "accepting the original of one of two drafts replays from the remaining one",
        "session": session("editing", accepted=["a", "b"], pending=pend("a", ORIG["a"], "reviewing"), report=CLEAN),
        "event": EVENTS["accept"],
        "expect": transition(session("replaying", accepted=["b"], generation=G + 1), [start([("b", DRAFT["b"])], "b")]),
    },
    {
        "name": "accepting an unchanged draft from reviewing returns to the kept report",
        "session": CONTEXTS["editing.fromReviewing.unchanged"],
        "event": EVENTS["accept"],
        "expect": transition(session("reviewing", accepted=["b"], report=CLEAN), []),
    },
    {
        "name": "discard after the base moved during editing replays the accepted drafts",
        "session": CONTEXTS["editing.fromReviewing.unchanged.reportLost"],
        "event": EVENTS["discard"],
        "expect": transition(session("replaying", accepted=["b"], generation=G + 1), [start([("b", DRAFT["b"])], "b")]),
    },
    {
        "name": "a stale draft is a silent no-op",
        "session": CONTEXTS["editing.fromInactive.changed"],
        "event": gen_event("draft", generation=G - 1, replacement=inp("0a0f")),
        "expect": {"rejection": "timeTravel.stale"},
    },
    {
        "name": "a stale replay completion is a silent no-op",
        "session": CONTEXTS["replaying"],
        "event": gen_event("replayCompleted", generation=G - 1, report=CLEAN),
        "expect": {"rejection": "timeTravel.stale"},
    },
    {
        "name": "a future generation is stale too, even where the event is illegal",
        "session": CONTEXTS["inactive"],
        "event": gen_event("accept", generation=G + 1),
        "expect": {"rejection": "timeTravel.stale"},
    },
    {
        "name": "progress is recorded while replaying",
        "session": CONTEXTS["replaying"],
        "event": EVENTS["replayProgressed"],
        "expect": transition(session("replaying", accepted=["a", "b"], progress={"done": 2, "total": 3}), []),
    },
    {
        "name": "a completed replay installs its report and drops progress",
        "session": CONTEXTS["replaying"],
        "event": EVENTS["replayCompleted"],
        "expect": transition(session("reviewing", accepted=["a", "b"], report=CLEAN), []),
    },
    {
        "name": "a faulted replay keeps the drafts and records the fault",
        "session": CONTEXTS["replaying"],
        "event": EVENTS["replayFaulted"],
        "expect": transition(session("reviewing", accepted=["a", "b"], fault="replay.targetMissing"), []),
    },
    {
        "name": "a warning-only report opens the finalize prompt",
        "session": CONTEXTS["reviewing.ready"],
        "event": EVENTS["requestFinalize"],
        "expect": transition(session("choosing", accepted=["a", "b"], report=CLEAN), [{"type": "openFinalizePrompt"}]),
    },
    {
        "name": "choosing overwrite commits an unscoped supersede of the drafts in history order",
        "session": CONTEXTS["choosing"],
        "event": EVENTS["chooseOverwrite"],
        "expect": transition(session("finalizing", accepted=["a", "b"], report=CLEAN, generation=G + 1), [{"type": "commitOverwrite", "inputs": INPUTS_AB}]),
    },
    {
        "name": "choosing a new alternative commits a named branch with the drafts",
        "session": CONTEXTS["choosing"],
        "event": EVENTS["chooseAlternative"],
        "expect": transition(session("finalizing", accepted=["a", "b"], report=CLEAN, generation=G + 1), [{"type": "commitAlternative", "name": "Edited history", "inputs": INPUTS_AB}]),
    },
    {
        "name": "a finalize fault revalidates on the current base and keeps the fault",
        "session": CONTEXTS["finalizing"],
        "event": EVENTS["finalizeFaulted"],
        "expect": transition(session("replaying", accepted=["a", "b"], fault="vcs.rejected", generation=G + 1), [start([("a", DRAFT["a"]), ("b", DRAFT["b"])], "a")]),
    },
    {
        "name": "a finalized commit closes the session",
        "session": CONTEXTS["finalizing"],
        "event": EVENTS["finalized"],
        "expect": transition(cleared(G + 1), [{"type": "close"}]),
    },
    {
        "name": "a base move re-resolves positions, reorders the drafts and restarts the replay",
        "session": CONTEXTS["reviewing.ready"],
        "event": {"type": "baseMoved", "base": B2, "positions": [tgt("a", 7), tgt("b", 6)]},
        "expect": transition(session("replaying", accepted=[draft("b", position=6), draft("a", position=7)], base=B2, generation=G + 1), [start([("b", DRAFT["b"]), ("a", DRAFT["a"])], "b")]),
    },
    {
        "name": "a base move while editing drops the kept report and re-previews",
        "session": CONTEXTS["editing.fromReviewing.changed"],
        "event": EVENTS["baseMoved"],
        "expect": transition(session("editing", accepted=[draft("b", position=6)], pending=pend("a", DRAFT["a"], "reviewing", position=3), base=B2), [show("a", DRAFT["a"])]),
    },
    {
        "name": "a base move while inactive only records the base",
        "session": CONTEXTS["inactive"],
        "event": EVENTS["baseMoved"],
        "expect": transition(session("inactive", base=B2), []),
    },
    {
        "name": "a base move while finalizing only records the base",
        "session": CONTEXTS["finalizing"],
        "event": EVENTS["baseMoved"],
        "expect": transition(session("finalizing", accepted=[draft("a", position=3), draft("b", position=6)], report=CLEAN, base=B2), []),
    },
    {
        "name": "a base move to the current base is stale",
        "session": CONTEXTS["reviewing.atNextBase"],
        "event": EVENTS["baseMoved"],
        "expect": {"rejection": "timeTravel.stale"},
    },
    {
        "name": "exit while replaying cancels the replay and closes, keeping the session id",
        "session": CONTEXTS["replaying"],
        "event": EVENTS["exit"],
        "expect": transition(cleared(G + 1), [{"type": "cancelReplay"}, {"type": "close"}]),
    },
    {
        "name": "exit while finalizing is blocked",
        "session": CONTEXTS["finalizing"],
        "event": EVENTS["exit"],
        "expect": {"rejection": "timeTravel.blocked"},
    },
    {
        "name": "a cancelled replay keeps the drafts and records the cancelled fault",
        "session": CONTEXTS["replaying"],
        "event": EVENTS["replayCancelled"],
        "expect": transition(session("reviewing", accepted=["a", "b"], fault=CANCELLED), []),
    },
    {
        "name": "rerun after a cancelled replay starts the replay again and clears the fault",
        "session": CONTEXTS["reviewing.cancelled"],
        "event": EVENTS["rerun"],
        "expect": transition(session("replaying", accepted=["a", "b"], generation=G + 1), [start([("a", DRAFT["a"]), ("b", DRAFT["b"])], "a")]),
    },
    {
        "name": "rerun after a failed finalize revalidates although a report exists",
        "session": CONTEXTS["reviewing.faultAfterReport"],
        "event": EVENTS["rerun"],
        "expect": transition(session("replaying", accepted=["a", "b"], generation=G + 1), [start([("a", DRAFT["a"]), ("b", DRAFT["b"])], "a")]),
    },
    {
        "name": "rerun while replaying is illegal",
        "session": CONTEXTS["replaying"],
        "event": EVENTS["rerun"],
        "expect": {"rejection": "timeTravel.illegal"},
    },
    {
        "name": "reverting the only draft after a faulted replay clears the stale fault",
        "session": session("editing", accepted=["a"], pending=pend("a", ORIG["a"], "reviewing"), fault="replay.targetMissing"),
        "event": EVENTS["accept"],
        "expect": transition(session("reviewing"), []),
    },
    {
        "name": "an empty replay fault code is illegal",
        "session": CONTEXTS["replaying"],
        "event": gen_event("replayFaulted", code=""),
        "expect": {"rejection": "timeTravel.illegal"},
    },
    {
        "name": "a fault code with whitespace is illegal",
        "session": CONTEXTS["finalizing"],
        "event": gen_event("finalizeFaulted", code="vcs rejected"),
        "expect": {"rejection": "timeTravel.illegal"},
    },
    {
        "name": "a fault code over 256 bytes is illegal",
        "session": CONTEXTS["replaying"],
        "event": gen_event("replayFaulted", code="x" * (TEXT_MAX_BYTES + 1)),
        "expect": {"rejection": "timeTravel.illegal"},
    },
    {
        "name": "a fault code of exactly 256 bytes is recorded",
        "session": CONTEXTS["replaying"],
        "event": gen_event("replayFaulted", code="x" * TEXT_MAX_BYTES),
        "expect": transition(session("reviewing", accepted=["a", "b"], fault="x" * TEXT_MAX_BYTES), []),
    },
    {
        "name": "a malformed fault code is illegal even outside its stage",
        "session": CONTEXTS["inactive"],
        "event": gen_event("replayFaulted", code=""),
        "expect": {"rejection": "timeTravel.illegal"},
    },
    {
        "name": "an empty alternative name is illegal",
        "session": CONTEXTS["choosing"],
        "event": gen_event("choose", choice={"kind": "alternative", "name": ""}),
        "expect": {"rejection": "timeTravel.illegal"},
    },
    {
        "name": "an alternative name of only Unicode white space is illegal",
        "session": CONTEXTS["choosing"],
        "event": gen_event("choose", choice={"kind": "alternative", "name": "\u00a0\u0085\u2003\t"}),
        "expect": {"rejection": "timeTravel.illegal"},
    },
    {
        "name": "an alternative name over 256 UTF-8 bytes is illegal",
        "session": CONTEXTS["choosing"],
        "event": gen_event("choose", choice={"kind": "alternative", "name": "ä" * 129}),
        "expect": {"rejection": "timeTravel.illegal"},
    },
    {
        "name": "an alternative name of exactly 256 UTF-8 bytes is committed",
        "session": CONTEXTS["choosing"],
        "event": gen_event("choose", choice={"kind": "alternative", "name": "ä" * 128}),
        "expect": transition(session("finalizing", accepted=["a", "b"], report=CLEAN, generation=G + 1), [{"type": "commitAlternative", "name": "ä" * 128, "inputs": INPUTS_AB}]),
    },
    {
        "name": "a padded alternative name is committed verbatim, never trimmed",
        "session": CONTEXTS["choosing"],
        "event": gen_event("choose", choice={"kind": "alternative", "name": " Edited history "}),
        "expect": transition(session("finalizing", accepted=["a", "b"], report=CLEAN, generation=G + 1), [{"type": "commitAlternative", "name": " Edited history ", "inputs": INPUTS_AB}]),
    },
]

INITIAL = session("inactive", generation=0, id=0)


def step(event, stage=None, effects=(), rejection=None):
    return {"event": event, "expect": {"rejection": rejection} if rejection else {"stage": stage, "effects": list(effects)}}


def begin(mutation):
    return {"type": "begin", "target": tgt(mutation, POS[mutation]), "original": ORIG[mutation]}


def at(kind, generation, **fields):
    return gen_event(kind, generation=generation, **fields)


SCENARIOS = [
    {
        "name": "edit, accept, replay and finalize as overwrite",
        "initial": INITIAL,
        "steps": [
            step(begin("a"), "editing", ["showPreview"]),
            step(at("draft", 1, replacement=DRAFT["a"]), "editing", ["showPreview"]),
            step(at("accept", 1), "replaying", ["startReplay"]),
            step(at("replayProgressed", 2, done=1, total=3), "replaying"),
            step(at("replayCompleted", 2, report=CLEAN), "reviewing"),
            step(at("requestFinalize", 2), "choosing", ["openFinalizePrompt"]),
            step(at("choose", 2, choice={"kind": "overwrite"}), "finalizing", ["commitOverwrite"]),
            step(at("finalized", 3), "inactive", ["close"]),
        ],
        "final": cleared(4, id=1),
    },
    {
        "name": "a blocking report is resolved by withdrawing the failing mutation, then finalized as a new alternative",
        "initial": INITIAL,
        "steps": [
            step(begin("a"), "editing", ["showPreview"]),
            step(at("draft", 1, replacement=DRAFT["a"]), "editing", ["showPreview"]),
            step(at("accept", 1), "replaying", ["startReplay"]),
            step(at("replayCompleted", 2, report=BLOCKING), "reviewing"),
            step(at("requestFinalize", 2), rejection="timeTravel.blocked"),
            step(begin("b"), "editing", ["showPreview"]),
            step(at("withdraw", 3), "editing", ["showPreview"]),
            step(at("accept", 3), "replaying", ["startReplay"]),
            step(at("replayCompleted", 4, report=CLEAN), "reviewing"),
            step(at("requestFinalize", 4), "choosing", ["openFinalizePrompt"]),
            step(at("back", 4), "reviewing"),
            step(at("requestFinalize", 4), "choosing", ["openFinalizePrompt"]),
            step(at("choose", 4, choice={"kind": "alternative", "name": "Edited history"}), "finalizing", ["commitAlternative"]),
            step(at("finalized", 5), "inactive", ["close"]),
        ],
        "final": cleared(6, id=1),
    },
    {
        "name": "a base move restarts the replay and fences the results of the old one",
        "initial": INITIAL,
        "steps": [
            step(begin("a"), "editing", ["showPreview"]),
            step(at("draft", 1, replacement=DRAFT["a"]), "editing", ["showPreview"]),
            step(at("accept", 1), "replaying", ["startReplay"]),
            step(at("replayProgressed", 2, done=1, total=3), "replaying"),
            step({"type": "baseMoved", "base": B2, "positions": [tgt("a", 3)]}, "replaying", ["startReplay"]),
            step(at("replayProgressed", 2, done=2, total=3), rejection="timeTravel.stale"),
            step(at("replayCompleted", 2, report=CLEAN), rejection="timeTravel.stale"),
            step({"type": "baseMoved", "base": B2, "positions": [tgt("a", 3)]}, rejection="timeTravel.stale"),
            step(at("replayCompleted", 3, report=CLEAN), "reviewing"),
            step({"type": "exit"}, "inactive", ["close"]),
        ],
        "final": cleared(4, base=B2, id=1),
    },
    {
        "name": "discarding a fresh edit leaves time travel",
        "initial": INITIAL,
        "steps": [
            step(begin("c"), "editing", ["showPreview"]),
            step(at("draft", 1, replacement=DRAFT["c"]), "editing", ["showPreview"]),
            step(at("discard", 1), "inactive", ["close"]),
            step({"type": "exit"}, rejection="timeTravel.illegal"),
        ],
        "final": cleared(2, id=1),
    },
    {
        "name": "a changed draft blocks switching target until it is reverted",
        "initial": INITIAL,
        "steps": [
            step(begin("a"), "editing", ["showPreview"]),
            step(at("draft", 1, replacement=DRAFT["a"]), "editing", ["showPreview"]),
            step(begin("b"), rejection="timeTravel.blocked"),
            step(at("draft", 1, replacement=ORIG["a"]), "editing", ["showPreview"]),
            step(begin("b"), "editing", ["showPreview"]),
            step(at("discard", 2), "inactive", ["close"]),
        ],
        "final": cleared(3, id=1),
    },
    {
        "name": "reverting the only draft leaves nothing to finalize",
        "initial": INITIAL,
        "steps": [
            step(begin("a"), "editing", ["showPreview"]),
            step(at("draft", 1, replacement=DRAFT["a"]), "editing", ["showPreview"]),
            step(at("accept", 1), "replaying", ["startReplay"]),
            step(at("replayCompleted", 2, report=CLEAN), "reviewing"),
            step(begin("a"), "editing", ["showPreview"]),
            step(at("draft", 3, replacement=ORIG["a"]), "editing", ["showPreview"]),
            step(at("accept", 3), "reviewing"),
            step(at("requestFinalize", 3), rejection="timeTravel.empty"),
            step({"type": "exit"}, "inactive", ["close"]),
        ],
        "final": cleared(4, id=1),
    },
    {
        "name": "a failed finalize revalidates and can be retried",
        "initial": INITIAL,
        "steps": [
            step(begin("a"), "editing", ["showPreview"]),
            step(at("draft", 1, replacement=DRAFT["a"]), "editing", ["showPreview"]),
            step(at("accept", 1), "replaying", ["startReplay"]),
            step(at("replayCompleted", 2, report=CLEAN), "reviewing"),
            step(at("requestFinalize", 2), "choosing", ["openFinalizePrompt"]),
            step(at("choose", 2, choice={"kind": "overwrite"}), "finalizing", ["commitOverwrite"]),
            step({"type": "exit"}, rejection="timeTravel.blocked"),
            step(at("finalizeFaulted", 3, code="vcs.rejected"), "replaying", ["startReplay"]),
            step(at("replayCompleted", 4, report=CLEAN), "reviewing"),
            step(at("requestFinalize", 4), "choosing", ["openFinalizePrompt"]),
            step(at("choose", 4, choice={"kind": "overwrite"}), "finalizing", ["commitOverwrite"]),
            step(at("finalized", 5), "inactive", ["close"]),
        ],
        "final": cleared(6, id=1),
    },
    {
        "name": "a cancelled replay is rerun when the next edit is discarded",
        "initial": INITIAL,
        "steps": [
            step(begin("a"), "editing", ["showPreview"]),
            step(at("draft", 1, replacement=DRAFT["a"]), "editing", ["showPreview"]),
            step(at("accept", 1), "replaying", ["startReplay"]),
            step(at("replayCancelled", 2), "reviewing"),
            step(at("requestFinalize", 2), rejection="timeTravel.blocked"),
            step(begin("b"), "editing", ["showPreview"]),
            step(at("discard", 3), "replaying", ["startReplay"]),
            step(at("replayCompleted", 4, report=CLEAN), "reviewing"),
            step(at("requestFinalize", 4), "choosing", ["openFinalizePrompt"]),
        ],
        "final": session("choosing", accepted=["a"], report=CLEAN, generation=4, id=1),
    },
    {
        "name": "a cancelled replay is rerun and finalized",
        "initial": INITIAL,
        "steps": [
            step(begin("a"), "editing", ["showPreview"]),
            step(at("draft", 1, replacement=DRAFT["a"]), "editing", ["showPreview"]),
            step(at("accept", 1), "replaying", ["startReplay"]),
            step(at("replayCancelled", 2), "reviewing"),
            step(at("requestFinalize", 2), rejection="timeTravel.blocked"),
            step(at("rerun", 2), "replaying", ["startReplay"]),
            step(at("replayCompleted", 3, report=CLEAN), "reviewing"),
            step(at("requestFinalize", 3), "choosing", ["openFinalizePrompt"]),
            step(at("choose", 3, choice={"kind": "overwrite"}), "finalizing", ["commitOverwrite"]),
            step(at("finalized", 4), "inactive", ["close"]),
        ],
        "final": cleared(5, id=1),
    },
    {
        "name": "a faulted replay is rerun once, then a current report makes rerun illegal",
        "initial": INITIAL,
        "steps": [
            step(begin("a"), "editing", ["showPreview"]),
            step(at("draft", 1, replacement=DRAFT["a"]), "editing", ["showPreview"]),
            step(at("accept", 1), "replaying", ["startReplay"]),
            step(at("replayFaulted", 2, code="replay.targetMissing"), "reviewing"),
            step(at("rerun", 1), rejection="timeTravel.stale"),
            step(at("rerun", 2), "replaying", ["startReplay"]),
            step(at("replayCompleted", 3, report=CLEAN), "reviewing"),
            step(at("rerun", 3), rejection="timeTravel.illegal"),
            step({"type": "exit"}, "inactive", ["close"]),
        ],
        "final": cleared(4, id=1),
    },
]

INVARIANTS = [
    ("pending-iff-editing", "A pending draft exists exactly while the stage is editing.", session("editing")),
    ("return-stage-inactive-or-reviewing", "A pending draft returns to inactive or reviewing.", session("editing", pending=pend("a", ORIG["a"], "choosing"))),
    ("return-inactive-means-nothing-accepted", "A pending draft that returns to inactive was begun with nothing accepted.", session("editing", accepted=["b"], pending=pend("a", ORIG["a"], "inactive"))),
    ("inactive-holds-nothing", "An inactive session holds no drafts, report, progress or fault.", session("inactive", accepted=["a"])),
    ("work-needs-accepted-drafts", "Replaying, choosing and finalizing always have accepted drafts.", session("replaying")),
    ("progress-only-while-replaying", "Progress exists only while replaying.", session("reviewing", accepted=["a"], report=CLEAN, progress={"done": 1, "total": 2})),
    ("replaying-has-no-report", "A running replay has no report yet.", session("replaying", accepted=["a"], report=CLEAN)),
    ("finalize-needs-a-clean-report", "Choosing and finalizing hold a report without Error or Fatal outcomes.", session("choosing", accepted=["a"], report=BLOCKING)),
    ("accepted-in-history-order", "Accepted drafts are ordered by applied position, then mutation id.", session("reviewing", accepted=["b", "a"])),
    ("accepted-targets-unique", "Each mutation has at most one accepted draft.", session("reviewing", accepted=[draft("a"), draft("a", position=3)])),
    ("fault-needs-drafts", "A fault is recorded only while drafts are accepted.", session("reviewing", fault="replay.targetMissing")),
]

REVIEWS = {
    "reviewing.ready": "ready",
    "reviewing.blocking": "blocked",
    "reviewing.noReport": "needsReplay",
    "reviewing.empty": "noChanges",
    "reviewing.atNextBase": "ready",
    "reviewing.cancelled": "needsReplay",
    "reviewing.faultAfterReport": "ready",
}

LABELS = [
    ("stageInactive", "Not editing history", "Verlauf wird nicht bearbeitet"),
    ("stageEditing", "Editing a mutation", "Mutation wird bearbeitet"),
    ("stageReplaying", "Replaying later mutations", "Spätere Mutationen werden neu angewendet"),
    ("stageReviewing", "Reviewing the edited history", "Bearbeiteter Verlauf wird geprüft"),
    ("stageChoosing", "Choose how to finalize", "Art des Abschlusses wählen"),
    ("stageFinalizing", "Finalizing the history edit", "Verlaufsbearbeitung wird abgeschlossen"),
    ("refusalIllegal", "Not possible right now", "Derzeit nicht möglich"),
    ("refusalStale", "Outdated request ignored", "Veraltete Anfrage ignoriert"),
    ("refusalBlocked", "Blocked: resolve the pending change or the errors first", "Blockiert: zuerst die offene Änderung oder die Fehler auflösen"),
    ("refusalEmpty", "Nothing to finalize: no accepted changes", "Nichts abzuschließen: keine übernommenen Änderungen"),
    ("frozen", "Editing is paused while history is being edited", "Bearbeiten ist pausiert, solange der Verlauf bearbeitet wird"),
    ("choiceOverwrite", "Overwrite history", "Verlauf überschreiben"),
    ("choiceOverwriteDescription", "Replaces the inputs in every alternative that contains these mutations", "Ersetzt die Eingaben in jeder Alternative, die diese Mutationen enthält"),
    ("choiceAlternative", "New alternative", "Neue Alternative"),
    ("choiceAlternativeDescription", "Keeps the original history and continues in a new alternative", "Behält den ursprünglichen Verlauf und arbeitet in einer neuen Alternative weiter"),
    ("alternativeNameDefault", "Edited history", "Bearbeiteter Verlauf"),
    ("noChanges", "No changes: showing the current history", "Keine Änderungen: aktueller Verlauf wird angezeigt"),
    ("needsReplay", "Replay needed: later mutations are not checked yet", "Neu anwenden nötig: spätere Mutationen sind noch nicht geprüft"),
    ("reportBlocking", "Errors must be fixed or withdrawn before finalizing", "Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden"),
    ("readyToFinalize", "Ready to finalize", "Bereit zum Abschließen"),
    ("replayCancelled", "Replay cancelled", "Neu anwenden abgebrochen"),
    ("actionRerun", "Replay again", "Erneut anwenden"),
    ("replayProgressValueText", "Replaying {done} of {total} mutations", "{done} von {total} Mutationen werden neu angewendet"),
    ("refusalBusy", "History editing is busy: finish the running tool or the other history edit first", "Verlaufsbearbeitung beschäftigt: zuerst das laufende Werkzeug oder die andere Verlaufsbearbeitung abschließen"),
    ("refusalUnknownMutation", "This mutation is no longer in the history", "Diese Mutation ist nicht mehr im Verlauf"),
    ("refusalNotEditable", "The inputs of this mutation cannot be edited", "Die Eingaben dieser Mutation können nicht bearbeitet werden"),
    ("refusalUnknownInput", "This input does not exist in the mutation", "Diese Eingabe gibt es in der Mutation nicht"),
    ("refusalInvalidInput", "Invalid value: the input keeps its previous value", "Ungültiger Wert: Die Eingabe behält ihren bisherigen Wert"),
    ("refusalNoSelection", "Nothing suitable is selected for this input", "Für diese Eingabe ist nichts Passendes ausgewählt"),
    ("refusalNameRequired", "Name the new alternative", "Einen Namen für die neue Alternative eingeben"),
    ("refusalNameInvalid", "Invalid alternative name: use 1 to 256 characters", "Ungültiger Name der Alternative: 1 bis 256 Zeichen verwenden"),
    ("refusalSchemaUnavailable", "The input schema of this mutation is unavailable", "Das Eingabeschema dieser Mutation ist nicht verfügbar"),
    ("replayFaulted", "Replay failed: later mutations could not be checked", "Erneutes Anwenden fehlgeschlagen: Spätere Mutationen konnten nicht geprüft werden"),
    ("commitFailed", "Finalizing failed: the history is unchanged", "Abschließen fehlgeschlagen: Der Verlauf ist unverändert"),
    ("outcomeIntroduced", "New since this edit", "Neu durch diese Bearbeitung"),
    ("refusalMemberGone", "The part this history edit targets was closed", "Der Teil, den diese Verlaufsbearbeitung betrifft, wurde geschlossen"),
]

CODE_LABELS = [
    ("timeTravel.frozen", "frozen"),
    ("timeTravel.illegal", "refusalIllegal"),
    ("timeTravel.stale", "refusalStale"),
    ("timeTravel.blocked", "refusalBlocked"),
    ("timeTravel.empty", "refusalEmpty"),
    ("timeTravel.cancelled", "replayCancelled"),
    ("timeTravel.busy", "refusalBusy"),
    ("timeTravel.unknown-mutation", "refusalUnknownMutation"),
    ("timeTravel.not-editable", "refusalNotEditable"),
    ("timeTravel.unknown-input", "refusalUnknownInput"),
    ("timeTravel.invalid-input", "refusalInvalidInput"),
    ("timeTravel.no-selection", "refusalNoSelection"),
    ("timeTravel.name-required", "refusalNameRequired"),
    ("timeTravel.name-invalid", "refusalNameInvalid"),
    ("timeTravel.schema-unavailable", "refusalSchemaUnavailable"),
    ("timeTravel.replay-faulted", "replayFaulted"),
    ("timeTravel.commit-failed", "commitFailed"),
    ("timeTravel.member-gone", "refusalMemberGone"),
]

FIXTURE = {
    "schema": "semio.framework.time-travel.lifecycle-law.v1",
    "stages": STAGES,
    "eventKeys": EVENT_KEYS,
    "effectKinds": EFFECT_KINDS,
    "refusals": REFUSALS,
    "guards": GUARDS,
    "reviewKinds": REVIEW_KINDS,
    "frozenCode": "timeTravel.frozen",
    "cancelledCode": CANCELLED,
    "limits": {"textMaxBytes": TEXT_MAX_BYTES},
    "contexts": CONTEXTS,
    "events": EVENTS,
    "reviews": [{"context": name, "review": REVIEWS.get(name)} for name in CONTEXTS],
    "matrix": matrix(),
    "cases": CASES,
    "scenarios": SCENARIOS,
    "invariants": [{"id": id, "statement": statement, "violation": violation} for id, statement, violation in INVARIANTS],
    "labels": [{"key": key, "en": en, "de": de} for key, en, de in LABELS],
    "codeLabels": [{"code": code, "key": key} for code, key in CODE_LABELS],
}

OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(FIXTURE, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(f"wrote {OUT.relative_to(ROOT)}: {len(FIXTURE['matrix'])} matrix rows, {len(CASES)} cases, {len(SCENARIOS)} scenarios")
