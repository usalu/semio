//#region 🔌️Adapters
import openCascadeWasmBundledUrl from "brepjs-opencascade/src/brepjs_single.wasm?url";
import * as implementation from "brepjs";
import initOpenCascade from "brepjs-opencascade";
//#endregion 🔌️Adapters

//#region 🔖️OwnedBrepContract
export type OwnedBrepEdge = object;
export type OwnedBrepFace = object;
export type OwnedBrepOrientedFace = object;
export type OwnedBrepShape = object;
export type OwnedBrepSolid = object;
export type OwnedBrepWire = object;
export type OwnedBrepShell = object;
export type OwnedBrepVertex = object;
export type OwnedBrepResult<T> = { readonly value: T } | { readonly error?: unknown };
export type OwnedBrepMesh = object;
export type OwnedBrepEdgeMesh = object & { readonly edgeGroups: readonly { readonly start: number; readonly count: number; readonly edgeId: number }[] };
export type OwnedBrepGroupedGeometry = {
  readonly position: Float32Array;
  readonly normal: Float32Array;
  readonly index: Uint32Array;
  readonly groups: readonly { readonly start: number; readonly count: number; readonly faceId: number }[];
};
export type OwnedBrepLineGeometry = { readonly position: Float32Array };
//#endregion 🔖️OwnedBrepContract

//#region 🧱️Implementation
type ImplementationFunction = (...args: readonly unknown[]) => unknown;

function invoke<T>(operation: keyof typeof implementation, args: readonly unknown[]): T {
  return (implementation[operation] as unknown as ImplementationFunction)(...args) as T;
}

export const ownedOpenCascadeWasmBundledUrl: string = openCascadeWasmBundledUrl;

/** @emoji 📂️ Resolves the owned OpenCascade WASM implementation for Node-based tests. */
export async function resolveOwnedOpenCascadeWasmFileUrl(): Promise<string> {
  const { createRequire } = await import("node:module");
  const { pathToFileURL } = await import("node:url");
  return pathToFileURL(createRequire(import.meta.url).resolve("brepjs-opencascade/src/brepjs_single.wasm")).href;
}

/** @emoji 🧩️ Initializes the external OpenCascade runtime and binds it to the owned B-Rep surface. */
export async function initializeOwnedOpenCascade(locateFile: (path: string) => string): Promise<void> {
  const openCascade = await (initOpenCascade as (options?: { locateFile?: (path: string) => string }) => Promise<unknown>)({ locateFile });
  invoke<void>("initFromOC", [openCascade]);
}

export function box(...args: readonly unknown[]): OwnedBrepSolid {
  return invoke("box", args);
}

export function bsplineApprox(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepEdge> {
  return invoke("bsplineApprox", args);
}

export function circle(...args: readonly unknown[]): OwnedBrepEdge {
  return invoke("circle", args);
}

export function cone(...args: readonly unknown[]): OwnedBrepSolid {
  return invoke("cone", args);
}

export function curveEndPoint(...args: readonly unknown[]): [number, number, number] {
  return invoke("curveEndPoint", args);
}

export function curveStartPoint(...args: readonly unknown[]): [number, number, number] {
  return invoke("curveStartPoint", args);
}

export function curveLength(...args: readonly unknown[]): number {
  return invoke("curveLength", args);
}

export function cylinder(...args: readonly unknown[]): OwnedBrepSolid {
  return invoke("cylinder", args);
}

export function cut(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepSolid> {
  return invoke("cut", args);
}

export function intersect(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepSolid> {
  return invoke("intersect", args);
}

export function sweep(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepShape> {
  return invoke("sweep", args);
}

export function extrude(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepSolid> {
  return invoke("extrude", args);
}

export function face(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepOrientedFace> {
  return invoke("face", args);
}

export function filledFace(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepOrientedFace> {
  return invoke("filledFace", args);
}

export function healSolid(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepSolid> {
  return invoke("healSolid", args);
}

export function loft(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepShape> {
  return invoke("loft", args);
}

export function thicken(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepShape> {
  return invoke("thicken", args);
}

export function translate(...args: readonly unknown[]): OwnedBrepShape {
  return invoke("translate", args);
}

export function wire(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepWire> {
  return invoke("wire", args);
}

export function getCurveType(...args: readonly unknown[]): unknown {
  return invoke("getCurveType", args);
}

export function getEdges(...args: readonly unknown[]): OwnedBrepEdge[] {
  return invoke("getEdges", args);
}

export function getFaces(...args: readonly unknown[]): OwnedBrepFace[] {
  return invoke("getFaces", args);
}

export function getHashCode(...args: readonly unknown[]): number {
  return invoke("getHashCode", args);
}

export function getSurfaceType(...args: readonly unknown[]): OwnedBrepResult<unknown> {
  return invoke("getSurfaceType", args);
}

export function isOk<T>(result: OwnedBrepResult<T>): result is { readonly value: T } {
  return invoke("isOk", [result]);
}

export function isSolid(value: unknown): value is OwnedBrepShape {
  return invoke("isSolid", [value]);
}

export function isValidSolid(value: unknown): value is OwnedBrepSolid {
  return invoke("isValidSolid", [value]);
}

export function verticesOfEdge(...args: readonly unknown[]): OwnedBrepVertex[] {
  return invoke("verticesOfEdge", args);
}

export function line(...args: readonly unknown[]): OwnedBrepEdge {
  return invoke("line", args);
}

export function measureArea(...args: readonly unknown[]): OwnedBrepResult<number> {
  return invoke("measureArea", args);
}

export function measureDistance(...args: readonly unknown[]): OwnedBrepResult<number> {
  return invoke("measureDistance", args);
}

export function measureLength(...args: readonly unknown[]): OwnedBrepResult<number> {
  return invoke("measureLength", args);
}

export function measureVolume(...args: readonly unknown[]): OwnedBrepResult<number> {
  return invoke("measureVolume", args);
}

export function mesh(...args: readonly unknown[]): OwnedBrepMesh {
  return invoke("mesh", args);
}

export function meshEdges(...args: readonly unknown[]): OwnedBrepEdgeMesh {
  return invoke("meshEdges", args);
}

export function normalAt(...args: readonly unknown[]): [number, number, number] {
  return invoke("normalAt", args);
}

export function offsetFace(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepShape> {
  return invoke("offsetFace", args);
}

export function fuseAll(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepSolid> {
  return invoke("fuseAll", args);
}

export function sewShells(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepShell> {
  return invoke("sewShells", args);
}

export function solidFromShell(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepSolid> {
  return invoke("solidFromShell", args);
}

export function sphere(...args: readonly unknown[]): OwnedBrepSolid {
  return invoke("sphere", args);
}

export function threePointArc(...args: readonly unknown[]): OwnedBrepEdge {
  return invoke("threePointArc", args);
}

export function toGroupedBufferGeometryData(...args: readonly unknown[]): OwnedBrepGroupedGeometry {
  return invoke("toGroupedBufferGeometryData", args);
}

export function toLineGeometryData(...args: readonly unknown[]): OwnedBrepLineGeometry {
  return invoke("toLineGeometryData", args);
}

export function unwrap<T>(result: OwnedBrepResult<T>): T {
  return invoke("unwrap", [result]);
}

export function vertex(...args: readonly unknown[]): OwnedBrepVertex {
  return invoke("vertex", args);
}

export function wireLoop(...args: readonly unknown[]): OwnedBrepResult<OwnedBrepWire> {
  return invoke("wireLoop", args);
}
//#endregion 🧱️Implementation
