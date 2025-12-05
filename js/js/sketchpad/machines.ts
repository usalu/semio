// #region Header

// machines.ts - Unified XState v5 machine for Sketchpad
// All app state in one machine. Y.js only for Kit data sync.

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
    Theme,
    ToolKind
} from "./shared";

// #endregion Imports

// #region Types

// Default panel visibility
export const defaultPanelVisibility: PanelVisibility = {
    toolbar: false,
    workbench: false,
    details: false,
    chat: false,
    settings: false,
};

// #region App State Types

export interface HomeAppSelection { kits?: Guid[]; }
export interface HomeAppState {
    panelVisibility: PanelVisibility;
    selection?: HomeAppSelection;
    hover?: { kits?: Guid[] };
    sortColumn?: string;
    sortDirection?: "asc" | "desc";
    loadingKits: Array<{ tempGuid: string; name: string }>;
}

export interface KitAppSelection { types?: Guid[]; designs?: Guid[]; qualities?: Guid[]; files?: Guid[]; authors?: Guid[]; }
export interface KitAppState {
    panelVisibility: PanelVisibility;
    selection?: KitAppSelection;
    hover?: any;
    filterSearch?: string;
    expandedRows: Set<string>;
    sortColumn?: string;
    sortDirection?: "asc" | "desc";
}

export interface TypeAppSelection { ports?: Guid[]; models?: Guid[]; }
export interface TypeAppState {
    panelVisibility: PanelVisibility;
    selection?: TypeAppSelection;
    hover?: any;
    focusedPort?: Guid;
    selectedModelTags: Guid[];
    camera?: { position: { x: number; y: number; z: number }; target: { x: number; y: number; z: number } };
}

export interface DesignAppSelection { pieces?: Guid[]; connections?: Guid[]; ports?: Array<{ piece: Guid; port: Guid }>; }
export interface DesignAppHover { pieces?: Guid[]; connections?: Guid[]; ports?: Array<{ piece: Guid; port: Guid }>; types?: Guid[]; designs?: Guid[]; }
export enum DesignAppFullscreenWindow {
    None = "none",
    Diagram = "diagram",
    Accessl = "accessl",
}
export interface DesignAppState {
    panelVisibility: PanelVisibility;
    selection?: DesignAppSelection;
    hover?: DesignAppHover;
    focusedPiece?: Guid;
    selectedModelTags: Record<Guid, Guid[]>;
    diagramCenter?: { x: number; y: number };
    diagramScale?: number;
    camera?: any;
    activeTool?: ToolKind;
    fullscreenWindow?: DesignAppFullscreenWindow;
}

export interface QualityAppSelection { benchmarks?: Guid[]; }
export interface QualityAppState {
    panelVisibility: PanelVisibility;
    selection?: QualityAppSelection;
    hover?: any;
    expandedBenchmarks: Set<string>;
}

// Tutorial types
export interface TutorialStep {
    id: string;
    title: string;
    description?: string;
    target?: string;
    action?: string;
    completed?: boolean;
}

export interface TutorialContext {
    activeTutorial?: string;
    currentStepIndex: number;
    steps: TutorialStep[];
    completedSteps: Set<string>;
    isRecording: boolean;
    recordingState: "idle" | "recording" | "paused";
    recordedEvents: any[];
}

// #endregion App State Types

/**
 * Input for creating the sketchpad machine
 */
export interface SketchpadMachineInput {
    yDoc: Y.Doc;
    ySketchpad: Y.Map<any>;
    id?: string;
}

/**
 * Unified context for the sketchpad machine.
 * Contains all app state - Y.js only for Kit data sync.
 */
export interface SketchpadContext {
    // Y.js references for sketchpad settings and kit sync
    yDoc: Y.Doc;
    ySketchpad: Y.Map<any>;
    id?: string;
    cache?: SketchpadState;
    dirty: boolean;

    // Kit actors for Y.js data sync
    kits: Record<Guid, AnyActorRef>;

    // All app state is pure in-memory XState
    homeApp: HomeAppState;
    kitApps: Record<Guid, KitAppState>;
    typeApps: Record<string, TypeAppState>; // key: `${kitGuid}:${typeGuid}`
    designApps: Record<string, DesignAppState>; // key: `${kitGuid}:${designGuid}`
    qualityApps: Record<string, QualityAppState>; // key: `${kitGuid}:${qualityGuid}`
    tutorial: TutorialContext;

