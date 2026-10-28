"""📦 Retain the official test-only Khronos validator with registry integrity checks."""
import base64
import hashlib
import io
import json
from pathlib import Path
import tarfile
import urllib.request

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated/gltfval"
OUTPUT.mkdir(parents=True, exist_ok=True)
metadata = json.load(urllib.request.urlopen("https://registry.npmjs.org/gltf-validator/2.0.0-dev.3.10"))
assert metadata["name"] == "gltf-validator"
assert "KhronosGroup/glTF-Validator" in metadata["repository"]["url"]
archive = urllib.request.urlopen(metadata["dist"]["tarball"]).read()
assert hashlib.sha256(archive).hexdigest() == "4e03dbdc3bc0d1342afd5d2d7ae341a7e3474502ccb872b02dbbaddaee0fdec6"
algorithm, digest = metadata["dist"]["integrity"].split("-", 1)
assert algorithm == "sha512"
assert base64.b64encode(hashlib.sha512(archive).digest()).decode("ascii") == digest
with tarfile.open(fileobj=io.BytesIO(archive), mode="r:gz") as bundle:
    for member in bundle.getmembers():
        assert len(str(OUTPUT / member.name)) <= 256
    bundle.extractall(OUTPUT, filter="data")
(OUTPUT / "provenance.json").write_text(json.dumps({"version": metadata["version"], "url": metadata["dist"]["tarball"], "integrity": metadata["dist"]["integrity"], "sha256": hashlib.sha256(archive).hexdigest(), "repository": metadata["repository"]["url"]}, indent=2) + "\n", encoding="utf-8")
print("[DEBUG] " + json.dumps({"validator": metadata["version"], "archiveSha256": hashlib.sha256(archive).hexdigest()}))
