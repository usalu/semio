"""🧪 Run ramp footprint boundary contracts under admitted frozen Python."""
import hashlib,importlib,json,sys,time,traceback,unittest
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if (p/'package.json').is_file() and (p/'AGENTS.md').is_file());sys.path.insert(0,str(root))
def peak_rss_bytes():
    """🧠 Read the native lifetime process RSS peak through an isolated standard-library interface."""
    if sys.platform!='win32':
        import resource
        return int(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)*(1 if sys.platform=='darwin' else 1024)
    import ctypes
    from ctypes import wintypes
    class Counters(ctypes.Structure):
        """📊 Native operating-system process counters, kept private to this input harness."""
        _fields_=[('cb',wintypes.DWORD),('faults',wintypes.DWORD),('sizes',ctypes.c_size_t*8)]
    counters=Counters();counters.cb=ctypes.sizeof(counters)
    kernel=ctypes.WinDLL('kernel32',use_last_error=True);kernel.GetCurrentProcess.restype=ctypes.c_void_p
    native=ctypes.WinDLL('psapi',use_last_error=True);native.GetProcessMemoryInfo.argtypes=[ctypes.c_void_p,ctypes.POINTER(Counters),wintypes.DWORD]
    if not native.GetProcessMemoryInfo(kernel.GetCurrentProcess(),ctypes.byref(counters),ctypes.sizeof(counters)):
        raise ctypes.WinError(ctypes.get_last_error())
    return int(counters.sizes[0])

module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🧩️regions.🧪️tests.🐍️')
paths=[root/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip"/p for p in ("🧱️model/🔣️.json","🧱️model/🐍️.py","🎨️visuals/🏛️savoye/📋️plan/🐍️.py","🎨️visuals/📐️drafting/🧩️regions/🐍️.py")]
before={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
names=sys.argv[2:] or unittest.defaultTestLoader.getTestCaseNames(module.OwnedUnion)
suite=unittest.TestSuite(module.OwnedUnion(name) for name in names);start=time.monotonic();result=unittest.TextTestRunner(verbosity=2).run(suite)
try:peak=peak_rss_bytes();peak_error=None
except Exception as error:peak=None;peak_error=repr(error)
(Path(__file__).parent/'🗑️generated'/('ramp-plan-union-'+sys.argv[1]+'-r93.json')).write_text(json.dumps({'peak_rss_bytes':peak,'peak_rss_scope':'native_process_lifetime','peak_rss_error':peak_error,'before':before,'after':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},'actual':module.RESULTS,'pass':result.wasSuccessful(),'tests':result.testsRun,'failures':[str(x) for x in result.failures+result.errors],'seconds':time.monotonic()-start},indent=2),encoding='utf8')
raise SystemExit(not result.wasSuccessful())
