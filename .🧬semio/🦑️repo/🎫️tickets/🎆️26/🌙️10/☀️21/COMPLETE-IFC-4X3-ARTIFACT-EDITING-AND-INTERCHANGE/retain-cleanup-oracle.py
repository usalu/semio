"""📦️ Retain a pinned independent IteratorClose implementation for test-only comparison."""
from pathlib import Path
import base64
import hashlib
import io
import json
import tarfile
import urllib.request

ticket = Path(__file__).parent
target = ticket / "🗑️generated" / "close-ref"
target.mkdir(exist_ok=True)
url = "https://registry.npmjs.org/@babel%2fruntime/7.28.4"
with urllib.request.urlopen(url, timeout=30) as response:
    metadata_bytes = response.read(2_000_001)
assert len(metadata_bytes) <= 2_000_000
metadata = json.loads(metadata_bytes)
assert metadata["name"] == "@babel/runtime" and metadata["version"] == "7.28.4"
dist = metadata["dist"]
with urllib.request.urlopen(dist["tarball"], timeout=30) as response:
    package = response.read(2_000_001)
assert len(package) <= 2_000_000
assert dist["integrity"] == "sha512-" + base64.b64encode(hashlib.sha512(package).digest()).decode("ascii")
(target / "registry.json").write_bytes(metadata_bytes)
(target / "runtime.tgz").write_bytes(package)
files = []
with tarfile.open(fileobj=io.BytesIO(package), mode="r:gz") as archive:
    for name in ["helpers/createForOfIteratorHelper.js", "helpers/unsupportedIterableToArray.js", "helpers/arrayLikeToArray.js", "package.json", "LICENSE"]:
        member = archive.getmember("package/" + name)
        assert member.isfile()
        source = archive.extractfile(member)
        assert source is not None
        path = target / Path(name).name
        assert len(str(path).encode("utf-16-le")) // 2 <= 256
        content = source.read()
        path.write_bytes(content)
        files.append({"path": str(path), "original": member.name, "sha256": hashlib.sha256(content).hexdigest()})
provenance = {"name": metadata["name"], "version": metadata["version"], "registry": url, "tarball": dist["tarball"], "integrity": dist["integrity"], "sha256": hashlib.sha256(package).hexdigest(), "files": files, "scope": "test-only independent IteratorClose precedence, no runtime dependency"}
(target / "provenance.json").write_text(json.dumps(provenance, ensure_ascii=False, indent=2), encoding="utf-8")
print("[DEBUG] " + json.dumps({"files": len(files), "integrity": "verified", "version": metadata["version"]}))
