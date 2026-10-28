"""🔦 [isolate] prints every occurrence of a pattern (default checkout references) in a file with ±W chars, numbered by line. usage: ctx.py <path> [regex] [W=110]"""
import re, sys
p=sys.argv[1]; rx=sys.argv[2] if len(sys.argv)>2 and sys.argv[2] else r"process\.cwd\(\)|SEMIO_WORKSPACE_ROOT|\.cache/b|fixture\.\w*[Ss]tore|repositoryLeaseDirectoryV1"; W=int(sys.argv[3]) if len(sys.argv)>3 else 110
t=open(p,encoding="utf-8").read()
for m in re.finditer(rx,t):
    print("L%d: …%s【%s】%s…" % (t.count("\n",0,m.start())+1, t[max(0,m.start()-W):m.start()].replace("\n","⏎"), m.group(0), t[m.end():m.end()+W].replace("\n","⏎")))
