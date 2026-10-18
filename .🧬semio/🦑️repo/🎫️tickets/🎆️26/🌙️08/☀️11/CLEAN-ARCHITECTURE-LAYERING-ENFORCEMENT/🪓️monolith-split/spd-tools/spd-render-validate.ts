/** ✅️ Renders every export of every schema document under `✏️s` and `🌎️hub` as GraphQL and protobuf and parses the text with the independent parsers (graphql-js, protobufjs): `bun spd-render-validate.ts [under]`. */
import { parse } from "graphql";
import protobuf from "protobufjs";
const root = process.cwd();
const base = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection`;
const { SchemaCatalog, readDocument, exportsOf } = await import(`${base}/🗂️documents/🟦️.ts`);
const { renderGraphql, renderProto, renderTsType } = await import(`${base}/📐️type-render/🟦️.ts`);
const catalog = new SchemaCatalog(root);
let total = 0, refusedGql = 0, refusedProto = 0, refusedTs = 0;
const failures: string[] = [];
for (const under of process.argv[2] ? [process.argv[2]] : ["✏️s", "🌎️hub"]) {
  for (const module of catalog.modules(under)) {
    for (const path of catalog.documents(module)) {
      const document = readDocument(root, path);
      if (document === undefined) continue;
      const exports = exportsOf(document);
      for (const [name, definition] of exports) {
        const restricted = definition["x-semio-formats"];
        total += 1;
        const taken = (other: string) => exports.has(other);
        const gql = renderGraphql(name, definition, taken);
        if (gql === undefined) refusedGql += 1;
        else {
          try { parse(gql); } catch (error) { failures.push(`graphql ${name} ${path}: ${(error as Error).message}`); }
          const declared = [...gql.matchAll(/^(?:type|scalar|enum)\s+(\w+)/gmu)].map((m) => m[1]);
          if (new Set(declared).size !== declared.length) failures.push(`graphql duplicate declaration in ${name} ${path}`);
        }
        const proto = renderProto(name, definition, taken);
        if (proto === undefined) refusedProto += 1;
        else {
          try { protobuf.parse(`syntax = "proto3";\n${proto}`); } catch (error) { failures.push(`proto ${name} ${path}: ${(error as Error).message}`); }
        }
        if (renderTsType(name, definition) === undefined) refusedTs += 1;
        void restricted;
      }
    }
  }
}
console.log(JSON.stringify({ total, refusedGql, refusedProto, refusedTs, failures: failures.length }));
for (const row of failures.slice(0, 40)) console.log(row.slice(0, 400));
