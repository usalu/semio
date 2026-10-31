"""🔎 Run an explicitly nonfinal whole-model diagnostic while coordinated repairs continue."""
import _thread,hashlib,importlib.util,json,sys,threading,time,traceback
from pathlib import Path
sys.dont_write_bytecode=True
ticket=Path(__file__).parent;out=ticket/'🗑️generated/whole-diagnostic-r95';out.mkdir(parents=True,exist_ok=True);path=ticket/'whole-expanded-interference-r92.py';spec=importlib.util.spec_from_file_location('savoye_diagnostic_r95',path);audit=importlib.util.module_from_spec(spec);spec.loader.exec_module(audit);audit.G=out;stopped=threading.Event()
def observe_cancel():
    """🛑 Interrupt only this owned diagnostic through its explicit cancellation file."""
    while not stopped.wait(.5):
        if(ticket/'whole-diagnostic-cancel-r95').exists():_thread.interrupt_main();return
threading.Thread(target=observe_cancel,daemon=True).start();start=time.monotonic();record={'purpose':'Diagnostic only, never a final frozen-model certificate; all positives and unresolved cases retained.','completed':False,'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
try:
    record['controls']=audit.controls();data=audit.capture();result=audit.audit(data,diagnostic=True);record.update(completed=True,status=result['status'],source=result['source'],physical_products=result['physical_products'],positive_groups=result['positive_groups'],unresolved_pairs=len(result['unresolved_part_pairs']),unsupported_parts=len(result['unsupported_parts']))
except BaseException:record['error']=traceback.format_exc();raise
finally:
    stopped.set();record['seconds']=time.monotonic()-start;(out/'diagnostic-run-r95.json').write_text(json.dumps(record,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in record.items()if k not in('controls','positive_groups','error')}),flush=True)
