/** 🕰️ Writes `🗑️generated/site-final/head-variant.json`: for every entry-relevant source the working tree changed since
 * `HEAD` (quiz core, quiz React target, quiz schema, UI, the site and its quizzes; tests and styles excluded), its text at
 * `HEAD`, keyed by the absolute path as Vite names modules. `head_variant.vite.ts` loads them in place of the tree's.
 * Usage: `bun head_variant.ts` */
import { spawnSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, "../../../../../../..");
const git = (...args: string[]): string => {
  const result = spawnSync("git", ["-c", "core.quotepath=false", ...args], { cwd: repo, encoding: "utf8", maxBuffer: 1 << 28 });
  if (result.status !== 0) throw new Error(result.stderr);
  return result.stdout;
};
const roots = ["🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react", "🧰️framework/🛍️products/❓️quiz/🔨️modules", "🧰️framework/🛍️products/❓️quiz/🧬️schema", "🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript", "🧰️framework/🔨️modules/🖱️ui", "🎓️teaching/🏛️architecture"];
const changed = git("diff", "--name-only", "--diff-filter=M", "HEAD", "--", ...roots).split("\n").filter((path) => /\.(?:ts|tsx|json)$/u.test(path) && !path.includes("🧪️tests") && !path.includes("🎭️e2e") && !path.startsWith("🎓️teaching/🏛️architecture/🐾️pets/"));
const variant = Object.fromEntries(changed.map((path) => [resolve(repo, path).replaceAll("\\", "/"), git("show", `HEAD:${path}`)]));
mkdirSync(resolve(here, "🗑️generated/site-final"), { recursive: true });
writeFileSync(resolve(here, "🗑️generated/site-final/head-variant.json"), JSON.stringify(variant));
console.log(changed.join("\n"));
console.log(`${changed.length} files at HEAD`);
