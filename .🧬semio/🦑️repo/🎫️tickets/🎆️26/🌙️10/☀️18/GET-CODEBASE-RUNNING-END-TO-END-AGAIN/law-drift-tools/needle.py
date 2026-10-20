import sys,re
f,line,col=sys.argv[1],int(sys.argv[2]),int(sys.argv[3])
L=open(f,encoding='utf-8').read().split("\n")
l=L[line-1]
# col is 1-based UTF-16 col? Bun reports 1-based col in UTF-16 units of the source line
u=l.encode('utf-16-le'); idx=len(u[:2*(col-1)].decode('utf-16-le',errors='ignore'))
s=max(0,idx-420); print(l[s:idx+60])
