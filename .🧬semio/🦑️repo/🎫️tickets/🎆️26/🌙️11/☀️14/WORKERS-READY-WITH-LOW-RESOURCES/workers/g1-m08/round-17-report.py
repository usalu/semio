import pathlib,json,re,datetime
R=pathlib.Path('/Users/ueli/Documents/semio');O=R/'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/workers/g1-m08'
def read(n):return json.loads((O/n).read_text())
def block(s):return '\n~~~text\n'+s+'\n~~~\n'
a=read('round-17-attempts.json');m=read('round-17-machine-before.json')
out=['# Round 17 — g1-m08\n',block(json.dumps(m,ensure_ascii=False,indent=2))]
if (O/'round-16-tools-setup.md').exists():out+=[(O/'round-16-tools-setup.md').read_text()]
out+=['P12 sampler follows the supplied block with OUT inside this worker folder and the newest host log refreshed every sample. Full sampler output is included per try. Closure counters run from the worktree root. No limit-off flag, repair, sudo, or service stop was used.\n']
for x in a:
 k=x['attempt'];pre=f'round-17-try-{k}'
 out+=['\n## Try '+str(k)+'\n',block(json.dumps(x,ensure_ascii=False,indent=2))]
 replica=read(pre+'-replica-before.json');out+=['Replica digest / observation:',block(json.dumps(replica,indent=2))]
 native=pathlib.Path(x['log']).read_text(errors='replace');lines=native.splitlines()
 phases=[];last=None
 for l in lines:
  if re.search(r'^\[semio\] ',l):
   phases.append(l)
 phase_events=[e for e in read(pre+'-events.json') if e['line'].startswith('[semio] ')]
 for i,e in enumerate(phase_events):
  finish=phase_events[i+1]['elapsed_s'] if i+1<len(phase_events) else x['wall_s']
  out+=['Observed phase interval: '+e['line']+'; '+str(round(finish-e['elapsed_s'],3))+' seconds (5-second polling resolution).']
 out+=['Phase boundaries as printed (individual wall times not measured where absent):',block('\n'.join(phases))]
 ev=read(pre+'-service-events.json')
 selected=[e['line'] for e in ev if re.search(r'^graph:|graph-plan-too-heavy|host-graph-plan-too-heavy|\[graph\] closure-missing|Graph: Plan-Stufe|\[graph-phase\]|\[capacity\]',e['line'])]
 restore=[l for l in selected if 'restored' in l]
 out+=['Restore line: '+('see verbatim block' if restore else 'not emitted during this try; age A and stale count S in the requested restore format not measured.')]
 grantlines=[l for l in selected if 'per project' in l and 'infer' in l]
 firstplan=grantlines[0] if grantlines else next((l for l in selected if 'plan stage' in l),'No plan line emitted during this try.')
 out+=['Per-project grant summary:',block('\n'.join(grantlines) if grantlines else 'Not emitted during this try; per-project grant not measured from a summary line.')] 
 out+=['First plan line:',block(firstplan)]
 if (O/(pre+'-zero-grant-memory.json')).exists():out+=['Zero-grant memory diagnostics:',block((O/(pre+'-zero-grant-memory.json')).read_text())]
 out+=['Restore age/stale counts, fleet/local/too-heavy, named plan stages and closure diagnostics verbatim:',block('\n'.join(selected) or 'No matching service lines observed.')]
 for name,path in x.get('service_full_logs',{}).items():
  full=pathlib.Path(path).read_text(errors='replace').splitlines()
  matches=[e['line'] for e in ev if e['source']==name and re.search(r'graph-plan-too-heavy|request-failed|closure-missing|where:',e['line'])]
  for l in dict.fromkeys(matches):
   inds=[i for i,s in enumerate(full) if s==l]
   if inds:
    i=inds[-1];j=i+1
    while j<len(full) and (full[j].startswith((' ','\t')) or not full[j].strip()):j+=1
    out+=['Service error/refusal with 20 preceding lines and complete following stack:',block('\n'.join(full[max(0,i-20):j]))]
 sp=pathlib.Path(x['p12_samples']);st=sp.read_text(errors='replace')
 peaks=[]
 for l in st.splitlines():
  z=re.search(r'pid=(\d+) rss=(\d+)MB',l)
  if z:peaks.append((int(z[2]),l))
 out+=['P12 sampler peak and progress line:',block(max(peaks,key=lambda t:t[0])[1] if peaks else 'No slice worker was observed. Peak not measured.'),'Complete P12 sampler, including closure counts:',block(st)]
 for sf in sp.parent.glob(sp.name+'.sample-*.txt'):
  text=sf.read_text(errors='replace');p=text.find('Sort by top of stack')
  out+=['Stack sample '+sf.name+':',block(text[:p+4000] if p>=0 else text)]
 samples=read(pre+'-ps.json')
 out+=['Exact 30-second top-three RSS samples:']
 for s in samples:out+=[block(s['at']+' pressure='+s['pressure']+'\n'+s['top3_base_verbatim'])]
 # Entire native log retains task succeeded/failed/not-run lists and first errors with preceding context verbatim.
 out+=['Complete native output: setup task lists, first errors and preceding context are retained verbatim:',block(native)]
 last=x
 ready=last['native_rc']==0 and (R/'.cache/f/native-bootstrap.done').exists()
 failed=[] if ready else ['consent'] if last.get('consent') else ['workspace:setup']
 now=datetime.datetime.now(datetime.timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
 (O/'state.json').write_text(json.dumps({'origin':'g1-m08','round':17,'native_rc':last['native_rc'],'ready':ready,'failed':failed,'at':now},indent=2)+'\n')
(O/'round-17.md').write_text('\n'.join(out))
(O/'round-17.tail.txt').write_bytes(pathlib.Path(a[-1]['log']).read_bytes().splitlines(keepends=True)[-200:] and b''.join(pathlib.Path(a[-1]['log']).read_bytes().splitlines(keepends=True)[-200:]))
print(json.dumps({'event':'per-try-report-written','round':17,'attempts':len(a),'native_rc':a[-1]['native_rc'],'ready':ready}),flush=True)

