"""🔬 Union-sandbox resolution oracle: every crate is simulated as a top-level module of one crate.

Usage: oracle2.py <stage_dir> <crates.json> [native|test|wasm|wasip2 ...]
crates.json: [{"lib": "db_x", "root": "<relative path of lib root file below stage_dir>"}]
Accepts only errors that name an external crate; everything else is a regression.
"""
import json,os,re,shutil,subprocess,sys,collections
from lib import *
from rs import strip

TC='+nightly-2026-07-20'
STD=('crate','self','super','std','core','alloc')
EXTERNAL_PREFIXES=('semio_framework_',)
EXTERNAL_NAMES={'protocol','store','pack','dsl','serde','serde_json','sqlx','neo4rs','tokio','rusqlite','futures','proptest','criterion','serde_derive','tempfile','blake3'}

def is_external(sym):
    h=sym.split('::')[0]
    return h in EXTERNAL_NAMES or h.startswith(EXTERNAL_PREFIXES)

def make_sandbox(stage, crates, out, root_name='union'):
    shutil.rmtree(out,ignore_errors=True)
    shutil.copytree(stage,out+'/db')
    libs={c['lib'] for c in crates}
    dir_of={}
    for c in crates:
        dir_of[c['lib']]=os.path.dirname(os.path.join(out+'/db',c['root']))
    # files per crate: all .rs files below the crate's #[path] closure; approximated by explicit file list in crates.json
    for c in crates:
        for rel in c['files']:
            p=os.path.join(out+'/db',rel)
            t=read(p)
            t=patch_text(t,c['lib'],libs)
            write(p,t)
    lines=['#![allow(async_fn_in_trait)]','']
    for c in crates:
        lines.append(f'#[path = "../../{c["root"]}"]')
        lines.append(f'pub mod {c["lib"]};')
    write(out+'/db/📦️packages/🦀️rust/🦀️.rs','\n'.join(lines)+'\n')
    return out+'/db/📦️packages/🦀️rust/🦀️.rs'

EXT_GLOB=re.compile(r'(?m)^([ \t]*)((?:pub(?:\([^)]*\))?[ \t]+)?)use[ \t\n]+([A-Za-z_][A-Za-z0-9_]*)::([^;]*\*)\s*;')

def patch_text(t,lib,libs):
    st=strip(t)
    edits=[]
    for m in re.finditer(r'(?m)^[ \t]*extern crate [^;]*;[ \t]*$',st):
        edits.append((m.start(),m.end(),''))
    for m in EXT_GLOB.finditer(st):
        head=m.group(3)
        if head in STD or head in libs: continue
        tail=m.group(4)
        # only whole glob leaf at the end of a plain path or inside braces: replace statement
        edits.append((m.start(),m.end(),m.group(1)+'const _: () = ();'))
    for m in re.finditer(r'(?<![A-Za-z0-9_:])(db_[a-z0-9_]+)::',st):
        if m.group(1) in libs and m.group(1)!=lib:
            edits.append((m.start(),m.end(),'crate::'+m.group(1)+'::'))
    for m in re.finditer(r'(?<![A-Za-z0-9_])crate::([A-Za-z_][A-Za-z0-9_]*)',st):
        if m.group(1) in libs: continue
        edits.append((m.start(),m.end(),f'crate::{lib}::{m.group(1)}'))
    for m in re.finditer(r'(?<![A-Za-z0-9_])pub\(crate\)',st):
        edits.append((m.start(),m.end(),f'pub(in crate::{lib})'))
    # drop overlaps: keep first by start
    edits.sort()
    final=[]
    for e in edits:
        if final and e[0]<final[-1][1]: continue
        final.append(e)
    for s,e,r in sorted(final,reverse=True):
        t=t[:s]+r+t[e:]
    return t

def run(rootfile,mode,out_err):
    args=['rustc',TC,'--edition','2021','--crate-type','lib','--crate-name','dbunion','--emit=metadata','-o','/dev/null','--cfg','feature="testkit"','--cfg','feature="claimed-backend-laws"']
    if mode=='test': args+=['--test']
    elif mode=='wasm': args+=['--target','wasm32-unknown-unknown']
    elif mode=='wasip2': args+=['--target','wasm32-wasip2']
    args.append(rootfile)
    r=subprocess.run(args,capture_output=True,text=True,cwd=os.path.dirname(rootfile))
    open(out_err,'w').write(r.stderr)
    return r.stderr

def classify(stderr):
    errs=[]
    cur=None
    for line in stderr.split('\n'):
        if line.startswith('error'):
            cur=[line]; errs.append(cur)
        elif cur is not None and line.strip().startswith('-->') and len(cur)==1:
            cur.append(line.strip())
    bad=[]; ok=0
    for e in errs:
        head=e[0]
        if head.startswith('error: aborting due to'): continue
        syms=re.findall(r'`([^`]*)`',head)
        if 'couldn\'t read' in head and '🏪️store' in head: ok+=1; continue
        if syms and all(is_external(s) for s in syms if re.match(r'[A-Za-z_]',s)) and any(re.match(r'[A-Za-z_]',s) for s in syms):
            ok+=1; continue
        if 'not found for' in head and 'E0220' in head: ok+=1; continue
        if any(is_external(s) for s in syms[:1]) :
            ok+=1; continue
        bad.append(e)
    return ok,bad

def main():
    stage=sys.argv[1]; crates=json.load(open(sys.argv[2])); modes=sys.argv[3:] or ['native']
    out=f'{SP}/sandbox'
    root=make_sandbox(stage,crates,out)
    total_bad=0
    for mode in modes:
        err=run(root,mode,f'{SP}/sandbox-{mode}.err')
        ok,bad=classify(err)
        print(f'[{mode}] external-accepted={ok} flagged={len(bad)}')
        seen=collections.Counter()
        for e in bad:
            msg=re.sub(r'\s+',' ',e[0]); loc=(e[1] if len(e)>1 else '').replace(SP+'/sandbox/db/📦️packages/🦀️rust/../../','')
            loc=re.sub(r':\d+:\d+$','',loc).replace('--> ','')
            seen[(msg[:150],loc)]+=1
        for (msg,loc),n in list(seen.most_common(45)): print('   ',n,msg,'@',loc[:90])
        total_bad+=len(bad)
    sys.exit(1 if total_bad else 0)
if __name__=='__main__': main()
