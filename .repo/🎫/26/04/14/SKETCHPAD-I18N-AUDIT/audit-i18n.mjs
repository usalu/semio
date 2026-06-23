import fs from "fs";

const uiPath = "c:/git/compose/elements/ui/index.tsx";
const sketchPath = "c:/git/compose/compose/sketchpad/index.tsx";

const s = fs.readFileSync(uiPath, "utf8");
const startMarker = "en: {\n    translation: JSON.parse(String.raw`";
const i = s.indexOf(startMarker);
if (i < 0) {
  console.error("start not found");
  process.exit(1);
}
const sub = s.slice(i + startMarker.length);
const endIdx = sub.indexOf("`\n  },");
if (endIdx < 0) {
  console.error("end not found");
  process.exit(1);
}
const jsonStr = sub.slice(0, endIdx);
const data = JSON.parse(jsonStr);

function get(obj, keyPath) {
  const parts = keyPath.split(".");
  let cur = obj;
  for (const p of parts) {
    if (cur == null || typeof cur !== "object") return undefined;
    cur = cur[p];
  }
  return cur;
}

const sketch = fs.readFileSync(sketchPath, "utf8");
const keys = new Set();

for (const re of [
  /useLabel\("(compose\.sketchpad[^"]+)"/g,
  /t\("(compose\.sketchpad[^"]+)"/g,
  /useLabel\("(compose\.file[^"]+)"/g,
  /useLabel\("(compose\.folder[^"]+)"/g,
]) {
  let m;
  while ((m = re.exec(sketch))) keys.add(m[1]);
}

const missing = [];
for (const key of [...keys].sort()) {
  if (get(data, key) === undefined) missing.push(key);
}

console.log("Total keys referenced:", keys.size);
console.log("Missing in en bundle:", missing.length);
for (const k of missing) console.log(k);
