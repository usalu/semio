"""🧠 Run one verification child within a resident-memory budget and stop on declining host headroom."""
from contextlib import contextmanager
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time

def headroom():
    if sys.platform=="darwin":
        report=subprocess.run(["memory_pressure","-Q"],text=True,capture_output=True,check=True).stdout
        return int(re.search(r"percentage: (\d+)%",report)[1])
    if sys.platform.startswith("linux"):
        rows=dict(re.findall(r"^(MemTotal|MemAvailable):\s*(\d+) kB",Path("/proc/meminfo").read_text(),re.M))
        return int(100*int(rows["MemAvailable"])/int(rows["MemTotal"]))
    import ctypes
    class Status(ctypes.Structure):
        _fields_=[("length",ctypes.c_ulong),("load",ctypes.c_ulong)]+[(k,ctypes.c_ulonglong) for k in ("phys","free","page","freePage","virtual","freeVirtual","freeExtended")]
    stat=Status();stat.length=ctypes.sizeof(stat)
    if not ctypes.windll.kernel32.GlobalMemoryStatusEx(ctypes.byref(stat)):raise OSError("GlobalMemoryStatusEx")
    return 100-stat.load

def process_group(command):
    child=subprocess.Popen(command,start_new_session=True)
    def resident():
        rows=subprocess.check_output(["ps","-axo","pgid=,rss="],text=True).splitlines()
        return sum(int(rss) for group,rss in (row.split() for row in rows if row.strip()) if int(group)==child.pid)
    def members():
        rows=subprocess.check_output(["ps","-axo","pid=,pgid=,uid=,stat="],text=True).splitlines()
        return [int(pid) for pid,group,uid,state in (r.split() for r in rows if r.strip())
                if int(group)==child.pid and int(uid)==os.getuid() and not state.startswith("Z")]
    def signal_members(sig):
        try:os.killpg(child.pid,sig)
        except ProcessLookupError:pass
        except PermissionError:
            for pid in members():
                try:os.kill(pid,sig)
                except ProcessLookupError:pass
    def stop():
        if not members():return
        signal_members(signal.SIGTERM)
        deadline=time.monotonic()+.6
        while time.monotonic()<deadline:
            child.poll()
            if not members():return
            time.sleep(.03)
        signal_members(signal.SIGKILL)
    return child,resident,stop


def windows_job(command,limit):
    import ctypes
    from ctypes import wintypes as w
    native=ctypes.WinDLL("kernel32",use_last_error=True)
    size=ctypes.c_size_t
    class Basic(ctypes.Structure):
        _fields_=[("processTime",ctypes.c_int64),("jobTime",ctypes.c_int64),("flags",w.DWORD),("minWorkingSet",size),("maxWorkingSet",size),("processLimit",w.DWORD),("affinity",size),("priority",w.DWORD),("scheduling",w.DWORD)]
    class Extended(ctypes.Structure):
        _fields_=[("basic",Basic),("io",ctypes.c_uint64*6),("processMemory",size),("jobMemory",size),("peakProcess",size),("peakJob",size)]
    def api(name,args,result=w.BOOL):
        fn=getattr(native,name);fn.argtypes=args;fn.restype=result
        return fn
    create=api("CreateJobObjectW",[w.LPVOID,w.LPCWSTR],w.HANDLE)
    configure=api("SetInformationJobObject",[w.HANDLE,ctypes.c_int,w.LPVOID,w.DWORD])
    query=api("QueryInformationJobObject",[w.HANDLE,ctypes.c_int,w.LPVOID,w.DWORD,w.LPVOID])
    attach=api("AssignProcessToJobObject",[w.HANDLE,w.HANDLE])
    close=api("CloseHandle",[w.HANDLE])
    handle=create(None,None)
    if not handle:raise ctypes.WinError(ctypes.get_last_error())
    child=None
    try:
        info=Extended();info.basic.flags=0x2000|0x200;info.jobMemory=limit*1024
        if not configure(handle,9,ctypes.byref(info),ctypes.sizeof(info)):raise ctypes.WinError(ctypes.get_last_error())
        launch="import sys,subprocess;go=sys.stdin.buffer.read(1);sys.exit(subprocess.call(sys.argv[1:]) if go else 125)"
        child=subprocess.Popen([sys.executable,"-c",launch,*command],stdin=subprocess.PIPE)
        if not attach(handle,w.HANDLE(int(child._handle))):raise ctypes.WinError(ctypes.get_last_error())
        child.stdin.write(b"1");child.stdin.close()
    except BaseException:
        if child is not None:child.kill();child.wait()
        close(handle)
        raise
    def peak():
        if not query(handle,9,ctypes.byref(info),ctypes.sizeof(info),None):raise ctypes.WinError(ctypes.get_last_error())
        return info.peakJob//1024
    return child,peak,lambda:close(handle)


