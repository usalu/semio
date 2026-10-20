#!/usr/bin/env python3
"""Generates the staged boot-recovery schema (canonical JSON, indent 2). usage: build_schema.py OUT.json"""
import json,sys

def obj(required,properties,extra=False):
    return {"type":"object","additionalProperties":extra,"required":required,"properties":properties}

REF=lambda n:{"$ref":"#/definitions/"+n}
STR=lambda lo=1,hi=256,pattern=None:({"type":"string","minLength":lo,"maxLength":hi}|({"pattern":pattern} if pattern else {}))
INT=lambda lo=0,hi=9007199254740991:{"type":"integer","minimum":lo,"maximum":hi}
FAMILIES=["publication-residue","lease-ticket","tool-server","directory-swap","persistent-record","creator-claim","image-receipt","leaked-image"]
ACTIONS=["remove-file","remove-directory","retire-ticket","retire-settled-claims","recover-directory-swap","detach-image"]
SCOPES=["stable","host-boot","image-attach","process"]

defs={}
defs["Protocol"]={"const":{
 "version":1,
 "toleranceMilliseconds":5000,
 "maximumSteps":4096,
 "maximumFacts":4096,
 "ledger":{"extension":".jsonl","maximumBytes":65536,"maximumLineBytes":2048,"retainedCommits":2},
 "publication":{"directory":".publication","files":["intent.json","receipt.json"],"directoryMode":448},
 "phases":{"lease-ticket":10,"lease-catalog":20,"publication-residue":30,"directory-swap":40,"tool-server":50,"leaked-image":60},
 "leakedImages":{"scratchRoots":["/private/tmp","/private/var/folders","/var/tmp","/tmp"],"environmentRoots":["SEMIO_TEST_ARTIFACT_DIR","TMPDIR"],"idleMilliseconds":900000,"sweepMilliseconds":300000,"maximumImages":256,"imageFile":"fixed.dmg","leaseSibling":"c","partitionPattern":"^(.+)/\\.cache/b/[a-z]/fixed\\.dmg$","leaseResourcePrefix":"native-fixed-image:","detach":{"program":"/usr/bin/hdiutil","arguments":["detach"],"maximumBytes":65536}},
 "serverFiles":{"bazel-server":["server/server.pid.txt","server/request_cookie","server/response_cookie"],"nx-daemon":["d/server-process.json","d/d.sock"]},
 "claimsDirectory":"claims",
 "bootEpoch":{"darwin":{"program":"/usr/sbin/sysctl","arguments":["-n","kern.boottime"],"pattern":"sec = ([0-9]{9,11}), usec = ([0-9]{1,6})"},"linux":{"file":"/proc/stat","pattern":"^btime ([0-9]{9,11})$"},"win32":{"program":"powershell.exe","arguments":["-NoProfile","-NonInteractive","-Command","(Get-CimInstance Win32_OperatingSystem).LastBootUpTime.ToUniversalTime().ToString('o')"],"pattern":"^([0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2})(?:\\.([0-9]{1,7}))?Z\\s*$"},"maximumBytes":65536},
 "scopes":{
  "stable":"inode, size, mode and name survive restarts and remounts and are compared exactly",
  "host-boot":"device number of the host volume is valid inside one boot session and is never compared across boots",
  "image-attach":"device number of a mounted image is valid for one attach of that image and is never compared from a record; only its equality relation between identities is compared",
  "process":"process identifiers and arrival times are valid inside one boot session; records older than the boot epoch belong to dead processes"
 },
 "identityFamilies":{
  "child-parent-identity":{"carrier":"journal ChildRequest.parentIdentity","fields":[{"name":"hostDev","scope":"host-boot"},{"name":"ino","scope":"stable"},{"name":"size","scope":"stable"},{"name":"mountDev","scope":"image-attach"}]},
  "creator-claim":{"carrier":"sidecar .native-birth-<allocation>.jsonl","fields":[{"name":"dev","scope":"image-attach"},{"name":"ino","scope":"stable"},{"name":"mode","scope":"stable"}]},
  "namespace-receipt":{"carrier":"journal Allocation.namespace","fields":[{"name":"dev","scope":"image-attach"},{"name":"ino","scope":"stable"},{"name":"mode","scope":"stable"}]},
  "image-receipt":{"carrier":"image.json","fields":[{"name":"bootSession","scope":"host-boot"},{"name":"dev","scope":"host-boot"},{"name":"ino","scope":"stable"},{"name":"size","scope":"stable"},{"name":"ancestry","scope":"stable"},{"name":"mountDirectory.dev","scope":"host-boot"},{"name":"mountDirectory.ino","scope":"stable"}]},
  "persistent-state-binding":{"carrier":"persistent-development.json","fields":[{"name":"ino","scope":"stable"},{"name":"size","scope":"stable"},{"name":"receipt","scope":"stable"}]},
  "persistent-data-binding":{"carrier":"data.binding.json","fields":[{"name":"ino","scope":"stable"}]},
  "recovery-ledger":{"carrier":"boot-recovery ledger","fields":[{"name":"boot","scope":"host-boot"},{"name":"at","scope":"process"}]}
 }
}}
defs["BootSession"]={"type":"string","pattern":"^[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$"}
defs["Sha256"]={"type":"string","pattern":"^[a-f0-9]{64}$"}
defs["Scope"]=STR(1,96,"^[A-Za-z0-9._:/-]+$")
defs["RelativePath"]=STR(1,256,"^[^/\\u0000][^\\u0000]*$")
defs["Millis"]=INT(0,281474976710655)
defs["Family"]={"enum":FAMILIES}
defs["Action"]={"enum":ACTIONS}
defs["Identity"]=obj(["dev","ino","mode"],{"dev":STR(1,20,"^[0-9]+$"),"ino":STR(1,20,"^[0-9]+$"),"mode":INT(0,65535)})
defs["Verdict"]={"enum":["keep","act","refuse"]}

