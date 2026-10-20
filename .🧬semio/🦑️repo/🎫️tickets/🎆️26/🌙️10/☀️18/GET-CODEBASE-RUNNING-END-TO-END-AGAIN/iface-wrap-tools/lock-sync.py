#!/usr/bin/env python3
"""Cargo.lock dependency edges of the iface-wrap set (stdlib only). `cargo` would rewrite the same lines on the next resolve; this keeps `--locked` builds green right after apply.
usage: lock-sync.py [check|apply] [--write] [--through GROUP] [--root REPO_ROOT]
Anchored by package block (not by file hash, because Cargo.lock is shared and moves under concurrent work): per package it removes and adds dependency names, keeps the list sorted, and is idempotent."""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
args = sys.argv[1:]
if "--root" in args:
    ROOT = os.path.abspath(args[args.index("--root") + 1])
mode = next((a for a in args if a in ("check", "apply")), "check")
GROUPS = ["tokio", "json", "wasm", "react"]
through = args[args.index("--through") + 1] if "--through" in args else GROUPS[-1]
allowed = GROUPS[: GROUPS.index(through) + 1]
RULES = [
    ("tokio", "semio-framework-os-mcp-plugin-runtime", ["tokio"], []),
    ("json", "semio-framework-plugin-host-imports", ["serde", "serde_json"], ["semio-framework-pack-json"]),
    ("wasm", "semio-framework-plugin-host-engine", [], ["wasmtime-wasi"]),
    ("wasm", "semio-framework-plugin-host-imports", ["wasmtime", "wasmtime-wasi"], []),
    ("wasm", "semio-framework-plugin-host-wit", ["wasmtime"], ["semio-framework-plugin-host-engine"]),
    ("wasm", "semio-framework-plugin-host-guest-runtime", ["wasmtime", "wasmtime-wasi"], []),
    ("wasm", "semio-framework-plugin-host-operation", ["wasmtime"], ["semio-framework-plugin-host-engine"]),
]
path = os.path.join(ROOT, "Cargo.lock")
text = open(path, encoding="utf8").read()
changed = 0
for group, package, remove, add in RULES:
    if group not in allowed:
        continue
    match = re.search(r'(?ms)^\[\[package\]\]\nname = "%s"\n.*?^dependencies = \[\n(.*?)^\]\n' % re.escape(package), text)
    if not match:
        print("MISSING package block:", package)
        sys.exit(2)
    names = [line.strip().strip(",").strip('"') for line in match.group(1).splitlines()]
    updated = sorted((set(names) - set(remove)) | set(add))
    if updated == names:
        print("in sync  %s" % package)
        continue
    print("%-8s %s  -%s +%s" % ("pending", package, sorted(set(names) & set(remove)), sorted(set(add) - set(names))))
    changed += 1
    body = "".join(' "%s",\n' % name for name in updated)
    text = text[: match.start(1)] + body + text[match.end(1):]
if mode == "apply" and "--write" in args and changed:
    temporary = path + ".iface-wrap.tmp"
    open(temporary, "w", encoding="utf8").write(text)
    os.replace(temporary, path)
    print("APPLIED: %d package blocks rewritten" % changed)
elif mode == "apply":
    print("DRY RUN: nothing written (use --write)")
sys.exit(1 if changed else 0)
