import { SchemaCatalog, exportsOf, readDocument } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🗂️documents/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const catalog = new SchemaCatalog(root);
const ex: Record<string, string[]> = {};
const add = (k: string, v: string) => { (ex[k] ??= []).length < 6 && ex[k].push(v); };
const walk = (node: any, where: string, trail: string) => {
  if (!node || typeof node !== "object") return;
  if (Array.isArray(node)) { node.forEach((n, i) => walk(n, where, trail + "[" + i + "]")); return; }
  if (node.additionalProperties === false) {
    const hasProps = node.properties && Object.keys(node.properties).length > 0;
    if (!hasProps) add("closed-noprops", `${where.slice(-90)} @${trail} keys=${Object.keys(node).join(",")}`);
    if (node.allOf) add("closed-allOf", `${where.slice(-90)} @${trail} keys=${Object.keys(node).join(",")}`);
    if (node.oneOf || node.anyOf) add("closed-oneOf", `${where.slice(-90)} @${trail}`);
    if (node.patternProperties) add("closed-pattern", `${where.slice(-90)} @${trail}`);
    if (node.type !== "object" && !Array.isArray(node.type)) add("closed-nontype", `${where.slice(-90)} @${trail} type=${node.type}`);
    if (Array.isArray(node.type)) add("closed-typearray", `${where.slice(-90)} @${trail} type=${JSON.stringify(node.type)}`);
  }
  for (const [k, v] of Object.entries(node)) if (!["enum", "const", "default", "examples"].includes(k)) walk(v, where, trail + "/" + k);
};
for (const scope of ["✏️s", "🌎️hub"]) for (const module of catalog.modules(scope)) for (const path of catalog.documents(module)) {
  const doc = readDocument(root, path); if (!doc) continue; walk(doc, path, "");
}
for (const [k, v] of Object.entries(ex)) console.log(k, "\n  " + v.join("\n  "));