    // Transaction state (per active app)
    transactions: Record<string, {
        isActive: boolean;
        currentStack: any[];
        pastStack: any[];
        redoStack: any[];
    }>;
}

/**
 * Events for the sketchpad machine - unified event type for all app state
 */
export type SketchpadEvent =
    // Global sketchpad events
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
    | { type: "Y_UPDATE"; data: any }
    // Home app events
    | { type: "HOME.TOGGLE_PANEL"; panel: keyof PanelVisibility }
    | { type: "HOME.SET_SORT"; column: string; direction: "asc" | "desc" }
    | { type: "HOME.SELECT_KIT"; guid: Guid }
    | { type: "HOME.DESELECT_KIT"; guid: Guid }
    | { type: "HOME.SET_HOVER"; kits?: Guid[] }
    | { type: "HOME.CLEAR_HOVER" }
    // Kit app events (scoped by kitGuid)
    | { type: "KIT.TOGGLE_PANEL"; kitGuid: Guid; panel: keyof PanelVisibility }
    | { type: "KIT.SET_FILTER"; kitGuid: Guid; search: string }
    | { type: "KIT.TOGGLE_ROW"; kitGuid: Guid; rowId: string }
    | { type: "KIT.SET_SORT"; kitGuid: Guid; column: string; direction: "asc" | "desc" }
    // Type app events (scoped by kitGuid:typeGuid)
    | { type: "TYPE.INIT"; kitGuid: Guid; typeGuid: Guid; state: TypeAppState }
    | { type: "TYPE.SYNC"; kitGuid: Guid; typeGuid: Guid; state: Partial<TypeAppState> }
    | { type: "TYPE.TOGGLE_PANEL"; kitGuid: Guid; typeGuid: Guid; panel: keyof PanelVisibility }
    | { type: "TYPE.FOCUS_PORT"; kitGuid: Guid; typeGuid: Guid; portGuid?: Guid }
    | { type: "TYPE.SELECT_MODEL_TAG"; kitGuid: Guid; typeGuid: Guid; tagGuid: Guid }
    | { type: "TYPE.DESELECT_MODEL_TAG"; kitGuid: Guid; typeGuid: Guid; tagGuid: Guid }
    | { type: "TYPE.SET_CAMERA"; kitGuid: Guid; typeGuid: Guid; camera: any }
    // Design app events (scoped by kitGuid:designGuid)
    | { type: "DESIGN.INIT"; kitGuid: Guid; designGuid: Guid; state: DesignAppState }
    | { type: "DESIGN.SYNC"; kitGuid: Guid; designGuid: Guid; state: Partial<DesignAppState> }
    | { type: "DESIGN.TOGGLE_PANEL"; kitGuid: Guid; designGuid: Guid; panel: keyof PanelVisibility }
    | { type: "DESIGN.SET_ACTIVE_TOOL"; kitGuid: Guid; designGuid: Guid; tool: ToolKind }
    | { type: "DESIGN.SET_FULLSCREEN"; kitGuid: Guid; designGuid: Guid; window: DesignAppFullscreenWindow }
    | { type: "DESIGN.SELECT_PIECE"; kitGuid: Guid; designGuid: Guid; pieceGuid: Guid }
    | { type: "DESIGN.DESELECT_PIECE"; kitGuid: Guid; designGuid: Guid; pieceGuid: Guid }
    | { type: "DESIGN.SELECT_CONNECTION"; kitGuid: Guid; designGuid: Guid; connectionGuid: Guid }
    | { type: "DESIGN.DESELECT_CONNECTION"; kitGuid: Guid; designGuid: Guid; connectionGuid: Guid }
    | { type: "DESIGN.SET_SELECTION"; kitGuid: Guid; designGuid: Guid; selection: DesignAppSelection }
    | { type: "DESIGN.CLEAR_SELECTION"; kitGuid: Guid; designGuid: Guid }
    | { type: "DESIGN.SET_HOVER"; kitGuid: Guid; designGuid: Guid; hover: DesignAppHover }
    | { type: "DESIGN.CLEAR_HOVER"; kitGuid: Guid; designGuid: Guid }
    | { type: "DESIGN.FOCUS_PIECE"; kitGuid: Guid; designGuid: Guid; pieceGuid?: Guid }
    | { type: "DESIGN.SELECT_MODEL_TAG"; kitGuid: Guid; designGuid: Guid; typeGuid: Guid; tagGuid: Guid }
    | { type: "DESIGN.DESELECT_MODEL_TAG"; kitGuid: Guid; designGuid: Guid; typeGuid: Guid; tagGuid: Guid }
    | { type: "DESIGN.SET_DIAGRAM_CENTER"; kitGuid: Guid; designGuid: Guid; center: { x: number; y: number } }
    | { type: "DESIGN.SET_DIAGRAM_SCALE"; kitGuid: Guid; designGuid: Guid; scale: number }
    | { type: "DESIGN.SET_CAMERA"; kitGuid: Guid; designGuid: Guid; camera: any }
    // Quality app events (scoped by kitGuid:qualityGuid)
    | { type: "QUALITY.TOGGLE_PANEL"; kitGuid: Guid; qualityGuid: Guid; panel: keyof PanelVisibility }
    | { type: "QUALITY.TOGGLE_BENCHMARK"; kitGuid: Guid; qualityGuid: Guid; benchmarkGuid: Guid }
    // Tutorial events
    | { type: "TUTORIAL.START"; tutorialId: string; steps: TutorialStep[] }
    | { type: "TUTORIAL.END" }
    | { type: "TUTORIAL.NEXT_STEP" }
    | { type: "TUTORIAL.PREV_STEP" }
    | { type: "TUTORIAL.GO_TO_STEP"; index: number }
    | { type: "TUTORIAL.COMPLETE_STEP"; stepId: string };

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

