// #region Header

// xstate-hooks.ts - Clean React hooks for XState-based state management
//
// All hooks in this file:
// - Use useSelector from @xstate/react to read from the XState actor
// - Use actor.send() to dispatch events
// - Do NOT use Y.js directly
//
// 2025 Ueli Saluz
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion

// #region Imports

import { useSelector } from "@xstate/react";
import { createContext, useContext, useMemo } from "react";
import { Guid } from "../semio";
import {
    createDesignActiveToolSelector,
    createDesignAppSelector,
    createDesignCameraSelector,
    createDesignDiagramCenterSelector,
    createDesignDiagramScaleSelector,
    createDesignFocusedPieceSelector,
    createDesignFullscreenWindowSelector,
    createDesignHoverSelector,
    createDesignPanelVisibilitySelector,
    createDesignSelectedModelTagsSelector,
    createDesignSelectionSelector,
    createKitAppSelector,
    createKitExpandedRowsSelector,
    createKitFilterSearchSelector,
    createKitHoverSelector,
    createKitPanelVisibilitySelector,
    createKitSelectionSelector,
    createTransactionCanRedoSelector,
    createTransactionCanUndoSelector,
    createTransactionIsActiveSelector,
    createTypeActiveToolSelector,
    createTypeAppSelector,
    createTypeCameraSelector,
    createTypeFocusedPortSelector,
    createTypeFullscreenWindowSelector,
    createTypeHoverSelector,
    createTypePanelVisibilitySelector,
    createTypeSelectedModelTagsSelector,
    createTypeSelectionSelector,
    DesignAppFullscreenWindow,
    DesignAppHover,
    DesignAppSelection,
    DesignAppState,
    HomeAppSelection,
    HomeAppState,
    KitAppSelection,
    KitAppState,
    selectHomeApp,
    selectHomeHover,
    selectHomeLoadingKits,
    selectHomePanelVisibility,
    selectHomeSelection,
    selectHomeSortColumn,
    selectHomeSortDirection,
    selectSketchpadExpertise,
    selectSketchpadIsFullscreen,
    selectSketchpadLanguage,
    selectSketchpadLayout,
    selectSketchpadMode,
    selectSketchpadNavigation,
    selectSketchpadNavigationHistory,
    selectSketchpadNavigationHistoryIndex,
    selectSketchpadPanelSizes,
    selectSketchpadSettings,
    selectSketchpadTheme,
    SketchpadActorRef,
    TypeAppFullscreenWindow,
    TypeAppHover,
    TypeAppSelection,
    TypeAppState
} from "./machines";
import { PanelVisibility, ToolKind } from "./shared";

// #endregion Imports

// #region Actor Context

/**
 * Context for the XState actor.
 * This is provided by SketchpadScopeProvider in Sketchpad.tsx.
 */
export const SketchpadActorContext = createContext<SketchpadActorRef | null>(null);

/**
 * Get the XState actor from context.
 * Must be used within a SketchpadScopeProvider.
 */
function useActor(): SketchpadActorRef {
    const actor = useContext(SketchpadActorContext);
    if (!actor) {
        throw new Error("useActor must be used within a SketchpadScopeProvider");
    }
    return actor;
}

/**
 * Export useActor as useSketchpadActorHook for external use.
 */
export { useActor as useSketchpadActorHook };

// #endregion Actor Context

// #region Home App Hooks

/**
 * Get the full home app state.
 */
export function useHomeApp(): HomeAppState {
    const actor = useActor();
    return useSelector(actor, selectHomeApp);
}

/**
 * Get home app panel visibility.
 */
export function useHomePanelVisibility(): PanelVisibility {
    const actor = useActor();
    return useSelector(actor, selectHomePanelVisibility);
}

/**
 * Get home app selection.
 */
export function useHomeSelection(): HomeAppSelection | undefined {
    const actor = useActor();
    return useSelector(actor, selectHomeSelection);
}

