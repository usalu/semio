// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🔧 `@procedural/react` — flow-based brep editor with R3F viewport. */
// #endregion 🧲Header

// #region 🔌Adapters
import { reactHostPort, sceneHostPort } from "@ui/react";
import {
	brepjsGeometryKernel,
	BrepjsGeometryKernel,
	ensureBrepWasmLoaded,
	isRenderableMeshTransfer,
	meshTransferToGeometryData,
	type BrepjsGeometryKernel as BrepKernelType,
	type GeometryRef,
	type Vec3,
} from "@geometry/brep/js";
import {
	FlowCanvas,
	type DagDrawLodKind,
	FlowExtensionHost,
	createFlowEvalBridge,
	type CatalogueSection,
	type FlowCanvasCommandRequest,
	type FlowCanvasContextMenuContext,
	type FlowExtensionEntry,
	type FlowFixtureV1,
	type FlowModuleCommandV1,
	type FlowModuleManifestV1,
	type FlowModuleNeuronKindV1,
	type FlowReorganizeRequest,
} from "@flow/react";
import type { ContextMenuItem } from "@ui/react";
import {
	applyOrbitProjectionToCameraState,
	DEFAULT_LOD_GRID_FACTOR,
	DEFAULT_MANUAL_LOD,
	worldEntityRenderMode,
	WorldCameraInvalidator,
	WorldCanvas,
	WorldLayer,
	WorldLodBridge,
	WorldOrbitCameraViewRig,
	WorldOrbitGated,
	WorldOrbitProjectionSwitch,
	WorldOrbitViewControls,
	WorldOrbitViewSnapGateProvider,
	WorldEventBindingController,
	type OrbitCameraProjection,
	type WorldCameraState,
} from "@infinite/world/r3f";
import {
	SelectionMarquee,
	marqueeCoverageFromDrag,
	marqueeModeFromModifiers,
	screenRectContainsRect,
	screenRectFromPoints,
	screenRectIntersectsRect,
	selectionMergeIds,
	type SelectionMergeMode,
	type SelectionMarqueeCoverage,
} from "@ui/react";
import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from "react";

const THREE = sceneHostPort.three;
// #endregion 🔌Adapters

// #region 🔖BrepWasmBridge
if (!import.meta.env.VITEST) {
	await ensureBrepWasmLoaded();
}
// #endregion 🔖BrepWasmBridge

// #region 🔖BrepFlowModule
function toPascalCase(label: string): string {
	return label
		.split(/[\s_-]+/)
		.filter(Boolean)
		.map((part) => part.charAt(0).toUpperCase() + part.slice(1).toLowerCase())
		.join("");
}

function brepAbbreviation(name: string): string {
	const pascal = toPascalCase(name);
	if (pascal.length <= 12) return pascal;
	return name
		.split(/[\s_-]+/)
		.filter(Boolean)
		.map((part) => part.charAt(0).toUpperCase())
		.join("");
}

function brepIcon(id: string): string {
	if (id.includes("point")) return "emoji:📍";
	if (id.includes("vector")) return "emoji:➡️";
	if (id.includes("box") || id.includes("rectangle") || id.includes("prim3d")) return "emoji:📦";
	if (id.includes("sphere") || id.includes("circle")) return "emoji:⚪";
	if (id.includes("cylinder") || id.includes("cone") || id.includes("torus") || id.includes("ellipsoid")) return "emoji:🛢️";
	if (id.includes("curve") || id.includes("wire") || id.includes("arc") || id.includes("helix") || id.includes("line")) return "emoji:〰️";
	if (id.includes("surface") || id.includes("face") || id.includes("fill")) return "emoji:🧩";
	if (id.includes("solid") || id.includes("extrude") || id.includes("revolve") || id.includes("loft") || id.includes("sweep") || id.includes("shell") || id.includes("hull")) return "emoji:🧱";
	if (id.includes("bool") || id.includes("fuse") || id.includes("cut") || id.includes("intersect") || id.includes("union")) return "emoji:🔗";
	if (id.includes("xform") || id.includes("translate") || id.includes("rotate") || id.includes("mirror") || id.includes("scale") || id.includes("pattern") || id.includes("clone")) return "emoji:🔁";
	if (id.includes("intersect") || id.includes("section") || id.includes("slice")) return "emoji:✂️";
	if (id.includes("eval") || id.includes("measure") || id.includes("query")) return "emoji:📐";
	if (id.includes("repair") || id.includes("heal")) return "emoji:🩹";
	if (id.includes("io") || id.includes("export")) return "emoji:💾";
	if (id.includes("gear")) return "emoji:⚙️";
	if (id.includes("sketch") || id.includes("draw2d")) return "emoji:✏️";
	return "emoji:🔷";
}

function brepKind(id: string, name: string, summary: string, inputs: readonly string[], outputs: readonly string[] = ["geometry"]): FlowModuleNeuronKindV1 {
	const displayName = toPascalCase(name);
	return { id: `brep.${id}`, module: "brep", name: displayName, abbreviation: brepAbbreviation(name), icon: brepIcon(id), summary, inputs, outputs };
}

