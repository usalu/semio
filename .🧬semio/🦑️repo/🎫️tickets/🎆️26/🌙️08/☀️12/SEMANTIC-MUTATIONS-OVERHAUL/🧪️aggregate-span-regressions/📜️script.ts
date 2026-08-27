import { inspectRustMutationAggregateSpan } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

const cases = [
  { name: "root-docs-and-attributes", source: "/// mutation docs\n#[derive(Debug)]\n#[cfg(any())]\npub enum RootMutation { Insert }\n", accepted: true, start: 0 },
  { name: "function-decoy", source: "fn helper() { pub enum InnerMutation { Insert } }\n", accepted: false },
  { name: "inline-module-decoy", source: "mod hidden { pub enum InnerMutation { Insert } }\n", accepted: false },
  { name: "macro-decoy", source: "macro_rules! make { () => { pub enum InnerMutation { Insert } } }\n", accepted: false },
  { name: "unrelated-doc-prefix", source: "/// unrelated\npub struct Other;\n#[derive(Debug)]\npub enum RootMutation { Insert }\n", accepted: true, start: "#[derive" },
  { name: "unclosed-root-item", source: "fn broken() {\npub enum RootMutation { Insert }\n", accepted: false },
] as const;

const results = cases.map((test) => {
  const span = inspectRustMutationAggregateSpan(test.source);
  const accepted = span !== null;
  const start = typeof test.start === "string" ? test.source.indexOf(test.start) : test.start;
  const correct = accepted === test.accepted && (!span || (span.declarationStart === start && test.source[span.bodyOpen] === "{" && test.source[span.bodyClose] === "}"));
  return { name: test.name, accepted, correct, span };
});
console.log(`[DEBUG] ${JSON.stringify(results)}`);
if (results.some((result) => !result.correct)) process.exitCode = 1;
