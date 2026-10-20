#!/bin/bash
# usage: harness.sh   copies the harness patch into the overlay adapter (exports the owner internals, appends the scratch host); idempotent
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
python3 -I - <<'PY'
import os,sys
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sys.path.insert(0,"/Users/ueli/Documents/semio/"+C+"/bzl-coverage-tools")
import edits
path=os.path.join("/private/tmp/bzlcov-ov",edits.SCRIPT_PATH)
text=open(path,encoding="utf8").read()
marker="/** 🧪️ A caller-provided Bazel host layout"
if marker not in text:
    for old in ("async function writeBazelRuntimeV1(","async function bazelCancellationControl("):
        assert text.count(old)==1,old
        text=text.replace(old,"export "+old)
    store='import{withRepositoryCacheStoreV1}from"../../../'
    start=text.index(store); end=text.index(";",start)+1
    text=text[:start]+"const withRepositoryCacheStoreV1=async(_root:string,_name:string,_invocation:unknown,phase:any,body:(graph:unknown,phase:any)=>Promise<any>)=>body(null,phase);"+text[end:]
    launcher='outputs = [launcher], mnemonic = "RepoLauncher", execution_requirements = _requirements(ctx))'
    assert text.count(launcher)==1
    text=text.replace(launcher,launcher[:-1]+", use_default_shell_env = True)")
    text+=open("/Users/ueli/Documents/semio/"+C+"/bzl-coverage-tools/scratch-host.part.ts",encoding="utf8").read()
    open(path,"w",encoding="utf8").write(text)
    print("patched",len(text))
else: print("already patched")
PY
