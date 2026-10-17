// 🔎️ Unfiltered layout differential: the same rustc diagnostics as the gate, but without the unlinked-cascade filter, to see what the filter may hide (kept root modules).
// usage: bun raw-layout-diff.ts <keptBaseDir> <keptCombinedDir> [native|test]
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
const [baseDir, combDir, mode] = process.argv.slice(2) as [string, string, string | undefined];
const run = (dir: string, test: boolean): string[] => {
  const root = resolve(dir);
  const r = spawnSync("rustc", ["+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "combined", "--emit=metadata", "--error-format=json", "-o", "/dev/null", ...(test ? ["--test", "--cfg", 'feature="testkit"'] : []), "__combined.rs"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 30 });
  const out: string[] = [];
  for (const line of (r.stderr ?? "").split("\n")) {
    if (!line.startsWith("{")) continue;
    try {
      const m = JSON.parse(line) as { level?: string; code?: { code?: string } | null; message?: string; spans?: { file_name: string; is_primary: boolean }[] };
      if (m.level !== "error" || (m.message ?? "").startsWith("aborting due to")) continue;
      const file = (m.spans?.find((s) => s.is_primary)?.file_name ?? "").replace(/\/\.\/(?:\.\/)*/gu, "/");
      const msg = (m.message ?? "").replace(/crate::__c_\w+::/gu, "crate::").replace(/__c_\w+/gu, "").replace(/\bcomponent::/gu, "");
      out.push([m.code?.code ?? "", msg, file].join("\t"));
    } catch { continue; }
  }
  return out;
};
for (const test of mode === "native" ? [false] : mode === "test" ? [true] : [false, true]) {
  const a = run(baseDir, test), b = run(combDir, test);
  const counts = new Map<string, number>();
  for (const e of a) counts.set(e, (counts.get(e) ?? 0) + 1);
  const only: string[] = [];
  for (const e of b) { const n = counts.get(e) ?? 0; if (n > 0) counts.set(e, n - 1); else only.push(e); }
  console.log(`${test ? "--test" : "native"}: base ${a.length} combined ${b.length} onlyCombined ${only.length}`);
  for (const e of only.slice(0, 25)) console.log("   " + e.slice(0, 330));
}
