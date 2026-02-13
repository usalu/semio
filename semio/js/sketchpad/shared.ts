// #region 🔖Header

// [💻semio/js/sketchpad/shared.ts](semiorepo://file/semio/js/sketchpad/shared.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

import { ChatIcon, CodeIcon, DetailsIcon, HudIcon, SettingsIcon, StatsIcon, ToolbarIcon, ToolsIcon, WorkbenchIcon } from "@semio/assets";
import { ComponentType, ReactNode } from "react";

// Shared state management types, hooks and store factories for sketchpad.

// #endregion 🔖Header

// #region 🔖Imports

// [🔖semio/js/sketchpad/shared.ts#Imports](semiorepo://section/semio/js/sketchpad/shared.ts/IMPORTS)
// MUST import XState, Y.js, and semio core types for shared sketchpad infrastructure.

import { AnyActorRef, assign, fromCallback } from "xstate";
import * as Y from "yjs";
import { Guid, Kit, KitDiff } from "../semio";

// #endregion 🔖Imports

// #region 🔖Types

// [🔖semio/js/sketchpad/shared.ts#Types](semiorepo://section/semio/js/sketchpad/shared.ts/TYPES)

// #region 🔖YPath Types

// [🔖semio/js/sketchpad/shared.ts#YPath Types](semiorepo://section/semio/js/sketchpad/shared.ts/YPATH-TYPES)
// MUST define path segment and path types for navigating Y.js document structures.

/**
 * A single segment in a Y.js document path, either a map key, array index, or array item by ID.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#YPath Types§YPathSegment](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YPATH-TYPES/YPATHSEGMENT)
 **/
export type YPathSegment = { kind: "mapKey"; key: string } | { kind: "arrayIndex"; index: number } | { kind: "arrayItemById"; id: string; idKey: string };

/**
 * An ordered sequence of YPathSegment values describing a path through a Y.js document.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#YPath Types§YPath](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YPATH-TYPES/YPATH)
 **/
export type YPath = YPathSegment[];

// #endregion 🔖YPath Types

// #region 🔖Granular Hook Types

// [🔖semio/js/sketchpad/shared.ts#Granular Hook Types](semiorepo://section/semio/js/sketchpad/shared.ts/GRANULAR-HOOK-TYPES)
// MUST define hook result tuples and field abstractions for granular reactive state access.

/**
 * A readonly tuple of value, optional setter, and canSet flag for granular hook access.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§HookResult](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/HOOKRESULT)
 **/
export type HookResult<T> = readonly [T, ((value: T) => void) | undefined, boolean];

/**
 * A readonly tuple of value, undefined setter, and canSet flag for read-only hook access.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§HookNoSetResult](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/HOOKNOSETRESULT)
 **/
export type HookNoSetResult<T> = readonly [T, undefined, boolean];

/**
 * Sentinel undefined value indicating that a hook result has no setter.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Granular Hook Types§READONLY_SETTER](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/READONLY-SETTER)
 **/
export const READONLY_SETTER = undefined as undefined;
/**
 * Sentinel false value indicating that a hook result is read-only.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Granular Hook Types§READONLY_CAN](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/READONLY-CAN)
 **/
export const READONLY_CAN = false;

/**
 * Wraps a value into a read-only HookResult tuple with no setter.
 *
 * MUST return a frozen readonly tuple with undefined setter and false canSet.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§readonlyHookResult](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/READONLYHOOKRESULT)
 **/
export function readonlyHookResult<T>(value: T): HookResult<T> {
  return [value, READONLY_SETTER, READONLY_CAN] as const;
}

/**
 * Wraps a value and setter into a writable HookResult tuple.
 *
 * MUST return a tuple with the setter included only when canSet is true.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§writableHookResult](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/WRITABLEHOOKRESULT)
 **/
export function writableHookResult<T>(value: T, setter: (value: T) => void, canSet: boolean = true): HookResult<T> {
  return [value, canSet ? setter : undefined, canSet] as const;
}

/**
 * Wraps a value into a HookResult tuple with a setter conditional on canSet.
 *
 * MUST return a tuple with the setter conditional on the canSet flag.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§conditionalHookResult](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/CONDITIONALHOOKRESULT)
 **/
export function conditionalHookResult<T>(canSet: boolean, value: T, setter: ((value: T) => void) | undefined): HookResult<T> {
  return [value, canSet ? setter : undefined, canSet] as const;
}

/**
 * A reactive field with a value, canSet flag, and setter function.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§Field](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/FIELD)
 **/
export interface Field<T> {
  value: T;
  canSet: boolean;
  set: (next: T) => void;
}

/**
 * A reactive action field with canExecute flag and execute function.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§ActionField](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/ACTIONFIELD)
 **/
export interface ActionField {
  canExecute: boolean;
  execute: () => void;
}

const NOOP_SETTER = () => {
  if (process.env.NODE_ENV === "development") {
    console.warn("[DEBUG] Attempted to set a disabled field");
  }
};

/**
 * Constructs a Field with a value, setter, and canSet flag.
 *
 * MUST use the provided setter when canSet is true, otherwise a no-op setter.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§createField](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/CREATEFIELD)
 **/
export function createField<T>(value: T, setter: (next: T) => void, canSet: boolean): Field<T> {
  return {
    value,
    canSet,
    set: canSet ? setter : NOOP_SETTER,
  };
}

/**
 * Constructs a read-only Field with a fixed value and no-op setter.
 *
 * MUST set canSet to false and use a no-op setter.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§createReadonlyField](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/CREATEREADONLYFIELD)
 **/
export function createReadonlyField<T>(value: T): Field<T> {
  return {
    value,
    canSet: false,
    set: NOOP_SETTER,
  };
}

/**
 * Constructs an ActionField with a guarded execute function.
 *
 * MUST guard execute behind canExecute, logging a warning in dev mode when disabled.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§createAction](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/CREATEACTION)
 **/
export function createAction(execute: () => void, canExecute: boolean): ActionField {
  return {
    canExecute,
    execute: canExecute
      ? execute
      : () => {
        if (process.env.NODE_ENV === "development") {
          console.warn("[DEBUG] Attempted to execute a disabled action");
        }
      },
  };
}

/**
 * Converts a Field to a HookResult tuple.
 *
 * MUST convert the field's canSet and set properties into the hook result format.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Granular Hook Types§fieldToHookResult](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/FIELDTOHOOKRESULT)
 **/
export function fieldToHookResult<T>(field: Field<T>): HookResult<T> {
  return [field.value, field.canSet ? field.set : undefined, field.canSet] as const;
}

/**
 * Converts a HookResult tuple back to a Field.
 *
 * MUST reconstruct a Field from the tuple, using a no-op setter when undefined.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Granular Hook Types§hookResultToField](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GRANULAR-HOOK-TYPES/HOOKRESULTTOFIELD)
 **/
export function hookResultToField<T>(result: HookResult<T>): Field<T> {
  const [value, setter, canSet] = result;
  return {
    value,
    canSet,
    set: setter ?? NOOP_SETTER,
  };
}

// #endregion 🔖Granular Hook Types

// #region 🔖Standard Empty Constants

// [🔖semio/js/sketchpad/shared.ts#Standard Empty Constants](semiorepo://section/semio/js/sketchpad/shared.ts/STANDARD-EMPTY-CONSTANTS)
// MUST provide frozen singleton constants for empty collections and default panel visibility.

/**
 * Frozen empty array singleton for default array values.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Standard Empty Constants§EMPTY_ARRAY](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/STANDARD-EMPTY-CONSTANTS/EMPTY-ARRAY)
 **/
export const EMPTY_ARRAY: readonly any[] = Object.freeze([]);
/**
 * Frozen empty object singleton for default record values.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Standard Empty Constants§EMPTY_OBJECT](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/STANDARD-EMPTY-CONSTANTS/EMPTY-OBJECT)
 **/
export const EMPTY_OBJECT: Readonly<Record<string, never>> = Object.freeze({});
/**
 * Frozen empty Guid array singleton for default guid collections.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Standard Empty Constants§EMPTY_GUID_ARRAY](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/STANDARD-EMPTY-CONSTANTS/EMPTY-GUID-ARRAY)
 **/
export const EMPTY_GUID_ARRAY: readonly Guid[] = Object.freeze([]);
/**
 * Frozen empty string array singleton for default string collections.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Standard Empty Constants§EMPTY_STRING_ARRAY](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/STANDARD-EMPTY-CONSTANTS/EMPTY-STRING-ARRAY)
 **/
export const EMPTY_STRING_ARRAY: readonly string[] = Object.freeze([]);

/**
 * Frozen default panel visibility with only toolbar visible.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Standard Empty Constants§EMPTY_PANEL_VISIBILITY](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/STANDARD-EMPTY-CONSTANTS/EMPTY-PANEL-VISIBILITY)
 **/
export const EMPTY_PANEL_VISIBILITY: Readonly<PanelVisibility> = Object.freeze({
  toolbar: true,
  workbench: false,
  details: false,
  chat: false,
  settings: false,
});

// #endregion 🔖Standard Empty Constants

// #region 🔖Generic Diff Types

// [🔖semio/js/sketchpad/shared.ts#Generic Diff Types](semiorepo://section/semio/js/sketchpad/shared.ts/GENERIC-DIFF-TYPES)
// MUST define generic array and selection diff types with apply and inverse operations.

/**
 * Describes added and removed items for an array diff operation.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Types#Generic Diff Types§ArrayDiff](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GENERIC-DIFF-TYPES/ARRAYDIFF)
 **/
export interface ArrayDiff<T> {
  added?: T[];
  removed?: T[];
}

/**
 * Maps selection keys to their corresponding array diffs.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Generic Diff Types§SelectionDiff](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GENERIC-DIFF-TYPES/SELECTIONDIFF)
 **/
export type SelectionDiff<TSelection extends Record<string, any[]>> = {
  [K in keyof TSelection]?: ArrayDiff<TSelection[K][number]>;
};

/**
 * Inverts an array diff by swapping added and removed items.
 *
 * MUST swap added and removed arrays to produce the inverse diff.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Generic Diff Types§inverseArrayDiff](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GENERIC-DIFF-TYPES/INVERSEARRAYDIFF)
 **/
export function inverseArrayDiff<T>(diff: ArrayDiff<T>): ArrayDiff<T> {
  const inverse: ArrayDiff<T> = {};
  if (diff.added) inverse.removed = diff.added;
  if (diff.removed) inverse.added = diff.removed;
  return inverse;
}

/**
 * Inverts all array diffs within a selection diff.
 *
 * MUST apply inverseArrayDiff to each key in the selection diff.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Generic Diff Types§inverseSelectionDiff](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GENERIC-DIFF-TYPES/INVERSESELECTIONDIFF)
 **/
export function inverseSelectionDiff<T extends Record<string, ArrayDiff<any>>>(diff: T): T {
  const inverse = {} as T;
  for (const key in diff) {
    if (Object.prototype.hasOwnProperty.call(diff, key)) {
      inverse[key] = inverseArrayDiff(diff[key]) as T[typeof key];
    }
  }
  return inverse;
}

/**
 * Applies an array diff to a current array, removing then adding items.
 *
 * MUST remove items first, then add non-duplicate items.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types#Generic Diff Types§applyArrayDiff](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GENERIC-DIFF-TYPES/APPLYARRAYDIFF)
 **/
export function applyArrayDiff<T>(current: T[] | undefined, diff: ArrayDiff<T>): T[] {
  let result = current ? [...current] : [];
  if (diff.removed) result = result.filter((item) => !diff.removed!.includes(item));
  if (diff.added) result = [...result, ...diff.added.filter((item) => !result.includes(item))];
  return result;
}

/**
 * Applies a selection diff to a partial selection state.
 *
 * MUST apply the array diff for each key present in the selection diff.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Types#Generic Diff Types§applySelectionDiff](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/GENERIC-DIFF-TYPES/APPLYSELECTIONDIFF)
 **/
export function applySelectionDiff<TSelection extends Record<string, any[]>>(current: Partial<TSelection>, diff: SelectionDiff<TSelection>): Partial<TSelection> {
  const result = { ...current } as Partial<TSelection>;
  for (const key in diff) {
    if (Object.prototype.hasOwnProperty.call(diff, key)) {
      const typedKey = key as keyof TSelection;
      result[typedKey] = applyArrayDiff(current[typedKey], diff[typedKey]!) as TSelection[typeof typedKey];
    }
  }
  return result;
}

// #endregion 🔖Generic Diff Types

/**
 * A string alias representing a URL.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§Url](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/URL)
 **/
export type Url = string;

/**
 * A callback subscription function that returns an unsubscribe disposer.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§Subscribe](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/SUBSCRIBE)
 **/
export type Subscribe = (callback: () => void) => () => void;

/**
 * A cleanup function that disposes of a resource.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§Disposable](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/DISPOSABLE)
 **/
export type Disposable = () => void;

/**
 * A function that executes a mutation within a transaction with optional origin.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§Transact](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/TRANSACT)
 **/
export type Transact = (fn: () => void, origin?: string) => void;

/**
 * A function that unsubscribes a previously registered callback.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§Unsubscribe](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/UNSUBSCRIBE)
 **/
export type Unsubscribe = () => void;

/**
 * A factory function that creates a Y.js document provider for a given ID.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§YProviderFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YPROVIDERFACTORY)
 **/
export type YProviderFactory = (doc: Y.Doc, id: string) => Promise<void>;

/**
 * A string alias identifying the kind of an app.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§AppKind](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/APPKIND)
 **/
export type AppKind = string;

/**
 * Union type for desktop, tablet, or mobile device contexts.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§Device](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/DEVICE)
 **/
export type Device = "desktop" | "tablet" | MobileDevice;

/**
 * Union of all panel identifier strings including side and HUD panels.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§PanelKey](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/PANELKEY)
 **/
export type PanelKey = "details" | "workbench" | "tools" | "hud" | "stats" | "console" | "chat" | "settings" | "toolbar" | "leftSidePanel" | "rightSidePanel" | "hudPanel";

/**
 * Union of left and right side panel keys.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§SidePanelKey](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/SIDEPANELKEY)
 **/
export type SidePanelKey = "leftSidePanel" | "rightSidePanel";

/**
 * The HUD panel key literal type.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§HudPanelKey](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/HUDPANELKEY)
 **/
export type HudPanelKey = "hudPanel";

/**
 * A string alias for a hotkey path identifier.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§HotkeyPath](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/HOTKEYPATH)
 **/
export type HotkeyPath = string;

/**
 * A string alias for a hotkey binding value.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§HotkeyValue](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/HOTKEYVALUE)
 **/
export type HotkeyValue = string;

/**
 * A record mapping hotkey paths to their override values.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§HotkeyOverrides](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/HOTKEYOVERRIDES)
 **/
export type HotkeyOverrides = Record<HotkeyPath, HotkeyValue>;

/**
 * A factory function that creates a FileProvider for a given kit ID.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§FileProviderFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/FILEPROVIDERFACTORY)
 **/
