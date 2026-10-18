/** 🔍️ Prints the three renderings of the cases of a corpus draft: `bun spd-golden.ts <draft.json>`; a case lists `taken` names. */
import { readFileSync } from "node:fs";
const root = process.cwd();
const base = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection`;
const { renderGraphql, renderProto, renderTsType } = await import(`${base}/📐️type-render/🟦️.ts`);
const draft = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { cases: { name: string; definition: object; taken?: string[] }[] };
const out = draft.cases.map((c) => {
  const taken = (n: string) => (c.taken ?? []).includes(n);
  return { name: c.name, definition: c.definition, ...(c.taken ? { taken: c.taken } : {}), graphql: renderGraphql("Row", c.definition as never, taken) ?? null, protobuf: renderProto("Row", c.definition as never, taken) ?? null, typescript: renderTsType("Row", c.definition as never) ?? null };
});
console.log(JSON.stringify({ cases: out }, null, 2));
