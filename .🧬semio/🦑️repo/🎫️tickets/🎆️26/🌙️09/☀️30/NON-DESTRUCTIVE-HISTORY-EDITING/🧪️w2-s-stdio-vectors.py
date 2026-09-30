#!/usr/bin/env python3
"""🧫️ W2-S stdio: makes every semio `mutate-*` specification vector carry a real `SemioXMutation` wire value.

- animation / video / audio vectors `{kind, params, before, after}` become `{kind, mutation, before, after}` whose `mutation` is
  the internally tagged wire value (`{"mutation": "<camelCaseVariant>", …leaf fields}`); their feature doc strings and
  Examples rows are rewritten the same way (`{"kind":"no-mutation","params":{}}` becomes the scenario sentinel
  `{"mutation":"noMutation"}` the cad/flow/model features already use). Video's case projection spells stream kinds as the
  grammar letters and sample bytes as hex; its wire value uses the `SemioVideoStreamKind` names and byte arrays.
- every `no-mutation` vector with a before-snapshot carries the identity `{"mutation": "setSnapshot", "snapshot": before}`.
- the two payload-only `noMutation` sentinels (presentation `🪞️no-mutation`, image `⏸️no-mutation`) are deleted: their
  baseline scenarios carry the sentinel as a doc string.

Idempotent; each file is re-read right before it is written.

    python3 🧪️w2-s-stdio-vectors.py [--dry-run]
"""
import json
import os
import re
import shutil
import sys

REPO = "/Users/ueli/Documents/semio"
SEMIO = REPO + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
DRY = "--dry-run" in sys.argv
KIND_NAMES = {"V": "video", "A": "audio", "S": "subtitle"}
INLINE = {
    "🌊️flow": "🌊️mutate-semio-flow",
    "📐️cad": "📐️mutate-semio-cad",
    "📑️document": "📃️mutate-semio-document",
    "🎞️animation": "🎞️mutate-semio-animation",
    "🎬️video": "🎥️mutate-semio-video",
    "🔊️audio": "🔊️mutate-semio-audio",
}
PARAMS = {"🎞️animation": "🎞️mutate-semio-animation", "🎬️video": "🎥️mutate-semio-video", "🔊️audio": "🔊️mutate-semio-audio"}
SEPARATE = {"🏛️model": "🏛️mutate-semio-model/⏸️no-mutation", "📽️presentation": "📽️mutate-semio-presentation/⏸️no-mutation"}
SENTINELS = {"📽️presentation": "📽️mutate-semio-presentation/🪞️no-mutation", "🖼️image": "🖼️mutate-semio-image/⏸️no-mutation"}


def camel(kind):
    head, *rest = kind.split("-")
    return head + "".join(part[:1].upper() + part[1:] for part in rest)


def wire_sample(sample):
    return {"pts": sample["pts"], "key": sample["key"], "data": list(bytes.fromhex(sample["data"]))}


def wire_stream(stream):
    return {**stream, "kind": KIND_NAMES[stream["kind"]], "samples": [wire_sample(sample) for sample in stream["samples"]]}


def wire_snapshot(subset, snapshot):
    return {**snapshot, "streams": [wire_stream(stream) for stream in snapshot["streams"]]} if subset == "🎬️video" else snapshot


def wire(subset, kind, params):
    """🦠️ The `SemioXMutation` wire value of one case-projection `{kind, params}` pair."""
    if kind == "no-mutation":
        return {"mutation": "noMutation"}
    args = dict(params)
    if subset == "🎬️video":
        if "snapshot" in args:
            args["snapshot"] = wire_snapshot(subset, args["snapshot"])
        if "stream" in args:
            args["stream"] = wire_stream(args["stream"])
        if "sample" in args:
            args["sample"] = wire_sample(args["sample"])
        if kind == "set-sample-data":
            args["data"] = list(bytes.fromhex(args["data"]))
        if kind == "set-stream-meta":
            args["kind"] = KIND_NAMES[args["kind"]]
    return {"mutation": camel(kind), **args}


