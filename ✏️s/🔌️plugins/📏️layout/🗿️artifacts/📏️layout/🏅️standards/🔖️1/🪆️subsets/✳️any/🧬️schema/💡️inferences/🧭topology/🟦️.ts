/** 🧭 `topology` — one named inference: parent-page/spread/page composition order, derived from
 * each `Page`'s `spreadId`/`parentPageId` refs. */

export interface LayoutTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
