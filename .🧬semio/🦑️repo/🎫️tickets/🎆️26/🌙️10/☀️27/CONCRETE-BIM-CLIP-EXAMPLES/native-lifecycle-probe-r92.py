"""🎥 Observe one native lifecycle in a fresh process with bounded memory/time and immediate failures."""
import os,sys,json,time,threading,_thread,faulthandler,traceback,importlib,unittest,ctypes,platform,hashlib
from pathlib import Path
ROOT=Path.cwd();sys.path.insert(0,str(ROOT));TOPIC,LANGUAGE,DIRECTORY=sys.argv[1:4];OUT=Path(DIRECTORY);OUT.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_OUTPUT_ROOT=str(OUT),SEMIO_TEST_MANIM='1',SEMIO_TEST_BIM_TOPICS=TOPIC,SEMIO_TEACHING_LANGUAGE=LANGUAGE,PYTHONUTF8='1')
SECONDS=int(os.environ.get('SEMIO_NATIVE_DIAGNOSTIC_SECONDS','900'));START=time.monotonic();STOP=threading.Event();STATE={'phase':'starting','reason':None};MAIN_THREAD=threading.get_ident()
def memory():
    """🌡️ Read this process only through native OS counters behind one portable diagnostic interface."""
    if os.name=='nt':
        from ctypes import wintypes
        class Counters(ctypes.Structure):
            _fields_=[('cb',wintypes.DWORD),('faults',wintypes.DWORD)]+[(name,ctypes.c_size_t) for name in ('peak','rss','peak_paged','paged','peak_nonpaged','nonpaged','pagefile','peak_pagefile','private')]
        counters=Counters();counters.cb=ctypes.sizeof(counters);kernel=ctypes.WinDLL('kernel32',use_last_error=True);kernel.GetCurrentProcess.restype=wintypes.HANDLE;psapi=ctypes.WinDLL('psapi',use_last_error=True);psapi.GetProcessMemoryInfo.argtypes=[wintypes.HANDLE,ctypes.c_void_p,wintypes.DWORD]
        if not psapi.GetProcessMemoryInfo(kernel.GetCurrentProcess(),ctypes.byref(counters),counters.cb):raise OSError(ctypes.get_last_error())
        return {'rss_bytes':counters.rss,'peak_rss_bytes':counters.peak,'private_bytes':counters.private}
    import resource
    peak=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*(1 if sys.platform=='darwin' else 1024)
    current=int(Path('/proc/self/statm').read_text().split()[1])*os.sysconf('SC_PAGE_SIZE') if Path('/proc/self/statm').exists() else peak
    return {'rss_bytes':current,'peak_rss_bytes':peak}
def event(kind,**values):
    """🧾 Flush actual phase, OS counters and diagnostic outcomes before another expensive body."""
    row={'kind':kind,'pid':os.getpid(),'time':time.time(),'elapsed':time.monotonic()-START,**memory(),**values}
    with (OUT/'events.jsonl').open('a',encoding='utf8') as stream:stream.write(json.dumps(row,ensure_ascii=True)+'\n');stream.flush()
    if kind!='sample':print('[DEBUG] '+json.dumps(row,ensure_ascii=True),flush=True)
    return row
def snapshot(reason):
    """🧵 Capture owned Python frame references while holding the interpreter lock, without a native watchdog."""
    frame=sys._current_frames().get(MAIN_THREAD)
    if frame is not None:
        with (OUT/'stack-samples.log').open('a',encoding='utf8') as stream:
            stream.write('[DEBUG] '+reason+' '+str(time.monotonic()-START)+'\n');stream.writelines(traceback.format_stack(frame));stream.flush()

def observe():
    """⏳ Keep real memory/progress visible and stop only this diagnostic on its declared bounds."""
    sampled=START
    while not STOP.wait(1):
        row=event('sample',phase=STATE['phase'])
        if time.monotonic()-sampled>=60:snapshot('periodic Python snapshot');sampled=time.monotonic()
        if (OUT/'cancel').exists():STATE['reason']='cooperative cancellation'
        elif row['elapsed']>SECONDS:STATE['reason']=str(SECONDS)+'second diagnostic bound'
        elif row['rss_bytes']>5905580032:STATE['reason']='5.5GiB own-process diagnostic bound'
        if STATE['reason']:
            event('bound',reason=STATE['reason']);snapshot('own bound');_thread.interrupt_main();return
