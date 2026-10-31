"""📏 Validate every planned native render artifact path against the256UTF-16-unit contract."""
import importlib,json,os,sys
from pathlib import Path
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'package.json').is_file())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
TICKET=Path(__file__).parent
BASE=TICKET/'🗑️generated/native-r92'
BASE.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker',SEMIO_TEACHING_OUTPUT_ROOT=str(BASE))
runtime=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
collection=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.📚️collection.🐍️')
IDS=['bim_information','bim_objects','bim_representations','bim_models','bim_exchange','bim_consistency']
videos=runtime.load_manifest(source_ids=IDS)
plan=[p for p in runtime.segment_plan(videos) if p['video_id'] in IDS]
ROWS=[]
def record(kind,path):
    """🪟 Count Windows UTF-16 code units even when this audit runs on another native platform."""
    path=Path(path).resolve()
    text=str(path)
    ROWS.append({'kind':kind,'path':text,'utf16':len(text.encode('utf-16-le'))//2})
    assert path.is_relative_to(TICKET/'🗑️generated'),f'Generated path escapes ticket: {path}'
    assert ROWS[-1]['utf16']<=256,f'Windows path exceeds256UTF-16 units: {path}'
modes=[sys.argv[1]] if len(sys.argv)>1 and sys.argv[1] in ('smoke','final') else ['smoke','final']
for mode in modes:
    root=BASE/mode
    os.environ['SEMIO_TEACHING_OUTPUT_ROOT']=str(root)
    width,height,fps=(1920,1080,30.) if mode=='final' else (480,270,4.)
    for language in ('en','de'):
        for audience in ('speaker','spectator'):
            variant=root/language/audience
            settings={'language':language,'audience':audience,'resolution':f'{width},{height}','fps':fps,'quality':'-qh' if mode=='final' else '-ql'}
            destination=variant/'rendered'
            for index,video in enumerate(videos,1):
                if video['id'] not in IDS:
                    continue
                movie=destination/f"{video['output']}_{audience}_{language}_{width}x{height}p{fps:g}.mp4"
                for suffix in ('.mp4','.render.json','.cursor.json'):
                    record('published-movie/certificate',movie.with_suffix(suffix))
                for chapter in video['chapters']:
                    chaptermovie=collection.chapter_output(destination,chapter['scene'],settings)
                    for suffix in ('.mp4','.render.json','.chapter.json','.cursor.json'):
                        record('published-chapter/certificate',chaptermovie.with_suffix(suffix))
                    for parent in (destination,chaptermovie.parent):
                        work=parent/'p-ffffffff'
                        for extra in ('normalized/o/9999.png','normalized/i/9999.png','i/9999.png','o/9999.png','collection-compose.log','body-ffffffff.log','p-ffffffff.partial.mp4'):
                            record('compositor-work',work/extra)
                    for suffix in ('.mp4','.render.json','.cursor.json'):
                        record('logical-chapter/certificate',(variant/str(index)/'v'/chapter['scene']).with_suffix(suffix))
                for extra in ('content.mp4','content.render.json','concat.txt','settings.json'):
                    record('logical-content',variant/str(index)/extra)
                record('movie-contact-sheet',BASE/'movie-qa'/f"{video['id']}-{language}-{audience}-contact.png")
                record('movie-snapshot',BASE/'movie-qa'/f"{video['id']}-{language}-{audience}-999999.png")
            for job in plan:
                segment=job['segment']
                media=variant/'s'/str(job['source_ordinal'])
                for key,template in runtime._output.MANIM_DIRECTORIES.items():
                    directory=Path(template.format(media_dir=str(media),scene_name=segment['scene']))
                    filename={'video_dir':segment['scene']+'.render.json','partial_movie_dir':'uncached_999999999999.mp4','images_dir':segment['scene']+'_ManimCE_v0.22.0.png','text_dir':'f'*64+'.svg','tex_dir':'f'*64+'.svg','log_dir':'attempt-9999999999999999999.log'}[key]
                    record('native-'+key,directory/filename)
                record('source-attempt-log',media/'attempt-9999999999999999999.log')
                trace=runtime.source_trace_path(segment,variant)
                record('source-trace',trace)
                record('source-cursor',trace.with_name('👆️.json'))
                for ext in ('.mp3','.wav','.json'):
                    record('source-speech-cache',runtime.source_speech_root(runtime.teaching_root(Path(runtime.__file__))/segment['script'],language=language)/('f'*64+ext))
            for parent in (root,variant):
                for ext in ('.png','.json'):
                    record('browser-ui-cache',parent/'ui/h'/('f'*64+ext))
report={'success':True,'limit':256,'modes':modes,'checked':len(ROWS),'maximum_utf16':max(r['utf16'] for r in ROWS),'longest':sorted(ROWS,key=lambda r:r['utf16'],reverse=True)[:12],'paths':ROWS}
(BASE/('path-preflight-'+('-'.join(modes))+'.json')).write_text(json.dumps(report,indent=2),encoding='utf-8')
print('[DEBUG] Native path preflight '+json.dumps({key:report[key] for key in ('success','checked','maximum_utf16')}),flush=True)
