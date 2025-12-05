// #region Header

// machines.ts - XState v5 machine definitions for Sketchpad

// 2025 Ueli Saluz

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

// #endregion

// #region Imports

import { AnyActorRef, assign, createActor, fromCallback, setup } from "xstate";
import * as Y from "yjs";
import { Guid, Kit, KitDiff } from "../semio";
import {
    Expertise,
    Layout,
    Mode,
    PanelSizes,
    PanelVisibility,
    SketchpadDiff,
    SketchpadState,
    Theme
} from "./shared";

// #endregion Imports

// #region Types

/**
 * Input for creating the sketchpad machine
 */
export interface SketchpadMachineInput {
    yDoc: Y.Doc;
    ySketchpad: Y.Map<any>;
    id?: string;
}

/**
 * Context for the sketchpad machine
 */
export interface SketchpadContext {
    yDoc: Y.Doc;
    ySketchpad: Y.Map<any>;
    id?: string;
    /** Cached snapshot - invalidated on Y_UPDATE */
    cache?: SketchpadState;
    dirty: boolean;
    /** Map of kit guids to their actor refs */
    kits: Record<Guid, AnyActorRef>;
}

/**
 * Events for the sketchpad machine
 */
export type SketchpadEvent =
    | { type: "NAVIGATE"; path: string }
    | { type: "NAVIGATE_BACK" }
    | { type: "NAVIGATE_FORWARD" }
    | { type: "SET_THEME"; theme: Theme }
    | { type: "SET_LANGUAGE"; language: string }
    | { type: "SET_EXPERTISE"; expertise: Expertise }
    | { type: "SET_MODE"; mode: Mode }
    | { type: "SET_LAYOUT"; layout: Layout }
    | { type: "TOGGLE_FULLSCREEN" }
    | { type: "SET_PANEL_SIZE"; panel: keyof PanelSizes; size: number }
    | { type: "CREATE_KIT"; kit: Kit; local?: boolean; remote?: boolean }
    | { type: "DELETE_KIT"; guid: Guid }
    | { type: "CHANGE"; diff: SketchpadDiff }
    | { type: "Y_UPDATE"; data: any };

// #endregion Types

// #region Helpers

/**
 * Path migration helper (matches existing implementation)
 */
function migratePath(path: string): string {
    // Remove any leading double slashes
    return path.replace(/^\/+/, "/");
}

/**
 * Build snapshot from Y.js data
 */
function buildSnapshot(ySketchpad: Y.Map<any>): SketchpadState {
    const settingsStr = ySketchpad.get("settings") as string;
    const settings = settingsStr
        ? JSON.parse(settingsStr)
        : {
            apps: {
                design: {
                    diagram: { proximityConnectDistance: 10 },
                    scene: { gridSize: 24 },
                },
            },
        };

    const panelSizesStr = ySketchpad.get("panelSizes") as string;
    const panelSizes = panelSizesStr
        ? JSON.parse(panelSizesStr)
        : {
            toolbarHeight: 52,
            workbenchWidth: 230,
            toolsWidth: 230,
            hudWidth: 230,
            statsWidth: 230,
            detailsWidth: 230,
            chatWidth: 230,
            settingsWidth: 230,
            consoleHeight: 200,
        };

    const navigationHistoryStr = ySketchpad.get("navigationHistory") as string;
    const navigationHistory = navigationHistoryStr
        ? JSON.parse(navigationHistoryStr).map(migratePath)
        : ["/"];

    const recentSearchesStr = ySketchpad.get("recentSearches") as string;
    const recentSearches = recentSearchesStr ? JSON.parse(recentSearchesStr) : [];

    const recentFocusItemsStr = ySketchpad.get("recentFocusItems") as string;
    const recentFocusItems = recentFocusItemsStr ? JSON.parse(recentFocusItemsStr) : {};

    const hotkeyOverridesStr = ySketchpad.get("hotkeyOverrides") as string;
    const hotkeyOverrides = hotkeyOverridesStr ? JSON.parse(hotkeyOverridesStr) : {};

    const layoutStr = ySketchpad.get("layout") as string;
    const layout: Layout = layoutStr ? JSON.parse(layoutStr) : "desktop";

    return {
        navigation: migratePath((ySketchpad.get("navigation") as string) || "/"),
        navigationHistory,
        navigationHistoryIndex: (ySketchpad.get("navigationHistoryIndex") as number) ?? 0,
        recentSearches,
        recentFocusItems,
        theme: ySketchpad.get("theme") as Theme,
        language: (ySketchpad.get("language") as string) || "en",
        layout,
        expertise: (ySketchpad.get("expertise") as Expertise) ?? Expertise.BEGINNER,
        mode: (ySketchpad.get("mode") as Mode) ?? Mode.USER,
        settings,
        panelSizes,
        isFullscreen: (ySketchpad.get("isFullscreen") as boolean) || false,
        isMobile: (ySketchpad.get("isMobile") as boolean) || false,
        activeInteraction: (ySketchpad.get("activeInteraction") as string) || undefined,
        hotkeyOverrides,
        activeHotkeySetting: (ySketchpad.get("activeHotkeySetting") as string) || undefined,
    };
}

/**
 * Apply diff to Y.js sketchpad map
 */
function applyDiff(yDoc: Y.Doc, ySketchpad: Y.Map<any>, diff: SketchpadDiff): void {
    yDoc.transact(() => {
        if (diff.navigationHistory !== undefined) {
            ySketchpad.set("navigationHistory", JSON.stringify(diff.navigationHistory));
        }
        if (diff.navigationHistoryIndex !== undefined) {
            ySketchpad.set("navigationHistoryIndex", diff.navigationHistoryIndex);
        }
        if (diff.navigation) {
            ySketchpad.set("navigation", diff.navigation);
        }
        if ("recentSearches" in diff) {
            ySketchpad.set("recentSearches", JSON.stringify(diff.recentSearches || []));
        }
        if ("recentFocusItems" in diff) {
            const current = JSON.parse((ySketchpad.get("recentFocusItems") as string) || "{}");
            ySketchpad.set("recentFocusItems", JSON.stringify({ ...current, ...(diff.recentFocusItems || {}) }));
        }
        if (diff.theme) ySketchpad.set("theme", diff.theme);
        if (diff.language !== undefined) {
            ySketchpad.set("language", diff.language);
        }
        if (diff.layout) ySketchpad.set("layout", JSON.stringify(diff.layout));
        if (diff.expertise) ySketchpad.set("expertise", diff.expertise);
        if (diff.mode) ySketchpad.set("mode", diff.mode);
        if (diff.isFullscreen !== undefined) ySketchpad.set("isFullscreen", diff.isFullscreen);
        if (diff.isMobile !== undefined) ySketchpad.set("isMobile", diff.isMobile);
        if ("activeInteraction" in diff) ySketchpad.set("activeInteraction", diff.activeInteraction || "");
        if (diff.settings) {
            const current = JSON.parse((ySketchpad.get("settings") as string) || "{}");
            const merged = { ...current, apps: { ...current.apps, ...diff.settings.apps } };
            ySketchpad.set("settings", JSON.stringify(merged));
        }
        if (diff.panelSizes) {
            const current = JSON.parse((ySketchpad.get("panelSizes") as string) || "{}");
            ySketchpad.set("panelSizes", JSON.stringify({ ...current, ...diff.panelSizes }));
        }
        if (diff.hotkeyOverrides) {
            const current = JSON.parse((ySketchpad.get("hotkeyOverrides") as string) || "{}");
            ySketchpad.set("hotkeyOverrides", JSON.stringify({ ...current, ...diff.hotkeyOverrides }));
        }
        if ("activeHotkeySetting" in diff) {
            ySketchpad.set("activeHotkeySetting", diff.activeHotkeySetting || "");
        }
    });
}

