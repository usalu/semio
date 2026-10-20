#!/usr/bin/env python3
"""Idempotently installs the x-keys regression contract (fixture, schema, test) into the Bazel adapter's test tree; prints one line per file: created, updated or unchanged."""
import pathlib,sys
here=pathlib.Path(__file__).resolve().parent
root=pathlib.Path(sys.argv[1]).resolve() if len(sys.argv)>1 else next(p for p in here.parents if (p/"AGENTS.md").exists())
if not (root/"AGENTS.md").exists(): sys.exit("repository root (with AGENTS.md) required: "+str(root))
target=root/"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests"
for source in sorted((here/"files").rglob("*")):
    if not source.is_file(): continue
    destination=target/source.relative_to(here/"files")
    destination.parent.mkdir(parents=True,exist_ok=True)
    data=source.read_bytes()
    state="unchanged" if destination.exists() and destination.read_bytes()==data else ("updated" if destination.exists() else "created")
    if state!="unchanged": destination.write_bytes(data)
    print(state,destination.relative_to(root))
