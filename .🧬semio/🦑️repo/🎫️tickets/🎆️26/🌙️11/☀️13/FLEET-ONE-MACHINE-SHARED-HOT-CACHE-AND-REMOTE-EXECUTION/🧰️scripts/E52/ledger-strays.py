import json,sys,os
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/"
def rw(path,fn):
    text=open(path,encoding="utf8").read();new=fn(text)
    if new!=text:open(path,"w",encoding="utf8").write(new)
def once(text,anchor,add,after=True):
    assert text.count(anchor)==1,(anchor,text.count(anchor))
    return text.replace(anchor,anchor+add if after else add+anchor)
def schema(text):
    s=json.loads(text)
    if "strays" in s["properties"]:return text
    s["properties"]["strays"]={"$ref":"#/$defs/strays"}
    defs={}
    for k,v in s["$defs"].items():
        defs[k]=v
        if k=="retirement":
            defs["strays"]={"description":"Directories below the checkout that carry the name of a cache root (the directory names of the loose roots) but are not that root: tools run from a member directory leave them. They hold nothing anyone reads, so the collector removes their entries once they have been quiet and nothing holds them. Directories inside an ignored segment or an excluded directory are not looked at.","type":"object","additionalProperties":False,"required":["quietMs","ignoredSegments","excludedDirectories","reason"],"properties":{"quietMs":{"$ref":"#/$defs/milliseconds"},"ignoredSegments":{"type":"array","maxItems":16,"uniqueItems":True,"items":{"type":"string","minLength":1,"maxLength":64}},"excludedDirectories":{"type":"array","maxItems":16,"uniqueItems":True,"items":{"type":"string","minLength":1,"maxLength":64}},"reason":{"$ref":"#/$defs/bilingual"}}}
    s["$defs"]=defs
    return json.dumps(s,ensure_ascii=False,indent=2)+"\n"
def policy(text):
    p=json.loads(text)
    if "strays" in p:return text
    p["strays"]={"quietMs":900000,"ignoredSegments":["node_modules",".git"],"excludedDirectories":[".🧬semio"],"reason":{"en":"A cache root outside the checkout root: a tool ran from a member directory. Nothing reads it; the entries are removed once quiet and unheld.","de":"Eine Cache-Wurzel ausserhalb der Checkout-Wurzel: ein Werkzeug lief in einem Mitgliedsverzeichnis. Niemand liest sie; die Einträge werden entfernt, sobald sie ruhig und unbelegt sind."}}
    return json.dumps(p,ensure_ascii=False,indent=2)+"\n"
def ts(text):
    if "CacheStrayV1" in text:return text
    text=once(text,"/** 🗃️ One loose root with the budget","/** 🥀️ The rule for cache roots that tools left below member directories: how long they stay quiet, which segments are never entered, which top-level directories are not looked at, and why they go. */\nexport type CacheStrayV1=Readonly<{quietMs:number;ignoredSegments:readonly string[];excludedDirectories:readonly string[];reason:Bilingual}>;\n",after=False)
    text=once(text,"retirements:readonly CacheRetirementV1[];needs?:readonly CacheNeedV1[]}>","",after=True)
    text=text.replace("retirements:readonly CacheRetirementV1[];needs?:readonly CacheNeedV1[]}>","retirements:readonly CacheRetirementV1[];strays?:CacheStrayV1;needs?:readonly CacheNeedV1[]}>")
    text=once(text,"/** 🧮️ Classifies one root-relative path against the ledger. */","/** 🌿️ The single-segment directory names of the loose roots (`.cache`, `.r`): a directory of such a name below any member directory is a stray root. */\nexport function cacheStrayNamesV1(ledger:CacheLedgerV1):readonly string[]{return Object.freeze([...new Set(CACHE_ROOT_IDS_V1.map(root=>ledger.roots[root].directory).filter(directory=>!directory.includes(\"/\")))]);}\n",after=False)
    return text
rw(L+"🧬️schema/🔣️.json",schema)
rw(L+"🔣️policy.json",policy)
rw(L+"🟦️.ts",ts)
print("ledger strays applied")
