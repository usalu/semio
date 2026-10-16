"""🔍️ Differential field/method privacy scan: candidates of the current tree that the baseline tree does not already have."""
import sys,json,re,os,subprocess
here=os.path.dirname(os.path.abspath(__file__))
def norm(p):
    p=re.sub(r'^.*?(🌎️hub/)', r'\1', os.path.normpath(p.replace('/📦️packages/🦀️rust/../../','/')))
    return p.replace('🌍️gis-map-binding','🔗️binding')
def keys(path):
    d=json.load(open(path))
    return {(x['kind'],x['owner'],x['name'],x['vis'],norm(x['def_file']),norm(x['use_file'])) for x in d}
base,cur=sys.argv[1],sys.argv[2]
A,B=keys(base),keys(cur)
new=sorted(B-A); gone=sorted(A-B)
print("baseline",len(A),"current",len(B),"new",len(new),"gone",len(gone))
for k in new: print("  NEW",k)
