import { readFileSync } from "node:fs";
const lib = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const root = "/Users/ueli/Documents/semio";
const { policyReadFileSafe, policyReadModuleTreeSource, policyReadWitPackageSource } = await import(`${lib}/🔍️discovery/📖️source-access/🟦️.ts`);
const { readSourceFamilySource } = await import(`${lib}/../../../../🔨️modules/📚️compiler/📦️packages/🟦️typescript/📖️syntax/🌳️module-tree/🟦️.ts`);
const { policyReadRustPolicyFamily, policyReadRustPolicySource } = await import(`${lib}/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts`);
const spec = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const law = await import(spec.module);
const readNode = (node: any): string => {
  if (node.parts) return node.parts.map(readNode).join(node.join ?? "\n");
  let text = node.reader === "rust-family" ? policyReadRustPolicyFamily(root, node.paths) : node.reader === "rust-source" ? policyReadRustPolicySource(root, node.paths[0]) : node.reader === "module-tree" ? policyReadModuleTreeSource(root, node.paths[0]) : node.reader === "wit-package" ? policyReadWitPackageSource(root, node.paths[0]) : node.reader === "source-family" ? readSourceFamilySource(`${root}/${node.paths[0]}`, { tests: node.tests === true }) : policyReadFileSafe(root, node.paths[0]);
  if (node.names) return node.names.filter((name: string) => text.includes(`${node.prefix}${name}`)).join("\n");
  if (node.slice) text = text.slice(text.indexOf(node.slice[0]), text.indexOf(node.slice[1]));
  return text;
};
const sources: Record<string, string> = {};
for (const [name, def] of Object.entries<any>(spec.sources)) sources[name] = readNode(def);
const outcome = (value: unknown) => (typeof value === "boolean" ? { accepted: value, messages: [] as string[] } : Array.isArray(value) ? { accepted: value.length === 0, messages: value.map(String) } : undefined);
const results: any[] = [];
for (const call of spec.calls) {
  const run = (overrides: Record<string, string>) => outcome((law as any)[call.fn](...call.args.map((arg: any) => (typeof arg === "string" ? (overrides[arg] ?? sources[arg]) : arg.root ? root : arg.lit))));
  let base;
  try { base = run({}); } catch (error) { results.push({ fn: call.fn, error: String(error) }); continue; }
  if (!base) { results.push({ fn: call.fn, error: "result is neither boolean nor list" }); continue; }
  const names = [...new Set(call.args.filter((arg: any) => typeof arg === "string"))] as string[];
  const found: any[] = [];
  for (const text of spec.candidates) for (const target of names) {
    const present = sources[target]!.includes(text);
    for (const op of present ? ["remove"] : /^[A-Za-z0-9 ,'\-\/+"]+$/.test(text) ? [] : ["append"]) {
      const mutated = op === "remove" ? sources[target]!.split(text).join("") : `${sources[target]}\n${text}`;
      let result;
      try { result = run({ [target]: mutated }); } catch { result = { accepted: false, messages: ["<throws>"] }; }
      if (!result) continue;
      const added = result.messages.filter((message) => !base!.messages.includes(message));
      if (result.accepted !== base.accepted || added.length > 0) found.push({ target, op, text, present, mutatedAccepted: result.accepted, added: [...new Set(added)].sort() });
    }
  }
  results.push({ fn: call.fn, baseAccepted: base.accepted, baseMessages: base.messages, found, sizes: Object.fromEntries(names.map((name) => [name, sources[name]!.length])) });
}
process.stdout.write(JSON.stringify(results));
