/** 🔎️ Round 3 herring gull: prints the world joints of the default pose of the trial document (metres, x ahead, y up, z left). */
import { Bench } from "./r3-herring-gull-kit.ts";
const S = new Bench(50);
for (const b of ["trunk", "tail", "neck-1", "neck-2", "head", "humerus-n", "forearm-n", "hand-n", "thigh-n", "shank-n", "tarsus-n", "foot-n", "foot-f"]) {
  const f = S.frame(b);
  console.log(b.padEnd(10), (f.o as number[]).map((v) => v.toFixed(4)).join(" "), "u", (f.u as number[]).map((v) => v.toFixed(3)).join(" "), "v", (f.v as number[]).map((v) => v.toFixed(3)).join(" "));
}
console.log("head tip", S.J("head", 0.125).map((v) => v.toFixed(4)).join(" "), "hand tip", S.J("hand-n", 0.43).map((v) => v.toFixed(4)).join(" "), "tail tip", S.J("tail", 0.17).map((v) => v.toFixed(4)).join(" "));