export type FileProviderFactory = (kitId: string) => Promise<FileProvider>;

/**
 * A string alias for a Y.js-compatible UUID.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§YUuid](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YUUID)
 **/
export type YUuid = string;

/**
 * A Y.js array of UUID strings.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§YUuidArray](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YUUIDARRAY)
 **/
export type YUuidArray = Y.Array<YUuid>;

/**
 * A string alias for a Y.js concept name.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§YConcept](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YCONCEPT)
 **/
export type YConcept = string;

/**
 * A Y.js array of concept name strings.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§YConcepts](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YCONCEPTS)
 **/
export type YConcepts = Y.Array<string>;

/**
 * A Y.js array of strings.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§YStringArray](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YSTRINGARRAY)
 **/
export type YStringArray = Y.Array<string>;

/**
 * A Y.js map with string leaf values.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§YLeafMapString](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YLEAFMAPSTRING)
 **/
export type YLeafMapString = Y.Map<string>;

/**
 * A Y.js map with number leaf values.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§YLeafMapNumber](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YLEAFMAPNUMBER)
 **/
export type YLeafMapNumber = Y.Map<number>;

/**
 * A Y.js array of Y.js maps representing attribute key-value pairs.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Types§YAttributes](semiorepo://definition/semio/js/sketchpad/shared.ts/TYPES/YATTRIBUTES)
 **/
export type YAttributes = Y.Array<Y.Map<string>>;

// #endregion 🔖Types

// #region 🔖Enums

// [🔖semio/js/sketchpad/shared.ts#Enums](semiorepo://section/semio/js/sketchpad/shared.ts/ENUMS)
// MUST enumerate theme, expertise, mode, store status, tool, window, and panel kinds.

/**
 * Available UI theme options: system, light, or dark.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Enums§Theme](semiorepo://definition/semio/js/sketchpad/shared.ts/ENUMS/THEME)
 **/
export enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

/**
 * User expertise levels: beginner, normal, or expert.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Enums§Expertise](semiorepo://definition/semio/js/sketchpad/shared.ts/ENUMS/EXPERTISE)
 **/
export enum Expertise {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}

/**
 * Application modes: user or dev.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Enums§Mode](semiorepo://definition/semio/js/sketchpad/shared.ts/ENUMS/MODE)
 **/
export enum Mode {
  USER = "user",
  DEV = "dev",
}

/**
 * Store lifecycle states: idle, loading, error, or ready.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Enums§StoreStatus](semiorepo://definition/semio/js/sketchpad/shared.ts/ENUMS/STORESTATUS)
 **/
export enum StoreStatus {
  IDLE = "idle",
  LOADING = "loading",
  ERROR = "error",
  READY = "ready",
}

/**
 * Available tool kinds for selection, lasso, connector, and hand interactions.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Enums§ToolKind](semiorepo://definition/semio/js/sketchpad/shared.ts/ENUMS/TOOLKIND)
 **/
export enum ToolKind {
  SELECTION_NORMAL = "selection-normal",
  SELECTION_ADDITIVE = "selection-additive",
  SELECTION_SUBTRACTIVE = "selection-subtractive",
  SELECTION_INTERSECT = "selection-intersect",
  LASSO_RECTANGULAR = "lasso-rectangular",
  LASSO_FREEFORM = "lasso-freeform",
  CONNECTOR = "connector",
  HAND = "hand",
}

/**
 * Window content kinds: table, scene, diagram, or custom.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Enums§WindowKind](semiorepo://definition/semio/js/sketchpad/shared.ts/ENUMS/WINDOWKIND)
 **/
export enum WindowKind {
  TABLE = "table",
  SCENE = "scene",
  DIAGRAM = "diagram",
  CUSTOM = "custom",
}

/**
 * Panel layout positions: left, right, middle, or bottom.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Enums§PanelPosition](semiorepo://definition/semio/js/sketchpad/shared.ts/ENUMS/PANELPOSITION)
 **/
export enum PanelPosition {
  LEFT = "left",
  RIGHT = "right",
  MIDDLE = "middle",
  BOTTOM = "bottom",
}

/**
 * Panel kinds: workbench, tools, toolbar, HUD, stats, details, chat, settings, params, or console.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Enums§PanelKind](semiorepo://definition/semio/js/sketchpad/shared.ts/ENUMS/PANELKIND)
 **/
export enum PanelKind {
  WORKBENCH = "workbench",
  TOOLS = "tools",
  TOOLBAR = "toolbar",
  HUD = "hud",
  STATS = "stats",
  DETAILS = "details",
  CHAT = "chat",
  SETTINGS = "settings",
  PARAMS = "params",
  CONSOLE = "console",
}

// #endregion 🔖Enums

// #region 🔖Ports

// [🔖semio/js/sketchpad/shared.ts#Ports](semiorepo://section/semio/js/sketchpad/shared.ts/PORTS)

// #region 🔖File Provider

// [🔖semio/js/sketchpad/shared.ts#File Provider](semiorepo://section/semio/js/sketchpad/shared.ts/FILE-PROVIDER)
// MUST define file storage provider interfaces for upload, download, and delete operations.

/**
 * Interface for file upload, download, delete, and URL retrieval operations.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#File Provider§FileProvider](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/FILE-PROVIDER/FILEPROVIDER)
 **/
export interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}

/**
 * Configuration interface for in-memory file provider.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#File Provider§MemoryFileProviderConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/FILE-PROVIDER/MEMORYFILEPROVIDERCONFIG)
 **/
export interface MemoryFileProviderConfig { }

/**
 * Configuration interface for local IndexedDB file provider.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#File Provider§LocalFileProviderConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/FILE-PROVIDER/LOCALFILEPROVIDERCONFIG)
 **/
export interface LocalFileProviderConfig {
  dbName?: string;
  storeName?: string;
}

/**
 * Configuration interface for remote file provider with base URL and headers.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#File Provider§RemoteFileProviderConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/FILE-PROVIDER/REMOTEFILEPROVIDERCONFIG)
 **/
export interface RemoteFileProviderConfig {
  baseUrl: string;
  headers?: Record<string, string>;
}

/**
 * Configuration interface combining memory, local, and remote file providers.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#File Provider§CompositeFileProviderConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/FILE-PROVIDER/COMPOSITEFILEPROVIDERCONFIG)
 **/
export interface CompositeFileProviderConfig {
  memory?: boolean;
  local?: boolean | LocalFileProviderConfig;
  remote?: RemoteFileProviderConfig;
}

/**
 * Interface for remote Y.js document and file provider factories.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#File Provider§RemoteProviders](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/FILE-PROVIDER/REMOTEPROVIDERS)
 **/
export interface RemoteProviders {
  yProvider: (yDoc: Y.Doc, name: string) => void;
  fileProvider: FileProviderFactory;
}

/**
 * Describes a file operation with type, kit ID, file ID, path, and optional blob.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#File Provider§FileOperation](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/FILE-PROVIDER/FILEOPERATION)
 **/
export interface FileOperation {
  type: "upload" | "download" | "delete";
  kitId: string;
  fileId: string;
  path: string;
  blob?: Blob;
}

// #endregion 🔖File Provider

// #region 🔖App IDs

// [🔖semio/js/sketchpad/shared.ts#App IDs](semiorepo://section/semio/js/sketchpad/shared.ts/APP-IDS)
// MUST define identifier interfaces for design, kit, type, and quality app scopes.

/**
 * Identifier for a design app scope with kit and design GUIDs.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#App IDs§DesignAppId](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/APP-IDS/DESIGNAPPID)
 **/
export interface DesignAppId {
  kit: Guid;
  design: Guid;
}

/**
 * Identifier for a kit app scope with a kit GUID.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#App IDs§KitAppId](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/APP-IDS/KITAPPID)
 **/
export interface KitAppId {
  kit: Guid;
}

/**
 * Identifier for a type app scope with kit and type GUIDs.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#App IDs§TypeAppId](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/APP-IDS/TYPEAPPID)
 **/
export interface TypeAppId {
  kit: Guid;
  type: Guid;
}

/**
 * Identifier for a quality app scope with kit and quality GUIDs.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#App IDs§QualityAppId](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/APP-IDS/QUALITYAPPID)
 **/
export interface QualityAppId {
  kit: Guid;
  quality: Guid;
}

// #endregion 🔖App IDs

// #region 🔖Panel

// [🔖semio/js/sketchpad/shared.ts#Panel](semiorepo://section/semio/js/sketchpad/shared.ts/PANEL)
// MUST define panel kind configurations, visibility, sizing, sections, and definition interfaces.

/**
 * Configuration for a panel kind including icon, position, group, and hotkey.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§PanelKindConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/PANELKINDCONFIG)
 **/
export interface PanelKindConfig {
  icon: ComponentType<{ size?: number }>;
  position: PanelPosition;
  group?: string;
  isTransparent?: boolean;
  isGroupable?: boolean;
  hotkey?: string;
}

/**
 * Registry mapping each PanelKind to its PanelKindConfig.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Ports#Panel§panelKindConfigs](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/PANELKINDCONFIGS)
 **/
