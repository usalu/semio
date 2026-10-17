import sys,glob,collections,csv,os
base=sys.argv[1]
ROOT="/Users/ueli/Documents/semio/"
for f in sorted(glob.glob(base+"/tc2-inputs-*lib-test*.txt")):
    name=os.path.basename(f)[len("tc2-inputs-"):].split("-lib-test-")[0].replace("semio-s-artifact-","")
    rows=[l.rstrip("\n").split("\t") for l in open(f) if l.strip()]
    emb=[r for r in rows if r[0]=="embedded"]; src=[r for r in rows if r[0]!="embedded"]
    tests=[r for r in src if "🧪️" in r[3]]
    print(f"{name}: total {len(rows)} files {sum(int(r[1]) for r in rows)} B; source {len(src)} ({sum(int(r[1]) for r in src)} B, {sum(int(r[2]) for r in src)} L); embedded {len(emb)} ({sum(int(r[1]) for r in emb)} B); test-dir sources {len(tests)} ({sum(int(r[1]) for r in tests)} B, {sum(int(r[2]) for r in tests)} L)")
