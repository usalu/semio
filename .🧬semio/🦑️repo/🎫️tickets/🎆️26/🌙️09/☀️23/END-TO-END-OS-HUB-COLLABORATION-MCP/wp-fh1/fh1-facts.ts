/** 🔬️ FH1 debug: the fault facts (and blanked production code) the census reads from one overlay source. */
const overlay = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults";
const { faultFactsOfText, rustProductionCode } = await import(`${overlay}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts`);
const { readFileSync } = await import("node:fs");
const path = process.argv[2];
const text = readFileSync(`${overlay}/${path}`, "utf8");
const code = rustProductionCode(text).split("\n");
const lines = text.split("\n");
const blanked = lines.map((line: string, index: number) => (line.trim() !== "" && code[index].trim() === "" ? index + 1 : 0)).filter(Boolean);
const ranges: string[] = []; for (const line of blanked) { const last = ranges.at(-1)?.split("-"); if (last && Number(last[1] ?? last[0]) === line - 1) ranges[ranges.length - 1] = `${last[0]}-${line}`; else ranges.push(`${line}`); } console.log(`blanked ${blanked.length}: ${ranges.join(" ")}`);
const facts = faultFactsOfText(path, text);
console.log(JSON.stringify({ raises: facts.raises.map((r: { code: string }) => r.code).slice(0, 40), violations: facts.violations.length }));
