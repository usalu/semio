/** 🧮️ S5-GATES P4: every anonymous refusal (`Fault::from(…)`, no code) of one plugin's guest sources with the `history-editing`
 * scope rule that brings it in — a tool-flow text, a tool-flow `fn`, a tool path, or only the "this source publishes `ChildEmit`"
 * rule — grouped by enclosing `fn` and by text family. `bun 🧪️s5-gates-retained-refusals.ts <plugin dir name> [<plugin dir name>…]` */
import { faultNoticeReport } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts";

const CODE = /(?:^|[.-])(?:tool|tools|gesture|gumball|drag|scrub|press|stroke|transaction|history|replay|ink|timeTravel|toolRun|toolTransaction)(?:[.-]|$)/u;
const FN = /(?:^|_)(?:tool|tools|gesture|gumball|drag|scrub|press|stroke|transaction|history|replay|ink|time_travel)(?:_|$)|(?:^|_)(?:document|archive|envelope|retained)_load(?:_|$)|(?:^|_)load_(?:document|archive|envelope)(?:_|$)/u;
const PATH = /🛠️|tool|gesture|gumball|scrub|press|transaction|time-travel/iu;
const repoRoot = new URL("../../../../../../../", import.meta.url).pathname;
const tally = (rows: readonly string[]): string => [...rows.reduce((counts, row) => counts.set(row, (counts.get(row) ?? 0) + 1), new Map<string, number>())].sort((left, right) => right[1] - left[1]).map(([row, count]) => `${row} ${count}`).join(", ");

for (const plugin of process.argv.slice(2)) {
  const under = `✏️s/🔌️plugins/${plugin}`;
  const report = faultNoticeReport(decodeURIComponent(repoRoot), under, "history-editing");
  const anonymous = report.sites.filter((site) => site.code === null && !site.defaulted);
  const reason = (site: (typeof anonymous)[number]): string => {
    const rules = [site.text !== null && CODE.test(site.text) ? "text" : "", site.within !== null && FN.test(site.within) ? "fn" : "", PATH.test(site.path) ? "path" : ""].filter((rule) => rule !== "");
    return rules.length > 0 ? rules.join("+") : site.text === null ? "variable-text" : "source-only";
  };
  console.log(`== ${plugin}: ${anonymous.length} anonymous site(s), ${report.diagnostics.filter((entry) => entry.detail.startsWith("faultAnonymous")).length} faultAnonymous finding(s) in scope`);
  console.log(`   by scope rule: ${tally(anonymous.map(reason))}`);
  console.log(`   by file: ${tally(anonymous.map((site) => site.path.slice(site.path.indexOf("/🪆️subsets/") + 1)))}`);
  console.log(`   by fn: ${tally(anonymous.map((site) => site.within ?? "<module>"))}`);
  console.log(`   by text family: ${tally(anonymous.map((site) => (site.text === null ? "<variable>" : site.text.replace(/[-.][a-z0-9]+$/u, ""))))}`);
  for (const site of anonymous.filter((entry) => reason(entry) === "source-only" || reason(entry) === "variable-text")) console.log(`   ${reason(site)}\t${site.within}\t${site.text}`);
}
