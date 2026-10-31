"""🌡️ Observe only the owning fixture test process with portable progress and cancellation."""
import _thread,ctypes,json,os,sys,threading,time,traceback
from pathlib import Path

def memory():
    """📏 Read current and peak resident bytes without a third-party runtime dependency."""
    if os.name=='nt':
        from ctypes import wintypes
        class Counters(ctypes.Structure):
            _fields_=[('size',wintypes.DWORD),('faults',wintypes.DWORD)]+[(name,ctypes.c_size_t)for name in('peak','rss','peak_paged','paged','peak_nonpaged','nonpaged','pagefile','peak_pagefile','private')]
        value=Counters();value.size=ctypes.sizeof(value);kernel=ctypes.WinDLL('kernel32',use_last_error=True);kernel.GetCurrentProcess.restype=wintypes.HANDLE;psapi=ctypes.WinDLL('psapi',use_last_error=True);psapi.GetProcessMemoryInfo.argtypes=[wintypes.HANDLE,ctypes.c_void_p,wintypes.DWORD];assert psapi.GetProcessMemoryInfo(kernel.GetCurrentProcess(),ctypes.byref(value),value.size);return{'rss_bytes':value.rss,'peak_rss_bytes':value.peak}
    import resource
    peak=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*(1 if sys.platform=='darwin'else 1024);stat=Path('/proc/self/statm');current=int(stat.read_text().split()[1])*os.sysconf('SC_PAGE_SIZE')if stat.exists()else peak;return{'rss_bytes':current,'peak_rss_bytes':peak}

class Observer:
    """🛑 Own the sampler and signal only this process main thread at its declared bound."""
    def __init__(self,ticket,mode):
        self.ticket=Path(ticket);self.mode=mode;self.started=time.monotonic();self.main=threading.main_thread().ident;self.stop=threading.Event();self.thread=threading.Thread(target=self.observe,daemon=True);self.reason=None;memory();self.thread.start()
    def observe(self):
        """🧾 Report real test/geometry frames and bounded local counts without source mutation."""
        while not self.stop.wait(10):
            row={'pid':os.getpid(),'mode':self.mode,'elapsed':time.monotonic()-self.started,**memory()};frame=sys._current_frames().get(self.main);frames=[];stack=traceback.format_stack(frame)if frame is not None else[]
            while frame is not None:
                name=frame.f_code.co_name
                if name.startswith('test_')or any(word in name for word in('reservation','_slab_','_reference_material','_column_partition','structural_joints')):
                    local=frame.f_locals;entry={'function':name,'file':Path(frame.f_code.co_filename).name,'line':frame.f_lineno}
                    for key in('tag','index','i'):
                        if isinstance(local.get(key),(str,int)):entry[key]=local[key]
                    for key in('hosts','common','expected','parts','owners','remaining','cells'):
                        if isinstance(local.get(key),(list,tuple,dict)):entry[key+'_count']=len(local[key])
                    frames.append(entry)
                frame=frame.f_back
            row['frames']=frames
            if(self.ticket/'fixture-fits-cancel-r95').exists():self.reason='Own fixture cancellation requested'
            elif row['rss_bytes']>5905580032:self.reason='Own 5.5 GiB native admission bound'
            elif row['elapsed']>3600:self.reason='Own 3600 second fixture diagnostic bound'
            row['reason']=self.reason;folder=self.ticket/'🗑️generated';(folder/('fixture-progress-'+self.mode+'-r95.json')).write_text(json.dumps(row,ensure_ascii=False,indent=2),encoding='utf8')
            with(folder/('fixture-stack-'+self.mode+'-r95.log')).open('a',encoding='utf8')as stream:stream.write('[DEBUG] '+str(row['elapsed'])+'\n');stream.writelines('[DEBUG] '+line for line in stack)
            print('[DEBUG] fixture progress '+json.dumps(row,ensure_ascii=True),flush=True)
            if self.reason:_thread.interrupt_main();return
    def close(self):
        """🧹 Settle the task-owned watcher without touching any peer thread or process."""
        self.stop.set();self.thread.join(timeout=2)