defs["PublicationFact"]=obj(["family","scope","writerFree","directory","intent","receipt","unknownEntries","intentReceiptSha256","image"],{
 "family":{"const":"publication-residue"},"scope":REF("Scope"),"writerFree":{"type":"boolean"},
 "directory":{"enum":["absent","ordinary","unsafe"]},
 "intent":{"enum":["absent","torn","valid","unsafe"]},
 "receipt":{"enum":["absent","torn","valid","unsafe"]},
 "unknownEntries":INT(0,4096),
 "intentReceiptSha256":{"anyOf":[REF("Sha256"),{"type":"null"}]},
 "image":obj(["state","sha256"],{"state":{"enum":["absent","invalid","valid"]},"sha256":{"anyOf":[REF("Sha256"),{"type":"null"}]}})})
defs["TicketFact"]=obj(["family","scope","queue","name","parsed","arrivedAtMs","pid"],{
 "family":{"const":"lease-ticket"},"scope":REF("Scope"),"queue":STR(1,80,"^[a-f0-9]{64}\\.queue$"),"name":STR(1,96),"parsed":{"type":"boolean"},
 "arrivedAtMs":{"anyOf":[REF("Millis"),{"type":"null"}]},"pid":{"anyOf":[INT(1,9999999999),{"type":"null"}]}})
defs["ClaimCatalogFact"]=obj(["family","scope","claimDatabases","previousBootClaimDatabases"],{
 "family":{"const":"lease-claims"},"scope":REF("Scope"),"claimDatabases":INT(0,1048576),"previousBootClaimDatabases":INT(0,1048576)})
defs["ServerFact"]=obj(["family","scope","files"],{
 "family":{"const":"tool-server"},"scope":REF("Scope"),
 "files":{"type":"array","maxItems":16,"items":obj(["path","modifiedAtMs"],{"path":REF("RelativePath"),"modifiedAtMs":REF("Millis")})}})
defs["SwapFact"]=obj(["family","scope","writerFree","intent"],{
 "family":{"const":"directory-swap"},"scope":REF("Scope"),"writerFree":{"type":"boolean"},"intent":{"enum":["absent","present"]}})
defs["RecordFact"]=obj(["family","scope","final","temporary","pair"],{
 "family":{"const":"persistent-record"},"scope":REF("Scope"),
 "final":obj(["state","links"],{"state":{"enum":["absent","present"]},"links":INT(0,65535)}),
 "temporary":obj(["state","links"],{"state":{"enum":["absent","present"]},"links":INT(0,65535)}),
 "pair":{"enum":["not-applicable","same-inode","different-inode"]}})
defs["ClaimFact"]=obj(["family","scope","sidecar","binding"],{
 "family":{"const":"creator-claim"},"scope":REF("Scope"),"sidecar":{"enum":["absent","partial","complete"]},"binding":{"enum":["not-applicable","exact","rebound","mismatch"]}})
defs["ReceiptFact"]=obj(["family","scope","state","binding"],{
 "family":{"const":"image-receipt"},"scope":REF("Scope"),"state":{"enum":["absent","valid","unreadable"]},"binding":{"enum":["not-applicable","exact","rebound","mismatch"]}})
defs["LeakFact"]=obj(["family","scope","key","backing","location","idle","lease"],{
 "family":{"const":"leaked-image"},"scope":REF("Scope"),"key":REF("Sha256"),"backing":{"enum":["present","absent"]},"location":{"enum":["scratch","workspace","other"]},"idle":{"type":"boolean"},"lease":{"enum":["free","held","unavailable"]}})
defs["Fact"]={"oneOf":[REF("LeakFact"),REF("PublicationFact"),REF("TicketFact"),REF("ClaimCatalogFact"),REF("ServerFact"),REF("SwapFact"),REF("RecordFact"),REF("ClaimFact"),REF("ReceiptFact")]}
defs["Observation"]=obj(["version","bootSession","bootEpochMs","facts"],{
 "version":{"const":1},"bootSession":REF("BootSession"),"bootEpochMs":REF("Millis"),
 "facts":{"type":"array","maxItems":4096,"items":REF("Fact")}})
