import { useCallback, useEffect, useMemo, useRef, useState, Suspense, type ComponentProps } from "react";
import { Box3, BufferAttribute, BufferGeometry, Color, DoubleSide, EdgesGeometry, Group, LineBasicMaterial, LineSegments, Mesh, MeshStandardMaterial, Object3D, PointsMaterial, Quaternion, TextureLoader, Vector3, type ThreeEvent } from "three";
import { useFrame, useLoader, useThree } from "@react-three/fiber";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import {
  DEFAULT_LOD_GRID_FACTOR,
  DEFAULT_MANUAL_LOD,
  GLB_MESH_FRAME_ROTATION_X,
  WORLD_MESH_OUTLINE_USER_DATA_KEY,
  WorldCanvas,
  WorldLayerStack,
  WorldLodBridge,
  WorldOrbitCameraViewRig,
  WorldOrbitGated,
  WorldOrbitProjectionSwitch,
  WorldOrbitViewSnapGateProvider,
  WorldReferenceLayer,
  WorldVolumeLayer,
  type OrbitCameraProjection,
  type WorldCameraState,
} from "@semio-tech/infinite-world-r3f";
import {
  IconShotFrame,
  UnifiedGumball,
  ContextMenuController,
  marqueeCoverageFromGesture,
  marqueeModeFromModifiers,
  menuListItemClassName,
  sceneHostPort,
  SelectionMarquee,
  sunPositionFromAzimuthElevation,
  type GumballConfig,
  type GumballHandleKind,
  type GumballPose,
  type SelectionMarqueeCoverage,
  type SelectionMarqueeMethod,
  type SelectionMarqueePoint,
  useCanvasAppearanceSync,
  useLabel,
} from "@semio-tech/ui-react";
import { clearColorResolveCache, resolveColorHex, semanticVar, themeColorVar, tokenVar } from "@semio-tech/ui-styling";
import type { ComponentSceneHostProps } from "@semio-tech/framework-core";
import { WorldTerrainLayer } from "./world-terrain-layer.tsx";

//#region WorldSceneParsing
type WorldMeshData = {
  readonly positions: readonly number[];
  readonly normals: readonly number[];
  readonly indices: readonly number[];
  readonly uvs?: readonly number[];
  readonly faceIds?: readonly number[];
  readonly vertexIds?: readonly number[];
  readonly edgePositions?: readonly number[];
  readonly edgeIds?: readonly number[];
  readonly paintTextureBase64?: string;
};

type WorldCameraRecord = {
  readonly position?: readonly [number, number, number];
  readonly target?: readonly [number, number, number];
  readonly fov?: number;
  readonly x?: number;
  readonly y?: number;
  readonly z?: number;
};

type WorldMeshRecord = {
  readonly id: string;
  readonly data?: WorldMeshData;
  readonly url?: string;
};

type WorldInstanceRecord = {
  readonly id: string;
  readonly meshId?: string;
  readonly position?: readonly [number, number, number];
  readonly rotation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number];
  readonly x?: number;
  readonly y?: number;
  readonly z?: number;
  readonly selected?: boolean;
  readonly hovered?: boolean;
  /** 🎨 Compatible/suggested state (e.g. catalog-kind hover in puzzle) — resolves to the secondary "highlighted" mesh style. */
  readonly highlighted?: boolean;
  /** 🎨 Non-interactive/locked state — resolves to the muted "disabled" mesh style at reduced opacity. */
  readonly disabled?: boolean;
  readonly smoothShading?: boolean;
};

type WorldSelectionTargets = {
  readonly mesh?: boolean;
  readonly vertex?: boolean;
  readonly edge?: boolean;
  readonly face?: boolean;
};

type WorldHoverComponent = {
  readonly objectId?: string;
  readonly mode?: string;
  readonly id?: number;
};

type WorldContextMenuItem = {
  readonly id: string;
  readonly label: string;
  readonly action: string;
  readonly args?: Record<string, unknown>;
};

type WorldSelectionRecord = {
  readonly method?: SelectionMarqueeMethod;
  readonly ids?: readonly string[];
  readonly hoveredId?: string | null;
  readonly referenceSelectedId?: string;
  readonly granularity?: string;
  readonly selectionMode?: string;
  readonly activeObjectId?: string;
  readonly componentIds?: readonly number[];
  readonly targets?: WorldSelectionTargets;
  readonly transformTool?: string;
  readonly interactionMode?: "model" | "paint";
  readonly gumballTarget?: readonly [number, number, number];
  readonly gumballActive?: boolean;
  readonly hoveredComponent?: WorldHoverComponent;
  readonly showEdges?: boolean;
  readonly engagementSessionActive?: boolean;
  /** 🖱️➡️ When true and `targets.face` is set, dragging an already-selected face starts a push/pull gesture (`worldFaceDragEnd` on release) instead of the default marquee/orbit. */
  readonly faceDragActive?: boolean;
};

type WorldSuggestionCandidateRecord = {
  readonly index: number;
  readonly objectLabel: string;
  readonly vortexLabel: string;
};

type WorldSuggestionMenuRecord = {
  readonly open: boolean;
  readonly x: number;
  readonly y: number;
  readonly pending: boolean;
  readonly candidates: readonly WorldSuggestionCandidateRecord[];
};

type WorldFillBuildRecord = {
  readonly count: number;
  readonly maxCount: number;
  readonly done: boolean;
};

type WorldInteractionRecord = {
  readonly activeUtility?: string;
  readonly brushCandidateIndex?: number;
  readonly hoveredVortexFullId?: string;
  readonly fillEditTargetVolumes?: boolean;
  readonly voxelDims?: readonly [number, number, number];
  readonly gridFactor?: number;
  readonly suggestionMenu?: WorldSuggestionMenuRecord | null;
  readonly fillBuild?: WorldFillBuildRecord;
};

type WorldLodRecord = {
  readonly gridFactor?: number;
  readonly gridSnapEnabled?: boolean;
  readonly showLodGrid?: boolean;
  readonly automaticLod?: boolean;
  readonly depthVariableLod?: boolean;
  readonly manualLod?: number;
};

type WorldVortexRecord = {
  readonly fullId: string;
  readonly objectId?: string;
  readonly vortexKind?: string;
  readonly position: readonly [number, number, number];
  readonly direction?: readonly [number, number, number];
  readonly radius?: number;
  readonly color?: string;
  readonly selected?: boolean;
  readonly hovered?: boolean;
};

type WorldAttractionRecord = {
  readonly id: string;
  readonly from: readonly [number, number, number];
  readonly to: readonly [number, number, number];
  readonly color?: string;
};

type WorldTargetVolumeRecord = {
  readonly id: string;
  readonly origin: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number] | number;
  readonly color?: string;
};

type WorldReferenceRecord = {
  readonly id: string;
  readonly url: string;
  readonly origin: readonly [number, number, number];
  readonly widthWorld?: number;
  readonly locked?: boolean;
  readonly hidden?: boolean;
  readonly opacity?: number;
};

type WorldBrushPreviewRecord = {
  readonly targetVortexFullId?: string;
  readonly objectKindId?: string;
  readonly sourceVortexIndex?: number;
  readonly meshUrl?: string;
  readonly origin?: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number] | number;
};

type WorldEngagementPreviewPoint = {
  readonly kind: "point";
  readonly role?: string;
  readonly position: readonly [number, number, number];
};

type WorldEngagementPreviewSegment = {
  readonly kind: "segment";
  readonly role?: string;
  readonly from: readonly [number, number, number];
  readonly to: readonly [number, number, number];
};

type WorldEngagementPreviewBox = {
  readonly kind: "box-preview";
  readonly role?: string;
  readonly cornerA?: readonly [number, number, number];
  readonly cornerB?: readonly [number, number, number];
  readonly height?: number;
};

type WorldEngagementPreviewLinearHandle = {
  readonly kind: "linear-handle";
  readonly role?: string;
  readonly axis: readonly [number, number, number];
  readonly origin: readonly [number, number, number];
};

type WorldEngagementPreviewItem = WorldEngagementPreviewPoint | WorldEngagementPreviewSegment | WorldEngagementPreviewBox | WorldEngagementPreviewLinearHandle;

//#region WorldMeshPaint
/** 🎨 Mesh style kinds, in {@link resolveMeshStyle} priority order (highest first). */
type MeshStyleKind = "disabled" | "selected" | "highlighted" | "hovered" | "neutral";

type MeshStyleColors = {
  readonly meshColor: string;
  readonly lineColor: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
};

type MeshStylePalette = Readonly<Record<MeshStyleKind, MeshStyleColors>>;

/** 🎨 CSS-expression paint spec per style kind, ported from the premigration puzzle 3d paint table. */
const MESH_STYLE_PAINT: Readonly<Record<MeshStyleKind, { readonly fill: string; readonly line: string; readonly emissiveIntensity: number; readonly opacity: number }>> = {
  neutral: { fill: "var(--panel)", line: semanticVar("border-normal-color"), emissiveIntensity: 0, opacity: 1 },
  hovered: { fill: semanticVar("hover-interactive-fill"), line: semanticVar("border-emphasized-color"), emissiveIntensity: 0.08, opacity: 1 },
  selected: { fill: tokenVar("primary"), line: tokenVar("primary"), emissiveIntensity: 0.35, opacity: 1 },
  highlighted: { fill: tokenVar("secondary"), line: tokenVar("secondary"), emissiveIntensity: 0.2, opacity: 1 },
  disabled: { fill: "color-mix(in oklab, var(--color-muted-foreground) 55%, var(--panel))", line: themeColorVar("muted-foreground"), emissiveIntensity: 0, opacity: 0.45 },
};

/** 🎨 Resolves the full {@link MeshStylePalette} from live CSS custom properties (theme/dark-mode aware). */
function resolveMeshStylePalette(): MeshStylePalette {
  const resolved = {} as Record<MeshStyleKind, MeshStyleColors>;
  for (const kind of Object.keys(MESH_STYLE_PAINT) as MeshStyleKind[]) {
    const spec = MESH_STYLE_PAINT[kind];
    resolved[kind] = {
      meshColor: resolveColorHex(spec.fill),
      lineColor: resolveColorHex(spec.line),
      emissiveIntensity: spec.emissiveIntensity,
      opacity: spec.opacity,
    };
  }
  return resolved;
}

function useMeshStylePalette(): MeshStylePalette {
  const [palette, setPalette] = useState(resolveMeshStylePalette);
  useCanvasAppearanceSync(
    useCallback(() => {
      // 🎨 resolveColorHex caches by CSS-expression string only (no theme key), so a theme flip must bust it before re-resolving or every kind keeps its stale color.
      clearColorResolveCache();
      setPalette(resolveMeshStylePalette());
    }, []),
  );
  return palette;
}

/** 🎨 Resolves the effective style kind for an instance/component, premigration priority: disabled → selected → highlighted → hovered → neutral. */
export function resolveMeshStyle(state: { readonly disabled?: boolean; readonly selected?: boolean; readonly highlighted?: boolean; readonly hovered?: boolean }): MeshStyleKind {
  if (state.disabled) return "disabled";
  if (state.selected) return "selected";
  if (state.highlighted) return "highlighted";
  if (state.hovered) return "hovered";
  return "neutral";
}

/** 🎨 Slim alias over {@link MeshStylePalette} for call sites that only need the four legacy semantic colors (face/edge/vertex component overlays, markers). */
type SemanticColors = {
  readonly mesh: string;
  readonly edge: string;
  readonly select: string;
  readonly hover: string;
};

function semanticColorsFromPalette(palette: MeshStylePalette): SemanticColors {
  return {
    mesh: palette.neutral.meshColor,
    edge: palette.neutral.lineColor,
    select: palette.selected.lineColor,
    hover: palette.hovered.meshColor,
  };
}
//#endregion WorldMeshPaint

type WorldParsedCameraState = WorldCameraState & { readonly fov: number; readonly explicitProjection: boolean };

