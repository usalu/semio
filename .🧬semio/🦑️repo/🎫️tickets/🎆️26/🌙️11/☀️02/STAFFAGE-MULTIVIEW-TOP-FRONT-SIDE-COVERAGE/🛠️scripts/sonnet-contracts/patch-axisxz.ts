import { readFileSync, writeFileSync } from "node:fs";
const dir = process.argv[2]!;
const sub = (t: string, from: string, to: string): string => {
  if (!t.includes(from)) throw new Error(`missing: ${from.slice(0, 60)}`);
  return t.replace(from, to);
};
let json = readFileSync(`${dir}/🔣️.json`, "utf8");
json = sub(json, "offset by axisX ahead and axisZ laterally in the bone-local frame, carried with that bone;", "the axis point is the rest origin of axisBone translated by axisX along world x and axisZ along world z at rest, and the axis is then carried with the bone frame;");
json = sub(json, "Offset of the axis from the rest origin of axisBone, ahead in the bone-local frame; the axis is carried with the bone frame.", "Translation of the axis point from the rest origin of axisBone along the world x axis (ahead) at rest, not along the bone's u; the axis is then carried with the bone frame.");
json = sub(json, "Offset of the axis from the rest origin of axisBone, laterally (left is +) in the bone-local frame; default 0; the axis is carried with the bone frame.", "Translation of the axis point from the rest origin of axisBone along the world z axis (left is +) at rest, not along the bone's w; default 0; the axis is then carried with the bone frame.");
writeFileSync(`${dir}/🔣️.json`, json);
let ts = readFileSync(`${dir}/🟦️.ts`, "utf8");
ts = sub(ts, "offset by `axisX` ahead and `axisZ` laterally (default 0) in the bone-local frame and carried with that bone;", "through the point translated from that origin by `axisX` along world x and `axisZ` along world z (default 0) at rest, the axis then carried with that bone;");
writeFileSync(`${dir}/🟦️.ts`, ts);
