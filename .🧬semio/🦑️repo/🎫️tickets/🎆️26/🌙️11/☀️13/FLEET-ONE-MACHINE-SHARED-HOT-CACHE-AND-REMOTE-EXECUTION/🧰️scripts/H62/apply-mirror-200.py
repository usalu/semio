import os,sys
f="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts"
s=open(f,encoding="utf8").read()
old=""" const raw=logTailV1(log,1,P.mirrorLineCharacters),quiet=failureLinesV1(log,P.mirrorLines-1,P.mirrorLineCharacters,[]),last=raw[0];
 return last===undefined||quiet.at(-1)===last?quiet:[...quiet,last];"""
new=""" const last=logTailV1(log,1,P.mirrorLineCharacters)[0],full=failureLinesV1(log,P.mirrorLines,P.mirrorLineCharacters,[]);
 return last===undefined||full.at(-1)===last?full:[...failureLinesV1(log,P.mirrorLines-1,P.mirrorLineCharacters,[]),last];"""
if s.count(old)!=1: sys.exit("anchor count %d"%s.count(old))
open(f+".h62.tmp","w",encoding="utf8").write(s.replace(old,new));os.replace(f+".h62.tmp",f)
print("applied")
