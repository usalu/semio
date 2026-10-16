const V = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🔗️graphql-types/";
const { readGraphqlTypesSource } = await import(V + "📥️source/🟦️.ts");
const { generatedTargets } = await import(V + "📋️plan/🟦️.ts");
const source = await readGraphqlTypesSource("/Users/ueli/Documents/semio");
const targets = generatedTargets(source);
if (process.argv[2] === "write") { const { mkdirSync, writeFileSync } = await import("node:fs"); const { dirname } = await import("node:path"); for (const t of targets) { mkdirSync(dirname(t.path), { recursive: true }); writeFileSync(t.path, t.content); console.log("wrote", t.path, t.content.length); } }
else console.log(targets[0].content.slice(0, Number(process.argv[2] ?? 3000)));
console.log("operations", source.operations.map((o) => o.name).join(","));
