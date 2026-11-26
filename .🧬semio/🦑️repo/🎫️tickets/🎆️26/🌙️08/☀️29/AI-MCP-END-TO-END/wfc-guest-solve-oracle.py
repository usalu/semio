"""🌐 Run the real WFC cold job through the repository's fixed Core ABI."""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import struct
import sqlite3
import subprocess
import tempfile

ticket = Path(__file__).resolve().parent
g = ticket / "🗑️generated"
fixture = json.loads(Path("✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🧫️initial-artifact.json").read_text())
service = json.loads((g / "wfc-seeded-solve-causal-short.json").read_text())["solve-service"]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--component", type=Path, default=g / "wfc-model-pQ0Ttc/browser/prototype/components/f0caf2db1142c1ba93f50ac6/📦️component.wasm")
parser.add_argument("--label", default="original")
args = parser.parse_args()
if not args.label or not all(c.isascii() and (c.isalnum() or c == "-") for c in args.label):
    parser.error("label must contain ASCII letters, digits or hyphens")
component = args.component.resolve(strict=True)
stem = "wfc-one-pixel-" + args.label
binary = g / "wfc-pair" / ("wfc-pinned-pair.exe" if os.name == "nt" else "wfc-pinned-pair")
def encode(obj):
    return json.dumps(obj,ensure_ascii=False,sort_keys=True,separators=(",",":")).encode()
with tempfile.TemporaryDirectory(dir=g) as folder:
    subprocess.run([binary,folder,"wfc-seeded-solve"],input=encode(fixture["solve"]),check=True)
    document = {key:base64.b64encode((Path(folder)/key.upper()).read_bytes()).decode() for key in ("pack","spr")}
route = {k:service[k] for k in ["owner","artifactKind","artifactSchema","artifactSchemaVersion","algorithmVersion","policyVersion","inferenceSchema","inferenceSchemaVersion"]}
route.update(wireVersion=2,revision=0,generation=1,sourceDialect=service["artifactSchema"],policy=[],budgets={"allocationBytes":1<<20,"workUnits":1000000,"recursionDepth":4},retained={"maximumItems":4096,"maximumCopyBytes":1<<20,"maximumCapacityBytes":1<<20,"maximumReleaseBytes":1<<20,"maximumDepth":128},cancellationId="wfc-seeded-solve",previousState=None,requestedCacheMode="cold",canonicalPayload=list(encode({"document":document})),dependencies=[])
body=encode(route)
kind=b"semio.infer"
wire=struct.pack("<4sIQQQQ",b"SJS1",1,1,len(kind),len(body),0)+kind+body
wire_path=g/(stem+".sjs")
wire_path.write_bytes(wire)
subprocess.run(["repo","tools","exec","--light","bun","--",ticket/"cold-job-frame-replay.ts","--prepare-only"],check=True)
src=(g/"cold-job-jit.ts").read_text()
decoder = Path("🧰️framework/🔨️modules/🎁️packs/🟦️.ts").resolve()
src = "import {decodePackWire} from " + json.dumps(str(decoder), ensure_ascii=False) + ";\n" + src
src=src.replace("  const readReceipt = (name: string, pointerArgs: number[] = []) => {", """  let jobReceipt = {kind:0,flags:0,body:[0n,0n,0n,0n]};
  const readReceipt = (name: string, pointerArgs: any[] = []) => {""")
src=src.replace("    if (pointerArgs.length) {",'    if (name === "admit_actor_input" && pointerArgs.length) {')
src=src.replace('    const offset = magic === "SBT1" ? 8 : 0;', '''    const job = magic === "SJH2";
    if (job) {
      assert.equal(bytes[6],0,"genuine job refusal at " + name);
      jobReceipt={kind:bytes[4],flags:bytes[5],body:[0,1,2,3].map(i=>bytes.readBigUInt64LE(152+i*8))};
    }
    const offset = job || magic === "SBT1" ? 8 : 0;''').replace('offset === 8 ? 200 : 144','job ? 184 : offset === 8 ? 200 : 144').replace('    if (offset) {','    if (magic === "SBT1") {').replace('    assert(isInput ? (kind','    assert(job || (isInput ? (kind').replace('=== 1 && flags <= 1, "Unexpected','=== 1 && flags <= 1), "Unexpected')
