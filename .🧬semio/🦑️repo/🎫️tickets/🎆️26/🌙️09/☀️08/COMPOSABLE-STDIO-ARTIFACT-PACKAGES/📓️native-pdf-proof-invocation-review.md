# Native PDF Proof Invocation Review

The original loop ended at PDF27 with a shared-source compiler error before any output move. Its terminal recovery handler executed successfully and the200-file PDF26 publication remains byte-exact. The original pre-move journal and recovery handler below are retained as evidence. Future execution uses only the guarded35–38 loop, then30–32 source probes and the restored consumer. These commands are audit inputs; native local cache/restoration/source/consumer acceptance remains pending.

Future moves persist and fsync a prepared owner/nonce/baseline receipt before rename, then restore exact owned output on failure. Run receipts must be fresh. Source marker cleanup preserves concurrent writes and replacement inodes. Both the source probe and consumer bind their starting inventory to the preceding accepted restored inventory. The helper validations passed; they do not replace native proof.

## Guarded Stable Recovery (Stages35–38, Pending)

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib,json,os,stat,uuid,shutil
root=Path.cwd()
ticket=root/'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES'
gen=ticket/'🗑️generated'
owner='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/Cargo.toml'
output=root/Path(owner).parent/'dist/build'
def inventory(directory=None):
 directory=directory or output
 for ancestor in [directory,*directory.parents]:
  if ancestor==root: break
  assert not ancestor.is_symlink(),str(ancestor)
 assert directory.is_dir(),str(directory)
 marker=json.loads((directory/'.nx-artifact.json').read_text())
 assert marker['version']==1 and marker['owner']==owner,marker
 files={}
 for p in sorted(directory.rglob('*')):
  assert not p.is_symlink(),str(p)
  if p.is_dir(): continue
  assert stat.S_ISREG(p.stat().st_mode),str(p)
  files[p.relative_to(directory).as_posix()]={'size':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
 assert sorted(marker['files'])==sorted(set(files)-{'.nx-artifact.json'})
 return {'owner':owner,'files':files,'fileCount':len(files),'totalBytes':sum(f['size'] for f in files.values())}
def capture(stage):
 run=json.loads((gen/'nx-cache-root-pdf-native-proof/run.json').read_text())
 tasks=[t for t in run['tasks'] if t['taskId']=='@semio-tech/stdio-pdf-rs:build']
 assert len(tasks)==1 and tasks[0]['status']==0,tasks
 assert all(t['status']==0 for t in run['tasks']),run
 receipt=inventory()
 (gen/f'pdf-native-cache-{stage}-run.json').write_text(json.dumps(run,ensure_ascii=False,indent=2)+'\n')
 (gen/f'pdf-native-baseline-{stage}-inventory.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n')
 print(json.dumps({'stage':stage,'task':tasks[0],'fileCount':receipt['fileCount'],'totalBytes':receipt['totalBytes']},ensure_ascii=False),flush=True)
 return tasks[0],receipt
def move_owned(stage,baseline):
 lease=output.with_name(output.name+'.lease')
 nonce=uuid.uuid4().hex
 lease_bytes=json.dumps({'owner':owner,'pid':os.getpid(),'nonce':nonce}).encode()
 with lease.open('xb') as f:f.write(lease_bytes);f.flush();os.fsync(f.fileno())
 backup=gen/f'pdf-native-output-backup-{stage}-{nonce}'
 try:
  assert inventory()==baseline,'published PDF closure differs from baseline'
  assert not backup.exists()
  receipt={'owner':owner,'backup':str(backup),'nonce':nonce,'baseline':baseline,'phase':'prepared'}
  receipt_path=gen/f'pdf-native-output-move-{stage}.json'
  with receipt_path.open('x') as f:
   json.dump(receipt,f,ensure_ascii=False,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
  directory_fd=os.open(gen,os.O_RDONLY)
  try:os.fsync(directory_fd)
  finally:os.close(directory_fd)
  output.rename(backup)
  assert inventory(backup)==baseline,'owned backup inventory changed during move'
  print(json.dumps({'moved':str(backup),'fileCount':baseline['fileCount'],'preparedReceipt':str(receipt_path)}),flush=True)
  return backup
 except BaseException:
  if backup.exists() and not output.exists() and not output.is_symlink():
   assert inventory(backup)==baseline,'refusing changed backup during immediate move recovery'
   backup.rename(output)
   assert inventory()==baseline
  raise
 finally:
  assert lease.read_bytes()==lease_bytes,'foreign lease replacement'
  lease.unlink()

def recover_owned_output(stage,backup,baseline):
 receipt=json.loads((gen/f'pdf-native-output-move-{stage}.json').read_text())
 assert receipt['owner']==owner and receipt['backup']==str(backup) and receipt['baseline']==baseline
 assert backup.parent==gen and backup.name==f"pdf-native-output-backup-{stage}-{receipt['nonce']}"
 assert inventory(backup)==baseline,'owned backup inventory changed'
 lease=output.with_name(output.name+'.lease')
 token=json.dumps({'owner':owner,'pid':os.getpid(),'nonce':uuid.uuid4().hex}).encode()
 with lease.open('xb') as f:f.write(token)
 try:
  if output.exists() or output.is_symlink():
   action='preserved-existing-output-and-owned-backup'
  else:
   backup.rename(output)
   assert inventory()==baseline,'failure recovery did not restore exact baseline'
   action='restored-exact-owned-backup'
  (gen/f'pdf-native-output-recovery-{stage}.json').write_text(json.dumps({'stage':stage,'action':action,'backup':str(backup),'owner':owner},ensure_ascii=False,indent=2)+'\n')
  print('[DEBUG] PDF output recovery '+action,flush=True)
 finally:
  assert lease.read_bytes()==token,'foreign recovery lease replacement'
  lease.unlink()

import subprocess,time,signal
env=os.environ.copy()
env.update(json.loads("{\"NX_DAEMON\":\"false\",\"NX_ISOLATE_PLUGINS\":\"false\",\"NX_WORKSPACE_DATA_DIRECTORY\":\"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/nx-root-pdf-native-proof\",\"NX_CACHE_DIRECTORY\":\"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/nx-cache-root-pdf-native-proof\",\"CARGO_TARGET_DIR\":\"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/cargo\",\"CARGO_BUILD_JOBS\":\"2\",\"CARGO_INCREMENTAL\":\"0\",\"CARGO_NET_OFFLINE\":\"true\",\"SEMIO_TEST_ARTIFACT_DIR\":\"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/pdf-native-proof-artifacts\",\"NX_NO_CLOUD\":\"true\"}"))
env['SEMIO_BUILD_BUDGET_MS']='0'
def run(stage):
 log=gen/f'pdf-native-cache-{stage}.txt'
 command=['bun','x','nx','run','@semio-tech/stdio-pdf-rs:build','--output-style=stream','--verbose']
 print(f'[DEBUG] PDF proof {stage} started',flush=True)
 receipt_path=gen/'nx-cache-root-pdf-native-proof/run.json'
 before_receipt=receipt_path.stat() if receipt_path.exists() else None
 started_ns=time.time_ns()
 with log.open('w') as target:
  process=subprocess.Popen(command,cwd=root,env=env,stdout=target,stderr=subprocess.STDOUT,start_new_session=True)
  try:
   while True:
    try:
     code=process.wait(timeout=30)
     break
    except subprocess.TimeoutExpired:
     print(f'[DEBUG] PDF proof {stage} still active pid={process.pid}',flush=True)
  except BaseException:
   os.killpg(process.pid,signal.SIGTERM)
   process.wait()
   raise
 assert code==0,f'PDF proof {stage} failed exit={code}; see {log}'
 current_receipt=receipt_path.stat()
 assert current_receipt.st_mtime_ns>=started_ns,'Nx run receipt predates invocation'
 assert before_receipt is None or (current_receipt.st_ino,current_receipt.st_mtime_ns)!=(before_receipt.st_ino,before_receipt.st_mtime_ns),'Nx run receipt was not refreshed'
 (gen/f'pdf-native-receipt-freshness-{stage}.json').write_text(json.dumps({'startedNs':started_ns,'receiptMtimeNs':current_receipt.st_mtime_ns,'receiptInode':current_receipt.st_ino})+'\n')
 return capture(stage)

for stage in ['35-qualified-owner-recovery-local','36-unchanged-local','37-unchanged-local']:
 task,baseline=run(stage)
 if task['cacheStatus']!='local-cache-hit':
  continue
 backup=move_owned('38-restored-local',baseline)
 try:
  restored_task,restored=run('38-restored-local')
  assert restored_task['hash']==task['hash'],'inputs changed during guarded restoration'
  assert restored_task['cacheStatus']=='local-cache-hit','guarded restoration did not use local cache'
  assert restored==baseline,'guarded restoration changed published files'
 except BaseException:
  recover_owned_output('38-restored-local',backup,baseline)
  raise
 shutil.rmtree(backup)
 proof={'baselineTask':task,'restoredTask':restored_task,'fileCount':restored['fileCount'],'totalBytes':restored['totalBytes'],'exactRestoration':True,'baselineInventoryFile':f'pdf-native-baseline-{stage}-inventory.json','restoredInventoryFile':'pdf-native-baseline-38-restored-local-inventory.json','guardedMove':True,'freshInvocationReceipts':True}
 (gen/'pdf-native-local-restoration-proof.json').write_text(json.dumps(proof,ensure_ascii=False,indent=2)+'\n')
 print('[DEBUG] fresh local PDF reuse and exact guarded output restoration passed',flush=True)
 break
else:
 raise RuntimeError('PDF source hashes changed across all guarded repeats; stable cache hit not established')

PY
```

## Terminal Recovery For The Completed Original Loop (Already Executed)

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib,json,os,stat,uuid,shutil
root=Path.cwd()
ticket=root/'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES'
gen=ticket/'🗑️generated'
owner='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/Cargo.toml'
output=root/Path(owner).parent/'dist/build'
def inventory(directory=None):
 directory=directory or output
 for ancestor in [directory,*directory.parents]:
  if ancestor==root: break
  assert not ancestor.is_symlink(),str(ancestor)
 assert directory.is_dir(),str(directory)
 marker=json.loads((directory/'.nx-artifact.json').read_text())
 assert marker['version']==1 and marker['owner']==owner,marker
 files={}
 for p in sorted(directory.rglob('*')):
  assert not p.is_symlink(),str(p)
  if p.is_dir(): continue
  assert stat.S_ISREG(p.stat().st_mode),str(p)
  files[p.relative_to(directory).as_posix()]={'size':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
 assert sorted(marker['files'])==sorted(set(files)-{'.nx-artifact.json'})
 return {'owner':owner,'files':files,'fileCount':len(files),'totalBytes':sum(f['size'] for f in files.values())}
def capture(stage):
 run=json.loads((gen/'nx-cache-root-pdf-native-proof/run.json').read_text())
 tasks=[t for t in run['tasks'] if t['taskId']=='@semio-tech/stdio-pdf-rs:build']
 assert len(tasks)==1 and tasks[0]['status']==0,tasks
 assert all(t['status']==0 for t in run['tasks']),run
 receipt=inventory()
 (gen/f'pdf-native-cache-{stage}-run.json').write_text(json.dumps(run,ensure_ascii=False,indent=2)+'\n')
 (gen/f'pdf-native-baseline-{stage}-inventory.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n')
 print(json.dumps({'stage':stage,'task':tasks[0],'fileCount':receipt['fileCount'],'totalBytes':receipt['totalBytes']},ensure_ascii=False),flush=True)
 return tasks[0],receipt
def move_owned(stage,baseline):
 lease=output.with_name(output.name+'.lease')
 nonce=uuid.uuid4().hex
 lease_bytes=json.dumps({'owner':owner,'pid':os.getpid(),'nonce':nonce}).encode()
 with lease.open('xb') as f:f.write(lease_bytes);f.flush();os.fsync(f.fileno())
 backup=gen/f'pdf-native-output-backup-{stage}-{nonce}'
 try:
  assert inventory()==baseline,'published PDF closure differs from baseline'
  assert not backup.exists()
  receipt={'owner':owner,'backup':str(backup),'nonce':nonce,'baseline':baseline,'phase':'prepared'}
  receipt_path=gen/f'pdf-native-output-move-{stage}.json'
  with receipt_path.open('x') as f:
   json.dump(receipt,f,ensure_ascii=False,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
  directory_fd=os.open(gen,os.O_RDONLY)
  try:os.fsync(directory_fd)
  finally:os.close(directory_fd)
  output.rename(backup)
  assert inventory(backup)==baseline,'owned backup inventory changed during move'
  print(json.dumps({'moved':str(backup),'fileCount':baseline['fileCount'],'preparedReceipt':str(receipt_path)}),flush=True)
  return backup
 except BaseException:
  if backup.exists() and not output.exists() and not output.is_symlink():
   assert inventory(backup)==baseline,'refusing changed backup during immediate move recovery'
   backup.rename(output)
   assert inventory()==baseline
  raise
 finally:
  assert lease.read_bytes()==lease_bytes,'foreign lease replacement'
  lease.unlink()


def recover_owned_output(stage,backup,baseline):
 receipt=json.loads((gen/f'pdf-native-output-move-{stage}.json').read_text())
 assert receipt['owner']==owner and receipt['backup']==str(backup) and receipt['baseline']==baseline
 assert backup.parent==gen and backup.name==f"pdf-native-output-backup-{stage}-{receipt['nonce']}"
 assert inventory(backup)==baseline,'owned backup inventory changed'
 lease=output.with_name(output.name+'.lease')
 token=json.dumps({'owner':owner,'pid':os.getpid(),'nonce':uuid.uuid4().hex}).encode()
 with lease.open('xb') as f:f.write(token)
 try:
  if output.exists() or output.is_symlink():
   action='preserved-existing-output-and-owned-backup'
  else:
   backup.rename(output)
   assert inventory()==baseline,'failure recovery did not restore exact baseline'
   action='restored-exact-owned-backup'
  (gen/f'pdf-native-output-recovery-{stage}.json').write_text(json.dumps({'stage':stage,'action':action,'backup':str(backup),'owner':owner},ensure_ascii=False,indent=2)+'\n')
  print('[DEBUG] PDF output recovery '+action,flush=True)
 finally:
  assert lease.read_bytes()==token,'foreign recovery lease replacement'
  lease.unlink()

stage='29-restored-local'
receipt_path=gen/f'pdf-native-output-move-{stage}.json'
receipt_valid=False
if receipt_path.exists():
 try:
  existing=json.loads(receipt_path.read_text())
  nonce=existing['nonce']
  receipt_valid=existing['owner']==owner and isinstance(existing['baseline'],dict) and isinstance(nonce,str) and len(nonce)==32 and all(c in '0123456789abcdef' for c in nonce) and existing['backup']==str(gen/f'pdf-native-output-backup-{stage}-{nonce}')
  if receipt_valid and Path(existing['backup']).exists():
   receipt_valid=inventory(Path(existing['backup']))==existing['baseline']
 except (OSError,ValueError,KeyError,TypeError):
  receipt_valid=False
if not receipt_valid:
 journal=json.loads((gen/'pdf-native-original-loop-recovery-journal.json').read_text())
 assert journal['owner']==owner and journal['stage']==stage
 candidates=list(gen.glob(journal['backupPrefix']+'*'))
 assert len(candidates)<=1,'ambiguous stage29 backups require scoped recovery'
 if candidates:
  backup=candidates[0]
  nonce=backup.name.removeprefix(journal['backupPrefix'])
  assert len(nonce)==32 and all(c in '0123456789abcdef' for c in nonce)
  actual=inventory(backup)
  matches=[]
  for baseline_path in gen.glob('pdf-native-baseline-*-inventory.json'):
   baseline=json.loads(baseline_path.read_text())
   if actual!=baseline:continue
   label=baseline_path.name.removeprefix('pdf-native-baseline-').removesuffix('-inventory.json')
   run_path=gen/f'pdf-native-cache-{label}-run.json'
   if not run_path.exists():continue
   run=json.loads(run_path.read_text())
   if all(t['status']==0 for t in run['tasks']):matches.append(label)
  assert matches,'backup has no exact successful accepted inventory'
  receipt={'owner':owner,'backup':str(backup),'nonce':nonce,'baseline':actual,'reconstructedFromJournal':True,'matchingAcceptedStages':matches}
  replacement=gen/f'pdf-native-output-move-{stage}-reconstructed-{uuid.uuid4().hex}.json'
  with replacement.open('x') as f:json.dump(receipt,f,ensure_ascii=False,indent=2);f.flush();os.fsync(f.fileno())
  if receipt_path.exists():
   damaged=gen/f'pdf-native-output-move-{stage}-damaged-{uuid.uuid4().hex}.json'
   receipt_path.rename(damaged)
  replacement.rename(receipt_path)
  receipt_valid=True
if receipt_valid:
 receipt=json.loads(receipt_path.read_text())
 backup=Path(receipt['backup'])
 if backup.exists():recover_owned_output(stage,backup,receipt['baseline'])
 else:
  assert output.is_dir(),'backup and output both absent'
  if not (gen/'pdf-native-local-restoration-proof.json').is_file():
   assert inventory()==receipt['baseline'],'backup absent without exact restored publication'
else:
 assert output.is_dir() and not output.is_symlink(),'publication absent without a uniquely recoverable owned backup'
 print('[DEBUG] no moved PDF publication needs terminal recovery',flush=True)

PY
```

## Freshness Verification For Already-Running Loop Receipts

```sh
python3 - <<'PY'
from pathlib import Path
import json
from datetime import datetime,timezone
p=Path('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated')
proof=json.loads((p/'pdf-native-local-restoration-proof.json').read_text())
assert proof['exactRestoration'] is True
checks=[]
for field in ['baselineTask','restoredTask']:
 task=proof[field]
 assert task['status']==0 and task['cacheStatus']=='local-cache-hit'
 matches=[]
 for receipt in p.glob('pdf-native-cache-*-run.json'):
  data=json.loads(receipt.read_text())
  if not any(t==task for t in data.get('tasks',[])):continue
  stage=receipt.name.removeprefix('pdf-native-cache-').removesuffix('-run.json')
  raw=p/f'pdf-native-cache-{stage}.txt'
  if not raw.exists():continue
  start=datetime.fromisoformat(task['startTime'].replace('Z','+00:00')).timestamp()
  end=datetime.fromisoformat(task['endTime'].replace('Z','+00:00')).timestamp()
  birth=raw.stat().st_birthtime
  assert start>=birth-0.002,('stale task receipt',stage,start,birth)
  assert end>=start and receipt.stat().st_mtime>=end-0.002
  assert 'Successfully ran target build' in raw.read_text(errors='replace'),stage
  matches.append({'stage':stage,'taskStartTime':task['startTime'],'taskEndTime':task['endTime'],'rawBirthTime':birth,'capturedReceiptMtime':receipt.stat().st_mtime})
 assert matches,(field,'no fresh stage receipt')
 checks.append({'field':field,'matches':matches})
(p/'pdf-native-local-restoration-receipt-freshness.json').write_text(json.dumps({'capturedAt':datetime.now(timezone.utc).isoformat(),'checks':checks,'fresh':True},indent=2)+'\n')
print('[DEBUG] PDF local hit and restoration task receipts correspond to their own fresh invocation logs')
PY
```

## Own-Source And Sibling-Source Isolation (Guarded, Pending)

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib,json,os,stat,uuid,shutil
root=Path.cwd()
ticket=root/'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES'
gen=ticket/'🗑️generated'
owner='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/Cargo.toml'
output=root/Path(owner).parent/'dist/build'
def inventory(directory=None):
 directory=directory or output
 for ancestor in [directory,*directory.parents]:
  if ancestor==root: break
  assert not ancestor.is_symlink(),str(ancestor)
 assert directory.is_dir(),str(directory)
 marker=json.loads((directory/'.nx-artifact.json').read_text())
 assert marker['version']==1 and marker['owner']==owner,marker
 files={}
 for p in sorted(directory.rglob('*')):
  assert not p.is_symlink(),str(p)
  if p.is_dir(): continue
  assert stat.S_ISREG(p.stat().st_mode),str(p)
  files[p.relative_to(directory).as_posix()]={'size':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
 assert sorted(marker['files'])==sorted(set(files)-{'.nx-artifact.json'})
 return {'owner':owner,'files':files,'fileCount':len(files),'totalBytes':sum(f['size'] for f in files.values())}
def capture(stage):
 run=json.loads((gen/'nx-cache-root-pdf-native-proof/run.json').read_text())
 tasks=[t for t in run['tasks'] if t['taskId']=='@semio-tech/stdio-pdf-rs:build']
 assert len(tasks)==1 and tasks[0]['status']==0,tasks
 assert all(t['status']==0 for t in run['tasks']),run
 receipt=inventory()
 (gen/f'pdf-native-cache-{stage}-run.json').write_text(json.dumps(run,ensure_ascii=False,indent=2)+'\n')
 (gen/f'pdf-native-baseline-{stage}-inventory.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n')
 print(json.dumps({'stage':stage,'task':tasks[0],'fileCount':receipt['fileCount'],'totalBytes':receipt['totalBytes']},ensure_ascii=False),flush=True)
 return tasks[0],receipt
def move_owned(stage,baseline):
 lease=output.with_name(output.name+'.lease')
 nonce=uuid.uuid4().hex
 lease_bytes=json.dumps({'owner':owner,'pid':os.getpid(),'nonce':nonce}).encode()
 with lease.open('xb') as f:f.write(lease_bytes);f.flush();os.fsync(f.fileno())
 backup=gen/f'pdf-native-output-backup-{stage}-{nonce}'
 try:
  assert inventory()==baseline,'published PDF closure differs from baseline'
  assert not backup.exists()
  receipt={'owner':owner,'backup':str(backup),'nonce':nonce,'baseline':baseline,'phase':'prepared'}
  receipt_path=gen/f'pdf-native-output-move-{stage}.json'
  with receipt_path.open('x') as f:
   json.dump(receipt,f,ensure_ascii=False,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
  directory_fd=os.open(gen,os.O_RDONLY)
  try:os.fsync(directory_fd)
  finally:os.close(directory_fd)
  output.rename(backup)
  assert inventory(backup)==baseline,'owned backup inventory changed during move'
  print(json.dumps({'moved':str(backup),'fileCount':baseline['fileCount'],'preparedReceipt':str(receipt_path)}),flush=True)
  return backup
 except BaseException:
  if backup.exists() and not output.exists() and not output.is_symlink():
   assert inventory(backup)==baseline,'refusing changed backup during immediate move recovery'
   backup.rename(output)
   assert inventory()==baseline
  raise
 finally:
  assert lease.read_bytes()==lease_bytes,'foreign lease replacement'
  lease.unlink()

def recover_owned_output(stage,backup,baseline):
 receipt=json.loads((gen/f'pdf-native-output-move-{stage}.json').read_text())
 assert receipt['owner']==owner and receipt['backup']==str(backup) and receipt['baseline']==baseline
 assert backup.parent==gen and backup.name==f"pdf-native-output-backup-{stage}-{receipt['nonce']}"
 assert inventory(backup)==baseline,'owned backup inventory changed'
 lease=output.with_name(output.name+'.lease')
 token=json.dumps({'owner':owner,'pid':os.getpid(),'nonce':uuid.uuid4().hex}).encode()
 with lease.open('xb') as f:f.write(token)
 try:
  if output.exists() or output.is_symlink():
   action='preserved-existing-output-and-owned-backup'
  else:
   backup.rename(output)
   assert inventory()==baseline,'failure recovery did not restore exact baseline'
   action='restored-exact-owned-backup'
  (gen/f'pdf-native-output-recovery-{stage}.json').write_text(json.dumps({'stage':stage,'action':action,'backup':str(backup),'owner':owner},ensure_ascii=False,indent=2)+'\n')
  print('[DEBUG] PDF output recovery '+action,flush=True)
 finally:
  assert lease.read_bytes()==token,'foreign recovery lease replacement'
  lease.unlink()

import subprocess,time,signal
env=os.environ.copy()
env.update(json.loads("{\"NX_DAEMON\":\"false\",\"NX_ISOLATE_PLUGINS\":\"false\",\"NX_WORKSPACE_DATA_DIRECTORY\":\"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/nx-root-pdf-native-proof\",\"NX_CACHE_DIRECTORY\":\"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/nx-cache-root-pdf-native-proof\",\"CARGO_TARGET_DIR\":\"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/cargo\",\"CARGO_BUILD_JOBS\":\"2\",\"CARGO_INCREMENTAL\":\"0\",\"CARGO_NET_OFFLINE\":\"true\",\"SEMIO_TEST_ARTIFACT_DIR\":\"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/pdf-native-proof-artifacts\",\"NX_NO_CLOUD\":\"true\"}"))
env['SEMIO_BUILD_BUDGET_MS']='0'
def run(stage):
 log=gen/f'pdf-native-cache-{stage}.txt'
 command=['bun','x','nx','run','@semio-tech/stdio-pdf-rs:build','--output-style=stream','--verbose']
 print(f'[DEBUG] PDF proof {stage} started',flush=True)
 receipt_path=gen/'nx-cache-root-pdf-native-proof/run.json'
 before_receipt=receipt_path.stat() if receipt_path.exists() else None
 started_ns=time.time_ns()
 with log.open('w') as target:
  process=subprocess.Popen(command,cwd=root,env=env,stdout=target,stderr=subprocess.STDOUT,start_new_session=True)
  try:
   while True:
    try:
     code=process.wait(timeout=30)
     break
    except subprocess.TimeoutExpired:
     print(f'[DEBUG] PDF proof {stage} still active pid={process.pid}',flush=True)
  except BaseException:
   os.killpg(process.pid,signal.SIGTERM)
   process.wait()
   raise
 assert code==0,f'PDF proof {stage} failed exit={code}; see {log}'
 current_receipt=receipt_path.stat()
 assert current_receipt.st_mtime_ns>=started_ns,'Nx run receipt predates invocation'
 assert before_receipt is None or (current_receipt.st_ino,current_receipt.st_mtime_ns)!=(before_receipt.st_ino,before_receipt.st_mtime_ns),'Nx run receipt was not refreshed'
 (gen/f'pdf-native-receipt-freshness-{stage}.json').write_text(json.dumps({'startedNs':started_ns,'receiptMtimeNs':current_receipt.st_mtime_ns,'receiptInode':current_receipt.st_ino})+'\n')
 return capture(stage)

proof=json.loads((gen/'pdf-native-local-restoration-proof.json').read_text())
baseline_task=proof['restoredTask']
baseline=inventory()
assert baseline==json.loads((gen/proof['restoredInventoryFile']).read_text()),'publication changed after guarded restoration proof'
pdf_source=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs'
jpg_source=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🦀️.rs'
def append_probe(source,label):
 assert source.is_file() and not source.is_symlink()
 identity=source.stat()
 original=source.read_bytes()
 marker=('\n// [DEBUG] native Nx '+label+' source isolation '+uuid.uuid4().hex+'\n').encode()
 assert marker not in original
 lease=source.with_name(source.name+'.native-nx-probe.lease')
 token=json.dumps({'pid':os.getpid(),'nonce':uuid.uuid4().hex}).encode()
 with lease.open('xb') as f:f.write(token)
 try:
  current=source.stat()
  assert (current.st_dev,current.st_ino)==(identity.st_dev,identity.st_ino) and source.read_bytes()==original
  with source.open('ab') as f:
   opened=os.fstat(f.fileno())
   assert (opened.st_dev,opened.st_ino)==(identity.st_dev,identity.st_ino)
   f.write(marker);f.flush();os.fsync(f.fileno())
  assert source.read_bytes()==original+marker,'source changed during owned marker insertion'
 except BaseException as error:
  (gen/f'pdf-native-{label}-marker-recovery-required.json').write_text(json.dumps({'path':str(source),'marker':marker.decode(),'reason':str(error)},ensure_ascii=False,indent=2)+'\n')
  if lease.read_bytes()==token:lease.unlink()
  raise
 probe={'source':source,'label':label,'marker':marker,'original':original,'dev':identity.st_dev,'ino':identity.st_ino,'lease':lease,'token':token}
 (gen/f'pdf-native-{label}-probe.json').write_text(json.dumps({'path':str(source),'marker':marker.decode(),'beforeSha256':hashlib.sha256(original).hexdigest(),'dev':identity.st_dev,'ino':identity.st_ino},ensure_ascii=False,indent=2)+'\n')
 return probe
def remove_probe(source,probe):
 assert source==probe['source']
 lease=probe['lease'];token=probe['token']
 assert lease.read_bytes()==token,'foreign source probe lease replacement'
 try:
  assert source.is_file() and not source.is_symlink(),'source identity changed'
  current=source.stat()
  assert (current.st_dev,current.st_ino)==(probe['dev'],probe['ino']),'source inode changed'
  assert source.read_bytes()==probe['original']+probe['marker'],'source changed during probe; scoped marker recovery required'
  with source.open('r+b') as f:
   current=os.fstat(f.fileno())
   assert (current.st_dev,current.st_ino)==(probe['dev'],probe['ino'])
   assert f.read()==probe['original']+probe['marker'],'source changed before marker cleanup'
   f.truncate(len(probe['original']));f.flush();os.fsync(f.fileno())
  assert source.read_bytes()==probe['original'],'source changed while restoring owned marker'
 except BaseException as error:
  (gen/f"pdf-native-{probe['label']}-marker-recovery-required.json").write_text(json.dumps({'path':str(source),'marker':probe['marker'].decode(),'reason':str(error)},ensure_ascii=False,indent=2)+'\n')
  raise
 finally:
  assert lease.read_bytes()==token,'foreign source probe lease replacement'
  lease.unlink()
marker=append_probe(pdf_source,'own')
try:
 own_task,own_inventory=run('30-own-source-edit')
 assert own_task['hash']!=baseline_task['hash'],'PDF source edit did not invalidate PDF'
 assert own_task['cacheStatus']=='cache-miss','unique PDF source edit did not miss local cache'
finally:
 remove_probe(pdf_source,marker)
restored_task,restored=run('31-own-source-restored')
assert restored_task['hash']==baseline_task['hash'],'independent inputs changed during PDF source proof'
assert restored_task['cacheStatus']=='local-cache-hit','restored PDF source did not reuse local cache'
assert restored==baseline,'restored PDF source did not recover byte-identical outputs'
marker=append_probe(jpg_source,'sibling')
try:
 backup=move_owned('32-sibling-source-edit',baseline)
 try:
  sibling_task,sibling=run('32-sibling-source-edit')
  assert sibling_task['hash']==baseline_task['hash'],'sibling JPG source edit changed PDF inputs'
  assert sibling_task['cacheStatus']=='local-cache-hit','sibling JPG source edit missed PDF local cache'
  assert sibling==baseline,'sibling JPG source edit failed exact PDF output restoration'
 except BaseException:
  recover_owned_output('32-sibling-source-edit',backup,baseline)
  raise
 shutil.rmtree(backup)
finally:
 remove_probe(jpg_source,marker)
(gen/'pdf-native-source-isolation-proof.json').write_text(json.dumps({'baselineTask':baseline_task,'ownTask':own_task,'restoredTask':restored_task,'siblingTask':sibling_task,'fileCount':baseline['fileCount'],'totalBytes':baseline['totalBytes'],'exactRestoration':True},ensure_ascii=False,indent=2)+'\n')
print('[DEBUG] native PDF own-source invalidation, sibling-source reuse and exact restoration passed',flush=True)

PY
```

## Consumer Of Restored Publication (System-Linkage Allowlist, Pending)

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib,json,os,stat,uuid,shutil
root=Path.cwd()
ticket=root/'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES'
gen=ticket/'🗑️generated'
owner='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/Cargo.toml'
output=root/Path(owner).parent/'dist/build'
def inventory():
 for ancestor in [output,*output.parents]:
  if ancestor==root: break
  assert not ancestor.is_symlink(),str(ancestor)
 assert output.is_dir(),str(output)
 marker=json.loads((output/'.nx-artifact.json').read_text())
 assert marker['version']==1 and marker['owner']==owner,marker
 files={}
 for p in sorted(output.rglob('*')):
  assert not p.is_symlink(),str(p)
  if p.is_dir(): continue
  assert stat.S_ISREG(p.stat().st_mode),str(p)
  files[p.relative_to(output).as_posix()]={'size':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
 assert sorted(marker['files'])==sorted(set(files)-{'.nx-artifact.json'})
 return {'owner':owner,'files':files,'fileCount':len(files),'totalBytes':sum(f['size'] for f in files.values())}
def capture(stage):
 run=json.loads((gen/'nx-cache-root-pdf-native-proof/run.json').read_text())
 tasks=[t for t in run['tasks'] if t['taskId']=='@semio-tech/stdio-pdf-rs:build']
 assert len(tasks)==1 and tasks[0]['status']==0,tasks
 assert all(t['status']==0 for t in run['tasks']),run
 receipt=inventory()
 (gen/f'pdf-native-cache-{stage}-run.json').write_text(json.dumps(run,ensure_ascii=False,indent=2)+'\n')
 (gen/f'pdf-native-baseline-{stage}-inventory.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n')
 print(json.dumps({'stage':stage,'task':tasks[0],'fileCount':receipt['fileCount'],'totalBytes':receipt['totalBytes']},ensure_ascii=False),flush=True)
 return tasks[0],receipt
def move_owned(stage,baseline):
 lease=output.with_name(output.name+'.lease')
 nonce=uuid.uuid4().hex
 lease_bytes=json.dumps({'owner':owner,'pid':os.getpid(),'nonce':nonce}).encode()
 with lease.open('xb') as f: f.write(lease_bytes)
 try:
  current=inventory()
  assert current==baseline,'published PDF closure differs from baseline'
  backup=gen/f'pdf-native-output-backup-{stage}-{nonce}'
  assert not backup.exists()
  output.rename(backup)
  receipt={'owner':owner,'backup':str(backup),'nonce':nonce,'baseline':baseline}
  (gen/f'pdf-native-output-move-{stage}.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n')
  print(json.dumps({'moved':str(backup),'fileCount':baseline['fileCount']}),flush=True)
  return backup
 finally:
  assert lease.read_bytes()==lease_bytes,'foreign lease replacement'
  lease.unlink()

import subprocess
baseline=inventory()
assert baseline==json.loads((gen/'pdf-native-baseline-32-sibling-source-edit-inventory.json').read_text()),'publication changed after verified sibling-source restoration'
assert json.loads((gen/'pdf-native-source-isolation-proof.json').read_text())['exactRestoration'] is True
serde_libraries=sorted(path for path in baseline['files'] if path.startswith('deps/libserde_json-') and path.endswith('.rlib'))
assert serde_libraries and all(path[:-5]+'.rmeta' in baseline['files'] for path in serde_libraries),serde_libraries
serde_paths=[serde_libraries[0],serde_libraries[0][:-5]+'.rmeta']
consumer=ticket/'🧪️native-pdf-consumer/🦀️.rs'
binary=gen/'pdf-restored-consumer'
command=['rustc','--edition=2024','--crate-name','pdf_restored_consumer',str(consumer),'-L','dependency='+str(output),'-L','dependency='+str(output/'deps')]
for crate,paths in [
 ('semio_s_artifact_stdio_pdf',['libsemio_s_artifact_stdio_pdf.rlib','libsemio_s_artifact_stdio_pdf.rmeta']),
 ('serde_json',serde_paths)
]:
 for path in paths:
  assert path in baseline['files']
  command.extend(['--extern',crate+'='+str(output/path)])
command.extend(['-o',str(binary)])
with (gen/'pdf-restored-consumer-build.txt').open('w') as log:
 build=subprocess.run(command,cwd=root,stdout=log,stderr=subprocess.STDOUT)
assert build.returncode==0, 'restored-output-only consumer failed compilation'
with (gen/'pdf-restored-consumer-run.txt').open('w') as log:
 run_consumer=subprocess.run([str(binary)],cwd=root,stdout=log,stderr=subprocess.STDOUT)
assert run_consumer.returncode==0,'restored PDF consumer failed runtime'
linked=subprocess.run(['otool','-L',str(binary)],capture_output=True,text=True)
(gen/'pdf-restored-consumer-linked-libraries.txt').write_text(linked.stdout+linked.stderr)
assert linked.returncode==0
libraries=[line.strip().split(' (compatibility version',1)[0] for line in linked.stdout.splitlines()[1:] if line.strip()]
assert libraries and all(path.startswith('/usr/lib/') or path.startswith('/System/Library/') for path in libraries),('consumer has non-system runtime linkage',libraries)
assert inventory()==baseline,'consumer altered restored deliverables'
(gen/'pdf-restored-consumer-proof.json').write_text(json.dumps({'command':command,'buildExit':build.returncode,'runtimeExit':run_consumer.returncode,'linkedLibraries':linked.stdout,'systemOnlyRuntimeLibraries':libraries,'fileCount':baseline['fileCount'],'totalBytes':baseline['totalBytes']},ensure_ascii=False,indent=2)+'\n')
print('[DEBUG] restored-output-only PDF consumer compiled and ran; fixture matched; runtime libraries restricted to macOS system paths',flush=True)

PY
```
