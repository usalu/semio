import sys,re
sys.path.insert(0,"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-tools/testdebt-g")
import tx
E="crate::effect"
def adv(m,a):
    r=m.group(1);p=[x.strip() for x in tx.split_args(a)]
    if p[-1].startswith("&mut") or p[-1]=="effect" or "effect" in p:return None
    if len(p)==1:
        if p[0].startswith("&"):return None
        return "%s::settle(%s,|effect|%s.advance(effect))"%(E,p[0],r)
    if len(p)==2 and p[1].startswith("|"):return "%s::run(%s,|effect|%s.advance(effect,%s))"%(E,p[0],r,p[1])
    if len(p)==2 and p[0].startswith("&"):return "%s::run(%s,|effect|%s.advance(%s,effect))"%(E,p[1],r,p[0])
    if len(p)==3:return "%s::settle(%s,|effect|%s.advance(%s,%s,effect))"%(E,p[2],r,p[0],p[1])
    if len(p)==5 and p[0].startswith("&"):return "%s::settle(%s,|effect|%s.advance(%s,%s::WORK,effect))"%(E,p[4],r,",".join(p[:4]),E)
    print("UNHANDLED",r,p);return None
R=[(r"SealedShared::admit\(",tx.admit),(r"(\w+)\.close_step\(",tx.close_step),(r"\b(owner|retired|close)\.step\(",tx.step),(r"\b(owner|plan|preparation|cursor)\.advance\(",adv)]
if __name__=="__main__":
    for p in sys.argv[1:]:tx.apply(p,R)
