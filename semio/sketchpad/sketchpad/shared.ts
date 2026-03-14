// #region Header

// js/semio/sketchpad/shared.ts

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

// #endregion Header

// #region Imports

import { CodeIcon, DetailsIcon, SettingsIcon, StatsIcon, ToolbarIcon, ToolsIcon, WorkbenchIcon } from "@semio/assets";
import { ComponentType, ReactNode } from "react";
import { AnyActorRef, assign, fromCallback } from "xstate";
import * as Y from "yjs";
import { Guid, Kit, KitDiff } from "@semio/js/semio";

// #endregion Imports

// #region Types

// #region YPath Types

// [👤semio📚js🗃️sketchpad💻sharedts🔖types](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/TYPES)
// MUST define path segment and path types for navigating Y.js document structures.

/**
 * A single segment in a Y.js document path, either a map key, array index, or array item by ID.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖ypathtypes🛠️ypathsegment](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/YPath%20Types/d/i/YPathSegment)
 **/
export type YPathSegment = { kind: "mapKey"; key: string } | { kind: "arrayIndex"; index: number } | { kind: "arrayItemById"; id: string; idKey: string };

/**
 * An ordered sequence of YPathSegment values describing a path through a Y.js document.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖ypathtypes🛠️ypath](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/YPath%20Types/d/i/YPath)
 **/
export type YPath = YPathSegment[];

// #endregion YPath Types

// #region Granular Hook Types

// [👤semio📚js🗃️sketchpad💻sharedts🔖types🔖granularhooktypes](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/TYPES/GRANULAR-HOOK-TYPES)
// MUST define hook result tuples and field abstractions for granular reactive state access.

/**
 * A readonly tuple of value, optional setter, and canSet flag for granular hook access.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️hookresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/HookResult)
 **/
export type HookResult<T> = readonly [T, ((value: T) => void) | undefined, boolean];

/**
 * A readonly tuple of value, undefined setter, and canSet flag for read-only hook access.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️hooknosetresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/HookNoSetResult)
 **/
export type HookNoSetResult<T> = readonly [T, undefined, boolean];

/**
 * Sentinel undefined value indicating that a hook result has no setter.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🪨readonlysetter](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/READONLY_SETTER)
 **/
export const READONLY_SETTER = undefined as undefined;
/**
 * Sentinel false value indicating that a hook result is read-only.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🪨readonlycan](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/READONLY_CAN)
 **/
export const READONLY_CAN = false;

/**
 * Wraps a value into a read-only HookResult tuple with no setter.
 *
 * MUST return a frozen readonly tuple with undefined setter and false canSet.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️readonlyhookresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/readonlyHookResult)
 **/
export function readonlyHookResult<T>(value: T): HookResult<T> {
  return [value, READONLY_SETTER, READONLY_CAN] as const;
}

/**
 * Wraps a value and setter into a writable HookResult tuple.
 *
 * MUST return a tuple with the setter included only when canSet is true.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️writablehookresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/writableHookResult)
 **/
export function writableHookResult<T>(value: T, setter: (value: T) => void, canSet: boolean = true): HookResult<T> {
  return [value, canSet ? setter : undefined, canSet] as const;
}

/**
 * Wraps a value into a HookResult tuple with a setter conditional on canSet.
 *
 * MUST return a tuple with the setter conditional on the canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️conditionalhookresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/conditionalHookResult)
 **/
export function conditionalHookResult<T>(canSet: boolean, value: T, setter: ((value: T) => void) | undefined): HookResult<T> {
  return [value, canSet ? setter : undefined, canSet] as const;
}

/**
 * A reactive field with a value, canSet flag, and setter function.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️field](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/Field)
 **/
export interface Field<T> {
  value: T;
  canSet: boolean;
  set: (next: T) => void;
}

/**
 * A reactive action field with canExecute flag and execute function.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️actionfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/ActionField)
 **/
export interface ActionField {
  canExecute: boolean;
  execute: () => void;
}

// [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🪨noopsetter](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/NOOP_SETTER)
/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🪨noopsetter](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/NOOP_SETTER)
 * NOOP_SETTER holds the data fields for a NOOP_SETTER record.
 **/
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️createfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/createField)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️createreadonlyfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/createReadonlyField)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️createaction](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/createAction)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️fieldtohookresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/fieldToHookResult)
 **/
export function fieldToHookResult<T>(field: Field<T>): HookResult<T> {
  return [field.value, field.canSet ? field.set : undefined, field.canSet] as const;
}

/**
 * Converts a HookResult tuple back to a Field.
 *
 * MUST reconstruct a Field from the tuple, using a no-op setter when undefined.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🛠️hookresulttofield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/d/i/hookResultToField)
 **/
export function hookResultToField<T>(result: HookResult<T>): Field<T> {
  const [value, setter, canSet] = result;
  return {
    value,
    canSet,
    set: setter ?? NOOP_SETTER,
  };
}

// #endregion Granular Hook Types

// #region Standard Empty Constants

// [👤semio📚js🗃️sketchpad💻sharedts🔖types🔖standardemptyconstants](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/TYPES/STANDARD-EMPTY-CONSTANTS)
// MUST provide frozen singleton constants for empty collections and default panel visibility.

/**
 * Frozen empty array singleton for default array values.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🪨emptyarray](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/EMPTY_ARRAY)
 **/
export const EMPTY_ARRAY: readonly any[] = Object.freeze([]);
/**
 * Frozen empty object singleton for default record values.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🪨emptyobject](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/EMPTY_OBJECT)
 **/
export const EMPTY_OBJECT: Readonly<Record<string, never>> = Object.freeze({});
/**
 * Frozen empty Guid array singleton for default guid collections.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🪨emptyguidarray](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/EMPTY_GUID_ARRAY)
 **/
export const EMPTY_GUID_ARRAY: readonly Guid[] = Object.freeze([]);
/**
 * Frozen empty string array singleton for default string collections.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🪨emptystringarray](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/EMPTY_STRING_ARRAY)
 **/
export const EMPTY_STRING_ARRAY: readonly string[] = Object.freeze([]);

/**
 * Frozen default panel visibility with only toolbar visible.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🪨emptypanelvisibility](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/EMPTY_PANEL_VISIBILITY)
 **/
export const EMPTY_PANEL_VISIBILITY: Readonly<PanelVisibility> = Object.freeze({
  toolbar: true,
  leftSidePanel: false,
  rightSidePanel: true,
  details: true,
  chat: false,
  settings: false,
});

// #endregion Standard Empty Constants

// #region Generic Diff Types

// [👤semio📚js🗃️sketchpad💻sharedts🔖types🔖genericdifftypes](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/TYPES/GENERIC-DIFF-TYPES)
// MUST define generic array and selection diff types with apply and inverse operations.

/**
 * Describes added and removed items for an array diff operation.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖genericdifftypes🛠️arraydiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Generic%20Diff%20Types/d/i/ArrayDiff)
 **/
export interface ArrayDiff<T> {
  added?: T[];
  removed?: T[];
}

/**
 * Maps selection keys to their corresponding array diffs.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖genericdifftypes🛠️selectiondiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Generic%20Diff%20Types/d/i/SelectionDiff)
 **/
export type SelectionDiff<TSelection extends Record<string, any[]>> = {
  [K in keyof TSelection]?: ArrayDiff<TSelection[K][number]>;
};

/**
 * Inverts an array diff by swapping added and removed items.
 *
 * MUST swap added and removed arrays to produce the inverse diff.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖genericdifftypes🛠️inversearraydiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Generic%20Diff%20Types/d/i/inverseArrayDiff)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖genericdifftypes🛠️inverseselectiondiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Generic%20Diff%20Types/d/i/inverseSelectionDiff)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖genericdifftypes🛠️applyarraydiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Generic%20Diff%20Types/d/i/applyArrayDiff)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖genericdifftypes🛠️applyselectiondiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Generic%20Diff%20Types/d/i/applySelectionDiff)
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

// #endregion Generic Diff Types

/**
 * A string alias representing a URL.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️url](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/Url)
 **/
export type Url = string;

/**
 * A callback subscription function that returns an unsubscribe disposer.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️subscribe](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/Subscribe)
 **/
