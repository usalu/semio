import os,shutil,sys
R="/Users/ueli/Documents/semio"
class Stage:
    def __init__(self,S):
        shutil.rmtree(S,ignore_errors=True);os.makedirs(S);self.S=S;self.rows=[];self.n=0
    def edit(self,rel,subs):
        i=str(self.n);self.n+=1;base=open(R+"/"+rel,encoding="utf8").read();cur=base
        for old,rep,count in subs:
            assert cur.count(old)==count,(rel,old[:70],cur.count(old))
            cur=cur.replace(old,rep)
        open(self.S+"/"+i+".base","w",encoding="utf8").write(base);open(self.S+"/"+i+".new","w",encoding="utf8").write(cur);self.rows.append((i,rel,"edit"))
    def new(self,rel,text):
        i=str(self.n);self.n+=1;open(self.S+"/"+i+".new","w",encoding="utf8").write(text);self.rows.append((i,rel,"new"))
    def finish(self,tests,note):
        open(self.S+"/index.txt","w",encoding="utf8").write("\n".join(" ".join(r) for r in self.rows)+"\n")
        open(self.S+"/tests.txt","w",encoding="utf8").write("\n".join(tests)+"\n");open(self.S+"/note.txt","w",encoding="utf8").write(note)
SCR="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R80/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/"
PREF=[]
