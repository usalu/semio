"""📨️ Independent Python oracle of the Bun bundle data stream (standard library only): it encodes file sets into canonical JSON-line frames with 32 KiB base64 chunks and decodes a wire of frames back into file digests and bytes. Requests arrive as JSON on stdin."""
import base64, hashlib, json, sys

CHUNK = 32768


def content(spec):
    return (spec["repeat"] * spec["times"]).encode() if "repeat" in spec else spec["text"].encode()


def dumps(row):
    return json.dumps(row, separators=(",", ":"), ensure_ascii=False)


def encode(files, request):
    frames = []
    for file in files:
        data = content(file["content"])
        frames.append(dumps({"type": "file", "name": file["name"], "byteLength": len(data), "sha256": hashlib.sha256(data).hexdigest()}) + "\n")
        for offset in range(0, len(data), CHUNK):
            frames.append(dumps({"type": "chunk", "data": base64.b64encode(data[offset:offset + CHUNK]).decode()}) + "\n")
        frames.append('{"type":"end"}\n')
    frames.append(dumps({"type": "complete", "requestSha256": request}) + "\n")
    return frames


def decode(wire):
    files, current, done = [], None, False
    for line in wire.split("\n"):
        if not line:
            continue
        row = json.loads(line)
        if row["type"] == "file":
            current = {"name": row["name"], "bytes": b"", "declared": row["sha256"], "length": row["byteLength"]}
        elif row["type"] == "chunk":
            current["bytes"] += base64.b64decode(row["data"], validate=True)
        elif row["type"] == "end":
            assert hashlib.sha256(current["bytes"]).hexdigest() == current["declared"] and len(current["bytes"]) == current["length"]
            files.append({"name": current["name"], "byteLength": len(current["bytes"]), "sha256": current["declared"], "base64": base64.b64encode(current["bytes"]).decode()})
            current = None
        else:
            done = row["requestSha256"]
    return {"files": files, "complete": done}


def main():
    request = json.load(sys.stdin)
    op = request["op"]
    if op == "encode":
        print(json.dumps([encode(row["files"], row["request"]) for row in request["rows"]]))
    elif op == "decode":
        print(json.dumps([decode(wire) for wire in request["wires"]]))
    else:
        raise SystemExit("unknown op " + op)


main()