function parseCameraState(cameraJson: string): WorldParsedCameraState {
  try {
    const parsed = JSON.parse(cameraJson) as WorldCameraRecord & { target?: readonly [number, number, number]; zoom?: number; up?: readonly [number, number, number]; projection?: string };
    const position: [number, number, number] = parsed.position ? [parsed.position[0], parsed.position[1], parsed.position[2]] : [parsed.x ?? 4, parsed.y ?? -4, parsed.z ?? 3];
    const target: [number, number, number] = parsed.target ? [parsed.target[0], parsed.target[1], parsed.target[2]] : [0, 0, 0];
    const explicitProjection = parsed.projection === "perspective" || parsed.projection === "orthographic";
    return {
      position,
      target,
      up: parsed.up ? [parsed.up[0], parsed.up[1], parsed.up[2]] : undefined,
      zoom: typeof parsed.zoom === "number" ? parsed.zoom : 1,
      projection: parsed.projection === "orthographic" ? "orthographic" : "perspective",
      fov: parsed.fov ?? 45,
      explicitProjection,
    };
  } catch {
    return { position: [4, -4, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: false };
  }
}

type WorldEnvironmentMaterialRecord = {
  readonly color?: string;
  readonly metalness?: number;
  readonly roughness?: number;
  readonly emissive?: string;
  readonly emissiveIntensity?: number;
};

type WorldEnvironmentRecord = {
  readonly background?: string;
  readonly ambient?: { readonly intensity?: number; readonly color?: string };
  readonly sun?: { readonly enabled?: boolean; readonly azimuth?: number; readonly elevation?: number; readonly intensity?: number; readonly color?: string };
  readonly shadow?: { readonly enabled?: boolean; readonly opacity?: number; readonly softness?: number };
  readonly material?: WorldEnvironmentMaterialRecord;
};

type WorldFrameRecord = {
  readonly width: number;
  readonly height: number;
  readonly shape?: string;
  readonly badge?: boolean;
  readonly background?: string;
};

type WorldFitRecord = {
  readonly enabled?: boolean;
  readonly revision?: number;
  readonly padding?: number;
};

function parseJsonRecord<T>(json?: string): T | null {
  if (!json) return null;
  try {
    const parsed = JSON.parse(json) as T | null;
    return typeof parsed === "object" ? parsed : null;
  } catch {
    return null;
  }
}

const parseEnvironment = (json?: string) => parseJsonRecord<WorldEnvironmentRecord>(json);
const parseFrame = (json?: string) => parseJsonRecord<WorldFrameRecord>(json);
const parseFit = (json?: string) => parseJsonRecord<WorldFitRecord>(json);

function isTransparentWorldBackground(background?: string): boolean {
  return !background || background === "transparent";
}

function fitCameraFromBounds(center: readonly [number, number, number], radius: number, camera: WorldParsedCameraState, padding: number): { position: [number, number, number]; target: [number, number, number]; zoom: number } {
  const distance = Math.max(radius * padding, 2);
  const dx = camera.position[0] - camera.target[0];
  const dy = camera.position[1] - camera.target[1];
  const dz = camera.position[2] - camera.target[2];
  const length = Math.hypot(dx, dy, dz);
  const nx = length > 1e-6 ? dx / length : 1;
  const ny = length > 1e-6 ? dy / length : -1;
  const nz = length > 1e-6 ? dz / length : 0.85;
  const norm = Math.hypot(nx, ny, nz) || 1;
  return {
    position: [center[0] + (nx / norm) * distance, center[1] + (ny / norm) * distance, center[2] + (nz / norm) * distance],
    target: [center[0], center[1], center[2]],
    zoom: camera.zoom,
  };
}

/** @emoji 🎯 Fits the orbit camera to the bounds of a scene group once per fit key, preserving the view direction. */
function WorldAutoFit({
  groupRef,
  fitKey,
  padding,
  camera,
  onFitted,
}: {
  readonly groupRef: React.RefObject<Group | null>;
  readonly fitKey: string;
  readonly padding: number;
  readonly camera: WorldParsedCameraState;
  readonly onFitted: (state: WorldCameraState) => void;
}): null {
  const { camera: sceneCamera, controls, invalidate } = useThree();
  const appliedKeyRef = useRef("");
  const targetScratch = useMemo(() => new Vector3(), []);
  useFrame(() => {
    if (!sceneCamera) return;
    const group = groupRef.current;
    if (!group) return;
    if (appliedKeyRef.current === fitKey) return;
    const box = new Box3().setFromObject(group);
    if (box.isEmpty()) return;
    const center = box.getCenter(new Vector3());
    const size = box.getSize(new Vector3());
    const radius = Math.max(size.x, size.y, size.z) * 0.5;
    if (radius <= 0) return;
    appliedKeyRef.current = fitKey;
    const fitted = fitCameraFromBounds([center.x, center.y, center.z], radius, camera, padding);
    const orbit = controls as { target: Vector3; update?: () => void } | null;
    const target = orbit?.target ?? targetScratch;
    target.set(fitted.target[0], fitted.target[1], fitted.target[2]);
    sceneCamera.position.set(fitted.position[0], fitted.position[1], fitted.position[2]);
    if ("zoom" in sceneCamera) sceneCamera.zoom = fitted.zoom;
    sceneCamera.updateProjectionMatrix();
    if (orbit) orbit.update?.();
    else sceneCamera.lookAt(target);
    invalidate();
    onFitted({ ...camera, position: fitted.position, target: fitted.target, zoom: fitted.zoom });
  });
  return null;
}

function autofitCameraFromInstances(instances: readonly WorldInstanceRecord[]): WorldParsedCameraState {
  if (instances.length === 0) {
    return { position: [4, -4, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: false };
  }
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let minZ = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  let maxZ = Number.NEGATIVE_INFINITY;
  for (const instance of instances) {
    const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
    minX = Math.min(minX, position[0]);
    minY = Math.min(minY, position[1]);
    minZ = Math.min(minZ, position[2]);
    maxX = Math.max(maxX, position[0]);
    maxY = Math.max(maxY, position[1]);
    maxZ = Math.max(maxZ, position[2]);
  }
  const center: [number, number, number] = [(minX + maxX) / 2, (minY + maxY) / 2, (minZ + maxZ) / 2];
  const span = Math.max(maxX - minX, maxY - minY, maxZ - minZ, 1);
  const distance = span * 2.5;
  return {
    position: [center[0] + distance * 0.7, center[1] - distance * 0.7, center[2] + distance * 0.45],
    target: center,
    zoom: 1,
    projection: "perspective",
    fov: 45,
    explicitProjection: false,
  };
}

function parseMeshes(meshesJson: string): WorldMeshRecord[] {
  try {
    const parsed = JSON.parse(meshesJson);
    return Array.isArray(parsed) ? (parsed as WorldMeshRecord[]) : [];
  } catch {
    return [];
  }
}

function parseInstances(instancesJson: string): WorldInstanceRecord[] {
  try {
    const parsed = JSON.parse(instancesJson);
    return Array.isArray(parsed) ? (parsed as WorldInstanceRecord[]) : [];
  } catch {
    return [];
  }
}

function parseSelection(selectionJson: string): WorldSelectionRecord {
  try {
    return JSON.parse(selectionJson) as WorldSelectionRecord;
  } catch {
    return { method: "rectangle", ids: [] };
  }
}

function parseJsonArray<T>(json: string | undefined): readonly T[] {
  if (!json) return [];
  try {
    const parsed = JSON.parse(json);
    return Array.isArray(parsed) ? (parsed as T[]) : [];
  } catch {
    return [];
  }
}

function parseInteraction(interactionJson: string | undefined): WorldInteractionRecord {
  if (!interactionJson) return {};
  try {
    return JSON.parse(interactionJson) as WorldInteractionRecord;
  } catch {
    return {};
  }
}

function parseLod(lodJson: string | undefined): WorldLodRecord {
  if (!lodJson) return {};
  try {
    return JSON.parse(lodJson) as WorldLodRecord;
  } catch {
    return {};
  }
}

function parseBrushPreview(brushPreviewJson: string | undefined): WorldBrushPreviewRecord | null {
  if (!brushPreviewJson) return null;
  try {
    return JSON.parse(brushPreviewJson) as WorldBrushPreviewRecord;
  } catch {
    return null;
  }
}

type WorldContextMenuTarget = { readonly kind: "vortex" | "object" | "reference"; readonly id: string };

/** @emoji 🖱️ Resolves which entity a plain right-click should select-then-open a menu for, by priority: hovered vortex, then hovered object component, then hovered reference. */
export function resolveWorldContextMenuTarget(interaction: WorldInteractionRecord, selection: WorldSelectionRecord): WorldContextMenuTarget | null {
  if (interaction.hoveredVortexFullId) return { kind: "vortex", id: interaction.hoveredVortexFullId };
  if (selection.hoveredComponent?.objectId) return { kind: "object", id: selection.hoveredComponent.objectId };
  const hoveredId = selection.hoveredId;
  if (hoveredId?.startsWith("reference:")) return { kind: "reference", id: hoveredId.slice("reference:".length) };
  return null;
}

/** @emoji 🚫 Instance-mesh picking must be disabled for fill/brush engagements — otherwise a click meant for a vortex marker or a fill/voxel gesture falls through and selects/gumballs the underlying object instead. */
export function worldInstancePickBlocked(activeUtility: string | undefined): boolean {
  return activeUtility === "fill" || activeUtility === "brush";
}

/** @emoji 🖱️ In brush mode or vertex selection mode, pointer-down on a vortex persists it as the brush target/selection (`worldVortexSelect`); outside these modes it starts a drag-to-connect gesture instead. */
export function resolveVortexPointerDownIntent(brushMode: boolean, selectionMode?: string): "select" | "connect-drag" {
  return (brushMode || selectionMode === "vertex") ? "select" : "connect-drag";
}

/** @emoji 🧱 Builds the `addBrushObject` action args from a parsed brush preview, or `null` if there is nothing to place yet. */
export function brushObjectPlacementArgs(preview: WorldBrushPreviewRecord | null): Record<string, unknown> | null {
  if (!preview) return null;
  return {
    targetVortexFullId: preview.targetVortexFullId,
    objectKindId: preview.objectKindId,
    sourceVortexIndex: preview.sourceVortexIndex ?? 0,
    origin: preview.origin,
    orientation: preview.orientation,
    scale: preview.scale,
  };
}

function parseEngagementPreview(engagementPreviewJson: string | undefined): readonly WorldEngagementPreviewItem[] {
  return parseJsonArray<WorldEngagementPreviewItem>(engagementPreviewJson);
}

function scaleTuple(scale: WorldBrushPreviewRecord["scale"]): [number, number, number] {
  if (typeof scale === "number") return [scale, scale, scale];
  if (Array.isArray(scale) && scale.length >= 3) return [scale[0]!, scale[1]!, scale[2]!];
  return [1, 1, 1];
}

function geometryFromMesh(mesh: WorldMeshData) {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.positions), 3));
  geometry.setAttribute("normal", new BufferAttribute(new Float32Array(mesh.normals), 3));
  if (mesh.uvs?.length) geometry.setAttribute("uv", new BufferAttribute(new Float32Array(mesh.uvs), 2));
  if (mesh.indices.length > 0) geometry.setIndex([...mesh.indices]);
  return geometry;
}

type VertexPickData = {
  readonly geometry: BufferGeometry;
  readonly vertexIds: readonly number[];
};

function buildVertexPickData(mesh: WorldMeshData): VertexPickData | null {
  if (!mesh.vertexIds?.length) return null;
  const positions: number[] = [];
  const vertexIds: number[] = [];
  const emitted = new Set<number>();
  for (let index = 0; index < mesh.vertexIds.length; index += 1) {
    const id = mesh.vertexIds[index]!;
    if (emitted.has(id)) continue;
    emitted.add(id);
    vertexIds.push(id);
    positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return { geometry, vertexIds };
}

function buildEdgeGeometry(mesh: WorldMeshData): BufferGeometry | null {
  if (!mesh.edgePositions?.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.edgePositions), 3));
  return geometry;
}

