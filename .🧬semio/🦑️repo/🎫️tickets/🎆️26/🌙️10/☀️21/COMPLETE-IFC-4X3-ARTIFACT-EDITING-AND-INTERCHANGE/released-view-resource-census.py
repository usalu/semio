"""🗂️ Bound primary release-resource discovery without promoting informative files to requirement authority."""
import hashlib
import json
import urllib.request
from pathlib import Path
from typing import Protocol

class ResourceOracle(Protocol):
    def get(self, url: str, limit: int) -> bytes: ...

class GithubResourceOracle:
    def get(self, url: str, limit: int) -> bytes:
        request = urllib.request.Request(url, headers={"User-Agent": "semio-ifc-release-audit"})
        with urllib.request.urlopen(request, timeout=20) as response:
            data = response.read(limit + 1)
        if len(data) > limit:
            raise ValueError("bounded release-resource census response exceeds admission")
        return data

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated" / "released-view-resources.json"
commit = "610ee3f95db9d0380b1eb8de4dc6e17833531518"
url = f"https://api.github.com/repos/buildingSMART/Standards-builds/contents/IFC/RELEASE/IFC4_3?ref={commit}"
oracle: ResourceOracle = GithubResourceOracle()
result = dict(commit=commit, url=url, completeViewRequirementSet=False, observations=[])
try:
    raw = oracle.get(url, 1048576)
    rows = json.loads(raw)
    result["listingSha256"] = hashlib.sha256(raw).hexdigest()
    result["observations"] = [dict(name=row["name"], type=row["type"], bytes=row["size"], gitBlob=row["sha"], url=row["download_url"], path=row["path"]) for row in rows]
except Exception as error:
    result["error"] = dict(kind=type(error).__name__, message=str(error))
output.write_text(json.dumps(result, indent=2), encoding="utf-8")
print("[DEBUG] " + json.dumps(result))
