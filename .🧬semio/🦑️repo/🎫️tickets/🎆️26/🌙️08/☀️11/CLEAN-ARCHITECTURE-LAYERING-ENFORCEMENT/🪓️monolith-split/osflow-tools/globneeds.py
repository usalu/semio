"""🔎 Which public names of a provider module are used by a set of files. usage: globneeds.py <provider.rs> <file>..."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from pubnames import names
from usage import toks
prov=names(sys.argv[1]);allt=set()
for f in sys.argv[2:]:allt|=toks(f)
used=sorted(n for n in prov if not n.startswith('EXTGLOB') and n in allt)
print(len(prov),"names;",len(used),"used:",used)
print("extglobs:",[n for n in prov if n.startswith('EXTGLOB')])
