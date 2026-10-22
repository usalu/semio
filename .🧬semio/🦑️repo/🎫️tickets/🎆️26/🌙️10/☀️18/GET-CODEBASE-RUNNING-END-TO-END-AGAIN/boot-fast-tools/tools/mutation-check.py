#!/usr/bin/env python3
"""mutation-check.py [OVERLAY] : seeds defects into a clone of the verified overlay (default /private/tmp/boot-fast/ov3 -> ovmut) and requires the boot-fast tests to fail for every one of them."""
import os,shutil,subprocess,sys
src=sys.argv[1] if len(sys.argv)>1 else "/private/tmp/boot-fast/ov3";ov="/private/tmp/boot-fast/ovmut"
shutil.rmtree(ov,ignore_errors=True);subprocess.run(["cp","-cR",src,ov],check=True)
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
DOOR=B+"🚪️door/";DT=DOOR+"🧪️tests/🟦️.ts";LAW=DOOR+"🧪️tests/🚦️startup/🟦️.ts";TAX=B+"🧭️invocation/🧪️tests/🧊️taxonomy/🟦️.ts";GLO=B+"📥️caller/🧪️tests/🌐️globals/🟦️.ts";VAL="🧰️framework/🔨️modules/🧬️schema/✅️validator/";VT=VAL+"🧪️tests/🏎️patterns/🟦️.ts"
M=[
 ("tool value not validated",DOOR+"🟦️.ts","&&allowed&&!allowed.includes(value))return {refused","&&allowed&&false)return {refused",[DT]),
 ("unknown command accepted",DOOR+"🟦️.ts","if(command!==undefined&&!command.startsWith(\"-\")&&!row)return","if(false)return",[DT]),
 ("closed subcommand set ignored",DOOR+"🟦️.ts","rules.closedSubcommands.includes(row.name)&&","false&&",[DT]),
 ("dashboard target not passed through",DOOR+"🟦️.ts","if(command===\"run\"&&rules.fullTargetPrefixes.some","if(false&&rules.fullTargetPrefixes.some",[DT]),
 ("short help flag ignored",DOOR+"🟦️.ts",'argument==="--help"||argument==="-h"','argument==="--help"',[DT]),
 ("graph policy value kept",DOOR+"🟦️.ts",'if(argument==="--graph-policy"){index++;continue;}','if(argument==="--graph-policy"){continue;}',[DT]),
 ("worker switch not validated",DOOR+"🟦️.ts",'(option==="--tool"||value!=="")&&allowed','false&&allowed',[DT]),
 ("passthrough routes planned",DOOR+"🟦️.ts",'if(rules.passthrough.includes(args[0]??""))return full;','',[DT]),
 ("refusal exit code ignored",DOOR+"🟦️.ts",'ports.error(renderBuildDoorRefusalV1(route,plan.language,rules));ports.exit(rules.refusedExitCode);','ports.error(renderBuildDoorRefusalV1(route,plan.language,rules));ports.exit(1);',[DT]),
 ("placeholder not replaced",DOOR+"🟦️.ts",'route.values[name]??`{${name}}`','`{${name}}`',[DT]),
 ("light command not routed",DOOR+"🔣️.json",'"operation": "capacity", ','"operation": "capacityx", ',[DT]),
 ("language ignored in help",DOOR+"🟦️.ts",'ports.write(buildHelpForArgumentsV1(route.args,localized))','ports.write(buildHelpForArgumentsV1(route.args,{...localized,SEMIO_LANGUAGE:"en"}))',[DT]),
 ("root found from the wrong directory",DOOR+"🟦️.ts",'let directory=resolve(start.trim()||','let directory=resolve(dirname(start.trim())||',[DT]),
 ("door statically imports the graph",DOOR+"🟦️.ts",'import {profilePhaseV1} from "../⏱️profile/🟨️.mjs";','import {profilePhaseV1} from "../⏱️profile/🟨️.mjs";\nimport "../🕸️graph/🟦️.ts";',[LAW]),
 ("door statically imports the invocation parser",DOOR+"🟦️.ts",'import {profilePhaseV1} from "../⏱️profile/🟨️.mjs";','import {profilePhaseV1} from "../⏱️profile/🟨️.mjs";\nimport "../🧭️invocation/🟦️.ts";',[LAW]),
 ("door loads a light command eagerly",DOOR+"🟦️.ts",'import {profilePhaseV1} from "../⏱️profile/🟨️.mjs";','import {profilePhaseV1} from "../⏱️profile/🟨️.mjs";\nimport "../🔒️leases/⌨️command/🟦️.ts";',[LAW]),
 ("caller loads cargo execution statically",B+"📥️caller/🟦️.ts",'import {createRequire} from "node:module";','import {createRequire} from "node:module";\nimport {issueRepositoryCargoExecutionBackendV1} from "../../🏃️process/🦀️cargo/🏃️execution/🟦️.ts";void issueRepositoryCargoExecutionBackendV1;',[LAW]),
 ("caller declared selection ignores global options",B+"📥️caller/🟦️.ts","request.push(...buildOperationArgumentsV1(selected));","request.push(...selected);",[GLO]),
 ("caller declared selection keeps the graph policy",B+"📥️caller/🟦️.ts","if(argument===\"--graph-policy\"){index++;continue;}if(argument.startsWith(\"--graph-policy=\")){continue;}","",[GLO]),
 ("linear path pattern drops the NUL exclusion",VAL+"🟦️.ts",r"\\\u0000\n\r\u2028",r"\\\n\r\u2028",[VT]),
 ("linear path pattern allows dotdot segments",VAL+"🟦️.ts",r"(?:(?!\.\.(?:/|$))[^/",r"(?:[^/",[VT]),
 ("taxonomy parsed on every selection",B+"🧭️invocation/🟦️.ts",'if(prior?.identity===identity)return prior.value;','',[TAX]),
 ("taxonomy keyed by path only",B+"🧭️invocation/🟦️.ts",'identity=[row.dev,row.ino,row.size,row.mtimeNs,row.ctimeNs].join(":")','identity="fixed"',[TAX]),
]
failed=[];print(f"{len(M)} seeded defects")
for name,rel,old,new,tests in M:
    path=os.path.join(ov,rel);text=open(path,encoding="utf8").read()
    if text.count(old)!=1:print(f"SKIP  {name}: anchor occurs {text.count(old)}x");failed.append(name);continue
    open(path,"w",encoding="utf8").write(text.replace(old,new))
    try:
        r=subprocess.run(["nice","-n","10","bun","test"]+["./"+t for t in tests],cwd=ov,capture_output=True,text=True,timeout=300,env={**os.environ,"SEMIO_TEST_ARTIFACT_DIR":"/private/tmp/boot-fast/art"})
        killed=r.returncode!=0
    finally:open(path,"w",encoding="utf8").write(text)
    print(("KILLED " if killed else "SURVIVED ")+name)
    if not killed:failed.append(name)
print(f"{len(M)-len(failed)} of {len(M)} killed");sys.exit(1 if failed else 0)