export type Subscribe = (callback: () => void) => () => void;

/**
 * A cleanup function that disposes of a resource.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️disposable](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/Disposable)
 **/
export type Disposable = () => void;

/**
 * A function that executes a mutation within a transaction with optional origin.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️transact](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/Transact)
 **/
export type Transact = (fn: () => void, origin?: string) => void;

/**
 * A function that unsubscribes a previously registered callback.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️unsubscribe](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/Unsubscribe)
 **/
export type Unsubscribe = () => void;

/**
 * A factory function that creates a Y.js document provider for a given ID.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️yproviderfactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/YProviderFactory)
 **/
export type YProviderFactory = (doc: Y.Doc, id: string) => Promise<void>;

/**
 * A string alias identifying the kind of an app.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️appkind](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/AppKind)
 **/
export type AppKind = string;

/**
 * Union type for desktop, tablet, or mobile device contexts.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️device](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/Device)
 **/
export type Device = "desktop" | "tablet" | MobileDevice;

/**
 * Union of all panel identifier strings including side panels.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️panelkey](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/PanelKey)
 **/
export type PanelKey = "workbench" | "details" | "tools" | "stats" | "console" | "toolbar" | "leftSidePanel" | "rightSidePanel";

/**
 * Union of left and right side panel keys.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️sidepanelkey](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/SidePanelKey)
 **/
export type SidePanelKey = "leftSidePanel" | "rightSidePanel";

/**
 * A string alias for a hotkey path identifier.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️hotkeypath](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/HotkeyPath)
 **/
export type HotkeyPath = string;

/**
 * A string alias for a hotkey binding value.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️hotkeyvalue](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/HotkeyValue)
 **/
export type HotkeyValue = string;

/**
 * A record mapping hotkey paths to their override values.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️hotkeyoverrides](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/HotkeyOverrides)
 **/
export type HotkeyOverrides = Record<HotkeyPath, HotkeyValue>;

/**
 * A factory function that creates a FileProvider for a given kit ID.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️fileproviderfactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/FileProviderFactory)
 **/
export type FileProviderFactory = (kitId: string) => Promise<FileProvider>;

/**
 * A string alias for a Y.js-compatible UUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️yuuid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/YUuid)
 **/
export type YUuid = string;

/**
 * A Y.js array of UUID strings.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️yuuidarray](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/YUuidArray)
 **/
export type YUuidArray = Y.Array<YUuid>;

/**
 * A string alias for a Y.js concept name.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️yconcept](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/YConcept)
 **/
export type YConcept = string;

/**
 * A Y.js array of concept name strings.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️yconcepts](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/YConcepts)
 **/
export type YConcepts = Y.Array<string>;

/**
 * A Y.js array of strings.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️ystringarray](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/YStringArray)
 **/
export type YStringArray = Y.Array<string>;

/**
 * A Y.js map with string leaf values.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️yleafmapstring](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/YLeafMapString)
 **/
export type YLeafMapString = Y.Map<string>;

/**
 * A Y.js map with number leaf values.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️yleafmapnumber](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/YLeafMapNumber)
 **/
export type YLeafMapNumber = Y.Map<number>;

/**
 * A Y.js array of Y.js maps representing attribute key-value pairs.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🛠️yattributes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/d/i/YAttributes)
 **/
export type YAttributes = Y.Array<Y.Map<string>>;

// #endregion Types

// #region Enums

// [👤semio📚js🗃️sketchpad💻sharedts🔖enums](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/ENUMS)
// MUST enumerate theme, expertise, mode, store status, tool, window, and panel kinds.

/**
 * Available UI theme options: system, light, or dark.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🛠️theme](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/d/i/Theme)
 **/
export enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

/**
 * User expertise levels: beginner, normal, or expert.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🛠️expertise](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/d/i/Expertise)
 **/
export enum Expertise {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}

/**
 * Application modes: user or dev.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🛠️mode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/d/i/Mode)
 **/
export enum Mode {
  USER = "user",
  DEV = "dev",
}

/**
 * Store lifecycle states: idle, loading, error, or ready.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🛠️storestatus](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/d/i/StoreStatus)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🛠️toolkind](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/d/i/ToolKind)
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

export type SelectionCompositionKind = "replace" | "additive" | "subtractive" | "intersect";

export interface SelectionKeyboardState {
  shiftKey?: boolean;
  altKey?: boolean;
  ctrlKey?: boolean;
  metaKey?: boolean;
}

export function isSelectionToolKind(toolKind: ToolKind | string): boolean {
  return (
    toolKind === ToolKind.SELECTION_NORMAL ||
    toolKind === ToolKind.SELECTION_ADDITIVE ||
    toolKind === ToolKind.SELECTION_SUBTRACTIVE ||
    toolKind === ToolKind.SELECTION_INTERSECT
  );
}

export function resolveSelectionCompositionKindFromTool(toolKind: ToolKind | string): SelectionCompositionKind {
  if (toolKind === ToolKind.SELECTION_ADDITIVE) return "additive";
  if (toolKind === ToolKind.SELECTION_SUBTRACTIVE) return "subtractive";
  if (toolKind === ToolKind.SELECTION_INTERSECT) return "intersect";
  return "replace";
}

export function resolveSelectionCompositionKind(toolKind: ToolKind | string, keyboard?: SelectionKeyboardState): SelectionCompositionKind {
  const hasAdditiveModifier = keyboard?.shiftKey === true;
  const hasSubtractiveModifier = keyboard?.altKey === true || keyboard?.ctrlKey === true || keyboard?.metaKey === true;
  if (hasAdditiveModifier && hasSubtractiveModifier) return "intersect";
  if (hasAdditiveModifier) return "additive";
  if (hasSubtractiveModifier) return "subtractive";
  return resolveSelectionCompositionKindFromTool(toolKind);
}

export function toSelectionToolKind(compositionKind: SelectionCompositionKind): ToolKind {
  if (compositionKind === "additive") return ToolKind.SELECTION_ADDITIVE;
  if (compositionKind === "subtractive") return ToolKind.SELECTION_SUBTRACTIVE;
  if (compositionKind === "intersect") return ToolKind.SELECTION_INTERSECT;
  return ToolKind.SELECTION_NORMAL;
}

export function applySelectionComposition<T>(previous: T[] | undefined, incoming: T[] | undefined, compositionKind: SelectionCompositionKind): T[] {
  const uniquePrevious = Array.from(new Set(previous ?? []));
  const uniqueIncoming = Array.from(new Set(incoming ?? []));
  if (compositionKind === "replace") return uniqueIncoming;
  if (compositionKind === "additive") {
    const previousSet = new Set(uniquePrevious);
    return [...uniquePrevious, ...uniqueIncoming.filter((value) => !previousSet.has(value))];
  }
  if (compositionKind === "subtractive") {
    const incomingSet = new Set(uniqueIncoming);
    return uniquePrevious.filter((value) => !incomingSet.has(value));
  }
  const incomingSet = new Set(uniqueIncoming);
  return uniquePrevious.filter((value) => incomingSet.has(value));
}

export enum WindowKind {
  TABLE = "table",
  SCENE = "scene",
  DIAGRAM = "diagram",
  CUSTOM = "custom",
  SETTINGS = "settings",
  CHAT = "chat",
  WORKBENCH = "workbench",
}

/**
 * Panel layout positions: left, right, middle, or bottom.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🛠️panelposition](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/d/i/PanelPosition)
 **/
export enum PanelPosition {
  LEFT = "left",
  RIGHT = "right",
  MIDDLE = "middle",
  BOTTOM = "bottom",
}

/**
 * Panel kinds: tools, toolbar, stats, details, params, or console.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🛠️panelkind](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/d/i/PanelKind)
 **/
export enum PanelKind {
  WORKBENCH = "workbench",
  TOOLS = "tools",
  TOOLBAR = "toolbar",
  STATS = "stats",
  DETAILS = "details",
  PARAMS = "params",
  CONSOLE = "console",
}

// #endregion Enums

// #region Ports

// #region File Provider

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖fileprovider](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/FILE-PROVIDER)
// MUST define file storage provider interfaces for upload, download, and delete operations.

