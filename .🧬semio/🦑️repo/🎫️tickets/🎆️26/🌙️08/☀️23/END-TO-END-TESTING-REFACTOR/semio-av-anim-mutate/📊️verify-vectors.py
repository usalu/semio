"""🔎️ Typo guard over the committed specification vectors: replays every `(before, mutation)` pair
through the mutation semantics TRANSCRIBED from each subset's own Rust `Mutation::diff`/`inverse`
impl and checks (a) the result equals the committed `after`, and (b) applying the transcribed
inverse to that result restores `before`.

This is NOT independent evidence — the semantics are read off the implementation, so agreement is a
transcription check, not an oracle. Its job is to catch a hand-authoring slip in the committed
vectors while the Rust subject phase is blocked upstream (semio-framework-job, 6 errors).
Ticket 26/08/23/END-TO-END-TESTING-REFACTOR.
"""
import copy, glob, io, json, os, sys

TESTS = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests"


def audio_apply(s, kind, p):
    s = copy.deepcopy(s)
    if kind == "no-mutation":
        return s
    if kind == "set-snapshot":
        return copy.deepcopy(p["snapshot"])
    if kind == "set-sample-rate":
        s["sampleRate"] = p["sampleRate"]
    elif kind == "set-format":
        s["format"] = p["format"]
    elif kind == "insert-channel":
        s["channels"].insert(min(p["index"], len(s["channels"])), copy.deepcopy(p["channel"]))
    elif kind == "remove-channel":
        del s["channels"][p["index"]]
    elif kind == "set-channel-samples":
        s["channels"][p["index"]]["samples"] = list(p["samples"])
    elif kind == "insert-tag":
        s["tags"].insert(min(p["index"], len(s["tags"])), copy.deepcopy(p["tag"]))
    elif kind == "remove-tag":
        del s["tags"][p["index"]]
    elif kind == "set-tag-value":
        s["tags"][p["index"]]["value"] = p["value"]
    else:
        raise AssertionError(kind)
    return s


def audio_inverse(base, kind, p):
    if kind in ("no-mutation",):
        return ("no-mutation", {})
    if kind == "set-snapshot":
        return ("set-snapshot", {"snapshot": base})
    if kind == "set-sample-rate":
        return ("set-sample-rate", {"sampleRate": base["sampleRate"]})
    if kind == "set-format":
        return ("set-format", {"format": base["format"]})
    if kind == "insert-channel":
        return ("remove-channel", {"index": min(p["index"], len(base["channels"]))})
    if kind == "remove-channel":
        return ("insert-channel", {"index": p["index"], "channel": base["channels"][p["index"]]})
    if kind == "set-channel-samples":
        return ("set-channel-samples", {"index": p["index"], "samples": base["channels"][p["index"]]["samples"]})
    if kind == "insert-tag":
        return ("remove-tag", {"index": min(p["index"], len(base["tags"]))})
    if kind == "remove-tag":
        return ("insert-tag", {"index": p["index"], "tag": base["tags"][p["index"]]})
    if kind == "set-tag-value":
        return ("set-tag-value", {"index": p["index"], "value": base["tags"][p["index"]]["value"]})
    raise AssertionError(kind)


def video_apply(s, kind, p):
    s = copy.deepcopy(s)
    if kind == "no-mutation":
        return s
    if kind == "set-snapshot":
        return copy.deepcopy(p["snapshot"])
    if kind == "insert-stream":
        s["streams"].insert(p["index"], copy.deepcopy(p["stream"]))
    elif kind == "remove-stream":
        del s["streams"][p["index"]]
    elif kind == "set-stream-meta":
        st = s["streams"][p["index"]]
        st.update({"kind": p["kind"], "codec": p["codec"], "width": p["width"], "height": p["height"], "rate": copy.deepcopy(p["rate"])})
    elif kind == "insert-sample":
        s["streams"][p["streamIndex"]]["samples"].insert(p["index"], copy.deepcopy(p["sample"]))
    elif kind == "remove-sample":
        del s["streams"][p["streamIndex"]]["samples"][p["index"]]
    elif kind == "set-sample-data":
        s["streams"][p["streamIndex"]]["samples"][p["index"]]["data"] = p["data"]
    elif kind == "set-sample-flags":
        sm = s["streams"][p["streamIndex"]]["samples"][p["index"]]
        sm["pts"], sm["key"] = p["pts"], p["key"]
    else:
        raise AssertionError(kind)
    return s


def video_inverse(base, kind, p):
    if kind == "no-mutation":
        return ("no-mutation", {})
    if kind == "set-snapshot":
        return ("set-snapshot", {"snapshot": base})
    if kind == "insert-stream":
        return ("remove-stream", {"index": p["index"]})
    if kind == "remove-stream":
        return ("insert-stream", {"index": p["index"], "stream": base["streams"][p["index"]]})
    if kind == "set-stream-meta":
        st = base["streams"][p["index"]]
        return ("set-stream-meta", {"index": p["index"], "kind": st["kind"], "codec": st["codec"], "width": st["width"], "height": st["height"], "rate": st["rate"]})
    if kind == "insert-sample":
        return ("remove-sample", {"streamIndex": p["streamIndex"], "index": p["index"]})
    if kind == "remove-sample":
        return ("insert-sample", {"streamIndex": p["streamIndex"], "index": p["index"], "sample": base["streams"][p["streamIndex"]]["samples"][p["index"]]})
    if kind == "set-sample-data":
        return ("set-sample-data", {"streamIndex": p["streamIndex"], "index": p["index"], "data": base["streams"][p["streamIndex"]]["samples"][p["index"]]["data"]})
    if kind == "set-sample-flags":
        sm = base["streams"][p["streamIndex"]]["samples"][p["index"]]
        return ("set-sample-flags", {"streamIndex": p["streamIndex"], "index": p["index"], "pts": sm["pts"], "key": sm["key"]})
    raise AssertionError(kind)