// #endregion Helpers

// #region Sketchpad Machine

/**
 * XState machine for the Sketchpad root store.
 * 
 * This machine:
 * - Observes Y.js changes via the yjsSync actor
 * - Handles navigation, theme, and settings changes
 * - Spawns kit actors for each kit
 * 
 * Y.js remains the source of truth - this machine provides:
 * - Structured event handling
 * - State machine logic
 * - React integration via @xstate/react
 */
export const sketchpadMachine = setup({
    types: {
        context: {} as SketchpadContext,
        events: {} as SketchpadEvent,
        input: {} as SketchpadMachineInput,
    },
    actions: {
        navigate: assign({
            dirty: () => true,
        }),
        navigateImpl: ({ context, event }) => {
            if (event.type !== "NAVIGATE") return;
            const { yDoc, ySketchpad } = context;
            const currentNav = ySketchpad.get("navigation") as string;
            const historyStr = ySketchpad.get("navigationHistory") as string;
            const history = historyStr ? JSON.parse(historyStr) : ["/"];
            const index = (ySketchpad.get("navigationHistoryIndex") as number) ?? 0;

            // Don't navigate if already there
            if (currentNav === event.path) return;

            // Truncate history at current index and add new path
            const newHistory = [...history.slice(0, index + 1), event.path];

            yDoc.transact(() => {
                ySketchpad.set("navigation", event.path);
                ySketchpad.set("navigationHistory", JSON.stringify(newHistory));
                ySketchpad.set("navigationHistoryIndex", newHistory.length - 1);
            });
        },
        navigateBack: ({ context }) => {
            const { yDoc, ySketchpad } = context;
            const historyStr = ySketchpad.get("navigationHistory") as string;
            const history = historyStr ? JSON.parse(historyStr) : ["/"];
            const index = (ySketchpad.get("navigationHistoryIndex") as number) ?? 0;

            if (index > 0) {
                const newIndex = index - 1;
                yDoc.transact(() => {
                    ySketchpad.set("navigation", history[newIndex]);
                    ySketchpad.set("navigationHistoryIndex", newIndex);
                });
            }
        },
        navigateForward: ({ context }) => {
            const { yDoc, ySketchpad } = context;
            const historyStr = ySketchpad.get("navigationHistory") as string;
            const history = historyStr ? JSON.parse(historyStr) : ["/"];
            const index = (ySketchpad.get("navigationHistoryIndex") as number) ?? 0;

            if (index < history.length - 1) {
                const newIndex = index + 1;
                yDoc.transact(() => {
                    ySketchpad.set("navigation", history[newIndex]);
                    ySketchpad.set("navigationHistoryIndex", newIndex);
                });
            }
        },
        setTheme: ({ context, event }) => {
            if (event.type !== "SET_THEME") return;
            context.yDoc.transact(() => {
                context.ySketchpad.set("theme", event.theme);
            });
        },
        setLanguage: ({ context, event }) => {
            if (event.type !== "SET_LANGUAGE") return;
            context.yDoc.transact(() => {
                context.ySketchpad.set("language", event.language);
            });
        },
        setExpertise: ({ context, event }) => {
            if (event.type !== "SET_EXPERTISE") return;
            context.yDoc.transact(() => {
                context.ySketchpad.set("expertise", event.expertise);
            });
        },
        setMode: ({ context, event }) => {
            if (event.type !== "SET_MODE") return;
            context.yDoc.transact(() => {
                context.ySketchpad.set("mode", event.mode);
            });
        },
        setLayout: ({ context, event }) => {
            if (event.type !== "SET_LAYOUT") return;
            context.yDoc.transact(() => {
                context.ySketchpad.set("layout", JSON.stringify(event.layout));
            });
        },
        toggleFullscreen: ({ context }) => {
            const current = context.ySketchpad.get("isFullscreen") as boolean;
            context.yDoc.transact(() => {
                context.ySketchpad.set("isFullscreen", !current);
            });
        },
        setPanelSize: ({ context, event }) => {
            if (event.type !== "SET_PANEL_SIZE") return;
            const currentStr = context.ySketchpad.get("panelSizes") as string;
            const current = currentStr ? JSON.parse(currentStr) : {};
            context.yDoc.transact(() => {
                context.ySketchpad.set("panelSizes", JSON.stringify({
                    ...current,
                    [event.panel]: event.size,
                }));
            });
        },
        applyChange: ({ context, event }) => {
            if (event.type !== "CHANGE") return;
            applyDiff(context.yDoc, context.ySketchpad, event.diff);
        },
        markDirty: assign({
            dirty: () => true,
            cache: () => undefined,
        }),
    },
    actors: {
        yjsSync: fromCallback<{ type: "Y_UPDATE"; data: any }, SketchpadMachineInput>(({ sendBack, input }) => {
            const observer = () => {
                sendBack({ type: "Y_UPDATE", data: input.ySketchpad.toJSON() });
            };

            // Send initial state
            observer();

            // Observe deep changes
            input.ySketchpad.observeDeep(observer);

            // Return cleanup function
            return () => {
                input.ySketchpad.unobserveDeep(observer);
            };
        }),
    },
}).createMachine({
    id: "sketchpad",
    context: ({ input }) => ({
        yDoc: input.yDoc,
        ySketchpad: input.ySketchpad,
        id: input.id,
        cache: undefined,
        dirty: true,
        kits: {},
    }),
    invoke: {
        id: "yjsSync",
        src: "yjsSync",
        input: ({ context }) => ({
            yDoc: context.yDoc,
            ySketchpad: context.ySketchpad,
            id: context.id,
        }),
    },
    on: {
        NAVIGATE: {
            actions: ["navigate", "navigateImpl"],
        },
        NAVIGATE_BACK: {
            actions: ["markDirty", "navigateBack"],
        },
        NAVIGATE_FORWARD: {
            actions: ["markDirty", "navigateForward"],
        },
        SET_THEME: {
            actions: ["markDirty", "setTheme"],
        },
        SET_LANGUAGE: {
            actions: ["markDirty", "setLanguage"],
        },
        SET_EXPERTISE: {
            actions: ["markDirty", "setExpertise"],
        },
        SET_MODE: {
            actions: ["markDirty", "setMode"],
        },
        SET_LAYOUT: {
            actions: ["markDirty", "setLayout"],
        },
        TOGGLE_FULLSCREEN: {
            actions: ["markDirty", "toggleFullscreen"],
        },
        SET_PANEL_SIZE: {
            actions: ["markDirty", "setPanelSize"],
        },
        CHANGE: {
            actions: ["markDirty", "applyChange"],
        },
        Y_UPDATE: {
            actions: "markDirty",
        },
    },
});

// #endregion Sketchpad Machine

// #region Transaction Machine

/**
 * Transaction machine for managing undo/redo stacks.
 * Can be composed into app machines.
 */
export interface TransactionContext<TEdit = any> {
    isTransactionActive: boolean;
    currentTransactionStack: TEdit[];
    pastTransactionsStack: TEdit[];
    redoStack: TEdit[];
    lastDeletedEdit?: TEdit;
}

export type TransactionEvent<TEdit = any> =
    | { type: "START_TRANSACTION" }
    | { type: "FINALIZE_TRANSACTION" }
    | { type: "ABORT_TRANSACTION" }
    | { type: "UNDO" }
    | { type: "REDO" }
    | { type: "RECORD_EDIT"; edit: TEdit };

