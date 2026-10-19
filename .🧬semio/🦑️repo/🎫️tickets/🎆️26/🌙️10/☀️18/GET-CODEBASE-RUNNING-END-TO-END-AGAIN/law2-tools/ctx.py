import sys,re,json
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
lst=open(C+"/law2-tools/list-mine.txt",encoding="utf-8").read().split("\n")
i=int(sys.argv[1]); f=lst[i]
s=open("/Users/ueli/Documents/semio/"+f,encoding="utf-8").read()
if len(sys.argv)>2:
    pat=sys.argv[2]; w=int(sys.argv[3]) if len(sys.argv)>3 else 300
    for m in list(re.finditer(pat,s))[:int(sys.argv[4]) if len(sys.argv)>4 else 3]:
        print("@",m.start(),s[max(0,m.start()-w):m.start()+w].replace("\n","\\n"));print()
else:
    print(f); print(len(s)); print(s[:3000])
