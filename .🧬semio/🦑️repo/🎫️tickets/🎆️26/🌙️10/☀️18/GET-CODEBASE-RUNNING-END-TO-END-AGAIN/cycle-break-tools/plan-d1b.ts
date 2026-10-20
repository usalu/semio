/** 🔗️ D1b: ui files import the appearance and i18n modules that now live inside ui instead of the react target's public keys. */
import { relative, dirname } from "node:path";
import { Plan } from "./plan.ts";
import { finish } from "./direct.ts";
import { posix } from "./lib.ts";
const U = "🧰️framework/🔨️modules/🖱️ui";
const W = "d1b-ui-self-imports";
const plan = new Plan();
const to = (file: string, target: string): string => { const r = posix(relative(dirname(file), target)); return r.startsWith(".") ? r : "./" + r; };
const rows: [string, string, string][] = [
  [`${U}/🔨️modules/📱️ui-mobile-context/🟦️.tsx`, "@semio-tech/ui-react/appearance", `${U}/🌓️appearance/🟦️.ts`],
  [`${U}/🔨️modules/🛟️chrome-safe-area/🟦️.tsx`, "@semio-tech/ui-react/appearance", `${U}/🌓️appearance/🟦️.ts`],
  [`${U}/🧱️elements/🐚️ShellScope/🟦️.tsx`, "@semio-tech/ui-react/i18n", `${U}/🌐️i18n/🟦️.ts`],
  [`${U}/🧱️elements/📨️UIDialog/🟦️.tsx`, "@semio-tech/ui-react/i18n", `${U}/🌐️i18n/🟦️.ts`],
];
for (const [file, from, target] of rows) plan.edit({ file, from, to: to(file, target), mode: "specifier", wave: W, why: "the module lives in ui now" });
finish(plan, "d1b", process.argv);
