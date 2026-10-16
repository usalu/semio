import sys,os,re,glob,shutil
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from sandbox import *
from run_backend_oracle import SPECS
def seeded(name,child_glob,pattern,repl,configs=('native',)):
    spec=SPECS[name]
    base=build(spec,name+'-sbase',orig_file=os.path.join(T,'orig/%s.rs.txt'%spec.get('orig',name)))
    split=build(spec,name+'-ssplit')
    tgt=os.path.join(split,spec['target'] if 'target' in spec else spec['real_dir'])
    hit=0
    for p in glob.glob(tgt+'/'+child_glob):
        t=open(p,encoding='utf-8').read()
        n,c=re.subn(pattern,repl,t,count=1)
        if c:open(p,'w',encoding='utf-8').write(n);hit+=1;print('mutated',os.path.relpath(p,tgt))
    assert hit==1,hit
    from run_backend_oracle import CONFIGS
    out=os.path.join(SC,'rhs-orc')
    for cn in configs:
        a=norm(run(base,spec,name+'-sbase-'+cn,CONFIGS[cn],out),base)
        b=norm(run(split,spec,name+'-ssplit-'+cn,CONFIGS[cn],out),split)
        ma,mb=compare(a,b)
        print(name,cn,'only-base',sum(ma.values()),'only-split',sum(mb.values()))
        for k,v in sorted(mb.items())[:6]:print('  +',v,k[:200])
if __name__=='__main__':
    seeded(sys.argv[1],sys.argv[2],sys.argv[3],sys.argv[4])
