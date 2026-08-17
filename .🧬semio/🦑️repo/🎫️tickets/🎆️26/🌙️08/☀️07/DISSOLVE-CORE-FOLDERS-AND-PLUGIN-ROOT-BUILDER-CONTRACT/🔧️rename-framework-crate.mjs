import { readFileSync, writeFileSync } from "fs";
import { execSync } from "child_process";

const cargoFiles = execSync(`rg -l 'semio-framework-core' --glob 'Cargo.toml' -g '!target/**'`, { encoding: "utf8" })
  .trim()
  .split("\n")
  .filter(Boolean);

let cargoN = 0;
for (const f of cargoFiles) {
  let t = readFileSync(f, "utf8");
  const o = t;
  t = t.replaceAll('package = "semio-framework-core"', 'package = "semio-framework"');
  t = t.replaceAll("semio-framework-core =", "semio-framework =");
  if (t !== o) {
    writeFileSync(f, t);
    cargoN++;
    console.log("cargo", f);
  }
}
console.log("cargo updated", cargoN);

const rsFiles = execSync(`rg -l 'semio_framework_core' --glob '*.rs' -g '!target/**' -g '!.🦑️repo/**'`, { encoding: "utf8" })
  .trim()
  .split("\n")
  .filter(Boolean);
let rsN = 0;
for (const f of rsFiles) {
  let t = readFileSync(f, "utf8");
  const o = t;
  t = t.replaceAll("semio_framework_core", "semio_framework");
  if (t !== o) {
    writeFileSync(f, t);
    rsN++;
  }
}
console.log("rs updated", rsN, "of", rsFiles.length);

// TS package rename consumers
const tsFiles = execSync(`rg -l '@semio-tech/framework-core' -g '!node_modules/**' -g '!target/**' -g '!.🦑️repo/**' -g '!storybook-static/**'`, { encoding: "utf8" })
  .trim()
  .split("\n")
  .filter(Boolean);
let tsN = 0;
for (const f of tsFiles) {
  let t = readFileSync(f, "utf8");
  const o = t;
  t = t.replaceAll("@semio-tech/framework-core", "@semio-tech/framework");
  if (t !== o) {
    writeFileSync(f, t);
    tsN++;
  }
}
console.log("ts updated", tsN, "of", tsFiles.length);
