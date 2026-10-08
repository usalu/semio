/** 🧩️ GisMapMutation — GIS map direct-mutation discriminated union. `GisMapMutation` carries only
 * `#[derive(dsl::Mutations)]` — no `#[serde(tag = ...)]` on the enum itself — so it serializes with
 * serde's default EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": { ...leaf-struct-fields }
 * }`, confirmed by the committed `🔀reorder-positions`/`🔁replace-position-data`
 * `🧪️tests/*​/🦠️mutation/🔣️.json` fixtures (`{"ReorderPositions":{"id":"pos-harbor",
 * "toIndex":2}}`). Unlike norm's shape-(B) artifacts, EVERY one of gismap's 12 leaf structs
 * individually carries `#[serde(rename_all = "camelCase")]`, so — despite the external tagging —
 * their own payload fields stay camelCase (`toIndex`, `newData`, …), also confirmed by fixture. */

import type { GisMapFeature } from "../📸️snapshot/🟦️.ts";

/** 🆕️ `create-position` payload — inserts `item` into `positions` at `index`. */
export interface CreatePosition {
  index: number;
  item: GisMapFeature;
}

/** 🗑️ `delete-position` payload — removes the `positions` entry addressed by `id`. */
export interface DeletePosition {
  id: string;
}

/** 🔁️ `replace-position-data` payload — replaces the `data` payload of the `positions` entry addressed by `id`. */
export interface ReplacePositionData {
  id: string;
  newData: Record<string, unknown>;
}

/** 🔀️ `reorder-positions` payload — moves the `positions` entry addressed by `id` to `toIndex`. */
export interface ReorderPositions {
  id: string;
  toIndex: number;
}

/** 🆕️ `create-route` payload — inserts `item` into `routes` at `index`. */
export interface CreateRoute {
  index: number;
  item: GisMapFeature;
}

/** 🗑️ `delete-route` payload — removes the `routes` entry addressed by `id`. */
export interface DeleteRoute {
  id: string;
}

/** 🔁️ `replace-route-data` payload — replaces the `data` payload of the `routes` entry addressed by `id`. */
export interface ReplaceRouteData {
  id: string;
  newData: Record<string, unknown>;
}

/** 🔀️ `reorder-routes` payload — moves the `routes` entry addressed by `id` to `toIndex`. */
export interface ReorderRoutes {
  id: string;
  toIndex: number;
}

/** 🆕️ `create-region` payload — inserts `item` into `regions` at `index`. */
export interface CreateRegion {
  index: number;
  item: GisMapFeature;
}

/** 🗑️ `delete-region` payload — removes the `regions` entry addressed by `id`. */
export interface DeleteRegion {
  id: string;
}

/** 🔁️ `replace-region-data` payload — replaces the `data` payload of the `regions` entry addressed by `id`. */
export interface ReplaceRegionData {
  id: string;
  newData: Record<string, unknown>;
}

/** 🔀️ `reorder-regions` payload — moves the `regions` entry addressed by `id` to `toIndex`. */
export interface ReorderRegions {
  id: string;
  toIndex: number;
}

/** ✏️ `set-position-property` payload — sets `key` in the payload of the `positions` entry addressed by `feature`; a new key is inserted before `before` (appended without one). */
export interface SetPositionProperty {
  feature: string;
  key: string;
  value: unknown;
  before?: string;
}

/** 🧽 `remove-position-property` payload — removes `key` from the payload of the `positions` entry addressed by `feature`. */
export interface RemovePositionProperty {
  feature: string;
  key: string;
}

/** ✏️ `set-route-property` payload — sets `key` in the payload of the `routes` entry addressed by `feature`; a new key is inserted before `before` (appended without one). */
export interface SetRouteProperty {
  feature: string;
  key: string;
  value: unknown;
  before?: string;
}

/** 🧽 `remove-route-property` payload — removes `key` from the payload of the `routes` entry addressed by `feature`. */
export interface RemoveRouteProperty {
  feature: string;
  key: string;
}

/** ✏️ `set-region-property` payload — sets `key` in the payload of the `regions` entry addressed by `feature`; a new key is inserted before `before` (appended without one). */
export interface SetRegionProperty {
  feature: string;
  key: string;
  value: unknown;
  before?: string;
}

/** 🧽 `remove-region-property` payload — removes `key` from the payload of the `regions` entry addressed by `feature`. */
export interface RemoveRegionProperty {
  feature: string;
  key: string;
}

export type GisMapMutation =
  | { CreatePosition: CreatePosition }
  | { DeletePosition: DeletePosition }
  | { ReplacePositionData: ReplacePositionData }
  | { ReorderPositions: ReorderPositions }
  | { CreateRoute: CreateRoute }
  | { DeleteRoute: DeleteRoute }
  | { ReplaceRouteData: ReplaceRouteData }
  | { ReorderRoutes: ReorderRoutes }
  | { CreateRegion: CreateRegion }
  | { DeleteRegion: DeleteRegion }
  | { ReplaceRegionData: ReplaceRegionData }
  | { ReorderRegions: ReorderRegions }
  | { SetPositionProperty: SetPositionProperty }
  | { RemovePositionProperty: RemovePositionProperty }
  | { SetRouteProperty: SetRouteProperty }
  | { RemoveRouteProperty: RemoveRouteProperty }
  | { SetRegionProperty: SetRegionProperty }
  | { RemoveRegionProperty: RemoveRegionProperty };