export const transactionMachine = setup({
    types: {
        context: {} as TransactionContext,
        events: {} as TransactionEvent,
    },
    actions: {
        startTransaction: assign({
            isTransactionActive: () => true,
            currentTransactionStack: () => [],
        }),
        finalizeTransaction: assign(({ context }) => {
            if (context.currentTransactionStack.length === 0) {
                return {
                    isTransactionActive: false,
                };
            }

            // Merge edits into single transaction for history
            const edits = context.currentTransactionStack;
            const mergedEdit = edits.length === 1
                ? edits[0]
                : { do: edits[edits.length - 1].do, undo: edits[0].undo };

            return {
                isTransactionActive: false,
                currentTransactionStack: [],
                pastTransactionsStack: [...context.pastTransactionsStack, mergedEdit],
                redoStack: [],
            };
        }),
        abortTransaction: assign(({ context }) => ({
            isTransactionActive: false,
            currentTransactionStack: [],
            // Note: actual undo actions need to be handled by the parent machine
        })),
        recordEdit: assign(({ context, event }) => {
            if (event.type !== "RECORD_EDIT") return {};
            return {
                currentTransactionStack: [...context.currentTransactionStack, event.edit],
                redoStack: [],
                lastDeletedEdit: undefined,
            };
        }),
        undoInTransaction: assign(({ context }) => {
            if (context.currentTransactionStack.length === 0) return {};
            const edit = context.currentTransactionStack[context.currentTransactionStack.length - 1];
            return {
                currentTransactionStack: context.currentTransactionStack.slice(0, -1),
                lastDeletedEdit: edit,
            };
        }),
        undoFromPast: assign(({ context }) => {
            if (context.pastTransactionsStack.length === 0) return {};
            const edit = context.pastTransactionsStack[context.pastTransactionsStack.length - 1];
            return {
                pastTransactionsStack: context.pastTransactionsStack.slice(0, -1),
                redoStack: [...context.redoStack, edit],
            };
        }),
        redoInTransaction: assign(({ context }) => {
            if (!context.lastDeletedEdit) return {};
            return {
                currentTransactionStack: [...context.currentTransactionStack, context.lastDeletedEdit],
                lastDeletedEdit: undefined,
            };
        }),
        redoFromStack: assign(({ context }) => {
            if (context.redoStack.length === 0) return {};
            const edit = context.redoStack[context.redoStack.length - 1];
            return {
                redoStack: context.redoStack.slice(0, -1),
                pastTransactionsStack: [...context.pastTransactionsStack, edit],
            };
        }),
    },
}).createMachine({
    id: "transaction",
    initial: "idle",
    context: {
        isTransactionActive: false,
        currentTransactionStack: [],
        pastTransactionsStack: [],
        redoStack: [],
    },
    states: {
        idle: {
            on: {
                START_TRANSACTION: {
                    target: "active",
                    actions: "startTransaction",
                },
                UNDO: {
                    actions: "undoFromPast",
                },
                REDO: {
                    actions: "redoFromStack",
                },
            },
        },
        active: {
            on: {
                FINALIZE_TRANSACTION: {
                    target: "idle",
                    actions: "finalizeTransaction",
                },
                ABORT_TRANSACTION: {
                    target: "idle",
                    actions: "abortTransaction",
                },
                RECORD_EDIT: {
                    actions: "recordEdit",
                },
                UNDO: {
                    actions: "undoInTransaction",
                },
                REDO: {
                    actions: "redoInTransaction",
                },
            },
        },
    },
});

// #endregion Transaction Machine

// #region Selectors

/**
 * Selector to get the current sketchpad state snapshot
 */
export function selectSnapshot(context: SketchpadContext): SketchpadState {
    if (!context.dirty && context.cache) {
        return context.cache;
    }
    return buildSnapshot(context.ySketchpad);
}

/**
 * Selector to get navigation
 */
export function selectNavigation(context: SketchpadContext): string {
    return migratePath((context.ySketchpad.get("navigation") as string) || "/");
}

/**
 * Selector to get theme
 */
export function selectTheme(context: SketchpadContext): Theme {
    return context.ySketchpad.get("theme") as Theme;
}

/**
 * Selector to get language
 */
export function selectLanguage(context: SketchpadContext): string {
    return (context.ySketchpad.get("language") as string) || "en";
}

/**
 * Selector to get expertise level
 */
export function selectExpertise(context: SketchpadContext): Expertise {
    return (context.ySketchpad.get("expertise") as Expertise) ?? Expertise.BEGINNER;
}

/**
 * Selector to get mode
 */
export function selectMode(context: SketchpadContext): Mode {
    return (context.ySketchpad.get("mode") as Mode) ?? Mode.USER;
}

/**
 * Selector to get layout
 */
export function selectLayout(context: SketchpadContext): Layout {
    const layoutStr = context.ySketchpad.get("layout") as string;
    return layoutStr ? JSON.parse(layoutStr) : "desktop";
}

/**
 * Selector to get fullscreen state
 */
export function selectIsFullscreen(context: SketchpadContext): boolean {
    return (context.ySketchpad.get("isFullscreen") as boolean) || false;
}

/**
 * Selector to get panel sizes
 */
export function selectPanelSizes(context: SketchpadContext): PanelSizes {
    const panelSizesStr = context.ySketchpad.get("panelSizes") as string;
    return panelSizesStr
        ? JSON.parse(panelSizesStr)
        : {
            toolbarHeight: 52,
            workbenchWidth: 230,
            toolsWidth: 230,
            hudWidth: 230,
            statsWidth: 230,
            detailsWidth: 230,
            chatWidth: 230,
            settingsWidth: 230,
            consoleHeight: 200,
        };
}

// #endregion Selectors

// #region Kit Machine

/**
 * Input for creating the kit machine
 */
export interface KitMachineInput {
    yDoc: Y.Doc;
    yKit: Y.Map<any>;
    guid: Guid;
    local?: boolean;
    remote?: boolean;
}

/**
 * Context for the kit machine
 */
export interface KitContext {
    yDoc: Y.Doc;
    yKit: Y.Map<any>;
    guid: Guid;
    local: boolean;
    remote: boolean;
    dirty: boolean;
    cache?: Kit;
}

/**
 * Events for the kit machine
 */
export type KitEvent =
    | { type: "CHANGE"; diff: KitDiff }
    | { type: "CREATE_TYPE"; typeData: any }
    | { type: "UPDATE_TYPE"; guid: Guid; diff: any }
    | { type: "DELETE_TYPE"; guid: Guid }
    | { type: "CREATE_DESIGN"; design: any }
    | { type: "UPDATE_DESIGN"; guid: Guid; diff: any }
    | { type: "DELETE_DESIGN"; guid: Guid }
    | { type: "Y_UPDATE"; data: any }
    | { type: "MARK_DIRTY" };

/**
 * Build kit snapshot from Y.js data
 */
function buildKitSnapshot(yKit: Y.Map<any>): Partial<Kit> {
    return {
        guid: yKit.get("guid") as string,
        name: yKit.get("name") as string,
        version: yKit.get("version") as string | undefined,
        description: yKit.get("description") as string | undefined,
        homepage: yKit.get("homepage") as string | undefined,
        license: yKit.get("license") as string | undefined,
        icon: yKit.get("icon") as string | undefined,
        image: yKit.get("image") as string | undefined,
        createdAt: yKit.get("createdAt") as string | undefined,
        updatedAt: yKit.get("updatedAt") as string | undefined,
    };
}