function buildFaceOverlayGeometry(mesh: WorldMeshData, faceIds: ReadonlySet<number>): BufferGeometry | null {
  if (!mesh.faceIds?.length || !mesh.indices.length || faceIds.size === 0) return null;
  const positions: number[] = [];
  const normals: number[] = [];
  for (let faceIndex = 0; faceIndex < mesh.faceIds.length; faceIndex += 1) {
    const faceId = mesh.faceIds[faceIndex]!;
    if (!faceIds.has(faceId)) continue;
    const i0 = mesh.indices[faceIndex * 3] ?? 0;
    const i1 = mesh.indices[faceIndex * 3 + 1] ?? 0;
    const i2 = mesh.indices[faceIndex * 3 + 2] ?? 0;
    for (const index of [i0, i1, i2]) {
      positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
      normals.push(mesh.normals[index * 3]!, mesh.normals[index * 3 + 1]!, mesh.normals[index * 3 + 2]!);
    }
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  geometry.setAttribute("normal", new BufferAttribute(new Float32Array(normals), 3));
  return geometry;
}

/** @emoji 🖱️➡️ Approximates a picked face's in-plane size from its triangles' local bounding box, dropping the
 * smallest axis (roughly the one aligned with the face normal for axis-aligned primitive faces) — good
 * enough to size a push/pull tool's footprint without needing a true tangent-plane projection. */
function faceExtentFromMesh(mesh: WorldMeshData, faceId: number): readonly [number, number] | undefined {
  if (!mesh.faceIds?.length || !mesh.indices.length) return undefined;
  let minX = Infinity;
  let maxX = -Infinity;
  let minY = Infinity;
  let maxY = -Infinity;
  let minZ = Infinity;
  let maxZ = -Infinity;
  let found = false;
  for (let faceIndex = 0; faceIndex < mesh.faceIds.length; faceIndex += 1) {
    if (mesh.faceIds[faceIndex] !== faceId) continue;
    found = true;
    for (const corner of [0, 1, 2]) {
      const vertexIndex = mesh.indices[faceIndex * 3 + corner];
      if (vertexIndex == null) continue;
      const x = mesh.positions[vertexIndex * 3] ?? 0;
      const y = mesh.positions[vertexIndex * 3 + 1] ?? 0;
      const z = mesh.positions[vertexIndex * 3 + 2] ?? 0;
      minX = Math.min(minX, x);
      maxX = Math.max(maxX, x);
      minY = Math.min(minY, y);
      maxY = Math.max(maxY, y);
      minZ = Math.min(minZ, z);
      maxZ = Math.max(maxZ, z);
    }
  }
  if (!found) return undefined;
  const extents = [maxX - minX, maxY - minY, maxZ - minZ].sort((a, b) => b - a);
  return [extents[0] ?? 0.2, extents[1] ?? 0.2];
}

function buildEdgeOverlayGeometry(mesh: WorldMeshData, edgeIds: ReadonlySet<number>): BufferGeometry | null {
  if (!mesh.edgeIds?.length || !mesh.edgePositions?.length || edgeIds.size === 0) return null;
  const positions: number[] = [];
  for (let edgeIndex = 0; edgeIndex < mesh.edgeIds.length; edgeIndex += 1) {
    if (!edgeIds.has(mesh.edgeIds[edgeIndex]!)) continue;
    const base = edgeIndex * 6;
    positions.push(mesh.edgePositions[base]!, mesh.edgePositions[base + 1]!, mesh.edgePositions[base + 2]!, mesh.edgePositions[base + 3]!, mesh.edgePositions[base + 4]!, mesh.edgePositions[base + 5]!);
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return geometry;
}

function buildVertexOverlayGeometry(mesh: WorldMeshData, vertexIds: ReadonlySet<number>): BufferGeometry | null {
  const pick = buildVertexPickData(mesh);
  if (!pick) return null;
  const positions: number[] = [];
  for (let index = 0; index < pick.vertexIds.length; index += 1) {
    if (!vertexIds.has(pick.vertexIds[index]!)) continue;
    positions.push(pick.geometry.attributes.position!.getX(index), pick.geometry.attributes.position!.getY(index), pick.geometry.attributes.position!.getZ(index));
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return geometry;
}

function paintTextureUrl(base64: string): string {
  return `data:image/png;base64,${base64}`;
}

function PaintTexturedMesh({
  geometry,
  style,
  textureBase64,
  flatShading,
  children,
  ...meshProps
}: {
  readonly geometry: BufferGeometry;
  readonly style: MeshStyleColors;
  readonly textureBase64?: string;
  readonly flatShading?: boolean;
  readonly children?: React.ReactNode;
} & ComponentProps<"mesh">) {
  const paintMap = textureBase64 ? useLoader(TextureLoader, paintTextureUrl(textureBase64)) : null;
  return (
    <mesh geometry={geometry} {...meshProps}>
      <meshStandardMaterial
        color={style.meshColor}
        map={paintMap ?? undefined}
        side={DoubleSide}
        flatShading={flatShading}
        metalness={0}
        roughness={1}
        emissive={style.meshColor}
        emissiveIntensity={style.emissiveIntensity}
        transparent={style.opacity < 1}
        opacity={style.opacity}
      />
      {children}
    </mesh>
  );
}

//#region GlbMeshStyling
/** 🎨 EdgesGeometry cache keyed by source BufferGeometry — `gltf.scene.clone(true)` shares geometries across every per-instance clone of the same GLB, so this dedupes edge computation across instances. */
const GLB_EDGE_GEOMETRY_CACHE = new WeakMap<BufferGeometry, EdgesGeometry>();

/** 🎨 Adds a border-color {@link EdgesGeometry} outline to every mesh under `root` (idempotent), using the shared {@link GLB_EDGE_GEOMETRY_CACHE}. */
function applyGlbMeshEdgeBorders(root: Object3D, borderColor: string): void {
  // 🧵 Collect targets before mutating: `object.add(...)` during `traverse()` would splice the new
  // (itself a Mesh) child into the live `children` array traverse is still walking, so it gets visited
  // and outlined again — and again — recursing until the stack overflows.
  const targets: Mesh[] = [];
  root.traverse((object) => {
    if (!(object instanceof Mesh)) return;
    const geometry = object.geometry;
    if (!geometry || object.children.some((child) => child.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY])) return;
    targets.push(object);
  });
  for (const object of targets) {
    let edges = GLB_EDGE_GEOMETRY_CACHE.get(object.geometry);
    if (!edges) {
      edges = new EdgesGeometry(object.geometry);
      GLB_EDGE_GEOMETRY_CACHE.set(object.geometry, edges);
    }
    const outline = new LineSegments(edges, new LineBasicMaterial({ color: new Color(borderColor) }));
    outline.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY] = true;
    outline.scale.setScalar(1.001);
    object.add(outline);
  }
}

//#endregion GlbMeshStyling

function GlbInstanceMesh({
  url,
  color,
  emissive,
  emissiveIntensity,
  opacity,
  borderColor,
  material,
  shadowEnabled,
}: {
  readonly url: string;
  readonly color: string;
  readonly emissive: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
  readonly borderColor: string;
  readonly material?: WorldEnvironmentMaterialRecord;
  readonly shadowEnabled?: boolean;
}) {
  const gltf = useLoader(GLTFLoader, url);
  const scene = useMemo(() => {
    const cloned = gltf.scene.clone(true);
    cloned.traverse((child) => {
      if (!(child instanceof Mesh)) return;
      child.material = new MeshStandardMaterial({ metalness: material?.metalness ?? 0, roughness: material?.roughness ?? 1 });
      child.castShadow = shadowEnabled === true;
      child.receiveShadow = shadowEnabled === true;
    });
    applyGlbMeshEdgeBorders(cloned, borderColor);
    return cloned;
    // eslint-disable-next-line react-hooks/exhaustive-deps -- borderColor intentionally excluded: applied once at clone time, then kept in sync imperatively by the effect below without rebuilding the clone.
  }, [gltf.scene, material?.metalness, material?.roughness, shadowEnabled]);

  useEffect(() => {
    scene.traverse((child) => {
      if (child instanceof Mesh) {
        const standard = child.material;
        if (standard instanceof MeshStandardMaterial) {
          standard.color.set(color);
          standard.emissive.set(emissive);
          standard.emissiveIntensity = emissiveIntensity;
          standard.transparent = opacity < 1;
          standard.opacity = opacity;
        }
        return;
      }
      if (child instanceof LineSegments && child.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY]) {
        (child.material as LineBasicMaterial).color.set(borderColor);
      }
    });
  }, [scene, color, emissive, emissiveIntensity, opacity, borderColor]);

  return (
    <group rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>
      <primitive object={scene} />
    </group>
  );
}

function extractGlbCollisionMesh(gltf: Awaited<ReturnType<GLTFLoader["loadAsync"]>>): {
  readonly positions: number[];
  readonly indices: number[];
} {
  const frame = new Object3D();
  frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
  frame.updateMatrixWorld(true);
  const positions: number[] = [];
  const indices: number[] = [];
  let vertexOffset = 0;
  const scratch = new Vector3();
  gltf.scene.updateMatrixWorld(true);
  gltf.scene.traverse((child) => {
    if (!(child instanceof Mesh)) return;
    const geometry = child.geometry;
    const positionAttr = geometry.getAttribute("position");
    if (!positionAttr) return;
    const worldMatrix = frame.matrixWorld.clone().multiply(child.matrixWorld);
    for (let index = 0; index < positionAttr.count; index += 1) {
      scratch.fromBufferAttribute(positionAttr, index).applyMatrix4(worldMatrix);
      positions.push(scratch.x, scratch.y, scratch.z);
    }
    const indexAttr = geometry.index;
    if (indexAttr) {
      for (let index = 0; index < indexAttr.count; index += 1) {
        indices.push(indexAttr.getX(index) + vertexOffset);
      }
    } else {
      for (let index = 0; index < positionAttr.count; index += 3) {
        indices.push(vertexOffset + index, vertexOffset + index + 1, vertexOffset + index + 2);
      }
    }
    vertexOffset += positionAttr.count;
  });
  return { positions, indices };
}

function BrushMeshRegistrar({ url, onRegister }: { readonly url: string; readonly onRegister: (url: string, positions: number[], indices: number[]) => void }) {
  const gltf = useLoader(GLTFLoader, url);
  useEffect(() => {
    const mesh = extractGlbCollisionMesh(gltf);
    if (mesh.positions.length === 0 || mesh.indices.length === 0) return;
    onRegister(url, mesh.positions, mesh.indices);
  }, [gltf, onRegister, url]);
  return null;
}

function gumballConfigForTransformTool(tool: string): GumballConfig {
  if (tool === "rotate") {
    return { moveAxes: false, movePlanes: false, rotate: true, scaleAxes: false, scalePlanes: false, scaleUniform: false };
  }
  if (tool === "scale") {
    return { moveAxes: false, movePlanes: false, rotate: false, scaleAxes: true, scalePlanes: true, scaleUniform: true };
  }
  return { moveAxes: true, movePlanes: true, rotate: false, scaleAxes: false, scalePlanes: false, scaleUniform: false };
}

function SceneGumball({
  target,
  config,
  active,
  onDraggingChanged,
  onDragEnd,
}: {
  readonly target?: readonly [number, number, number];
  readonly config: GumballConfig;
  readonly active: boolean;
  readonly onDraggingChanged: (dragging: boolean) => void;
  readonly onDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
}) {
  const pivotRef = useRef<Object3D>(new Object3D());
  const [ready, setReady] = useState(false);
  useEffect(() => {
    if (!target) return;
    pivotRef.current.position.set(target[0], target[1], target[2]);
    pivotRef.current.quaternion.set(0, 0, 0, 1);
    pivotRef.current.scale.set(1, 1, 1);
    pivotRef.current.updateMatrixWorld(true);
    setReady(true);
  }, [target]);
  if (!active || !target || !ready) return null;
  return (
    <>
      <primitive object={pivotRef.current} />
      <UnifiedGumball
        target={pivotRef.current}
        config={config}
        onDraggingChanged={onDraggingChanged}
        onDragEnd={(kind, before, after) => {
          onDragEnd(kind, before, after);
          pivotRef.current.position.set(target[0], target[1], target[2]);
          pivotRef.current.quaternion.set(0, 0, 0, 1);
          pivotRef.current.scale.set(1, 1, 1);
          pivotRef.current.updateMatrixWorld(true);
        }}
      />
    </>
  );
}

