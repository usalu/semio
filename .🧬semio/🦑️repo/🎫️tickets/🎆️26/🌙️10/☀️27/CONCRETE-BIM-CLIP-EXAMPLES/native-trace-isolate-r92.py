"""🔬 Compare identical bounded standard-library work with and without scheduled stack dumping."""
import sys,time,json,faulthandler,threading,traceback
from fractions import Fraction
from pathlib import Path
MODE,DIRECTORY=sys.argv[1:3];OUT=Path(DIRECTORY);OUT.mkdir(parents=True,exist_ok=True);start=time.monotonic();fatal=(OUT/'fatal.log').open('w');trace=(OUT/'timeout.log').open('w');faulthandler.enable(file=fatal,all_threads=True)
stop=threading.Event();worker=None;main=threading.get_ident()
if MODE=='timer':faulthandler.dump_traceback_later(.02,repeat=True,file=trace)
if MODE=='safe-thread':
    def capture():
        while not stop.wait(.02):
            frame=sys._current_frames().get(main)
            if frame is not None:trace.write(''.join(traceback.format_stack(frame)));trace.flush()
    worker=threading.Thread(target=capture);worker.start()
row={'mode':MODE,'python':sys.version,'start':time.time()};(OUT/'started.json').write_text(json.dumps(row),encoding='utf8');print('[DEBUG] trace isolate '+json.dumps(row),flush=True)
def inner(index):
    """🧮 Exercise normal Python frame entry and Fraction arithmetic without native dependencies."""
    a,b=Fraction(index%103,107),Fraction(index%109,113)
    return (a-b)*(a+b)/(a+1)
def loop():
    """⏳ Run a finite identical workload and cooperative own cancellation bound."""
    total=Fraction(0)
    for i in range(1000000):
        total=inner(i)
        if i%10000==0:
            if (OUT/'cancel').exists():raise InterruptedError('Own cooperative cancellation')
            if time.monotonic()-start>20:break
            if i%100000==0:print('[DEBUG] iterations '+str(i),flush=True)
    return i,str(total)
try:
    count,value=loop();report={'mode':MODE,'count':count,'value':value,'seconds':time.monotonic()-start,'success':True};(OUT/'result.json').write_text(json.dumps(report),encoding='utf8');print('[DEBUG] '+json.dumps(report),flush=True)
finally:
    stop.set()
    if worker is not None:worker.join()
    faulthandler.cancel_dump_traceback_later();faulthandler.disable();fatal.close();trace.close()