/**
 * Get home app hover state.
 */
export function useHomeHover(): { kits?: Guid[] } | undefined {
    const actor = useActor();
    return useSelector(actor, selectHomeHover);
}

/**
 * Get home app sort column.
 */
export function useHomeSortColumn(): string | undefined {
    const actor = useActor();
    return useSelector(actor, selectHomeSortColumn);
}

/**
 * Get home app sort direction.
 */
export function useHomeSortDirection(): "asc" | "desc" | undefined {
    const actor = useActor();
    return useSelector(actor, selectHomeSortDirection);
}

/**
 * Get loading kits list.
 */
export function useHomeLoadingKits(): Array<{ tempGuid: string; name: string }> {
    const actor = useActor();
    return useSelector(actor, selectHomeLoadingKits);
}

/**
 * Commands for the home app.
 * Functions accept optional origin string as first arg for backwards compatibility.
 */
export function useHomeCommands() {
    const actor = useActor();

    return useMemo(() => ({
        togglePanel: (originOrPanel: string | keyof PanelVisibility, panel?: keyof PanelVisibility) => {
            const actualPanel = panel ?? (originOrPanel as keyof PanelVisibility);
            actor.send({ type: "HOME.TOGGLE_PANEL", panel: actualPanel });
        },
        setSort: (originOrColumn: string, columnOrDirection?: string | "asc" | "desc", direction?: "asc" | "desc") => {
            // Handle both (origin, column, direction) and (column, direction) signatures
            let actualColumn: string;
            let actualDirection: "asc" | "desc";
            if (direction !== undefined) {
                actualColumn = columnOrDirection as string;
                actualDirection = direction;
            } else if (columnOrDirection === "asc" || columnOrDirection === "desc") {
                actualColumn = originOrColumn;
                actualDirection = columnOrDirection;
            } else {
                actualColumn = originOrColumn;
                actualDirection = "asc";
            }
            actor.send({ type: "HOME.SET_SORT", column: actualColumn, direction: actualDirection });
        },
        selectKit: (originOrGuid: string, guid?: Guid) => {
            const actualGuid = guid ?? originOrGuid;
            actor.send({ type: "HOME.SELECT_KIT", guid: actualGuid });
        },
        // Alias for compatibility - accepts (origin, guids) or just (guids)
        selectKits: (originOrGuids: string | Guid[], guids?: Guid[]) => {
            const actualGuids = guids ?? (Array.isArray(originOrGuids) ? originOrGuids : [originOrGuids]);
            // Select multiple kits - send multiple events
            actualGuids.forEach(g => actor.send({ type: "HOME.SELECT_KIT", guid: g }));
        },
        deselectKit: (originOrGuid: string, guid?: Guid) => {
            const actualGuid = guid ?? originOrGuid;
            actor.send({ type: "HOME.DESELECT_KIT", guid: actualGuid });
        },
        // Alias for add kit to selection
        addKitToSelection: (originOrGuid: string, guid?: Guid) => {
            const actualGuid = guid ?? originOrGuid;
            actor.send({ type: "HOME.SELECT_KIT", guid: actualGuid });
        },
        // Alias for remove kit from selection
        removeKitFromSelection: (originOrGuid: string, guid?: Guid) => {
            const actualGuid = guid ?? originOrGuid;
            actor.send({ type: "HOME.DESELECT_KIT", guid: actualGuid });
        },
        // Deselect all kits
        deselectAll: (_origin?: string) => {
            // Clear selection by setting empty hover (simplified)
            actor.send({ type: "HOME.SET_HOVER", kits: [] });
        },
        setHover: (originOrKits?: string | Guid[], kits?: Guid[]) => {
            const actualKits = kits ?? (Array.isArray(originOrKits) ? originOrKits : undefined);
            actor.send({ type: "HOME.SET_HOVER", kits: actualKits });
        },
        clearHover: (_origin?: string) => {
            actor.send({ type: "HOME.CLEAR_HOVER" });
        },
        // Sort column/direction setters
        setSortColumn: (originOrColumn: string, column?: string) => {
            const actualColumn = column ?? originOrColumn;
            actor.send({ type: "HOME.SET_SORT", column: actualColumn, direction: "asc" });
        },
        setSortDirection: (originOrDirection: string, direction?: "asc" | "desc") => {
            const actualDirection = (direction ?? originOrDirection) as "asc" | "desc";
            actor.send({ type: "HOME.SET_SORT", column: "name", direction: actualDirection });
        },
        // Toggle sort direction
        toggleSort: (_origin?: string, _column?: string) => {
            // Simplified toggle
            actor.send({ type: "HOME.SET_SORT", column: "name", direction: "asc" });
        },
    }), [actor]);
}