const BREP_CATALOGUE_SECTIONS: readonly { readonly id: string; readonly title: string; readonly kinds: readonly FlowModuleNeuronKindV1[] }[] = [
	{
		id: "brep-construct",
		title: "Brep · Construct",
		kinds: [
			brepKind("point", "Point", "3D point from x,y,z", ["x", "y", "z"], ["point"]),
			brepKind("vector", "Vector", "3D vector from x,y,z", ["x", "y", "z"], ["vector"]),
		],
	},
	{
		id: "brep-prim3d",
		title: "Brep · Primitives 3D",
		kinds: [
			brepKind("prim3d.box", "Box", "Axis-aligned box", ["width", "depth", "height"]),
			brepKind("prim3d.sphere", "Sphere", "Sphere solid", ["radius"]),
			brepKind("prim3d.cylinder", "Cylinder", "Cylinder solid", ["radius", "height"]),
			brepKind("prim3d.cone", "Cone", "Cone solid", ["radius", "height"]),
			brepKind("prim3d.torus", "Torus", "Torus solid", ["major", "minor"]),
			brepKind("prim3d.ellipsoid", "Ellipsoid", "Ellipsoid solid", ["rx", "ry", "rz"]),
		],
	},
	{
		id: "brep-draw2d",
		title: "Brep · Draw 2D",
		kinds: [
			brepKind("draw2d.rectangle", "Draw Rectangle", "2D rectangle drawing", ["width", "height"]),
			brepKind("draw2d.circle", "Draw Circle", "2D circle drawing", ["radius"]),
			brepKind("draw2d.ellipse", "Draw Ellipse", "2D ellipse drawing", ["major", "minor"]),
			brepKind("draw2d.roundedRectangle", "Draw Rounded Rect", "2D rounded rectangle", ["width", "height", "radius"]),
			brepKind("draw2d.polysides", "Draw Polysides", "2D regular polygon", ["radius", "sides"]),
			brepKind("sketch2d.circle", "Sketch Circle", "Sketched circle profile", ["radius"]),
			brepKind("sketch2d.rectangle", "Sketch Rectangle", "Sketched rectangle profile", ["width", "height"]),
		],
	},
	{
		id: "brep-curves",
		title: "Brep · Curves",
		kinds: [
			brepKind("curve.line", "Line", "Line edge", ["start", "end"]),
			brepKind("curve.circle", "Circle", "Circle edge", ["radius"]),
			brepKind("curve.ellipse", "Ellipse", "Ellipse edge", ["major", "minor"]),
			brepKind("curve.helix", "Helix", "Helix edge", ["radius", "pitch", "height"]),
			brepKind("curve.threePointArc", "Three Point Arc", "Arc through three points", ["a", "b", "c"]),
			brepKind("curve.tangentArc", "Tangent Arc", "Tangent arc", ["start", "tangent", "end"]),
			brepKind("curve.ellipseArc", "Ellipse Arc", "Elliptical arc", ["major", "minor", "startAngle", "endAngle"]),
			brepKind("curve.interpolate", "Interpolate", "Interpolated curve", ["geometry"]),
			brepKind("curve.wire", "Wire", "Wire from edges", ["geometry"]),
			brepKind("curve.wireLoop", "Wire Loop", "Closed wire loop", ["geometry"]),
		],
	},
	{
		id: "brep-surfaces",
		title: "Brep · Surfaces",
		kinds: [
			brepKind("surface.face", "Face", "Face from wires", ["geometry"]),
			brepKind("surface.filledFace", "Filled Face", "Filled face from wire", ["geometry"]),
			brepKind("surface.fill", "Fill", "Fill from edges", ["geometry"]),
			brepKind("surface.offsetFace", "Offset Face", "Offset face", ["geometry", "distance"]),
		],
	},
	{
		id: "brep-solid",
		title: "Brep · Solid Tools",
		kinds: [
			brepKind("solid.extrude", "Extrude", "Extrude profile", ["geometry", "distance"]),
			brepKind("solid.revolve", "Revolve", "Revolve profile", ["geometry", "angle"]),
			brepKind("solid.loft", "Loft", "Loft sections", ["a", "b"]),
			brepKind("solid.sweep", "Sweep", "Sweep profile along path", ["profile", "path"]),
			brepKind("solid.fillet", "Fillet", "Fillet solid", ["geometry", "radius"]),
			brepKind("solid.chamfer", "Chamfer", "Chamfer solid", ["geometry", "distance"]),
			brepKind("solid.shell", "Shell", "Shell solid", ["geometry", "thickness"]),
			brepKind("solid.offset", "Offset", "Offset shape", ["geometry", "distance"]),
			brepKind("solid.thicken", "Thicken", "Thicken face", ["geometry", "thickness"]),
			brepKind("solid.hull", "Hull", "Convex hull of shapes", ["a", "b"]),
			brepKind("solid.minkowski", "Minkowski", "Minkowski sum", ["a", "b"]),
			brepKind("solid.convexHull", "Convex Hull", "Convex hull", ["geometry"]),
		],
	},
	{
		id: "brep-bool",
		title: "Brep · Booleans",
		kinds: [
			brepKind("bool.fuse", "Fuse", "Boolean union", ["a", "b"]),
			brepKind("bool.cut", "Cut", "Boolean difference", ["a", "b"]),
			brepKind("bool.intersect", "Intersect", "Boolean intersection", ["a", "b"]),
			brepKind("bool.fuseAll", "Fuse All", "Fuse multiple solids", ["a", "b"]),
			brepKind("bool.fuse2d", "Fuse 2D", "2D boolean union", ["a", "b"]),
			brepKind("bool.cut2d", "Cut 2D", "2D boolean cut", ["a", "b"]),
			brepKind("bool.intersect2d", "Intersect 2D", "2D boolean intersect", ["a", "b"]),
		],
	},
	{
		id: "brep-xform",
		title: "Brep · Transforms",
		kinds: [
			brepKind("xform.translate", "Translate", "Translate geometry", ["geometry", "offset"]),
			brepKind("xform.rotate", "Rotate", "Rotate geometry", ["geometry", "angle"]),
			brepKind("xform.mirror", "Mirror", "Mirror geometry", ["geometry"]),
			brepKind("xform.scale", "Scale", "Scale geometry", ["geometry", "factor"]),
			brepKind("xform.clone", "Clone", "Clone geometry", ["geometry"]),
			brepKind("xform.linearPattern", "Linear Pattern", "Linear array", ["geometry", "count", "spacing"]),
			brepKind("xform.circularPattern", "Circular Pattern", "Circular array", ["geometry", "count", "angle"]),
			brepKind("xform.rectangularPattern", "Rect Pattern", "Rectangular array", ["geometry", "countA", "countB", "spacing"]),
		],
	},
	{
		id: "brep-intersect",
		title: "Brep · Intersections",
		kinds: [
			brepKind("intersect.section", "Section", "Section two solids", ["a", "b"]),
			brepKind("intersect.sectionToFace", "Section To Face", "Section as face", ["a", "b"]),
			brepKind("intersect.slice", "Slice", "Slice solid by plane", ["geometry"]),
			brepKind("intersect.check", "Check Interference", "Check interference", ["a", "b"], ["number"]),
		],
	},
	{
		id: "brep-eval",
		title: "Brep · Evaluate",
		kinds: [
			brepKind("eval.pointOnCurve", "Point On Curve", "Evaluate point on curve", ["geometry", "t"], ["point"]),
			brepKind("eval.tangentOnCurve", "Tangent On Curve", "Tangent on curve", ["geometry", "t"], ["vector"]),
			brepKind("eval.curveLength", "Curve Length", "Curve length", ["geometry"], ["number"]),
			brepKind("eval.pointOnSurface", "Point On Surface", "Point on surface", ["geometry", "u", "v"], ["point"]),
			brepKind("eval.normalAt", "Normal At", "Surface normal", ["geometry", "u", "v"], ["vector"]),
			brepKind("eval.faceCenter", "Face Center", "Face center point", ["geometry"], ["point"]),
		],
	},
	{
		id: "brep-measure",
		title: "Brep · Measure",
		kinds: [
			brepKind("measure.volume", "Volume", "Solid volume", ["geometry"], ["number"]),
			brepKind("measure.area", "Area", "Face/solid area", ["geometry"], ["number"]),
			brepKind("measure.length", "Length", "Edge/wire length", ["geometry"], ["number"]),
			brepKind("measure.distance", "Distance", "Distance between shapes", ["a", "b"], ["number"]),
		],
	},
	{
		id: "brep-query",
		title: "Brep · Query",
		kinds: [
			brepKind("query.bounds", "Bounds", "Axis-aligned bounds", ["geometry"], ["dictionary"]),
			brepKind("query.edges", "Edges", "List edges", ["geometry"], ["list"]),
			brepKind("query.faces", "Faces", "List faces", ["geometry"], ["list"]),
		],
	},
	{
		id: "brep-repair",
		title: "Brep · Repair",
		kinds: [
			brepKind("repair.heal", "Heal Solid", "Heal solid", ["geometry"]),
			brepKind("repair.autoHeal", "Auto Heal", "Auto heal shape", ["geometry"]),
			brepKind("repair.solidFromShell", "Solid From Shell", "Make solid from shell", ["geometry"]),
		],
	},
	{
		id: "brep-io",
		title: "Brep · IO",
		kinds: [
			brepKind("io.exportStep", "Export STEP", "Export STEP bytes as base64", ["geometry"], ["text"]),
			brepKind("io.exportStl", "Export STL", "Export STL bytes as base64", ["geometry"], ["text"]),
		],
	},
	{
		id: "brep-gear",
		title: "Brep · Gears",
		kinds: [
			brepKind("gear.external", "External Gear", "Spur external gear", ["teeth", "module"]),
			brepKind("gear.internal", "Internal Gear", "Spur internal gear", ["teeth", "module"]),
		],
	},
	{
		id: "brep-legacy",
		title: "Brep · Legacy",
		kinds: [
			brepKind("box", "Box Corners", "Box from corners (legacy)", ["cornerA", "cornerB", "height"]),
			brepKind("sphere", "Sphere Center", "Sphere from center (legacy)", ["center", "radius"]),
			brepKind("cylinder", "Cylinder Base", "Cylinder from base (legacy)", ["base", "axis", "radius", "height"]),
			brepKind("extrude", "Extrude Legacy", "Extrude solid (legacy)", ["geometry", "direction", "distance"]),
			brepKind("translate", "Translate Legacy", "Translate solid (legacy)", ["geometry", "offset"]),
			brepKind("union", "Union Legacy", "Boolean union (legacy)", ["a", "b"]),
		],
	},
];

const BREP_FLOW_KINDS: readonly FlowModuleNeuronKindV1[] = BREP_CATALOGUE_SECTIONS.flatMap((section) => section.kinds);

const BREP_MODULE_MANIFEST: FlowModuleManifestV1 = {
	schema: "flow.module/v1",
	id: "brep",
	name: "Brep",
	version: "0.2.0",
	activationEvents: ["onStartup"],
	contributes: { neuronKinds: BREP_FLOW_KINDS, widgets: [], commands: [], settings: [] },
};

function parseNumber(input: unknown, fallback = 0): number {
	const n = Number(input);
	return Number.isFinite(n) ? n : fallback;
}

function parseVec3(input: unknown, fallback: Vec3 = [0, 0, 0]): Vec3 {
	if (!Array.isArray(input) || input.length < 3) return fallback;
	return [parseNumber(input[0], fallback[0]), parseNumber(input[1], fallback[1]), parseNumber(input[2], fallback[2])];
}

function parseVec3Loose(input: unknown, fallback: Vec3 = [0, 0, 0]): Vec3 | null {
	if (Array.isArray(input)) return parseVec3(input, fallback);
	if (!input || typeof input !== "object") return null;
	const record = input as Record<string, unknown>;
	if ("x" in record || "y" in record || "z" in record) {
		return [parseNumber(record.x, fallback[0]), parseNumber(record.y, fallback[1]), parseNumber(record.z, fallback[2])];
	}
	if ("point" in record) return parseVec3Loose(record.point, fallback);
	if ("vector" in record) return parseVec3Loose(record.vector, fallback);
	return null;
}

function vec3PortOut(port: "point" | "vector", vec: Vec3): Record<string, unknown> {
	return { [port]: { x: vec[0], y: vec[1], z: vec[2] } };
}

function parseVec3Input(input: Record<string, unknown>, key: string, fallback: Vec3 = [0, 0, 0]): Vec3 {
	const raw = input[key];
	return parseVec3Loose(raw, fallback) ?? fallback;
}

function parseGeometry(input: Record<string, unknown>, key: string): GeometryRef | null {
	const raw = input[key];
	return typeof raw === "string" && raw.length > 0 ? (raw as GeometryRef) : null;
}

function geoOut(ref: GeometryRef): Record<string, unknown> {
	return { geometry: String(ref) };
}

function bytesToBase64(bytes: Uint8Array): string {
	let binary = "";
	for (const byte of bytes) binary += String.fromCharCode(byte);
	return btoa(binary);
}

type BrepEvalFn = (input: Record<string, unknown>, kernel: BrepKernelType) => Record<string, unknown>;