/**
 * XState machine for a Kit store.
 */
export const kitMachine = setup({
    types: {
        context: {} as KitContext,
        events: {} as KitEvent,
        input: {} as KitMachineInput,
    },
    actions: {
        markDirty: assign({
            dirty: () => true,
            cache: () => undefined,
        }),
        applyChange: ({ context, event }) => {
            if (event.type !== "CHANGE") return;
            const { yDoc, yKit } = context;
            const diff = event.diff;

            yDoc.transact(() => {
                if (diff.name) yKit.set("name", diff.name);
                if (diff.version) yKit.set("version", diff.version);
                if (diff.description !== undefined) yKit.set("description", diff.description);
                if (diff.homepage !== undefined) yKit.set("homepage", diff.homepage);
                if (diff.license !== undefined) yKit.set("license", diff.license);
                yKit.set("updatedAt", new Date().toISOString());
            });
        },
    },
    actors: {
        yjsSync: fromCallback<{ type: "Y_UPDATE"; data: any }, KitMachineInput>(({ sendBack, input }) => {
            const observer = () => {
                sendBack({ type: "Y_UPDATE", data: input.yKit.toJSON() });
            };
            observer();
            input.yKit.observeDeep(observer);
            return () => {
                input.yKit.unobserveDeep(observer);
            };
        }),
    },
}).createMachine({
    id: "kit",
    context: ({ input }) => ({
        yDoc: input.yDoc,
        yKit: input.yKit,
        guid: input.guid,
        local: input.local ?? false,
        remote: input.remote ?? false,
        dirty: true,
        cache: undefined,
    }),
    invoke: {
        id: "yjsSync",
        src: "yjsSync",
        input: ({ context }) => ({
            yDoc: context.yDoc,
            yKit: context.yKit,
            guid: context.guid,
        }),
    },
    on: {
        CHANGE: {
            actions: ["markDirty", "applyChange"],
        },
        Y_UPDATE: {
            actions: "markDirty",
        },
        MARK_DIRTY: {
            actions: "markDirty",
        },
    },
});

// Kit selectors
export function selectKitGuid(context: KitContext): Guid {
    return context.yKit.get("guid") as Guid;
}

export function selectKitName(context: KitContext): string {
    return context.yKit.get("name") as string;
}

export function selectKitSnapshot(context: KitContext): Partial<Kit> {
    if (!context.dirty && context.cache) {
        return context.cache;
    }
    return buildKitSnapshot(context.yKit);
}

export function createKitActor(input: KitMachineInput) {
    return createActor(kitMachine, { input });
}

// #endregion Kit Machine

// #region App Machine Template

/**
 * Default panel visibility
 */
export const defaultPanelVisibility: PanelVisibility = {
    toolbar: false,
    workbench: false,
    details: false,
    chat: false,
    settings: false,
};

/**
 * Pure in-memory App Machine Input (no Y.js)
 * App state is managed entirely by XState, not persisted to Y.js
 */
export interface AppMachineInput<TId = any> {
    id?: TId;
}

/**
 * Pure in-memory App Machine Context (no Y.js)
 * All app state is managed in XState context, not Y.js
 */
export interface AppMachineContext<TSelection = any, TId = any> {
    id?: TId;
    panelVisibility: PanelVisibility;
    selection?: TSelection;
    hover?: any;
    isTransactionActive: boolean;
    currentTransactionStack: any[];
    pastTransactionsStack: any[];
    redoStack: any[];
}

/**
 * Pure in-memory App Machine Events (no Y.js)
 */
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
    | { type: "RECORD_EDIT"; edit: any };

/**
 * Create a pure in-memory app machine with transaction support.
 * No Y.js backing - all state is managed in XState context.
 */
export function createAppMachine<TSelection = any, TDiff = any, TSelectionDiff = any, TId = any>(
    machineId: string,
    initialPanelVisibility: PanelVisibility = defaultPanelVisibility,
) {
    return setup({
        types: {
            context: {} as AppMachineContext<TSelection, TId>,
            events: {} as AppMachineEvent<TSelectionDiff, TDiff>,
            input: {} as AppMachineInput<TId>,
        },
        actions: {
            togglePanel: assign(({ context, event }) => {
                if (event.type !== "TOGGLE_PANEL") return {};
                return {
                    panelVisibility: {
                        ...context.panelVisibility,
                        [event.panel]: !context.panelVisibility[event.panel],
                    },
                };
            }),
            startTransaction: assign({
                isTransactionActive: () => true,
                currentTransactionStack: () => [],
            }),
            finalizeTransaction: assign(({ context }) => {
                if (context.currentTransactionStack.length === 0) {
                    return { isTransactionActive: false };
                }
                return {
                    isTransactionActive: false,
                    currentTransactionStack: [],
                    pastTransactionsStack: [...context.pastTransactionsStack, ...context.currentTransactionStack],
                    redoStack: [],
                };
            }),
            abortTransaction: assign({
                isTransactionActive: () => false,
                currentTransactionStack: () => [],
            }),
            recordEdit: assign(({ context, event }) => {
                if (event.type !== "RECORD_EDIT") return {};
                return {
                    currentTransactionStack: [...context.currentTransactionStack, event.edit],
                    redoStack: [],
                };
            }),
            undoFromPast: assign(({ context }) => {
                if (context.pastTransactionsStack.length === 0) return {};
                const edit = context.pastTransactionsStack[context.pastTransactionsStack.length - 1];
                return {
                    pastTransactionsStack: context.pastTransactionsStack.slice(0, -1),
                    redoStack: [...context.redoStack, edit],
                };
            }),
            redoFromStack: assign(({ context }) => {
                if (context.redoStack.length === 0) return {};
                const edit = context.redoStack[context.redoStack.length - 1];
                return {
                    redoStack: context.redoStack.slice(0, -1),
                    pastTransactionsStack: [...context.pastTransactionsStack, edit],
                };
            }),
            clearHover: assign({ hover: () => undefined }),
            setHover: assign(({ event }) => {
                if (event.type !== "HOVER") return {};
                return { hover: event.data };
            }),
            deselect: assign({ selection: () => undefined }),
        },
    }).createMachine({
        id: machineId,
        initial: "idle",
        context: ({ input }) => ({
            id: input.id,
            panelVisibility: initialPanelVisibility,
            selection: undefined,
            hover: undefined,
            isTransactionActive: false,
            currentTransactionStack: [],
            pastTransactionsStack: [],
            redoStack: [],
        }),
        states: {
            idle: {
                on: {
                    START_TRANSACTION: { target: "transaction", actions: "startTransaction" },
                    UNDO: { actions: "undoFromPast" },
                    REDO: { actions: "redoFromStack" },
                },
            },
            transaction: {
                on: {
                    FINALIZE_TRANSACTION: { target: "idle", actions: "finalizeTransaction" },
                    ABORT_TRANSACTION: { target: "idle", actions: "abortTransaction" },
                    RECORD_EDIT: { actions: "recordEdit" },
                },
            },
        },
        on: {
            TOGGLE_PANEL: { actions: "togglePanel" },
            HOVER: { actions: "setHover" },
            CLEAR_HOVER: { actions: "clearHover" },
            DESELECT: { actions: "deselect" },
        },
    });
}

// #endregion App Machine Template

// #region Home App Machine

export interface HomeSelection {
    kits?: Guid[];
}

export interface HomeSelectionDiff {
    added?: Guid[];
    removed?: Guid[];
}

