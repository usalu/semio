"""🥞 Render measured pointer motion and chip previews through the actual shared UI."""
import importlib,json,os,shutil,sys,time
from pathlib import Path
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT));sys.dont_write_bytecode=True
BASE='🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring'
browser=importlib.import_module(BASE+'.🎬️clip.🪟️presentation.🌐️browser.🐍️')
runtime=importlib.import_module(BASE+'.🎬️clip.⚙️runtime.🐍️')
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT']).resolve();output.mkdir(parents=True,exist_ok=True)
fps=12
catalog=importlib.import_module(BASE+'.🎬️clip.🪟️presentation.🥞️hierarchy.🐍️').hierarchy_contract()
root={'opened_topic':None,'opened_video':None,'revealed_topic':None,'revealed_video':None,'camera':{'x':.5,'y':.5}}
energy={**root,'opened_topic':'energy'}
opened={**energy,'opened_video':'physical_fundamentals'}


def packet(frames):
    """🌐 Keep the private browser protocol behind the production Python port."""
    return browser.render_batch({'schema':'semio-presentation-browser/v1','work_directory':str(output),'viewport':{'width':1600,'height':900},'language':'de','frames':frames,'catalog':catalog})


probes=[]
for i,state in enumerate([root,energy,opened]):
    probes.append({'output':str(output/f'probe-{i}.png'),'hierarchy':state,'media':[],'cursor':{'point':None},'transition_elapsed':2000,'canonical_overview':i==0})
metadata=packet(probes)['metadata']
center=lambda r:[r['x']+r['width']/2,r['y']+r['height']/2]
root_layer=next(l for l in metadata[0]['layers'] if l['interactive'])
video_layer=next(l for l in metadata[1]['layers'] if l['interactive'])
topic_card=next(c for c in root_layer['cards'] if c['id']=='energy')
video_card=next(c for c in video_layer['cards'] if c['id']=='physical_fundamentals')
topic_chip=center(topic_card['chip_rect']);video_chip=center(video_card['chip_rect'])
fullscreen=center(next(w for w in metadata[2]['windows'] if w['id']=='physical_fundamentals')['control_rect'])
parent=center(metadata[1]['chrome']['range']);park=[1552,844]
frames=[];events=[]


def move(state,start,end,seconds,*,pressed=False,canonical=False,event=None):
    """🖱️ Author pointer events against measured component hit regions."""
    if event:events.append({'frame':len(frames),'event':event})
    count=max(1,round(seconds*fps))
    for i in range(count):
        t=i/max(1,count-1);u=t*t*(3-2*t)
        point=[start[a]*(1-u)+end[a]*u for a in (0,1)]
        current={**state,'camera':{'x':point[0]/1600,'y':max(0,min(1,(point[1]-28.8)/842.4))}}
        frames.append({'output':str(output/'f'/f'{len(frames):04d}.png'),'hierarchy':current,'media':[],
                       'cursor':{'point':point,'kind':'arrow','pressed':pressed},'transition_elapsed':2000 if canonical else 1000/fps,'canonical_overview':canonical})


move(root,park,park,.5,canonical=True,event='topics')
move(root,park,[180,100],.9,event='root parallax left')
move(root,[180,100],[1350,770],1.1,event='root parallax right')
card_center=center(topic_card['rect'])
move(root,[1350,770],card_center,.7,event='card body hover does not reveal')
move(root,card_center,card_center,.4)
move(root,card_center,topic_chip,.5)
revealed={**root,'revealed_topic':'energy'}
move(revealed,topic_chip,topic_chip,.75,event='Ansehen reveals topic at full viewport')
move(revealed,topic_chip,topic_chip,.25,pressed=True,event='press topic')
move(energy,topic_chip,topic_chip,.25,event='release opens topic')
move(energy,topic_chip,[1380,180],.9,event='video level parallax right')
move(energy,[1380,180],[250,740],1,event='video level parallax left')
move(energy,[250,740],video_chip,.65)
video_reveal={**energy,'revealed_video':'physical_fundamentals'}
move(video_reveal,video_chip,video_chip,.8,event='Ansehen reveals video at full viewport')
move(video_reveal,video_chip,video_chip,.25,pressed=True,event='press video')
move(opened,video_chip,video_chip,.25,event='release opens video')
move(opened,video_chip,park,.6)
move(opened,park,park,.6,event='opened video')
move(opened,park,fullscreen,.7,event='cursor targets real fullscreen button')
move(opened,fullscreen,fullscreen,.25,pressed=True,event='press restore')
move(energy,fullscreen,fullscreen,.25,event='release restores video level')
move(energy,fullscreen,parent,.65,event='cursor targets parent level control')
move(energy,parent,parent,.25,pressed=True,event='press parent level')
move(root,parent,parent,.25,event='release restores topics')
move(root,parent,park,.65)
move(root,park,park,.5,canonical=True,event='canonical topic handoff')
result=json.loads((output/'metadata.json').read_text(encoding='utf8')) if '--reuse-frames' in sys.argv else packet(frames)
(output/'metadata.json').write_text(json.dumps(result),encoding='utf8')
runtime.run_command([*runtime.ffmpeg_command(),'-hide_banner','-loglevel','error','-y','-framerate',str(fps),'-i',str(output/'f'/'%04d.png'),'-an','-c:v','libx264','-preset','fast','-crf','16','-pix_fmt','yuv420p','-movflags','+faststart',str(output/'layered-navigation.mp4')],log=output/f'encode-{time.time_ns()}.log')
shutil.copy2(output/'layered-navigation.mp4',Path(__file__).with_name('layered-navigation.mp4'))
for name,event in [('topics','topics'),('videos','video level parallax right'),('preview','Ansehen reveals video at full viewport')]:
    at=next(e['frame'] for e in events if e['event']==event)
    if name=='preview':at=round(9.17*fps)
    shutil.copy2(output/'f'/f'{at:04d}.png',Path(__file__).with_name('layered-'+name+'.png'))
Path(__file__).with_name('layered-navigation-events.md').write_text('# Layered Navigation Events\n\n'+'\n'.join(f"- {e['frame']/fps:.3f}s: {e['event']}" for e in events)+'\n',encoding='utf8')
print('[DEBUG] Layered navigation review film retained',flush=True)
