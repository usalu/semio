/** 🖼️ Display primitives, the preview items, the host display-kind registry and the `InteractionDisplay` mapper.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { reactHostPort, type ReactNode } from "@semio-tech/framework/ui/targets/react";
import { ephemeralMap } from "@semio-tech/framework";
import { type MeshTransfer, type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { type ModelEntityKind } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { type DisplayItem, type DisplayModel } from "../../🎬️actions/🟦️.ts";
import { Line, scenePreview, Text, THREE } from "../🔌️ports/🟦️.ts";
import { spatialSceneColors } from "../🎨️style/🟦️.ts";
import { TessellatedCommitMesh } from "../🧲️geometry-targets/🟦️.tsx";
import { computeBoxPreviewLayout, computeSpherePreviewLayout, parseDisplaySelectionTargets, previewKindUsesGeometryWireframe, raycastNone, readNumber, readVec3, readVec3Array, transformPointsForPreviewKind } from "../🧲️geometry-targets/📐️preview-layout/🟦️.ts";
import { type SpatialPickGeometry } from "../🧲️geometry-targets/🎯️pick-model/🟦️.ts";
import { geometryBuckets, geometryEntityWireSegments } from "../🧲️geometry-targets/🧬️geometry-index/🟦️.ts";

// #region 🖼️DisplayPrimitives
function BoxPreviewItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const palette = spatialSceneColors();
  const p = item.params;
  const edgeGeo = reactHostPort.useMemo(() => new THREE.EdgesGeometry(new THREE.BoxGeometry(1, 1, 1)), []);
  if (!p) return null;
  const a = readVec3(p.cornerA);
  const b = readVec3(p.cornerB);
  const hRaw = readNumber(p.height);
  if (!a || !b) return null;
  const h = hRaw === null || hRaw <= 0 ? 0.06 : hRaw;
  const { position, scale } = computeBoxPreviewLayout(a, b, h);
  const archived = item.role === "archived";
  return (
    <group position={position} scale={scale}>
      <mesh raycast={raycastNone}>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial
          color={archived ? palette.archived : palette.committed}
          emissive={archived ? palette.archivedEmissive : palette.committedEmissive}
          emissiveIntensity={archived ? 0.22 : 0.35}
          transparent
          opacity={archived ? 0.38 : 0.52}
          depthWrite={false}
        />
      </mesh>
      <lineSegments raycast={raycastNone} geometry={edgeGeo}>
        <lineBasicMaterial color={archived ? palette.archived : palette.committedWire} transparent opacity={archived ? 0.55 : 0.85} />
      </lineSegments>
    </group>
  );
}

function PointItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const palette = spatialSceneColors();
  const pos = readVec3(item.params?.position);
  if (!pos) return null;
  const cursor = item.role === "cursor";
  const r = cursor ? 0.045 : 0.06;
  return (
    <mesh position={pos} raycast={raycastNone}>
      <sphereGeometry args={[r, 16, 16]} />
      <meshStandardMaterial color={cursor ? palette.hovered : palette.construction} emissive={cursor ? palette.hoveredEmissive : palette.constructionEmissive} emissiveIntensity={cursor ? 0.45 : 0.35} />
    </mesh>
  );
}

function LinearHandleItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const p = item.params;
  if (!p) return null;
  const origin = readVec3(p.origin);
  const axis = readVec3(p.axis);
  if (!origin || !axis) return null;
  const ax = axis[0];
  const ay = axis[1];
  const az = axis[2];
  const len = Math.hypot(ax, ay, az) || 1;
  const ux = ax / len;
  const uy = ay / len;
  const uz = az / len;
  const span = 5;
  const x1 = origin[0] + ux * span;
  const y1 = origin[1] + uy * span;
  const z1 = origin[2] + uz * span;
  return (
    <Line
      raycast={raycastNone}
      points={[
        [origin[0], origin[1], origin[2]],
        [x1, y1, z1],
      ]}
      color={spatialSceneColors().dimension}
      lineWidth={2}
      dashed={false}
    />
  );
}

function SegmentItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const palette = spatialSceneColors();
  const p = item.params;
  if (!p) return null;
  const a = readVec3(p.from);
  const b = readVec3(p.to);
  if (!a || !b) return null;
  const guide = item.role === "guide";
  const heightLine = item.role === "height";
  return (
    <Line
      raycast={raycastNone}
      points={[
        [a[0], a[1], a[2]],
        [b[0], b[1], b[2]],
      ]}
      color={guide ? palette.guide : heightLine ? palette.hovered : palette.accent}
      lineWidth={guide ? 1 : heightLine ? 2.5 : 2}
      dashed={guide}
      {...(guide ? { dashSize: 0.12, gapSize: 0.08 } : {})}
    />
  );
}

function LabelItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const p = item.params;
  if (!p) return null;
  const pos = readVec3(p.position);
  const text = p.text;
  if (!pos || typeof text !== "string") return null;
  return (
    <reactHostPort.Suspense fallback={null}>
      <Text position={pos} fontSize={0.22} color={spatialSceneColors().foreground} anchorX="left" anchorY="bottom" raycast={raycastNone}>
        {text}
      </Text>
    </reactHostPort.Suspense>
  );
}

function GeometryTargetWireframes({
  geometry,
  targets,
  transform,
  color,
  opacity,
}: {
  readonly geometry: SpatialPickGeometry;
  readonly targets: readonly { readonly kind: ModelEntityKind; readonly id: string }[];
  readonly transform: (point: Vec3) => Vec3;
  readonly color: string;
  readonly opacity: number;
}): ReactNode {
  const buckets = reactHostPort.useMemo(() => geometryBuckets(geometry), [geometry]);
  const segments = reactHostPort.useMemo(() => {
    const out: (readonly [Vec3, Vec3])[] = [];
    for (const target of targets) {
      for (const [a, b] of geometryEntityWireSegments(buckets, target.kind, target.id)) {
        out.push([transform(a), transform(b)]);
      }
    }
    return out;
  }, [buckets, targets, transform]);
  if (!segments.length) return null;
  return (
    <group>
      {segments.map(([a, b], i) => (
        <Line
          key={`${a[0]}-${a[1]}-${a[2]}-${b[0]}-${b[1]}-${b[2]}-${i}`}
          raycast={raycastNone}
          points={[
            [a[0], a[1], a[2]],
            [b[0], b[1], b[2]],
          ]}
          color={color}
          lineWidth={2}
          transparent
          opacity={opacity}
        />
      ))}
    </group>
  );
}

export function PreviewItem({ item, geometry }: { readonly item: DisplayItem; readonly geometry?: SpatialPickGeometry | null }): ReactNode {
  const palette = spatialSceneColors();
  const p = item.params;
  const previewKind = typeof p?.previewKind === "string" ? p.previewKind : "preview";
  const transform = reactHostPort.useMemo(() => (p ? transformPointsForPreviewKind(previewKind, p) : (point: Vec3) => point), [previewKind, p]);
  if (!p) return null;
  const targets = parseDisplaySelectionTargets(p.targets);
  const points = readVec3Array(p.points);
  const cursor = readVec3(p.cursor);
  const prevPoint = readVec3(p.prevPoint);
  const from = readVec3(p.from) ?? prevPoint;
  const linePoints = points.length ? [...points, ...(cursor ? [cursor] : [])] : from && cursor ? [from, cursor] : [];
  const ghost = previewKind === "move-preview" || previewKind === "copy-preview" || previewKind === "mirror-preview";
  const wireColor = previewKind === "selected-objects" || previewKind.endsWith("-selection") ? palette.construction : ghost ? palette.committed : palette.accent;
  const wireOpacity = ghost ? 0.92 : 0.78;
  if (geometry && targets.length && previewKindUsesGeometryWireframe(previewKind)) {
    return (
      <group>
        {ghost ? <GeometryTargetWireframes geometry={geometry} targets={targets} transform={(pt) => pt} color={palette.ghost} opacity={0.35} /> : null}
        <GeometryTargetWireframes geometry={geometry} targets={targets} transform={transform} color={wireColor} opacity={wireOpacity} />
        {from ? (
          <mesh position={from} raycast={raycastNone}>
            <sphereGeometry args={[0.05, 12, 12]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.4} />
          </mesh>
        ) : null}
        {linePoints.length >= 2 ? <Line raycast={raycastNone} points={linePoints.map((pt) => [pt[0], pt[1], pt[2]])} color={palette.dimension} lineWidth={2} /> : null}
      </group>
    );
  }
  if (previewKind === "sphere" && points.length >= 1 && cursor) {
    const sphere = computeSpherePreviewLayout(points[0]!, cursor);
    if (sphere) {
      return (
        <group>
          <mesh position={sphere.position} raycast={raycastNone}>
            <sphereGeometry args={[sphere.radius, 32, 16]} />
            <meshStandardMaterial color={palette.committed} emissive={palette.committedEmissive} emissiveIntensity={0.28} transparent opacity={0.34} depthWrite={false} side={THREE.DoubleSide} />
          </mesh>
          <mesh position={sphere.position} raycast={raycastNone}>
            <sphereGeometry args={[sphere.radius, 32, 16]} />
            <meshBasicMaterial color={palette.committedWire} wireframe transparent opacity={0.55} depthWrite={false} />
          </mesh>
          <Line
            raycast={raycastNone}
            points={[
              [sphere.position[0], sphere.position[1], sphere.position[2]],
              [cursor[0], cursor[1], cursor[2]],
            ]}
            color={palette.dimension}
            lineWidth={1.5}
            dashed
            dashSize={0.08}
            gapSize={0.06}
          />
          <mesh position={sphere.position} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} />
          </mesh>
        </group>
      );
    }
  }
  // #region 🔵️CircleArcPreview
  if ((previewKind === "circle-outline" || previewKind === "circle") && points.length >= 1 && cursor) {
    const center = points[0]!;
    const radius = Math.hypot(cursor[0] - center[0], cursor[1] - center[1], cursor[2] - center[2]);
    if (radius > 1e-9) {
      const segments = 64;
      const circlePts: [number, number, number][] = [];
      for (let i = 0; i <= segments; i++) {
        const a = (i / segments) * Math.PI * 2;
        circlePts.push([center[0] + Math.cos(a) * radius, center[1] + Math.sin(a) * radius, center[2]]);
      }
      return (
        <group>
          <Line raycast={raycastNone} points={circlePts} color={palette.accent} lineWidth={2} />
          <Line
            raycast={raycastNone}
            points={[
              [center[0], center[1], center[2]],
              [cursor[0], cursor[1], cursor[2]],
            ]}
            color={palette.dimension}
            lineWidth={1.5}
            dashed
            dashSize={0.08}
            gapSize={0.06}
          />
          <mesh position={center} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} />
          </mesh>
        </group>
      );
    }
  }
  if (previewKind === "arc" && points.length >= 2 && cursor) {
    const center = points[0]!;
    const start = points[1]!;
    const arcEnd = scenePreview().arcEndOnCircle(center, start, cursor);
    const arcPts = scenePreview().arcSamplePoints(center, start, arcEnd, 64);
    if (arcPts.length >= 2) {
      return (
        <group>
          <Line raycast={raycastNone} points={arcPts.map((pt) => [pt[0], pt[1], pt[2]])} color={palette.accent} lineWidth={2} />
          <Line
            raycast={raycastNone}
            points={[
              [center[0], center[1], center[2]],
              [start[0], start[1], start[2]],
            ]}
            color={palette.dimension}
            lineWidth={1.5}
            dashed
            dashSize={0.08}
            gapSize={0.06}
          />
          <Line
            raycast={raycastNone}
            points={[
              [center[0], center[1], center[2]],
              [arcEnd[0], arcEnd[1], arcEnd[2]],
            ]}
            color={palette.dimension}
            lineWidth={1.5}
            dashed
            dashSize={0.08}
            gapSize={0.06}
          />
          <mesh position={center} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} />
          </mesh>
          <mesh position={start} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} />
          </mesh>
          <mesh position={arcEnd} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.accent} emissive={palette.accentEmissive} emissiveIntensity={0.35} />
          </mesh>
        </group>
      );
    }
  }
  // #endregion 🔵️CircleArcPreview
  if (previewKind === "interpolated-curve" && linePoints.length >= 2) {
    const sampled = scenePreview().nurbsDisplaySamplePoints(linePoints, Math.max(12, linePoints.length * 8));
    const placedCount = cursor ? linePoints.length - 1 : linePoints.length;
    return (
      <group>
        <Line raycast={raycastNone} points={sampled.map((pt) => [pt[0], pt[1], pt[2]])} color={palette.accent} lineWidth={2} />
        {linePoints.slice(0, placedCount).map((pt, i) => (
          <mesh key={i} position={pt} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} />
          </mesh>
        ))}
      </group>
    );
  }
  return <group>{linePoints.length >= 2 ? <Line raycast={raycastNone} points={linePoints.map((pt) => [pt[0], pt[1], pt[2]])} color={palette.accent} lineWidth={2} /> : null}</group>;
}

function EntityHighlightItem({ item, geometry }: { readonly item: DisplayItem; readonly geometry?: SpatialPickGeometry | null }): ReactNode {
  const p = item.params;
  if (!p || !geometry) return null;
  const entity = p.entity;
  if (!entity || typeof entity !== "object") return null;
  const kind = (entity as { kind?: unknown }).kind;
  const id = (entity as { id?: unknown }).id;
  if (typeof kind !== "string" || typeof id !== "string") return null;
  return <GeometryTargetWireframes geometry={geometry} targets={[{ kind: kind as ModelEntityKind, id }]} transform={(pt) => pt} color={spatialSceneColors().construction} opacity={0.85} />;
}

function CurveItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const points = readVec3Array(item.params?.points);
  if (points.length < 2) return null;
  return <Line raycast={raycastNone} points={points.map((pt) => [pt[0], pt[1], pt[2]])} color={spatialSceneColors().accent} lineWidth={2} />;
}

function isMeshTransferLike(v: unknown): v is MeshTransfer {
  if (!v || typeof v !== "object") return false;
  const m = v as MeshTransfer;
  return m.position instanceof Float32Array && m.index instanceof Uint32Array && Array.isArray(m.faceGroups);
}

function MeshItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const raw = item.params?.mesh ?? item.params?.transfer;
  if (!isMeshTransferLike(raw)) return null;
  return <TessellatedCommitMesh mesh={raw} />;
}

function defaultDisplayItemNode(item: DisplayItem, geometry?: SpatialPickGeometry | null): ReactNode {
  switch (item.kind) {
    case "box-preview":
      return <BoxPreviewItem item={item} />;
    case "point":
      return <PointItem item={item} />;
    case "linear-handle":
      return <LinearHandleItem item={item} />;
    case "segment":
      return <SegmentItem item={item} />;
    case "label":
      return <LabelItem item={item} />;
    case "preview":
      return <PreviewItem item={item} geometry={geometry} />;
    case "entity-highlight":
      return <EntityHighlightItem item={item} geometry={geometry} />;
    case "curve":
      return <CurveItem item={item} />;
    case "mesh":
      return <MeshItem item={item} />;
    default:
      return null;
  }
}

// #region 🎨️HostCustomization
/** 🖼️ Host hook that renders one resolved `DisplayItem` inside `<InteractionDisplay>`. */
export type SpatialDisplayItemRenderer = (item: DisplayItem, geometry: SpatialPickGeometry | null | undefined, defaultRender: () => ReactNode) => ReactNode;

