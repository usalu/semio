"""🧪 Exercise a silent owned worker and real grandchild without modifying production input."""
import argparse,json,os,subprocess,sys,time
from pathlib import Path
P=argparse.ArgumentParser();P.add_argument('--mode',choices=['success','tree','leaf'],required=True);P.add_argument('--record',type=Path,required=True);P.add_argument('--leaf-record',type=Path);P.add_argument('--leaf-label');args=P.parse_args()
T=Path(__file__).resolve().parent;R=next(p for p in T.parents if (p/'🧰️framework').is_dir())
args.record.parent.mkdir(parents=True,exist_ok=True)
if args.mode=='success':
    time.sleep(.35);args.record.write_text(json.dumps({'pid':os.getpid(),'mode':args.mode}));print('[DEBUG] owned fixture success',flush=True)
elif args.mode=='leaf':
    args.record.write_text(json.dumps({'pid':os.getpid(),'parent':os.getppid(),'mode':args.mode}));time.sleep(120)
else:
    command=['repo','tools','exec','uv','--','run','--python',sys.executable,'--no-project','python','-B',str(T/'verify.py'),str(Path(__file__).relative_to(R)),'--label',args.leaf_label,'--mode','leaf','--record',str(args.leaf_record)]
    child=subprocess.Popen(command,cwd=R)
    args.record.write_text(json.dumps({'pid':os.getpid(),'grandchild_root':child.pid,'mode':args.mode}))
    child.wait(timeout=120)
