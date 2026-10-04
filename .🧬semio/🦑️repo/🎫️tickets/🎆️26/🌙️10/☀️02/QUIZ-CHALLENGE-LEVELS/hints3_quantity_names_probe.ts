/** 🔭️ Lists every quantity and axis name of the live quizzes (label, short, scale, additive) — input for the round-3 hint wording. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = "C:/git/semio/🎓️teaching/🏛️architecture";
const files = ["⚡️energy/❄️cooling/❓️quiz/🔣️.json", "⚡️energy/📊️demand/❓️quiz/🔣️.json", "⚡️energy/🔥️heating/❓️quiz/🔣️.json", "⚡️energy/🧲️physics/❓️quiz/🔣️.json", "❓️quiz/🔣️.json"];
type Named = { label?: { en: string; de: string }; short?: { en: string; de: string } };
const show = (named: Named): string => `${named.label?.en} | ${named.short?.en ?? "-"} || ${named.label?.de} | ${named.short?.de ?? "-"}`;

const walk = (node: unknown, file: string, path: string): void => {
  if (Array.isArray(node)) return node.forEach((child, index) => walk(child, file, `${path}[${index}]`));
  if (node === null || typeof node !== "object") return;
  const record = node as Record<string, unknown>;
  if (record["quantity"] && typeof record["quantity"] === "object") {
    const quantity = record["quantity"] as Named & { scale: string; additive: boolean; unit: string };
    console.log(`[DEBUG] ${file} quantity ${quantity.scale} additive=${quantity.additive} unit=${quantity.unit}: ${show(quantity)}`);
  }
  if (Array.isArray(record["axes"])) for (const axis of record["axes"] as Named[]) console.log(`[DEBUG] ${file} axis: ${show(axis)}`);
  for (const [key, child] of Object.entries(record)) walk(child, file, `${path}.${key}`);
};

for (const file of files) walk(JSON.parse(readFileSync(join(root, file), "utf8")), file.split("/").slice(-3, -2)[0] ?? file, "");
