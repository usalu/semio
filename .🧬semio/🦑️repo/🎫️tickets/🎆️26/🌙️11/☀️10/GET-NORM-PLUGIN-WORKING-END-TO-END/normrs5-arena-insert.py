import re,subprocess,collections
import subprocess
from pathlib import Path
T = Path(__file__).resolve().parent
R = Path(subprocess.check_output(['git', '-C', str(T), 'rev-parse', '--show-toplevel'], text=True).strip())
N=str(R)+"/✏️s/🔌️plugins/📕️norm"
names=open("normrs5-arena-names.txt").read().split("\n")
files=collections.defaultdict(set)
for n in names:
    fn=n.split("::")[-1]
    out=subprocess.run(["grep","-rl","--include=*.rs",f"fn {fn}(",N+"/📇️registry"],capture_output=True,text=True).stdout.split()
    for o in out: files[o].add(fn)
INS="    let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();\n"
tot=0
for f,fns in files.items():
    s=open(f).read()
    for fn in fns:
        m=re.search(r"((?:async )?fn "+re.escape(fn)+r"\([^)]*\)[^{]*\{\n)",s)
        if not m: print("nomatch",fn,f); continue
        if "UiArenaFixture" in s[m.end():m.end()+200]: continue
        if m.group(1).startswith("async"): print("ASYNC",fn)
        s=s[:m.end()]+INS+s[m.end():]; tot+=1
    open(f,"w").write(s)
print(tot)