export type HomeSortColumn = "name" | "type" | "updatedAt" | "createdAt";
export type HomeSortDirection = "asc" | "desc";

export interface HomeAppContext extends AppMachineContext<HomeSelection, void> {
    sortColumn?: HomeSortColumn;
    sortDirection?: HomeSortDirection;
    loadingKits: { tempGuid: Guid; name: string }[];
}

export type HomeAppEvent = AppMachineEvent<HomeSelectionDiff>
    | { type: "SET_SORT"; column: HomeSortColumn; direction: HomeSortDirection }
    | { type: "ADD_LOADING_KIT"; kit: { tempGuid: Guid; name: string } }
    | { type: "REMOVE_LOADING_KIT"; tempGuid: Guid }
    | { type: "SELECT_KIT"; guid: Guid }
    | { type: "DESELECT_KIT"; guid: Guid };

export const homeAppMachine = setup({
    types: {
        context: {} as HomeAppContext,
        events: {} as HomeAppEvent,
        input: {} as AppMachineInput<void>,
    },
    actions: {
        togglePanel: assign(({ context, event }) => {
            if (event.type !== "TOGGLE_PANEL") return {};
            return { panelVisibility: { ...context.panelVisibility, [event.panel]: !context.panelVisibility[event.panel] } };
        }),
        setSort: assign(({ event }) => {
            if (event.type !== "SET_SORT") return {};
            return { sortColumn: event.column, sortDirection: event.direction };
        }),
        addLoadingKit: assign(({ context, event }) => {
            if (event.type !== "ADD_LOADING_KIT") return {};
            return { loadingKits: [...context.loadingKits, event.kit] };
        }),
        removeLoadingKit: assign(({ context, event }) => {
            if (event.type !== "REMOVE_LOADING_KIT") return {};
            return { loadingKits: context.loadingKits.filter(k => k.tempGuid !== event.tempGuid) };
        }),
        selectKit: assign(({ context, event }) => {
            if (event.type !== "SELECT_KIT") return {};
            const currentKits = context.selection?.kits || [];
            if (currentKits.includes(event.guid)) return {};
            return { selection: { kits: [...currentKits, event.guid] } };
        }),
        deselectKit: assign(({ context, event }) => {
            if (event.type !== "DESELECT_KIT") return {};
            const currentKits = context.selection?.kits || [];
            return { selection: { kits: currentKits.filter(k => k !== event.guid) } };
        }),
        startTransaction: assign({ isTransactionActive: () => true, currentTransactionStack: () => [] }),
        finalizeTransaction: assign(({ context }) => ({
            isTransactionActive: false,
            currentTransactionStack: [],
            pastTransactionsStack: context.currentTransactionStack.length > 0
                ? [...context.pastTransactionsStack, ...context.currentTransactionStack]
                : context.pastTransactionsStack,
            redoStack: [],
        })),
        abortTransaction: assign({ isTransactionActive: () => false, currentTransactionStack: () => [] }),
        recordEdit: assign(({ context, event }) => {
            if (event.type !== "RECORD_EDIT") return {};
            return { currentTransactionStack: [...context.currentTransactionStack, event.edit], redoStack: [] };
        }),
        undoFromPast: assign(({ context }) => {
            if (context.pastTransactionsStack.length === 0) return {};
            const edit = context.pastTransactionsStack[context.pastTransactionsStack.length - 1];
            return { pastTransactionsStack: context.pastTransactionsStack.slice(0, -1), redoStack: [...context.redoStack, edit] };
        }),
        redoFromStack: assign(({ context }) => {
            if (context.redoStack.length === 0) return {};
            const edit = context.redoStack[context.redoStack.length - 1];
            return { redoStack: context.redoStack.slice(0, -1), pastTransactionsStack: [...context.pastTransactionsStack, edit] };
        }),
    },
}).createMachine({
    id: "homeApp",
    initial: "idle",
    context: () => ({
        id: undefined,
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        isTransactionActive: false,
        currentTransactionStack: [],
        pastTransactionsStack: [],
        redoStack: [],
        sortColumn: undefined,
        sortDirection: undefined,
        loadingKits: [],
    }),
    states: {
        idle: {
            on: {
                START_TRANSACTION: { target: "transaction", actions: "startTransaction" },
                UNDO: { actions: "undoFromPast" },
                REDO: { actions: "redoFromStack" },
            },
        },
        transaction: {
            on: {
                FINALIZE_TRANSACTION: { target: "idle", actions: "finalizeTransaction" },
                ABORT_TRANSACTION: { target: "idle", actions: "abortTransaction" },
                RECORD_EDIT: { actions: "recordEdit" },
            },
        },
    },
    on: {
        TOGGLE_PANEL: { actions: "togglePanel" },
        SET_SORT: { actions: "setSort" },
        ADD_LOADING_KIT: { actions: "addLoadingKit" },
        REMOVE_LOADING_KIT: { actions: "removeLoadingKit" },
        SELECT_KIT: { actions: "selectKit" },
        DESELECT_KIT: { actions: "deselectKit" },
    },
});

// #endregion Home App Machine

// #region Kit App Machine

export interface KitAppSelection {
    types?: Guid[];
    designs?: Guid[];
    qualities?: Guid[];
    files?: Guid[];
    authors?: Guid[];
}

export interface KitAppContext extends AppMachineContext<KitAppSelection, { kit: Guid }> {
    filterSearch?: string;
    expandedRows: Set<string>;
    sortColumn?: string;
    sortDirection?: "asc" | "desc";
}

export type KitAppEvent = AppMachineEvent
    | { type: "SET_FILTER"; search: string }
    | { type: "TOGGLE_ROW"; rowId: string }
    | { type: "SET_SORT"; column: string; direction: "asc" | "desc" };

