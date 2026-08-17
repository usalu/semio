import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const MODULES="/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const BAD = "../�️️manifest/";
const GOOD = "../🛂️manifest/";

for (const rel of ["🔺️mesh/🟦️component.ts", "🎠️kernel/🟦️component.ts"]) {
  const p = join(MODULES, rel);
  let t = readFileSync(p, "utf8");
  if (!t.includes(BAD) && !t.includes("�️️")) {
    // try replace any corrupted sequence
    console.log(rel, "no BAD literal, scanning...");
    t = t.replaceAll("../�️️manifest/", GOOD);
  } else {
    t = t.replaceAll(BAD, GOOD);
  }
  // also fix if written as replacement char variants
  t = t.replace(/\.\.\/.manifest\//g, (m) => {
    if (m.includes("🛂️")) return m;
    console.log("replacing odd", JSON.stringify(m));
    return GOOD;
  });
  writeFileSync(p, t);
  console.log("fixed", rel, "still bad?", t.includes("�️️"));
}

// Remove circular test imports from manifest — move Tests into package glue instead
const manifestPath = join(MODULES, "�️️manifest/🟦️component.ts");
