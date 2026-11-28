import os,shutil
R="/Users/ueli/Documents/semio/";H=os.path.dirname(os.path.abspath(__file__))
d=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🧱️minimal-closure/"
os.makedirs(d,exist_ok=True)
for n in ["🔣️.json","🟦️.ts"]:
    shutil.copy(H+"/closure-law/"+n,d+n+".tmp");os.replace(d+n+".tmp",d+n)
print("installed")