STACK=(OUT/'stack.log').open('w',encoding='utf8');faulthandler.enable(file=STACK,all_threads=True)
(OUT/'started.json').write_text(json.dumps({'pid':os.getpid(),'topic':TOPIC,'language':LANGUAGE,'time':time.time(),'python':sys.version,'platform':platform.platform(),'bounds':{'seconds':SECONDS,'rss_bytes':5905580032}}),encoding='utf8')
WATCHER=threading.Thread(target=observe,daemon=True);WATCHER.start();event('start')
def provenance(stage):
    """🪪️ Bind loaded code, native binaries and canonical source inputs to one actual diagnostic run."""
    import marshal,types,importlib.metadata
    files={Path(__file__),Path(__file__).with_name('native-lifecycle-supervisor-r92.mjs'),ROOT/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/uv.lock'}
    clip=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip'
    files.update((clip/'🧱️model').rglob('*.py'));files.update((clip/'🧱️model').rglob('*.json'))
    visuals=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️');files.update(visuals.source_files())
    loaded={};extensions={}
    for name,module in tuple(sys.modules.items()):
        file=getattr(module,'__file__',None)
        if not file:continue
        path=Path(file)
        if path.suffix.lower() in ('.pyd','.so') and path.is_file():extensions[name]={'file':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
        if not name.startswith('🎓️teaching.') or not path.is_file():continue
        files.add(path);functions={}
        def code(key,value):
            if isinstance(value,(staticmethod,classmethod)):value=value.__func__
            if isinstance(value,types.FunctionType):functions[key]=hashlib.sha256(marshal.dumps(value.__code__)).hexdigest()
            if isinstance(value,property):
                for suffix,method in (('get',value.fget),('set',value.fset)):
                    if method is not None:code(key+'.'+suffix,method)
        for key,value in vars(module).items():
            code(key,value)
            if isinstance(value,type) and value.__module__==name:
                for member,method in vars(value).items():code(key+'.'+member,method)
        loaded[name]=functions
    hashes={str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(files) if path.is_file() and path.is_relative_to(ROOT)}
    versions={}
    for name in ('manim','numpy','manimpango','pycairo','shapely','scipy'):
        try:versions[name]=importlib.metadata.version(name)
        except importlib.metadata.PackageNotFoundError:pass
    report={'stage':stage,'python':sys.version,'executable':sys.executable,'versions':versions,'source_files':hashes,'loaded_code':loaded,'loaded_native_extensions':extensions}
    (OUT/('provenance-'+stage+'.json')).write_text(json.dumps(report,ensure_ascii=True,indent=2),encoding='utf8')
    return report

class Result(unittest.TextTestResult):
    """🚦 Retain complete subtest exceptions immediately rather than only at cohort shutdown."""
    def issue(self,test,error,kind):
        detail=self._exc_info_to_string(error,test);event(kind,test=test.id(),traceback=detail)
    def addError(self,test,error):self.issue(test,error,'error');super().addError(test,error)
    def addFailure(self,test,error):self.issue(test,error,'failure');super().addFailure(test,error)
    def addSubTest(self,test,subtest,error):
        if error is not None:self.issue(subtest,error,'subtest')
        super().addSubTest(test,subtest,error)
try:
    module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🎭️presentation.🐍️')
    chapter=next(c for c in module.CASES['chapters'] if c['topic']==TOPIC);base=getattr(importlib.import_module('🎓️teaching.'+chapter['script'].removesuffix('.py').replace('/','.')),chapter['scene']);original=base.step;BEFORE=provenance('before')
    def step(self,key,*args,**kwargs):
        """🎬 Observe actual authored clauses without changing their native execution."""
        STATE['phase']=key;event('beat-start',key=key)
        try:return original(self,key,*args,**kwargs)
        finally:event('beat-end',key=key)
    from unittest.mock import patch
    from contextlib import ExitStack
    def factory(name,original):
        def run(self,*args,**kwargs):
            before=STATE['phase'];STATE['phase']='factory:'+name;event('factory-start',name=name)
            try:return original(self,*args,**kwargs)
            finally:event('factory-end',name=name);STATE['phase']=before
        return run
    with ExitStack() as hooks:
        for name in ('_facts','_schema','_axon','_overlay','_chip'):
            if hasattr(base,name):hooks.enter_context(patch.object(base,name,factory(name,getattr(base,name))))
        hooks.enter_context(patch.dict(module.CASES,{'languages':[LANGUAGE]}));hooks.enter_context(patch.object(base,'step',step))
        result=unittest.TextTestRunner(verbosity=2,resultclass=Result).run(unittest.TestSuite([module.BimPresentationContract('test_native_lifecycle')]))
    AFTER=provenance('after');changed=sorted(key for key in set(BEFORE['source_files'])|set(AFTER['source_files']) if BEFORE['source_files'].get(key)!=AFTER['source_files'].get(key));(OUT/'provenance-comparison.json').write_text(json.dumps({'changed_files':changed,'stable_source_files':not changed,'loaded_code_before':BEFORE['loaded_code'],'loaded_code_after':AFTER['loaded_code']},ensure_ascii=True,indent=2),encoding='utf8')
    report={'diagnostic':True,'topic':TOPIC,'language':LANGUAGE,'tests':result.testsRun,'failure_count':len(result.failures),'error_count':len(result.errors),'success':result.wasSuccessful(),'reason':STATE['reason']};event('result',**report);(OUT/'result.json').write_text(json.dumps(report),encoding='utf8');exit_code=0 if report['success'] else 1
except BaseException:
    event('exception',traceback=traceback.format_exc(),reason=STATE['reason']);exit_code=1
finally:
    STOP.set();WATCHER.join(timeout=2)
    if 'BEFORE' in globals() and not (OUT/'provenance-after.json').exists():
        try:
            AFTER=provenance('after')
            changed=sorted(key for key in set(BEFORE['source_files'])|set(AFTER['source_files']) if BEFORE['source_files'].get(key)!=AFTER['source_files'].get(key))
            (OUT/'provenance-comparison.json').write_text(json.dumps({'changed_files':changed,'stable_source_files':not changed,'loaded_code_before':BEFORE['loaded_code'],'loaded_code_after':AFTER['loaded_code']},ensure_ascii=True,indent=2),encoding='utf8')
        except BaseException:event('provenance-error',traceback=traceback.format_exc())

raise SystemExit(exit_code)
