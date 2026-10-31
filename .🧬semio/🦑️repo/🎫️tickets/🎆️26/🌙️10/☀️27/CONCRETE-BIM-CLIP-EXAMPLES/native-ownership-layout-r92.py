"""⚖️ Validate the new bilingual allowance clause using actual native text metrics and1920×1080 Cairo frames."""
import hashlib,importlib,json,os,sys
from pathlib import Path
ROOT=Path.cwd();sys.path.insert(0,str(ROOT));TICKET=Path(__file__).parent;OUT=TICKET/'🗑️generated/native-r92/ownership'
OUT.mkdir(parents=True,exist_ok=True);os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker')
v=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️');v._fonts.apply_body_font()
from manim import tempconfig
case=json.loads((ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🧩️chapter-source/🔣️.json').read_text())['finish_ownership']
source=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔌️exchange/🐍️.py';before=hashlib.sha256(source.read_bytes()).hexdigest();rows=[]
with tempconfig({'media_dir':str(OUT),'pixel_width':1920,'pixel_height':1080,'frame_width':128/9,'frame_height':8.,'background_color':'#F7F7F2','dry_run':True}):
 for language in ('en','de'):
  for audience in ('speaker','spectator'):
   os.environ.update(SEMIO_TEACHING_LANGUAGE=language,SEMIO_TEACHING_AUDIENCE=audience)
   if (OUT/'cancel').exists():raise KeyboardInterrupt('Ownership layout cancelled')
   scene=v.ClipScene();note=scene.label(case['note_en'],case['note_de'],(-3.4,-1.4,0),size=16.5,color=v.MUTED,width=6.5);caption=v._art.caption_bar(case[language])
   assert note.get_left()[0]>=-128/18 and note.get_right()[0]<=0
   assert note.get_bottom()[1]>caption.get_top()[1]+.1
   assert note.get_top()[1]<0 and note.font_size>=16
   assert len(case[language].split())/case['seconds']<=3
   scene.camera.capture_mobjects([note,caption]);path=OUT/(language+'-'+audience+'.png');scene.camera.get_image().save(path)
   assert len(str(path).encode('utf-16-le'))//2<=256
   rows.append({'language':language,'audience':audience,'note_width':note.width,'note_height':note.height,'font_size':note.font_size,'caption_width':caption.width,'caption_height':caption.height,'words_per_second':len(case[language].split())/case['seconds'],'image':str(path.relative_to(TICKET))})
after=hashlib.sha256(source.read_bytes()).hexdigest();report={'stable':before==after,'source_before':before,'source_after':after,'rows':rows}
(OUT/'result.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(report),flush=True)
raise SystemExit(0 if report['stable'] else 1)
