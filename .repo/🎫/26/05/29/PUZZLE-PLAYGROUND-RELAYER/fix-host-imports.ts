#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";

const boardHostHeader = `/** @emoji 🛝 Board play React host — entry-only via play/main.ts. */
import {
	Button,
	ContextMenuController,
	IconSelector,
	Label,
	Input,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Slider,
	LevelProvider,
	getLevelBgClass,
	useElementsSurfaceChrome,
	useNativeDragAndDrop,
	type ContextMenuItem,
	type ElementsSurfaceDevice,
	type ElementsSurfaceTheme,
	type TreeDataSection,
} from "@semio-tech/ui-react";
import { Expertise, ProductRuntime, type FooterItem } from "@semio-tech/framework-playground-core";
import {
	PlaygroundView,
	mountPlaygroundApp,
	registerTabIcon,
	registerUiBoardSurfaceHost,
	registerWindowBody,
	type UiBoardHostSurfaceNode,
} from "@semio-tech/framework-playground-core-renderer-react";
import { ClipboardList, Library, ListTree, Settings } from "lucide-react";
import React, {
	createContext,
	useCallback,
	useContext,
	useEffect,
	useMemo,
	useRef,
	useState,
	useSyncExternalStore,
	type ReactElement,
	type ReactNode,
} from "react";
import * as Board from "../react/index.tsx";
import {
	BOARD_PLAY_APP_ID,
	BOARD_PLAY_BOARD_SURFACE_ID,
	BOARD_PLAY_BODY_KEY_DETAIL,
	BOARD_PLAY_BODY_KEY_OVERVIEW,
	BOARD_PLAY_BODY_KEY_SELECTION,
	BOARD_PLAY_CONTROLLER_ID,
	BOARD_PLAY_DEFAULT_FIXTURE,
	BOARD_PLAY_HIERARCHY_TAB_ID,
	BoardPlayShellController,
	buildBoardPlayHierarchySections,
	buildBoardPlayOverviewDeclarativeBody,
	buildBoardPlayDetailDeclarativeBody,
	buildBoardPlaySelectionDeclarativeBody,
	buildBoardPlayRuntime,
	type BoardPlayPaneId,
} from "./index.ts";

const {
	BoardCanvas,
	Node,
	Handle,
	Edge,
	Wire,
	mergeBoardKindCatalogBundleByRowId,
	BOARD_DEFAULT_KIND_CATALOG_BUNDLE,
	boardFixtureMetaKindCatalogBundle,
	parseBoardFixtureV1,
	DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
	encodeBoardFixtureForDragV1,
	BOARD_FIXTURE_DRAG_V1_MIME,
	BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE,
	newBoardAuthoringId,
	type BoardFixtureV1,
	type BoardFixtureNodeV1,
	type BoardFixtureRectangleNodeV1,
	type BoardFixtureHandleV1,
	type BoardFixtureEdgeV1,
	type BoardFixtureDropDetail,
	type BoardDrawLodKind,
	type BoardLodModeKind,
	type BoardSelectionMethod,
	type BoardSelectionMode,
	type BoardSelectionTargets,
} = Board;

`;

const path2d = "c:/git/compose/puzzle/2d/play/host.tsx";
let h2 = readFileSync(path2d, "utf8");
h2 = h2.replace(/^\/\*\* @emoji 🛝 Board play[\s\S]*?\/\/ #region 🛝PlayHost\n/, "");
writeFileSync(path2d, boardHostHeader + h2);

const sceneHostHeader = `/** @emoji 🛝 Scene play React host — entry-only via play/main.ts. */
import { Expertise, type FooterItem } from "@semio-tech/framework-playground-core";
import {
	Button,
	Input,
	Label,
	LevelProvider,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	applyElementsSurfaceChrome,
	getLevelBgClass,
	type ElementsSurfaceDevice,
	type ElementsSurfaceTheme,
} from "@semio-tech/ui-react";
import { useGLTF } from "@react-three/drei";
import React, { useCallback, useMemo, useSyncExternalStore, type ReactElement } from "react";
import {
	registerTabIcon,
	registerUiScene3DSurfaceHost,
	registerWindowBody,
	useApp,
	type UiScene3DHostSurfaceNode,
} from "@semio-tech/framework-playground-core-renderer-react";
import { ClipboardList, ListTree, Settings, Tags } from "lucide-react";
import nakaginSceneFixtureJson from "./fixtures/nakagin-capsule-tower.scene.json";
import {
	PlaySceneCanvas,
	SceneObjectStateProvider,
	parseFixtureV1,
	applyConnectToSceneFixture,
	applyRelocateToSceneFixture,
	blockedVortexFullIdsFromAttractions,
	parseKindCatalogs,
	parseKindCompatibility,
	sceneLodCanvasProps,
	sliderValueFromLod,
	DEFAULT_MANUAL_LOD,
	type FixtureV1,
	type RelocatePayload,
} from "../react/index.tsx";
import {
	SCENE_PLAY_BODY_KEY,
	SCENE_PLAY_CONTROLLER_ID,
	SCENE_PLAY_EMPTY_SELECTION,
	SCENE_PLAY_ICON_HIERARCHY,
	SCENE_PLAY_ICON_INSPECTOR,
	SCENE_PLAY_ICON_KINDS,
	SCENE_PLAY_ICON_SETTINGS,
	SCENE_PLAY_SCENE_SURFACE_ID,
	ScenePlayShellController,
	buildScenePlayDeclarativeBody,
	setScenePlaySurfaceHostRegistrar,
	type ScenePlaySnapshot,
} from "./index.ts";

`;

const path3d = "c:/git/compose/puzzle/3d/play/host.tsx";
let h3 = readFileSync(path3d, "utf8");
h3 = h3.replace(/^\/\*\* @emoji 🛝 Scene play[\s\S]*?\/\/ #endregion 🧲Header\n\n/, "");
h3 = h3.replace(/^\/\*\* @emoji 🛝 Scene play[\s\S]*?\/\/ #region 🛝PlayHost\n/, "");
writeFileSync(path3d, sceneHostHeader + h3);

console.log("[fix-host] headers prepended");