const spatialDisplayItemRenderers = ephemeralMap<string, SpatialDisplayItemRenderer>("s.plugins.cad.modules.renderer.component.tsx.spatialDisplayItemRenderers");

/** 🖼️ Registers a custom display kind; returns unregister. Libraries extend without forking the package. */
export function registerSpatialDisplayItemKind(kind: string, render: SpatialDisplayItemRenderer): () => void {
  spatialDisplayItemRenderers.set(kind, render);
  return () => spatialDisplayItemRenderers.delete(kind);
}

/** 🖼️ Looks up a host-registered display kind renderer. */
export function getSpatialDisplayItemKindRenderer(kind: string): SpatialDisplayItemRenderer | undefined {
  return spatialDisplayItemRenderers.get(kind);
}

function renderDisplayItem(item: DisplayItem, geometry: SpatialPickGeometry | null | undefined, renderItem?: SpatialDisplayItemRenderer): ReactNode {
  const fallback = () => defaultDisplayItemNode(item, geometry);
  const custom = renderItem ?? spatialDisplayItemRenderers.get(item.kind);
  return custom ? custom(item, geometry, fallback) : fallback();
}
// #endregion 🎨️HostCustomization

/** 🖼️ Maps `DisplayModel.items` to R3F nodes (must live under `<Canvas>`). */
export function InteractionDisplay({ model, geometry, renderItem }: { readonly model: DisplayModel; readonly geometry?: SpatialPickGeometry | null; readonly renderItem?: SpatialDisplayItemRenderer }): ReactNode {
  return (
    <group>
      {model.items.map((item) => (
        <group key={item.id}>{renderDisplayItem(item, geometry, renderItem)}</group>
      ))}
    </group>
  );
}
// #endregion 🖼️DisplayPrimitives
