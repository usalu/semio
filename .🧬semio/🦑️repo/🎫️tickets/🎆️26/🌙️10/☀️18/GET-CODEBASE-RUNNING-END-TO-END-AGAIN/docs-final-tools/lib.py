import sys
class Doc:
    def __init__(self,path):
        self.path=path
        self.L=open(path,encoding="utf-8").read().split("\n")
    def split(self):
        marks=[i for i,l in enumerate(self.L) if l.startswith("## 🇩🇪️ Deutsch")]
        assert len(marks)<=1,marks
        return marks[0] if marks else len(self.L)
    def idx(self,prefix,lang=None,nth=0):
        s=self.split()
        rng=range(len(self.L)) if lang is None else (range(0,s) if lang=="en" else range(s,len(self.L)))
        r=[i for i in rng if self.L[i].startswith(prefix)]
        assert len(r)>nth,("missing",prefix,lang,r)
        if nth==0: assert len(r)==1,("ambiguous",prefix,lang,r)
        return r[nth]
    def append(self,prefix,text,lang=None):
        i=self.idx(prefix,lang); assert text not in self.L[i],("already",prefix); self.L[i]+=text
    def replace_in(self,prefix,old,new,lang=None):
        i=self.idx(prefix,lang); assert self.L[i].count(old)==1,("old count",prefix,old[:60],self.L[i].count(old)); self.L[i]=self.L[i].replace(old,new)
    def replace_line(self,prefix,new,lang=None):
        i=self.idx(prefix,lang); self.L[i]=new
    def insert_after(self,prefix,lines,lang=None):
        i=self.idx(prefix,lang); 
        if isinstance(lines,str): lines=lines.split("\n")
        assert lines[-1] not in self.L[i:i+len(lines)+3],("already",prefix)
        self.L[i+1:i+1]=lines
    def insert_before(self,prefix,lines,lang=None):
        i=self.idx(prefix,lang)
        if isinstance(lines,str): lines=lines.split("\n")
        self.L[i:i]=lines
    def tail_replace(self,prefix,start,new,lang=None):
        i=self.idx(prefix,lang); j=self.L[i].index(start); assert self.L[i].count(start)==1; self.L[i]=self.L[i][:j]+new
    def save(self):
        open(self.path,"w",encoding="utf-8").write("\n".join(self.L))
