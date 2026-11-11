import ast,importlib,json,unittest
from pathlib import Path
import jsonschema,numpy as np
from manim import Camera,ValueTracker,tempconfig
r=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.📍️references.🐍️')
case=json.loads(Path(__file__).with_name('126-roof-cases.json').read_text(encoding='utf-8'))
schema=json.loads(Path(__file__).with_name('126-roof-cases.schema.json').read_text(encoding='utf-8'))
class Roof(unittest.TestCase):
 def test_live_roof_result(self):
  jsonschema.validate(case,schema)
  tree=ast.parse(Path(r.__file__).read_text(encoding='utf-8'))
  cls=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=='Beat2_References')
  method=next(n for n in cls.body if isinstance(n,ast.FunctionDef) and n.name=='present_references')
  start=next(i for i,n in enumerate(method.body) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Attribute) and t.attr=='formula_pieces' for t in n.targets))
  end=next(i for i in range(start+1,len(method.body)) if isinstance(method.body[i],ast.Expr) and 'self.w.animate.set_value' in ast.unparse(method.body[i]))
  code=compile(ast.fix_missing_locations(ast.Module(body=method.body[start:end+1],type_ignores=[])),r.__file__,'exec')
  with tempconfig({'dry_run':True,'pixel_width':320,'pixel_height':180,'frame_rate':3,'background_color':'#000000'}):
   for lang in case['languages']:
    scene=r.Beat2_References();scene.language=lambda:lang;scene.w=ValueTracker(case['inputStart'])
    book=r._xl.workbook()
    def build():
     w=scene._window(book,r.SHEET,'A:D',(1,10),'D3',language=lang)
     w.bind('D3',lambda:scene.xl_display(scene.w.get_value()*case['length'],'0.0 "m²"',lang))
     return w
    win=build();win.edit('=B2*B3');scene.add(win)
    expected=build();expected.edit(None)
    env={'self':scene,'win':win,'width0':case['inputStart'],'marks':[],'rest':[],'_m':r._m,'blue':'#4472C4','red':'#FF0000'}
    exec(code,env)
    expected.update(0);scene.renderer.update_frame(scene)
    oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
    oracle.capture_mobjects([expected])
    a=np.asarray(scene.camera.get_image(),dtype=int);b=np.asarray(oracle.get_image(),dtype=int)
    left,top,right,bottom=win.rect('D3');fw=scene.camera.frame_width;fh=scene.camera.frame_height
    x0=int((left/fw+.5)*320)+3;x1=int((right/fw+.5)*320)-3
    y0=int((.5-top/fh)*180)+3;y1=int((.5-bottom/fh)*180)-3
    self.assertLessEqual(int(np.abs(a[y0:y1,x0:x1]-b[y0:y1,x0:x1]).max()),3,lang)
if __name__=='__main__':unittest.main()