const BREP_EVAL_HANDLERS: Record<string, BrepEvalFn> = {
	"brep.point": (input) => vec3PortOut("point", [parseNumber(input.x), parseNumber(input.y), parseNumber(input.z)]),
	"brep.vector": (input) => vec3PortOut("vector", [parseNumber(input.x), parseNumber(input.y), parseNumber(input.z)]),
	"brep.prim3d.box": (input, k) => geoOut(k.boxSync(parseNumber(input.width, 1), parseNumber(input.depth, 1), parseNumber(input.height, 1))),
	"brep.prim3d.sphere": (input, k) => geoOut(k.spherePrimSync(parseNumber(input.radius, 1))),
	"brep.prim3d.cylinder": (input, k) => geoOut(k.cylinderPrimSync(parseNumber(input.radius, 1), parseNumber(input.height, 1))),
	"brep.prim3d.cone": (input, k) => geoOut(k.coneSync(parseNumber(input.radius, 1), parseNumber(input.height, 1))),
	"brep.prim3d.torus": (input, k) => geoOut(k.torusSync(parseNumber(input.major, 2), parseNumber(input.minor, 0.5))),
	"brep.prim3d.ellipsoid": (input, k) => geoOut(k.ellipsoidSync(parseNumber(input.rx, 1), parseNumber(input.ry, 1), parseNumber(input.rz, 1))),
	"brep.draw2d.rectangle": (input, k) => geoOut(k.drawRectangleSync(parseNumber(input.width, 2), parseNumber(input.height, 1))),
	"brep.draw2d.circle": (input, k) => geoOut(k.drawCircleSync(parseNumber(input.radius, 1))),
	"brep.draw2d.ellipse": (input, k) => geoOut(k.drawEllipseSync(parseNumber(input.major, 1), parseNumber(input.minor, 0.5))),
	"brep.draw2d.roundedRectangle": (input, k) => geoOut(k.drawRoundedRectangleSync(parseNumber(input.width, 2), parseNumber(input.height, 1), parseNumber(input.radius, 0.2))),
	"brep.draw2d.polysides": (input, k) => geoOut(k.drawPolysidesSync(parseNumber(input.radius, 1), parseNumber(input.sides, 6))),
	"brep.sketch2d.circle": (input, k) => geoOut(k.sketchCircleSync(parseNumber(input.radius, 1))),
	"brep.sketch2d.rectangle": (input, k) => geoOut(k.sketchRectangleSync(parseNumber(input.width, 2), parseNumber(input.height, 1))),
	"brep.curve.line": (input, k) => geoOut(k.lineSync(parseVec3Input(input, "start", [0, 0, 0]), parseVec3Input(input, "end", [1, 0, 0]))),
	"brep.curve.circle": (input, k) => geoOut(k.circleCurveSync(parseNumber(input.radius, 1))),
	"brep.curve.ellipse": (input, k) => geoOut(k.ellipseCurveSync(parseNumber(input.major, 1), parseNumber(input.minor, 0.5))),
	"brep.curve.helix": (input, k) => geoOut(k.helixSync(parseNumber(input.radius, 1), parseNumber(input.pitch, 1), parseNumber(input.height, 3))),
	"brep.curve.threePointArc": (input, k) => geoOut(k.threePointArcSync(parseVec3Input(input, "a"), parseVec3Input(input, "b", [1, 0, 0]), parseVec3Input(input, "c", [1, 1, 0]))),
	"brep.curve.tangentArc": (input, k) => geoOut(k.tangentArcSync(parseVec3Input(input, "start"), parseVec3Input(input, "tangent", [1, 0, 0]), parseVec3Input(input, "end", [1, 1, 0]))),
	"brep.curve.ellipseArc": (input, k) => geoOut(k.ellipseArcSync(parseNumber(input.major, 1), parseNumber(input.minor, 0.5), parseNumber(input.startAngle), parseNumber(input.endAngle, Math.PI))),
	"brep.curve.interpolate": (input, k) => geoOut(k.interpolateCurveSync([parseVec3Input(input, "geometry", [0, 0, 0]), [1, 0, 0], [1, 1, 0]])),
	"brep.curve.wire": (input, k) => {
		const edge = parseGeometry(input, "geometry");
		if (!edge) throw new Error("missing geometry");
		return geoOut(k.wireSync([edge]));
	},
	"brep.curve.wireLoop": (input, k) => {
		const edge = parseGeometry(input, "geometry");
		if (!edge) throw new Error("missing geometry");
		return geoOut(k.wireLoopSync([edge]));
	},
	"brep.surface.face": (input, k) => {
		const wire = parseGeometry(input, "geometry");
		if (!wire) throw new Error("missing geometry");
		return geoOut(k.faceSync([wire]));
	},
	"brep.surface.filledFace": (input, k) => {
		const wire = parseGeometry(input, "geometry");
		if (!wire) throw new Error("missing geometry");
		return geoOut(k.filledFaceSync(wire));
	},
	"brep.surface.fill": (input, k) => {
		const edge = parseGeometry(input, "geometry");
		if (!edge) throw new Error("missing geometry");
		return geoOut(k.fillSync([edge]));
	},
	"brep.surface.offsetFace": (input, k) => {
		const face = parseGeometry(input, "geometry");
		if (!face) throw new Error("missing geometry");
		return geoOut(k.offsetFaceSync(face, parseNumber(input.distance, 0.1)));
	},
	"brep.solid.extrude": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.extrudeSync(shape, [0, 0, 1], parseNumber(input.distance, 1)));
	},
	"brep.solid.revolve": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.revolveSync(shape, [0, 0, 1], parseNumber(input.angle, Math.PI * 2)));
	},
	"brep.solid.loft": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing loft sections");
		return geoOut(k.loftSync([a, b]));
	},
	"brep.solid.sweep": (input, k) => {
		const profile = parseGeometry(input, "profile");
		const path = parseGeometry(input, "path");
		if (!profile || !path) throw new Error("missing sweep inputs");
		return geoOut(k.sweepSync(profile, path));
	},
	"brep.solid.fillet": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.filletSync(shape, parseNumber(input.radius, 0.1)));
	},
	"brep.solid.chamfer": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.chamferSync(shape, parseNumber(input.distance, 0.1)));
	},
	"brep.solid.shell": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.shellSync(shape, parseNumber(input.thickness, 0.1)));
	},
	"brep.solid.offset": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.offsetSync(shape, parseNumber(input.distance, 0.1)));
	},
	"brep.solid.thicken": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.thickenSync(shape, parseNumber(input.thickness, 0.1)));
	},
	"brep.solid.hull": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing hull inputs");
		return geoOut(k.hullSync([a, b]));
	},
	"brep.solid.minkowski": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing minkowski inputs");
		return geoOut(k.minkowskiSync(a, b));
	},
	"brep.solid.convexHull": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.convexHullSync([shape]));
	},
	"brep.bool.fuse": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing fuse inputs");
		return geoOut(k.fuseSync(a, b));
	},
	"brep.bool.cut": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing cut inputs");
		return geoOut(k.cutSync(a, b));
	},
	"brep.bool.intersect": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing intersect inputs");
		return geoOut(k.intersectSync(a, b));
	},
	"brep.bool.fuseAll": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing fuseAll inputs");
		return geoOut(k.fuseAllSync([a, b]));
	},
	"brep.bool.fuse2d": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing fuse2d inputs");
		return geoOut(k.fuse2DSync(a, b));
	},
	"brep.bool.cut2d": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing cut2d inputs");
		return geoOut(k.cut2DSync(a, b));
	},
	"brep.bool.intersect2d": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing intersect2d inputs");
		return geoOut(k.intersect2DSync(a, b));
	},
	"brep.xform.translate": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.translateGeomSync(shape, parseVec3Input(input, "offset", [1, 0, 0])));
	},
	"brep.xform.rotate": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.rotateGeomSync(shape, [0, 0, 1], parseNumber(input.angle, Math.PI / 4)));
	},
	"brep.xform.mirror": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.mirrorGeomSync(shape, [0, 0, 0], [1, 0, 0]));
	},
	"brep.xform.scale": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.scaleGeomSync(shape, parseNumber(input.factor, 2)));
	},
	"brep.xform.clone": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.cloneGeomSync(shape));
	},
	"brep.xform.linearPattern": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.linearPatternSync(shape, [1, 0, 0], parseNumber(input.count, 3), parseNumber(input.spacing, 2)));
	},
	"brep.xform.circularPattern": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.circularPatternSync(shape, [0, 0, 1], parseNumber(input.count, 4), parseNumber(input.angle, Math.PI * 2)));
	},
	"brep.xform.rectangularPattern": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.rectangularPatternSync(shape, [1, 0, 0], parseNumber(input.countA, 2), [0, 1, 0], parseNumber(input.countB, 2), parseNumber(input.spacing, 2)));
	},
	"brep.intersect.section": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing section inputs");
		return geoOut(k.sectionSync(a, b));
	},
	"brep.intersect.sectionToFace": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing sectionToFace inputs");
		return geoOut(k.sectionToFaceSync(a, b));
	},
	"brep.intersect.slice": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.sliceSync(shape, [0, 0, 0], [0, 0, 1]));
	},
	"brep.intersect.check": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing interference inputs");
		return { number: k.checkInterferenceSync(a, b) ? 1 : 0 };
	},
	"brep.eval.pointOnCurve": (input, k) => {
		const curve = parseGeometry(input, "geometry");
		if (!curve) throw new Error("missing geometry");
		return vec3PortOut("point", k.curvePointAtSync(curve, parseNumber(input.t, 0.5)));
	},
	"brep.eval.tangentOnCurve": (input, k) => {
		const curve = parseGeometry(input, "geometry");
		if (!curve) throw new Error("missing geometry");
		return vec3PortOut("vector", k.curveTangentAtSync(curve, parseNumber(input.t, 0.5)));
	},
	"brep.eval.curveLength": (input, k) => {
		const curve = parseGeometry(input, "geometry");
		if (!curve) throw new Error("missing geometry");
		return { number: k.curveLengthSync(curve) };
	},
	"brep.eval.pointOnSurface": (input, k) => {
		const face = parseGeometry(input, "geometry");
		if (!face) throw new Error("missing geometry");
		return vec3PortOut("point", k.pointOnSurfaceSync(face, parseNumber(input.u, 0.5), parseNumber(input.v, 0.5)));
	},
	"brep.eval.normalAt": (input, k) => {
		const face = parseGeometry(input, "geometry");
		if (!face) throw new Error("missing geometry");
		return vec3PortOut("vector", k.normalAtSync(face, parseNumber(input.u, 0.5), parseNumber(input.v, 0.5)));
	},
	"brep.eval.faceCenter": (input, k) => {
		const face = parseGeometry(input, "geometry");
		if (!face) throw new Error("missing geometry");
		return vec3PortOut("point", k.faceCenterSync(face));
	},
	"brep.measure.volume": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return { number: k.measureVolumeSync(shape) };
	},
	"brep.measure.area": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return { number: k.measureAreaSync(shape) };
	},
	"brep.measure.length": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return { number: k.measureLengthSync(shape) };
	},
	"brep.measure.distance": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing distance inputs");
		return { number: k.measureDistanceSync(a, b) };
	},
	"brep.query.bounds": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		const bounds = k.getBoundsSync(shape);
		return { dictionary: { min: bounds.min, max: bounds.max } };
	},
	"brep.query.edges": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return { list: k.getEdgesSync(shape).map(String) };
	},
	"brep.query.faces": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return { list: k.getFacesSync(shape).map(String) };
	},
	"brep.repair.heal": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.healSolidSync(shape));
	},
	"brep.repair.autoHeal": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.autoHealSync(shape));
	},
	"brep.repair.solidFromShell": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.solidFromShellSync(shape));
	},
	"brep.io.exportStep": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		const bytes = k.exportStepSync(shape);
		return { text: bytesToBase64(bytes) };
	},
	"brep.io.exportStl": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		const bytes = k.exportStlSync(shape);
		return { text: bytesToBase64(bytes) };
	},
	"brep.gear.external": (input, k) => geoOut(k.makeExternalGearSync(parseNumber(input.teeth, 20), parseNumber(input.module, 2))),
	"brep.gear.internal": (input, k) => geoOut(k.makeInternalGearSync(parseNumber(input.teeth, 20), parseNumber(input.module, 2))),
	"brep.box": (input, k) => geoOut(k.createBoxFromCornersSync({ cornerA: parseVec3(input.cornerA), cornerB: parseVec3(input.cornerB, [1, 1, 0]), height: parseNumber(input.height, 1) }) as GeometryRef),
	"brep.sphere": (input, k) => geoOut(k.createSphereSync(parseVec3(input.center), parseNumber(input.radius, 1)) as GeometryRef),
	"brep.cylinder": (input, k) => geoOut(k.createCylinderSync(parseVec3(input.base), parseVec3(input.axis, [0, 0, 1]), parseNumber(input.radius, 1), parseNumber(input.height, 1)) as GeometryRef),
	"brep.extrude": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.extrudeSync(shape, parseVec3(input.direction, [0, 0, 1]), parseNumber(input.distance, 1)));
	},
	"brep.translate": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.translateGeomSync(shape, parseVec3(input.offset)));
	},
	"brep.union": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing union inputs");
		return geoOut(k.fuseAllSync([a, b]));
	},
};

