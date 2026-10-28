#!/usr/bin/env python3
"""tsc-run.py: one measured tsc invocation. usage: tsc-run.py <label> <cwd> <out.json> -- <tsc args...>
Records wall, user+sys CPU, peak RSS (RUSAGE_CHILDREN), exit code and the sorted diagnostic lines (stdout is streamed to a file, never held)."""
import json,os,resource,subprocess,sys,time
label,cwd,out=sys.argv[1:4]
args=sys.argv[sys.argv.index('--')+1:]
tsc=os.environ.get('TSC','/Users/ueli/Documents/semio/node_modules/typescript/bin/tsc')
node=os.environ.get('NODE','node')
extra=os.environ.get('NODE_OPTS','').split()
log=out+'.log'
t=time.time();before=resource.getrusage(resource.RUSAGE_CHILDREN)
with open(log,'w') as f:
    rc=subprocess.run(['nice','-n','10',node,*extra,tsc,*args],cwd=cwd,stdout=f,stderr=subprocess.STDOUT).returncode
wall=time.time()-t;after=resource.getrusage(resource.RUSAGE_CHILDREN)
diag=[]
with open(log) as f:
    for line in f:
        if ': error TS' in line or line.startswith('error TS'): diag.append(line.rstrip())
row={'label':label,'rc':rc,'wallS':round(wall,2),'cpuS':round(after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,2),'peakRssMb':round(after.ru_maxrss/1048576,0),'errors':len(diag),'diag':sorted(diag)}
json.dump(row,open(out,'w'),ensure_ascii=False)
print(json.dumps({k:v for k,v in row.items() if k!='diag'},ensure_ascii=False))
