#!/usr/bin/env python3
"""Moves the gesture preview vocabulary (Style, Shape, Mark, Preview) from gestures::session into the new editor::bim::marks so the world, sheet and family windows no longer depend on the gestures. Mark::snap becomes the session function snap_mark (it needs the snap hit). usage: marks.py [--apply]"""
import re,os,subprocess,sys
APPLY="--apply" in sys.argv
E="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
sess=E+"🧵️gestures/🧭️session/🦀️.rs"
t=open(sess).read()
a=t.index("//#region 🔖️Preview"); b=t.index("//#endregion 🔖️Preview")
reg=t[a:b+len("//#endregion 🔖️Preview")]
# split off length_label
ll=re.search(r"/// 📏️ A length label.*?\n}\n",reg,re.S).group(0)
body=reg.replace(ll,"")
# split off Mark::snap
sm=re.search(r"\n    pub fn snap\(hit: &SnapHit\) -> Self \{.*?\n    \}\n",body,re.S).group(0)
body=body.replace(sm,"")
snap_fn='''
/// 🧲️ The marker a snap hit shows: a snap mark named by the kind of the hit.
pub fn snap_mark(hit: &SnapHit) -> Mark {
    Mark { shape: Shape::Snap, style: Style::Snap, closed: false, points: hit.point.to_vec(), text: format!("{:?}", hit.kind).to_lowercase() }
}
'''
body=body.replace("//#region 🔖️Preview","//#region 🔖️Marks").replace("//#endregion 🔖️Preview","//#endregion 🔖️Marks")
marks_doc="//! 🖍️ The marks a tool shows over a window while it works: how a mark is drawn, what it is, the mark itself and the preview of a tool, shared by the gestures and by the windows that paint them.\n\nuse crate::editor::bim::gestures::plane::P;\nuse value_derive::{FromValue, ToValue};\n\n"
new_session=t.replace(reg,"//#region 🔖️Preview\n"+ll+snap_fn+"//#endregion 🔖️Preview")
new_session=new_session.replace("use super::plane::P;\n","use super::plane::P;\nuse crate::editor::bim::marks::{Mark, Preview, Shape, Style};\n",1)
N=["Style","Shape","Mark","Preview"]
M="crate::editor::bim::marks::"
fs=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","--","✏️s/🔌️plugins/🏙️bim"],capture_output=True,text=True).stdout.split("\n")
edits={}
grp=re.compile(r"use ((?:crate::editor::bim::gestures|super|self)?(?:::)?session)::\{([^}]*)\};")
for f in fs:
    if not (f.endswith(".rs") and os.path.isfile(f)) or "✏️editor" not in f or f==sess: continue
    s=open(f).read(); o=s
    def g(m):
        names=[x.strip() for x in m.group(2).split(",") if x.strip()]
        mv=[n for n in names if n in N]; rest=[n for n in names if n not in N]
        if not mv: return m.group(0)
        out=[]
        if rest: out.append(f"use {m.group(1)}::{{{', '.join(rest)}}};" if len(rest)>1 else f"use {m.group(1)}::{rest[0]};")
        out.append(f"use {M}{{{', '.join(mv)}}};" if len(mv)>1 else f"use {M}{mv[0]};")
        return "\n".join(out)
    s=grp.sub(g,s)
    s=re.sub(r"use ((?:crate::editor::bim::gestures|super|self)?(?:::)?session)::("+"|".join(N)+r");",lambda m:f"use {M}{m.group(2)};",s)
    s=re.sub(r"((?:crate::editor::bim::gestures::|super::|self::)?\bsession)::("+"|".join(N)+r")\b(?!\s*;)",lambda m:M+m.group(2),s)
    s=s.replace("Mark::snap(","snap_mark(")
    if "snap_mark(" in s and "snap_mark" not in o: 
        s=re.sub(r"(use super::session::\{)",r"\1snap_mark, ",s,count=1) if "use super::session::{" in s else s.replace("snap_mark(","crate::editor::bim::gestures::session::snap_mark(")
    if s!=o: edits[f]=s
print("edited",len(edits))
if APPLY:
    for f,s in edits.items(): open(f,"w").write(s)
    open(sess,"w").write(new_session)
    d=E+"🖍️marks"; os.makedirs(d,exist_ok=True); open(d+"/🦀️.rs","w").write(marks_doc+body+"\n")