/** @emoji 🧊 Synchronous brep neuron evaluation (requires pre-initialized WASM). */
export function evaluateBrepFlowKind(kindId: string, inputJson: string, kernel: BrepKernelType): string {
	const input = JSON.parse(inputJson) as Record<string, unknown>;
	try {
		const handler = BREP_EVAL_HANDLERS[kindId];
		if (!handler) return JSON.stringify({ error: `unknown brep kind: ${kindId}` });
		return JSON.stringify(handler(input, kernel));
	} catch (e) {
		return JSON.stringify({ error: e instanceof Error ? e.message : String(e) });
	}
}

/** @emoji 🔌 Flow extension host with brep JS module + default flow modules. */
export class ProceduralExtensionHost extends FlowExtensionHost {
	private brepKernel: BrepKernelType;
	private brepReady = false;

	constructor(kernel: BrepKernelType = new BrepjsGeometryKernel()) {
		super();
		this.brepKernel = kernel;
		this.brepReady = import.meta.env.VITEST === true;
	}

	getBrepKernel(): BrepKernelType {
		return this.brepKernel;
	}

	async activateDefaults(): Promise<void> {
		await ensureBrepWasmLoaded();
		this.brepReady = true;
		await super.activateDefaults();
	}

	override evaluate(kindId: string, inputJson: string): string {
		if (kindId.startsWith("brep.")) {
			if (!this.brepReady) return JSON.stringify({ error: "brep wasm not ready" });
			return evaluateBrepFlowKind(kindId, inputJson, this.brepKernel);
		}
		return super.evaluate(kindId, inputJson);
	}

	override catalogueSections(): CatalogueSection[] {
		const sections = [...super.catalogueSections()];
		for (const section of BREP_CATALOGUE_SECTIONS) {
			sections.push({
				id: section.id,
				title: section.title,
				items: section.kinds.map((kind) => ({
					kind: "neuron",
					neuronKind: kind.id,
					name: kind.name,
					abbreviation: kind.abbreviation,
					icon: kind.icon,
					summary: kind.summary,
				})),
			});
		}
		return sections;
	}

	override kindInfosJson(): string {
		const kinds = JSON.parse(super.kindInfosJson()) as FlowModuleNeuronKindV1[];
		return JSON.stringify([...kinds, ...BREP_FLOW_KINDS]);
	}

	override listEntries(): readonly FlowExtensionEntry[] {
		return [
			...super.listEntries(),
			{ id: "brep", manifest: BREP_MODULE_MANIFEST, active: this.brepReady },
		];
	}
}

export const proceduralExtensionHost = new ProceduralExtensionHost();
// #endregion 🔖BrepFlowModule

// #region 🔖Fixture
export const PROCEDURAL_DEFAULT_FIXTURE: FlowFixtureV1 = {
	schema: "flow.fixture/v1",
	camera: { x: 0, y: 0, zoom: 1 },
	widgets: [
		{ kind: "neuron", id: "sketch", neuronKind: "brep.sketch2d.rectangle" },
		{ kind: "neuron", id: "solid", neuronKind: "brep.solid.extrude" },
		{ kind: "outputPreview", id: "preview" },
	],
	synapses: [
		{ id: "s1", from: "sketch", to: "solid", fromPort: "out", toPort: "in" },
		{ id: "s2", from: "solid", to: "preview", fromPort: "out", toPort: "in" },
	],
};

export function proceduralFixtureToJson(fixture: FlowFixtureV1 = PROCEDURAL_DEFAULT_FIXTURE): string {
	return JSON.stringify(fixture);
}
// #endregion 🔖Fixture

// #region 🔖BrepViewport
export type ProceduralPreviewItem =
	| { readonly widgetId: string; readonly kind: "geometry"; readonly handle: GeometryRef }
	| { readonly widgetId: string; readonly kind: "point"; readonly position: Vec3 }
	| { readonly widgetId: string; readonly kind: "vector"; readonly direction: Vec3 };

/** @deprecated Use {@link ProceduralPreviewItem} with `kind: "geometry"`. */
export type ProceduralGeometryHandle = Extract<ProceduralPreviewItem, { kind: "geometry" }>;

export type ProceduralPreviewShowMode = "everything" | "selected";
export type ProceduralSelectionMode = SelectionMergeMode;
export type ProceduralSelectionMethod = "rectangle" | "lasso";

export interface ProceduralPreviewProps {
	readonly items: readonly ProceduralPreviewItem[];
	readonly selectedNodeIds?: readonly string[];
	readonly preselectNodeIds?: readonly string[];
	readonly preselectRemovedNodeIds?: readonly string[];
	readonly hoveredNodeId?: string | null;
	readonly previewOffNodeIds?: readonly string[];
	readonly showMode?: ProceduralPreviewShowMode;
	readonly selectionMode?: ProceduralSelectionMode;
	readonly selectionMethod?: ProceduralSelectionMethod;
	readonly onHover?: (widgetId: string | null) => void;
	readonly onSelect?: (widgetId: string) => void;
	readonly onSelectionChange?: (ids: readonly string[], mode: ProceduralSelectionMode) => void;
	readonly kernel?: BrepKernelType;
	readonly tolerance?: number;
	readonly className?: string;
}

const PROCEDURAL_PREVIEW_MARQUEE_THRESHOLD_PX = 4;

/** @deprecated Use {@link ProceduralPreview} with {@link ProceduralPreviewItem} entries. */
export interface BrepViewportProps {
	readonly geometryIds: readonly string[];
	readonly kernel?: BrepKernelType;
	readonly tolerance?: number;
	readonly className?: string;
}

interface BrepMeshBuffers {
	readonly surface: THREE.BufferGeometry | null;
	readonly lines: THREE.BufferGeometry | null;
	readonly points: THREE.BufferGeometry | null;
}

function buildMeshBuffers(data: ReturnType<typeof meshTransferToGeometryData>): BrepMeshBuffers {
	let surface: THREE.BufferGeometry | null = null;
	if (data.position.length > 0 && data.index.length > 0) {
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute("position", new THREE.Float32BufferAttribute(data.position, 3));
		geometry.setAttribute("normal", new THREE.Float32BufferAttribute(data.normal, 3));
		geometry.setIndex(new THREE.BufferAttribute(data.index, 1));
		for (const g of data.faceGroups) geometry.addGroup(g.start, g.count, 0);
		surface = geometry;
	}
	let lines: THREE.BufferGeometry | null = null;
	if (data.edges.length > 0) {
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute("position", new THREE.Float32BufferAttribute(data.edges, 3));
		lines = geometry;
	}
	let points: THREE.BufferGeometry | null = null;
	if (data.points.length > 0) {
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute("position", new THREE.Float32BufferAttribute(data.points, 3));
		points = geometry;
	}
	return { surface, lines, points };
}

const PROCEDURAL_PREVIEW_POINT_RADIUS = 0.08;
const PROCEDURAL_PREVIEW_BOUNDS_PAD = 0.05;
const PROCEDURAL_GEOMETRY_REF_PATTERN = /^(vertex|edge|wire|face|shell|solid|compound|drawing)-/;