/**
 * Interface for file upload, download, delete, and URL retrieval operations.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🛠️fileprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/d/i/FileProvider)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🛠️memoryfileproviderconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/d/i/MemoryFileProviderConfig)
 **/
export interface MemoryFileProviderConfig { }

/**
 * Configuration interface for local IndexedDB file provider.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🛠️localfileproviderconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/d/i/LocalFileProviderConfig)
 **/
export interface LocalFileProviderConfig {
  dbName?: string;
  storeName?: string;
}

/**
 * Configuration interface for remote file provider with base URL and headers.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🛠️remotefileproviderconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/d/i/RemoteFileProviderConfig)
 **/
export interface RemoteFileProviderConfig {
  baseUrl: string;
  headers?: Record<string, string>;
}

/**
 * Configuration interface combining memory, local, and remote file providers.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🛠️compositefileproviderconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/d/i/CompositeFileProviderConfig)
 **/
export interface CompositeFileProviderConfig {
  memory?: boolean;
  local?: boolean | LocalFileProviderConfig;
  remote?: RemoteFileProviderConfig;
}

/**
 * Interface for remote Y.js document and file provider factories.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🛠️remoteproviders](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/d/i/RemoteProviders)
 **/
export interface RemoteProviders {
  yProvider: (yDoc: Y.Doc, name: string) => void;
  fileProvider: FileProviderFactory;
}

/**
 * Describes a file operation with type, kit ID, file ID, path, and optional blob.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🛠️fileoperation](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/d/i/FileOperation)
 **/
export interface FileOperation {
  type: "upload" | "download" | "delete";
  kitId: string;
  fileId: string;
  path: string;
  blob?: Blob;
}

// #endregion File Provider

// #region App IDs

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖appids](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/APP-I-DS)
// MUST define identifier interfaces for design, kit, type, and quality app scopes.

/**
 * Identifier for a design app scope with kit and design GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🛠️designappid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/d/i/DesignAppId)
 **/
export interface DesignAppId {
  kit: Guid;
  design: Guid;
}

/**
 * Identifier for a kit app scope with a kit GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🛠️kitappid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/d/i/KitAppId)
 **/
export interface KitAppId {
  kit: Guid;
}

/**
 * Identifier for a type app scope with kit and type GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🛠️typeappid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/d/i/TypeAppId)
 **/
export interface TypeAppId {
  kit: Guid;
  type: Guid;
}

/**
 * Identifier for a quality app scope with kit and quality GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🛠️qualityappid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/d/i/QualityAppId)
 **/
export interface QualityAppId {
  kit: Guid;
  quality: Guid;
}

// #endregion App IDs

// #region Panel

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖panel](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/PANEL)
// MUST define panel kind configurations, visibility, sizing, sections, and definition interfaces.

/**
 * Configuration for a panel kind including icon, position, group, and hotkey.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️panelkindconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/PanelKindConfig)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🪨panelkindconfigs](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/panelKindConfigs)
 **/
export const panelKindConfigs: Record<PanelKind, PanelKindConfig> = {
  [PanelKind.WORKBENCH]: {
    icon: WorkbenchIcon,
    position: PanelPosition.LEFT,
    group: "left",
    isGroupable: true,
  },
  [PanelKind.TOOLS]: {
    icon: ToolsIcon,
    position: PanelPosition.LEFT,
    group: "left",
    isGroupable: true,
    hotkey: "ctrl+j",
  },
  [PanelKind.TOOLBAR]: {
    icon: ToolbarIcon,
    position: PanelPosition.BOTTOM,
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️sidepanelposition](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/SidePanelPosition)
 **/
export enum SidePanelPosition {
  LEFT = "left",
  RIGHT = "right",
}

/**
 * A tab entry for a side panel with ID, icon, order, and content.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️sidepaneltab](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/SidePanelTab)
 **/
export interface SidePanelTab {
  id: string;
  icon: ComponentType<{ size?: number }>;
  order?: number;
  content: ReactNode | (() => ReactNode);
}

/**
 * Visibility flags for left and right side panels.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️sidepanelvisibility](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/SidePanelVisibility)
 **/
export interface SidePanelVisibility {
  left: boolean;
  right: boolean;
}

/**
 * Optional visibility flags for all panel kinds.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️panelvisibility](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/PanelVisibility)
 **/
export interface PanelVisibility {
  toolbar?: boolean;
  leftSidePanel?: boolean;
  rightSidePanel?: boolean;
  tools?: boolean;
  hud?: boolean;
  stats?: boolean;
  details?: boolean;
  params?: boolean;
  console?: boolean;
  chat?: boolean;
  settings?: boolean;
}

/**
 * Numeric sizes for all panel dimensions including widths and heights.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️panelsizes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/PanelSizes)
 **/
export interface PanelSizes {
  toolbarHeight: number;
  toolsWidth: number;
  hudWidth: number;
  statsWidth: number;
  detailsWidth: number;
  consoleHeight: number;
  leftSidePanelWidth: number;
  rightSidePanelWidth: number;
  chatWidth: number;
  settingsWidth: number;
}

/**
 * A collapsible section within a panel with content, actions, and toolbar group.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️panelsection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/PanelSection)
 **/
export interface PanelSection {
  id: string;
  content: ReactNode | (() => ReactNode);
  specificity?: number;
  defaultOpen?: boolean;
  order?: number;
  toolbarGroup?: {
    id: string; // "selection", "filter", "create", "view", "actions"
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
  toolbarPlaceholder?: boolean;
}

/**
 * Left and right arrays of side panel tabs.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️sidepaneltabs](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/SidePanelTabs)
 **/
export interface SidePanelTabs {
  left: SidePanelTab[];
  right: SidePanelTab[];
}

/**
 * Collections of panel sections and tabs organized by panel kind.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️panelsections](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/PanelSections)
 **/
export interface PanelSections {
  workbench: PanelSection[];
  details: PanelSection[];
  tools: PanelSection[];
  hud: PanelSection[];
  stats: PanelSection[];
  console: PanelSection[];
  toolbar: PanelSection[];
  leftSidePanel: SidePanelTab[];
  rightSidePanel: SidePanelTab[];
}

/**
 * Definition of a panel with ID, kind, hotkey, and tooltip.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️paneldefinition](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/PanelDefinition)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️enrichedpaneldefinition](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/EnrichedPanelDefinition)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️createpaneldefinition](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/createPanelDefinition)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️enrichpaneldefinition](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/enrichPanelDefinition)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️panelconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/PanelConfig)
 **/
export interface PanelConfig {
  id: string;
  key: "workbench" | "details" | "tools" | "hud" | "stats" | "toolbar" | "console";
  label: string;
  order?: number;
  defaultOpen?: boolean;
  content: ReactNode | (() => ReactNode);
}

/**
 * Container for an array of panel configurations.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🛠️apppanels](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/d/i/AppPanels)
 **/
export interface AppPanels {
  panels: PanelConfig[];
}

// #endregion Panel

// #region App Registry

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖appregistry](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/APP-REGISTRY)
// MUST define route segment and app configuration interfaces for app registration.

/**
 * A URL route segment with path, optional param name, and scope provider.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🛠️routesegment](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/d/i/RouteSegment)
 **/
export interface RouteSegment {
  path: string;
  paramName?: string;
  scopeProvider?: ComponentType<{ guid: string; children: ReactNode }>;
}

/**
 * Full app configuration with ID, component, routes, panels, and order.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🛠️appconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/d/i/AppConfig)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🛠️appregistration](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/d/i/AppRegistration)
 **/
export interface AppRegistration extends AppConfig { }

// #endregion App Registry

// #region Sketchpad State

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖sketchpadstate](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/SKETCHPAD-STATE)
// MUST define mutable and immutable sketchpad state interfaces with diff types.

/**
 * Mobile device state with navbar and footer expansion flags.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🛠️mobiledevice](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/d/i/MobileDevice)
 **/
export interface MobileDevice {
  isNavbarExpanded: boolean;
  isFooterExpanded: boolean;
}

