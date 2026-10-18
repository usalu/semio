"""🔎 C6DB1 static resolver of TS source readers: for every `readFileSync(join(<var>|<repo>, ...parts), ...)` it resolves the path through the nearest preceding `const <var> = join(...)` and reports the reads that hit one of the split files (tags of files.tsv). usage: tsreaders.py [files...]"""
import os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio"
targets = {}
for l in open(HERE + "/files.tsv", encoding="utf-8"):
    tag, rel = l.rstrip("\n").split("\t")
    targets[rel] = tag
STR = r'"([^"]*)"'
def parts_of(args, env, repo_ok=True):
    out = []
    toks = re.findall(r'"([^"]*)"|([A-Za-z_.][A-Za-z0-9_.]*)', args)
    first = True
    for s, v in toks:
        if s:
            out.extend(s.split("/"))
        else:
            if v in ("this.repoRoot", "repoRoot", "this.root", "root") :
                out = []
            elif v in env:
                out = list(env[v])
            else:
                return None
    return out
def norm(parts):
    st = []
    for p in parts:
        if p == "..":
            if not st: return None
            st.pop()
        elif p and p != ".":
            st.append(p)
    return "/".join(st)
files = sys.argv[1:] or subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-l", "-e", "🛢️db", "--", "*.ts", ":!.🧬semio"], cwd=R, capture_output=True, text=True).stdout.split("\n")
hits = 0
for f in files:
    if not f or not os.path.isfile(os.path.join(R, f)): continue
    text = open(os.path.join(R, f), encoding="utf-8").read()
    env = {}
    for ln, line in enumerate(text.split("\n"), 1):
        m = re.match(r'\s*(?:const|let)\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(?:join|resolve)\((.*)\)\s*;?\s*$', line)
        if m:
            p = parts_of(m.group(2), env)
            if p is not None: env[m.group(1)] = p
        for rm in re.finditer(r'(readFileSync|readModuleTreeSource|policyRead\w+)\((?:join|resolve)\(((?:[^()]|\([^()]*\))*)\)', line):
            p = parts_of(rm.group(2), env)
            if p is None: continue
            path = norm(p)
            if path is None: continue
            for rel, tag in targets.items():
                if path == rel:
                    print("%s:%d  [%s] %s -> %s" % (f[-70:], ln, rm.group(1), tag, rel[-40:])); hits += 1
print("hits", hits)
