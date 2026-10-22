#!/usr/bin/env python3
"""Rebuilds the doctor overlay with the staged runtime-seam, tool-iface and agent-session modules added as real files, so the tools and sessions probes can be exercised against the real registry and the real session store code before those sets are applied.

usage: integration-overlay.py [OVERLAY_DIR]"""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import overlay, paths  # noqa: E402

TICKET = os.path.dirname(HERE)
J = os.path.join


def tool_iface():
    manifest = json.load(open(J(TICKET, "tool-iface-tools", "staged", "manifest.json"), encoding="utf-8"))
    extra = {}
    for row in manifest["files"]:
        path = row["path"]
        if row["kind"] == "new" and ("/🧰️tools/" in path or path.endswith("typescript/🧰️tools/🟦️.ts")) and "/🧪️tests/" not in path:
            extra[path] = open(J(TICKET, "tool-iface-tools", "staged", "after", row["after"]), "rb").read()
    return extra


def agent_session():
    edits = json.load(open(J(TICKET, "agent-session-tools", "staged", "edits.json"), encoding="utf-8"))["edits"]
    extra = {}
    for row in edits:
        if row["kind"] == "new" and "agent-sessions" in row["file"] and "/🧪️tests/" not in row["file"]:
            extra[row["file"]] = open(J(TICKET, "agent-session-tools", "staged", "files", row["blob"]), "rb").read()
    return extra


def runtime_seam():
    base = J(TICKET, "runtime-seam-tools", "staged", "new")
    extra = {}
    for folder, _, names in os.walk(base):
        for name in names:
            absolute = J(folder, name)
            relative = os.path.relpath(absolute, base)
            if "/🧪️test" not in "/" + relative:
                extra[relative] = open(absolute, "rb").read()
    return extra


if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/repo-doctor/ovi"
    extra = {**runtime_seam(), **tool_iface(), **agent_session()}
    overlay.build(target, True, extra)
    print("integration overlay", target, len(extra), "staged files of other sets")
