/** 🧭 Verify one shipped digital scale with the same rule evaluator as the complete catalog check. */
import { readFileSync } from "node:fs";
import { checkVolumes, VOLUME_RULES } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import type { Asset, ScaleDenominator } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
const [path, denominator, selected] = process.argv.slice(2);
if (path === "--rules") { console.log(JSON.stringify(VOLUME_RULES)); process.exit(0); }
const asset: Asset = JSON.parse(readFileSync(path!, "utf8")).asset;
const scale = Number(denominator) as ScaleDenominator;
if (![50,100,200,500].includes(scale)) throw new Error("Unknown staffage scale");
const rules = selected ? VOLUME_RULES.filter(rule=>selected.split(",").includes(rule)) : VOLUME_RULES;
if (selected && rules.length !== new Set(selected.split(",")).size) throw new Error("Unknown volume rule");
const issues = checkVolumes(asset,{scale,rules},{ progress: (_done,_total,phase)=>{ if (phase) process.stderr.write(`[DEBUG] ${phase.en}\n`); } });
console.log(JSON.stringify({asset:asset.id,scale,rules,issues:issues.map(i=>[i.code,i.path,i.message.en])}));
process.exitCode=issues.length?1:0;