export const panelKindConfigs: Record<PanelKind, PanelKindConfig> = {
  [PanelKind.WORKBENCH]: {
    icon: WorkbenchIcon,
    position: PanelPosition.LEFT,
    group: "workbench",
    isGroupable: true,
    hotkey: "ctrl+j",
  },
  [PanelKind.TOOLS]: {
    icon: ToolsIcon,
    position: PanelPosition.LEFT,
    group: "workbench",
    isGroupable: true,
    hotkey: "ctrl+j",
  },
  [PanelKind.TOOLBAR]: {
    icon: ToolbarIcon,
    position: PanelPosition.BOTTOM,
  },
  [PanelKind.HUD]: {
    icon: HudIcon,
    position: PanelPosition.MIDDLE,
    group: "hud",
    isGroupable: true,
    isTransparent: true,
    hotkey: "ctrl+k",
  },
  [PanelKind.STATS]: {
    icon: StatsIcon,
    position: PanelPosition.MIDDLE,
    group: "hud",
    isGroupable: true,
    isTransparent: true,
    hotkey: "ctrl+k",
  },
  [PanelKind.DETAILS]: {
    icon: DetailsIcon,
    position: PanelPosition.RIGHT,
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
  [PanelKind.CHAT]: {
    icon: ChatIcon,
    position: PanelPosition.RIGHT,
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
  [PanelKind.SETTINGS]: {
    icon: SettingsIcon,
    position: PanelPosition.RIGHT,
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
  [PanelKind.PARAMS]: {
    icon: SettingsIcon,
    position: PanelPosition.RIGHT,
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
  [PanelKind.CONSOLE]: {
    icon: CodeIcon,
    position: PanelPosition.BOTTOM,
    hotkey: "ctrl+k",
  },
};

/**
 * Side panel positions: left or right.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Ports#Panel§SidePanelPosition](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/SIDEPANELPOSITION)
 **/
export enum SidePanelPosition {
  LEFT = "left",
  RIGHT = "right",
}

/**
 * A tab entry for a side panel with ID, icon, order, and content.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§SidePanelTab](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/SIDEPANELTAB)
 **/
export interface SidePanelTab {
  id: string;
  icon: ComponentType<{ size?: number }>;
  order?: number;
  content: ReactNode | (() => ReactNode);
}

/**
 * A tab entry for the HUD panel with ID, icon, order, and content.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§HudPanelTab](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/HUDPANELTAB)
 **/
export interface HudPanelTab {
  id: string;
  icon: ComponentType<{ size?: number }>;
  order?: number;
  content: ReactNode | (() => ReactNode);
}

/**
 * Visibility flags for left and right side panels.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§SidePanelVisibility](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/SIDEPANELVISIBILITY)
 **/
export interface SidePanelVisibility {
  left: boolean;
  right: boolean;
}

/**
 * Visibility flag for the HUD panel.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§HudPanelVisibility](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/HUDPANELVISIBILITY)
 **/
export interface HudPanelVisibility {
  visible: boolean;
}

/**
 * Optional visibility flags for all panel kinds.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§PanelVisibility](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/PANELVISIBILITY)
 **/
export interface PanelVisibility {
  toolbar?: boolean;
  leftSidePanel?: boolean;
  rightSidePanel?: boolean;
  hudPanel?: boolean;
  workbench?: boolean;
  tools?: boolean;
  hud?: boolean;
  stats?: boolean;
  details?: boolean;
  chat?: boolean;
  settings?: boolean;
  params?: boolean;
  console?: boolean;
}

/**
 * Numeric sizes for all panel dimensions including widths and heights.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§PanelSizes](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/PANELSIZES)
 **/
export interface PanelSizes {
  toolbarHeight: number;
  workbenchWidth: number;
  toolsWidth: number;
  hudWidth: number;
  statsWidth: number;
  detailsWidth: number;
  chatWidth: number;
  settingsWidth: number;
  consoleHeight: number;
  leftSidePanelWidth: number;
  rightSidePanelWidth: number;
  hudPanelWidth: number;
}

/**
 * A collapsible section within a panel with content, actions, and toolbar group.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§PanelSection](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/PANELSECTION)
 **/
export interface PanelSection {
  id: string;
  content: ReactNode | (() => ReactNode);
  specificity?: number;
  defaultOpen?: boolean;
  order?: number;
  toolbarGroup?: {
    id: string;
    labelId?: string;
    order?: number;
    subToolId?: string;
    subToolLabelId?: string;
    subToolIcon?: ReactNode;
    onActivate?: () => void;
  };
  actions?: Array<{
    id: string;
    icon: ReactNode;
    onClick: () => void;
  }>;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: () => void;
}

/**
 * Left and right arrays of side panel tabs.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§SidePanelTabs](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/SIDEPANELTABS)
 **/
export interface SidePanelTabs {
  left: SidePanelTab[];
  right: SidePanelTab[];
}

/**
 * Array of HUD panel tabs.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§HudPanelTabs](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/HUDPANELTABS)
 **/
export interface HudPanelTabs {
  tabs: HudPanelTab[];
}

/**
 * Collections of panel sections and tabs organized by panel kind.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§PanelSections](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/PANELSECTIONS)
 **/
export interface PanelSections {
  details: PanelSection[];
  workbench: PanelSection[];
  tools: PanelSection[];
  hud: PanelSection[];
  stats: PanelSection[];
  console: PanelSection[];
  chat: PanelSection[];
  settings: PanelSection[];
  toolbar: PanelSection[];
  leftSidePanel: SidePanelTab[];
  rightSidePanel: SidePanelTab[];
  hudPanel: HudPanelTab[];
}

/**
 * Definition of a panel with ID, kind, hotkey, and tooltip.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§PanelDefinition](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/PANELDEFINITION)
 **/
export interface PanelDefinition {
  id: string;
  kind: PanelKind;
  hotkey?: string;
  tooltip?: {
    labelKey?: string;
    manualPath?: string;
  };
}

/**
 * Extended panel definition with resolved icon, position, group, and transparency.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§EnrichedPanelDefinition](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/ENRICHEDPANELDEFINITION)
 **/
export interface EnrichedPanelDefinition extends PanelDefinition {
  key: string;
  icon: ComponentType<{ size?: number }>;
  position: PanelPosition;
  group?: string;
  isTransparent?: boolean;
  isGroupable?: boolean;
  hotkey?: string;
}

/**
 * Constructs a PanelDefinition from a kind, ID, hotkey, and tooltip.
 *
 * MUST use the panelKindConfigs hotkey as fallback when no explicit hotkey is provided.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Ports#Panel§createPanelDefinition](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/CREATEPANELDEFINITION)
 **/
export function createPanelDefinition(kind: PanelKind, id: string, hotkey?: string, tooltip?: { labelKey?: string; manualPath?: string }): PanelDefinition {
  const config = panelKindConfigs[kind];
  return {
    id,
    kind,
    hotkey: hotkey ?? config.hotkey,
    tooltip,
  };
}

/**
 * Enriches a PanelDefinition with resolved config properties from panelKindConfigs.
 *
 * MUST resolve all config properties from panelKindConfigs for the panel's kind.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Ports#Panel§enrichPanelDefinition](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/ENRICHPANELDEFINITION)
 **/
export function enrichPanelDefinition(panel: PanelDefinition): EnrichedPanelDefinition {
  const config = panelKindConfigs[panel.kind];
  return {
    ...panel,
    key: panel.kind,
    icon: config.icon,
    position: config.position,
    group: config.group,
    isTransparent: config.isTransparent,
    isGroupable: config.isGroupable,
    hotkey: panel.hotkey ?? config.hotkey,
  };
}

/**
 * Configuration for a panel instance with ID, key, label, order, and content.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§PanelConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/PANELCONFIG)
 **/
export interface PanelConfig {
  id: string;
  key: "workbench" | "details" | "settings" | "tools" | "hud" | "stats" | "toolbar" | "chat" | "console";
  label: string;
  order?: number;
  defaultOpen?: boolean;
  content: ReactNode | (() => ReactNode);
}

/**
 * Container for an array of panel configurations.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel§AppPanels](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL/APPPANELS)
 **/
export interface AppPanels {
  panels: PanelConfig[];
}

// #endregion 🔖Panel

// #region 🔖App Registry

// [🔖semio/js/sketchpad/shared.ts#App Registry](semiorepo://section/semio/js/sketchpad/shared.ts/APP-REGISTRY)
// MUST define route segment and app configuration interfaces for app registration.

/**
 * A URL route segment with path, optional param name, and scope provider.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#App Registry§RouteSegment](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/APP-REGISTRY/ROUTESEGMENT)
 **/
export interface RouteSegment {
  path: string;
  paramName?: string;
  scopeProvider?: ComponentType<{ guid: string; children: ReactNode }>;
}

/**
 * Full app configuration with ID, component, routes, panels, and order.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#App Registry§AppConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/APP-REGISTRY/APPCONFIG)
 **/
export interface AppConfig {
  id: string;
  component: ComponentType;
  routeSegments: RouteSegment[];
  additionalPaths?: string[];
  getPanels: (() => PanelDefinition[]) | ((getLabelFn: (key: string) => string) => PanelDefinition[]) | ((getLabelFn: (key: string) => string, getHotkeyFn: (key: string) => string) => PanelDefinition[]);
  matchesPath?: (pathParts: string[]) => boolean;
  order?: number;
}

/**
 * App registration entry extending AppConfig.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#App Registry§AppRegistration](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/APP-REGISTRY/APPREGISTRATION)
 **/
export interface AppRegistration extends AppConfig { }

// #endregion 🔖App Registry

// #region 🔖Sketchpad State

// [🔖semio/js/sketchpad/shared.ts#Sketchpad State](semiorepo://section/semio/js/sketchpad/shared.ts/SKETCHPAD-STATE)
// MUST define mutable and immutable sketchpad state interfaces with diff types.

/**
 * Mobile device state with navbar and footer expansion flags.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Sketchpad State§MobileDevice](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/SKETCHPAD-STATE/MOBILEDEVICE)
 **/
export interface MobileDevice {
  isNavbarExpanded: boolean;
  isFooterExpanded: boolean;
}

/**
 * Mutable fields of sketchpad state including navigation, theme, device, and settings.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Sketchpad State§SketchpadChangableState](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/SKETCHPAD-STATE/SKETCHPADCHANGABLESTATE)
 **/
export interface SketchpadChangableState {
  navigation: string;
  navigationHistory: string[];
  navigationHistoryIndex: number;
  recentSearches: string[];
  recentFocusItems: Record<string, string[]>;
  theme: Theme;
  language: string;
  device: Device;
  expertise: Expertise;
  mode: Mode;
  settings: {
    apps: Record<string, any>;
  };
  panelSizes: PanelSizes;
  isFullscreen: boolean;
  isMobile: boolean;
  activeInteraction?: string;
  hotkeyOverrides?: Record<string, string>;
  activeHotkeySetting?: string;
}

/**
 * Full sketchpad state extending changeable state with ID and persistence flag.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Sketchpad State§SketchpadState](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/SKETCHPAD-STATE/SKETCHPADSTATE)
 **/
export interface SketchpadState extends SketchpadChangableState {
  id?: string;
  persisted?: boolean;
}

/**
 * Partial diff of sketchpad state fields for incremental updates.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Sketchpad State§SketchpadDiff](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/SKETCHPAD-STATE/SKETCHPADDIFF)
 **/
export interface SketchpadDiff {
  navigation?: string;
  navigationHistory?: string[];
  navigationHistoryIndex?: number;
  recentSearches?: string[];
  recentFocusItems?: Record<string, string[]>;
  theme?: Theme;
  language?: string;
  device?: Device;
  expertise?: Expertise;
  mode?: Mode;
  settings?: {
    apps?: Record<string, any>;
  };
  panelSizes?: Partial<PanelSizes>;
  isFullscreen?: boolean;
  isMobile?: boolean;
  activeInteraction?: string;
  hotkeyOverrides?: Record<string, string>;
  activeHotkeySetting?: string;
}

/**
 * Initial kit state with kit data and local/remote flags.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Sketchpad State§InitialStateKit](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/SKETCHPAD-STATE/INITIALSTATEKIT)
 **/
export interface InitialStateKit {
  kit: Kit;
  local?: boolean;
  remote?: boolean;
}

/**
 * Extended initial state combining partial sketchpad state with initial kits.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Sketchpad State§ExtendedInitialState](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/SKETCHPAD-STATE/EXTENDEDINITIALSTATE)
 **/
export interface ExtendedInitialState extends Partial<SketchpadState> {
  kits?: InitialStateKit[];
}

/**
 * Callback functions for window minimize, maximize, and close events.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Ports#Sketchpad State§WindowEvents](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/SKETCHPAD-STATE/WINDOWEVENTS)
 **/
export type WindowEvents = {
  minimize: () => void;
  maximize: () => void;
  close: () => void;
};

/**
 * Scoped sketchpad context with ID, optional remote providers, and window events.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Ports#Sketchpad State§SketchpadScope](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/SKETCHPAD-STATE/SKETCHPADSCOPE)
 **/
export type SketchpadScope = { id: string; remote?: RemoteProviders; onWindowEvents?: WindowEvents };

// #endregion 🔖Sketchpad State

// #region 🔖Commands

// [🔖semio/js/sketchpad/shared.ts#Commands](semiorepo://section/semio/js/sketchpad/shared.ts/COMMANDS)
// MUST define command context and result interfaces for kit and sketchpad operations.

/**
 * Context for kit commands including kit data, file URLs, and origin.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Commands§KitCommandContext](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/COMMANDS/KITCOMMANDCONTEXT)
 **/
export interface KitCommandContext {
  kit: Kit;
  fileUrls: Map<Url, Url>;
  origin?: string;
}

/**
 * Result of a kit command with optional diff, files, and origin.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Commands§KitCommandResult](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/COMMANDS/KITCOMMANDRESULT)
 **/
export interface KitCommandResult {
  diff?: KitDiff;
  files?: File[];
  origin?: string;
}

/**
 * Context for sketchpad commands including sketchpad state and origin.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Commands§SketchpadCommandContext](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/COMMANDS/SKETCHPADCOMMANDCONTEXT)
 **/
export interface SketchpadCommandContext {
  sketchpad: SketchpadState;
  origin?: string;
}

/**
 * Result of a sketchpad command with optional diff and origin.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Commands§SketchpadCommandResult](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/COMMANDS/SKETCHPADCOMMANDRESULT)
 **/
export interface SketchpadCommandResult {
  diff?: SketchpadDiff;
  origin?: string;
}

// #endregion 🔖Commands

// #region 🔖Store

// [🔖semio/js/sketchpad/shared.ts#Store](semiorepo://section/semio/js/sketchpad/shared.ts/STORE)
// MUST define store state, app step, edit, diff, and command result interfaces.

/**
 * Interface for objects that support change subscription and snapshot retrieval.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Store§Synchronizable](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/STORE/SYNCHRONIZABLE)
 **/
export interface Synchronizable<TAccessl> {
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onChangedDeep: (subscribe: Subscribe) => Unsubscribe;
  snapshot: () => TAccessl;
}

/**
 * Wrapper for store status, data, and error.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Store§StoreState](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/STORE/STORESTATE)
 **/
export interface StoreState<TState> {
  status: StoreStatus;
  data?: TState;
  error?: Error;
}

/**
 * A single app step with optional selection diff.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Store§AppStep](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/STORE/APPSTEP)
 **/
export interface AppStep<TSelectionDiff = any> {
  selectionDiff?: TSelectionDiff;
}

/**
 * An undoable edit consisting of do and undo app steps.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Store§AppEdit](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/STORE/APPEDIT)
 **/
export interface AppEdit<TSelectionDiff = any> {
  do: AppStep<TSelectionDiff>;
  undo: AppStep<TSelectionDiff>;
}

/**
 * A diff containing selection, presence, hover, fullscreen, and panel visibility changes.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Store§AppDiff](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/STORE/APPDIFF)
 **/
export interface AppDiff<TSelectionDiff = any> {
  selection?: TSelectionDiff;
  presence?: any;
  hover?: any;
  fullscreenWindow?: any;
  panelVisibility?: Partial<PanelVisibility>;
}

/**
 * Result of an app command with optional diff and origin.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Store§AppCommandResult](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/STORE/APPCOMMANDRESULT)
 **/
export interface AppCommandResult<TDiff = any> {
  diff?: TDiff;
  origin?: string;
}

/**
 * An app step extended with an optional kit diff.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Store§KitDiffAppStep](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/STORE/KITDIFFAPPSTEP)
 **/
export interface KitDiffAppStep<TSelectionDiff = any> extends AppStep<TSelectionDiff> {
  kitDiff?: KitDiff;
}

/**
 * An undoable edit with kit diff-aware do and undo steps.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Store§KitDiffAppEdit](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/STORE/KITDIFFAPPEDIT)
 **/
export interface KitDiffAppEdit<TSelectionDiff = any> {
  do: KitDiffAppStep<TSelectionDiff>;
  undo: KitDiffAppStep<TSelectionDiff>;
}

/**
 * An app command result extended with an optional kit diff.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Store§KitDiffAppCommandResult](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/STORE/KITDIFFAPPCOMMANDRESULT)
 **/
export interface KitDiffAppCommandResult<TDiff = any> extends AppCommandResult<TDiff> {
  kitDiff?: KitDiff;
}

/**
 * Interface for objects that support change subscription and snapshot retrieval.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Store§Synchronizable](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/STORE/SYNCHRONIZABLE)
 **/
export interface Synchronizable<TAccessl> {
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onChangedDeep: (subscribe: Subscribe) => Unsubscribe;
  snapshot: () => TAccessl;
}

// #endregion 🔖Store

// #region 🔖Complete State

// [🔖semio/js/sketchpad/shared.ts#Complete State](semiorepo://section/semio/js/sketchpad/shared.ts/COMPLETE-STATE)
// MUST define the complete aggregated state interface for the entire sketchpad.

/**
 * Full aggregated state containing sketchpad, kits, and all app states.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Complete State§CompleteState](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/COMPLETE-STATE/COMPLETESTATE)
 **/
export interface CompleteState {
  sketchpad: SketchpadState;
  kits: Array<{
    guid: string;
    local: boolean;
    remote: boolean;
    kit: Kit;
  }>;
  kitApps: Record<string, any>;
  typeApps: Record<string, any>;
  qualityApps: Record<string, any>;
  designApps: Record<string, Record<string, any>>;
  home?: any;
  tutorials: any;
}

// #endregion 🔖Complete State

// #region 🔖Window

// [🔖semio/js/sketchpad/shared.ts#Window](semiorepo://section/semio/js/sketchpad/shared.ts/WINDOW)
// MUST define window configuration, control, layout parsing, and default layout creation.

/**
 * Configuration for a window with ID, title, icon, component, and default size.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Window§WindowConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/WINDOW/WINDOWCONFIG)
 **/
export interface WindowConfig {
  id: string;
  title?: string;
  icon?: ReactNode;
  component: ComponentType<any>;
  componentProps?: any;
  defaultSize?: number;
}

/**
 * A window control with kind, ID, icon, options, and change handler.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Window§WindowControl](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/WINDOW/WINDOWCONTROL)
 **/
export interface WindowControl {
  kind: "toggle" | "dropdown";
  id: string;
  icon?: ReactNode;
  value?: string;
  options?: {
    id: string;
    value: string;
    icon?: ReactNode;
  }[];
  onChange?: (value: string) => void;
}

/**
 * Definition of a window kind with label, icon, component, controls, and variants.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Window§WindowKindDefinition](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/WINDOW/WINDOWKINDDEFINITION)
 **/
export interface WindowKindDefinition {
  id: string;
  label?: string | any;
  icon?: ReactNode;
  component: (props: any) => ReactNode;
  controls?: WindowControl[];
  variants?: {
    id: string;
    icon?: ReactNode;
    componentProps?: Record<string, any>;
  }[];
}

/**
 * App-level window configuration with window kinds and default layout.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Window§AppWindowConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/WINDOW/APPWINDOWCONFIG)
 **/
export interface AppWindowConfig {
  windowKinds: WindowKindDefinition[];
  defaultLayout?: any;
}

/**
 * Parses a window layout from a string, object, or undefined input.
 *
 * MUST return undefined for null, empty, or unparseable inputs and parse valid JSON strings.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Ports#Window§parseWindowLayout](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/WINDOW/PARSEWINDOWLAYOUT)
 **/
export function parseWindowLayout(layout: unknown): any | undefined {
  if (layout === undefined || layout === null) return undefined;
  if (typeof layout === "string") {
    const trimmed = layout.trim();
    if (!trimmed) return undefined;
    try {
      return JSON.parse(trimmed);
    } catch {
      return undefined;
    }
  }
  if (typeof layout === "object") return layout;
  return undefined;
}

/**
 * Removes duplicate and disallowed window components from a layout.
 *
 * MUST remove duplicate component entries and filter out disallowed window IDs.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Ports#Window§deduplicateWindowLayout](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/WINDOW/DEDUPLICATEWINDOWLAYOUT)
 **/
export function deduplicateWindowLayout(layout: any, allowedWindowIds: string[]): any | undefined {
  if (!layout || typeof layout !== "object") return layout;

  const seenComponents = new Set<string>();

  const deduplicateContent = (content: any[]): any[] => {
    if (!Array.isArray(content)) return content;

    return content
      .map((item) => {
        if (!item || typeof item !== "object") return item;

        if (item.type === "component") {
          const componentName = item.componentName;
          if (seenComponents.has(componentName)) {
            return null;
          }
          if (!allowedWindowIds.includes(componentName)) {
            return null;
          }
          seenComponents.add(componentName);
          return item;
        }

        if (item.content && Array.isArray(item.content)) {
          const deduped = deduplicateContent(item.content);
          if (deduped.length === 0) return null;
          return { ...item, content: deduped };
        }

        return item;
      })
      .filter((item) => item !== null);
  };

  const root = layout.root;
  if (!root || typeof root !== "object") return layout;

  if (root.content && Array.isArray(root.content)) {
    const dedupedContent = deduplicateContent(root.content);
    return { ...layout, root: { ...root, content: dedupedContent } };
  }

  return layout;
}

/**
 * Serializes a window layout to a JSON string.
 *
 * MUST return undefined when serialization fails.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Ports#Window§stringifyWindowLayout](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/WINDOW/STRINGIFYWINDOWLAYOUT)
 **/
export function stringifyWindowLayout(layout: unknown): string | undefined {
  if (layout === undefined || layout === null) return undefined;
  try {
    return JSON.stringify(layout);
  } catch {
    return undefined;
  }
}

/**
 * Props for an app window component with kind, children, and className.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Window§AppWindowProps](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/WINDOW/APPWINDOWPROPS)
 **/
export interface AppWindowProps {
  kind: WindowKind;
  children: ReactNode;
  className?: string;
}

/**
 * Creates a default GoldenLayout configuration from window IDs and direction.
 *
 * MUST generate a GoldenLayout config with one stack per window ID.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Ports#Window§createDefaultLayout](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/WINDOW/CREATEDEFAULTLAYOUT)
 **/
export function createDefaultLayout(windowIds: string[], direction: "row" | "column" = "row", sizes?: number[], titles?: string[]): any {
  return {
    root: {
      type: direction === "row" ? "row" : "column",
      content: windowIds.map((id, index) => ({
        type: "stack",
        content: [
          {
            type: "component",
            componentName: id,
            title: titles && titles[index] ? titles[index] : id,
            componentState: {},
          },
        ],
        ...(sizes && sizes[index] !== undefined ? { size: `${sizes[index]}%` } : {}),
      })),
    },
  };
}

// #endregion 🔖Window

// #region 🔖Tool

// [🔖semio/js/sketchpad/shared.ts#Tool](semiorepo://section/semio/js/sketchpad/shared.ts/TOOL)
// MUST define tool interfaces for selection, lasso, connector, and hand interactions.

/**
 * A tool with ID, icon, and render function returning scene, diagram, and table nodes.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Tool§Tool](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/TOOL/TOOL)
 **/
export interface Tool<TState = any> {
  id: ToolKind | string;
  icon?: ReactNode;
  render: (context: ToolRenderContext<TState>) => { scene?: ReactNode; diagram?: ReactNode | null; table?: ReactNode | null };
}

/**
 * A tool mode with ID, icon, label, and tooltip.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Tool§ToolMode](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/TOOL/TOOLMODE)
 **/
export interface ToolMode {
  id: string;
  icon?: ReactNode;
  label?: string;
  tooltipId?: string;
}

/**
 * Definition of a tool with ID, default mode, and available modes.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Tool§ToolDefinition](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/TOOL/TOOLDEFINITION)
 **/
export interface ToolDefinition {
  id: string;
  defaultMode: ToolKind | string;
  modes: ToolMode[];
}

/**
 * Context passed to a tool's render function containing the current state.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Tool§ToolRenderContext](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/TOOL/TOOLRENDERCONTEXT)
 **/
export interface ToolRenderContext<TState = any> {
  state: TState;
}

/**
 * Props for a tool group component with tools, active tool, and change handler.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Tool§ToolGroupProps](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/TOOL/TOOLGROUPPROPS)
 **/
export interface ToolGroupProps {
  tools: ToolDefinition[];
  activeTool: ToolKind | string;
  onToolChange: (tool: ToolKind | string) => void;
}

// #endregion 🔖Tool

// #region 🔖Focus

// [🔖semio/js/sketchpad/shared.ts#Focus](semiorepo://section/semio/js/sketchpad/shared.ts/FOCUS)
// MUST define the focus item interface for search and navigation targets.

/**
 * A focusable item with ID, label, optional description, and category.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Focus§FocusItem](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/FOCUS/FOCUSITEM)
 **/
export interface FocusItem {
  id: string;
  label: string;
  description?: string;
  category?: string;
}

// #endregion 🔖Focus

// #region 🔖Footer

// [🔖semio/js/sketchpad/shared.ts#Footer](semiorepo://section/semio/js/sketchpad/shared.ts/FOOTER)
// MUST define the footer item interface for status bar entries.

/**
 * A footer status bar item with ID, icon, text, content, and click handler.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Footer§FooterItem](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/FOOTER/FOOTERITEM)
 **/
export interface FooterItem {
  id: string;
  icon?: ReactNode;
  text?: string;
  content?: ReactNode;
  onClick?: () => void;
  order?: number;
  className?: string;
  disabled?: boolean;
}

// #endregion 🔖Footer

// #region 🔖Panel Props

// [🔖semio/js/sketchpad/shared.ts#Panel Props](semiorepo://section/semio/js/sketchpad/shared.ts/PANEL-PROPS)
// MUST define resizable panel props interface for panel width management.

/**
 * Props for a resizable panel with visibility, width, and width change handler.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Ports#Panel Props§ResizablePanelProps](semiorepo://definition/semio/js/sketchpad/shared.ts/PORTS/PANEL-PROPS/RESIZABLEPANELPROPS)
 **/
export interface ResizablePanelProps {
  visible: boolean;
  onWidthChange?: (width: number) => void;
  width: number;
}

// #endregion 🔖Panel Props

// #endregion 🔖Ports

// #region 🔖XState Integration

// [🔖semio/js/sketchpad/shared.ts#XState Integration](semiorepo://section/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION)

// #region 🔖XState Types

// [🔖semio/js/sketchpad/shared.ts#XState Types](semiorepo://section/semio/js/sketchpad/shared.ts/XSTATE-TYPES)
// MUST define XState machine context and event type interfaces for sketchpad, kit, and app machines.

/**
 * Base context for Y.js-synced machines with dirty flag and cache.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#XState Integration#XState Types§YjsSyncContext](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/XSTATE-TYPES/YJSSYNCCONTEXT)
 **/
export interface YjsSyncContext {
  dirty: boolean;

  cache?: any;
}

/**
 * XState context for the sketchpad machine with navigation, theme, kits, and refs.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#XState Integration#XState Types§SketchpadMachineContext](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/XSTATE-TYPES/SKETCHPADMACHINECONTEXT)
 **/
export interface SketchpadMachineContext extends YjsSyncContext {
  navigation: string;
  navigationHistory: string[];
  navigationHistoryIndex: number;
  recentSearches: string[];
  recentFocusItems: Record<string, string[]>;
  theme: Theme;
  language: string;
  device: Device;
  expertise: Expertise;
  mode: Mode;
  settings: {
    apps: Record<string, any>;
  };
  panelSizes: PanelSizes;
  isFullscreen: boolean;
  isMobile: boolean;
  activeInteraction?: string;
  hotkeyOverrides?: Record<string, string>;
  activeHotkeySetting?: string;

  kits: Record<Guid, AnyActorRef>;

  homeRef?: AnyActorRef;

  docsRef?: AnyActorRef;
}

/**
 * Union of all events the sketchpad machine can receive.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#XState Integration#XState Types§SketchpadMachineEvent](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/XSTATE-TYPES/SKETCHPADMACHINEEVENT)
 **/
export type SketchpadMachineEvent =
  | { type: "NAVIGATE"; path: string }
  | { type: "NAVIGATE_BACK" }
  | { type: "NAVIGATE_FORWARD" }
  | { type: "SET_THEME"; theme: Theme }
  | { type: "SET_LANGUAGE"; language: string }
  | { type: "SET_EXPERTISE"; expertise: Expertise }
  | { type: "SET_MODE"; mode: Mode }
  | { type: "SET_DEVICE"; device: Device }
  | { type: "TOGGLE_FULLSCREEN" }
  | { type: "SET_PANEL_SIZE"; panel: keyof PanelSizes; size: number }
  | { type: "CREATE_KIT"; kit: Kit }
  | { type: "DELETE_KIT"; guid: Guid }
  | { type: "Y_UPDATE"; data: any }
  | { type: "Y_FIELD_UPDATE"; field: string; value: any };

/**
 * XState context for a kit machine with GUID, kit data, types, designs, and files.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#XState Integration#XState Types§KitMachineContext](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/XSTATE-TYPES/KITMACHINECONTEXT)
 **/
export interface KitMachineContext extends YjsSyncContext {
  guid: Guid;
  kit: Kit;

  types: Record<Guid, any>;

  designs: Record<Guid, any>;

  fileUrls: Map<string, string>;

  local: boolean;

  remote: boolean;
}

/**
 * Union of all events the kit machine can receive.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#XState Integration#XState Types§KitMachineEvent](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/XSTATE-TYPES/KITMACHINEEVENT)
 **/
export type KitMachineEvent =
  | { type: "LOAD" }
  | { type: "CHANGE"; diff: KitDiff }
  | { type: "CREATE_TYPE"; typeData: any }
  | { type: "UPDATE_TYPE"; guid: Guid; diff: any }
  | { type: "DELETE_TYPE"; guid: Guid }
  | { type: "CREATE_DESIGN"; design: any }
  | { type: "UPDATE_DESIGN"; guid: Guid; diff: any }
  | { type: "DELETE_DESIGN"; guid: Guid }
  | { type: "Y_UPDATE"; data: any };

/**
 * XState context for an app machine with panels, selection, hover, and transaction state.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#XState Integration#XState Types§AppMachineContext](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/XSTATE-TYPES/APPMACHINECONTEXT)
 **/
export interface AppMachineContext<TSelection = any> extends YjsSyncContext {
  panelVisibility: PanelVisibility;
  selection?: TSelection;
  hover?: any;
  presence?: any;
  others: any[];

  isTransactionActive: boolean;
  currentTransactionStack: any[];
  pastTransactionsStack: any[];
  redoStack: any[];
}

/**
 * Union of all events an app machine can receive.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#XState Integration#XState Types§AppMachineEvent](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/XSTATE-TYPES/APPMACHINEEVENT)
 **/
export type AppMachineEvent<TSelectionDiff = any, TDiff = any> =
  | { type: "START_TRANSACTION" }
  | { type: "FINALIZE_TRANSACTION" }
  | { type: "ABORT_TRANSACTION" }
  | { type: "UNDO" }
  | { type: "REDO" }
  | { type: "TOGGLE_PANEL"; panel: keyof PanelVisibility }
  | { type: "SELECT"; diff: TSelectionDiff }
  | { type: "DESELECT" }
  | { type: "HOVER"; data: any }
  | { type: "CLEAR_HOVER" }
  | { type: "CHANGE"; diff: TDiff }
  | { type: "Y_UPDATE"; data: any };

/**
 * Extended app machine context with a kit GUID for kit-diff-aware apps.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#XState Integration#XState Types§KitDiffAppMachineContext](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/XSTATE-TYPES/KITDIFFAPPMACHINECONTEXT)
 **/
export interface KitDiffAppMachineContext<TSelection = any> extends AppMachineContext<TSelection> {
  kitGuid: Guid;
}

// #endregion 🔖XState Types

// #region 🔖Y.js-XState Bridge

// [🔖semio/js/sketchpad/shared.ts#Y.js-XState Bridge](semiorepo://section/semio/js/sketchpad/shared.ts/Y-JS-XSTATE-BRIDGE)
// MUST bridge Y.js document observation to XState machine events.

/**
 * Creates an XState callback actor that observes a Y.js map and sends Y_UPDATE events.
 *
 * MUST observe the Y.js map deeply and send Y_UPDATE events on every change.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#XState Integration#Y.js-XState Bridge§createYjsSyncActor](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/Y-JS-XSTATE-BRIDGE/CREATEYJSSYNCACTOR)
 **/
export function createYjsSyncActor(yMap: Y.Map<any>) {
  return fromCallback<{ type: "Y_UPDATE"; data: any }>(({ sendBack }: { sendBack: (event: { type: "Y_UPDATE"; data: any }) => void }) => {
    const observer = () => {
      sendBack({ type: "Y_UPDATE", data: yMap.toJSON() });
    };

    observer();

    yMap.observeDeep(observer);

    return () => {
      yMap.unobserveDeep(observer);
    };
  });
}

/**
 * Creates an XState callback actor that observes a single field in a Y.js map.
 *
 * MUST observe a specific field in the Y.js map and send Y_FIELD_UPDATE events.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#XState Integration#Y.js-XState Bridge§createYjsFieldSyncActor](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/Y-JS-XSTATE-BRIDGE/CREATEYJSFIELDSYNCACTOR)
 **/
export function createYjsFieldSyncActor(yMap: Y.Map<any>, field: string) {
  return fromCallback<{ type: "Y_FIELD_UPDATE"; field: string; value: any }>(({ sendBack }: { sendBack: (event: { type: "Y_FIELD_UPDATE"; field: string; value: any }) => void }) => {
    const observer = (events: Y.YMapEvent<any>[]) => {
      for (const event of events) {
        if (event.keysChanged.has(field)) {
          sendBack({ type: "Y_FIELD_UPDATE", field, value: yMap.get(field) });
        }
      }
    };

    sendBack({ type: "Y_FIELD_UPDATE", field, value: yMap.get(field) });

    yMap.observe(observer as any);

    return () => {
      yMap.unobserve(observer as any);
    };
  });
}

/**
 * Executes a function within a Y.js document transaction.
 *
 * MUST delegate to the Y.Doc transact method with the given origin.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#XState Integration#Y.js-XState Bridge§yTransact](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/Y-JS-XSTATE-BRIDGE/YTRANSACT)
 **/
export function yTransact(yDoc: Y.Doc, fn: () => void, origin?: string): void {
  yDoc.transact(fn, origin);
}

/**
 * Creates an XState assign action that marks dirty and caches Y_UPDATE event data.
 *
 * MUST return an XState assign that sets dirty to true and caches event data.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#XState Integration#Y.js-XState Bridge§createYjsUpdateAssign](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/Y-JS-XSTATE-BRIDGE/CREATEYJSUPDATEASSIGN)
 **/
export function createYjsUpdateAssign() {
  return assign({
    dirty: () => true,
    cache: ({ event }: { event: { type: "Y_UPDATE"; data: any } }) => (event as any).data,
  });
}

/**
 * Creates a memoized XState selector that rebuilds only when context is dirty.
 *
 * MUST return cached snapshot when not dirty, rebuilding only when dirty.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#XState Integration#Y.js-XState Bridge§createYjsSelector](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/Y-JS-XSTATE-BRIDGE/CREATEYJSSELECTOR)
 **/
export function createYjsSelector<TContext extends YjsSyncContext, TSnapshot>(buildSnapshot: (context: TContext) => TSnapshot): (context: TContext) => TSnapshot {
  return (context: TContext): TSnapshot => {
    if (!context.dirty && context.cache) {
      return context.cache as TSnapshot;
    }
    return buildSnapshot(context);
  };
}

// #endregion 🔖Y.js-XState Bridge

// #region 🔖Machine Factories

// [🔖semio/js/sketchpad/shared.ts#Machine Factories](semiorepo://section/semio/js/sketchpad/shared.ts/MACHINE-FACTORIES)
// MUST define machine input and transaction configuration interfaces for state machine creation.

/**
 * Input for creating an app machine with Y.js map and transact function.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#XState Integration#Machine Factories§AppMachineInput](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/MACHINE-FACTORIES/APPMACHINEINPUT)
 **/
export interface AppMachineInput {
  yMap: Y.Map<any>;
  transact: Transact;
}

/**
 * Extended app machine input with a kit GUID.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#XState Integration#Machine Factories§KitDiffAppMachineInput](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/MACHINE-FACTORIES/KITDIFFAPPMACHINEINPUT)
 **/
export interface KitDiffAppMachineInput extends AppMachineInput {
  kitGuid: Guid;
}

/**
 * Configuration for transaction handling with apply and inverse functions.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#XState Integration#Machine Factories§TransactionMachineConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/XSTATE-INTEGRATION/MACHINE-FACTORIES/TRANSACTIONMACHINECONFIG)
 **/
export interface TransactionMachineConfig<TEdit = any> {
  applySelectionDiff: (selectionDiff: any) => void;

  inverseSelectionDiff: (selection: any, diff: any) => any;

  applyKitDiff?: (kitDiff: KitDiff) => void;

  inverseKitDiff?: (kit: Kit, diff: KitDiff) => KitDiff;
}

// #endregion 🔖Machine Factories

// #endregion 🔖XState Integration

// #region 🔖YPath Helpers

// [🔖semio/js/sketchpad/shared.ts#YPath Helpers](semiorepo://section/semio/js/sketchpad/shared.ts/YPATH-HELPERS)
// MUST provide path segment constructors, value retrieval, and observation functions for Y.js paths.

/**
 * Creates a YPathSegment for accessing a map key.
 *
 * MUST return a mapKey segment with the given key.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#YPath Helpers§yPathMapKey](semiorepo://definition/semio/js/sketchpad/shared.ts/YPATH-HELPERS/YPATHMAPKEY)
 **/
export function yPathMapKey(key: string): YPathSegment {
  return { kind: "mapKey", key };
}

/**
 * Creates a YPathSegment for accessing an array element by index.
 *
 * MUST return an arrayIndex segment with the given index.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#YPath Helpers§yPathArrayIndex](semiorepo://definition/semio/js/sketchpad/shared.ts/YPATH-HELPERS/YPATHARRAYINDEX)
 **/
export function yPathArrayIndex(index: number): YPathSegment {
  return { kind: "arrayIndex", index };
}

/**
 * Creates a YPathSegment for accessing an array item by its ID field.
 *
 * MUST return an arrayItemById segment with the given ID and idKey.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#YPath Helpers§yPathArrayItemById](semiorepo://definition/semio/js/sketchpad/shared.ts/YPATH-HELPERS/YPATHARRAYITEMBYID)
 **/
export function yPathArrayItemById(id: string, idKey: string = "guid"): YPathSegment {
  return { kind: "arrayItemById", id, idKey };
}

/**
 * Traverses a Y.js map or array along a YPath and returns the value at the end.
 *
 * MUST traverse each path segment, returning undefined when a segment cannot be resolved.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#YPath Helpers§getValueAtPath](semiorepo://definition/semio/js/sketchpad/shared.ts/YPATH-HELPERS/GETVALUEATPATH)
 **/
export function getValueAtPath(root: Y.Map<any> | Y.Array<any>, path: YPath): any {
  let current: any = root;
  for (const segment of path) {
    if (current === undefined || current === null) return undefined;
    if (segment.kind === "mapKey") {
      if (!(current instanceof Y.Map)) return undefined;
      current = current.get(segment.key);
    } else if (segment.kind === "arrayIndex") {
      if (!(current instanceof Y.Array)) return undefined;
      current = current.get(segment.index);
    } else if (segment.kind === "arrayItemById") {
      if (!(current instanceof Y.Array)) return undefined;
      const arr = current.toArray();
      const item = arr.find((item: any) => {
        if (item instanceof Y.Map) return item.get(segment.idKey) === segment.id;
        return item?.[segment.idKey] === segment.id;
      });
      current = item;
    }
  }
  return current;
}

/**
 * Sets up deep observers along a YPath and calls subscribe when the leaf value changes.
 *
 * MUST set up nested observers along the path and notify when the leaf value changes.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#YPath Helpers§createPathObserver](semiorepo://definition/semio/js/sketchpad/shared.ts/YPATH-HELPERS/CREATEPATHOBSERVER)
 **/
export function createPathObserver(root: Y.Map<any>, path: YPath, subscribe: Subscribe): Disposable {
  if (path.length === 0) {
    const callback = () => subscribe(() => { });
    root.observeDeep(callback);
    return () => root.unobserveDeep(callback);
  }
  const disposables: Disposable[] = [];
  let lastValue = getValueAtPath(root, path);
  const notifyIfChanged = () => {
    const newValue = getValueAtPath(root, path);
    const lastJson = JSON.stringify(lastValue instanceof Y.Map || lastValue instanceof Y.Array ? lastValue.toJSON() : lastValue);
    const newJson = JSON.stringify(newValue instanceof Y.Map || newValue instanceof Y.Array ? newValue.toJSON() : newValue);
    if (lastJson !== newJson) {
      lastValue = newValue;
      subscribe(() => { });
    }
  };
  const setupObservers = (current: any, remainingPath: YPath, depth: number) => {
    if (!current || remainingPath.length === 0) return;
    const segment = remainingPath[0];
    const rest = remainingPath.slice(1);
    if (segment.kind === "mapKey" && current instanceof Y.Map) {
      const mapCallback = (event: Y.YMapEvent<any>) => {
        if (event.keysChanged.has(segment.key)) {
          disposables.slice(depth + 1).forEach((d) => d());
          disposables.length = depth + 1;
          const next = current.get(segment.key);
          if (rest.length > 0 && next) setupObservers(next, rest, depth + 1);
          notifyIfChanged();
        }
      };
      current.observe(mapCallback);
      disposables.push(() => current.unobserve(mapCallback));
      const next = current.get(segment.key);
      if (rest.length > 0 && next) setupObservers(next, rest, depth + 1);
      else if (rest.length === 0 && next instanceof Y.Map) {
        const deepCallback = () => notifyIfChanged();
        next.observeDeep(deepCallback);
        disposables.push(() => next.unobserveDeep(deepCallback));
      } else if (rest.length === 0 && next instanceof Y.Array) {
        const deepCallback = () => notifyIfChanged();
        next.observeDeep(deepCallback);
        disposables.push(() => next.unobserveDeep(deepCallback));
      }
    } else if (segment.kind === "arrayIndex" && current instanceof Y.Array) {
      const arrayCallback = () => notifyIfChanged();
      current.observe(arrayCallback);
      disposables.push(() => current.unobserve(arrayCallback));
      const next = current.get(segment.index);
      if (rest.length > 0 && next) setupObservers(next, rest, depth + 1);
    } else if (segment.kind === "arrayItemById" && current instanceof Y.Array) {
      const arrayCallback = () => {
        disposables.slice(depth + 1).forEach((d) => d());
        disposables.length = depth + 1;
        const arr = current.toArray();
        const item = arr.find((item: any) => {
          if (item instanceof Y.Map) return item.get(segment.idKey) === segment.id;
          return item?.[segment.idKey] === segment.id;
        });
        if (rest.length > 0 && item) setupObservers(item, rest, depth + 1);
        notifyIfChanged();
      };
      current.observe(arrayCallback);
      disposables.push(() => current.unobserve(arrayCallback));
      const arr = current.toArray();
      const item = arr.find((item: any) => {
        if (item instanceof Y.Map) return item.get(segment.idKey) === segment.id;
        return item?.[segment.idKey] === segment.id;
      });
      if (rest.length > 0 && item) setupObservers(item, rest, depth + 1);
    }
  };
  setupObservers(root, path, 0);
  return () => disposables.forEach((d) => d());
}

// #endregion 🔖YPath Helpers

// #region 🔖Derived Store

// [🔖semio/js/sketchpad/shared.ts#Derived Store](semiorepo://section/semio/js/sketchpad/shared.ts/DERIVED-STORE)
// MUST provide reactive derived computation nodes with dependency tracking and caching.

/**
 * A dependency on a store path used by DerivedNode for change tracking.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Derived Store§BaseDependency](semiorepo://definition/semio/js/sketchpad/shared.ts/DERIVED-STORE/BASEDEPENDENCY)
 **/
export interface BaseDependency {
  store: { onPathChanged: (path: YPath, subscribe: Subscribe) => Disposable; getPathSnapshot: (path: YPath) => any };
  path: YPath;
}

/**
 * A reactive computation node that recomputes when its dependencies change.
 *
 * MUST lazily initialize observers and recompute only when dependency values change.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Derived Store§DerivedNode](semiorepo://definition/semio/js/sketchpad/shared.ts/DERIVED-STORE/DERIVEDNODE)
 **/
export class DerivedNode<T> {
  private deps: BaseDependency[];
  private compute: () => T;
  private value: T | undefined;
  private valueJson?: string;
  private subscribers = new Set<() => void>();
  private unsubscribers: Disposable[] = [];
  private initialized = false;

  constructor(deps: BaseDependency[], compute: () => T) {
    this.deps = deps;
    this.compute = compute;
  }

  private init() {
    if (this.initialized) return;
    this.initialized = true;
    this.unsubscribers = this.deps.map((d) =>
      d.store.onPathChanged(d.path, () => {
        this.recompute();
        return () => { };
      }),
    );
    this.recompute();
  }

  private recompute() {
    const next = this.compute();
    const nextJson = JSON.stringify(next);
    if (nextJson !== this.valueJson) {
      this.value = next;
      this.valueJson = nextJson;
      this.subscribers.forEach((cb) => cb());
    }
  }

  snapshot(): T {
    if (!this.initialized) this.init();
    if (this.value === undefined) this.recompute();
    return this.value!;
  }

  subscribe(cb: () => void): Disposable {
    if (!this.initialized) this.init();
    this.subscribers.add(cb);
    return () => {
      this.subscribers.delete(cb);
      if (this.subscribers.size === 0) {
        this.unsubscribers.forEach((u) => u());
        this.unsubscribers = [];
        this.initialized = false;
        this.value = undefined;
        this.valueJson = undefined;
      }
    };
  }

  dispose() {
    this.unsubscribers.forEach((u) => u());
    this.unsubscribers = [];
    this.subscribers.clear();
    this.initialized = false;
    this.value = undefined;
    this.valueJson = undefined;
  }
}

/**
 * A keyed collection of DerivedNode instances with lifecycle management.
 *
 * MUST manage DerivedNode lifecycle including creation, retrieval, and disposal.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Derived Store§DerivedStore](semiorepo://definition/semio/js/sketchpad/shared.ts/DERIVED-STORE/DERIVEDSTORE)
 **/
export class DerivedStore {
  private nodes = new Map<string, DerivedNode<any>>();

  getOrCreate<T>(key: string, deps: BaseDependency[], compute: () => T): DerivedNode<T> {
    if (!this.nodes.has(key)) {
      this.nodes.set(key, new DerivedNode<T>(deps, compute));
    }
    return this.nodes.get(key)! as DerivedNode<T>;
  }

  get<T>(key: string): DerivedNode<T> | undefined {
    return this.nodes.get(key) as DerivedNode<T> | undefined;
  }

  delete(key: string): boolean {
    const node = this.nodes.get(key);
    if (node) {
      node.dispose();
      this.nodes.delete(key);
      return true;
    }
    return false;
  }

  clear() {
    this.nodes.forEach((node) => node.dispose());
    this.nodes.clear();
  }

  has(key: string): boolean {
    return this.nodes.has(key);
  }

  keys(): IterableIterator<string> {
    return this.nodes.keys();
  }
}

// #endregion 🔖Derived Store

// #region 🔖Store Factory Registry

// [🔖semio/js/sketchpad/shared.ts#Store Factory Registry](semiorepo://section/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY)
// MUST manage registration and retrieval of app-specific store factory functions.

/**
 * Factory function type for creating a design app store.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§DesignAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/DESIGNAPPSTOREFACTORY)
 **/
export type DesignAppStoreFactory = (parent: any, id: any, state?: any) => any;
/**
 * Factory function type for creating a kit app store.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§KitAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/KITAPPSTOREFACTORY)
 **/
export type KitAppStoreFactory = (parent: any, yMap: any, transact: (fn: () => void) => void, id: any, state?: any) => any;
/**
 * Factory function type for creating a type app store.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§TypeAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/TYPEAPPSTOREFACTORY)
 **/
export type TypeAppStoreFactory = (parent: any, id: any, state?: any) => any;
/**
 * Factory function type for creating a quality app store.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§QualityAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/QUALITYAPPSTOREFACTORY)
 **/
export type QualityAppStoreFactory = (parent: any, id: any, state?: any) => any;

let designAppStoreFactory: DesignAppStoreFactory | undefined;
let kitAppStoreFactory: KitAppStoreFactory | undefined;
let typeAppStoreFactory: TypeAppStoreFactory | undefined;
let qualityAppStoreFactory: QualityAppStoreFactory | undefined;

/**
 * Registers the design app store factory.
 *
 * MUST replace any previously registered design app store factory.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§registerDesignAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/REGISTERDESIGNAPPSTOREFACTORY)
 **/
export function registerDesignAppStoreFactory(factory: DesignAppStoreFactory) {
  designAppStoreFactory = factory;
}

/**
 * Registers the kit app store factory.
 *
 * MUST replace any previously registered kit app store factory.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§registerKitAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/REGISTERKITAPPSTOREFACTORY)
 **/
export function registerKitAppStoreFactory(factory: KitAppStoreFactory) {
  kitAppStoreFactory = factory;
}

/**
 * Registers the type app store factory.
 *
 * MUST replace any previously registered type app store factory.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§registerTypeAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/REGISTERTYPEAPPSTOREFACTORY)
 **/
export function registerTypeAppStoreFactory(factory: TypeAppStoreFactory) {
  typeAppStoreFactory = factory;
}

/**
 * Registers the quality app store factory.
 *
 * MUST replace any previously registered quality app store factory.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§registerQualityAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/REGISTERQUALITYAPPSTOREFACTORY)
 **/
export function registerQualityAppStoreFactory(factory: QualityAppStoreFactory) {
  qualityAppStoreFactory = factory;
}

/**
 * Retrieves the registered design app store factory or throws if not registered.
 *
 * MUST throw if no design app store factory has been registered.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§getDesignAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/GETDESIGNAPPSTOREFACTORY)
 **/
export function getDesignAppStoreFactory(): DesignAppStoreFactory {
  if (!designAppStoreFactory) throw new Error("Design app store factory not registered");
  return designAppStoreFactory;
}

/**
 * Retrieves the registered kit app store factory or throws if not registered.
 *
 * MUST throw if no kit app store factory has been registered.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§getKitAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/GETKITAPPSTOREFACTORY)
 **/
export function getKitAppStoreFactory(): KitAppStoreFactory {
  if (!kitAppStoreFactory) throw new Error("Kit app store factory not registered");
  return kitAppStoreFactory;
}

/**
 * Retrieves the registered type app store factory or throws if not registered.
 *
 * MUST throw if no type app store factory has been registered.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§getTypeAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/GETTYPEAPPSTOREFACTORY)
 **/
export function getTypeAppStoreFactory(): TypeAppStoreFactory {
  if (!typeAppStoreFactory) throw new Error("Type app store factory not registered");
  return typeAppStoreFactory;
}

/**
 * Retrieves the registered quality app store factory or throws if not registered.
 *
 * MUST throw if no quality app store factory has been registered.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Store Factory Registry§getQualityAppStoreFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/STORE-FACTORY-REGISTRY/GETQUALITYAPPSTOREFACTORY)
 **/
export function getQualityAppStoreFactory(): QualityAppStoreFactory {
  if (!qualityAppStoreFactory) throw new Error("Quality app store factory not registered");
  return qualityAppStoreFactory;
}

// #endregion 🔖Store Factory Registry

// #region 🔖App Plugin Registry

// [🔖semio/js/sketchpad/shared.ts#App Plugin Registry](semiorepo://section/semio/js/sketchpad/shared.ts/APP-PLUGIN-REGISTRY)
// MUST manage plugin registration, retrieval, and contribution composition for app extensions.

/**
 * Plugin contribution of event types, actions, guards, handlers, selectors, and default state.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#App Plugin Registry§AppMachineContribution](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-PLUGIN-REGISTRY/APPMACHINECONTRIBUTION)
 **/
export interface AppMachineContribution {
  eventTypes?: Record<string, any>;

  actions?: Record<string, (context: any, event: any) => any>;

  guards?: Record<string, (context: any, event: any) => boolean>;

  eventHandlers?: Record<string, { guard?: string; actions?: string | string[] }>;

  selectors?: Record<string, (context: any, ...args: any[]) => any>;

  createDefaultState?: () => any;
}

/**
 * An app plugin with ID, namespace, machine contribution, and lifecycle hooks.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#App Plugin Registry§AppPlugin](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-PLUGIN-REGISTRY/APPPLUGIN)
 **/
export interface AppPlugin {
  id: string;

  namespace: string;

  machine: AppMachineContribution;

  registerStores?: () => void;

  onRegister?: () => void;
}

const appPlugins: Map<string, AppPlugin> = new Map();

/**
 * Registers an app plugin, invoking its store registration and onRegister hooks.
 *
 * MUST store the plugin and invoke registerStores and onRegister hooks if present.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Plugin Registry§registerAppPlugin](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-PLUGIN-REGISTRY/REGISTERAPPPLUGIN)
 **/
export function registerAppPlugin(plugin: AppPlugin): void {
  if (appPlugins.has(plugin.id)) {
    console.warn(`App plugin "${plugin.id}" already registered, replacing...`);
  }
  appPlugins.set(plugin.id, plugin);

  if (plugin.registerStores) {
    plugin.registerStores();
  }

  if (plugin.onRegister) {
    plugin.onRegister();
  }
}

/**
 * Returns all registered app plugins.
 *
 * MUST return all registered plugins as an array.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Plugin Registry§getAppPlugins](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-PLUGIN-REGISTRY/GETAPPPLUGINS)
 **/
export function getAppPlugins(): AppPlugin[] {
  return Array.from(appPlugins.values());
}

/**
 * Returns the registered app plugin with the given ID, or undefined.
 *
 * MUST look up the plugin by ID in the registry.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Plugin Registry§getAppPlugin](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-PLUGIN-REGISTRY/GETAPPPLUGIN)
 **/
export function getAppPlugin(id: string): AppPlugin | undefined {
  return appPlugins.get(id);
}

/**
 * Checks whether an app plugin with the given ID is registered.
 *
 * MUST check the registry for the given plugin ID.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Plugin Registry§hasAppPlugin](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-PLUGIN-REGISTRY/HASAPPPLUGIN)
 **/
export function hasAppPlugin(id: string): boolean {
  return appPlugins.has(id);
}

/**
 * Merges actions, guards, event handlers, and selectors from all registered plugins.
 *
 * MUST iterate all plugins and merge their contributions into single records.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Plugin Registry§composePluginContributions](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-PLUGIN-REGISTRY/COMPOSEPLUGINCONTRIBUTIONS)
 **/
export function composePluginContributions(): {
  actions: Record<string, (context: any, event: any) => any>;
  guards: Record<string, (context: any, event: any) => boolean>;
  eventHandlers: Record<string, { guard?: string; actions?: string | string[] }>;
  selectors: Record<string, (context: any, ...args: any[]) => any>;
} {
  const actions: Record<string, any> = {};
  const guards: Record<string, any> = {};
  const eventHandlers: Record<string, any> = {};
  const selectors: Record<string, any> = {};

  for (const plugin of appPlugins.values()) {
    const contribution = plugin.machine;

    if (contribution.actions) {
      for (const [name, fn] of Object.entries(contribution.actions)) {
        actions[name] = fn;
      }
    }

    if (contribution.guards) {
      for (const [name, fn] of Object.entries(contribution.guards)) {
        guards[name] = fn;
      }
    }

    if (contribution.eventHandlers) {
      for (const [eventType, handler] of Object.entries(contribution.eventHandlers)) {
        eventHandlers[eventType] = handler;
      }
    }

    if (contribution.selectors) {
      for (const [name, fn] of Object.entries(contribution.selectors)) {
        selectors[`${plugin.id}.${name}`] = fn;
      }
    }
  }

  return { actions, guards, eventHandlers, selectors };
}

/**
 * Collects default states from all registered plugins.
 *
 * MUST call createDefaultState on each plugin that defines it.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Plugin Registry§getPluginDefaultStates](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-PLUGIN-REGISTRY/GETPLUGINDEFAULTSTATES)
 **/
export function getPluginDefaultStates(): Record<string, any> {
  const defaults: Record<string, any> = {};
  for (const plugin of appPlugins.values()) {
    const createDefaultState = plugin.machine.createDefaultState;
    if (!createDefaultState) continue;
    defaults[plugin.id] = createDefaultState();
  }
  return defaults;
}

// #endregion 🔖App Plugin Registry

// #region 🔖Dynamic Event Dispatch Registry

// [🔖semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry](semiorepo://section/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY)
// MUST manage dynamic event handler and guard registration with namespace-based dispatch.

/**
 * Configuration for a dynamic event handler with optional guard and action.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§EventHandlerConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/EVENTHANDLERCONFIG)
 **/
export interface EventHandlerConfig<TContext = any, TEvent = any> {
  guard?: (context: TContext, event: TEvent) => boolean;

  action: (context: TContext, event: TEvent) => Partial<TContext>;
}

const eventHandlerRegistry: Map<string, EventHandlerConfig> = new Map();

const guardRegistry: Map<string, (context: any, event: any) => boolean> = new Map();

/**
 * Registers a dynamic event handler for a given event type.
 *
 * MUST store the handler config in the registry keyed by event type.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§registerEventHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/REGISTEREVENTHANDLER)
 **/
export function registerEventHandler<TContext = any, TEvent = any>(eventType: string, config: EventHandlerConfig<TContext, TEvent>): void {
  eventHandlerRegistry.set(eventType, config as EventHandlerConfig);
}

/**
 * Removes a registered event handler for a given event type.
 *
 * MUST remove the handler for the given event type.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§unregisterEventHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/UNREGISTEREVENTHANDLER)
 **/
export function unregisterEventHandler(eventType: string): void {
  eventHandlerRegistry.delete(eventType);
}

/**
 * Checks whether an event handler is registered for a given event type.
 *
 * MUST check the registry for the given event type.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§hasEventHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/HASEVENTHANDLER)
 **/
export function hasEventHandler(eventType: string): boolean {
  return eventHandlerRegistry.has(eventType);
}

/**
 * Retrieves the event handler configuration for a given event type.
 *
 * MUST return the handler config or undefined.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§getEventHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/GETEVENTHANDLER)
 **/
export function getEventHandler(eventType: string): EventHandlerConfig | undefined {
  return eventHandlerRegistry.get(eventType);
}

/**
 * Executes the registered event handler for the given event, applying guard and action.
 *
 * MUST run the guard before the action, returning empty context when guard fails.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§executeEventHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/EXECUTEEVENTHANDLER)
 **/
export function executeEventHandler<TContext = any, TEvent extends { type: string } = any>(context: TContext, event: TEvent): Partial<TContext> {
  const handler = eventHandlerRegistry.get(event.type);
  if (!handler) return {};

  if (handler.guard && !handler.guard(context, event)) {
    return {};
  }

  return handler.action(context, event);
}

/**
 * Registers a named guard function.
 *
 * MUST store the guard function keyed by name.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§registerGuard](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/REGISTERGUARD)
 **/
export function registerGuard(name: string, guard: (context: any, event: any) => boolean): void {
  guardRegistry.set(name, guard);
}

/**
 * Removes a registered guard function by name.
 *
 * MUST remove the guard function by name.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§unregisterGuard](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/UNREGISTERGUARD)
 **/
export function unregisterGuard(name: string): void {
  guardRegistry.delete(name);
}

/**
 * Retrieves a registered guard function by name.
 *
 * MUST return the guard function or undefined.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§getGuard](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/GETGUARD)
 **/
export function getGuard(name: string): ((context: any, event: any) => boolean) | undefined {
  return guardRegistry.get(name);
}

/**
 * Checks whether a guard with the given name is registered.
 *
 * MUST check the guard registry for the given name.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§hasGuard](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/HASGUARD)
 **/
export function hasGuard(name: string): boolean {
  return guardRegistry.has(name);
}

/**
 * Executes a registered guard and returns its boolean result.
 *
 * MUST return false when the guard is not registered.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§executeGuard](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/EXECUTEGUARD)
 **/
export function executeGuard(name: string, context: any, event: any): boolean {
  const guard = guardRegistry.get(name);
  if (!guard) return false;
  return guard(context, event);
}

/**
 * Returns all registered event types matching a given namespace prefix.
 *
 * MUST filter event types by the namespace prefix.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§getEventTypesForNamespace](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/GETEVENTTYPESFORNAMESPACE)
 **/
export function getEventTypesForNamespace(namespace: string): string[] {
  const prefix = `${namespace}.`;
  return Array.from(eventHandlerRegistry.keys()).filter((key) => key.startsWith(prefix));
}

/**
 * Returns all unique namespaces from registered event types.
 *
 * MUST extract unique namespace prefixes from all registered event types.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§getRegisteredNamespaces](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/GETREGISTEREDNAMESPACES)
 **/
export function getRegisteredNamespaces(): string[] {
  const namespaces = new Set<string>();
  for (const eventType of eventHandlerRegistry.keys()) {
    const dotIndex = eventType.indexOf(".");
    if (dotIndex > 0) {
      namespaces.add(eventType.substring(0, dotIndex));
    }
  }
  return Array.from(namespaces);
}

/**
 * Returns all registered event type strings.
 *
 * MUST return all event type strings from the registry.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Dynamic Event Dispatch Registry§getRegisteredEventTypes](semiorepo://definition/semio/js/sketchpad/shared.ts/DYNAMIC-EVENT-DISPATCH-REGISTRY/GETREGISTEREDEVENTTYPES)
 **/
export function getRegisteredEventTypes(): string[] {
  return Array.from(eventHandlerRegistry.keys());
}

// #endregion 🔖Dynamic Event Dispatch Registry

// #region 🔖App Event Handler Factories

// [🔖semio/js/sketchpad/shared.ts#App Event Handler Factories](semiorepo://section/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES)
// MUST provide factory functions for creating standard app event handlers for panels, hover, selection, and windows.

/**
 * Configuration for an app event handler with namespace, app key, and default state factory.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#App Event Handler Factories§AppEventHandlerConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/APPEVENTHANDLERCONFIG)
 **/
export interface AppEventHandlerConfig<TAppKey extends string, TAppState> {
  namespace: string;
  appKey: TAppKey;
  createDefaultState: () => TAppState;
}

/**
 * Registers a toggle panel event handler for the given app config.
 *
 * MUST register a handler that toggles the specified panel in panelVisibility.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createTogglePanelHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATETOGGLEPANELHANDLER)
 **/
export function createTogglePanelHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: AppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.TOGGLE_PANEL`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...app,
          panelVisibility: {
            ...app.panelVisibility,
            [event.panel]: !app.panelVisibility[event.panel],
          },
        },
      };
    },
  });
}

/**
 * Registers a set panel visibility event handler for the given app config.
 *
 * MUST register a handler that replaces the entire panelVisibility.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSetPanelVisibilityHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESETPANELVISIBILITYHANDLER)
 **/
export function createSetPanelVisibilityHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: AppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_PANEL_VISIBILITY`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: { ...app, panelVisibility: event.panelVisibility },
      };
    },
  });
}

