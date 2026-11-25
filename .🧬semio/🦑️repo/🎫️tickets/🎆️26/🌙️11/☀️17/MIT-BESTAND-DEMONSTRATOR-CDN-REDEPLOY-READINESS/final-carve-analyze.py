import os,sys,glob,re,collections
os.environ['LOCK']='🌎️hub/Cargo.lock'
sys.path.insert(0,os.path.dirname(__file__))
import importlib.util
spec=importlib.util.spec_from_file_location('c',os.path.join(os.path.dirname(os.path.abspath(__file__)),'final-carve-closure.py'));c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c)
sizes={}
for f in glob.glob('.cache/b/r/m/**/*.rlib',recursive=True):
    m=re.match(r'lib(.+)-[0-9a-f]+\.rlib',os.path.basename(f))
    if m: sizes[m.group(1)]=max(sizes.get(m.group(1),0),os.path.getsize(f)/1e6)
def sz(n): return sizes.get(n.replace('-','_'),0)
def report(title,cl):
    tot=sum(sz(n) for n in cl); known=sum(1 for n in cl if sz(n))
    print('##',title,len(cl),'crates; rlib MB known for',known,'sum',round(tot))
proc=c.closure(['semio-hub-process-component']); dem=c.closure(['semio-hub-demonstrator-component'])
report('process',proc); report('demonstrator',dem)
fam=lambda n:('stdio-'+n.split('stdio-')[1].split('-')[0]) if 'stdio-' in n else ('hub' if n.startswith('semio-hub') else 'other')
for t,cl in (('process',proc),('demonstrator',dem)):
    d=collections.Counter()
    for n in cl:
        if n.startswith('semio-s-artifact-stdio'): d['stdio']+=1
        elif n.startswith('semio-hub'): d['hub']+=1
        elif n.startswith('semio-s-artifact'): d['artifact']+=1
        elif n.startswith('semio-plugin'): d['plugin']+=1
        elif n.startswith('semio-'): d['framework']+=1
        else: d['external']+=1
    print(t,dict(d))
    print(t,'hub crates:',sorted(n for n in cl if n.startswith('semio-hub')))
    print(t,'stdio crates:',sorted(n for n in cl if n.startswith('semio-s-artifact-stdio')))
    print(t,'top rlibs:',[(n,round(sz(n))) for n in sorted(cl,key=sz,reverse=True)[:25]])
print('demonstrator minus process:',sorted(dem-proc)[:0],len(dem-proc))
print('process minus demonstrator:',sorted(proc-dem))
