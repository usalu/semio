"""🩹 Rebuilds audit8's corrected copies from the lanes' staged batches with anchored edits (each anchor exactly once, exit 1 = anchor lost).
usage: python3 -I make_fixes.py   (rewrites audit8-tools/staged/{psgate,windows,consumers}/files + manifest `new` hashes; base/ and stage.py stay the lane's)"""
import hashlib, json, pathlib, shutil, sys
C = pathlib.Path(__file__).resolve().parent.parent
OUT = C / "audit8-tools" / "staged"
EDITS = {
    "psgate": ("td-capacity-tools/staged/psgate", "🧪️tests/🚪️gate/🟦️.ts", [(
        ' const readable=liveChildPidsV1()!==undefined;\n expect(await run("waiting"),readable?"a process that only waits is lent after the authored time":"a process whose process table cannot be read is never lent").toBe(readable);',
        ' const ps=["/bin/ps","/usr/bin/ps"].find(path=>existsSync(path)),readable=process.platform==="win32"||ps!==undefined&&spawnSync(ps,["-o","pid=","-p",String(process.pid)],{encoding:"utf8",timeout:10000}).status===0;\n expect(liveChildPidsV1()!==undefined,"the production process-table reader answers exactly where an independent ps call can read the table").toBe(readable);\n expect(await run("waiting"),readable?"a process that only waits is lent after the authored time":"a process whose process table cannot be read is never lent").toBe(readable);',
    )]),
    "windows": ("td-misc-tools/staged/windows", "📏️windows/🟦️.ts", [(
        'checks.push(id==="leaf-transient"?[row.readOperations>=single,true]:[row.readOperations,single]);',
        'checks.push(id==="leaf-transient"?[row.readOperations>single&&row.readOperations<=single*churn.definitions.Protocol.const.profiles.read.maximumAttempts,true]:[row.readOperations,single]);',
    ), (
        'import schema from"./🧬️schema/🔣️.json";',
        'import schema from"./🧬️schema/🔣️.json";\nimport churn from"../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json";',
    )]),
    "consumers": ("peeredit2-tools/staged/consumers", "🔎️owner-discovery/🟨️.mjs", [(
        'else if(typeof input?.input==="string"&&Array.isArray(input.projects))for(const project of input.projects)addInput(project,input.input);}',
        'else if(input!==null&&typeof input==="object"&&!Array.isArray(input)&&Object.keys(input).length===2&&typeof input.input==="string"&&Array.isArray(input.projects)&&input.projects.length>0)for(const project of input.projects)addInput(project,input.input);else throw Error("Canonical command closure reference required: "+JSON.stringify(input).slice(0,256));}',
    )]),
}
for name, (source, suffix, edits) in EDITS.items():
    src, dst = C / source, OUT / name
    if not dst.exists(): shutil.copytree(src, dst)
    manifest = json.loads((src / "manifest.json").read_text())
    rel = next(r for r in manifest if r.endswith(suffix))
    text = (src / "files" / manifest[rel]["file"]).read_text()
    for old, new in edits:
        if text.count(old) != 1: sys.exit(f"anchor lost in {name}: {old[:80]}")
        text = text.replace(old, new)
    (dst / "files" / manifest[rel]["file"]).write_text(text)
    manifest[rel]["new"] = hashlib.sha256(text.encode()).hexdigest()
    (dst / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=1) + "\n")
    print(name, rel[-80:], manifest[rel]["new"][:12])
