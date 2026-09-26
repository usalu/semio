#!/usr/bin/env python3
"""📝️ Prepared patch (rule 20, apply after W2's `--packages all`): `📝️mutate-txt-utf-8`'s two `set-trailing-newline`
rows are red in the ORACLE role only. The feature documents them as a refusal (the fixture's last line is empty, so
`false` has no representable result); the reference module states that refusal as an `Err` naming the loss, which its
own unit laws assert — but the case adapter's oracle handlers propagated that `Err` as a failed row, while the subject
handlers already answer the documented refusal with the untouched document. The oracle handlers now answer it the same
way, through one helper that admits exactly that kind and that reason, so any other refusal still fails the row.
Usage: txt-oracle-refusal.py --dry-run | --write [--root <dir>]"""
import sys
from pathlib import Path

root = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
PATH = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧪️tests/📝️mutate-txt-utf-8/🦀️.rs"
EDITS = [
    ("//#region 🔖️Oracle\nfn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let spec = ctx.doc_json()?;\n    let output = oracle_apply_mutation(&input, &spec)?;\n",
     "//#region 🔖️Oracle\n"
     "/// 🔒️ The reference's forward application with the feature's one documented refusal answered as the subject answers\n"
     "/// it: `set-trailing-newline` whose result is not representable leaves the bytes exactly where they were. Any other\n"
     "/// kind, or any other reason, is still an error — a reference that started refusing everything fails the row.\n"
     "fn oracle_apply_or_documented_refusal(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {\n"
     "    match oracle_apply_mutation(input, spec) {\n"
     "        Err(refusal) if spec.str(\"kind\") == \"set-trailing-newline\" && refusal.contains(\"not representable\") => Ok(input.to_vec()),\n"
     "        applied => applied,\n"
     "    }\n"
     "}\n\n"
     "fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let spec = ctx.doc_json()?;\n    let output = oracle_apply_or_documented_refusal(&input, &spec)?;\n"),
    ("    let spec = ctx.doc_json()?;\n    let mutated = oracle_apply_mutation(&input, &spec)?;\n    let undo = oracle_inverse_spec(&input, &spec)?;\n",
     "    let spec = ctx.doc_json()?;\n    let mutated = oracle_apply_or_documented_refusal(&input, &spec)?;\n    let undo = oracle_inverse_spec(&input, &spec)?;\n"),
]
write = "--write" in sys.argv
source = PATH.read_text(encoding="utf-8")
problems = ["already applied"] if "fn oracle_apply_or_documented_refusal(" in source else []
for old, new in EDITS:
    if source.count(old) != 1:
        problems.append(f"{source.count(old)} × {old[:70]!r}")
        continue
    source = source.replace(old, new)
print(f"files=1 problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    PATH.write_text(source, encoding="utf-8")