/**
 * Registers a set hover event handler with a mapper for the given app config.
 *
 * MUST register a handler that sets hover using the provided mapper.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSetHoverHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESETHOVERHANDLER)
 **/
export function createSetHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: AppEventHandlerConfig<TAppKey, TAppState>, hoverMapper: (event: any) => any): void {
  const eventType = `${config.namespace}.SET_HOVER`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: { ...app, hover: hoverMapper(event) },
      };
    },
  });
}

/**
 * Registers a clear hover event handler with a guard for the given app config.
 *
 * MUST register a handler with a guard that only clears non-empty hover state.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createClearHoverHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATECLEARHOVERHANDLER)
 **/
export function createClearHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: AppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.CLEAR_HOVER`;
  registerEventHandler(eventType, {
    guard: (context: any) => {
      const app = context[config.appKey];
      const hover = app?.hover;
      return hover !== undefined && Object.keys(hover).some((k) => hover[k] !== undefined && (Array.isArray(hover[k]) ? hover[k].length > 0 : true));
    },
    action: (context: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: { ...app, hover: undefined },
      };
    },
  });
}

/**
 * Registers a set window layout event handler for the given app config.
 *
 * MUST register a handler that sets the windowLayout from the event.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSetWindowLayoutHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESETWINDOWLAYOUTHANDLER)
 **/
export function createSetWindowLayoutHandler<TAppKey extends string, TAppState extends { windowLayout?: any }>(config: AppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_WINDOW_LAYOUT`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: { ...app, windowLayout: event.windowLayout },
      };
    },
  });
}

