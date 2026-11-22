ROOT="/Users/ueli/Documents/semio/🧰️framework/"
L=ROOT+"🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/"
def sub(p,o,n,c=1):
    t=open(p,encoding="utf8").read()
    if t.count(o)==0 and n in t:print("already",p[-30:],o[:30]);return
    assert t.count(o)==c,(p[-50:],o[:70],t.count(o));open(p,"w",encoding="utf8").write(t.replace(o,n))
p=L+"🧬️schema/🔣️.json";s=open(p,encoding="utf8").read()
if '"within"' not in s:
    i=s.index('"retirement": {');k=s.index('"properties": {\n',i)+len('"properties": {\n')
    s=s[:k]+'        "within": {\n          "type": "string",\n          "pattern": "^[a-z][a-z0-9-]{0,39}$",\n          "description": "The control area that this obsolete path lies inside. Without it a retired path may not lie inside any declared home; with it the path is carved out of that area, which keeps owning the rest of its directory."\n        },\n'+s[k:]
    open(p,"w",encoding="utf8").write(s)
sub(L+"🟦️.ts","export type CacheRetirementV1=Readonly<{root:CacheRootIdV1;directory:string;quietMs:number;reason:Bilingual}>;","export type CacheRetirementV1=Readonly<{root:CacheRootIdV1;directory:string;quietMs:number;reason:Bilingual;within?:string}>;")
sub(L+"🟦️.ts",''' for(const retirement of ledger.retirements){
  if(segments(retirement.directory).some(part=>part==="."||part===".."))refuse("canonical retirement required: "+retirement.directory);
  claimed.get(retirement.root)!.push(segments(retirement.directory));
 }''',''' const carvedOut:string[][][]=[];
 for(const retirement of ledger.retirements){
  if(segments(retirement.directory).some(part=>part==="."||part===".."))refuse("canonical retirement required: "+retirement.directory);
  if(retirement.within!==undefined){
   const home=ledger.areas.find(area=>area.id===retirement.within);
   if(home===undefined||home.kind!=="control"||home.root!==retirement.root||home.directory===undefined||!inside(segments(home.directory),segments(retirement.directory))||segments(home.directory).length===segments(retirement.directory).length)refuse("a carved-out retirement lies strictly inside a control area of its root: "+retirement.directory);
   if(carvedOut.some(other=>other.some(path=>inside(path,segments(retirement.directory))||inside(segments(retirement.directory),path))))refuse("overlapping carved-out retirements: "+retirement.directory);
   carvedOut.push([segments(retirement.directory)]);continue;
  }
  claimed.get(retirement.root)!.push(segments(retirement.directory));
 }''')
sub(L+"🟦️.ts",''' const parts=segments(path);if(parts.length===1&&ledger.roots[root].ignored.includes(parts[0]!))return{kind:"ignored"};
''',''' const parts=segments(path);if(parts.length===1&&ledger.roots[root].ignored.includes(parts[0]!))return{kind:"ignored"};
 for(const retirement of ledger.retirements)if(retirement.within!==undefined&&retirement.root===root&&inside(segments(retirement.directory),parts))return{kind:"retired",id:retirement.directory};
''')
t=open(L+"🔣️policy.json",encoding="utf8").read()
o='''  "retirements": [
'''
n='''  "retirements": [
    {
      "root": "native",
      "directory": "f/cache-server",
      "within": "fleet-state",
      "quietMs": 3600000,
      "reason": {
        "en": "The fleet cache index moved into the products partition (area fleet-cache-index).",
        "de": "Der Index des Flotten-Caches liegt jetzt in der Produkt-Partition (Bereich fleet-cache-index)."
      }
    },
'''
assert t.count(o)==1 or n in t
if n not in t:open(L+"🔣️policy.json","w",encoding="utf8").write(t.replace(o,n))
print("ok")
