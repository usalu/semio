const fs = require("fs");
const lc = require("/Users/ueli/Documents/semio/node_modules/lightningcss");
const root = process.argv[2], snap = process.argv[3];
const a = lc.bundle({ filename: root, minify: true, errorRecovery: false }).code.toString();
const b = lc.transform({ filename: "snap.css", code: fs.readFileSync(snap), minify: true }).code.toString();
console.log("bundled root:", a.length, "bytes; original:", b.length, "bytes; identical:", a === b);
process.exit(a === b ? 0 : 1);