/**
 * Create default design app state
 */
function createDefaultDesignAppState(): DesignAppState {
    return {
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        focusedPiece: undefined,
        selectedModelTags: {},
        diagramCenter: undefined,
        diagramScale: undefined,
        camera: undefined,
        activeTool: undefined,
        fullscreenWindow: undefined,
    };
}

/**
 * Create default type app state
 */
function createDefaultTypeAppState(): TypeAppState {
    return {
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        focusedPort: undefined,
        selectedModelTags: [],
        camera: undefined,
    };
}

/**
 * Create default kit app state
 */
function createDefaultKitAppState(): KitAppState {
    return {
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        filterSearch: undefined,
        expandedRows: new Set<string>(),
        sortColumn: undefined,
        sortDirection: undefined,
    };
}

/**
 * Create default quality app state
 */
function createDefaultQualityAppState(): QualityAppState {
    return {
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        expandedBenchmarks: new Set<string>(),
    };
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
        // Home app actions
        homeTogglePanel: assign(({ context, event }) => {
            if (event.type !== "HOME.TOGGLE_PANEL") return {};
            return {
                homeApp: {
                    ...context.homeApp,
                    panelVisibility: {
                        ...context.homeApp.panelVisibility,
                        [event.panel]: !context.homeApp.panelVisibility[event.panel],
                    },
                },
            };
        }),
        homeSetSort: assign(({ context, event }) => {
            if (event.type !== "HOME.SET_SORT") return {};
            return {
                homeApp: { ...context.homeApp, sortColumn: event.column, sortDirection: event.direction },
            };
        }),
        homeSelectKit: assign(({ context, event }) => {
            if (event.type !== "HOME.SELECT_KIT") return {};
            const kits = context.homeApp.selection?.kits || [];
            if (kits.includes(event.guid)) return {};
            return {
                homeApp: { ...context.homeApp, selection: { kits: [...kits, event.guid] } },
            };
        }),
        homeDeselectKit: assign(({ context, event }) => {
            if (event.type !== "HOME.DESELECT_KIT") return {};
            const kits = context.homeApp.selection?.kits || [];
            return {
                homeApp: { ...context.homeApp, selection: { kits: kits.filter((k: Guid) => k !== event.guid) } },
            };
        }),
        homeSetHover: assign(({ context, event }) => {
            if (event.type !== "HOME.SET_HOVER") return {};
            return { homeApp: { ...context.homeApp, hover: { kits: event.kits } } };
        }),
        homeClearHover: assign(({ context }) => ({
            homeApp: { ...context.homeApp, hover: undefined },
        })),
        // Type app INIT/SYNC actions
        typeInit: assign(({ context, event }) => {
            if (event.type !== "TYPE.INIT") return {};
            const key = `${event.kitGuid}:${event.typeGuid}`;
            return { typeApps: { ...context.typeApps, [key]: event.state } };
        }),
        typeSync: assign(({ context, event }) => {
            if (event.type !== "TYPE.SYNC") return {};
            const key = `${event.kitGuid}:${event.typeGuid}`;
            const app = context.typeApps[key] || createDefaultTypeAppState();
            return { typeApps: { ...context.typeApps, [key]: { ...app, ...event.state } } };
        }),
        // Design app INIT/SYNC actions
        designInit: assign(({ context, event }) => {
            if (event.type !== "DESIGN.INIT") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            return { designApps: { ...context.designApps, [key]: event.state } };
        }),
        designSync: assign(({ context, event }) => {
            if (event.type !== "DESIGN.SYNC") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, ...event.state } } };
        }),
        // Design app state actions
        designSetActiveTool: assign(({ context, event }) => {
            if (event.type !== "DESIGN.SET_ACTIVE_TOOL") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, activeTool: event.tool } } };
        }),
        designSetFullscreen: assign(({ context, event }) => {
            if (event.type !== "DESIGN.SET_FULLSCREEN") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, fullscreenWindow: event.window } } };
        }),
        // Design app actions (most important for the migration)
        designTogglePanel: assign(({ context, event }) => {
            if (event.type !== "DESIGN.TOGGLE_PANEL") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return {
                designApps: {
                    ...context.designApps,
                    [key]: {
                        ...app,
                        panelVisibility: { ...app.panelVisibility, [event.panel]: !app.panelVisibility[event.panel] },
                    },
                },
            };
        }),
        designSetSelection: assign(({ context, event }) => {
            if (event.type !== "DESIGN.SET_SELECTION") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, selection: event.selection } } };
        }),
        designClearSelection: assign(({ context, event }) => {
            if (event.type !== "DESIGN.CLEAR_SELECTION") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, selection: undefined } } };
        }),
        designSetHover: assign(({ context, event }) => {
            if (event.type !== "DESIGN.SET_HOVER") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, hover: event.hover } } };
        }),
        designClearHover: assign(({ context, event }) => {
            if (event.type !== "DESIGN.CLEAR_HOVER") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, hover: undefined } } };
        }),
        designFocusPiece: assign(({ context, event }) => {
            if (event.type !== "DESIGN.FOCUS_PIECE") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, focusedPiece: event.pieceGuid } } };
        }),
        designSetDiagramCenter: assign(({ context, event }) => {
            if (event.type !== "DESIGN.SET_DIAGRAM_CENTER") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, diagramCenter: event.center } } };
        }),
        designSetDiagramScale: assign(({ context, event }) => {
            if (event.type !== "DESIGN.SET_DIAGRAM_SCALE") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, diagramScale: event.scale } } };
        }),
        designSetCamera: assign(({ context, event }) => {
            if (event.type !== "DESIGN.SET_CAMERA") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            return { designApps: { ...context.designApps, [key]: { ...app, camera: event.camera } } };
        }),
        designSelectModelTag: assign(({ context, event }) => {
            if (event.type !== "DESIGN.SELECT_MODEL_TAG") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            const tags = app.selectedModelTags[event.typeGuid] || [];
            if (tags.includes(event.tagGuid)) return {};
            return {
                designApps: {
                    ...context.designApps,
                    [key]: {
                        ...app,
                        selectedModelTags: { ...app.selectedModelTags, [event.typeGuid]: [...tags, event.tagGuid] },
                    },
                },
            };
        }),
        designDeselectModelTag: assign(({ context, event }) => {
            if (event.type !== "DESIGN.DESELECT_MODEL_TAG") return {};
            const key = `${event.kitGuid}:${event.designGuid}`;
            const app = context.designApps[key] || createDefaultDesignAppState();
            const tags = app.selectedModelTags[event.typeGuid] || [];
            return {
                designApps: {
                    ...context.designApps,
                    [key]: {
                        ...app,
                        selectedModelTags: { ...app.selectedModelTags, [event.typeGuid]: tags.filter((g: Guid) => g !== event.tagGuid) },
                    },
                },
            };
        }),
        // Type app actions
        typeTogglePanel: assign(({ context, event }) => {
            if (event.type !== "TYPE.TOGGLE_PANEL") return {};
            const key = `${event.kitGuid}:${event.typeGuid}`;
            const app = context.typeApps[key] || createDefaultTypeAppState();
            return {
                typeApps: {
                    ...context.typeApps,
                    [key]: {
                        ...app,
                        panelVisibility: { ...app.panelVisibility, [event.panel]: !app.panelVisibility[event.panel] },
                    },
                },
            };
        }),
        typeFocusPort: assign(({ context, event }) => {
            if (event.type !== "TYPE.FOCUS_PORT") return {};
            const key = `${event.kitGuid}:${event.typeGuid}`;
            const app = context.typeApps[key] || createDefaultTypeAppState();
            return { typeApps: { ...context.typeApps, [key]: { ...app, focusedPort: event.portGuid } } };
        }),
        typeSelectModelTag: assign(({ context, event }) => {
            if (event.type !== "TYPE.SELECT_MODEL_TAG") return {};
            const key = `${event.kitGuid}:${event.typeGuid}`;
            const app = context.typeApps[key] || createDefaultTypeAppState();
            const tags = app.selectedModelTags || [];
            if (tags.includes(event.tagGuid)) return {};
            return {
                typeApps: {
                    ...context.typeApps,
                    [key]: { ...app, selectedModelTags: [...tags, event.tagGuid] },
                },
            };
        }),
        typeDeselectModelTag: assign(({ context, event }) => {
            if (event.type !== "TYPE.DESELECT_MODEL_TAG") return {};
            const key = `${event.kitGuid}:${event.typeGuid}`;
            const app = context.typeApps[key] || createDefaultTypeAppState();
            const tags = app.selectedModelTags || [];
            return {
                typeApps: {
                    ...context.typeApps,
                    [key]: { ...app, selectedModelTags: tags.filter((g: Guid) => g !== event.tagGuid) },
                },
            };
        }),
        typeSetCamera: assign(({ context, event }) => {
            if (event.type !== "TYPE.SET_CAMERA") return {};
            const key = `${event.kitGuid}:${event.typeGuid}`;
            const app = context.typeApps[key] || createDefaultTypeAppState();
            return { typeApps: { ...context.typeApps, [key]: { ...app, camera: event.camera } } };
        }),
        // Kit app actions
        kitTogglePanel: assign(({ context, event }) => {
            if (event.type !== "KIT.TOGGLE_PANEL") return {};
            const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
            return {
                kitApps: {
                    ...context.kitApps,
                    [event.kitGuid]: {
                        ...app,
                        panelVisibility: { ...app.panelVisibility, [event.panel]: !app.panelVisibility[event.panel] },
                    },
                },
            };
        }),
        kitSetFilter: assign(({ context, event }) => {
            if (event.type !== "KIT.SET_FILTER") return {};
            const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
            return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, filterSearch: event.search } } };
        }),
        kitToggleRow: assign(({ context, event }) => {
            if (event.type !== "KIT.TOGGLE_ROW") return {};
            const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
            const expanded = new Set(app.expandedRows);
            if (expanded.has(event.rowId)) {
                expanded.delete(event.rowId);
            } else {
                expanded.add(event.rowId);
            }
            return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, expandedRows: expanded } } };
        }),
        kitSetSort: assign(({ context, event }) => {
            if (event.type !== "KIT.SET_SORT") return {};
            const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
            return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, sortColumn: event.column, sortDirection: event.direction } } };
        }),
        // Quality app actions
        qualityTogglePanel: assign(({ context, event }) => {
            if (event.type !== "QUALITY.TOGGLE_PANEL") return {};
            const key = `${event.kitGuid}:${event.qualityGuid}`;
            const app = context.qualityApps[key] || createDefaultQualityAppState();
            return {
                qualityApps: {
                    ...context.qualityApps,
                    [key]: {
                        ...app,
                        panelVisibility: { ...app.panelVisibility, [event.panel]: !app.panelVisibility[event.panel] },
                    },
                },
            };
        }),
        qualityToggleBenchmark: assign(({ context, event }) => {
            if (event.type !== "QUALITY.TOGGLE_BENCHMARK") return {};
            const key = `${event.kitGuid}:${event.qualityGuid}`;
            const app = context.qualityApps[key] || createDefaultQualityAppState();
            const expanded = new Set(app.expandedBenchmarks);
            if (expanded.has(event.benchmarkGuid)) {
                expanded.delete(event.benchmarkGuid);
            } else {
                expanded.add(event.benchmarkGuid);
            }
            return { qualityApps: { ...context.qualityApps, [key]: { ...app, expandedBenchmarks: expanded } } };
        }),
        // Tutorial actions
        tutorialStart: assign(({ context, event }) => {
            if (event.type !== "TUTORIAL.START") return {};
            return {
                tutorial: {
                    ...context.tutorial,
                    activeTutorial: event.tutorialId,
                    steps: event.steps,
                    currentStepIndex: 0,
                },
            };
        }),
        tutorialEnd: assign(({ context }) => ({
            tutorial: {
                ...context.tutorial,
                activeTutorial: undefined,
                steps: [],
                currentStepIndex: 0,
            },
        })),
        tutorialNextStep: assign(({ context }) => ({
            tutorial: {
                ...context.tutorial,
                currentStepIndex: Math.min(context.tutorial.currentStepIndex + 1, context.tutorial.steps.length - 1),
            },
        })),
        tutorialPrevStep: assign(({ context }) => ({
            tutorial: {
                ...context.tutorial,
                currentStepIndex: Math.max(context.tutorial.currentStepIndex - 1, 0),
            },
        })),
        tutorialGoToStep: assign(({ context, event }) => {
            if (event.type !== "TUTORIAL.GO_TO_STEP") return {};
            return {
                tutorial: {
                    ...context.tutorial,
                    currentStepIndex: Math.max(0, Math.min(event.index, context.tutorial.steps.length - 1)),
                },
            };
        }),
        tutorialCompleteStep: assign(({ context, event }) => {
            if (event.type !== "TUTORIAL.COMPLETE_STEP") return {};
            const completed = new Set(context.tutorial.completedSteps);
            completed.add(event.stepId);
            return {
                tutorial: { ...context.tutorial, completedSteps: completed },
            };
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
        // All app state is pure in-memory XState
        homeApp: {
            panelVisibility: defaultPanelVisibility,
            selection: undefined,
            hover: undefined,
            sortColumn: undefined,
            sortDirection: undefined,
            loadingKits: [],
        },
        kitApps: {},
        typeApps: {},
        designApps: {},
        qualityApps: {},
        tutorial: {
            activeTutorial: undefined,
            currentStepIndex: 0,
            steps: [],
            completedSteps: new Set<string>(),
            isRecording: false,
            recordingState: "idle" as const,
            recordedEvents: [],
        },
        transactions: {},
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
        // Home app events
        "HOME.TOGGLE_PANEL": { actions: "homeTogglePanel" },
        "HOME.SET_SORT": { actions: "homeSetSort" },
        "HOME.SELECT_KIT": { actions: "homeSelectKit" },
        "HOME.DESELECT_KIT": { actions: "homeDeselectKit" },
        "HOME.SET_HOVER": { actions: "homeSetHover" },
        "HOME.CLEAR_HOVER": { actions: "homeClearHover" },
        // Design app events
        "DESIGN.INIT": { actions: "designInit" },
        "DESIGN.SYNC": { actions: "designSync" },
        "DESIGN.TOGGLE_PANEL": { actions: "designTogglePanel" },
        "DESIGN.SET_ACTIVE_TOOL": { actions: "designSetActiveTool" },
        "DESIGN.SET_FULLSCREEN": { actions: "designSetFullscreen" },
        "DESIGN.SET_SELECTION": { actions: "designSetSelection" },
        "DESIGN.CLEAR_SELECTION": { actions: "designClearSelection" },
        "DESIGN.SET_HOVER": { actions: "designSetHover" },
        "DESIGN.CLEAR_HOVER": { actions: "designClearHover" },
        "DESIGN.FOCUS_PIECE": { actions: "designFocusPiece" },
        "DESIGN.SET_DIAGRAM_CENTER": { actions: "designSetDiagramCenter" },
        "DESIGN.SET_DIAGRAM_SCALE": { actions: "designSetDiagramScale" },
        "DESIGN.SET_CAMERA": { actions: "designSetCamera" },
        "DESIGN.SELECT_MODEL_TAG": { actions: "designSelectModelTag" },
        "DESIGN.DESELECT_MODEL_TAG": { actions: "designDeselectModelTag" },
        // Type app events
        "TYPE.INIT": { actions: "typeInit" },
        "TYPE.SYNC": { actions: "typeSync" },
        "TYPE.TOGGLE_PANEL": { actions: "typeTogglePanel" },
        "TYPE.FOCUS_PORT": { actions: "typeFocusPort" },
        "TYPE.SELECT_MODEL_TAG": { actions: "typeSelectModelTag" },
        "TYPE.DESELECT_MODEL_TAG": { actions: "typeDeselectModelTag" },
        "TYPE.SET_CAMERA": { actions: "typeSetCamera" },
        // Kit app events
        "KIT.TOGGLE_PANEL": { actions: "kitTogglePanel" },
        "KIT.SET_FILTER": { actions: "kitSetFilter" },
        "KIT.TOGGLE_ROW": { actions: "kitToggleRow" },
        "KIT.SET_SORT": { actions: "kitSetSort" },
        // Quality app events
        "QUALITY.TOGGLE_PANEL": { actions: "qualityTogglePanel" },
        "QUALITY.TOGGLE_BENCHMARK": { actions: "qualityToggleBenchmark" },
        // Tutorial events
        "TUTORIAL.START": { actions: "tutorialStart" },
        "TUTORIAL.END": { actions: "tutorialEnd" },
        "TUTORIAL.NEXT_STEP": { actions: "tutorialNextStep" },
        "TUTORIAL.PREV_STEP": { actions: "tutorialPrevStep" },
        "TUTORIAL.GO_TO_STEP": { actions: "tutorialGoToStep" },
        "TUTORIAL.COMPLETE_STEP": { actions: "tutorialCompleteStep" },
    },
});

