#!/usr/bin/env bun
/** Temporary migration script — splits cad/js/renderer/play/index.tsx into core + react. */
import { readFileSync, writeFileSync, copyFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const root = "/Users/ueli/Documents/semio";
const srcPath = join(root, "cad/js/renderer/play/index.tsx");
const lines = readFileSync(srcPath, "utf8").split("\n");

function slice(start: number, end: number): string {
  return lines.slice(start - 1, end).join("\n");
}

const coreDir = join(root, "cad/js/renderer/core");
const reactDir = join(root, "cad/js/renderer/react");
mkdirSync(coreDir, { recursive: true });
mkdirSync(reactDir, { recursive: true });
copyFileSync(join(root, "cad/js/renderer/play/fixture-slugs.ts"), join(coreDir, "fixture-slugs.ts"));
copyFileSync(join(root, "cad/js/renderer/play/globals.css"), join(reactDir, "globals.css"));

const coreImports = `import {
  CommandBus,
  Controller,
  Platform,
  AppRuntime,
  ModeRuntime,
  WindowKindRuntime,
  buildCadWindowBody,
  createNamedLayout,
  createPlayAppRuntime,
  createWindowLayout,
  PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY,
  namedLayoutsFromOrbitViewDescriptors,
  registerWindowBody,
  type AppTools,
  type ToolLeaf,
  toolCollection,
  type WindowBodyViewContext,
  type WindowEngagement,
  type WindowMeasure,
  type UiNode,
  type WindowLayout,
  type WindowTemplate,
  enforcePlaygroundWindowEngagementInput,
  windowEngagementsEqual,
  type CommandDescriptor,
  isPlaygroundFixtureLocked,
  playgroundLockedFixtureId,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  uiDeclarativeSectionsToTree,
  type UiTreeNode,
  Playground,
  createProductPlaygroundPlatform,
  type PlaygroundAppDefinition,
} from "@semio-tech/framework-playground-core";
${slice(42, 109)}
import type { TreeDataItem, TreeDataSection } from "@semio-tech/ui-react";
`;

const coreIndex = `// #region 🧲Header
/** @emoji 📐 CAD play app core — controller, document, inspector, runtime. */
// #endregion 🧲Header

${coreImports}

${slice(120, 1143)}

${slice(1545, 1593)}

${slice(3032, 3486)}

export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for cad. */
export function buildCadProgramDefinition(): PlatformDefinition {
	return {
		id: "cad",
		name: "CAD",
		apiVersion: "1",
		apps: [{ id: "cad", label: "CAD", controllerId: CAD_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🧪Tests
${slice(3583, 4334)}
//#endregion 🧪Tests
`;

const reactIndex = `// #region 🧲Header
/** @emoji 📐 CAD play React host — viewport chrome, model space, playground root. */
// #endregion 🧲Header

import "./globals.css";

${slice(1147, 1207)}

import {
  CAD_PLAY_APP_ID,
  CAD_PLAY_CONTROLLER_ID,
  CAD_PLAY_DOCUMENT_TAB_ID,
  CAD_PLAY_PANE_SPECS,
  CAD_PLAY_CONCRETE_FOREST_FIXTURE_IDS,
  CadPlayShellController,
  buildCadPlayRuntime,
  buildCadPlayDocumentPendingSections,
  buildCadPlayDocumentSections,
  buildCadPlayDetailsTree,
  buildCadPlayCatalogTree,
  cadPlaySceneSurfaceIdForPane,
  cadPlayPaneFromSurfaceId,
  cadPlayPaneFromWindowKindId,
  cadPlayPaneFromShellWindowId,
  cadPlayDefaultReferencesByModelDefinitionId,
  cadPlayReferencesForFixture,
  cadPlayIsConcreteForestFixture,
  ensurePlayQuadModelSlots,
  ensureCadPlayQuadModels,
  cadPlayPaneGeometry,
  type CadPlayChromeSnapshot,
  type CadPlayPaneId,
  type CadPlayReferencesByModelDefinitionId,
  type CadPlaySelectedReference,
} from "@semio-tech/cad-js-renderer-core";
import { CAD_PLAY_SHAPE_ASSETS, resolveCadPlayFixtureSlug } from "@semio-tech/cad-js-renderer-core/playground";

${slice(1208, 1544).replace('export { resolveCadPlayFixtureSlug } from "./fixture-slugs.ts";', "").replace("const SHAPE_ASSETS", "const SHAPE_ASSETS_UNUSED")}

${slice(1594, 3031)}

${slice(3488, 3518)}

${slice(3521, 3565)}
`;

const playgroundTs = `// #region 🧲Header
/** @emoji 🛝 CAD playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import geometryConcreteForestLeft from "../../../asset/play/hexagonal-cut-concrete-forest-left.model.json";
import geometryConcreteForestRight from "../../../asset/play/hexagonal-cut-concrete-forest-right.model.json";
import { defaultModelDefinitionId } from "@semio-tech/cad-js-core";
import {
	CAD_PLAY_APP_ID,
	CAD_PLAY_CONTROLLER_ID,
	CadPlayShellController,
	buildCadPlayAppRuntime,
	registerCadPlayDeclarativeBodies,
} from "./index.ts";
import { CAD_PLAY_SHAPE_ASSET_IDS, resolveCadPlayFixtureSlug } from "./fixture-slugs.ts";

export { CAD_PLAY_SHAPE_ASSET_IDS, resolveCadPlayFixtureSlug } from "./fixture-slugs.ts";

/** @emoji 🧩 Shape fixture assets for CAD play navbar. */
export const CAD_PLAY_SHAPE_ASSETS = [
	{ id: "concrete-forest-left", key: "c", label: "Concrete forest (left)", json: { modelSpace: geometryConcreteForestLeft, activeModelDefinitionId: defaultModelDefinitionId() } as Record<string, unknown> },
	{ id: "concrete-forest-right", key: "d", label: "Concrete forest (right)", json: geometryConcreteForestRight as Record<string, unknown> },
] as const;

/** @emoji 🛝 CAD playground app. */
export class PlaygroundCad extends Playground {
	readonly id = CAD_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		registerCadPlayDeclarativeBodies();
		const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildCadPlayAppRuntime(controller));
		return runtime;
	}

	registerBodies(): void {
		registerCadPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition

/** @emoji 🛝 CAD playground app definition. */
export const cadPlayAppDefinition: PlaygroundAppDefinition = {
	id: CAD_PLAY_APP_ID,
	label: "CAD",
	controllerId: CAD_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundCad(),
	bootRenderer: async (pg) => {
		const { bootCadPlay } = await import("@semio-tech/framework-playground-renderer-react/cad");
		bootCadPlay(pg);
	},
	devHost: {
		playEntryKind: "cad",
		resolveDedupe: ["react", "react-dom", "three"],
		optimizeDeps: {
			include: [
				"react",
				"react-dom",
				"react/jsx-runtime",
				"react/jsx-dev-runtime",
				"three",
				"@react-three/fiber",
				"@react-three/drei",
				"@semio-tech/infinite-world-r3f",
				"brepjs",
				"brepjs-opencascade",
				"golden-layout",
				"lucide-react",
				"chevrotain",
			],
		},
	},
};
//#endregion 🔖PlaygroundAppDefinition
`;

writeFileSync(join(coreDir, "index.ts"), coreIndex);
writeFileSync(join(reactDir, "index.tsx"), reactIndex);
writeFileSync(join(coreDir, "playground.ts"), playgroundTs);
console.log("Split complete");
