// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🔧 `@procedural/react` — flow-based brep editor with R3F viewport. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
	applyGumballPose,
	borderNormalClass,
	canvasHostRootClass,
	canvasViewportClass,
	cn,
	gumballHandleKindToTransformMode,
	gumballPointerConsumesCanvasEventRef,
	reactHostPort,
	sceneHostPort,
	UnifiedGumball,
	type GumballHandleKind,
	type GumballPose,
} from "@ui/react";
import { clearColorResolveCache, resolveSemanticColorHex } from "@ui/styling";
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
	createEphemeralFlowStore,
	type DagDrawLodKind,
	FlowExtensionHost,
	createFlowEvalBridge,
	type CatalogueSection,
	type FlowCanvasCommandRequest,
	type FlowCanvasContextMenuContext,
	type FlowExtensionEntry,
	type FlowFixtureV1,
	type FlowModuleCommandV1,
	type FlowInputSpecV1,
	type FlowModuleManifestV1,
	type FlowModuleNeuronKindV1,
	type FlowReorganizeRequest,
	nestNeuronKindsIntoCatalogueSection,
} from "@flow/react";
import type { ContextMenuItem } from "@ui/react";
import { meshStyleColors, resolveMeshStyle, type MeshStyleKind } from "@puzzle/3d/react";
import {
	applyOrbitProjectionToCameraState,
	DEFAULT_LOD_GRID_FACTOR,
	DEFAULT_MANUAL_LOD,
	WORLD_LOCKED_OPACITY_SCALE,
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
	marqueeCoverageFromGesture,
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

function num(id: string, defaultValue: number): FlowInputSpecV1 {
	return { id, type: "number", default: defaultValue };
}

function inp(id: string, type: string): FlowInputSpecV1 {
	return { id, type };
}

function vec(id: string, defaultValue: readonly [number, number, number]): FlowInputSpecV1 {
	return { id, type: "vector", default: [...defaultValue] };
}

function brepKind(
	id: string,
	name: string,
	summary: string,
	inputs: readonly FlowInputSpecV1[],
	outputs: readonly string[] = ["geometry"],
	group: readonly string[],
): FlowModuleNeuronKindV1 {
	const displayName = toPascalCase(name);
	return { id: `brep.${id}`, module: "brep", name: displayName, abbreviation: brepAbbreviation(name), icon: brepIcon(id), summary, inputs, outputs, group };
}

function applyBrepInputDefaults(kindId: string, input: Record<string, unknown>): Record<string, unknown> {
	const kind = BREP_FLOW_KINDS.find((entry) => entry.id === kindId);
	if (!kind) return input;
	const out = { ...input };
	for (const spec of kind.inputs) {
		if (spec.id === "*" || Object.prototype.hasOwnProperty.call(out, spec.id)) continue;
		if (spec.default !== undefined) out[spec.id] = spec.default;
	}
	return out;
}

const BREP_FLOW_KINDS: readonly FlowModuleNeuronKindV1[] = [
	brepKind("point", "Point", "3D point from x,y,z", [num("x", 0), num("y", 0), num("z", 0)], ["point"], ["Construct"]),
	brepKind("vector", "Vector", "3D vector from x,y,z", [num("x", 0), num("y", 0), num("z", 0)], ["vector"], ["Construct"]),
	brepKind("prim3d.box", "Box", "Axis-aligned box", [num("width", 1), num("depth", 1), num("height", 1)], ["geometry"], ["Primitives 3D"]),
	brepKind("prim3d.sphere", "Sphere", "Sphere solid", [num("radius", 1)], ["geometry"], ["Primitives 3D"]),
	brepKind("prim3d.cylinder", "Cylinder", "Cylinder solid", [num("radius", 1), num("height", 1)], ["geometry"], ["Primitives 3D"]),
	brepKind("prim3d.cone", "Cone", "Cone solid", [num("radius", 1), num("height", 1)], ["geometry"], ["Primitives 3D"]),
	brepKind("prim3d.torus", "Torus", "Torus solid", [num("major", 2), num("minor", 0.5)], ["geometry"], ["Primitives 3D"]),
	brepKind("prim3d.ellipsoid", "Ellipsoid", "Ellipsoid solid", [num("rx", 1), num("ry", 1), num("rz", 1)], ["geometry"], ["Primitives 3D"]),
	brepKind("prim3d.polyhedron", "Polyhedron", "Polyhedron from vertices and faces", [inp("vertices", "value"), inp("faces", "list")], ["geometry"], ["Primitives 3D"]),
	brepKind("draw2d.rectangle", "Draw Rectangle", "2D rectangle drawing", [num("width", 2), num("height", 1)], ["geometry"], ["Draw 2D"]),
	brepKind("draw2d.circle", "Draw Circle", "2D circle drawing", [num("radius", 1)], ["geometry"], ["Draw 2D"]),
	brepKind("draw2d.ellipse", "Draw Ellipse", "2D ellipse drawing", [num("major", 1), num("minor", 0.5)], ["geometry"], ["Draw 2D"]),
	brepKind("draw2d.roundedRectangle", "Draw Rounded Rect", "2D rounded rectangle", [num("width", 2), num("height", 1), num("radius", 0.2)], ["geometry"], ["Draw 2D"]),
	brepKind("draw2d.polysides", "Draw Polysides", "2D regular polygon", [num("radius", 1), num("sides", 6)], ["geometry"], ["Draw 2D"]),
	brepKind("sketch2d.circle", "Sketch Circle", "Sketched circle profile", [num("radius", 1)], ["geometry"], ["Draw 2D"]),
	brepKind("sketch2d.rectangle", "Sketch Rectangle", "Sketched rectangle profile", [num("width", 2), num("height", 1)], ["geometry"], ["Draw 2D"]),
	brepKind("curve.line", "Line", "Line edge", [inp("start", "point"), inp("end", "point")], ["geometry"], ["Curves"]),
	brepKind("curve.circle", "Circle", "Circle edge", [num("radius", 1)], ["geometry"], ["Curves"]),
	brepKind("curve.ellipse", "Ellipse", "Ellipse edge", [num("major", 1), num("minor", 0.5)], ["geometry"], ["Curves"]),
	brepKind("curve.helix", "Helix", "Helix edge", [num("radius", 1), num("pitch", 1), num("height", 3)], ["geometry"], ["Curves"]),
	brepKind("curve.threePointArc", "Three Point Arc", "Arc through three points", [inp("a", "geometry"), inp("b", "geometry"), inp("c", "point")], ["geometry"], ["Curves"]),
	brepKind("curve.tangentArc", "Tangent Arc", "Tangent arc", [inp("start", "point"), inp("tangent", "vector"), inp("end", "point")], ["geometry"], ["Curves"]),
	brepKind("curve.ellipseArc", "Ellipse Arc", "Elliptical arc", [num("major", 1), num("minor", 0.5), num("startAngle", 0), num("endAngle", Math.PI)], ["geometry"], ["Curves"]),
	brepKind("curve.bezier", "Bezier", "Bezier curve from poles", [inp("poles", "list")], ["geometry"], ["Curves"]),
	brepKind("curve.bspline", "B-Spline", "B-spline approximation", [inp("poles", "list"), num("degree", 3)], ["geometry"], ["Curves"]),
	brepKind("curve.approximate", "Approximate Curve", "Approximate curve through points", [inp("points", "list"), num("tolerance", 0.01)], ["geometry"], ["Curves"]),
	brepKind("curve.polygon", "Polygon", "Polygon wire from points", [inp("points", "list")], ["geometry"], ["Curves"]),
	brepKind("curve.reparametrize", "Reparametrize Curve", "Resample curve to unit domain", [inp("geometry", "geometry"), num("samples", 64)], ["geometry"], ["Curves"]),
	brepKind("curve.interpolate", "Interpolate", "Interpolated curve", [inp("geometry", "geometry")], ["geometry"], ["Curves"]),
	brepKind("curve.wire", "Wire", "Wire from edges", [inp("geometry", "geometry")], ["geometry"], ["Curves"]),
	brepKind("curve.wireLoop", "Wire Loop", "Closed wire loop", [inp("geometry", "geometry")], ["geometry"], ["Curves"]),
	brepKind("surface.face", "Face", "Face from wires", [inp("geometry", "geometry")], ["geometry"], ["Surfaces"]),
	brepKind("surface.filledFace", "Filled Face", "Filled face from wire", [inp("geometry", "geometry")], ["geometry"], ["Surfaces"]),
	brepKind("surface.fill", "Fill", "Fill from edges", [inp("geometry", "geometry")], ["geometry"], ["Surfaces"]),
	brepKind("surface.offsetFace", "Offset Face", "Offset face", [inp("geometry", "geometry"), num("distance", 0.1)], ["geometry"], ["Surfaces"]),
	brepKind("surface.subFace", "Sub Face", "Face with hole wire", [inp("geometry", "geometry"), inp("wire", "geometry")], ["geometry"], ["Surfaces"]),
	brepKind("surface.fromGrid", "Surface From Grid", "NURBS surface from point grid", [inp("grid", "value")], ["geometry"], ["Surfaces"]),
	brepKind("surface.reparametrize", "Reparametrize Surface", "Resample face to unit UV domain", [inp("geometry", "geometry"), num("uSamples", 12), num("vSamples", 12)], ["geometry"], ["Surfaces"]),
	brepKind("solid.extrude", "Extrude", "Extrude profile", [inp("geometry", "geometry"), vec("vector", [0, 0, 5])], ["geometry"], ["Solid"]),
	brepKind("solid.supportExtrude", "Support Extrude", "Support extrude", [inp("geometry", "geometry"), vec("vector", [0, 0, 5])], ["geometry"], ["Solid"]),
	brepKind("solid.twistExtrude", "Twist Extrude", "Twist extrude profile", [inp("geometry", "geometry"), vec("vector", [0, 0, 5]), num("angle", Math.PI / 4)], ["geometry"], ["Solid"]),
	brepKind("solid.draft", "Draft", "Draft solid faces", [inp("geometry", "geometry"), num("angle", Math.PI / 12), inp("direction", "vector")], ["geometry"], ["Solid"]),
	brepKind("solid.revolve", "Revolve", "Revolve profile", [inp("geometry", "geometry"), num("angle", Math.PI * 2), inp("axis", "vector")], ["geometry"], ["Solid"]),
	brepKind("solid.loft", "Loft", "Loft sections", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Solid"]),
	brepKind("solid.sweep", "Sweep", "Sweep profile along path", [inp("profile", "geometry"), inp("path", "geometry")], ["geometry"], ["Solid"]),
	brepKind("solid.fillet", "Fillet", "Fillet solid", [inp("geometry", "geometry"), num("radius", 0.1)], ["geometry"], ["Solid"]),
	brepKind("solid.chamfer", "Chamfer", "Chamfer solid", [inp("geometry", "geometry"), num("distance", 0.1)], ["geometry"], ["Solid"]),
	brepKind("solid.shell", "Shell", "Shell solid", [inp("geometry", "geometry"), num("thickness", 0.1)], ["geometry"], ["Solid"]),
	brepKind("solid.offset", "Offset", "Offset shape", [inp("geometry", "geometry"), num("distance", 0.1)], ["geometry"], ["Solid"]),
	brepKind("solid.thicken", "Thicken", "Thicken face", [inp("geometry", "geometry"), num("thickness", 0.1)], ["geometry"], ["Solid"]),
	brepKind("solid.hull", "Hull", "Convex hull of shapes", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Solid"]),
	brepKind("solid.minkowski", "Minkowski", "Minkowski sum", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Solid"]),
	brepKind("solid.convexHull", "Convex Hull", "Convex hull", [inp("geometry", "geometry")], ["geometry"], ["Solid"]),
	brepKind("bool.fuse", "Fuse", "Boolean union", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Booleans"]),
	brepKind("bool.cut", "Cut", "Boolean difference", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Booleans"]),
	brepKind("bool.intersect", "Intersect", "Boolean intersection", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Booleans"]),
	brepKind("bool.fuseAll", "Fuse All", "Fuse multiple solids", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Booleans"]),
	brepKind("bool.fuse2d", "Fuse 2D", "2D boolean union", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Booleans"]),
	brepKind("bool.cut2d", "Cut 2D", "2D boolean cut", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Booleans"]),
	brepKind("bool.intersect2d", "Intersect 2D", "2D boolean intersect", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Booleans"]),
	brepKind("bool.cutAll", "Cut All", "Cut solid with multiple cutters", [inp("geometry", "geometry"), inp("cutters", "list")], ["geometry"], ["Booleans"]),
	brepKind("xform.translate", "Translate", "Translate geometry", [inp("geometry", "geometry"), inp("offset", "vector")], ["geometry"], ["Transforms"]),
	brepKind("xform.rotate", "Rotate", "Rotate geometry", [inp("geometry", "geometry"), num("angle", Math.PI / 4), inp("axis", "vector")], ["geometry"], ["Transforms"]),
	brepKind("xform.mirror", "Mirror", "Mirror geometry", [inp("geometry", "geometry"), inp("origin", "point"), inp("normal", "vector")], ["geometry"], ["Transforms"]),
	brepKind("xform.scale", "Scale", "Scale geometry", [inp("geometry", "geometry"), num("factor", 2), inp("center", "point")], ["geometry"], ["Transforms"]),
	brepKind("xform.clone", "Clone", "Clone geometry", [inp("geometry", "geometry")], ["geometry"], ["Transforms"]),
	brepKind("xform.linearPattern", "Linear Pattern", "Linear array", [inp("geometry", "geometry"), inp("direction", "vector"), num("count", 1), num("spacing", 1)], ["geometry"], ["Transforms"]),
	brepKind("xform.circularPattern", "Circular Pattern", "Circular array", [inp("geometry", "geometry"), inp("axis", "vector"), num("count", 1), num("angle", Math.PI * 2)], ["geometry"], ["Transforms"]),
	brepKind("xform.rectangularPattern", "Rect Pattern", "Rectangular array", [inp("geometry", "geometry"), num("countA", 2), num("countB", 2), num("spacing", 1)], ["geometry"], ["Transforms"]),
	brepKind("intersect.section", "Section", "Section two solids", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Intersections"]),
	brepKind("intersect.sectionToFace", "Section To Face", "Section as face", [inp("a", "geometry"), inp("b", "geometry")], ["geometry"], ["Intersections"]),
	brepKind("intersect.slice", "Slice", "Slice solid by plane", [inp("geometry", "geometry"), inp("origin", "point"), inp("normal", "vector")], ["geometry"], ["Intersections"]),
	brepKind("intersect.check", "Check Interference", "Check interference", [inp("a", "geometry"), inp("b", "geometry")], ["number"], ["Intersections"]),
	brepKind("intersect.split", "Split", "Split solid by tool", [inp("geometry", "geometry"), inp("tool", "geometry")], ["list"], ["Intersections"]),
	brepKind("eval.pointOnCurve", "Point On Curve", "Evaluate point on curve", [inp("geometry", "geometry"), num("t", 0)], ["point"], ["Evaluate"]),
	brepKind("eval.tangentOnCurve", "Tangent On Curve", "Tangent on curve", [inp("geometry", "geometry"), num("t", 0)], ["vector"], ["Evaluate"]),
	brepKind("eval.curveLength", "Curve Length", "Curve length", [inp("geometry", "geometry")], ["number"], ["Evaluate"]),
	brepKind("eval.pointOnSurface", "Point On Surface", "Point on surface", [inp("geometry", "geometry"), num("u", 0), num("v", 0)], ["point"], ["Evaluate"]),
	brepKind("eval.normalAt", "Normal At", "Surface normal", [inp("geometry", "geometry"), num("u", 0), num("v", 0)], ["vector"], ["Evaluate"]),
	brepKind("eval.faceCenter", "Face Center", "Face center point", [inp("geometry", "geometry")], ["point"], ["Evaluate"]),
	brepKind("eval.curveStart", "Curve Start", "Curve start point", [inp("geometry", "geometry")], ["point"], ["Evaluate"]),
	brepKind("eval.curveEnd", "Curve End", "Curve end point", [inp("geometry", "geometry")], ["point"], ["Evaluate"]),
	brepKind("eval.curveClosed", "Curve Closed", "Whether curve is closed", [inp("geometry", "geometry")], ["number"], ["Evaluate"]),
	brepKind("eval.uvBounds", "UV Bounds", "Face UV parameter bounds", [inp("geometry", "geometry")], ["dictionary"], ["Evaluate"]),
	brepKind("eval.vertexPosition", "Vertex Position", "Vertex position", [inp("geometry", "geometry")], ["point"], ["Evaluate"]),
	brepKind("eval.divideCurve", "Divide Curve", "Divide curve into points", [inp("geometry", "geometry"), num("count", 1)], ["list"], ["Evaluate"]),
	brepKind("measure.volume", "Volume", "Solid volume", [inp("geometry", "geometry")], ["number"], ["Measure"]),
	brepKind("measure.area", "Area", "Face/solid area", [inp("geometry", "geometry")], ["number"], ["Measure"]),
	brepKind("measure.length", "Length", "Edge/wire length", [inp("geometry", "geometry")], ["number"], ["Measure"]),
	brepKind("measure.distance", "Distance", "Distance between shapes", [inp("a", "geometry"), inp("b", "geometry")], ["number"], ["Measure"]),
	brepKind("query.bounds", "Bounds", "Axis-aligned bounds", [inp("geometry", "geometry")], ["dictionary"], ["Query"]),
	brepKind("query.edges", "Edges", "List edges", [inp("geometry", "geometry")], ["list"], ["Query"]),
	brepKind("query.faces", "Faces", "List faces", [inp("geometry", "geometry")], ["list"], ["Query"]),
	brepKind("query.wires", "Wires", "List wires", [inp("geometry", "geometry")], ["list"], ["Query"]),
	brepKind("query.vertices", "Vertices", "List vertices", [inp("geometry", "geometry")], ["list"], ["Query"]),
	brepKind("repair.heal", "Heal Solid", "Heal solid", [inp("geometry", "geometry")], ["geometry"], ["Repair"]),
	brepKind("repair.healFace", "Heal Face", "Heal face", [inp("geometry", "geometry")], ["geometry"], ["Repair"]),
	brepKind("repair.sewShells", "Sew Shells", "Sew faces into shell", [inp("faces", "list")], ["geometry"], ["Repair"]),
	brepKind("repair.autoHeal", "Auto Heal", "Auto heal shape", [inp("geometry", "geometry")], ["geometry"], ["Repair"]),
	brepKind("repair.solidFromShell", "Solid From Shell", "Make solid from shell", [inp("geometry", "geometry")], ["geometry"], ["Repair"]),
	brepKind("io.exportStep", "Export STEP", "Export STEP bytes as base64", [inp("geometry", "geometry")], ["text"], ["IO"]),
	brepKind("io.exportStl", "Export STL", "Export STL bytes as base64", [inp("geometry", "geometry")], ["text"], ["IO"]),
	brepKind("io.importStep", "Import STEP", "Import STEP from base64", [inp("text", "text")], ["geometry"], ["IO"]),
	brepKind("io.importStl", "Import STL", "Import STL from base64", [inp("text", "text")], ["geometry"], ["IO"]),
	brepKind("gear.external", "External Gear", "Spur external gear", [num("teeth", 20), num("module", 2)], ["geometry"], ["Gears"]),
	brepKind("gear.internal", "Internal Gear", "Spur internal gear", [num("teeth", 20), num("module", 2)], ["geometry"], ["Gears"]),
];

const BREP_MODULE_MANIFEST: FlowModuleManifestV1 = {
	schema: "flow.module/v1",
	id: "brep",
	name: "Brep",
	version: "0.3.0",
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

const EXTRUDE_DEFAULT_VECTOR: Vec3 = [0, 0, 5];

function extrudeFromVector(vec: Vec3): { direction: Vec3; distance: number } {
	const [x, y, z] = vec;
	const len = Math.hypot(x, y, z);
	if (len < 1e-12) return extrudeFromVector(EXTRUDE_DEFAULT_VECTOR);
	return { direction: [x / len, y / len, z / len], distance: len };
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

function base64ToBytes(text: string): Uint8Array {
	const binary = atob(text);
	const bytes = new Uint8Array(binary.length);
	for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
	return bytes;
}

function listDictIndices(dict: Record<string, unknown>): number[] {
	return Object.keys(dict)
		.filter((key) => /^\d+$/.test(key))
		.map((key) => Number(key))
		.sort((a, b) => a - b);
}

function parseVec3List(input: Record<string, unknown>, key: string): Vec3[] {
	const raw = input[key];
	if (!raw || typeof raw !== "object" || Array.isArray(raw)) return [];
	const dict = raw as Record<string, unknown>;
	const points: Vec3[] = [];
	for (const index of listDictIndices(dict)) {
		const vec = parseVec3Loose(dict[String(index)]);
		if (vec) points.push(vec);
	}
	return points;
}

function parseGeometryList(input: Record<string, unknown>, key: string): GeometryRef[] {
	const raw = input[key];
	if (!raw || typeof raw !== "object" || Array.isArray(raw)) {
		const single = parseGeometry(input, key);
		return single ? [single] : [];
	}
	const dict = raw as Record<string, unknown>;
	const refs: GeometryRef[] = [];
	for (const index of listDictIndices(dict)) {
		const item = dict[String(index)];
		if (typeof item === "string" && item.length > 0) refs.push(item as GeometryRef);
	}
	return refs;
}

function parseNestedVec3Grid(input: Record<string, unknown>, key: string): Vec3[][] {
	const raw = input[key];
	if (!raw || typeof raw !== "object" || Array.isArray(raw)) return [];
	const dict = raw as Record<string, unknown>;
	const rows: Vec3[][] = [];
	for (const rowIndex of listDictIndices(dict)) {
		const rowRaw = dict[String(rowIndex)];
		if (!rowRaw || typeof rowRaw !== "object" || Array.isArray(rowRaw)) continue;
		const rowDict = rowRaw as Record<string, unknown>;
		const row: Vec3[] = [];
		for (const colIndex of listDictIndices(rowDict)) {
			const vec = parseVec3Loose(rowDict[String(colIndex)]);
			if (vec) row.push(vec);
		}
		if (row.length > 0) rows.push(row);
	}
	return rows;
}

function parseFaceIndexLists(input: Record<string, unknown>, key: string): number[][] {
	const raw = input[key];
	if (!raw || typeof raw !== "object" || Array.isArray(raw)) return [];
	const dict = raw as Record<string, unknown>;
	const faces: number[][] = [];
	for (const faceIndex of listDictIndices(dict)) {
		const faceRaw = dict[String(faceIndex)];
		if (!faceRaw || typeof faceRaw !== "object" || Array.isArray(faceRaw)) continue;
		const faceDict = faceRaw as Record<string, unknown>;
		const indices: number[] = [];
		for (const vertexIndex of listDictIndices(faceDict)) {
			indices.push(parseNumber(faceDict[String(vertexIndex)]));
		}
		if (indices.length > 0) faces.push(indices);
	}
	return faces;
}

function geometryListOut(refs: readonly GeometryRef[]): Record<string, unknown> {
	const list: Record<string, string> = {};
	refs.forEach((ref, index) => {
		list[String(index)] = String(ref);
	});
	return { list };
}

function vec3ListOut(points: readonly Vec3[]): Record<string, unknown> {
	const list: Record<string, { x: number; y: number; z: number }> = {};
	points.forEach((point, index) => {
		list[String(index)] = { x: point[0], y: point[1], z: point[2] };
	});
	return { list };
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
	"brep.prim3d.polyhedron": (input, k) => {
		const vertices = parseVec3List(input, "vertices");
		const faces = parseFaceIndexLists(input, "faces");
		if (vertices.length < 4 || faces.length === 0) throw new Error("missing polyhedron vertices or faces");
		return geoOut(k.polyhedronSync(vertices, faces));
	},
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
	"brep.curve.bezier": (input, k) => {
		const poles = parseVec3List(input, "poles");
		if (poles.length < 2) throw new Error("missing bezier poles");
		return geoOut(k.bezierSync(poles));
	},
	"brep.curve.bspline": (input, k) => {
		const poles = parseVec3List(input, "poles");
		if (poles.length < 2) throw new Error("missing bspline poles");
		return geoOut(k.bsplineApproxSync(poles, parseNumber(input.degree, 3)));
	},
	"brep.curve.approximate": (input, k) => {
		const points = parseVec3List(input, "points");
		if (points.length < 2) throw new Error("missing approximate points");
		return geoOut(k.approximateCurveSync(points, parseNumber(input.tolerance, 0.01)));
	},
	"brep.curve.polygon": (input, k) => {
		const points = parseVec3List(input, "points");
		if (points.length < 3) throw new Error("missing polygon points");
		return geoOut(k.polygonSync(points));
	},
	"brep.curve.reparametrize": (input, k) => {
		const curve = parseGeometry(input, "geometry");
		if (!curve) throw new Error("missing geometry");
		return geoOut(k.reparametrizeCurveSync(curve, parseNumber(input.samples, 64)));
	},
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
	"brep.surface.subFace": (input, k) => {
		const face = parseGeometry(input, "geometry");
		const wire = parseGeometry(input, "wire");
		if (!face || !wire) throw new Error("missing subFace inputs");
		return geoOut(k.subFaceSync(face, wire));
	},
	"brep.surface.fromGrid": (input, k) => {
		const grid = parseNestedVec3Grid(input, "grid");
		if (grid.length < 2 || grid[0]!.length < 2) throw new Error("missing surface grid");
		return geoOut(k.surfaceFromGridSync(grid));
	},
	"brep.surface.reparametrize": (input, k) => {
		const face = parseGeometry(input, "geometry");
		if (!face) throw new Error("missing geometry");
		return geoOut(k.reparametrizeSurfaceSync(face, parseNumber(input.uSamples, 12), parseNumber(input.vSamples, 12)));
	},
	"brep.solid.extrude": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		const { direction, distance } = extrudeFromVector(parseVec3Input(input, "vector", EXTRUDE_DEFAULT_VECTOR));
		return geoOut(k.extrudeSync(shape, direction, distance));
	},
	"brep.solid.revolve": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.revolveSync(shape, parseVec3Input(input, "axis", [0, 0, 1]), parseNumber(input.angle, Math.PI * 2)));
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
	"brep.solid.supportExtrude": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		const { direction, distance } = extrudeFromVector(parseVec3Input(input, "vector", EXTRUDE_DEFAULT_VECTOR));
		return geoOut(k.supportExtrudeSync(shape, direction, distance));
	},
	"brep.solid.twistExtrude": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		const { direction, distance } = extrudeFromVector(parseVec3Input(input, "vector", EXTRUDE_DEFAULT_VECTOR));
		return geoOut(k.twistExtrudeSync(shape, direction, distance, parseNumber(input.angle, Math.PI / 4)));
	},
	"brep.solid.draft": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.draftSync(shape, parseNumber(input.angle, Math.PI / 12), parseVec3Input(input, "direction", [0, 0, 1])));
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
	"brep.bool.cutAll": (input, k) => {
		const base = parseGeometry(input, "geometry");
		const cutters = parseGeometryList(input, "cutters");
		if (!base || cutters.length === 0) throw new Error("missing cutAll inputs");
		return geoOut(k.cutAllSync(base, cutters));
	},
	"brep.xform.translate": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.translateGeomSync(shape, parseVec3Input(input, "offset", [1, 0, 0])));
	},
	"brep.xform.rotate": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.rotateGeomSync(shape, parseVec3Input(input, "axis", [0, 0, 1]), parseNumber(input.angle, Math.PI / 4)));
	},
	"brep.xform.mirror": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.mirrorGeomSync(shape, parseVec3Input(input, "origin", [0, 0, 0]), parseVec3Input(input, "normal", [1, 0, 0])));
	},
	"brep.xform.scale": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.scaleGeomSync(shape, parseNumber(input.factor, 2), parseVec3Input(input, "center", [0, 0, 0])));
	},
	"brep.xform.clone": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(k.cloneGeomSync(shape));
	},
	"brep.xform.linearPattern": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(
			k.linearPatternSync(shape, parseVec3Input(input, "direction", [1, 0, 0]), parseNumber(input.count, 3), parseNumber(input.spacing, 2)),
		);
	},
	"brep.xform.circularPattern": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geoOut(
			k.circularPatternSync(shape, parseVec3Input(input, "axis", [0, 0, 1]), parseNumber(input.count, 4), parseNumber(input.angle, Math.PI * 2)),
		);
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
		return geoOut(k.sliceSync(shape, parseVec3Input(input, "origin", [0, 0, 0]), parseVec3Input(input, "normal", [0, 0, 1])));
	},
	"brep.intersect.check": (input, k) => {
		const a = parseGeometry(input, "a");
		const b = parseGeometry(input, "b");
		if (!a || !b) throw new Error("missing interference inputs");
		return { number: k.checkInterferenceSync(a, b) ? 1 : 0 };
	},
	"brep.intersect.split": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		const tool = parseGeometry(input, "tool");
		if (!shape || !tool) throw new Error("missing split inputs");
		return geometryListOut(k.splitSync(shape, tool));
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
	"brep.eval.curveStart": (input, k) => {
		const curve = parseGeometry(input, "geometry");
		if (!curve) throw new Error("missing geometry");
		return vec3PortOut("point", k.curveStartPointSync(curve));
	},
	"brep.eval.curveEnd": (input, k) => {
		const curve = parseGeometry(input, "geometry");
		if (!curve) throw new Error("missing geometry");
		return vec3PortOut("point", k.curveEndPointSync(curve));
	},
	"brep.eval.curveClosed": (input, k) => {
		const curve = parseGeometry(input, "geometry");
		if (!curve) throw new Error("missing geometry");
		return { number: k.curveIsClosedSync(curve) ? 1 : 0 };
	},
	"brep.eval.uvBounds": (input, k) => {
		const face = parseGeometry(input, "geometry");
		if (!face) throw new Error("missing geometry");
		const bounds = k.uvBoundsSync(face);
		return { dictionary: bounds };
	},
	"brep.eval.vertexPosition": (input, k) => {
		const vertex = parseGeometry(input, "geometry");
		if (!vertex) throw new Error("missing geometry");
		return vec3PortOut("point", k.vertexPositionSync(vertex));
	},
	"brep.eval.divideCurve": (input, k) => {
		const curve = parseGeometry(input, "geometry");
		if (!curve) throw new Error("missing geometry");
		return vec3ListOut(k.divideCurveSync(curve, parseNumber(input.count, 5)));
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
		return geometryListOut(k.getFacesSync(shape));
	},
	"brep.query.wires": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geometryListOut(k.getWiresSync(shape));
	},
	"brep.query.vertices": (input, k) => {
		const shape = parseGeometry(input, "geometry");
		if (!shape) throw new Error("missing geometry");
		return geometryListOut(k.getVerticesSync(shape));
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
	"brep.repair.healFace": (input, k) => {
		const face = parseGeometry(input, "geometry");
		if (!face) throw new Error("missing geometry");
		return geoOut(k.healFaceSync(face));
	},
	"brep.repair.sewShells": (input, k) => {
		const faces = parseGeometryList(input, "faces");
		if (faces.length === 0) throw new Error("missing faces");
		return geoOut(k.sewShellsSync(faces));
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
	"brep.io.importStep": (input, k) => {
		const text = input.text;
		if (typeof text !== "string" || text.length === 0) throw new Error("missing text");
		return geoOut(k.importStepSync(base64ToBytes(text)));
	},
	"brep.io.importStl": (input, k) => {
		const text = input.text;
		if (typeof text !== "string" || text.length === 0) throw new Error("missing text");
		return geoOut(k.importStlSync(base64ToBytes(text)));
	},
	"brep.gear.external": (input, k) => geoOut(k.makeExternalGearSync(parseNumber(input.teeth, 20), parseNumber(input.module, 2))),
	"brep.gear.internal": (input, k) => geoOut(k.makeInternalGearSync(parseNumber(input.teeth, 20), parseNumber(input.module, 2))),
};

/** @emoji 🧊 Synchronous brep neuron evaluation (requires pre-initialized WASM). */
export function evaluateBrepFlowKind(kindId: string, inputJson: string, kernel: BrepKernelType): string {
	const input = applyBrepInputDefaults(kindId, JSON.parse(inputJson) as Record<string, unknown>);
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
		return [...super.catalogueSections(), nestNeuronKindsIntoCatalogueSection("brep", "Brep", BREP_FLOW_KINDS)];
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
		{ kind: "neuron", id: "sketch", neuronKind: "brep.sketch2d.circle" },
		{ kind: "inputSlider", id: "vz", value: 5, min: 0, max: 50, step: 0.5 },
		{ kind: "neuron", id: "vector", neuronKind: "brep.vector" },
		{ kind: "neuron", id: "solid", neuronKind: "brep.solid.extrude" },
		{ kind: "outputPreview", id: "preview" },
	],
	synapses: [
		{ id: "s1", from: "sketch", to: "solid", fromPort: "out", toPort: "geometry" },
		{ id: "s2", from: "vz", to: "vector", fromPort: "out", toPort: "z" },
		{ id: "s3", from: "vector", to: "solid", fromPort: "out", toPort: "vector" },
		{ id: "s4", from: "solid", to: "preview", fromPort: "out", toPort: "in" },
	],
};