// #region Sketchpad Selectors

/**
 * Selectors for accessing unified state from the sketchpadMachine.
 * Use these with useSelector(actor, selector) in React components.
 */

// Home app selectors
export const selectHomeApp = (state: { context: SketchpadContext }) => state.context.homeApp;
export const selectHomePanelVisibility = (state: { context: SketchpadContext }) => state.context.homeApp.panelVisibility;
export const selectHomeSelection = (state: { context: SketchpadContext }) => state.context.homeApp.selection;
export const selectHomeHover = (state: { context: SketchpadContext }) => state.context.homeApp.hover;
export const selectHomeSortColumn = (state: { context: SketchpadContext }) => state.context.homeApp.sortColumn;
export const selectHomeSortDirection = (state: { context: SketchpadContext }) => state.context.homeApp.sortDirection;
export const selectHomeLoadingKits = (state: { context: SketchpadContext }) => state.context.homeApp.loadingKits;

// Design app selectors (take kitGuid and designGuid as curried parameters)
export const createDesignAppSelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) => state.context.designApps[key] || createDefaultDesignAppState();
};

export const createDesignPanelVisibilitySelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.designApps[key]?.panelVisibility ?? defaultPanelVisibility;
};

export const createDesignSelectionSelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.designApps[key]?.selection;
};