def reservation(path,mib,capacity,start_mib=0,existing=False):
    import errno
    units=(mib+63)//64
    if units<=0:raise ValueError("Memory reservation must be positive.")
    if units>(capacity-start_mib)//64:return None
    try:handle=open(path,"r+b" if existing else "a+b")
    except FileNotFoundError:return None
    if os.name=="nt":
        import msvcrt
        def take(start,count):
            handle.seek(start)
            msvcrt.locking(handle.fileno(),msvcrt.LK_NBLCK,count)
    else:
        import fcntl
        def take(start,count):
            fcntl.lockf(handle,fcntl.LOCK_EX|fcntl.LOCK_NB,count,start)
    def available(start,count):
        try:take(start,count);return True
        except OSError as exc:
            if exc.errno not in (errno.EACCES,errno.EAGAIN,errno.EDEADLK):raise
            return False
    try:
        for start in range(start_mib//64,1+capacity//64-units):
            if available(start,units):return handle
        if units>1:
            remaining=units
            for start in range(start_mib//64,capacity//64):
                if available(start,1):remaining-=1
                if not remaining:return handle
    except BaseException:
        handle.close();raise
    handle.close()
    return None


@contextmanager
def admission_order(path):
    directory=path.parent/("queue-"+path.stem.rsplit("-",1)[-1]);directory.mkdir(exist_ok=True)
    now=time.time_ns();marker=directory/f"{now:020d}-{os.getpid()}.wait"
    owner=reservation(marker,64,64)
    if owner is None:raise RuntimeError("Cannot register native memory request")
    def first():
        for earlier in sorted(directory.glob("*.wait")):
            if earlier.name>=marker.name:break
            try:created=int(earlier.name.split("-",1)[0])
            except ValueError:continue
            if time.time_ns()-created>2_000_000_000:
                abandoned=reservation(earlier,64,64,existing=True)
                if abandoned is not None:
                    abandoned.close();earlier.unlink(missing_ok=True);continue
                if not earlier.exists():continue
            return False
        return True
    try:yield first
    finally:
        owner.close();marker.unlink(missing_ok=True)


def parent_wait_alive(result):
    if result==258:return True
    if result==0:return False
    raise OSError("Waiting for the validation scheduler failed.")


@contextmanager
def parent_monitor():
    owner=os.getppid()
    if os.name!="nt":
        yield lambda:os.getppid()==owner
        return
    import ctypes
    from ctypes import wintypes as w
    native=ctypes.WinDLL("kernel32",use_last_error=True)
    native.OpenProcess.argtypes=[w.DWORD,w.BOOL,w.DWORD]
    native.OpenProcess.restype=w.HANDLE
    native.WaitForSingleObject.argtypes=[w.HANDLE,w.DWORD]
    native.WaitForSingleObject.restype=w.DWORD
    native.CloseHandle.argtypes=[w.HANDLE]
    native.CloseHandle.restype=w.BOOL
    handle=native.OpenProcess(0x00100000,False,owner)
    if not handle:
        if ctypes.get_last_error()!=87:raise ctypes.WinError(ctypes.get_last_error())
        yield lambda:False
        return
    try:yield lambda:parent_wait_alive(native.WaitForSingleObject(handle,0))
    finally:native.CloseHandle(handle)


def main(command):
    with parent_monitor() as alive:
        return admitted(command,alive)


def admitted(command,alive):
    if not alive():return 125
    floor=max(30,min(95,int(os.environ.get("EQUIPMENT_HEADROOM_PERCENT","48"))))
    if headroom()<floor+5:sys.exit(f"Host headroom below {floor+5} percent")
    limit=min(1150,int(os.environ.get("EQUIPMENT_RSS_MIB","1150")))*1024
    def cancelled(signum,_frame):raise SystemExit(128+signum)
    signal.signal(signal.SIGTERM,cancelled)
    import hashlib,socket
    host=hashlib.sha256(socket.gethostname().encode()).hexdigest()[:12]
    path=Path(__file__).resolve().parent/"🗑️generated"/("resident-"+host+".lock")
    pool=2816 if os.environ.get("EQUIPMENT_BACKGROUND")=="1" else 3584
    with admission_order(path) as first:
        while True:
            start_mib=0 if first() else 2816
            slot=reservation(path,limit//1024,pool,start_mib)
            if slot is not None:break
            time.sleep(.35)
            if not alive():return 125
            if headroom()<floor:return 75
    try:
        return observe(command,limit,floor,alive)
    finally:slot.close()


def observe(command,limit,floor,alive):
    child,resident,stop=windows_job(command,limit) if os.name=="nt" else process_group(command)
    try:
        while child.poll() is None:
            try:return child.wait(timeout=.35)
            except subprocess.TimeoutExpired:pass
            if not alive():return 125
            used=resident()
            if used>limit or headroom()<floor:
                print("[DEBUG] paused for headroom",child.pid,used,file=sys.stderr)
                return 75
        return child.returncode
    finally:
        stop()
        child.wait()


if __name__=="__main__":sys.exit(main(sys.argv[1:]))