/**
 * Mutable fields of sketchpad state including navigation, theme, device, and settings.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🛠️sketchpadchangablestate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/d/i/SketchpadChangableState)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🛠️sketchpadstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/d/i/SketchpadState)
 **/
export interface SketchpadState extends SketchpadChangableState {
  id?: string;
  persisted?: boolean;
}

/**
 * Partial diff of sketchpad state fields for incremental updates.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🛠️sketchpaddiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/d/i/SketchpadDiff)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🛠️initialstatekit](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/d/i/InitialStateKit)
 **/
export interface InitialStateKit {
  kit: Kit;
  local?: boolean;
  remote?: boolean;
}

/**
 * Extended initial state combining partial sketchpad state with initial kits.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🛠️extendedinitialstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/d/i/ExtendedInitialState)
 **/
export interface ExtendedInitialState extends Partial<SketchpadState> {
  kits?: InitialStateKit[];
}

/**
 * Callback functions for window minimize, maximize, and close events.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🛠️windowevents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/d/i/WindowEvents)
 **/
export type WindowEvents = {
  minimize: () => void;
  maximize: () => void;
  close: () => void;
};

/**
 * Scoped sketchpad context with ID, optional remote providers, and window events.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🛠️sketchpadscope](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/d/i/SketchpadScope)
 **/
export type SketchpadScope = { id: string; remote?: RemoteProviders; onWindowEvents?: WindowEvents };

// #endregion Sketchpad State

// #region Commands

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖commands](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/COMMANDS)
// MUST define command context and result interfaces for kit and sketchpad operations.

/**
 * Context for kit commands including kit data, file URLs, and origin.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🛠️kitcommandcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/d/i/KitCommandContext)
 **/
export interface KitCommandContext {
  kit: Kit;
  fileUrls: Map<Url, Url>;
  origin?: string;
}

/**
 * Result of a kit command with optional diff, files, and origin.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🛠️kitcommandresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/d/i/KitCommandResult)
 **/
export interface KitCommandResult {
  diff?: KitDiff;
  files?: File[];
  origin?: string;
}

/**
 * Context for sketchpad commands including sketchpad state and origin.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🛠️sketchpadcommandcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/d/i/SketchpadCommandContext)
 **/
export interface SketchpadCommandContext {
  sketchpad: SketchpadState;
  origin?: string;
}

/**
 * Result of a sketchpad command with optional diff and origin.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🛠️sketchpadcommandresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/d/i/SketchpadCommandResult)
 **/
export interface SketchpadCommandResult {
  diff?: SketchpadDiff;
  origin?: string;
}

// #endregion Commands

// #region Store

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖store](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/STORE)
// MUST define store state, app step, edit, diff, and command result interfaces.

/**
 * Interface for objects that support change subscription and snapshot retrieval.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🛠️synchronizable](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/d/i/Synchronizable)
 **/
export interface Synchronizable<TAccessl> {
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onChangedDeep: (subscribe: Subscribe) => Unsubscribe;
  snapshot: () => TAccessl;
}

/**
 * Wrapper for store status, data, and error.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🛠️storestate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/d/i/StoreState)
 **/
export interface StoreState<TState> {
  status: StoreStatus;
  data?: TState;
  error?: Error;
}

/**
 * A single app step with optional selection diff.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🛠️appstep](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/d/i/AppStep)
 **/
export interface AppStep<TSelectionDiff = any> {
  selectionDiff?: TSelectionDiff;
}

/**
 * An undoable edit consisting of do and undo app steps.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🛠️appedit](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/d/i/AppEdit)
 **/
export interface AppEdit<TSelectionDiff = any> {
  do: AppStep<TSelectionDiff>;
  undo: AppStep<TSelectionDiff>;
}

/**
 * A diff containing selection, presence, hover, fullscreen, and panel visibility changes.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🛠️appdiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/d/i/AppDiff)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🛠️appcommandresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/d/i/AppCommandResult)
 **/
export interface AppCommandResult<TDiff = any> {
  diff?: TDiff;
  origin?: string;
}

/**
 * An app step extended with an optional kit diff.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🛠️kitdiffappstep](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/d/i/KitDiffAppStep)
 **/
export interface KitDiffAppStep<TSelectionDiff = any> extends AppStep<TSelectionDiff> {
  kitDiff?: KitDiff;
}

/**
 * An undoable edit with kit diff-aware do and undo steps.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🛠️kitdiffappedit](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/d/i/KitDiffAppEdit)
 **/
export interface KitDiffAppEdit<TSelectionDiff = any> {
  do: KitDiffAppStep<TSelectionDiff>;
  undo: KitDiffAppStep<TSelectionDiff>;
}

/**
 * An app command result extended with an optional kit diff.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🛠️kitdiffappcommandresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/d/i/KitDiffAppCommandResult)
 **/
export interface KitDiffAppCommandResult<TDiff = any> extends AppCommandResult<TDiff> {
  kitDiff?: KitDiff;
}

/**
 * Interface for objects that support change subscription and snapshot retrieval.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🛠️synchronizable](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/d/i/Synchronizable)
 **/
export interface Synchronizable<TAccessl> {
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onChangedDeep: (subscribe: Subscribe) => Unsubscribe;
  snapshot: () => TAccessl;
}

// #endregion Store

// #region Complete State

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖completestate](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/COMPLETE-STATE)
// MUST define the complete aggregated state interface for the entire sketchpad.

/**
 * Full aggregated state containing sketchpad, kits, and all app states.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🛠️completestate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/d/i/CompleteState)
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

// #endregion Complete State

// #region Window

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖window](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/WINDOW)
// MUST define window configuration, control, layout parsing, and default layout creation.

/**
 * Configuration for a window with ID, title, icon, component, and default size.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🛠️windowconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/d/i/WindowConfig)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🛠️windowcontrol](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/d/i/WindowControl)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🛠️windowkinddefinition](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/d/i/WindowKindDefinition)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🛠️appwindowconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/d/i/AppWindowConfig)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🛠️parsewindowlayout](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/d/i/parseWindowLayout)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🛠️deduplicatewindowlayout](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/d/i/deduplicateWindowLayout)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🛠️stringifywindowlayout](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/d/i/stringifyWindowLayout)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🛠️appwindowprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/d/i/AppWindowProps)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🛠️createdefaultlayout](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/d/i/createDefaultLayout)
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

// #endregion Window

// #region Tool

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖tool](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/TOOL)
// MUST define tool interfaces for selection, lasso, connector, and hand interactions.

/**
 * A tool with ID, icon, and render function returning scene, diagram, and table nodes.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🛠️tool](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/d/i/Tool)
 **/
export interface Tool<TState = any> {
  id: ToolKind | string;
  icon?: ReactNode;
  render: (context: ToolRenderContext<TState>) => { scene?: ReactNode; diagram?: ReactNode | null; table?: ReactNode | null };
}

/**
 * A tool mode with ID, icon, label, and tooltip.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🛠️toolmode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/d/i/ToolMode)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🛠️tooldefinition](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/d/i/ToolDefinition)
 **/
export interface ToolDefinition {
  id: string;
  defaultMode: ToolKind | string;
  modes: ToolMode[];
}

/**
 * Context passed to a tool's render function containing the current state.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🛠️toolrendercontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/d/i/ToolRenderContext)
 **/
export interface ToolRenderContext<TState = any> {
  state: TState;
}

/**
 * Props for a tool group component with tools, active tool, and change handler.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🛠️toolgroupprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/d/i/ToolGroupProps)
 **/
export interface ToolGroupProps {
  tools: ToolDefinition[];
  activeTool: ToolKind | string;
  onToolChange: (tool: ToolKind | string) => void;
}

// #endregion Tool

// #region Focus

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖focus](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/FOCUS)
// MUST define the focus item interface for search and navigation targets.

/**
 * A focusable item with ID, label, optional description, and category.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🛠️focusitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/d/i/FocusItem)
 **/
export interface FocusItem {
  id: string;
  label: string;
  description?: string;
  category?: string;
}

// #endregion Focus

// #region Footer

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖footer](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/FOOTER)
// MUST define the footer item interface for status bar entries.

