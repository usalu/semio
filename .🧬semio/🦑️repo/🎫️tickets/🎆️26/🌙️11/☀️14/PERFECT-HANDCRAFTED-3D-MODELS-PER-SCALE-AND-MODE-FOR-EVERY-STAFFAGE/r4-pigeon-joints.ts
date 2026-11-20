/** 🧭️ Round 4 pigeon: prints the joints and bone directions of the default pose in world metres (the numbers the hand-stated sections are placed on). */
import { Bench } from "./r3-pigeon-kit.ts";

const S = new Bench(50);
for (const b of ["trunk", "pelvis", "neck-1", "neck-2", "neck-3", "head", "jaw", "tail", "humerus-n", "forearm-n", "hand-n", "thigh-n", "shank-n", "tarsus-n", "toes-n"]) {
  const f = S.frame(b);
  console.log(b, (f.o as number[]).map((v) => v.toFixed(4)).join(" "), "u", (f.u as number[]).map((v) => v.toFixed(3)).join(" "));
}
console.log("tailBase", S.tailBase.map((v) => v.toFixed(4)).join(" "), "axis", S.axis.map((v) => v.toFixed(3)).join(" "));
