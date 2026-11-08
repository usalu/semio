import re,sys
sys.path.insert(0,'.')
from effectify import match_paren,split_args
def wrap(path,names=('step','close_step')):
    s=open(path,encoding='utf8').read();n=0
    for name in names:
        pat=re.compile(r'pub fn %s\b(<[^(]*?>)?\('%name)
        i=0
        while True:
            m=pat.search(s,i)
            if not m:break
            po=m.end()-1;pc=match_paren(s,po);args=s[po+1:pc]
            if 'effect:&mut RetainedAllocationEffect' not in args:i=pc;continue
            b=s.index('{',pc);ret=s[pc+1:b]
            parts=[a for a in split_args(args) if a.strip()!='&mut self']
            names_=[a.split(':')[0].strip() for a in parts]
            gen=m.group(1) or ''
            body=name+'_body'
            wrapper=('pub fn %s%s(&mut self,%s)%s{let start=effect.performed_work();let outcome=self.%s(%s);match crate::settle_since(effect,start,self.last_work){Ok(work)=>{self.last_work=work;outcome},Err(error)=>if outcome.is_ok(){Err(error.into())}else{outcome}}}\n '%(name,gen,args.replace('&mut self,','',1) if args.strip().startswith('&mut self') else args,ret,body,','.join(names_)))
            sig_old=s[m.start():b]
            sig_new=sig_old.replace('pub fn %s'%name,'fn %s'%body,1)
            s=s[:m.start()]+wrapper+sig_new+s[b:]
            n+=1
            i=m.start()+len(wrapper)+len(sig_new)
    open(path,'w',encoding='utf8').write(s);print(path,n)
for p in sys.argv[1:]:wrap(p)
