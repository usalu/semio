const { resolve } = require("node:path");
const base = require("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs");
const area = process.env.LAY13_AREA;
const escape = (s) => s.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
const areas = area.split("|").map(escape).join("|");
module.exports = { ...base, options: { ...base.options, doNotFollow: { ...base.options.doNotFollow, path: `${base.options.doNotFollow.path}|^(?!(${areas})(/|$))` } } };
