import zlib from "node:zlib";
const raw = Buffer.from("0087b135201cabc2b7708ae6d5410d1b10578cbdfbd359c82d21bcea70477f7bd92b0e01200b0000000004010000", "hex");
console.log("raw len", raw.length);
for (let level = 0; level <= 9; level++) {
  const c = zlib.deflateRawSync(raw, { level });
  console.log(`level=${level} len=${c.length} hex=${c.toString('hex')}`);
}
console.log("expected (from fixture) len=51 hex=012e00d1ff" + raw.toString('hex'));
