"""🖼️ Render the actual bilingual paper-scale label and check native Cairo bounds."""
import ast,importlib,os,sys
from pathlib import Path
from manim import Camera,tempconfig
sys.path.insert(0,str(Path.cwd()))
m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.👁️representations.🐍️")
source=Path(m.__file__)
tree=ast.parse(source.read_text(encoding="utf8"))
owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat3_SameObject")
method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_beat_scale20")
label=next(n for n in ast.walk(method) if isinstance(n,ast.JoinedStr) and any(isinstance(v,ast.Constant) and " mm" in str(v.value) for v in n.values))
out=Path(__file__).parent/"🗑️generated"
for language in ("en","de"):
    with tempconfig({"dry_run":True,"pixel_width":854,"pixel_height":480,"background_color":"#000000"}):
        scene=m.Beat3_SameObject()
        scene.language=lambda:language
        text=eval(compile(ast.Expression(label),str(source),"eval"),{"__builtins__":{}},{"self":scene,"t":0.29})
        assert ("14.5" if language=="en" else "14,5") in text,text
        drawing=scene.text(text,(0,0,0),size=m.S_LABEL,color=m.DATA)
        assert drawing.width<3.0 and drawing.height<0.6,(language,drawing.width,drawing.height)
        camera=Camera(pixel_width=854,pixel_height=480,background_color="#000000",background_opacity=1)
        camera.capture_mobjects([drawing])
        camera.get_image().save(out/f"bim116-paper-{language}.png")
        print("[DEBUG]",language,repr(text),"width",drawing.width,"height",drawing.height,flush=True)