// #endregion Home App Hooks

// #region Kit App Hooks

/**
 * Get the full kit app state.
 */
export function useKitApp(kitGuid: Guid): KitAppState {
    const actor = useActor();
    const selector = useMemo(() => createKitAppSelector(kitGuid), [kitGuid]);
    return useSelector(actor, selector);
}

/**
 * Get kit app panel visibility.
 */
export function useKitPanelVisibility(kitGuid: Guid): PanelVisibility {
    const actor = useActor();
    const selector = useMemo(() => createKitPanelVisibilitySelector(kitGuid), [kitGuid]);
    return useSelector(actor, selector);
}

/**
 * Get kit app selection.
 */
export function useKitSelection(kitGuid: Guid): KitAppSelection | undefined {
    const actor = useActor();
    const selector = useMemo(() => createKitSelectionSelector(kitGuid), [kitGuid]);
    return useSelector(actor, selector);
}

/**
 * Get kit app hover state.
 */
export function useKitHover(kitGuid: Guid): any {
    const actor = useActor();
    const selector = useMemo(() => createKitHoverSelector(kitGuid), [kitGuid]);
    return useSelector(actor, selector);
}

/**
 * Get kit app filter search.
 */
export function useKitFilterSearch(kitGuid: Guid): string {
    const actor = useActor();
    const selector = useMemo(() => createKitFilterSearchSelector(kitGuid), [kitGuid]);
    return useSelector(actor, selector);
}

/**
 * Get kit app expanded rows.
 */
export function useKitExpandedRows(kitGuid: Guid): Set<string> {
    const actor = useActor();
    const selector = useMemo(() => createKitExpandedRowsSelector(kitGuid), [kitGuid]);
    return useSelector(actor, selector);
}

/**
 * Commands for the kit app.
 */
export function useKitAppCommands(kitGuid: Guid) {
    const actor = useActor();

    return useMemo(() => ({
        togglePanel: (panel: keyof PanelVisibility) => {
            actor.send({ type: "KIT.TOGGLE_PANEL", kitGuid, panel });
        },
        setFilter: (search: string) => {
            actor.send({ type: "KIT.SET_FILTER", kitGuid, search });
        },
        toggleRow: (rowId: string) => {
            actor.send({ type: "KIT.TOGGLE_ROW", kitGuid, rowId });
        },
        setSort: (column: string, direction: "asc" | "desc") => {
            actor.send({ type: "KIT.SET_SORT", kitGuid, column, direction });
        },
        selectType: (typeGuid: Guid) => {
            actor.send({ type: "KIT.SELECT_TYPE", kitGuid, typeGuid });
        },
        deselectType: (typeGuid: Guid) => {
            actor.send({ type: "KIT.DESELECT_TYPE", kitGuid, typeGuid });
        },
        selectDesign: (designGuid: Guid) => {
            actor.send({ type: "KIT.SELECT_DESIGN", kitGuid, designGuid });
        },
        deselectDesign: (designGuid: Guid) => {
            actor.send({ type: "KIT.DESELECT_DESIGN", kitGuid, designGuid });
        },
        setSelection: (selection: KitAppSelection) => {
            actor.send({ type: "KIT.SET_SELECTION", kitGuid, selection });
        },
        clearSelection: () => {
            actor.send({ type: "KIT.CLEAR_SELECTION", kitGuid });
        },
        setHover: (hover: any) => {
            actor.send({ type: "KIT.SET_HOVER", kitGuid, hover });
        },
        clearHover: () => {
            actor.send({ type: "KIT.CLEAR_HOVER", kitGuid });
        },
    }), [actor, kitGuid]);
}