export const kitAppMachine = setup({
    types: {
        context: {} as KitAppContext,
        events: {} as KitAppEvent,
        input: {} as AppMachineInput<{ kit: Guid }>,
    },
    actions: {
        togglePanel: assign(({ context, event }) => {
            if (event.type !== "TOGGLE_PANEL") return {};
            return { panelVisibility: { ...context.panelVisibility, [event.panel]: !context.panelVisibility[event.panel] } };
        }),
        setFilter: assign(({ event }) => {
            if (event.type !== "SET_FILTER") return {};
            return { filterSearch: event.search };
        }),
        toggleRow: assign(({ context, event }) => {
            if (event.type !== "TOGGLE_ROW") return {};
            const newExpanded = new Set(context.expandedRows);
            if (newExpanded.has(event.rowId)) newExpanded.delete(event.rowId);
            else newExpanded.add(event.rowId);
            return { expandedRows: newExpanded };
        }),
        setSort: assign(({ event }) => {
            if (event.type !== "SET_SORT") return {};
            return { sortColumn: event.column, sortDirection: event.direction };
        }),
        startTransaction: assign({ isTransactionActive: () => true, currentTransactionStack: () => [] }),
        finalizeTransaction: assign(({ context }) => ({
            isTransactionActive: false,
            currentTransactionStack: [],
            pastTransactionsStack: context.currentTransactionStack.length > 0
                ? [...context.pastTransactionsStack, ...context.currentTransactionStack]
                : context.pastTransactionsStack,
            redoStack: [],
        })),
        abortTransaction: assign({ isTransactionActive: () => false, currentTransactionStack: () => [] }),
        recordEdit: assign(({ context, event }) => {
            if (event.type !== "RECORD_EDIT") return {};
            return { currentTransactionStack: [...context.currentTransactionStack, event.edit], redoStack: [] };
        }),
        undoFromPast: assign(({ context }) => {
            if (context.pastTransactionsStack.length === 0) return {};
            return { pastTransactionsStack: context.pastTransactionsStack.slice(0, -1), redoStack: [...context.redoStack, context.pastTransactionsStack[context.pastTransactionsStack.length - 1]] };
        }),
        redoFromStack: assign(({ context }) => {
            if (context.redoStack.length === 0) return {};
            return { redoStack: context.redoStack.slice(0, -1), pastTransactionsStack: [...context.pastTransactionsStack, context.redoStack[context.redoStack.length - 1]] };
        }),
    },
}).createMachine({
    id: "kitApp",
    initial: "idle",
    context: ({ input }) => ({
        id: input.id,
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        isTransactionActive: false,
        currentTransactionStack: [],
        pastTransactionsStack: [],
        redoStack: [],
        filterSearch: undefined,
        expandedRows: new Set<string>(),
        sortColumn: undefined,
        sortDirection: undefined,
    }),
    states: {
        idle: {
            on: {
                START_TRANSACTION: { target: "transaction", actions: "startTransaction" },
                UNDO: { actions: "undoFromPast" },
                REDO: { actions: "redoFromStack" },
            },
        },
        transaction: {
            on: {
                FINALIZE_TRANSACTION: { target: "idle", actions: "finalizeTransaction" },
                ABORT_TRANSACTION: { target: "idle", actions: "abortTransaction" },
                RECORD_EDIT: { actions: "recordEdit" },
            },
        },
    },
    on: {
        TOGGLE_PANEL: { actions: "togglePanel" },
        SET_FILTER: { actions: "setFilter" },
        TOGGLE_ROW: { actions: "toggleRow" },
        SET_SORT: { actions: "setSort" },
    },
});

// #endregion Kit App Machine

// #region Type App Machine

export interface TypeAppSelection {
    ports?: Guid[];
    models?: Guid[];
}

export interface TypeAppContext extends AppMachineContext<TypeAppSelection, { kit: Guid; type: Guid }> {
    focusedPort?: Guid;
    selectedModelTags: Guid[];
    camera?: { position: { x: number; y: number; z: number }; target: { x: number; y: number; z: number } };
}

export type TypeAppEvent = AppMachineEvent
    | { type: "FOCUS_PORT"; guid?: Guid }
    | { type: "SELECT_MODEL_TAG"; guid: Guid }
    | { type: "DESELECT_MODEL_TAG"; guid: Guid }
    | { type: "SET_CAMERA"; camera: any };

export const typeAppMachine = setup({
    types: {
        context: {} as TypeAppContext,
        events: {} as TypeAppEvent,
        input: {} as AppMachineInput<{ kit: Guid; type: Guid }>,
    },
    actions: {
        togglePanel: assign(({ context, event }) => {
            if (event.type !== "TOGGLE_PANEL") return {};
            return { panelVisibility: { ...context.panelVisibility, [event.panel]: !context.panelVisibility[event.panel] } };
        }),
        focusPort: assign(({ event }) => {
            if (event.type !== "FOCUS_PORT") return {};
            return { focusedPort: event.guid };
        }),
        selectModelTag: assign(({ context, event }) => {
            if (event.type !== "SELECT_MODEL_TAG") return {};
            if (context.selectedModelTags.includes(event.guid)) return {};
            return { selectedModelTags: [...context.selectedModelTags, event.guid] };
        }),
        deselectModelTag: assign(({ context, event }) => {
            if (event.type !== "DESELECT_MODEL_TAG") return {};
            return { selectedModelTags: context.selectedModelTags.filter(g => g !== event.guid) };
        }),
        setCamera: assign(({ event }) => {
            if (event.type !== "SET_CAMERA") return {};
            return { camera: event.camera };
        }),
        startTransaction: assign({ isTransactionActive: () => true, currentTransactionStack: () => [] }),
        finalizeTransaction: assign(({ context }) => ({
            isTransactionActive: false, currentTransactionStack: [],
            pastTransactionsStack: context.currentTransactionStack.length > 0 ? [...context.pastTransactionsStack, ...context.currentTransactionStack] : context.pastTransactionsStack,
            redoStack: [],
        })),
        abortTransaction: assign({ isTransactionActive: () => false, currentTransactionStack: () => [] }),
        recordEdit: assign(({ context, event }) => {
            if (event.type !== "RECORD_EDIT") return {};
            return { currentTransactionStack: [...context.currentTransactionStack, event.edit], redoStack: [] };
        }),
        undoFromPast: assign(({ context }) => {
            if (context.pastTransactionsStack.length === 0) return {};
            return { pastTransactionsStack: context.pastTransactionsStack.slice(0, -1), redoStack: [...context.redoStack, context.pastTransactionsStack[context.pastTransactionsStack.length - 1]] };
        }),
        redoFromStack: assign(({ context }) => {
            if (context.redoStack.length === 0) return {};
            return { redoStack: context.redoStack.slice(0, -1), pastTransactionsStack: [...context.pastTransactionsStack, context.redoStack[context.redoStack.length - 1]] };
        }),
    },
}).createMachine({
    id: "typeApp",
    initial: "idle",
    context: ({ input }) => ({
        id: input.id,
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        isTransactionActive: false,
        currentTransactionStack: [],
        pastTransactionsStack: [],
        redoStack: [],
        focusedPort: undefined,
        selectedModelTags: [],
        camera: undefined,
    }),
    states: {
        idle: {
            on: {
                START_TRANSACTION: { target: "transaction", actions: "startTransaction" },
                UNDO: { actions: "undoFromPast" },
                REDO: { actions: "redoFromStack" },
            },
        },
        transaction: {
            on: {
                FINALIZE_TRANSACTION: { target: "idle", actions: "finalizeTransaction" },
                ABORT_TRANSACTION: { target: "idle", actions: "abortTransaction" },
                RECORD_EDIT: { actions: "recordEdit" },
            },
        },
    },
    on: {
        TOGGLE_PANEL: { actions: "togglePanel" },
        FOCUS_PORT: { actions: "focusPort" },
        SELECT_MODEL_TAG: { actions: "selectModelTag" },
        DESELECT_MODEL_TAG: { actions: "deselectModelTag" },
        SET_CAMERA: { actions: "setCamera" },
    },
});

// #endregion Type App Machine

// #region Design App Machine

export interface DesignAppSelection {
    pieces?: Guid[];
    connections?: Guid[];
    ports?: Guid[];
}

export interface DesignAppContext extends AppMachineContext<DesignAppSelection, { kit: Guid; design: Guid }> {
    focusedPiece?: Guid;
    selectedModelTags: Record<Guid, Guid[]>;
    diagramCenter?: { x: number; y: number };
    diagramScale?: number;
    camera?: { position: { x: number; y: number; z: number }; target: { x: number; y: number; z: number } };
}

export type DesignAppEvent = AppMachineEvent
    | { type: "FOCUS_PIECE"; guid?: Guid }
    | { type: "SELECT_MODEL_TAG"; typeGuid: Guid; tagGuid: Guid }
    | { type: "DESELECT_MODEL_TAG"; typeGuid: Guid; tagGuid: Guid }
    | { type: "SET_DIAGRAM_CENTER"; center: { x: number; y: number } }
    | { type: "SET_DIAGRAM_SCALE"; scale: number }
    | { type: "SET_CAMERA"; camera: any }
    | { type: "SELECT_PIECE"; guid: Guid }
    | { type: "DESELECT_PIECE"; guid: Guid }
    | { type: "SELECT_CONNECTION"; guid: Guid }
    | { type: "DESELECT_CONNECTION"; guid: Guid };

