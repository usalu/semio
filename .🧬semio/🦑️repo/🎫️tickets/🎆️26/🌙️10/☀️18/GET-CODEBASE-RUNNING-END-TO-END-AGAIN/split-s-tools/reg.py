import json,sys
tag,path,crate,root=sys.argv[1:5]
for f,k,v in (('tags.json',tag,path),('owners.json',tag,[crate]),('crateroots.json',crate,'/Users/ueli/Documents/semio/'+root)):
    d=json.load(open(f,encoding='utf-8')); d[k]=v; json.dump(d,open(f,'w',encoding='utf-8'),ensure_ascii=False,indent=1)
