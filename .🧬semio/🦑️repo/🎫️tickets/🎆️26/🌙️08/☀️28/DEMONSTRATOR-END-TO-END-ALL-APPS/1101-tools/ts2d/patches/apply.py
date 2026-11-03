import shutil,os
here=os.path.dirname(os.path.abspath(__file__))
os.chdir("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/◻️2d")
shutil.copy(here+"/booleans.ts","🔀️booleans/🟦️.ts")
shutil.copy(here+"/paths.ts","🔀️booleans/🛤️paths/🟦️.ts")
shutil.copy(here+"/booleans.test.ts","🔀️booleans/🧪️tests/🟦️.ts")
for p in ["🟦️.ts","📦️packages/🟦️typescript/🟦️.ts"]:
    s=open(p).read()
    for o in ["booleanRegions, ","type BooleanOptions, ","BooleanOptions, "]:
        s=s.replace(o,"")
    open(p,"w").write(s)
