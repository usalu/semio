"""🔁 pregate-g: retarget a lane shell/py script from the live root to the mirror root and run it in the mirror. usage: rs.py [--seal] SCRIPT [args...]; env PG_MIRROR (name), PG_ROOT. Ticket-folder and .cache paths stay live (read only)."""
import os,re,subprocess,sys,hashlib
args=sys.argv[1:];seal=False
if args and args[0]=="--seal":seal=True;args=args[1:]
src=os.path.abspath(args[0]);rest=args[1:]
name=os.environ.get("PG_MIRROR","pg-all");root=os.environ["PG_ROOT"];T=os.environ["T"];C=os.environ["C"]
LIVE="/Users/ueli/Documents/semio"
text=open(src,encoding="utf8").read();d=os.path.dirname(src)
text=re.sub(r'\$\(dirname "\$0"\)',d,text);text=re.sub(r'\$\(dirname \$0\)',d,text);text=re.sub(r'\$\{0%/\*\}',d,text)
text=re.sub(r'^R=\$\(.*$','R='+root,text,flags=re.M)
text=re.sub(r'(?<![/\w])\.🧬semio/[^\'" \n]*?GET-CODEBASE-RUNNING-END-TO-END-AGAIN',lambda m:LIVE+"/"+m.group(0),text)
text=re.sub(r'(?<![/\w])\.cache/b/t/',LIVE+"/.cache/b/t/",text)
text=re.sub(re.escape(LIVE)+r'(?!/\.🧬semio|/\.cache|/\.nx)',root,text)
ext=os.path.splitext(src)[1] or ".sh"
out=f"{T}/out/rs-{hashlib.md5(src.encode()).hexdigest()[:8]}{ext}"
open(out,"w",encoding="utf8").write(text)
interp=["python3","-I"] if ext==".py" else ["zsh"] if text.startswith("#!/bin/zsh") else ["sh"]
cmd=["python3","-I",C+"/mirror-tools/mirror.py","run",name,"--peers-ok"]+([] if seal else ["--no-seal"])+["--"]+interp+[out]+rest
r=subprocess.run(cmd,capture_output=True,text=True)
for l in (r.stdout+r.stderr).splitlines():
    if l.startswith("mirror: guard clean") or l.startswith("mirror: run in") or re.match(r"^  [MAD] ",l) or l.startswith("mirror: GUARD WARN"):continue
    print(l)
sys.exit(r.returncode)