function parsePreviewVec3(input: unknown): Vec3 | null {
	return parseVec3Loose(input);
}

function collectGeometryRefsFromValue(value: unknown, refs: GeometryRef[]): void {
	if (typeof value === "string" && PROCEDURAL_GEOMETRY_REF_PATTERN.test(value)) {
		refs.push(value as GeometryRef);
		return;
	}
	if (!value || typeof value !== "object" || Array.isArray(value)) return;
	for (const nested of Object.values(value as Record<string, unknown>)) {
		collectGeometryRefsFromValue(nested, refs);
	}
}

function previewLayerChrome(
	previewOff: boolean,
	hovered: boolean,
	selected: boolean,
	highlighted: boolean,
): { readonly renderMode: ReturnType<typeof worldEntityRenderMode>; readonly opacity: number; readonly emissive: string; readonly emissiveIntensity: number } {
	const renderMode = worldEntityRenderMode({ hidden: previewOff }, { hovered, selected: selected || highlighted, revealed: hovered });
	const opacity = renderMode.dim ? 0.35 : renderMode.asHover ? 0.95 : 0.85;
	const emissive = highlighted ? "#a78bfa" : renderMode.showSelectedOutline ? "#3b82f6" : renderMode.asHover ? "#60a5fa" : "#000000";
	const emissiveIntensity = highlighted ? 0.28 : renderMode.showSelectedOutline ? 0.35 : renderMode.asHover ? 0.2 : 0;
	return { renderMode, opacity, emissive, emissiveIntensity };
}

function createPreviewPointerHandlers(
	widgetId: string,
	onHover?: (widgetId: string | null) => void,
	onPick?: (widgetId: string, mode: ProceduralSelectionMode) => void,
) {
	if (!onHover && !onPick) return {};
	return {
		onPointerDown: (event: { stopPropagation: () => void; nativeEvent: PointerEvent }) => {
			if (event.nativeEvent.button !== 0) return;
			event.stopPropagation();
		},
		onPointerOver: (event: { stopPropagation: () => void }) => {
			event.stopPropagation();
			onHover?.(widgetId);
		},
		onPointerOut: (event: { stopPropagation: () => void }) => {
			event.stopPropagation();
			onHover?.(null);
		},
		onClick: (event: { stopPropagation: () => void; shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
			event.stopPropagation();
			const mode = marqueeModeFromModifiers(event);
			onPick?.(widgetId, mode);
		},
	};
}

function worldBoundsForPreviewItem(item: ProceduralPreviewItem, kernel: BrepKernelType): { min: Vec3; max: Vec3 } | null {
	const pad = PROCEDURAL_PREVIEW_BOUNDS_PAD;
	if (item.kind === "geometry") {
		try {
			return kernel.getBoundsSync(item.handle);
		} catch {
			return null;
		}
	}
	if (item.kind === "point") {
		const [x, y, z] = item.position;
		return { min: [x - pad, y - pad, z - pad], max: [x + pad, y + pad, z + pad] };
	}
	const [x, y, z] = item.direction;
	return {
		min: [Math.min(0, x) - pad, Math.min(0, y) - pad, Math.min(0, z) - pad],
		max: [Math.max(0, x) + pad, Math.max(0, y) + pad, Math.max(0, z) + pad],
	};
}

function previewItemKey(item: ProceduralPreviewItem): string {
	if (item.kind === "geometry") return `${item.widgetId}:geometry:${item.handle}`;
	if (item.kind === "point") return `${item.widgetId}:point`;
	return `${item.widgetId}:vector`;
}

function BrepPointLayer({
	widgetId,
	position,
	color,
	selected,
	highlighted,
	hovered,
	previewOff,
	onHover,
	onPick,
}: {
	readonly widgetId: string;
	readonly position: Vec3;
	readonly color: string;
	readonly selected: boolean;
	readonly highlighted: boolean;
	readonly hovered: boolean;
	readonly previewOff: boolean;
	readonly onHover?: (widgetId: string | null) => void;
	readonly onPick?: (widgetId: string, mode: ProceduralSelectionMode) => void;
}): ReactNode {
	const { renderMode, opacity, emissive, emissiveIntensity } = previewLayerChrome(previewOff, hovered, selected, highlighted);
	if (!renderMode.visible) return null;
	const radius = renderMode.asHover ? PROCEDURAL_PREVIEW_POINT_RADIUS * 1.25 : PROCEDURAL_PREVIEW_POINT_RADIUS;
	return (
		<group position={position} {...createPreviewPointerHandlers(widgetId, onHover, onPick)}>
			<mesh>
				<sphereGeometry args={[radius, 16, 12]} />
				<meshStandardMaterial color={color} emissive={emissive} emissiveIntensity={emissiveIntensity} metalness={0.1} roughness={0.5} transparent opacity={opacity} />
			</mesh>
		</group>
	);
}

function BrepVectorLayer({
	widgetId,
	direction,
	color,
	selected,
	highlighted,
	hovered,
	previewOff,
	onHover,
	onPick,
}: {
	readonly widgetId: string;
	readonly direction: Vec3;
	readonly color: string;
	readonly selected: boolean;
	readonly highlighted: boolean;
	readonly hovered: boolean;
	readonly previewOff: boolean;
	readonly onHover?: (widgetId: string | null) => void;
	readonly onPick?: (widgetId: string, mode: ProceduralSelectionMode) => void;
}): ReactNode {
	const { renderMode, opacity, emissive, emissiveIntensity } = previewLayerChrome(previewOff, hovered, selected, highlighted);
	const arrow = useMemo(() => {
		const tip = new THREE.Vector3(direction[0], direction[1], direction[2]);
		const length = tip.length();
		if (length < 1e-6) return null;
		const unit = tip.clone().normalize();
		const shaftEnd = unit.clone().multiplyScalar(length * 0.85);
		const shaft = new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(0, 0, 0), shaftEnd]);
		const headHeight = length * 0.15;
		const headRadius = headHeight * 0.35;
		const head = new THREE.ConeGeometry(headRadius, headHeight, 10);
		const quaternion = new THREE.Quaternion().setFromUnitVectors(new THREE.Vector3(0, 1, 0), unit);
		const headPosition = unit.clone().multiplyScalar(length - headHeight * 0.5);
		return { shaft, head, headPosition, quaternion };
	}, [direction]);
	if (!renderMode.visible || !arrow) return null;
	const lineColor = renderMode.asHover ? "#93c5fd" : "#e2e8f0";
	return (
		<group {...createPreviewPointerHandlers(widgetId, onHover, onPick)}>
			<line geometry={arrow.shaft}>
				<lineBasicMaterial color={lineColor} linewidth={1} transparent opacity={opacity} />
			</line>
			<mesh geometry={arrow.head} position={arrow.headPosition} quaternion={arrow.quaternion}>
				<meshStandardMaterial color={color} emissive={emissive} emissiveIntensity={emissiveIntensity} metalness={0.1} roughness={0.5} transparent opacity={opacity} />
			</mesh>
		</group>
	);
}

function BrepGeometryLayer({
	widgetId,
	geometryId,
	kernel,
	tolerance,
	color,
	selected,
	highlighted,
	hovered,
	previewOff,
	onHover,
	onPick,
}: {
	readonly widgetId: string;
	readonly geometryId: string;
	readonly kernel: BrepKernelType;
	readonly tolerance: number;
	readonly color: string;
	readonly selected: boolean;
	readonly highlighted: boolean;
	readonly hovered: boolean;
	readonly previewOff: boolean;
	readonly onHover?: (widgetId: string | null) => void;
	readonly onPick?: (widgetId: string, mode: ProceduralSelectionMode) => void;
}): ReactNode {
	const [buffers, setBuffers] = useState<BrepMeshBuffers>({ surface: null, lines: null, points: null });
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	const ref = geometryId as GeometryRef;
	const { renderMode, opacity, emissive, emissiveIntensity } = previewLayerChrome(previewOff, hovered, selected, highlighted);

	useEffect(() => {
		let cancelled = false;
		void (async () => {
			await ensureBrepWasmLoaded();
			const mesh = await kernel.tessellateGeometry(ref, tolerance);
			if (cancelled) return;
			if (!isRenderableMeshTransfer(mesh)) {
				setBuffers({ surface: null, lines: null, points: null });
				invalidate();
				return;
			}
			setBuffers(buildMeshBuffers(meshTransferToGeometryData(mesh)));
			invalidate();
		})();
		return () => {
			cancelled = true;
		};
	}, [invalidate, kernel, ref, tolerance]);

	if (!renderMode.visible) return null;

	return (
		<group {...createPreviewPointerHandlers(widgetId, onHover, onPick)}>
			{buffers.surface ? (
				<mesh geometry={buffers.surface}>
					<meshStandardMaterial color={color} emissive={emissive} emissiveIntensity={emissiveIntensity} metalness={0.1} roughness={0.6} transparent opacity={opacity} side={THREE.DoubleSide} />
				</mesh>
			) : null}
			{buffers.lines ? (
				<lineSegments geometry={buffers.lines}>
					<lineBasicMaterial color={renderMode.asHover ? "#93c5fd" : "#e2e8f0"} linewidth={1} transparent opacity={opacity} />
				</lineSegments>
			) : null}
			{buffers.points ? (
				<points geometry={buffers.points}>
					<pointsMaterial color={renderMode.asHover ? "#fde68a" : "#fbbf24"} size={renderMode.asHover ? 0.16 : 0.12} transparent opacity={opacity} />
				</points>
			) : null}
		</group>
	);
}

const BREP_VIEWPORT_COLORS = ["#6b9bd1", "#7ec8a3", "#d18b6b", "#b16bd1", "#d1c46b"];

