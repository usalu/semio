#!/usr/bin/env python3
"""🔢️ H13 (session 14b, rule 22): moves H13's four channel-derived fixtures from appChannelVersion 18 to the pin 19 with the
independent oracles of the 17→18 move (`wp-h11/channel18_oracle.py`, `wp-h11/channel18_inference_cascade.py`), each first proven
equal to the fixture's CURRENT values at 18, then re-derived at 19. Text-level rewrites (formatting kept):
  - document-open plan: literals + catalog `expectedHex` + generation id (sha256 of the encoding) everywhere it is quoted;
  - GIS Map frozen binding: literal + `expectedDigest`, the fixtures that quote the digest, and the frozen GIS inference identity
    chain it feeds (identity digest, job id, proposal hash, mutation id, command bytes + hash) in job / reconcile / WAL-proof
    fixtures and the inference schema JSON + TS;
  - browser document-open transport + execution-target lease corpus: the literals only (their generation ids are opaque corpus
    constants — the 17→18 move changed only the literal and every document-open / execution-target law stayed green).
Idempotent (already at 19 → nothing to do); dry run unless `--write`. usage: python3 h13-channel19.py [--write]"""
import hashlib, json, re, struct, sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
OLD, NEW = 18, 19
PLAN = "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"
BROWSER = "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json"
LEASE = "🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json"
FROZEN = "🌎️hub/🧫️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json"
QUOTING = ["🌎️hub/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json", "🌎️hub/🧫️fixtures/🗺️gis-inference-job-v1/🔣️.json"]
JOB = "🌎️hub/🧫️fixtures/🗺️gis-inference-job-v1/🔣️.json"
CASCADE = [JOB, "🌎️hub/🧫️fixtures/🧭️inference-job-reconcile-v1/🔣️.json", "🌎️hub/🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json", "🌎️hub/💡️inference/🧬️schema/🔣️.json", "🌎️hub/💡️inference/🧬️schema/🟦️.ts"]
LITERALS = {PLAN: 3, BROWSER: 2, LEASE: 2, FROZEN: 1}
LITERAL = re.compile(r'("appChannelVersion":\s*)(\d+)')

texts: dict[str, str] = {}
read = lambda path: texts.setdefault(path, (ROOT / path).read_text(encoding="utf-8"))
h = lambda text: hashlib.sha256(text.encode()).hexdigest()
js = lambda value: json.dumps(value, separators=(",", ":"), ensure_ascii=False)
lp = lambda b: struct.pack(">Q", len(b)) + b


def catalog_encoding(rows) -> bytes:
    out = [b"semio/hub/openable-document-catalog/v1\0", struct.pack(">I", len(rows))]
    for r in rows:
        p, a, pd, s, g = r["package"], r["artifact"], r["parentDialect"], r["surface"], r["grant"]
        fields = [p["pluginId"].encode(), p["packageId"].encode(), p["version"].encode(), bytes.fromhex(p["componentSha256"]), bytes.fromhex(p["componentBlake3"]), bytes.fromhex(p["descriptorByteSha256"]), struct.pack(">I", p["executionProtocol"]["appChannelVersion"]), a["kind"].encode(), a["schema"].encode(), bytes.fromhex(a["packSchemaHash"]), pd["artifactKind"].encode(), pd["standard"].encode(), pd["subset"].encode(), s["surfaceId"].encode(), s["appId"].encode(), s["windowKindId"].encode(), s["role"].encode(), s["rendererTarget"].encode(), bytes([int(g["read"]), int(g["write"]), int(g["observe"])])]
        out.append(b"".join(lp(x) for x in fields))
    return b"".join(out)


def binding_digest(binding) -> str:
    return hashlib.sha256(b"semio.hub.gis-map-frozen-binding/v1\x00" + js(binding).encode()).hexdigest()


def bump_json(value):
    if isinstance(value, dict):
        return {k: (NEW if k == "appChannelVersion" and v == OLD else bump_json(v)) for k, v in value.items()}
    if isinstance(value, list):
        return [bump_json(v) for v in value]
    return value


def replace(path: str, old: str, new: str, expect_at_least: int = 1) -> int:
    count = read(path).count(old)
    if count < expect_at_least:
        sys.exit(f"problem: {old[:24]}… found {count}× in {path}")
    texts[path] = read(path).replace(old, new)
    return count


