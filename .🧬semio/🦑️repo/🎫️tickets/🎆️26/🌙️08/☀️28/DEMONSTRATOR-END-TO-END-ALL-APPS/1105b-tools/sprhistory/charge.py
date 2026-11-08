import sys
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/"
def balanced_end(t,k):
    depth=1
    while depth:
        c=t[k];depth+=(c=='(')-(c==')');k+=1
    return k
def strip(t,pat,repl):
    out=[];i=0;n=0
    while True:
        j=t.find(pat,i)
        if j<0:
            out.append(t[i:]);break
        out.append(t[i:j])
        k=balanced_end(t,j+len(pat))
        assert t[k:k+2]=="?;",t[k:k+12]
        out.append(repl);i=k+2;n+=1
    return "".join(out),n
want={"🧵️edit":7,"🔀️transition":2,"⚔️conflict":5,"📇️identity-records":1,"🧾️metadata":7,"🗂️file":8}
for d,w in want.items():
    p=B+d+"/🦀️.rs";t=open(p,encoding="utf-8").read();total=0
    t,n=strip(t,"control.charge(","");total+=n
    t,n=strip(t,"Self::charge_work(&mut self.last_work,control,","self.last_work=1;");total+=n
    print(d,total)
    open(p,"w",encoding="utf-8").write(t)
