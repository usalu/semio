#!/usr/bin/env python3
"""🔎️ S19 census (read-only): every norm family's mutation KINDS vs its binary op protocol records and text grammar keywords.
usage: s19-protocol-drift.py <root>"""
import glob, os, re, sys
root = sys.argv[1]
for mutations in sorted(glob.glob(os.path.join(root, "✏️s/🔌️plugins/📕️norm/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"))):
    family = mutations.split("/🗿️artifacts/")[1].split("/")[0]
    text = open(mutations, encoding="utf-8").read()
    block = re.search(r"pub const KINDS: &\[&str\] = &\[(.*?)\];", text, re.S)
    kinds = re.findall(r'"([a-z0-9-]+)"', block.group(1)) if block else []
    base = os.path.dirname(mutations)
    proto = os.path.join(base, "💾️binary/📡️.protocol.semio")
    records = re.findall(r"^record ([a-z0-9-]+) tag=", open(proto, encoding="utf-8").read(), re.M) if os.path.exists(proto) else None
    missing = [k for k in kinds if records is not None and k not in records]
    extra = [r for r in (records or []) if r not in kinds]
    print(f"{family}: kinds={len(kinds)} records={None if records is None else len(records)} missing={len(missing)} extra={len(extra)}")
    if missing: print(f"  missing: {missing}")
    if extra: print(f"  extra: {extra[:12]}{' …' if len(extra) > 12 else ''}")
