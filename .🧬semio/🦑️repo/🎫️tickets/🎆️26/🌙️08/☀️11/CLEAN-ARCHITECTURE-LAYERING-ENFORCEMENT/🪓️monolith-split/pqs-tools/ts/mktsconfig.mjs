import fs from "node:fs";
const [out, ...files] = process.argv.slice(2);
const U = "../../../../../../../../../";
const cfg = {
  extends: `${U}🧰️framework/🛍️products/💻️os/tsconfig.json`,
  compilerOptions: { noEmit: true, incremental: false, tsBuildInfoFile: null },
  include: [],
  files: files.map((f) => U + f),
};
fs.writeFileSync(out, JSON.stringify(cfg, null, 2));
