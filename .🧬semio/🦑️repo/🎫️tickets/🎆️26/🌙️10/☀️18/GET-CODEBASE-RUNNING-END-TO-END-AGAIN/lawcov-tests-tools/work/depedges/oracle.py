"""🕸 Independent Python oracle of how a module's import specifiers become project dependency edges (standard library only): relative imports go to the deepest owning root, bare imports to the workspace package, the lock file or nowhere, and core modules to nothing; and of the tracked source file choice. Requests arrive as JSON on stdin."""
import json, posixpath, sys

CORE = {"fs", "path", "os", "crypto", "child_process", "events", "url", "util", "stream", "http", "zlib", "assert", "module", "buffer"}


def owners(projects):
    rows = []
    for name, project in projects.items():
        for root in [project["root"]] + ([project["metadata"]["nativeRoot"]] if project.get("metadata", {}).get("nativeRoot") else []):
            rows.append((name, root))
    return rows


def deepest(rows, path):
    best = None
    for name, root in rows:
        if root == "." or not (path == root or path.startswith(root + "/")):
            continue
        if best is None or len(root) > len(best[1]) or (len(root) == len(best[1]) and name < best[0]):
            best = (name, root)
    return best[0] if best else None


def is_core(package):
    return package.startswith("node:") or package in CORE


def targets(request):
    rows = owners(request["projects"])
    found, queried = set(), []
    locked = request.get("locked")
    for spec in request["imports"]:
        if spec.startswith("."):
            joined = posixpath.normpath(posixpath.join(request["workspaceRoot"], posixpath.dirname(request["file"]), spec))
            path = posixpath.relpath(joined, request["workspaceRoot"])
            path = "" if path == "." else path
            owner = deepest(rows, path)
            if owner:
                found.add(owner)
            continue
        package = "/".join(spec.split("/")[:2]) if spec.startswith("@") else spec.split("/")[0]
        if is_core(package):
            continue
        if package in request["byPackage"]:
            found.add(request["byPackage"][package])
            continue
        queried.append([request["file"], package])
        key = locked["keys"].get(package) if locked else None
        if key and "npm:" + key in locked["external"]:
            found.add("npm:" + key)
    return {"targets": sorted(found), "queried": queried}


def tracked(request):
    exists = lambda path: path in request["files"]
    preferred = request["preferred"]
    if exists(preferred):
        return preferred
    files = (request.get("map") or {}).get(request["project"])
    hint = request.get("hint") or preferred.split("/")[-1]
    if files:
        matches = [entry for entry in files if (entry == hint or entry.endswith("/" + hint)) and exists(entry)]
        if len(matches) == 1:
            return matches[0]
    return preferred


request = json.load(sys.stdin)
print(json.dumps([targets(r) if r["kind"] == "targets" else tracked(r) for r in request["requests"]], ensure_ascii=False))