/**
 * A footer status bar item with ID, icon, text, content, and click handler.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🛠️footeritem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/d/i/FooterItem)
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

// #endregion Footer

// #region Panel Props

// [👤semio📚js🗃️sketchpad💻sharedts🔖ports🔖panelprops](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/PORTS/PANEL-PROPS)
// MUST define resizable panel props interface for panel width management.

/**
 * Props for a resizable panel with visibility, width, and width change handler.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖panelprops🛠️resizablepanelprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/Panel%20Props/d/i/ResizablePanelProps)
 **/
export interface ResizablePanelProps {
  visible: boolean;
  onWidthChange?: (width: number) => void;
  width: number;
}

// #endregion Panel Props

// #endregion Ports

// #region XState Integration

// #region XState Types

// [👤semio📚js🗃️sketchpad💻sharedts🔖xstateintegration🔖xstatetypes](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/X-STATE-INTEGRATION/X-STATE-TYPES)
// MUST define XState machine context and event type interfaces for sketchpad, kit, and app machines.

/**
 * Base context for Y.js-synced machines with dirty flag and cache.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🛠️yjssynccontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/d/i/YjsSyncContext)
 **/
export interface YjsSyncContext {
  dirty: boolean;

  cache?: any;
}

/**
 * XState context for the sketchpad machine with navigation, theme, kits, and refs.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🛠️sketchpadmachinecontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/d/i/SketchpadMachineContext)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🛠️sketchpadmachineevent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/d/i/SketchpadMachineEvent)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🛠️kitmachinecontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/d/i/KitMachineContext)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🛠️kitmachineevent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/d/i/KitMachineEvent)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🛠️appmachinecontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/d/i/AppMachineContext)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🛠️appmachineevent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/d/i/AppMachineEvent)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🛠️kitdiffappmachinecontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/d/i/KitDiffAppMachineContext)
 **/
export interface KitDiffAppMachineContext<TSelection = any> extends AppMachineContext<TSelection> {
  kitGuid: Guid;
}

// #endregion XState Types

// #region Y.js-XState Bridge

// [👤semio📚js🗃️sketchpad💻sharedts🔖xstateintegration🔖yjsxstatebridge](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/X-STATE-INTEGRATION/Y-JS-X-STATE-BRIDGE)
// MUST bridge Y.js document observation to XState machine events.

/**
 * Creates an XState callback actor that observes a Y.js map and sends Y_UPDATE events.
 *
 * MUST observe the Y.js map deeply and send Y_UPDATE events on every change.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🛠️createyjssyncactor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/d/i/createYjsSyncActor)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🛠️createyjsfieldsyncactor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/d/i/createYjsFieldSyncActor)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🛠️ytransact](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/d/i/yTransact)
 **/
export function yTransact(yDoc: Y.Doc, fn: () => void, origin?: string): void {
  yDoc.transact(fn, origin);
}

/**
 * Creates an XState assign action that marks dirty and caches Y_UPDATE event data.
 *
 * MUST return an XState assign that sets dirty to true and caches event data.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🛠️createyjsupdateassign](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/d/i/createYjsUpdateAssign)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🛠️createyjsselector](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/d/i/createYjsSelector)
 **/
export function createYjsSelector<TContext extends YjsSyncContext, TSnapshot>(buildSnapshot: (context: TContext) => TSnapshot): (context: TContext) => TSnapshot {
  return (context: TContext): TSnapshot => {
    if (!context.dirty && context.cache) {
      return context.cache as TSnapshot;
    }
    return buildSnapshot(context);
  };
}

// #endregion Y.js-XState Bridge

// #region Machine Factories

// [👤semio📚js🗃️sketchpad💻sharedts🔖xstateintegration🔖machinefactories](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/X-STATE-INTEGRATION/MACHINE-FACTORIES)
// MUST define machine input and transaction configuration interfaces for state machine creation.

/**
 * Input for creating an app machine with Y.js map and transact function.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖machinefactories🛠️appmachineinput](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/Machine%20Factories/d/i/AppMachineInput)
 **/
export interface AppMachineInput {
  yMap: Y.Map<any>;
  transact: Transact;
}

/**
 * Extended app machine input with a kit GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖machinefactories🛠️kitdiffappmachineinput](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/Machine%20Factories/d/i/KitDiffAppMachineInput)
 **/
export interface KitDiffAppMachineInput extends AppMachineInput {
  kitGuid: Guid;
}

/**
 * Configuration for transaction handling with apply and inverse functions.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖machinefactories🛠️transactionmachineconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/Machine%20Factories/d/i/TransactionMachineConfig)
 **/
export interface TransactionMachineConfig<TEdit = any> {
  applySelectionDiff: (selectionDiff: any) => void;

  inverseSelectionDiff: (selection: any, diff: any) => any;

  applyKitDiff?: (kitDiff: KitDiff) => void;

  inverseKitDiff?: (kit: Kit, diff: KitDiff) => KitDiff;
}

// #endregion Machine Factories

// #endregion XState Integration

// #region YPath Helpers

// [👤semio📚js🗃️sketchpad💻sharedts🔖ypathhelpers](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/Y-PATH-HELPERS)
// MUST provide path segment constructors, value retrieval, and observation functions for Y.js paths.

/**
 * Creates a YPathSegment for accessing a map key.
 *
 * MUST return a mapKey segment with the given key.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🛠️ypathmapkey](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/d/i/yPathMapKey)
 **/
export function yPathMapKey(key: string): YPathSegment {
  return { kind: "mapKey", key };
}

/**
 * Creates a YPathSegment for accessing an array element by index.
 *
 * MUST return an arrayIndex segment with the given index.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🛠️ypatharrayindex](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/d/i/yPathArrayIndex)
 **/
export function yPathArrayIndex(index: number): YPathSegment {
  return { kind: "arrayIndex", index };
}

/**
 * Creates a YPathSegment for accessing an array item by its ID field.
 *
 * MUST return an arrayItemById segment with the given ID and idKey.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🛠️ypatharrayitembyid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/d/i/yPathArrayItemById)
 **/
export function yPathArrayItemById(id: string, idKey: string = "guid"): YPathSegment {
  return { kind: "arrayItemById", id, idKey };
}

/**
 * Traverses a Y.js map or array along a YPath and returns the value at the end.
 *
 * MUST traverse each path segment, returning undefined when a segment cannot be resolved.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🛠️getvalueatpath](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/d/i/getValueAtPath)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🛠️createpathobserver](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/d/i/createPathObserver)
 **/
