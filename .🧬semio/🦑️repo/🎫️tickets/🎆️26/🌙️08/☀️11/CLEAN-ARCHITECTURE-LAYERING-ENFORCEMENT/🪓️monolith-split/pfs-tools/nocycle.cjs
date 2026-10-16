const base = require("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs");
module.exports = { options: base.options, forbidden: base.forbidden.filter(r => r.name === "no-circular") };
