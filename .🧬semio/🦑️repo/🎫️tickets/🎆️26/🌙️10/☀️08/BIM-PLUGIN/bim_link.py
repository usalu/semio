"""🔗️ Serialize memory-intensive BIM verification links on the shared build host."""
from contextlib import contextmanager
from pathlib import Path
import os,time

@contextmanager
def large_link():
    path=Path(__file__).resolve().parent/'🗑️generated/bim-large-link.lock'
    with path.open('a+b') as stream:
        if os.name=='nt':
            import msvcrt
            if stream.tell()==0:stream.write(b'0');stream.flush()
            while True:
                stream.seek(0)
                try:msvcrt.locking(stream.fileno(),msvcrt.LK_NBLCK,1);break
                except OSError:time.sleep(1)
        else:
            import fcntl
            fcntl.flock(stream.fileno(),fcntl.LOCK_EX)
        yield
