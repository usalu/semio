const [vscodeDir, outDir] = process.argv.slice(2);
const { build } = await import("/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js");
const { extensionBuildConfig } = await import(vscodeDir + "/🏗️builder/🟦️.ts");
await build(extensionBuildConfig(vscodeDir + "/📦️packages/🟦️typescript", "🟦️.ts", outDir, "extension.js", false, "prototype"));
console.log("built", outDir);
