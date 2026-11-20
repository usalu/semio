/** 🖨️ Round 3 child-on-kick-scooter: prints how every preset scale of an asset document resolves under the default print profile, as a whole and view by view (the other two views reduced to their largest shape), with the measured violations. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { GENERATED } from "./volumes-lib.ts";

const text = readFileSync(process.argv[2] ?? join(GENERATED, "r3-child-on-kick-scooter", "🔣️.json"), "utf8");
const VIEWS = ["elevation", "front", "plan"] as const;
for (const denominator of [50, 100, 200, 500]) {
  for (const [only, pose] of [[undefined, undefined], ...VIEWS.flatMap((v) => ["gliding", "pushing-off", "braking"].map((p) => [v, p]))] as [string | undefined, string | undefined][]) {
    const asset = JSON.parse(text).asset;
    if (pose !== undefined) for (const p of asset.poses) if (p.id !== pose) p.key = true;
    const g = asset.representations[String(denominator)].geometry;
    if (only !== undefined) for (const v of VIEWS) if (v !== only) g[v] = [g[v].find((s: { id: string }) => s.id === "torso" || s.id === "trunk") ?? g[v][0]];
    const r = resolveRepresentation(asset, { denominator } as never, DEFAULT_PRINT_PROFILE) as { kind: string; suitability?: { violations: { check: string; shapes: string[]; measuredMm: number }[] } };
    process.stdout.write(`1:${denominator} ${only ?? "all"} ${pose ?? ""} ${r.kind} ${(r.suitability?.violations ?? []).map((v) => `${v.check}:${v.shapes.join("+")}=${v.measuredMm.toFixed(3)}`).join(" ")}\n`);
  }
}
