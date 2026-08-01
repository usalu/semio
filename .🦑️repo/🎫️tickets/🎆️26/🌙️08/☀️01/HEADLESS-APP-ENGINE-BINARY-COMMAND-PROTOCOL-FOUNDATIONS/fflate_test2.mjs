import { inflateSync, deflateSync } from "fflate";
const raw = Buffer.from("0087b135201cabc2b7708ae6d5410d1b10578cbdfbd359c82d21bcea70477f7bd92b0e01200b0000000004010000", "hex");
for (const level of [0,1,6,9]) {
  const c = deflateSync(new Uint8Array(raw), { level });
  console.log("level", level, c.length, Buffer.from(c).toString('hex'));
}
console.log("expect stored: 012e00d1ff" + raw.toString('hex'));
