import json,os,shutil,sys
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
BK=C+"/🗑️generated/boot-rebind2/backup"
L=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
O=L+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening"
STORAGE=ROOT+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json"
STAMPS=L+"/🏗️build/🧷️format-stamps/🧬️schema/🔣️.json"
RESET=O+"/♻️reset/🧬️schema/🔣️.json"
RECOVERY=O+"/♻️reset/⚡️recovery/🧬️schema/🔣️.json"
PARTITIONS=["rust","tools","graph","dependencies","products","diagnostics","nx","leases"]
def load(p):
    raw=open(p).read();d=json.loads(raw)
    if json.dumps(d,indent=2,ensure_ascii=False)+"\n"!=raw: sys.exit("not canonical: "+p)
    return d
def save(p,d):
    os.makedirs(BK,exist_ok=True)
    b=BK+"/"+p[len(ROOT)+1:].replace("/","__")
    if not os.path.exists(b): shutil.copyfile(p,b)
    open(p,"w").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
    print("patched",p[-60:])
def ordered(d,key,value,after=None):
    items=list(d.items())
    if after is None: items.append((key,value))
    else:
        i=[k for k,_ in items].index(after)+1;items.insert(i,(key,value))
    d.clear();d.update(items)

s=load(STORAGE)
c=s["definitions"]["ImageProtocol"]["const"]
if "publicationFormat" not in c:
    c["publicationFormat"]="ancestry-bound-v1"
    save(STORAGE,s)
else: print("storage already applied")

t=load(STAMPS)
proto=t["$defs"]["Protocol"]["const"]
if not any(r["id"]=="native.publication" for r in proto["stamps"]):
    proto["stamps"].append({"id":"native.publication","schema":"../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json","pointer":"/definitions/ImageProtocol/const/publicationFormat"})
    for r in proto["resources"]:
        if r["prefix"]=="native-fixed-image:": r["stamps"].append("native.publication")
    save(STAMPS,t)
else: print("stamps already applied")

r=load(RESET)
defs=r["$defs"]
if "Partition" not in defs:
    r["title"]="Inactive Recomputable Cache Partition Reset"
    ordered(defs,"Partition",{"enum":PARTITIONS},after="Protocol")
    intent=defs["Intent"]["properties"]
    intent["kind"]={"const":"inactive-reset-intent"}
    intent["partition"]={"$ref":"#/$defs/Partition"}
    defs["InactivePublicationOutcome"]["properties"]["partition"]={"$ref":"#/$defs/Partition"}
    pc=defs["InactivePublicationProtocol"]["const"]
    del pc["partition"]
    pc["ledger"]="boot-recovery"
    pc["maximumLedgerEntries"]=64
    pc["texts"]={
        "en":{"obsolete":"The cache image {partition} ({directory}) was published in an unsupported receipt format, for example before the boot-stable ancestry binding. Its contents are recomputable: it is retired and rebuilt automatically."},
        "de":{"obsolete":"Das Cache-Image {partition} ({directory}) wurde in einem nicht unterstützten Quittungsformat veröffentlicht, zum Beispiel vor der bootstabilen Vorfahrenbindung. Sein Inhalt ist neu berechenbar: es wird ausgemustert und automatisch neu aufgebaut."},
    }
    save(RESET,r)
else: print("reset already applied")

v=load(RECOVERY)
if "partition" in v["properties"]:
    v["title"]="Canonical Inactive Publication Preflight Recovery"
    v["required"].remove("partition")
    del v["properties"]["partition"]
    save(RECOVERY,v)
else: print("recovery already applied")
