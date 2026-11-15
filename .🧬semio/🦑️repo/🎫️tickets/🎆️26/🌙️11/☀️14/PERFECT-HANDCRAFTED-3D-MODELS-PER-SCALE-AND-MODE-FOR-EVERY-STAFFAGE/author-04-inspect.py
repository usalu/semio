import json,glob,sys
slug=sys.argv[1]
p=glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/%s/🔣️.json"%slug)[0]
a=json.load(open(p))['asset']
print({k:len(json.dumps(v,ensure_ascii=False)) for k,v in a.items()})
def J(x,n=4000): print(json.dumps(x,ensure_ascii=False,separators=(',',':'))[:n])
for k in sys.argv[2:]:
    print('==',k); J(a[k],int(2e5))
