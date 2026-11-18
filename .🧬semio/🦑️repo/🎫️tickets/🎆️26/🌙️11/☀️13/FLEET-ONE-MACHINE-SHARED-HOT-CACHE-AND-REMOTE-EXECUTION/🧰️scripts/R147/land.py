import hashlib,os,shutil,subprocess,sys,json
R="/Users/ueli/Documents/semio/"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R147/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/";OP=PN+"🌐️owner-pool/"
PROC="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
# live, stage, base (None: new file)
rows=[
 (OP+"🟦️.ts","stage/pool.ts","base/pool.ts"),
 (OP+"🧬️schema/🔣️.json","stage/schema.json","base/schema.json"),
 (OP+"🧪️tests/🧫️fixtures/🔣️.json","stage/fixture.json","base/fixture.json"),
 (OP+"🧪️tests/🧫️fixtures/🐍️oracle.py","stage/oracle.py","base/oracle.py"),
 (OP+"🧪️tests/🎛️one-budget/🟦️.ts","stage/onebudget.test.ts","base/onebudget.test.ts"),
 (OP+"🧪️tests/🧮️shared-budget/🟦️.ts","stage/shared.test.ts",None),
 (PN+"🟨️.mjs","stage/pn.mjs","base/pn.mjs"),
 (B+"🧠️memory/🟨️.mjs","stage/memory.mjs","base/memory.mjs"),
 (B+"🧠️memory/🟨️.d.mts","stage/memory.d.mts","base/memory.d.mts"),
 (B+"🧠️memory/🧪️tests/🙋️ask/🟦️.ts","stage/governor.test.ts",None),
 (B+"🌐️remote-execution/🌡️load/🟦️.ts","stage/load.ts","base/load.ts"),
 (B+"🌐️remote-execution/🛰️sidecar/🏊️pool/🟦️.ts","stage/sidecar.ts","base/sidecar.ts"),
 (B+"🌐️remote-execution/🛰️sidecar/🧪️tests/🏊️pool/🟦️.ts","stage/sidecar.test.ts","base/sidecar.test.ts"),
 (PROC+"🙋️asks/🟦️.ts","mirror/🙋️asks/🟦️.ts",None),
 (PROC+"🙋️asks/🧬️schema/🔣️.json","mirror/🙋️asks/🧬️schema/🔣️.json",None),
 (PROC+"🙋️asks/🧪️tests/🟦️.ts","stage/asks.test.ts",None),
]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
mode=sys.argv[1]
os.makedirs(S+"backup",exist_ok=True)
manifest=S+"landed.json"
if mode=="dry":
    bad=0
    for live,stage,base in rows:
        p=R+live
        if base is None:
            print("new  ",os.path.exists(p) and "EXISTS" or "ok",live[-60:]);bad+=os.path.exists(p)
        else:
            same=sha(p)==sha(S+base);print("same " if same else "CHANGED",live[-60:])
    sys.exit(1 if bad else 0)
if mode=="land":
    done=[]
    try:
        for i,(live,stage,base) in enumerate(rows):
            p=R+live;src=S+stage
            if base is None:
                assert not os.path.exists(p),p
                os.makedirs(os.path.dirname(p),exist_ok=True);shutil.copyfile(src,p);done.append({"live":live,"backup":None})
            else:
                bk=S+"backup/%02d"%i;shutil.copyfile(p,bk)
                if sha(p)==sha(S+base):
                    shutil.copyfile(src,p)
                else:
                    tmp=S+"merge.tmp";shutil.copyfile(p,tmp)
                    r=subprocess.run(["git","merge-file",tmp,S+base,src],capture_output=True)
                    assert r.returncode==0,("conflict",live)
                    shutil.copyfile(tmp,p)
                done.append({"live":live,"backup":bk,"after":sha(p)})
        json.dump(done,open(manifest,"w"))
    except Exception as e:
        json.dump(done,open(manifest,"w"));print("FAILED",e);sys.exit(3)
if mode=="restore":
    done=json.load(open(manifest))
    for row in done:
        p=R+row["live"]
        if row["backup"] is None:
            if os.path.exists(p):os.remove(p)
        else:shutil.copyfile(row["backup"],p)
    for d in [PROC+"🙋️asks/🧪️tests",PROC+"🙋️asks/🧬️schema",PROC+"🙋️asks",OP+"🧪️tests/🧮️shared-budget",B+"🧠️memory/🧪️tests/🙋️ask"]:
        try:os.rmdir(R+d)
        except OSError:pass
    print("restored",len(done))
if mode=="status":
    for row in json.load(open(manifest)):print(row["live"][-50:],row.get("after"))