def literals(path: str) -> int:
    values = [int(m.group(2)) for m in LITERAL.finditer(read(path))]
    if len(values) != LITERALS[path] or set(values) != {OLD}:
        sys.exit(f"problem: {path} literals {values}, expected {LITERALS[path]}× {OLD}")
    texts[path] = LITERAL.sub(lambda m: f"{m.group(1)}{NEW}", read(path))
    return len(values)


def main() -> int:
    states = {path: {int(m.group(2)) for m in LITERAL.finditer(read(path))} for path in LITERALS}
    if all(values == {NEW} for values in states.values()):
        print("already at", NEW)
        return 0
    if not all(values == {OLD} for values in states.values()):
        sys.exit(f"problem: mixed literal state {states}")
    notes = []

    plan = json.loads(read(PLAN))
    enc = catalog_encoding(plan["catalogRows"])
    old_generation = plan["catalogEncoding"]["expectedGenerationId"]
    assert enc.hex() == plan["catalogEncoding"]["expectedHex"] and hashlib.sha256(enc).hexdigest() == old_generation, "plan oracle disagrees at 18"
    new_enc = catalog_encoding(bump_json(plan)["catalogRows"])
    new_generation = hashlib.sha256(new_enc).hexdigest()
    literals(PLAN)
    replace(PLAN, enc.hex(), new_enc.hex())
    notes.append(f"plan generation {old_generation[:12]}→{new_generation[:12]} ({replace(PLAN, old_generation, new_generation, 2)} sites)")

    frozen = json.loads(read(FROZEN))
    old_binding = frozen["expectedDigest"]
    assert binding_digest(frozen["binding"]) == old_binding, "frozen-binding oracle disagrees at 18"
    new_binding = binding_digest(bump_json(frozen["binding"]))
    literals(FROZEN)
    replace(FROZEN, old_binding, new_binding)
    notes.append(f"frozen binding {old_binding[:12]}→{new_binding[:12]} quoted in {[replace(path, old_binding, new_binding) for path in QUOTING]}")

    job = json.loads((ROOT / JOB).read_text(encoding="utf-8"))
    o = job["outbox"]
    chain = lambda identity: (lambda d: (d, h("semio.hub.inference-job-id/v1\0" + d)[:32]))(h("semio.hub.inference-identity/v1\0" + js(identity)))
    old_digest, old_job = chain(job["identity"])
    assert (old_digest, old_job) == (job["identityDigest"], o["jobId"]), "inference identity oracle disagrees at 18"
    assert h(o["proposal"]) == o["proposalHash"] and h(f"semio.hub.inference-approval-mutation/v1\0{old_job}\0{o['proposalHash']}")[:32] == o["mutationId"]
    assert hashlib.sha256(bytes.fromhex(o["commandHex"])).hexdigest() == o["commandHash"]
    new_digest, new_job = chain(json.loads(js(job["identity"]).replace(old_binding, new_binding)))
    new_proposal_hash = h(o["proposal"].replace(old_job, new_job))
    new_mutation = h(f"semio.hub.inference-approval-mutation/v1\0{new_job}\0{new_proposal_hash}")[:32]
    command = bytes.fromhex(o["commandHex"]).replace(old_job.encode(), new_job.encode()).replace(o["mutationId"].encode(), new_mutation.encode())
    assert len(command) == len(bytes.fromhex(o["commandHex"]))
    pairs = [(o["commandHex"], command.hex()), (old_job.encode().hex(), new_job.encode().hex()), (o["mutationId"].encode().hex(), new_mutation.encode().hex()), (old_digest, new_digest), (old_job, new_job), (o["mutationId"], new_mutation), (o["proposalHash"], new_proposal_hash), (o["commandHash"], hashlib.sha256(command).hexdigest())]
    assert all(len(a) == len(b) for a, b in pairs)
    for path in CASCADE:
        counts = [read(path).count(a) for a, _ in pairs]
        for a, b in pairs:
            texts[path] = read(path).replace(a, b)
        notes.append(f"cascade {path.split('/')[-2]} {counts}")
    notes.append(f"identity {old_digest[:12]}→{new_digest[:12]} job {old_job}→{new_job} mutation {o['mutationId']}→{new_mutation}")

    for path in (BROWSER, LEASE):
        notes.append(f"{path.split('/')[-2] if path.endswith('🔣️.json') else path.split('/')[-1]}: {literals(path)} literal(s)")

    for note in notes:
        print(("WRITE " if WRITE else "DRY ") + note)
    if WRITE:
        for path, text in texts.items():
            (ROOT / path).write_text(text, encoding="utf-8")
        print(f"wrote {len(texts)} files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