export function proceduralFixtureToJson(fixture: FlowFixtureV1 = PROCEDURAL_DEFAULT_FIXTURE): string {
	return JSON.stringify(fixture);
}

const PROCEDURAL_FLOW_STORE = createEphemeralFlowStore();
// #endregion 🔖Fixture

// #region 🔖ProceduralPreview
export type ProceduralChannelDirection = "in" | "out";

export interface ProceduralChannelRef {
	readonly widgetId: string;
	readonly port: string;
	readonly direction: ProceduralChannelDirection;
}

export type ProceduralPreviewItem =
	| { readonly widgetId: string; readonly port: string; readonly direction: ProceduralChannelDirection; readonly kind: "geometry"; readonly handle: GeometryRef }
	| { readonly widgetId: string; readonly port: string; readonly direction: ProceduralChannelDirection; readonly kind: "point"; readonly position: Vec3 }
	| { readonly widgetId: string; readonly port: string; readonly direction: ProceduralChannelDirection; readonly kind: "vector"; readonly directionVec: Vec3 };

export interface ProceduralPreviewExtractContext {
	readonly widgetId: string;
	readonly port: string;
	readonly direction: ProceduralChannelDirection;
	readonly value: unknown;
}

export type ProceduralPreviewExtractor = (context: ProceduralPreviewExtractContext) => readonly ProceduralPreviewItem[];