defs["Step"]=obj(["index","phase","family","scope","action","target","reason"],{
 "index":INT(0,4095),"phase":INT(0,100),"family":{"enum":["publication-residue","lease-ticket","lease-claims","tool-server","directory-swap","leaked-image"]},
 "scope":REF("Scope"),"action":REF("Action"),"target":REF("RelativePath"),"reason":STR(1,64,"^[a-z0-9-]+$")})
defs["Retained"]=obj(["family","scope","verdict","reason"],{
 "family":{"enum":FAMILIES+["lease-claims"]},"scope":REF("Scope"),"verdict":{"enum":["keep","refuse"]},"reason":STR(1,64,"^[a-z0-9-]+$")})
defs["Plan"]=obj(["version","bootSession","steps","retained"],{
 "version":{"const":1},"bootSession":REF("BootSession"),
 "steps":{"type":"array","maxItems":4096,"items":REF("Step")},
 "retained":{"type":"array","maxItems":4096,"items":REF("Retained")}})
defs["PlanCase"]=obj(["name","source","observation","plan"],{
 "name":STR(1,120),"source":{"enum":["captured","derived"]},"observation":REF("Observation"),"plan":REF("Plan")})
defs["IdentityTuple"]={"type":"array","minItems":1,"maxItems":8,"items":REF("Identity")}
defs["IdentityBindingCase"]=obj(["name","source","recorded","current","expected"],{
 "name":STR(1,120),"source":{"enum":["captured","derived"]},"recorded":REF("IdentityTuple"),"current":REF("IdentityTuple"),"expected":{"enum":["exact","rebound","mismatch"]}})
defs["BackingIdentityCase"]=obj(["name","source","recorded","current","expected"],{
 "name":STR(1,120),"source":{"enum":["captured","derived"]},"recorded":STR(1,4096),"current":STR(1,4096),"expected":{"enum":["exact","rebound","mismatch"]}})
defs["BootEpochCase"]=obj(["name","platform","text","expectedMs"],{
 "name":STR(1,120),"platform":{"enum":["darwin","linux","win32"]},"text":STR(0,4096),"expectedMs":{"anyOf":[REF("Millis"),{"type":"null"}]}})
defs["LedgerEvent"]={"oneOf":[
 obj(["v","type","boot","at","steps","plan"],{"v":{"const":1},"type":{"const":"begin"},"boot":REF("BootSession"),"at":REF("Millis"),"steps":INT(0,4096),"plan":REF("Sha256")}),
 obj(["v","type","boot","index","action","target","outcome","detail"],{"v":{"const":1},"type":{"const":"step"},"boot":REF("BootSession"),"index":INT(0,4095),"action":REF("Action"),"target":REF("RelativePath"),"outcome":{"enum":["done","skipped"]},"detail":STR(0,200)}),
 obj(["v","type","boot","at","done","skipped","retained"],{"v":{"const":1},"type":{"const":"commit"},"boot":REF("BootSession"),"at":REF("Millis"),"done":INT(0,4096),"skipped":INT(0,4096),"retained":INT(0,4096)}),
 obj(["v","type","boot","at","index","reason"],{"v":{"const":1},"type":{"const":"abort"},"boot":REF("BootSession"),"at":REF("Millis"),"index":INT(0,4096),"reason":{"enum":["cancelled","failed"]}})]}
defs["LedgerCase"]=obj(["name","text","expected"],{
 "name":STR(1,120),"text":{"type":"string","maxLength":65536},
 "expected":obj(["state","boot","corrupt","tornTail"],{"state":{"enum":["empty","committed","incomplete"]},"boot":{"anyOf":[REF("BootSession"),{"type":"null"}]},"corrupt":{"type":"boolean"},"tornTail":{"type":"boolean"}})})
defs["Corpus"]=obj(["version","plans","identities","backings","epochs","ledgers"],{
 "version":{"const":1},
 "plans":{"type":"array","minItems":1,"maxItems":256,"items":REF("PlanCase")},
 "identities":{"type":"array","minItems":1,"maxItems":256,"items":REF("IdentityBindingCase")},
 "backings":{"type":"array","minItems":1,"maxItems":256,"items":REF("BackingIdentityCase")},
 "epochs":{"type":"array","minItems":1,"maxItems":64,"items":REF("BootEpochCase")},
 "ledgers":{"type":"array","minItems":1,"maxItems":64,"items":REF("LedgerCase")}})

doc={"$schema":"http://json-schema.org/draft-07/schema#","$id":"https://semio.tech/framework/process/native-storage/boot-recovery/v1","definitions":defs}
open(sys.argv[1],"w",encoding="utf8").write(json.dumps(doc,indent=2,ensure_ascii=False)+"\n")
print("schema",sys.argv[1],len(defs),"definitions")
