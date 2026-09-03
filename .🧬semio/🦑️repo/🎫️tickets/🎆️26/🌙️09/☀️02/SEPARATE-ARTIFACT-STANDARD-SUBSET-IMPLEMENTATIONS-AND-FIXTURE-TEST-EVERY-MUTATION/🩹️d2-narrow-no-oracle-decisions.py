import json, sys, io

NOTE = (
    " [Narrowed 2026-09-02, ticket 26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-"
    "FIXTURE-TEST-EVERY-MUTATION shard D2: capabilities emptied because a real mutationManifest now "
    "requires a qualifying third-party oracle for this exact capability, and noOracleMisuseBreaches "
    "correctly refuses to let a recorded decision paper over a real runtime requirement (per shard "
    "A10's precedent for mathematical/sequence/draw) -- the honest state is a visible "
    "missing-external-oracle gap, not a hidden one. Rationale above is kept for the record: it still "
    "explains why no third-party library is credible for this domain, which is exactly why the gap "
    "is expected to stay open rather than be filled.]"
)

targets = [
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "raw-buffer-no-format"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/🧪️oracle/🔣️.json", "jpg-jfif-1-01-baseline-conformance-class-semantics"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "dwg-ac1018-proprietary-container"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "dwg-ac1024-proprietary-container"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧪️oracle/🔣️.json", "tiff-6-0-baseline-conformance-class-semantics"),
]

for path, decid in targets:
    with io.open(path, encoding="utf-8") as f:
        text = f.read()
    d = json.loads(text)
    found = False
    for nd in d.get("noOracleDecisions", []):
        if nd.get("id") == decid:
            before = nd["capabilities"]
            nd["capabilities"] = []
            nd["rationale"] = nd["rationale"] + NOTE
            found = True
            print(path, decid, "capabilities", before, "-> []")
    if not found:
        print("MISSING", path, decid)
        sys.exit(1)
    # preserve 2-space indent, trailing newline, non-ascii unescaped
    out = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    with io.open(path, "w", encoding="utf-8") as f:
        f.write(out)
