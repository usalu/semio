/** 🧪 Exercises exact browser archive packet custody with the independent native disk fixture. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { join } from "node:path";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { persistNaturalArchive, type NaturalArchiveExchange, type NaturalArchiveFrontier } from "./natural-pdf-browser-archive.ts";
import laws from "./natural-pdf-browser-archive.json";
import schema from "./natural-pdf-browser-archive.schema.json";
import natural from "./natural-pdf-admission.json";

const validate = new Ajv({strict:true}).compile(schema);
assert(validate(laws),JSON.stringify(validate.errors));
const pack = Buffer.from(await Bun.file(join(import.meta.dir,"🗑️generated/paged-jpeg-assets/large.dct-placement.pack")).bytes());
const varint = (value: number) => {
  const bytes: number[]=[];
  while (value>=128) { bytes.push(value%128+128); value=Math.floor(value/128); }
  bytes.push(value);
  return Buffer.from(bytes);
};
const spr = Buffer.from("schema=stdio.pdf.1.7\ndocument=pdf-document");
const archive = Buffer.concat([Buffer.of(1),varint(pack.length),pack,varint(spr.length),spr,Buffer.of(0)]);
assert.equal(archive.length,2_130_150);
const digest = createHash("sha256").update(archive).digest();
const declared = natural.offer.declaration;
const {extent, instance} = natural.trusted;
const accepted = {
  loan: { operationId:natural.trusted.operation, generation:natural.trusted.generation,
    instanceId:instance, declaration:declared, extent:natural.offer.extent },
  owner:BigInt(instance),seq:BigInt("0x"+natural.begin.seq),kind:241,extent:natural.offer.extent,
  phase:"complete",result:"accepted",
} as const;
assert(accepted.seq>BigInt(Number.MAX_SAFE_INTEGER));
const documentId = "pdf-document";
const frontier: NaturalArchiveFrontier = {documentId,generation:7n,revision:new Uint8Array(32).fill(5),extent:archive.length};
const database = new Database(":memory:");
database.run("create table capture(offset integer primary key, bytes blob)");
const insert = database.query("insert into capture(offset,bytes) values(?,?)");
const read = database.query("select offset, bytes from capture order by offset");
const geometry = database.query("select min(4096, ?1 - ?2) as length");
for (const law of laws) {
  database.run("delete from capture");
  let writes=0,acks=0,closed=0,cancelled=0,localRetired=0;
  let pending=true, seals=0, published=false, progress=0, retired=false;
  let result="accepted";
  const cancellation=new AbortController();
  const exchange: NaturalArchiveExchange = {
    retired:()=>retired,now:()=>1,yieldTurn:async()=>{},
    async run(bytes,closing) {
      const request=Buffer.from(bytes);
      assert.equal(request.length,43);
      assert.equal(request.toString("ascii",0,8),"SEMIOFA1");
      assert.equal(request.readUInt32LE(8),instance);
      assert.equal(request.readBigUInt64LE(12),BigInt("0x"+accepted.loan.operationId));
      assert.equal(request.readBigUInt64LE(20),BigInt("0x"+accepted.loan.generation));
      assert.equal(request.readBigUInt64LE(28),accepted.seq);
      const action=request[36];
      const offset=request.readUInt32LE(37);
      const length=request.readUInt16LE(41);
      assert.equal(closing,action===5||action===6);
      const answer=(tag:number,body:Uint8Array=Buffer.alloc(0))=>
        Buffer.concat([Buffer.from("SEMIOFR1"),request.subarray(8),Buffer.of(tag),body]);
      if (action===1) {
        assert.equal(offset+length,0);
        if (pending) { pending=false; return undefined; }
        const head=Buffer.alloc(44);
        head.writeUInt32LE(law.name==="oversized-archive" ? 4*1024*1024+1 : frontier.extent,0);
        head.writeBigUInt64LE(frontier.generation,4);
        head.set(frontier.revision,12);
        return answer(1,Buffer.concat([head,Buffer.from(law.name==="foreign-document" ? "other-document" : documentId)]));
      }
      if (action===2) {
        assert.equal(offset,acks*4096);
        assert.equal(length,0);
        const size=(geometry.get(archive.length,offset) as {length:number}).length;
        const page=answer(2,archive.subarray(offset,offset+size));
        if(law.name==="foreign-page-receipt") page.writeBigUInt64LE(accepted.seq+1n,28);
        return page;
      }
      if (action===3) {
        assert.equal(offset,acks*4096);
        assert.equal(writes,acks+1);
        assert.equal(length,(geometry.get(archive.length,offset) as {length:number}).length);
        acks++;
        return answer(3);
      }
      assert.equal(offset+length,0);
      if (action===4) {
        assert.equal(acks,Math.ceil(archive.length/4096));
        assert.equal(published,seals===1);
        const emitted=Buffer.from(digest);
        if (++seals===2 && law.name==="frontier-changed-after-write") emitted[0]^=1;
        return answer(4,emitted);
      }
      if (action===5) { cancelled++; return answer(5); }
      assert.equal(action,6);
      assert.equal(writes,Math.ceil(archive.length/4096));
      assert.equal(seals,2);
      if (law.name==="close-refused") return answer(6);
      if (law.name==="owner-lost-after-close") retired=true;
      closed++;
      return answer(5);
    },
  };
  try {
    const persisted=await persistNaturalArchive(accepted,documentId,exchange,async offer=>{
      assert.deepEqual(offer,frontier);
      return {
        async write(offset,page) {
          assert.equal(offset,writes*4096);
          assert.deepEqual(Buffer.from(page),archive.subarray(offset,offset+page.length));
          insert.run(offset,page);
          writes++;
          if (law.name==="cancel-first-write") cancellation.abort(Error("archive cancelled"));
          return {offset:offset+Number(law.name==="foreign-write-receipt"),length:page.length};
        },
        async publish(seal) {
          published=true;
          const hash=createHash("sha256");
          let covered=0;
          for (const row of read.iterate() as Iterable<{offset:number;bytes:Uint8Array}>) {
            assert.equal(row.offset,covered);
            hash.update(row.bytes);
            covered+=row.bytes.length;
          }
          const measured=hash.digest();
          assert.equal(covered,archive.length);
          assert.deepEqual(measured,Buffer.from(seal));
          if (law.name==="foreign-seal-readback") measured[0]^=1;
          return {extent:covered,digest:measured,frontier:offer};
        },
        async retire() { localRetired++; database.run("delete from capture"); },
      };
    }, cancellation.signal,()=>true,(completed,total)=>{
      assert.equal(total,archive.length);
      assert.equal(completed,Math.min(acks*4096,total));
      assert(completed>=progress);
      progress=completed;
    });
    assert.deepEqual(persisted,{extent:archive.length,digest:Uint8Array.from(digest),frontier});
    assert.equal(progress,archive.length);
  } catch (error) {
    result=(error as Error).message;
  }
  assert.deepEqual({name:law.name,expected:result,writes,acks,closed,cancelled,localRetired},law);
}
database.close();
console.log(`[DEBUG] Browser first-archive exchange passed ${laws.length} cases with SQLite and ${archive.length} framed bytes (${digest.toString("hex")})`);
