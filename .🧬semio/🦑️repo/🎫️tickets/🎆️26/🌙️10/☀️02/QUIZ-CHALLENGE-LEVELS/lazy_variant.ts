/** 💤️ Experiment: writes `🗑️generated/site-final/lazy-variant.json` — the working tree's quiz React sources with the run
 * and results screens loaded lazily by the root (and `TaskGlyph` moved to the chrome), so the entry can be weighed
 * before the real sources are touched. Build with `VARIANT=lazy-variant.json` and `head_variant.vite.ts`.
 * Usage: `bun lazy_variant.ts` */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const target = resolve(here, "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react");
const variant: Record<string, string> = {};
const edit = (path: string, ...pairs: readonly (readonly [string, string])[]): void => {
  const absolute = resolve(target, path);
  let text = readFileSync(absolute, "utf8");
  for (const [from, to] of pairs) {
    if (!text.includes(from)) throw new Error(`${path}: missing ${from}`);
    text = text.replace(from, to);
  }
  variant[absolute.replaceAll("\\", "/")] = text;
};
const glyph = /\/\*\* 🏷️ The icon of each task kind[\s\S]*?\n\}\n/u;
const runText = readFileSync(resolve(target, "🔨️modules/▶️run/🟦️.tsx"), "utf8");
const glyphCode = glyph.exec(runText)![0];
edit("🔨️modules/▶️run/🟦️.tsx", [glyphCode, ""], ["const SteadyTaskView = memo(TaskView);", "const SteadyTaskView = /* @__PURE__ */ memo(TaskView);"],['import { BodyButton, CardAction, CardIcon, Dialog, Glyph,', 'import { TaskGlyph, BodyButton, CardAction, CardIcon, Dialog, Glyph,']);
edit("🔨️modules/🪟️chrome/🟦️.tsx", ['import type { Icon as QuizIcon, Motion } from "@semio-tech/quiz";', 'import type { Icon as QuizIcon, Motion, SheetTask, TaskKind } from "@semio-tech/quiz";'], ["export function CardIcon(", `${glyphCode.replace("readonly icon?: Icon }", "readonly icon?: QuizIcon }")}\nexport function CardIcon(`]);
edit("🔨️modules/📖️quiz-page/🟦️.tsx", ['import { TaskGlyph } from "../▶️run/🟦️.tsx";\n', ""], ["import { CardAction, CardIcon,", "import { TaskGlyph, CardAction, CardIcon,"]);
edit("🔨️modules/🏁️results/🟦️.tsx", ["const HEAD = cn(", "const HEAD = /* @__PURE__ */ cn("], ["const NUMBER = cn(", "const NUMBER = /* @__PURE__ */ cn("], ["const ANSWER = cn(", "const ANSWER = /* @__PURE__ */ cn("], ['import { TaskGlyph } from "../▶️run/🟦️.tsx";\n', ""], ["import { CardAction, CardIcon,", "import { TaskGlyph, CardAction, CardIcon,"]);
edit(
  "🟦️.tsx",
  ["import { StrictMode, useEffect,", "import { StrictMode, Suspense, lazy, useEffect,"],
  ['import { RunScreen } from "./🔨️modules/▶️run/🟦️.tsx";\nimport { ResultsScreen } from "./🔨️modules/🏁️results/🟦️.tsx";\n', ""],
  ["export { RunScreen, TASK_KIND_ICONS, TaskClock, TaskGlyph, TaskView,", "export { TASK_KIND_ICONS, TaskGlyph } from \"./🔨️modules/🪟️chrome/🟦️.tsx\";\nexport { RunScreen, TaskClock, TaskView,"],
  ["function useRootTextScale(", 'const RunScreen = lazy(() => import("./🔨️modules/▶️run/🟦️.tsx").then((module) => ({ default: module.RunScreen })));\nconst ResultsScreen = lazy(() => import("./🔨️modules/🏁️results/🟦️.tsx").then((module) => ({ default: module.ResultsScreen })));\n\nfunction useRootTextScale('],
  ["<RunScreen key={step.run}", "<Suspense fallback={waiting}><RunScreen key={step.run}"],
  ["others={preferences.others} />\n        </Page>\n      );\n    case \"results\":", "others={preferences.others} /></Suspense>\n        </Page>\n      );\n    case \"results\":"],
  ["<ResultsScreen session={session}", "<Suspense fallback={waiting}><ResultsScreen session={session}"],
);
const root = variant[resolve(target, "🟦️.tsx").replaceAll("\\", "/")]!;
variant[resolve(target, "🟦️.tsx").replaceAll("\\", "/")] = root.replace(/(<ResultsScreen session=\{session\}[^\n]*?\/>)/u, "$1</Suspense>");
mkdirSync(resolve(here, "🗑️generated/site-final"), { recursive: true });
writeFileSync(resolve(here, "🗑️generated/site-final/lazy-variant.json"), JSON.stringify(variant));
console.log(Object.keys(variant).join("\n"));
