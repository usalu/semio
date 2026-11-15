import {readdirSync,readFileSync,writeFileSync,renameSync,rmSync} from "node:fs";
import {brotliCompressSync,brotliDecompressSync,constants} from "node:zlib";
import {join} from "node:path";

const [directory,shard,shards] = [process.argv[2], Number(process.argv[3]), Number(process.argv[4])];
const entries = join(directory, "e"), names = readdirSync(entries).filter(name => name.endsWith(".json.br")).sort().filter((_, at) => at % shards === shard);
let converted = 0, skipped = 0;
const started = performance.now();
for (const name of names) {
  const path = join(entries, name);
  let record;
  try { record = JSON.parse(brotliDecompressSync(readFileSync(path), {maxOutputLength: 268435456}).toString("utf8")); } catch { rmSync(path, {force: true}); skipped++; continue; }
  if (record.version === 3) { skipped++; continue; }
  if (record.version !== 2) { rmSync(path); skipped++; continue; }
  if (record.sidecar) {
    const walks = [], index = new Map(), at = walk => { let found = index.get(walk); if (found === undefined) { found = walks.length; walks.push(walk); index.set(walk, found); } return found; };
    record.sidecar = {...record.sidecar, members: record.sidecar.members?.map(list => list && list.map(at)), walks};
  }
  record.version = 3;
  const temporary = path + ".convert." + shard + ".tmp";
  writeFileSync(temporary, brotliCompressSync(Buffer.from(JSON.stringify(record), "utf8"), {params: {[constants.BROTLI_PARAM_QUALITY]: 4}}));
  renameSync(temporary, path);
  if (++converted % 500 === 0) console.log("[DEBUG] shard", shard, "converted", converted, "of", names.length, Math.round((performance.now() - started) / 1000), "s");
}
console.log("[DEBUG] shard", shard, "done converted", converted, "skipped", skipped, Math.round((performance.now() - started) / 1000), "s");
