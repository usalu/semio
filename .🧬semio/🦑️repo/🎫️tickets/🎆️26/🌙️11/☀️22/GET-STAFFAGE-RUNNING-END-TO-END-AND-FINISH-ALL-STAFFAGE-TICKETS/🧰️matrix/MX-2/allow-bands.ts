/** 🗝️ In which bands an entry of the derived-view allowlist is needed: `bun allow-bands.ts` evaluates every asset of `derivedViewAllowlist` without any allowlist and prints per entry the authored bands in which `derived-view-in-band` of its view is found, beside the bands the entry names. Nothing is written. Run it from `T`.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts — `derivedFindings` */
import { join } from "node:path";
import { S } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
for (const entry of engine.allowlists().derived) {
  const asset = engine.loadAsset(entry.asset);
  const keys = Object.keys(asset.representations).filter((candidate) => engine.authoredOf(asset, candidate) !== undefined);
  const found = keys.flatMap((key) => engine.derivedFindings(asset, key, Number(key), undefined, { derived: [], bandCopy: [] }).filter((finding: any) => finding.check === "derived-view-in-band" && finding.view === entry.view).map((finding: any) => `${key}: ${finding.message}`));
  console.log(`${entry.asset} ${entry.view} named ${entry.bands?.join(",") ?? "every band"} of ${keys.join(",")}; needed in\n   ${found.join("\n   ")}`);
}
