# Windows Strategy Corpus Preparation

Ticket-only proposal input: `windows-strategy-corpus.json`. No permanent W/P/F/X/C/H/schema/test/launcher/declaration changed. This future batch has no production API execution or Native/Windows/privilege qualification. Symbolic lifecycle rows are DATA expectations, not issued controls, capabilities or proofs. Actual future Source laws must bind the existing genuine one-Original/accepted SQLite EX and same-wire worker pattern.

The corpus retains existing POSIX frame bytes and proposes explicit Windows attributes/security/full128+serial frames without fake mode. It includes complete sorted tree/journal hashes, same-low64/different-high128, distinct serial, positive/zero/overflow links, canonical root journal/case/short-name/type/multilink refusals, nested ordinary basename, UTF8/zero-byte streams. Work rows retain existing native charges and add actual next/EOF/yield/scan/frame debits under one nonreset account. Lifetime rows cover exact/copy/foreign/control/role/consumption/pending/close/drain/system constraints. Their outputs are model predictions only.

The following inline Python input uses independent struct/hashlib and SQLite JSON/recursive state arithmetic. It writes only Generated output; no script file or executable command was added.

```python oracle
import json,hashlib,struct,sqlite3,re
from pathlib import Path
T=Path('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL')
v=json.loads((T/'windows-strategy-corpus.json').read_text())
canonical=lambda value:json.dumps(value,ensure_ascii=False,separators=(',',':'))
digest=lambda value:hashlib.sha256(value).hexdigest()
frames=[]
for case in v['frames']:
 data=case['input'];accepted=True;frame=None
 try:
  platform=data['platform'];kind=data['kind'];name=data['path'];win=platform=='win32';directory=kind=='directory'
  assert platform in ('posix','win32') and kind in ('file','directory') and len(name.encode('utf-16-le'))//2<=256
  assert not name.startswith('/') and '\\' not in name and (name=='' and directory or all(part not in ('','..','.') for part in name.split('/')))
  keys={'platform','path','kind'}|({'attributes','securitySha256'} if win else {'mode'})
  if not directory:keys|={'payload','links'}|({'fileIdLow','fileIdHigh','volumeSerial'} if win else {'ino'})
  assert set(data)<=keys|{'finalName'} and keys<=set(data)
  if 'finalName' in data:assert data['finalName']==name.split('/')[-1]
  journal=next((entry for entry in v['protocol']['journalNames'] if name.lower()==entry),None)
  if win and journal:assert name==journal
  if journal:assert not directory and data['links']==1
  frame={'path':name,'kind':kind}
  if win:
   assert 0<=data['attributes']<=4294967295 and re.fullmatch('[a-f0-9]{64}',data['securitySha256'])
   frame.update(platform='win32',attributes=data['attributes'],securitySha256=data['securitySha256'])
  else:
   assert 0<=data['mode']<=4095
   frame['mode']=data['mode']
  if not directory:
   links=data['links'];assert isinstance(links,int) and 1<=links<=(4294967295 if win else 18446744073709551615)
   payload=data['payload'].encode();frame.update(bytes=str(len(payload)),sha256=digest(payload))
   if win:
    nativeid=struct.pack('<QQ',int(data['fileIdLow']),int(data['fileIdHigh'])).hex();serial=struct.unpack('<Q',struct.pack('<Q',int(data['volumeSerial'])))[0]
    if links>1:frame.update(fileId=nativeid,volumeSerial=str(serial),links=str(links))
   elif links>1:
    assert int(data['ino'])>0
    frame.update(ino=data['ino'],links=str(links))
 except (AssertionError,KeyError,ValueError,OverflowError,struct.error):accepted=False;frame=None
 assert accepted==case['accepted'],case['id']
 frames.append(dict(id=case['id'],accepted=accepted,frame=frame,frameSha256=digest((canonical(frame)+'\n').encode()) if accepted else None))
byid={row['id']:row for row in frames};streams=[]
for case in v['streams']:
 rows=[byid[key]['frame'] for key in case['rows']];assert all(rows)
 rows.sort(key=lambda row:row['path'].encode());assert len({row['path'] for row in rows})==len(rows)
 journals=[dict(path=row['path'],bytes=row['bytes'],sha256=row['sha256']) for row in rows if row['path'] in v['protocol']['journalNames']]
 sha=digest(''.join(canonical(row)+'\n' for row in rows).encode())
 streams.append(dict(id=case['id'],version=1,digest=sha,applicationSha256=sha,files=sum(row['kind']=='file' for row in rows),directories=sum(row['kind']=='directory' for row in rows),links=0,sockets=0,bytes=str(sum(int(row.get('bytes','0')) for row in rows)),journalSha256=digest(canonical(journals).encode()) if journals else None,service=None))
assert streams[1]['digest']==streams[2]['digest'] and streams[1]['journalSha256']==streams[2]['journalSha256']
assert byid['native-hardlink']['frame']['fileId']!=byid['native-high-half-differs']['frame']['fileId']
assert byid['native-hardlink']['frame']['volumeSerial']!=byid['native-serial-differs']['frame']['volumeSerial']
db=sqlite3.connect(':memory:');work=[]
for case in v['work']:
 limit=case['limit'];spent=case['spent'];live=isinstance(limit,int) and 1<=limit<=9007199254740991 and 0<=spent<=limit;refused=None
 for index,units in enumerate(case['units']):
  if not live:continue
  if not isinstance(units,int) or not 1<=units<=9007199254740991 or units>limit-spent or index>=case.get('cancelAt',len(case['units'])):live=False;refused=index
  else:spent+=units
 raw=canonical(case)
 sql='''with recursive r(i,spent,live) as (select 0,json_extract(?1,'$.spent'),json_extract(?1,'$.limit') between 1 and 9007199254740991 and json_extract(?1,'$.spent') between 0 and json_extract(?1,'$.limit') union all select i+1,spent+case when live and json_extract(?1,'$.units['||i||']') between 1 and 9007199254740991 and json_extract(?1,'$.units['||i||']')<=json_extract(?1,'$.limit')-spent and i<coalesce(json_extract(?1,'$.cancelAt'),json_array_length(?1,'$.units')) then json_extract(?1,'$.units['||i||']') else 0 end,live and json_extract(?1,'$.units['||i||']') between 1 and 9007199254740991 and json_extract(?1,'$.units['||i||']')<=json_extract(?1,'$.limit')-spent and i<coalesce(json_extract(?1,'$.cancelAt'),json_array_length(?1,'$.units')) from r where i<json_array_length(?1,'$.units')) select spent,live from r order by i desc limit 1'''
 observed=db.execute(sql,(raw,)).fetchone();assert observed==(spent,int(live)),case['id'];assert spent==case['expectedSpent'] and live==case['accepted'],case['id']
 work.append(dict(id=case['id'],spent=spent,accepted=live,firstRefusal=refused))
lifetime=[]
for case in v['lifetime']:
 data=case['input'];allowed=data['control']=='exact' and data['live'] and data['accepting'] and not data['consumed'] and not data['pending'] and data['owner']=='same' and data['role']=='body' and (not data['transition'] or data['payloadReturned']) and (data['platform']=='posix' and data['context']=='none' and data['system']=='none' or data['platform']=='win32' and data['context']=='exact' and data['mapped'] and data['system']=='captured')
 sql='''select json_extract(?1,'$.control')='exact' and json_extract(?1,'$.live') and json_extract(?1,'$.accepting') and not json_extract(?1,'$.consumed') and not json_extract(?1,'$.pending') and json_extract(?1,'$.owner')='same' and json_extract(?1,'$.role')='body' and (not json_extract(?1,'$.transition') or json_extract(?1,'$.payloadReturned')) and ((json_extract(?1,'$.platform')='posix' and json_extract(?1,'$.context')='none' and json_extract(?1,'$.system')='none') or (json_extract(?1,'$.platform')='win32' and json_extract(?1,'$.context')='exact' and json_extract(?1,'$.mapped') and json_extract(?1,'$.system')='captured'))'''
 assert bool(db.execute(sql,(canonical(data),)).fetchone()[0])==allowed==case['accepted'],case['id']
 lifetime.append(dict(id=case['id'],wouldAdmit=allowed))
db.close();output=dict(version=1,sourceOnly=True,productionApiExecuted=False,windowsRuntime=False,frames=frames,streams=streams,work=work,lifetime=lifetime)
(T/'🗑️generated/windows-strategy-python-oracle.json').write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(dict(frames=len(frames),streams=len(streams),work=len(work),lifetime=len(lifetime),productionApiExecuted=False)))
```

