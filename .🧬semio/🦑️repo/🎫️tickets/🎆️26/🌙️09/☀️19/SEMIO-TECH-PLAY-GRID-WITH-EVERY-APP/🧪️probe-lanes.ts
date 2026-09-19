import { runtimeComponentClosure } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { PLAY_RUNTIME_TARGETS, playRuntimeComponentIds } from "../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🟦️.ts";
const all = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
const union = new Set(playRuntimeComponentIds()), covered = new Set<string>(), lanes: string[] = [];
const closures = PLAY_RUNTIME_TARGETS.map(t => ({ variant: t.variant, ids: runtimeComponentClosure(all, [t.pluginId]) as string[] }));
for (const c of closures) if (c.ids.includes("stdio")) console.log("stdio in", c.variant);
while (covered.size < union.size) {
  const best = closures.map(c => ({ c, gain: c.ids.filter(id => !covered.has(id)).length })).sort((a, b) => b.gain - a.gain)[0]!;
  if (best.gain === 0) throw new Error("uncoverable");
  lanes.push(best.c.variant); for (const id of best.c.ids) covered.add(id);
  console.log(best.c.variant, best.gain);
}
console.log(lanes.length, JSON.stringify(lanes));
