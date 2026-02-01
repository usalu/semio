// #region Header

// js/semio/sketchpad/kitSelectionHelpers.ts

// SPDX-License-Identifier: LGPL-3.0-or-later

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

// #region Imports

import type { KitAppSelection } from "./Kit";

// #endregion Imports

// #region Types

/**
 * Helper type to extract array element type from a selection dimension
 */
export type SelectionValue<K extends keyof KitAppSelection> = NonNullable<KitAppSelection[K]> extends (infer T)[] ? T : never;

// #endregion Types

// #region Generic Utilities

/**
 * Adds a value to a selection dimension without clearing other dimensions.
 * 
 * @param selection - Current selection object
 * @param key - Dimension key (e.g., "types", "designs")
 * @param value - Value to add (e.g., guid)
 * @returns New selection object with value added
 * 
 * @example
 * const newSelection = addToSelection(
 *   { types: ["guid1"] },
 *   "types",
 *   "guid2"
 * );
 * // Result: { types: ["guid1", "guid2"] }
 */
export function addToSelection<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: SelectionValue<K>
): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  
  if (currentArray.includes(value)) {
    return selection;
  }
  
  return {
    ...selection,
    [key]: [...currentArray, value],
  };
}

/**
 * Removes a value from a selection dimension without affecting other dimensions.
 * 
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param value - Value to remove
 * @returns New selection object with value removed
 * 
 * @example
 * const newSelection = removeFromSelection(
 *   { types: ["guid1", "guid2"], designs: ["guid3"] },
 *   "types",
 *   "guid2"
 * );
 * // Result: { types: ["guid1"], designs: ["guid3"] }
 */
export function removeFromSelection<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: SelectionValue<K>
): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  const newArray = currentArray.filter((v) => v !== value);
  
  if (newArray.length === 0) {
    const { [key]: _, ...rest } = selection;
    return rest;
  }
  
  return {
    ...selection,
    [key]: newArray,
  };
}

/**
 * Toggles a value in a selection dimension (add if missing, remove if present).
 * 
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param value - Value to toggle
 * @returns New selection object with value toggled
 * 
 * @example
 * toggleInSelection({ types: ["guid1"] }, "types", "guid2") 
 * // => { types: ["guid1", "guid2"] }
 * 
 * toggleInSelection({ types: ["guid1", "guid2"] }, "types", "guid2")
 * // => { types: ["guid1"] }
 */
export function toggleInSelection<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: SelectionValue<K>
): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  
  if (currentArray.includes(value)) {
    return removeFromSelection(selection, key, value);
  } else {
    return addToSelection(selection, key, value);
  }
}

/**
 * Replaces an entire selection dimension without affecting other dimensions.
 * 
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param values - New values for the dimension (undefined to clear)
 * @returns New selection object with dimension replaced
 * 
 * @example
 * replaceSelectionDimension(
 *   { types: ["guid1"], designs: ["guid2"] },
 *   "types",
 *   ["guid3", "guid4"]
 * );
 * // Result: { types: ["guid3", "guid4"], designs: ["guid2"] }
 */
export function replaceSelectionDimension<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  values: KitAppSelection[K] | undefined
): KitAppSelection {
  if (!values || (Array.isArray(values) && values.length === 0)) {
    const { [key]: _, ...rest } = selection;
    return rest;
  }
  
  return {
    ...selection,
    [key]: values,
  };
}

/**
 * Clears a single selection dimension without affecting others.
 * 
 * @param selection - Current selection object
 * @param key - Dimension key to clear
 * @returns New selection object with dimension cleared
 * 
 * @example
 * clearSelectionDimension({ types: ["guid1"], designs: ["guid2"] }, "types")
 * // Result: { designs: ["guid2"] }
 */
export function clearSelectionDimension<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K
): KitAppSelection {
  const { [key]: _, ...rest } = selection;
  return rest;
}

/**
 * Clears all selection dimensions.
 * 
 * @returns Empty selection object
 * 
 * @example
 * clearSelection()
 * // Result: {}
 */
export function clearSelection(): KitAppSelection {
  return {};
}

/**
 * Selects all items in a dimension (replaces existing selection for that dimension).
 * 
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param allValues - All available values for the dimension
 * @returns New selection object with all values selected
 * 
 * @example
 * selectAllInDimension({ types: ["guid1"] }, "types", ["guid1", "guid2", "guid3"])
 * // Result: { types: ["guid1", "guid2", "guid3"] }
 */
export function selectAllInDimension<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  allValues: SelectionValue<K>[]
): KitAppSelection {
  return replaceSelectionDimension(selection, key, allValues as KitAppSelection[K]);
}

/**
 * Checks if a value is selected in a dimension.
 * 
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param value - Value to check
 * @returns True if value is selected
 */
export function isSelected<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: SelectionValue<K>
): boolean {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  return currentArray.includes(value);
}

// #endregion Generic Utilities
