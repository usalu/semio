const fs = require("fs");
const lc = require("/Users/ueli/Documents/semio/node_modules/lightningcss");
const root = process.argv[2], snap = process.argv[3];
const strip = (s) => { const n = (s.match(/@source "\.";/g) || []).length; return [s.replace(/@source "\.";/g, ""), n]; };
const [a, na] = strip(lc.bundle({ filename: root, minify: true, errorRecovery: false }).code.toString());
const [b, nb] = strip(lc.transform({ filename: "snap.css", code: fs.readFileSync(snap), minify: true }).code.toString());
const same = a === b && na === 1 && nb === 1;
console.log("bundled root:", a.length, "bytes; original:", b.length, "bytes; one @source each:", na === 1 && nb === 1, "; identical (apart from the @source position):", same);
process.exit(same ? 0 : 1);
