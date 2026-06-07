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
	FlowExtensionHost,
	createFlowEvalBridge,
	type CatalogueSection,
	type FlowExtensionEntry,
	type FlowFixtureV1,
	type FlowModuleCommandV1,
	type FlowModuleManifestV1,
	type FlowModuleNeuronKindV1,
	type FlowReorganizeRequest,
} from "@flow/react";
import {
	worldEntityRenderMode,
	WorldCameraInvalidator,
	WorldCanvas,
	WorldOrbitGated,
} from "@infinite/world/r3f";
import { useEffect, useRef, useState, type ReactNode } from "react";

const THREE = sceneHostPort.three;
// #endregion 🔌Adapters

// #region 🔖BrepWasmBridge
if (!import.meta.env.VITEST) {
	await ensureBrepWasmLoaded();
}
// #endregion 🔖BrepWasmBridge

// #region 🔖BrepFlowModule
function brepKind(id: string, name: string, summary: string, inputs: readonly string[], outputs: readonly string[] = ["geometry"]): FlowModuleNeuronKindV1 {
	return { id: `brep.${id}`, module: "brep", name, summary, inputs, outputs };
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

function parseVec3Input(input: Record<string, unknown>, key: string, fallback: Vec3 = [0, 0, 0]): Vec3 {
	const raw = input[key];
	if (Array.isArray(raw)) return parseVec3(raw, fallback);
	if (raw && typeof raw === "object" && "point" in (raw as object)) return parseVec3((raw as { point: unknown }).point, fallback);
	if (raw && typeof raw === "object" && "vector" in (raw as object)) return parseVec3((raw as { vector: unknown }).vector, fallback);
	return fallback;
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
	"brep.point": (input) => ({ point: [parseNumber(input.x), parseNumber(input.y), parseNumber(input.z)] }),
	"brep.vector": (input) => ({ vector: [parseNumber(input.x), parseNumber(input.y), parseNumber(input.z)] }),
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
		return { point: k.curvePointAtSync(curve, parseNumber(input.t, 0.5)) };
	},
	"brep.eval.tangentOnCurve": (input, k) => {
		const curve = parseGeometry(input, "geometry");
		if (!curve) throw new Error("missing geometry");
		return { vector: k.curveTangentAtSync(curve, parseNumber(input.t, 0.5)) };
	},
	"brep.eval.curveLength": (input, k) => {
		const curve = parseGeometry(input, "geometry");
		if (!curve) throw new Error("missing geometry");
		return { number: k.curveLengthSync(curve) };
	},
	"brep.eval.pointOnSurface": (input, k) => {
		const face = parseGeometry(input, "geometry");
		if (!face) throw new Error("missing geometry");
		return { point: k.pointOnSurfaceSync(face, parseNumber(input.u, 0.5), parseNumber(input.v, 0.5)) };
	},
	"brep.eval.normalAt": (input, k) => {
		const face = parseGeometry(input, "geometry");
		if (!face) throw new Error("missing geometry");
		return { vector: k.normalAtSync(face, parseNumber(input.u, 0.5), parseNumber(input.v, 0.5)) };
	},
	"brep.eval.faceCenter": (input, k) => {
		const face = parseGeometry(input, "geometry");
		if (!face) throw new Error("missing geometry");
		return { point: k.faceCenterSync(face) };
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
export interface ProceduralGeometryHandle {
	readonly widgetId: string;
	readonly handle: string;
}

export type ProceduralPreviewShowMode = "everything" | "selected";

export interface ProceduralPreviewProps {
	readonly handles: readonly ProceduralGeometryHandle[];
	readonly selectedNodeIds?: readonly string[];
	readonly hoveredNodeId?: string | null;
	readonly previewOffNodeIds?: readonly string[];
	readonly showMode?: ProceduralPreviewShowMode;
	readonly onHover?: (widgetId: string | null) => void;
	readonly onSelect?: (widgetId: string) => void;
	readonly kernel?: BrepKernelType;
	readonly tolerance?: number;
	readonly className?: string;
}

/** @deprecated Use {@link ProceduralPreview} with {@link ProceduralGeometryHandle} entries. */
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

function BrepGeometryLayer({
	widgetId,
	geometryId,
	kernel,
	tolerance,
	color,
	selected,
	hovered,
	previewOff,
	onHover,
	onSelect,
}: {
	readonly widgetId: string;
	readonly geometryId: string;
	readonly kernel: BrepKernelType;
	readonly tolerance: number;
	readonly color: string;
	readonly selected: boolean;
	readonly hovered: boolean;
	readonly previewOff: boolean;
	readonly onHover?: (widgetId: string | null) => void;
	readonly onSelect?: (widgetId: string) => void;
}): ReactNode {
	const [buffers, setBuffers] = useState<BrepMeshBuffers>({ surface: null, lines: null, points: null });
	const ref = geometryId as GeometryRef;
	const renderMode = worldEntityRenderMode({ hidden: previewOff }, { hovered, selected, revealed: hovered });

	useEffect(() => {
		let cancelled = false;
		void (async () => {
			await ensureBrepWasmLoaded();
			const mesh = await kernel.tessellateGeometry(ref, tolerance);
			if (cancelled || !isRenderableMeshTransfer(mesh)) {
				setBuffers({ surface: null, lines: null, points: null });
				return;
			}
			setBuffers(buildMeshBuffers(meshTransferToGeometryData(mesh)));
		})();
		return () => {
			cancelled = true;
		};
	}, [kernel, ref, tolerance]);

	if (!renderMode.visible) return null;

	const opacity = renderMode.dim ? 0.35 : renderMode.asHover ? 0.95 : 0.85;
	const emissive = renderMode.showSelectedOutline ? "#3b82f6" : renderMode.asHover ? "#60a5fa" : "#000000";
	const emissiveIntensity = renderMode.showSelectedOutline ? 0.35 : renderMode.asHover ? 0.2 : 0;
	const pointerHandlers =
		onHover || onSelect
			? {
					onPointerOver: (event: { stopPropagation: () => void }) => {
						event.stopPropagation();
						onHover?.(widgetId);
					},
					onPointerOut: (event: { stopPropagation: () => void }) => {
						event.stopPropagation();
						onHover?.(null);
					},
					onClick: (event: { stopPropagation: () => void }) => {
						event.stopPropagation();
						onSelect?.(widgetId);
					},
				}
			: {};

	return (
		<group {...pointerHandlers}>
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

export function ProceduralPreview({
	handles,
	selectedNodeIds = [],
	hoveredNodeId = null,
	previewOffNodeIds = [],
	showMode = "everything",
	onHover,
	onSelect,
	kernel = brepjsGeometryKernel,
	tolerance = 0.02,
	className,
}: ProceduralPreviewProps): ReactNode {
	const visibleHandles =
		showMode === "selected" ? handles.filter((entry) => selectedNodeIds.includes(entry.widgetId)) : handles;

	return (
		<div className={className ?? "relative h-full w-full bg-zinc-900"}>
			<WorldCanvas frameloop="demand" cameraPosition={[8, 8, 6]} background="#18181b">
				<WorldCameraInvalidator />
				<ambientLight intensity={0.45} />
				<directionalLight position={[12, 18, 10]} intensity={1.1} />
				<WorldOrbitGated />
				{visibleHandles.map((entry, index) => (
					<BrepGeometryLayer
						key={`${entry.widgetId}:${entry.handle}`}
						widgetId={entry.widgetId}
						geometryId={entry.handle}
						kernel={kernel}
						tolerance={tolerance}
						color={BREP_VIEWPORT_COLORS[index % BREP_VIEWPORT_COLORS.length]!}
						selected={selectedNodeIds.includes(entry.widgetId)}
						hovered={hoveredNodeId === entry.widgetId}
						previewOff={previewOffNodeIds.includes(entry.widgetId)}
						onHover={onHover}
						onSelect={onSelect}
					/>
				))}
			</WorldCanvas>
		</div>
	);
}

export function BrepViewport({ geometryIds, kernel = brepjsGeometryKernel, tolerance = 0.02, className }: BrepViewportProps): ReactNode {
	return (
		<ProceduralPreview
			handles={geometryIds.map((handle, index) => ({ widgetId: `geometry-${index}`, handle }))}
			kernel={kernel}
			tolerance={tolerance}
			className={className}
		/>
	);
}

export function extractGeometryHandles(outputsJson: string): ProceduralGeometryHandle[] {
	const handles: ProceduralGeometryHandle[] = [];
	try {
		const outputs = JSON.parse(outputsJson) as Record<string, Record<string, unknown>>;
		for (const [widgetId, dict] of Object.entries(outputs)) {
			const handle =
				typeof dict.geometry === "string" && dict.geometry.length > 0
					? dict.geometry
					: typeof dict.brep === "string" && dict.brep.length > 0
						? dict.brep
						: null;
			if (handle) handles.push({ widgetId, handle });
		}
	} catch {
		/* ignore */
	}
	return handles;
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
	readonly onHoverChange?: (id: string | null) => void;
	readonly selectedNodeIds?: readonly string[];
	readonly hoveredNodeId?: string | null;
	readonly previewOffNodeIds?: readonly string[];
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
	onHoverChange,
	selectedNodeIds,
	hoveredNodeId,
	previewOffNodeIds,
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
			onHoverChange={onHoverChange}
			selectedNodeIds={selectedNodeIds}
			hoveredNodeId={hoveredNodeId}
			previewOffNodeIds={previewOffNodeIds}
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
	const { ProceduralExtensionHost, evaluateBrepFlowKind } = await import("./index.tsx");
	const { BrepjsGeometryKernel, ensureBrepWasmLoaded } = await import("@geometry/brep/js");

	describe("@procedural/react", () => {
		const kernel = new BrepjsGeometryKernel();

		beforeAll(async () => {
			await ensureBrepWasmLoaded();
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

		it("extractGeometryHandles collects geometry outputs per widget id", () => {
			const handles = extractGeometryHandles(JSON.stringify({ box: { geometry: "solid-1" }, line: { geometry: "edge-2" } }));
			expect(handles).toEqual([
				{ widgetId: "box", handle: "solid-1" },
				{ widgetId: "line", handle: "edge-2" },
			]);
		});

		it("procedural host includes multiple brep catalogue sections", async () => {
			const host = new ProceduralExtensionHost(kernel);
			await host.activateDefaults();
			const sections = host.catalogueSections();
			expect(sections.some((s) => s.id === "brep-prim3d")).toBe(true);
			expect(sections.some((s) => s.id === "brep-curves")).toBe(true);
			expect(sections.some((s) => s.id === "brep-solid")).toBe(true);
		});
	});
}
// #endregion 🧪Tests
