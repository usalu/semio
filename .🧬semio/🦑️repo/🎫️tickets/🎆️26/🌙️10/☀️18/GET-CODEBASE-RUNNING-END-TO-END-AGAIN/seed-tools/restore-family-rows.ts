const R = "/Users/ueli/Documents/semio/";
const OUT = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/seed-run/";
const LIB = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/";
const catalog = await import(LIB + "🚀️launch-catalog/🟦️.ts");
const old = JSON.parse(await Bun.file(OUT + "old-launch.json").text());
const seed = catalog.readLaunchSeed(R.slice(0, -1)) as any;
const want = ["🛠️dev📋️", "🛠️serve📋️", "⚖️source-check📋️"];
const names = () => new Map(seed.configurations.map((c: any, i: number) => [typeof c === "string" ? c : c.name, i]));
for (const name of want) {
  if (names().has(name)) continue;
  const at = old.configurations.findIndex((c: any) => c.name === name);
  const entry = old.configurations[at];
  const before = old.configurations.slice(0, at).reverse().find((c: any) => names().has(c.name));
  const index = before ? (names().get(before.name) as number) + 1 : seed.configurations.length;
  seed.configurations.splice(index, 0, entry);
  console.log("restored", name, "at", index);
}
catalog.writeLaunchSeed(R.slice(0, -1), seed);
console.log(catalog.writeLaunchCatalog(R.slice(0, -1)));
