/** 🔍️ For every refused TypeScript/GraphQL/protobuf row of a dump finds the smallest schema node the type renderer cannot express: `bun spd-ts-why.ts <dump.json>`. */
import { readFileSync } from "node:fs";
const root = process.cwd();
const base = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection`;
const { readDocument, exportsOf } = await import(`${base}/🗂️documents/🟦️.ts`);
const { tsType } = await import(`${base}/📐️type-render/🟦️.ts`);
const rows: string[] = JSON.parse(readFileSync(process.argv[2]!, "utf8")).refused;
const isDict = (v: unknown): v is Record<string, unknown> => typeof v === "object" && v !== null && !Array.isArray(v);
for (const row of rows) {
  const m = /^(🟦️typescript|🔗️graphql|🛰️protobuf) (\S+) (\S+)$/u.exec(row);
  if (m === null) continue;
  const [, format, name, leaf] = m;
  const document = readDocument(root, `${leaf!.slice(0, leaf!.lastIndexOf("/"))}/🔣️.json`);
  const definition = document === undefined ? undefined : exportsOf(document).get(name!);
  if (definition === undefined) { console.log(format, name, "no definition"); continue; }
  const culprit = (node: Record<string, unknown>): Record<string, unknown> => {
    const kids: Record<string, unknown>[] = [];
    if (isDict(node.properties)) for (const v of Object.values(node.properties)) if (isDict(v)) kids.push(v);
    if (isDict(node.items)) kids.push(node.items);
    if (Array.isArray(node.items)) for (const v of node.items) if (isDict(v)) kids.push(v);
    for (const key of ["oneOf", "anyOf", "allOf"]) if (Array.isArray(node[key])) for (const v of node[key] as unknown[]) if (isDict(v)) kids.push(v);
    for (const kid of kids) if (tsType(kid) === undefined) return culprit(kid);
    return node;
  };
  const bad = culprit(definition);
  console.log(format, name, leaf!.split("/")[2], "|", JSON.stringify(bad).slice(0, 200));
}
