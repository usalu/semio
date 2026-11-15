import os,pathlib,subprocess,time,datetime,json,re,signal
N=14;MAXTRIES=12;R=pathlib.Path('/Users/ueli/Documents/semio');W=R/'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES';O=W/'workers/g1-m08';H=pathlib.Path('/var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501')
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
def cmd(a):return subprocess.run(a,capture_output=True,text=True).stdout.strip()
def safe(s):return re.sub(r'(?i)((?:token|password|secret|authorization)[=: ]+)[^\s]+',r'\1[redacted]',s)
def save(n,o):(O/n).write_text(json.dumps(o,ensure_ascii=False,indent=2)+'\n')
def chain(pid,byid):
 out=[];seen=set()
 for _ in range(32):
  if pid<1 or pid in seen:break
  seen.add(pid);r=byid.get(pid)
  if r is None:out.append({'pid':pid,'observation':safe(cmd(['ps','-o','pid,ppid,args','-p',str(pid)]))});break
  out.append({'pid':r[0],'ppid':r[1],'rss_kib':r[3],'args':r[4]})
  if pid==1:break
  pid=r[1]
 return out
def control():
 selected=O/'next.md' if (O/'next.md').exists() else W/'ROUND.md'
 try:
  text=selected.read_text();m=re.search(r'^round:\s*(\d+)',text,re.M)
  if m and int(m[1])>N:
   d={'at':utc(),'round':int(m[1]),'selected':str(selected),'instructions':text,'global_round':(W/'ROUND.md').read_text(),'protocol':(W/'PROTOCOL.md').read_text()};save('coordinator-control.json',d);return d
 except OSError:pass
 return None

def launcher_identity():
 paths=[pathlib.Path.home()/'.local/share/semio/launcher/bin/repo',pathlib.Path.home()/'.local/bin/repo'];d={'at':utc(),'ls_li':cmd(['ls','-li',*[str(p) for p in paths]]),'paths':{}}
 for p in paths:
  v={'exists':p.exists(),'is_symlink':p.is_symlink()}
  try:
   l=p.lstat();s=p.stat();v.update({'inode':l.st_ino,'mtime_ns':l.st_mtime_ns,'mtime_utc':datetime.datetime.fromtimestamp(l.st_mtime,datetime.timezone.utc).isoformat(),'target_inode':s.st_ino,'target_mtime_ns':s.st_mtime_ns,'size':s.st_size,'readlink':os.readlink(p) if p.is_symlink() else None})
  except OSError as e:v['error']=str(e)
  d['paths'][str(p)]=v
 p=subprocess.run(['codesign','-dvvv',str(paths[0])],capture_output=True,text=True);d['codesign_rc']=p.returncode;d['codesign_lines']=[l for l in (p.stdout+p.stderr).splitlines() if 'Identifier' in l or 'CDHash' in l];return d
def launcher_key(d):
 v=d['paths'].get(str(pathlib.Path.home()/'.local/share/semio/launcher/bin/repo'),{})
 return {k:v.get(k) for k in ('inode','mtime_ns','target_inode','target_mtime_ns','readlink')}|{'codesign_lines':d.get('codesign_lines')}
def replica_check(attempt):
 p=subprocess.run(['launchctl','print',f'gui/{os.getuid()}/tech.semio.fleet.replica'],capture_output=True,text=True);ls=[l for l in (p.stdout+p.stderr).splitlines() if re.search(r'^\s*(?:state|runs|last exit code)\s*=',l)];d={'at':utc(),'selected_lines':ls,'kickstart_count':0}

 h=json.loads((R/'.cache/f/health.json').read_text());d.update({'digest':h.get('code'),'pid':h.get('pid'),'health_age_s':round(time.time()-(R/'.cache/f/health.json').stat().st_mtime,3)});save(f'round-{N}-try-{attempt}-replica-before.json',d);return d