function WorldInstanceNode({
  instance,
  index,
  meshRecord,
  meshData,
  geometry,
  borderGeometry,
  palette,
  vertexPick,
  edgeGeometry,
  paintTextureBase64,
  position,
  scale,
  quaternion,
  targets,
  activeObjectId,
  selectionMode,
  selectedComponentIds,
  previewComponentIds,
  hoveredComponent,
  showEdges,
  pickEnabled,
  onPaintAt,
  paintFromHit,
  flatShading,
  onInstancePointerDown,
  onInstancePointerMove,
  onWorldPick,
  onComponentHover,
  mergeMode,
  previewInstanceSelected,
  environmentMaterial,
  environmentShadowEnabled,
  faceDragActive,
  onFaceDragStart,
}: {
  readonly instance: WorldInstanceRecord;
  readonly index: number;
  readonly meshRecord?: WorldMeshRecord;
  readonly meshData?: WorldMeshData;
  readonly geometry?: BufferGeometry;
  /** 🎨 Shared per-meshId edge outline geometry (see {@link WorldInstancesLayer}'s `geometries` memo); never rebuilt per instance. */
  readonly borderGeometry?: EdgesGeometry;
  readonly palette: MeshStylePalette;
  readonly vertexPick: VertexPickData | null;
  readonly edgeGeometry: BufferGeometry | null;
  readonly paintTextureBase64?: string;
  readonly position: readonly [number, number, number];
  readonly scale: readonly [number, number, number];
  readonly quaternion?: Quaternion;
  readonly targets: WorldSelectionTargets;
  readonly activeObjectId?: string;
  readonly selectionMode: string;
  readonly selectedComponentIds: ReadonlySet<number>;
  readonly previewComponentIds: ReadonlySet<number>;
  readonly hoveredComponent?: WorldHoverComponent;
  readonly showEdges?: boolean;
  readonly pickEnabled: boolean;
  readonly onPaintAt?: (objectId: string, u: number, v: number) => void;
  readonly paintFromHit: (objectId: string, mesh: WorldMeshData, event: ThreeEvent<PointerEvent> & { faceIndex?: number | null; uv?: { x: number; y: number } }) => void;
  readonly flatShading?: boolean;
  readonly onInstancePointerDown: (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => void;
  readonly onInstancePointerMove: (id: string | null) => void;
  readonly onWorldPick: (args: { granularity: string; id: number; merge: string }) => void;
  readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
  readonly mergeMode: (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => string;
  /** 🖱️➡️ When true, pointer-down on an already-selected face starts a push/pull drag instead of falling through to selection/orbit. */
  readonly faceDragActive?: boolean;
  readonly onFaceDragStart?: (args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => void;
  /** Live marquee-drag merged selection state for this instance; undefined when no drag is in progress. */
  readonly previewInstanceSelected?: boolean;
  readonly environmentMaterial?: WorldEnvironmentMaterialRecord;
  readonly environmentShadowEnabled?: boolean;
}) {
  const isActiveObject = instance.id === activeObjectId;
  const effectiveSelected = previewInstanceSelected ?? instance.selected;
  const previewAddedInstance = previewInstanceSelected === true && !instance.selected;
  const colors = semanticColorsFromPalette(palette);
  const styleKind = previewAddedInstance ? "highlighted" : resolveMeshStyle({ disabled: instance.disabled, selected: effectiveSelected, highlighted: instance.highlighted, hovered: instance.hovered });
  const style = palette[styleKind];
  const glbUsesEnvironmentColor = styleKind === "neutral" && environmentMaterial?.color != null;
  const glbColor = glbUsesEnvironmentColor ? environmentMaterial!.color! : style.meshColor;
  const glbEmissive = glbUsesEnvironmentColor && environmentMaterial?.emissive ? environmentMaterial.emissive : style.meshColor;
  const glbEmissiveIntensity = glbUsesEnvironmentColor && environmentMaterial?.emissive ? (environmentMaterial.emissiveIntensity ?? 1) : style.emissiveIntensity;
  const hoveredFaceId = hoveredComponent?.mode === "face" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const hoveredVertexId = hoveredComponent?.mode === "vertex" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const hoveredEdgeId = hoveredComponent?.mode === "edge" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const selectedFaceIds = isActiveObject && selectionMode === "face" ? selectedComponentIds : new Set<number>();
  const selectedVertexIds = isActiveObject && selectionMode === "vertex" ? selectedComponentIds : new Set<number>();
  const selectedEdgeIds = isActiveObject && selectionMode === "edge" ? selectedComponentIds : new Set<number>();
  const previewFaceIds = isActiveObject && selectionMode === "face" ? previewComponentIds : new Set<number>();
  const previewVertexIds = isActiveObject && selectionMode === "vertex" ? previewComponentIds : new Set<number>();
  const previewEdgeIds = isActiveObject && selectionMode === "edge" ? previewComponentIds : new Set<number>();
  const facePreviewOverlay = meshData && previewFaceIds.size > 0 ? buildFaceOverlayGeometry(meshData, previewFaceIds) : null;
  const edgePreviewOverlay = meshData && previewEdgeIds.size > 0 ? buildEdgeOverlayGeometry(meshData, previewEdgeIds) : null;
  const vertexPreviewOverlay = meshData && previewVertexIds.size > 0 ? buildVertexOverlayGeometry(meshData, previewVertexIds) : null;
  const faceSelectedOverlay = meshData ? buildFaceOverlayGeometry(meshData, selectedFaceIds) : null;
  const faceHoveredOverlay = meshData && hoveredFaceId != null ? buildFaceOverlayGeometry(meshData, new Set([hoveredFaceId])) : null;
  const edgeSelectedOverlay = meshData ? buildEdgeOverlayGeometry(meshData, selectedEdgeIds) : null;
  const edgeHoveredOverlay = meshData && hoveredEdgeId != null ? buildEdgeOverlayGeometry(meshData, new Set([hoveredEdgeId])) : null;
  const vertexSelectedOverlay = meshData ? buildVertexOverlayGeometry(meshData, selectedVertexIds) : null;
  const vertexHoveredOverlay = meshData && hoveredVertexId != null ? buildVertexOverlayGeometry(meshData, new Set([hoveredVertexId])) : null;

  return (
    <group position={position as [number, number, number]} scale={scale as [number, number, number]} quaternion={quaternion}>
      {geometry && meshData ? (
        <>
          <PaintTexturedMesh
            geometry={geometry}
            style={style}
            textureBase64={paintTextureBase64}
            flatShading={flatShading}
            onPointerDown={(event) => {
              if (onPaintAt || !faceDragActive || !onFaceDragStart || !event.face) return;
              if (!(targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null)) return;
              const faceId = meshData.faceIds[event.faceIndex]!;
              if (!(isActiveObject && selectionMode === "face" && selectedComponentIds.has(faceId))) return;
              event.stopPropagation();
              const normal = event.face.normal.clone().transformDirection(event.object.matrixWorld).normalize();
              onFaceDragStart({
                objectId: instance.id,
                faceId,
                normal: [normal.x, normal.y, normal.z],
                point: [event.point.x, event.point.y, event.point.z],
                faceExtent: faceExtentFromMesh(meshData, faceId),
              });
            }}
            onClick={(event) => {
              if (onPaintAt) {
                paintFromHit(instance.id, meshData, event);
                return;
              }
              if (!pickEnabled) return;
              event.stopPropagation();
              if (targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null) {
                onWorldPick({
                  granularity: "face",
                  id: meshData.faceIds[event.faceIndex]!,
                  merge: mergeMode(event),
                });
              } else if (targets.mesh) {
                onInstancePointerDown(instance.id, index, event);
              }
            }}
            onPointerMove={(event) => {
              if (onPaintAt) {
                if ((event.buttons & 1) !== 0) paintFromHit(instance.id, meshData, event);
                return;
              }
              if (!pickEnabled) return;
              event.stopPropagation();
              if (targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null) {
                onComponentHover({
                  objectId: instance.id,
                  mode: "face",
                  id: meshData.faceIds[event.faceIndex]!,
                });
              } else {
                onInstancePointerMove(instance.id);
              }
            }}
            onPointerOut={() => {
              onInstancePointerMove(null);
              onComponentHover(null);
            }}
          ></PaintTexturedMesh>
          {borderGeometry && (showEdges ?? true) ? (
            <lineSegments geometry={borderGeometry} scale={1.001} raycast={() => null}>
              <lineBasicMaterial color={palette.neutral.lineColor} />
            </lineSegments>
          ) : null}
          {(targets.edge || (showEdges ?? true) || (selectionMode === "mesh" && selectedComponentIds.size > 0)) && edgeGeometry ? (
            <lineSegments
              geometry={edgeGeometry}
              onClick={(event) => {
                if (!pickEnabled || !meshData?.edgeIds?.length) return;
                event.stopPropagation();
                const edgeIndex = Math.floor((event.index ?? 0) / 2);
                const edgeId = meshData.edgeIds[edgeIndex];
                if (edgeId == null) return;
                onWorldPick({ granularity: "edge", id: edgeId, merge: mergeMode(event) });
              }}
              onPointerMove={(event) => {
                if (!pickEnabled || !meshData?.edgeIds?.length) return;
                event.stopPropagation();
                const edgeIndex = Math.floor((event.index ?? 0) / 2);
                const edgeId = meshData.edgeIds[edgeIndex];
                if (edgeId == null) return;
                onComponentHover({ objectId: instance.id, mode: "edge", id: edgeId });
              }}
              onPointerOut={() => onComponentHover(null)}
            >
              <lineBasicMaterial color={colors.edge} linewidth={1} />
            </lineSegments>
          ) : null}
          {targets.vertex && vertexPick ? (
            <points
              geometry={vertexPick.geometry}
              onClick={(event) => {
                if (!pickEnabled) return;
                event.stopPropagation();
                const idx = event.index ?? 0;
                const vertexId = vertexPick.vertexIds[idx];
                if (vertexId == null) return;
                onWorldPick({ granularity: "vertex", id: vertexId, merge: mergeMode(event) });
              }}
              onPointerMove={(event) => {
                if (!pickEnabled) return;
                event.stopPropagation();
                const idx = event.index ?? 0;
                const vertexId = vertexPick.vertexIds[idx];
                if (vertexId == null) return;
                onComponentHover({ objectId: instance.id, mode: "vertex", id: vertexId });
              }}
              onPointerOut={() => onComponentHover(null)}
            >
              <pointsMaterial color={colors.edge} size={0.05} sizeAttenuation />
            </points>
          ) : null}
          {faceSelectedOverlay ? (
            <mesh geometry={faceSelectedOverlay} raycast={() => null}>
              <meshBasicMaterial color={colors.select} transparent opacity={0.62} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-2} />
            </mesh>
          ) : null}
          {faceHoveredOverlay ? (
            <mesh geometry={faceHoveredOverlay} raycast={() => null}>
              <meshBasicMaterial color={colors.hover} transparent opacity={0.48} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-3} />
            </mesh>
          ) : null}
          {facePreviewOverlay ? (
            <mesh geometry={facePreviewOverlay} raycast={() => null}>
              <meshBasicMaterial color={colors.hover} transparent opacity={0.36} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-4} />
            </mesh>
          ) : null}
          {edgeSelectedOverlay ? (
            <lineSegments geometry={edgeSelectedOverlay} raycast={() => null}>
              <lineBasicMaterial color={colors.select} linewidth={3} />
            </lineSegments>
          ) : null}
          {edgeHoveredOverlay ? (
            <lineSegments geometry={edgeHoveredOverlay} raycast={() => null}>
              <lineBasicMaterial color={colors.hover} linewidth={3} />
            </lineSegments>
          ) : null}
          {edgePreviewOverlay ? (
            <lineSegments geometry={edgePreviewOverlay} raycast={() => null}>
              <lineBasicMaterial color={colors.hover} linewidth={2} />
            </lineSegments>
          ) : null}
          {vertexSelectedOverlay ? (
            <points geometry={vertexSelectedOverlay} raycast={() => null}>
              <pointsMaterial color={colors.select} size={0.09} sizeAttenuation depthTest={false} />
            </points>
          ) : null}
          {vertexHoveredOverlay ? (
            <points geometry={vertexHoveredOverlay} raycast={() => null}>
              <pointsMaterial color={colors.hover} size={0.09} sizeAttenuation depthTest={false} />
            </points>
          ) : null}
          {vertexPreviewOverlay ? (
            <points geometry={vertexPreviewOverlay} raycast={() => null}>
              <pointsMaterial color={colors.hover} size={0.09} sizeAttenuation depthTest={false} />
            </points>
          ) : null}
        </>
      ) : meshRecord?.url ? (
        <group
          onPointerDown={(event) => {
            event.stopPropagation();
            onInstancePointerDown(instance.id, index, event);
          }}
          onPointerMove={(event) => {
            event.stopPropagation();
            onInstancePointerMove(instance.id);
          }}
          onPointerOut={() => onInstancePointerMove(null)}
        >
          <Suspense fallback={null}>
            <GlbInstanceMesh
              url={meshRecord.url}
              color={glbColor}
              emissive={glbEmissive}
              emissiveIntensity={glbEmissiveIntensity}
              opacity={style.opacity}
              borderColor={palette.neutral.lineColor}
              material={environmentMaterial}
              shadowEnabled={environmentShadowEnabled}
            />
          </Suspense>
        </group>
      ) : (
        <mesh
          onPointerDown={(event) => {
            event.stopPropagation();
            onInstancePointerDown(instance.id, index, event);
          }}
        >
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial color={style.meshColor} metalness={0} roughness={1} emissive={style.meshColor} emissiveIntensity={style.emissiveIntensity} transparent={style.opacity < 1} opacity={style.opacity} />
        </mesh>
      )}
    </group>
  );
}
//#endregion WorldSceneParsing

