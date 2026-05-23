#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";

const path = "c:/git/semio/puzzle/2d/react/index.tsx";
let c = readFileSync(path, "utf8");
const start = c.indexOf("/** @emoji 🛝 Board play React host");
const endMarker = "} = Board;\n\n";
const end = c.indexOf(endMarker, start);
if (start < 0 || end < 0) {
	console.error("[fix-2d] block not found");
	process.exit(1);
}
c = c.slice(0, start) + c.slice(end + endMarker.length);

const inject = `import { Expertise, ProductRuntime, type FooterItem } from "@framework/playground";
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
	type BoardPlayPaneId,
} from "../play/index.ts";

`;

const anchor = '} from "@ui/react";\n';
const pos = c.indexOf(anchor);
if (pos < 0) {
	console.error("[fix-2d] ui/react anchor missing");
	process.exit(1);
}
c = c.slice(0, pos + anchor.length) + "\n" + inject + c.slice(pos + anchor.length);
c = c.replace('type ReactElement } from "react";', "type ReactElement, type ReactNode } from \"react\";");
c = c.replace(/\nimport type \{ ReactElement \} from "react";\nimport React from "react";/, "\nimport React from \"react\";");

writeFileSync(path, c);
console.log("[fix-2d] cleaned");
