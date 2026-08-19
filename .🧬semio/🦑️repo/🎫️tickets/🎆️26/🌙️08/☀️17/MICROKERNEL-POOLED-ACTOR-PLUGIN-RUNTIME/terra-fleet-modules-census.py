#!/usr/bin/env python3
# 🔎️ Census of ✏️s/🔨️modules (minus 🏗️fem, owned elsewhere) for the U-program async/dyn program.
import os
import re
import json

ROOT = "✏️s/🔨️modules"
EXCLUDE_DIRS = {"🏗️fem"}  # owned by another agent per brief

RUST_EXT = ".rs"
TS_EXT = (".ts", ".tsx")

# 🧵️ patterns
RE_ASYNC_FN = re.compile(r'\basync\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)')
RE_PLAIN_FN = re.compile(r'(?<!async\s)\bfn\s+([A-Za-z_][A-Za-z0-9_]*)')
RE_DYN = re.compile(r'\bdyn\s+([A-Za-z_][A-Za-z0-9_:<>]*)')
RE_TAG = re.compile(r'🚫️async:\s*(E\d)')
RE_TEST_ASYNC_FN = re.compile(r'#\[test\]\s*(?:\n\s*)?(?:pub\s+)?async\s+fn')
RE_ASYNC_TRAIT_ATTR = re.compile(r'#\[async_trait\]')
RE_BLOCK_ON = re.compile(r'\bblock_on\s*\(')
RE_TRAIT_DECL = re.compile(r'\btrait\s+([A-Za-z_][A-Za-z0-9_]*)')
RE_IMPL_DECL = re.compile(r'\bimpl(?:<[^>]*>)?\s+([A-Za-z_][A-Za-z0-9_:]*)(?:<[^>]*>)?\s+for\s+')

STD_DYN_ALLOW = {"Future", "Fn", "FnMut", "FnOnce", "Any", "Error"}

def walk_files():
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in EXCLUDE_DIRS]
        for f in sorted(filenames):
            yield os.path.join(dirpath, f)

def census_rust(path, text):
    async_fns = RE_ASYNC_FN.findall(text)
    all_fn_starts = [m.start() for m in RE_PLAIN_FN.finditer(text)]
    async_fn_starts = set(m.start() for m in RE_ASYNC_FN.finditer(text))
    # RE_PLAIN_FN already excludes "async fn" via negative lookbehind on 'async\s' before 'fn'
    plain_fns = [m.group(1) for m in RE_PLAIN_FN.finditer(text)]
    dyn_uses = []
    for m in RE_DYN.finditer(text):
        name = m.group(1)
        base = name.split("<")[0].split("::")[-1]
        if base not in STD_DYN_ALLOW:
            dyn_uses.append(name)
    tags = RE_TAG.findall(text)
    test_async_residue = RE_TEST_ASYNC_FN.findall(text)
    async_trait_attrs = RE_ASYNC_TRAIT_ATTR.findall(text)
    block_ons = RE_BLOCK_ON.findall(text)
    traits = RE_TRAIT_DECL.findall(text)
    impls = RE_IMPL_DECL.findall(text)
    return {
        "path": path,
        "async_fn_count": len(async_fns),
        "plain_fn_count": len(plain_fns),
        "dyn_uses": dyn_uses,
        "tag_count": len(tags),
        "tags": tags,
        "test_async_fn_residue": len(test_async_residue),
        "async_trait_attr_count": len(async_trait_attrs),
        "block_on_count": len(block_ons),
        "traits_declared": traits,
        "impls_for": impls,
    }

def census_ts(path, text):
    async_fns = len(re.findall(r'\basync\s+(function\b|\w+\s*\()', text))
    return {"path": path, "async_markers": async_fns, "size": len(text)}

results_rust = []
results_ts = []
other_files = []

for path in walk_files():
    if path.endswith(RUST_EXT):
        with open(path, "r", encoding="utf-8", errors="replace") as fh:
            text = fh.read()
        results_rust.append(census_rust(path, text))
    elif path.endswith(TS_EXT):
        with open(path, "r", encoding="utf-8", errors="replace") as fh:
            text = fh.read()
        results_ts.append(census_ts(path, text))
    else:
        other_files.append(path)

print("=== RUST FILES ===")
for r in results_rust:
    print(json.dumps(r, ensure_ascii=False))

print("\n=== TS FILES ===")
for r in results_ts:
    print(json.dumps(r, ensure_ascii=False))

print("\n=== OTHER FILES ===")
for p in other_files:
    print(p)

print("\n=== TOTALS ===")
total_async = sum(r["async_fn_count"] for r in results_rust)
total_plain = sum(r["plain_fn_count"] for r in results_rust)
total_dyn = sum(len(r["dyn_uses"]) for r in results_rust)
total_tags = sum(r["tag_count"] for r in results_rust)
total_test_residue = sum(r["test_async_fn_residue"] for r in results_rust)
total_async_trait = sum(r["async_trait_attr_count"] for r in results_rust)
total_block_on = sum(r["block_on_count"] for r in results_rust)
print(f"rust files: {len(results_rust)}")
print(f"async fn total: {total_async}")
print(f"plain fn total: {total_plain}")
print(f"first-party dyn uses total: {total_dyn}")
print(f"🚫️async tag total: {total_tags}")
print(f"#[test] async fn residue total: {total_test_residue}")
print(f"#[async_trait] attr total: {total_async_trait}")
print(f"block_on total: {total_block_on}")
print(f"ts files: {len(results_ts)}")
