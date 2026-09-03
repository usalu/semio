#!/usr/bin/env python3
"""🔨️ F1 — real before/after JSON snapshot pairs for every `mutation-without-fixture` breach owned
by this shard in `s.stdio.semio`'s seven subsets that already carry a COMPLETE, executed
`verified-native-second-implementation` Python reference (D3): each subset's own `🐍️.py` already
implements `apply_mutation` for EVERY kind its own `KINDS` tuple names — D3 registered only the
`set-snapshot` vector, leaving the rest of an already-complete second implementation undischarged.
Nothing here writes new mutation semantics: every payload below is built from the subset's own real
committed base document (cloning or swapping ALREADY-PRESENT, domain-valid content — never an
invented enum value or a fabricated id) and run through that subset's own, unmodified
`apply_mutation`.

Run: `uv run --group test python3 🔨️f1-semio-generate.py`
"""
import hashlib
import importlib.util
import json
import os
import sys
import types

REPO = os.getcwd()
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"


def stub_harness():
    mod = types.ModuleType("semio_repo_test")

    class Adapter:
        def __init__(self, lang):
            self.lang = lang

        def oracle(self, name, fn):
            return self

    class Context:
        pass

    class Outcome:
        def __init__(self, value, raw=None):
            self.value = value
            self.raw = raw

    def digest(b):
        return hashlib.sha256(b).hexdigest()

    mod.Adapter = Adapter
    mod.Context = Context
    mod.Outcome = Outcome
    mod.digest = digest
    sys.modules["semio_repo_test"] = mod


