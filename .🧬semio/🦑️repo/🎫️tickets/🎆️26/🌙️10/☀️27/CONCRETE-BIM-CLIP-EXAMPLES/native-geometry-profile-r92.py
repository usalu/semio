"""⏱️ Bound a real current whole-model profile and preserve partial native stacks without publishing incomplete geometry."""
import _thread,cProfile,faulthandler,hashlib,importlib,io,json,pstats,sys,threading,time,traceback
from pathlib import Path
ROOT=Path.cwd();sys.path.insert(0,str(ROOT));OUT=Path(__file__).parent/'🗑️generated/native-r92'
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
paths=[Path(api.__file__),Path(api.__file__).with_name('🔣️.json'),Path(api._surface_finishes().__file__)]
def hashes():
    """🔐 Bind exact current first-party producer and source inputs."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes();m=api.load_model();profiler=cProfile.Profile();start=time.monotonic();done=threading.Event();bounded=False
log=(OUT/'geometry-profile-stacks.txt').open('w',encoding='utf8');log.write('[DEBUG] Current native geometry stack samples;20second interval\n');log.flush()
faulthandler.enable(log)
def deadline():
    """🛑 End only this explicitly bounded profile, leaving any memo value unpublished."""
    while not done.wait(min(20,max(0,90-(time.monotonic()-start)))):
        if time.monotonic()-start>=90:_thread.interrupt_main();return
        frame=sys._current_frames().get(threading.main_thread().ident)
        if frame is not None:traceback.print_stack(frame,file=log);log.flush()
observer=threading.Thread(target=deadline,daemon=True);observer.start()
try:
    print('[DEBUG] current native profile90s',flush=True);profiler.enable();g=api.building_geometry(m)
except KeyboardInterrupt:bounded=True
finally:
    profiler.disable();done.set();observer.join();log.close();profiler.dump_stats(str(OUT/'geometry-profile.pstats'))
    stream=io.StringIO();pstats.Stats(profiler,stream=stream).sort_stats('cumulative').print_stats(45)
    (OUT/'geometry-profile.txt').write_text('\n'.join('[DEBUG] '+line for line in stream.getvalue().splitlines()),encoding='utf8')
    after=hashes();report={'seconds':time.monotonic()-start,'bounded_profile':bounded,'stable':before==after,'before':before,'after':after,'memo_info':api._geometry_models_cached.cache_info()._asdict()}
    (OUT/'geometry-profile.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(report),flush=True)
