/** 🧾 Assembles the final audit report from the header and the selected finding rows. */
import { readFileSync, writeFileSync } from "node:fs";
const base = process.argv[2];
const header = readFileSync(`${base}/cache-audit-tools/header.md`, "utf8");
const rows = readFileSync(`${base}/cache-audit-tools/top50.md`, "utf8").split("\n").filter((l) => l.startsWith("| "));
const cells = (l: string) => l.trim().replace(/^\|/, "").replace(/\|\s*$/, "").split(" | ").map((s) => s.trim());
const kept: string[][] = [];
for (const l of rows) {
  const c = cells(l); const [, file, target, issue, fix, sev] = c;
  if (/leutwiler/.test(file!) && target === "build") continue;
  if (/window-config/.test(target!)) continue;
  if (/^`📋️project\.json`$/.test(file!) && target === "test-storybook") { kept.push([file!, target!, issue!, fix!, sev!]); continue; }
  let s = sev!;
  if (issue!.startsWith("Output resolves outside")) s = "medium";
  let f = file!, t = target!, i = issue!, x = fix!;
  if (t === "build" && /🖼️assets/.test(f)) i = "Cached build writes to the module root (`🖼️assets/README.md`, `🔣️icons/…`, `🌱️metabolism/…`), outside the package root and owned by other trees.";
  if (f.startsWith("📋️project.json (workspace nx.json")) { f = "`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎯️target-policy/🟨️.mjs` (`targetPolicy`)"; i = "Fail-open cache policy: any target that matches no family regex gets `cache: target.cache !== false`, so unclassified targets are cached by default with implicit inputs (563 explicit targets)."; x = "Fail closed: unclassified targets default to `cache: false`; require an explicit `cache` flag on new targets."; s = "high"; }
  kept.push([f, t, i, x, s]);
}
const body = kept.map((c, n) => `| ${n + 1} | ${c.map((v) => v.replace(/\|/g, "/")).join(" | ")} |`);
const footer = `
Notes on the table: the \`graph-generate\`, \`typegen\`, and \`generate-actor-network\` outputs land one level above the package root but stay within their owning artifact or module; they are medium because cache ownership is blurred, not because they leave the workspace. The \`prepare\` row is the one to fix first. Rows 1 to 3 and the \`browserBootSources\` row are the strongest because they are visible in project.json without the inference.

## Final classification

- Fix now (A): \`prepare\`, dev \`build\` (\`inputs: []\`), wgpu \`browserBootSources\`, the seven \`options.env\` gaps.
- Confirm on the generated graph (B): producer targets with empty outputs, outside-project outputs, emoji-prefixed generators.
- Policy (C): fail-open fallthrough, name-regex classification, workspace lock and toolchain files outside \`default\`.
- Could cache (D): six candidates, with \`test-bestest-regenerate\` excluded.
- Bazel (E): snapshot-only outputs, restore unverified.

Scratch and raw output: \`cache-audit-tools/\` (scripts, \`pj-list.txt\`, \`rows-all.md\`) and \`🗑️generated/cache-audit-scan.json\`.
`;
writeFileSync(`${base}/audit-cache-correctness-1020.md`, header + "| # | File | Target | Issue | Suggested fix | Severity |\n|---|---|---|---|---|---|\n" + body.join("\n") + "\n" + footer);
console.log("rows", body.length);
