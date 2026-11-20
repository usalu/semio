/** 🧪️ Prints the print-profile suitability of every authored scale of an asset (catalog or `--file` document): each violation with shapes and measurements. */
import { resolve } from "node:path";
import { resolveRepresentation, DEFAULT_PRINT_PROFILE, printProfile } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { loadAsset } from "./volumes-lib.ts";
import { cli, load } from "./r3-lib.ts";

const { spec, flag } = cli(["--file"]);
const file = flag("--file");
const asset = file === undefined ? load(spec, "r3-sliding-door-suit.ts <asset> [--file f]").asset : loadAsset(resolve(file));
const profile = DEFAULT_PRINT_PROFILE;
for (const s of [50, 100, 200, 500]) {
  const r = resolveRepresentation(asset as never, { denominator: s } as never, profile as never) as any;
  process.stdout.write(`1:${s} ${r.kind} ${r.suitability ? `minStroke ${r.suitability.minStrokeMm} minGap ${r.suitability.minGapMm} minFeature ${r.suitability.minFeatureMm}` : ""}\n`);
  for (const v of r.suitability?.violations ?? []) process.stdout.write(`  ${JSON.stringify(v)}\n`);
}
