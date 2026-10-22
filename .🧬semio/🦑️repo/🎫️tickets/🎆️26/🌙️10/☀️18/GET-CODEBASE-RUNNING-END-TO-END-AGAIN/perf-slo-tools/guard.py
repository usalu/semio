#!/usr/bin/env python3
"""Waits (at most --max seconds) until the one-minute load average of the host is below --below, then exits 0; exits 3 on timeout."""
import argparse,os,time
ap=argparse.ArgumentParser();ap.add_argument("--below",type=float,default=30);ap.add_argument("--max",type=int,default=600);a=ap.parse_args()
end=time.time()+a.max
while True:
    load=os.getloadavg()[0]
    if load<a.below: print(f"load {load:.1f} < {a.below}");raise SystemExit(0)
    if time.time()>end: print(f"load {load:.1f} still >= {a.below}");raise SystemExit(3)
    time.sleep(10)
