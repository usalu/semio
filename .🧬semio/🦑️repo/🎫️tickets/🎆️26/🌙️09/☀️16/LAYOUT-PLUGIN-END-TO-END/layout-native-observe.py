import pathlib,json,sys
ticket=pathlib.Path(__file__).resolve().parent
native=sys.argv[1] if len(sys.argv)>1 else '8'
assert native in ['8','9','10','11']
spec=json.loads((ticket/('layout-native-'+native+'-command.json')).read_text())
log=pathlib.Path(spec['log'])
statefile=log.parent/'observation.json'
state=json.loads(statefile.read_text()) if statefile.exists() else {'offset':0,'events':[]}
with log.open('rb') as stream:
 stream.seek(state['offset']); data=stream.read(); complete,_,partial=data.rpartition(b'\n'); state['offset']+=len(complete)+1 if b'\n' in data else 0
lines=complete.decode('utf-8',errors='replace').splitlines() if b'\n' in data else []
events=[line for line in lines if line.startswith('error') or line.startswith('ENOSPC:') or line.startswith('rustc-LLVM ERROR:') or 'test result:' in line or 'Running unittests' in line or (line.startswith('   Compiling ') and line.split()[1] in spec['packages']+['semio-s-artifact-layout-layout-diff']) or '[owned-layout-' in line or '[media-handoff]' in line or '[media-child]' in line]
state['events'].extend(events)
statefile.write_text(json.dumps(state,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'bytesRead':len(data),'offset':state['offset'],'events':events,'latestCompile':next((line for line in reversed(lines) if line.startswith('   Compiling ')),None),'latestHeartbeat':next((line for line in reversed(lines) if 'still running after' in line),None),'latestAdmission':next((line for line in reversed(lines) if line.startswith('[capacity]') or line.startswith('[repo cargo]') or 'cargo:source-admission:' in line or 'Waiting for access to native-fixed-image:' in line),None)},ensure_ascii=False))
