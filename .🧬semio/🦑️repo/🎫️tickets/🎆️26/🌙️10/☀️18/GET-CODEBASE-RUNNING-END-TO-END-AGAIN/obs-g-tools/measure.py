#!/usr/bin/env python3
"""[obs-g] wall time, CPU time and peak RSS of light door commands, N runs each (median and worst), through the real front door of a workspace tree. usage: measure.py <workspace dir> <runs> -- <repo args...> [-- <repo args...>]"""
import os,resource,statistics,subprocess,sys,time
W=sys.argv[1];N=int(sys.argv[2]);rest=sys.argv[4:];cmds=[[]]
for a in rest:
    if a=="--": cmds.append([])
    else: cmds[-1].append(a)
SCRIPT=os.path.join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts")
env=dict(os.environ,SEMIO_REPO_IMPLEMENTATION="live",SEMIO_LANGUAGE="en")
for args in cmds:
    walls=[];cpus=[];rss=[]
    for _ in range(N):
        before=resource.getrusage(resource.RUSAGE_CHILDREN);t=time.time()
        subprocess.run(["bun",SCRIPT,"repo"]+args,cwd=W,env=env,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        walls.append(time.time()-t);after=resource.getrusage(resource.RUSAGE_CHILDREN)
        cpus.append((after.ru_utime+after.ru_stime)-(before.ru_utime+before.ru_stime));rss.append(after.ru_maxrss/1e6)
    print("repo %-22s wall median %.2f s worst %.2f s | cpu median %.2f s | peak rss %.0f MB"%(" ".join(args),statistics.median(walls),max(walls),statistics.median(cpus),max(rss)))