export function createPathObserver(root: Y.Map<any>, path: YPath, subscribe: Subscribe): Disposable {
  if (path.length === 0) {
    const callback = () => subscribe(() => { });
    root.observeDeep(callback);
    return () => root.unobserveDeep(callback);
  }
  const disposables: Disposable[] = [];
  const serializeValue = (v: any) => JSON.stringify(v instanceof Y.Map || v instanceof Y.Array ? v.toJSON() : v);
  let lastJson = serializeValue(getValueAtPath(root, path));
  const notifyIfChanged = () => {
    const newJson = serializeValue(getValueAtPath(root, path));
    if (lastJson !== newJson) {
      lastJson = newJson;
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

// #endregion YPath Helpers

// #region Derived Store

// [👤semio📚js🗃️sketchpad💻sharedts🔖derivedstore](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/DERIVED-STORE)
// MUST provide reactive derived computation nodes with dependency tracking and caching.

/**
 * A dependency on a store path used by DerivedNode for change tracking.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🛠️basedependency](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/d/i/BaseDependency)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🛠️derivednode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/d/i/DerivedNode)
 **/
export class DerivedNode<T> {
  private deps: BaseDependency[];
  private compute: () => T;
  private value: T | undefined;
  private valueJson?: string;
  private subscribers = new Set<() => void>();
  private unsubscribers: Disposable[] = [];
  private initialized = false;
  version: number = 0;

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

  private static jsonReplacer(_key: string, value: any): any {
    if (value instanceof Map) return Object.fromEntries(value);
    if (value instanceof Set) return [...value];
    return value;
  }

  private recompute() {
    const next = this.compute();
    const nextJson = JSON.stringify(next, DerivedNode.jsonReplacer);
    if (nextJson !== this.valueJson) {
      this.value = next;
      this.valueJson = nextJson;
      this.version++;
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
        this.version = 0;
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
    this.version = 0;
  }
}

/**
 * A keyed collection of DerivedNode instances with lifecycle management.
 *
 * MUST manage DerivedNode lifecycle including creation, retrieval, and disposal.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🛠️derivedstore](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/d/i/DerivedStore)
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

// #endregion Derived Store

// #region Store Factory Registry

// [👤semio📚js🗃️sketchpad💻sharedts🔖storefactoryregistry](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/STORE-FACTORY-REGISTRY)
// MUST manage registration and retrieval of app-specific store factory functions.

/**
 * Factory function type for creating a design app store.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️designappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/DesignAppStoreFactory)
 **/
export type DesignAppStoreFactory = (parent: any, id: any, state?: any) => any;
/**
 * Factory function type for creating a kit app store.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️kitappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/KitAppStoreFactory)
 **/
export type KitAppStoreFactory = (parent: any, yMap: any, transact: (fn: () => void) => void, id: any, state?: any) => any;
/**
 * Factory function type for creating a type app store.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️typeappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/TypeAppStoreFactory)
 **/
export type TypeAppStoreFactory = (parent: any, id: any, state?: any) => any;
/**
 * Factory function type for creating a quality app store.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️qualityappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/QualityAppStoreFactory)
 **/
export type QualityAppStoreFactory = (parent: any, id: any, state?: any) => any;

/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🪨designappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/designAppStoreFactory)
 * designAppStoreFactory holds the data fields for a designAppStoreFactory record.
 **/
let designAppStoreFactory: DesignAppStoreFactory | undefined;
/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🪨kitappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/kitAppStoreFactory)
 * kitAppStoreFactory holds the data fields for a kitAppStoreFactory record.
 **/
let kitAppStoreFactory: KitAppStoreFactory | undefined;
/**
 * typeAppStoreFactory holds the data fields for a typeAppStoreFactory record.
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🪨typeappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/typeAppStoreFactory)
 **/
let typeAppStoreFactory: TypeAppStoreFactory | undefined;
/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🪨qualityappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/qualityAppStoreFactory)
 * qualityAppStoreFactory holds the data fields for a qualityAppStoreFactory record.
 **/
let qualityAppStoreFactory: QualityAppStoreFactory | undefined;

/**
 * Registers the design app store factory.
 *
 * MUST replace any previously registered design app store factory.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️registerdesignappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/registerDesignAppStoreFactory)
 **/
export function registerDesignAppStoreFactory(factory: DesignAppStoreFactory) {
  designAppStoreFactory = factory;
}

/**
 * Registers the kit app store factory.
 *
 * MUST replace any previously registered kit app store factory.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️registerkitappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/registerKitAppStoreFactory)
 **/
export function registerKitAppStoreFactory(factory: KitAppStoreFactory) {
  kitAppStoreFactory = factory;
}

/**
 * Registers the type app store factory.
 *
 * MUST replace any previously registered type app store factory.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️registertypeappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/registerTypeAppStoreFactory)
 **/
export function registerTypeAppStoreFactory(factory: TypeAppStoreFactory) {
  typeAppStoreFactory = factory;
}

/**
 * Registers the quality app store factory.
 *
 * MUST replace any previously registered quality app store factory.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️registerqualityappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/registerQualityAppStoreFactory)
 **/
export function registerQualityAppStoreFactory(factory: QualityAppStoreFactory) {
  qualityAppStoreFactory = factory;
}

/**
 * Retrieves the registered design app store factory or throws if not registered.
 *
 * MUST throw if no design app store factory has been registered.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️getdesignappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/getDesignAppStoreFactory)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️getkitappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/getKitAppStoreFactory)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️gettypeappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/getTypeAppStoreFactory)
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
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖storefactoryregistry🛠️getqualityappstorefactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Store%20Factory%20Registry/d/i/getQualityAppStoreFactory)
 **/
export function getQualityAppStoreFactory(): QualityAppStoreFactory {
  if (!qualityAppStoreFactory) throw new Error("Quality app store factory not registered");
  return qualityAppStoreFactory;
}

// #endregion Store Factory Registry

// #region App Plugin Registry

// [👤semio📚js🗃️sketchpad💻sharedts🔖apppluginregistry](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/APP-PLUGIN-REGISTRY)
// MUST manage plugin registration, retrieval, and contribution composition for app extensions.

/**
 * Plugin contribution of event types, actions, guards, handlers, selectors, and default state.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apppluginregistry🛠️appmachinecontribution](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Plugin%20Registry/d/i/AppMachineContribution)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apppluginregistry🛠️appplugin](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Plugin%20Registry/d/i/AppPlugin)
 **/
export interface AppPlugin {
  id: string;

  namespace: string;

  machine: AppMachineContribution;

  registerStores?: () => void;

  onRegister?: () => void;
}

/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apppluginregistry🪨appplugins](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Plugin%20Registry/d/i/appPlugins)
 * appPlugins holds the data fields for a appPlugins record.
 **/
const appPlugins: Map<string, AppPlugin> = new Map();

/**
 * Registers an app plugin, invoking its store registration and onRegister hooks.
 *
 * MUST store the plugin and invoke registerStores and onRegister hooks if present.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apppluginregistry🛠️registerappplugin](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Plugin%20Registry/d/i/registerAppPlugin)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apppluginregistry🛠️getappplugins](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Plugin%20Registry/d/i/getAppPlugins)
 **/
export function getAppPlugins(): AppPlugin[] {
  return Array.from(appPlugins.values());
}

/**
 * Returns the registered app plugin with the given ID, or undefined.
 *
 * MUST look up the plugin by ID in the registry.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apppluginregistry🛠️getappplugin](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Plugin%20Registry/d/i/getAppPlugin)
 **/
export function getAppPlugin(id: string): AppPlugin | undefined {
  return appPlugins.get(id);
}

/**
 * Checks whether an app plugin with the given ID is registered.
 *
 * MUST check the registry for the given plugin ID.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apppluginregistry🛠️hasappplugin](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Plugin%20Registry/d/i/hasAppPlugin)
 **/
export function hasAppPlugin(id: string): boolean {
  return appPlugins.has(id);
}

/**
 * Merges actions, guards, event handlers, and selectors from all registered plugins.
 *
 * MUST iterate all plugins and merge their contributions into single records.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apppluginregistry🛠️composeplugincontributions](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Plugin%20Registry/d/i/composePluginContributions)
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
 *
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apppluginregistry🛠️getplugindefaultstates](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Plugin%20Registry/d/i/getPluginDefaultStates)
 **/
export function getPluginDefaultStates(): Record<string, any> {
  for (const plugin of appPlugins.values()) {
    const createDefaultState = plugin.machine.createDefaultState;
    if (!createDefaultState) continue;
    defaults[plugin.id] = createDefaultState();
  }
  return defaults;
}

// #endregion App Plugin Registry

// #region Dynamic Event Dispatch Registry

// [👤semio📚js🗃️sketchpad💻sharedts🔖dynamiceventdispatchregistry](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/DYNAMIC-EVENT-DISPATCH-REGISTRY)
// MUST manage dynamic event handler and guard registration with namespace-based dispatch.

/**
 * Configuration for a dynamic event handler with optional guard and action.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️eventhandlerconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/EventHandlerConfig)
 **/
export interface EventHandlerConfig<TContext = any, TEvent = any> {
  guard?: (context: TContext, event: TEvent) => boolean;

  action: (context: TContext, event: TEvent) => Partial<TContext>;
}

/**
 * eventHandlerRegistry holds the data fields for a eventHandlerRegistry record.
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🪨eventhandlerregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/eventHandlerRegistry)
 **/
const eventHandlerRegistry: Map<string, EventHandlerConfig> = new Map();

/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🪨guardregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/guardRegistry)
 * guardRegistry holds the data fields for a guardRegistry record.
 **/
const guardRegistry: Map<string, (context: any, event: any) => boolean> = new Map();

