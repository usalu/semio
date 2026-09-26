"""🧫️ F2 — writes the language-agnostic fixture of `semio.io.stream-mux/v1` (codec vectors + hostiles, server and channel
scenarios). Canonical frame texts come from Python's own `json` encoder (fields in contract order, no whitespace), independent
of the TypeScript encoder the law checks. usage: python3 f2-stream-mux-fixture.py <out.json>"""
import json, sys

E = "0123456789abcdef"
OTHER = "fedcba9876543210"


def canon(frame):
    return json.dumps(frame, separators=(",", ":"), ensure_ascii=False)


def f(kind, **fields):
    frame = {"kind": kind}
    frame.update(fields)
    return frame


def open_(stream, route, key, resume, credit):
    return f("open", stream=stream, route=route, key=key, resume=resume, credit=credit)


def opened(stream, mode, seq, epoch=E):
    return f("opened", stream=stream, mode=mode, epoch=epoch, seq=seq)


def data(stream, seq, value):
    return f("data", stream=stream, seq=seq, data=value)


def end(stream, reason, detail=""):
    return f("end", stream=stream, reason=reason, detail=detail)


def progress(stream, done, total, note):
    return f("progress", stream=stream, done=done, total=total, note=note)


HELLO = f("hello", version=1, epoch=E, beatMs=10000)
PW = "plugin-modules.watch"
FOLDER = "backbone.folder"
JOB = "plugin-modules.activation"
SNAP = {"kind": "snapshot", "plugins": [{"pluginId": "alpha", "rebuiltAt": 1}]}
built = lambda plugin, at: {"kind": "built", "pluginId": plugin, "rebuiltAt": at}

vectors = [
    ("client", open_(1, PW, "", None, 32)),
    ("client", open_(7, FOLDER, "folder:///Users/ada/.semio/spaces/01a0/Grüße 🗂️", {"epoch": E, "seq": 42}, 0)),
    ("client", open_(2147483647, "a.b-c.d0.e", "x" * 1024, {"epoch": OTHER, "seq": 9007199254740991}, 256)),
    ("client", f("grant", stream=1, credit=1)),
    ("client", f("grant", stream=3, credit=256)),
    ("client", f("cancel", stream=1)),
    ("server", HELLO),
    ("server", opened(1, "fresh", 0)),
    ("server", opened(2, "resumed", 9007199254740991, OTHER)),
    ("server", data(1, 0, SNAP)),
    ("server", data(1, 5, None)),
    ("server", data(1, 6, [1, 2.5, "ß", True, False, {"nested": {"a": []}}])),
    ("server", data(1, 7, "Änderung ✏️")),
    ("server", progress(3, 1, 3, "materialize note")),
    ("server", progress(3, 0, None, "")),
    ("server", end(1, "done")),
    ("server", end(1, "cancelled")),
    ("server", end(1, "refused", "unknown-route")),
    ("server", end(1, "failed", "materialize failed: exit 1")),
    ("server", f("beat", at=0)),
    ("server", f("beat", at=1790441896668)),
]

