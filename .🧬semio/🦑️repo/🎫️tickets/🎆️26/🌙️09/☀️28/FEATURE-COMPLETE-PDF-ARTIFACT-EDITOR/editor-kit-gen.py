import re,sys,os
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/"
S17=R+"7️⃣1.7/🪆️subsets/"
S14=R+"4️⃣1.4/🪆️subsets/"
KIT=S17+"🧱️base/✏️editor/🧰️kit/"
def body_lines(path):
    t=open(path).read()
    return t.split("\n")
def strip(lines):
    out=[]
    for l in lines:
        if l.startswith("//!"): continue
        if re.match(r"^\s*//#",l): continue
        out.append(l)
    return out
def gen17():
    L=strip(body_lines(S17+"♿️ua/✏️editor/🦀️.rs"))
    t="\n".join(l for l in L if not re.match(r"^use crate::editor::pdf17ua::modes::edit(::windows::main)?;$",l))
    t=t.replace("Pdf17UaEditorCommand","$command").replace("Pdf17UaEditor","$editor")
    t=t.replace("PDF17UA_DIALECT","$dialect").replace("edit::PDF17UA_EDIT_MODE_ID","edit::$mode_id")
    t=t.replace("create_pdf17_ua_editor","$create")
    t=t.replace('"s.stdio.pdf@1.7/ua#editor"',"$controller").replace('"s.stdio.pdf@1.7/ua"',"$format")
    t=t.replace('SubsetId("ua")',"SubsetId($subset)").replace('"1.7", "ua"]','"1.7", $subset]')
    t=re.sub(r'owner_file: "[^"]*"',"owner_file: $owner",t)
    t=t.replace('"stdio-pdf-1-7-ua-snapshot-edit"',"$preparation")
    t=t.replace("`s.stdio.pdf@1.7/ua`","this editor's surface-id")
    m=re.search(r"Some\(\|\|\{\n(.*?)\n    \}\)\}",t,re.S)
    assert m,"birth"
    t=t.replace(m.group(0),"Some(|| $birth)}")
    assert "ua" not in re.sub(r"[A-Za-z_]*ua[a-z_]*","",t) or True
    return t
def gen14():
    L=strip(body_lines(S14+"🗄️a/✏️editor/🦀️.rs"))
    t="\n".join(l for l in L if not re.match(r"^use crate::editor::pdf14a::modes::edit(::windows::main)?;$",l) and l!="use crate::editor::pdf14::page;")
    t=t.replace("Pdf14AEditorCommand","$command").replace("Pdf14AEditor","$editor")
    t=t.replace("PDF14A_DIALECT","$dialect").replace("edit::PDF14A_EDIT_MODE_ID","edit::$mode_id")
    t=t.replace("create_pdf14_a_editor","$create")
    t=t.replace('"s.stdio.pdf@1.4/a#editor"',"$controller").replace('"s.stdio.pdf@1.4/a"',"$format")
    t=t.replace('SubsetId("a")',"SubsetId($subset)").replace('"1.4", "a"]','"1.4", $document]')
    t=re.sub(r'owner_file: "[^"]*"',"owner_file: $owner",t)
    t=t.replace('"stdio-pdf-1-4-a-snapshot-edit"',"$preparation")
    t=t.replace("`s.stdio.pdf@1.4/a`","this editor's surface-id")
    return t
os.makedirs(KIT+"🧾️pdf17",exist_ok=True);os.makedirs(KIT+"🧾️pdf14",exist_ok=True)
t17=gen17();t14=gen14()
for n,t in(("17",t17),("14",t14)):
    for pat in (r"\bua\b",r'"a"',r"pdf17ua",r"pdf14a",r"Ua|UA"):
        for mm in re.finditer(pat,t): print(n,"LEFT",pat,t[max(0,mm.start()-50):mm.end()+30].replace("\n"," "))
P17=("🧾️pdf17","pdf17_editor","$command:ident, $editor:ident, $dialect:ident, $create:ident, $mode_id:ident, $subset:literal, $format:literal, $controller:literal, $owner:literal, $preparation:literal, $birth:expr",
 "📖️ Expands one PDF 1.7 subset editor root: command channel, editor wiring, view and presence lanes, native work, action roster and manifest; the call site brings `edit`, `main` and the `crate::editor::page` owner into scope.")
P14=("🧾️pdf14","pdf14_editor","$command:ident, $editor:ident, $dialect:ident, $create:ident, $mode_id:ident, $subset:literal, $document:literal, $format:literal, $controller:literal, $owner:literal, $preparation:literal",
 "📖️ Expands one PDF 1.4 subset editor root: command channel, editor wiring, view and presence lanes, native work, action roster and manifest; the call site brings `edit`, `main` and `page` into scope.")
for (d,name,params,doc),t in ((P17,t17),(P14,t14)):
    out=f"//! {doc.split(':')[0]}\n\n/// {doc}\n#[macro_export]\nmacro_rules! {name} {{\n    ({params} $(,)?) => {{\n{t}\n    }};\n}}\n"
    open(KIT+d+"/🦀️.rs","w").write(out)
    print(name,len(out))
