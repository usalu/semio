import sys,hashlib,re
out=set(); n=0
norm=re.compile(r"\s+")
own=re.compile(r"^(?://[/!]?|/\*\*?|\*)\s*(.*)$")
trail=re.compile(r"(?:^|[\s;{}(),])(?://[/!]?|/\*\*?)\s*(.*)$")
def add(body):
    global n
    body=norm.sub(" ",body).strip()
    body=re.sub(r"\s*\*/$","",body).strip()
    if not body: return
    out.add(hashlib.blake2b(body.encode("utf-8"),digest_size=8).digest()); n+=1
for raw in sys.stdin.buffer:
    s=raw.decode("utf-8","replace").strip()
    m=own.match(s)
    if m: add(m.group(1)); continue
    for m in re.finditer(r"(?:^|[\s;{}(),])(?://[/!]?|/\*\*?)\s*",s):
        add(s[m.end():]); break
sys.stderr.write(f"lines {n} unique {len(out)}\n")
open(sys.argv[1],"wb").write(b"".join(sorted(out)))
