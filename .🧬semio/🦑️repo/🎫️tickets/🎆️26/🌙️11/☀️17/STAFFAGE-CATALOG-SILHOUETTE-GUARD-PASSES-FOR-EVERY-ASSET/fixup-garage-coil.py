P="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/sectional-garage-door/🔣️.json"
s=open(P,encoding="utf8").read()
o='"count": 122,\n              "translation": [0, 0, 0.0082]'
n='"count": 125,\n              "translation": [0, 0, 0.008]'
assert s.count(o)==1
open(P,"w",encoding="utf8").write(s.replace(o,n))
