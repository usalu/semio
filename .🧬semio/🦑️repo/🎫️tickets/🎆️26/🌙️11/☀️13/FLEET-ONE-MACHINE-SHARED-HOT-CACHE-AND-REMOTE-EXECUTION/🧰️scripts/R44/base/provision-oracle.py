"""Independent oracle for the provisioning kit and the fleet-code overlay: it re-implements the seal check, the path rule, the kit layout and the overlay counts with hashlib only, so the TypeScript and shell implementations are compared with code that shares nothing with them."""
import hashlib
import json
import os
import sys


def sha(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def safe(path):
    if path == "" or path.startswith("/") or "\\" in path:
        return False
    return all(part not in ("", ".", "..") for part in path.split("/"))


def seal_of(snapshot):
    with open(os.path.join(snapshot, "sealed.json"), encoding="utf8") as handle:
        return json.load(handle)


def verify(snapshot):
    seal = seal_of(snapshot)
    if seal.get("version") != 1 or not seal["files"]:
        return "not a version 1 seal"
    for item in seal["files"]:
        if not safe(item["path"]):
            return "unsafe " + item["path"]
        file = os.path.join(snapshot, "files", *item["path"].split("/"))
        if not os.path.isfile(file):
            return "missing " + item["path"]
        if os.path.getsize(file) != item["bytes"] or sha(file) != item["sha256"]:
            return "changed " + item["path"]
    return "ok"


def overlay(snapshot, worktree):
    replaced = added = identical = 0
    for item in seal_of(snapshot)["files"]:
        target = os.path.join(worktree, *item["path"].split("/"))
        if os.path.isfile(target) and not os.path.islink(target) and sha(target) == item["sha256"]:
            identical += 1
        elif os.path.lexists(target):
            replaced += 1
        else:
            added += 1
    return "%d %d %d" % (replaced, added, identical)


def kit(directory, snapshot):
    seal = seal_of(snapshot)
    prefix = seal["moduleRoot"] + "/🗺️topology/"
    expected = {"topology/" + item["path"][len(prefix):]: item["sha256"] for item in seal["files"] if item["path"].startswith(prefix)}
    with open(os.path.join(directory, "SHA256SUMS"), encoding="utf8") as handle:
        lines = [line for line in handle.read().split("\n") if line]
    listed = {}
    for line in lines:
        digest, path = line.split("  ", 1)
        listed[path] = digest
    if listed != expected:
        return "SHA256SUMS differs from the sealed topology files"
    for path, digest in listed.items():
        if sha(os.path.join(directory, *path.split("/"))) != digest:
            return "changed " + path
    present = set()
    for root, _, names in os.walk(directory):
        for name in names:
            present.add(os.path.relpath(os.path.join(root, name), directory).replace(os.sep, "/"))
    if present - {"SHA256SUMS", "published"} != set(listed):
        return "unexpected files in the kit"
    with open(os.path.join(directory, "published"), encoding="utf8") as handle:
        receipt = json.load(handle)
    if receipt["code"] != seal["digest"] or receipt["files"] != len(listed) or receipt["sums"] != sha(os.path.join(directory, "SHA256SUMS")):
        return "receipt differs"
    return "ok"


if __name__ == "__main__":
    command = sys.argv[1]
    if command == "verify":
        print(verify(sys.argv[2]))
    elif command == "overlay":
        print(overlay(sys.argv[2], sys.argv[3]))
    elif command == "kit":
        print(kit(sys.argv[2], sys.argv[3]))
    else:
        sys.exit(2)
