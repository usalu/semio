/** 🖨️ Whether an asset prints at given denominators under the default print profile, with the ops applied in memory (`--no-ops` without; `--skip view:id,view:id` leaves out the shape ops of those shapes, to find the op that causes a violation): the violations in full.
 *  bun print.ts <id> <denominator,denominator> [--no-ops] [--skip plan:soundboard] */
import { applyAll } from "./apply.ts";
import { S, catalogFile, opsFile } from "./files.ts";
const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const args = process.argv.slice(2);
const asset = (await Bun.file(catalogFile(args[0]!)).json()).asset;
const skip = new Set(args.includes("--skip") ? args[args.indexOf("--skip") + 1]!.split(",") : []);
if (!args.includes("--no-ops")) applyAll(asset, (JSON.parse(await Bun.file(opsFile(args[0]!)).text()) as any[]).filter((o) => !skip.has(`${o.view}:${o.id ?? o.shape?.id}`)));
for (const denominator of args[1]!.split(",").map(Number)) {
  const resolution = resolveRepresentation(asset, { denominator }, DEFAULT_PRINT_PROFILE);
  console.log(`1:${denominator} ${resolution.kind} ${JSON.stringify(resolution.suitability?.violations ?? [])}`);
}
