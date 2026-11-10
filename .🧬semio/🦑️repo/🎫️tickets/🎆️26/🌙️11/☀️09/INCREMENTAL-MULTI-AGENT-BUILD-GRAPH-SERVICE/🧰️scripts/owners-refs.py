"""Who holds the string cells of a v8 snapshot: bytes of the strings by the class of the first referrer and the property name, then the arrays that hold the most string bytes by the property that holds the array."""
import sys, importlib.util, numpy as np, collections
spec=importlib.util.spec_from_file_location("oh",__file__.rsplit("/",1)[0]+"/owners-heap.py");oh=importlib.util.module_from_spec(spec);spec.loader.exec_module(oh)
h=oh.prepare(sys.argv[1]);strings=h["strings"];N=h["N"];tn=h["meta"]["node_types"][0];etn=h["meta"]["edge_types"][0]
st=tn.index("string");src=np.repeat(np.arange(N),h["count"]);to=h["to"]
is_str=(h["typ"][to]==st);e_idx=np.where(is_str)[0];s_src=src[e_idx];s_to=to[e_idx]
et=h["edges"][e_idx,h["ei"]["type"]];nm=h["edges"][e_idx,h["ei"]["name_or_index"]]
named=np.isin(et,[etn.index("property"),etn.index("internal"),etn.index("context")]);label=np.where(named,nm,-1)
W=len(strings)+2;key=(h["typ"][s_src].astype(np.int64)*(len(strings)+1)+h["name"][s_src])*W+(label+1)
u,inv=np.unique(key,return_inverse=True);b=np.bincount(inv,weights=h["self"][s_to]);c=np.bincount(inv)
for k in np.argsort(-b)[:int(sys.argv[2]) if len(sys.argv)>2 else 14]:
    kk=int(u[k]);l=kk%W-1;t,n=divmod(kk//W,len(strings)+1);print(round(b[k]/1048576,1),"MB",int(c[k]),tn[t],strings[n][:28],"."+(strings[l] if l>=0 else "[idx]"))
elem=etn.index("element");m=is_str&(h["edges"][:,h["ei"]["type"]]==elem);a_src=src[m];a_to=to[m]
sb=np.bincount(a_src,weights=h["self"][a_to],minlength=N);arr=sb>0
into=arr[to];hold=collections.Counter()
for s_,t_,l_,e_ in zip(src[into].tolist(),to[into].tolist(),h["edges"][:,h["ei"]["name_or_index"]][into].tolist(),h["edges"][:,h["ei"]["type"]][into].tolist()):
    name=strings[l_] if e_ in (etn.index("property"),etn.index("internal"),etn.index("context")) and l_<len(strings) else "[idx]"
    hold[(tn[h["typ"][s_]],strings[h["name"][s_]][:24],name)]+=sb[t_]
print("== arrays of strings by holder");[print(round(v/1048576,1),"MB",k) for k,v in hold.most_common(12)]