//#region WorldInstancesLayer
function WorldInstancesLayer({
  instances,
  meshes,
  selection,
  palette,
  onInstancePointerDown,
  onInstancePointerMove,
  onWorldPick,
  onComponentHover,
  onPaintAt,
  gumballDragActive,
  onGumballDraggingChanged,
  onGumballDragEnd,
  onFaceDragStart,
  mergedComponentIds,
  mergedInstanceIds,
  blockPick,
  environment,
}: {
  readonly instances: readonly WorldInstanceRecord[];
  readonly meshes: readonly WorldMeshRecord[];
  readonly selection: WorldSelectionRecord;
  readonly palette: MeshStylePalette;
  readonly onInstancePointerDown: (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => void;
  readonly onInstancePointerMove: (id: string | null) => void;
  readonly onWorldPick: (args: { granularity: string; id: number; merge: string }) => void;
  readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
  readonly onPaintAt?: (objectId: string, u: number, v: number) => void;
  readonly gumballDragActive: boolean;
  readonly onGumballDraggingChanged: (dragging: boolean) => void;
  readonly onGumballDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
  readonly onFaceDragStart?: (args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => void;
  /** Live drag-preview merged component id set (null when no marquee drag is in progress). */
  readonly mergedComponentIds?: readonly number[] | null;
  /** Live drag-preview merged whole-instance id set (null when no marquee drag is in progress). */
  readonly mergedInstanceIds?: readonly string[] | null;
  /** Disables instance picking; passed for fill and brush engagements so a click meant for a vortex marker can't fall through and select/gumball the underlying object instead. */
  readonly blockPick?: boolean;
  readonly environment?: WorldEnvironmentRecord | null;
}) {
  const meshById = useMemo(() => new Map(meshes.map((mesh) => [mesh.id, mesh])), [meshes]);
  const geometries = useMemo(() => {
    const map = new Map<string, BufferGeometry>();
    for (const mesh of meshes) {
      if (mesh.data) map.set(mesh.id, geometryFromMesh(mesh.data));
    }
    return map;
  }, [meshes]);
  /** 🎨 Per-meshId border outline geometry, shared by every instance of that mesh — never rebuilt per instance. */
  const borderGeometries = useMemo(() => {
    const map = new Map<string, EdgesGeometry>();
    for (const [meshId, geometry] of geometries) map.set(meshId, new EdgesGeometry(geometry));
    return map;
  }, [geometries]);
  const targets = selection.targets ?? { mesh: true, vertex: false, edge: false, face: false };
  const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
  const currentComponentIds = new Set(selection.componentIds ?? []);
  const mergedComponentIdsSet = mergedComponentIds ? new Set(mergedComponentIds) : null;
  // Still-selected (solid) = current ∩ merged when dragging; newly-added (preview tint) = merged − current.
  const selectedComponentIds = mergedComponentIdsSet ? new Set([...currentComponentIds].filter((id) => mergedComponentIdsSet.has(id))) : currentComponentIds;
  const previewComponentIds = mergedComponentIdsSet ? new Set([...mergedComponentIdsSet].filter((id) => !currentComponentIds.has(id))) : new Set<number>();
  const mergedInstanceIdsSet = mergedInstanceIds ? new Set(mergedInstanceIds) : null;
  const pickEnabled = !gumballDragActive && !onPaintAt && !blockPick && !mergedComponentIdsSet && !mergedInstanceIdsSet;
  const transformTool = selection.transformTool ?? "move";
  const gumballConfig = useMemo(() => gumballConfigForTransformTool(transformTool), [transformTool]);
  const paintMode = selection.interactionMode === "paint";

  const mergeMode = (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => componentMergeArg(marqueeModeFromModifiers(event));

  const paintFromHit = (objectId: string, mesh: WorldMeshData, event: ThreeEvent<PointerEvent> & { faceIndex?: number | null; uv?: { x: number; y: number } }) => {
    if (!onPaintAt) return;
    let u = event.uv?.x;
    let v = event.uv?.y;
    if (u == null || v == null) {
      if (event.faceIndex == null || !mesh.indices.length) return;
      const i0 = mesh.indices[event.faceIndex * 3] ?? 0;
      const i1 = mesh.indices[event.faceIndex * 3 + 1] ?? 0;
      const i2 = mesh.indices[event.faceIndex * 3 + 2] ?? 0;
      if (!mesh.uvs || mesh.uvs.length < 6) return;
      u = (mesh.uvs[i0 * 2]! + mesh.uvs[i1 * 2]! + mesh.uvs[i2 * 2]!) / 3;
      v = (mesh.uvs[i0 * 2 + 1]! + mesh.uvs[i1 * 2 + 1]! + mesh.uvs[i2 * 2 + 1]!) / 3;
    }
    onPaintAt(objectId, u, v);
  };

  return (
    <WorldLayerStack>
      <group>
        {instances.map((instance, index) => {
          const meshId = instance.meshId ?? instance.id;
          const meshRecord = meshById.get(meshId);
          const meshData = meshRecord?.data;
          const geometry = geometries.get(meshId);
          const position = instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0];
          const scale = instance.scale ?? [1, 1, 1];
          const rotation = instance.rotation;
          const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
          const previewInstanceSelected = mergedInstanceIdsSet ? mergedInstanceIdsSet.has(instance.id) : undefined;
          return (
            <WorldInstanceNode
              key={instance.id}
              instance={instance}
              previewInstanceSelected={previewInstanceSelected}
              index={index}
              meshRecord={meshRecord}
              meshData={meshData}
              geometry={geometry}
              borderGeometry={borderGeometries.get(meshId)}
              palette={palette}
              vertexPick={meshData ? buildVertexPickData(meshData) : null}
              edgeGeometry={meshData ? buildEdgeGeometry(meshData) : null}
              paintTextureBase64={meshData?.paintTextureBase64}
              position={position as [number, number, number]}
              scale={scale as [number, number, number]}
              quaternion={quaternion}
              targets={targets}
              activeObjectId={selection.activeObjectId}
              selectionMode={selectionMode}
              selectedComponentIds={selectedComponentIds}
              previewComponentIds={previewComponentIds}
              hoveredComponent={selection.hoveredComponent}
              showEdges={selection.showEdges}
              pickEnabled={pickEnabled}
              onPaintAt={onPaintAt}
              paintFromHit={paintFromHit}
              flatShading={instance.smoothShading === false}
              onInstancePointerDown={onInstancePointerDown}
              onInstancePointerMove={onInstancePointerMove}
              onWorldPick={onWorldPick}
              onComponentHover={onComponentHover}
              mergeMode={mergeMode}
              faceDragActive={selection.faceDragActive === true}
              onFaceDragStart={onFaceDragStart}
              environmentMaterial={environment?.material}
              environmentShadowEnabled={environment?.shadow?.enabled === true}
            />
          );
        })}
      </group>
      <SceneGumball target={selection.gumballTarget} config={gumballConfig} active={Boolean(selection.gumballActive) && !paintMode} onDraggingChanged={onGumballDraggingChanged} onDragEnd={onGumballDragEnd} />
    </WorldLayerStack>
  );
}
//#endregion WorldInstancesLayer

function WorldVortexMarkers({
  vortices,
  palette,
  brushMode,
  selectionMode,
  connectSourceFullId,
  onHover,
  onVortexSelect,
  onBrushPlace,
  onConnectDragStart,
  onConnectDragHover,
  onConnectDragDrop,
}: {
  readonly vortices: readonly WorldVortexRecord[];
  readonly palette: MeshStylePalette;
  readonly brushMode: boolean;
  readonly selectionMode?: string;
  readonly connectSourceFullId?: string;
  readonly onHover: (fullId: string | null) => void;
  readonly onVortexSelect: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
  readonly onBrushPlace: () => void;
  readonly onConnectDragStart: (fullId: string, position: readonly [number, number, number]) => void;
  readonly onConnectDragHover: (position: readonly [number, number, number]) => void;
  readonly onConnectDragDrop: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
}) {
  if (!vortices.length) return null;
  return (
    <group>
      {vortices.map((vortex) => {
        const radius = vortex.radius ?? 0.36;
        const isConnectSource = connectSourceFullId === vortex.fullId;
        const style = vortex.selected ? palette.selected : vortex.hovered ? palette.hovered : null;
        const color = isConnectSource ? "#f59e0b" : (style?.meshColor ?? vortex.color ?? "#38bdf8");
        return (
          <mesh
            key={vortex.fullId}
            position={vortex.position as [number, number, number]}
            onPointerOver={(event) => {
              event.stopPropagation();
              onHover(vortex.fullId);
              if (connectSourceFullId) onConnectDragHover(vortex.position);
            }}
            onPointerOut={(event) => {
              event.stopPropagation();
              onHover(null);
            }}
            onPointerDown={(event) => {
              event.stopPropagation();
              if (resolveVortexPointerDownIntent(brushMode, selectionMode) === "select") {
                onVortexSelect(vortex.fullId, event);
                return;
              }
              onConnectDragStart(vortex.fullId, vortex.position);
            }}
            onPointerUp={(event) => {
              if (brushMode || !connectSourceFullId) return;
              event.stopPropagation();
              onConnectDragDrop(vortex.fullId, event);
            }}
            onClick={(event) => {
              event.stopPropagation();
              if (brushMode) onBrushPlace();
            }}
          >
            <sphereGeometry args={[radius, 16, 16]} />
            <meshStandardMaterial color={color} emissive={style?.meshColor ?? "#000000"} emissiveIntensity={style?.emissiveIntensity ?? 0} transparent opacity={0.88} />
          </mesh>
        );
      })}
    </group>
  );
}

/** @emoji 🧲 Rubber-band line drawn from the drag-connect source vortex to the currently hovered vortex (or itself, if hovering nothing). */
function WorldConnectRubberBand({ from, to }: { readonly from: readonly [number, number, number]; readonly to: readonly [number, number, number] }) {
  const geometry = useMemo(() => {
    const positions = new Float32Array([from[0], from[1], from[2], to[0], to[1], to[2]]);
    const geom = new BufferGeometry();
    geom.setAttribute("position", new BufferAttribute(positions, 3));
    return geom;
  }, [from, to]);
  return (
    <lineSegments geometry={geometry} raycast={() => null}>
      <lineBasicMaterial color="#f59e0b" linewidth={2} />
    </lineSegments>
  );
}

/** @emoji 🧊 Invisible ground plane (Z-up XY plane, matching this world's up axis) that tracks the grid-snapped cursor while voxel-editing target volumes; Alt+click commits a volume there. */
function WorldVoxelGroundPlane({ gridFactor, onHover, onPlace }: { readonly gridFactor: number; readonly onHover: (origin: readonly [number, number, number] | null) => void; readonly onPlace: (origin: readonly [number, number, number]) => void }) {
  const snap = (value: number) => Math.round(value / gridFactor) * gridFactor;
  return (
    <mesh
      onPointerMove={(event) => {
        event.stopPropagation();
        onHover([snap(event.point.x), snap(event.point.y), snap(event.point.z)]);
      }}
      onPointerOut={(event) => {
        event.stopPropagation();
        onHover(null);
      }}
      onClick={(event) => {
        event.stopPropagation();
        if (!event.nativeEvent.altKey) return;
        onPlace([snap(event.point.x), snap(event.point.y), snap(event.point.z)]);
      }}
    >
      <planeGeometry args={[10000, 10000]} />
      <meshBasicMaterial visible={false} />
    </mesh>
  );
}

/** @emoji 🧊 Cursor-follow ghost box previewing the target volume that Alt+click would place, sized by the engagement's W/D/H steppers. */
function WorldVoxelPreviewBox({ origin, dims, gridFactor }: { readonly origin: readonly [number, number, number]; readonly dims: readonly [number, number, number]; readonly gridFactor: number }) {
  return (
    <mesh position={origin as [number, number, number]} raycast={() => null}>
      <boxGeometry args={[dims[0] * gridFactor, dims[1] * gridFactor, dims[2] * gridFactor]} />
      <meshStandardMaterial color="#38bdf8" transparent opacity={0.48} />
    </mesh>
  );
}

function WorldAttractionLines({ attractions }: { readonly attractions: readonly WorldAttractionRecord[] }) {
  if (!attractions.length) return null;
  return (
    <group>
      {attractions.map((attraction) => {
        const positions = new Float32Array([attraction.from[0], attraction.from[1], attraction.from[2], attraction.to[0], attraction.to[1], attraction.to[2]]);
        const geometry = new BufferGeometry();
        geometry.setAttribute("position", new BufferAttribute(positions, 3));
        return (
          <lineSegments key={attraction.id} geometry={geometry} raycast={() => null}>
            <lineBasicMaterial color={attraction.color ?? "#60a5fa"} linewidth={2} />
          </lineSegments>
        );
      })}
    </group>
  );
}

function BrushPreviewGhost({ preview, meshes, palette }: { readonly preview: WorldBrushPreviewRecord; readonly meshes: readonly WorldMeshRecord[]; readonly palette: MeshStylePalette }) {
  if (!preview.origin) return null;
  const style = palette.highlighted;
  const meshUrl = preview.meshUrl;
  const meshRecord = meshUrl ? meshes.find((mesh) => mesh.url === meshUrl) : undefined;
  const position = preview.origin as [number, number, number];
  const rotation = preview.orientation as [number, number, number, number] | undefined;
  const scale = scaleTuple(preview.scale);
  const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
  return (
    <group position={position} scale={scale} quaternion={quaternion}>
      {meshRecord?.url ? (
        <Suspense fallback={null}>
          <GlbInstanceMesh url={meshRecord.url} color={style.meshColor} emissive={style.meshColor} emissiveIntensity={0.6} opacity={1} borderColor={palette.neutral.lineColor} />
        </Suspense>
      ) : (
        <mesh raycast={() => null}>
          <boxGeometry args={[1, 1, 1]} />
          <meshBasicMaterial color={style.meshColor} transparent opacity={0.42} depthWrite={false} />
        </mesh>
      )}
    </group>
  );
}

function EngagementPreviewLayer({ items, color }: { readonly items: readonly WorldEngagementPreviewItem[]; readonly color: string }) {
  if (!items.length) return null;
  return (
    <group>
      {items.map((item, index) => {
        if (item.kind === "point") {
          return (
            <mesh key={`preview-point-${index}`} position={item.position as [number, number, number]} raycast={() => null}>
              <sphereGeometry args={[0.08, 12, 12]} />
              <meshStandardMaterial color={color} />
            </mesh>
          );
        }
        if (item.kind === "segment") {
          const positions = new Float32Array([item.from[0], item.from[1], item.from[2], item.to[0], item.to[1], item.to[2]]);
          const geometry = new BufferGeometry();
          geometry.setAttribute("position", new BufferAttribute(positions, 3));
          return (
            <lineSegments key={`preview-segment-${index}`} geometry={geometry} raycast={() => null}>
              <lineBasicMaterial color={color} linewidth={2} />
            </lineSegments>
          );
        }
        if (item.kind === "box-preview" && item.cornerA && item.cornerB) {
          const [ax, ay, az] = item.cornerA;
          const [bx, by] = item.cornerB;
          const width = Math.max(Math.abs(bx - ax), 0.05);
          const depth = Math.max(Math.abs(by - ay), 0.05);
          // `height` is a separate vertical extrusion from the footprint plane (az), not derived
          // from cornerB's z — the interaction specs author cornerA/cornerB as ground-plane points.
          const height = Math.max(Math.abs(item.height ?? 0.05), 0.05);
          return (
            <mesh key={`preview-box-${index}`} position={[(ax + bx) * 0.5, (ay + by) * 0.5, az + height * 0.5]} raycast={() => null}>
              <boxGeometry args={[width, depth, height]} />
              <meshBasicMaterial color={color} transparent opacity={0.35} depthWrite={false} wireframe />
            </mesh>
          );
        }
        if (item.kind === "linear-handle") {
          const [ox, oy, oz] = item.origin;
          const [dx, dy, dz] = item.axis;
          const length = Math.max(Math.hypot(dx, dy, dz), 0.05);
          const direction = new Vector3(dx, dy, dz).normalize();
          const quaternion = new Quaternion().setFromUnitVectors(new Vector3(0, 1, 0), direction);
          return (
            <mesh key={`preview-handle-${index}`} position={[ox + dx * 0.5, oy + dy * 0.5, oz + dz * 0.5]} quaternion={quaternion} raycast={() => null}>
              <cylinderGeometry args={[0.02, 0.02, length, 8]} />
              <meshBasicMaterial color={color} transparent opacity={0.6} depthWrite={false} />
            </mesh>
          );
        }
        return null;
      })}
    </group>
  );
}

/** @emoji 🧭 Floating per-vortex brush-candidate popup opened by Alt+right-click or the context menu's "Suggest objects" — hovering a row previews it as the brush ghost, clicking places it. */
function WorldSuggestionMenu({
  menu,
  activeIndex,
  onHoverCandidate,
  onAcceptCandidate,
  onClose,
}: {
  readonly menu: WorldSuggestionMenuRecord;
  readonly activeIndex: number;
  readonly onHoverCandidate: (index: number) => void;
  readonly onAcceptCandidate: (index: number) => void;
  readonly onClose: () => void;
}) {
  const rootRef = useRef<HTMLDivElement | null>(null);
  const checkingPlacementLabel = useLabel("ui.host.checkingPlacement");
  const noPlacementLabel = useLabel("ui.host.noPlacement");
  useEffect(() => {
    const handlePointerDown = (event: PointerEvent) => {
      if (rootRef.current && !rootRef.current.contains(event.target as Node)) onClose();
    };
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") onClose();
    };
    window.addEventListener("pointerdown", handlePointerDown, true);
    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("pointerdown", handlePointerDown, true);
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [onClose]);
  return (
    <div
      ref={rootRef}
      className="semio-world-suggestion-menu"
      style={{
        position: "absolute",
        left: menu.x,
        top: menu.y,
        zIndex: 50,
        minWidth: "12rem",
        borderRadius: "0.375rem",
        border: "1px solid var(--border-normal-color)",
        background: "var(--panel)",
        padding: "0.25rem 0",
        boxShadow: "0 4px 16px rgba(0, 0, 0, 0.24)",
      }}
    >
      {menu.pending ? (
        <div style={{ padding: "0.375rem 0.75rem", fontSize: "0.8125rem", opacity: 0.7 }}>{checkingPlacementLabel}</div>
      ) : menu.candidates.length === 0 ? (
        <div style={{ padding: "0.375rem 0.75rem", fontSize: "0.8125rem", opacity: 0.7 }}>{noPlacementLabel}</div>
      ) : (
        menu.candidates.map((candidate) => (
          <div
            key={candidate.index}
            className={menuListItemClassName}
            data-selected={candidate.index === activeIndex}
            style={{ padding: "0.375rem 0.75rem", fontSize: "0.8125rem", cursor: "pointer" }}
            onMouseEnter={() => onHoverCandidate(candidate.index)}
            onClick={() => onAcceptCandidate(candidate.index)}
          >
            {candidate.objectLabel} · {candidate.vortexLabel}
          </div>
        ))
      )}
    </div>
  );
}

const MARQUEE_DRAG_THRESHOLD_PX = 4;

/** @emoji 🎯 Generic add/remove/toggle/replace merge, mirrors `selectionMergeIds` from `@semio-tech/ui-react` for non-string id sets. */
function mergeIdSet<T>(mode: ReturnType<typeof marqueeModeFromModifiers>, current: readonly T[], incoming: readonly T[]): T[] {
  const currentSet = new Set(current);
  const incomingSet = new Set(incoming);
  if (mode === "default") return [...incomingSet];
  if (mode === "additive") {
    for (const id of incomingSet) currentSet.add(id);
    return [...currentSet];
  }
  if (mode === "subtractive") {
    for (const id of incomingSet) currentSet.delete(id);
    return [...currentSet];
  }
  for (const id of incomingSet) {
    if (currentSet.has(id)) currentSet.delete(id);
    else currentSet.add(id);
  }
  return [...currentSet];
}

/** @emoji 🖱️ additive→add, subtractive→remove, invertive→toggle, default→replace (whole-instance picks/marquee). */
function instanceMergeArg(mode: ReturnType<typeof marqueeModeFromModifiers>): string {
  if (mode === "additive") return "add";
  if (mode === "subtractive") return "remove";
  if (mode === "invertive") return "toggle";
  return "replace";
}

/** @emoji 🖱️ Same as {@link instanceMergeArg} but a bare click (no modifiers) defaults to invertive. */
function componentMergeArg(mode: ReturnType<typeof marqueeModeFromModifiers>): string {
  if (mode === "additive") return "add";
  if (mode === "subtractive") return "remove";
  return "toggle";
}

function pointInMarqueeRect(sx: number, sy: number, marquee: readonly SelectionMarqueePoint[]): boolean {
  if (marquee.length < 2) return false;
  const start = marquee[0]!;
  const end = marquee[marquee.length - 1]!;
  const minX = Math.min(start.x, end.x);
  const maxX = Math.max(start.x, end.x);
  const minY = Math.min(start.y, end.y);
  const maxY = Math.max(start.y, end.y);
  return sx >= minX && sx <= maxX && sy >= minY && sy <= maxY;
}

/** @emoji 🎯 Even-odd point-in-polygon test for lasso selection. */
function pointInPolygon(sx: number, sy: number, polygon: readonly SelectionMarqueePoint[]): boolean {
  let inside = false;
  for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
    const a = polygon[i]!;
    const b = polygon[j]!;
    const intersects = a.y > sy !== b.y > sy && sx < ((b.x - a.x) * (sy - a.y)) / (b.y - a.y) + a.x;
    if (intersects) inside = !inside;
  }
  return inside;
}

function pointInMarqueeRegion(sx: number, sy: number, method: SelectionMarqueeMethod, marquee: readonly SelectionMarqueePoint[]): boolean {
  if (method === "lasso" && marquee.length >= 3) return pointInPolygon(sx, sy, marquee);
  return pointInMarqueeRect(sx, sy, marquee);
}

/** @emoji 🎯 Window (full containment, all points) vs crossing (partial, any point) semantics for multi-point elements. */
function pointsSatisfyMarquee(points: readonly (readonly [number, number])[], method: SelectionMarqueeMethod, marquee: readonly SelectionMarqueePoint[], coverage: SelectionMarqueeCoverage): boolean {
  if (points.length === 0) return false;
  const test = (point: readonly [number, number]) => pointInMarqueeRegion(point[0], point[1], method, marquee);
  return coverage === "full" ? points.every(test) : points.some(test);
}

function projectWorldPoint(point: readonly [number, number, number], offset: readonly [number, number, number], camera: import("three").Camera, rect: DOMRect): { readonly x: number; readonly y: number } {
  const projected = new Vector3(point[0] + offset[0], point[1] + offset[1], point[2] + offset[2]).project(camera);
  return {
    x: ((projected.x + 1) / 2) * rect.width,
    y: ((-projected.y + 1) / 2) * rect.height,
  };
}

function resolveMarqueeComponentIds(
  instances: readonly WorldInstanceRecord[],
  meshes: readonly WorldMeshRecord[],
  selectionMode: string,
  activeObjectId: string | undefined,
  marquee: readonly SelectionMarqueePoint[],
  rect: DOMRect,
  camera: import("three").Camera,
  method: SelectionMarqueeMethod,
  coverage: SelectionMarqueeCoverage,
): readonly number[] {
  const active = instances.find((instance) => instance.id === activeObjectId);
  if (!active) return [];
  const meshId = active.meshId ?? active.id;
  const meshData = meshes.find((mesh) => mesh.id === meshId)?.data;
  if (!meshData) return [];
  const offset = (active.position ?? [0, 0, 0]) as [number, number, number];
  const project = (point: readonly [number, number, number]): readonly [number, number] => {
    const screen = projectWorldPoint(point, offset, camera, rect);
    return [screen.x, screen.y];
  };
  const hits = new Set<number>();
  if (selectionMode === "vertex") {
    const pick = buildVertexPickData(meshData);
    if (!pick) return [];
    const positions = pick.geometry.attributes.position!;
    for (let index = 0; index < pick.vertexIds.length; index += 1) {
      const point = project([positions.getX(index), positions.getY(index), positions.getZ(index)]);
      if (pointsSatisfyMarquee([point], method, marquee, coverage)) hits.add(pick.vertexIds[index]!);
    }
  } else if (selectionMode === "edge" && meshData.edgeIds && meshData.edgePositions) {
    for (let edgeIndex = 0; edgeIndex < meshData.edgeIds.length; edgeIndex += 1) {
      const base = edgeIndex * 6;
      const a = project([meshData.edgePositions[base]!, meshData.edgePositions[base + 1]!, meshData.edgePositions[base + 2]!]);
      const b = project([meshData.edgePositions[base + 3]!, meshData.edgePositions[base + 4]!, meshData.edgePositions[base + 5]!]);
      if (pointsSatisfyMarquee([a, b], method, marquee, coverage)) hits.add(meshData.edgeIds[edgeIndex]!);
    }
  } else if (selectionMode === "face" && meshData.faceIds && meshData.indices.length) {
    for (let faceIndex = 0; faceIndex < meshData.faceIds.length; faceIndex += 1) {
      const i0 = meshData.indices[faceIndex * 3] ?? 0;
      const i1 = meshData.indices[faceIndex * 3 + 1] ?? 0;
      const i2 = meshData.indices[faceIndex * 3 + 2] ?? 0;
      const p0 = project([meshData.positions[i0 * 3]!, meshData.positions[i0 * 3 + 1]!, meshData.positions[i0 * 3 + 2]!]);
      const p1 = project([meshData.positions[i1 * 3]!, meshData.positions[i1 * 3 + 1]!, meshData.positions[i1 * 3 + 2]!]);
      const p2 = project([meshData.positions[i2 * 3]!, meshData.positions[i2 * 3 + 1]!, meshData.positions[i2 * 3 + 2]!]);
      if (pointsSatisfyMarquee([p0, p1, p2], method, marquee, coverage)) hits.add(meshData.faceIds[faceIndex]!);
    }
  }
  return [...hits];
}

/** @emoji 📦 Local-space AABB corners of a mesh's vertex positions (fallback: origin only). */
function meshBoundsCorners(meshData: WorldMeshData): readonly (readonly [number, number, number])[] {
  let minX = Infinity;
  let minY = Infinity;
  let minZ = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  let maxZ = -Infinity;
  for (let index = 0; index < meshData.positions.length; index += 3) {
    const x = meshData.positions[index]!;
    const y = meshData.positions[index + 1]!;
    const z = meshData.positions[index + 2]!;
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
    if (y > maxY) maxY = y;
    if (z < minZ) minZ = z;
    if (z > maxZ) maxZ = z;
  }
  if (!Number.isFinite(minX)) return [[0, 0, 0]];
  return [
    [minX, minY, minZ],
    [maxX, minY, minZ],
    [minX, maxY, minZ],
    [maxX, maxY, minZ],
    [minX, minY, maxZ],
    [maxX, minY, maxZ],
    [minX, maxY, maxZ],
    [maxX, maxY, maxZ],
  ];
}

function resolveMarqueeInstanceIds(
  instances: readonly WorldInstanceRecord[],
  meshes: readonly WorldMeshRecord[],
  marquee: readonly SelectionMarqueePoint[],
  rect: DOMRect,
  camera: import("three").Camera,
  method: SelectionMarqueeMethod,
  coverage: SelectionMarqueeCoverage,
): readonly string[] {
  const meshById = new Map(meshes.map((mesh) => [mesh.id, mesh]));
  const hits: string[] = [];
  instances.forEach((instance, index) => {
    const meshId = instance.meshId ?? instance.id;
    const meshData = meshById.get(meshId)?.data;
    const position = (instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0]) as [number, number, number];
    const scale = (instance.scale ?? [1, 1, 1]) as [number, number, number];
    const rotation = instance.rotation;
    const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
    const localCorners = meshData ? meshBoundsCorners(meshData) : [[0, 0, 0] as const];
    const worldCorners = localCorners.map((corner) => {
      const v = new Vector3(corner[0] * scale[0], corner[1] * scale[1], corner[2] * scale[2]);
      if (quaternion) v.applyQuaternion(quaternion);
      v.add(new Vector3(position[0], position[1], position[2]));
      return [v.x, v.y, v.z] as const;
    });
    const points = worldCorners.map((corner) => {
      const screen = projectWorldPoint(corner, [0, 0, 0], camera, rect);
      return [screen.x, screen.y] as const;
    });
    if (pointsSatisfyMarquee(points, method, marquee, coverage)) hits.push(instance.id);
  });
  return hits;
}

