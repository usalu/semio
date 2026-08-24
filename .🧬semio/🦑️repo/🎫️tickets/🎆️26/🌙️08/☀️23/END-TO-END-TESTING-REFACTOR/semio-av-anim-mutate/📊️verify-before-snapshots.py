"""🔎️ Independent check that every committed specification vector's BEFORE snapshot really is the
decoded content of the committed real artifact it claims to come from.

The decoder below is written from the committed DSL wire grammar alone (`📖️component.grammar.semio`
plus the `enc_*`/`dec_*` pairs in each subset's own snapshot facet), in a different language from the
implementation, so agreeing with it is evidence rather than a tautology. Ticket
26/08/23/END-TO-END-TESTING-REFACTOR.
"""
import io, json, struct, sys

ART = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
EX = ART + "/🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples"
TESTS = ART + "/🧪️tests"


def strip_brackets(s):
    assert s.startswith("[") and s.endswith("]"), s
    return s[1:-1]


def split_top(s, sep=","):
    out, depth, start = [], 0, 0
    for i, ch in enumerate(s):
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        elif ch == sep and depth == 0:
            out.append(s[start:i])
            start = i + 1
    out.append(s[start:])
    return [p for p in out] if s else []


def unhex_str(s):
    return bytes.fromhex(s).decode("utf-8")


def body(path):
    text = io.open(path, encoding="utf-8").read()
    lines = [l for l in text.splitlines() if l.strip()]
    return {l.split("=", 1)[0]: l.split("=", 1)[1] for l in lines if "=" in l}


def audio():
    f = body(EX + "/🎵️tone/🖼️assets/🗣️example.dsl.semio")
    channels = [{"samples": [struct.unpack(">f", bytes.fromhex(h))[0] for h in split_top(strip_brackets(c))]} for c in split_top(strip_brackets(f["channels"]))]
    tags = []
    for t in split_top(strip_brackets(f["tags"])):
        k, v = split_top(strip_brackets(t))
        tags.append({"key": unhex_str(k), "value": unhex_str(v)})
    return {"schema": unhex_str(f["schema"]), "sampleRate": int(f["sampleRate"]), "format": f["format"], "channels": channels, "tags": tags}


def video():
    f = body(EX + "/🎥️clip/🖼️assets/🗣️example.dsl.semio")
    streams = []
    for s in split_top(strip_brackets(f["streams"])):
        kind, codec, w, h, rate, samples = split_top(strip_brackets(s))
        num, den = split_top(strip_brackets(rate))
        entries = [e for e in split_top(strip_brackets(samples)) if e]
        parsed = []
        for e in entries:
            pts, key, data = split_top(strip_brackets(e))
            parsed.append({"pts": int(pts), "key": key == "1", "data": data})
        streams.append({"kind": kind, "codec": unhex_str(codec), "width": int(w), "height": int(h), "rate": {"num": int(num), "den": int(den)}, "samples": parsed})
    return {"schema": unhex_str(f["schema"]), "streams": streams}


PROP = {"t": "translation", "r": "rotation", "s": "scale", "w": "weights"}
INTERP = {"l": "linear", "s": "step", "c": "cubicSpline"}


def anim_value(s):
    tag, rest = s.split(":", 1)
    if tag == "S":
        return {"kind": "scalar", "value": float(rest)}
    if tag == "V":
        x, y, z = [float(v) for v in split_top(strip_brackets(rest))]
        return {"kind": "vec3", "value": {"x": x, "y": y, "z": z}}
    if tag == "Q":
        x, y, z, w = [float(v) for v in split_top(strip_brackets(rest))]
        return {"kind": "quat", "value": {"x": x, "y": y, "z": z, "w": w}}
    if tag == "W":
        return {"kind": "weights", "values": [float(v) for v in split_top(strip_brackets(rest))]}
    raise AssertionError(s)


def animation():
    f = body(EX + "/🚶️walk/🖼️assets/🗣️example.dsl.semio")
    timelines = []
    for t in split_top(strip_brackets(f["timelines"])):
        name_enc, channels_enc = split_top(strip_brackets(t))
        opt = split_top(strip_brackets(name_enc))
        name = None if opt == ["0"] else unhex_str(opt[1])
        channels = []
        for c in split_top(strip_brackets(channels_enc)):
            target_enc, interp, kfs = split_top(strip_brackets(c))
            node, prop = split_top(strip_brackets(target_enc))
            property = {"kind": "custom", "name": unhex_str(prop.split(":", 1)[1])} if prop.startswith("c:") else {"kind": PROP[prop]}
            keyframes = []
            for k in split_top(strip_brackets(kfs)):
                at, value = split_top(strip_brackets(k))
                keyframes.append({"t": float(at), "value": anim_value(value)})
            channels.append({"target": {"node": unhex_str(node), "property": property}, "interpolation": INTERP[interp], "keyframes": keyframes})
        timelines.append({"name": name, "channels": channels})
    return {"schema": unhex_str(f["schema"]), "timelines": timelines}


def check(case, decoded):
    import glob, os
    failures = 0
    files = sorted(glob.glob(TESTS + "/" + case + "/🧫️fixtures/*.json"))
    for path in files:
        vector = json.load(io.open(path, encoding="utf-8"))
        if vector["before"] != decoded:
            failures += 1
            print("MISMATCH", case, os.path.basename(path))
            print("  fixture :", json.dumps(vector["before"], ensure_ascii=False, sort_keys=True)[:400])
            print("  artifact:", json.dumps(decoded, ensure_ascii=False, sort_keys=True)[:400])
    print("%-24s %2d vector(s), %d before-snapshot mismatch(es)" % (case, len(files), failures))
    return failures


total = 0
total += check("mutate-semio-audio", audio())
total += check("mutate-semio-video", video())
total += check("mutate-semio-animation", animation())
print("TOTAL MISMATCHES", total)
sys.exit(1 if total else 0)