export const designAppMachine = setup({
    types: {
        context: {} as DesignAppContext,
        events: {} as DesignAppEvent,
        input: {} as AppMachineInput<{ kit: Guid; design: Guid }>,
    },
    actions: {
        togglePanel: assign(({ context, event }) => {
            if (event.type !== "TOGGLE_PANEL") return {};
            return { panelVisibility: { ...context.panelVisibility, [event.panel]: !context.panelVisibility[event.panel] } };
        }),
        focusPiece: assign(({ event }) => {
            if (event.type !== "FOCUS_PIECE") return {};
            return { focusedPiece: event.guid };
        }),
        selectModelTag: assign(({ context, event }) => {
            if (event.type !== "SELECT_MODEL_TAG") return {};
            const currentTags = context.selectedModelTags[event.typeGuid] || [];
            if (currentTags.includes(event.tagGuid)) return {};
            return { selectedModelTags: { ...context.selectedModelTags, [event.typeGuid]: [...currentTags, event.tagGuid] } };
        }),
        deselectModelTag: assign(({ context, event }) => {
            if (event.type !== "DESELECT_MODEL_TAG") return {};
            const currentTags = context.selectedModelTags[event.typeGuid] || [];
            return { selectedModelTags: { ...context.selectedModelTags, [event.typeGuid]: currentTags.filter(g => g !== event.tagGuid) } };
        }),
        setDiagramCenter: assign(({ event }) => {
            if (event.type !== "SET_DIAGRAM_CENTER") return {};
            return { diagramCenter: event.center };
        }),
        setDiagramScale: assign(({ event }) => {
            if (event.type !== "SET_DIAGRAM_SCALE") return {};
            return { diagramScale: event.scale };
        }),
        setCamera: assign(({ event }) => {
            if (event.type !== "SET_CAMERA") return {};
            return { camera: event.camera };
        }),
        selectPiece: assign(({ context, event }) => {
            if (event.type !== "SELECT_PIECE") return {};
            const currentPieces = context.selection?.pieces || [];
            if (currentPieces.includes(event.guid)) return {};
            return { selection: { ...context.selection, pieces: [...currentPieces, event.guid] } };
        }),
        deselectPiece: assign(({ context, event }) => {
            if (event.type !== "DESELECT_PIECE") return {};
            const currentPieces = context.selection?.pieces || [];
            return { selection: { ...context.selection, pieces: currentPieces.filter(g => g !== event.guid) } };
        }),
        selectConnection: assign(({ context, event }) => {
            if (event.type !== "SELECT_CONNECTION") return {};
            const currentConnections = context.selection?.connections || [];
            if (currentConnections.includes(event.guid)) return {};
            return { selection: { ...context.selection, connections: [...currentConnections, event.guid] } };
        }),
        deselectConnection: assign(({ context, event }) => {
            if (event.type !== "DESELECT_CONNECTION") return {};
            const currentConnections = context.selection?.connections || [];
            return { selection: { ...context.selection, connections: currentConnections.filter(g => g !== event.guid) } };
        }),
        startTransaction: assign({ isTransactionActive: () => true, currentTransactionStack: () => [] }),
        finalizeTransaction: assign(({ context }) => ({
            isTransactionActive: false, currentTransactionStack: [],
            pastTransactionsStack: context.currentTransactionStack.length > 0 ? [...context.pastTransactionsStack, ...context.currentTransactionStack] : context.pastTransactionsStack,
            redoStack: [],
        })),
        abortTransaction: assign({ isTransactionActive: () => false, currentTransactionStack: () => [] }),
        recordEdit: assign(({ context, event }) => {
            if (event.type !== "RECORD_EDIT") return {};
            return { currentTransactionStack: [...context.currentTransactionStack, event.edit], redoStack: [] };
        }),
        undoFromPast: assign(({ context }) => {
            if (context.pastTransactionsStack.length === 0) return {};
            return { pastTransactionsStack: context.pastTransactionsStack.slice(0, -1), redoStack: [...context.redoStack, context.pastTransactionsStack[context.pastTransactionsStack.length - 1]] };
        }),
        redoFromStack: assign(({ context }) => {
            if (context.redoStack.length === 0) return {};
            return { redoStack: context.redoStack.slice(0, -1), pastTransactionsStack: [...context.pastTransactionsStack, context.redoStack[context.redoStack.length - 1]] };
        }),
    },
}).createMachine({
    id: "designApp",
    initial: "idle",
    context: ({ input }) => ({
        id: input.id,
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        isTransactionActive: false,
        currentTransactionStack: [],
        pastTransactionsStack: [],
        redoStack: [],
        focusedPiece: undefined,
        selectedModelTags: {},
        diagramCenter: undefined,
        diagramScale: undefined,
        camera: undefined,
    }),
    states: {
        idle: {
            on: {
                START_TRANSACTION: { target: "transaction", actions: "startTransaction" },
                UNDO: { actions: "undoFromPast" },
                REDO: { actions: "redoFromStack" },
            },
        },
        transaction: {
            on: {
                FINALIZE_TRANSACTION: { target: "idle", actions: "finalizeTransaction" },
                ABORT_TRANSACTION: { target: "idle", actions: "abortTransaction" },
                RECORD_EDIT: { actions: "recordEdit" },
            },
        },
    },
    on: {
        TOGGLE_PANEL: { actions: "togglePanel" },
        FOCUS_PIECE: { actions: "focusPiece" },
        SELECT_MODEL_TAG: { actions: "selectModelTag" },
        DESELECT_MODEL_TAG: { actions: "deselectModelTag" },
        SET_DIAGRAM_CENTER: { actions: "setDiagramCenter" },
        SET_DIAGRAM_SCALE: { actions: "setDiagramScale" },
        SET_CAMERA: { actions: "setCamera" },
        SELECT_PIECE: { actions: "selectPiece" },
        DESELECT_PIECE: { actions: "deselectPiece" },
        SELECT_CONNECTION: { actions: "selectConnection" },
        DESELECT_CONNECTION: { actions: "deselectConnection" },
    },
});

// #endregion Design App Machine

// #region Quality App Machine

export interface QualityAppSelection {
    benchmarks?: Guid[];
}

export interface QualityAppContext extends AppMachineContext<QualityAppSelection, { kit: Guid; quality: Guid }> {
    expandedBenchmarks: Set<string>;
}

export type QualityAppEvent = AppMachineEvent
    | { type: "TOGGLE_BENCHMARK"; guid: Guid };