// #endregion Kit App Hooks

// #region Type App Hooks

/**
 * Get the full type app state.
 */
export function useTypeApp(kitGuid: Guid, typeGuid: Guid): TypeAppState {
    const actor = useActor();
    const selector = useMemo(() => createTypeAppSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
    return useSelector(actor, selector);
}

/**
 * Get type app panel visibility.
 */
export function useTypePanelVisibility(kitGuid: Guid, typeGuid: Guid): PanelVisibility {
    const actor = useActor();
    const selector = useMemo(() => createTypePanelVisibilitySelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
    return useSelector(actor, selector);
}

/**
 * Get type app selection.
 */
export function useTypeSelection(kitGuid: Guid, typeGuid: Guid): TypeAppSelection | undefined {
    const actor = useActor();
    const selector = useMemo(() => createTypeSelectionSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
    return useSelector(actor, selector);
}

/**
 * Get type app hover state.
 */
export function useTypeHover(kitGuid: Guid, typeGuid: Guid): TypeAppHover | undefined {
    const actor = useActor();
    const selector = useMemo(() => createTypeHoverSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
    return useSelector(actor, selector);
}

/**
 * Get type app focused port.
 */
export function useTypeFocusedPort(kitGuid: Guid, typeGuid: Guid): Guid | undefined {
    const actor = useActor();
    const selector = useMemo(() => createTypeFocusedPortSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
    return useSelector(actor, selector);
}

/**
 * Get type app selected model tags.
 */
export function useTypeSelectedModelTags(kitGuid: Guid, typeGuid: Guid): Guid[] {
    const actor = useActor();
    const selector = useMemo(() => createTypeSelectedModelTagsSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
    return useSelector(actor, selector);
}

/**
 * Get type app camera.
 */
export function useTypeCamera(kitGuid: Guid, typeGuid: Guid): any {
    const actor = useActor();
    const selector = useMemo(() => createTypeCameraSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
    return useSelector(actor, selector);
}

/**
 * Get type app active tool.
 */
export function useTypeActiveTool(kitGuid: Guid, typeGuid: Guid): ToolKind {
    const actor = useActor();
    const selector = useMemo(() => createTypeActiveToolSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
    return useSelector(actor, selector);
}

/**
 * Get type app fullscreen window.
 */
export function useTypeFullscreenWindow(kitGuid: Guid, typeGuid: Guid): TypeAppFullscreenWindow {
    const actor = useActor();
    const selector = useMemo(() => createTypeFullscreenWindowSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
    return useSelector(actor, selector);
}

/**
 * Commands for the type app.
 */
export function useTypeAppCommands(kitGuid: Guid, typeGuid: Guid) {
    const actor = useActor();

    return useMemo(() => ({
        togglePanel: (panel: keyof PanelVisibility) => {
            actor.send({ type: "TYPE.TOGGLE_PANEL", kitGuid, typeGuid, panel });
        },
        setActiveTool: (tool: ToolKind) => {
            actor.send({ type: "TYPE.SET_ACTIVE_TOOL", kitGuid, typeGuid, tool });
        },
        setSelection: (selection: TypeAppSelection) => {
            actor.send({ type: "TYPE.SET_SELECTION", kitGuid, typeGuid, selection });
        },
        clearSelection: () => {
            actor.send({ type: "TYPE.CLEAR_SELECTION", kitGuid, typeGuid });
        },
        selectPort: (portGuid: Guid) => {
            actor.send({ type: "TYPE.SELECT_PORT", kitGuid, typeGuid, portGuid });
        },
        deselectPort: (portGuid: Guid) => {
            actor.send({ type: "TYPE.DESELECT_PORT", kitGuid, typeGuid, portGuid });
        },
        setHover: (hover: TypeAppHover) => {
            actor.send({ type: "TYPE.SET_HOVER", kitGuid, typeGuid, hover });
        },
        clearHover: () => {
            actor.send({ type: "TYPE.CLEAR_HOVER", kitGuid, typeGuid });
        },
        focusPort: (portGuid?: Guid) => {
            actor.send({ type: "TYPE.FOCUS_PORT", kitGuid, typeGuid, portGuid });
        },
        selectModelTag: (tagGuid: Guid) => {
            actor.send({ type: "TYPE.SELECT_MODEL_TAG", kitGuid, typeGuid, tagGuid });
        },
        deselectModelTag: (tagGuid: Guid) => {
            actor.send({ type: "TYPE.DESELECT_MODEL_TAG", kitGuid, typeGuid, tagGuid });
        },
        setModelTags: (tags: Guid[]) => {
            actor.send({ type: "TYPE.SET_MODEL_TAGS", kitGuid, typeGuid, tags });
        },
        setCamera: (camera: any) => {
            actor.send({ type: "TYPE.SET_CAMERA", kitGuid, typeGuid, camera });
        },
    }), [actor, kitGuid, typeGuid]);
}

// #endregion Type App Hooks

// #region Design App Hooks

/**
 * Get the full design app state.
 */
export function useDesignApp(kitGuid: Guid, designGuid: Guid): DesignAppState {
    const actor = useActor();
    const selector = useMemo(() => createDesignAppSelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Get design app panel visibility.
 */
export function useDesignPanelVisibility(kitGuid: Guid, designGuid: Guid): PanelVisibility {
    const actor = useActor();
    const selector = useMemo(() => createDesignPanelVisibilitySelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Get design app selection.
 */
export function useDesignSelection(kitGuid: Guid, designGuid: Guid): DesignAppSelection | undefined {
    const actor = useActor();
    const selector = useMemo(() => createDesignSelectionSelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Get design app hover state.
 */
export function useDesignHover(kitGuid: Guid, designGuid: Guid): DesignAppHover | undefined {
    const actor = useActor();
    const selector = useMemo(() => createDesignHoverSelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Get design app focused piece.
 */
export function useDesignFocusedPiece(kitGuid: Guid, designGuid: Guid): Guid | undefined {
    const actor = useActor();
    const selector = useMemo(() => createDesignFocusedPieceSelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Get design app selected model tags.
 */
export function useDesignSelectedModelTags(kitGuid: Guid, designGuid: Guid): Record<Guid, Guid[]> {
    const actor = useActor();
    const selector = useMemo(() => createDesignSelectedModelTagsSelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Get design app diagram center.
 */
export function useDesignDiagramCenter(kitGuid: Guid, designGuid: Guid): { x: number; y: number } | undefined {
    const actor = useActor();
    const selector = useMemo(() => createDesignDiagramCenterSelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Get design app diagram scale.
 */
export function useDesignDiagramScale(kitGuid: Guid, designGuid: Guid): number | undefined {
    const actor = useActor();
    const selector = useMemo(() => createDesignDiagramScaleSelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Get design app camera.
 */
export function useDesignCamera(kitGuid: Guid, designGuid: Guid): any {
    const actor = useActor();
    const selector = useMemo(() => createDesignCameraSelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Get design app active tool.
 */
export function useDesignActiveTool(kitGuid: Guid, designGuid: Guid): ToolKind | undefined {
    const actor = useActor();
    const selector = useMemo(() => createDesignActiveToolSelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Get design app fullscreen window.
 */
export function useDesignFullscreenWindow(kitGuid: Guid, designGuid: Guid): DesignAppFullscreenWindow | undefined {
    const actor = useActor();
    const selector = useMemo(() => createDesignFullscreenWindowSelector(kitGuid, designGuid), [kitGuid, designGuid]);
    return useSelector(actor, selector);
}

/**
 * Commands for the design app.
 */
export function useDesignAppCommands(kitGuid: Guid, designGuid: Guid) {
    const actor = useActor();

    return useMemo(() => ({
        togglePanel: (panel: keyof PanelVisibility) => {
            actor.send({ type: "DESIGN.TOGGLE_PANEL", kitGuid, designGuid, panel });
        },
        setActiveTool: (tool: ToolKind) => {
            actor.send({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, tool });
        },
        setFullscreen: (window: DesignAppFullscreenWindow) => {
            actor.send({ type: "DESIGN.SET_FULLSCREEN", kitGuid, designGuid, window });
        },
        selectPiece: (pieceGuid: Guid) => {
            actor.send({ type: "DESIGN.SELECT_PIECE", kitGuid, designGuid, pieceGuid });
        },
        deselectPiece: (pieceGuid: Guid) => {
            actor.send({ type: "DESIGN.DESELECT_PIECE", kitGuid, designGuid, pieceGuid });
        },
        selectConnection: (connectionGuid: Guid) => {
            actor.send({ type: "DESIGN.SELECT_CONNECTION", kitGuid, designGuid, connectionGuid });
        },
        deselectConnection: (connectionGuid: Guid) => {
            actor.send({ type: "DESIGN.DESELECT_CONNECTION", kitGuid, designGuid, connectionGuid });
        },
        setSelection: (selection: DesignAppSelection) => {
            actor.send({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection });
        },
        clearSelection: () => {
            actor.send({ type: "DESIGN.CLEAR_SELECTION", kitGuid, designGuid });
        },
        setHover: (hover: DesignAppHover) => {
            actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover });
        },
        clearHover: () => {
            actor.send({ type: "DESIGN.CLEAR_HOVER", kitGuid, designGuid });
        },
        focusPiece: (pieceGuid?: Guid) => {
            actor.send({ type: "DESIGN.FOCUS_PIECE", kitGuid, designGuid, pieceGuid });
        },
        selectModelTag: (typeGuid: Guid, tagGuid: Guid) => {
            actor.send({ type: "DESIGN.SELECT_MODEL_TAG", kitGuid, designGuid, typeGuid, tagGuid });
        },
        deselectModelTag: (typeGuid: Guid, tagGuid: Guid) => {
            actor.send({ type: "DESIGN.DESELECT_MODEL_TAG", kitGuid, designGuid, typeGuid, tagGuid });
        },
        setDiagramCenter: (center: { x: number; y: number }) => {
            actor.send({ type: "DESIGN.SET_DIAGRAM_CENTER", kitGuid, designGuid, center });
        },
        setDiagramScale: (scale: number) => {
            actor.send({ type: "DESIGN.SET_DIAGRAM_SCALE", kitGuid, designGuid, scale });
        },
        setCamera: (camera: any) => {
            actor.send({ type: "DESIGN.SET_CAMERA", kitGuid, designGuid, camera });
        },
        selectAll: () => {
            actor.send({ type: "DESIGN.SELECT_ALL", kitGuid, designGuid });
        },
        deleteSelected: () => {
            actor.send({ type: "DESIGN.DELETE_SELECTED", kitGuid, designGuid });
        },
    }), [actor, kitGuid, designGuid]);
}

// #endregion Design App Hooks

// #region Utility Hooks

/**
 * Check if a piece is selected.
 */
export function useIsPieceSelected(kitGuid: Guid, designGuid: Guid, pieceGuid: Guid): boolean {
    const selection = useDesignSelection(kitGuid, designGuid);
    return selection?.pieces?.includes(pieceGuid) ?? false;
}

/**
 * Check if a piece is hovered.
 */
export function useIsPieceHovered(kitGuid: Guid, designGuid: Guid, pieceGuid: Guid): boolean {
    const hover = useDesignHover(kitGuid, designGuid);
    return hover?.pieces?.includes(pieceGuid) ?? false;
}

/**
 * Check if a connection is selected.
 */
export function useIsConnectionSelected(kitGuid: Guid, designGuid: Guid, connectionGuid: Guid): boolean {
    const selection = useDesignSelection(kitGuid, designGuid);
    return selection?.connections?.includes(connectionGuid) ?? false;
}

/**
 * Check if a connection is hovered.
 */
export function useIsConnectionHovered(kitGuid: Guid, designGuid: Guid, connectionGuid: Guid): boolean {
    const hover = useDesignHover(kitGuid, designGuid);
    return hover?.connections?.includes(connectionGuid) ?? false;
}

/**
 * Check if a port is selected in type app.
 */
export function useIsPortSelected(kitGuid: Guid, typeGuid: Guid, portGuid: Guid): boolean {
    const selection = useTypeSelection(kitGuid, typeGuid);
    return selection?.ports?.includes(portGuid) ?? false;
}

/**
 * Check if a port is hovered in type app.
 */
export function useIsPortHovered(kitGuid: Guid, typeGuid: Guid, portGuid: Guid): boolean {
    const hover = useTypeHover(kitGuid, typeGuid);
    return hover?.port === portGuid;
}

// #endregion Utility Hooks

// #region Sketchpad Global State Hooks

/**
 * Get the current navigation path.
 */
export function useSketchpadNavigation(): string {
    const actor = useActor();
    return useSelector(actor, selectSketchpadNavigation);
}

/**
 * Get the current theme.
 */
export function useSketchpadTheme() {
    const actor = useActor();
    return useSelector(actor, selectSketchpadTheme);
}

/**
 * Get the current language.
 */
export function useSketchpadLanguage(): string {
    const actor = useActor();
    return useSelector(actor, selectSketchpadLanguage);
}

/**
 * Get the current expertise level.
 */
export function useSketchpadExpertise() {
    const actor = useActor();
    return useSelector(actor, selectSketchpadExpertise);
}

/**
 * Get the current mode.
 */
export function useSketchpadMode() {
    const actor = useActor();
    return useSelector(actor, selectSketchpadMode);
}

/**
 * Get the current layout.
 */
export function useSketchpadLayout() {
    const actor = useActor();
    return useSelector(actor, selectSketchpadLayout);
}

/**
 * Get whether fullscreen is active.
 */
export function useSketchpadIsFullscreen(): boolean {
    const actor = useActor();
    return useSelector(actor, selectSketchpadIsFullscreen);
}

/**
 * Get the panel sizes.
 */
export function useSketchpadPanelSizes() {
    const actor = useActor();
    return useSelector(actor, selectSketchpadPanelSizes);
}

/**
 * Get the navigation history.
 */
export function useSketchpadNavigationHistory(): string[] {
    const actor = useActor();
    return useSelector(actor, selectSketchpadNavigationHistory);
}

/**
 * Get the navigation history index.
 */
export function useSketchpadNavigationHistoryIndex(): number {
    const actor = useActor();
    return useSelector(actor, selectSketchpadNavigationHistoryIndex);
}

/**
 * Get the settings.
 */
export function useSketchpadSettings() {
    const actor = useActor();
    return useSelector(actor, selectSketchpadSettings);
}

/**
 * Commands for the sketchpad.
 */
export function useSketchpadActorCommands() {
    const actor = useActor();

    return useMemo(() => ({
        navigate: (path: string) => {
            actor.send({ type: "NAVIGATE", path });
        },
        navigateBack: () => {
            actor.send({ type: "NAVIGATE_BACK" });
        },
        navigateForward: () => {
            actor.send({ type: "NAVIGATE_FORWARD" });
        },
        setTheme: (theme: any) => {
            actor.send({ type: "SET_THEME", theme });
        },
        setLanguage: (language: string) => {
            actor.send({ type: "SET_LANGUAGE", language });
        },
        setExpertise: (expertise: any) => {
            actor.send({ type: "SET_EXPERTISE", expertise });
        },
        setMode: (mode: any) => {
            actor.send({ type: "SET_MODE", mode });
        },
        setLayout: (layout: any) => {
            actor.send({ type: "SET_LAYOUT", layout });
        },
        toggleFullscreen: () => {
            actor.send({ type: "TOGGLE_FULLSCREEN" });
        },
        setPanelSize: (panel: string, size: number) => {
            actor.send({ type: "SET_PANEL_SIZE", panel: panel as any, size });
        },
    }), [actor]);
}

// #endregion Sketchpad Global State Hooks

// #region Transaction Hooks

/**
 * Check if a transaction is active for an app.
 */
export function useTransactionIsActive(appKey: string): boolean {
    const actor = useActor();
    const selector = useMemo(() => createTransactionIsActiveSelector(appKey), [appKey]);
    return useSelector(actor, selector);
}

/**
 * Check if undo is available for an app.
 */
export function useTransactionCanUndo(appKey: string): boolean {
    const actor = useActor();
    const selector = useMemo(() => createTransactionCanUndoSelector(appKey), [appKey]);
    return useSelector(actor, selector);
}

/**
 * Check if redo is available for an app.
 */
export function useTransactionCanRedo(appKey: string): boolean {
    const actor = useActor();
    const selector = useMemo(() => createTransactionCanRedoSelector(appKey), [appKey]);
    return useSelector(actor, selector);
}

/**
 * Transaction commands for an app.
 */
export function useTransactionCommands(appKey: string) {
    const actor = useActor();

    return useMemo(() => ({
        start: () => {
            actor.send({ type: "TRANSACTION.START", appKey });
        },
        commit: () => {
            actor.send({ type: "TRANSACTION.COMMIT", appKey });
        },
        abort: () => {
            actor.send({ type: "TRANSACTION.ABORT", appKey });
        },
        undo: () => {
            actor.send({ type: "TRANSACTION.UNDO", appKey });
        },
        redo: () => {
            actor.send({ type: "TRANSACTION.REDO", appKey });
        },
    }), [actor, appKey]);
}

// #endregion Transaction Hooks

// #region Re-exports

// Re-export selectors from machines.ts for use in app files
export {
    createDesignActiveToolSelector,
    createDesignCameraSelector,
    createDesignDiagramCenterSelector,
    createDesignDiagramScaleSelector,
    createDesignFocusedPieceSelector,
    createDesignFullscreenWindowSelector,
    createDesignHoverSelector,
    createDesignPanelVisibilitySelector,
    createDesignSelectedModelTagsSelector,
    createDesignSelectionSelector,
    createKitAppSelector,
    createKitExpandedRowsSelector,
    createKitFilterSearchSelector,
    createKitHoverSelector,
    createKitPanelVisibilitySelector,
    createKitSelectionSelector,
    createTypeActiveToolSelector,
    createTypeAppSelector,
    createTypeCameraSelector,
    createTypeFocusedPortSelector,
    createTypeHoverSelector,
    createTypePanelVisibilitySelector,
    createTypeSelectedModelTagsSelector,
    createTypeSelectionSelector
};

// Re-export types from machines.ts
export type {
    DesignAppHover,
    DesignAppSelection,
    DesignAppState,
    KitAppSelection,
    KitAppState,
    SketchpadActorRef, TypeAppHover,
    TypeAppSelection,
    TypeAppState
};

// #endregion Re-exports