/**
 * Registers a clear selection event handler for the given app config.
 *
 * MUST register a handler that sets selection to undefined.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createClearSelectionHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATECLEARSELECTIONHANDLER)
 **/
export function createClearSelectionHandler<TAppKey extends string, TAppState extends { selection?: any }>(config: AppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.CLEAR_SELECTION`;
  registerEventHandler(eventType, {
    action: (context: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: { ...app, selection: undefined },
      };
    },
  });
}

/**
 * Extended app event handler config with a getKey function for keyed state.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#App Event Handler Factories§KeyedAppEventHandlerConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/KEYEDAPPEVENTHANDLERCONFIG)
 **/
export interface KeyedAppEventHandlerConfig<TAppKey extends string, TAppState> extends AppEventHandlerConfig<TAppKey, TAppState> {
  getKey: (event: any) => string;
}

/**
 * Registers a keyed toggle panel event handler for multi-instance app state.
 *
 * MUST register a keyed handler that toggles the panel for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedTogglePanelHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDTOGGLEPANELHANDLER)
 **/
export function createKeyedTogglePanelHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.TOGGLE_PANEL`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: {
            ...app,
            panelVisibility: {
              ...app.panelVisibility,
              [event.panel]: !app.panelVisibility[event.panel],
            },
          },
        },
      };
    },
  });
}

