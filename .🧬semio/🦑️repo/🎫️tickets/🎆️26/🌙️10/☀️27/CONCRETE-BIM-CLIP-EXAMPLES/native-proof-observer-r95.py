"""🌡️ Observe only the current native proof with portable counters and cooperative diagnostic bounds."""
import ctypes,json,os,sys,time,threading,_thread,traceback
from pathlib import Path


def memory():
    """📏 Read this process through native platform counters without inspecting or controlling peers."""
    if os.name=='nt':
        from ctypes import wintypes
        class Counters(ctypes.Structure):
            _fields_=[('cb',wintypes.DWORD),('faults',wintypes.DWORD)]+[(name,ctypes.c_size_t) for name in ('peak','rss','peak_paged','paged','peak_nonpaged','nonpaged','pagefile','peak_pagefile','private')]
        counters=Counters();counters.cb=ctypes.sizeof(counters);kernel=ctypes.WinDLL('kernel32',use_last_error=True);kernel.GetCurrentProcess.restype=wintypes.HANDLE;psapi=ctypes.WinDLL('psapi',use_last_error=True);psapi.GetProcessMemoryInfo.argtypes=[wintypes.HANDLE,ctypes.c_void_p,wintypes.DWORD]
        if not psapi.GetProcessMemoryInfo(kernel.GetCurrentProcess(),ctypes.byref(counters),counters.cb):raise OSError(ctypes.get_last_error())
        return {'rss_bytes':counters.rss,'peak_rss_bytes':counters.peak}
    import resource
    peak=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*(1 if sys.platform=='darwin' else 1024);stat=Path('/proc/self/statm');current=int(stat.read_text().split()[1])*os.sysconf('SC_PAGE_SIZE') if stat.exists() else peak
    return {'rss_bytes':current,'peak_rss_bytes':peak}


class Observer:
    """🛑 Retain phase, own memory and source-cardinality time bounds without hard process termination."""
    def __init__(self,directory,products):
        self.out=Path(directory);self.started=time.monotonic();self.seconds=512+8*products;self.phase='starting';self.reason=None;self.stop=threading.Event();self.main=threading.get_ident();self.thread=threading.Thread(target=self.observe,daemon=True)

    def event(self,kind,**values):
        """🧾 Flush actual owned-process state into the ticket's generated observation receipt."""
        row={'kind':kind,'pid':os.getpid(),'elapsed':time.monotonic()-self.started,'phase':self.phase,**memory(),**values}
        with (self.out/'observation.jsonl').open('a',encoding='utf8') as stream:stream.write(json.dumps(row)+'\n')
        return row

    def progress(self,value):
        """⏳ Retain the current physical phase while keeping frequent callbacks inexpensive."""
        self.phase=value

    def cancelled(self):
        """🚦 Allow native producers and independent integration to stop cooperatively."""
        return self.reason is not None or (self.out/'cancel').exists()

    def observe(self):
        """🔭 Sample one admitted proof and interrupt its main Python thread only at declared bounds."""
        sampled=self.started
        while not self.stop.wait(1):
            row=self.event('sample')
            if self.cancelled():self.reason=self.reason or 'cooperative cancellation'
            elif row['elapsed']>self.seconds:self.reason=str(self.seconds)+'second source-cardinality diagnostic bound'
            elif row['rss_bytes']>5905580032:self.reason='5.5GiB own-process diagnostic bound'
            if self.reason or time.monotonic()-sampled>=60:
                frame=sys._current_frames().get(self.main)
                if frame is not None:
                    with (self.out/'observation-stack.log').open('a',encoding='utf8') as stream:stream.write('[DEBUG] '+str(self.reason or 'periodic snapshot')+'\n');stream.writelines(traceback.format_stack(frame))
                sampled=time.monotonic()
            if self.reason:self.event('bound',reason=self.reason);_thread.interrupt_main();return

    def __enter__(self):
        self.event('start',bounds={'seconds':self.seconds,'rss_bytes':5905580032});self.thread.start();return self

    def __exit__(self,*error):
        self.stop.set();self.thread.join(timeout=2);self.event('end',reason=self.reason,error=None if error[0] is None else error[0].__name__)