const proceduralPreviewExtractors: ProceduralPreviewExtractor[] = [];

/** @emoji 📋 Registers a channel-value preview extractor for procedural flow eval. */
export function registerPreviewExtractor(extractor: ProceduralPreviewExtractor): void {
	proceduralPreviewExtractors.push(extractor);
}

function runPreviewExtractors(context: ProceduralPreviewExtractContext): ProceduralPreviewItem[] {
	const items: ProceduralPreviewItem[] = [];
	for (const extractor of proceduralPreviewExtractors) {
		items.push(...extractor(context));
	}
	return items;
}

function previewItemsFromChannelValue(widgetId: string, port: string, direction: ProceduralChannelDirection, value: unknown): ProceduralPreviewItem[] {
	if (value && typeof value === "object" && !Array.isArray(value) && typeof (value as Record<string, unknown>).error === "string") {
		return [];
	}
	return runPreviewExtractors({ widgetId, port, direction, value });
}

function proceduralChannelMatches(item: ProceduralPreviewItem, channel: ProceduralChannelRef): boolean {
	return item.widgetId === channel.widgetId && item.port === channel.port && item.direction === channel.direction;
}

export function filterVisiblePreviewItems(
	items: readonly ProceduralPreviewItem[],
	options: {
		readonly showMode: ProceduralPreviewShowMode;
		readonly selectedNodeIds: readonly string[];
		readonly selectedChannels: readonly ProceduralChannelRef[];
		readonly hoveredNodeId: string | null;
		readonly hoveredChannel: ProceduralChannelRef | null;
	},
): ProceduralPreviewItem[] {
	const { showMode, selectedNodeIds, selectedChannels } = options;
	if (showMode === "selected") {
		if (selectedChannels.length > 0) {
			return items.filter((entry) => selectedChannels.some((channel) => proceduralChannelMatches(entry, channel)));
		}
		if (selectedNodeIds.length > 0) {
			return items.filter((entry) => selectedNodeIds.includes(entry.widgetId));
		}
		return [];
	}
	return items.filter((entry) => entry.direction === "out");
}