/**
 * Registers a keyed set panel visibility event handler for multi-instance app state.
 *
 * MUST register a keyed handler that replaces panelVisibility for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedSetPanelVisibilityHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDSETPANELVISIBILITYHANDLER)
 **/
export function createKeyedSetPanelVisibilityHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_PANEL_VISIBILITY`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, panelVisibility: event.panelVisibility },
        },
      };
    },
  });
}

/**
 * Registers a keyed set hover event handler for multi-instance app state.
 *
 * MUST register a keyed handler that sets hover for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedSetHoverHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDSETHOVERHANDLER)
 **/
export function createKeyedSetHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>, hoverMapper: (event: any) => any): void {
  const eventType = `${config.namespace}.SET_HOVER`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, hover: hoverMapper(event) },
        },
      };
    },
  });
}

/**
 * Registers a keyed clear hover event handler for multi-instance app state.
 *
 * MUST register a keyed handler that clears hover for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedClearHoverHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDCLEARHOVERHANDLER)
 **/
export function createKeyedClearHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.CLEAR_HOVER`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, hover: undefined },
        },
      };
    },
  });
}

/**
 * Registers a keyed set selection event handler for multi-instance app state.
 *
 * MUST register a keyed handler that sets selection for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedSetSelectionHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDSETSELECTIONHANDLER)
 **/
