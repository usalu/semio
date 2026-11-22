/** 📥️ Acquires pinned URW Base35 OpenType source fonts and extracts their original CFF tables. */
import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import Ajv from "ajv";

const fixture = await Bun.file(join(import.meta.dir,"🔣️pdf-base14-outlines.json")).json();
const schema = await Bun.file(join(import.meta.dir,"🔣️pdf-base14-outlines.schema.json")).json();
const validate = new Ajv({strict:true}).compile(schema);
assert(validate(fixture), JSON.stringify(validate.errors));
const output = join(import.meta.dir, "🗑️generated", "pdf-base14");
await mkdir(output, { recursive:true });
const origin = "https://raw.githubusercontent.com/ArtifexSoftware/urw-base35-fonts/" + fixture.commit + "/";
const inputs = ["LICENSE","COPYING",...fixture.faces.map((row: {program:string}) => "fonts/" + row.program + ".otf")];
const digests: Record<string,string> = {};
for (let from = 0; from < inputs.length; from += 4) {
  await Promise.all(inputs.slice(from,from+4).map(async path => {
    const name = path.split("/").at(-1)!;
    const file = Bun.file(join(output,name));
    let bytes: Uint8Array;
    if (await file.exists()) bytes = await file.bytes();
    else {
      const signal = AbortSignal.timeout(30_000);
      const result = await fetch(origin+path,{signal});
      assert(result.ok, path + " " + result.status);
      assert(!result.headers.get("content-length") || Number(result.headers.get("content-length")) <= 512*1024);
      bytes = await result.bytes();
      assert(bytes.byteLength > 0 && bytes.byteLength <= 512*1024);
      await writeFile(file.name!,bytes);
    }
    digests[path] = new Bun.CryptoHasher("sha256").update(bytes).digest("hex");
    if (name.endsWith(".otf")) {
      const header = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
      assert.equal(header.getUint32(0), 0x4f54544f);
      const tables = header.getUint16(4);
      const record = Array.from({length:tables},(_,i)=>12+i*16).find(at=>header.getUint32(at) === 0x43464620);
      assert(record, name + " has no CFF table");
      const offset = header.getUint32(record+8), length = header.getUint32(record+12);
      assert(offset + length <= bytes.length && bytes[offset] === 1);
      await writeFile(join(output,name.replace(/\.otf$/u,".cff")),bytes.subarray(offset,offset+length));
    }
  }));
}
await writeFile(join(output,"assets.json"),JSON.stringify({origin,digests},null,2));
console.log("[DEBUG] Staged the 14 exact Base14 CFF font sources from",fixture.release);
