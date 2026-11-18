import re,sys
P="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/tealight/🔣️.json"
e,f=sys.argv[1:3]
lines=open(P,encoding="utf8").read().split("\n")
for k,pat,new in ((823,r"C 0\.0105 [0-9.]+ 0\.0105 0\.01 ","C 0.0105 %s 0.0105 0.01 "%e),(873,r"C 0\.0105 [0-9.]+ 0\.0105 0\.03258 ","C 0.0105 %s 0.0105 0.03258 "%f)):
    assert re.search(pat,lines[k-1]),k
    lines[k-1]=re.sub(pat,new,lines[k-1],count=1)
open(P,"w",encoding="utf8").write("\n".join(lines))