registerPreviewExtractor((context) => {
	const refs: GeometryRef[] = [];
	collectGeometryRefsFromValue(context.value, refs);
	return [...new Set(refs)].map((handle) => ({
		widgetId: context.widgetId,
		port: context.port,
		direction: context.direction,
		kind: "geometry" as const,
		handle,
	}));
});

registerPreviewExtractor((context) => {
	if (!context.value || typeof context.value !== "object" || Array.isArray(context.value)) return [];
	const dict = context.value as Record<string, unknown>;
	const point = parsePreviewVec3(dict.point);
	if (!point) return [];
	return [{ widgetId: context.widgetId, port: context.port, direction: context.direction, kind: "point", position: point }];
});

registerPreviewExtractor((context) => {
	if (!context.value || typeof context.value !== "object" || Array.isArray(context.value)) return [];
	const dict = context.value as Record<string, unknown>;
	const vector = parsePreviewVec3(dict.vector);
	if (!vector) return [];
	return [{ widgetId: context.widgetId, port: context.port, direction: context.direction, kind: "vector", directionVec: vector }];
});

export type ProceduralPreviewShowMode = "everything" | "selected";
export type ProceduralSelectionMode = SelectionMergeMode;
export type ProceduralSelectionMethod = "rectangle" | "lasso";
export type ProceduralTransformGranularity = "compact" | "full";
export type ProceduralGumballTransformOp = "translate" | "rotate" | "scale";