export function createKeyedSetSelectionHandler<TAppKey extends string, TAppState extends { selection?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_SELECTION`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, selection: event.selection },
        },
      };
    },
  });
}

/**
 * Registers a keyed clear selection event handler for multi-instance app state.
 *
 * MUST register a keyed handler that clears selection for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedClearSelectionHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDCLEARSELECTIONHANDLER)
 **/
export function createKeyedClearSelectionHandler<TAppKey extends string, TAppState extends { selection?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.CLEAR_SELECTION`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, selection: undefined },
        },
      };
    },
  });
}

/**
 * Registers a keyed set window layout event handler for multi-instance app state.
 *
 * MUST register a keyed handler that sets windowLayout for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedSetWindowLayoutHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDSETWINDOWLAYOUTHANDLER)
 **/
export function createKeyedSetWindowLayoutHandler<TAppKey extends string, TAppState extends { windowLayout?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_WINDOW_LAYOUT`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, windowLayout: event.windowLayout },
        },
      };
    },
  });
}

/**
 * Registers a keyed set camera event handler for multi-instance app state.
 *
 * MUST register a keyed handler that sets camera for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedSetCameraHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDSETCAMERAHANDLER)
 **/
export function createKeyedSetCameraHandler<TAppKey extends string, TAppState extends { camera?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_CAMERA`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, camera: event.camera },
        },
      };
    },
  });
}

/**
 * Registers a keyed set active tool event handler for multi-instance app state.
 *
 * MUST register a keyed handler that sets activeTool for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedSetActiveToolHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDSETACTIVETOOLHANDLER)
 **/
export function createKeyedSetActiveToolHandler<TAppKey extends string, TAppState extends { activeTool?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_ACTIVE_TOOL`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, activeTool: event.tool },
        },
      };
    },
  });
}

/**
 * Registers a keyed set fullscreen window event handler for multi-instance app state.
 *
 * MUST register a keyed handler that sets fullscreenWindow for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedSetFullscreenWindowHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDSETFULLSCREENWINDOWHANDLER)
 **/
export function createKeyedSetFullscreenWindowHandler<TAppKey extends string, TAppState extends { fullscreenWindow?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_FULLSCREEN_WINDOW`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, fullscreenWindow: event.window },
        },
      };
    },
  });
}

/**
 * Registers a keyed init event handler that sets initial keyed app state.
 *
 * MUST register a keyed handler that initializes state for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedInitHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDINITHANDLER)
 **/
export function createKeyedInitHandler<TAppKey extends string, TAppState>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.INIT`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      return {
        [config.appKey]: {
          ...apps,
          [key]: event.state,
        },
      };
    },
  });
}

/**
 * Registers a keyed sync event handler that merges state for keyed app state.
 *
 * MUST register a keyed handler that merges state for the resolved key.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createKeyedSyncHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATEKEYEDSYNCHANDLER)
 **/
export function createKeyedSyncHandler<TAppKey extends string, TAppState>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SYNC`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, ...event.state },
        },
      };
    },
  });
}

/**
 * Registers all standard event handlers for a non-keyed app.
 *
 * MUST register toggle panel, set panel visibility, hover, clear hover, window layout, and clear selection handlers.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Event Handler Factories§registerStandardAppEventHandlers](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/REGISTERSTANDARDAPPEVENTHANDLERS)
 **/
export function registerStandardAppEventHandlers<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility; hover?: any; selection?: any; windowLayout?: any }>(
  config: AppEventHandlerConfig<TAppKey, TAppState>,
  hoverMapper: (event: any) => any = (e) => e.hover,
): void {
  createTogglePanelHandler(config);
  createSetPanelVisibilityHandler(config);
  createSetHoverHandler(config, hoverMapper);
  createClearHoverHandler(config);
  createSetWindowLayoutHandler(config);
  createClearSelectionHandler(config);
}

/**
 * Registers all standard event handlers for a keyed multi-instance app.
 *
 * MUST register init, sync, and all standard keyed handlers including camera, tool, and fullscreen.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Event Handler Factories§registerKeyedAppEventHandlers](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/REGISTERKEYEDAPPEVENTHANDLERS)
 **/
export function registerKeyedAppEventHandlers<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility; hover?: any; selection?: any; windowLayout?: any; camera?: any; activeTool?: any; fullscreenWindow?: any }>(
  config: KeyedAppEventHandlerConfig<TAppKey, TAppState>,
  hoverMapper: (event: any) => any = (e) => e.hover,
): void {
  createKeyedInitHandler(config);
  createKeyedSyncHandler(config);
  createKeyedTogglePanelHandler(config);
  createKeyedSetPanelVisibilityHandler(config);
  createKeyedSetHoverHandler(config, hoverMapper);
  createKeyedClearHoverHandler(config);
  createKeyedSetSelectionHandler(config);
  createKeyedClearSelectionHandler(config);
  createKeyedSetWindowLayoutHandler(config);
  createKeyedSetCameraHandler(config);
  createKeyedSetActiveToolHandler(config);
  createKeyedSetFullscreenWindowHandler(config);
}

/**
 * Configuration for single-key event handlers with namespace, app key, key field, and default state.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#App Event Handler Factories§SingleKeyAppEventHandlerConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/SINGLEKEYAPPEVENTHANDLERCONFIG)
 **/
export interface SingleKeyAppEventHandlerConfig<TAppKey extends string, TAppState> {
  namespace: string;
  appKey: TAppKey;
  keyField: string;
  createDefaultState: () => TAppState;
}

/**
 * Registers a single-key init event handler.
 *
 * MUST register a handler that initializes state for the event's key field value.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSingleKeyInitHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESINGLEKEYINITHANDLER)
 **/
export function createSingleKeyInitHandler<TAppKey extends string, TAppState>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.INIT`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      return { [appKey]: { ...context[appKey], [key]: event.state } };
    },
  });
}

/**
 * Registers a single-key sync event handler.
 *
 * MUST register a handler that merges state for the event's key field value.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSingleKeySyncHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESINGLEKEYSYNCHANDLER)
 **/
export function createSingleKeySyncHandler<TAppKey extends string, TAppState>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SYNC`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, ...event.state } } };
    },
  });
}

/**
 * Registers a single-key toggle panel event handler.
 *
 * MUST register a handler that toggles the panel for the event's key field value.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSingleKeyTogglePanelHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESINGLEKEYTOGGLEPANELHANDLER)
 **/
export function createSingleKeyTogglePanelHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.TOGGLE_PANEL`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, panelVisibility: { ...app.panelVisibility, [event.panel]: !app.panelVisibility[event.panel] } } } };
    },
  });
}

/**
 * Registers a single-key set panel visibility event handler.
 *
 * MUST register a handler that replaces panelVisibility for the event's key field value.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSingleKeySetPanelVisibilityHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESINGLEKEYSETPANELVISIBILITYHANDLER)
 **/
export function createSingleKeySetPanelVisibilityHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SET_PANEL_VISIBILITY`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, panelVisibility: event.panelVisibility } } };
    },
  });
}

/**
 * Registers a single-key set hover event handler with a mapper.
 *
 * MUST register a handler that sets hover for the event's key field value.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSingleKeySetHoverHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESINGLEKEYSETHOVERHANDLER)
 **/
export function createSingleKeySetHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>, hoverMapper: (event: any) => any): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SET_HOVER`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, hover: hoverMapper(event) } } };
    },
  });
}

/**
 * Registers a single-key clear hover event handler.
 *
 * MUST register a handler that clears hover for the event's key field value.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSingleKeyClearHoverHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESINGLEKEYCLEARHOVERHANDLER)
 **/
export function createSingleKeyClearHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.CLEAR_HOVER`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, hover: undefined } } };
    },
  });
}

/**
 * Registers a single-key set selection event handler.
 *
 * MUST register a handler that sets selection for the event's key field value.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSingleKeySetSelectionHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESINGLEKEYSETSELECTIONHANDLER)
 **/
export function createSingleKeySetSelectionHandler<TAppKey extends string, TAppState extends { selection?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SET_SELECTION`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, selection: event.selection } } };
    },
  });
}

/**
 * Registers a single-key clear selection event handler.
 *
 * MUST register a handler that clears selection for the event's key field value.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSingleKeyClearSelectionHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESINGLEKEYCLEARSELECTIONHANDLER)
 **/
export function createSingleKeyClearSelectionHandler<TAppKey extends string, TAppState extends { selection?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.CLEAR_SELECTION`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, selection: undefined } } };
    },
  });
}

/**
 * Registers a single-key set window layout event handler.
 *
 * MUST register a handler that sets windowLayout for the event's key field value.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSingleKeySetWindowLayoutHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESINGLEKEYSETWINDOWLAYOUTHANDLER)
 **/
export function createSingleKeySetWindowLayoutHandler<TAppKey extends string, TAppState extends { windowLayout?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SET_WINDOW_LAYOUT`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, windowLayout: event.windowLayout } } };
    },
  });
}

/**
 * Registers a single-key set fullscreen window event handler.
 *
 * MUST register a handler that sets fullscreenWindow for the event's key field value.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#App Event Handler Factories§createSingleKeySetFullscreenWindowHandler](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/CREATESINGLEKEYSETFULLSCREENWINDOWHANDLER)
 **/
export function createSingleKeySetFullscreenWindowHandler<TAppKey extends string, TAppState extends { fullscreenWindow?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SET_FULLSCREEN`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, fullscreenWindow: event.window } } };
    },
  });
}

/**
 * Registers all standard event handlers for a single-key app.
 *
 * MUST register init, sync, and all standard single-key handlers.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Event Handler Factories§registerSingleKeyAppEventHandlers](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-EVENT-HANDLER-FACTORIES/REGISTERSINGLEKEYAPPEVENTHANDLERS)
 **/
export function registerSingleKeyAppEventHandlers<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility; hover?: any; selection?: any; windowLayout?: any; fullscreenWindow?: any }>(
  config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>,
  hoverMapper: (event: any) => any = (e) => e.hover,
): void {
  createSingleKeyInitHandler(config);
  createSingleKeySyncHandler(config);
  createSingleKeyTogglePanelHandler(config);
  createSingleKeySetPanelVisibilityHandler(config);
  createSingleKeySetHoverHandler(config, hoverMapper);
  createSingleKeyClearHoverHandler(config);
  createSingleKeySetSelectionHandler(config);
  createSingleKeyClearSelectionHandler(config);
  createSingleKeySetWindowLayoutHandler(config);
  createSingleKeySetFullscreenWindowHandler(config);
}

// #endregion 🔖App Event Handler Factories

// #region 🔖Transaction Handler Factory

// [🔖semio/js/sketchpad/shared.ts#Transaction Handler Factory](semiorepo://section/semio/js/sketchpad/shared.ts/TRANSACTION-HANDLER-FACTORY)
// MUST provide factory functions for creating undo/redo transaction event handlers.

/**
 * Configuration for keyed transaction handlers with namespace, app key, key fields, and default state.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Transaction Handler Factory§KeyedTransactionHandlerConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/TRANSACTION-HANDLER-FACTORY/KEYEDTRANSACTIONHANDLERCONFIG)
 **/
export interface KeyedTransactionHandlerConfig {
  namespace: string;
  appKey: string;
  keyFields: [string, string];
  createDefaultState: () => { transaction: AppTransactionState };
}

/**
 * Transaction state with active flag, current stack, past stack, and redo stack.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Transaction Handler Factory§AppTransactionState](semiorepo://definition/semio/js/sketchpad/shared.ts/TRANSACTION-HANDLER-FACTORY/APPTRANSACTIONSTATE)
 **/
export interface AppTransactionState<TEdit = any> {
  isTransactionActive: boolean;
  currentTransactionStack: TEdit[];
  pastTransactionStack: TEdit[];
  redoStack: TEdit[];
}

