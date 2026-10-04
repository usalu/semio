/** 🔎️ S4-GATES triage of the `history-editing` fault-notice scope: every in-scope anonymous site and every in-scope coded finding,
 * with the scope rule(s) that admitted it (`path`, `fn:<name>`, `emit`, `text`, `framework`). */
import { readFileSync, writeFileSync } from "node:fs";
import { faultNoticeReport } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const all = faultNoticeReport(repoRoot, "", "all");
const scoped = faultNoticeReport(repoRoot, "", "history-editing");
const CODE = /(?:^|[.-])(?:tool|tools|gesture|gumball|drag|scrub|press|stroke|transaction|history|replay|ink|timeTravel|toolRun|toolTransaction)(?:[.-]|$)/u;
const PATH = /🛠️|tool|gesture|gumball|scrub|press|transaction|time-travel/iu;
const FN = /(?:^|_)(?:tool|tools|gesture|gumball|drag|scrub|press|stroke|transaction|history|replay|ink|time_travel)(?:_|$)|(?:^|_)(?:document|archive|envelope|retained)_load(?:_|$)|(?:^|_)load_(?:document|archive|envelope)(?:_|$)/u;
const SRC = /\bChildEmit\b|\bEmit::[a-z_]*drag[a-z_]*\b/u;
const emitting = new Map<string, boolean>();
const isEmitting = (path: string): boolean => {
  if (!emitting.has(path)) emitting.set(path, !path.startsWith("🧰️framework/") && SRC.test(readFileSync(`${repoRoot}/${path}`, "utf8")));
  return emitting.get(path)!;
};
type Site = (typeof all.sites)[number];
const reasons = (site: Site): string[] => site.defaulted ? [] : [
  ...(site.path.split("/").some((segment) => PATH.test(segment)) ? ["path"] : []),
  ...(site.within !== null && FN.test(site.within) ? [`fn:${site.within}`] : []),
  ...(isEmitting(site.path) ? ["emit"] : []),
  ...((site.code ?? site.text) !== null && CODE.test((site.code ?? site.text)!) ? ["text"] : []),
];
const owner = (path: string): string => (path.startsWith("🧰️framework/") ? "🔌️plugin" : path.split("/")[2]!);
const anonymous = all.sites.filter((site) => (site.code === null || site.code === "plugin.internal") && site.within !== "plugin_sdk_fault").map((site) => ({ kind: "anonymous", owner: owner(site.path), path: site.path, within: site.within, code: site.code, text: site.text, why: reasons(site) })).filter((row) => row.why.length > 0);
const coded = scoped.diagnostics.filter((d) => !d.detail.startsWith("faultAnonymous")).map((d) => {
  const [verdict, ...rest] = d.detail.split(": ");
  const detail = rest.join(": ");
  const code = /^([^ ]+) \(/u.exec(detail)?.[1] ?? null;
  const sites = code === null ? [] : all.sites.filter((site) => site.code === code && owner(site.path) === owner(d.path));
  return { kind: verdict!, owner: owner(d.path), path: d.path, code, detail, why: [...new Set(sites.flatMap(reasons))], within: [...new Set(sites.map((site) => site.within))], at: sites.map((site) => site.path).filter((path, index, all) => all.indexOf(path) === index) };
});
const anonScoped = scoped.diagnostics.filter((d) => d.detail.startsWith("faultAnonymous")).length;
writeFileSync(process.argv[2]!, `${JSON.stringify({ scopedTotal: scoped.diagnostics.length, anonScoped, anonTriaged: anonymous.length, anonymous, coded }, null, 1)}\n`);
console.log(scoped.diagnostics.length, anonScoped, anonymous.length, coded.length);
