import { auditParsers } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";

const [root, under] = process.argv.slice(2);
const drift = auditParsers({ root: root!, under: under || "✏️s" });
const count = (text: string | undefined, needle: string): number => (text ?? "").split(needle).length - 1;
const norm = (text: string): string => text.replace(/\s+/g, "").replace(/key\d?|value\d?|item\d?|index\d?/g, "_");
const classes = new Map<string, string[]>();
for (const row of drift) {
  let cls: string;
  if (row.reason === "unrenderable") cls = "unrenderable";
  else if (count(row.expected, "Object.fromEntries") > count(row.actual, "Object.fromEntries")) cls = "weaker-map";
  else if (norm(row.actual) === norm(row.expected!)) cls = "naming-only";
  else if (count(row.actual, "parse") < count(row.expected, "parse")) cls = "inline-vs-call";
  else cls = "other";
  (classes.get(cls) ?? classes.set(cls, []).get(cls)!).push(`${row.name} ${row.path}`);
}
for (const [cls, rows] of classes) console.log(cls, rows.length, rows.slice(0, 3).map((row) => row.slice(0, 60)).join(" | "));