/**
 * Registers all transaction event handlers for keyed app state.
 *
 * MUST register start, commit, abort, undo, redo, and record edit handlers for keyed state.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Transaction Handler Factory§createKeyedTransactionHandlers](semiorepo://definition/semio/js/sketchpad/shared.ts/TRANSACTION-HANDLER-FACTORY/CREATEKEYEDTRANSACTIONHANDLERS)
 **/
export function createKeyedTransactionHandlers(config: KeyedTransactionHandlerConfig): void {
  const { namespace, appKey, keyFields, createDefaultState } = config;
  const [keyField1, keyField2] = keyFields;

  registerEventHandler(`${namespace}.TRANSACTION.START`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key] || createDefaultState();
      const tx = app.transaction;
      if (tx.isTransactionActive) {
        const pastStack = [...tx.pastTransactionStack];
        if (tx.currentTransactionStack.length > 0) {
          const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
          pastStack.push(merged);
        }
        return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { isTransactionActive: true, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
      }
      return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...tx, isTransactionActive: true, currentTransactionStack: [], redoStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.TRANSACTION.COMMIT`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key];
      if (!app || !app.transaction.isTransactionActive) return {};
      const tx = app.transaction;
      const pastStack = [...tx.pastTransactionStack];
      if (tx.currentTransactionStack.length > 0) {
        const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
        pastStack.push(merged);
      }
      return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { isTransactionActive: false, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.TRANSACTION.ABORT`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key];
      if (!app || !app.transaction.isTransactionActive) return {};
      return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...app.transaction, isTransactionActive: false, currentTransactionStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.TRANSACTION.UNDO`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key];
      if (!app) return {};
      const tx = app.transaction;
      if (tx.isTransactionActive && tx.currentTransactionStack.length > 0) {
        const currentStack = [...tx.currentTransactionStack];
        currentStack.pop();
        return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...tx, currentTransactionStack: currentStack } } } };
      } else if (!tx.isTransactionActive && tx.pastTransactionStack.length > 0) {
        const pastStack = [...tx.pastTransactionStack];
        const edit = pastStack.pop()!;
        const redoStack = [...tx.redoStack, edit];
        return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
      }
      return {};
    },
  });

  registerEventHandler(`${namespace}.TRANSACTION.REDO`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key];
      if (!app || app.transaction.isTransactionActive || app.transaction.redoStack.length === 0) return {};
      const tx = app.transaction;
      const redoStack = [...tx.redoStack];
      const edit = redoStack.pop()!;
      const pastStack = [...tx.pastTransactionStack, edit];
      return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
    },
  });

  registerEventHandler(`${namespace}.TRANSACTION.RECORD_EDIT`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key];
      if (!app || !app.transaction.isTransactionActive) return {};
      const currentStack = [...app.transaction.currentTransactionStack, event.edit];
      return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...app.transaction, currentTransactionStack: currentStack, redoStack: [] } } } };
    },
  });
}

/**
 * Configuration for single-key transaction handlers with namespace, app key, key field, and default state.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#Transaction Handler Factory§SingleKeyTransactionHandlerConfig](semiorepo://definition/semio/js/sketchpad/shared.ts/TRANSACTION-HANDLER-FACTORY/SINGLEKEYTRANSACTIONHANDLERCONFIG)
 **/
export interface SingleKeyTransactionHandlerConfig {
  namespace: string;
  appKey: string;
  keyField: string;
  createDefaultState: () => { transaction: AppTransactionState };
}

/**
 * Registers all transaction event handlers for single-key app state.
 *
 * MUST register start, commit, abort, undo, redo, and record edit handlers for single-key state.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Transaction Handler Factory§createSingleKeyTransactionHandlers](semiorepo://definition/semio/js/sketchpad/shared.ts/TRANSACTION-HANDLER-FACTORY/CREATESINGLEKEYTRANSACTIONHANDLERS)
 **/
export function createSingleKeyTransactionHandlers(config: SingleKeyTransactionHandlerConfig): void {
  const { namespace, appKey, keyField, createDefaultState } = config;

  registerEventHandler(`${namespace}.TRANSACTION.START`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      const tx = app.transaction;
      if (tx.isTransactionActive) {
        const pastStack = [...tx.pastTransactionStack];
        if (tx.currentTransactionStack.length > 0) {
          const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
          pastStack.push(merged);
        }
        return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { isTransactionActive: true, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
      }
      return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...tx, isTransactionActive: true, currentTransactionStack: [], redoStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.TRANSACTION.COMMIT`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key];
      if (!app || !app.transaction.isTransactionActive) return {};
      const tx = app.transaction;
      const pastStack = [...tx.pastTransactionStack];
      if (tx.currentTransactionStack.length > 0) {
        const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
        pastStack.push(merged);
      }
      return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { isTransactionActive: false, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.TRANSACTION.ABORT`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key];
      if (!app || !app.transaction.isTransactionActive) return {};
      return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...app.transaction, isTransactionActive: false, currentTransactionStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.TRANSACTION.UNDO`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key];
      if (!app) return {};
      const tx = app.transaction;
      if (tx.isTransactionActive && tx.currentTransactionStack.length > 0) {
        const currentStack = [...tx.currentTransactionStack];
        currentStack.pop();
        return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...tx, currentTransactionStack: currentStack } } } };
      } else if (!tx.isTransactionActive && tx.pastTransactionStack.length > 0) {
        const pastStack = [...tx.pastTransactionStack];
        const edit = pastStack.pop()!;
        const redoStack = [...tx.redoStack, edit];
        return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
      }
      return {};
    },
  });

  registerEventHandler(`${namespace}.TRANSACTION.REDO`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key];
      if (!app || app.transaction.isTransactionActive || app.transaction.redoStack.length === 0) return {};
      const tx = app.transaction;
      const redoStack = [...tx.redoStack];
      const edit = redoStack.pop()!;
      const pastStack = [...tx.pastTransactionStack, edit];
      return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
    },
  });

  registerEventHandler(`${namespace}.TRANSACTION.RECORD_EDIT`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key];
      if (!app || !app.transaction.isTransactionActive) return {};
      const currentStack = [...app.transaction.currentTransactionStack, event.edit];
      return { [appKey]: { ...context[appKey], [key]: { ...app, transaction: { ...app.transaction, currentTransactionStack: currentStack, redoStack: [] } } } };
    },
  });
}

// #endregion 🔖Transaction Handler Factory

// #region 🔖Selector Factory Pattern

// [🔖semio/js/sketchpad/shared.ts#Selector Factory Pattern](semiorepo://section/semio/js/sketchpad/shared.ts/SELECTOR-FACTORY-PATTERN)
// MUST provide factory functions for creating property selectors with app key scoping.

/**
 * Creates a factory for selectors that read a property from a non-keyed app state.
 *
 * MUST return a factory that creates selectors reading from the given app key.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Selector Factory Pattern§createAppPropertySelectorFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/SELECTOR-FACTORY-PATTERN/CREATEAPPPROPERTYSELECTORFACTORY)
 **/
export function createAppPropertySelectorFactory<TApps extends Record<string, any>>(appKey: string) {
  return function createPropertySelector<TProperty>(propertyKey: keyof TApps[string], fallback: TProperty) {
    return (snapshot: { context: Record<string, TApps> }) => {
      const app = snapshot.context[appKey];
      return (app?.[propertyKey] ?? fallback) as TProperty;
    };
  };
}

/**
 * Creates a factory for selectors that read a property from a keyed app state.
 *
 * MUST return a factory that creates keyed selectors reading from the given app key.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Selector Factory Pattern§createKeyedAppPropertySelectorFactory](semiorepo://definition/semio/js/sketchpad/shared.ts/SELECTOR-FACTORY-PATTERN/CREATEKEYEDAPPPROPERTYSELECTORFACTORY)
 **/
export function createKeyedAppPropertySelectorFactory<TAppState>(appKey: string) {
  return function createPropertySelector<TProperty>(propertyKey: keyof TAppState, fallback: TProperty) {
    return (key: string) => (snapshot: { context: Record<string, Record<string, TAppState>> }) => {
      const apps = snapshot.context[appKey] || {};
      const app = apps[key];
      return (app?.[propertyKey] ?? fallback) as TProperty;
    };
  };
}

/**
 * Joins scope strings into a colon-separated app key.
 *
 * MUST join all scope strings with colon separators.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#Selector Factory Pattern§getAppKey](semiorepo://definition/semio/js/sketchpad/shared.ts/SELECTOR-FACTORY-PATTERN/GETAPPKEY)
 **/
export function getAppKey(...scopes: string[]): string {
  return scopes.join(":");
}

/**
 * Retrieves existing app state or creates it from a default factory.
 *
 * MUST return existing state or call the default factory to create it.
 *
 *  * [🪨semio/js/sketchpad/shared.ts#Selector Factory Pattern§getOrCreateAppState](semiorepo://definition/semio/js/sketchpad/shared.ts/SELECTOR-FACTORY-PATTERN/GETORCREATEAPPSTATE)
 **/
export function getOrCreateAppState<TState>(context: Record<string, Record<string, TState>>, appKey: string, key: string, defaultFactory: () => TState): TState {
  const apps = context[appKey] || {};
  return apps[key] || defaultFactory();
}

// #endregion 🔖Selector Factory Pattern

// #region 🔖App Hooks Registry

// [🔖semio/js/sketchpad/shared.ts#App Hooks Registry](semiorepo://section/semio/js/sketchpad/shared.ts/APP-HOOKS-REGISTRY)
// MUST manage registration and retrieval of design and kit app hook implementations.

/**
 * Interface for design app hook functions including commands, diff, hover, and selection.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#App Hooks Registry§DesignAppHooks](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-HOOKS-REGISTRY/DESIGNAPPHOOKS)
 **/
export interface DesignAppHooks {
  useDesignAppCommands: (id?: { kit: string; design: string }) => any;
  useDesignAppDiff: () => any;
  useDesignAppHover: () => any;
  useDesignAppIsPieceHovered: (id?: DesignAppId, pieceId?: string) => boolean;
  useDesignAppIsPieceTransitiveHovered: (id?: DesignAppId, pieceId?: string) => boolean;
  useDesignAppIsConnectionHovered: (id?: DesignAppId, connectionId?: string) => boolean;
  useDesignAppSelection: () => any;
  useDesignAppIsPieceSelected: (id?: DesignAppId, pieceId?: string) => boolean;
  useDesignAppIsConnectionSelected: (id?: DesignAppId, connectionId?: string) => boolean;
  useDesignAppStore: <T>(selector?: (store: any) => T, id?: DesignAppId) => T | null;
}

/**
 * Interface for kit app hook functions including commands.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#App Hooks Registry§KitAppHooks](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-HOOKS-REGISTRY/KITAPPHOOKS)
 **/
export interface KitAppHooks {
  useKitAppCommands: (id?: { kit: string }) => any;
}

const defaultDesignAppHooks: DesignAppHooks = {
  useDesignAppCommands: () => ({ togglePanel: () => { }, execute: () => Promise.resolve({}) }),
  useDesignAppDiff: () => ({}),
  useDesignAppHover: () => undefined,
  useDesignAppIsPieceHovered: () => false,
  useDesignAppIsPieceTransitiveHovered: () => false,
  useDesignAppIsConnectionHovered: () => false,
  useDesignAppSelection: () => ({}),
  useDesignAppIsPieceSelected: () => false,
  useDesignAppIsConnectionSelected: () => false,
  useDesignAppStore: () => null,
};

const defaultKitAppHooks: KitAppHooks = {
  useKitAppCommands: () => ({ togglePanel: () => { }, execute: () => Promise.resolve({}) }),
};

let registeredDesignAppHooks: DesignAppHooks | null = null;
let registeredKitAppHooks: KitAppHooks | null = null;

/**
 * Registers design app hook implementations.
 *
 * MUST store the provided hooks, replacing any previously registered.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Hooks Registry§registerDesignAppHooks](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-HOOKS-REGISTRY/REGISTERDESIGNAPPHOOKS)
 **/
export function registerDesignAppHooks(hooks: DesignAppHooks): void {
  registeredDesignAppHooks = hooks;
}

/**
 * Registers kit app hook implementations.
 *
 * MUST store the provided hooks, replacing any previously registered.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Hooks Registry§registerKitAppHooks](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-HOOKS-REGISTRY/REGISTERKITAPPHOOKS)
 **/
export function registerKitAppHooks(hooks: KitAppHooks): void {
  registeredKitAppHooks = hooks;
}

/**
 * Returns registered design app hooks or defaults.
 *
 * MUST fall back to default no-op hooks when none are registered.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Hooks Registry§getDesignAppHooks](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-HOOKS-REGISTRY/GETDESIGNAPPHOOKS)
 **/
export function getDesignAppHooks(): DesignAppHooks {
  return registeredDesignAppHooks ?? defaultDesignAppHooks;
}

/**
 * Returns registered kit app hooks or defaults.
 *
 * MUST fall back to default no-op hooks when none are registered.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Hooks Registry§getKitAppHooks](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-HOOKS-REGISTRY/GETKITAPPHOOKS)
 **/
export function getKitAppHooks(): KitAppHooks {
  return registeredKitAppHooks ?? defaultKitAppHooks;
}

// #endregion 🔖App Hooks Registry

// #region 🔖App Registry Exports

// [🔖semio/js/sketchpad/shared.ts#App Registry Exports](semiorepo://section/semio/js/sketchpad/shared.ts/APP-REGISTRY-EXPORTS)
// MUST provide docs registry port interface and registration for documentation section access.

/**
 * Port interface for retrieving documentation section trees and pages.
 *
 *  * [✂️semio/js/sketchpad/shared.ts#App Registry Exports§DocsRegistryPort](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-REGISTRY-EXPORTS/DOCSREGISTRYPORT)
 **/
export interface DocsRegistryPort {
  getSectionTree: (section: string) => any[];
  getAllPages: () => any[];
  getPage?: (path: string) => any;
}

let registeredDocsRegistry: DocsRegistryPort | null = null;

/**
 * Registers a docs registry implementation.
 *
 * MUST store the given docs registry, replacing any previous one.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Registry Exports§registerDocsRegistry](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-REGISTRY-EXPORTS/REGISTERDOCSREGISTRY)
 **/
export function registerDocsRegistry(registry: DocsRegistryPort): void {
  registeredDocsRegistry = registry;
}

/**
 * Returns the registered docs registry or null.
 *
 * MUST return the registered docs registry or null when none is registered.
 *
 *  * [🛠️semio/js/sketchpad/shared.ts#App Registry Exports§getDocsRegistry](semiorepo://definition/semio/js/sketchpad/shared.ts/APP-REGISTRY-EXPORTS/GETDOCSREGISTRY)
 **/
export function getDocsRegistry(): DocsRegistryPort | null {
  return registeredDocsRegistry;
}

// #endregion 🔖App Registry Exports
