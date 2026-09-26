#!/usr/bin/env python3
"""WG10 s13 one-off: writes the language-agnostic canonical-checkpoint-pair fixture. The bodies are framed by this
independent encoder (the hub's `append_canonical_pair_{header,data,terminal}` wire, re-derived from the wire contract) and
every digest is Python's hashlib SHA-256, so neither the Rust kernel decoder nor the TS twin grades its own homework."""
import hashlib, json, pathlib, struct

OUT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json")
RECORD = 4096
SPACE, DOC = "space-wg10", "artifact-0f1e2d3c4b5a69788796a5b4c3d2e1f0"
ZERO = bytes(32)


def lp(data: bytes) -> bytes:
    return struct.pack(">I", len(data)) + data


def frame(payload: bytes) -> bytes:
    return struct.pack(">I", len(payload)) + payload


def part_bytes(spec):
    return bytes([spec["byte"]]) * spec["length"]


def header(sel, pack, spr, **over):
    s = dict(sel)
    s.update(over)
    out = bytes([s.get("tag", 1)]) + struct.pack(">I", s.get("version", 1))
    out += lp(s["spaceId"].encode()) + lp(s["documentId"].encode())
    out += bytes.fromhex(s["descriptorDigestV1"]) + bytes.fromhex(s["checkpointId"])
    b = s["baselineFrontier"]
    out += lp(b["documentId"].encode()) + struct.pack(">Q", b["headEditOrdinal"]) + lp(b["headEditId"].encode()) + struct.pack(">Q", b["lastCommitSeq"]) + bytes(b["chainHash"])
    out += bytes.fromhex(s.get("packSha256", hashlib.sha256(pack).hexdigest())) + struct.pack(">Q", s.get("packLength", len(pack)))
    out += bytes.fromhex(s.get("sprSha256", hashlib.sha256(spr).hexdigest())) + struct.pack(">Q", s.get("sprLength", len(spr)))
    out += bytes.fromhex(s.get("aggregateSha256", hashlib.sha256(pack + spr).hexdigest()))
    return out + s.get("headerTrailing", b"")


def records(pack, spr):
    out, ordinal = [], 0
    for part, data in ((1, pack), (2, spr)):
        for offset in range(0, len(data), RECORD):
            chunk = data[offset:offset + RECORD]
            out.append(bytes([2, part]) + struct.pack(">I", ordinal) + struct.pack(">Q", offset) + lp(chunk))
            ordinal += 1
    return out


def body(sel, pack, spr, hdr=None, recs=None, terminal=bytes([3, 0]), trailing=b""):
    hdr = header(sel, pack, spr) if hdr is None else hdr
    recs = records(pack, spr) if recs is None else recs
    return frame(hdr) + b"".join(frame(r) for r in recs) + frame(terminal) + trailing


def selection(checkpoint, descriptor, baseline):
    return {"spaceId": SPACE, "documentId": DOC, "descriptorDigestV1": descriptor, "checkpointId": checkpoint, "baselineFrontier": baseline}


GENESIS = {"documentId": DOC, "headEditOrdinal": 0, "headEditId": "", "lastCommitSeq": 0, "chainHash": [0] * 32}
EDITED = {"documentId": DOC, "headEditOrdinal": 9, "headEditId": "edit-9", "lastCommitSeq": 4, "chainHash": [6] * 32}
PAIRS = [
    {"id": "genesis", "selection": selection("11" * 32, "22" * 32, GENESIS), "pack": {"byte": 7, "length": 3}, "spr": {"byte": 3, "length": 4}},
    {"id": "checked-in-multi-record", "selection": selection("33" * 32, "22" * 32, EDITED), "pack": {"byte": 9, "length": RECORD + 905}, "spr": {"byte": 5, "length": 2 * RECORD + 1}},
]
for pair in PAIRS:
    pack, spr = part_bytes(pair["pack"]), part_bytes(pair["spr"])
    pair["packSha256"] = hashlib.sha256(pack).hexdigest()
    pair["sprSha256"] = hashlib.sha256(spr).hexdigest()
    pair["aggregateSha256"] = hashlib.sha256(pack + spr).hexdigest()
    pair["bodyHex"] = body(pair["selection"], pack, spr).hex()

