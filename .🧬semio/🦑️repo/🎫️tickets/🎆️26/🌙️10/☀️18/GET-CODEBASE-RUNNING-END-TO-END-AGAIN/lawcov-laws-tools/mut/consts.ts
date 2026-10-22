const lib = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const files = await import(`${lib}/⚖️laws/🔬️interactivity-audit/🗂️files/🟦️.ts`);
const access = await import(`${lib}/🔍️discovery/📖️source-access/🟦️.ts`);
const out: Record<string, unknown> = {};
for (const [name, value] of Object.entries(files)) if (typeof value === "string" || (Array.isArray(value) && value.every((item) => typeof item === "string"))) out[name] = value;
const evidence = await import(`${lib}/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts`);
out.OS_STORE_ROOT_SOURCES = [...evidence.OS_STORE_ROOT_SOURCES];
out.POLICY_SPLIT_SOURCE_ROOTS = [...access.POLICY_SPLIT_SOURCE_ROOTS];
process.stdout.write(JSON.stringify(out));
