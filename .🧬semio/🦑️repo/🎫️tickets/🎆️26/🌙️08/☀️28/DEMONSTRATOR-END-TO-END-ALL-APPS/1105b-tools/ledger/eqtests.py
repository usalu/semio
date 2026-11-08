import re,sys
B="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/"
def call_end(s,i):
    d=0
    while True:
        c=s[i]
        if c=="(":d+=1
        elif c==")":
            d-=1
            if d==0:return i
        i+=1
def rewrite(s,recv,method,new,count_args=None):
    pat=re.compile(r'(?<![\w.])'+re.escape(recv)+r'\.'+method+r'\(')
    out="";k=0
    while True:
        m=pat.search(s,k)
        if not m: return out+s[k:]
        e=call_end(s,m.end()-1)
        args=s[m.end():e]
        out+=s[k:m.start()]+new.replace("{r}",recv)+"("+("&mut "+recv+("," if args else ""))+args+")"
        k=e+1
def apply(path,rules,prefix=""):
    s=open(B+path,encoding='utf8').read()
    for r in rules: s=rewrite(s,*r)
    open(B+path,'w',encoding='utf8').write(s)
T="🌳️tree/🟰️equal/"
apply(T+"🧪️tests/🦀️.rs",[("owner","advance","crate::eff::adv_owner"),("owner","close_step","crate::eff::close_owner"),("field","advance","crate::eff::adv_cursor"),("field","close_step","crate::eff::close_cursor")])
apply(T+"🧪️tests/actual.rs",[("owner","advance","crate::eff::adv_owner"),("owner","close_step","crate::eff::close_owner")])
apply(T+"🌱️intrinsic/🧪️tests.rs",[("cursor","advance","crate::eff::adv_value"),("cursor","close_step","crate::eff::close_value")])
apply(T+"📋️sequence/🦀️.rs",[("c","advance","crate::eff::adv_cursor"),("c","close_step","crate::eff::close_cursor")])
print("ok")
