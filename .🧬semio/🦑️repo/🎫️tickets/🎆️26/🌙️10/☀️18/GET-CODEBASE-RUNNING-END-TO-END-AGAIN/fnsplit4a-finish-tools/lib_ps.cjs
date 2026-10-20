// 🧩 helpers for psplit specs
const V = "️";
exports.R = "/Users/ueli/Documents/semio/";
exports.dir = (e, n) => e + V + n;
exports.doc = (e, t) => `/** ${e}${V} ${t} */`;
exports.ph = (name, emoji, d, from, to) => ({ name, emoji, doc: d, from, to });
exports.f = (e, n, t, phases) => ({ dir: exports.dir(e, n), doc: exports.doc(e, t), phases });
exports.write = (spec, out) => require("fs").writeFileSync(out, JSON.stringify(spec, null, 1));
