import sys,re
hits=[l.rstrip("\n").rsplit(":",1) for l in open(sys.argv[1])]
n=0
for path,ln in hits:
    ln=int(ln)
    try: line=open(path,encoding="utf8").read().split("\n")[ln-1]
    except Exception as e: print("ERR",path,ln,e); continue
    for m in re.finditer(re.escape("[DEBUG]"),line):
        n+=1
        s=max(0,m.start()-int(sys.argv[2])); e=min(len(line),m.end()+int(sys.argv[3]))
        short="/".join(path.split("/")[-4:])
        print(f"{n}|{short}:{ln}|{len(line)}| …{line[s:e]}")
