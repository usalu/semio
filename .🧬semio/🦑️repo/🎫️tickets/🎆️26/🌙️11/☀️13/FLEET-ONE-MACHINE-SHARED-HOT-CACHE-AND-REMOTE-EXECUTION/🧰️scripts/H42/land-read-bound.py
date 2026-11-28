import os,sys
p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts"
s=open(p,encoding="utf8").read()
n=s.count("policy.maximumFileReadBytes")
assert n==2,n
s=s.replace("policy.maximumFileReadBytes","policy.maximumReadBytes")
open(p+".h42.tmp","w",encoding="utf8").write(s);os.replace(p+".h42.tmp",p)
print("replaced",n)