The independent Bun input uses installed Ajv and node:crypto/Buffer against that output. It computes every frame/stream again; it does not take the Python digest as input to its own hash. No application runtime/Native function is imported.

```typescript oracle
import Ajv from 'ajv';
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const ticket='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL',v=JSON.parse(readFileSync(ticket+'/windows-strategy-corpus.json','utf8')),expected=JSON.parse(readFileSync(ticket+'/🗑️generated/windows-strategy-python-oracle.json','utf8')),hash=(value:string|Uint8Array)=>createHash('sha256').update(value).digest('hex'),ajv=new Ajv({strict:true});let assertions=0;
for(const[group,key]of[['frames','frameCaseSchema'],['work','workCaseSchema'],['lifetime','lifetimeCaseSchema'],['streams','streamCaseSchema']])for(const row of v[group]){assert.equal(ajv.validate(v[key],row),true,JSON.stringify(ajv.errors));assertions++;}
const frames=v.frames.map((row:any)=>{let accepted=true,frame:any=null;try{const i=row.input,win=i.platform==='win32',directory=i.kind==='directory',journal=v.protocol.journalNames.find((name:string)=>name===i.path.toLowerCase()),keys=['platform','path','kind',...(win?['attributes','securitySha256']:['mode']),...(!directory?['payload','links',...(win?['fileIdLow','fileIdHigh','volumeSerial']:['ino'])]:[])];assert.ok(['posix','win32'].includes(i.platform)&&['file','directory'].includes(i.kind));assert.ok(i.path.length<=256&&!i.path.startsWith('/')&&!i.path.includes('\\')&&(i.path===''&&directory||i.path.split('/').every((part:string)=>!['','..','.'].includes(part))));assert.ok(Object.keys(i).every(key=>keys.includes(key)||key==='finalName')&&keys.every(key=>Object.hasOwn(i,key)));if(Object.hasOwn(i,'finalName'))assert.equal(i.finalName,i.path.split('/').at(-1));if(win&&journal)assert.equal(i.path,journal);if(journal)assert.ok(!directory&&i.links===1);frame={path:i.path,kind:i.kind};if(win){assert.ok(Number.isInteger(i.attributes)&&i.attributes>=0&&i.attributes<=4294967295&&/^[a-f0-9]{64}$/.test(i.securitySha256));Object.assign(frame,{platform:'win32',attributes:i.attributes,securitySha256:i.securitySha256});}else{assert.ok(Number.isInteger(i.mode)&&i.mode>=0&&i.mode<=4095);frame.mode=i.mode;}if(!directory){assert.ok(Number.isInteger(i.links)&&i.links>=1&&i.links<=(win?4294967295:Number.MAX_SAFE_INTEGER));const bytes=Buffer.from(i.payload);Object.assign(frame,{bytes:String(bytes.length),sha256:hash(bytes)});if(win){const id=Buffer.alloc(16);id.writeBigUInt64LE(BigInt(i.fileIdLow));id.writeBigUInt64LE(BigInt(i.fileIdHigh),8);const serial=Buffer.alloc(8);serial.writeBigUInt64LE(BigInt(i.volumeSerial));if(i.links>1)Object.assign(frame,{fileId:id.toString('hex'),volumeSerial:String(serial.readBigUInt64LE()),links:String(i.links)});}else if(i.links>1){assert.ok(BigInt(i.ino)>0n);Object.assign(frame,{ino:i.ino,links:String(i.links)});}}assert.equal(ajv.validate(v.frameSchema,frame),true,JSON.stringify(ajv.errors));}catch{accepted=false;frame=null;}assert.equal(accepted,row.accepted,row.id);assertions++;return{id:row.id,accepted,frame,frameSha256:accepted?hash(JSON.stringify(frame)+'\n'):null};});assert.deepEqual(frames,expected.frames);assertions++;
const byId=new Map(frames.map((row:any)=>[row.id,row])),streams=v.streams.map((row:any)=>{const nodes=row.rows.map((key:string)=>(byId.get(key)as any).frame).sort((a:any,b:any)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path))),journals=nodes.filter((entry:any)=>v.protocol.journalNames.includes(entry.path)).map((entry:any)=>({path:entry.path,bytes:entry.bytes,sha256:entry.sha256})),digest=hash(nodes.map((entry:any)=>JSON.stringify(entry)+'\n').join(''));assert.equal(new Set(nodes.map((entry:any)=>entry.path)).size,nodes.length);return{id:row.id,version:1,digest,applicationSha256:digest,files:nodes.filter((entry:any)=>entry.kind==='file').length,directories:nodes.filter((entry:any)=>entry.kind==='directory').length,links:0,sockets:0,bytes:String(nodes.reduce((sum:number,entry:any)=>sum+Number(entry.bytes??0),0)),journalSha256:journals.length?hash(JSON.stringify(journals)):null,service:null};});assert.deepEqual(streams,expected.streams);assert.equal(streams[1].digest,streams[2].digest);assertions+=2;
const work=v.work.map((row:any)=>{let spent=row.spent,live=Number.isSafeInteger(row.limit)&&row.limit>=1&&Number.isSafeInteger(spent)&&spent>=0&&spent<=row.limit,firstRefusal:number|null=null;for(const[index,units]of row.units.entries()){if(!live)continue;if(!Number.isSafeInteger(units)||units<1||units>row.limit-spent||index>=(row.cancelAt??row.units.length)){live=false;firstRefusal=index;}else spent+=units;}assert.equal(spent,row.expectedSpent,row.id);assert.equal(live,row.accepted,row.id);assertions+=2;return{id:row.id,spent,accepted:live,firstRefusal};});assert.deepEqual(work,expected.work);assertions++;
const lifetime=v.lifetime.map((row:any)=>{const i=row.input,wouldAdmit=i.control==='exact'&&i.live&&i.accepting&&!i.consumed&&!i.pending&&i.owner==='same'&&i.role==='body'&&(!i.transition||i.payloadReturned)&&(i.platform==='posix'&&i.context==='none'&&i.system==='none'||i.platform==='win32'&&i.context==='exact'&&i.mapped&&i.system==='captured');assert.equal(wouldAdmit,row.accepted,row.id);assertions++;return{id:row.id,wouldAdmit};});assert.deepEqual(lifetime,expected.lifetime);assertions++;
const receipt={version:1,sourceOnly:true,productionApiExecuted:false,windowsRuntime:false,oracle:'independent Python struct/hashlib/SQLite versus Bun Buffer/node:crypto/Ajv',frames:frames.length,streams:streams.length,work:work.length,lifetime:lifetime.length,assertions,corpusSha256:hash(readFileSync(ticket+'/windows-strategy-corpus.json')),pythonOutputSha256:hash(readFileSync(ticket+'/🗑️generated/windows-strategy-python-oracle.json'))};writeFileSync(ticket+'/🗑️generated/windows-strategy-oracle-receipt.json',JSON.stringify(receipt,null,2)+'\n');console.log('[DEBUG] '+JSON.stringify(receipt));
```

