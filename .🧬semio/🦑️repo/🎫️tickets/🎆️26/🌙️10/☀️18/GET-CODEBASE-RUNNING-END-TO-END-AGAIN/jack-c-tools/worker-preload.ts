import { plugin } from "bun";
const L = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🧠️lsp";
plugin({ name: "alias", setup(build) {
  build.onResolve({ filter: /^@semio-tech\/trinity-jack-lsp$/ }, () => ({ path: `${L}/🧪️tests/📡️worker/🪞️session-double/🟦️.ts` }));
  build.onResolve({ filter: /^@semio-tech\/framework$/ }, () => ({ path: "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🫧️transient/🟦️.ts" }));
} });
