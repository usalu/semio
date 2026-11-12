const path = require("node:path");
const folder = path.dirname(require.resolve("playwright-core/package.json", { paths: [process.cwd()] }));
const { BrowserType } = require(path.join(folder, "lib/client/browserType.js"));
const original = BrowserType.prototype.launch;
BrowserType.prototype.launch = function (options = {}) {
  return original.call(this, { ...options, args: [...(options.args ?? []), "--in-process-gpu"] });
};
