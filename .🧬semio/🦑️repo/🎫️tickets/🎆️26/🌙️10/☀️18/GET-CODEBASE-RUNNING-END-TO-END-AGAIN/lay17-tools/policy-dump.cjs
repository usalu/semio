/** 🗃️ Dumps the live dependency-cruiser policy (`node policy-dump.cjs <out.json>`). */
const base = require((process.env.SEMIO_ROOT ?? "/Users/ueli/Documents/semio") + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs");
require("fs").writeFileSync(process.argv[2], JSON.stringify({ options: base.options, forbidden: base.forbidden, allowed: base.allowed, allowedSeverity: base.allowedSeverity }));
console.log(Object.keys(base), Object.keys(base.options ?? {}));
