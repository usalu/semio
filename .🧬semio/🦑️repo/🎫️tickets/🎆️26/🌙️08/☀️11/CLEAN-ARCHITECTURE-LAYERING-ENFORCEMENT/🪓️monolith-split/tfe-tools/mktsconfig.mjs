import fs from "node:fs";
const [out, listFile, ...extra] = process.argv.slice(2);
const U = "../../../../../../../../../";
const files = fs.readFileSync(listFile, "utf8").split("\n").filter(Boolean).concat(extra).filter((f) => fs.existsSync("/Users/ueli/Documents/semio/" + f));
fs.writeFileSync(out, JSON.stringify({ extends: `${U}🧰️framework/🛍️products/💻️os/tsconfig.json`, compilerOptions: { noEmit: true, incremental: false, tsBuildInfoFile: null }, include: [], files: files.map((f) => U + f) }, null, 2));
console.log(files.length);
