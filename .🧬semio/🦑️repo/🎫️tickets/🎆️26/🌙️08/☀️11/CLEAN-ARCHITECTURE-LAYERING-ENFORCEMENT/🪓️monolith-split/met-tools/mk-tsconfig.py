import json,sys
R="/Users/ueli/Documents/semio"
F=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
files=[R+"/"+l.strip() for l in open(F+"/met-tools/gis-new-files.txt",encoding="utf8") if l.strip()]
files.append(R+"/✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/👷️worker/🟦️.ts")
files.append(R+"/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts")
cfg={"extends":R+"/🧰️framework/🛍️products/💻️os/tsconfig.json","compilerOptions":{"noEmit":True,"incremental":False,"tsBuildInfoFile":None,"composite":False},"include":[],"files":files}
json.dump(cfg,open(F+"/met-tools/tsconfig.gis-after.json","w"),indent=1,ensure_ascii=False)
print(len(files))
