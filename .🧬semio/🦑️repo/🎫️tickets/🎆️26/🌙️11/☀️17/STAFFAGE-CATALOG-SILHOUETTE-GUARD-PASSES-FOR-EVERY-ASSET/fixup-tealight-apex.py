import sys
P="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/tealight/🔣️.json"
a,b=sys.argv[1:3]
s=open(P,encoding="utf8").read()
o1='[[0.0001, 0.0], [0.0, 0.0], [0.0, 0.0165], [0.014, 0.02088], [0.016, 0.0215], [0.016, 0.0214], [0.014, 0.02077], [0.0001, 0.0164]]'
n1=o1.replace("0.0165",a).replace("0.0164",b)
o2='[[8e-05, 0.0], [8e-05, 0.01648], [0.014, 0.02083]'
n2='[[8e-05, 0.0], [8e-05, %s], [0.014, 0.02083]'%b
assert s.count(o1)==1 and s.count(o2)==1
open(P,"w",encoding="utf8").write(s.replace(o1,n1).replace(o2,n2))
