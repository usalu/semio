import json,os,sys
S=os.path.dirname(os.path.abspath(__file__));R="/Users/ueli/Documents/semio"
m={}
for d,_,fs in os.walk(S+"/new"):
    for f in fs:
        p=os.path.join(d,f);m[R+"/"+os.path.relpath(p,S+"/new")]=p
json.dump(m,open(S+"/map.json","w"),ensure_ascii=False)
print(len(m))
