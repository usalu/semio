/** 📖️ Dry-loads `.storybook/main.ts` and runs `viteFinal` on a stub config; starts no server: `bun sb-load.mjs`. */
const R = process.env.R;
const main = await import(`${R}/.storybook/main.ts`);
const { repositoryCachePartitionRootDataV1 } = await import(`${R}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts`);
process.env.SEMIO_VITE_CACHE_DIRECTORY = `${repositoryCachePartitionRootDataV1(R, "products")}/slf-dry-load`;
const cfg = main.default;
console.log("keys", Object.keys(cfg).join(","));
console.log("staticDirs", JSON.stringify(cfg.staticDirs.map((d) => d.to + " <- " + d.from)));
console.log("addons", JSON.stringify(cfg.addons));
console.log("stories", Array.isArray(cfg.stories) ? cfg.stories.length : typeof cfg.stories);
for (const configType of ["DEVELOPMENT", "PRODUCTION"]) {
  const out = await cfg.viteFinal({ plugins: [], resolve: {} }, { configType });
  console.log(configType, "alias entries", out.resolve.alias.length, "plugins", out.plugins.length, "cacheDir", out.cacheDir);
}
