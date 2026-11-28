import json, math, sys
OUT = sys.argv[1]
def pose(p=(0,0,0), q=(0,0,0,1), s=(1,1,1)):
    return {"position": list(p), "quaternion": list(q), "scale": list(s)}
def zq(angle):
    return [0, 0, math.sin(angle / 2), math.cos(angle / 2)]
T = {"mode": "object", "ids": ["target"]}
def ev(kind, handle=None, **kw):
    e = {"kind": kind}
    if handle: e["handle"] = handle
    e.update(kw)
    return e
def move(action, **a):
    return {"action": action, "args": {"mode": "object", "ids": ["target"], **a}}
def case(name, mode, steps, guest=None):
    c = {"name": name, "live": True, "cumulative": True, "transformMode": mode, "targets": T, "steps": [{"event": e, "dispatch": d} for e, d in steps]}
    if guest: c["guest"] = guest
    return c
x = lambda v: pose(p=(v, 0, 0))
cases = [
    case("a cumulative drag streams the whole offset from the grab, the newest pose wins while a tick is in flight, and the release commits the whole offset", "move", [
        (ev("start", "moveX", pose=x(0)), None),
        (ev("drag", "moveX", pose=x(0.25)), move("translateSelection", dx=0.25, dy=0, dz=0, phase="stream")),
        (ev("drag", "moveX", pose=x(0.5)), None),
        (ev("drag", "moveX", pose=x(0.75)), None),
        (ev("settled"), move("translateSelection", dx=0.75, dy=0, dz=0, phase="stream")),
        (ev("settled"), None),
        (ev("release", "moveX", before=x(0), after=x(1)), move("translateSelection", dx=1, dy=0, dz=0, phase="commit")),
    ], {"edits": 1, "offset": [1, 0, 0]}),
    case("a cumulative release without any streamed tick commits the whole offset alone", "move", [
        (ev("start", "moveX", pose=x(0)), None),
        (ev("release", "moveX", before=x(0), after=x(0.5)), move("translateSelection", dx=0.5, dy=0, dz=0, phase="commit")),
    ], {"edits": 1, "offset": [0.5, 0, 0]}),
    case("a cumulative drag back to the grab pose streams the identity and commits it", "move", [
        (ev("start", "moveX", pose=x(0)), None),
        (ev("drag", "moveX", pose=x(0.5)), move("translateSelection", dx=0.5, dy=0, dz=0, phase="stream")),
        (ev("settled"), None),
        (ev("drag", "moveX", pose=x(0)), move("translateSelection", dx=0, dy=0, dz=0, phase="stream")),
        (ev("settled"), None),
        (ev("release", "moveX", before=x(0), after=x(0)), move("translateSelection", dx=0, dy=0, dz=0, phase="commit")),
    ]),
    case("a cumulative turn streams the whole angle from the grab", "rotate", [
        (ev("start", "rotateZ", pose=pose()), None),
        (ev("drag", "rotateZ", pose=pose(q=zq(0.5))), move("rotateSelection", ax=0, ay=0, az=1, angle=0.5, phase="stream")),
        (ev("settled"), None),
        (ev("drag", "rotateZ", pose=pose(q=zq(0.75))), move("rotateSelection", ax=0, ay=0, az=1, angle=0.75, phase="stream")),
        (ev("settled"), None),
        (ev("release", "rotateZ", before=pose(), after=pose(q=zq(1.0))), move("rotateSelection", ax=0, ay=0, az=1, angle=1.0, phase="commit")),
    ]),
    case("a cumulative scaling streams the whole factor from the grab", "scale", [
        (ev("start", "scaleUniform", pose=pose()), None),
        (ev("drag", "scaleUniform", pose=pose(s=(2, 2, 2))), move("scaleSelection", sx=2, sy=2, sz=2, phase="stream")),
        (ev("drag", "scaleUniform", pose=pose(s=(3, 3, 3))), None),
        (ev("settled"), move("scaleSelection", sx=3, sy=3, sz=3, phase="stream")),
        (ev("settled"), None),
        (ev("release", "scaleUniform", before=pose(), after=pose(s=(3, 3, 3))), move("scaleSelection", sx=3, sy=3, sz=3, phase="commit")),
    ]),
    case("a host blur after a streamed cumulative tick aborts with the verb's identity delta", "move", [
        (ev("start", "moveX", pose=x(0)), None),
        (ev("drag", "moveX", pose=x(0.5)), move("translateSelection", dx=0.5, dy=0, dz=0, phase="stream")),
        (ev("cancel", reason="blur"), move("translateSelection", dx=0, dy=0, dz=0, phase="abort", reason="blur")),
        (ev("settled"), None),
        (ev("drag", "moveX", pose=x(0.75)), None),
        (ev("release", "moveX", before=x(0), after=x(0.75)), None),
    ], {"edits": 0, "offset": [0, 0, 0]}),
]
why = ("The cumulative variant of the host side of the gumball tool protocol: a selection record with `gumballCumulative` makes `worldGumballStep` send every `stream` tick and the `commit` as the WHOLE offset from the grab pose "
       "(angle, factor) instead of the delta since the previous tick, and the identity while the pose is back on the grab, so each tick is idempotent and a tick the guest drops as stale costs the gesture nothing. "
       "A case's `cumulative`, `live`, `transformMode` and `targets` fill its `start` events. A guest that consumes cumulative ticks (fem 3d) replays a case's dispatches with the target ids replaced by one movable node of its own document, "
       "then again with each single stream dispatch dropped and again with every stream dispatch dropped, and must publish exactly `guest.edits` edits moving it by `guest.offset` every time. "
       "Guests that compose deltas replay `gumball-live-protocol.json` instead.")
json.dump({"why": why, "cases": cases}, open(OUT, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
open(OUT, "a", encoding="utf-8").write("\n")