export type ProceduralGumballTransformDelta =
	| { readonly op: "translate"; readonly offset: readonly [number, number, number] }
	| { readonly op: "rotate"; readonly angle: number }
	| { readonly op: "scale"; readonly factor: number };

export type ProceduralGumballTransformPhase = "start" | "live" | "end";

export interface ProceduralGumballTransformRequest {
	readonly widgetId: string;
	readonly delta: ProceduralGumballTransformDelta;
	readonly granularity: ProceduralTransformGranularity;
	readonly phase?: ProceduralGumballTransformPhase;
}

function gumballDeltaFromPoses(
	mode: ProceduralGumballTransformRequest["delta"]["op"],
	before: GumballPose,
	after: GumballPose,
): ProceduralGumballTransformRequest["delta"] {
	if (mode === "translate") {
		return {
			op: "translate",
			offset: [
				after.position[0] - before.position[0],
				after.position[1] - before.position[1],
				after.position[2] - before.position[2],
			],
		};
	}
	if (mode === "rotate") {
		const qb = new THREE.Quaternion(...before.quaternion);
		const qa = new THREE.Quaternion(...after.quaternion);
		const eulerBefore = new THREE.Euler().setFromQuaternion(qb, "XYZ");
		const eulerAfter = new THREE.Euler().setFromQuaternion(qa, "XYZ");
		return { op: "rotate", angle: eulerAfter.z - eulerBefore.z };
	}
	const beforeScale = before.scale[0] || 1;
	return { op: "scale", factor: after.scale[0] / beforeScale };
}

function ProceduralPreviewGumball({
	item,
	kernel,
	transformGranularity,
	onGumballTransform,
	onInteractionChange,
}: {
	readonly item: Extract<ProceduralPreviewItem, { kind: "geometry" }>;
	readonly kernel: BrepKernelType;
	readonly transformGranularity: ProceduralTransformGranularity;
	readonly onGumballTransform?: (request: ProceduralGumballTransformRequest) => void;
	readonly onInteractionChange?: (widgetId: string | null) => void;
}): ReactNode {
	const [target, setTarget] = reactHostPort.useState<THREE.Object3D | null>(null);
	const dragBeforeRef = reactHostPort.useRef<GumballPose | null>(null);
	const center = reactHostPort.useMemo(() => {
		const bounds = worldBoundsForPreviewItem(item, kernel);
		if (!bounds) return [0, 0, 0] as Vec3;
		return [
			(bounds.min[0] + bounds.max[0]) * 0.5,
			(bounds.min[1] + bounds.max[1]) * 0.5,
			(bounds.min[2] + bounds.max[2]) * 0.5,
		] as Vec3;
	}, [item, kernel]);
	const setGumballInteractionActive = reactHostPort.useCallback(
		(active: boolean) => {
			proceduralGumballDragActiveRef.current = active;
			gumballPointerConsumesCanvasEventRef.current = active;
			onInteractionChange?.(active ? item.widgetId : null);
		},
		[item.widgetId, onInteractionChange],
	);
	const emitGumballTransform = reactHostPort.useCallback(
		(phase: ProceduralGumballTransformPhase, kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
			const mode = gumballHandleKindToTransformMode(kind);
			const delta = gumballDeltaFromPoses(mode, before, after);
			console.log(`[DEBUG] procedural gumball ${item.widgetId} ${mode} ${phase}`, delta);
			onGumballTransform?.({ widgetId: item.widgetId, delta, granularity: transformGranularity, phase });
		},
		[item.widgetId, onGumballTransform, transformGranularity],
	);
	if (!onGumballTransform) return null;
	return (
		<group position={center}>
			<group ref={(node) => setTarget(node)} />
			{target ? (
				<UnifiedGumball
					target={target}
					onDragStart={(kind: GumballHandleKind, pose: GumballPose) => {
						dragBeforeRef.current = pose;
						setGumballInteractionActive(true);
						emitGumballTransform("start", kind, pose, pose);
					}}
					onDraggingChanged={(active) => {
						setGumballInteractionActive(active);
					}}
					onDrag={(kind: GumballHandleKind, after: GumballPose) => {
						const before = dragBeforeRef.current;
						if (!before) return;
						emitGumballTransform("live", kind, before, after);
						applyGumballPose(target, before);
					}}
					onDragEnd={(kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
						emitGumballTransform("end", kind, before, after);
						applyGumballPose(target, before);
						dragBeforeRef.current = null;
						setGumballInteractionActive(false);
					}}
				/>
			) : null}
		</group>
	);
}

export interface ProceduralPreviewProps {
	readonly items: readonly ProceduralPreviewItem[];
	readonly selectedNodeIds?: readonly string[];
	readonly selectedChannels?: readonly ProceduralChannelRef[];
	readonly preselectNodeIds?: readonly string[];
	readonly preselectRemovedNodeIds?: readonly string[];
	readonly hoveredNodeId?: string | null;
	readonly hoveredChannel?: ProceduralChannelRef | null;
	readonly previewOffNodeIds?: readonly string[];
	readonly showMode?: ProceduralPreviewShowMode;
	readonly selectionMode?: ProceduralSelectionMode;
	readonly selectionMethod?: ProceduralSelectionMethod;
	readonly transformGranularity?: ProceduralTransformGranularity;
	readonly onGumballTransform?: (request: ProceduralGumballTransformRequest) => void;
	readonly gumballActiveWidgetIds?: readonly string[];
	readonly onHover?: (widgetId: string | null) => void;
	readonly onSelect?: (widgetId: string) => void;
	readonly onSelectionChange?: (ids: readonly string[], mode: ProceduralSelectionMode) => void;
	readonly kernel?: BrepKernelType;
	readonly tolerance?: number;
	readonly className?: string;
}

const PROCEDURAL_PREVIEW_MARQUEE_THRESHOLD_PX = 4;

/** @emoji 🎛 True while a procedural preview gumball drag is active (blocks marquee selection). */
export const proceduralGumballDragActiveRef = { current: false };

type PreviewLayerChrome = {
	readonly selected: boolean;
	readonly highlighted: boolean;
	readonly hovered: boolean;
	readonly previewOff: boolean;
	readonly locked: boolean;
	readonly interactionHighlighted: boolean;
	readonly pickEnabled: boolean;
	readonly onHover?: (widgetId: string | null) => void;
	readonly onPick?: (widgetId: string, mode: ProceduralSelectionMode) => void;
};

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
const PROCEDURAL_PREVIEW_LINE_PICK_MIN = 0.08;
const PROCEDURAL_GEOMETRY_REF_PATTERN = /^(vertex|edge|wire|face|shell|solid|compound|drawing)-/;

export function previewPickProxyFromBounds(bounds: { min: Vec3; max: Vec3 }): { position: Vec3; size: Vec3 } {
	const [minX, minY, minZ] = bounds.min;
	const [maxX, maxY, maxZ] = bounds.max;
	const minPick = PROCEDURAL_PREVIEW_LINE_PICK_MIN;
	return {
		position: [(minX + maxX) * 0.5, (minY + maxY) * 0.5, (minZ + maxZ) * 0.5],
		size: [Math.max(maxX - minX, minPick), Math.max(maxY - minY, minPick), Math.max(maxZ - minZ, minPick)],
	};
}

function previewLineColor(colors: NonNullable<ReturnType<typeof meshStyleColors>>, hovered: boolean, hasSurface: boolean): string {
	if (hovered && !hasSurface) {
		return meshStyleColors("selected")?.lineColor ?? meshStyleColors("highlighted")?.lineColor ?? colors.lineColor;
	}
	return colors.lineColor;
}

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

function previewLayerPaint(chrome: Pick<PreviewLayerChrome, "previewOff" | "hovered" | "selected" | "highlighted" | "locked" | "interactionHighlighted">): {
	readonly renderMode: ReturnType<typeof worldEntityRenderMode>;
	readonly style: MeshStyleKind;
	readonly colors: NonNullable<ReturnType<typeof meshStyleColors>>;
	readonly opacity: number;
} {
	const interactionHighlighted = chrome.interactionHighlighted;
	const locked = chrome.locked;
	const renderMode = worldEntityRenderMode(
		{ hidden: chrome.previewOff, locked },
		{
			hovered: chrome.hovered && !locked,
			selected: chrome.selected || chrome.highlighted || interactionHighlighted,
			revealed: chrome.hovered,
		},
	);
	const style = resolveMeshStyle({
		selected: renderMode.showSelectedOutline || interactionHighlighted,
		highlighted: chrome.highlighted || interactionHighlighted,
		hovered: renderMode.asHover || chrome.hovered || interactionHighlighted,
	});
	const colors = meshStyleColors(style) ?? meshStyleColors("neutral")!;
	const opacity = colors.opacity * (renderMode.dim ? WORLD_LOCKED_OPACITY_SCALE : 1);
	return { renderMode, style, colors, opacity };
}

function createPreviewPointerHandlers(
	widgetId: string,
	onHover?: (widgetId: string | null) => void,
	onPick?: (widgetId: string, mode: ProceduralSelectionMode) => void,
	pickEnabled = true,
) {
	if (!pickEnabled || (!onHover && !onPick)) return {};
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
	const [x, y, z] = item.directionVec;
	return {
		min: [Math.min(0, x) - pad, Math.min(0, y) - pad, Math.min(0, z) - pad],
		max: [Math.max(0, x) + pad, Math.max(0, y) + pad, Math.max(0, z) + pad],
	};
}

