"""🖐️ Finger and thumb offsets of the 15-bone hands of `standing-adult` and `walking-commuter` (right side authored, left mirrored):
flexion lives in `abductionDeg` of the proximal phalanx (its joint axis is turned by `twistDeg` ∓90) and in `angleDeg` of the middle and
distal phalanges."""

FINGERS = ("index", "middle", "ring", "little")


def hand(side, mcp, pip, dip, thumb=(0, 10, 0, 16, 0, 6), spread=(0, 0, 0, 0)):
    """✋️ Offsets of one hand: `mcp`, `pip`, `dip` per finger (index…little), `thumb` = (meta angle, meta abd, prox angle, prox abd,
    dist angle, dist abd), `spread` = proximal `angleDeg` per finger (side-to-side splay)."""
    s = 1 if side == "r" else -1
    out = {
        f"thumb-metacarpal-{side}": {"angleDeg": thumb[0], "abductionDeg": s * thumb[1], "twistDeg": -90 * s},
        f"thumb-proximal-{side}": {"angleDeg": thumb[2], "abductionDeg": s * thumb[3]},
        f"thumb-distal-{side}": {"angleDeg": thumb[4], "abductionDeg": s * thumb[5]},
    }
    for i, f in enumerate(FINGERS):
        out[f"{f}-proximal-{side}"] = {"angleDeg": spread[i], "abductionDeg": s * mcp[i], "twistDeg": -90 * s}
        out[f"{f}-middle-{side}"] = {"angleDeg": pip[i]}
        out[f"{f}-distal-{side}"] = {"angleDeg": dip[i]}
    return out


def both(**k):
    return {**hand("r", **k), **hand("l", **k)}


RELAXED = dict(mcp=(7, 8, 9, 10), pip=(15, 17, 19, 21), dip=(10, 11, 12, 12), thumb=(0, 9.8, -1, 16, -1.6, 6))
TYPING = dict(mcp=(24, 26, 27, 29), pip=(38, 42, 42, 40), dip=(18, 20, 20, 18), thumb=(4, 14, 8, 18, 6, 8))
PRESS = dict(mcp=(30, 26, 27, 29), pip=(46, 42, 42, 40), dip=(24, 20, 20, 18), thumb=(4, 14, 8, 18, 6, 8))
TRACKPAD = dict(mcp=(6, 30, 34, 36), pip=(12, 52, 56, 56), dip=(8, 26, 28, 26), thumb=(2, 16, 10, 20, 8, 8))
GRIP = dict(mcp=(22, 25, 26, 29), pip=(30, 34, 37, 39), dip=(12, 15, 18, 22), thumb=(0, 18, 14, 16, 8, 6))
FLAT = dict(mcp=(4, 5, 6, 7), pip=(6, 7, 8, 9), dip=(4, 4, 5, 5), thumb=(0, 12, 2, 14, 2, 6))
PINCH = dict(mcp=(28, 36, 40, 44), pip=(44, 56, 60, 60), dip=(22, 28, 30, 28), thumb=(10, 24, 18, 20, 12, 8))
