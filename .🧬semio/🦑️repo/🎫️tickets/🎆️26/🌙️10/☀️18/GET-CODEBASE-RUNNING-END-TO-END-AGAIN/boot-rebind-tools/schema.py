import json,os,shutil,sys
ROOT="/Users/ueli/Documents/semio"
S=ROOT+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
P=S+"/🧬️schema/🔣️.json"
BK=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/boot-rebind/schema.json.orig"
raw=open(P).read()
d=json.loads(raw)
if json.dumps(d,indent=2,ensure_ascii=False)+"\n"!=raw: sys.exit("schema not canonical")
defs=d["definitions"]
if "BootSession" in defs: print("already applied"); sys.exit(0)
if not os.path.exists(BK): shutil.copyfile(P,BK)
const=defs["ImageProtocol"]["const"]
const["bootSession"]={"program":"/usr/sbin/sysctl","arguments":["-n","kern.bootsessionuuid"],"linuxFile":"/proc/sys/kernel/random/boot_id","maximumBytes":128,"maximumAncestors":64}
mount="https://json.schemas.assets.semio-tech.com/framework/process/artifacts/native-build/storage/platform/contract/mount/component.json"
defs["BootSession"]={"type":"string","pattern":"^[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$"}
defs["Ancestry"]={"type":"array","minItems":1,"maxItems":64,"items":{"type":"string","pattern":"^[0-9]+$"}}
rec=defs["ImageReceipt"]
rec["required"]=["version","filesystemBytes","maximumPhysicalBytes","dev","ino","size","mountDirectory","bootSession","ancestry"]
rec["properties"]["bootSession"]={"$ref":"#/definitions/BootSession"}
rec["properties"]["ancestry"]={"$ref":"#/definitions/Ancestry"}
prior=json.loads(json.dumps(rec))
prior["required"]=["version","filesystemBytes","maximumPhysicalBytes","ino","size","mountDirectory"]
defs["PriorImageReceipt"]=prior
ident={"type":"object","additionalProperties":False,"required":["bootSession","dev","ino","size","mountDirectory","maximumPhysicalBytes","ancestry"],"properties":{"bootSession":{"oneOf":[{"type":"null"},{"$ref":"#/definitions/BootSession"}]},"dev":{"type":"integer","minimum":1,"maximum":9007199254740991},"ino":{"type":"integer","minimum":1,"maximum":9007199254740991},"size":{"type":"integer","minimum":1,"maximum":9007199254740991},"maximumPhysicalBytes":{"type":"integer","minimum":1,"maximum":9007199254740991},"mountDirectory":{"type":"object","additionalProperties":False,"required":["ino","mode"],"properties":{"ino":{"type":"string","pattern":"^[0-9]+$"},"mode":{"type":"integer","minimum":0,"maximum":65535}}},"ancestry":{"oneOf":[{"type":"null"},{"$ref":"#/definitions/Ancestry"}]}}}
current={"type":"object","additionalProperties":False,"required":["bootSession","dev","ino","size","allocatedBytes","mountIno","mountMode","ancestry"],"properties":{"bootSession":{"$ref":"#/definitions/BootSession"},"dev":{"type":"integer","minimum":1,"maximum":9007199254740991},"ino":{"type":"integer","minimum":1,"maximum":9007199254740991},"size":{"type":"integer","minimum":1,"maximum":9007199254740991},"allocatedBytes":{"type":"integer","minimum":0,"maximum":9007199254740991},"mountIno":{"type":"string","pattern":"^[0-9]+$"},"mountMode":{"type":"integer","minimum":0,"maximum":65535},"ancestry":{"$ref":"#/definitions/Ancestry"}}}
defs["BootRebindObservation"]={"type":"object","additionalProperties":False,"required":["version","receipt","current","attached"],"properties":{"version":{"const":1},"receipt":ident,"current":current,"attached":{"type":"boolean"}}}
defs["BootRebindCase"]={"type":"object","additionalProperties":False,"required":["name","observation","expected"],"properties":{"name":{"type":"string","minLength":1,"maxLength":96},"observation":{"$ref":"#/definitions/BootRebindObservation"},"expected":{"enum":["keep","rebind","refuse"]}}}
open(P,"w").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
print("schema patched")
