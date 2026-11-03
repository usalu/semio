import re,sys
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/"
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old,s.count(old))
    return s.replace(old,new)
for a in ("toaster","vacuum-cleaner"):
    p=B+a+"/🔣️.json"
    s=open(p,encoding="utf-8",newline="").read()
    nl="\r\n" if "\r\n" in s else "\n"
    def blk(b,d,t,g): return nl.join(['"pens": {',f'          "body": {b},',f'          "detail": {d},',f'          "texture": {t},','          "glazing": null,',f'          "ground": {g}'])
    s=sub(s,blk("0.35","0.18","0.13","0.5"),blk("0.18","0.13","0.1","0.35"))
    s=sub(s,blk("0.25","0.13","null","0.35"),blk("0.18","0.13","null","0.25"))
    if a=="toaster":
        s=sub(s,"body pen 0.35 mm, detail 0.18 mm.","body pen 0.18 mm, detail 0.13 mm, texture 0.1 mm.")
        s=sub(s,"Körperstift 0,35 mm, Detail 0,18 mm.","Körperstift 0,18 mm, Detail 0,13 mm, Textur 0,1 mm.")
        s=sub(s,"body pen 0.25 mm, detail 0.13 mm.","body pen 0.18 mm, detail 0.13 mm.")
        s=sub(s,"Körperstift 0,25 mm, Detail 0,13 mm.","Körperstift 0,18 mm, Detail 0,13 mm.")
    else:
        s=sub(s,"body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm.","body pen 0.18 mm, detail 0.13 mm, texture 0.1 mm.")
        s=sub(s,"Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm.","Körperstift 0,18 mm, Detail 0,13 mm, Textur 0,1 mm.")
        s=sub(s,"body pen 0.25 mm, detail 0.13 mm.","body pen 0.18 mm, detail 0.13 mm.")
        s=sub(s,"Körperstift 0,25 mm, Detail 0,13 mm.","Körperstift 0,18 mm, Detail 0,13 mm.")
    open(p,"w",encoding="utf-8",newline="").write(s)
    print("ok",a)
