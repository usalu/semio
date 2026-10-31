// 🕐️ Latest compile evidence per package from the pattern-scan hits (read-only).
// Input hits: lines "<abs log path>:<match>" from rg -o -H -F -f patterns.txt over 🗑️generated/1029.
import { readFileSync, statSync, existsSync } from "node:fs";

const hitsFile = process.argv[2];
const names = readFileSync(process.argv[3], "utf8").split("\n").filter(Boolean);
const hits = readFileSync(hitsFile, "utf8").split("\n").filter(Boolean);

type Ev = { file: string; mtime: number; kind: "fail" | "ok"; rc?: string };
const byName = new Map<string, Ev[]>();
for (const line of hits) {
  const idx = line.indexOf(".txt:");
  if (idx < 0) continue;
  const file = line.slice(0, idx + 4), match = line.slice(idx + 5);
  const m = match.match(/^(Checking|Compiling) (\S+) v|^could not compile `([^`]+)`/);
  if (!m) continue;
  const name = m[2] ?? m[3];
  const kind = m[3] ? "fail" : "ok";
  const rcFile = file.replace(/\.txt$/, ".rc");
  const rc = existsSync(rcFile) ? readFileSync(rcFile, "utf8").trim() : undefined;
  const mtime = statSync(file).mtimeMs;
  byName.set(name, [...(byName.get(name) ?? []), { file, mtime, kind, rc }]);
}

const fmt = (ms: number) => new Date(ms).toISOString().slice(5, 16).replace("T", " ");
const out: string[] = [];
for (const n of names) {
  const evs = (byName.get(n) ?? []).sort((a, b) => b.mtime - a.mtime);
  if (!evs.length) { out.push(`${n} | NO-EVIDENCE`); continue; }
  const lastFail = evs.find(e => e.kind === "fail");
  const lastOk = evs.find(e => e.kind === "ok" && e.rc === "0");
  const short = (p: string) => p.split("/🗑️generated/")[1] ?? p;
  out.push(`${n} | lastFail=${lastFail ? fmt(lastFail.mtime) + " " + short(lastFail.file) : "-"} | lastOkRc0=${lastOk ? fmt(lastOk.mtime) + " " + short(lastOk.file) : "-"} | hits=${evs.length}`);
}
console.log(out.join("\n"));
