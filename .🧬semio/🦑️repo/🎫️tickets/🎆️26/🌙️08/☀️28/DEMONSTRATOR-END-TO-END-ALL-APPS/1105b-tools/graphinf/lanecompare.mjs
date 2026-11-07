import {execFileSync} from "node:child_process";
import {readFileSync, existsSync} from "node:fs";
const [root, closureFile, targetsFile] = process.argv.slice(2);
const files = execFileSync("git", ["ls-files", "-z", "--", "*Cargo.toml"], {cwd: root, maxBuffer: 1 << 28}).toString().split("\0").filter(f => f && !f.includes(".🧬semio") && existsSync(`${root}/${f}`));
const pkgToId = new Map();
for (const f of files) { const t = Bun.TOML.parse(readFileSync(`${root}/${f}`, "utf8")); const id = t.package?.metadata?.component?.package; if (id && ["plugin", "extension"].includes(t.package.metadata.semio?.["component-kind"])) pkgToId.set(t.package.name, id.slice(6)); }
const closure = JSON.parse(readFileSync(closureFile)), targets = JSON.parse(readFileSync(targetsFile));
const res = {};
for (const lane of ["generator", "energy", "fem3d"]) {
  const inferred = targets[`prepare-${lane}-react-prototype`].dependsOn.filter(d => d.endsWith(":component")).map(d => d.slice(0, -10));
  const unknown = inferred.filter(p => !pkgToId.has(p)), ids = inferred.filter(p => pkgToId.has(p)).map(p => pkgToId.get(p)).sort();
  const want = closure.lanes[lane].selected;
  res[lane] = {inferred: ids.length, expected: want.length, missingInInferred: want.filter(i => !ids.includes(i)), extraInInferred: ids.filter(i => !want.includes(i)), unknown};
}
console.log(JSON.stringify(res, null, 1));
