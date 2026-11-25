import glob,os,sys
def edit(pattern, pairs):
    p=glob.glob(pattern)[0]
    s=open(p,encoding='utf-8',newline='').read()
    for a,b in pairs:
        assert a in s,(p,a[:60])
        s=s.replace(a,b,1)
    with open(p,'r+',encoding='utf-8',newline='') as f:
        f.seek(0); f.write(s); f.truncate()
    print('ok',p)