export const createDesignHoverSelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.designApps[key]?.hover;
};

export const createDesignFocusedPieceSelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.designApps[key]?.focusedPiece;
};

export const createDesignSelectedModelTagsSelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.designApps[key]?.selectedModelTags ?? {};
};

export const createDesignDiagramCenterSelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.designApps[key]?.diagramCenter;
};

export const createDesignDiagramScaleSelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.designApps[key]?.diagramScale;
};

export const createDesignCameraSelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.designApps[key]?.camera;
};

export const createDesignActiveToolSelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.designApps[key]?.activeTool;
};

export const createDesignFullscreenWindowSelector = (kitGuid: Guid, designGuid: Guid) => {
    const key = `${kitGuid}:${designGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.designApps[key]?.fullscreenWindow;
};

// Type app selectors
export const createTypeAppSelector = (kitGuid: Guid, typeGuid: Guid) => {
    const key = `${kitGuid}:${typeGuid}`;
    return (state: { context: SketchpadContext }) => {
        const app = state.context.typeApps[key];
        if (!app) {
            return {
                panelVisibility: defaultPanelVisibility,
                selection: undefined,
                hover: undefined,
                focusedPort: undefined,
                selectedModelTags: [],
                camera: undefined,
            } as TypeAppState;
        }
        return app;
    };
};

export const createTypePanelVisibilitySelector = (kitGuid: Guid, typeGuid: Guid) => {
    const key = `${kitGuid}:${typeGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.typeApps[key]?.panelVisibility ?? defaultPanelVisibility;
};

export const createTypeSelectionSelector = (kitGuid: Guid, typeGuid: Guid) => {
    const key = `${kitGuid}:${typeGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.typeApps[key]?.selection;
};

export const createTypeFocusedPortSelector = (kitGuid: Guid, typeGuid: Guid) => {
    const key = `${kitGuid}:${typeGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.typeApps[key]?.focusedPort;
};

export const createTypeSelectedModelTagsSelector = (kitGuid: Guid, typeGuid: Guid) => {
    const key = `${kitGuid}:${typeGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.typeApps[key]?.selectedModelTags ?? [];
};

export const createTypeCameraSelector = (kitGuid: Guid, typeGuid: Guid) => {
    const key = `${kitGuid}:${typeGuid}`;
    return (state: { context: SketchpadContext }) =>
        state.context.typeApps[key]?.camera;
};

// Quality app selectors
export const createQualityAppSelector = (kitGuid: Guid, qualityGuid: Guid) => {
    const key = `${kitGuid}:${qualityGuid}`;
    return (state: { context: SketchpadContext }) => {
        const app = state.context.qualityApps[key];
        if (!app) {
            return {
                panelVisibility: defaultPanelVisibility,
                selection: undefined,
                hover: undefined,
                expandedBenchmarks: new Set<string>(),
            } as QualityAppState;
        }
        return app;
    };
};

// Tutorial selectors
export const selectTutorial = (state: { context: SketchpadContext }) => state.context.tutorial;
export const selectActiveTutorial = (state: { context: SketchpadContext }) => state.context.tutorial.activeTutorial;
export const selectTutorialCurrentStep = (state: { context: SketchpadContext }) => state.context.tutorial.currentStepIndex;
export const selectTutorialSteps = (state: { context: SketchpadContext }) => state.context.tutorial.steps;

// Sketchpad global selectors
export const selectSketchpadCache = (state: { context: SketchpadContext }) => state.context.cache;
export const selectSketchpadDirty = (state: { context: SketchpadContext }) => state.context.dirty;
export const selectSketchpadKits = (state: { context: SketchpadContext }) => state.context.kits;

// #endregion Sketchpad Selectors

// #endregion Sketchpad Machine

// #region Factory

/**
 * Create a sketchpad actor - the single unified actor for all app state.
 * All old separate machines have been consolidated into this single machine.
 */
export function createSketchpadActor(input: SketchpadMachineInput) {
    return createActor(sketchpadMachine, { input });
}

// #endregion Factory

// #region Legacy Type Exports

// These types are kept for backwards compatibility with code that imports them.

export interface TransactionContext<TEdit = any> {
    isTransactionActive: boolean;
    currentTransactionStack: TEdit[];
    pastTransactionsStack: TEdit[];
    redoStack: TEdit[];
    lastDeletedEdit?: TEdit;
}

export interface AppMachineInput<TId = any> {
    id?: TId;
}

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

export interface KitMachineInput {
    yDoc: Y.Doc;
    yKit: Y.Map<any>;
    guid: Guid;
    local?: boolean;
    remote?: boolean;
}

export interface KitContext {
    yDoc: Y.Doc;
    yKit: Y.Map<any>;
    guid: Guid;
    local: boolean;
    remote: boolean;
    dirty: boolean;
    cache?: Kit;
}

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

// Legacy selectors
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

export function selectSnapshot(context: SketchpadContext): SketchpadState {
    if (!context.dirty && context.cache) {
        return context.cache;
    }
    return buildSnapshot(context.ySketchpad);
}

export function selectNavigation(context: SketchpadContext): string {
    return migratePath((context.ySketchpad.get("navigation") as string) || "/");
}

export function selectTheme(context: SketchpadContext): Theme {
    return context.ySketchpad.get("theme") as Theme;
}

export function selectLanguage(context: SketchpadContext): string {
    return (context.ySketchpad.get("language") as string) || "en";
}

export function selectExpertise(context: SketchpadContext): Expertise {
    return (context.ySketchpad.get("expertise") as Expertise) ?? Expertise.BEGINNER;
}

export function selectMode(context: SketchpadContext): Mode {
    return (context.ySketchpad.get("mode") as Mode) ?? Mode.USER;
}

export function selectLayout(context: SketchpadContext): Layout {
    const layoutStr = context.ySketchpad.get("layout") as string;
    return layoutStr ? JSON.parse(layoutStr) : "desktop";
}

export function selectIsFullscreen(context: SketchpadContext): boolean {
    return (context.ySketchpad.get("isFullscreen") as boolean) || false;
}

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

// #endregion Legacy Type Exports

// NOTE: All old separate machines (kitMachine, transactionMachine, homeAppMachine,
// kitAppMachine, typeAppMachine, designAppMachine, qualityAppMachine, tutorialMachine)
// have been consolidated into the unified sketchpadMachine above.
// Use createSketchpadActor() to create the single actor for all app state.

// END OF FILE MARKER - Everything below this line has been removed
// The old machines are no longer needed as all state is managed by sketchpadMachine
