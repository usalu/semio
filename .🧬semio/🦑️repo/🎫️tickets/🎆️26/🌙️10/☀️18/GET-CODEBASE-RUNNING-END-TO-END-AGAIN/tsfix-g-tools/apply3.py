import sys
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
steps=[
 (L+"⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🟦️.ts","{const api=await import(\"../🟦️.ts\"),fixture=(await import(\"./♻️recovery.json\")).default,","{const api=await import(\"../🟦️.ts\"),recovery=await import(\"../♻️recovery/🟦️.ts\"),fixture=(await import(\"./♻️recovery.json\")).default,"),
 (L+"⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🧫️fixtures/🟦️echo.ts","const status=process.env.XPLAT_EXIT;\nif(status!==undefined)process.exit(Number(status));","const exitStatus=process.env.XPLAT_EXIT;\nif(exitStatus!==undefined)process.exit(Number(exitStatus));"),
 (L+"🏗️build/🩺️doctor/🧪️tests/🔎️probes/🟦️.ts","tools:{rows:[],wasmLocks:facts}} as never,DOCTOR_POLICY_V1);","tools:{rows:[],wasmLocks:facts}},{policy:DOCTOR_POLICY_V1});"),
]
data={}
for f,a,b in steps:
  t=data.get(f) if f in data else open(f,encoding="utf8").read()
  if t.count(a)!=1: sys.exit(f"anchor count {t.count(a)}: {f[-50:]}: {a[:70]}")
  data[f]=t.replace(a,b)
if sys.argv[1:]==["apply"]:
  for f,t in data.items(): open(f,"w",encoding="utf8").write(t)
  print("applied")
else: print("check ok")
