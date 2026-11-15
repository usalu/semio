import pathlib,subprocess,os,time,datetime,json,signal,re
R=pathlib.Path('/Users/ueli/Documents/semio');O=R/'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/workers/g1-m08'
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
def save(n,d):(O/n).write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n')
assert not (O/'round-16-tools-setup-started.json').exists(),'tools setup is once per machine in round16'
log=pathlib.Path.home()/'tools-setup-round-16.log';assert not log.exists(),'do not overwrite tools setup log'
env=os.environ.copy();env['PATH']=str(pathlib.Path.home()/'.local/bin')+':'+env.get('PATH','');env['SEMIO_WORKSPACE_ROOT']=str(R);env['SEMIO_LANGUAGE']='en'
start=time.monotonic();at=utc();waitstart=None;waitline=None;signal_note=None;nextbeat=start;pos=0
with (O/'round-16-tools-setup-console.txt').open('w') as f:
 p=subprocess.Popen(['bash','-c','repo tools setup all 2>&1 | tee "$HOME/tools-setup-round-16.log"; task_rc=${PIPESTATUS[0]}; exit "$task_rc"'],cwd=R,env=env,stdout=f,stderr=subprocess.STDOUT,start_new_session=True)
 save('round-16-tools-setup-started.json',{'at':at,'pid':p.pid,'log':str(log)});print(json.dumps({'event':'tools-setup-start','at':at,'pid':p.pid}),flush=True)
 while p.poll() is None:
  now=time.monotonic();body=log.read_text(errors='replace') if log.exists() else '';fresh=body[pos:];pos=len(body)
  for line in fresh.splitlines():
   if 'Another process holds the dependency store' in line and waitstart is None:waitstart=now;waitline=line
  if waitstart is not None and now-waitstart>120 and signal_note is None:
   ps=subprocess.run(['ps','-axo','pid,ppid,etime,command'],capture_output=True,text=True).stdout
   save('round-16-tools-setup-dependency-wait.json',{'at':utc(),'wait_s':round(now-waitstart,3),'line':waitline,'processes':'\n'.join(l for l in ps.splitlines() if re.search(r'bun|repo',l))})
   signal_note={'at':utc(),'reason':'Dependency store wait above two minutes','target_process_group':p.pid}
   try:os.killpg(p.pid,signal.SIGINT);signal_note['sigint_attempt']='sent'
   except (ProcessLookupError,PermissionError) as e:signal_note['sigint_attempt']=str(e)
   save('round-16-tools-setup-interruption.json',signal_note);print(json.dumps({'event':'tools-setup-interruption',**signal_note}),flush=True)
  if now>=nextbeat:
   print(json.dumps({'event':'tools-setup-observation','at':utc(),'elapsed_s':round(now-start),'dependency_wait_s':round(now-waitstart) if waitstart else None,'latest':body.splitlines()[-1][:350] if body.splitlines() else None}),flush=True);nextbeat=now+55
  time.sleep(5)
 result={'start':at,'end':utc(),'wall_s':round(time.monotonic()-start,3),'exit_code':p.returncode,'log':str(log),'interruption':signal_note,'last15':log.read_text(errors='replace').splitlines()[-15:] if log.exists() else []}
 save('round-16-tools-setup-result.json',result);(O/'round-16-tools-setup.md').write_text('# Round 16 tools setup\n\n~~~json\n'+json.dumps({k:v for k,v in result.items() if k!='last15'},indent=2)+'\n~~~\n\nLast 15 lines verbatim:\n\n~~~text\n'+'\n'.join(result['last15'])+'\n~~~\n');print(json.dumps({'event':'tools-setup-end',**result}),flush=True)

