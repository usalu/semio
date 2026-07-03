#!/usr/bin/env bun
/** Batch-add OsProgramContribution exports and update semio.app manifests. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

type ResourceSpec = {
	readonly apps: Readonly<Record<string, string>>;
	readonly vcsImport?: { readonly pkg: string; readonly symbol: string; readonly local?: boolean };
	readonly vcsLocal?: string;
	readonly extraRegister?: string;
};

const SPECS: Record<string, { readonly pkgJson: string; readonly indexTs: string; readonly exportName: string; readonly programId: string; readonly buildFn: string; readonly resource: ResourceSpec }> = {
	draw: {
		pkgJson: "draw/core/package.json",
		indexTs: "draw/core/js/index.ts",
		exportName: "drawProgramContribution",
		programId: "draw",
		buildFn: "buildDrawProgramDefinition",
		resource: { apps: { draw: `osBaselineResource("2d.drawing", "draw.document", "draw")` }, vcsLocal: "createDrawAppVcsHandler" },
	},
	note: {
		pkgJson: "note/core/package.json",
		indexTs: "note/core/js/index.ts",
		exportName: "noteProgramContribution",
		programId: "note",
		buildFn: "buildNoteProgramDefinition",
		resource: { apps: { note: `osBaselineResource("2d.note", "note.document", "note")` }, vcsLocal: "createNoteAppVcsHandler" },
	},
	writer: {
		pkgJson: "writer/core/package.json",
		indexTs: "writer/core/js/index.ts",
		exportName: "writerProgramContribution",
		programId: "writer",
		buildFn: "buildWriterProgramDefinition",
		resource: { apps: { writer: `osBaselineResource("text.document", "writer.document", "writer")` }, vcsLocal: "createWriterAppVcsHandler" },
	},
	raster: {
		pkgJson: "raster/core/package.json",
		indexTs: "raster/core/js/index.ts",
		exportName: "rasterProgramContribution",
		programId: "raster",
		buildFn: "buildRasterProgramDefinition",
		resource: {
			apps: {
				raster: `{ ...osBaselineResource("2d.raster", "raster.document", "raster"), parameterFields: [{ fieldPath: "/brushSize", label: "Brush size", type: "numeric" }, { fieldPath: "/brushOpacity", label: "Brush opacity", type: "numeric" }] }`,
			},
			vcsLocal: "createRasterAppVcsHandler",
		},
	},
	flow: {
		pkgJson: "flow/core/package.json",
		indexTs: "flow/core/js/index.ts",
		exportName: "flowProgramContribution",
		programId: "flow",
		buildFn: "buildFlowProgramDefinition",
		resource: {
			apps: {
				flow: `{ ...osBaselineResource("computation.flow", "flow.document", "flow"), parameterFields: [{ fieldPath: "/camera/zoom", label: "Camera zoom", type: "numeric" }] }`,
			},
			vcsLocal: "createFlowAppVcsHandler",
		},
	},
	forms: {
		pkgJson: "forms/core/package.json",
		indexTs: "forms/core/js/index.ts",
		exportName: "formsProgramContribution",
		programId: "forms",
		buildFn: "buildFormsProgramDefinition",
		resource: { apps: { forms: `osBaselineResource("form.dictionary", "forms.form", "forms")` }, vcsLocal: "createFormsAppVcsHandler" },
	},
	"puzzle.2d": {
		pkgJson: "puzzle/2d/core/package.json",
		indexTs: "puzzle/2d/core/js/index.ts",
		exportName: "puzzle2dProgramContribution",
		programId: "puzzle.2d",
		buildFn: "buildPuzzle2dProgramDefinition",
		resource: { apps: { puzzle2d: `osBaselineResource("2d.puzzle", "puzzle.2d", "puzzle2d")` }, vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createPuzzle2dAppVcsHandler" } },
	},
	"puzzle.3d": {
		pkgJson: "puzzle/3d/core/package.json",
		indexTs: "puzzle/3d/core/js/index.ts",
		exportName: "puzzle3dProgramContribution",
		programId: "puzzle.3d",
		buildFn: "buildPuzzle3dProgramDefinition",
		resource: { apps: { puzzle3d: `osBaselineResource("3d.puzzle", "puzzle.3d", "puzzle3d")` }, vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createPuzzle3dAppVcsHandler" } },
	},
	"puzzle.5d": {
		pkgJson: "puzzle/5d/core/package.json",
		indexTs: "puzzle/5d/core/js/index.ts",
		exportName: "puzzle5dProgramContribution",
		programId: "puzzle.5d",
		buildFn: "buildPuzzle5dProgramDefinition",
		resource: {
			apps: {
				puzzle5d: `{ inputs: [osInPort("catalogue.kinds", "catalogue", "Catalogue")], outputs: [osOutPort("2d.puzzle", "graph2d", "2D Graph"), osOutPort("3d.mesh", "mesh3d", "3D Mesh")], sourceFormat: "puzzle.5d", componentKind: "puzzle5d", modes: [{ id: "edit", label: "Edit" }] }`,
			},
			vcsLocal: "createPuzzle5dAppVcsHandler",
		},
	},
	trinity: {
		pkgJson: "trinity/jack/host-core/package.json",
		indexTs: "trinity/jack/host-core/js/index.ts",
		exportName: "trinityProgramContribution",
		programId: "trinity",
		buildFn: "buildTrinityProgramDefinition",
		resource: {
			apps: { "trinity-jack": `osBaselineResource("graph.trinity", "trinity.graph", "trinity", [{ id: "query", label: "Query" }])` },
			vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createTrinityGraphAppVcsHandler" },
		},
	},
	"trinity.rewrite": {
		pkgJson: "trinity/rewrite/core/package.json",
		indexTs: "trinity/rewrite/core/js/index.ts",
		exportName: "trinityRewriteProgramContribution",
		programId: "trinity.rewrite",
		buildFn: "buildTrinityRewriteProgramDefinition",
		resource: {
			apps: { "trinity-rewrite": `osBaselineResource("graph.trinity", "trinity.graph", "trinityRewrite", [{ id: "edit", label: "Edit" }])` },
			vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createTrinityGraphAppVcsHandler" },
		},
	},
	shooting: {
		pkgJson: "shooting/core/package.json",
		indexTs: "shooting/core/js/index.ts",
		exportName: "shootingProgramContribution",
		programId: "shooting",
		buildFn: "buildShootingProgramDefinition",
		resource: {
			apps: {
				shooting: `{ inputs: [osInPort("3d.mesh", "mesh", "Mesh")], outputs: [osOutPort("2d.shooting")], sourceFormat: "shooting.scene", componentKind: "shooting", modes: [{ id: "edit", label: "Edit" }], parameterFields: [{ fieldPath: "/camera/zoom", label: "Camera zoom", type: "numeric" }] }`,
			},
			vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createShootingAppVcsHandler" },
		},
	},
	"procedural.2d": {
		pkgJson: "procedural/2d/core/package.json",
		indexTs: "procedural/2d/core/js/index.ts",
		exportName: "procedural2dProgramContribution",
		programId: "procedural.2d",
		buildFn: "buildProcedural2dProgramDefinition",
		resource: { apps: { procedural2d: `osBaselineResource("2d.procedural", "procedural.2d", "puzzle2d")` }, vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createProcedural2dAppVcsHandler" } },
	},
	"procedural.3d": {
		pkgJson: "procedural/3d/core/package.json",
		indexTs: "procedural/3d/core/js/index.ts",
		exportName: "procedural3dProgramContribution",
		programId: "procedural.3d",
		buildFn: "buildProcedural3dProgramDefinition",
		resource: { apps: { procedural3d: `osBaselineResource("3d.procedural", "procedural.3d", "puzzle3d")` }, vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createProcedural3dAppVcsHandler" } },
	},
	"gis.map": {
		pkgJson: "gis/2d/core/package.json",
		indexTs: "gis/2d/core/js/index.ts",
		exportName: "gisMapProgramContribution",
		programId: "gis.map",
		buildFn: "buildGisMapProgramDefinition",
		resource: {
			apps: {
				map: `{ ...osBaselineResource("2d.map", "gis.map", "gismap"), parameterFields: [{ fieldPath: "/view/zoom", label: "Map zoom", type: "numeric" }] }`,
			},
			vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createGisMapAppVcsHandler" },
		},
	},
	presentation: {
		pkgJson: "framework/product/presentation/core/package.json",
		indexTs: "framework/product/presentation/core/js/index.ts",
		exportName: "presentationProgramContribution",
		programId: "presentation",
		buildFn: "buildPresentationProgramDefinition",
		resource: {
			apps: { presentation: `osBaselineResource("presentation.deck", "presentation.deck", "panel", [{ id: "edit", label: "Edit" }])` },
			vcsLocal: "createPresentationAppVcsHandler",
			extraRegister: `mergeOsProgramDefinition("presentation.deck", buildPresentationDeckProgramDefinition(), { "presentation.deck": osBaselineResource("presentation.deck", "presentation.deck", "panel", [{ id: "edit", label: "Edit" }]) });\n\t\tregisterAppVcsHandler(createPresentationDeckAppVcsHandler());`,
		},
	},
	sequence: {
		pkgJson: "sequence/core/package.json",
		indexTs: "sequence/core/js/index.ts",
		exportName: "sequenceProgramContribution",
		programId: "sequence",
		buildFn: "buildSequenceProgramDefinition",
		resource: {
			apps: {
				sequence: `{ ...osBaselineResource("computation.sequence", "sequence.fixture", "sequence"), parameterFields: [{ fieldPath: "/camera/zoom", label: "Camera zoom", type: "numeric" }] }`,
			},
			vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createSequenceAppVcsHandler" },
		},
	},
	layout: {
		pkgJson: "layout/core/package.json",
		indexTs: "layout/core/js/index.ts",
		exportName: "layoutProgramContribution",
		programId: "layout",
		buildFn: "buildLayoutProgramDefinition",
		resource: {
			apps: {
				layout: `{ ...osBaselineResource("2d.layout", "layout.fixture", "layout"), parameterFields: [{ fieldPath: "/pages/0/width", label: "Page width", type: "numeric" }, { fieldPath: "/pages/0/height", label: "Page height", type: "numeric" }] }`,
			},
			vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createLayoutAppVcsHandler" },
		},
	},
	imperative: {
		pkgJson: "imperative/core/package.json",
		indexTs: "imperative/core/js/index.ts",
		exportName: "imperativeProgramContribution",
		programId: "imperative",
		buildFn: "buildImperativeProgramDefinition",
		resource: {
			apps: {
				imperative: `{ ...osBaselineResource("computation.imperative", "imperative.document", "imperative"), parameterFields: [{ fieldPath: "/camera/zoom", label: "Camera zoom", type: "numeric" }] }`,
			},
			vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createImperativeAppVcsHandler" },
		},
	},
	lowpoly: {
		pkgJson: "lowpoly/core/package.json",
		indexTs: "lowpoly/core/js/index.ts",
		exportName: "lowpolyProgramContribution",
		programId: "lowpoly",
		buildFn: "buildLowpolyProgramDefinition",
		resource: {
			apps: {
				lowpoly: `{ ...osBaselineResource("3d.lowpoly", "lowpoly.fixture", "lowpoly"), parameterFields: [{ fieldPath: "/paint/opacity", label: "Paint opacity", type: "numeric" }] }`,
			},
			vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createLowpolyAppVcsHandler" },
		},
	},
	vcs: {
		pkgJson: "vcs/core/package.json",
		indexTs: "vcs/core/js/index.ts",
		exportName: "vcsProgramContribution",
		programId: "vcs",
		buildFn: "buildVcsProgramDefinition",
		resource: {
			apps: {
				vcs: `{ ...osBaselineResource("vcs.document", "vcs.demo", "vcs", [{ id: "explore", label: "Explore" }]), parameterFields: [{ fieldPath: "/counter", label: "Counter", type: "numeric" }] }`,
			},
			vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createVcsDemoAppVcsHandler" },
		},
	},
	dag: {
		pkgJson: "mathematical/graph/port/directed/dag/core/package.json",
		indexTs: "mathematical/graph/port/directed/dag/core/js/index.ts",
		exportName: "dagProgramContribution",
		programId: "dag",
		buildFn: "buildDagProgramDefinition",
		resource: { apps: { dag: `osBaselineResource("graph.dag", "flow.dag", "dag")` }, vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createFlowDagAppVcsHandler" } },
	},
	cad: {
		pkgJson: "cad/renderer/core/package.json",
		indexTs: "cad/renderer/core/js/index.ts",
		exportName: "cadProgramContribution",
		programId: "cad",
		buildFn: "buildCadProgramDefinition",
		resource: { apps: { cad: `osBaselineResource("3d.cad", "cad.scene", "cad")` } },
	},
	"reasoning.wires": {
		pkgJson: "reasoning/mindmap/wires/core/package.json",
		indexTs: "reasoning/mindmap/wires/core/js/index.ts",
		exportName: "wiresProgramContribution",
		programId: "reasoning.wires",
		buildFn: "buildReasoningWiresProgramDefinition",
		resource: {
			apps: { wires: `osBaselineResource("2d.puzzle", "puzzle.2d", "puzzle2d")` },
			vcsImport: { pkg: "@semio-tech/framework-os-core", symbol: "createPuzzle2dAppVcsHandler" },
			extraRegister: `mergeOsProgramDefinition("reasoning.mindmap", { id: "reasoning.mindmap", name: "Reasoning Mindmap", apiVersion: "1", apps: [{ id: "mindmap", label: "Mindmap", controllerId: "reasoning-mindmap", modes: [{ id: "explore", label: "Explore" }], defaultModeId: "explore" }], createPlatformApi: () => ({}) }, { mindmap: osBaselineResource("2d.puzzle", "puzzle.2d", "puzzle2d", [{ id: "explore", label: "Explore" }]) });`,
		},
	},
};

function resourceBlock(spec: ResourceSpec): string {
	const entries = Object.entries(spec.apps)
		.map(([appId, expr]) => `\t\t${JSON.stringify(appId)}: ${expr},`)
		.join("\n");
	return `{\n${entries}\n\t}`;
}

function contributionBlock(spec: (typeof SPECS)[string]): string {
	const { exportName, programId, buildFn, resource } = spec;
	const needsInOut = Object.values(resource.apps).some((v) => v.includes("osInPort") || v.includes("osOutPort"));
	const osImports = ["mergeOsProgramDefinition", "osBaselineResource", "registerAppVcsHandler", ...(needsInOut ? ["osInPort", "osOutPort"] : [])];
	const vcsImports: string[] = [];
	if (resource.vcsImport) vcsImports.push(resource.vcsImport.symbol);
	if (resource.extraRegister?.includes("createPresentationDeckAppVcsHandler")) vcsImports.push("createPresentationDeckAppVcsHandler");

	const vcsCall = resource.vcsLocal
		? `registerAppVcsHandler(${resource.vcsLocal}());`
		: resource.vcsImport
			? `registerAppVcsHandler(${resource.vcsImport.symbol}());`
			: "";
	const extra = resource.extraRegister ? `\n\t\t${resource.extraRegister}` : "";

	return `//#region 🔖OsProgram
import { ${osImports.join(", ")} } from "@semio-tech/framework-os-core";
import type { OsProgramContribution } from "@semio-tech/framework-platform-core";
${vcsImports.length ? `import { ${vcsImports.join(", ")} } from "@semio-tech/framework-os-core";\n` : ""}
const ${exportName}Resources = ${resourceBlock(resource)};

/** @emoji 🧩 OS program contribution for ${programId}. */
export const ${exportName}: OsProgramContribution = {
\tprogramId: ${JSON.stringify(programId)},
\tregister() {
\t\tmergeOsProgramDefinition(${JSON.stringify(programId)}, ${buildFn}(), ${exportName}Resources);
\t\t${vcsCall}${extra}
\t},
};
//#endregion 🔖OsProgram`;
}

