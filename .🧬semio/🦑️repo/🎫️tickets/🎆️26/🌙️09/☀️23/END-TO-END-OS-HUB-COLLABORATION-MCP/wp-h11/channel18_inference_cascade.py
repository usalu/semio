"""🔗️ H11 (ticket 26/09/23, session 13): after the frozen GIS Map binding digest moved with CHANNEL_VERSION 18, re-derive the
frozen GIS inference identity chain with the formulas of the independent TS oracle (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`,
GIS inference job fixture proof): identityDigest = sha256("semio.hub.inference-identity/v1\\0" + JSON(identity)), jobId =
sha256("semio.hub.inference-job-id/v1\\0" + identityDigest)[:32], proposalHash = sha256(proposal), mutationId =
sha256("semio.hub.inference-approval-mutation/v1\\0" + jobId + "\\0" + proposalHash)[:32], commandHash = sha256(command bytes).
Every old value is first recomputed and proven equal to the fixture (old identity with the old binding digest), then every
equal-length token (ASCII and hex-of-ASCII) is rewritten across the fixtures that quote it. usage: python3 … [--write]"""
import hashlib, json, sys
WRITE = "--write" in sys.argv
OLD_BINDING = "6343843a352cf26c26ecc8c871124dd46ce378d8f14d2dbd7fcb6b0d9e34dc09"
JOB = "🌎️hub/🧫️fixtures/🗺️gis-inference-job-v1/🔣️.json"
FILES = [JOB, "🌎️hub/🧫️fixtures/🧭️inference-job-reconcile-v1/🔣️.json", "🌎️hub/🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json", "🌎️hub/💡️inference/🧬️schema/🔣️.json", "🌎️hub/💡️inference/🧬️schema/🟦️.ts"]
h = lambda text: hashlib.sha256(text.encode()).hexdigest()
js = lambda value: json.dumps(value, separators=(",", ":"), ensure_ascii=False)

def chain(identity, proposal):
    identity_digest = h("semio.hub.inference-identity/v1\0" + js(identity))
    job = h("semio.hub.inference-job-id/v1\0" + identity_digest)[:32]
    return identity_digest, job

job = json.load(open(JOB))
new_binding = job["identity"]["binding"]["digest"]
old_identity = json.loads(js(job["identity"]).replace(new_binding, OLD_BINDING))
o = job["outbox"]
old_digest, old_job = chain(old_identity, o["proposal"])
assert (old_digest, old_job) == (job["identityDigest"], o["jobId"]), "oracle disagrees with the fixture's old identity chain"
assert h(o["proposal"]) == o["proposalHash"]
assert h(f"semio.hub.inference-approval-mutation/v1\0{old_job}\0{o['proposalHash']}")[:32] == o["mutationId"]
assert hashlib.sha256(bytes.fromhex(o["commandHex"])).hexdigest() == o["commandHash"]
new_digest, new_job = chain(job["identity"], None)
new_proposal = o["proposal"].replace(old_job, new_job)
new_proposal_hash = h(new_proposal)
new_mutation = h(f"semio.hub.inference-approval-mutation/v1\0{new_job}\0{new_proposal_hash}")[:32]
command = bytes.fromhex(o["commandHex"]).replace(old_job.encode(), new_job.encode()).replace(o["mutationId"].encode(), new_mutation.encode())
assert len(command) == len(bytes.fromhex(o["commandHex"]))
new_command_hash = hashlib.sha256(command).hexdigest()
pairs = [(o["commandHex"], command.hex()), (old_job.encode().hex(), new_job.encode().hex()), (o["mutationId"].encode().hex(), new_mutation.encode().hex()), (old_digest, new_digest), (old_job, new_job), (o["mutationId"], new_mutation), (o["proposalHash"], new_proposal_hash), (o["commandHash"], new_command_hash)]
for old, new in pairs:
    assert len(old) == len(new)
for path in FILES:
    text = open(path, encoding="utf-8").read()
    counts = []
    for old, new in pairs:
        counts.append(text.count(old))
        text = text.replace(old, new)
    print(("WRITE " if WRITE else "DRY ") + path.split("/")[-2] + "/" + path.split("/")[-1], counts)
    if WRITE:
        open(path, "w", encoding="utf-8").write(text)
print(f"identity {old_digest[:12]}→{new_digest[:12]} job {old_job}→{new_job} mutation {o['mutationId']}→{new_mutation} proposal {o['proposalHash'][:12]}→{new_proposal_hash[:12]} command {o['commandHash'][:12]}→{new_command_hash[:12]}")
