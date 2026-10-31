"""🗣 Regenerate bilingual teaching transcripts from authored narration and shared constants."""
import ast
import json
import importlib
import sys
from pathlib import Path

ROOT=Path.cwd()/"🎓️teaching/🏛️architecture"
model=json.loads((ROOT/"🏢️nakagin/🔣️.json").read_text(encoding="utf-8"))
storeys=next(t for t in model["towers"] if t["key"]=="B")["capsule_storeys"]["value"]
sys.path.insert(0,str(Path.cwd()))
context={"STOREYS":storeys,"SLIDER":storeys-.4,"NB":storeys,"NA":model["towers"][0]["capsule_storeys"]["value"],"_gh":importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️"),"_nk":importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")}
context.update(_nak=context["_nk"],_A=context["_nk"].towers()[0],_B=context["_nk"].towers()[1],_PER=context["_nk"].totals()["per_tower"],_TOTAL=context["_nk"].totals()["count"])
series={"🦗️grasshopper":(("🦴️anatomy","grasshopper_anatomy"),("💊️capsule","grasshopper_capsule"),("🗼️tower","grasshopper_tower"),("🫗️casting","grasshopper_casting"),("🌳️trees","grasshopper_trees")),"📊️excel":(("🦴️anatomy","excel_anatomy"),("📍️references","excel_references"),("📋️program","excel_program"),("⚡️energy","excel_energy"))}
for folder,chapters in series.items():
    root=ROOT/folder/"🎬️clip"
    for language,index in (("en",1),("de",2)):
        target=root/f"🗣️transcript.{language}.md"
        original=target.read_bytes()
        intro=original.decode("utf-8").split("\n## ",1)[0].rstrip()
        sections=[intro]
        for topic,video in chapters:
            module=ast.parse((root/topic/"🐍️.py").read_text(encoding="utf-8"))
            namespace={**context}
            for helper in module.body:
                if isinstance(helper,ast.FunctionDef) and helper.name in ("_figures","_m"):
                    exec(compile(ast.Module([helper],type_ignores=[]),str(target),"exec"),namespace)
            values={node.targets[0].id:eval(compile(ast.Expression(node.value),str(target),"eval"),namespace) for cls in module.body if isinstance(cls,ast.ClassDef) for node in cls.body if isinstance(node,ast.Assign) and isinstance(node.targets[0],ast.Name) and node.targets[0].id in ("TITLE","NARRATION")}
            sections.append("## "+values["TITLE"][language]+"\n\nVideo: "+chr(96)+video+chr(96)+"\n\n"+"\n\n".join(row[index] for row in values["NARRATION"]))
        payload=("\n\n".join(sections)+"\n").encode("utf-8")
        if target.read_bytes()!=original:
            raise RuntimeError(f"Concurrent transcript change: {target}")
        temporary=Path(__file__).parent/"🗑️generated"/"transcript-sync"/folder/target.name
        temporary.parent.mkdir(parents=True,exist_ok=True)
        temporary.write_bytes(payload)
        temporary.replace(target)
print("[DEBUG] Bilingual narration and transcripts synchronized")