function CameraRefBridge({ cameraRef }: { readonly cameraRef: React.MutableRefObject<import("three").Camera | null> }) {
  const camera = useThree((state) => state.camera);
  useEffect(() => {
    cameraRef.current = camera;
  }, [camera, cameraRef]);
  return null;
}

/** @emoji 🎯 Widens Line/Points raycast hit area so thin edge/vertex geometry is reliably pickable without stealing face clicks. */
function RaycasterPickTuning() {
  const raycaster = useThree((state) => state.raycaster);
  useEffect(() => {
    raycaster.params.Line = { threshold: 0.05 };
    raycaster.params.Points = { threshold: 0.05 };
  }, [raycaster]);
  return null;
}

function paneSuffixFromSurfaceId(surfaceId?: string): string | undefined {
  if (!surfaceId) return undefined;
  const slash = surfaceId.lastIndexOf("/");
  return slash >= 0 ? surfaceId.slice(slash + 1) : surfaceId;
}

function raycastGroundPoint(clientX: number, clientY: number, hostRect: DOMRect, camera: import("three").Camera): [number, number, number] | null {
  const ndcX = ((clientX - hostRect.left) / hostRect.width) * 2 - 1;
  const ndcY = -(((clientY - hostRect.top) / hostRect.height) * 2 - 1);
  const ray = new Vector3(ndcX, ndcY, 0.5).unproject(camera);
  const origin = camera.position.clone();
  const direction = ray.sub(origin).normalize();
  if (Math.abs(direction.z) < 1e-6) return null;
  const t = -origin.z / direction.z;
  if (t < 0) return null;
  const hit = origin.add(direction.multiplyScalar(t));
  return [hit.x, hit.y, hit.z];
}

/** @emoji 🖱️➡️ Signed distance along `axis` (unit vector) from `origin` to the point on that line closest to the
 * camera ray through the current pointer position — the standard closest-point-between-two-lines
 * construction, used so a face-normal drag tracks naturally instead of needing a ground/tangent-plane
 * intersection (which is undefined for motion parallel to the plane, i.e. exactly along the normal). */
function axisDragParam(clientX: number, clientY: number, hostRect: DOMRect, camera: import("three").Camera, origin: readonly [number, number, number], axis: readonly [number, number, number]): number | null {
  const ndcX = ((clientX - hostRect.left) / hostRect.width) * 2 - 1;
  const ndcY = -(((clientY - hostRect.top) / hostRect.height) * 2 - 1);
  const rayOrigin = camera.position.clone();
  const rayDirection = new Vector3(ndcX, ndcY, 0.5).unproject(camera).sub(rayOrigin).normalize();
  const axisOrigin = new Vector3(origin[0], origin[1], origin[2]);
  const axisDirection = new Vector3(axis[0], axis[1], axis[2]).normalize();
  const originDelta = rayOrigin.clone().sub(axisOrigin);
  const a = rayDirection.dot(rayDirection);
  const b = rayDirection.dot(axisDirection);
  const c = axisDirection.dot(axisDirection);
  const d = rayDirection.dot(originDelta);
  const e = axisDirection.dot(originDelta);
  const denominator = a * c - b * b;
  if (Math.abs(denominator) < 1e-9) return null;
  return (a * e - b * d) / denominator;
}

