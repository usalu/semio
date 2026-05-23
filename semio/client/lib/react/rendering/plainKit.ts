// #region 🧲Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// Lightweight plain-kit JSON helpers (no React Three / Storybook deps).
// #endregion 🧲Header

// #region 🧾PlainJson
type PlainJsonObject = Readonly<Record<string, unknown>>;

/** @emoji 🧾 Storybook / diagram design diff (structural; not GraphQL). */
export type DesignDiff = Readonly<{
  pieces?: Readonly<{
    added?: readonly unknown[];
    removed?: readonly unknown[];
    updated?: readonly unknown[];
    modified?: readonly unknown[];
  }>;
  connections?: Readonly<{
    added?: readonly unknown[];
    removed?: readonly unknown[];
    updated?: readonly unknown[];
  }>;
}>;

/** @emoji 🧾 Flat port row from plain kit `families[].ports` and `types[].ports`. */
export type KitPortPlain = PlainJsonObject & {
  id?: string;
  name?: string;
  description?: string;
  icon?: string;
  maxChildren?: number;
};
// #endregion 🧾PlainJson

// #region 🔖PlainKitSurface
/** @emoji 🧾 Algorithm / diagram JSON kit surface (`wip.initialKit` or legacy root). */
export function kitSurface(kit: unknown): Record<string, unknown> {
  const root = kit as { wip?: { initialKit?: Record<string, unknown> } };
  const inner = root.wip?.initialKit;
  if (inner && typeof inner === "object") return inner;
  return (kit as Record<string, unknown>) ?? {};
}

/** @emoji 🧾 `items` array or `{ items: T[] }` row list from plain kit JSON. */
export function kitItemsOf<T>(node: unknown): readonly T[] {
  if (Array.isArray(node)) return node as readonly T[];
  if (node && typeof node === "object" && "items" in node && Array.isArray((node as { items: unknown }).items)) {
    return (node as { items: T[] }).items;
  }
  return [];
}

/** @emoji 🧾 Named collection rows on a plain kit snapshot. */
export function kitJsRows(kit: unknown, key: string): readonly PlainJsonObject[] {
  return kitItemsOf(kitSurface(kit)[key]);
}

/** @emoji 🧾 Ports flattened from kit `families[].ports` then `types[].ports`. */
export function getKitPorts(kit: unknown): KitPortPlain[] {
  const out: KitPortPlain[] = [];
  for (const f of kitItemsOf(kitSurface(kit)["families"])) {
    for (const p of kitItemsOf((f as { ports?: unknown }).ports)) out.push(p as KitPortPlain);
  }
  for (const t of kitItemsOf(kitSurface(kit)["types"])) {
    for (const p of kitItemsOf((t as { ports?: unknown }).ports)) out.push(p as KitPortPlain);
  }
  return out;
}
// #endregion 🔖PlainKitSurface
