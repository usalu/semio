import { componentVertexIds, parsePolygonMesh } from "../../../../../../../../../🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🟦️.ts";

/** 🎯️ The portable contract for a component picked in a procedural mesh preview. */
export interface ComponentTarget {
  id: string;
  instance: string;
  widget: string;
  channel: string;
  index: number;
  granularity: "vertex" | "edge" | "face";
  component: number;
}

export function parseComponentTarget(id: string): ComponentTarget | undefined {
  const match = /^([^@#]+)@([^@#]+)#(0|[1-9][0-9]*)\.(vertex|edge|face)\.(0|[1-9][0-9]*)$/.exec(id);
  if (!match || match[0] !== id) return undefined;
  const [, widget, channel, indexText, granularity, componentText] = match;
  const index = Number(indexText), component = Number(componentText);
  if (index > 0xffffffff || component > 0xffffffff) return undefined;
  return { id, instance: `${widget}@${channel}#${index}`, widget, channel, index, granularity: granularity as ComponentTarget["granularity"], component };
}

export function componentGroup(ids: readonly string[]): { instance: string; granularity: ComponentTarget["granularity"]; components: number[] } {
  const first = ids.length ? parseComponentTarget(ids[0]) : undefined;
  if (!first) throw new Error("Select mesh components");
  const targets = ids.map(parseComponentTarget);
  if (targets.some(target => !target || target.instance !== first.instance || target.granularity !== first.granularity)) throw new Error("Select components of one mesh at the same granularity");
  return { instance: first.instance, granularity: first.granularity, components: [...new Set(targets.map(target => target!.component))].sort((a, b) => a - b) };
}

/** 🥽️ Resolves an admissible single-mesh selection against its evaluated indexed topology. */
export function selectedMeshVertices(ids: readonly string[], data: string): number[] {
  const group = componentGroup(ids);
  if (parseComponentTarget(ids[0])!.index !== 0) throw new Error("Extract one mesh from the list before editing its components");
  return componentVertexIds(parsePolygonMesh(data), group.granularity, group.components);
}