//#region World3dHost
export function World3dHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.world3d;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const meshStylePalette = useMeshStylePalette();
  const colors = useMemo(() => semanticColorsFromPalette(meshStylePalette), [meshStylePalette]);
  const parsedCamera = useMemo(() => parseCameraState(scene?.cameraJson ?? "{}"), [scene?.cameraJson]);
  const instances = useMemo(() => parseInstances(scene?.instancesJson ?? "[]"), [scene?.instancesJson]);
  const cameraState = useMemo(() => {
    if ((scene?.cameraJson ?? "").includes('"position"')) return parsedCamera;
    return instances.length > 0 ? autofitCameraFromInstances(instances) : parsedCamera;
  }, [instances, parsedCamera, scene?.cameraJson]);
  const meshes = useMemo(() => parseMeshes(scene?.meshesJson ?? "[]"), [scene?.meshesJson]);
  const selection = useMemo(() => parseSelection(scene?.selectionJson ?? "{}"), [scene?.selectionJson]);
  const vortices = useMemo(() => parseJsonArray<WorldVortexRecord>(scene?.vorticesJson), [scene?.vorticesJson]);
  const attractions = useMemo(() => parseJsonArray<WorldAttractionRecord>(scene?.attractionsJson), [scene?.attractionsJson]);
  const targetVolumes = useMemo(() => parseJsonArray<WorldTargetVolumeRecord>(scene?.targetVolumesJson), [scene?.targetVolumesJson]);
  const references = useMemo(() => parseJsonArray<WorldReferenceRecord>(scene?.referencesJson), [scene?.referencesJson]);
  const interaction = useMemo(() => parseInteraction(scene?.interactionJson), [scene?.interactionJson]);
  const lod = useMemo(() => parseLod(scene?.lodJson), [scene?.lodJson]);
  const engagementPreview = useMemo(() => parseEngagementPreview(scene?.engagementPreviewJson), [scene?.engagementPreviewJson]);
  const brushPreview = useMemo(() => parseBrushPreview(scene?.brushPreviewJson), [scene?.brushPreviewJson]);
  const contextMenuItems = useMemo(() => parseJsonArray<WorldContextMenuItem>(scene?.contextMenuJson), [scene?.contextMenuJson]);
  const environment = useMemo(() => parseEnvironment(scene?.environmentJson), [scene?.environmentJson]);
  const frame = useMemo(() => parseFrame(scene?.frameJson), [scene?.frameJson]);
  const fit = useMemo(() => parseFit(scene?.fitJson), [scene?.fitJson]);
  const activeUtility = interaction.activeUtility ?? "select";
  const fillMode = activeUtility === "fill";
  const brushMode = activeUtility === "brush";
  const hostRef = useRef<HTMLDivElement | null>(null);
  const instancesGroupRef = useRef<Group | null>(null);
  const lodRef = useRef(DEFAULT_MANUAL_LOD);
  const [marqueePath, setMarqueePath] = useState<readonly SelectionMarqueePoint[]>([]);
  const [marqueeModifiers, setMarqueeModifiers] = useState<{ readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }>({ shiftKey: false, ctrlKey: false, metaKey: false });
  const [gumballDragActive, setGumballDragActive] = useState(false);
  const [faceDragSession, setFaceDragSession] = useState<{
    readonly objectId: string;
    readonly faceId: number;
    readonly normal: readonly [number, number, number];
    readonly startPoint: readonly [number, number, number];
    readonly faceExtent?: readonly [number, number];
  } | null>(null);
  const [connectDragSource, setConnectDragSource] = useState<{ readonly fullId: string; readonly position: readonly [number, number, number] } | null>(null);
  const [connectDragHoverPosition, setConnectDragHoverPosition] = useState<readonly [number, number, number] | null>(null);
  const [voxelHoverOrigin, setVoxelHoverOrigin] = useState<readonly [number, number, number] | null>(null);
  const [paintStrokeActive, setPaintStrokeActive] = useState(false);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number } | null>(null);
  const cameraRef = useRef<import("three").Camera | null>(null);
  const wasMarqueeDragRef = useRef(false);
  const connectDropConsumedRef = useRef(false);
  const engagementPointerMoveInFlightRef = useRef(false);
  const engagementPointerMoveLastPointRef = useRef<readonly [number, number, number] | null>(null);
  const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
  const marqueeDown = marqueePath.length > 0;
  const method = selection.method ?? "rectangle";
  const marqueeStart = marqueePath[0];
  const marqueeEnd = marqueePath[marqueePath.length - 1];
  const marqueeDragActive = marqueeDown && marqueePath.length > 1 && marqueeStart != null && marqueeEnd != null && Math.hypot(marqueeEnd.x - marqueeStart.x, marqueeEnd.y - marqueeStart.y) > MARQUEE_DRAG_THRESHOLD_PX;
  const marqueeMergeMode = useMemo(() => marqueeModeFromModifiers(marqueeModifiers), [marqueeModifiers]);
  const marqueeCoverage: SelectionMarqueeCoverage = useMemo(() => {
    if (!marqueeDragActive || !marqueeStart || !marqueeEnd) return "full";
    return marqueeCoverageFromGesture({ method, startX: marqueeStart.x, endX: marqueeEnd.x, path: marqueePath });
  }, [marqueeDragActive, marqueeEnd, marqueePath, marqueeStart, method]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({
        controllerId: node.controllerId,
        action,
        args: { surfaceId: node.surfaceId, ...args },
      });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const referenceSelectedIds = useMemo(() => {
    if (!selection.referenceSelectedId) return new Set<string>();
    return new Set([selection.referenceSelectedId]);
  }, [selection.referenceSelectedId]);

  const referenceHoveredId = useMemo(() => {
    const hovered = selection.hoveredId;
    if (!hovered?.startsWith("reference:")) return null;
    return hovered.slice("reference:".length);
  }, [selection.hoveredId]);

  const handleReferenceSelect = useCallback(
    (id: string) => {
      dispatch("setReferenceSelection", {
        pane: paneSuffixFromSurfaceId(node.surfaceId),
        referenceId: id,
      });
    },
    [dispatch, node.surfaceId],
  );

  const handleReferenceHover = useCallback(
    (id: string | null) => {
      if (!id) {
        dispatch("referenceHover", {});
        return;
      }
      dispatch("referenceHover", { referenceId: id });
    },
    [dispatch],
  );

  const registeredBrushMeshesRef = useRef(new Set<string>());
  const handleRegisterBrushMesh = useCallback(
    (url: string, positions: number[], indices: number[]) => {
      if (registeredBrushMeshesRef.current.has(url)) return;
      registeredBrushMeshesRef.current.add(url);
      dispatch("registerBrushMesh", { url, positions, indices });
    },
    [dispatch],
  );

  const brushMeshUrls = useMemo(() => [...new Set(meshes.map((mesh) => mesh.url).filter((url): url is string => Boolean(url)))], [meshes]);

  const handleZoomToSelection = useCallback(() => {
    const selectedIds = new Set(selection.ids ?? []);
    if (selectedIds.size === 0) return;
    const selected = instances.filter((instance) => selectedIds.has(instance.id));
    if (selected.length === 0) return;
    let centerX = 0;
    let centerY = 0;
    let centerZ = 0;
    for (const instance of selected) {
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      centerX += position[0];
      centerY += position[1];
      centerZ += position[2];
    }
    const count = selected.length;
    centerX /= count;
    centerY /= count;
    centerZ /= count;
    let maxDistance = 1;
    for (const instance of selected) {
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const dx = position[0] - centerX;
      const dy = position[1] - centerY;
      const dz = position[2] - centerZ;
      maxDistance = Math.max(maxDistance, Math.hypot(dx, dy, dz));
    }
    const distance = maxDistance * 3 + 2;
    dispatch("setCamera", {
      camera: {
        position: [centerX + distance * 0.6, centerY - distance * 0.6, centerZ + distance * 0.5],
        target: [centerX, centerY, centerZ],
        fov: cameraState.fov,
      },
    });
  }, [cameraState.fov, dispatch, instances, selection.ids]);

  const handleContextMenuSelect = useCallback(
    (item: WorldContextMenuItem) => {
      if (item.action === "zoomToSelection") {
        handleZoomToSelection();
        return;
      }
      if (item.action === "openVortexSuggestions") {
        dispatch(item.action, { ...item.args, x: contextMenu?.x ?? 0, y: contextMenu?.y ?? 0 });
        return;
      }
      dispatch(item.action, item.args);
    },
    [contextMenu, dispatch, handleZoomToSelection],
  );

  const hoveredVortexFullIdRef = useRef<string | null>(null);
  useEffect(() => {
    hoveredVortexFullIdRef.current = interaction.hoveredVortexFullId ?? null;
  }, [interaction.hoveredVortexFullId]);

  const handleWorldOrbitRightPointerDown = useCallback(
    (event: PointerEvent) => {
      if (event.altKey && hoveredVortexFullIdRef.current) {
        dispatch("openVortexSuggestions", { fullId: hoveredVortexFullIdRef.current, x: event.clientX, y: event.clientY });
        return false;
      }
      return true;
    },
    [dispatch],
  );

  const handleSuggestionHover = useCallback((index: number) => dispatch("hoverSuggestion", { index }), [dispatch]);
  const handleSuggestionAccept = useCallback((index: number) => dispatch("acceptSuggestion", { index }), [dispatch]);
  const handleSuggestionClose = useCallback(() => dispatch("closeVortexSuggestions"), [dispatch]);

  useEffect(() => {
    if (interaction.suggestionMenu?.open && interaction.suggestionMenu.pending) {
      const timer = window.setInterval(() => dispatch("suggestionsTick"), 120);
      return () => window.clearInterval(timer);
    }
  }, [dispatch, interaction.suggestionMenu?.open, interaction.suggestionMenu?.pending]);

  useEffect(() => {
    if (activeUtility === "fill" && interaction.fillBuild && !interaction.fillBuild.done) {
      const timer = window.setInterval(() => dispatch("fillBuildTick"), 120);
      return () => window.clearInterval(timer);
    }
  }, [activeUtility, dispatch, interaction.fillBuild]);

  const selectionArgs = useCallback(
    () => ({
      mode: selection.selectionMode ?? selection.granularity ?? "mesh",
      ids: selection.componentIds ?? [],
    }),
    [selection.componentIds, selection.granularity, selection.selectionMode],
  );

  const handleInstancePointerDown = useCallback(
    (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => {
      const merge = instanceMergeArg(marqueeModeFromModifiers(event));
      if (selectionMode === "mesh" || selectionMode === "object") {
        dispatch("worldPick", { granularity: "mesh", id: index, merge });
        return;
      }
      dispatch("worldSelect", {
        ids: [id],
        merge,
      });
    },
    [dispatch, selectionMode],
  );

  const handleInstancePointerMove = useCallback(
    (id: string | null) => {
      if (id == null) {
        dispatch("setHover", {});
        return;
      }
      dispatch("setHover", { objectId: id, mode: "mesh", id: 0 });
    },
    [dispatch],
  );

  const handleComponentHover = useCallback(
    (args: { objectId: string; mode: string; id: number } | null) => {
      if (!args) {
        dispatch("setHover", {});
        return;
      }
      dispatch("setHover", args);
    },
    [dispatch],
  );

  const handleVortexHover = useCallback(
    (fullId: string | null) => {
      if (!fullId) {
        dispatch("worldVortexHover", {});
        return;
      }
      dispatch("worldVortexHover", { fullId });
    },
    [dispatch],
  );

  const handleVortexSelect = useCallback(
    (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      const merge = event ? instanceMergeArg(marqueeModeFromModifiers(event)) : (globalThis as any).__selectionMode || "default";
      dispatch("worldVortexSelect", { fullId, merge });
    },
    [dispatch],
  );

  const handleConnectDragStart = useCallback((fullId: string, position: readonly [number, number, number]) => {
    setConnectDragSource({ fullId, position });
    setConnectDragHoverPosition(position);
  }, []);

  const handleConnectDragHover = useCallback((position: readonly [number, number, number]) => {
    setConnectDragHoverPosition(position);
  }, []);

  const handleConnectDragDrop = useCallback(
    (targetFullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      connectDropConsumedRef.current = true;
      setConnectDragSource((source) => {
        if (source) {
          if (source.fullId === targetFullId) {
            handleVortexSelect(targetFullId, event);
          } else {
            dispatch("createAttraction", { attracting: source.fullId, attracted: targetFullId });
          }
        }
        return null;
      });
      setConnectDragHoverPosition(null);
    },
    [dispatch, handleVortexSelect],
  );

  const handleConnectDragCancel = useCallback(() => {
    setConnectDragSource(null);
    setConnectDragHoverPosition(null);
  }, []);

  const handleVoxelPlace = useCallback(
    (origin: readonly [number, number, number]) => {
      dispatch("addTargetVolume", { origin });
    },
    [dispatch],
  );

  const handleBrushPlace = useCallback(() => {
    const args = brushObjectPlacementArgs(brushPreview);
    if (!args) return;
    dispatch("addBrushObject", args);
  }, [brushPreview, dispatch]);

  const handleWorldPick = useCallback(
    (args: { granularity: string; id: number; merge: string }) => {
      dispatch("worldPick", args);
    },
    [dispatch],
  );

  const paintMode = selection.interactionMode === "paint";
  const handlePaintAt = useCallback(
    (objectId: string, u: number, v: number) => {
      dispatch("paintAt", { objectId, u, v });
    },
    [dispatch],
  );

  const handleCameraChange = useCallback(
    (state: WorldCameraState) => {
      dispatch("setCamera", {
        camera: {
          position: state.position,
          target: state.target,
          zoom: state.zoom,
          fov: cameraState.fov,
          ...(cameraState.explicitProjection ? { projection: state.projection ?? cameraState.projection } : {}),
          ...(cameraState.up ? { up: cameraState.up } : {}),
        },
      });
    },
    [cameraState.explicitProjection, cameraState.fov, cameraState.projection, cameraState.up, dispatch],
  );

  const handleProjectionChange = useCallback(
    (projection: OrbitCameraProjection) => {
      dispatch("setCamera", {
        camera: {
          position: cameraState.position,
          target: cameraState.target,
          zoom: cameraState.zoom,
          fov: cameraState.fov,
          projection,
          ...(cameraState.up ? { up: cameraState.up } : {}),
        },
      });
    },
    [cameraState, dispatch],
  );

  const marqueePreview = useMemo<{ readonly mergedComponentIds: readonly number[] | null; readonly mergedInstanceIds: readonly string[] | null }>(() => {
    if (!marqueeDragActive || !hostRef.current || !cameraRef.current) return { mergedComponentIds: null, mergedInstanceIds: null };
    const rect = hostRef.current.getBoundingClientRect();
    const camera = cameraRef.current;
    if (selectionMode === "mesh" || selectionMode === "object") {
      const hits = resolveMarqueeInstanceIds(instances, meshes, marqueePath, rect, camera, method, marqueeCoverage);
      return { mergedComponentIds: null, mergedInstanceIds: mergeIdSet(marqueeMergeMode, selection.ids ?? [], hits) };
    }
    const hits = resolveMarqueeComponentIds(instances, meshes, selectionMode, selection.activeObjectId, marqueePath, rect, camera, method, marqueeCoverage);
    return { mergedComponentIds: mergeIdSet(marqueeMergeMode, selection.componentIds ?? [], hits), mergedInstanceIds: null };
  }, [instances, marqueeCoverage, marqueeDragActive, marqueeMergeMode, marqueePath, meshes, method, selection.activeObjectId, selection.componentIds, selection.ids, selectionMode]);

  const handleGumballDragEnd = useCallback(
    (_kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
      const tool = selection.transformTool === "rotate" ? "rotate" : selection.transformTool === "scale" ? "scale" : "translate";
      const base = selectionArgs();
      if (tool === "translate") {
        dispatch("translateSelection", {
          ...base,
          dx: after.position[0] - before.position[0],
          dy: after.position[1] - before.position[1],
          dz: after.position[2] - before.position[2],
        });
        return;
      }
      if (tool === "rotate") {
        const beforeQuat = new Quaternion(...before.quaternion);
        const afterQuat = new Quaternion(...after.quaternion);
        const delta = afterQuat.multiply(beforeQuat.invert());
        // Quaternion.w = cos(angle/2); clamp for asin/acos precision at the identity boundary.
        const angle = 2 * Math.acos(Math.min(1, Math.max(-1, delta.w)));
        const sinHalfAngle = Math.sqrt(Math.max(0, 1 - delta.w * delta.w));
        const axis = sinHalfAngle < 1e-6 ? { x: 0, y: 0, z: 1 } : { x: delta.x / sinHalfAngle, y: delta.y / sinHalfAngle, z: delta.z / sinHalfAngle };
        dispatch("rotateSelection", {
          ...base,
          ax: axis.x,
          ay: axis.y,
          az: axis.z,
          angle,
        });
        return;
      }
      const sx = after.scale[0] / Math.max(before.scale[0], 1e-6);
      const sy = after.scale[1] / Math.max(before.scale[1], 1e-6);
      const sz = after.scale[2] / Math.max(before.scale[2], 1e-6);
      dispatch("scaleSelection", { ...base, sx, sy, sz });
    },
    [dispatch, selection.transformTool, selectionArgs],
  );

  const handleFaceDragStart = useCallback((args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => {
    setFaceDragSession({ objectId: args.objectId, faceId: args.faceId, normal: args.normal, startPoint: args.point, faceExtent: args.faceExtent });
  }, []);

  const toLocalPoint = useCallback((event: React.PointerEvent<HTMLDivElement>): SelectionMarqueePoint => {
    const rect = hostRef.current?.getBoundingClientRect();
    if (!rect) return { x: event.clientX, y: event.clientY };
    return { x: event.clientX - rect.left, y: event.clientY - rect.top };
  }, []);

  const handlePointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (event.button !== 0) return;
      if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
        const rect = hostRef.current.getBoundingClientRect();
        const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
        if (point) {
          dispatch("worldPointerDown", {
            pane: paneSuffixFromSurfaceId(node.surfaceId),
            position: point,
            shiftKey: event.shiftKey,
            ctrlKey: event.ctrlKey,
            metaKey: event.metaKey,
          });
          return;
        }
      }
      if (paintMode) {
        setPaintStrokeActive(true);
        dispatch("paintStrokeBegin");
      }
      setMarqueeModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
      setMarqueePath([toLocalPoint(event)]);
    },
    [dispatch, node.surfaceId, paintMode, selection.engagementSessionActive, toLocalPoint],
  );

  const handlePointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
        const rect = hostRef.current.getBoundingClientRect();
        const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
        const last = engagementPointerMoveLastPointRef.current;
        const unchanged = point && last && point[0] === last[0] && point[1] === last[1] && point[2] === last[2];
        if (point && !unchanged && !engagementPointerMoveInFlightRef.current) {
          engagementPointerMoveInFlightRef.current = true;
          engagementPointerMoveLastPointRef.current = point;
          requestAnimationFrame(() => {
            engagementPointerMoveInFlightRef.current = false;
            dispatch("worldPointerMove", { pane: paneSuffixFromSurfaceId(node.surfaceId), position: point });
          });
        }
        return;
      }
      if (!marqueeDown) return;
      setMarqueeModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
      setMarqueePath((path) => [...path, toLocalPoint(event)]);
    },
    [dispatch, marqueeDown, node.surfaceId, selection.engagementSessionActive, toLocalPoint],
  );

  const handlePointerUp = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (faceDragSession) {
        const session = faceDragSession;
        setFaceDragSession(null);
        if (hostRef.current && cameraRef.current) {
          const rect = hostRef.current.getBoundingClientRect();
          const distance = axisDragParam(event.clientX, event.clientY, rect, cameraRef.current, session.startPoint, session.normal);
          if (distance != null && Math.abs(distance) > 1e-4) {
            dispatch("worldFaceDragEnd", {
              pane: paneSuffixFromSurfaceId(node.surfaceId),
              objectId: session.objectId,
              faceId: session.faceId,
              normal: session.normal,
              startPoint: session.startPoint,
              distance,
              faceExtent: session.faceExtent,
            });
          }
        }
        return;
      }
      if (marqueeDragActive) {
        if (marqueePreview.mergedInstanceIds) {
          dispatch("worldSelect", { ids: marqueePreview.mergedInstanceIds, merge: "replace" });
        } else if (marqueePreview.mergedComponentIds) {
          dispatch("setSelection", { mode: selectionMode, ids: marqueePreview.mergedComponentIds });
        }
      }
      wasMarqueeDragRef.current = marqueeDragActive;
      if (paintStrokeActive) {
        dispatch("paintStrokeEnd");
        setPaintStrokeActive(false);
      }
      setMarqueePath([]);
      if (connectDropConsumedRef.current) {
        connectDropConsumedRef.current = false;
      } else {
        handleConnectDragCancel();
      }
    },
    [dispatch, faceDragSession, handleConnectDragCancel, marqueeDragActive, marqueePreview, node.surfaceId, paintStrokeActive, selectionMode],
  );

  const handleEmptyClick = useCallback(
    (event: MouseEvent) => {
      if (wasMarqueeDragRef.current) return;
      if (selection.engagementSessionActive || paintMode) return;
      dispatch("worldPick", { granularity: selectionMode, id: null, merge: instanceMergeArg(marqueeModeFromModifiers(event)) });
    },
    [dispatch, paintMode, selection.engagementSessionActive, selectionMode],
  );

  if (!scene) return <div className="semio-world-3d-empty">{emptySceneLabel}</div>;

  return (
    <div
      ref={hostRef}
      className="semio-world-3d-host relative h-full min-h-[24rem] w-full"
      data-surface-id={node.surfaceId}
      onContextMenu={(event) => {
        if (event.altKey) return;
        const target = resolveWorldContextMenuTarget(interaction, selection);
        if (!target) return;
        event.preventDefault();
        dispatch("contextMenuAt", { kind: target.kind, id: target.id });
        setContextMenu({ x: event.clientX, y: event.clientY });
      }}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
    >
      <WorldCanvas
        className="h-full w-full"
        cameraUp={(cameraState.up as [number, number, number] | undefined) ?? [0, 0, 1]}
        cameraFov={cameraState.fov}
        background={environment && !isTransparentWorldBackground(environment.background) ? environment.background : undefined}
        gl={environment && isTransparentWorldBackground(environment.background) ? { antialias: true, alpha: true } : undefined}
        shadows={environment?.shadow?.enabled === true ? true : undefined}
        onPointerMissed={handleEmptyClick}
        overlay={
          frame || cameraState.explicitProjection ? (
            <>
              {frame ? <IconShotFrame width={frame.width} height={frame.height} shape={frame.shape === "ellipse" ? "ellipse" : "rectangle"} badge={frame.badge !== false} background={frame.background} /> : null}
              {cameraState.explicitProjection ? <WorldOrbitProjectionSwitch projection={cameraState.projection ?? "perspective"} onProjectionChange={handleProjectionChange} /> : null}
            </>
          ) : undefined
        }
      >
        <WorldOrbitViewSnapGateProvider>
          <WorldOrbitCameraViewRig state={cameraState} seedKey={scene?.cameraJson ?? "default"} perspectiveFov={cameraState.fov} />
          <WorldOrbitGated
            controlsGate={marqueeDown || gumballDragActive || connectDragSource !== null || faceDragSession !== null}
            onCamera={handleCameraChange}
            zoom={cameraState.zoom}
            projection={cameraState.explicitProjection ? cameraState.projection : undefined}
            onRightPointerDown={handleWorldOrbitRightPointerDown}
          />
          <WorldLodBridge
            lodRef={lodRef}
            distanceReference={100}
            gridFactor={lod.gridFactor ?? DEFAULT_LOD_GRID_FACTOR}
            gridSnapEnabled={lod.gridSnapEnabled ?? false}
            showLodGrid={lod.showLodGrid ?? true}
            automaticLod={lod.automaticLod ?? true}
            depthVariableLod={lod.depthVariableLod ?? false}
            manualLod={lod.manualLod ?? DEFAULT_MANUAL_LOD}
            gridDatum={[0, 0, 0]}
          >
            <ambientLight color={environment?.ambient?.color ?? "#ffffff"} intensity={environment?.ambient?.intensity ?? 1.15} />
            {environment?.sun?.enabled === true ? (
              <directionalLight
                color={environment.sun?.color ?? "#ffffff"}
                intensity={environment.sun?.intensity ?? 0.85}
                position={sunPositionFromAzimuthElevation(environment.sun?.azimuth ?? 45, environment.sun?.elevation ?? 35)}
                castShadow={environment.shadow?.enabled === true}
              />
            ) : (
              <>
                <hemisphereLight color="#ffffff" groundColor="#9aa0ab" intensity={1.35} position={[0, 0, 1]} />
                <directionalLight position={[12, 18, 10]} intensity={2.4} />
                <directionalLight position={[-14, -10, 6]} intensity={1.2} />
                <directionalLight position={[0, 0, -16]} intensity={0.75} />
              </>
            )}
            {fit?.enabled ? <WorldAutoFit groupRef={instancesGroupRef} fitKey={`${fit.revision ?? 0}:${meshes.map((mesh) => mesh.url ?? mesh.id).join(",")}`} padding={fit.padding ?? 1.25} camera={cameraState} onFitted={handleCameraChange} /> : null}
            <CameraRefBridge cameraRef={cameraRef} />
            <RaycasterPickTuning />
            {brushMeshUrls.map((url) => (
              <Suspense key={url} fallback={null}>
                <BrushMeshRegistrar url={url} onRegister={handleRegisterBrushMesh} />
              </Suspense>
            ))}
            <WorldTerrainLayer terrainJson={scene?.terrainJson} cameraPosition={cameraState.position} cameraTarget={cameraState.target} />
            <group ref={instancesGroupRef}>
              <WorldInstancesLayer
                instances={instances}
                meshes={meshes}
                selection={selection}
                palette={meshStylePalette}
                onInstancePointerDown={handleInstancePointerDown}
                onInstancePointerMove={handleInstancePointerMove}
                onWorldPick={handleWorldPick}
                onComponentHover={handleComponentHover}
                onPaintAt={paintMode ? handlePaintAt : undefined}
                gumballDragActive={gumballDragActive}
                onGumballDraggingChanged={setGumballDragActive}
                onGumballDragEnd={handleGumballDragEnd}
                onFaceDragStart={handleFaceDragStart}
                mergedComponentIds={marqueePreview.mergedComponentIds}
                mergedInstanceIds={marqueePreview.mergedInstanceIds}
                blockPick={worldInstancePickBlocked(activeUtility)}
                environment={environment}
              />
            </group>
            <WorldVortexMarkers
              vortices={vortices}
              palette={meshStylePalette}
              brushMode={brushMode}
              selectionMode={selectionMode}
              connectSourceFullId={connectDragSource?.fullId}
              onHover={handleVortexHover}
              onVortexSelect={handleVortexSelect}
              onBrushPlace={handleBrushPlace}
              onConnectDragStart={handleConnectDragStart}
              onConnectDragHover={handleConnectDragHover}
              onConnectDragDrop={handleConnectDragDrop}
            />
            {connectDragSource && connectDragHoverPosition ? <WorldConnectRubberBand from={connectDragSource.position} to={connectDragHoverPosition} /> : null}
            <WorldAttractionLines attractions={attractions} />
            {brushPreview ? <BrushPreviewGhost preview={brushPreview} meshes={meshes} palette={meshStylePalette} /> : null}
            {engagementPreview.length > 0 ? <EngagementPreviewLayer items={engagementPreview} color={colors.hover} /> : null}
            <WorldVolumeLayer
              volumes={targetVolumes.map((volume) => ({
                id: volume.id,
                origin: volume.origin as [number, number, number],
                orientation: volume.orientation as [number, number, number, number] | undefined,
                scale: volume.scale,
                color: volume.color,
              }))}
              interactive={false}
            />
            {interaction.fillEditTargetVolumes ? <WorldVoxelGroundPlane gridFactor={interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR} onHover={setVoxelHoverOrigin} onPlace={handleVoxelPlace} /> : null}
            {interaction.fillEditTargetVolumes && voxelHoverOrigin ? <WorldVoxelPreviewBox origin={voxelHoverOrigin} dims={interaction.voxelDims ?? [1, 1, 1]} gridFactor={interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR} /> : null}
            <WorldReferenceLayer
              references={references
                .filter((reference) => !reference.hidden)
                .map((reference) => ({
                  id: reference.id,
                  source: { url: reference.url, mediaKind: "image" as const },
                  origin: reference.origin as [number, number, number],
                  widthWorld: reference.widthWorld,
                  locked: reference.locked,
                  opacity: reference.opacity,
                }))}
              selectedIds={referenceSelectedIds}
              hoveredId={referenceHoveredId}
              onSelect={(id) => handleReferenceSelect(id)}
              onHover={handleReferenceHover}
            />
          </WorldLodBridge>
        </WorldOrbitViewSnapGateProvider>
      </WorldCanvas>
      {marqueeDragActive && marqueeStart && marqueeEnd ? (
        method === "lasso" ? (
          <SelectionMarquee coverage={marqueeCoverage} shape="polygon" points={marqueePath} />
        ) : (
          <SelectionMarquee
            coverage={marqueeCoverage}
            shape="rect"
            rect={{
              x: Math.min(marqueeStart.x, marqueeEnd.x),
              y: Math.min(marqueeStart.y, marqueeEnd.y),
              width: Math.abs(marqueeEnd.x - marqueeStart.x),
              height: Math.abs(marqueeEnd.y - marqueeStart.y),
            }}
          />
        )
      ) : null}
      <ContextMenuController
        open={contextMenu != null && contextMenuItems.length > 0}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenuItems.map((item) => ({
          id: item.id,
          label: item.label,
          onSelect: () => handleContextMenuSelect(item),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
      {interaction.suggestionMenu?.open ? (
        <WorldSuggestionMenu menu={interaction.suggestionMenu} activeIndex={interaction.brushCandidateIndex ?? 0} onHoverCandidate={handleSuggestionHover} onAcceptCandidate={handleSuggestionAccept} onClose={handleSuggestionClose} />
      ) : null}
    </div>
  );
}
//#endregion World3dHost
