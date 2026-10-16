"""🪡 Anchor-verified text replacement used by every relo-hubgis edit script."""
import sys
class Edit:
    def __init__(self,path):
        self.path=path; self.t=open(path,encoding='utf-8').read()
    def rep(self,a,b,n=1):
        c=self.t.count(a)
        assert c==n,(self.path,a[:120],c)
        self.t=self.t.replace(a,b)
    def cut(self,start,end,fns):
        s=self.t.index(start); e=self.t.index(end)
        assert s<e and self.t[s:e].count('fn ')==fns,(self.t[s:e].count('fn '),)
        self.t=self.t[:s]+self.t[e:]
    def save(self):
        open(self.path,'w',encoding='utf-8').write(self.t)
