import sys, re, os, subprocess, json, shutil
sys.path.insert(0,'.')
from apply7 import *
ROOTDIR=ROOT
def git_files():
    out=subprocess.run(['git','-c','core.fsmonitor=false','-c','core.quotepath=false','grep','-l','-E',r'Plugin(::<[^>]*>)?::builder\(','--','*.rs',':!.🧬semio'],cwd=ROOT,capture_output=True,text=True).stdout.split('\n')
    return [f for f in out if f]
def conv(text, rel):
    def sub_gen(m):
        qual=m.group(1) or ''
        gen=m.group(2)
        if gen:
            nl='semio_framework_plugin::NeedsLabel' if qual else 'NeedsLabel'
            return '%sPluginBuilder::<%s, %s>::new('%(qual,nl,gen)
        return '%sPluginBuilder::new('%qual
    # skip comment lines
    out=[]
    for line in text.split('\n'):
        s=line.lstrip()
        if s.startswith('//') or s.startswith('*'):
            out.append(line); continue
        # code part before a trailing // comment is not separated; plugin test lines have no trailing comments with builder( in practice
        out.append(re.sub(r'((?:semio_framework_plugin::|super::)?)Plugin(?:::<([^>]*(?:<[^>]*>)?[^>]*)>)?::builder\(',sub_gen,line))
    return '\n'.join(out)
if __name__=='__main__':
    files=git_files()
    plan={}
    for f in files:
        t=open(ROOT+f,encoding='utf-8').read()
        n=conv(t,f)
        if n!=t: plan[f]=(t,n)
    for f,(t,n) in plan.items():
        import difflib
        d=[l for l in difflib.unified_diff(t.splitlines(),n.splitlines(),n=0,lineterm='') if l.startswith('+') and not l.startswith('+++')]
        print(f.split('/')[-5:-1],len(d),d[0][:200])
    json.dump({f:n for f,(t,n) in plan.items()},open(GEN+'/pwp7-d6.json','w'),ensure_ascii=False)
