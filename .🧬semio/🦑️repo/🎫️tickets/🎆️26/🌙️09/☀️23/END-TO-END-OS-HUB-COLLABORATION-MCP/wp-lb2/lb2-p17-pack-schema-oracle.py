#!/usr/bin/env python3
"""🔮️ LB2 p17 third-party oracle (Python hashlib): every pack-schema identity a stdio package answers in its own process (law f's
children, `SEMIO_STDIO_CODEC_HASH_OUT=<dir>`) is the SHA-256 of a committed `📸️snapshot/💾️binary/📡️.protocol.semio` of that
kind's own artifact, and every linked native codec receipt (`📇️registry/📜️native-codec-factories.json`) pins the SHA-256 of the
protocol file it names.

usage: python3 lb2-p17-pack-schema-oracle.py <answers-dir> [<tree>]
"""
import glob, hashlib, json, os, sys

ANSWERS = sys.argv[1]
TREE = sys.argv[2] if len(sys.argv) > 2 else "/Users/ueli/Documents/semio"
STDIO = os.path.join(TREE, "✏️s/🔌️plugins/🗄️stdio")
ART = os.path.join(STDIO, "🗿️artifacts")

protocols = {}
for path in glob.glob(os.path.join(ART, "*/🏅️standards/**/📸️snapshot/💾️binary/📡️.protocol.semio"), recursive=True):
    artifact = os.path.relpath(path, ART).split("/")[0]
    protocols.setdefault(artifact, {})[hashlib.sha256(open(path, "rb").read()).hexdigest()] = os.path.relpath(path, ART)

def artifact_of(kind):
    name = kind.split(".")[2]
    found = [folder for folder in protocols if folder.endswith(name) and not folder[: -len(name)].rstrip("️")[-1:].isalnum()]
    return found[0] if len(found) == 1 else None

failures, answered, per_kind = [], 0, {}
files = sorted(glob.glob(os.path.join(ANSWERS, "*.json")))
for path in files:
    package = os.path.basename(path)[:-5]
    for row in json.load(open(path, encoding="utf-8")):
        answered += 1
        artifact = artifact_of(row["kind"])
        if artifact is None:
            failures.append(f"{package} {row['kind']}: no single artifact folder")
        elif row["hash"] not in protocols[artifact]:
            failures.append(f"{package} {row['kind']}={row['schema']}: {row['hash']} is no protocol of {artifact}")
        else:
            per_kind.setdefault((row["kind"], row["schema"]), set()).add(row["hash"])
for (kind, schema), hashes in sorted(per_kind.items()):
    if len(hashes) != 1:
        failures.append(f"{kind}={schema}: packages answer {len(hashes)} different identities")

registry = json.load(open(os.path.join(STDIO, "📇️registry/📜️native-codec-factories.json"), encoding="utf-8"))
for receipt in registry["receipts"]:
    digest = hashlib.sha256(open(os.path.join(STDIO, receipt["protocol_path"]), "rb").read()).hexdigest()
    if digest != receipt["pack_schema_sha256"]:
        failures.append(f"receipt {receipt['artifact_kind']}: pins {receipt['pack_schema_sha256']}, its protocol hashes {digest}")
    answer = per_kind.get((receipt["artifact_kind"], receipt["artifact_schema"]))
    if answer is not None and answer != {digest}:
        failures.append(f"receipt {receipt['artifact_kind']}: packages answer {sorted(answer)}, the protocol hashes {digest}")

print(f"packages {len(files)}, answers {answered}, (kind, schema) {len(per_kind)}, protocols {sum(map(len, protocols.values()))}, linked receipts {len(registry['receipts'])}, failures {len(failures)}")
for line in failures:
    print("  ", line)
sys.exit(1 if failures or not files else 0)
