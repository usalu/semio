// 🔎️ Raw (unfiltered) rustc errors located in the hoisted root test files, base layout versus combined layout.
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
const [baseDir, combDir, ...tails] = process.argv.slice(2) as string[];
const run = (dir: string): { file: string; key: string }[] => {
  const r = spawnSync("rustc", ["+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "combined", "--emit=metadata", "--error-format=json", "-o", "/dev/null", "--test", "--cfg", 'feature="testkit"', "__combined.rs"], { cwd: resolve(dir), encoding: "utf8", maxBuffer: 1 << 30 });
  const out: { file: string; key: string }[] = [];
  for (const line of (r.stderr ?? "").split("\n")) {
    if (!line.startsWith("{")) continue;
    try {
      const m = JSON.parse(line) as { level?: string; code?: { code?: string } | null; message?: string; spans?: { file_name: string; is_primary: boolean; line_start?: number }[] };
      if (m.level !== "error") continue;
      const p = m.spans?.find((s) => s.is_primary); if (!p) continue;
      const file = p.file_name.replace(/\/\.\/(?:\.\/)*/gu, "/");
      const msg = (m.message ?? "").replace(/crate::__c_\w+::/gu, "crate::").replace(/__c_\w+/gu, "").replace(/\bcomponent::/gu, "");
      out.push({ file, key: `${m.code?.code ?? ""}\t${msg}` });
    } catch { continue; }
  }
  return out;
};
const a = run(baseDir!), b = run(combDir!);
for (const tail of tails) {
  const fa = a.filter((e) => e.file.endsWith(tail)).map((e) => e.key), fb = b.filter((e) => e.file.endsWith(tail)).map((e) => e.key);
  const counts = new Map<string, number>(); for (const k of fa) counts.set(k, (counts.get(k) ?? 0) + 1);
  const only: string[] = []; for (const k of fb) { const n = counts.get(k) ?? 0; if (n > 0) counts.set(k, n - 1); else only.push(k); }
  console.log(`${tail.slice(-70)}: base ${fa.length} combined ${fb.length} onlyCombined ${only.length}`);
  for (const k of only.slice(0, 20)) console.log("   " + k.slice(0, 300));
}
