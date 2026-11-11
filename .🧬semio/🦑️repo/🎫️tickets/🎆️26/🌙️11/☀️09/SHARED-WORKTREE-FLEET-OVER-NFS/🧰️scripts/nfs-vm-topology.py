import json,glob,sys
p=glob.glob("/srv/semio/[!n]*/*/*/*/*shared-worktree/*topology/*.json")
p=[x for x in p if "/schema" not in x and "/tests" not in x][0]
t=json.load(open(p))
ip=sys.argv[1]; net=sys.argv[2]
g=t["groups"][0]
g["nfs"]["host"]=ip
g["cache"]["endpoint"]="http://%s:6090"%ip
g["server"]["network"]=net
json.dump(t,open(p,"w"),indent=1,ensure_ascii=False)
print(p)
