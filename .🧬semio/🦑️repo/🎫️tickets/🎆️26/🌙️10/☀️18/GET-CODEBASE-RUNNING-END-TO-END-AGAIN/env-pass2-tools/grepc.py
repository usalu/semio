import sys,re
# usage: grepc.py <file> <regex> [width=250] [max=6]
p,rx=sys.argv[1],sys.argv[2];w=int(sys.argv[3]) if len(sys.argv)>3 else 250;mx=int(sys.argv[4]) if len(sys.argv)>4 else 6
t=open(p,encoding="utf-8",errors="replace").read()
n=0
for m in re.finditer(rx,t):
    print("@",m.start(),":",t[max(0,m.start()-w):m.end()+w].replace("\n","\\n"));print()
    n+=1
    if n>=mx: break
