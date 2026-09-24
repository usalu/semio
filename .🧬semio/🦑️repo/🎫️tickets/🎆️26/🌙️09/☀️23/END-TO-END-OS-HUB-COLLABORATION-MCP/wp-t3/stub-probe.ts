import { stubDeserializerBreaches, stubSerializerBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const rows = [...stubSerializerBreaches(root), ...stubDeserializerBreaches(root)];
const pattern = process.argv[2];
const shown = pattern ? rows.filter((r) => JSON.stringify(r).includes(pattern)) : rows;
console.log(`${shown.length} stub rows (${shown.filter((r) => r.id === "stub-serializer").length} serializer, ${shown.filter((r) => r.id === "stub-deserializer").length} deserializer)`);
for (const r of shown) {
  const o = r as unknown as Record<string, string>;
  const m = (o.scope).match(/🔌️plugins\/([^/]+)\/🗿️artifacts\/([^/]+)\/.*?(📤️export|📥️import)\/.*?🗿️artifacts\/([^/]+)\/🔖️([^/]+)/);
  console.log(m ? `${m[1]} ${m[2]} ${m[3]} ${m[4]}@${m[5]} | ${o.summary}` : JSON.stringify(r).slice(0, 300));
}
