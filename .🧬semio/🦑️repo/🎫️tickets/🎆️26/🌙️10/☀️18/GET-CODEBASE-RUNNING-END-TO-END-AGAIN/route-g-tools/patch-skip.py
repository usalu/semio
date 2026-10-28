import sys
p = sys.argv[1]
t = open(p, encoding="utf8").read()
old = r'''const done=new Set(existsSync(summary)?readFileSync(summary,"utf8").split("\n").map(l=>l.split("\t")[0]):[]);'''
new = r'''const done=new Set([summary,...(process.env.CENSUS_SKIP_DIRS?.split(",")??[]).map(d=>d+"/summary.txt")].flatMap(path=>existsSync(path)?readFileSync(path,"utf8").split("\n").map(l=>l.split("\t")[0]):[]));'''
assert old in t
open(p, "w", encoding="utf8").write(t.replace(old, new))