def anim_apply(s, kind, p):
    s = copy.deepcopy(s)
    if kind == "no-mutation":
        return s
    if kind == "set-snapshot":
        return copy.deepcopy(p["snapshot"])
    T = s["timelines"]
    if kind == "insert-timeline":
        T.insert(p["index"], copy.deepcopy(p["timeline"]))
    elif kind == "remove-timeline":
        del T[p["index"]]
    elif kind == "set-timeline-name":
        T[p["index"]]["name"] = p["name"]
    elif kind == "insert-channel":
        T[p["timelineIndex"]]["channels"].insert(p["index"], copy.deepcopy(p["channel"]))
    elif kind == "remove-channel":
        del T[p["timelineIndex"]]["channels"][p["index"]]
    elif kind == "set-channel-target":
        T[p["timelineIndex"]]["channels"][p["index"]]["target"] = copy.deepcopy(p["target"])
    elif kind == "set-channel-interpolation":
        T[p["timelineIndex"]]["channels"][p["index"]]["interpolation"] = p["interpolation"]
    elif kind == "insert-keyframe":
        T[p["timelineIndex"]]["channels"][p["channelIndex"]]["keyframes"].insert(p["index"], copy.deepcopy(p["keyframe"]))
    elif kind == "remove-keyframe":
        del T[p["timelineIndex"]]["channels"][p["channelIndex"]]["keyframes"][p["index"]]
    elif kind == "set-keyframe-time":
        T[p["timelineIndex"]]["channels"][p["channelIndex"]]["keyframes"][p["index"]]["t"] = p["t"]
    elif kind == "set-keyframe-value":
        T[p["timelineIndex"]]["channels"][p["channelIndex"]]["keyframes"][p["index"]]["value"] = copy.deepcopy(p["value"])
    else:
        raise AssertionError(kind)
    return s


def anim_inverse(base, kind, p):
    T = base["timelines"]
    if kind == "no-mutation":
        return ("no-mutation", {})
    if kind == "set-snapshot":
        return ("set-snapshot", {"snapshot": base})
    if kind == "insert-timeline":
        return ("remove-timeline", {"index": p["index"]})
    if kind == "remove-timeline":
        return ("insert-timeline", {"index": p["index"], "timeline": T[p["index"]]})
    if kind == "set-timeline-name":
        return ("set-timeline-name", {"index": p["index"], "name": T[p["index"]]["name"]})
    if kind == "insert-channel":
        return ("remove-channel", {"timelineIndex": p["timelineIndex"], "index": p["index"]})
    if kind == "remove-channel":
        return ("insert-channel", {"timelineIndex": p["timelineIndex"], "index": p["index"], "channel": T[p["timelineIndex"]]["channels"][p["index"]]})
    if kind == "set-channel-target":
        return ("set-channel-target", {"timelineIndex": p["timelineIndex"], "index": p["index"], "target": T[p["timelineIndex"]]["channels"][p["index"]]["target"]})
    if kind == "set-channel-interpolation":
        return ("set-channel-interpolation", {"timelineIndex": p["timelineIndex"], "index": p["index"], "interpolation": T[p["timelineIndex"]]["channels"][p["index"]]["interpolation"]})
    if kind == "insert-keyframe":
        return ("remove-keyframe", {"timelineIndex": p["timelineIndex"], "channelIndex": p["channelIndex"], "index": p["index"]})
    kf = lambda: T[p["timelineIndex"]]["channels"][p["channelIndex"]]["keyframes"][p["index"]]
    if kind == "remove-keyframe":
        return ("insert-keyframe", {"timelineIndex": p["timelineIndex"], "channelIndex": p["channelIndex"], "index": p["index"], "keyframe": kf()})
    if kind == "set-keyframe-time":
        return ("set-keyframe-time", {"timelineIndex": p["timelineIndex"], "channelIndex": p["channelIndex"], "index": p["index"], "t": kf()["t"]})
    if kind == "set-keyframe-value":
        return ("set-keyframe-value", {"timelineIndex": p["timelineIndex"], "channelIndex": p["channelIndex"], "index": p["index"], "value": kf()["value"]})
    raise AssertionError(kind)


SUBSETS = [
    ("mutate-semio-audio", audio_apply, audio_inverse),
    ("mutate-semio-video", video_apply, video_inverse),
    ("mutate-semio-animation", anim_apply, anim_inverse),
]

total = 0
for case, apply_fn, inverse_fn in SUBSETS:
    forward = undo = 0
    files = sorted(glob.glob(TESTS + "/" + case + "/🧫️fixtures/*.json"))
    for path in files:
        v = json.load(io.open(path, encoding="utf-8"))
        got = apply_fn(v["before"], v["kind"], v["params"])
        if got != v["after"]:
            forward += 1
            print("FORWARD MISMATCH", case, os.path.basename(path))
            print("  expected:", json.dumps(v["after"], ensure_ascii=False)[:300])
            print("  got     :", json.dumps(got, ensure_ascii=False)[:300])
        ik, ip = inverse_fn(v["before"], v["kind"], v["params"])
        back = apply_fn(got, ik, ip)
        if back != v["before"]:
            undo += 1
            print("INVERSE MISMATCH", case, os.path.basename(path))
            print("  restored:", json.dumps(back, ensure_ascii=False)[:300])
    print("%-24s %2d vector(s), forward mismatches %d, inverse mismatches %d" % (case, len(files), forward, undo))
    total += forward + undo
print("TOTAL MISMATCHES", total)
sys.exit(1 if total else 0)
