/** 🧩 D1: the React target's appearance and i18n modules belong to the ui core (elements use them), so ui no longer imports its own target. */
import { Plan } from "./plan.ts";
import { planMoves } from "./moves.ts";
import { finish } from "./direct.ts";
import { scan } from "../exports-stage-tools/scan.ts";
if (process.env.REFRESH) scan(true);
const U = "🧰️framework/🔨️modules/🖱️ui";
const plan = new Plan();
const r = planMoves(plan, "d1-ui-core-owns-appearance-i18n", "ui elements import appearance and i18n: they belong to ui, the react target re-exports them", [
  { from: `${U}/🎯️targets/⚛️react/🌐️i18n`, to: `${U}/🌐️i18n` },
  { from: `${U}/🎯️targets/⚛️react/🌓️appearance`, to: `${U}/🌓️appearance` },
]);
const TS = "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/tsconfig.json";
const W = "d1-ui-core-owns-appearance-i18n";
plan.edit({ file: TS, from: `"@semio-tech/ui-react/i18n": ["../../../../../${U}/🎯️targets/⚛️react/🌐️i18n/🟦️.ts"]`, to: `"@semio-tech/ui-react/i18n": ["../../../../../${U}/🌐️i18n/🟦️.ts"]`, mode: "text", wave: W, why: "path mapping follows the moved module" });
const RS = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs";
plan.edit({ file: RS, from: `"${U}/🎯️targets/⚛️react/🌓️appearance/🟦️.ts",`, to: `"${U}/🌓️appearance/🟦️.ts",`, mode: "text", wave: W, why: "storage-key census reads the moved module" });
plan.edit({ file: RS, from: `"${U}/🎯️targets/⚛️react/🌐️i18n/🟦️.ts",`, to: `"${U}/🌐️i18n/🟦️.ts",`, mode: "text", wave: W, why: "storage-key census reads the moved module" });
for (const x of r.reports) console.log(x);
finish(plan, "d1", process.argv);
