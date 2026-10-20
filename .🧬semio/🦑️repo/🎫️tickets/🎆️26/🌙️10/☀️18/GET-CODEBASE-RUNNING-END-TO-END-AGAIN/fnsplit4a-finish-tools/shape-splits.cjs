// 🧪 Lists "<dir>|<only>" rows (new phase dirs of every applied split) for shape.mjs. usage: node shape-splits.cjs
const fs = require("fs"), path = require("path");
const OV = "/private/tmp/fnsplit4a-finish/ov", W = "/private/tmp/fnsplit4a-finish/work";
for (const f of fs.readdirSync(W + "/specs").filter((x) => /^p-.*\.json$/.test(x))) {
  const id = f.slice(2, 10), s = JSON.parse(fs.readFileSync(`${W}/specs/${f}`, "utf8"));
  const ext = path.extname(s.file), snap = `${W}/orig/p-${id}${ext}`;
  if (!fs.existsSync(snap) || fs.readFileSync(snap, "utf8") === fs.readFileSync(s.file, "utf8")) continue;
  const names = [...(s.files || []).map((x) => x.dir), ...(s.shared ? [s.shared.dir] : []), ...(s.helpers ? [s.helpers.dir] : [])];
  console.log(path.relative(OV, path.dirname(s.file)) + "|" + names.join(","));
}