src=src.replace('    log({ ...details, kind, flags, refusal, ready, inputReturned,','    if (!job || (epoch & (epoch-1n))===0n) log({ ...details, kind, flags, refusal, ready, inputReturned,')
cleanup_path=Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/💼️job/📤️receipt/🧪️tests/🧫️fixtures/cleanup-exports.json")
cleanup=json.loads(cleanup_path.read_bytes())
for case in cleanup:
    tag=sqlite3.connect(":memory:").execute("select case ? when 'cancel' then 5 when 'close' then 6 end",(case["status"],)).fetchone()[0]
    assert tag==case["tag"]
close_tag=next(row["tag"] for row in cleanup if row["export"]=="close_jobs")
src=src.replace('  let outputClosed = false;', '''  const id=1n;
  function jobCall(name:string, tag:number, args:bigint[]=[id]){
    readReceipt(name,args);
    assert.equal(jobReceipt.kind,tag);
    return jobReceipt;
  }
  let started=false;
  for(let i=0;i<10000&&!started;i++) started=!!(jobCall("start_job",1).flags&1);
  assert(started,"guest start reached its ceiling");
  let state=0n, size=0n, steps=0;
  while(steps++<100000 && state===0n){
    const r=jobCall("step_job",2);
    if((r.flags&1)!==0){state=r.body[1];size=r.body[2];}
  }
  log({stage:"wfc-job-terminal",state:Number(state),steps,size:Number(size)});
  assert(state===1n || state===2n);
  let position=0n,closed=false;
  const chunks:Buffer[]=[];
  for(let reads=0;!closed && reads<100000;reads++){
    const r=jobCall("read_job_output",3);
    if((r.flags&1)===0) continue;
    assert.equal(r.body[0],position);
    if(r.flags&4){closed=true;break;}
    if((r.flags&2)===0 || r.body[1]===0n) continue;
    const length=r.body[1];
    const borrowed=unpack(invoke("borrow_job_output",id,1n,1n,position,length));
    assert.equal(BigInt(borrowed.length),length);
    chunks.push(Buffer.from(borrowed.bytes));
    let acknowledged=false;
    for(let i=0;i<10000&&!acknowledged;i++) acknowledged=!!(jobCall("acknowledge_job_output_chunk",4,[id,position,length]).flags&1);
    assert(acknowledged); position+=length;
  }
  assert(closed); assert.equal(position,size);
  const bytes=Buffer.concat(chunks);
  writeFileSync(process.env.SEMIO_COLD_JOB_RESULT!+".bin",bytes);
  const result=state===2n ? decodePackWire(bytes) : JSON.parse(bytes.toString());
  log({stage:"actual-wfc-result",steps,result});
  writeFileSync(process.env.SEMIO_COLD_JOB_RESULT!,JSON.stringify({state:Number(state),steps,result}));
  let acknowledged=false,jobsClosed=false;
  for(let i=0;i<10000&&!acknowledged;i++) acknowledged=!!(jobCall("acknowledge_job_output",4).flags&1);
  for(let i=0;i<10000&&!jobsClosed;i++) jobsClosed=!!(jobCall("close_jobs",6,[]).flags&1);
  assert(acknowledged&&jobsClosed);
  let outputClosed = false;''')
src=src.replace('jobCall("close_jobs",6,[])','jobCall("close_jobs",'+str(close_tag)+',[])')
runner=g/(stem+"-guest.ts")
runner.write_text(src)
env=dict(os.environ,SEMIO_COLD_JOB_WIRE=str(wire_path),SEMIO_COLD_JOB_RESULT=str(g/(stem+"-result.json")))
digest=hashlib.sha256(component.read_bytes()).hexdigest()
print("[DEBUG] real guest",digest,"reconstructed donor bytes",len(wire),flush=True)
subprocess.run(["repo","tools","exec","--light","bun","--",runner,component,digest,"wfc"],env=env,check=True,timeout=180)
outcome=json.loads(Path(env["SEMIO_COLD_JOB_RESULT"]).read_bytes())
assert outcome["state"]==1, outcome
reply=outcome["result"]
for key in ("owner","artifactKind","artifactSchema","inferenceSchema","revision","generation","cancellationId"):
    assert reply[key]==route[key], key
assert reply["validity"]=="valid" and reply["complete"] is True
payload=json.loads(bytes(reply["canonicalPayload"]))
with sqlite3.connect(":memory:") as oracle:
    for field in fixture["result"]:
        query="select fullkey,type,atom from json_tree(?,?) where type not in ('object','array') order by fullkey"
        def projection(value):
            return [(key, "number" if kind in ("integer","real") else kind, atom) for key,kind,atom in oracle.execute(query,(json.dumps(value),"$."+field))]
        assert projection(payload)==projection(fixture["result"]), field
