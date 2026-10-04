/** 🔺️ generation3d update-widget/🔺️diff — mirror of the whole-body widget patch delta builder. */
import type { UpdateWidget } from "../🦠️mutation/🟦️.ts";
import type { Widget } from "../../🌱️create-widget/🦠️mutation/🟦️.ts";
import type { SynapseSpec } from "../../../🟦️.ts";

export function diff(payload: UpdateWidget, previous: Widget, connections: readonly SynapseSpec[]): { widgets: { removed: string[]; set: Array<[bigint, Widget]> }; synapses: {removed:string[];set:Array<[bigint,SynapseSpec]>} } {
  if (previous.id !== payload.widget.id) throw new Error("Widget update does not address its base");
  const next = payload.widget;
  const set: Array<[bigint,SynapseSpec]> = [];
  if (previous.kind === "variable" && next.kind === "variable" && previous.name !== next.name) {
    connections.forEach((connection,index) => {
      const changed = {...connection};
      if (changed.from === previous.id && changed.fromPort === previous.name) changed.fromPort = next.name;
      if (changed.to === previous.id && changed.toPort === previous.name) changed.toPort = next.name;
      if (changed.fromPort !== connection.fromPort || changed.toPort !== connection.toPort) set.push([BigInt(index),changed]);
    });
  }
  return { widgets: { removed: [], set: [[0n, next]] }, synapses:{removed:[],set} };
}
