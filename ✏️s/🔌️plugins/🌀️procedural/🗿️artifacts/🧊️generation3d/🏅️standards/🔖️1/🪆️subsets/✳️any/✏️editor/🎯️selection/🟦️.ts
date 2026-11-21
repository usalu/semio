import { parseGraphInstanceId } from "../../🧬️schema/🔌️graph-ports/🟦️.ts";
import { componentVertexIds, parsePolygonMesh } from "@semio-tech/framework/mesh/polygon";

/** 🎯️ The portable contract for a component picked in a procedural mesh preview. */
export interface ComponentTarget {
  id: string;
  instance: string;
  widget: string;
  channel: string;
  index: number;
  granularity: "vertex" | "edge" | "face";
  component: number;
  analytic?: { handle: string; label: string; revision: string };
}

export function parseComponentTarget(id: string): ComponentTarget | undefined {
  const match = /^([\s\S]+)\.(vertex|edge|face)\.(0|[1-9][0-9]*)(?:~([0-9a-f]{64})~([1-9][0-9]{0,19})~([0-9a-f]{64}))?(?![\s\S])/u.exec(id);
  if (!match) return undefined;
  const [, instance, granularity, componentText, handle, label, revision] = match;
  if (!instance || (granularity !== "vertex" && granularity !== "edge" && granularity !== "face")) return undefined;
  const address = parseGraphInstanceId(instance);
  const component = Number(componentText);
  if (!address || address.port.input || address.index > 0xffffffff || component > 0xffffffff || label && BigInt(label) > 18446744073709551615n) return undefined;
  const { port: { widget, channel }, index } = address;
  return { id, instance, widget, channel, index, granularity, component, ...(handle && label && revision ? { analytic: { handle, label, revision } } : {}) };
}

export function componentGroup(ids: readonly string[]): { instance: string; granularity: ComponentTarget["granularity"]; components: number[] } {
  const id=ids[0],first = id===undefined ? undefined:parseComponentTarget(id);
  if (!first) throw new Error("Select mesh components");
  const targets = ids.map(id=>{const target=parseComponentTarget(id);if(!target)throw Error("Select mesh components");return target;});
  if (targets.some(target => !target || target.instance !== first.instance || target.granularity !== first.granularity || target.analytic?.handle !== first.analytic?.handle || target.analytic?.revision !== first.analytic?.revision)) throw new Error("Select components of one evaluated geometry at the same granularity");
  const labels = new Map<number, string>();
  for (const target of targets) if (target.analytic) {
    const previous = labels.get(target.component);
    if (previous !== undefined && previous !== target.analytic.label) throw new Error("A renderer group has inconsistent component identity");
    labels.set(target.component, target.analytic.label);
  }
  return { instance: first.instance, granularity: first.granularity, components: [...new Set(targets.map(target => target.component))].sort((a, b) => a - b) };
}

/** 🥽️ Resolves an admissible single-mesh selection against its evaluated indexed topology. */
export function selectedMeshVertices(ids: readonly string[], data: string): number[] {
  const group = componentGroup(ids);
  const id=ids[0],target=id===undefined?undefined:parseComponentTarget(id);if(!target)throw Error("Select mesh components");
  if (target.analytic) throw new Error("Analytic components require a B-Rep operation");
  if (target.index !== 0) throw new Error("Extract one mesh from the list before editing its components");
  return componentVertexIds(parsePolygonMesh(data), group.granularity, group.components);
}

/** 🏷️ Exact labels are scoped to one painted source and authoring revision. */
export function selectedAnalyticLabels(ids: readonly string[]): string[] {
  componentGroup(ids);
  const targets = ids.map(id=>{const target=parseComponentTarget(id);if(!target)throw Error("Select analytic faces or edges");return target;});
  if (targets.some(target => !target.analytic || target.granularity === "vertex")) throw new Error("Select analytic faces or edges");
  return [...new Set(targets.map(target => {if(!target.analytic)throw Error("Select analytic faces or edges");return target.analytic.label;}))].sort((left,right) => BigInt(left) < BigInt(right) ? -1 : BigInt(left) > BigInt(right) ? 1 : 0);
}

/** 🚦️ Admits exact component labels only against the current evaluated source. */
export function validateAnalyticSource(ids: readonly string[], revision: string, handle: string, references: Readonly<Record<string, readonly string[]>>): string[] {
  const labels = selectedAnalyticLabels(ids);
  const id=ids[0],target=id===undefined?undefined:parseComponentTarget(id);
  if (!target?.analytic||target.analytic.revision !== revision || target.analytic.handle !== handle) throw new Error("The selected geometry has changed; select its current components");
  const counts = new Map<string, number>();
  for (const label of references[target.granularity] ?? []) counts.set(label, (counts.get(label) ?? 0) + 1);
  if (labels.some(label => counts.get(label) !== 1)) throw new Error("The selected components no longer exist or are ambiguous");
  return labels;
}
