/** 🔭 Scans a project's command script for repository paths it reads outside the Cargo module walk of its own crate. */
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { ROOT } from "./lib.ts";

const LITERAL = /(["'`])((?:\\.|(?!\1)[^\\\n])*)\1/gu;
const REPO_PREFIX = /^(✏️s|🧰️framework|🌎️hub|🎓️teaching|🏢️semio-tech|♻️mit-bestand|👴️leutwiler)\//u;
const strip = (s: string) => s.replace(/️/gu, "");

export type Reach = { dirs: string[]; indirect: boolean; unresolved: string[] };

export function scriptPath(projectRoot: string): string | undefined {
  const p = join(ROOT, projectRoot, "📜️script.ts");
  return existsSync(p) ? p : undefined;
}

export function reach(projectRoot: string, owner: string): Reach {
  const script = scriptPath(projectRoot);
  const out: Reach = { dirs: [], indirect: false, unresolved: [] };
  if (!script) return out;
  const text = readFileSync(script, "utf8");
  const seen = new Set<string>();
  for (const m of text.matchAll(LITERAL)) {
    const lit = m[2]!;
    const before = text.slice(Math.max(0, m.index! - 24), m.index!);
    if (/(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s+|\brequire\s*\(\s*)$/u.test(before)) continue;
    if (lit.includes("${") || lit.length < 3) { if (/\$\{[^}]*(?:repoRoot|root)[^}]*\}/u.test(lit) && /[🧪🏅🧰✏]/u.test(lit)) out.indirect = true; continue; }
    let abs: string | undefined;
    if (lit.startsWith("../") || lit.startsWith("./")) abs = resolve(join(ROOT, projectRoot), lit);
    else if (REPO_PREFIX.test(strip(lit)) || REPO_PREFIX.test(lit)) abs = resolve(ROOT, lit);
    if (!abs || !abs.startsWith(ROOT + "/")) continue;
    if (/\/(node_modules|target|dist)\//u.test(abs)) continue;
    if (!existsSync(abs)) { if (/[🧪🏅🧰]/u.test(lit)) out.unresolved.push(lit); continue; }
    const rel = relative(ROOT, abs);
    if (rel === "" || rel.startsWith("..")) continue;
    if (statSync(abs).isFile() && /\.(rs|toml)$/u.test(abs)) continue;
    seen.add(rel);
  }
  out.dirs = [...seen].sort();
  void owner;
  return out;
}
