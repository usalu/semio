import ctypes,ctypes.util,subprocess,sys,time
lib=ctypes.CDLL(ctypes.util.find_library("proc"))
class RU(ctypes.Structure):
    _fields_=[("uuid",ctypes.c_uint8*16)]+[(n,ctypes.c_uint64) for n in ["ri_user_time","ri_system_time","ri_pkg_idle_wkups","ri_interrupt_wkups","ri_pageins","ri_wired_size","ri_resident_size","ri_phys_footprint","ri_proc_start_abstime","ri_proc_exit_abstime","ri_child_user_time","ri_child_system_time","ri_child_pkg_idle_wkups","ri_child_interrupt_wkups","ri_child_pageins","ri_child_elapsed_abstime","ri_diskio_bytesread","ri_diskio_byteswritten"]]
def written():
    out={}
    for l in subprocess.run(["ps","-axo","pid=,command="],capture_output=True,text=True).stdout.splitlines():
        p,_,c=l.strip().partition(" ")
        r=RU()
        if lib.proc_pid_rusage(int(p),2,ctypes.byref(r))==0: out[int(p)]=(r.ri_diskio_byteswritten,r.ri_phys_footprint,c)
    return out
secs=float(sys.argv[1]);a=written();time.sleep(secs);b=written()
rows=sorted(((b[p][0]-a[p][0],p) for p in b if p in a),reverse=True)[:12]
for d,p in rows:
    print("%8.1f KB/s  pid %6d  fp %6.0f MB  total written %8.0f MB  %s"%(d/1024/secs,p,b[p][1]/1048576,b[p][0]/1048576,b[p][2][:110]))
