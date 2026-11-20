/** 🔎️ Why a representation of an asset is unsuitable: the violations of the default print profile per scale and pose, as `depict` resolves them. */
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { load } from "./r3-lib.ts";
import { loadAsset } from "./volumes-lib.ts";

const arg = process.argv[2]!;
const asset = arg.endsWith(".json") ? loadAsset(arg) : load(arg, "r3-bamboo-clump-why.ts <asset|file> [scales]").asset;
for (const s of (process.argv[3] ?? "50,100,200,500").split(",")) {
  const r = resolveRepresentation(asset as never, { denominator: Number(s) } as never, DEFAULT_PRINT_PROFILE) as { kind: string; suitability?: { violations: unknown[] } };
  const v = r.suitability?.violations ?? [];
  console.log(`1:${s} ${r.kind} violations ${v.length}`);
  const byCheck = new Map<string, number>();
  for (const x of v as { check: string }[]) byCheck.set(x.check, (byCheck.get(x.check) ?? 0) + 1);
  console.log("   ", JSON.stringify(Object.fromEntries(byCheck)));
  for (const x of v.slice(0, 6)) console.log("   ", JSON.stringify(x).slice(0, 260));
}
