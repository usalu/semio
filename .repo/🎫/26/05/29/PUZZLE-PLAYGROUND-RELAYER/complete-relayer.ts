#!/usr/bin/env bun
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/semio";

function extract2d(): void {
	const indexPath = join(root, "puzzle/2d/react/index.tsx");
	let c = readFileSync(indexPath, "utf8");
	const marker = "\nconst NAKAGIN_BOARD_PLAY_KIND_CATALOGS";
	const pos = c.indexOf(marker);
	if (pos < 0) throw new Error("2d marker missing");
	const head = c
		.slice(0, pos)
		.replace(/\nimport \{ Expertise[\s\S]*?from "@framework\/playground";\n/, "\n")
		.replace(
			/\nimport \{\n\tPlaygroundView[\s\S]*?from "@framework\/playground-renderer-react";\n/,
			"\n",
		)
		.replace(/\nimport \{[\s\S]*?\} from "\.\.\/play\/index\.ts";\n/, "\n")
		.replace(/\nexport type \{ BoardPlayPaneId \} from "\.\/index\.ts";\n/g, "\n");
	const tail = c.slice(pos);
	const hostPath = join(root, "puzzle/2d/play/host.tsx");
	const hostHeader = `// #region 🧲Header
/** @emoji 🛝 Board play React host — entry-only; imported from play/main.ts. */
// #endregion 🧲Header

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
} from "@ui/react";
import { Expertise, ProductRuntime, type FooterItem } from "@framework/playground";
import {
	PlaygroundView,
	mountPlaygroundApp,
	registerTabIcon,
	registerUiBoardSurfaceHost,
	registerWindowBody,
	type UiBoardHostSurfaceNode,
} from "@framework/playground-renderer-react";
import { ClipboardList, Library, ListTree, Settings } from "lucide-react";
import {
	createContext,
	useCallback,
	useContext,
	useEffect,
	useMemo,
	useRef,
	useState,
	useSyncExternalStore,
	type ReactNode,
} from "react";
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
	setBoardPlaySurfaceHostRegistrar,
	type BoardPlayPaneId,
} from "./index.ts";
import * as Board from "../react/index.tsx";

`;
	writeFileSync(indexPath, head.trimEnd() + "\n");
	writeFileSync(hostPath, hostHeader + tail.replace(/^export type \{ BoardPlayPaneId \} from "\.\/index\.ts";\n\n/, ""));
	console.log("[relayer] 2d host extracted");
}

function extract3d(): void {
	const indexPath = join(root, "puzzle/3d/react/index.tsx");
	let c = readFileSync(indexPath, "utf8");
	const marker = "\n// #region";
	const markers = [
		"\nfunction useScenePlayController():",
		"\n// #region 🧲Header\n// 💻 elements/client/lib/system/renderer/react/scene/scene-play-host",
	];
	let pos = -1;
	for (const m of markers) {
		const p = c.indexOf(m);
		if (p >= 0) {
			pos = p;
			break;
		}
	}
	if (pos < 0) throw new Error("3d play host marker missing");
	const head = c
		.slice(0, pos)
		.replace(/\nimport \{ Expertise, type FooterItem \} from "@framework\/playground";\n/, "\n")
		.replace(/\nimport \{[\s\S]*?\} from "@framework\/playground-renderer-react";\n/, "\n")
		.replace(/\nimport nakaginSceneFixtureJson from "\.\.\/play\/fixtures[\s\S]*?from "\.\.\/play\/index\.ts";\n/, "\n")
		.trimEnd();
	const tail = c.slice(pos).replace(/^\/\/ #region[\s\S]*?\/\/ #endregion[\s\S]*?\n\n/, "");
	const hostPath = join(root, "puzzle/3d/play/host.tsx");
	const hostHeader = `// #region 🧲Header
/** @emoji 🛝 Scene play React host — entry-only; imported from play/main.ts. */
// #endregion 🧲Header

import { LevelProvider, getLevelBgClass } from "@ui/react";
import { useGLTF } from "@react-three/drei";
import { ClipboardList, ListTree, Settings, Tags } from "lucide-react";
import React, { useCallback, useMemo, useSyncExternalStore } from "react";
import {
	registerTabIcon,
	registerUiScene3DSurfaceHost,
	registerWindowBody,
	useApp,
	type UiScene3DHostSurfaceNode,
} from "@framework/playground-renderer-react";
import nakaginSceneFixtureJson from "./fixtures/nakagin-capsule-tower.scene.json";
import {
	PlaySceneCanvas,
	SceneObjectStateProvider,
	parseFixtureV1,
	applyConnectToSceneFixture,
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
	writeFileSync(indexPath, head + "\n");
	writeFileSync(hostPath, hostHeader + tail);
	console.log("[relayer] 3d host extracted");
}

function extract5d(): void {
	const indexPath = join(root, "puzzle/5d/react/index.tsx");
	let c = readFileSync(indexPath, "utf8");
	const marker = "\n// #region 🛝PlayHost";
	const pos = c.indexOf(marker);
	if (pos < 0) throw new Error("5d marker missing");
	const head = c.slice(0, pos).trimEnd();
	const tail = c.slice(pos + marker.length);
	const hostPath = join(root, "puzzle/5d/play/host.tsx");
	const hostHeader = `// #region 🧲Header
/** @emoji 🛝 Topology play React host — entry-only; imported from play/main.ts. */
// #endregion 🧲Header

`;
	writeFileSync(indexPath, head + "\n");
	writeFileSync(hostPath, hostHeader + tail.replace(/from "\.\/index\.ts"/, 'from "./index.ts"').replace('import "./globals.css";', 'import "./globals.css";\n'));
	console.log("[relayer] 5d host extracted");
}

extract2d();
extract3d();
extract5d();
