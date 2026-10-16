/** 🫧️ UI-owned ephemeral local-only lane: keyed get-or-create boxes, maps and sets that outlive module re-evaluation and are never persisted, synced or serialized. */

/** 🫧️ Mutable box shared by every caller that asks for the same key. */
export type EphemeralBox<T> = { current: T };

const boxes = new Map<string, EphemeralBox<unknown>>();
const maps = new Map<string, Map<unknown, unknown>>();
const sets = new Map<string, Set<unknown>>();

function keyed<V>(lane: Map<string, V>, key: string, create: () => V): V {
  let value = lane.get(key);
  if (value === undefined) lane.set(key, (value = create()));
  return value;
}

/** 🫧️ Get-or-create the box for `key`; `init` is stored as-is and is never treated as a lazy factory. */
export function ephemeralBox<T>(key: string, init: T): EphemeralBox<T> {
  return keyed(boxes, key, () => ({ current: init })) as EphemeralBox<T>;
}

/** 🫧️ Get-or-create the Map for `key`. */
export function ephemeralMap<K, V>(key: string): Map<K, V> {
  return keyed(maps, key, () => new Map()) as Map<K, V>;
}

/** 🫧️ Get-or-create the Set for `key`. */
export function ephemeralSet<T>(key: string): Set<T> {
  return keyed(sets, key, () => new Set()) as Set<T>;
}
