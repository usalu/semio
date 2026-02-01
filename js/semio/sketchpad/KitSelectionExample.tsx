// #region Header

// js/semio/sketchpad/KitSelectionExample.tsx

// SPDX-License-Identifier: LGPL-3.0-or-later

// 2025 Ueli Saluz <ueli@semio-tech.com>

// Example component showing how to wire Kit selection helpers with modifier keys

// #endregion Header

// #region Imports

import React, { FC, useCallback } from "react";
import { Guid } from "../semio";
import {
  useKitAppAddTypeToSelection,
  useKitAppRemoveTypeFromSelection,
  useKitAppToggleTypeInSelection,
  useKitAppSelectSingleType,
  useKitAppClearSelection,
  useKitAppSelectAll,
} from "./Kit";

// #endregion Imports

// #region Example Table Component

/**
 * Example table component demonstrating modifier key selection patterns.
 * 
 * Modifier Key Behavior:
 * - Click (no modifier): Replace selection with single item
 * - Ctrl/Cmd + Click: Toggle item in selection
 * - Shift + Click: Add item to selection
 * - Alt + Click: Remove item from selection
 * - Background click: Clear all selections
 * - Escape key: Clear all selections (handled elsewhere)
 * - Ctrl/Cmd + A: Select all (handled elsewhere)
 */
