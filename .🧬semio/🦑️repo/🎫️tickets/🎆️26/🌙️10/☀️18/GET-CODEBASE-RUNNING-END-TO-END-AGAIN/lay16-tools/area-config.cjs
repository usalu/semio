const base = require("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs");
const area = process.env.LAY16_AREA;
const escape = (s) => s.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
const areas = area.split("|").map(escape).join("|");
const forbidden = base.forbidden.filter((r) => !/^no-cross-package-relative/u.test(r.name));
module.exports = { ...base, forbidden, options: { ...base.options, doNotFollow: { ...base.options.doNotFollow, path: `${base.options.doNotFollow.path}|^(?!(${areas})(/|$))` } } };
