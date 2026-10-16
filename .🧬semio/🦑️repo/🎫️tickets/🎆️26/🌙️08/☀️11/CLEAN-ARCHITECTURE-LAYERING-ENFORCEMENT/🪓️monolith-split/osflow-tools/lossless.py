"""🧮 Line-multiset losslessness: original file vs the files that replace it (visibility words normalised). usage: lossless.py orig.rs new1.rs new2.rs ..."""
import collections,re,sys
def norm(l):
    l=re.sub(r'\bpub\((?:super|crate|in [\w:]+)\)\s+','',l)
    l=re.sub(r'^(\s*)pub\s+',r'\1',l)
    return l.strip()
def lines(p):
    return [norm(l) for l in open(p,encoding='utf-8').read().split('\n') if l.strip()]
orig=collections.Counter(lines(sys.argv[1]))
new=collections.Counter()
for p in sys.argv[2:]:new.update(lines(p))
missing=orig-new;added=new-orig
print("missing",sum(missing.values()),"added",sum(added.values()))
for k,v in sorted(missing.items())[:40]:print("  -",v,k[:140])
for k,v in sorted(added.items())[:60]:print("  +",v,k[:140])
