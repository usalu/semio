const U = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui";
const files = [`${U}/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`, `${U}/🎨️styling/📽️projection/🟦️.ts`, `${U}/🎨️styling/📦️packages/🦀️rust/📜️script.ts`, `${U}/🎨️styling/📦️packages/🟦️typescript/🟦️.ts`, `${U}/🎨️styling/🫧️ephemeral-state/🟦️.ts`, `${U}/🧪️tests/🫧️ephemeral-lane/🟦️.ts`, `${U}/🔨️modules/🗄️storage-port/🟦️.ts`, `${U}/🔨️modules/🐳️dock-skeleton/🟦️.ts`, `${U}/🔨️modules/🆔️element-identity/🟦️.ts`, "/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript/🟦️.ts"];
const cfg = { extends: `${U}/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json`, compilerOptions: { incremental: false, noEmit: true }, files, include: [] };
await Bun.write(process.argv[2]!, JSON.stringify(cfg, null, 1));