## Actual Preparation Gate

Both inline inputs completed terminal0. Independent Python struct/hashlib/SQLite outputs agree with recomputed Bun Buffer/node:crypto/Ajv: **23 frame cases,4 complete stream cases,17 work cases,15 lifecycle cases;136 explicit Bun assertions**. Four closed case schemas and every accepted closed frame were validated with actual installed Ajv. The full-frame/two-journal stream is identical under different input order; the high-half and serial collision rows produce distinct native identities/frames. Recursive SQLite and separate state loops agree on nonreset accounting, cachedEOF exhaustion, refusal latching, cancellation and unsafe arithmetic refusal.

Retained Generated receipts: `windows-strategy-python-oracle.json` and `windows-strategy-oracle-receipt.json`. Corpus SHA256 `56cde0d2b3bf4433fd76da19a8239b9ff7f4fe895a92a6f8425c5c976fb8bb2a`; Python output SHA256 `e59dd4e18f0f709114c8c1aaf81c80734f91bd0cec0b8a35b98ef6067dec7c2b`. Model rows executed59; **production API cases executed0**. No missing-strategy Source RED or implementation GREEN is claimed by this preparation. Future release must migrate these into the existing permanent W owner and add real genuine phase/accepted-context Source counterlaws before algorithm changes.

All permanent Source/runtime/schema/test/reader bytes remain held by their respective owners. This task only authored this Markdown input/report and the ticket JSON. No actual filesystem inventory, Windows handle, Native image, system command, grant, privilege, deadline, cap or budget was changed.
