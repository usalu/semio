// 🧩 helpers for ksplit specs (emoji carry U+FE0F explicitly)
const V = "️";
exports.R = "/Users/ueli/Documents/semio/";
exports.dir = (e, n) => e + V + n;
exports.doc = (e, t) => `/** ${e}${V} ${t} */`;
exports.g = (e, n, fn, fe, t, ft, from, to) => ({ dir: exports.dir(e, n), doc: exports.doc(e, t), name: fn, emoji: fe, fdoc: ft, from, to });
exports.kit = (e, n, name, em, t, ft, typeName) => ({ dir: exports.dir(e, n), doc: exports.doc(e, t), name, emoji: em, fdoc: ft, typeName });
exports.write = (spec, out) => require("fs").writeFileSync(out, JSON.stringify(spec, null, 1));
