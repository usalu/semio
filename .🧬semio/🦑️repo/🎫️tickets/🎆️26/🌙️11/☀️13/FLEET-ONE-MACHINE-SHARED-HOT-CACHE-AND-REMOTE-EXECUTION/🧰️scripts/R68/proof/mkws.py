import os,shutil,sys
R="/Users/ueli/Documents/semio";D="/private/tmp/r68/ws"
skip={".cache","node_modules","target","dist",".git","🗑️generated","🤖️generated","🖼️assets","🐾️pets",".nx","🧪️snapshots"}
ext={".ts",".mjs",".cjs",".js",".json",".toml",".md",".py",".lock",".yaml",".yml",".txt",".go",".mod",".sum",".rs",".tsx",".jsx",".cs",".csproj",".html",".css",".svg"}
n=0;tot=0
def cp(p):
    global n,tot
    rel=os.path.relpath(p,R);d=os.path.join(D,rel)
    if os.path.islink(p):return
    try:z=os.path.getsize(p)
    except:return
    if z>1_500_000:return
    os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(p,d);n+=1;tot+=z
for f in os.listdir(R):
    p=os.path.join(R,f)
    if os.path.isfile(p) and not f.startswith(".DS"):cp(p)
for top in sys.argv[1:]:
    for root,dirs,files in os.walk(os.path.join(R,top)):
        dirs[:]=[d for d in dirs if d not in skip and not d.startswith(".")]
        for f in files:
            if os.path.splitext(f)[1] in ext or f in("Cargo.toml",".gitignore"):cp(os.path.join(root,f))
print(n,tot//1048576,"MB")