function previewItemKey(item: ProceduralPreviewItem): string {
	if (item.kind === "geometry") return `${item.widgetId}:${item.direction}:${item.port}:geometry:${item.handle}`;
	if (item.kind === "point") return `${item.widgetId}:${item.direction}:${item.port}:point`;
	return `${item.widgetId}:${item.direction}:${item.port}:vector`;
}

function BrepPreviewLayer({
	item,
	kernel,
	tolerance,
	...chrome
}: {
	readonly item: ProceduralPreviewItem;
	readonly kernel: BrepKernelType;
	readonly tolerance: number;
} & PreviewLayerChrome): ReactNode {
	const { renderMode, colors, opacity } = previewLayerPaint(chrome);
	const handlers = createPreviewPointerHandlers(item.widgetId, chrome.onHover, chrome.onPick, chrome.pickEnabled);
	const [buffers, setBuffers] = useState<BrepMeshBuffers>({ surface: null, lines: null, points: null });
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	const geometryRef = item.kind === "geometry" ? item.handle : null;
	const lineOnlyGeometry = item.kind === "geometry" && !buffers.surface && Boolean(buffers.lines);
	const pickProxy = useMemo(() => {
		if (!lineOnlyGeometry) return null;
		const bounds = worldBoundsForPreviewItem(item, kernel);
		return bounds ? previewPickProxyFromBounds(bounds) : null;
	}, [item, kernel, lineOnlyGeometry]);
	const lineColor = previewLineColor(colors, renderMode.asHover, Boolean(buffers.surface));
	const arrow = useMemo(() => {
		if (item.kind !== "vector") return null;
		const tip = new THREE.Vector3(item.directionVec[0], item.directionVec[1], item.directionVec[2]);
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
	}, [item]);

	useEffect(() => {
		if (!geometryRef) return;
		let cancelled = false;
		void (async () => {
			await ensureBrepWasmLoaded();
			const mesh = await kernel.tessellateGeometry(geometryRef, tolerance);
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
	}, [geometryRef, invalidate, kernel, tolerance]);

	if (!renderMode.visible) return null;

	if (item.kind === "point") {
		const radius = renderMode.asHover ? PROCEDURAL_PREVIEW_POINT_RADIUS * 1.25 : PROCEDURAL_PREVIEW_POINT_RADIUS;
		return (
			<group position={item.position} {...handlers}>
				<mesh>
					<sphereGeometry args={[radius, 16, 12]} />
					<meshStandardMaterial
						color={colors.meshColor}
						emissive={colors.emissiveColor}
						emissiveIntensity={colors.emissiveIntensity}
						metalness={0}
						roughness={1}
						transparent={opacity < 1}
						opacity={opacity}
					/>
				</mesh>
			</group>
		);
	}

	if (item.kind === "vector") {
		if (!arrow) return null;
		return (
			<group {...handlers}>
				<line geometry={arrow.shaft}>
					<lineBasicMaterial color={colors.lineColor} linewidth={1} transparent={opacity < 1} opacity={opacity} />
				</line>
				<mesh geometry={arrow.head} position={arrow.headPosition} quaternion={arrow.quaternion}>
					<meshStandardMaterial
						color={colors.meshColor}
						emissive={colors.emissiveColor}
						emissiveIntensity={colors.emissiveIntensity}
						metalness={0}
						roughness={1}
						transparent={opacity < 1}
						opacity={opacity}
					/>
				</mesh>
			</group>
		);
	}

	return (
		<group {...(pickProxy ? {} : handlers)}>
			{pickProxy ? (
				<mesh position={pickProxy.position} {...handlers}>
					<boxGeometry args={pickProxy.size} />
					<meshBasicMaterial transparent opacity={0} depthWrite={false} side={THREE.DoubleSide} />
				</mesh>
			) : null}
			{buffers.surface ? (
				<mesh geometry={buffers.surface} raycast={chrome.pickEnabled ? undefined : () => null}>
					<meshStandardMaterial
						color={colors.meshColor}
						emissive={colors.emissiveColor}
						emissiveIntensity={colors.emissiveIntensity}
						metalness={chrome.locked ? 0.15 : 0}
						roughness={chrome.locked ? 0.35 : 1}
						transparent={opacity < 1}
						opacity={opacity}
						side={THREE.DoubleSide}
					/>
				</mesh>
			) : null}
			{buffers.lines ? (
				<lineSegments geometry={buffers.lines} raycast={() => null}>
					<lineBasicMaterial
						color={lineColor}
						linewidth={renderMode.asHover && !buffers.surface ? 2 : 1}
						transparent={opacity < 1}
						opacity={opacity}
					/>
				</lineSegments>
			) : null}
			{buffers.points ? (
				<points geometry={buffers.points}>
					<pointsMaterial color={lineColor} size={renderMode.asHover ? 0.16 : 0.12} transparent={opacity < 1} opacity={opacity} />
				</points>
			) : null}
		</group>
	);
}

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
		const gumballBlocksSelection = () => gumballPointerConsumesCanvasEventRef.current || proceduralGumballDragActiveRef.current;
		const onPointerDown = (event: PointerEvent) => {
			if (event.button !== 0) return;
			if (gumballBlocksSelection()) return;
			if ((event.target as HTMLElement | null)?.closest("[data-world-projection-switch]")) return;
			const point = clientToLocal(event.clientX, event.clientY);
			marqueeRef.current = { tracking: true, active: false, start: point, points: [point], initial: [...selectedNodeIdsRef.current] };
			onMarqueeOverlayRef.current(null);
			onLivePreselectRef.current({ ids: [], removedIds: [] });
		};
		const onPointerMove = (event: PointerEvent) => {
			if (gumballBlocksSelection()) {
				if (marqueeRef.current.tracking) resetGesture();
				return;
			}
			if (!marqueeRef.current.tracking) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const start = marqueeRef.current.start;
			const distance = Math.hypot(point.x - start.x, point.y - start.y);
			if (!marqueeRef.current.active && distance < PROCEDURAL_PREVIEW_MARQUEE_THRESHOLD_PX) return;
			marqueeRef.current.active = true;
			const points = selectionMethodRef.current === "lasso" ? [...marqueeRef.current.points, point] : [start, point];
			marqueeRef.current.points = points;
			const coverage = marqueeCoverageFromGesture({ method: selectionMethodRef.current, startX: start.x, endX: point.x, path: points });
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
			if (gumballBlocksSelection()) {
				if (marqueeRef.current.tracking) resetGesture();
				return;
			}
			if (!marqueeRef.current.tracking) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const start = marqueeRef.current.start;
			const distance = Math.hypot(point.x - start.x, point.y - start.y);
			const mode = marqueeModeFromModifiers(event);
			if (marqueeRef.current.active && distance >= PROCEDURAL_PREVIEW_MARQUEE_THRESHOLD_PX) {
				const points = selectionMethodRef.current === "lasso" ? [...marqueeRef.current.points, point] : [start, point];
				const coverage = marqueeCoverageFromGesture({ method: selectionMethodRef.current, startX: start.x, endX: point.x, path: points });
				const hits = resolveMarqueeHitsRef.current(points, coverage === "partial");
				const next = selectionMergeIds(mode, marqueeRef.current.initial, hits);
				commitSelectionRef.current(next, mode);
			}
			resetGesture();
			invalidate();
		};
		const bindings = new WorldEventBindingController();
		bindings.listen(canvas, "pointerdown", onPointerDown as EventListener, true);
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
	selectedChannels = [],
	preselectNodeIds = [],
	preselectRemovedNodeIds = [],
	hoveredNodeId = null,
	hoveredChannel = null,
	previewOffNodeIds = [],
	showMode = "everything",
	selectionMode = "default",
	selectionMethod = "rectangle",
	transformGranularity = "full",
	onGumballTransform,
	gumballActiveWidgetIds = [],
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
	const [gumballInteractionWidgetId, setGumballInteractionWidgetId] = useState<string | null>(null);
	const gumballHighlightIds = useMemo(() => new Set(gumballActiveWidgetIds), [gumballActiveWidgetIds]);
	const gumballDragActive = gumballInteractionWidgetId !== null;
	const [canvasBackground, setCanvasBackground] = useState(() => resolveSemanticColorHex("--canvas", "light-8-9"));

	useEffect(() => {
		if (typeof document === "undefined") return;
		const sync = () => {
			clearColorResolveCache();
			setCanvasBackground(resolveSemanticColorHex("--canvas", "light-8-9"));
		};
		sync();
		const obs = new MutationObserver(sync);
		obs.observe(document.documentElement, { attributes: true, attributeFilter: ["class", "style", "data-theme", "data-ui-theme"] });
		return () => obs.disconnect();
	}, []);

	const visibleItems = useMemo(
		() =>
			filterVisiblePreviewItems(items, {
				showMode,
				selectedNodeIds,
				selectedChannels,
				hoveredNodeId,
				hoveredChannel,
			}),
		[hoveredChannel, hoveredNodeId, items, selectedChannels, selectedNodeIds, showMode],
	);

	const gumballAnchorId = reactHostPort.useMemo(() => {
		if (!onGumballTransform) return null;
		if (gumballDragActive) {
			const transformGeometry = gumballActiveWidgetIds.find((id) =>
				items.some((entry) => entry.widgetId === id && entry.kind === "geometry"),
			);
			if (transformGeometry) return transformGeometry;
			if (gumballInteractionWidgetId) return gumballInteractionWidgetId;
		}
		if (selectedNodeIds.length !== 1) return null;
		return selectedNodeIds[0]!;
	}, [gumballActiveWidgetIds, gumballDragActive, gumballInteractionWidgetId, items, onGumballTransform, selectedNodeIds]);

	const gumballItem = reactHostPort.useMemo(() => {
		if (!gumballAnchorId) return null;
		return (
			visibleItems.find(
				(entry): entry is Extract<ProceduralPreviewItem, { kind: "geometry" }> => entry.widgetId === gumballAnchorId && entry.kind === "geometry",
			) ?? null
		);
	}, [gumballAnchorId, visibleItems]);

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
		<div ref={containerRef} className={cn("absolute inset-0", canvasHostRootClass, className)}>
			<WorldCanvas
				className="h-full w-full"
				frameloop={gumballDragActive ? "always" : "demand"}
				background={canvasBackground}
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
							{visibleItems.map((entry) => {
								const interactionHighlighted = gumballHighlightIds.has(entry.widgetId) || gumballInteractionWidgetId === entry.widgetId;
								const locked = gumballDragActive && !interactionHighlighted;
								const chrome: PreviewLayerChrome = {
									selected: effectiveSelected.has(entry.widgetId) || effectivePreselect.has(entry.widgetId),
									highlighted: effectivePreselectRemoved.has(entry.widgetId),
									hovered: hoveredChannel
										? proceduralChannelMatches(entry, hoveredChannel)
										: hoveredNodeId === entry.widgetId,
									previewOff: previewOffNodeIds.includes(entry.widgetId),
									locked,
									interactionHighlighted,
									pickEnabled: !gumballDragActive,
									onHover,
									onPick,
								};
								return <BrepPreviewLayer key={previewItemKey(entry)} item={entry} kernel={kernel} tolerance={tolerance} {...chrome} />;
							})}
							{gumballItem ? (
								<ProceduralPreviewGumball
									key="procedural-gumball"
									item={gumballItem}
									kernel={kernel}
									transformGranularity={transformGranularity}
									onGumballTransform={onGumballTransform}
									onInteractionChange={setGumballInteractionWidgetId}
								/>
							) : null}
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

/** @emoji 🔍 Collects geometry, point, and vector preview items from channel-structured flow eval JSON. */
export function extractChannelPreviewItems(channelJson: string): ProceduralPreviewItem[] {
	const items: ProceduralPreviewItem[] = [];
	try {
		const parsed = JSON.parse(channelJson) as unknown;
		if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) return items;
		if ("error" in (parsed as Record<string, unknown>) && Object.keys(parsed as object).length === 1) return items;
		for (const [widgetId, entry] of Object.entries(parsed as Record<string, unknown>)) {
			if (!entry || typeof entry !== "object" || Array.isArray(entry)) continue;
			const channels = entry as { in?: Record<string, unknown>; out?: Record<string, unknown> };
			for (const [port, value] of Object.entries(channels.in ?? {})) {
				items.push(...previewItemsFromChannelValue(widgetId, port, "in", value));
			}
			for (const [port, value] of Object.entries(channels.out ?? {})) {
				items.push(...previewItemsFromChannelValue(widgetId, port, "out", value));
			}
		}
	} catch {
		/* ignore */
	}
	return items;
}

/** @emoji 🔍 Back-compat alias for channel preview extraction. */
export const extractPreviewItems = extractChannelPreviewItems;
// #endregion 🔖ProceduralPreview

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
	readonly onChannelHoverChange?: (channel: ProceduralChannelRef | null) => void;
	readonly onSelectedChannelsChange?: (channels: readonly ProceduralChannelRef[]) => void;
	readonly selectedNodeIds?: readonly string[];
	readonly preselectNodeIds?: readonly string[];
	readonly preselectRemovedNodeIds?: readonly string[];
	readonly hoveredNodeId?: string | null;
	readonly hoveredChannel?: ProceduralChannelRef | null;
	readonly selectedChannels?: readonly ProceduralChannelRef[];
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
	onChannelHoverChange,
	onSelectedChannelsChange,
	selectedNodeIds,
	preselectNodeIds,
	preselectRemovedNodeIds,
	hoveredNodeId,
	hoveredChannel,
	selectedChannels,
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
			store={PROCEDURAL_FLOW_STORE}
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
			onChannelHoverChange={onChannelHoverChange}
			onSelectedChannelsChange={onSelectedChannelsChange}
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

		it("applyBrepInputDefaults injects declared brep defaults", () => {
			const out = applyBrepInputDefaults("brep.prim3d.box", {});
			expect(out).toEqual({ width: 1, depth: 1, height: 1 });
			const extrude = applyBrepInputDefaults("brep.solid.extrude", {});
			expect(extrude.vector).toEqual([0, 0, 5]);
		});

		it("brep.prim3d.box evaluates to geometry handle", () => {
			const out = evaluateBrepFlowKind("brep.prim3d.box", JSON.stringify({ width: 1, depth: 1, height: 1 }), kernel);
			const parsed = JSON.parse(out) as { geometry?: string };
			expect(parsed.geometry).toMatch(/^solid-/);
		});

		it("brep.solid.extrude extrudes sketch and curve circles by vector magnitude", async () => {
			for (const sketchKind of ["brep.sketch2d.circle", "brep.curve.circle"] as const) {
				const sketch = evaluateBrepFlowKind(sketchKind, JSON.stringify({ radius: 2 }), kernel);
				const sketchParsed = JSON.parse(sketch) as { geometry?: string };
				const tallOut = evaluateBrepFlowKind(
					"brep.solid.extrude",
					JSON.stringify({ geometry: sketchParsed.geometry, vector: { x: 0, y: 0, z: 5 } }),
					kernel,
				);
				const tallParsed = JSON.parse(tallOut) as { geometry?: string };
				expect(tallParsed.geometry).toMatch(/^solid-/);
				const mesh = await kernel.tessellateGeometry(tallParsed.geometry as GeometryRef, 0.05);
				expect(isRenderableMeshTransfer(mesh)).toBe(true);
				const bounds = kernel.getBoundsSync(tallParsed.geometry as GeometryRef);
				expect(bounds.max[2] - bounds.min[2]).toBeCloseTo(5, 1);
			}
		});

		it("brep.solid.extrude honors non-z vector direction", async () => {
			const sketch = evaluateBrepFlowKind("brep.sketch2d.circle", JSON.stringify({ radius: 1 }), kernel);
			const sketchParsed = JSON.parse(sketch) as { geometry?: string };
			const out = evaluateBrepFlowKind(
				"brep.solid.extrude",
				JSON.stringify({ geometry: sketchParsed.geometry, vector: { x: 4, y: 0, z: 0 } }),
				kernel,
			);
			const parsed = JSON.parse(out) as { geometry?: string; error?: string };
			expect(parsed.error).toBeUndefined();
			const bounds = kernel.getBoundsSync(parsed.geometry as GeometryRef);
			const profileBounds = kernel.getBoundsSync(sketchParsed.geometry as GeometryRef);
			expect(bounds.max[0] - bounds.min[0]).toBeGreaterThan(profileBounds.max[0] - profileBounds.min[0] + 3);
		});

		it("brep.bool.cut reduces base volume", () => {
			const base = evaluateBrepFlowKind("brep.prim3d.box", JSON.stringify({ width: 2, depth: 2, height: 2 }), kernel);
			const tool = evaluateBrepFlowKind("brep.prim3d.box", JSON.stringify({ width: 1, depth: 1, height: 1 }), kernel);
			const baseParsed = JSON.parse(base) as { geometry?: string };
			const toolParsed = JSON.parse(tool) as { geometry?: string };
			const baseVolume = kernel.measureVolumeSync(baseParsed.geometry as GeometryRef);
			const cutOut = evaluateBrepFlowKind(
				"brep.bool.cut",
				JSON.stringify({ a: baseParsed.geometry, b: toolParsed.geometry }),
				kernel,
			);
			const cutParsed = JSON.parse(cutOut) as { geometry?: string; error?: string };
			expect(cutParsed.error).toBeUndefined();
			expect(kernel.measureVolumeSync(cutParsed.geometry as GeometryRef)).toBeLessThan(baseVolume);
		});

		it("brep.xform.rotate uses axis input", () => {
			const box = evaluateBrepFlowKind("brep.prim3d.box", JSON.stringify({ width: 2, depth: 1, height: 1 }), kernel);
			const boxParsed = JSON.parse(box) as { geometry?: string };
			const before = kernel.getBoundsSync(boxParsed.geometry as GeometryRef);
			const out = evaluateBrepFlowKind(
				"brep.xform.rotate",
				JSON.stringify({ geometry: boxParsed.geometry, angle: Math.PI / 2, axis: { x: 0, y: 0, z: 1 } }),
				kernel,
			);
			const parsed = JSON.parse(out) as { geometry?: string; error?: string };
			expect(parsed.error).toBeUndefined();
			const after = kernel.getBoundsSync(parsed.geometry as GeometryRef);
			expect(after.min[0]).not.toBeCloseTo(before.min[0], 1);
		});

		it("brep.xform.mirror reflects across plane", () => {
			const box = evaluateBrepFlowKind("brep.prim3d.box", JSON.stringify({ width: 1, depth: 1, height: 1 }), kernel);
			const boxParsed = JSON.parse(box) as { geometry?: string };
			const translated = evaluateBrepFlowKind(
				"brep.xform.translate",
				JSON.stringify({ geometry: boxParsed.geometry, offset: { x: 2, y: 0, z: 0 } }),
				kernel,
			);
			const translatedParsed = JSON.parse(translated) as { geometry?: string };
			const before = kernel.getBoundsSync(translatedParsed.geometry as GeometryRef);
			const out = evaluateBrepFlowKind(
				"brep.xform.mirror",
				JSON.stringify({
					geometry: translatedParsed.geometry,
					origin: { x: 0, y: 0, z: 0 },
					normal: { x: 1, y: 0, z: 0 },
				}),
				kernel,
			);
			const parsed = JSON.parse(out) as { geometry?: string; error?: string };
			expect(parsed.error).toBeUndefined();
			const after = kernel.getBoundsSync(parsed.geometry as GeometryRef);
			expect(after.max[0]).toBeLessThan(0);
			expect(after.min[0]).toBeLessThan(before.min[0]);
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

		it("extractChannelPreviewItems collects geometry, point, and vector outputs per channel", () => {
			const items = extractPreviewItems(
				JSON.stringify({
					box: { in: {}, out: { out: { geometry: "solid-1" } } },
					line: { in: {}, out: { out: { geometry: "edge-2" } } },
					pt: { in: {}, out: { out: { point: { x: 1, y: 2, z: 3 } } } },
					vec: { in: {}, out: { out: { vector: { x: 0, y: 0, z: 1 } } } },
				}),
			);
			expect(items).toEqual([
				{ widgetId: "box", port: "out", direction: "out", kind: "geometry", handle: "solid-1" },
				{ widgetId: "line", port: "out", direction: "out", kind: "geometry", handle: "edge-2" },
				{ widgetId: "pt", port: "out", direction: "out", kind: "point", position: [1, 2, 3] },
				{ widgetId: "vec", port: "out", direction: "out", kind: "vector", directionVec: [0, 0, 1] },
			]);
		});

		it("extractChannelPreviewItems keeps input channels when output errors", () => {
			const items = extractPreviewItems(
				JSON.stringify({
					offset: {
						in: { geometry: "drawing-1", distance: 0.1 },
						out: { out: { error: "brep: drawing-1 expected solid|face|wire, got drawing" } },
						error: "brep: drawing-1 expected solid|face|wire, got drawing",
					},
				}),
			);
			expect(items).toContainEqual({
				widgetId: "offset",
				port: "geometry",
				direction: "in",
				kind: "geometry",
				handle: "drawing-1",
			});
		});

		it("previewPickProxyFromBounds enforces a minimum pick size for flat profiles", async () => {
			const { previewPickProxyFromBounds: pickProxy } = await import("./index.tsx");
			const proxy = pickProxy({ min: [-2, -2, -1e-7], max: [2, 2, 1e-7] });
			expect(proxy.position).toEqual([0, 0, 0]);
			expect(proxy.size[0]).toBe(4);
			expect(proxy.size[2]).toBeGreaterThanOrEqual(0.08);
		});

		it("filterVisiblePreviewItems keeps all items visible on hover", async () => {
			const { filterVisiblePreviewItems: filter } = await import("./index.tsx");
			const items = [
				{ widgetId: "offset", port: "geometry", direction: "in" as const, kind: "geometry" as const, handle: "drawing-1" as const },
				{ widgetId: "offset", port: "out", direction: "out" as const, kind: "geometry" as const, handle: "wire-2" as const },
				{ widgetId: "box", port: "out", direction: "out" as const, kind: "geometry" as const, handle: "solid-3" as const },
			];
			const visible = filter(items, {
				showMode: "everything",
				selectedNodeIds: [],
				selectedChannels: [],
				hoveredNodeId: "offset",
				hoveredChannel: { widgetId: "offset", port: "geometry", direction: "in" },
			});
			expect(visible).toHaveLength(2);
			expect(visible.map((entry) => entry.widgetId).sort()).toEqual(["box", "offset"]);
		});

		it("filterVisiblePreviewItems keeps all outputs visible when flow selection is active in everything mode", async () => {
			const { filterVisiblePreviewItems: filter } = await import("./index.tsx");
			const items = [
				{ widgetId: "circle", port: "out", direction: "out" as const, kind: "geometry" as const, handle: "drawing-1" as const },
				{ widgetId: "box", port: "out", direction: "out" as const, kind: "geometry" as const, handle: "solid-2" as const },
			];
			const visible = filter(items, {
				showMode: "everything",
				selectedNodeIds: ["circle"],
				selectedChannels: [{ widgetId: "circle", port: "out", direction: "out" }],
				hoveredNodeId: null,
				hoveredChannel: null,
			});
			expect(visible).toHaveLength(2);
		});

		it("filterVisiblePreviewItems isolates selection only in selected mode", async () => {
			const { filterVisiblePreviewItems: filter } = await import("./index.tsx");
			const items = [
				{ widgetId: "circle", port: "out", direction: "out" as const, kind: "geometry" as const, handle: "drawing-1" as const },
				{ widgetId: "box", port: "out", direction: "out" as const, kind: "geometry" as const, handle: "solid-2" as const },
			];
			const visible = filter(items, {
				showMode: "selected",
				selectedNodeIds: ["circle"],
				selectedChannels: [],
				hoveredNodeId: null,
				hoveredChannel: null,
			});
			expect(visible).toHaveLength(1);
			expect(visible[0]?.widgetId).toBe("circle");
		});

		it("brep.point and brep.vector evaluate to previewable outputs", () => {
			const pointOut = evaluateBrepFlowKind("brep.point", JSON.stringify({ x: 1, y: 2, z: 3 }), kernel);
			const vectorOut = evaluateBrepFlowKind("brep.vector", JSON.stringify({ x: 0, y: 0, z: 5 }), kernel);
			const items = extractPreviewItems(
				JSON.stringify({
					pointNode: { in: {}, out: { out: JSON.parse(pointOut) } },
					vectorNode: { in: {}, out: { out: JSON.parse(vectorOut) } },
				}),
			);
			expect(items).toContainEqual({ widgetId: "pointNode", port: "out", direction: "out", kind: "point", position: [1, 2, 3] });
			expect(items).toContainEqual({ widgetId: "vectorNode", port: "out", direction: "out", kind: "vector", directionVec: [0, 0, 5] });
		});

		it("brep.solid.offset offsets sketch and curve circles", () => {
			for (const sketchKind of ["brep.sketch2d.circle", "brep.curve.circle"] as const) {
				const sketch = evaluateBrepFlowKind(sketchKind, JSON.stringify({ radius: 2 }), kernel);
				const sketchParsed = JSON.parse(sketch) as { geometry?: string };
				const out = evaluateBrepFlowKind(
					"brep.solid.offset",
					JSON.stringify({ geometry: sketchParsed.geometry, distance: 0.25 }),
					kernel,
				);
				const parsed = JSON.parse(out) as { geometry?: string; error?: string };
				expect(parsed.error).toBeUndefined();
				expect(parsed.geometry).toMatch(/^(wire|face|edge|drawing|solid)-/);
			}
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
							{ widgetId: "pt", port: "out", direction: "out", kind: "point", position: [1, 0, 0] },
							{ widgetId: "vec", port: "out", direction: "out", kind: "vector", directionVec: [0, 0, 2] },
						]}
					/>,
				);
			});
			expect(host.querySelector("[data-world-projection-switch]")).not.toBeNull();
			root.unmount();
			host.remove();
		});

		it("procedural host nests brep kinds into one authored tree section", async () => {
			const host = new ProceduralExtensionHost(kernel);
			await host.activateDefaults();
			const sections = host.catalogueSections();
			const brep = sections.find((section) => section.id === "brep");
			expect(brep?.title).toBe("Brep");
			expect(brep?.groups?.some((group) => group.title === "Primitives 3D")).toBe(true);
			expect(brep?.groups?.some((group) => group.title === "Curves")).toBe(true);
			expect(brep?.groups?.some((group) => group.title === "Solid")).toBe(true);
			const prim3d = brep?.groups?.find((group) => group.title === "Primitives 3D");
			expect(prim3d?.items.some((item) => item.neuronKind === "brep.prim3d.box")).toBe(true);
			expect(prim3d?.items[0]?.abbreviation).toBeTruthy();
			expect(prim3d?.items[0]?.icon).toBeTruthy();
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

		it("brep.eval.divideCurve returns indexed point list", () => {
			const line = evaluateBrepFlowKind("brep.curve.line", JSON.stringify({ start: [0, 0, 0], end: [4, 0, 0] }), kernel);
			const lineParsed = JSON.parse(line) as { geometry?: string };
			const out = evaluateBrepFlowKind(
				"brep.eval.divideCurve",
				JSON.stringify({ geometry: lineParsed.geometry, count: 3 }),
				kernel,
			);
			const parsed = JSON.parse(out) as { list?: Record<string, { x: number }> };
			expect(parsed.list?.["0"]?.x).toBeCloseTo(0, 3);
			expect(parsed.list?.["2"]?.x).toBeCloseTo(4, 3);
		});

		it("brep.curve.reparametrize rebuilds curve geometry", () => {
			const line = evaluateBrepFlowKind("brep.curve.line", JSON.stringify({ start: [0, 0, 0], end: [2, 0, 0] }), kernel);
			const lineParsed = JSON.parse(line) as { geometry?: string };
			const out = evaluateBrepFlowKind(
				"brep.curve.reparametrize",
				JSON.stringify({ geometry: lineParsed.geometry, samples: 16 }),
				kernel,
			);
			const parsed = JSON.parse(out) as { geometry?: string };
			expect(parsed.geometry).toMatch(/^wire-/);
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
