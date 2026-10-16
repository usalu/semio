import json,sys
R="/Users/ueli/Documents/semio"
F=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
mode=sys.argv[1]
files=[R+"/"+l.strip() for l in open(F+"/met-tools/tt-new-files.txt",encoding="utf8") if l.strip()]
D=R+"/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/"
files+=[D+"✅️verification/🟦️.ts"]
if mode=="tests":
    files=[D+"🧪️time-travel/🚀️boot/🧪️tests/🟦️.ts", D+"🧪️time-travel/🪞️inventory/🧪️tests/🟦️.ts"]
cfg={"extends":R+"/🧰️framework/🛍️products/💻️os/tsconfig.json","compilerOptions":{"noEmit":True,"incremental":False,"tsBuildInfoFile":None,"composite":False},"include":[],"files":files}
json.dump(cfg,open(F+f"/met-tools/tsconfig.tt-{mode}.json","w"),indent=1,ensure_ascii=False)
print(len(files))