hostiles = [
    {"side": "client", "text": "{\"kind\":\"cancel\",\"stream\":1", "refusal": "malformed-json", "field": None},
    {"side": "client", "text": "[1,2]", "refusal": "not-an-object", "field": None},
    {"side": "client", "text": "42", "refusal": "not-an-object", "field": None},
    {"side": "client", "text": "null", "refusal": "not-an-object", "field": None},
    {"side": "client", "text": canon({"stream": 1}), "refusal": "unknown-kind", "field": "kind"},
    {"side": "client", "text": canon(f("close", stream=1)), "refusal": "unknown-kind", "field": "kind"},
    {"side": "client", "text": canon(f("hello", version=1, epoch=E, beatMs=10000)), "refusal": "unknown-kind", "field": "kind"},
    {"side": "server", "text": canon(f("open", stream=1, route=PW, key="", resume=None, credit=1)), "refusal": "unknown-kind", "field": "kind"},
    {"side": "client", "text": canon(f("cancel", stream=1, reason="x")), "refusal": "unknown-field", "field": "reason"},
    {"side": "client", "text": canon(f("open", stream=1, route=PW, key="", credit=1)), "refusal": "missing-field", "field": "resume"},
    {"side": "client", "text": canon(f("grant", stream=1)), "refusal": "missing-field", "field": "credit"},
    {"side": "client", "text": canon(f("cancel", stream=0)), "refusal": "invalid-field", "field": "stream"},
    {"side": "client", "text": canon(f("cancel", stream=2147483648)), "refusal": "invalid-field", "field": "stream"},
    {"side": "client", "text": canon(f("cancel", stream=1.5)), "refusal": "invalid-field", "field": "stream"},
    {"side": "client", "text": canon(f("cancel", stream="1")), "refusal": "invalid-field", "field": "stream"},
    {"side": "client", "text": canon(open_(1, "Plugin.watch", "", None, 1)), "refusal": "invalid-field", "field": "route"},
    {"side": "client", "text": canon(open_(1, "a.b.c.d.e", "", None, 1)), "refusal": "invalid-field", "field": "route"},
    {"side": "client", "text": canon(open_(1, "watch.", "", None, 1)), "refusal": "invalid-field", "field": "route"},
    {"side": "client", "text": canon(open_(1, PW, "🗂️" * 513, None, 1)), "refusal": "invalid-field", "field": "key"},
    {"side": "client", "text": canon(open_(1, PW, "", {"epoch": "0123", "seq": 1}, 1)), "refusal": "invalid-field", "field": "resume"},
    {"side": "client", "text": canon(open_(1, PW, "", {"epoch": E, "seq": -1}, 1)), "refusal": "invalid-field", "field": "resume"},
    {"side": "client", "text": canon(open_(1, PW, "", {"epoch": E, "seq": 1, "extra": 0}, 1)), "refusal": "invalid-field", "field": "resume"},
    {"side": "client", "text": canon(open_(1, PW, "", None, 257)), "refusal": "invalid-field", "field": "credit"},
    {"side": "client", "text": canon(f("grant", stream=1, credit=0)), "refusal": "invalid-field", "field": "credit"},
    {"side": "server", "text": canon(f("hello", version=2, epoch=E, beatMs=10000)), "refusal": "invalid-field", "field": "version"},
    {"side": "server", "text": canon(opened(1, "stale", 0)), "refusal": "invalid-field", "field": "mode"},
    {"side": "server", "text": canon(f("opened", stream=1, mode="fresh", epoch="0123456789ABCDEF", seq=0)), "refusal": "invalid-field", "field": "epoch"},
    {"side": "server", "text": canon(f("data", stream=1, seq=0)), "refusal": "missing-field", "field": "data"},
    {"side": "server", "text": canon(f("data", stream=1, seq=9007199254740993, data=None)), "refusal": "invalid-field", "field": "seq"},
    {"side": "server", "text": canon(progress(1, 1, -1, "")), "refusal": "invalid-field", "field": "total"},
    {"side": "server", "text": canon(progress(1, 1, None, "n" * 513)), "refusal": "invalid-field", "field": "note"},
    {"side": "server", "text": canon(end(1, "timeout")), "refusal": "invalid-field", "field": "reason"},
    {"side": "server", "text": canon(f("beat", at=-1)), "refusal": "invalid-field", "field": "at"},
    {"side": "client", "fill": {"prefix": "{\"kind\":\"cancel\",\"stream\":1,\"pad\":\"", "char": "x", "count": 262144, "suffix": "\"}"}, "refusal": "too-large", "field": None},
    {"side": "server", "fill": {"prefix": "{\"kind\":\"data\",\"stream\":1,\"seq\":0,\"data\":\"", "char": "ü", "count": 131072, "suffix": "\"}"}, "refusal": "too-large", "field": None},
]

routes = {
    PW: {"snapshot": SNAP, "coalesceBy": "pluginId"},
    "plain.watch": {"snapshot": {"kind": "snapshot"}},
    FOLDER: {"admitKey": "^folder://", "coalesceAll": True, "source": True},
    JOB: {"job": True},
}

