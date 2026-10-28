#!/usr/bin/env python3
"""Mutation check of the staged source memo: each mutant must make the memo test fail."""
import subprocess, shutil, sys, os
R = "/private/tmp/noopg/stage"
A = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers"
memo = f"{R}/{A}/🧊️memo/🟦️.ts"; auth = f"{R}/{A}/🟦️.ts"
mutants = [
 ("skip directory identity", memo, "const directories=await pool(memo.directories,row=>directoryHoldsV1(row,base),signal);\n if(!directories)return undefined;", ""),
 ("skip absent/listing observations", memo, " if(!observed)return undefined;", ""),
 ("accept any digest", memo, "return digestOf(bytes)===sha256?bytes:undefined;", "return bytes;"),
 ("accept hard links", memo, "if(!before.isFile()||before.nlink!==1n||before.size>MAXIMUM_FILE_BYTES)return undefined;", "if(!before.isFile())return undefined;"),
 ("skip package proof", auth, "if(row.candidate===null||candidate!==row.candidate||mark!==row.mark)return undefined;", ""),
 ("ignore installed flag", auth, "||memo.installed!==installed)return undefined;", ")return undefined;"),
 ("ignore implementation", memo, "||memo.implementation!==nxAuthorizerMemoImplementationV1())return undefined;", ")return undefined;"),
 ("ignore root", memo, "if(memo.version!==1||memo.root!==base||", "if(memo.version!==1||"),
 ("serve other entry", auth, "if(!memo||memo.entry!==entry||", "if(!memo||"),
]
base = {p: open(p, encoding="utf8").read() for p in (memo, auth)}
failed = 0
for name, path, old, new in mutants:
    text = base[path]
    if text.count(old) != 1:
        print("ANCHOR", name, text.count(old)); failed += 1; continue
    for p, t in base.items(): open(p, "w", encoding="utf8").write(t)
    open(path, "w", encoding="utf8").write(text.replace(old, new))
    r = subprocess.run(["bun", "test", f"./{A}/🧪️tests/🧊️memo/🟦️.ts"], cwd=R, capture_output=True, text=True)
    killed = r.returncode != 0
    print(("KILLED  " if killed else "SURVIVED"), name)
    failed += 0 if killed else 1
for p, t in base.items(): open(p, "w", encoding="utf8").write(t)
sys.exit(1 if failed else 0)
