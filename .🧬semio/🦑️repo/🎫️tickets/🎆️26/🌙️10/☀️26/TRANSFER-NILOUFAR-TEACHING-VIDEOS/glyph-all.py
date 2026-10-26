import ast,json,unicodedata
from pathlib import Path
from fontTools.ttLib import TTFont,TTCollection
root=Path("C:/git/semio/🎓️teaching")
tree=ast.parse((root/"manim_visuals.py").read_text(encoding="utf-8-sig"))
symbols=next(ast.literal_eval(n.value) for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(x,ast.Name) and x.id=="_MATH_SYMBOLS" for x in n.targets))
declared=set("".join(symbols.values()))
direct=set(); literals=set(); examples={}
def relevant(c):
 return unicodedata.category(c)=="Sm" or "GREEK" in unicodedata.name(c,"") or 0x2190<=ord(c)<=0x21ff or c in "²³⁴⁰¹₀₁₂₃₄₅₆₇₈₉·×÷−°"
for p in root.rglob("*.py"):
 if any(x in p.parts for x in [".agents","node_modules",".venv"]): continue
 try: tr=ast.parse(p.read_text(encoding="utf-8-sig"))
 except SyntaxError: continue
 docs={id(n.body[0].value) for n in ast.walk(tr) if isinstance(n,(ast.Module,ast.ClassDef,ast.FunctionDef,ast.AsyncFunctionDef)) and n.body and isinstance(n.body[0],ast.Expr) and isinstance(n.body[0].value,ast.Constant) and isinstance(n.body[0].value.value,str)}
 for n in ast.walk(tr):
  if isinstance(n,ast.Constant) and isinstance(n.value,str) and id(n) not in docs:
   for c in n.value:
    if relevant(c):
     literals.add(c);examples.setdefault(c,[]).append(str(p.relative_to(root))+":"+str(n.lineno))
  if isinstance(n,ast.Call):
   name=n.func.id if isinstance(n.func,ast.Name) else n.func.attr if isinstance(n.func,ast.Attribute) else ""
   if name in {"Text","body_text","centered_body_text","math_label","math_text","equation_row","symbol_token","note_line","chip","_back_chip","beat_subtitle"}:
    for arg in n.args:
     for sub in ast.walk(arg):
      if isinstance(sub,ast.Constant) and isinstance(sub.value,str):
       direct.update(c for c in sub.value if relevant(c))
required=declared|literals
fonts=[("Georgia",Path("C:/Windows/Fonts/georgia.ttf"),0),("Times New Roman",Path("C:/Windows/Fonts/times.ttf"),0),("Cambria Math",Path("C:/Windows/Fonts/cambria.ttc"),1)]
for p in Path("C:/git/semio/.venv").rglob("DejaVuSerif.ttf"): fonts.append(("DejaVu Serif",p,0))
coverage=[]
for name,p,index in fonts:
 f=TTCollection(p).fonts[index] if p.suffix==".ttc" else TTFont(p); cmap=f.getBestCmap()
 coverage.append({"font":name,"path":str(p),"missing_declared":"".join(sorted(declared-set(map(chr,cmap)))),"missing_direct":"".join(sorted(direct-set(map(chr,cmap)))),"missing_all_literals":"".join(sorted(required-set(map(chr,cmap))))})
out={"declared_symbols":symbols,"direct_call_characters":"".join(sorted(direct)),"all_non_docstring_math_literals":"".join(sorted(required)),"fonts":coverage,"georgia_missing_examples":{c:examples.get(c,[])[:5] for c in required if ord(c) not in TTFont("C:/Windows/Fonts/georgia.ttf").getBestCmap()}}
print(json.dumps(out,ensure_ascii=False,indent=2))
output=Path(__file__).parent/"🗑️generated"/"glyph-all.json"
output.parent.mkdir(parents=True,exist_ok=True)
output.write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding="utf-8")