server_scenarios = [
    {
        "name": "fresh open sends the snapshot at the head, then live events in order",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, PW, "", None, 4)},
            {"publish": {"route": PW, "key": "", "data": built("alpha", 2)}},
            {"publish": {"route": PW, "key": "", "data": built("beta", 3)}},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), data(1, 0, SNAP), data(1, 1, built("alpha", 2)), data(1, 2, built("beta", 3))]}},
        ],
    },
    {
        "name": "credit bounds what is sent; a grant releases the queue",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, "plain.watch", "", None, 2)},
            {"publishMany": {"route": "plain.watch", "key": "", "count": 3}},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), data(1, 0, {"kind": "snapshot"}), data(1, 1, {"n": 1})]}},
            {"send": "a", "frame": f("grant", stream=1, credit=2)},
            {"expect": {"a": [data(1, 2, {"n": 2}), data(1, 3, {"n": 3})]}},
        ],
    },
    {
        "name": "queued events coalesce by the route key while the reader holds no credit",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, PW, "", None, 1)},
            {"publish": {"route": PW, "key": "", "data": built("alpha", 2)}},
            {"publish": {"route": PW, "key": "", "data": built("beta", 3)}},
            {"publish": {"route": PW, "key": "", "data": built("alpha", 4)}},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), data(1, 0, SNAP)]}},
            {"send": "a", "frame": f("grant", stream=1, credit=5)},
            {"expect": {"a": [data(1, 2, built("beta", 3)), data(1, 3, built("alpha", 4))]}},
        ],
    },
    {
        "name": "a queue that overflows its bound resynchronizes with a fresh snapshot once credit returns",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, "plain.watch", "", None, 1)},
            {"publishMany": {"route": "plain.watch", "key": "", "count": 65}},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), data(1, 0, {"kind": "snapshot"})]}},
            {"send": "a", "frame": f("grant", stream=1, credit=1)},
            {"expect": {"a": [opened(1, "fresh", 65), data(1, 65, {"kind": "snapshot"})]}},
        ],
    },
    {
        "name": "a reader that lost its link resumes after its last sequence number and gets exactly the missed events",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, PW, "", None, 8)},
            {"publish": {"route": PW, "key": "", "data": built("alpha", 2)}},
            {"drop": "a"},
            {"publish": {"route": PW, "key": "", "data": built("beta", 3)}},
            {"publish": {"route": PW, "key": "", "data": built("gamma", 4)}},
            {"connect": "b"},
            {"send": "b", "frame": open_(1, PW, "", {"epoch": E, "seq": 1}, 8)},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), data(1, 0, SNAP), data(1, 1, built("alpha", 2))], "b": [HELLO, opened(1, "resumed", 1), data(1, 2, built("beta", 3)), data(1, 3, built("gamma", 4))]}},
        ],
    },
    {
        "name": "a resume from another epoch starts fresh",
        "steps": [
            {"connect": "a"},
            {"publish": {"route": PW, "key": "", "data": built("alpha", 2)}},
            {"send": "a", "frame": open_(1, PW, "", {"epoch": OTHER, "seq": 1}, 4)},
            {"expect": {"a": [HELLO, opened(1, "fresh", 1), data(1, 1, SNAP)]}},
        ],
    },
    {
        "name": "after the resume grace the instance is gone and an event nobody read makes the resume fresh",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, PW, "", None, 4)},
            {"drop": "a"},
            {"advance": 60000},
            {"publish": {"route": PW, "key": "", "data": built("alpha", 2)}},
            {"connect": "b"},
            {"send": "b", "frame": open_(1, PW, "", {"epoch": E, "seq": 0}, 4)},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), data(1, 0, SNAP)], "b": [HELLO, opened(1, "fresh", 1), data(1, 1, SNAP)]}},
            {"census": {"channels": 1, "streams": 1, "instances": 1}},
        ],
    },
    {
        "name": "a resume older than the retained ring starts fresh",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, "plain.watch", "", None, 1)},
            {"drop": "a"},
            {"publishMany": {"route": "plain.watch", "key": "", "count": 130}},
            {"connect": "b"},
            {"send": "b", "frame": open_(1, "plain.watch", "", {"epoch": E, "seq": 1}, 2)},
            {"send": "b", "frame": open_(2, "plain.watch", "", {"epoch": E, "seq": 128}, 2)},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), data(1, 0, {"kind": "snapshot"})], "b": [HELLO, opened(1, "fresh", 130), data(1, 130, {"kind": "snapshot"}), opened(2, "resumed", 128), data(2, 129, {"n": 129}), data(2, 130, {"n": 130})]}},
        ],
    },
    {
        "name": "unknown routes and refused keys end refused; a duplicate stream id closes the channel",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, "no.such-route", "", None, 1)},
            {"send": "a", "frame": open_(2, FOLDER, "file:///etc", None, 1)},
            {"send": "a", "frame": open_(3, FOLDER, "folder:///a", None, 1)},
            {"send": "a", "frame": open_(3, FOLDER, "folder:///b", None, 1)},
            {"expect": {"a": [HELLO, end(1, "refused", "unknown-route"), end(2, "refused", "key-refused"), opened(3, "fresh", 0)]}},
            {"closed": {"a": [1008, "stream-mux: duplicate-stream"]}},
            {"census": {"channels": 0, "streams": 0, "instances": 1}},
        ],
    },
    {
        "name": "a malformed reader frame closes the channel with its refusal",
        "steps": [
            {"connect": "a"},
            {"sendText": "a", "text": "{\"kind\":\"open\"}"},
            {"expect": {"a": [HELLO]}},
            {"closed": {"a": [1008, "stream-mux: missing-field stream"]}},
        ],
    },
    {
        "name": "a job reports progress and ends done; a later reader within the grace sees the retained end without a rerun",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, JOB, "note", None, 4)},
            {"jobProgress": {"key": "note", "done": 1, "total": 3, "note": "materialize note"}},
            {"jobSettle": {"key": "note", "outcome": "done"}},
            {"send": "a", "frame": open_(2, JOB, "note", None, 4)},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), progress(1, 1, 3, "materialize note"), end(1, "done"), opened(2, "fresh", 0), progress(2, 1, 3, "materialize note"), end(2, "done")]}},
            {"jobRuns": {"note": 1}},
        ],
    },
    {
        "name": "a failed job ends failed; the next reader starts it again",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, JOB, "raster", None, 4)},
            {"jobSettle": {"key": "raster", "outcome": "failed", "detail": "materialize failed: exit 1"}},
            {"send": "a", "frame": open_(2, JOB, "raster", None, 4)},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), end(1, "failed", "materialize failed: exit 1"), opened(2, "fresh", 0)]}},
            {"jobRuns": {"raster": 2}},
        ],
    },
    {
        "name": "the last reader's cancel cancels the job",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, JOB, "cad", None, 4)},
            {"send": "a", "frame": f("cancel", stream=1)},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), end(1, "cancelled")]}},
            {"jobAborted": {"cad": True}},
        ],
    },
    {
        "name": "a job survives a lost link within the grace and the returning reader sees its latest progress",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, JOB, "flow", None, 4)},
            {"jobProgress": {"key": "flow", "done": 2, "total": 5, "note": "compile"}},
            {"drop": "a"},
            {"advance": 59999},
            {"connect": "b"},
            {"send": "b", "frame": open_(1, JOB, "flow", None, 4)},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), progress(1, 2, 5, "compile")], "b": [HELLO, opened(1, "fresh", 0), progress(1, 2, 5, "compile")]}},
            {"jobAborted": {"flow": False}},
            {"jobRuns": {"flow": 1}},
        ],
    },
    {
        "name": "a live source runs while its route instance lives and stops when the resume grace retires it",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, FOLDER, "folder:///a", None, 4)},
            {"sources": {"folder:///a": "live"}},
            {"sourceEmit": {"key": "folder:///a", "data": "changed"}},
            {"sourceEmit": {"key": "folder:///a", "data": "changed"}},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), data(1, 1, "changed"), data(1, 2, "changed")]}},
            {"drop": "a"},
            {"advance": 59999},
            {"sources": {"folder:///a": "live"}},
            {"advance": 1},
            {"sources": {"folder:///a": "stopped"}},
            {"census": {"channels": 0, "streams": 0, "instances": 0}},
        ],
    },
    {
        "name": "queued change notices of one folder collapse into one",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, FOLDER, "folder:///b", None, 1)},
            {"sourceEmit": {"key": "folder:///b", "data": "changed"}},
            {"sourceEmit": {"key": "folder:///b", "data": "changed"}},
            {"sourceEmit": {"key": "folder:///b", "data": "changed"}},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), data(1, 1, "changed")]}},
            {"send": "a", "frame": f("grant", stream=1, credit=4)},
            {"expect": {"a": [data(1, 3, "changed")]}},
        ],
    },
    {
        "name": "the server beats every beatMs",
        "steps": [
            {"connect": "a"},
            {"advance": 20000},
            {"expect": {"a": [HELLO, f("beat", at=10000), f("beat", at=20000)]}},
        ],
    },
    {
        "name": "a congested socket holds the queue until it drains",
        "steps": [
            {"connect": "a"},
            {"send": "a", "frame": open_(1, PW, "", None, 4)},
            {"expect": {"a": [HELLO, opened(1, "fresh", 0), data(1, 0, SNAP)]}},
            {"buffered": {"a": 1048577}},
            {"publish": {"route": PW, "key": "", "data": built("alpha", 2)}},
            {"advance": 20},
            {"expect": {"a": []}},
            {"buffered": {"a": 0}},
            {"advance": 20},
            {"expect": {"a": [data(1, 1, built("alpha", 2))]}},
        ],
    },
]