def dump(value):
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


def write(path, before, after):
    print(("[dry] " if DRY else "") + os.path.relpath(path, REPO))
    if DRY:
        return
    if open(path, encoding="utf-8").read() != before:
        print("  changed while scanning; rerun")
        return
    open(path, "w", encoding="utf-8").write(after)


def vectors():
    for subset, case in INLINE.items():
        root = "%s/%s/🧫️fixtures/%s" % (SEMIO, subset, case)
        for name in sorted(os.listdir(root)):
            path = "%s/%s/🦠️mutation/🔣️.json" % (root, name)
            if not os.path.exists(path):
                continue
            text = open(path, encoding="utf-8").read()
            vector = json.loads(text)
            if "params" in vector:
                vector = {"kind": vector["kind"], "mutation": wire(subset, vector["kind"], vector["params"]), "before": vector["before"], "after": vector["after"]}
            if vector["kind"] == "no-mutation":
                vector["mutation"] = {"mutation": "setSnapshot", "snapshot": wire_snapshot(subset, vector["before"])}
            if dump(vector) != text:
                write(path, text, dump(vector))
    for subset, case in SEPARATE.items():
        root = "%s/%s/🧫️fixtures/%s" % (SEMIO, subset, case)
        path = root + "/🦠️mutation/🔣️.json"
        text = open(path, encoding="utf-8").read()
        identity = {"mutation": "setSnapshot", "snapshot": json.load(open(root + "/⬅️before/🔣️.json", encoding="utf-8"))}
        if dump(identity) != text:
            write(path, text, dump(identity))
    for subset, case in SENTINELS.items():
        root = "%s/%s/🧫️fixtures/%s" % (SEMIO, subset, case)
        if os.path.isdir(root):
            print(("[dry] " if DRY else "") + "delete " + os.path.relpath(root, REPO))
            if not DRY:
                shutil.rmtree(root)


def features():
    pattern = re.compile(r'\{"kind":"([a-z-]+)","params":(\{.*?\})\}(?=\s*(?:\||$))')
    for subset, case in PARAMS.items():
        directory = "%s/%s/🧪️tests/%s" % (SEMIO, subset, case)
        path = directory + "/🥒️.feature"
        text = open(path, encoding="utf-8").read()
        lines = text.split("\n")
        for index, line in enumerate(lines):
            match = pattern.search(line)
            if match is None:
                continue
            value = json.dumps(wire(subset, match.group(1), json.loads(match.group(2))), ensure_ascii=False, separators=(",", ":"))
            lines[index] = line[: match.start()] + value + line[match.end():]
        updated = "\n".join(lines)
        updated = realign(updated, {index for index, (a, b) in enumerate(zip(text.split("\n"), updated.split("\n"))) if a != b})
        if updated != text:
            write(path, text, updated)


def realign(text, touched):
    """📐️ Pads every Gherkin table block holding a `touched` line to one column width per column."""
    lines, out, block = text.split("\n"), [], []

    def flush():
        if not block:
            return
        if not any(index in touched for index, _ in block):
            out.extend(line for _, line in block)
        else:
            rows = [[cell.strip() for cell in line.strip()[1:-1].split("|")] for _, line in block]
            widths = [max(len(row[index]) for row in rows if index < len(row)) for index in range(max(len(row) for row in rows))]
            first = block[0][1]
            indent = first[: len(first) - len(first.lstrip())]
            out.extend(indent + "| " + " | ".join(cell.ljust(widths[index]) for index, cell in enumerate(row)) + " |" for row in rows)
        block.clear()

    for index, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("|") and stripped.endswith("|"):
            block.append((index, line))
            continue
        flush()
        out.append(line)
    flush()
    return "\n".join(out)


if __name__ == "__main__":
    vectors()
    features()