export const qualityAppMachine = setup({
    types: {
        context: {} as QualityAppContext,
        events: {} as QualityAppEvent,
        input: {} as AppMachineInput<{ kit: Guid; quality: Guid }>,
    },
    actions: {
        togglePanel: assign(({ context, event }) => {
            if (event.type !== "TOGGLE_PANEL") return {};
            return { panelVisibility: { ...context.panelVisibility, [event.panel]: !context.panelVisibility[event.panel] } };
        }),
        toggleBenchmark: assign(({ context, event }) => {
            if (event.type !== "TOGGLE_BENCHMARK") return {};
            const newExpanded = new Set(context.expandedBenchmarks);
            if (newExpanded.has(event.guid)) newExpanded.delete(event.guid);
            else newExpanded.add(event.guid);
            return { expandedBenchmarks: newExpanded };
        }),
        startTransaction: assign({ isTransactionActive: () => true, currentTransactionStack: () => [] }),
        finalizeTransaction: assign(({ context }) => ({
            isTransactionActive: false, currentTransactionStack: [],
            pastTransactionsStack: context.currentTransactionStack.length > 0 ? [...context.pastTransactionsStack, ...context.currentTransactionStack] : context.pastTransactionsStack,
            redoStack: [],
        })),
        abortTransaction: assign({ isTransactionActive: () => false, currentTransactionStack: () => [] }),
        recordEdit: assign(({ context, event }) => {
            if (event.type !== "RECORD_EDIT") return {};
            return { currentTransactionStack: [...context.currentTransactionStack, event.edit], redoStack: [] };
        }),
        undoFromPast: assign(({ context }) => {
            if (context.pastTransactionsStack.length === 0) return {};
            return { pastTransactionsStack: context.pastTransactionsStack.slice(0, -1), redoStack: [...context.redoStack, context.pastTransactionsStack[context.pastTransactionsStack.length - 1]] };
        }),
        redoFromStack: assign(({ context }) => {
            if (context.redoStack.length === 0) return {};
            return { redoStack: context.redoStack.slice(0, -1), pastTransactionsStack: [...context.pastTransactionsStack, context.redoStack[context.redoStack.length - 1]] };
        }),
    },
}).createMachine({
    id: "qualityApp",
    initial: "idle",
    context: ({ input }) => ({
        id: input.id,
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        isTransactionActive: false,
        currentTransactionStack: [],
        pastTransactionsStack: [],
        redoStack: [],
        expandedBenchmarks: new Set<string>(),
    }),
    states: {
        idle: {
            on: {
                START_TRANSACTION: { target: "transaction", actions: "startTransaction" },
                UNDO: { actions: "undoFromPast" },
                REDO: { actions: "redoFromStack" },
            },
        },
        transaction: {
            on: {
                FINALIZE_TRANSACTION: { target: "idle", actions: "finalizeTransaction" },
                ABORT_TRANSACTION: { target: "idle", actions: "abortTransaction" },
                RECORD_EDIT: { actions: "recordEdit" },
            },
        },
    },
    on: {
        TOGGLE_PANEL: { actions: "togglePanel" },
        TOGGLE_BENCHMARK: { actions: "toggleBenchmark" },
    },
});

// #endregion Quality App Machine

// #region Tutorial Machine

export type TutorialStep = {
    id: string;
    title: string;
    description?: string;
    target?: string;
    action?: string;
};

export interface TutorialContext {
    activeTutorial?: string;
    currentStepIndex: number;
    steps: TutorialStep[];
    completedSteps: Set<string>;
    recordingState: "idle" | "recording" | "paused";
    recordedEvents: any[];
}

export type TutorialEvent =
    | { type: "START_TUTORIAL"; tutorialId: string; steps: TutorialStep[] }
    | { type: "END_TUTORIAL" }
    | { type: "NEXT_STEP" }
    | { type: "PREV_STEP" }
    | { type: "GO_TO_STEP"; index: number }
    | { type: "COMPLETE_STEP"; stepId: string }
    | { type: "START_RECORDING" }
    | { type: "STOP_RECORDING" }
    | { type: "PAUSE_RECORDING" }
    | { type: "RESUME_RECORDING" }
    | { type: "RECORD_EVENT"; event: any };

export const tutorialMachine = setup({
    types: {
        context: {} as TutorialContext,
        events: {} as TutorialEvent,
        input: {} as Record<string, never>,
    },
    actions: {
        startTutorial: assign(({ event }) => {
            if (event.type !== "START_TUTORIAL") return {};
            return { activeTutorial: event.tutorialId, steps: event.steps, currentStepIndex: 0, completedSteps: new Set<string>() };
        }),
        endTutorial: assign({ activeTutorial: () => undefined, steps: () => [], currentStepIndex: () => 0 }),
        nextStep: assign(({ context }) => ({ currentStepIndex: Math.min(context.currentStepIndex + 1, context.steps.length - 1) })),
        prevStep: assign(({ context }) => ({ currentStepIndex: Math.max(context.currentStepIndex - 1, 0) })),
        goToStep: assign(({ context, event }) => {
            if (event.type !== "GO_TO_STEP") return {};
            return { currentStepIndex: Math.max(0, Math.min(event.index, context.steps.length - 1)) };
        }),
        completeStep: assign(({ context, event }) => {
            if (event.type !== "COMPLETE_STEP") return {};
            const newCompleted = new Set(context.completedSteps);
            newCompleted.add(event.stepId);
            return { completedSteps: newCompleted };
        }),
        startRecording: assign({ recordingState: () => "recording" as const, recordedEvents: () => [] }),
        stopRecording: assign({ recordingState: () => "idle" as const }),
        pauseRecording: assign({ recordingState: () => "paused" as const }),
        resumeRecording: assign({ recordingState: () => "recording" as const }),
        recordEvent: assign(({ context, event }) => {
            if (event.type !== "RECORD_EVENT") return {};
            if (context.recordingState !== "recording") return {};
            return { recordedEvents: [...context.recordedEvents, event.event] };
        }),
    },
}).createMachine({
    id: "tutorial",
    initial: "inactive",
    context: () => ({
        activeTutorial: undefined,
        currentStepIndex: 0,
        steps: [],
        completedSteps: new Set<string>(),
        recordingState: "idle" as const,
        recordedEvents: [],
    }),
    states: {
        inactive: {
            on: {
                START_TUTORIAL: { target: "active", actions: "startTutorial" },
                START_RECORDING: { target: "recording", actions: "startRecording" },
            },
        },
        active: {
            on: {
                END_TUTORIAL: { target: "inactive", actions: "endTutorial" },
                NEXT_STEP: { actions: "nextStep" },
                PREV_STEP: { actions: "prevStep" },
                GO_TO_STEP: { actions: "goToStep" },
                COMPLETE_STEP: { actions: "completeStep" },
            },
        },
        recording: {
            on: {
                STOP_RECORDING: { target: "inactive", actions: "stopRecording" },
                PAUSE_RECORDING: { target: "recordingPaused", actions: "pauseRecording" },
                RECORD_EVENT: { actions: "recordEvent" },
            },
        },
        recordingPaused: {
            on: {
                RESUME_RECORDING: { target: "recording", actions: "resumeRecording" },
                STOP_RECORDING: { target: "inactive", actions: "stopRecording" },
            },
        },
    },
});

// #endregion Tutorial Machine

// #region Actor Factories

export function createHomeAppActor(input: AppMachineInput<void>) {
    return createActor(homeAppMachine, { input });
}

export function createKitAppActor(input: AppMachineInput<{ kit: Guid }>) {
    return createActor(kitAppMachine, { input });
}

export function createTypeAppActor(input: AppMachineInput<{ kit: Guid; type: Guid }>) {
    return createActor(typeAppMachine, { input });
}

export function createDesignAppActor(input: AppMachineInput<{ kit: Guid; design: Guid }>) {
    return createActor(designAppMachine, { input });
}

export function createQualityAppActor(input: AppMachineInput<{ kit: Guid; quality: Guid }>) {
    return createActor(qualityAppMachine, { input });
}

export function createTutorialActor() {
    return createActor(tutorialMachine, { input: {} });
}

// #endregion Actor Factories

// #region Factory

/**
 * Create a sketchpad actor
 */
export function createSketchpadActor(input: SketchpadMachineInput) {
    return createActor(sketchpadMachine, { input });
}

// #endregion Factory
