"""🧹️ Retire only superseded ticket-owned native Rust libraries with no current dependency owner."""

import json
import re
import sys
import time
from collections import defaultdict
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = (ticket / "🗑️generated/native-host/debug/build").resolve()
if root.name != "build" or root.parents[2].name != ".ticket-AI-MCP-END-TO-END":
    raise SystemExit("Expected the ticket-owned native build directory")

variants = defaultdict(list)
receipts = defaultdict(list)
dependencies = defaultdict(set)
now = time.time()
for package in root.iterdir():
    if not package.is_dir() or package.is_symlink():
        continue
    for variant in package.iterdir():
        if not re.fullmatch("[0-9a-f]{16}", variant.name) or variant.is_symlink():
            continue
        ledger = variant / "fingerprint"
        files = list(ledger.glob("*.json"))
        if not files:
            continue
        age = now - max(file.stat().st_mtime for file in files)
        variants[package.name].append((age, variant))
        for file in files:
            digest = file.with_suffix("")
            if not digest.is_file():
                continue
            receipts[digest.read_text().strip()].append(variant)
            payload = json.loads(file.read_text())
            dependencies[variant].update(
                int(record[3]).to_bytes(8, "little").hex()
                for record in payload.get("deps", ())
            )

protected = set()
for package, entries in variants.items():
    keep = entries if package.startswith(("wasmtime", "cranelift", "semio-framework-test-fixtures")) else sorted(entries)[:2]
    protected.update(variant for _, variant in keep)
todo = list(protected)
while todo:
    for digest in dependencies[todo.pop()]:
        for variant in receipts[digest]:
            if variant not in protected:
                protected.add(variant)
                todo.append(variant)

obsolete = []
for entries in variants.values():
    for age, variant in entries:
        if age < 60 * 60 or variant in protected:
            continue
        output = variant / "out"
        if not output.is_symlink():
            obsolete.extend(file for file in output.glob("*.rlib") if file.is_file() and not file.is_symlink())
size = sum(file.stat().st_size for file in obsolete)
if sys.argv[1:] == ["--retire-checked"]:
    for file in obsolete:
        file.unlink()
elif len(sys.argv) != 1:
    raise SystemExit("Use the dry run or --retire-checked after checking live readers")
print(f"[DEBUG] {len(obsolete)} superseded ticket-native archives, {size / 2**30:.2f} GiB")
