import sys,os,re,json
def leb(b,i):
    r=0;s=0
    while True:
        x=b[i];i+=1;r|=(x&0x7f)<<s;s+=7
        if not x&0x80:return r,i
def fcount(b):
    if b[:4]!=b'\0asm':return 0
    i=8;n=len(b)
    while i<n:
        id=b[i];i+=1
        sz,i=leb(b,i)
        if id==3:
            c,_=leb(b,i);return c
        i+=sz
    return 0
def rlib(path):
    b=open(path,'rb').read();i=8;tot=0
    while i+60<=len(b):
        hdr=b[i:i+60];sz=int(hdr[48:58]);i+=60
        body=b[i:i+sz]
        tot+=fcount(body)
        i+=sz+(sz&1)
    return tot
res={}
for dp,_,fs in os.walk(sys.argv[1]):
    for f in fs:
        if f.endswith('.rlib'):
            m=re.match(r'lib(.+)-[0-9a-f]+\.rlib',f)
            res[m.group(1)]=res.get(m.group(1),0)+rlib(os.path.join(dp,f))
json.dump(res,open(sys.argv[2],'w'))
print(len(res),sum(res.values()))
