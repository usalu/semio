/** 🎲️ C12 14c: 3000 seeded random (text, splice) pairs with the TS reference's located/result/inverse — the Python twin must agree. */
import { applyTextSpliceV1, textSpliceFromEditV1 } from "./patch/tree/🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🟦️.ts";
let seed = 0x14c_c12;
const random = () => ((seed = (Math.imul(seed ^ (seed >>> 15), 0x2c1b3c6d) + 0x6d2b79f5) >>> 0) / 2 ** 32);
const alphabet = Array.from("ab ab😀\nxé");
const word = (max: number) => Array.from({ length: Math.floor(random() * max) }, () => alphabet[Math.floor(random() * alphabet.length)]).join("");
const rows = [];
for (let index = 0; index < 3000; index += 1) {
  const author = word(40);
  const chars = Array.from(author);
  const at = Math.floor(random() * (chars.length + 1)), cut = Math.floor(random() * 4);
  const edited = [...chars.slice(0, at), ...Array.from(word(4)), ...chars.slice(Math.min(chars.length, at + cut))].join("");
  const splice = textSpliceFromEditV1(author, edited);
  if (splice === null) continue;
  const target = random() < 0.3 ? word(40) : [word(6), author, word(6)].join("");
  const applied = applyTextSpliceV1(target, splice);
  rows.push({ target, splice, located: applied.located, text: applied.text, inverse: applied.inverse });
}
console.log(JSON.stringify(rows));