def consent_wait(attempt,text):
 at=utc();save('state.json',{'origin':'g1-m08','round':N,'native_rc':4,'ready':False,'failed':['consent'],'at':at})
 with (O/f'round-{N}.md').open('a') as f:f.write('\nconsent: native_rc=4; waiting for owner under round 8 step 3. Printed native text (last 60 lines):\n\n'+'\n'.join(text.splitlines()[-60:])+'\n')
 started=time.monotonic();nextpoll=started;rows=[];reason='30-minute bound';newround=None
 print(json.dumps({'event':'consent-wait','round':N,'attempt':attempt,'at':at,'maximum_seconds':1800}),flush=True)
 while time.monotonic()-started<1800:
  newround=control()
  if newround:reason='coordinator superseded round';break
  now=time.monotonic()
  if now<nextpoll:time.sleep(min(30,nextpoll-now));continue
  began=time.monotonic();env=os.environ.copy();env['SEMIO_WORKSPACE_ROOT']=str(R);env['SEMIO_LANGUAGE']='en';p=subprocess.run(['/Users/ueli/.local/bin/repo','preflight','--lang','en'],cwd=R,env=env,capture_output=True,text=True);output=p.stdout+p.stderr;access=[l for l in output.splitlines() if re.search(r'access|consent|heavy|oversized|too large|under.*access|direct child|caller|Documents|macOS',l,re.I)];ok=p.returncode==0 and any(re.search(r'\baccess\b.*\bok\b|\bok\b.*\baccess\b',l,re.I) for l in access);row={'at':utc(),'elapsed_s':round(time.monotonic()-started,3),'rc':p.returncode,'access_lines':access,'access_ok_explicit':ok,'stdout':p.stdout,'stderr':p.stderr};rows.append(row);save(f'round-{N}-try-{attempt}-consent-preflight-polls.json',rows)
  print(json.dumps({'event':'consent-preflight','round':N,'attempt':attempt,'at':row['at'],'elapsed_s':row['elapsed_s'],'rc':p.returncode,'access_ok_explicit':ok,'access_lines':access[-3:]}),flush=True)
  if ok:reason='preflight access ok';break
  nextpoll=began+60
 d={'start':at,'end':utc(),'wait_s':round(time.monotonic()-started,3),'reason':reason,'polls':len(rows)};save(f'round-{N}-try-{attempt}-consent-wait.json',d);return d,newround


def fleet_connectivity():
 import socket
 out={}
 for port in (2049,6094,8981):
  try:
   with socket.create_connection(('192.168.178.70',port),timeout=2):pass
   out[str(port)]={'reachable':True}
  except OSError as e:out[str(port)]={'reachable':False,'error':str(e)}
 return {'at':utc(),'ports':out}


def normalized_progress(line):
 line=re.sub(r'\x1b\[[0-9;]*[A-Za-z]','',line).strip()
 if not line:return None
 if re.search(r'still (?:working|running) after|remains active on|^\[capacity\]|^graph-server: alive|^graph: waiting for memory headroom',line,re.I):return None
 if line.startswith('[script] {'):
  try:
   d=json.loads(line[len('[script] '):]);a=d.get('activity',{});r=a.get('resource','') if isinstance(a,dict) else ''
   if r.startswith(('lease-sweep','boot-recovery')):return None
   if r.startswith(('build:graph-phase:owners:','build:graph-phase:inference:')):
    parts=r.split(':',7)
    if len(parts)==8:
     parts[6]='<elapsed>';entities=[]
     for entity in parts[7].split(','):
      f=entity.split('|')
      if len(f)>=4:f[2]='<elapsed>';f=f[:4]
      entities.append('|'.join(f))
     parts[7]=','.join(entities);a['resource']=':'.join(parts)
   def clean(v):
    if isinstance(v,dict):return {k:clean(x) for k,x in v.items() if k not in ('elapsedMs','elapsedMilliseconds','at','wallMs','ageMs','ageMilliseconds','timestamp','updatedAt','pid')}
    if isinstance(v,list):return [clean(x) for x in v]
    return v
   return json.dumps(clean(d),sort_keys=True,ensure_ascii=False)
  except ValueError:pass
 line=re.sub(r'^Reading the project graph slice \([^)]*\):\s*','graph-progress: ',line)
 line=re.sub(r'\(\d+(?:\.\d+)?\s*(?:ms|s)\)','(<elapsed>)',line)
 line=re.sub(r'\b\d+(?:\.\d+)?\s*(?:seconds?|minutes?)\b','<elapsed>',line)
 line=re.sub(r'^\d{4}-\d\d-\d\dT[0-9:.]+Z\s*','',line)
 line=re.sub(r'(?:;|·)\s*waiting for memory headroom.*$','',line)
 line=re.sub(r'(\([^(),]+,\s*)\d+(\s*,\s*\d+\))',r'\1<elapsed>\2',line)
 line=re.sub(r'\b\d+(?:\.\d+)?\s*s\b','<elapsed>s',line)
 return line