/**
 * Registers a dynamic event handler for a given event type.
 *
 * MUST store the handler config in the registry keyed by event type.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️registereventhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/registerEventHandler)
 **/
export function registerEventHandler<TContext = any, TEvent = any>(eventType: string, config: EventHandlerConfig<TContext, TEvent>): void {
  eventHandlerRegistry.set(eventType, config as EventHandlerConfig);
}

/**
 * Removes a registered event handler for a given event type.
 *
 * MUST remove the handler for the given event type.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️unregistereventhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/unregisterEventHandler)
 **/
export function unregisterEventHandler(eventType: string): void {
  eventHandlerRegistry.delete(eventType);
}

/**
 * Checks whether an event handler is registered for a given event type.
 *
 * MUST check the registry for the given event type.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️haseventhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/hasEventHandler)
 **/
export function hasEventHandler(eventType: string): boolean {
  return eventHandlerRegistry.has(eventType);
}

/**
 * Retrieves the event handler configuration for a given event type.
 *
 * MUST return the handler config or undefined.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️geteventhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/getEventHandler)
 **/
export function getEventHandler(eventType: string): EventHandlerConfig | undefined {
  return eventHandlerRegistry.get(eventType);
}

/**
 * Executes the registered event handler for the given event, applying guard and action.
 *
 * MUST run the guard before the action, returning empty context when guard fails.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️executeeventhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/executeEventHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️registerguard](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/registerGuard)
 **/
export function registerGuard(name: string, guard: (context: any, event: any) => boolean): void {
  guardRegistry.set(name, guard);
}

/**
 * Removes a registered guard function by name.
 *
 * MUST remove the guard function by name.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️unregisterguard](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/unregisterGuard)
 **/
export function unregisterGuard(name: string): void {
  guardRegistry.delete(name);
}

/**
 * Retrieves a registered guard function by name.
 *
 * MUST return the guard function or undefined.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️getguard](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/getGuard)
 **/
export function getGuard(name: string): ((context: any, event: any) => boolean) | undefined {
  return guardRegistry.get(name);
}

/**
 * Checks whether a guard with the given name is registered.
 *
 * MUST check the guard registry for the given name.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️hasguard](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/hasGuard)
 **/
export function hasGuard(name: string): boolean {
  return guardRegistry.has(name);
}

/**
 * Executes a registered guard and returns its boolean result.
 *
 * MUST return false when the guard is not registered.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️executeguard](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/executeGuard)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️geteventtypesfornamespace](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/getEventTypesForNamespace)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️getregisterednamespaces](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/getRegisteredNamespaces)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖dynamiceventdispatchregistry🛠️getregisteredeventtypes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Dynamic%20Event%20Dispatch%20Registry/d/i/getRegisteredEventTypes)
 **/
export function getRegisteredEventTypes(): string[] {
  return Array.from(eventHandlerRegistry.keys());
}

// #endregion Dynamic Event Dispatch Registry

// #region App Event Handler Factories

// [👤semio📚js🗃️sketchpad💻sharedts🔖appeventhandlerfactories](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/APP-EVENT-HANDLER-FACTORIES)
// MUST provide factory functions for creating standard app event handlers for panels, hover, selection, and windows.

/**
 * Configuration for an app event handler with namespace, app key, and default state factory.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️appeventhandlerconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/AppEventHandlerConfig)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createtogglepanelhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createTogglePanelHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsetpanelvisibilityhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSetPanelVisibilityHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsethoverhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSetHoverHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createclearhoverhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createClearHoverHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsetwindowlayouthandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSetWindowLayoutHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createclearselectionhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createClearSelectionHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️keyedappeventhandlerconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/KeyedAppEventHandlerConfig)
 **/
export interface KeyedAppEventHandlerConfig<TAppKey extends string, TAppState> extends AppEventHandlerConfig<TAppKey, TAppState> {
  getKey: (event: any) => string;
}

const EXCLUSIVE_SIDE_PANEL_KEYS: (keyof PanelVisibility)[] = ["rightSidePanel", "chat", "settings"];

/**
 * Returns the next panel visibility state for a toggle event.
 *
 * MUST keep right side panel, chat, and settings mutually exclusive when one is activated.
 *
 *  * [👤semio📚js🗃️sketchpad💻sharedts🔖appeventhandlerfactories🛠️getnextpanelvisibilityfromtoggle](semiorepo://definition/SEMIO/JS/SKETCHPAD/SHARED.TS/APP-EVENT-HANDLER-FACTORIES/GET-NEXT-PANEL-VISIBILITY-FROM-TOGGLE)
 **/
export function getNextPanelVisibilityFromToggle(panelVisibility: PanelVisibility, panel: keyof PanelVisibility): PanelVisibility {
  const nextValue = !panelVisibility[panel];
  if (!EXCLUSIVE_SIDE_PANEL_KEYS.includes(panel) || !nextValue) {
    return {
      ...panelVisibility,
      [panel]: nextValue,
    };
  }
  const nextPanelVisibility: PanelVisibility = {
    ...panelVisibility,
    [panel]: true,
  };
  EXCLUSIVE_SIDE_PANEL_KEYS.forEach((key) => {
    if (key !== panel) {
      nextPanelVisibility[key] = false;
    }
  });
  return nextPanelVisibility;
}

/**
 * Registers a keyed toggle panel event handler for multi-instance app state.
 *
 * MUST register a keyed handler that toggles the panel for the resolved key.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedtogglepanelhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedTogglePanelHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedsetpanelvisibilityhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedSetPanelVisibilityHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedsethoverhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedSetHoverHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedclearhoverhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedClearHoverHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedsetselectionhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedSetSelectionHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedclearselectionhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedClearSelectionHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedsetwindowlayouthandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedSetWindowLayoutHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedsetcamerahandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedSetCameraHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedsetactivetoolhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedSetActiveToolHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedsetfullscreenwindowhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedSetFullscreenWindowHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedinithandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedInitHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createkeyedsynchandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createKeyedSyncHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️registerstandardappeventhandlers](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/registerStandardAppEventHandlers)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️registerkeyedappeventhandlers](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/registerKeyedAppEventHandlers)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️singlekeyappeventhandlerconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/SingleKeyAppEventHandlerConfig)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsinglekeyinithandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSingleKeyInitHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsinglekeysynchandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSingleKeySyncHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsinglekeytogglepanelhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSingleKeyTogglePanelHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsinglekeysetpanelvisibilityhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSingleKeySetPanelVisibilityHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsinglekeysethoverhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSingleKeySetHoverHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsinglekeyclearhoverhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSingleKeyClearHoverHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsinglekeysetselectionhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSingleKeySetSelectionHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsinglekeyclearselectionhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSingleKeyClearSelectionHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsinglekeysetwindowlayouthandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSingleKeySetWindowLayoutHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appeventhandlerfactories🛠️createsinglekeysetfullscreenwindowhandler](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Event%20Handler%20Factories/d/i/createSingleKeySetFullscreenWindowHandler)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖appeventhandlerfactories🛠️registersinglekeyappeventhandlers](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/App%20Event%20Handler%20Factories/d/i/registerSingleKeyAppEventHandlers)
 **/
