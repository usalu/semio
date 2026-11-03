import { load, save } from "./lib.ts";
import { shift } from "./lateral.ts";
const plan: [string, string[], number][] = [
  ["parent-carrying-toddler", ["50", "100"], 0.162],
  ["parent-with-stroller", ["50", "100", "200"], 0.092],
  ["person-cooking", ["50", "100", "200"], 0.068],
  ["person-on-crutches", ["50", "100", "200"], 0.085],
  ["person-with-rollator", ["50", "100", "200"], 0.051],
];
for (const [slug, scales, d] of plan) {
  const doc = load(slug);
  for (const k of scales) {
    for (const s of doc.asset.representations[k].geometry.front) {
      const m = /^(upper-arm|arm|forearm|hand)-([lr])$/.exec(s.bone);
      if (!m || s.role === "ground") continue;
      shift(s, "front", m[2] === "l" ? d : -d);
    }
    if (slug === "parent-carrying-toddler") for (const s of doc.asset.representations[k].geometry.front) {
      if (s.id === "toddler-arm-n") shift(s, "front", -0.008);
      if (/^(thigh-r|shank-r|shoe-r|crease-thigh-r|crease-shank-r)$/.test(s.id) && s.bone.startsWith("toddler")) shift(s, "front", 0.008);
    }
  }
  save(slug, doc);
}