export const KitTypeTableExample: FC<{ types: Array<{ guid: Guid; name: string }> }> = ({ types }) => {
  // Get all selection operation hooks
  const [selectSingleType] = useKitAppSelectSingleType();
  const [addTypeToSelection] = useKitAppAddTypeToSelection();
  const [removeTypeFromSelection] = useKitAppRemoveTypeFromSelection();
  const [toggleTypeInSelection] = useKitAppToggleTypeInSelection();
  const [clearSelection] = useKitAppClearSelection();
  const [selectAll] = useKitAppSelectAll();

  /**
   * Handle row click with modifier key detection.
   */
  const handleRowClick = useCallback(
    (typeGuid: Guid, event: React.MouseEvent) => {
      // Prevent event bubbling to background
      event.stopPropagation();

      const isCtrlOrCmd = event.ctrlKey || event.metaKey;
      const isShift = event.shiftKey;
      const isAlt = event.altKey;

      // Priority: Ctrl/Cmd > Shift > Alt > None
      if (isCtrlOrCmd) {
        // TOGGLE: Add if missing, remove if present
        toggleTypeInSelection?.(typeGuid);
      } else if (isShift) {
        // ADD: Add to selection without removing others
        addTypeToSelection?.(typeGuid);
      } else if (isAlt) {
        // REMOVE: Remove from selection
        removeTypeFromSelection?.(typeGuid);
      } else {
        // REPLACE: Clear others in dimension, select this one
        selectSingleType?.(typeGuid);
      }
    },
    [selectSingleType, addTypeToSelection, removeTypeFromSelection, toggleTypeInSelection]
  );

  /**
   * Handle background click to clear selection.
   */
  const handleBackgroundClick = useCallback(
    (event: React.MouseEvent) => {
      // Only clear if clicking background (not propagated from row)
      if (event.currentTarget === event.target) {
        clearSelection?.();
      }
    },
    [clearSelection]
  );

  return (
    <div onClick={handleBackgroundClick} className="w-full h-full">
      <table className="w-full">
        <thead>
          <tr>
            <th>Name</th>
            <th>GUID</th>
          </tr>
        </thead>
        <tbody>
          {types.map((type) => (
            <tr
              key={type.guid}
              onClick={(e) => handleRowClick(type.guid, e)}
              className="cursor-pointer hover:bg-hover"
            >
              <td>{type.name}</td>
              <td>{type.guid}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

// #endregion Example Table Component

// #region Example with Keyboard Shortcuts

/**
 * Example component showing keyboard shortcut integration.
 */
export const KitTableWithKeyboardShortcuts: FC<{ types: Array<{ guid: Guid; name: string }> }> = ({ types }) => {
  const [clearSelection] = useKitAppClearSelection();
  const [selectAll] = useKitAppSelectAll();

  // Handle keyboard shortcuts
  React.useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // Escape: Clear selection
      if (event.key === "Escape") {
        event.preventDefault();
        clearSelection?.();
      }

      // Ctrl/Cmd + A: Select all
      if ((event.ctrlKey || event.metaKey) && event.key === "a") {
        event.preventDefault();
        selectAll?.();
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [clearSelection, selectAll]);

  return <KitTypeTableExample types={types} />;
};

// #endregion Example with Keyboard Shortcuts

// #region Example with Multiple Dimensions

/**
 * Example showing selection across multiple dimensions.
 */
export const KitMultiDimensionExample: FC = () => {
  const [selectSingleType] = useKitAppSelectSingleType();
  const [selectSingleDesign] = useKitAppSelectSingleType(); // Note: Should use useKitAppSelectSingleDesign
  const [clearSelection] = useKitAppClearSelection();

  // Selecting a type doesn't clear designs
  const handleTypeClick = useCallback(
    (typeGuid: Guid) => {
      selectSingleType?.(typeGuid);
      // Selection now: { types: [typeGuid], designs: [...kept...] }
    },
    [selectSingleType]
  );

  // Selecting a design doesn't clear types
  const handleDesignClick = useCallback(
    (designGuid: Guid) => {
      selectSingleDesign?.(designGuid);
      // Selection now: { types: [...kept...], designs: [designGuid] }
    },
    [selectSingleDesign]
  );

  // Clear all dimensions
  const handleClearAll = useCallback(() => {
    clearSelection?.();
    // Selection now: {}
  }, [clearSelection]);

  return (
    <div>
      {/* Types and designs can be selected independently */}
      <button onClick={() => handleTypeClick("type-guid-1")}>Select Type 1</button>
      <button onClick={() => handleDesignClick("design-guid-1")}>Select Design 1</button>
      <button onClick={handleClearAll}>Clear All</button>
    </div>
  );
};

// #endregion Example with Multiple Dimensions

// #region Example with Diagram Integration

/**
 * Example showing selection in diagram context.
 */
export const KitDiagramExample: FC<{ nodes: Array<{ id: Guid; kind: "type" | "design" }> }> = ({ nodes }) => {
  const [selectSingleType] = useKitAppSelectSingleType();
  const [toggleTypeInSelection] = useKitAppToggleTypeInSelection();
  const [addTypeToSelection] = useKitAppAddTypeToSelection();

  const handleNodeClick = useCallback(
    (nodeId: Guid, nodeKind: "type" | "design", event: React.MouseEvent) => {
      if (nodeKind !== "type") return;

      const isCtrlOrCmd = event.ctrlKey || event.metaKey;
      const isShift = event.shiftKey;

      if (isCtrlOrCmd) {
        toggleTypeInSelection?.(nodeId);
      } else if (isShift) {
        addTypeToSelection?.(nodeId);
      } else {
        selectSingleType?.(nodeId);
      }
    },
    [selectSingleType, toggleTypeInSelection, addTypeToSelection]
  );

  return (
    <div>
      {nodes.map((node) => (
        <div
          key={node.id}
          onClick={(e) => handleNodeClick(node.id, node.kind, e)}
          className="cursor-pointer"
        >
          {node.kind}: {node.id}
        </div>
      ))}
    </div>
  );
};

// #endregion Example with Diagram Integration

// #region Summary

/**
 * SELECTION HOOK USAGE SUMMARY
 * ============================
 * 
 * ## Available Hooks (per dimension)
 * 
 * - useKitAppAdd{Dimension}ToSelection() - Add without clearing others
 * - useKitAppRemove{Dimension}FromSelection() - Remove from selection
 * - useKitAppToggle{Dimension}InSelection() - Toggle (add/remove)
 * - useKitAppSelectSingle{Dimension}() - Replace dimension selection
 * - useKitAppSelect{Dimension}() - Replace with multiple items
 * - useKitAppClear{Dimension}() - Clear dimension only
 * 
 * ## Global Hooks
 * 
 * - useKitAppSelectAll() - Select all artifacts
 * - useKitAppClearSelection() - Clear all dimensions
 * 
 * ## All Dimensions
 * 
 * Types, Designs, Qualities, Ports, Tags, Concepts, Files, Folders, Authors
 * 
 * ## Modifier Key Pattern
 * 
 * - No modifier: selectSingle{Dimension}()
 * - Ctrl/Cmd: toggle{Dimension}InSelection()
 * - Shift: add{Dimension}ToSelection()
 * - Alt: remove{Dimension}FromSelection()
 * 
 * ## Return Pattern
 * 
 * All hooks return: [action, canAct]
 * - action: Function to call (undefined if canAct is false)
 * - canAct: Boolean indicating if action is available
 * 
 * ## Example Usage
 * 
 * ```typescript
 * const [selectType] = useKitAppSelectSingleType();
 * const [toggleType] = useKitAppToggleTypeInSelection();
 * 
 * // Simple click
 * onClick={() => selectType?.(typeGuid)}
 * 
 * // With modifier detection
 * onClick={(e) => {
 *   if (e.ctrlKey || e.metaKey) {
 *     toggleType?.(typeGuid);
 *   } else {
 *     selectType?.(typeGuid);
 *   }
 * }}
 * ```
 * 
 * ## Dimension Independence
 * 
 * Selecting a type NEVER clears designs/ports/tags/etc.
 * Each dimension is independent unless explicitly cleared.
 * 
 * ```typescript
 * selectSingleType(guid1);
 * // Selection: { types: [guid1] }
 * 
 * selectSingleDesign(guid2);
 * // Selection: { types: [guid1], designs: [guid2] }
 * 
 * clearTypes();
 * // Selection: { designs: [guid2] }
 * ```
 */

// #endregion Summary
