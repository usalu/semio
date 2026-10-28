import sys
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"
G=L+"🕰️generations/"
steps=[
 (G+"🟨️.d.mts","readonly waitNoticeMilliseconds:number}","readonly waitNoticeMilliseconds:number;readonly sweepWaitMilliseconds:number}"),
 (G+"🧩️claims/🟨️.d.mts","readonly environmentSlots:string}","readonly environmentSlots:string;readonly failedRetryMilliseconds:number}"),
 (G+"🧩️claims/🟨️.d.mts","readonly startedMs:number;readonly endedMs?:number}","readonly startedMs:number;readonly endedMs?:number;readonly stamps?:Readonly<Record<string,string>>}"),
 (G+"🧩️claims/🟨️.d.mts","export function chunkOfV1(options:{wanted:readonly string[];claims:readonly Pick<SeenClaimV1,\"token\"|\"base\"|\"implementation\"|\"owners\"|\"state\"|\"live\">[];base:number;implementation:string;chunk?:number;tail?:number})","export function chunkOfV1(options:{wanted:readonly string[];stamps?:Readonly<Record<string,string>>;claims:readonly Pick<SeenClaimV1,\"token\"|\"base\"|\"implementation\"|\"owners\"|\"state\"|\"live\"|\"endedMs\"|\"stamps\">[];base:number;implementation:string;chunk?:number;tail?:number;nowMs?:number})"),
 (G+"🧩️claims/🟨️.d.mts","wanted:readonly string[];run(owners:string[]","wanted:readonly string[];stamps?:Readonly<Record<string,string>>;run(owners:string[]"),
 (L+"🟦️.ts","derivedFacts=new Map<string,{row:BuildGraphRowV1;support:BuildGraphRowV1[]}>()","derivedFacts=new Map<string,{readonly row:BuildGraphRowV1;readonly support:readonly BuildGraphRowV1[]}>()"),
 (L+"🟦️.ts","derived?:{row:BuildGraphRowV1;support:BuildGraphRowV1[]}[])=>{nodeRows[name]=rows;","derived?:{readonly row:BuildGraphRowV1;readonly support:readonly BuildGraphRowV1[]}[])=>{nodeRows[name]=rows;"),
 (L+"📋️model/🟦️.ts","derived?:{row:BuildGraphRowV1;support:BuildGraphRowV1[]}[];","derived?:{readonly row:BuildGraphRowV1;readonly support:readonly BuildGraphRowV1[]}[];"),
]
data={}
for f,a,b in steps:
  t=data.get(f) or open(f,encoding="utf8").read()
  if t.count(a)!=1: sys.exit(f"anchor count {t.count(a)}: {f}: {a[:60]}")
  data[f]=t.replace(a,b)
if sys.argv[1:]==["apply"]:
  for f,t in data.items(): open(f,"w",encoding="utf8").write(t)
  print("applied",len(steps),"steps in",len(data),"files")
else: print("check ok")
