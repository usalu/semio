import zlib from "node:zlib";

const symbolsPayload = Buffer.from([0x00]);
const docPayload = Buffer.from([0x01,0x01,0x11,0x12]);

for (let level = 0; level <= 9; level++) {
  const s = zlib.deflateRawSync(symbolsPayload, { level });
  const d = zlib.deflateRawSync(docPayload, { level });
  console.log(`level=${level} symbols=${s.toString('hex')} doc=${d.toString('hex')}`);
}
console.log("expected symbols=630000 doc=636414140200");
