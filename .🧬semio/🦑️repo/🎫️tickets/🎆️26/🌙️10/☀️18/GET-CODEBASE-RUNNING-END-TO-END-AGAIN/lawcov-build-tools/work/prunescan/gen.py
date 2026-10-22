"""📝 Authoring aid: writes the cache area projection corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
u = lambda path, kind="directory": {"path": path, "bytes": 10, "recencyMs": 1000, "lockHeld": False, "kind": kind}
c = {"version": 1, "repoRoot": "/repo"}
c["layouts"] = [
    {"name": "build and target differ", "build": "/repo/b", "target": "/repo/t", "scans": {"build": [u("b1", "cargo-build")], "target": [u("t1", "cargo-target-file")], "incremental": {"units": [u("i1", "cargo-incremental")], "staleSessions": [u("s1", "cargo-incremental-session")]}, "vite": [u("v1")], "agents": [u("a1"), u("a2")]}},
    {"name": "a shared build and target directory is scanned once", "build": "/repo/b", "target": "/repo/b", "scans": {"build": [u("b1", "cargo-build"), u("b2", "cargo-build")], "target": [u("t1", "cargo-target-file")], "incremental": {"units": [], "staleSessions": []}, "vite": [], "agents": []}},
    {"name": "everything empty", "build": "/repo/b", "target": "/repo/t", "scans": {"build": [], "target": [], "incremental": {"units": [], "staleSessions": []}, "vite": [], "agents": []}},
]
c["roots"] = [
    {"area": "cargo", "kind": "cargo-build", "expected": "build"}, {"area": "cargo", "kind": "cargo-target-file", "expected": "target"}, {"area": "cargo-incremental", "kind": "cargo-incremental", "expected": "build"},
    {"area": "cargo-incremental", "kind": "cargo-target-file", "expected": "target"}, {"area": "cargo-incremental-sessions", "kind": "cargo-incremental-session", "expected": "build"},
    {"area": "vite", "kind": "directory", "expected": "cache:vite"}, {"area": "agents", "kind": "directory", "expected": "cache:agents"}, {"area": "vite", "kind": "cargo-target-file", "expected": "cache:vite"}, {"area": "other", "kind": "directory", "expected": "cache:other"},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