function updateIndex(path: string, exportName: string, spec: (typeof SPECS)[string]): void {
	const full = join(REPO, path);
	let content = readFileSync(full, "utf8");
	if (content.includes(`export const ${exportName}`)) {
		console.log(`skip index (exists): ${path}`);
		return;
	}
	const marker = "//#endregion 🔖SExtension";
	const idx = content.indexOf(marker);
	const block = contributionBlock(spec);
	if (idx === -1) {
		const playIdx = content.indexOf("//#region 🔖Play");
		if (playIdx === -1) throw new Error(`no SExtension/Play anchor in ${path}`);
		content = content.slice(0, playIdx) + block + "\n\n" + content.slice(playIdx);
	} else {
		content = content.slice(0, idx + marker.length) + "\n\n" + block + content.slice(idx + marker.length);
	}
	writeFileSync(full, content);
	console.log(`updated index: ${path}`);
}

function updatePackageJson(path: string, exportName: string, programId: string, kind: string): void {
	const full = join(REPO, path);
	const pkg = JSON.parse(readFileSync(full, "utf8")) as {
		dependencies?: Record<string, string>;
		semio?: { playgroundApp?: Record<string, unknown>; app?: Record<string, unknown> };
	};
	const manifest = (pkg.semio?.app ?? pkg.semio?.playgroundApp) as Record<string, unknown> | undefined;
	if (!manifest) throw new Error(`no manifest in ${path}`);
	pkg.dependencies = pkg.dependencies ?? {};
	if (!pkg.dependencies["@semio-tech/framework-os-core"]) pkg.dependencies["@semio-tech/framework-os-core"] = "workspace:*";
	if (!pkg.dependencies["@semio-tech/framework-platform-core"]) pkg.dependencies["@semio-tech/framework-platform-core"] = "workspace:*";
	pkg.semio = {
		app: {
			...manifest,
			programExport: exportName,
			programId,
		},
	};
	delete pkg.semio.playgroundApp;
	writeFileSync(full, JSON.stringify(pkg, null, 2) + "\n");
	console.log(`updated package: ${path}`);
}

for (const [key, spec] of Object.entries(SPECS)) {
	const manifest = JSON.parse(readFileSync(join(REPO, spec.pkgJson), "utf8")) as { semio?: { playgroundApp?: { kind: string }; app?: { kind: string } } };
	const kind = manifest.semio?.app?.kind ?? manifest.semio?.playgroundApp?.kind ?? key;
	updateIndex(spec.indexTs, spec.exportName, spec);
	updatePackageJson(spec.pkgJson, spec.exportName, spec.programId, kind);
}

console.log("done");