export const PROCEDURAL_PREVIEW_DEFAULT_CAMERA: WorldCameraState = {
	position: [8, 8, 6],
	target: [0, 0, 0],
	zoom: 1,
	up: [0, 0, 1],
	projection: "perspective",
};

export function proceduralPreviewCameraSeed(seed: number): string {
	return String(seed);
}

type ScreenBounds = { readonly left: number; readonly top: number; readonly right: number; readonly bottom: number };

function projectWorldBoundsToScreen(bounds: { min: Vec3; max: Vec3 }, camera: THREE.Camera, width: number, height: number): ScreenBounds | null {
	const corners: Vec3[] = [
		[bounds.min[0], bounds.min[1], bounds.min[2]],
		[bounds.max[0], bounds.min[1], bounds.min[2]],
		[bounds.min[0], bounds.max[1], bounds.min[2]],
		[bounds.max[0], bounds.max[1], bounds.min[2]],
		[bounds.min[0], bounds.min[1], bounds.max[2]],
		[bounds.max[0], bounds.min[1], bounds.max[2]],
		[bounds.min[0], bounds.max[1], bounds.max[2]],
		[bounds.max[0], bounds.max[1], bounds.max[2]],
	];
	const vector = new THREE.Vector3();
	let left = Number.POSITIVE_INFINITY;
	let top = Number.POSITIVE_INFINITY;
	let right = Number.NEGATIVE_INFINITY;
	let bottom = Number.NEGATIVE_INFINITY;
	for (const corner of corners) {
		vector.set(corner[0], corner[1], corner[2]).project(camera);
		const x = ((vector.x + 1) / 2) * width;
		const y = ((-vector.y + 1) / 2) * height;
		left = Math.min(left, x);
		top = Math.min(top, y);
		right = Math.max(right, x);
		bottom = Math.max(bottom, y);
	}
	if (!Number.isFinite(left) || !Number.isFinite(top) || !Number.isFinite(right) || !Number.isFinite(bottom)) return null;
	return { left, top, right, bottom };
}

function ProceduralPreviewCameraBridge({
	onCamera,
}: {
	readonly onCamera: (camera: THREE.Camera, size: { width: number; height: number }) => void;
}): null {
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	const camera = sceneHostPort.fiber.useThree((state) => state.camera);
	const size = sceneHostPort.fiber.useThree((state) => state.size);
	useEffect(() => {
		onCamera(camera, size);
		invalidate();
	}, [camera, invalidate, onCamera, size, size.height, size.width]);
	return null;
}

function ProceduralPreviewMarqueeBridge({
	containerRef,
	selectionMethod,
	selectedNodeIds,
	resolveMarqueeHits,
	commitSelection,
	onMarqueeOverlay,
	onLivePreselect,
}: {
	readonly containerRef: React.RefObject<HTMLDivElement | null>;
	readonly selectionMethod: ProceduralSelectionMethod;
	readonly selectedNodeIds: readonly string[];
	readonly resolveMarqueeHits: (points: readonly { x: number; y: number }[], crossing: boolean) => string[];
	readonly commitSelection: (ids: readonly string[], mode: ProceduralSelectionMode) => void;
	readonly onMarqueeOverlay: (overlay: {
		coverage: SelectionMarqueeCoverage;
		shape: "rect" | "polygon";
		rect?: { x: number; y: number; width: number; height: number };
		points?: readonly { x: number; y: number }[];
	} | null) => void;
	readonly onLivePreselect: (snapshot: { ids: string[]; removedIds: string[] }) => void;
}): null {
	const gl = sceneHostPort.fiber.useThree((state) => state.gl);
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	const marqueeRef = useRef<{ tracking: boolean; active: boolean; start: { x: number; y: number }; points: { x: number; y: number }[]; initial: string[] }>({
		tracking: false,
		active: false,
		start: { x: 0, y: 0 },
		points: [],
		initial: [],
	});
	const resolveMarqueeHitsRef = useRef(resolveMarqueeHits);
	const commitSelectionRef = useRef(commitSelection);
	const onMarqueeOverlayRef = useRef(onMarqueeOverlay);
	const onLivePreselectRef = useRef(onLivePreselect);
	const selectionMethodRef = useRef(selectionMethod);
	const selectedNodeIdsRef = useRef(selectedNodeIds);
	resolveMarqueeHitsRef.current = resolveMarqueeHits;
	commitSelectionRef.current = commitSelection;
	onMarqueeOverlayRef.current = onMarqueeOverlay;
	onLivePreselectRef.current = onLivePreselect;
	selectionMethodRef.current = selectionMethod;
	selectedNodeIdsRef.current = selectedNodeIds;

	const clientToLocal = useCallback(
		(clientX: number, clientY: number) => {
			const host = containerRef.current;
			if (!host) return { x: clientX, y: clientY };
			const rect = host.getBoundingClientRect();
			return { x: clientX - rect.left, y: clientY - rect.top };
		},
		[containerRef],
	);

	useEffect(() => {
		const canvas = gl.domElement;
		if (!canvas) return;
		const resetGesture = () => {
			marqueeRef.current = { tracking: false, active: false, start: { x: 0, y: 0 }, points: [], initial: [] };
			onMarqueeOverlayRef.current(null);
			onLivePreselectRef.current({ ids: [], removedIds: [] });
		};
		const onPointerDown = (event: PointerEvent) => {
			if (event.button !== 0) return;
			if ((event.target as HTMLElement | null)?.closest("[data-world-projection-switch]")) return;
			const point = clientToLocal(event.clientX, event.clientY);
			marqueeRef.current = { tracking: true, active: false, start: point, points: [point], initial: [...selectedNodeIdsRef.current] };
			onMarqueeOverlayRef.current(null);
			onLivePreselectRef.current({ ids: [], removedIds: [] });
		};
		const onPointerMove = (event: PointerEvent) => {
			if (!marqueeRef.current.tracking) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const start = marqueeRef.current.start;
			const distance = Math.hypot(point.x - start.x, point.y - start.y);
			if (!marqueeRef.current.active && distance < PROCEDURAL_PREVIEW_MARQUEE_THRESHOLD_PX) return;
			marqueeRef.current.active = true;
			const points = selectionMethodRef.current === "lasso" ? [...marqueeRef.current.points, point] : [start, point];
			marqueeRef.current.points = points;
			const coverage = marqueeCoverageFromDrag(start.x, point.x);
			if (selectionMethodRef.current === "lasso" && points.length >= 3) {
				onMarqueeOverlayRef.current({ coverage, shape: "polygon", points });
			} else {
				const rect = screenRectFromPoints(points);
				if (rect) onMarqueeOverlayRef.current({ coverage, shape: "rect", rect });
			}
			const mode = marqueeModeFromModifiers(event);
			const hits = resolveMarqueeHitsRef.current(points, coverage === "partial");
			const merged = selectionMergeIds(mode, marqueeRef.current.initial, hits);
			const removed = marqueeRef.current.initial.filter((id) => !merged.includes(id));
			onLivePreselectRef.current({ ids: merged.filter((id) => !marqueeRef.current.initial.includes(id)), removedIds: removed });
			invalidate();
		};
		const onPointerUp = (event: PointerEvent) => {
			if (!marqueeRef.current.tracking) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const start = marqueeRef.current.start;
			const distance = Math.hypot(point.x - start.x, point.y - start.y);
			const mode = marqueeModeFromModifiers(event);
			if (marqueeRef.current.active && distance >= PROCEDURAL_PREVIEW_MARQUEE_THRESHOLD_PX) {
				const points = selectionMethodRef.current === "lasso" ? [...marqueeRef.current.points, point] : [start, point];
				const coverage = marqueeCoverageFromDrag(start.x, point.x);
				const hits = resolveMarqueeHitsRef.current(points, coverage === "partial");
				const next = selectionMergeIds(mode, marqueeRef.current.initial, hits);
				commitSelectionRef.current(next, mode);
			}
			resetGesture();
			invalidate();
		};
		const bindings = new WorldEventBindingController();
		bindings.listen(canvas, "pointerdown", onPointerDown as EventListener);
		bindings.listen(window, "pointermove", onPointerMove as EventListener);
		bindings.listen(window, "pointerup", onPointerUp as EventListener, true);
		bindings.listen(window, "pointercancel", onPointerUp as EventListener, true);
		return () => bindings.dispose();
	}, [clientToLocal, gl, invalidate]);

	return null;
}

