/** 🔌️ Host-port bindings (`useFrame`, `useThree`, drei `Line`/`Text`, `THREE` with the world-up side effect), kernel record aliases and the fast R3F preview kernel with its bound scene preview kernel.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { sceneHostPort } from "@semio-tech/ui-react";
import { kernelGeometry, type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { type SpatialPreviewKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { PreciseSpatialKernelMath, preciseSpatialKernelMath } from "@semio-tech/s-spatial-kernel-ts/preview";

// #region 🔌️PortWiring
const useFrame = sceneHostPort.fiber.useFrame;
export const useThree = sceneHostPort.fiber.useThree;
export const { Line, Text } = sceneHostPort.drei;
export const THREE = sceneHostPort.three;
THREE.Object3D.DEFAULT_UP.set(0, 0, 1);
// #endregion 🔌️PortWiring

export type AnchorRecord = kernelGeometry.AnchorRecord;
type AnchorRef = kernelGeometry.AnchorRef;
type VertexRef = kernelGeometry.VertexRef;
export type ShellRef = kernelGeometry.ShellRef;
export type SolidRef = kernelGeometry.SolidRef;
export type SolidRecord = kernelGeometry.SolidRecord;
export type EdgeRecord = kernelGeometry.EdgeRecord;
export type FaceRecord = kernelGeometry.FaceRecord;
export type ShellRecord = kernelGeometry.ShellRecord;
export type VertexRecord = kernelGeometry.VertexRecord;
export type WireRecord = kernelGeometry.WireRecord;
export type WireRef = kernelGeometry.WireRef;
export type EdgeRef = kernelGeometry.EdgeRef;

// #region ⚡️R3FPreviewKernel
/** ⚡️ Fast approximate `SpatialPreviewKernel` for live R3F previews (lower tessellation). */
export class R3FPreviewKernel extends PreciseSpatialKernelMath {
  override arcSamplePoints = (center: Vec3, start: Vec3, end: Vec3, segments = 12): readonly Vec3[] => preciseSpatialKernelMath.arcSamplePoints(center, start, end, segments);

  override edgeSamplePoints = (vertices: Readonly<Record<string, VertexRecord>>, edge: EdgeRecord, segments = 12): readonly Vec3[] => preciseSpatialKernelMath.edgeSamplePoints(vertices, edge, segments);

  override circleSamplePoints = (center: Vec3, normal: Vec3, radius: number, segments = 24): readonly Vec3[] => preciseSpatialKernelMath.circleSamplePoints(center, normal, radius, segments);

  override nurbsDisplaySamplePoints = (poles: readonly Vec3[], segmentsPerSpan = 6): readonly Vec3[] => preciseSpatialKernelMath.nurbsDisplaySamplePoints(poles, segmentsPerSpan);
}

/** ⚡️ Default fast preview kernel for play and R3F hosts. */
export const r3fPreviewKernel = new R3FPreviewKernel();

const scenePreviewKernelRef: { current: SpatialPreviewKernel } = { current: r3fPreviewKernel };

/** ⚡️ Binds the active scene preview kernel (fast vs precise) for R3F wireframe helpers. */
export function bindScenePreviewKernel(kernel: SpatialPreviewKernel): void {
  scenePreviewKernelRef.current = kernel;
}

export function scenePreview(): SpatialPreviewKernel {
  return scenePreviewKernelRef.current;
}
// #endregion ⚡️R3FPreviewKernel
