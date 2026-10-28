import sys,subprocess,time,os,signal,threading
# usage: stamp.py [--sigint-after S] -- cmd...
args=sys.argv[1:]; sig=None
if args[0]=="--sigint-after": sig=float(args[1]); args=args[2:]
if args[0]=="--": args=args[1:]
t0=time.time()
p=subprocess.Popen(args,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,start_new_session=True)
def killer():
    time.sleep(sig)
    sys.stdout.write(f"[DEBUG] {time.time()-t0:7.2f}s >>> SIGINT to group\n");sys.stdout.flush()
    os.killpg(p.pid,signal.SIGINT)
    global tsig; tsig=time.time()
tsig=None
if sig is not None: threading.Thread(target=killer,daemon=True).start()
for ln in iter(p.stdout.readline,b""):
    print(f"[DEBUG] {time.time()-t0:7.2f}s {ln.decode(errors='replace').rstrip()[:240]}",flush=True)
rc=p.wait()
print(f"[DEBUG] {time.time()-t0:7.2f}s exit {rc}"+(f" ({time.time()-tsig:.2f}s after SIGINT)" if tsig else ""))