def load_module(py_path, name):
    spec = importlib.util.spec_from_file_location(name, py_path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def kebab_to_camel(kind):
    parts = kind.split("-")
    return parts[0] + "".join(w.capitalize() for w in parts[1:])


def sha256_of_bytes(b):
    return f"sha256:{hashlib.sha256(b).hexdigest()}", len(b)


def write_json_pair(fixture_dir, before, after):
    os.makedirs(fixture_dir, exist_ok=True)
    before_bytes = (json.dumps(before, indent=2, ensure_ascii=False, sort_keys=False) + "\n").encode("utf-8")
    after_bytes = (json.dumps(after, indent=2, ensure_ascii=False, sort_keys=False) + "\n").encode("utf-8")
    with open(os.path.join(fixture_dir, "before.json"), "wb") as f:
        f.write(before_bytes)
    with open(os.path.join(fixture_dir, "after.json"), "wb") as f:
        f.write(after_bytes)
    return before_bytes, after_bytes


# ═════════════════════════════ animation ═══════════════════════════════════
def animation_kind_mutation(doc, kind):
    timelines = doc["timelines"]
    t0 = timelines[0]
    channels = t0["channels"]
    c0 = channels[0]
    kf = c0["keyframes"]
    if kind == "insert-timeline":
        new_timeline = json.loads(json.dumps(t0))
        new_timeline["name"] = "inserted-timeline"
        return {"index": len(timelines), "timeline": new_timeline}
    if kind == "remove-timeline":
        return {"index": len(timelines) - 1}
    if kind == "set-timeline-name":
        return {"index": 0, "name": "renamed-walk"}
    if kind == "insert-channel":
        new_channel = json.loads(json.dumps(channels[-1]))
        return {"timelineIndex": 0, "index": len(channels), "channel": new_channel}
    if kind == "remove-channel":
        return {"timelineIndex": 0, "index": len(channels) - 1}
    if kind == "set-channel-target":
        return {"timelineIndex": 0, "index": 0, "target": {"node": "inserted-node", "property": {"kind": "translation"}}}
    if kind == "set-channel-interpolation":
        current = c0["interpolation"]
        alt = "step" if current != "step" else "linear"
        return {"timelineIndex": 0, "index": 0, "interpolation": alt}
    if kind == "insert-keyframe":
        new_kf = json.loads(json.dumps(kf[-1]))
        new_kf["t"] = kf[-1]["t"] + 10
        return {"timelineIndex": 0, "channelIndex": 0, "index": len(kf), "keyframe": new_kf}
    if kind == "remove-keyframe":
        return {"timelineIndex": 0, "channelIndex": 0, "index": len(kf) - 1}
    if kind == "set-keyframe-time":
        return {"timelineIndex": 0, "channelIndex": 0, "index": 0, "t": 42}
    if kind == "set-keyframe-value":
        return {"timelineIndex": 0, "channelIndex": 0, "index": 0, "value": {"kind": "vec3", "value": {"x": 9.0, "y": 9.0, "z": 9.0}}}
    raise AssertionError("no builder for animation kind %r" % kind)


# ═════════════════════════════ audio ════════════════════════════════════════
def audio_kind_mutation(doc, kind):
    channels = doc["channels"]
    tags = doc["tags"]
    if kind == "set-sample-rate":
        return {"sampleRate": 44100}
    if kind == "set-format":
        alt = "pcm8" if doc["format"] != "pcm8" else "pcm16"
        return {"format": alt}
    if kind == "insert-channel":
        new_channel = json.loads(json.dumps(channels[0]))
        return {"index": len(channels), "channel": new_channel}
    if kind == "remove-channel":
        return {"index": len(channels) - 1} if len(channels) > 1 else {"index": 0}
    if kind == "set-channel-samples":
        return {"index": 0, "samples": list(reversed(channels[0]["samples"]))}
    if kind == "insert-tag":
        new_tag = json.loads(json.dumps(tags[0]))
        new_tag["key"] = "F1MUTATED"
        return {"index": len(tags), "tag": new_tag}
    if kind == "remove-tag":
        return {"index": len(tags) - 1}
    if kind == "set-tag-value":
        return {"index": 0, "value": "F1 mutated value"}
    raise AssertionError("no builder for audio kind %r" % kind)


# ═════════════════════════════ flow ═════════════════════════════════════════
def flow_mutation(doc, kind):
    nodes = doc["nodes"]
    edges = doc["edges"]
    n0 = nodes[0]
    e0 = edges[0]
    if kind == "insert-node":
        node = json.loads(json.dumps(n0))
        node["id"] = "f1-inserted-node"
        return {"node": node}
    if kind == "remove-node":
        return {"id": nodes[-1]["id"]}
    if kind == "set-node-kind":
        alt_kind = next((n["kind"] for n in nodes if n["kind"] != n0["kind"]), n0["kind"] + "Renamed")
        return {"id": n0["id"], "kind": alt_kind}
    if kind == "set-node-label":
        return {"id": n0["id"], "label": "f1-relabelled"}
    if kind == "set-node-position":
        return {"id": n0["id"], "position": {"x": n0["position"]["x"] + 1000.0, "y": n0["position"]["y"] + 1000.0}}
    if kind == "set-node-param":
        key = n0["params"][0]["key"] if n0["params"] else "f1.mutated"
        return {"id": n0["id"], "key": key, "value": "f1-mutated-value"}
    if kind == "remove-node-param":
        holder = next((n for n in nodes if n["params"]), n0)
        return {"id": holder["id"], "key": holder["params"][0]["key"]}
    if kind == "insert-edge":
        edge = json.loads(json.dumps(e0))
        edge["id"] = "f1-inserted-edge"
        return {"edge": edge}
    if kind == "remove-edge":
        return {"id": edges[-1]["id"]}
    if kind == "set-edge-endpoints":
        return {"id": e0["id"], "from": e0["to"], "to": e0["from"]}
    if kind == "set-edge-kind":
        alt_kind = next((e["kind"] for e in edges if e["kind"] != e0["kind"]), "f1-mutated-kind")
        return {"id": e0["id"], "kind": alt_kind}
    raise AssertionError("no builder for flow kind %r" % kind)


# ═════════════════════════════ model ════════════════════════════════════════
def model_mutation(doc, kind):
    spatial = doc["spatial"]
    elements = doc["elements"]
    relations = doc["relations"]
    s0 = spatial[0]
    e0 = elements[0]
    r0 = relations[0]
    if kind == "insert-spatial-node":
        node = json.loads(json.dumps(s0))
        node["id"] = "f1-inserted-spatial"
        node["parentId"] = None
        return {"node": node}
    if kind == "remove-spatial-node":
        return {"id": spatial[-1]["id"]}
    if kind == "set-spatial-node":
        alt_kind = next((s["kind"] for s in spatial if s["kind"] != s0["kind"]), s0["kind"])
        return {"id": s0["id"], "kind": alt_kind}
    if kind == "insert-element":
        elem = json.loads(json.dumps(e0))
        elem["id"] = "f1-inserted-element"
        return {"element": elem}
    if kind == "remove-element":
        return {"id": elements[-1]["id"]}
    if kind == "set-element":
        alt_class = next((el["class"] for el in elements if el["class"] != e0["class"]), e0["class"])
        return {"id": e0["id"], "class": alt_class}
    if kind == "insert-relation":
        rel = json.loads(json.dumps(r0))
        rel["id"] = "f1-inserted-relation"
        return {"relation": rel}
    if kind == "remove-relation":
        return {"id": relations[-1]["id"]}
    if kind == "set-relation":
        alt_kind = next((r["kind"] for r in relations if r["kind"] != r0["kind"]), r0["kind"])
        return {"id": r0["id"], "kind": alt_kind}
    raise AssertionError("no builder for model kind %r" % kind)


# ═════════════════════════════ presentation ═════════════════════════════════
def presentation_mutation(doc, kind):
    slides = doc["slides"]
    masters = doc["masters"]
    layouts = doc["layouts"]
    s0 = slides[0]
    if kind == "insert-slide":
        slide = json.loads(json.dumps(s0))
        slide["id"] = "f1-inserted-slide"
        return {"index": len(slides), "slide": slide}
    if kind == "remove-slide":
        return {"index": len(slides) - 1}
    if kind == "set-slide-layout":
        alt_layout = next((lay["id"] for lay in layouts if lay["id"] != s0["layoutId"]), layouts[0]["id"])
        return {"index": 0, "layout_id": alt_layout}
    if kind == "set-slide-notes":
        return {"index": 0, "notes": [{"kind": "paragraph", "style_id": None, "runs": [{"text": "F1 mutated notes", "style": {"bold": False, "italic": False, "underline": False, "size": None, "font": None, "color": None, "link": None}}]}]}
    if kind == "insert-shape":
        shape = json.loads(json.dumps(s0["shapes"][0])) if s0["shapes"] else {"shapeKind": "textBox", "frame": {"origin": {"x": 0.0, "y": 0.0}, "width": 1.0, "height": 1.0}, "blocks": []}
        return {"slide_index": 0, "shape_index": len(s0["shapes"]), "shape": shape}
    if kind == "remove-shape":
        return {"slide_index": 0, "shape_index": len(s0["shapes"]) - 1}
    if kind == "set-shape-frame":
        frame = s0["shapes"][0]["frame"]
        return {"slide_index": 0, "shape_index": 0, "frame": {"origin": {"x": frame["origin"]["x"] + 1000.0, "y": frame["origin"]["y"] + 1000.0}, "width": frame["width"], "height": frame["height"]}}
    if kind == "set-text-box-blocks":
        text_box_index = next(i for i, sh in enumerate(s0["shapes"]) if sh["shapeKind"] == "textBox")
        return {"slide_index": 0, "shape_index": text_box_index, "blocks": [{"kind": "paragraph", "style_id": None, "runs": [{"text": "F1 mutated block", "style": {"bold": False, "italic": False, "underline": False, "size": None, "font": None, "color": None, "link": None}}]}]}
    if kind == "insert-master":
        master = json.loads(json.dumps(masters[0]))
        master["id"] = "f1-inserted-master"
        return {"master": master}
    if kind == "remove-master":
        return {"id": masters[0]["id"]} if len(masters) == 1 else {"id": masters[-1]["id"]}
    if kind == "insert-layout":
        layout = json.loads(json.dumps(layouts[0]))
        layout["id"] = "f1-inserted-layout"
        return {"layout": layout}
    if kind == "remove-layout":
        return {"id": layouts[-1]["id"]}
    if kind == "set-layout-master":
        alt_master_id = masters[0]["id"] + "-f1-mutated" if layouts[0]["masterId"] == masters[0]["id"] else masters[0]["id"]
        return {"id": layouts[0]["id"], "master_id": alt_master_id}
    raise AssertionError("no builder for presentation kind %r" % kind)


# ═════════════════════════════ value ════════════════════════════════════════
def value_mutation(doc, kind):
    nodes = doc["nodes"]
    if kind == "set-value":
        return {"path": [{"kind": "key", "key": "revision"}], "value": {"kind": "int", "lexeme": "999"}}
    if kind == "set-map-entry":
        return {"path": [], "key": "f1Mutated", "value": {"kind": "str", "value": "f1-mutated-value"}}
    if kind == "remove-map-entry":
        return {"path": [], "key": "revision"}
    if kind == "insert-list-item":
        models = next(e["value"] for e in doc["root"]["entries"] if e["key"] == "models")
        item = json.loads(json.dumps(models["items"][0]))
        return {"path": [{"kind": "key", "key": "models"}], "index": len(models["items"]), "value": item}
    if kind == "remove-list-item":
        models = next(e["value"] for e in doc["root"]["entries"] if e["key"] == "models")
        return {"path": [{"kind": "key", "key": "models"}], "index": len(models["items"]) - 1}
    if kind == "set-node":
        return {"id": {"value": nodes[0]["id"]["value"]}, "value": json.loads(json.dumps(nodes[1]["value"]))}
    if kind == "remove-node":
        return {"id": {"value": nodes[-1]["id"]["value"]}}
    raise AssertionError("no builder for value kind %r" % kind)


# ═════════════════════════════ video ════════════════════════════════════════
def video_mutation(doc, kind):
    streams = doc["streams"]
    s0 = streams[0]
    samples = s0["samples"]
    if kind == "insert-stream":
        stream = json.loads(json.dumps(streams[-1]))
        return {"index": len(streams), "stream": stream}
    if kind == "remove-stream":
        return {"index": len(streams) - 1}
    if kind == "set-stream-meta":
        return {"index": 0, "kind": s0["kind"], "codec": s0["codec"], "width": (s0.get("width") or 0) + 1, "height": (s0.get("height") or 0) + 1, "rate": s0["rate"]}
    if kind == "insert-sample":
        sample = json.loads(json.dumps(samples[0]))
        return {"streamIndex": 0, "index": len(samples), "sample": sample}
    if kind == "remove-sample":
        return {"streamIndex": 0, "index": len(samples) - 1}
    if kind == "set-sample-data":
        data = samples[0]["data"]
        return {"streamIndex": 0, "index": 0, "data": data[2:] + data[:2] if len(data) >= 2 else data}
    if kind == "set-sample-flags":
        return {"streamIndex": 0, "index": 0, "pts": samples[0]["pts"] + 1, "key": not samples[0]["key"]}
    raise AssertionError("no builder for video kind %r" % kind)


ENVELOPE_NESTED = {"animation", "audio", "video"}  # {"kind":..., "params":{...}}
ENVELOPE_FLAT = {"flow", "model", "presentation", "value"}  # {"mutation":<camel tag>, ...fields}

SUBSETS = {
    "animation": {
        "py": f"{ROOT}/✳️animation/🧪️tests/mutate-semio-animation/🐍️.py",
        "base_doc": f"{ROOT}/✳️animation/📚️examples/🚶️walk/🖼️assets/🗣️.dsl.semio",
        "kinds": ["insert-channel", "insert-keyframe", "insert-timeline", "remove-channel", "remove-keyframe", "remove-timeline", "set-channel-interpolation", "set-channel-target", "set-keyframe-time", "set-keyframe-value", "set-timeline-name"],
        "builder": animation_kind_mutation,
        "oracle": "semio-animation-python-independent",
        "capability": "semio-v1-animation-mutate",
        "subset_dir": "animation",
    },
    "audio": {
        "py": f"{ROOT}/✳️audio/🧪️tests/mutate-semio-audio/🐍️.py",
        "base_doc": f"{ROOT}/✳️audio/🧪️tests/mutate-semio-audio/🧫️fixtures/🧪️bauen-mit-bestand-ausschnitt/🗣️.dsl.semio",
        "kinds": ["insert-channel", "insert-tag", "remove-channel", "remove-tag", "set-channel-samples", "set-format", "set-sample-rate", "set-tag-value"],
        "builder": audio_kind_mutation,
        "oracle": "semio-audio-python-independent",
        "capability": "semio-v1-audio-mutate",
        "subset_dir": "audio",
    },
    "flow": {
        "py": f"{ROOT}/✳️flow/🧪️tests/mutate-semio-flow/🐍️.py",
        "base_doc": f"{ROOT}/✳️flow/🧪️tests/mutate-semio-flow/🧫️fixtures/🏗️nakagin-capsule-tower.dsl.semio",
        "kinds": ["insert-edge", "insert-node", "remove-edge", "remove-node", "remove-node-param", "set-edge-endpoints", "set-edge-kind", "set-node-kind", "set-node-label", "set-node-param", "set-node-position"],
        "builder": flow_mutation,
        "oracle": "semio-flow-python-independent",
        "capability": "semio-v1-flow-mutate",
        "subset_dir": "flow",
    },
    "model": {
        "py": f"{ROOT}/✳️model/🧪️tests/mutate-semio-model/🐍️.py",
        "base_doc": f"{ROOT}/✳️model/🧪️tests/mutate-semio-model/🧫️fixtures/🏗️nakagin-capsule-tower.dsl.semio",
        "kinds": ["insert-element", "insert-relation", "insert-spatial-node", "remove-element", "remove-relation", "remove-spatial-node", "set-element", "set-relation", "set-spatial-node"],
        "builder": model_mutation,
        "oracle": "semio-model-python-independent",
        "capability": "semio-v1-model-mutate",
        "subset_dir": "model",
    },
    "presentation": {
        "py": f"{ROOT}/✳️presentation/🧪️tests/mutate-semio-presentation/🐍️.py",
        "base_doc": f"{ROOT}/✳️presentation/🧪️tests/mutate-semio-presentation/🧫️fixtures/🧪️talk/🗣️.dsl.semio",
        "kinds": ["insert-layout", "insert-master", "insert-shape", "insert-slide", "remove-layout", "remove-master", "remove-shape", "remove-slide", "set-layout-master", "set-shape-frame", "set-slide-layout", "set-slide-notes", "set-text-box-blocks"],
        "builder": presentation_mutation,
        "oracle": "semio-presentation-python-independent",
        "capability": "semio-v1-presentation-mutate",
        "subset_dir": "presentation",
    },
    "value": {
        "py": f"{ROOT}/✳️value/🧪️tests/mutate-semio-value/🐍️.py",
        "base_doc": f"{ROOT}/✳️value/🧪️tests/mutate-semio-value/🧫️fixtures/🧪️hexagonal-cut-concrete-forest/🗣️.dsl.semio",
        "kinds": ["insert-list-item", "remove-list-item", "remove-map-entry", "remove-node", "set-map-entry", "set-node", "set-value"],
        "builder": value_mutation,
        "oracle": "semio-value-python-independent",
        "capability": "semio-v1-value-mutate",
        "subset_dir": "value",
    },
    "video": {
        "py": f"{ROOT}/✳️video/🧪️tests/mutate-semio-video/🐍️.py",
        "base_doc": f"{ROOT}/✳️video/🧪️tests/mutate-semio-video/🧫️fixtures/🧪️bauen-mit-bestand-ausschnitt/🗣️.dsl.semio",
        "kinds": ["insert-sample", "insert-stream", "remove-sample", "remove-stream", "set-sample-data", "set-sample-flags", "set-stream-meta"],
        "builder": video_mutation,
        "oracle": "semio-video-python-independent",
        "capability": "semio-v1-video-mutate",
        "subset_dir": "video",
    },
}


def main():
    stub_harness()
    fragments = {}
    for name, cfg in SUBSETS.items():
        mod = load_module(os.path.join(REPO, cfg["py"]), f"semio_gen_{name}")
        with open(os.path.join(REPO, cfg["base_doc"]), encoding="utf-8") as f:
            text = f.read()
        base_doc = mod.parse_dsl(text)

        subset_frags = []
        for kind in cfg["kinds"]:
            payload = cfg["builder"](base_doc, kind)
            if name in ENVELOPE_NESTED:
                mutation = {"kind": kind, "params": payload}
            else:
                mutation = dict(payload)
                mutation["mutation"] = kebab_to_camel(kind)
            after_doc = mod.apply_mutation(base_doc, mutation)

            fixture_id = f"{kind}-applied"
            fixture_dir = os.path.join(REPO, ROOT, f"✳️{cfg['subset_dir']}", "🧫️fixtures", fixture_id)
            before_bytes, after_bytes = write_json_pair(fixture_dir, base_doc, after_doc)
            before_sha, before_len = sha256_of_bytes(before_bytes)
            after_sha, after_len = sha256_of_bytes(after_bytes)
            subset_frags.append(
                {
                    "id": fixture_id,
                    "mutation": kind,
                    "mutation_payload": mutation,
                    "files": [
                        {"role": "expected-before-json", "path": f"../🧫️fixtures/{fixture_id}/before.json", "mediaType": "application/json", "sha256": before_sha, "bytes": before_len},
                        {"role": "expected-after-json", "path": f"../🧫️fixtures/{fixture_id}/after.json", "mediaType": "application/json", "sha256": after_sha, "bytes": after_len},
                    ],
                    "oracle": cfg["oracle"],
                }
            )
            print(f"{name}/{fixture_id}: before={before_len}B after={after_len}B")
        fragments[cfg["subset_dir"]] = subset_frags

    out = os.path.join(REPO, TICKET, "🗑️generated", "f1-semio-fragments.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w") as f:
        json.dump(fragments, f, indent=2, ensure_ascii=False)
    print("wrote", out)


if __name__ == "__main__":
    main()
