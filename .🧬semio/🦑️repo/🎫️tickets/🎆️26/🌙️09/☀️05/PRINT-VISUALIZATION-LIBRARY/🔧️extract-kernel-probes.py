"""🔬️ Moves the twin's d3-driven check table out of `📜️script.ts` and into the taxonomy's own
probe directory, so the oracle-purity rule sees the measurement tool where the taxonomy says a
measurement tool lives and the package's production source imports no registered oracle."""
import io, os

os.chdir(os.path.join(os.environ["SEMIO_ROOT"], "🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript"))
with io.open("📜️script.ts", encoding="utf-8") as handle:
    lines = handle.read().split("\n")

header = """/** 🔬️ The differential check table of `@semio-tech/print-viz-kernel`: every kernel module measured
 * against the d3 package registered as its oracle. d3 is a devDependency reached from HERE and
 * nowhere else, which is why this table lives in the taxonomy's probe directory rather than beside
 * the library — the kernel modules themselves depend on nothing outside this repository.
 */
import * as scale from "../../../📐scale/🟦️.ts";
import * as format from "../../../🔢format/🟦️.ts";
import * as transform from "../../../🧮transform/🟦️.ts";
import * as mark from "../../../✒️mark/🟦️.ts";
import * as shape from "../../../🥧shape/🟦️.ts";
import * as coordinate from "../../../🧭coordinate/🟦️.ts";
import * as hierarchy from "../../../🌳hierarchy/🟦️.ts";
import * as network from "../../../🕸️network/🟦️.ts";
import * as flow from "../../../🌊flow/🟦️.ts";
import * as geo from "../../../🌍geo/🟦️.ts";
import * as spatial from "../../../📍spatial/🟦️.ts";
import * as theme from "../../../🎨theme/🟦️.ts";
import * as render from "../../../🖼️render/🟦️.ts";

//#region 🔖️Level
/** 🎚️ The three sampling densities every check table honours. */
export const VIZ_KERNEL_LEVELS = ["quick", "long", "exhaustive"] as const;

export type Level = (typeof VIZ_KERNEL_LEVELS)[number];
//#endregion 🔖️Level
""".split("\n")

checks = lines[21:1579]
checks = [line for line in checks]
body = "\n".join(header + checks + ["//#endregion 🔖️Checks", ""])
body = body.replace("\ntype VizKernelCheck = {", "\nexport type VizKernelCheck = {")
body = body.replace("\nasync function vizKernelChecks(", "\nexport async function vizKernelChecks(")
with io.open("tmp-probes.ts", "w", encoding="utf-8", newline="\n") as handle:
    handle.write(body)
os.makedirs("🔬️probes", exist_ok=True)
os.replace("tmp-probes.ts", os.path.join("🔬️probes", "🟦️.ts"))

script = lines[:7] + [
    'import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";',
    'import { VIZ_KERNEL_LEVELS, vizKernelChecks, type Level } from "./🔬️probes/🟦️.ts";',
] + lines[1579:]
text = "\n".join(script)
text = text.replace('const LEVELS = ["quick", "long", "exhaustive"] as const;\n\ntype Level = (typeof LEVELS)[number];\n\n', "const LEVELS = VIZ_KERNEL_LEVELS;\n\n")
text = text.replace(
    " * `test` is the kernel's own differential harness: every module is checked against the d3 package\n * that is its registered oracle. d3 is a devDependency and is imported HERE only — the kernel\n * modules themselves have no runtime dependency on anything outside this repository.",
    " * `test` runs the kernel's own differential harness: every module measured against the d3 package\n * that is its registered oracle. The check table and its d3 imports live in `🔬️probes/🟦️.ts`; the\n * kernel modules themselves have no runtime dependency on anything outside this repository.",
)
with io.open("tmp-script.ts", "w", encoding="utf-8", newline="\n") as handle:
    handle.write(text)
os.replace("tmp-script.ts", "📜️script.ts")
print("probes lines", len(body.split("\n")), "script lines", len(text.split("\n")))
