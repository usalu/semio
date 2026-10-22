"""🧷️ Independent Python oracle of captured authorizer snapshots (standard library only): it builds the manifest, its digest and the base64 file records of a snapshot from declared sources, injects the declared faults, and hashes the closure of a workspace. Requests arrive as JSON on stdin."""
import base64, hashlib, json, sys


def content(spec):
    return (spec["repeat"] * spec["times"]).encode() if "repeat" in spec else spec["text"].encode()


def dumps(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def manifest_of(row, sources):
    manifest = {"version": 1, "mode": "release", "entry": row["entry"], "resolutions": [], "sources": sources}
    manifest.update(row.get("manifest", {}))
    return manifest


def build(row):
    data = [(source["path"], content(source["content"]), source) for source in row["sources"]]
    sources = [{"path": path, "sha256": source.get("sha256", hashlib.sha256(raw).hexdigest())} for path, raw, source in data]
    manifest = manifest_of(row, sources)
    snapshot = {"version": 1, "manifest": manifest, "digest": hashlib.sha256((dumps(manifest) + "\n").encode()).hexdigest(),
                "files": [{"path": path, "base64": source.get("base64", base64.b64encode(raw).decode())} for path, raw, source in data]}
    patch = row.get("patch", {})
    if "digest" in patch:
        snapshot["digest"] = patch["digest"]
    if "extra" in patch:
        snapshot["extra"] = patch["extra"]
    if "version" in patch:
        snapshot["version"] = patch["version"]
    if "dropFile" in patch:
        del snapshot["files"][patch["dropFile"]]
    if "addFile" in patch:
        snapshot["files"].append({"path": patch["addFile"], "base64": "eA=="})
    return snapshot


def closure(files, paths):
    manifest = {"version": 1, "mode": "release", "entry": paths[0], "resolutions": [], "sources": []}
    by_path = {source["path"]: content(source["content"]) for source in files}
    manifest["sources"] = [{"path": path, "sha256": hashlib.sha256(by_path[path]).hexdigest()} for path in sorted(paths, key=lambda p: p.encode())]
    return {"manifest": manifest, "digest": hashlib.sha256((dumps(manifest) + "\n").encode()).hexdigest()}


def main():
    request = json.load(sys.stdin)
    if request["op"] == "build":
        print(json.dumps([build(row) for row in request["rows"]], ensure_ascii=False))
    else:
        print(json.dumps(closure(request["files"], request["paths"]), ensure_ascii=False))


main()
