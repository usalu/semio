"""Execute the SPO1 frame vectors with Python, SQLite, and the authored Rust headers."""
import json
from pathlib import Path
import sqlite3
import struct
import subprocess
import sys

ticket = Path(__file__).parent
generated = ticket / "🗑️generated"
schema = json.loads((ticket / "instance-open-frame.schema.json").read_text(encoding="utf-8"))
law = json.loads((ticket / "instance-open-frame.law.json").read_text(encoding="utf-8"))
sqlite = sqlite3.connect(":memory:")
suffix = ".exe" if sys.platform == "win32" else ""
binary = generated / f"instance-open-frame-smoke{suffix}"


def sqlite_word(value, length):
    cause = (
        "WITH RECURSIVE LE(value,i) AS "
        "(SELECT '',length(?1) UNION ALL SELECT value||substr(?1,i-1,2),i-2 FROM LE WHERE i>0) "
        "SELECT value FROM LE WHERE i=0"
    )
    return bytes.fromhex(sqlite.execute(cause, (f"{value:0{length*2}x}",)).fetchone()[0])


def encode(row, sql):
    event = row["event"]
    app = event["appId"].encode()
    actor = event["actor"].encode()
    config = bytes(event["config"])
    parts = [app, actor, config]
    word = sqlite_word if sql else lambda n, size: int(n).to_bytes(size, "little")
    for name, data in event["assets"]:
        name, data = name.encode(), bytes(data)
        parts.extend([word(len(name), 4), word(len(data), 4), name, data])
    for capability in event["capabilities"]:
        identifier = capability["id"].encode()
        scope = capability["scope"].encode()
        parts.extend([
            word(int(capability["token"]), 16), word(int(capability.get("expiresMs", 0)), 8),
            word(int("expiresMs" in capability), 4), word(len(identifier), 4),
            word(len(scope), 4), identifier, scope,
        ])
    payload = b"".join(parts)
    quotas = event["quotas"]
    presence = sum(1 << index for index, name in enumerate(schema["quotaWords"]) if name in quotas)
    request = event["request"]
    fixed = [
        schema["headerBytes"]+len(payload), int(request["activationGeneration"]), int(request["instanceId"]),
        int(request["requestSequence"]), *(map(int,row["limits"])), presence,
        len(app),len(actor),len(config),len(event["assets"]),len(event["capabilities"]),
    ]
    amounts = [int(quotas.get(name, 0)) for name in schema["quotaWords"]]
    if sql:
        assert len(schema["header"][1:-1]) == len(fixed)
        fields = [word(value, field["length"]) for field, value in zip(schema["header"][1:-1], fixed)]
        header = b"SPO1" + b"".join(fields) + b"".join(word(value,8) for value in amounts)
    else:
        header = struct.pack("<4sIQIQ5I3Q3I6I17Q",b"SPO1",*fixed,*amounts)
    assert len(header) == schema["headerBytes"]
    return header + payload


def run(wire):
    return subprocess.run([str(binary.resolve())], input=wire, capture_output=True, check=True).stdout.decode("ascii").splitlines()


frames = []
for row in law["cases"]:
    frame = encode(row, False)
    assert encode(row, True) == frame
    assert frame.hex() == row["hex"]
    result = run(frame)
    event = row["event"]
    request = event["request"]
    assert result[0] == frame.hex()
    assert result[1].split() == [request[key] for key in ("activationGeneration","instanceId","requestSequence")]
    assert result[2].split() == row["limits"]
    assert result[3].split() == [event["quotas"].get(key, "-") for key in schema["quotaWords"]]
    assert result[4:] == [f"{cap['token']} {cap.get('expiresMs', '-')}" for cap in event["capabilities"]]
    frames.append(frame)
for refusal in law["refusals"]:
    frame = bytearray(frames[refusal["case"]])
    frame[refusal["index"]] = refusal["replacement"]
    assert run(frame) == ["invalid"], refusal["name"]
assert run(frames[0][:-1]) == ["invalid"]
assert run(frames[0]+b"\0") == ["invalid"]
generated.joinpath("note-instance-open.spo1").write_bytes(frames[0])
print(f"[DEBUG] Rust agreed with Python struct and SQLite for {len(frames)} wires. Note is {len(frames[0])} bytes. Seven malformed forms refused.")
