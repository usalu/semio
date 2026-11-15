import sys,os,re
import subprocess
from pathlib import Path
T = Path(__file__).resolve().parent
R = Path(subprocess.check_output(['git', '-C', str(T), 'rev-parse', '--show-toplevel'], text=True).strip())
A=str(R)+'/✏️s/🔌️plugins/📕️norm/🗿️artifacts/'
def relpath(a,b): return os.path.relpath(b,a)
def run(fam):
    E=A+fam+'/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'
    man=E+'/📦️packages/🦀️rust/Cargo.toml'
    name=re.search(r'^name = "([^"]+)"',open(man).read(),re.M).group(1)
    ident=name.replace('-','_')
    pat=re.compile(r'#\[cfg\(test\)\]\n#\[path = "([^"]+)"\]\n((?:pub\(crate\) )?mod (\w+);)\n')
    moved=[]
    for root,ds,fs in os.walk(E):
        if 'target' in root: continue
        for f in fs:
            if f!='🦀️.rs': continue
            p=os.path.join(root,f);
            if '/🧪️tests/' in p: continue
            s=open(p).read();ns=s
            for m in list(pat.finditer(s)):
                tp=os.path.normpath(os.path.join(root,m.group(1)))
                body=open(tp).read()
                if re.search(r'\bsuper::',body) or '🧪️tests/🧩️example' in tp or 'incremental' in tp or 'field-meta' in tp or 'nested' in tp: continue
                moved.append((tp,m.group(3),m.group(2).startswith('pub(crate)')))
                ns=ns.replace(m.group(0),'',1)
            if ns!=s: open(p,'w').write(ns)
    d=E+'/🧪️tests/🧩️integration'
    os.makedirs(d,exist_ok=True)
    lines=[f'//! 🧩️ Editor unit tests of every window, panel and command, run against the public editor surface.\n',f'pub use {ident}::*;\n']
    i=0
    for tp,mn,isunit in sorted(moved):
        rel=relpath(d,tp)
        if mn=='unit_tests': lines.append(f'#[path = "{rel}"]\npub(crate) mod unit_tests;\n')
        else:
            i+=1;lines.append(f'#[path = "{rel}"]\nmod tests_{i};\n')
    open(d+'/🦀️.rs','w').write('\n'.join(lines))
    t=open(man).read()
    if '[[test]]' not in t:
        t=t.replace('\n[dependencies]','\n[[test]]\nname = "integration"\npath = "../../🧪️tests/🧩️integration/🦀️.rs"\n\n[dependencies]',1)
        open(man,'w').write(t)
    print(fam,len(moved),'moved')
for f in sys.argv[1:]: run(f)
