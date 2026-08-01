import { inflateSync, deflateSync } from "fflate";

const raw = Buffer.from("0087b135201cabc2b7708ae6d5410d1b10578cbdfbd359c82d21bcea70477f7bd92b0e01200b0000000004010000", "hex");
const expectedStored = Buffer.from("012e00d1ff0087b135201cabc2b7708ae6d5410d1b10578cbdfbd359c82d21bcea70477f7bd92b0e01200b0000000004010000", "hex");
// try inflate on expectedStored
const inflated = inflateSync(new Uint8Array(expectedStored));
console.log("inflate matches raw:", Buffer.compare(Buffer.from(inflated), raw) === 0);

const docRaw = Buffer.from([0x01,0x01,0x11,0x12]);
const expectedDocStored = Buffer.from("636414140200", "hex");
const inflatedDoc = inflateSync(new Uint8Array(expectedDocStored));
console.log("doc inflate matches:", Buffer.compare(Buffer.from(inflatedDoc), docRaw) === 0);

for (const level of [0,1,6,9]) {
  const c = deflateSync(new Uint8Array(docRaw), { level });
  console.log("level", level, Buffer.from(c).toString('hex'));
}
