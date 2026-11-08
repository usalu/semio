import sys
# usage: rep.py file  (reads pairs from stdin separated: <<<OLD\n..\n===\n..\n>>>) ; each OLD must occur exactly count times (default 1) unless "xN" given
import re
p=sys.argv[1]; s=open(p).read()
spec=sys.stdin.read()
for m in re.finditer(r"<<<(?:x(\d+))?\n(.*?)\n===\n(.*?)\n>>>",spec,re.S):
    n=int(m.group(1) or 1); old,new=m.group(2),m.group(3)
    c=s.count(old)
    if c!=n: print(f"FAIL count {c}!={n}: {old[:80]}"); sys.exit(1)
    s=s.replace(old,new)
open(p,"w").write(s); print("ok")
