#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";

function extract2d(): void {
	const indexPath = "c:/git/compose/puzzle/2d/react/index.tsx";
	const c = readFileSync(indexPath, "utf8");
	const marker = "\nconst NAKAGIN_BOARD_PLAY_KIND_CATALOGS";
	const pos = c.indexOf(marker);
	if (pos < 0) {
		console.error("2d marker missing");
		process.exit(1);
	}
	const head = c.slice(0, pos).replace(
		/\nimport \{ Expertise[\s\S]*?from "\.\.\/play\/index\.ts";\n/,
		"\n",
	);
	const tail = c.slice(pos);
	const hostPath = "c:/git/compose/puzzle/2d/play/host.tsx";
	const hostHeader = `/** @emoji 🛝 Board play React host — imported only from play/main.ts. */
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
} from "@semio-tech/ui-react";
import { Expertise, ProductRuntime, type FooterItem } from "@semio-tech/framework-playground-core";
import {
	PlaygroundView,
	mountPlaygroundApp,
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
import * as BoardReact from "../react/index.tsx";
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
	nakaginBoardMarkers,
	BoardPaneChrome,
	BoardStructuralDeleteReporter,
	BoardPlayRedrawProgressReset,
	BOARD_LOD_MODE_AUTOMATIC,
	BOARD_PLAY_DEFAULT_NODE_SIZE_PX,
	BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE: _,
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
} = BoardReact as Record<string, unknown>;

`;
	writeFileSync(indexPath, head.trimEnd() + "\n");
	writeFileSync(hostPath, hostHeader + tail);
	console.log("[extract] 2d split");
}

extract2d();
