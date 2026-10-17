/** 🔬️ Groups every textual reference to a needle by the owning workspace package (longest sourceOwner prefix). Usage: bun census.ts <policy.json> <needle>... */
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const nfc = (s: string) => s.normalize("NFC");
const [policyPath, ...needles] = process.argv.slice(2);
const policy = JSON.parse(readFileSync(policyPath, "utf8"));
const owners: { src: string; name: string }[] = policy.packages.map((p: any) => ({ src: nfc(p.sourceOwner), name: p.name }));
const bySrc = new Map<string, Set<string>>();
for (const o of owners) { const s = bySrc.get(o.src) ?? new Set(); s.add(o.name); bySrc.set(o.src, s); }
const srcs = [...bySrc.keys()].sort((a, b) => b.length - a.length);
const ownerOf = (f: string) => { const n = nfc(f); for (const s of srcs) if (n === s || n.startsWith(s + "/")) return s; return "(no package) " + n.split("/").slice(0, 3).join("/"); };
for (const needle of needles) {
  let out = "";
  try { out = execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "grep", "-n", "-I", "-F", "--untracked", "-e", needle, "--", ".", ":(exclude).🧬semio/*", ":(exclude)*/🗑️generated/*", ":(exclude)*.lock"], { cwd: ROOT, maxBuffer: 1 << 28 }).toString(); } catch (e: any) { out = e.stdout?.toString() ?? ""; }
  const groups = new Map<string, string[]>();
  for (const line of out.split("\n").filter(Boolean)) { const m = line.match(/^(.*?):(\d+):/u); if (!m) continue; const o = ownerOf(m[1]); (groups.get(o) ?? groups.set(o, []).get(o)!).push(m[1].split("/").slice(-4).join("/") + ":" + m[2]); }
  console.log(`## ${needle}  (${[...groups.values()].reduce((a, b) => a + b.length, 0)} hits, ${groups.size} owners)`);
  for (const [o, l] of groups) console.log(`  ${l.length}x ${o}  e.g. ${l.slice(0, 2).join(" | ")}`);
}