def observe_progress(line,at):
 global last_progress,writer_failure
 if False:writer_failure=True
 signature=normalized_progress(line)
 if signature is None:return
 import hashlib
 key=hashlib.sha256(signature.encode()).hexdigest();progress_seen.add(key)
 if signature.startswith('{'):
  try:
   d=json.loads(signature);r=d.get('activity',{}).get('resource','');family=str(d.get('owner',''))+'|'+str(d.get('command',''))+'|'+re.split(r':\d',r,maxsplit=1)[0]
  except (ValueError,AttributeError):family=re.sub(r'\d+(?:\.\d+)?','<n>',signature)
 else:family=re.sub(r'\d+(?:\.\d+)?','<n>',signature)
 if progress_family_last.get(family)!=signature:
  progress_family_last[family]=signature;last_progress=time.monotonic();progress_recent.append({'at':at,'line':safe(line)})
  if len(progress_recent)>100:del progress_recent[:-100]
def stall_snapshot(at,hosts,rows):
 pids={v['pid'] for v in hosts.values()};found=set(pids)
 for _ in range(32):
  old=len(found);found.update(r[0] for r in rows if r[1] in found)
  if len(found)==old:break
 existing={r[0] for r in rows};found &= existing
 ps=cmd(['ps','-o','pid,ppid,etime,rss,args','-p',','.join(map(str,sorted(found)))]) if found else 'No live service or child PID observed.'
 tails={}
 for name in hosts:
  lp=H/pathlib.Path(name).with_suffix('.log')
  if lp.exists():tails[lp.name]=lp.read_text(errors='replace').splitlines()[-60:]
 return {'at':at,'no_new_progress_s':round(time.monotonic()-last_progress,3),'recent_progress':progress_recent,'service_and_children_ps':safe(ps),'service_last60':tails}

pre={};knownhostpids=set();save(f'round-{N}-machine-before.json',{'at':utc(),'os':cmd(['sw_vers']),'memory_bytes':cmd(['sysctl','-n','hw.memsize']),'disk':cmd(['df','-k',str(R)])});results=[];superseded=None;previous_launcher_after=None
live={int(x) for x in cmd(['ps','-axo','pid=']).split()}
for hp in H.glob('host-*.json'):
 try:
  d=json.loads(hp.read_text())
  if d.get('pid') in live:knownhostpids.add(d['pid'])
 except (OSError,ValueError):pass
(O/f'round-{N}.md').write_text(f'# Round {N} — g1-m08 — In progress\n\nFive-second P12 sampler starts before each try; attempts are three minutes apart.\n')
results=json.loads((O/f'round-{N}-attempts.json').read_text()) if (O/f'round-{N}-attempts.json').exists() else []
attempt=len(results);counted_tries=len(results)
if results:
 wait_until=datetime.datetime.fromisoformat(results[-1]['end'].replace('Z','+00:00')).timestamp()+180
 while time.time()<wait_until:
  if control():raise SystemExit('Round superseded during recovery retry wait')
  time.sleep(min(30,wait_until-time.time()))
