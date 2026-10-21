#!/usr/bin/env python3
"""Jobserver token pump (workaround for the scheduler hand-back deadlock): when every scheduler of OUR cargo run waits to reacquire a
token that cargo's helper thread already consumed (fifo empty, no rustc running below our launcher), add one token.
Only fifos opened by descendants of our run.sh are touched. usage: pump.py <run.sh pid>"""
import os, sys, time, subprocess, fcntl, termios, struct, re
root = int(sys.argv[1])
def ps():
    out = subprocess.run(['ps','-Ao','pid=,ppid=,stat=,etime=,command='],capture_output=True,text=True).stdout.splitlines()
    rows = {}
    for l in out:
        p = l.split(None, 4)
        if len(p) < 5: continue
        rows[int(p[0])] = (int(p[1]), p[2], p[4])
    return rows
def descendants(rows, r):
    kids = {}
    for pid,(pp,_,_) in rows.items(): kids.setdefault(pp, []).append(pid)
    seen, st = set(), [r]
    while st:
        x = st.pop()
        for k in kids.get(x, []):
            if k not in seen: seen.add(k); st.append(k)
    return seen
def fifo_of(pid):
    out = subprocess.run(['lsof','-p',str(pid),'-Fn'],capture_output=True,text=True).stdout
    m = re.search(r'n(/\S*jobserver\.fifo)', out)
    return m.group(1) if m else None
stuck_since = {}
pumped = 0
while True:
    time.sleep(2)
    try: os.kill(root, 0)
    except OSError: print('root gone, pumped', pumped); break
    rows = ps(); d = descendants(rows, root)
    scheds = [p for p in d if 'semio-build-scheduler' in rows[p][2] and ' rustc ' in rows[p][2]]
    compiling = [p for p in d if re.search(r'(^|/)rustc( |$)', rows[p][2]) and 'semio-build-scheduler' not in rows[p][2]]
    if not scheds or compiling: stuck_since.clear(); continue
    fifo = fifo_of(scheds[0])
    if not fifo: continue
    fd = os.open(fifo, os.O_RDWR | os.O_NONBLOCK)
    avail = struct.unpack('i', fcntl.ioctl(fd, termios.FIONREAD, struct.pack('i', 0)))[0]
    if avail == 0:
        t0 = stuck_since.setdefault(fifo, time.time())
        if time.time() - t0 >= 6:
            os.write(fd, b'+'); pumped += 1; stuck_since[fifo] = time.time()
            print(time.strftime('%H:%M:%S'), 'pumped token', pumped, 'schedulers', len(scheds), flush=True)
    else: stuck_since.pop(fifo, None)
    os.close(fd)
