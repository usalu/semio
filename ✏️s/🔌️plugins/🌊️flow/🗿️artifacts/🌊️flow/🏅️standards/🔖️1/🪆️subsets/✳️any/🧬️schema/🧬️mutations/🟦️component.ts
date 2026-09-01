/** 🌊️ Flow direct-mutation discriminated union. */
import type { Widget, WidgetLayout } from "../🟦️component.ts";

/** 📍️ One layout assignment; a `null` layout clears the existing entry. */
export interface FlowLayoutEntry {
  id: string;
  layout: WidgetLayout | null;
}

/** ➕️ `create-widget` payload — brings a new widget into existence at `index`. */
export interface CreateWidget {
  index: number;
  widget: Widget;
}

/** 🗑️ `delete-widget` payload — the widget's id. */
export interface DeleteWidget {
  id: string;
}

/** 🔀️ `reorder-widgets` payload — repositions a widget within the ordered widget list. */
export interface ReorderWidgets {
  id: string;
  toIndex: number;
}

/** 🔁️ `replace-widget` payload — whole-value swap of a widget's payload. */
export interface ReplaceWidget {
  id: string;
  widget: Widget;
}

/** 🔗️ `connect-widgets` payload — creates a synapse edge between two widget ports. */
export interface ConnectWidgets {
  index: number;
  id: string;
  from: string;
  fromPort: string;
  to: string;
  toPort: string;
}

/** ✂️ `disconnect-widgets` payload — removes a synapse edge by id. */
export interface DisconnectWidgets {
  id: string;
}

/** 🔀️ `reorder-synapses` payload — repositions a synapse within the ordered synapse list. */
export interface ReorderSynapses {
  id: string;
  toIndex: number;
}

/** 🔄️ `update-synapse-endpoints` payload — atomically updates a synapse's endpoints. */
export interface UpdateSynapseEndpoints {
  id: string;
  from: string;
  fromPort: string;
  to: string;
  toPort: string;
}

/** 📍️ `move-widgets` payload — absolute repositions (or clears) one or more widgets at once. */
export interface MoveWidgets {
  entries: FlowLayoutEntry[];
}

/**
 * 👯️ `duplicate-widget` payload — the repo's pilot composite mutation (plans `create-widget` then
 * `connect-widgets`). It carries no `#[serde(rename_all = "camelCase")]` on its Rust struct, so its
 * fields stay snake_case on the wire, unlike every other sibling in this union.
 */
export interface DuplicateWidget {
  source_id: string;
  new_id: string;
  synapse_id: string;
  from_port: string;
  to_port: string;
}

export type FlowMutation =
  | ({ mutation: "createWidget" } & CreateWidget)
  | ({ mutation: "deleteWidget" } & DeleteWidget)
  | ({ mutation: "reorderWidgets" } & ReorderWidgets)
  | ({ mutation: "replaceWidget" } & ReplaceWidget)
  | ({ mutation: "connectWidgets" } & ConnectWidgets)
  | ({ mutation: "disconnectWidgets" } & DisconnectWidgets)
  | ({ mutation: "reorderSynapses" } & ReorderSynapses)
  | ({ mutation: "updateSynapseEndpoints" } & UpdateSynapseEndpoints)
  | ({ mutation: "moveWidgets" } & MoveWidgets)
  | ({ mutation: "duplicateWidget" } & DuplicateWidget);