export function ProceduralPreview({
	items,
	selectedNodeIds = [],
	preselectNodeIds = [],
	preselectRemovedNodeIds = [],
	hoveredNodeId = null,
	previewOffNodeIds = [],
	showMode = "everything",
	selectionMode = "default",
	selectionMethod = "rectangle",
	onHover,
	onSelect,
	onSelectionChange,
	kernel = brepjsGeometryKernel,
	tolerance = 0.02,
	className,
}: ProceduralPreviewProps): ReactNode {
	const containerRef = useRef<HTMLDivElement>(null);
	const cameraRef = useRef<THREE.Camera | null>(null);
	const sizeRef = useRef({ width: 1, height: 1 });
	const lodRef = useRef(DEFAULT_MANUAL_LOD);
	const [camera, setCamera] = useState<WorldCameraState>(PROCEDURAL_PREVIEW_DEFAULT_CAMERA);
	const [cameraSeed, setCameraSeed] = useState(0);
	const cameraSeedKey = proceduralPreviewCameraSeed(cameraSeed);
	const projection = camera.projection ?? "perspective";
	const [marqueeOverlay, setMarqueeOverlay] = useState<{
		coverage: SelectionMarqueeCoverage;
		shape: "rect" | "polygon";
		rect?: { x: number; y: number; width: number; height: number };
		points?: readonly { x: number; y: number }[];
	} | null>(null);
	const [livePreselect, setLivePreselect] = useState<{ ids: string[]; removedIds: string[] }>({ ids: [], removedIds: [] });

	const visibleItems =
		showMode === "selected" ? items.filter((entry) => selectedNodeIds.includes(entry.widgetId)) : items;

	const effectiveSelected = useMemo(() => new Set(selectedNodeIds), [selectedNodeIds]);
	const effectivePreselect = useMemo(() => new Set(livePreselect.ids.length ? livePreselect.ids : preselectNodeIds), [livePreselect.ids, preselectNodeIds]);
	const effectivePreselectRemoved = useMemo(
		() => new Set(livePreselect.removedIds.length ? livePreselect.removedIds : preselectRemovedNodeIds),
		[livePreselect.removedIds, preselectRemovedNodeIds],
	);

	const handleCamera = useCallback((camera: THREE.Camera, size: { width: number; height: number }) => {
		cameraRef.current = camera;
		sizeRef.current = { width: size.width, height: size.height };
	}, []);

	const screenBoundsForItem = useCallback(
		(item: ProceduralPreviewItem): ScreenBounds | null => {
			const camera = cameraRef.current;
			if (!camera) return null;
			const bounds = worldBoundsForPreviewItem(item, kernel);
			if (!bounds) return null;
			return projectWorldBoundsToScreen(bounds, camera, sizeRef.current.width, sizeRef.current.height);
		},
		[kernel],
	);

	const resolveMarqueeHits = useCallback(
		(points: readonly { x: number; y: number }[], crossing: boolean): string[] => {
			const marqueeRect = screenRectFromPoints(points);
			if (!marqueeRect) return [];
			const hits: string[] = [];
			for (const entry of visibleItems) {
				const bounds = screenBoundsForItem(entry);
				if (!bounds) continue;
				const target = { x: bounds.left, y: bounds.top, width: bounds.right - bounds.left, height: bounds.bottom - bounds.top };
				const marquee = { x: marqueeRect.x, y: marqueeRect.y, width: marqueeRect.width, height: marqueeRect.height };
				const contained = screenRectContainsRect(marquee, target);
				const intersects = screenRectIntersectsRect(marquee, target);
				if (crossing ? intersects : contained) hits.push(entry.widgetId);
			}
			return hits;
		},
		[screenBoundsForItem, visibleItems],
	);

	const commitSelection = useCallback(
		(ids: readonly string[], mode: ProceduralSelectionMode) => {
			if (onSelectionChange) {
				onSelectionChange(ids, mode);
				return;
			}
			if (ids.length === 1 && onSelect) {
				onSelect(ids[0]!);
			}
		},
		[onSelect, onSelectionChange],
	);

	const onPick = useCallback(
		(widgetId: string, mode: ProceduralSelectionMode) => {
			const next = selectionMergeIds(mode, selectedNodeIds, [widgetId]);
			commitSelection(next, mode);
		},
		[commitSelection, selectedNodeIds],
	);

	const onProjectionChange = useCallback((nextProjection: OrbitCameraProjection) => {
		setCamera((prev) => applyOrbitProjectionToCameraState(prev, nextProjection));
		setCameraSeed((seed) => seed + 1);
	}, []);

	const onViewportGizmoCameraChange = useCallback((next: WorldCameraState) => {
		setCamera(next);
		setCameraSeed((seed) => seed + 1);
	}, []);

	return (
		<div ref={containerRef} className={`absolute inset-0 min-h-0 min-w-0 bg-zinc-900 ${className ?? ""}`.trim()}>
			<WorldCanvas
				className="h-full w-full"
				frameloop="demand"
				background="#18181b"
				overlay={<WorldOrbitProjectionSwitch projection={projection} onProjectionChange={onProjectionChange} />}
			>
				<WorldLodBridge
					lodRef={lodRef}
					distanceReference={100}
					gridFactor={DEFAULT_LOD_GRID_FACTOR}
					gridSnapEnabled={false}
					showLodGrid
					automaticLod
					depthVariableLod={false}
					manualLod={DEFAULT_MANUAL_LOD}
					gridDatum={[0, 0, 0]}
				>
					<WorldOrbitViewSnapGateProvider>
						<WorldOrbitCameraViewRig state={camera} seedKey={cameraSeedKey} perspectiveFov={45} />
						<WorldOrbitGated controlsKey={cameraSeedKey} projection={projection} zoom={camera.zoom} />
						<WorldOrbitViewControls onCameraChange={onViewportGizmoCameraChange} />
						<ProceduralPreviewCameraBridge onCamera={handleCamera} />
						<ProceduralPreviewMarqueeBridge
							containerRef={containerRef}
							selectionMethod={selectionMethod}
							selectedNodeIds={selectedNodeIds}
							resolveMarqueeHits={resolveMarqueeHits}
							commitSelection={commitSelection}
							onMarqueeOverlay={setMarqueeOverlay}
							onLivePreselect={setLivePreselect}
						/>
						<WorldCameraInvalidator />
						<ambientLight intensity={0.45} />
						<directionalLight position={[12, 18, 10]} intensity={1.1} />
						<WorldLayer order={10} name="procedural.preview">
							{visibleItems.map((entry, index) => {
								const color = BREP_VIEWPORT_COLORS[index % BREP_VIEWPORT_COLORS.length]!;
								const chrome = {
									selected: effectiveSelected.has(entry.widgetId) || effectivePreselect.has(entry.widgetId),
									highlighted: effectivePreselectRemoved.has(entry.widgetId),
									hovered: hoveredNodeId === entry.widgetId,
									previewOff: previewOffNodeIds.includes(entry.widgetId),
									onHover,
									onPick,
								};
								if (entry.kind === "point") {
									return <BrepPointLayer key={previewItemKey(entry)} widgetId={entry.widgetId} position={entry.position} color={color} {...chrome} />;
								}
								if (entry.kind === "vector") {
									return <BrepVectorLayer key={previewItemKey(entry)} widgetId={entry.widgetId} direction={entry.direction} color={color} {...chrome} />;
								}
								return (
									<BrepGeometryLayer
										key={previewItemKey(entry)}
										widgetId={entry.widgetId}
										geometryId={entry.handle}
										kernel={kernel}
										tolerance={tolerance}
										color={color}
										{...chrome}
									/>
								);
							})}
						</WorldLayer>
					</WorldOrbitViewSnapGateProvider>
				</WorldLodBridge>
			</WorldCanvas>
			{marqueeOverlay?.shape === "rect" && marqueeOverlay.rect ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} />
			) : null}
			{marqueeOverlay?.shape === "polygon" && marqueeOverlay.points ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} />
			) : null}
		</div>
	);
}

export function BrepViewport({ geometryIds, kernel = brepjsGeometryKernel, tolerance = 0.02, className }: BrepViewportProps): ReactNode {
	return (
		<ProceduralPreview
			items={geometryIds.map((handle, index) => ({ widgetId: `geometry-${index}`, kind: "geometry" as const, handle: handle as GeometryRef }))}
			kernel={kernel}
			tolerance={tolerance}
			className={className}
		/>
	);
}

/** @emoji 🔍 Collects geometry, point, and vector preview items from flow eval outputs. */
export function extractPreviewItems(outputsJson: string): ProceduralPreviewItem[] {
	const items: ProceduralPreviewItem[] = [];
	try {
		const parsed = JSON.parse(outputsJson) as unknown;
		if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) return items;
		if ("error" in (parsed as Record<string, unknown>) && Object.keys(parsed as object).length === 1) return items;
		const outputs = parsed as Record<string, Record<string, unknown>>;
		for (const [widgetId, dict] of Object.entries(outputs)) {
			if (!dict || typeof dict !== "object" || Array.isArray(dict)) continue;
			if (typeof dict.error === "string") continue;
			const geometryRefs: GeometryRef[] = [];
			collectGeometryRefsFromValue(dict.geometry, geometryRefs);
			collectGeometryRefsFromValue(dict.brep, geometryRefs);
			for (const value of Object.values(dict)) collectGeometryRefsFromValue(value, geometryRefs);
			for (const handle of [...new Set(geometryRefs)]) {
				items.push({ widgetId, kind: "geometry", handle });
			}
			const point = parsePreviewVec3(dict.point);
			if (point) items.push({ widgetId, kind: "point", position: point });
			const vector = parsePreviewVec3(dict.vector);
			if (vector) items.push({ widgetId, kind: "vector", direction: vector });
		}
	} catch {
		/* ignore */
	}
	return items;
}

/** @deprecated Use {@link extractPreviewItems}. */
export function extractGeometryHandles(outputsJson: string): ProceduralGeometryHandle[] {
	return extractPreviewItems(outputsJson).filter((item): item is ProceduralGeometryHandle => item.kind === "geometry");
}
// #endregion 🔖BrepViewport

