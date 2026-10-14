#!/usr/bin/env python3
"""🧬️ Makes every mutation kind of one artifact generic over the dispatch enum (P5 decision D2): the kind's
`impl MutationKind<Snapshot, Union>` becomes `impl<Op> MutationKind<Snapshot, Op> where Op: Mutation<Snapshot, Diff = D> +
From<K>…` and its inverse builds `Op::from(payload)` instead of `Union::Variant(payload)`, so kinds depend on the model only.

usage: 🧬️generic-kinds.py <subsets dir> <Union> <Snapshot> <Diff path> <union file>
"""
import os, re, sys, glob
subsets, union, snapshot, diff_path, union_file = sys.argv[1:6]
u = open(union_file, encoding="utf-8").read()
body = u[u.index(f"pub enum {union} {{"):]
body = body[:body.index("\n}")]
variants = dict(re.findall(r"\n\s+(\w+)\((\w+::\w+)\),", body))
print("variants", len(variants))
def paren(s, i):
    d = 0
    for j in range(i, len(s)):
        if s[j] == "(": d += 1
        elif s[j] == ")":
            d -= 1
            if d == 0: return j
    raise SystemExit("unbalanced")
for kind_dir in sorted(glob.glob(os.path.join(subsets, "*", "🧬️schema", "🧬️mutations", "*"))):
    kf, inv = os.path.join(kind_dir, "🦀️.rs"), os.path.join(kind_dir, "↩️inverse", "🦀️.rs")
    if not (os.path.exists(kf) and os.path.exists(inv)): continue
    if "impl<Op> MutationKind" in open(kf, encoding="utf-8").read(): continue
    s = open(inv, encoding="utf-8").read()
    used = []
    out, i = [], 0
    for m in re.finditer(re.escape(union) + r"::(\w+)\(", s):
        if m.start() < i: continue
        close = paren(s, m.end() - 1)
        out.append(s[i:m.start()]); out.append("Op::from(" + s[m.end():close] + ")"); i = close + 1
        ty = variants[m.group(1)]
        if ty not in used: used.append(ty)
    out.append(s[i:]); s = "".join(out)
    bounds = " + ".join(f"From<{t}>" for t in used)
    s, n = re.subn(r"pub fn inverse\((.*?)\) -> Result<Vec<" + re.escape(union) + r">, semio_framework_value::ValueError>", lambda m: f"pub fn inverse<Op: {bounds}>({m.group(1)}) -> Result<Vec<Op>, semio_framework_value::ValueError>", s)
    assert n == 1, inv
    s = re.sub(r"use ([\w:]+)::mutations::\{([^}]*)\};", lambda m: (lambda items: f"use {m.group(1)}::mutations::{{{', '.join(items)}}};" if len(items) > 1 else (f"use {m.group(1)}::mutations::{items[0]};" if items else ""))([x.strip() for x in m.group(2).split(",") if x.strip() and x.strip() != union]), s)
    s = re.sub(r"use [\w:]+::mutations::" + re.escape(union) + r";\n", "", s)
    open(inv, "w", encoding="utf-8").write(s)
    k = open(kf, encoding="utf-8").read()
    imports = sorted({t.split("::")[0] for t in used})
    k = re.sub(r"use [\w:]+::mutations::" + re.escape(union) + r";\n", "", k)
    k = re.sub(r"use ([\w:]+)::mutations::\{([^}]*)\};", lambda m: (lambda items: f"use {m.group(1)}::mutations::{{{', '.join(items)}}};" if len(items) > 1 else (f"use {m.group(1)}::mutations::{items[0]};" if items else ""))([x.strip() for x in m.group(2).split(",") if x.strip() and x.strip() != union]), k)
    own = re.search(r"impl MutationKind<" + re.escape(snapshot) + r", " + re.escape(union) + r"> for (\w+) \{", k)
    assert own, kf
    k = k.replace(own.group(0), f"impl<Op> MutationKind<{snapshot}, Op> for {own.group(1)}\nwhere\n    Op: protocol::Mutation<{snapshot}, Diff = {diff_path}> + {bounds},\n{{")
    k = k.replace(f"-> Result<Vec<{union}>, semio_framework_value::ValueError>", "-> Result<Vec<Op>, semio_framework_value::ValueError>")
    assert union not in k, (kf, [l for l in k.split("\n") if union in l][:3])
    # the bound names payload types of sibling kinds through their module path
    for t in used:
        mod = t.split("::")[0]
        if mod == os.path.basename(kind_dir).split("️")[-1].replace("-", "_"): continue
        if not re.search(r"\b" + mod + r"\b", k.split("impl<Op>")[0]):
            k = re.sub(r"(\nuse protocol::)", f"\nuse crate::standards::v1::subsets::any::schema::mutations::{mod};\\1", k, count=1)
    open(kf, "w", encoding="utf-8").write(k)
    print(os.path.basename(kind_dir), bounds)
