#!/usr/bin/env python3
"""Kills-or-fails proof: each mutation of the patched settlement function must make the repeat law fail. Usage: mutation-check.py PATCHED_OBSERVATION_MJS"""
import pathlib,subprocess,sys,os,shutil
HERE=pathlib.Path(__file__).resolve().parent
patched=pathlib.Path(sys.argv[1]);text=patched.read_text(encoding="utf8")
run=HERE/"🗑️generated/run-test"
mutations={
 "no-repeat-rule":("||signature===previousFailure",""),
 "no-restart-rule":("failure?.code===\"BUILD_AUTHORITY_CHANGED\"||",""),
 "signature-ignores-message":("JSON.stringify([failure?.code??null,(()=>{try{return String(failure?.message??failure);}catch{return\"\";}})()])","JSON.stringify([failure?.code??null])"),
 "signature-ignores-code":("JSON.stringify([failure?.code??null,","JSON.stringify([null,"),
 "conflict-keeps-signature":("  previousFailure=signature;\n","  if(signature!==undefined)previousFailure=signature;\n"),
 "signature-never-remembered":("  previousFailure=signature;\n",""),
 "repeat-ignores-failed":("if(failed&&(failure?.code","if((failure?.code"),
}
killed=0
for name,(a,b) in mutations.items():
    if text.count(a)!=1:print("ANCHOR",name,text.count(a));sys.exit(2)
    out=HERE/"🗑️generated"/("mutant-"+name+".mjs");out.write_text(text.replace(a,b),encoding="utf8")
    r=subprocess.run(["bun","test","./🟦️.ts"],cwd=run,env={**os.environ,"SETTLE_SOURCE":str(out)},capture_output=True,text=True)
    dead=r.returncode!=0;killed+=dead;print(("killed  " if dead else "SURVIVED"),name);out.unlink()
print(f"{killed} of {len(mutations)} killed")
sys.exit(0 if killed==len(mutations) else 1)