while counted_tries<MAXTRIES:
 attempt+=1
 superseded=control()
 if superseded:break
 log=pathlib.Path.home()/(f'native-bootstrap-round-{N}.log' if attempt==1 else f'native-bootstrap-round-{N}-try-{attempt}.log')
 if log.exists():raise SystemExit('Refusing to overwrite '+str(log))
 if results:
  prior=results[-1];prior['retry_wait_s']=round((datetime.datetime.now(datetime.timezone.utc)-datetime.datetime.fromisoformat(prior['end'].replace('Z','+00:00'))).total_seconds(),3);save(f'round-{N}-try-{prior["attempt"]}-result.json',prior);save(f'round-{N}-attempts.json',results)
 replica_observation=replica_check(attempt);launcher_before=launcher_identity();save(f'round-{N}-try-{attempt}-launcher-before.json',launcher_before)
 start=time.monotonic();epoch=time.time();started=utc();samples=[];events=[];pressures=[];owned=set(knownhostpids);hosts={};host_history=[];hard=None;signaled=None;readpos=0;partial='';nextsample=start;nextbeat=start;nextcontrol=start;warn=0;critical=0;peak=0;service_events=[];service_offsets={};host_stamps={};last_progress=start;progress_seen=set();progress_family_last={};progress_recent=[];writer_failure=False;stall_stop=False;stall_evidence=None;dependency_wait_start=None;dependency_wait_line=None
 for hp in H.glob('host-*.json'):
  try:
   d=json.loads(hp.read_text());host_stamps[hp.name]=d.get('startedAt');lp=hp.with_suffix('.log');service_offsets[lp.name]=lp.stat().st_size if lp.exists() else 0
  except (OSError,ValueError):pass
 sample_out=O/f'round-{N}-try-{attempt}-p12-samples.txt'
 sampler=subprocess.Popen(['zsh',str(O/'round-14-sampler.zsh'),str(sample_out)],cwd=R)
 time.sleep(1)
 p=subprocess.Popen(['bash',str(W/'native-bootstrap.sh'),str(log)],cwd=R,start_new_session=True);owned.add(p.pid)
 print(json.dumps({'event':'native-start','round':N,'attempt':attempt,'at':started,'pid':p.pid}),flush=True)
 while True:
  now=time.monotonic();at=utc();rows=[]
  for line in cmd(['ps','-axo','pid=,ppid=,pgid=,rss=,args=']).splitlines():
   f=line.strip().split(None,4)
   if len(f)==5:
    try:rows.append((int(f[0]),int(f[1]),int(f[2]),int(f[3]),safe(f[4])))
    except ValueError:pass
  byid={x[0]:x for x in rows}
  active_log_stamp=0
  for ap in H.glob('host-*.json'):
   try:
    ad=json.loads(ap.read_text());apid=ad.get('pid');astamp=ad.get('startedAt',0)
    if isinstance(apid,int) and apid in byid and (apid in knownhostpids or isinstance(astamp,(int,float)) and astamp>=epoch*1000-1000):active_log_stamp=max(active_log_stamp,astamp)
   except (OSError,ValueError):pass
  for hp in H.glob('host-*.json'):
   try:
    d=json.loads(hp.read_text());pid=d.get('pid');stamp=d.get('startedAt',0)
    if isinstance(pid,int) and (pid in knownhostpids or isinstance(stamp,(int,float)) and stamp>=epoch*1000-1000):
     owned.add(pid);knownhostpids.add(pid)
     if (hp.name not in hosts or hosts[hp.name]['pid']!=pid) and pid in byid:hosts[hp.name]={'at':at,'pid':pid,'startedAt':stamp,'implementation':d.get('implementation'),'parent_chain':chain(pid,byid),'started_by':'coordinator native bootstrap; reused if started during prior try/round'};host_history.append({'metadata_file':hp.name,**hosts[hp.name]})
     if host_stamps.get(hp.name)!=stamp:host_stamps[hp.name]=stamp
     lp=hp.with_suffix('.log')
     if lp.exists():
      pos=service_offsets.get(lp.name,0)
      if lp.stat().st_size<pos:pos=0
      with lp.open(errors='replace') as f:f.seek(pos);fresh=f.read();service_offsets[lp.name]=f.tell()
      for l in fresh.splitlines():
       if stamp==active_log_stamp:observe_progress(l,at)
       if re.search(r'waiting for the fleet|taken from the fleet|inferred here|refreshing \d+ projects|asking the host for|stale facts|\[capacity\]|graph:|fleet|wanted|arrived|serving|restored|Restarting|adopted|closureRepaired|access-denied|consent|heavy|oversized|too large|under.*access|direct child|caller|request-failed|EPERM|where:|closureRepaired|access-denied|consent|heavy|oversized|too large|under.*access|direct child|caller',l,re.I):service_events.append({'at':at,'elapsed_s':round(now-start,3),'source':lp.name,'line':safe(l)})
   except (OSError,ValueError):pass
  for _ in range(24):
   old=len(owned);owned.update(x[0] for x in rows if x[1] in owned or x[2]==p.pid)
   if len(owned)==old:break
  own=sorted((x for x in rows if x[0] in owned),key=lambda x:x[3],reverse=True)
  if own:peak=max(peak,own[0][3])
  pressure=cmd(['sysctl','-n','kern.memorystatus_vm_pressure_level']);pressures.append({'at':at,'level':pressure});reason=None
  if own and own[0][3]>8*1024*1024:reason=f'run-owned process above 8 GiB RSS: PID {own[0][0]}, {own[0][3]} KiB'

  if now>=nextsample:
   top=cmd(['/bin/sh','-c','ps -axo rss,pid,ppid,args | sort -nr | head -3']);base=cmd(['/bin/sh','-c','ps -axo rss,command | sort -nr | head -3']);chains=[]
   for l in top.splitlines():
    f=l.strip().split(None,3)
    if len(f)==4 and f[1].isdigit():chains.append(chain(int(f[1]),byid))
   warn=warn+1 if pressure=='2' else 0;critical=critical+1 if pressure=='4' else 0;samples.append({'at':at,'elapsed_s':round(now-start,3),'pressure':pressure,'warning_streak':warn,'critical_streak':critical,'top3_with_parents_verbatim':safe(top),'top3_base_verbatim':safe(base),'ancestor_chains':chains,'owned_top3':own[:3]})
   if warn>=6:reason='macOS memory pressure 2 in six consecutive 30-second samples'
   if critical>=2:reason='macOS memory pressure 4 in two consecutive 30-second samples'
   nextsample=now+30
   save(f'round-{N}-try-{attempt}-ps-live.json',samples)
   save(f'round-{N}-try-{attempt}-pressure-live.json',pressures)
   save(f'round-{N}-try-{attempt}-service-events-live.json',service_events)
  if log.exists():
   with log.open(errors='replace') as f:f.seek(readpos);chunk=f.read();readpos=f.tell()
   ls=(partial+chunk).split('\n');partial=ls.pop()
   for l in ls:
    observe_progress(l,at)
    if 'Another process holds the dependency store; waiting for it' in l and dependency_wait_start is None:dependency_wait_start=time.monotonic();dependency_wait_line=l
    if re.search(r'\[semio\]|\[capacity\]|graph:|fleet|wanted|arrived|serving|restored|Restarting|adopted|closureRepaired|access-denied|consent|heavy|oversized|too large|under.*access|direct child|caller|\[script\]|Reading the project graph|NX|❌|✖',l,re.I):events.append({'at':at,'elapsed_s':round(now-start,3),'line':safe(l)})
  if dependency_wait_start is not None and now-dependency_wait_start>=300 and reason is None:reason='Dependency store wait exceeded 5 minutes'
  if now-last_progress>=900 and reason is None:stall_stop=True;reason='No new progress line for 15 minutes'
  if writer_failure:reason='Reference 3420bc16 returned; round 11 requests full stack and stop'
  if now>=nextcontrol:
   superseded=control();nextcontrol=now+30
   if superseded:reason='Coordinator superseded this run with round '+str(superseded['round'])
  if reason and signaled is None:
   interruption_kind='writer' if writer_failure else 'coordinator' if superseded else 'stall' if stall_stop and reason=='No new progress line for 15 minutes' else 'resource'
   hard={'interruption_kind':interruption_kind,'at':at,'reason':reason,'owned_top3':own[:3],'pressure':pressure,'warning_streak':warn,'critical_streak':critical,'coordinator_interruption':bool(superseded)};signaled=time.monotonic()
   if reason=='Dependency store wait exceeded 5 minutes':
    waitps='\n'.join(line for line in cmd(['ps','-axo','pid,ppid,etime,command']).splitlines() if re.search(r'bun|nx|repo',line,re.I));save(f'round-{N}-try-{attempt}-dependency-wait.json',{'at':at,'wait_line':dependency_wait_line,'wait_s':round(now-dependency_wait_start,3),'processes':safe(waitps)})
   if interruption_kind=='stall':stall_evidence=stall_snapshot(at,hosts,rows);save(f'round-{N}-try-{attempt}-stall-evidence.json',stall_evidence)
   try:os.killpg(p.pid,signal.SIGINT);hard['sigint_sent']=True
   except ProcessLookupError:hard['sigint_sent']=False
   except PermissionError as e:
    hard['sigint_sent']=False;hard['signal_error']=str(e);hard['wrapper_poll']=p.poll()
    hard['delivery']='unknown; no second signal attempted'
   save(f'round-{N}-try-{attempt}-interruption-live.json',hard)
   print(json.dumps({'event':'interruption','round':N,'attempt':attempt,'at':at,'reason':reason,'pressure':pressure,'warning_streak':warn,'critical_streak':critical,'sigint_sent':hard['sigint_sent']}),flush=True)
  if now>=nextbeat:
   latest=service_events[-1]['line'] if service_events else events[-1]['line'] if events else None
   print(json.dumps({'event':'native-observation','round':N,'attempt':attempt,'at':at,'elapsed_s':round(now-start),'pressure':pressure,'warning_streak':warn,'critical_streak':critical,'peak_owned_rss_kib':peak,'graph_host_pids':[x['pid'] for x in hosts.values()],'latest':latest[:500] if latest else None}),flush=True);nextbeat=now+55
  if signaled is not None:
   if time.monotonic()-signaled>=60 and p.poll() is not None:break
  elif p.poll() is not None:break
  time.sleep(5)
 pathlib.Path(str(sample_out)+'.stop').touch();sampler.wait()
 ended=utc();wall=round(time.monotonic()-start,3);raw=log.read_bytes() if log.exists() else b'';text=raw.decode(errors='replace');lines=text.splitlines();matches=re.findall(r'^native_rc=(\d+)',text,re.M);native=int(matches[-1]) if matches else None
 tails={}
 for name in hosts:
  lp=H/pathlib.Path(name).with_suffix('.log')
  if lp.exists():tails[lp.name]=lp.read_text(errors='replace').splitlines()[-30:]
 service_full={}
 for name in hosts:
  lp=H/pathlib.Path(name).with_suffix('.log')
  if lp.exists():
   body=lp.read_text(errors='replace'); dest=O/f'round-{N}-try-{attempt}-{lp.name}';dest.write_text(body);service_full[lp.name]=str(dest)
 combined=lines+[e['line'] for e in service_events];refusal=bool(native not in (None,0) and (native==75 or any(re.search(r'graph-wanted(?:\.expired)?|nothing to wait for any longer|asked the fleet.*(?:remain|missing)|fleet was asked for.*still missing',l,re.I) for l in lines)));values=None
 for l in reversed(lines):
  m=re.search(r'\{"wanted":\d+.*?\}',l)
  if m:
   try:values=json.loads(m.group());break
   except ValueError:pass
  m=re.search(r'fleet was asked for (\d+) and delivered (\d+) within (\d+) s, (\d+) are still missing',l)
  if m:values={'wanted':int(m[1]),'arrived':int(m[2]),'seconds':int(m[3]),'missing':int(m[4])};break
  m=re.search(r'asked the fleet for (\d+).*?received (\d+) within (\d+) s.*?(\d+) remain',l)
  if m:values={'wanted':int(m[1]),'arrived':int(m[2]),'seconds':int(m[3]),'missing':int(m[4])};break
 launcher_after=launcher_identity();save(f'round-{N}-try-{attempt}-launcher-after.json',launcher_after);launcher_changed_during=launcher_key(launcher_before)!=launcher_key(launcher_after);launcher_changed_since_previous_after=None if previous_launcher_after is None else launcher_key(previous_launcher_after)!=launcher_key(launcher_after);previous_launcher_after=launcher_after
 setup_text=text.split('[semio] workspace:setup')[-1];has_failed_tasks=bool(re.search(r'Failed tasks:|The following tasks failed|failed to run target|NX[^\n]*failed|\b[1-9]\d* failed\b|✖',setup_text,re.I));error_rows=[l for l in setup_text.splitlines() if not l.startswith(('Reading the project graph slice','[script]')) and re.search(r'\berror(?:\[|:|=)|\bfailed\b|\bfatal\b|\bstale\b|closure\.moving|could not answer',l,re.I)];error_signature=[re.sub(r'\b[0-9a-f]{16,64}\b','<hash>',re.sub(r'\d{4}-\d\d-\d\dT[0-9:.]+Z','<time>',l)) for l in error_rows];error_signature=sorted(set(error_signature));transient_task=bool(re.search(r'source-differs|closure\.moving',setup_text,re.I) or has_failed_tasks and any(re.search(r'\bretry\b',l,re.I) for l in error_rows));same_nontransient=False
 graph_retry_advice=bool(re.search(r'The (?:graph|resident) service[^\n]*(?:lost|timeout|unexpected failure)[^\n]*(?:try again|retry)',text,re.I))
 source_differs=bool(re.search(r'stale[^\n]*build\.closure\.source-differs',text,re.I));closure_enoent=bool(re.search(r'ENOENT[^\n]*build-input-closures/[^\s]+\.json',text,re.I));consent=(native==4 or bool(re.search(r'repo[^\n]*ended with 4(?:\D|$)',text,re.I)))
 connectivity=fleet_connectivity();baseline=pre.get('fleet_server_baseline_tcp',{});observed_closed=[port for port,state in connectivity['ports'].items() if not state['reachable']];outage_error=bool(re.search(r'(?:cache|store)[^\n]*(?:timeout|timed out|not reachable|unreachable|connection refused)|ECONNREFUSED|ETIMEDOUT|ENETUNREACH',setup_text,re.I));outage_exempt=bool((not hard or hard.get('interruption_kind')=='stall') and not writer_failure and not superseded and native not in (None,0,4) and (outage_error or observed_closed and (refusal or native==75 or stall_stop)));outage_exempt=False;counted_tries+=1
 arrived=[e['line'] for e in service_events if 'projects arrived' in e['line']]
 current_service='\n'.join(e['line'] for e in service_events)
 closure_missing=bool(re.search(r'closure-missing|ENOENT[^\n]*build-input-closures|closure[^\n]*missing',current_service+'\n'+setup_text,re.I))
 retry_wording=bool(re.search(r'source-differs|closure\.moving|\bretry\b|try again',setup_text,re.I))
 retry_class=bool(native==75 or hard or closure_missing or retry_wording)
 # Underlying service error identifies repeated errors even if frontend says try again.
 underlying=[e['line'] for e in service_events if re.search(r'request-failed.*(?:Error|Exception)|where:',e['line'])]
 key='\n'.join(underlying[-3:]) if underlying else '\n'.join(error_rows[-6:])
 key=re.sub(r'\d{4}-\d\d-\d\dT[0-9:.]+Z','<time>',key)
 key=re.sub(r'request-failed [0-9a-f]+','request-failed <ref>',key)
 key=re.sub(r'\b[0-9a-f]{16,64}\b','<hash>',key)
 other=not (native==75 or hard or closure_missing or re.search(r'source-differs|closure\.moving',setup_text,re.I))
 same_nontransient=bool(other and results and results[-1].get('other_error') and results[-1].get('underlying_error_signature')==key)
 if same_nontransient:retry_class=False
 writer_logs={}
 if writer_failure:
  for lp in H.glob('host-*.log'):
   try:
    body=lp.read_text(errors='replace')
    if '3420bc16' in body:
     dest=O/f'round-{N}-try-{attempt}-writer-{lp.name}';dest.write_text(body);writer_logs[str(lp)]=str(dest)
   except OSError:pass
 save(f'round-{N}-try-{attempt}-progress-summary.json',{'at':utc(),'last_progress_gap_s':round(time.monotonic()-last_progress,3),'unique_progress_lines':len(progress_seen),'recent':progress_recent,'stall':stall_stop,'writer_failure':writer_failure})
 result={'round':N,'attempt':attempt,'other_error':other,'underlying_error_signature':key,'closure_missing':closure_missing,'retry_wording':retry_wording,'service_full_logs':service_full,'p12_samples':str(sample_out),'start':started,'end':ended,'wall_s':wall,'log':str(log),'wrapper_rc':p.poll(),'native_rc':native,'interruption':hard,'peak_owned_rss_kib':peak,'sample_count':len(samples),'fleet_refusal':refusal,'fleet_numbers':values,'first_arrived_line':arrived[0] if arrived else None,'last_arrived_line':arrived[-1] if arrived else None,'retry_wait_s':0,'marker_exists':(R/'.cache/f/native-bootstrap.done').exists(),'graph_hosts':hosts,'superseded_round':superseded['round'] if superseded else None,'consent':consent,'source_differs':source_differs,'closure_enoent':closure_enoent,'launcher_changed_during_try':launcher_changed_during,'launcher_changed_since_previous_after':launcher_changed_since_previous_after,'has_failed_tasks':has_failed_tasks,'transient_task':transient_task,'same_nontransient_errors_twice':same_nontransient,'task_error_signature':error_signature,'host_history':host_history,'outage_exempt':outage_exempt,'counted_tries':counted_tries,'server_connectivity':connectivity,'baseline_ports_now_closed':observed_closed,'stall_stop':stall_stop,'writer_reference_failure':writer_failure,'graph_retry_advice':graph_retry_advice,'writer_full_logs':writer_logs,'last_progress_gap_s':round(time.monotonic()-last_progress,3)}
 for suffix,obj in [('result',result),('ps',samples),('pressure',pressures),('events',events),('service-events',service_events),('service-tails',tails)]:save(f'round-{N}-try-{attempt}-{suffix}.json',obj)
 (O/f'round-{N}-try-{attempt}.tail.txt').write_bytes(b''.join(raw.splitlines(keepends=True)[-200:]));(O/f'round-{N}-try-{attempt}-required-lines.txt').write_text('NATIVE\n'+'\n'.join(l for l in lines if re.search(r'\[capacity\]|graph:|fleet|wanted|arrived|serving|restored|Restarting|adopted|closureRepaired|access-denied|consent|heavy|oversized|too large|under.*access|direct child|caller',l,re.I))+'\n\nRESIDENT SERVICE\n'+'\n'.join(e['line'] for e in service_events)+'\n')
 results.append(result);save(f'round-{N}-attempts.json',results)
 subprocess.run(['python3',str(O/'round-14-report.py')],check=True)
 with (O/f'round-{N}.md').open('a') as f:f.write('\nCompleted attempt '+str(attempt)+'\n'+json.dumps({k:v for k,v in result.items() if k!='graph_hosts'},ensure_ascii=False,indent=2)+'\n')
 print(json.dumps({'event':'native-end',**{k:v for k,v in result.items() if k not in ('graph_hosts','interruption','host_history')}}),flush=True)
 if writer_failure:break
 if superseded:break
 if native==0:break
 if consent:
  save('state.json',{'origin':'g1-m08','round':N,'native_rc':4,'ready':False,'failed':['consent'],'at':utc()})
  with (O/f'round-{N}.md').open('a') as f:f.write('\nconsent: native_rc=4. Round 13 says stop on native_rc=4. Native printed text:\n'+'\n'.join(text.splitlines()[-60:])+'\n')
  break
 if same_nontransient and not outage_exempt:break
 if (retry_class or other) and counted_tries<MAXTRIES:
  waitseconds=180;ws=time.monotonic();print(json.dumps({'event':'authorized-retry-wait','round':N,'seconds':waitseconds,'next_attempt':attempt+1,'at':utc()}),flush=True)
  while time.monotonic()-ws<waitseconds:
   superseded=control()
   if superseded:break
   time.sleep(min(30,waitseconds-(time.monotonic()-ws)))
  result['retry_wait_s']=round(time.monotonic()-ws,3);save(f'round-{N}-try-{attempt}-result.json',result);save(f'round-{N}-attempts.json',results)
  if superseded:break
  continue
 break
print(json.dumps({'event':f'round-{N}-observation-complete','at':utc(),'attempts':len(results),'superseded_round':superseded['round'] if superseded else None}),flush=True)
