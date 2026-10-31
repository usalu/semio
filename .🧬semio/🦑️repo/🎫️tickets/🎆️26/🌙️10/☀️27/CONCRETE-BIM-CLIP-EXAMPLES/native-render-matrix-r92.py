"""🎬 Execute the complete bilingual two-audience native render matrix with provenance."""
import hashlib,importlib,json,os,sys,time,threading,_thread
from pathlib import Path
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'package.json').is_file())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
TICKET=Path(__file__).parent
MODE=sys.argv[1] if len(sys.argv)>1 else 'smoke'
if MODE not in ('smoke','final'):
    raise ValueError('Use explicit smoke or final render mode')
OUT=TICKET/'🗑️generated/native-r92'/MODE
OUT.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker',SEMIO_TEACHING_OUTPUT_ROOT=str(OUT),PYTHONUTF8='1')
import runpy
runpy.run_path(str(TICKET/'native-path-preflight-r92.py'),run_name='native_render_path_preflight')
runtime=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
theme=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️')
IDS=['bim_information','bim_objects','bim_representations','bim_models','bim_exchange','bim_consistency']
CLIP=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip'
VIDEOS=[v for v in runtime.load_manifest(source_ids=IDS) if v['id'] in IDS]
FPS,RESOLUTION,QUALITY=(30.,'1920,1080','-qh') if MODE=='final' else (4.,'480,270','-ql')
CANCEL=TICKET/'native-r92-cancel'
def hashes():
    """🔐 Bind movies to actual chapter producers, source inputs, native font and package declarations."""
    paths={p for p in CLIP.rglob('*') if p.is_file() and p.suffix in ('.py','.json','.md','.mjs','.cjs') and '__pycache__' not in p.parts}
    for chapter in ('📊️information','🧬️objects','👁️representations','🧩️models','🔌️exchange','🔗️consistency'):
        paths.update(runtime.source_dependencies(CLIP/chapter/'🐍️.py'))
    paths.update(theme.source_files())
    package=ROOT/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python'
    paths.update((Path(__file__),TICKET/'native-path-preflight-r92.py',package/'pyproject.toml',package/'uv.lock'))
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths) if p.is_file()}
BEFORE=hashes()
ROWS=[]
START=time.time()
def save(stage,success=False):
    """🧾 Retain current render progress and exact source stability independently of console logs."""
    after=hashes()
    report={'mode':MODE,'diagnostic':MODE!='final','stage':stage,'expected':24,'completed':len(ROWS),'movies':ROWS,'fps':FPS,'resolution':RESOLUTION,'jobs':1,'seconds':time.time()-START,'inputs_before':BEFORE,'inputs_after':after,'stable':BEFORE==after,'success':success and BEFORE==after}
    (OUT/'matrix-progress.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
    return report
stopped=threading.Event()
def observe_cancel():
    """🛑 Forward the ticket cancellation request to the native session, which owns process-tree shutdown."""
    while not stopped.wait(.5):
        if CANCEL.exists():_thread.interrupt_main();return
threading.Thread(target=observe_cancel,daemon=True).start()
try:
    for language in ('en','de'):
        for audience in ('speaker','spectator'):
            if CANCEL.exists():
                raise KeyboardInterrupt('Native matrix cancellation requested')
            assert hashes()==BEFORE,'Source changed between native render batches'
            save(f'rendering {language}/{audience}')
            outputs=runtime.compose_videos(IDS,language=language,audience=audience,output_root=OUT,quality_flag=QUALITY,resolution=RESOLUTION,fps=FPS,jobs=1,play=False,log_output=True)
            assert set(outputs)==set(IDS)
            for video,path in outputs.items():
                state=json.loads(path.with_suffix('.render.json').read_text(encoding='utf-8'))
                assert runtime.state_inputs_current(state),f'Stale native render: {video}'
                ROWS.append({'video':video,'language':language,'audience':audience,'path':str(path),'bytes':path.stat().st_size})
            save(f'complete {language}/{audience}')
    assert len(ROWS)==24
    report=save('complete',True)
except BaseException:
    stopped.set()
    save('stopped')
    raise

stopped.set()
raise SystemExit(0 if report['success'] else 1)