g = PAIRS[0]
gs, gp, gr = g["selection"], part_bytes(g["pack"]), part_bytes(g["spr"])
exact = bytes.fromhex(g["bodyHex"])
recs = records(gp, gr)
flipped_part = [bytes([2, 2]) + recs[0][2:]] + recs[1:]
wrong_ordinal = [recs[0][:2] + struct.pack(">I", 1) + recs[0][6:]] + recs[1:]
gap_offset = [recs[0][:6] + struct.pack(">Q", 1) + recs[0][14:]] + recs[1:]
record_trailing = [recs[0] + b"\x00"] + recs[1:]
tampered_pack = gp[:-1] + bytes([gp[-1] ^ 1])
REFUSALS = [
    ("truncated", "decode", exact[:-1]),
    ("terminal", "decode", body(gs, gp, gr, terminal=bytes([3, 1]))),
    ("incomplete", "decode", body(gs, gp, gr, trailing=b"\x00")),
    ("header", "decode", body(gs, gp, gr, hdr=header(gs, gp, gr, version=2))),
    ("header-trailing-bytes", "decode", body(gs, gp, gr, hdr=header(gs, gp, gr, headerTrailing=b"\x00"))),
    ("zero-digest", "decode", body(gs, gp, gr, hdr=header(gs, gp, gr, descriptorDigestV1="00" * 32))),
    ("baseline-frontier", "decode", body(gs, gp, gr, hdr=header(gs, gp, gr, baselineFrontier=dict(GENESIS, headEditOrdinal=9)))),
    ("baseline-frontier", "decode", body(gs, gp, gr, hdr=header(gs, gp, gr, baselineFrontier=dict(GENESIS, documentId="artifact-foreign")))),
    ("pair-length", "decode", body(gs, gp, b"", recs=records(gp, b""))),
    ("record-part", "decode", body(gs, gp, gr, recs=flipped_part)),
    ("record-ordinal", "decode", body(gs, gp, gr, recs=wrong_ordinal)),
    ("record-offset", "decode", body(gs, gp, gr, recs=gap_offset)),
    ("record-trailing-bytes", "decode", body(gs, gp, gr, recs=record_trailing)),
    ("pack-digest", "digest", body(gs, tampered_pack, gr, hdr=header(gs, gp, gr), recs=records(tampered_pack, gr))),
    ("aggregate-digest", "digest", body(gs, gp, gr, hdr=header(gs, gp, gr, aggregateSha256=hashlib.sha256(gr + gp).hexdigest()))),
]


def checkpoint_of(pair, **over):
    s = pair["selection"]
    expected = {"checkpointId": s["checkpointId"], "descriptorDigestV1": s["descriptorDigestV1"], "baselineFrontier": s["baselineFrontier"], "aggregateSha256": pair["aggregateSha256"]}
    expected.update(over)
    return expected


SCOPE = {"spaceId": SPACE, "documentId": DOC}
ADMISSIONS = [
    {"id": "genesis-exact", "pair": "genesis", "scope": SCOPE, "expected": checkpoint_of(g), "refusal": None},
    {"id": "checked-in-exact", "pair": "checked-in-multi-record", "scope": SCOPE, "expected": checkpoint_of(PAIRS[1]), "refusal": None},
    {"id": "foreign-space", "pair": "genesis", "scope": dict(SCOPE, spaceId="space-other"), "expected": checkpoint_of(g), "refusal": "canonical-checkpoint-pair.scope"},
    {"id": "checkpoint-moved", "pair": "genesis", "scope": SCOPE, "expected": checkpoint_of(g, checkpointId="44" * 32), "refusal": "canonical-checkpoint-pair.checkpoint"},
    {"id": "descriptor-changed", "pair": "genesis", "scope": SCOPE, "expected": checkpoint_of(g, descriptorDigestV1="55" * 32), "refusal": "canonical-checkpoint-pair.descriptor"},
    {"id": "baseline-differs", "pair": "checked-in-multi-record", "scope": SCOPE, "expected": checkpoint_of(PAIRS[1], baselineFrontier=GENESIS), "refusal": "canonical-checkpoint-pair.baseline"},
    {"id": "aggregate-differs", "pair": "genesis", "scope": SCOPE, "expected": checkpoint_of(g, aggregateSha256="66" * 32), "refusal": "canonical-checkpoint-pair.aggregate"},
]

fixture = {
    "schema": "semio.hub.canonical-checkpoint-pair-fixture/v1",
    "mediaType": "application/vnd.semio.canonical-checkpoint-pair.v1",
    "limits": {"headerBytes": 16384, "recordBytes": RECORD, "records": 16384, "pairBytes": 67108864},
    "pairs": PAIRS,
    "refusals": [{"id": f"{code}-{index}", "stage": stage, "bodyHex": data.hex(), "refusal": f"canonical-checkpoint-pair.{code}"} for index, (code, stage, data) in enumerate(REFUSALS)],
    "admissions": ADMISSIONS,
}
OUT.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")
print(OUT, OUT.stat().st_size)
