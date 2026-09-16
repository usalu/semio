import { mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { developmentRuntimeRoot, parseActivationReceipt, publishActivationReceipt, readActivationReceipt, type ActivationReceipt } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { demonstratorRuntimeComponentIds } from "../📦️assets/🟦️.ts";
import { demonstratorRuntimeBuildVariants, demonstratorRuntimePluginId } from "../🟦️.ts";
import type { ActivationComponentSpec } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts";
import { runtimeComponentClosure } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { moduleDirectoryName } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

//#region 🔖️DemonstratorActivationLanes
/** @emoji 🛣️ The pane whose own activation lane carries the Demonstrator's OWN component closure — the
 * `demonstrator` plugin and everything it depends on. Every other lane exists only to add a component
 * that closure does not already reach. */
export const DEMONSTRATOR_PRIMARY_ACTIVATION_LANE = "generator";

/** @emoji 📦️ Repository-relative root of the `@semio-tech/framework-os-dev` package every lane's
 * `activate-<lane>-react-dev` target writes its receipt below. */
const OS_DEV_PACKAGE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";

/** @emoji 🧾️ The DEMONSTRATOR-OWNED directory holding the merged receipt this product's Vite server
 * consumes. It is deliberately not one of the framework's per-variant lane directories: no single
 * `activate-<variant>-react-dev` target can ever produce the Demonstrator's cross-app union, because
 * each of them publishes exactly one playground session's plugin list. */
export const DEMONSTRATOR_UNION_RECEIPT_DIRECTORY = "♻️mit-bestand/🧺️demonstrator/dist/♻️activation/dev";

/** @emoji 🛣️ The activation lanes whose receipts together cover the Demonstrator's runtime union.
 *
 * The primary lane already carries its own component closure, so a pane's runtime variant earns a lane
 * of its own only when its plugin is NOT inside an already-covered closure. That is what keeps
 * `generation3d` out: `procedural` is reached through `demonstrator`, so the `generation3d` lane would
 * contribute no component while adding a second, independently cached receipt for eleven shared plugins
 * — and two lanes disagreeing about one plugin's `artifactSha256` is precisely the staleness this merge
 * refuses (the two receipts on disk disagree about all eleven today). `energy` and `fem` are genuinely
 * unreachable from `demonstrator`, so those two lanes are required. */
export function demonstratorActivationLanes(): readonly string[] {
  const components = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
  const covered = new Set(runtimeComponentClosure(components, [demonstratorRuntimePluginId(DEMONSTRATOR_PRIMARY_ACTIVATION_LANE)]));
  const lanes = [DEMONSTRATOR_PRIMARY_ACTIVATION_LANE];
  for (const variant of demonstratorRuntimeBuildVariants(DEMONSTRATOR_PRIMARY_ACTIVATION_LANE)) {
    const pluginId = demonstratorRuntimePluginId(variant);
    if (covered.has(pluginId)) continue;
    for (const id of runtimeComponentClosure(components, [pluginId])) covered.add(id);
    lanes.push(variant);
  }
  return lanes;
}

/** 🗂️ One lane's framework-owned receipt directory. */
export function demonstratorActivationLaneReceiptDirectory(workspace: string, lane: string): string {
  return join(developmentRuntimeRoot(join(workspace, OS_DEV_PACKAGE_ROOT), lane, "dev", "react"), "activation");
}

/** 🗂️ Every lane's receipt directory, in lane order — what the union watcher subscribes to. */
export function demonstratorActivationLaneReceiptDirectories(workspace: string): readonly string[] {
  return demonstratorActivationLanes().map(lane => demonstratorActivationLaneReceiptDirectory(workspace, lane));
}
//#endregion 🔖️DemonstratorActivationLanes

//#region 🔖️DemonstratorUnionReceipt
export type DemonstratorActivationLaneReceipt = { readonly lane: string; readonly receipt: ActivationReceipt };

/** @emoji 🧾️ Merges every lane's completion receipt into the ONE receipt describing the Demonstrator's
 * exact runtime union — pure, so the union rule is testable without a staged workspace.
 *
 * Every lane stages into the SAME `🔌️plugin-modules` root, so one plugin's `artifactSha256` cannot
 * legitimately differ between two lanes: a disagreement means one lane's `activate-…` ran against
 * different bytes and its receipt is stale. Overlapping rows otherwise keep the EARLIEST `rebuiltAt`,
 * because the hot-swap watcher treats a bumped timestamp as "this component changed" and a lane that
 * merely re-activated unchanged bytes must not fake a change. */
export function mergeDemonstratorActivationReceipts(lanes: readonly DemonstratorActivationLaneReceipt[], expectedPluginIds: readonly string[], variant: string = DEMONSTRATOR_PRIMARY_ACTIVATION_LANE): ActivationReceipt {
  const rows = new Map<string, { readonly lane: string; readonly row: ActivationReceipt["plugins"][number] }>();
  for (const { lane, receipt } of lanes) {
    parseActivationReceipt(receipt);
    if (receipt.profile !== "dev") throw new Error(`Demonstrator activation lane ${lane} is not a dev activation: ${receipt.profile}`);
    if (receipt.variant !== lane) throw new Error(`Demonstrator activation lane ${lane} carries a ${receipt.variant} receipt`);
    for (const row of receipt.plugins) {
      const previous = rows.get(row.pluginId);
      if (!previous) { rows.set(row.pluginId, { lane, row }); continue; }
      if (previous.row.artifactSha256 !== row.artifactSha256) throw new Error(`Stale Demonstrator activation lane: ${row.pluginId} is ${previous.row.artifactSha256} in ${previous.lane} but ${row.artifactSha256} in ${lane} (run bun nx run @semio-tech/framework-os-dev:activate-${lane}-react-dev)`);
      if (row.rebuiltAt < previous.row.rebuiltAt) rows.set(row.pluginId, { lane, row });
    }
  }
  const expected = [...expectedPluginIds].sort(), present = [...rows.keys()].sort();
  const missing = expected.filter(id => !rows.has(id)), extra = present.filter(id => !expected.includes(id));
  if (missing.length > 0 || extra.length > 0) throw new Error(`Demonstrator activation does not contain its exact runtime union — missing: ${missing.join(", ") || "(none)"}; extra: ${extra.join(", ") || "(none)"}`);
  return parseActivationReceipt({ schema: "semio.dev.activation/v1", variant, profile: "dev", plugins: present.map(id => ({ ...rows.get(id)!.row })) });
}

/** 📖️ Reads one lane's receipt, naming the exact target that produces it when it is absent. */
function readDemonstratorActivationLane(workspace: string, lane: string): DemonstratorActivationLaneReceipt {
  try { return { lane, receipt: readActivationReceipt(demonstratorActivationLaneReceiptDirectory(workspace, lane)) }; }
  catch (error) {
    if ((error as NodeJS.ErrnoException | undefined)?.code !== "ENOENT") throw error;
    throw new Error(`Missing Demonstrator activation lane receipt: ${lane} (run bun nx run @semio-tech/framework-os-dev:activate-${lane}-react-dev)`);
  }
}

/** 📬️ Publishes the merged union receipt into the Demonstrator's own `dist` and returns it. */
export function publishDemonstratorUnionReceipt(workspace: string): { readonly receiptDirectory: string; readonly receipt: ActivationReceipt; readonly laneReceiptDirectories: readonly string[] } {
  const lanes = demonstratorActivationLanes();
  const receipt = mergeDemonstratorActivationReceipts(lanes.map(lane => readDemonstratorActivationLane(workspace, lane)), demonstratorRuntimeComponentIds());
  const receiptDirectory = join(workspace, DEMONSTRATOR_UNION_RECEIPT_DIRECTORY);
  mkdirSync(receiptDirectory, { recursive: true });
  publishActivationReceipt(receiptDirectory, receipt);
  return { receiptDirectory, receipt, laneReceiptDirectories: lanes.map(lane => demonstratorActivationLaneReceiptDirectory(workspace, lane)) };
}

/** 🧾️ Requires the exact completed Demonstrator component union before starting its development host. */
export function readDemonstratorActivation(workspace: string) {
  const { receiptDirectory, receipt, laneReceiptDirectories } = publishDemonstratorUnionReceipt(workspace);
  const primary = developmentRuntimeRoot(join(workspace, OS_DEV_PACKAGE_ROOT), DEMONSTRATOR_PRIMARY_ACTIVATION_LANE, "dev", "react");
  return { receiptDirectory, extensionsDirectory: join(primary, "extensions"), receipt, laneReceiptDirectories };
}
//#endregion 🔖️DemonstratorUnionReceipt

/** 🔎️ Describes every closure component for the activation-receipt freshness watcher, keyed by the owner root descriptors are staged from. */
export function demonstratorActivationComponents(workspace: string): readonly ActivationComponentSpec[] {
  const byId = new Map([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map(row => [row.pluginId, row]));
  return demonstratorRuntimeComponentIds().map(id => {
    const row = byId.get(id)!;
    return { pluginId: id, directoryName: moduleDirectoryName(id), role: row.role === "extension" ? "extension" : "plugin", sourceRoot: resolve(workspace, row.cratePath, "..", "..") };
  });
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️demonstratorunionreceipt/🟦️.ts");
  await registerTests1(import.meta.vitest, { demonstratorActivationLanes, demonstratorRuntimeComponentIds, mergeDemonstratorActivationReceipts }, { directory: import.meta.dirname, url: import.meta.url });
}