channel_scenarios = [
    {
        "name": "one link carries every stream and reopens them from their last sequence numbers after a loss",
        "random": 0,
        "steps": [
            {"open": {"sub": "p", "route": PW, "key": "", "credit": 4}},
            {"open": {"sub": "q", "route": FOLDER, "key": "folder:///a", "credit": 2}},
            {"expect": {"links": 1, "sent": []}},
            {"linkOpen": True},
            {"expect": {"sent": [open_(1, PW, "", None, 4), open_(2, FOLDER, "folder:///a", None, 2)]}},
            {"server": HELLO},
            {"server": opened(1, "fresh", 0)},
            {"server": data(1, 0, SNAP)},
            {"server": data(1, 1, built("alpha", 2))},
            {"server": opened(2, "fresh", 5)},
            {"expect": {"sent": [f("grant", stream=1, credit=2)], "events": {"p": [["opened", "fresh"], ["data", SNAP], ["data", built("alpha", 2)]], "q": [["opened", "fresh"]]}}},
            {"linkClosed": True},
            {"advance": 249},
            {"expect": {"links": 1}},
            {"advance": 1},
            {"expect": {"links": 2}},
            {"linkOpen": True},
            {"expect": {"sent": [open_(1, PW, "", {"epoch": E, "seq": 1}, 4), open_(2, FOLDER, "folder:///a", {"epoch": E, "seq": 5}, 2)]}},
            {"server": opened(1, "resumed", 1)},
            {"server": data(1, 2, built("beta", 3))},
            {"expect": {"events": {"p": [["opened", "resumed"], ["data", built("beta", 3)]], "q": []}}},
        ],
    },
    {
        "name": "a silent link is declared dead after deadAfterMs and reconnected",
        "random": 0,
        "steps": [
            {"open": {"sub": "p", "route": PW, "key": "", "credit": 4}},
            {"linkOpen": True},
            {"server": HELLO},
            {"advance": 24999},
            {"expect": {"links": 1, "linkClosed": False}},
            {"advance": 1},
            {"expect": {"linkClosed": True}},
            {"advance": 250},
            {"expect": {"links": 2}},
        ],
    },
    {
        "name": "a data handler that has not settled holds its credit",
        "random": 0,
        "steps": [
            {"open": {"sub": "p", "route": PW, "key": "", "credit": 2, "deferData": True}},
            {"linkOpen": True},
            {"server": opened(1, "fresh", 0)},
            {"server": data(1, 0, SNAP)},
            {"expect": {"sent": [open_(1, PW, "", None, 2)], "events": {"p": [["opened", "fresh"], ["data", SNAP]]}}},
            {"release": "p"},
            {"expect": {"sent": [f("grant", stream=1, credit=1)]}},
        ],
    },
    {
        "name": "server end closes the stream; a refused server frame closes the link",
        "random": 0,
        "steps": [
            {"open": {"sub": "p", "route": JOB, "key": "note", "credit": 4}},
            {"linkOpen": True},
            {"server": opened(1, "fresh", 0)},
            {"server": progress(1, 1, 3, "materialize note")},
            {"server": end(1, "done")},
            {"expect": {"events": {"p": [["opened", "fresh"], ["progress", 1, 3, "materialize note"], ["end", "done", ""]]}}},
            {"open": {"sub": "q", "route": PW, "key": "", "credit": 4}},
            {"serverText": "{\"kind\":\"data\"}"},
            {"expect": {"linkClosed": True}},
        ],
    },
    {
        "name": "the idle channel lingers, is reused within the linger, and closes after it",
        "random": 0,
        "steps": [
            {"open": {"sub": "p", "route": PW, "key": "", "credit": 4}},
            {"linkOpen": True},
            {"close": "p"},
            {"advance": 4999},
            {"open": {"sub": "r", "route": PW, "key": "", "credit": 4}},
            {"close": "r"},
            {"expect": {"links": 1, "linkClosed": False, "sent": [open_(1, PW, "", None, 4), f("cancel", stream=1), open_(2, PW, "", None, 4), f("cancel", stream=2)]}},
            {"advance": 5000},
            {"expect": {"links": 1, "linkClosed": True}},
        ],
    },
]

fixture = {
    "$schema": "semio.io.stream-mux.fixture/v1",
    "contract": "semio.io.stream-mux/v1",
    "epoch": E,
    "vectors": [{"side": side, "text": canon(frame), "frame": frame} for side, frame in vectors],
    "hostiles": hostiles,
    "routes": routes,
    "serverScenarios": server_scenarios,
    "channelScenarios": channel_scenarios,
}
out = json.dumps(fixture, indent=1, ensure_ascii=False) + "\n"
open(sys.argv[1], "w", encoding="utf-8").write(out)
print(f"vectors={len(vectors)} hostiles={len(hostiles)} server={len(server_scenarios)} channel={len(channel_scenarios)} bytes={len(out.encode())}")