export function registerSingleKeyAppEventHandlers<TAppKey extends string, TAppState extends object>(
  config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>,
  hoverMapper: (event: any) => any = (e) => e.hover,
) {
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

// #endregion App Event Handler Factories

// #region Transaction Handler Factory

// [👤semio📚js🗃️sketchpad💻sharedts🔖transactionhandlerfactory](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/TRANSACTION-HANDLER-FACTORY)
// MUST provide factory functions for creating undo/redo transaction event handlers.

/**
 * Configuration for keyed transaction handlers with namespace, app key, key fields, and default state.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖transactionhandlerfactory🛠️keyedtransactionhandlerconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Transaction%20Handler%20Factory/d/i/KeyedTransactionHandlerConfig)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖transactionhandlerfactory🛠️apptransactionstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Transaction%20Handler%20Factory/d/i/AppTransactionState)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖transactionhandlerfactory🛠️createkeyedtransactionhandlers](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Transaction%20Handler%20Factory/d/i/createKeyedTransactionHandlers)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖transactionhandlerfactory🛠️singlekeytransactionhandlerconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Transaction%20Handler%20Factory/d/i/SingleKeyTransactionHandlerConfig)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖transactionhandlerfactory🛠️createsinglekeytransactionhandlers](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Transaction%20Handler%20Factory/d/i/createSingleKeyTransactionHandlers)
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

// #endregion Transaction Handler Factory

// #region Selector Factory Pattern

// [👤semio📚js🗃️sketchpad💻sharedts🔖selectorfactorypattern](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/SELECTOR-FACTORY-PATTERN)
// MUST provide factory functions for creating property selectors with app key scoping.

/**
 * Creates a factory for selectors that read a property from a non-keyed app state.
 *
 * MUST return a factory that creates selectors reading from the given app key.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖selectorfactorypattern🛠️createapppropertyselectorfactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Selector%20Factory%20Pattern/d/i/createAppPropertySelectorFactory)
 **/
export function createAppPropertySelectorFactory<TApps extends Record<string, any>>(appKey: string) {
  return function createPropertySelector<TProperty>(propertyKey: keyof TApps[string], fallback: TProperty) {
    return (snapshot: { context: Record<string, TApps> }) => {
      const app = snapshot.context[appKey];
      return ((app as any)?.[propertyKey] ?? fallback) as TProperty;
    };
  };
}

/**
 * Creates a factory for selectors that read a property from a keyed app state.
 *
 * MUST return a factory that creates keyed selectors reading from the given app key.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖selectorfactorypattern🛠️createkeyedapppropertyselectorfactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Selector%20Factory%20Pattern/d/i/createKeyedAppPropertySelectorFactory)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖selectorfactorypattern🛠️getappkey](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Selector%20Factory%20Pattern/d/i/getAppKey)
 **/
export function getAppKey(...scopes: string[]): string {
  return scopes.join(":");
}
/**
 * Retrieves existing app state or creates it from a default factory.
 *
 * MUST return existing state or call the default factory to create it.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖selectorfactorypattern🛠️getorcreateappstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/Selector%20Factory%20Pattern/d/i/getOrCreateAppState)
 **/
export function getOrCreateAppState<TState>(context: Record<string, Record<string, TState>>, appKey: string, key: string, defaultFactory: () => TState): TState {
  const apps = context[appKey] || {};
  return apps[key] || defaultFactory();
}

// #endregion Selector Factory Pattern

// #region App Hooks Registry

// [👤semio📚js🗃️sketchpad💻sharedts🔖apphooksregistry](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/APP-HOOKS-REGISTRY)
// MUST manage registration and retrieval of design and kit app hook implementations.

/**
 * Interface for design app hook functions including commands, diff, hover, and selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apphooksregistry🛠️designapphooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Hooks%20Registry/d/i/DesignAppHooks)
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
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apphooksregistry🛠️kitapphooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Hooks%20Registry/d/i/KitAppHooks)
 **/
export interface KitAppHooks {
  useKitAppCommands: (id?: { kit: string }) => any;
}

/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apphooksregistry🪨defaultdesignapphooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Hooks%20Registry/d/i/defaultDesignAppHooks)
 * defaultDesignAppHooks holds the data fields for a defaultDesignAppHooks record.
 **/
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

/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apphooksregistry🪨defaultkitapphooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Hooks%20Registry/d/i/defaultKitAppHooks)
 * defaultKitAppHooks holds the data fields for a defaultKitAppHooks record.
 **/
const defaultKitAppHooks: KitAppHooks = {
  useKitAppCommands: () => ({ togglePanel: () => { }, execute: () => Promise.resolve({}) }),
};

/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apphooksregistry🪨registereddesignapphooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Hooks%20Registry/d/i/registeredDesignAppHooks)
 * registeredDesignAppHooks holds the data fields for a registeredDesignAppHooks record.
 **/
let registeredDesignAppHooks: DesignAppHooks | null = null;
/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apphooksregistry🪨registeredkitapphooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Hooks%20Registry/d/i/registeredKitAppHooks)
 * registeredKitAppHooks holds the data fields for a registeredKitAppHooks record.
 **/
let registeredKitAppHooks: KitAppHooks | null = null;

/**
 * Registers design app hook implementations.
 *
 * MUST store the provided hooks, replacing any previously registered.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apphooksregistry🛠️registerdesignapphooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Hooks%20Registry/d/i/registerDesignAppHooks)
 **/
export function registerDesignAppHooks(hooks: DesignAppHooks): void {
  registeredDesignAppHooks = hooks;
}

/**
 * Registers kit app hook implementations.
 *
 * MUST store the provided hooks, replacing any previously registered.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apphooksregistry🛠️registerkitapphooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Hooks%20Registry/d/i/registerKitAppHooks)
 **/
export function registerKitAppHooks(hooks: KitAppHooks): void {
  registeredKitAppHooks = hooks;
}

/** getDesignAppHooks holds the data fields for a getDesignAppHooks record.
 *
 * MUST fall back to default no-op hooks when none are registered.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apphooksregistry🛠️getdesignapphooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Hooks%20Registry/d/i/getDesignAppHooks)
 **/
export function getDesignAppHooks(): DesignAppHooks {
  return registeredDesignAppHooks ?? defaultDesignAppHooks;
}

/**
 * Returns registered kit app hooks or defaults.
 *
 * MUST fall back to default no-op hooks when none are registered.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖apphooksregistry🛠️getkitapphooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Hooks%20Registry/d/i/getKitAppHooks)
 **/
export function getKitAppHooks(): KitAppHooks {
  return registeredKitAppHooks ?? defaultKitAppHooks;
}

// #endregion App Hooks Registry

// #region App Registry Exports

// [👤semio📚js🗃️sketchpad💻sharedts🔖appregistryexports](semiorepo://section/SEMIO/JS/SKETCHPAD/SHARED.TS/APP-REGISTRY-EXPORTS)
// MUST provide docs registry port interface and registration for documentation section access.

/**
 * Port interface for retrieving documentation section trees and pages.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appregistryexports🛠️docsregistryport](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Registry%20Exports/d/i/DocsRegistryPort)
 **/
export interface DocsRegistryPort {
  getSectionTree: (section: string) => any[];
  getAllPages: () => any[];
  getPage?: (path: string) => any;
}

/**
 * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appregistryexports🪨registereddocsregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Registry%20Exports/d/i/registeredDocsRegistry)
 * registeredDocsRegistry holds the data fields for a registeredDocsRegistry record.
 **/
let registeredDocsRegistry: DocsRegistryPort | null = null;

/**
 * Registers a docs registry implementation.
 *
 * MUST store the given docs registry, replacing any previous one.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appregistryexports🛠️registerdocsregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Registry%20Exports/d/i/registerDocsRegistry)
 **/
export function registerDocsRegistry(registry: DocsRegistryPort): void {
  registeredDocsRegistry = registry;
}

/**
 * Returns the registered docs registry or null.
 *
 * MUST return the registered docs registry or null when none is registered.
 *
 *  * [👤semio📚js🗃️sketchpad💻shared🔖types🔖granularhooktypes🔖standardemptyconstants🔖enums🔖ports🔖fileprovider🔖appids🔖panel🔖appregistry🔖sketchpadstate🔖commands🔖store🔖completestate🔖window🔖tool🔖focus🔖footer🔖xstateintegration🔖xstatetypes🔖yjsxstatebridge🔖ypathhelpers🔖derivedstore🔖appregistryexports🛠️getdocsregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types/s/Granular%20Hook%20Types/s/Standard%20Empty%20Constants/s/Enums/s/Ports/s/File%20Provider/s/App%20IDs/s/Panel/s/App%20Registry/s/Sketchpad%20State/s/Commands/s/Store/s/Complete%20State/s/Window/s/Tool/s/Focus/s/Footer/s/XState%20Integration/s/XState%20Types/s/Y.js-XState%20Bridge/s/YPath%20Helpers/s/Derived%20Store/s/App%20Registry%20Exports/d/i/getDocsRegistry)
 **/
export function getDocsRegistry(): DocsRegistryPort | null {
  return registeredDocsRegistry;
}

// #endregion App Registry Exports
