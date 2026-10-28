"""📌️ capgrow proof holder: really holds <MB> of incompressible resident memory (a `sleep` holds nothing and stops counting after the 60 s ramp) until <release-file> exists or <seconds> passed: python3 -I pin.py <MB> <seconds> [release-file]."""
import os,sys,time
megabytes,seconds=int(sys.argv[1]),float(sys.argv[2]);release=sys.argv[3] if len(sys.argv)>3 else None;block=os.urandom(64<<20)
held=bytearray(block)*max(1,megabytes>>6);end=time.monotonic()+seconds
print(f"[DEBUG] pinned {len(held)>>20} MB",flush=True)
while time.monotonic()<end and not (release and os.path.exists(release)):time.sleep(0.2)
print(f"[DEBUG] released {len(held)>>20} MB at {time.strftime('%H:%M:%S')}",flush=True)
