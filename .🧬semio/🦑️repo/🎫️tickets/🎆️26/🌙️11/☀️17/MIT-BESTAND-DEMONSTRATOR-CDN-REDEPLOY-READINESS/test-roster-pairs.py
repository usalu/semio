#!/usr/bin/env python3
"""🧪️ Exercise roster orchestration using only synthetic subprocesses."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
from unittest.mock import patch
from types import SimpleNamespace

T = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('roster_pairs', T / 'roster-pairs.py')
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
G = T / '🗑️generated'; G.mkdir(exist_ok=True)
pair_cases = json.loads((T/'🔣️roster-pair-cases.json').read_text())
expected_pairs = pair_cases['established'] + pair_cases['additional']
with tempfile.TemporaryDirectory(prefix='roster-fixture-', dir=G) as directory:
    d = Path(directory); events = d / 'events.jsonl'; confirmed = d / 'confirmed'; confirmed.mkdir(); (confirmed/'demonstrator.json').write_text('old')
    lock_state=d/'lock-state.json'; marker=d/'lock-state.json.lock'; owner=marker/'owner.json'
    cases=json.loads((T/'🔣️roster-lock-cases.json').read_text())
    for row in cases:
        marker.mkdir()
        value={'pid':os.getpid(),'identity':'birth','state':str(lock_state.resolve()),'retry_protocol':1}
        owner.write_text('{' if row['case']=='malformed' else json.dumps(value))
        def status(pid):
            if row['case']=='unknown-status': raise RuntimeError('unknown')
            return row['case']=='dead'
        accepted=False
        try:
            with patch.object(m,'definitely_dead',status):
                with m.coordinator_lock(lock_state): accepted=True
        except RuntimeError: pass
        assert accepted==row['accepted'],row
        if marker.exists(): owner.unlink(); marker.rmdir()
    script='import importlib.util,os,sys,time; from pathlib import Path; s=importlib.util.spec_from_file_location("roster",sys.argv[1]); m=importlib.util.module_from_spec(s); s.loader.exec_module(m); c=m.coordinator_lock(Path(sys.argv[2])); c.__enter__(); print("locked",flush=True); time.sleep(60)'
    child=subprocess.Popen([sys.executable,'-c',script,str(T/'roster-pairs.py'),str(lock_state)],stdout=subprocess.PIPE,text=True)
    try:
        assert child.stdout.readline().strip()=='locked'
        try:
            with m.coordinator_lock(lock_state): raise AssertionError('concurrent owner admitted')
        except RuntimeError: pass
    finally:
        child.kill(); child.wait()
    assert marker.exists()
    with m.coordinator_lock(lock_state):
        assert json.loads(owner.read_text())['pid']==os.getpid()
    assert not marker.exists() and (d/'lock-state.json.lock.advisory').exists()
    modules = d/'modules'; (modules/'🧩️sample').mkdir(parents=True)
    receipt = {'pluginId':'sample','schema':'semio.plugin.component-receipt/v1','target':'wasm32-wasip2','profile':'release','wasmSha256':'1'*64,'semanticDescriptorSha256':'2'*64,'sourceTreeDigest':'3'*64,'provenanceSha256':'4'*64}
    (modules/'🧩️sample'/'🧾️receipt.json').write_text(json.dumps(receipt))
    (confirmed/'sample.json').write_text(json.dumps(receipt))
    assert m.receipt_valid('sample',modules,confirmed)
    receipt['sourceTreeDigest']='5'*64
    (confirmed/'sample.json').write_text(json.dumps(receipt))
    assert not m.receipt_valid('sample',modules,confirmed)
    (confirmed/'sample.json').unlink()
    worker = d / 'worker.py'
    worker.write_text('''import json,os,sys,time\nfrom pathlib import Path\np=Path(sys.argv[1]); d=Path(sys.argv[2]); delay=float(sys.argv[3]); roots=sys.argv[4:]\ndef log(kind):\n with p.open("a") as f:f.write(json.dumps([kind,time.time(),os.getpid(),roots])+"\\n")\nlog("start");time.sleep(delay)\nfor r in roots:\n if r!="broken" or (d/"allow-broken").exists():(d/(r+".json")).write_text("ok")\nlog("end")\n''')
    def spawn(roots, delay):
        return subprocess.Popen([sys.executable, str(worker), str(events), str(confirmed), str(delay), *roots])
    a, b = spawn(['procedural'], 1.0), spawn(['gis'], .12)
    identities = {a.pid: 'Sat Nov 21 00:38:30 2026 bash ./native-queue.sh procedural', b.pid: 'Sat Nov 21 00:38:52 2026 bash ./native-queue.sh gis'}
    def identity(pid):
        if pid in identities:
            proc = a if pid == a.pid else b
            return identities[pid] if proc.poll() is None else ''
        return m.process_identity(pid)
    def launch(roots):
        p = spawn(roots, .12); return p, m.process_identity(p.pid)
    def valid(root): return (confirmed/(root+".json")).exists()
    roster=['procedural','gis','demonstrator','cad','broken'] + [root for pair in expected_pairs for root in reversed(pair)]
    result=m.run_scheduler(roster,d/'state.json',{'procedural':a.pid,'gis':b.pid},launch,valid,identity,lambda _:time.sleep(.015),.015,lambda pid: ('bash ./native-queue.sh procedural' if pid==a.pid else 'bash ./native-queue.sh gis'),lambda module: m.receipt_stamp(module, confirmed))
    assert not result
    state=json.loads((d/'state.json').read_text());assert state['failed']==['broken'] and state['forced_demonstrator'] and not state['active']
    rows=[json.loads(x) for x in events.read_text().splitlines()]; running=0
    for event,_,_,roots in rows:
        running += len(roots) if event=='start' else -len(roots)
        assert 0 <= running <= 2, (running,event,roots)
    starts=[r for r in rows if r[0]=='start']; names=[r[3] for r in starts]
    assert ['demonstrator'] in names and names.index(['demonstrator']) < names.index(['cad'])
    actual_pairs = [roots for roots in names if len(roots) == 2]
    assert actual_pairs == expected_pairs, (actual_pairs, expected_pairs)
    assert names.index(['flow-extension-text','flow-extension-math']) < names.index(['flow','playbook'])
    assert next(x[1] for x in rows if x[0]=='end' and x[3]==['procedural']) < next(x[1] for x in starts if x[3]==['cad'])
    assert len([x for x in names if x==['demonstrator']])==1
    (confirmed/'broken.json').write_text('stale')
    (confirmed/'allow-broken').write_text('ok')
    assert m.run_scheduler(roster,d/'state.json',{},launch,valid,identity,lambda _:time.sleep(.015),.015,lambda pid: ('bash ./native-queue.sh procedural' if pid==a.pid else 'bash ./native-queue.sh gis'),lambda module: m.receipt_stamp(module, confirmed))
    assert len([json.loads(x) for x in events.read_text().splitlines() if json.loads(x)[0]=='start' and json.loads(x)[3]==['demonstrator']])==1
    assert len([json.loads(x) for x in events.read_text().splitlines() if json.loads(x)[0]=='start' and json.loads(x)[3]==['broken']])==2
    for file in confirmed.iterdir(): file.unlink()
    events.write_text('')
    heavy_roots=['cad','procedural','independent']
    assert m.run_scheduler(heavy_roots,d/'two-large-roots.json',{},launch,valid,sleep=lambda _:time.sleep(.015),stamp=lambda module:m.receipt_stamp(module,confirmed))
    rows=[json.loads(line) for line in events.read_text().splitlines()]
    started={row[3][0]:row[1] for row in rows if row[0]=='start'}
    ended={row[3][0]:row[1] for row in rows if row[0]=='end'}
    assert started['procedural'] < ended['procedural'] <= started['cad'], 'large roots overlapped or deferred Generator'
    assert started['independent'] < ended['procedural'], 'spare root slot unused'
    for file in confirmed.iterdir(): file.unlink()
    events.write_text('')
    pair_first=['independent','flow-extension-text','flow-extension-math']
    assert m.run_scheduler(pair_first,d/'pair-first.json',{},launch,valid,identity,lambda _:time.sleep(.015),.015,lambda pid: ('bash ./native-queue.sh procedural' if pid==a.pid else 'bash ./native-queue.sh gis'),lambda module: m.receipt_stamp(module, confirmed))
    starts=[json.loads(line)[3] for line in events.read_text().splitlines() if json.loads(line)[0]=='start']
    assert starts[0]==['flow-extension-text','flow-extension-math'] and starts[1]==['independent'], starts
    for file in confirmed.iterdir(): file.unlink()
    events.write_text('')
    split = pair_cases['oneConfirmed']
    (confirmed/(split['confirmed'] + '.json')).write_text('ok')
    assert m.run_scheduler([split['confirmed'],split['pending']], d/'split-new-pair.json', {}, launch, valid, sleep=lambda _:time.sleep(.015), stamp=lambda module:m.receipt_stamp(module,confirmed))
    starts=[row[3] for row in map(json.loads, events.read_text().splitlines()) if row[0]=='start']
    assert starts == [[split['pending']]], starts
    for file in confirmed.iterdir(): file.unlink()
    events.write_text('')
    retry_state=d/'request-state.json'; lock=retry_state.with_name(retry_state.name+'.lock'); lock.mkdir()
    (lock/'owner.json').write_text(json.dumps({'pid':777,'identity':'owner-birth','state':str(retry_state.resolve()),'retry_protocol':1}))
    def request_identity(pid): return 'owner-birth' if pid==777 else ''
    def request_command(pid): return 'python3 /ticket/roster-pairs.py' if pid==777 else ''
    assert m.request_retry(retry_state,'procedural',['procedural'],request_identity,request_command)
    assert not m.request_retry(retry_state,'procedural',['procedural'],request_identity,request_command)
    try:
        m.request_retry(retry_state,'not-roster',['procedural'],request_identity,request_command)
        raise AssertionError('unknown retry accepted')
    except ValueError: pass
    try:
        m.request_retry(retry_state,'procedural',['procedural'],lambda _: 'reused',request_command)
        raise AssertionError('reused coordinator accepted')
    except RuntimeError: pass
    predecessor=spawn(['procedural'],.25)
    def retry_identity(pid): return 'pred-birth' if pid==predecessor.pid and predecessor.poll() is None else m.process_identity(pid) if pid!=predecessor.pid else ''
    def retry_command(pid): return 'bash ./native-queue.sh procedural' if pid==predecessor.pid else ''
    assert m.run_scheduler(['procedural'],retry_state,{'procedural':predecessor.pid},launch,valid,retry_identity,lambda _:time.sleep(.015),.015,retry_command,lambda module:m.receipt_stamp(module,confirmed))
    rows=[json.loads(line) for line in events.read_text().splitlines()]
    starts=[row for row in rows if row[0]=='start' and row[3]==['procedural']]
    ends=[row for row in rows if row[0]=='end' and row[3]==['procedural']]
    assert len(starts)==2 and len(ends)==2 and ends[0][1]<starts[1][1], rows
    assert not (m.request_directory(retry_state)/'procedural').exists()
    assert m.queue_command_matches('bash ./native-queue.sh procedural',['procedural'])
    assert m.queue_command_matches(r'"C:\Program Files\Git\bin\bash.exe" "C:\work folder\native-queue.sh" process',['process'])
    assert not m.queue_command_matches('bash ./native-queue.sh process',['procedural'])
    assert not m.queue_command_matches('bash -c "echo /tmp/native-queue.sh procedural"',['procedural'])
    try:
        m.run_scheduler(['procedural'],d/'reused.json',{'procedural':123},lambda roots: (_ for _ in ()).throw(AssertionError('must not launch')),lambda root:False,lambda pid:'same-start',lambda _:None,.01,lambda pid:'bash ./native-queue.sh process')
        raise AssertionError('reused predecessor PID accepted')
    except RuntimeError as error:
        assert 'identity does not match' in str(error)
    (d/'recovered.json').write_text(json.dumps({'version':1,'forced_demonstrator':False,'failed':[],'active':[{'roots':['procedural'],'pid':123,'identity':'same-start','predecessor':True,'before':{'procedural':None}}]}))
    try:
        m.run_scheduler(['procedural'],d/'recovered.json',{},lambda roots: (_ for _ in ()).throw(AssertionError('must not launch')),lambda root:False,lambda pid:'same-start',lambda _:None,.01,lambda pid:'bash ./native-queue.sh process')
        raise AssertionError('reused recovered PID accepted')
    except RuntimeError as error:
        assert 'identity or command changed' in str(error)
    response=SimpleNamespace(returncode=0,stdout=json.dumps({'identity':'2026-11-21T09:00:00.0000000Z','command':r'bash C:\repo\native-queue.sh process'}),stderr='')
    with patch.object(m.subprocess,'run',return_value=response) as query:
        assert m.windows_process(123)==('2026-11-21T09:00:00.0000000Z',r'bash C:\repo\native-queue.sh process')
        assert 'ProcessId = 123' in query.call_args.args[0][-1]
    with patch.object(m.subprocess,'run',return_value=SimpleNamespace(returncode=0,stdout='null',stderr='')):
        assert m.windows_process(123)==('','')
    with patch.object(m.subprocess,'run',return_value=SimpleNamespace(returncode=1,stdout='',stderr='denied')):
        try: m.windows_process(123); raise AssertionError('query failure accepted')
        except RuntimeError: pass
    print('synthetic predecessor, capacity, priority, all pairs, confirmed sibling, pair preference, failure, resume, Windows query, PID reuse and queued active retry checks passed')
