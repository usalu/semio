"""🧑‍🏭 Run one owned repository capture worker with silent-child checkpoints and bounded tree cleanup."""
import json,os,queue,re,shutil,subprocess,sys,threading,time
from pathlib import Path

class CaptureWorkerError(RuntimeError):
    """🛑 A worker or its ownership-checked cleanup did not complete successfully."""

def source_files():
    """🔗 Bind the worker interface and first-party process adapters for capture provenance."""
    ticket=Path(__file__).resolve().parent
    root=next(path for path in ticket.parents if (path/'🧰️framework').is_dir())
    return (Path(__file__),ticket/'stop-capture-worker.ts',ticket/'capture-process.schema.json',root/'🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts',root/'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts')

def _bounded_path(path):
    """📏 Keep every owned artifact portable within the workspace path-length contract."""
    path=Path(path).resolve()
    if len(str(path).encode('utf-16-le'))//2>256:raise ValueError('Capture artifact exceeds 256 path characters')
    return path

def _repo():
    """🦑 Resolve the managed repository executable from the registered tool environment."""
    value=shutil.which('repo')
    if not value:raise CaptureWorkerError('Managed repo executable is unavailable')
    return value

def _stop_tree(root,ticket,output,child,label,started):
    """🌳 Delegate only this live owned tree to the existing first-party termination interface."""
    if child.poll() is not None:return
    request=_bounded_path(output/(label+'.request.json'))
    record=_bounded_path(output/(label+'.tree.json'))
    request.write_text(json.dumps({'root_pid':child.pid,'parent_pid':os.getpid(),'label':label,'started_at':started,'record':str(record)}),encoding='utf8')
    command=[_repo(),'tools','exec','--light','bun','--',str(ticket/'stop-capture-worker.ts'),'--request',str(request)]
    with _bounded_path(output/(label+'.stop.log')).open('w',encoding='utf8') as log:
        stopper=subprocess.Popen(command,cwd=root,stdout=log,stderr=subprocess.STDOUT)
        try:code=stopper.wait(timeout=30)
        except subprocess.TimeoutExpired:
            stopper.kill()
            try:stopper.wait(timeout=2)
            except subprocess.TimeoutExpired:pass
            raise CaptureWorkerError('Owned-tree termination adapter exceeded its bounded wait')
    if code or not record.is_file():raise CaptureWorkerError('Owned-tree termination adapter failed: '+str(code))
    report=json.loads(record.read_text(encoding='utf8'))
    if not report.get('success') or report.get('remaining'):raise CaptureWorkerError('Owned descendants remain after termination')
    try:child.wait(timeout=5)
    except subprocess.TimeoutExpired:raise CaptureWorkerError('Owned repository root did not exit after tree termination')

def run_worker(root,ticket,output,worker,label,checkpoint,*,arguments=(),interval=5.0):
    """🎬 Run a ticket worker through repo→uv→verify, checking frozen inputs even during silence."""
    root,ticket,output,worker=map(_bounded_path,(root,ticket,output,worker))
    if not ticket.is_relative_to(root) or not worker.is_relative_to(ticket) or worker.suffix!='.py' or not worker.is_file():raise ValueError('Capture worker must be a Python script inside its ticket')
    if not output.is_relative_to(ticket/'🗑️generated'):raise ValueError('Capture outputs must remain inside ticket generated output')
    if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,63}',label):raise ValueError('Capture label must be a portable identifier')
    if not isinstance(interval,(int,float)) or not 0<interval<=60:raise ValueError('Checkpoint interval must be positive and bounded')
    output.mkdir(parents=True,exist_ok=True)
    logfile=_bounded_path(output/(label+'.log'))
    for suffix in ('.request.json','.tree.json','.stop.log'):_bounded_path(output/(label+suffix))
    command=[_repo(),'tools','exec','uv','--','run','--python',sys.executable,'--no-project','python','-B',str(ticket/'verify.py'),str(worker.relative_to(root)),'--label',label,*map(str,arguments)]
    environment={**os.environ,'PYTHONIOENCODING':'utf-8','PYTHONUTF8':'1','PYTHONDONTWRITEBYTECODE':'1'}
    stream=queue.Queue();child=None;pump=None;began=time.monotonic();started=time.time()
    def event(phase):
        """📣 Keep the capture controller's source checkpoint independent of worker output."""
        checkpoint({'label':label,'phase':phase,'pid':None if child is None else child.pid,'elapsed_seconds':time.monotonic()-began})
    def reader():
        """📥 Drain stdout without blocking the controller's checkpoint clock."""
        try:
            for line in child.stdout:stream.put(line)
        finally:stream.put(None)
    with logfile.open('w',encoding='utf8') as log:
        try:
            event('launch')
            options={'creationflags':getattr(subprocess,'CREATE_NO_WINDOW',0)|getattr(subprocess,'CREATE_NEW_PROCESS_GROUP',0)} if os.name=='nt' else {'start_new_session':True}
            child=subprocess.Popen(command,cwd=root,env=environment,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,encoding='utf8',errors='backslashreplace',**options)
            pump=threading.Thread(target=reader,name='capture-'+label,daemon=True);pump.start()
            event('running');next_check=time.monotonic()+interval;eof=False;exited_at=None
            while True:
                now=time.monotonic()
                if now>=next_check:event('running');next_check=time.monotonic()+interval
                try:line=stream.get(timeout=min(.2,max(.001,next_check-time.monotonic())))
                except queue.Empty:line=''
                if line is None:eof=True
                elif line:log.write(line);log.flush();print(line,end='',flush=True)
                code=child.poll()
                if code is not None:
                    exited_at=now if exited_at is None else exited_at
                    if eof:break
                    if now-exited_at>2:
                        record=_bounded_path(output/(label+'.tree.json'))
                        record.write_text(json.dumps({'success':False,'root_pid':child.pid,'parent_pid':os.getpid(),'label':label,'started_at':started,'reason':'exited-root-held-pipe','remaining':None,'cleanup_verified':False}),encoding='utf8')
                        error=CaptureWorkerError('Exited worker left stdout held open')
                        error.add_note('Cleanup is unverified; exited root PID was not signalled. Audit: '+str(record))
                        raise error
            if code:raise CaptureWorkerError('Capture worker '+label+' exited '+str(code))
            event('complete')
            return code
        except BaseException as error:
            if child is not None:
                try:_stop_tree(root,ticket,output,child,label,started)
                except BaseException as cleanup:error.add_note('Capture cleanup could not be verified: '+repr(cleanup))
            raise
        finally:
            if pump is not None:pump.join(timeout=2)
            if child is not None and child.stdout is not None and (pump is None or not pump.is_alive()):child.stdout.close()
