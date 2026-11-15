import re,json
F="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/height-adjustable-desk/🔣️.json"
t=open(F,encoding="utf-8").read()
pat=re.compile(r'("layers": \[\n)(            \{\n              "id": "laminate",.*?\n            \}),\n(            \{\n              "id": "board",.*?\n            \})(\n          \])',re.S)
n=0
def sw(m):
    global n; n+=1
    return m.group(1)+m.group(3)+",\n"+m.group(2)+m.group(4)
t2=pat.sub(sw,t); assert n==1,n
json.loads(t2); open(F,"w",encoding="utf-8").write(t2); print("swapped",n)