// #region 🔖ProceduralEditor
export interface ProceduralFlowEditorProps {
	readonly fixtureJson?: string;
	readonly className?: string;
	readonly extensionHost?: ProceduralExtensionHost;
	readonly reorganize?: FlowReorganizeRequest;
	readonly extensionRevision?: number;
	readonly onPreviewText?: (text: string) => void;
	readonly onEvalOutputs?: (outputsJson: string) => void;
	readonly onCatalogueReady?: (sections: readonly CatalogueSection[]) => void;
	readonly onFixtureChange?: (fixtureJson: string) => void;
	readonly onSelectionChange?: (ids: readonly string[]) => void;
	readonly onPreselectChange?: (snapshot: { readonly ids: readonly string[]; readonly removedIds: readonly string[] }) => void;
	readonly onHoverChange?: (id: string | null) => void;
	readonly selectedNodeIds?: readonly string[];
	readonly preselectNodeIds?: readonly string[];
	readonly preselectRemovedNodeIds?: readonly string[];
	readonly hoveredNodeId?: string | null;
	readonly previewOffNodeIds?: readonly string[];
	readonly selectionMode?: ProceduralSelectionMode;
	readonly selectionMethod?: ProceduralSelectionMethod;
	readonly contextMenu?: (ctx: FlowCanvasContextMenuContext) => readonly ContextMenuItem[];
	readonly commandRequest?: FlowCanvasCommandRequest;
	readonly onPreviewOffChange?: (ids: readonly string[]) => void;
	readonly automaticLod?: boolean;
	readonly lod?: DagDrawLodKind;
	readonly onLodChange?: (lod: DagDrawLodKind) => void;
}

export function ProceduralFlowEditor({
	fixtureJson,
	className,
	extensionHost = proceduralExtensionHost,
	reorganize,
	extensionRevision = 0,
	onPreviewText,
	onEvalOutputs,
	onCatalogueReady,
	onFixtureChange,
	onSelectionChange,
	onPreselectChange,
	onHoverChange,
	selectedNodeIds,
	preselectNodeIds,
	preselectRemovedNodeIds,
	hoveredNodeId,
	previewOffNodeIds,
	selectionMode,
	selectionMethod,
	contextMenu,
	commandRequest,
	onPreviewOffChange,
	automaticLod,
	lod,
	onLodChange,
}: ProceduralFlowEditorProps): ReactNode {
	const hostRef = useRef(extensionHost);

	useEffect(() => {
		hostRef.current = extensionHost;
		void extensionHost.activateDefaults();
	}, [extensionHost]);

	return (
		<FlowCanvas
			fixtureJson={fixtureJson}
			fixtureDragDrop
			reorganize={reorganize}
			extensionRevision={extensionRevision}
			extensionHost={extensionHost}
			onPreviewText={onPreviewText}
			onEvalOutputs={onEvalOutputs}
			onCatalogueReady={onCatalogueReady}
			onFixtureChange={onFixtureChange}
			onSelectionChange={onSelectionChange}
			onPreselectChange={onPreselectChange}
			onHoverChange={onHoverChange}
			selectedNodeIds={selectedNodeIds}
			preselectNodeIds={preselectNodeIds}
			preselectRemovedNodeIds={preselectRemovedNodeIds}
			hoveredNodeId={hoveredNodeId}
			previewOffNodeIds={previewOffNodeIds}
			selectionMode={selectionMode}
			selectionMethod={selectionMethod}
			contextMenu={contextMenu}
			commandRequest={commandRequest}
			onPreviewOffChange={onPreviewOffChange}
			automaticLod={automaticLod}
			lod={lod}
			onLodChange={onLodChange}
			className={className ?? "h-full w-full"}
		/>
	);
}

/** @deprecated Use {@link ProceduralFlowEditor} and {@link ProceduralPreview} in separate playground windows. */
export interface ProceduralEditorProps extends ProceduralFlowEditorProps {}

/** @deprecated Use {@link ProceduralFlowEditor}. */
export function ProceduralEditor(props: ProceduralEditorProps): ReactNode {
	return <ProceduralFlowEditor {...props} />;
}

export { createFlowEvalBridge, type FlowModuleCommandV1, type FlowReorganizeRequest };
// #endregion 🔖ProceduralEditor

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it, beforeAll } = import.meta.vitest;
	const { createRoot } = await import("react-dom/client");
	const { act } = await import("react");
	const {
		ProceduralExtensionHost,
		ProceduralPreview,
		PROCEDURAL_PREVIEW_DEFAULT_CAMERA,
		evaluateBrepFlowKind,
		extractPreviewItems,
		proceduralPreviewCameraSeed,
	} = await import("./index.tsx");
	const { applyOrbitProjectionToCameraState } = await import("@infinite/world/r3f");
	const { BrepjsGeometryKernel, ensureBrepWasmLoaded } = await import("@geometry/brep/js");

	describe("@procedural/react", () => {
		const kernel = new BrepjsGeometryKernel();

		beforeAll(async () => {
			await ensureBrepWasmLoaded();
			if (typeof globalThis.ResizeObserver === "undefined") {
				globalThis.ResizeObserver = class {
					observe() {}
					unobserve() {}
					disconnect() {}
				} as typeof ResizeObserver;
			}
			(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
		});

		it("brep.prim3d.box evaluates to geometry handle", () => {
			const out = evaluateBrepFlowKind("brep.prim3d.box", JSON.stringify({ width: 1, depth: 1, height: 1 }), kernel);
			const parsed = JSON.parse(out) as { geometry?: string };
			expect(parsed.geometry).toMatch(/^solid-/);
		});

		it("brep.curve.line evaluates to edge handle", () => {
			const out = evaluateBrepFlowKind("brep.curve.line", JSON.stringify({ start: [0, 0, 0], end: [2, 0, 0] }), kernel);
			const parsed = JSON.parse(out) as { geometry?: string };
			expect(parsed.geometry).toMatch(/^edge-/);
		});

		it("brep.eval.curveLength returns number", () => {
			const line = evaluateBrepFlowKind("brep.curve.line", JSON.stringify({ start: [0, 0, 0], end: [4, 0, 0] }), kernel);
			const lineParsed = JSON.parse(line) as { geometry?: string };
			const out = evaluateBrepFlowKind("brep.eval.curveLength", JSON.stringify({ geometry: lineParsed.geometry }), kernel);
			const parsed = JSON.parse(out) as { number?: number };
			expect(parsed.number).toBeGreaterThan(3);
		});

		it("extractPreviewItems collects geometry, point, and vector outputs per widget id", () => {
			const items = extractPreviewItems(
				JSON.stringify({
					box: { geometry: "solid-1" },
					line: { geometry: "edge-2" },
					pt: { point: { x: 1, y: 2, z: 3 } },
					vec: { vector: { x: 0, y: 0, z: 1 } },
				}),
			);
			expect(items).toEqual([
				{ widgetId: "box", kind: "geometry", handle: "solid-1" },
				{ widgetId: "line", kind: "geometry", handle: "edge-2" },
				{ widgetId: "pt", kind: "point", position: [1, 2, 3] },
				{ widgetId: "vec", kind: "vector", direction: [0, 0, 1] },
			]);
		});

		it("brep.point and brep.vector evaluate to previewable outputs", () => {
			const pointOut = evaluateBrepFlowKind("brep.point", JSON.stringify({ x: 1, y: 2, z: 3 }), kernel);
			const vectorOut = evaluateBrepFlowKind("brep.vector", JSON.stringify({ x: 0, y: 0, z: 5 }), kernel);
			const items = extractPreviewItems(JSON.stringify({ pointNode: JSON.parse(pointOut), vectorNode: JSON.parse(vectorOut) }));
			expect(items).toContainEqual({ widgetId: "pointNode", kind: "point", position: [1, 2, 3] });
			expect(items).toContainEqual({ widgetId: "vectorNode", kind: "vector", direction: [0, 0, 5] });
		});

		it("extractPreviewItems ignores top-level evaluate errors", () => {
			expect(extractPreviewItems(JSON.stringify({ error: "cycle detected" }))).toEqual([]);
		});

		it("procedural preview mounts with point and vector items", async () => {
			const host = document.createElement("div");
			document.body.appendChild(host);
			const root = createRoot(host);
			await act(async () => {
				root.render(
					<ProceduralPreview
						items={[
							{ widgetId: "pt", kind: "point", position: [1, 0, 0] },
							{ widgetId: "vec", kind: "vector", direction: [0, 0, 2] },
						]}
					/>,
				);
			});
			expect(host.querySelector("[data-world-projection-switch]")).not.toBeNull();
			root.unmount();
			host.remove();
		});

		it("procedural host includes multiple brep catalogue sections", async () => {
			const host = new ProceduralExtensionHost(kernel);
			await host.activateDefaults();
			const sections = host.catalogueSections();
			expect(sections.some((s) => s.id === "brep-prim3d")).toBe(true);
			expect(sections.some((s) => s.id === "brep-curves")).toBe(true);
			expect(sections.some((s) => s.id === "brep-solid")).toBe(true);
			const brepItem = sections.find((s) => s.id === "brep-prim3d")?.items[0];
			expect(brepItem?.abbreviation).toBeTruthy();
			expect(brepItem?.icon).toBeTruthy();
			expect(JSON.stringify(sections)).toContain('"abbreviation"');
		});

		it("procedural preview default camera is z-up perspective", () => {
			expect(PROCEDURAL_PREVIEW_DEFAULT_CAMERA).toMatchObject({
				position: [8, 8, 6],
				target: [0, 0, 0],
				zoom: 1,
				up: [0, 0, 1],
				projection: "perspective",
			});
		});

		it("procedural preview camera seed bumps only on intentional view re-seeds", () => {
			expect(proceduralPreviewCameraSeed(0)).toBe("0");
			expect(proceduralPreviewCameraSeed(1)).toBe("1");
			const ortho = applyOrbitProjectionToCameraState(PROCEDURAL_PREVIEW_DEFAULT_CAMERA, "orthographic");
			expect(ortho.projection).toBe("orthographic");
		});

		it("procedural preview mounts the infinite-world viewport stack", async () => {
			const host = document.createElement("div");
			document.body.appendChild(host);
			const root = createRoot(host);
			await act(async () => {
				root.render(<ProceduralPreview items={[]} />);
			});
			expect(host.querySelector("[data-world-projection-switch]")).not.toBeNull();
			root.unmount();
			host.remove();
		});
	});
}
// #endregion 🧪Tests
