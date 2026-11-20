/** 🧯️ Runs the product `measureVolumes` on the bamboo clump for one scale and prints the full stack when it throws (the check tool shows only the first line). Usage: `bun r3-bamboo-clump-stack.ts <scale> [rule]` */
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { load } from "./r3-lib.ts";

const { asset } = load("vegetation/bamboo-clump", "r3-bamboo-clump-stack.ts <scale> [rule]");
const scale = Number(process.argv[2] ?? 50);
const rules = process.argv[3] ? [process.argv[3]] : undefined;
try {
  const out = measureVolumes(asset as never, { scale: scale as never, ...(rules ? { rules: rules as never } : {}) } as never, { signal: { aborted: false, throwIfAborted() {} }, progress: (d: number, t: number, l: { en: string }) => process.stderr.write(`[DEBUG] ${d}/${t} ${l.en}\n`) } as never);
  for (const m of out as { rule: string; name: string; value: number; limit: number; ok: boolean }[]) if (!m.ok || /iou|bbox|outside|area/i.test(m.name)) console.log(m.ok ? "ok  " : "FAIL", m.rule, m.name, Number(m.value.toFixed(4)), Number(m.limit.toFixed(4)));
} catch (error) {
  console.log((error as Error).stack);
}
