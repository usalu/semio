import os,re,glob,collections,subprocess,sys
HUB='/Users/ueli/Documents/semio/🌎️hub/'
B=HUB+'🏗️bootstrap/'
orig=open(os.path.join(os.path.dirname(os.path.abspath(__file__)),'orig-bootstrap.rs.txt'),encoding='utf-8').read().split('\n')
files=[B+'🦀️.rs']+sorted(glob.glob(B+'*/🦀️.rs'))
new=[]
for f in files: new+=open(f,encoding='utf-8').read().split('\n')
standalone_new=open(HUB+'🧪️tests/🔬️standalone/🦀️.rs',encoding='utf-8').read().split('\n')
def norm(l):
    l=l.strip()
    l=re.sub(r'\bpub\(crate\) ','',l)
    l=l.replace('include!("../../🧪️tests','include!("../🧪️tests')
    return l
def keep(l):
    t=l.strip()
    if not t: return False
    if re.match(r'^//#(end)?region',t): return False
    return True
def is_use(l): return l.startswith('use ') or l.startswith('#[cfg') and False
oc=collections.Counter(norm(l) for l in orig if keep(l))
nc=collections.Counter(norm(l) for l in new if keep(l))
miss=oc-nc
extra=nc-oc
# exclude use/import lines and headers
def lbl(l): return l.startswith('use ') or l.startswith('//!') or l.startswith('#[path = "') or l.startswith('mod ') or l.startswith('#[cfg(test)]') 
print('missing lines (excluding use/cfg preamble):')
n=0
for l,c in miss.items():
    if l.startswith('use ') or l.startswith('#[cfg(') : continue
    print('  -',c,l[:150]);n+=c
print('missing total',n)
print('extra lines (excluding use/doc/path/mod):')
n=0
for l,c in extra.items():
    if l.startswith('use ') or l.startswith('//! ') or l.startswith('//!') or l.startswith('#[path = "') or l.startswith('mod ') or l.startswith('#[cfg(') : continue
    print('  +',c,l[:150]);n+=c
print('extra total',n)
