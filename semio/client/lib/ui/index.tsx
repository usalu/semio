// #region 🧲Header
// semio/ui/index.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// 🖱️Shared semio ui components.
// #endregion 🧲Header

// #region 🧾PlainJson
/** @emoji 🧾 Local JSON object/value shapes for diagram and Storybook payloads (not re-exported from @semio/js). */
type PlainJsonObject = Readonly<Record<string, unknown>>;
type PlainJsonValue = string | number | boolean | null | readonly PlainJsonValue[] | PlainJsonObject;
// #endregion 🧾PlainJson

import { Breadcrumb, Button, Section } from "@elements/ui/elements";
import { Bounds, useBounds } from "@react-three/drei/core/Bounds.js";
import { Clone } from "@react-three/drei/core/Clone.js";
import { Edges } from "@react-three/drei/core/Edges.js";
import { GizmoHelper } from "@react-three/drei/core/GizmoHelper.js";
import { GizmoViewport } from "@react-three/drei/core/GizmoViewport.js";
import { useGLTF } from "@react-three/drei/core/Gltf.js";
import { Grid } from "@react-three/drei/core/Grid.js";
import { OrbitControls } from "@react-three/drei/core/OrbitControls.js";
import { Canvas as ThreeCanvas, useFrame, useThree } from "@react-three/fiber";
/** @emoji 🧾 Kit attribute row (plain JSON; UI / Storybook only). */
export type Attribute = PlainJsonObject;
/** @emoji 🧾 UV coordinate (plain JSON). */
export type Coordinate = PlainJsonObject;
/** @emoji 🧾 Plane origin+axes (plain JSON). */
export type Plane = PlainJsonObject;
/** @emoji 🧾 XYZ vector (plain JSON). */
export type SemioVector = PlainJsonObject;

/** @emoji 🧾 Scene/canvas camera JSON (position+target, optional legacy up/forward). */
export type SceneCamera = PlainJsonObject & {
  position: { x: number; y: number; z: number };
  target?: { x: number; y: number; z: number };
  forward?: { x: number; y: number; z: number };
  up?: { x: number; y: number; z: number };
};

const DiffStatus = Object.freeze({ Removed: "removed", Added: "added", Modified: "modified" } as const);
type SemioFile = Record<string, unknown>;

/** @emoji 🧾 Flat port row from plain kit `types[].ports` (not CQRS {@link import("@semio/js").Port}). */
export type KitPortPlain = PlainJsonObject & {
  id?: string;
  name?: string;
  description?: string;
  icon?: string;
  maxChildren?: number;
};

/** @emoji 🧾 Plain diagram piece JSON (story/MCP payloads; not CQRS {@link import("@semio/js").Piece}). */
type Piece = PlainJsonObject & {
  id?: string;
  center?: Coordinate | { u?: number; v?: number } | null;
  plane?: Plane | null;
  attributes?: readonly Attribute[];
  type?: PlainJsonObject & { id?: string; name?: string };
  design?: PlainJsonObject;
  designId?: string;
  name?: string;
  scale?: number;
};

/** @emoji 🧾 Plain connection JSON between two piece endpoints. */
type Connection = PlainJsonObject & {
  id?: string;
  connected?: { piece?: { id?: string } };
  connecting?: { piece?: { id?: string } };
  attributes?: readonly Attribute[];
};

/** @emoji 🧾 Plain connector row from kit kind JSON. */
type Connector = PlainJsonObject & {
  id?: string;
  point?: { x: number; y: number; z: number };
  direction?: { x: number; y: number; z: number };
};

/** @emoji 🧾 Kind (type) row for {@link SemioType} and kit browsers (plain `kit.types[]`). */
export type KitKindPlain = PlainJsonObject & {
  id?: string;
  name?: string;
  description?: string;
  createdAt?: string;
  updatedAt?: string;
  icon?: string;
  image?: string;
  representations?: readonly PlainJsonObject[];
  connectors?: readonly Connector[];
};

function pickPreferredRepresentation(representations: readonly PlainJsonObject[], _hints: readonly PlainJsonObject[] = []): PlainJsonObject | undefined {
  return representations[0];
}

//#region 🔖PlainKitAndDiff
/** @emoji 🧾 Algorithm / diagram JSON kit surface (`wip.initialKit` or legacy root). */
export function kitSurface(kit: unknown): Record<string, unknown> {
  const root = kit as { wip?: { initialKit?: Record<string, unknown> } };
  const inner = root.wip?.initialKit;
  if (inner && typeof inner === "object") return inner;
  return (kit as Record<string, unknown>) ?? {};
}

function __itemsOf<T>(node: unknown): readonly T[] {
  if (Array.isArray(node)) return node as readonly T[];
  if (node && typeof node === "object" && "items" in node && Array.isArray((node as { items: unknown }).items)) return (node as { items: T[] }).items;
  return [];
}

function kitJsRows(kit: unknown, key: string): readonly PlainJsonObject[] {
  return __itemsOf(kitSurface(kit)[key]);
}

/** @emoji 🧾 Ports flattened from every type entry on a plain kit snapshot. */
export function getKitPorts(kit: unknown): KitPortPlain[] {
  const types = __itemsOf(kitSurface(kit)["types"]);
  const out: KitPortPlain[] = [];
  for (const t of types) {
    const ports = __itemsOf((t as { ports?: unknown }).ports);
    for (const p of ports) out.push(p as KitPortPlain);
  }
  return out;
}

/** @emoji 🧾 Gap/shift/rise tuple used by move algorithm stories. */
export type MoveVector = Readonly<{ gap: number; shift: number; rise: number }>;

/** @emoji 🧾 Plain kit JSON / bootstrap bundle passed into algorithm + scene surfaces. */
export type Kit = unknown;

/** @emoji 🧾 Storybook / diagram design diff (structural; not GraphQL). */
export type DesignDiff = Readonly<{
  pieces?: Readonly<{
    added?: readonly unknown[];
    removed?: readonly unknown[];
    updated?: readonly unknown[];
    modified?: readonly unknown[];
  }>;
  connections?: Readonly<{
    added?: readonly unknown[];
    removed?: readonly unknown[];
    updated?: readonly unknown[];
  }>;
}>;

/** @emoji 🧾 Plain design JSON accepted by {@link SemioDiagram}. */
export type DesignPlain = Readonly<Record<string, unknown>>;

/** @emoji 🧾 Diagram-facing design value (plain JSON or legacy class with `toPlain`). */
export type Design = unknown;

function __plainFromDesign(d: unknown): Record<string, unknown> {
  const anyD = d as { toPlain?: () => DesignPlain };
  if (typeof anyD.toPlain === "function") return { ...anyD.toPlain() } as Record<string, unknown>;
  return JSON.parse(JSON.stringify(d ?? {})) as Record<string, unknown>;
}

function __applyDesignDiffToPlain(plain: Record<string, unknown>, diff: DesignDiff): void {
  const pieces = (plain["pieces"] as unknown[] | undefined) ? [...(plain["pieces"] as unknown[])] : [];
  const byId = new Map(pieces.filter((p): p is Record<string, unknown> => p != null && typeof p === "object").map((p) => [String((p as Record<string, unknown>).id ?? ""), p as Record<string, unknown>]));
  const pDiff = diff.pieces;
  if (pDiff?.removed) {
    const rm = new Set(pDiff.removed.map((x) => String((x as { id?: string }).id ?? "")));
    plain["pieces"] = pieces.filter((p) => !rm.has(String((p as { id?: string }).id ?? "")));
  }
  if (pDiff?.updated) {
    for (const u of pDiff.updated) {
      const row = u as { piece?: { id?: string }; diff?: Record<string, unknown> };
      const id = String(row.piece?.id ?? "");
      if (!id) continue;
      const cur = byId.get(id) ?? { id };
      byId.set(id, { ...cur, ...row.diff });
    }
    plain["pieces"] = Array.from(byId.values());
  }
  if (pDiff?.added?.length) {
    const merged = [...((plain["pieces"] as unknown[]) ?? [])];
    for (const a of pDiff.added) merged.push(a);
    plain["pieces"] = merged;
  }
  const cDiff = diff.connections;
  if (cDiff?.removed) {
    const conns = ((plain["connections"] as unknown[]) ?? []).filter((c) => !cDiff.removed?.some((r) => String((r as { id?: string }).id ?? "") === String((c as { id?: string }).id ?? "")));
    plain["connections"] = conns;
  }
  if (cDiff?.added?.length) {
    plain["connections"] = [...((plain["connections"] as unknown[]) ?? []), ...cDiff.added];
  }
}

/** @emoji 🧾 Immutable preview merge for diagram diff rendering (no legacy {@link DesignEntity}). */
export function previewDesignWithDiff(design: unknown, diff: DesignDiff | undefined): unknown {
  if (!diff) return design;
  const merged = __plainFromDesign(design);
  __applyDesignDiffToPlain(merged, diff);
  return merged;
}
//#endregion 🔖PlainKitAndDiff
import * as React from "react";
import * as THREE from "three";
import { clone as cloneSkeleton } from "three/examples/jsm/utils/SkeletonUtils.js";

// #region 💻ControllableState
// Specs: Semio UI components MUST support controlled/uncontrolled and partial/full control.
// This hook is the shared mechanism used by multiple components for interactive state that can
// be externally controlled while still supporting internal defaults.
// Summary: Shared controllable state hook for Semio UI components.

const useResolvedValue = <T,>(value: T | undefined, defaultValue: T) => value ?? defaultValue;

const useInteractiveControllableValue = <T,>(value: T | undefined, defaultValue: T, onChange?: (nextValue: T) => void) => {
  const [internalValue, setInternalValue] = React.useState(value ?? defaultValue);
  const isControlled = value !== undefined && onChange !== undefined;
  const lastExternalValueRef = React.useRef(value);

  React.useEffect(() => {
    if (isControlled) return;
    if (value === undefined) return;
    if (Object.is(lastExternalValueRef.current, value)) return;
    lastExternalValueRef.current = value;
    setInternalValue(value);
  }, [isControlled, value]);

  const resolvedValue = isControlled ? value : internalValue;
  const setValue = React.useCallback(
    (nextValue: T) => {
      if (!isControlled) {
        setInternalValue(nextValue);
      }
      onChange?.(nextValue);
    },
    [isControlled, onChange],
  );
  return [resolvedValue, setValue, isControlled] as const;
};

// #endregion 💻ControllableState

// #region 🗃️Exports

// Re-export the runtime-safe ui primitives from @elements/ui/elements.

export * from "@elements/ui/elements";

// #endregion 🗃️Exports

// #region ⏱️Kit
// Specs: Kit provides a kit-scoped artifact picker (designs, kinds, family-owned ports, type connectors)
// with the standard Semio UI controllable-state pattern: partial/full controlled/uncontrolled for
// both available data and selection. It supports partial/full select via per-group enable flags.
// Summary: Kit hierarchy browser with composed Semio viewers, browse history, metadata, and open action.
// Specs (navigation): Back/forward buttons appear before the breadcrumb; double-click on a scene or diagram piece
// navigates to the nested design or referenced kind when that node exists in the current hierarchy.

export type KitGroupKind = "design" | "type" | "port" | "connector";

/** Kit-level {@link Port} rows (from {@link Kit.ports}), not type {@link Connector}s. */
export interface KitPortArtifact {
  id: string;
  name: string;
  description?: string;
  icon?: string;
  maxChildren?: number;
}

/** Flattened type {@link Connector} rows for browsing (nested under kinds in the hierarchy). */
export interface KitConnectorArtifact {
  id: string;
  typeId: string;
  port?: string;
  name?: string;
  description?: string;
  mandatory?: boolean;
  maxChildren?: number;
}

export type KitDesignData = {
  id?: string;
  name?: string;
  description?: string;
  createdAt?: string;
  updatedAt?: string;
  unit?: string;
  icon?: string;
  image?: string;
  parent?: { id: string };
};

export interface KitKindData {
  id?: string;
  name?: string;
  description?: string;
  createdAt?: string;
  updatedAt?: string;
  icon?: string;
  image?: string;
  parent?: { id: string };
}

export interface KitData {
  id?: string;
  name?: string;
  description?: string;
  version?: string;
  createdAt?: string;
  updatedAt?: string;
  homepage?: string;
  remote?: string;
  preview?: string;
  icon?: string;
  image?: string;
  license?: string;
  designs?: KitDesignData[];
  types?: KitKindData[];
  /** Family-owned ports flattened for artifact browsing. */
  ports?: KitPortArtifact[];
  /** Flattened connectors from kinds ({@link Connector} on {@link KitKindPlain}). */
  connectors?: KitConnectorArtifact[];
}

export interface KitSelection {
  designIds?: string[];
  typeIds?: string[];
  portIds?: string[];
  connectorIds?: string[];
}

export interface KitProps {
  kit?: Kit;
  data?: KitData;
  defaultData?: KitData;
  onDataChange?: (data: KitData) => void;

  selection?: KitSelection;
  defaultSelection?: KitSelection;
  onSelectionChange?: (selection: KitSelection) => void;

  selectionEnabled?: boolean;
  designSelectionEnabled?: boolean;
  typeSelectionEnabled?: boolean;
  portSelectionEnabled?: boolean;
  connectorSelectionEnabled?: boolean;

  dataEnabled?: boolean;
  designDataEnabled?: boolean;
  typeDataEnabled?: boolean;
  portDataEnabled?: boolean;
  connectorDataEnabled?: boolean;

  onOpenArtifact?: (artifact: KitHierarchyNode) => void;

  title?: string;
  className?: string;
}

const normalizeKitSelection = (selection?: KitSelection): KitSelection => ({
  designIds: selection?.designIds ?? [],
  typeIds: selection?.typeIds ?? [],
  portIds: selection?.portIds ?? [],
  connectorIds: selection?.connectorIds ?? [],
});

const getReferenceId = (value: unknown): string | undefined => {
  if (typeof value === "string") return value;
  if (value && typeof value === "object" && "id" in value && typeof value.id === "string") return value.id;
  return undefined;
};

const getReferenceLabel = (value: unknown): string | undefined => {
  if (typeof value === "string") return value;
  if (value && typeof value === "object") {
    if ("name" in value && typeof value.name === "string" && value.name.length > 0) return value.name;
    if ("id" in value && typeof value.id === "string" && value.id.length > 0) return value.id;
    if ("id" in value && typeof value.id === "string" && value.id.length > 0) return value.id;
  }
  return undefined;
};

const buildKitDataFromKit = (kit: unknown | undefined): KitData => {
  if (!kit) return {};
  const surface = kitSurface(kit);
  const designsRaw = __itemsOf(surface["designs"]);
  const typesRaw = __itemsOf(surface["types"]);
  const designs = designsRaw.map((d) => {
    const row = d as Record<string, unknown>;
    const parent = row["parent"] as { id?: string } | undefined;
    return {
      id: String(row["id"] ?? ""),
      name: String(row["name"] ?? ""),
      description: String(row["description"] ?? ""),
      createdAt: row["createdAt"] as string | undefined,
      updatedAt: row["updatedAt"] as string | undefined,
      unit: row["unit"] as string | undefined,
      icon: row["icon"] as string | undefined,
      image: row["image"] as string | undefined,
      parent: parent?.id ? { id: parent.id } : undefined,
    };
  });
  const types = typesRaw.map((t) => {
    const row = t as Record<string, unknown>;
    const parent = row["parent"] as { id?: string } | undefined;
    return {
      id: String(row["id"] ?? ""),
      name: String(row["name"] ?? ""),
      description: String(row["description"] ?? ""),
      createdAt: row["createdAt"] as string | undefined,
      updatedAt: row["updatedAt"] as string | undefined,
      icon: row["icon"] as string | undefined,
      image: row["image"] as string | undefined,
      parent: parent?.id ? { id: parent.id } : undefined,
    };
  });
  const ports: KitPortArtifact[] = getKitPorts(kit).map((p: KitPortPlain) => ({
    id: String(p.id ?? ""),
    name: String(p.name ?? ""),
    description: typeof p.description === "string" ? p.description : undefined,
    icon: typeof p.icon === "string" ? p.icon : undefined,
    maxChildren: typeof p.maxChildren === "number" ? p.maxChildren : undefined,
  }));
  const connectors: KitConnectorArtifact[] = typesRaw.flatMap((t) => {
    const row = t as { id?: string; connectors?: unknown };
    const conns = __itemsOf(row.connectors);
    return conns.map((c) => {
      const cc = c as Record<string, unknown>;
      return {
        id: String(cc["id"] ?? ""),
        typeId: String(row.id ?? ""),
        port: getReferenceId(cc["port"]),
        name: String(cc["name"] ?? getReferenceLabel(cc["port"]) ?? "connector"),
        description: cc["description"] as string | undefined,
        mandatory: cc["mandatory"] as boolean | undefined,
        maxChildren: cc["maxChildren"] as number | undefined,
      };
    });
  });
  return {
    id: String(surface["id"] ?? ""),
    name: String(surface["name"] ?? ""),
    description: String(surface["description"] ?? ""),
    version: String(surface["version"] ?? ""),
    createdAt: surface["createdAt"] as string | undefined,
    updatedAt: surface["updatedAt"] as string | undefined,
    homepage: surface["homepage"] as string | undefined,
    remote: surface["remote"] as string | undefined,
    preview: surface["preview"] as string | undefined,
    icon: surface["icon"] as string | undefined,
    image: surface["image"] as string | undefined,
    license: surface["license"] as string | undefined,
    designs,
    types,
    ports,
    connectors,
  };
};

type KitHierarchyNodeKind = "scope" | "kit" | "group" | "design" | "kind" | "port" | "connector";

export interface KitHierarchyNode {
  key: string;
  kind: KitHierarchyNodeKind;
  label: string;
  parentKey?: string;
  id?: string;
  groupKind?: KitGroupKind;
  href?: string;
  summary?: string;
  metadata: Array<{ label: string; value: string }>;
}

interface KitHierarchy {
  rootKey: string;
  nodesByKey: Map<string, KitHierarchyNode>;
  childKeysByParentKey: Map<string, string[]>;
}

const addKitMetaEntry = (entries: Array<{ label: string; value: string }>, label: string, value: unknown) => {
  if (typeof value !== "string" || value.trim().length === 0) return;
  entries.push({ label, value });
};

const getKitArtifactHref = (value: Partial<KitData & KitDesignData & KitKindData>): string | undefined => value.image ?? value.icon ?? value.preview ?? value.homepage ?? value.remote;

const buildKitHierarchy = (data: KitData, options: { designDataEnabled: boolean; typeDataEnabled: boolean; portDataEnabled: boolean; connectorDataEnabled: boolean }): KitHierarchy => {
  const rootKey = "scope:kit";
  const kitKey = "kit:root";
  const designGroupKey = "group:designs";
  const kindGroupKey = "group:types";
  const kitPortsGroupKey = "group:kit-ports";
  const orphanConnectorGroupKey = "group:orphan-connectors";
  const nodesByKey = new Map<string, KitHierarchyNode>();
  const childKeysByParentKey = new Map<string, string[]>();

  const registerNode = (node: KitHierarchyNode) => {
    nodesByKey.set(node.key, node);
    if (!childKeysByParentKey.has(node.key)) childKeysByParentKey.set(node.key, []);
    if (!node.parentKey) return;
    const currentChildren = childKeysByParentKey.get(node.parentKey) ?? [];
    currentChildren.push(node.key);
    childKeysByParentKey.set(node.parentKey, currentChildren);
  };

  const designCount = String(data.designs?.length ?? 0);
  const kindCount = String(data.types?.length ?? 0);
  const kitPortCount = String(data.ports?.length ?? 0);
  const connectorCount = String(data.connectors?.length ?? 0);

  registerNode({
    key: rootKey,
    kind: "scope",
    label: "Kit",
    summary: "Kit hierarchy root.",
    metadata: [
      { label: "Designs", value: designCount },
      { label: "Types", value: kindCount },
      { label: "Ports", value: kitPortCount },
      { label: "Connectors", value: connectorCount },
    ],
  });

  const kitMetadata: Array<{ label: string; value: string }> = [];
  addKitMetaEntry(kitMetadata, "Name", data.name);
  addKitMetaEntry(kitMetadata, "Id", data.id);
  addKitMetaEntry(kitMetadata, "Description", data.description);
  addKitMetaEntry(kitMetadata, "Version", data.version);
  addKitMetaEntry(kitMetadata, "License", data.license);
  addKitMetaEntry(kitMetadata, "Homepage", data.homepage);
  addKitMetaEntry(kitMetadata, "Remote", data.remote);
  addKitMetaEntry(kitMetadata, "Created", data.createdAt);
  addKitMetaEntry(kitMetadata, "Updated", data.updatedAt);
  registerNode({
    key: kitKey,
    kind: "kit",
    label: data.name?.trim() || "Unnamed Kit",
    parentKey: rootKey,
    id: data.id,
    href: getKitArtifactHref(data),
    summary: data.description || "Kit metadata.",
    metadata: kitMetadata,
  });

  if (options.designDataEnabled) {
    registerNode({
      key: designGroupKey,
      kind: "group",
      label: "Designs",
      parentKey: kitKey,
      groupKind: "design",
      summary: "Design hierarchy.",
      metadata: [{ label: "Count", value: designCount }],
    });
  }

  if (options.typeDataEnabled) {
    registerNode({
      key: kindGroupKey,
      kind: "group",
      label: "Types",
      parentKey: kitKey,
      groupKind: "type",
      summary: "Type hierarchy.",
      metadata: [{ label: "Count", value: kindCount }],
    });
  }

  if (options.portDataEnabled && (data.ports?.length ?? 0) > 0) {
    registerNode({
      key: kitPortsGroupKey,
      kind: "group",
      label: "Ports",
      parentKey: kitKey,
      groupKind: "port",
      summary: "Kit-level port definitions.",
      metadata: [{ label: "Count", value: kitPortCount }],
    });
  }

  const kindKeyById = new Map<string, string>();
  (data.types ?? []).forEach((kind) => {
    const kindId = String(kind.id ?? "");
    if (!kindId) return;
    const metadata: Array<{ label: string; value: string }> = [];
    addKitMetaEntry(metadata, "Kind", "Type");
    addKitMetaEntry(metadata, "Name", kind.name ?? "");
    addKitMetaEntry(metadata, "Id", kindId);
    addKitMetaEntry(metadata, "Description", kind.description ?? "");
    addKitMetaEntry(metadata, "Created", kind.createdAt ?? "");
    addKitMetaEntry(metadata, "Updated", kind.updatedAt ?? "");
    const key = `kind:${kindId}`;
    kindKeyById.set(kindId, key);
    registerNode({
      key,
      kind: "kind",
      label: kind.name || kindId,
      parentKey: kind.parent?.id ? `kind:${kind.parent.id}` : kindGroupKey,
      id: kindId,
      groupKind: "type",
      href: getKitArtifactHref(kind),
      summary: kind.description || "Type artifact.",
      metadata,
    });
  });

  (data.designs ?? []).forEach((design) => {
    const designId = String(design.id ?? "");
    if (!designId) return;
    const metadata: Array<{ label: string; value: string }> = [];
    addKitMetaEntry(metadata, "Kind", "Design");
    addKitMetaEntry(metadata, "Name", design.name ?? "");
    addKitMetaEntry(metadata, "Id", designId);
    addKitMetaEntry(metadata, "Description", design.description ?? "");
    addKitMetaEntry(metadata, "Unit", design.unit ?? "");
    addKitMetaEntry(metadata, "Created", design.createdAt ?? "");
    addKitMetaEntry(metadata, "Updated", design.updatedAt ?? "");
    registerNode({
      key: `design:${designId}`,
      kind: "design",
      label: design.name || designId,
      parentKey: design.parent?.id ? `design:${design.parent.id}` : designGroupKey,
      id: designId,
      groupKind: "design",
      href: getKitArtifactHref(design),
      summary: design.description || "Design artifact.",
      metadata,
    });
  });

  if (options.portDataEnabled) {
    (data.ports ?? []).forEach((port) => {
      const metadata: Array<{ label: string; value: string }> = [];
      addKitMetaEntry(metadata, "Kind", "Port");
      addKitMetaEntry(metadata, "Name", port.name);
      addKitMetaEntry(metadata, "Id", port.id);
      addKitMetaEntry(metadata, "Description", port.description);
      registerNode({
        key: `port:${port.id}`,
        kind: "port",
        label: port.name || port.id,
        parentKey: kitPortsGroupKey,
        id: port.id,
        groupKind: "port",
        href: port.icon,
        summary: port.description || "Port definition.",
        metadata,
      });
    });
  }

  let orphanConnectorCount = 0;
  if (options.connectorDataEnabled) {
    (data.connectors ?? []).forEach((connector) => {
      const metadata: Array<{ label: string; value: string }> = [];
      addKitMetaEntry(metadata, "Kind", "Connector");
      addKitMetaEntry(metadata, "Name", connector.name);
      addKitMetaEntry(metadata, "Id", connector.id);
      addKitMetaEntry(metadata, "Connector Id", connector.id);
      addKitMetaEntry(metadata, "Port", connector.port);
      addKitMetaEntry(metadata, "Description", connector.description);
      addKitMetaEntry(metadata, "Mandatory", connector.mandatory === undefined ? undefined : String(connector.mandatory));
      const parentKey = kindKeyById.get(connector.typeId) ?? orphanConnectorGroupKey;
      if (parentKey === orphanConnectorGroupKey) orphanConnectorCount += 1;
      registerNode({
        key: `connector:${connector.id}`,
        kind: "connector",
        label: connector.name || connector.id,
        parentKey,
        id: connector.id,
        groupKind: "connector",
        summary: connector.description || "Connector on a kind.",
        metadata,
      });
    });
  }

  if (orphanConnectorCount > 0 && options.connectorDataEnabled) {
    registerNode({
      key: orphanConnectorGroupKey,
      kind: "group",
      label: "Connectors",
      parentKey: kitKey,
      groupKind: "connector",
      summary: "Connectors without a resolved kind parent.",
      metadata: [{ label: "Count", value: String(orphanConnectorCount) }],
    });
  }

  return { rootKey, nodesByKey, childKeysByParentKey };
};

const getKitNodePath = (hierarchy: KitHierarchy, nodeKey: string | undefined): KitHierarchyNode[] => {
  if (!nodeKey) return [];
  const path: KitHierarchyNode[] = [];
  let currentKey: string | undefined = nodeKey;
  while (currentKey) {
    const node = hierarchy.nodesByKey.get(currentKey);
    if (!node) break;
    path.unshift(node);
    currentKey = node.parentKey;
  }
  return path;
};

const getKitChildNodes = (hierarchy: KitHierarchy, node: KitHierarchyNode): KitHierarchyNode[] => {
  const childKeys = hierarchy.childKeysByParentKey.get(node.key) ?? [];
  return childKeys
    .map((key) => hierarchy.nodesByKey.get(key))
    .filter((value): value is KitHierarchyNode => Boolean(value))
    .sort((left, right) => {
      if (left.kind === "group" && right.kind !== "group") return -1;
      if (left.kind !== "group" && right.kind === "group") return 1;
      return left.label.localeCompare(right.label);
    });
};

const getKitNodeSelection = (node: KitHierarchyNode): KitSelection => {
  if (node.kind === "design") return { designIds: node.id ? [node.id] : [], typeIds: [], portIds: [], connectorIds: [] };
  if (node.kind === "kind") return { designIds: [], typeIds: node.id ? [node.id] : [], portIds: [], connectorIds: [] };
  if (node.kind === "port") return { designIds: [], typeIds: [], portIds: node.id ? [node.id] : [], connectorIds: [] };
  if (node.kind === "connector") return { designIds: [], typeIds: [], portIds: [], connectorIds: node.id ? [node.id] : [] };
  return { designIds: [], typeIds: [], portIds: [], connectorIds: [] };
};

const getSelectedKitNodeKey = (hierarchy: KitHierarchy, selection: KitSelection): string | undefined => {
  const selectedConnector = selection.connectorIds?.[0];
  if (selectedConnector && hierarchy.nodesByKey.has(`connector:${selectedConnector}`)) return `connector:${selectedConnector}`;
  const selectedPort = selection.portIds?.[0];
  if (selectedPort && hierarchy.nodesByKey.has(`port:${selectedPort}`)) return `port:${selectedPort}`;
  const selectedKind = selection.typeIds?.[0];
  if (selectedKind && hierarchy.nodesByKey.has(`kind:${selectedKind}`)) return `kind:${selectedKind}`;
  const selectedDesign = selection.designIds?.[0];
  if (selectedDesign && hierarchy.nodesByKey.has(`design:${selectedDesign}`)) return `design:${selectedDesign}`;
  return undefined;
};

const getDefaultKitNodeKey = (hierarchy: KitHierarchy): string => {
  const groupKeys = hierarchy.childKeysByParentKey.get("kit:root") ?? [];
  for (const groupKey of groupKeys) {
    const node = hierarchy.nodesByKey.get(groupKey);
    if (node?.kind !== "group") continue;
    const firstChildKey = hierarchy.childKeysByParentKey.get(groupKey)?.[0];
    if (firstChildKey) return firstChildKey;
  }
  return "kit:root";
};

const getReadableKitMetaLabel = (label: string, value: string): string => {
  if (label === "Id") return "ID";
  if (label === "Connector Id") return "Connector";
  if (label === "Mandatory") return value === "true" ? "Required" : "Optional";
  if (label === "Created") return "Created";
  if (label === "Updated") return "Updated";
  return label;
};

const truncateKitText = (value: string, maxLength = 48): string => {
  const normalizedValue = value.replace(/\s+/g, " ").trim();
  if (normalizedValue.length <= maxLength) return normalizedValue;
  return `${normalizedValue.slice(0, Math.max(0, maxLength - 1)).trimEnd()}…`;
};

const isKitUrlLabel = (label: string): boolean => ["Homepage", "Remote", "View", "Preview", "Image", "Icon"].includes(label);

const formatKitRelativeTime = (value: string): string => {
  const timestamp = Date.parse(value);
  if (Number.isNaN(timestamp)) return value;
  const diffMs = timestamp - Date.now();
  const absMs = Math.abs(diffMs);
  const units: Array<[Intl.RelativeTimeFormatUnit, number]> = [
    ["year", 1000 * 60 * 60 * 24 * 365],
    ["month", 1000 * 60 * 60 * 24 * 30],
    ["week", 1000 * 60 * 60 * 24 * 7],
    ["day", 1000 * 60 * 60 * 24],
    ["hour", 1000 * 60 * 60],
    ["minute", 1000 * 60],
  ];
  const formatter = new Intl.RelativeTimeFormat(undefined, { numeric: "auto" });
  for (const [unit, unitMs] of units) {
    if (absMs >= unitMs) return formatter.format(Math.round(diffMs / unitMs), unit);
  }
  return "just now";
};

const getKitMetaDisplay = (entry: { label: string; value: string }): { text: string; href?: string } => {
  const readableLabel = getReadableKitMetaLabel(entry.label, entry.value);
  if (readableLabel === "Required" || readableLabel === "Optional") {
    return { text: readableLabel };
  }
  if (isKitUrlLabel(entry.label)) {
    return { text: readableLabel, href: entry.value };
  }
  if (entry.label === "Created" || entry.label === "Updated") {
    return { text: `${readableLabel} ${formatKitRelativeTime(entry.value)}` };
  }
  return { text: `${readableLabel}: ${entry.value}` };
};

const getVisibleKitMetadata = (node: KitHierarchyNode): Array<{ label: string; value: string }> =>
  node.metadata.filter((entry) => {
    if (entry.label === "Kind") return false;
    if (entry.label === "Id") return false;
    if (entry.label === "Connector Id") return false;
    if (entry.label === "Unit") return false;
    if (entry.label === "Name" && entry.value === node.label) return false;
    if (entry.label === "Mandatory" && entry.value === "false") return false;
    if (entry.label === "Description" && entry.value === node.summary) return false;
    return true;
  });

const getSecondaryKitMetadata = (node: KitHierarchyNode): Array<{ label: string; value: string }> => node.metadata.filter((entry) => entry.label === "Created" || entry.label === "Updated");

const getKitTitle = (data: KitData, fallbackTitle: string): string => {
  const name = data.name?.trim();
  const version = data.version?.trim();
  if (name && version) return `${name} · ${version}`;
  if (name) return name;
  return fallbackTitle;
};

//#region 🧲KitArtifactShells
// Specs: When only {@link KitData} shells exist, synthesize minimal {@link Design}/{@link KitKindPlain} so {@link SemioDesign}/{@link SemioType} can mount.
// Summary: Shell entities for kit-browser previews without full geometry.

const kitDesignDataToShellDesign = (d: KitDesignData): Design =>
  ({
    id: d.id,
    name: d.name,
    description: d.description,
    createdAt: d.createdAt,
    updatedAt: d.updatedAt,
    unit: d.unit,
    icon: d.icon,
    image: d.image,
    pieces: [],
    connections: [],
  }) as unknown as Design;

const kitKindDataToShellKind = (k: KitKindData): KitKindPlain =>
  ({
    id: k.id,
    name: k.name,
    description: k.description,
    createdAt: k.createdAt,
    updatedAt: k.updatedAt,
    icon: k.icon,
    image: k.image,
    representations: [],
    connectors: [],
  }) as KitKindPlain;

const resolveKitArtifactDesignForPreview = (design: Design, kit: Kit | undefined): Design => {
  if (!kit) return design;
  return mcpFlattenDesignForSemioSurface(design, kit, "design");
};

//#endregion 🧲KitArtifactShells

export const SemioKit: React.FC<KitProps> = ({
  kit,
  data,
  defaultData,
  onDataChange,
  selection,
  defaultSelection,
  onSelectionChange,
  selectionEnabled,
  designSelectionEnabled = true,
  typeSelectionEnabled = true,
  portSelectionEnabled = true,
  connectorSelectionEnabled = true,
  dataEnabled,
  designDataEnabled = true,
  typeDataEnabled = true,
  portDataEnabled = true,
  connectorDataEnabled = true,
  onOpenArtifact,
  title = "Kit Artifacts",
  className,
}) => {
  const effectiveDataEnabled = dataEnabled ?? true;
  const effectiveSelectionEnabled = selectionEnabled ?? true;

  const derivedData = React.useMemo(() => buildKitDataFromKit(kit), [kit]);
  const computedDefaultData = React.useMemo(() => defaultData ?? derivedData, [defaultData, derivedData]);

  const [resolvedData, setResolvedData] = useInteractiveControllableValue<KitData>(data, computedDefaultData, onDataChange);
  const [resolvedSelection, setResolvedSelection] = useInteractiveControllableValue(selection, normalizeKitSelection(defaultSelection), onSelectionChange);

  const effectiveData = effectiveDataEnabled ? resolvedData : {};
  const effectiveDesigns = designDataEnabled ? (effectiveData.designs ?? []) : [];
  const effectiveTypes = typeDataEnabled ? (effectiveData.types ?? []) : [];
  const effectiveKitPorts = portDataEnabled ? (effectiveData.ports ?? []) : [];
  const effectiveConnectors = connectorDataEnabled ? (effectiveData.connectors ?? []) : [];

  const setNextSelection = React.useCallback(
    (next: { designIds?: string[]; typeIds?: string[]; portIds?: string[]; connectorIds?: string[] }) => {
      if (!effectiveSelectionEnabled) return;
      setResolvedSelection({
        designIds: designSelectionEnabled ? (next.designIds ?? []) : [],
        typeIds: typeSelectionEnabled ? (next.typeIds ?? []) : [],
        portIds: portSelectionEnabled ? (next.portIds ?? []) : [],
        connectorIds: connectorSelectionEnabled ? (next.connectorIds ?? []) : [],
      });
    },
    [connectorSelectionEnabled, designSelectionEnabled, effectiveSelectionEnabled, portSelectionEnabled, setResolvedSelection, typeSelectionEnabled],
  );

  // If kit changes and data is uncontrolled, adopt derived data from `kit`.
  // When `data` is set without `onDataChange`, {@link useInteractiveControllableValue} is still "uncontrolled"
  // (isControlled requires both props); without this guard we would overwrite synced `data` with `{}` from
  // {@link buildKitDataFromKit}(undefined) — breaking MCP viewers that pass `data={payload.kitArtifacts}` only.
  React.useEffect(() => {
    if (!effectiveDataEnabled) return;
    if (data !== undefined && onDataChange !== undefined) return;
    if (data !== undefined) return;
    setResolvedData(derivedData);
  }, [data, derivedData, effectiveDataEnabled, onDataChange, setResolvedData]);

  const headerStats = React.useMemo(() => {
    const parts: string[] = [];
    if (designDataEnabled) parts.push(`${effectiveDesigns.length} designs`);
    if (typeDataEnabled) parts.push(`${effectiveTypes.length} types`);
    if (portDataEnabled) parts.push(`${effectiveKitPorts.length} ports`);
    if (connectorDataEnabled) parts.push(`${effectiveConnectors.length} connectors`);
    return parts.join(" · ");
  }, [connectorDataEnabled, designDataEnabled, effectiveConnectors.length, effectiveDesigns.length, effectiveKitPorts.length, effectiveTypes.length, portDataEnabled, typeDataEnabled]);

  const hierarchy = React.useMemo(
    () =>
      buildKitHierarchy(
        {
          ...effectiveData,
          designs: effectiveDesigns,
          types: effectiveTypes,
          ports: effectiveKitPorts,
          connectors: effectiveConnectors,
        },
        {
          designDataEnabled,
          typeDataEnabled,
          portDataEnabled,
          connectorDataEnabled,
        },
      ),
    [connectorDataEnabled, designDataEnabled, effectiveConnectors, effectiveData, effectiveDesigns, effectiveKitPorts, effectiveTypes, portDataEnabled, typeDataEnabled],
  );

  const selectedNodeKey = React.useMemo(() => getSelectedKitNodeKey(hierarchy, resolvedSelection), [hierarchy, resolvedSelection]);
  const [focusedNodeKey, setFocusedNodeKey] = React.useState<string>(() => selectedNodeKey ?? getDefaultKitNodeKey(hierarchy));

  const kitBrowseScopeKey = effectiveData.id ?? derivedData.id ?? effectiveData.name ?? "__semio-kit__";

  const [browse, setBrowse] = React.useState<{ stack: string[]; index: number }>(() => ({
    stack: [selectedNodeKey ?? getDefaultKitNodeKey(hierarchy)],
    index: 0,
  }));

  React.useEffect(() => {
    const preferredKey = selectedNodeKey ?? focusedNodeKey;
    if (preferredKey && hierarchy.nodesByKey.has(preferredKey)) {
      if (preferredKey !== focusedNodeKey) setFocusedNodeKey(preferredKey);
      return;
    }
    const fallbackKey = getDefaultKitNodeKey(hierarchy);
    if (fallbackKey !== focusedNodeKey) setFocusedNodeKey(fallbackKey);
  }, [focusedNodeKey, hierarchy, selectedNodeKey]);

  const focusedNode = hierarchy.nodesByKey.get(focusedNodeKey) ?? hierarchy.nodesByKey.get(getDefaultKitNodeKey(hierarchy))!;
  const path = React.useMemo(() => getKitNodePath(hierarchy, focusedNode.key).filter((node) => node.kind !== "scope" && node.kind !== "kit"), [focusedNode.key, hierarchy]);
  const resolvedTitle = React.useMemo(() => getKitTitle(effectiveData, title), [effectiveData, title]);

  const applyKitFocus = React.useCallback(
    (nodeKey: string) => {
      const node = hierarchy.nodesByKey.get(nodeKey);
      if (!node) return;
      setFocusedNodeKey(node.key);
      if (!effectiveSelectionEnabled) return;
      if (node.kind === "design" && !designSelectionEnabled) return;
      if (node.kind === "kind" && !typeSelectionEnabled) return;
      if (node.kind === "port" && !portSelectionEnabled) return;
      if (node.kind === "connector" && !connectorSelectionEnabled) return;
      setNextSelection(getKitNodeSelection(node));
    },
    [connectorSelectionEnabled, designSelectionEnabled, effectiveSelectionEnabled, hierarchy.nodesByKey, portSelectionEnabled, setNextSelection, typeSelectionEnabled],
  );

  const prevKitBrowseScopeRef = React.useRef<string | null>(null);
  React.useEffect(() => {
    if (prevKitBrowseScopeRef.current === null) {
      prevKitBrowseScopeRef.current = kitBrowseScopeKey;
      return;
    }
    if (prevKitBrowseScopeRef.current === kitBrowseScopeKey) return;
    prevKitBrowseScopeRef.current = kitBrowseScopeKey;
    const k = getDefaultKitNodeKey(hierarchy);
    setBrowse({ stack: [k], index: 0 });
    applyKitFocus(k);
  }, [applyKitFocus, hierarchy, kitBrowseScopeKey]);

  const browsePush = React.useCallback(
    (nodeKey: string) => {
      setBrowse((b) => {
        const base = b.stack.slice(0, b.index + 1);
        if (base[base.length - 1] === nodeKey) return { stack: base, index: base.length - 1 };
        const stack = [...base, nodeKey];
        return { stack, index: stack.length - 1 };
      });
      applyKitFocus(nodeKey);
    },
    [applyKitFocus],
  );

  const browseBack = React.useCallback(() => {
    setBrowse((b) => {
      if (b.index <= 0) return b;
      const nextIndex = b.index - 1;
      const key = b.stack[nextIndex];
      applyKitFocus(key);
      return { ...b, index: nextIndex };
    });
  }, [applyKitFocus]);

  const browseForward = React.useCallback(() => {
    setBrowse((b) => {
      if (b.index >= b.stack.length - 1) return b;
      const nextIndex = b.index + 1;
      const key = b.stack[nextIndex];
      applyKitFocus(key);
      return { ...b, index: nextIndex };
    });
  }, [applyKitFocus]);

  const canBrowseBack = browse.index > 0;
  const canBrowseForward = browse.index < browse.stack.length - 1;

  const onKitPieceDoubleClick = React.useCallback(
    (piece: Piece) => {
      const nestedDesignId = piece.design?.id;
      if (nestedDesignId && hierarchy.nodesByKey.has(`design:${nestedDesignId}`)) {
        browsePush(`design:${nestedDesignId}`);
        return;
      }
      const kindId = piece.type?.id;
      if (kindId && hierarchy.nodesByKey.has(`kind:${kindId}`)) {
        browsePush(`kind:${kindId}`);
      }
    },
    [browsePush, hierarchy.nodesByKey],
  );

  const artifactDesign: Design | null = React.useMemo(() => {
    if (focusedNode.kind !== "design" || !focusedNode.id) return null;
    const full = derivedData.designs?.find((d) => d.id === focusedNode.id);
    if (full) return full;
    const shell = effectiveDesigns.find((d) => d.id === focusedNode.id);
    return shell ? kitDesignDataToShellDesign(shell) : null;
  }, [effectiveDesigns, focusedNode.id, focusedNode.kind, derivedData.designs]);

  const artifactPreviewDesign = React.useMemo(() => (artifactDesign ? resolveKitArtifactDesignForPreview(artifactDesign, kit) : null), [artifactDesign, kit]);

  const artifactKind: KitKindPlain | null = React.useMemo(() => {
    if (focusedNode.kind !== "kind" || !focusedNode.id) return null;
    const full = derivedData.types?.find((t) => t.id === focusedNode.id);
    if (full) return full as KitKindPlain;
    const shell = effectiveTypes.find((t) => t.id === focusedNode.id);
    return shell ? kitKindDataToShellKind(shell) : null;
  }, [effectiveTypes, focusedNode.id, focusedNode.kind, derivedData.types]);

  const connectorParentKindNode = React.useMemo(() => {
    if (focusedNode.kind !== "connector" || !focusedNode.parentKey) return null;
    const parent = hierarchy.nodesByKey.get(focusedNode.parentKey);
    if (parent?.kind !== "kind" || !parent.id) return null;
    return parent;
  }, [focusedNode, hierarchy.nodesByKey]);

  const connectorHostKind: KitKindPlain | null = React.useMemo(() => {
    if (!connectorParentKindNode?.id) return null;
    const full = derivedData.types?.find((t) => t.id === connectorParentKindNode.id);
    if (full) return full as KitKindPlain;
    const shell = effectiveTypes.find((t) => t.id === connectorParentKindNode.id);
    return shell ? kitKindDataToShellKind(shell) : null;
  }, [connectorParentKindNode?.id, effectiveTypes, derivedData.types]);

  const breadcrumbItems = React.useMemo(() => {
    const rootOptions = (hierarchy.childKeysByParentKey.get("kit:root") ?? [])
      .map((key) => hierarchy.nodesByKey.get(key))
      .filter((node): node is KitHierarchyNode => Boolean(node))
      .map((node) => ({ label: node.label, href: node.key, id: node.id }));

    return [
      {
        content: <span style={{ display: "inline-block", width: 1, overflow: "hidden" }}>&nbsp;</span>,
        options: rootOptions,
        onNavigate: browsePush,
      },
      ...path.map((node) => ({
        id: node.id,
        content: (
          <button
            type="button"
            onClick={() => browsePush(node.key)}
            style={{
              border: 0,
              background: "transparent",
              padding: 0,
              cursor: "pointer",
              color: node.key === focusedNode.key ? "var(--accent)" : "inherit",
              fontWeight: node.key === focusedNode.key ? 700 : 500,
            }}
          >
            {node.label}
          </button>
        ),
        options: getKitChildNodes(hierarchy, node).map((child) => ({ label: child.label, href: child.key, id: child.id })),
        onNavigate: browsePush,
      })),
    ];
  }, [browsePush, focusedNode.key, hierarchy, path]);

  const openArtifact = React.useCallback(() => {
    if (focusedNode.kind === "scope" || focusedNode.kind === "group") return;
    if (onOpenArtifact) {
      onOpenArtifact(focusedNode);
      return;
    }
    if (focusedNode.href && typeof window !== "undefined") {
      window.open(focusedNode.href, "_blank", "noopener,noreferrer");
    }
  }, [focusedNode, onOpenArtifact]);

  const canOpenArtifact = focusedNode.kind !== "scope" && focusedNode.kind !== "group" && (Boolean(onOpenArtifact) || Boolean(focusedNode.href));
  const visibleMetadata = React.useMemo(() => getVisibleKitMetadata(focusedNode), [focusedNode]);
  const secondaryMetadata = React.useMemo(() => getSecondaryKitMetadata(focusedNode), [focusedNode]);
  const descriptionEntry = React.useMemo(() => visibleMetadata.find((entry) => entry.label === "Description"), [visibleMetadata]);
  const detailMetadata = React.useMemo(() => visibleMetadata.filter((entry) => entry.label !== "Description" && entry.label !== "Created" && entry.label !== "Updated"), [visibleMetadata]);
  const renderedBreadcrumbItems = React.useMemo(() => breadcrumbItems.filter((item, index) => index < breadcrumbItems.length - 1 || (item.options?.length ?? 0) > 0), [breadcrumbItems]);

  return (
    <Section title={resolvedTitle} className={className}>
      <div style={{ display: "grid", gap: 6 }}>
        <div style={{ fontSize: 12, opacity: 0.75, lineHeight: 1.2 }}>{headerStats}</div>

        <div style={{ display: "flex", alignItems: "center", gap: 8, flexWrap: "wrap" }}>
          <Button onClick={browseBack} disabled={!canBrowseBack} text="Back" />
          <Button onClick={browseForward} disabled={!canBrowseForward} text="Forward" />
          <div style={{ flex: "1 1 200px", minWidth: 0 }}>
            <Breadcrumb items={renderedBreadcrumbItems} />
          </div>
        </div>

        {secondaryMetadata.length > 0 ? (
          <div style={{ display: "flex", flexWrap: "wrap", gap: 8, fontSize: 11, opacity: 0.58, lineHeight: 1.1 }}>
            {secondaryMetadata.map((entry) => (
              <span key={`${focusedNode.key}:${entry.label}`}>{getKitMetaDisplay(entry).text}</span>
            ))}
          </div>
        ) : null}

        {descriptionEntry ? (
          <div
            title={descriptionEntry.value}
            style={{
              fontSize: 12,
              lineHeight: 1.35,
              overflow: "hidden",
              textOverflow: "ellipsis",
              display: "-webkit-box",
              WebkitLineClamp: 3,
              WebkitBoxOrient: "vertical",
            }}
          >
            {descriptionEntry.value}
          </div>
        ) : focusedNode.summary ? (
          <div
            title={focusedNode.summary}
            style={{
              fontSize: 12,
              lineHeight: 1.35,
              overflow: "hidden",
              textOverflow: "ellipsis",
              display: "-webkit-box",
              WebkitLineClamp: 3,
              WebkitBoxOrient: "vertical",
            }}
          >
            {focusedNode.summary}
          </div>
        ) : null}

        {artifactPreviewDesign && designDataEnabled ? (
          <div
            style={{
              height: 480,
              minHeight: 360,
              width: "100%",
              borderRadius: 8,
              overflow: "hidden",
              border: "1px solid var(--border, rgba(0,0,0,0.12))",
            }}
          >
            <SemioDesign design={artifactPreviewDesign} kit={kit} title={(artifactPreviewDesign as { name?: string }).name ?? focusedNode.label} selectionEnabled={false} splitLayout="always" onPieceDoubleClick={onKitPieceDoubleClick} />
          </div>
        ) : null}

        {artifactKind && typeDataEnabled && focusedNode.kind === "kind" ? (
          <div
            style={{
              height: 480,
              minHeight: 360,
              width: "100%",
              borderRadius: 8,
              overflow: "hidden",
              border: "1px solid var(--border, rgba(0,0,0,0.12))",
            }}
          >
            <SemioType type={artifactKind} kit={kit} title={artifactKind.name ?? focusedNode.label} selectionEnabled={false} />
          </div>
        ) : null}

        {focusedNode.kind === "connector" && connectorHostKind && typeDataEnabled ? (
          <div
            style={{
              height: 480,
              minHeight: 360,
              width: "100%",
              borderRadius: 8,
              overflow: "hidden",
              border: "1px solid var(--border, rgba(0,0,0,0.12))",
            }}
          >
            <SemioType
              type={connectorHostKind}
              kit={kit}
              title={connectorHostKind.name ?? connectorParentKindNode?.label ?? "Type"}
              defaultSelection={{ connectorIds: focusedNode.id ? [focusedNode.id] : [] }}
              selectionEnabled={true}
              connectorSelectionEnabled={false}
            />
          </div>
        ) : null}

        <div style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
          {detailMetadata.length === 0 ? <div style={{ fontSize: 12, opacity: 0.7 }}>No metadata available.</div> : null}
          {detailMetadata.map((entry) => {
            const display = getKitMetaDisplay(entry);
            const truncatedText = truncateKitText(display.text, 40);
            return (
              <div
                key={`${focusedNode.key}:${entry.label}`}
                title={display.text}
                style={{
                  border: "1px solid var(--border, rgba(0,0,0,0.12))",
                  borderRadius: 6,
                  padding: "3px 8px",
                  background: "var(--card, rgba(255,255,255,0.7))",
                  lineHeight: 1.1,
                  fontSize: 12,
                  whiteSpace: "nowrap",
                  maxWidth: "100%",
                  overflow: "hidden",
                }}
              >
                {display.href ? (
                  <a
                    href={display.href}
                    target="_blank"
                    rel="noreferrer noopener"
                    style={{
                      opacity: 0.82,
                      textDecoration: "underline",
                      display: "inline-block",
                      maxWidth: "100%",
                      overflow: "hidden",
                      textOverflow: "ellipsis",
                      verticalAlign: "bottom",
                    }}
                  >
                    {truncatedText}
                  </a>
                ) : (
                  <span
                    style={{
                      opacity: 0.82,
                      display: "inline-block",
                      maxWidth: "100%",
                      overflow: "hidden",
                      textOverflow: "ellipsis",
                      verticalAlign: "bottom",
                    }}
                  >
                    {truncatedText}
                  </span>
                )}
              </div>
            );
          })}
        </div>

        <Button onClick={openArtifact} disabled={!canOpenArtifact} text="Open Artifact" className="w-full" />
      </div>
    </Section>
  );
};

// #endregion ⏱️Kit

// #region 🧫Diagram

const DEFAULT_DIAGRAM_PADDING = 12;
// Specs: Node geometry is measured in the same u / v units as piece centers and the origin rulers.
// Summary: Default radius 0.5 → on-screen diameter 1 after multiplying by scale*resolvedZoom.
const DEFAULT_DIAGRAM_PIECE_RADIUS = 0.5;
const DIAGRAM_PIECE_HOVER_RADIUS_EXTRA = 0.06;
const DIAGRAM_PIECE_SELECTED_RADIUS_EXTRA = 0.12;
const DEFAULT_DIAGRAM_STROKE_WIDTH = 2;
const DEFAULT_DIAGRAM_ZOOM = 1;
/** Lower bound for wheel zoom and fit-to-bounds (smaller = zoom out further). */
const MIN_DIAGRAM_ZOOM = 0.15;
const MAX_DIAGRAM_ZOOM = 12;
const DIAGRAM_ZOOM_STEP = 0.0015;
const MIN_DIAGRAM_SPAN = 1;

// #region 🔖DiagramOrigin
// Specs: Diagram maps piece (u, v) with screen Y = -v; the layout origin is u=0, v=0 (diagram Y=0).
// Summary: Resolves origin and unit-length (1 along u, 1 along v) axis tips in SVG pixel space.

/** Resolves diagram origin (u=0, v=0) into SemioDiagram SVG pixel coordinates. */
const resolveSemioDiagramOriginPixels = (toPixelX: (u: number) => number, toPixelY: (diagramY: number) => number): { x: number; y: number } => ({
  x: toPixelX(0),
  y: toPixelY(0),
});

/** Tips of unit segments from the origin: +1 in u at v=0, +1 in v (diagramY is -v so tip at diagramY=-1). */
const resolveSemioDiagramUnitAxisTips = (toPixelX: (u: number) => number, toPixelY: (diagramY: number) => number): { uTip: { x: number; y: number }; vTip: { x: number; y: number } } => ({
  uTip: { x: toPixelX(1), y: toPixelY(0) },
  vTip: { x: toPixelX(0), y: toPixelY(-1) },
});
// #endregion 🔖DiagramOrigin

export type ZoomTarget = "design" | "diff" | "none";

type DiagramEntityStatus = "default" | "removed" | "added" | "modified";

const DIFF_STATUS_KEY = "semio.diffStatus";
const getDiffStatusFromAttributes = (attributes: readonly Attribute[] | undefined): DiagramEntityStatus => {
  const attr = attributes?.find((a) => a.key === DIFF_STATUS_KEY);
  if (!attr?.value) return "default";
  if (attr.value === DiffStatus.Removed) return "removed";
  if (attr.value === DiffStatus.Added) return "added";
  if (attr.value === DiffStatus.Modified) return "modified";
  return "default";
};

export interface DiagramSelection {
  pieceIds?: string[];
  connectionIds?: string[];
}

export interface DiagramHover {
  pieceId?: string | null;
  connectionId?: string | null;
}

export interface DiagramPan {
  x: number;
  y: number;
}

export interface SemioDiagramProps {
  design: Design;
  designDiff?: DesignDiff;
  defaultDesignDiff?: DesignDiff;
  diffEnabled?: boolean;
  zoomTarget?: ZoomTarget;
  selection?: DiagramSelection;
  defaultSelection?: DiagramSelection;
  selectionEnabled?: boolean;
  pieceSelectionEnabled?: boolean;
  connectionSelectionEnabled?: boolean;
  onSelectionChange?: (selection: DiagramSelection) => void;
  hover?: DiagramHover;
  defaultHover?: DiagramHover;
  hoverEnabled?: boolean;
  pieceHoverEnabled?: boolean;
  connectionHoverEnabled?: boolean;
  onHoverChange?: (hover: DiagramHover) => void;
  pan?: DiagramPan;
  defaultPan?: DiagramPan;
  panEnabled?: boolean;
  onPanChange?: (pan: DiagramPan) => void;
  zoom?: number;
  defaultZoom?: number;
  zoomEnabled?: boolean;
  onZoomChange?: (zoom: number) => void;
  className?: string;
  padding?: number;
  /** Radius of piece nodes in diagram u/v units (default 0.5 → diameter 1). */
  pieceRadius?: number;
  strokeWidth?: number;
  title?: string;
  /** When true, draws origin + unit-length u/v markers (length 1 in piece u,v) behind edges and nodes. */
  showOrigin?: boolean;
  onPieceClick?: (piece: Piece) => void;
  /** Fires on piece double-click in the diagram (does not reset zoom; stops propagation to the diagram shell). */
  onPieceDoubleClick?: (piece: Piece) => void;
  onConnectionClick?: (connection: Connection) => void;
  /** Optional diff whose `pieces.updated[].diff.center` supplies u/v when a piece has no `center` (diagram layout only). */
  layoutDiff?: DesignDiff;
}

interface DiagramPoint {
  id: string;
  piece: Piece;
  u: number;
  v: number;
  status: DiagramEntityStatus;
}

interface DiagramLine {
  id: string;
  connection: Connection;
  source: DiagramPoint;
  target: DiagramPoint;
  status: DiagramEntityStatus;
}

interface DiagramSnapshot {
  lines: DiagramLine[];
  points: DiagramPoint[];
  minU: number;
  maxU: number;
  minY: number;
  maxY: number;
  width: number;
  height: number;
}

interface DiagramBounds {
  minU: number;
  maxU: number;
  minY: number;
  maxY: number;
  width: number;
  height: number;
}

const getEntityStatusColor = (status: DiagramEntityStatus): string => {
  if (status === "removed") return "var(--color-removed, #ef4444)";
  if (status === "added") return "var(--color-new, #22c55e)";
  if (status === "modified") return "var(--color-modified, #f59e0b)";
  return "var(--foreground, #1b1a17)";
};

const getInteractiveEntityColor = (status: DiagramEntityStatus, isSelected: boolean, isHovered: boolean): string => {
  if (isSelected) {
    return status === "default" ? "var(--accent)" : "var(--color-changed-selected)";
  }
  if (isHovered) {
    return status === "default" ? "var(--accent-secondary)" : "var(--color-changed-hovered)";
  }
  return getEntityStatusColor(status);
};

// #region 🎯SelectionBoundsOverlay

// Specs: Diagram SVG overlay and scene AABB use the same accent-relative fill and stroke opacity so selection reads identically in 2D and 3D.
// Summary: Shared opacity tokens for semitransparent selection bounds.

const SEMIO_SELECTION_BOUNDS_FILL_OPACITY = 0.18;
const SEMIO_SELECTION_BOUNDS_STROKE_OPACITY = 0.55;

const computeDiagramSelectionOverlayRect = (
  snapshot: DiagramSnapshot,
  selectedPieceIds: Set<string>,
  selectedConnectionIds: Set<string>,
  toPixelX: (u: number) => number,
  toPixelY: (diagramY: number) => number,
  piecePadPx: number,
  connectionPadPx: number,
): { x: number; y: number; width: number; height: number } | null => {
  const xs: number[] = [];
  const ys: number[] = [];
  for (const point of snapshot.points) {
    if (!selectedPieceIds.has(point.id)) continue;
    xs.push(toPixelX(point.u));
    ys.push(toPixelY(-point.v));
  }
  for (const line of snapshot.lines) {
    if (!selectedConnectionIds.has(line.id)) continue;
    xs.push(toPixelX(line.source.u), toPixelX(line.target.u));
    ys.push(toPixelY(-line.source.v), toPixelY(-line.target.v));
  }
  if (xs.length === 0) return null;
  const hasPieceSelection = snapshot.points.some((p) => selectedPieceIds.has(p.id));
  const hasConnectionSelection = snapshot.lines.some((l) => selectedConnectionIds.has(l.id));
  const pad = hasPieceSelection && hasConnectionSelection ? Math.max(piecePadPx, connectionPadPx) : hasPieceSelection ? piecePadPx : connectionPadPx;
  const minX = Math.min(...xs) - pad;
  const maxX = Math.max(...xs) + pad;
  const minY = Math.min(...ys) - pad;
  const maxY = Math.max(...ys) + pad;
  return { x: minX, y: minY, width: Math.max(maxX - minX, 1), height: Math.max(maxY - minY, 1) };
};

// #endregion 🎯SelectionBoundsOverlay

/** u/v from `pieces.updated[].diff.center` for diagram layout when a piece omits `center`. */
const centersFromLayoutDiff = (layoutDiff?: DesignDiff): Map<string, { u: number; v: number }> => {
  const m = new Map<string, { u: number; v: number }>();
  for (const urow of layoutDiff?.pieces?.updated ?? []) {
    const row = urow as { piece?: { id?: string }; diff?: { center?: { u?: unknown; v?: unknown } } };
    const g = row.piece?.id;
    const c = row.diff?.center;
    if (g && c && typeof c.u === "number" && typeof c.v === "number") m.set(g, { u: c.u, v: c.v });
  }
  return m;
};

const cloneDesignApplyDiff = (d: Design, diff: DesignDiff): Design => {
  const plain = __plainFromDesign(d);
  __applyDesignDiffToPlain(plain, diff);
  return plain as Design;
};

/** Connection rows for a live {@link Design} or plain design-shaped objects (tests / MCP payloads). */
const snapshotDesignConnections = (design: Design): Connection[] => {
  const anyD = design as { getConnections?: () => Connection[] };
  if (typeof anyD.getConnections === "function") {
    return [...anyD.getConnections()];
  }
  const raw = (design as unknown as { connections?: unknown }).connections;
  return Array.isArray(raw) ? (raw as Connection[]) : [];
};

const buildDiagramSnapshot = (design: Design, padding: number, designDiff?: DesignDiff, layoutDiff?: DesignDiff): DiagramSnapshot => {
  const mergedRow = (designDiff ? previewDesignWithDiff(design, designDiff) : design) as PlainJsonObject;
  const layoutRow = (designDiff ? cloneDesignApplyDiff(design, designDiff) : design) as PlainJsonObject;
  const layoutCenters = centersFromLayoutDiff(layoutDiff);
  const layoutPieces = __itemsOf(layoutRow["pieces"]) as Piece[];
  const geometryCentersById = new Map(layoutPieces.filter((p): p is Piece & { id: string } => typeof p.id === "string" && p.id.length > 0).map((p) => [p.id, p.center] as const));

  const pointMap = new Map<string, DiagramPoint>();
  const mergedPieces = __itemsOf(mergedRow["pieces"]) as Piece[];
  mergedPieces.forEach((piece: Piece) => {
    if (!piece.id) return;
    const center = geometryCentersById.get(piece.id) ?? piece.center ?? layoutCenters.get(piece.id);
    if (!center || typeof (center as { u?: unknown }).u !== "number" || typeof (center as { v?: unknown }).v !== "number") return;
    const status: DiagramEntityStatus = designDiff ? getDiffStatusFromAttributes(piece.attributes) : "default";
    pointMap.set(piece.id, { id: piece.id, piece, u: (center as { u: number }).u, v: (center as { v: number }).v, status });
  });

  const pointsById = new Map(Array.from(pointMap.values()).map((point) => [point.id, point]));
  const lineMap = new Map<string, DiagramLine>();
  snapshotDesignConnections(mergedRow as unknown as Design).forEach((connection) => {
    if (!connection.id) return;
    const srcId = connection.connected?.piece?.id;
    const tgtId = connection.connecting?.piece?.id;
    if (!srcId || !tgtId) return;
    const source = pointsById.get(srcId);
    const target = pointsById.get(tgtId);
    if (!source || !target) return;
    const status: DiagramEntityStatus = designDiff ? getDiffStatusFromAttributes(connection.attributes) : "default";
    lineMap.set(connection.id, { id: connection.id, connection, source, target, status });
  });

  // One-hop propagation: a connection whose endpoint piece has non-default status becomes "modified".
  // This does NOT cascade back from connections to pieces — only pieces propagate to connections.
  if (designDiff) {
    for (const line of lineMap.values()) {
      if (line.status === "default") {
        const srcId = line.connection.connected?.piece?.id;
        const tgtId = line.connection.connecting?.piece?.id;
        if (!srcId || !tgtId) continue;
        const src = pointMap.get(srcId);
        const tgt = pointMap.get(tgtId);
        if ((src && src.status !== "default") || (tgt && tgt.status !== "default")) {
          (line as { status: DiagramEntityStatus }).status = "modified";
        }
      }
    }
  }

  const lines = Array.from(lineMap.values());

  const points = Array.from(pointMap.values());
  const minU = points.length > 0 ? Math.min(...points.map((point) => point.u)) : -0.5;
  const maxU = points.length > 0 ? Math.max(...points.map((point) => point.u)) : 0.5;
  const minY = points.length > 0 ? Math.min(...points.map((point) => -point.v)) : -0.5;
  const maxY = points.length > 0 ? Math.max(...points.map((point) => -point.v)) : 0.5;
  const width = Math.max(maxU - minU, MIN_DIAGRAM_SPAN);
  const height = Math.max(maxY - minY, MIN_DIAGRAM_SPAN);

  return { lines, points, minU, maxU, minY, maxY, width, height };
};

const buildDiagramBounds = (points: Array<{ u: number; v: number }>): DiagramBounds | null => {
  if (points.length === 0) return null;
  const minU = Math.min(...points.map((point) => point.u));
  const maxU = Math.max(...points.map((point) => point.u));
  const minY = Math.min(...points.map((point) => -point.v));
  const maxY = Math.max(...points.map((point) => -point.v));
  return {
    minU,
    maxU,
    minY,
    maxY,
    width: Math.max(maxU - minU, MIN_DIAGRAM_SPAN),
    height: Math.max(maxY - minY, MIN_DIAGRAM_SPAN),
  };
};

const isSelected = (id: string, idSet: Set<string>): boolean => idSet.has(id);

const useElementSize = <T extends HTMLElement>() => {
  const ref = React.useRef<T | null>(null);
  const [size, setSize] = React.useState({ width: 0, height: 0 });

  React.useLayoutEffect(() => {
    const element = ref.current;
    if (!element) return;
    const update = () => {
      setSize({
        width: element.clientWidth,
        height: element.clientHeight,
      });
    };
    update();
    const observer = new ResizeObserver(update);
    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  return { ref, size };
};

const normalizeSelection = (selection?: DiagramSelection): DiagramSelection => ({
  pieceIds: selection?.pieceIds ?? [],
  connectionIds: selection?.connectionIds ?? [],
});

const normalizeHover = (hover?: DiagramHover): DiagramHover => ({
  pieceId: hover?.pieceId ?? null,
  connectionId: hover?.connectionId ?? null,
});

export const SemioDiagram: React.FC<SemioDiagramProps> = ({
  design,
  designDiff,
  defaultDesignDiff,
  diffEnabled,
  zoomTarget,
  selection,
  defaultSelection,
  selectionEnabled,
  pieceSelectionEnabled = true,
  connectionSelectionEnabled = true,
  onSelectionChange,
  hover,
  defaultHover,
  hoverEnabled = true,
  pieceHoverEnabled = true,
  connectionHoverEnabled = true,
  onHoverChange,
  pan,
  defaultPan,
  panEnabled = true,
  onPanChange,
  zoom,
  defaultZoom,
  zoomEnabled = true,
  onZoomChange,
  className = "",
  padding = DEFAULT_DIAGRAM_PADDING,
  pieceRadius = DEFAULT_DIAGRAM_PIECE_RADIUS,
  strokeWidth = DEFAULT_DIAGRAM_STROKE_WIDTH,
  title = "Design Diagram",
  showOrigin = true,
  onPieceClick,
  onPieceDoubleClick,
  onConnectionClick,
  layoutDiff,
}) => {
  const effectiveDiffEnabled = diffEnabled ?? true;
  const effectiveSelectionEnabled = selectionEnabled ?? true;
  const effectivePieceSelectionEnabled = effectiveSelectionEnabled && pieceSelectionEnabled;
  const effectiveConnectionSelectionEnabled = effectiveSelectionEnabled && connectionSelectionEnabled;
  const effectiveHoverEnabled = hoverEnabled ?? true;
  const effectivePieceHoverEnabled = effectiveHoverEnabled && pieceHoverEnabled && (effectivePieceSelectionEnabled || !!onPieceClick || !!onPieceDoubleClick);
  const effectiveConnectionHoverEnabled = effectiveHoverEnabled && connectionHoverEnabled && (effectiveConnectionSelectionEnabled || !!onConnectionClick);
  const resolvedDesignDiff = useResolvedValue(designDiff, defaultDesignDiff);
  const [resolvedSelection, setResolvedSelection] = useInteractiveControllableValue(selection, normalizeSelection(defaultSelection), onSelectionChange);
  const [resolvedHover, setResolvedHover] = useInteractiveControllableValue(hover, normalizeHover(defaultHover), onHoverChange);
  const [resolvedPan, setResolvedPan, isPanControlled] = useInteractiveControllableValue(pan, defaultPan ?? { x: 0, y: 0 }, onPanChange);
  const [resolvedZoom, setResolvedZoom, isZoomControlled] = useInteractiveControllableValue(zoom, defaultZoom ?? DEFAULT_DIAGRAM_ZOOM, onZoomChange);
  const snapshot = React.useMemo(() => buildDiagramSnapshot(design, padding, effectiveDiffEnabled ? resolvedDesignDiff : undefined, layoutDiff), [design, effectiveDiffEnabled, layoutDiff, padding, resolvedDesignDiff]);
  const selectedPieceIds = React.useMemo(() => new Set(effectiveSelectionEnabled ? (resolvedSelection.pieceIds ?? []) : []), [effectiveSelectionEnabled, resolvedSelection.pieceIds]);
  const selectedConnectionIds = React.useMemo(() => new Set(effectiveSelectionEnabled ? (resolvedSelection.connectionIds ?? []) : []), [effectiveSelectionEnabled, resolvedSelection.connectionIds]);
  const hoveredPieceId = effectivePieceHoverEnabled ? (resolvedHover.pieceId ?? null) : null;
  const hoveredConnectionId = effectiveConnectionHoverEnabled ? (resolvedHover.connectionId ?? null) : null;
  const { ref, size } = useElementSize<HTMLDivElement>();
  useDesignClipboard(ref, design, effectiveDiffEnabled ? resolvedDesignDiff : undefined, resolvedSelection);
  const panPointerIdRef = React.useRef<number | null>(null);
  const panOriginRef = React.useRef({ x: 0, y: 0, panX: 0, panY: 0 });
  const didPanDragRef = React.useRef(false);
  const [isPanning, setIsPanning] = React.useState(false);
  const innerPadding = padding;
  const drawableWidth = Math.max(size.width - innerPadding * 2, 1);
  const drawableHeight = Math.max(size.height - innerPadding * 2, 1);
  const scale = Math.min(drawableWidth / snapshot.width, drawableHeight / snapshot.height);
  const offsetX = (size.width - snapshot.width * scale) / 2;
  const offsetY = (size.height - snapshot.height * scale) / 2;
  const centerX = size.width / 2;
  const centerY = size.height / 2;
  const toBasePixelX = (u: number) => offsetX + (u - snapshot.minU) * scale;
  const toBasePixelY = (y: number) => offsetY + (y - snapshot.minY) * scale;
  const effectiveZoomTarget: ZoomTarget = zoomTarget ?? (effectiveDiffEnabled && resolvedDesignDiff ? "diff" : "design");
  const fittedViewport = React.useMemo(() => {
    if (effectiveZoomTarget === "none") return { zoom: defaultZoom ?? DEFAULT_DIAGRAM_ZOOM, pan: defaultPan ?? { x: 0, y: 0 } };
    const diffPoints = snapshot.points.filter((point) => point.status !== "default");
    const diffLinePoints = snapshot.lines.filter((line) => line.status !== "default").flatMap((line) => [line.source, line.target]);
    const hasDiffEntities = diffPoints.length > 0 || diffLinePoints.length > 0;
    const targetBounds = (effectiveZoomTarget === "diff" && hasDiffEntities ? buildDiagramBounds([...diffPoints, ...diffLinePoints]) : null) ?? {
      minU: snapshot.minU,
      maxU: snapshot.maxU,
      minY: snapshot.minY,
      maxY: snapshot.maxY,
      width: snapshot.width,
      height: snapshot.height,
    };
    const localToBasePixelX = (u: number) => offsetX + (u - snapshot.minU) * scale;
    const localToBasePixelY = (y: number) => offsetY + (y - snapshot.minY) * scale;
    const targetMinX = localToBasePixelX(targetBounds.minU);
    const targetMaxX = localToBasePixelX(targetBounds.maxU);
    const targetMinY = localToBasePixelY(targetBounds.minY);
    const targetMaxY = localToBasePixelY(targetBounds.maxY);
    const targetWidth = Math.max(targetMaxX - targetMinX, 1);
    const targetHeight = Math.max(targetMaxY - targetMinY, 1);
    const targetCenterX = (targetMinX + targetMaxX) / 2;
    const targetCenterY = (targetMinY + targetMaxY) / 2;
    const zoomToFit = Math.min(MAX_DIAGRAM_ZOOM, Math.max(MIN_DIAGRAM_ZOOM, Math.min(drawableWidth / targetWidth, drawableHeight / targetHeight)));

    return {
      zoom: defaultZoom ?? zoomToFit,
      pan: defaultPan ?? {
        x: -zoomToFit * (targetCenterX - centerX),
        y: -zoomToFit * (targetCenterY - centerY),
      },
    };
  }, [centerX, centerY, defaultPan, defaultZoom, drawableHeight, drawableWidth, effectiveZoomTarget, offsetX, offsetY, scale, snapshot]);
  const applyViewportX = (x: number) => centerX + resolvedPan.x + resolvedZoom * (x - centerX);
  const applyViewportY = (y: number) => centerY + resolvedPan.y + resolvedZoom * (y - centerY);
  const toPixelX = (u: number) => applyViewportX(toBasePixelX(u));
  const toPixelY = (y: number) => applyViewportY(toBasePixelY(y));
  const pxPerDiagramUnit = scale * resolvedZoom;

  const diagramSelectionOverlay = React.useMemo(() => {
    if (!effectiveSelectionEnabled) return null;
    const piecePxPerUnit = scale * resolvedZoom;
    const selectedStrokePx = Math.max(1, 0.1 * piecePxPerUnit);
    // Match selected node: r=(pieceRadius+DIAGRAM_PIECE_SELECTED_RADIUS_EXTRA)*pxPerUnit, stroke ~0.1*px → outer r+stroke/2.
    const selectedNodeOuterRadiusPx = (pieceRadius + DIAGRAM_PIECE_SELECTED_RADIUS_EXTRA) * piecePxPerUnit + selectedStrokePx / 2;
    const piecePadPx = selectedNodeOuterRadiusPx + 8;
    const selectedEdgeHalfThicknessPx = ((strokeWidth + 1.5) * resolvedZoom) / 2;
    const connectionPadPx = selectedEdgeHalfThicknessPx + 8;
    const vpX = (x: number) => centerX + resolvedPan.x + resolvedZoom * (x - centerX);
    const vpY = (y: number) => centerY + resolvedPan.y + resolvedZoom * (y - centerY);
    const toPxX = (u: number) => vpX(offsetX + (u - snapshot.minU) * scale);
    const toPxY = (diagramY: number) => vpY(offsetY + (diagramY - snapshot.minY) * scale);
    return computeDiagramSelectionOverlayRect(snapshot, selectedPieceIds, selectedConnectionIds, toPxX, toPxY, piecePadPx, connectionPadPx);
  }, [centerX, centerY, effectiveSelectionEnabled, offsetX, offsetY, resolvedPan.x, resolvedPan.y, resolvedZoom, scale, selectedConnectionIds, selectedPieceIds, snapshot, pieceRadius, strokeWidth]);

  const fittedPanX = fittedViewport.pan.x;
  const fittedPanY = fittedViewport.pan.y;
  const fittedZoom = fittedViewport.zoom;

  React.useEffect(() => {
    if (!isZoomControlled) {
      setResolvedZoom(fittedZoom);
    }
    if (!isPanControlled) {
      setResolvedPan({ x: fittedPanX, y: fittedPanY });
    }
  }, [fittedPanX, fittedPanY, fittedZoom, isPanControlled, isZoomControlled, setResolvedPan, setResolvedZoom]);

  const handleWheel = React.useCallback(
    (event: WheelEvent) => {
      if (!zoomEnabled) return;
      event.preventDefault();
      if (size.width <= 0 || size.height <= 0 || !ref.current) return;
      const nextZoom = Math.min(MAX_DIAGRAM_ZOOM, Math.max(MIN_DIAGRAM_ZOOM, resolvedZoom * Math.exp(-event.deltaY * DIAGRAM_ZOOM_STEP)));
      if (Math.abs(nextZoom - resolvedZoom) < 0.0001) return;
      const rect = ref.current.getBoundingClientRect();
      const cursorX = event.clientX - rect.left;
      const cursorY = event.clientY - rect.top;
      const baseX = centerX + (cursorX - centerX - resolvedPan.x) / resolvedZoom;
      const baseY = centerY + (cursorY - centerY - resolvedPan.y) / resolvedZoom;
      setResolvedZoom(nextZoom);
      setResolvedPan({
        x: cursorX - centerX - nextZoom * (baseX - centerX),
        y: cursorY - centerY - nextZoom * (baseY - centerY),
      });
    },
    [centerX, centerY, resolvedPan.x, resolvedPan.y, resolvedZoom, setResolvedPan, setResolvedZoom, size.height, size.width, zoomEnabled, ref],
  );

  React.useEffect(() => {
    const element = ref.current;
    if (!element) return;
    element.addEventListener("wheel", handleWheel, { passive: false });
    return () => element.removeEventListener("wheel", handleWheel);
  }, [handleWheel, ref]);

  const handleDoubleClick = React.useCallback(() => {
    if (!zoomEnabled && !panEnabled) return;
    if (zoomEnabled) {
      setResolvedZoom(fittedViewport.zoom);
    }
    if (panEnabled) {
      setResolvedPan(fittedViewport.pan);
    }
  }, [fittedViewport.pan, fittedViewport.zoom, panEnabled, setResolvedPan, setResolvedZoom, zoomEnabled]);

  const handlePointerDown = React.useCallback(
    (event: React.PointerEvent<SVGSVGElement>) => {
      if (!panEnabled) return;
      if (event.button !== 0) return;
      if (event.target !== event.currentTarget) return;
      panPointerIdRef.current = event.pointerId;
      panOriginRef.current = {
        x: event.clientX,
        y: event.clientY,
        panX: resolvedPan.x,
        panY: resolvedPan.y,
      };
      didPanDragRef.current = false;
      setIsPanning(true);
      event.currentTarget.setPointerCapture(event.pointerId);
    },
    [panEnabled, resolvedPan.x, resolvedPan.y],
  );

  const clearSelection = React.useCallback(() => {
    if (!effectiveSelectionEnabled) return;
    setResolvedSelection({
      pieceIds: [],
      connectionIds: [],
    });
  }, [effectiveSelectionEnabled, setResolvedSelection]);

  const setHoveredPiece = React.useCallback(
    (pieceId: string | null) => {
      if (!effectivePieceHoverEnabled) return;
      setResolvedHover({
        pieceId,
        connectionId: resolvedHover.connectionId ?? null,
      });
    },
    [effectivePieceHoverEnabled, resolvedHover.connectionId, setResolvedHover],
  );

  const setHoveredConnection = React.useCallback(
    (connectionId: string | null) => {
      if (!effectiveConnectionHoverEnabled) return;
      setResolvedHover({
        pieceId: resolvedHover.pieceId ?? null,
        connectionId,
      });
    },
    [effectiveConnectionHoverEnabled, resolvedHover.pieceId, setResolvedHover],
  );

  const handleSvgClick = React.useCallback(
    (event: React.MouseEvent<SVGSVGElement>) => {
      // Suppress deselection after a pan drag.
      if (didPanDragRef.current) {
        didPanDragRef.current = false;
        return;
      }
      // Only clear selection if clicking on the SVG background (not on child elements)
      if (event.target === event.currentTarget) {
        clearSelection();
      }
    },
    [clearSelection],
  );

  const handlePointerMove = React.useCallback(
    (event: React.PointerEvent<SVGSVGElement>) => {
      if (panPointerIdRef.current !== event.pointerId) return;
      const deltaX = event.clientX - panOriginRef.current.x;
      const deltaY = event.clientY - panOriginRef.current.y;
      if (Math.abs(deltaX) > 2 || Math.abs(deltaY) > 2) {
        didPanDragRef.current = true;
      }
      setResolvedPan({
        x: panOriginRef.current.panX + deltaX,
        y: panOriginRef.current.panY + deltaY,
      });
    },
    [setResolvedPan],
  );

  const handlePointerEnd = React.useCallback(
    (event: React.PointerEvent<SVGSVGElement>) => {
      if (panPointerIdRef.current !== event.pointerId) return;
      panPointerIdRef.current = null;
      setIsPanning(false);
      if (event.currentTarget.hasPointerCapture(event.pointerId)) {
        event.currentTarget.releasePointerCapture(event.pointerId);
      }
      // Deselect on background click (no drag). Pointer capture may suppress the
      // click event so we handle deselection here for that case. Do NOT reset
      // didPanDragRef here — handleSvgClick resets it so it can suppress the
      // click event that some browsers still fire after a drag + pointer capture.
      if (!didPanDragRef.current) {
        clearSelection();
      }
    },
    [clearSelection],
  );

  const selectPiece = React.useCallback(
    (pieceId: string) => {
      if (!effectivePieceSelectionEnabled) return;
      const nextPieceIds = new Set(resolvedSelection.pieceIds ?? []);
      if (nextPieceIds.has(pieceId)) {
        nextPieceIds.delete(pieceId);
      } else {
        nextPieceIds.add(pieceId);
      }
      setResolvedSelection({
        pieceIds: Array.from(nextPieceIds),
        connectionIds: resolvedSelection.connectionIds ?? [],
      });
    },
    [effectivePieceSelectionEnabled, resolvedSelection.connectionIds, resolvedSelection.pieceIds, setResolvedSelection],
  );

  const selectConnection = React.useCallback(
    (connectionId: string) => {
      if (!effectiveConnectionSelectionEnabled) return;
      const nextConnectionIds = new Set(resolvedSelection.connectionIds ?? []);
      if (nextConnectionIds.has(connectionId)) {
        nextConnectionIds.delete(connectionId);
      } else {
        nextConnectionIds.add(connectionId);
      }
      setResolvedSelection({
        pieceIds: resolvedSelection.pieceIds ?? [],
        connectionIds: Array.from(nextConnectionIds),
      });
    },
    [effectiveConnectionSelectionEnabled, resolvedSelection.connectionIds, resolvedSelection.pieceIds, setResolvedSelection],
  );

  const { x: originOx, y: originOy } = resolveSemioDiagramOriginPixels(toPixelX, toPixelY);
  const { uTip, vTip } = resolveSemioDiagramUnitAxisTips(toPixelX, toPixelY);
  const originStrokePx = showOrigin ? Math.max(1, 0.85 * resolvedZoom) : 0;
  const originLabelPx = showOrigin ? Math.max(10, Math.min(18, 10 + resolvedZoom)) : 0;
  const originLabelOff = showOrigin ? Math.max(3, 4 * Math.min(resolvedZoom, 2)) : 0;
  const uDirLen = Math.hypot(uTip.x - originOx, uTip.y - originOy) || 1;
  const uNx = (uTip.x - originOx) / uDirLen;
  const uNy = (uTip.y - originOy) / uDirLen;
  const vDirLen = Math.hypot(vTip.x - originOx, vTip.y - originOy) || 1;
  const vNx = (vTip.x - originOx) / vDirLen;
  const vNy = (vTip.y - originOy) / vDirLen;

  return (
    <div ref={ref} className={`h-full w-full ${className}`} onDoubleClick={handleDoubleClick} tabIndex={0} style={{ outline: "none", position: "relative" }}>
      <svg
        aria-label={title}
        className="h-full w-full overflow-visible text-foreground"
        onPointerCancel={handlePointerEnd}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerEnd}
        role="img"
        style={{ cursor: panEnabled ? (isPanning ? "grabbing" : "grab") : "default", touchAction: panEnabled ? "none" : "auto" }}
        onClick={handleSvgClick}
      >
        {diagramSelectionOverlay ? (
          <rect
            fill="var(--accent)"
            fillOpacity={SEMIO_SELECTION_BOUNDS_FILL_OPACITY}
            height={diagramSelectionOverlay.height}
            pointerEvents="none"
            stroke="var(--accent)"
            strokeOpacity={SEMIO_SELECTION_BOUNDS_STROKE_OPACITY}
            strokeWidth={Math.max(1, 1.25 * resolvedZoom)}
            width={diagramSelectionOverlay.width}
            x={diagramSelectionOverlay.x}
            y={diagramSelectionOverlay.y}
          />
        ) : null}
        {showOrigin ? (
          <g aria-hidden="true" className="text-muted-foreground" pointerEvents="none">
            <circle cx={originOx} cy={originOy} fill="currentColor" fillOpacity={0.5} r={Math.max(1.25, 1.75 * resolvedZoom)} />
            <line stroke="currentColor" strokeOpacity={0.65} strokeWidth={originStrokePx} x1={originOx} x2={uTip.x} y1={originOy} y2={uTip.y} />
            <line stroke="currentColor" strokeOpacity={0.65} strokeWidth={originStrokePx} x1={originOx} x2={vTip.x} y1={originOy} y2={vTip.y} />
            <text dominantBaseline="middle" fill="currentColor" fillOpacity={0.88} fontSize={originLabelPx} textAnchor="middle" x={uTip.x + uNx * originLabelOff} y={uTip.y + uNy * originLabelOff}>
              u
            </text>
            <text dominantBaseline="middle" fill="currentColor" fillOpacity={0.88} fontSize={originLabelPx} textAnchor="middle" x={vTip.x + vNx * originLabelOff} y={vTip.y + vNy * originLabelOff}>
              v
            </text>
          </g>
        ) : null}
        {snapshot.lines.map((line) => {
          const selected = isSelected(line.id, selectedConnectionIds);
          const hovered = hoveredConnectionId === line.id;
          return (
            <line
              key={line.id}
              onClick={
                onConnectionClick || effectiveConnectionSelectionEnabled
                  ? (event) => {
                      event.stopPropagation();
                      selectConnection(line.id);
                      onConnectionClick?.(line.connection);
                    }
                  : undefined
              }
              pointerEvents="stroke"
              stroke={getInteractiveEntityColor(line.status, selected, hovered)}
              strokeLinecap="round"
              strokeOpacity={selected || hovered ? 1 : line.status === "default" ? 0.45 : 0.8}
              strokeWidth={(selected ? strokeWidth + 1.5 : hovered ? strokeWidth + 0.75 : strokeWidth) * resolvedZoom}
              style={{ cursor: onConnectionClick || effectiveConnectionSelectionEnabled ? "pointer" : "default" }}
              onPointerEnter={effectiveConnectionHoverEnabled ? () => setHoveredConnection(line.id) : undefined}
              onPointerLeave={effectiveConnectionHoverEnabled ? () => setHoveredConnection((resolvedHover.connectionId ?? null) === line.id ? null : (resolvedHover.connectionId ?? null)) : undefined}
              x1={toPixelX(line.source.u)}
              x2={toPixelX(line.target.u)}
              y1={toPixelY(-line.source.v)}
              y2={toPixelY(-line.target.v)}
            />
          );
        })}
        {snapshot.points.map((point) => {
          const selected = isSelected(point.id, selectedPieceIds);
          const hovered = hoveredPieceId === point.id;
          const pieceInteractive = Boolean(onPieceClick || onPieceDoubleClick || effectivePieceSelectionEnabled);
          return (
            <circle
              key={point.id}
              cx={toPixelX(point.u)}
              cy={toPixelY(-point.v)}
              fill={getEntityStatusColor(point.status)}
              onClick={
                onPieceClick || effectivePieceSelectionEnabled
                  ? (event) => {
                      event.stopPropagation();
                      selectPiece(point.id);
                      onPieceClick?.(point.piece);
                    }
                  : undefined
              }
              onDoubleClick={
                onPieceDoubleClick
                  ? (event) => {
                      event.stopPropagation();
                      event.preventDefault();
                      onPieceDoubleClick(point.piece);
                    }
                  : undefined
              }
              onPointerEnter={effectivePieceHoverEnabled ? () => setHoveredPiece(point.id) : undefined}
              onPointerLeave={effectivePieceHoverEnabled ? () => setHoveredPiece((resolvedHover.pieceId ?? null) === point.id ? null : (resolvedHover.pieceId ?? null)) : undefined}
              r={(selected ? pieceRadius + DIAGRAM_PIECE_SELECTED_RADIUS_EXTRA : hovered ? pieceRadius + DIAGRAM_PIECE_HOVER_RADIUS_EXTRA : pieceRadius) * pxPerDiagramUnit}
              stroke={selected || hovered ? getInteractiveEntityColor(point.status, selected, hovered) : "none"}
              strokeWidth={selected ? Math.max(1, 0.1 * pxPerDiagramUnit) : hovered ? Math.max(1, 0.06 * pxPerDiagramUnit) : 0}
              style={{ cursor: pieceInteractive ? "pointer" : "default" }}
            />
          );
        })}
      </svg>
    </div>
  );
};

// #endregion 🧫Diagram

// #region 📩PieceSelection

/**
 * ⚙️PieceSelection is a constrained Diagram configuration that only supports selecting pieces.
 *
 * Specs:
 * - Connection selection is always disabled (no connection hover/click selection state).
 * - Selection callbacks only return `pieceIds`.
 */
export interface PieceSelectionState {
  pieceIds?: string[];
}

export interface PieceSelectionProps extends Omit<SemioDiagramProps, "pieceSelectionEnabled" | "connectionSelectionEnabled" | "onConnectionClick" | "selection" | "defaultSelection" | "onSelectionChange"> {
  selection?: PieceSelectionState;
  defaultSelection?: PieceSelectionState;
  onSelectionChange?: (selection: PieceSelectionState) => void;
}

export const PieceSelection: React.FC<PieceSelectionProps> = ({ selection, defaultSelection, onSelectionChange, ...rest }) => {
  const mappedSelection = selection ? { pieceIds: selection.pieceIds ?? [], connectionIds: [] } : undefined;
  const mappedDefaultSelection = defaultSelection ? { pieceIds: defaultSelection.pieceIds ?? [], connectionIds: [] } : undefined;

  return (
    <SemioDiagram
      {...rest}
      pieceSelectionEnabled={true}
      connectionSelectionEnabled={false}
      selection={mappedSelection}
      defaultSelection={mappedDefaultSelection}
      onSelectionChange={
        onSelectionChange
          ? (next) => {
              onSelectionChange({ pieceIds: next.pieceIds ?? [] });
            }
          : undefined
      }
    />
  );
};

// #endregion 📩PieceSelection

// #region 🔧ConnectionSelection
// Constrained Diagram wrapper that only supports selecting connections.

export interface ConnectionSelectionState {
  connectionIds?: string[];
}

export interface ConnectionSelectionProps extends Omit<SemioDiagramProps, "pieceSelectionEnabled" | "connectionSelectionEnabled" | "onPieceClick" | "selection" | "defaultSelection" | "onSelectionChange"> {
  selection?: ConnectionSelectionState;
  defaultSelection?: ConnectionSelectionState;
  onSelectionChange?: (selection: ConnectionSelectionState) => void;
}

export const ConnectionSelection: React.FC<ConnectionSelectionProps> = ({ selection, defaultSelection, onSelectionChange, ...rest }) => {
  const mappedSelection = selection ? { pieceIds: [], connectionIds: selection.connectionIds ?? [] } : undefined;
  const mappedDefaultSelection = defaultSelection ? { pieceIds: [], connectionIds: defaultSelection.connectionIds ?? [] } : undefined;

  return (
    <SemioDiagram
      {...rest}
      pieceSelectionEnabled={false}
      connectionSelectionEnabled={true}
      selection={mappedSelection}
      defaultSelection={mappedDefaultSelection}
      onSelectionChange={
        onSelectionChange
          ? (next) => {
              onSelectionChange({ connectionIds: next.connectionIds ?? [] });
            }
          : undefined
      }
    />
  );
};

// #endregion 🔧ConnectionSelection

// #region 🎵Selection

/**
 * 🔌DiagramSelection is a constrained Diagram wrapper that supports selecting both pieces and connections.
 */
export interface DiagramSelectionState {
  pieceIds?: string[];
  connectionIds?: string[];
}

export interface DiagramSelectionProps extends Omit<SemioDiagramProps, "pieceSelectionEnabled" | "connectionSelectionEnabled" | "selection" | "defaultSelection" | "onSelectionChange"> {
  selection?: DiagramSelectionState;
  defaultSelection?: DiagramSelectionState;
  onSelectionChange?: (selection: DiagramSelectionState) => void;
}

export const DiagramSelection: React.FC<DiagramSelectionProps> = ({ selection, defaultSelection, onSelectionChange, ...rest }) => {
  const mappedSelection = selection ? { pieceIds: selection.pieceIds ?? [], connectionIds: selection.connectionIds ?? [] } : undefined;
  const mappedDefaultSelection = defaultSelection ? { pieceIds: defaultSelection.pieceIds ?? [], connectionIds: defaultSelection.connectionIds ?? [] } : undefined;

  return <SemioDiagram {...rest} pieceSelectionEnabled={true} connectionSelectionEnabled={true} selection={mappedSelection} defaultSelection={mappedDefaultSelection} onSelectionChange={onSelectionChange} />;
};

// #endregion 🎵Selection

// #region ⚗️Clipboard
// Specs: Pure logic to compute clipboard data from a design, optional diff, and optional selection.
// When no diff and no selection: copy the full design.
// When no diff and selection present: copy only selected pieces and connections.
// When diff present and no selection: copy the full diff.
// When diff and selection present: copy only the selected parts of the diff.
// Summary: Computes design clipboard data based on diff and selection state, with Ctrl+C hook.

export interface DesignClipboardData {
  design?: Design;
  designDiff?: DesignDiff;
}

export const buildDesignClipboardData = (design: Design, designDiff: DesignDiff | undefined, selection: DiagramSelection | undefined): DesignClipboardData => {
  const selectedPieceIds = new Set(selection?.pieceIds ?? []);
  const selectedConnectionIds = new Set(selection?.connectionIds ?? []);
  const hasSelection = selectedPieceIds.size > 0 || selectedConnectionIds.size > 0;
  const hasDiff = designDiff !== undefined;

  if (!hasDiff && !hasSelection) {
    // No diff, no selection: copy the full design
    return { design };
  }

  if (!hasDiff && hasSelection) {
    // 🔌No diff, selection present: copy selected pieces and connections
    const row = design as PlainJsonObject;
    const pieceRows = __itemsOf(row["pieces"]) as Piece[];
    const pieces = pieceRows.filter((p) => p.id && selectedPieceIds.has(p.id));
    const connections = snapshotDesignConnections(design).filter((c) => c.id && selectedConnectionIds.has(c.id));
    return {
      design: { ...row, pieces: pieces.length > 0 ? pieces : undefined, connections: connections.length > 0 ? connections : undefined } as unknown as Design,
    };
  }

  if (hasDiff && !hasSelection) {
    // Diff present, no selection: copy the full diff
    return { design, designDiff };
  }

  // 🧹Diff and selection present: copy only selected parts of the diff
  const filteredDiff: DesignDiff = {
    pieces: designDiff!.pieces
      ? {
          added: (designDiff!.pieces.added ?? []).filter((p) => (p as Piece).id !== undefined && selectedPieceIds.has(String((p as Piece).id))),
          removed: (designDiff!.pieces.removed ?? []).filter((p) => (p as Piece).id !== undefined && selectedPieceIds.has(String((p as Piece).id))),
          updated: (designDiff!.pieces.updated ?? []).filter((u: unknown) => {
            const row = u as { piece?: { id?: string } };
            return row.piece?.id !== undefined && selectedPieceIds.has(String(row.piece.id));
          }),
        }
      : undefined,
    connections: designDiff!.connections
      ? {
          added: (designDiff!.connections.added ?? []).filter((c: unknown) => {
            const row = c as { id?: string };
            return row.id !== undefined && selectedConnectionIds.has(String(row.id));
          }),
          removed: (designDiff!.connections.removed ?? []).filter((c: unknown) => {
            const row = c as { id?: string };
            return row.id !== undefined && selectedConnectionIds.has(String(row.id));
          }),
          updated: (designDiff!.connections.updated ?? []).filter((u: unknown) => {
            const row = u as { connection?: { id?: string } };
            return row.connection?.id !== undefined && selectedConnectionIds.has(String(row.connection.id));
          }),
        }
      : undefined,
  };
  return { design, designDiff: filteredDiff };
};

const serializeClipboardData = (data: DesignClipboardData): string => JSON.stringify(data);

const useDesignClipboard = (containerRef: React.RefObject<HTMLElement | null>, design: Design, designDiff: DesignDiff | undefined, selection: DiagramSelection | undefined): void => {
  React.useEffect(() => {
    const element = containerRef.current;
    if (!element) return;
    const handler = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && event.key === "c") {
        // 📋Don't override native text selection copy
        const windowSelection = window.getSelection();
        if (windowSelection && windowSelection.toString().length > 0) return;
        event.preventDefault();
        const data = buildDesignClipboardData(design, designDiff, selection);
        navigator.clipboard.writeText(serializeClipboardData(data));
      }
    };
    element.addEventListener("keydown", handler);
    return () => element.removeEventListener("keydown", handler);
  }, [containerRef, design, designDiff, selection]);
};

// #endregion ⚗️Clipboard

// #region 🏪Vec

// Specs: SVG 2D vector input with draggable handle, visible origin and axes.
// Summary: Draggable XY pad mapping pointer position to a {u,v} vector in a bounded domain.

export interface VecValue {
  u: number;
  v: number;
}

export interface VecProps {
  id: string;
  vec: VecValue;
  minU?: number;
  maxU?: number;
  minV?: number;
  maxV?: number;
  showAxes?: boolean;
  showOrigin?: boolean;
  size?: number;
  onVecChange?: (vec: VecValue) => void;
  className?: string;
}

const vecClamp = (val: number, lo: number, hi: number) => Math.min(hi, Math.max(lo, val));

/**
 * Vec displays a 2D vector input as an SVG pad with a draggable handle.
 * The U axis points right, V axis points up. Origin and axes are optionally visible.
 **/
export const Vec: React.FC<VecProps> = ({ id, vec, minU = -1, maxU = 1, minV = -1, maxV = 1, showAxes = true, showOrigin = true, size = 120, onVecChange, className = "" }) => {
  const svgRef = React.useRef<SVGSVGElement>(null);
  const [dragging, setDragging] = React.useState(false);
  const [localVec, setLocalVec] = React.useState<VecValue | null>(null);
  const rafId = React.useRef<number>(0);
  const pendingVec = React.useRef<VecValue | null>(null);
  const pad = 8;
  const inner = size - pad * 2;

  const vecFromEvent = React.useCallback(
    (e: React.PointerEvent | PointerEvent): VecValue => {
      if (!svgRef.current) return { u: 0, v: 0 };
      const rect = svgRef.current.getBoundingClientRect();
      const px = e.clientX - rect.left - pad;
      const py = e.clientY - rect.top - pad;
      const u = vecClamp(minU + (px / inner) * (maxU - minU), minU, maxU);
      const v = vecClamp(maxV - (py / inner) * (maxV - minV), minV, maxV);
      return { u, v };
    },
    [inner, minU, maxU, minV, maxV],
  );

  const toSvgX = (u: number) => pad + ((u - minU) / (maxU - minU)) * inner;
  const toSvgY = (v: number) => pad + ((maxV - v) / (maxV - minV)) * inner;

  const flushPending = React.useCallback(() => {
    if (pendingVec.current !== null) {
      onVecChange?.(pendingVec.current);
      pendingVec.current = null;
    }
  }, [onVecChange]);

  const handlePointerDown = React.useCallback(
    (e: React.PointerEvent<SVGSVGElement>) => {
      e.preventDefault();
      svgRef.current?.setPointerCapture(e.pointerId);
      const v = vecFromEvent(e);
      setDragging(true);
      setLocalVec(v);
      pendingVec.current = null;
      onVecChange?.(v);
    },
    [vecFromEvent, onVecChange],
  );

  const handlePointerMove = React.useCallback(
    (e: React.PointerEvent<SVGSVGElement>) => {
      if (!dragging) return;
      const v = vecFromEvent(e);
      setLocalVec(v);
      pendingVec.current = v;
      if (!rafId.current) {
        rafId.current = requestAnimationFrame(() => {
          rafId.current = 0;
          flushPending();
        });
      }
    },
    [dragging, vecFromEvent, flushPending],
  );

  const handlePointerUp = React.useCallback(
    (e: React.PointerEvent<SVGSVGElement>) => {
      if (!dragging) return;
      if (rafId.current) {
        cancelAnimationFrame(rafId.current);
        rafId.current = 0;
      }
      const v = vecFromEvent(e);
      setLocalVec(null);
      setDragging(false);
      onVecChange?.(v);
    },
    [dragging, vecFromEvent, onVecChange],
  );

  const handlePointerCancel = React.useCallback(() => {
    if (!dragging) return;
    if (rafId.current) {
      cancelAnimationFrame(rafId.current);
      rafId.current = 0;
    }
    setLocalVec(null);
    setDragging(false);
  }, [dragging]);

  React.useEffect(() => {
    return () => {
      if (rafId.current) cancelAnimationFrame(rafId.current);
    };
  }, []);

  const displayVec = localVec ?? vec;
  const handleX = toSvgX(displayVec.u);
  const handleY = toSvgY(displayVec.v);
  const originX = toSvgX(0);
  const originY = toSvgY(0);
  const originInBounds = minU <= 0 && maxU >= 0 && minV <= 0 && maxV >= 0;

  return (
    <svg
      ref={svgRef}
      data-slot="vec"
      id={id}
      width={size}
      height={size}
      viewBox={`0 0 ${size} ${size}`}
      className={`touch-none select-none ${className}`}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerCancel}
    >
      <rect x={pad} y={pad} width={inner} height={inner} rx={2} className="fill-muted/40 stroke-muted-foreground/20" strokeWidth={0.5} />
      {showAxes && originInBounds && (
        <>
          <line x1={pad} y1={originY} x2={pad + inner} y2={originY} className="stroke-muted-foreground/40" strokeWidth={0.5} strokeDasharray="2 2" />
          <line x1={originX} y1={pad} x2={originX} y2={pad + inner} className="stroke-muted-foreground/40" strokeWidth={0.5} strokeDasharray="2 2" />
        </>
      )}
      {showOrigin && originInBounds && <circle cx={originX} cy={originY} r={2} className="fill-muted-foreground/60" />}
      {originInBounds && <line x1={originX} y1={originY} x2={handleX} y2={handleY} className="stroke-foreground/50" strokeWidth={1} />}
      <circle data-slot="vec-handle" cx={handleX} cy={handleY} r={dragging ? 6 : 5} className={`fill-foreground cursor-grab active:cursor-grabbing ${dragging ? "" : "transition-all duration-150"}`} />
    </svg>
  );
};

// #endregion 🏪Vec

// #region 🔤Vector

// Specs: Semio 3D vector component supporting display/select modes with partial/full
// controlled/uncontrolled behavior. Supports per-axis enable flags for partial selection.
// Summary: 3D vector editor/viewer with semio Vector (x,y,z) and per-axis controllable state.

export type VectorValue = { x: number; y: number; z: number };

export interface VectorProps {
  id: string;
  vector?: VectorValue;
  defaultVector?: VectorValue;
  onVectorChange?: (vector: VectorValue) => void;

  x?: number;
  defaultX?: number;
  onXChange?: (x: number) => void;
  y?: number;
  defaultY?: number;
  onYChange?: (y: number) => void;
  z?: number;
  defaultZ?: number;
  onZChange?: (z: number) => void;

  selectionEnabled?: boolean;
  xSelectionEnabled?: boolean;
  ySelectionEnabled?: boolean;
  zSelectionEnabled?: boolean;
  displayEnabled?: boolean;
  xDisplayEnabled?: boolean;
  yDisplayEnabled?: boolean;
  zDisplayEnabled?: boolean;

  minX?: number;
  maxX?: number;
  minY?: number;
  maxY?: number;
  minZ?: number;
  maxZ?: number;
  step?: number;
  /** Overrides default X/Y/Z labels (e.g. shift/gap/rise for placement vectors). */
  axisLabels?: Partial<Record<"x" | "y" | "z", string>>;
  className?: string;
}

const normalizeVector = (vector?: VectorValue): VectorValue => ({ x: vector?.x ?? 0, y: vector?.y ?? 0, z: vector?.z ?? 0 });
const clampVectorAxis = (value: number, min: number, max: number): number => Math.min(max, Math.max(min, value));

const VectorPreview3D: React.FC<{ vector: VectorValue }> = ({ vector }) => {
  const target = React.useMemo(() => new THREE.Vector3(vector.x, vector.y, vector.z), [vector.x, vector.y, vector.z]);
  const length = Math.max(0.0001, target.length());
  const dir = React.useMemo(() => target.clone().normalize(), [target]);
  const midpoint = React.useMemo(() => target.clone().multiplyScalar(0.5), [target]);
  const sphereSize = Math.max(0.03, Math.min(0.08, length * 0.1));

  return (
    <ThreeCanvas orthographic camera={{ zoom: 75, position: [2.5, 2, 2.5], up: [0, 0, 1], near: 0.1, far: 100 }}>
      <ambientLight intensity={0.8} />
      <directionalLight intensity={0.8} position={[3, 4, 5]} />
      <gridHelper args={[4, 8, "#94a3b8", "#cbd5e1"]} rotation={[Math.PI / 2, 0, 0]} />
      <axesHelper args={[1.5]} />
      <mesh position={[target.x, target.y, target.z]}>
        <sphereGeometry args={[sphereSize, 18, 18]} />
        <meshStandardMaterial color="#2563eb" />
      </mesh>
      <mesh position={[midpoint.x, midpoint.y, midpoint.z]} quaternion={new THREE.Quaternion().setFromUnitVectors(new THREE.Vector3(0, 1, 0), dir)} scale={[1, length, 1]}>
        <cylinderGeometry args={[0.02, 0.02, 1, 12]} />
        <meshStandardMaterial color="#334155" />
      </mesh>
      <OrbitControls makeDefault />
    </ThreeCanvas>
  );
};

export const Vector: React.FC<VectorProps> = ({
  id,
  vector,
  defaultVector,
  onVectorChange,
  x,
  defaultX,
  onXChange,
  y,
  defaultY,
  onYChange,
  z,
  defaultZ,
  onZChange,
  selectionEnabled = true,
  xSelectionEnabled = true,
  ySelectionEnabled = true,
  zSelectionEnabled = true,
  displayEnabled = true,
  xDisplayEnabled = true,
  yDisplayEnabled = true,
  zDisplayEnabled = true,
  minX = -1,
  maxX = 1,
  minY = -1,
  maxY = 1,
  minZ = -1,
  maxZ = 1,
  step = 0.1,
  axisLabels,
  className = "",
}) => {
  const [resolvedVector, setResolvedVector] = useInteractiveControllableValue<VectorValue>(vector, normalizeVector(defaultVector), onVectorChange);
  const hasXPartialControl = x !== undefined || defaultX !== undefined || onXChange !== undefined;
  const hasYPartialControl = y !== undefined || defaultY !== undefined || onYChange !== undefined;
  const hasZPartialControl = z !== undefined || defaultZ !== undefined || onZChange !== undefined;
  const [resolvedX, setResolvedX] = useInteractiveControllableValue<number>(x, defaultX ?? resolvedVector.x, onXChange);
  const [resolvedY, setResolvedY] = useInteractiveControllableValue<number>(y, defaultY ?? resolvedVector.y, onYChange);
  const [resolvedZ, setResolvedZ] = useInteractiveControllableValue<number>(z, defaultZ ?? resolvedVector.z, onZChange);
  const currentVector: VectorValue = {
    x: hasXPartialControl ? resolvedX : resolvedVector.x,
    y: hasYPartialControl ? resolvedY : resolvedVector.y,
    z: hasZPartialControl ? resolvedZ : resolvedVector.z,
  };

  const updateAxis = React.useCallback(
    (axis: "x" | "y" | "z", rawValue: number) => {
      const parsedValue = Number.isFinite(rawValue) ? rawValue : 0;
      const clampedValue = axis === "x" ? clampVectorAxis(parsedValue, minX, maxX) : axis === "y" ? clampVectorAxis(parsedValue, minY, maxY) : clampVectorAxis(parsedValue, minZ, maxZ);
      const nextVector: VectorValue = { ...currentVector, [axis]: clampedValue };
      if (axis === "x" && hasXPartialControl) setResolvedX(clampedValue);
      if (axis === "y" && hasYPartialControl) setResolvedY(clampedValue);
      if (axis === "z" && hasZPartialControl) setResolvedZ(clampedValue);
      setResolvedVector(nextVector);
    },
    [currentVector, hasXPartialControl, hasYPartialControl, hasZPartialControl, maxX, maxY, maxZ, minX, minY, minZ, setResolvedVector, setResolvedX, setResolvedY, setResolvedZ],
  );

  const renderAxisRow = (axis: "x" | "y" | "z", label: string, value: number, min: number, max: number, axisDisplayEnabled: boolean, axisSelectionEnabled: boolean) => {
    if (!axisDisplayEnabled) return null;
    const canSelect = selectionEnabled && axisSelectionEnabled;
    return (
      <div key={axis} className="grid grid-cols-[24px_1fr_88px] items-center gap-2">
        <label htmlFor={`${id}-${axis}`} className={`text-xs font-semibold text-muted-foreground ${label.length <= 1 ? "uppercase" : ""}`}>
          {label}
        </label>
        {canSelect ? <input id={`${id}-${axis}`} type="range" min={min} max={max} step={step} value={value} onChange={(event) => updateAxis(axis, Number(event.target.value))} /> : <div className="h-2 rounded-full bg-muted/60" />}
        <input
          type="number"
          min={min}
          max={max}
          step={step}
          value={value}
          readOnly={!canSelect}
          onChange={canSelect ? (event) => updateAxis(axis, Number(event.target.value)) : undefined}
          className="w-full rounded-md border border-input bg-background px-2 py-1 text-right text-sm font-mono"
        />
      </div>
    );
  };

  const labelFor = (axis: "x" | "y" | "z") => axisLabels?.[axis] ?? axis.toUpperCase();

  if (!displayEnabled) return null;
  return (
    <div id={id} data-slot="vector" className={`flex flex-col gap-3 ${className}`}>
      <div className="h-40 w-full overflow-hidden rounded-md border border-border bg-background">
        <VectorPreview3D vector={currentVector} />
      </div>
      {renderAxisRow("x", labelFor("x"), currentVector.x, minX, maxX, xDisplayEnabled, xSelectionEnabled)}
      {renderAxisRow("y", labelFor("y"), currentVector.y, minY, maxY, yDisplayEnabled, ySelectionEnabled)}
      {renderAxisRow("z", labelFor("z"), currentVector.z, minZ, maxZ, zDisplayEnabled, zSelectionEnabled)}
    </div>
  );
};

// #endregion 🔤Vector

// #region 📦MoveVectorInput

// Specs: Maps MoveVector (gap/shift/rise) onto Vector axes x/y/z per placement frame (shift to x, gap to y, rise to z).
// Summary: Algorithm VECTOR_INPUT control backed by the shared Vector editor and preview.

export interface MoveVectorInputProps {
  id: string;
  value: MoveVector;
  min?: MoveVector;
  max?: MoveVector;
  onChange?: (next: MoveVector) => void;
  className?: string;
}

export const MoveVectorInput: React.FC<MoveVectorInputProps> = ({ id, value, min = { gap: -10, shift: -10, rise: -10 }, max = { gap: 10, shift: 10, rise: 10 }, onChange, className = "" }) => (
  <Vector
    id={id}
    vector={{ x: value.shift, y: value.gap, z: value.rise }}
    onVectorChange={(v) => onChange?.({ shift: v.x, gap: v.y, rise: v.z })}
    minX={min.shift}
    maxX={max.shift}
    minY={min.gap}
    maxY={max.gap}
    minZ={min.rise}
    maxZ={max.rise}
    step={0.1}
    axisLabels={{ x: "shift", y: "gap", z: "rise" }}
    className={className}
  />
);

// #endregion 📦MoveVectorInput

// #region 📍Scene

// Specs: Minimal 3D scene rendering a design from a kit. Uses React Three Fiber Canvas
// with orthographic camera, grid, gizmo, and orbit controls. Pieces use kit GLTF or unit boxes;
// selection hull unions mesh AABBs from registered scene roots (not planes). frameloop="demand".
// Summary: Lightweight 3D scene viewer with representation-accurate selection bounds.

const SCENE_BOX_SIZE = 1;

// Specs: Small world-space inflate on the union of mesh AABBs so the highlight stroke does not coincide exactly with geometry facets.
// Summary: Padding after representation-derived scene selection hull.

const SCENE_SELECTION_BOUNDS_EXPAND = 0.04;

const getSceneComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
const resolveSceneColor = (cssValue: string, fallback: string): string => {
  if (cssValue.startsWith("var(")) {
    const variableExpression = cssValue.slice(4, -1);
    const [variableNameRaw, inlineFallbackRaw] = variableExpression.split(",");
    const variableName = variableNameRaw?.trim() ?? "";
    const inlineFallback = inlineFallbackRaw?.trim();
    return (variableName ? getSceneComputedColor(variableName) : "") || inlineFallback || fallback;
  }
  if (cssValue === "currentColor") return fallback;
  return cssValue;
};
const SEMIO_TO_THREE_BASIS = new THREE.Matrix4().identity();
const THREE_TO_SEMIO_BASIS = SEMIO_TO_THREE_BASIS.clone().invert();

interface ScenePieceAsset {
  piece: Piece;
  status: DiagramEntityStatus;
  representationName?: string;
  representationSource?: string;
}

interface SceneConnectionAsset {
  connection: Connection;
  sourcePiece: Piece;
  targetPiece: Piece;
  status: DiagramEntityStatus;
}

interface SceneGizmoViewportPlacement {
  alignment: "top-left" | "top-right" | "bottom-left" | "bottom-right";
  margin: [number, number];
}

interface SceneSnapshot {
  pieces: ScenePieceAsset[];
  connections: SceneConnectionAsset[];
}

// Specs: Match sketchpad {@link getReadableKitFileUrl} — kit JSON often carries `remote` (http(s)) or `blob` (data:/blob:) without a separate `url` field.
// Summary: Resolves a browser-loadable representation URL from a kit {@link SemioFile}.

const isBrowserReadableRepresentationUrl = (url: string): boolean => /^(blob:|data:|https?:)/i.test(url.trim());

const getSceneFileSource = (file?: SemioFile): string | undefined => {
  if (!file) return undefined;
  if (typeof file.blob === "string" && file.blob.length > 0 && isBrowserReadableRepresentationUrl(file.blob)) return file.blob.trim();
  const optionalFileUrl = typeof (file as SemioFile & { url?: string }).url === "string" ? (file as SemioFile & { url?: string }).url!.trim() : "";
  if (optionalFileUrl.length > 0 && isBrowserReadableRepresentationUrl(optionalFileUrl)) return optionalFileUrl;
  if (typeof file.remote === "string" && file.remote.length > 0 && isBrowserReadableRepresentationUrl(file.remote)) return file.remote.trim();
  return undefined;
};

const isSceneGltfSource = (source?: string, representationName?: string): boolean => {
  if (!source) return false;
  if (source.startsWith("data:representation/gltf")) return true;
  const loweredName = representationName?.toLowerCase() ?? "";
  const loweredSource = source.split("?")[0].toLowerCase();
  return loweredName.endsWith(".glb") || loweredName.endsWith(".gltf") || loweredSource.endsWith(".glb") || loweredSource.endsWith(".gltf");
};

export const resolveSceneGizmoViewportPlacement = (viewport: { width: number; height: number }): SceneGizmoViewportPlacement => {
  const clampHorizontalMargin = (width: number): number => Math.min(56, Math.max(26, Math.floor(width / 5)));
  const clampVerticalMargin = (height: number): number => Math.min(40, Math.max(18, Math.floor(height / 7)));
  return {
    alignment: "bottom-right",
    margin: [clampHorizontalMargin(viewport.width), clampVerticalMargin(viewport.height)],
  };
};

const buildScenePieceAssets = (kit: Kit, pieces: Array<{ piece: Piece; status: DiagramEntityStatus }>): ScenePieceAsset[] => {
  const kindsById = new Map(
    kitJsRows(kit, "types")
      .filter((k) => String(k.id ?? "").length > 0)
      .map((kind) => [String(kind.id), kind] as const),
  );
  const filesById = new Map(
    kitJsRows(kit, "files")
      .filter((f) => String(f.id ?? "").length > 0)
      .map((file) => [String(file.id), file] as const),
  );
  const typeRows = kitJsRows(kit, "types");
  const withPlaneAndCenter = pieces.filter(({ piece }) => piece.plane && piece.center);
  const result = withPlaneAndCenter.map(({ piece, status }) => {
    const kindId = piece.type?.id;
    let kind = kindId ? kindsById.get(String(kindId)) : undefined;
    if (!kind && piece.type && typeof piece.type === "object" && "name" in piece.type && typeof (piece.type as { name?: unknown }).name === "string") {
      const nm = String((piece.type as { name: string }).name);
      kind = typeRows.find((candidateKind) => String(candidateKind.name ?? "") === nm);
    }
    let file: SemioFile | undefined;
    const representations = (kind && typeof kind === "object" && "representations" in kind ? __itemsOf((kind as { representations?: unknown }).representations) : []) as readonly PlainJsonObject[];
    let selectedRepresentation = representations.length ? pickPreferredRepresentation(representations, []) : undefined;
    const rep0 = selectedRepresentation as { file?: { id?: string }; name?: string } | undefined;
    if (rep0?.file?.id) file = filesById.get(String(rep0.file.id)) as SemioFile | undefined;
    const fileName = file && typeof file.name === "string" ? file.name : undefined;
    if (!isSceneGltfSource(getSceneFileSource(file), fileName) && representations.length > 0) {
      for (const m of representations) {
        const mr = m as { file?: { id?: string }; name?: string };
        const f = mr.file?.id ? filesById.get(String(mr.file.id)) : undefined;
        const fn = f && typeof f.name === "string" ? f.name : undefined;
        if (f && isSceneGltfSource(getSceneFileSource(f as SemioFile), fn)) {
          selectedRepresentation = m;
          file = f as SemioFile;
          break;
        }
      }
    }
    const repName = file && typeof file.name === "string" ? file.name : undefined;
    return {
      piece,
      status,
      representationName: repName,
      representationSource: getSceneFileSource(file),
    };
  });
  return result;
};

const toSceneVector = (coordinate: PlainJsonValue | undefined | null | unknown): THREE.Vector3 => {
  const o =
    coordinate != null && typeof coordinate === "object" && !Array.isArray(coordinate)
      ? (coordinate as PlainJsonObject)
      : ({} as PlainJsonObject);
  const x = Number(o["x"] ?? 0);
  const y = Number(o["y"] ?? 0);
  const z = Number(o["z"] ?? 0);
  return new THREE.Vector3(x, y, z).applyMatrix4(SEMIO_TO_THREE_BASIS);
};

const buildSceneSnapshot = (design: Design, designDiff?: DesignDiff): SceneSnapshot => {
  const mergedRow = (designDiff ? previewDesignWithDiff(design, designDiff) : design) as PlainJsonObject;
  const mergedPieces = __itemsOf(mergedRow["pieces"]) as Piece[];

  const pieceMap = new Map<string, ScenePieceAsset>();
  mergedPieces.forEach((piece: Piece) => {
    if (!piece.id) return;
    const status: DiagramEntityStatus = designDiff ? getDiffStatusFromAttributes(piece.attributes) : "default";
    const existing = pieceMap.get(piece.id);
    const resolvedPiece = (
      piece.plane && piece.center ? piece : existing?.piece?.plane && existing?.piece?.center ? { ...piece, plane: existing.piece.plane, center: existing.piece.center } : piece
    ) as Piece;
    if (!resolvedPiece.plane || !resolvedPiece.center) return;
    pieceMap.set(piece.id, { piece: resolvedPiece, status: existing ? (status !== "default" ? status : existing.status) : status });
  });

  const piecesById = new Map(Array.from(pieceMap.values()).map((asset) => [asset.piece.id, asset.piece] as const));
  const connectionMap = new Map<string, SceneConnectionAsset>();
  snapshotDesignConnections(mergedRow as unknown as Design).forEach((connection) => {
    if (!connection.id) return;
    const srcId = connection.connected?.piece?.id;
    const tgtId = connection.connecting?.piece?.id;
    if (!srcId || !tgtId) return;
    const sourcePiece = piecesById.get(srcId);
    const targetPiece = piecesById.get(tgtId);
    if (!sourcePiece?.plane || !targetPiece?.plane || !sourcePiece?.center || !targetPiece?.center) return;
    const status: DiagramEntityStatus = designDiff ? getDiffStatusFromAttributes(connection.attributes) : "default";
    connectionMap.set(connection.id, { connection, sourcePiece, targetPiece, status });
  });

  // One-hop propagation: a modified connection colors its child (connecting) piece.
  // This does NOT cascade back from pieces to connections — only connections propagate to child pieces.
  if (designDiff) {
    for (const connAsset of connectionMap.values()) {
      if (connAsset.status !== "default") {
        const childId = connAsset.connection.connecting?.piece?.id;
        if (!childId) continue;
        const childAsset = pieceMap.get(childId);
        if (childAsset && childAsset.status === "default") {
          childAsset.status = "modified";
        }
      }
    }
  }

  return {
    pieces: Array.from(pieceMap.values()),
    connections: Array.from(connectionMap.values()),
  };
};

const toScenePieceMatrix = (plane: Plane): THREE.Matrix4 => {
  const planeMatrix = new THREE.Matrix4().identity();
  return new THREE.Matrix4().multiplyMatrices(SEMIO_TO_THREE_BASIS, planeMatrix).multiply(THREE_TO_SEMIO_BASIS);
};

// Specs: Union of {@link THREE.Box3.setFromObject} on registered roots — pieces use transform groups that wrap GLTF or placeholders; connections use cylinder meshes. Excludes abstract planes/connectors.
// Summary: World AABB for selected rendered scene objects only.

const computeSceneSelectionUnionBox = (rootsById: Map<string, THREE.Object3D>, selectedPieceIds: Set<string>, selectedConnectionIds: Set<string>): THREE.Box3 | null => {
  const box = new THREE.Box3();
  let any = false;
  const unionObject = (obj: THREE.Object3D) => {
    const b = new THREE.Box3().setFromObject(obj);
    if (!Number.isFinite(b.min.x) || !Number.isFinite(b.max.x) || b.isEmpty()) return;
    if (!any) {
      box.copy(b);
      any = true;
    } else {
      box.union(b);
    }
  };
  for (const id of selectedPieceIds) {
    const obj = rootsById.get(id);
    if (obj) unionObject(obj);
  }
  for (const id of selectedConnectionIds) {
    const obj = rootsById.get(id);
    if (obj) unionObject(obj);
  }
  if (!any) return null;
  box.expandByScalar(SCENE_SELECTION_BOUNDS_EXPAND);
  return box;
};

// #region 🎯SceneSelectionBounds

// Specs: Registry maps piece/connection ids to Object3D roots; hull updates under demand frameloop via invalidate + useFrame.
// Summary: Provider and representation-driven selection overlay mesh.

type SceneRepresentationBoundsRegistryValue = {
  registerBoundsRoot: (id: string, root: THREE.Object3D) => void;
  unregisterBoundsRoot: (id: string) => void;
  rootsRef: React.MutableRefObject<Map<string, THREE.Object3D>>;
};

const SceneRepresentationBoundsRegistryContext = React.createContext<SceneRepresentationBoundsRegistryValue | null>(null);

const SceneRepresentationBoundsRegistryProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const rootsRef = React.useRef(new Map<string, THREE.Object3D>());
  const value = React.useMemo<SceneRepresentationBoundsRegistryValue>(
    () => ({
      registerBoundsRoot: (id: string, root: THREE.Object3D) => {
        rootsRef.current.set(id, root);
      },
      unregisterBoundsRoot: (id: string) => {
        rootsRef.current.delete(id);
      },
      rootsRef,
    }),
    [],
  );
  return <SceneRepresentationBoundsRegistryContext.Provider value={value}>{children}</SceneRepresentationBoundsRegistryContext.Provider>;
};

const SceneSelectionBoundsFromRepresentations: React.FC<{
  selectedPieceIds: Set<string>;
  selectedConnectionIds: Set<string>;
}> = ({ selectedPieceIds, selectedConnectionIds }) => {
  const registry = React.useContext(SceneRepresentationBoundsRegistryContext);
  const { invalidate } = useThree();
  const groupRef = React.useRef<THREE.Group>(null);
  const meshRef = React.useRef<THREE.Mesh>(null);
  const [accent, setAccent] = React.useState(() => getSceneComputedColor("--accent") || "#3b82f6");
  React.useEffect(() => {
    const sync = () => setAccent(getSceneComputedColor("--accent") || "#3b82f6");
    sync();
    const observer = new MutationObserver(sync);
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ["class"] });
    return () => observer.disconnect();
  }, []);

  const selectionNonEmpty = selectedPieceIds.size > 0 || selectedConnectionIds.size > 0;
  const selectionKey = React.useMemo(() => `${Array.from(selectedPieceIds).sort().join(",")}|${Array.from(selectedConnectionIds).sort().join(",")}`, [selectedPieceIds, selectedConnectionIds]);

  React.useLayoutEffect(() => {
    if (!selectionNonEmpty) return;
    invalidate();
  }, [invalidate, selectionKey, selectionNonEmpty]);

  useFrame(() => {
    if (!registry || !selectionNonEmpty) {
      if (groupRef.current) groupRef.current.visible = false;
      return;
    }
    const computed = computeSceneSelectionUnionBox(registry.rootsRef.current, selectedPieceIds, selectedConnectionIds);
    if (!computed || computed.isEmpty()) {
      if (groupRef.current) groupRef.current.visible = false;
      return;
    }
    const g = groupRef.current;
    const mesh = meshRef.current;
    if (!g || !mesh) return;
    g.visible = true;
    const c = new THREE.Vector3();
    const s = new THREE.Vector3();
    computed.getCenter(c);
    computed.getSize(s);
    g.position.copy(c);
    mesh.scale.set(Math.max(s.x, 0.06), Math.max(s.y, 0.06), Math.max(s.z, 0.06));
  });

  return (
    <group ref={groupRef} raycast={() => null} visible={false}>
      <mesh ref={meshRef} raycast={() => null}>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial color={accent} depthWrite={false} metalness={0} opacity={SEMIO_SELECTION_BOUNDS_FILL_OPACITY} roughness={1} transparent />
        <Edges color={accent} opacity={SEMIO_SELECTION_BOUNDS_STROKE_OPACITY} threshold={18} transparent />
      </mesh>
    </group>
  );
};

// #endregion 🎯SceneSelectionBounds

// #region 🔗SceneRepresentationMaterials
// Specs: Imported scene assets in semio/ui MUST ignore embedded mesh, line, and point colors.
// The runtime replaces them with homogeneous scene materials so status and interaction colors stay consistent.
// Summary: Shared helpers that normalize imported scene materials and recolor them consistently.

interface SceneRepresentationColorState {
  meshColor: string;
  lineColor: string;
  emissiveColor?: string;
  emissiveIntensity: number;
  opacity: number;
}

const createSceneMeshMaterial = (color: string): THREE.MeshStandardMaterial =>
  new THREE.MeshStandardMaterial({
    color,
    metalness: 0,
    roughness: 1,
  });

const createSceneLineMaterial = (color: string): THREE.LineBasicMaterial => new THREE.LineBasicMaterial({ color });

const createScenePointsMaterial = (color: string): THREE.PointsMaterial => new THREE.PointsMaterial({ color, size: 1 });

const SCENE_MESH_OUTLINE_USER_DATA_KEY = "__semioSceneMeshOutline";

const createSceneMeshOutline = (geometry: THREE.BufferGeometry, color: string): THREE.LineSegments => {
  const outline = new THREE.LineSegments(new THREE.EdgesGeometry(geometry), createSceneLineMaterial(color));
  outline.name = "SemioSceneMeshOutline";
  outline.scale.setScalar(1.001);
  outline.userData[SCENE_MESH_OUTLINE_USER_DATA_KEY] = true;
  return outline;
};

const cloneSceneRepresentationWithHomogeneousMaterials = (scene: THREE.Object3D, meshColor: string, lineColor: string): THREE.Object3D => {
  const cloned = cloneSkeleton(scene);
  cloned.traverse((object) => {
    if (object instanceof THREE.Mesh) {
      if (Array.isArray(object.material)) {
        object.material = object.material.map(() => createSceneMeshMaterial(meshColor));
      } else {
        object.material = createSceneMeshMaterial(meshColor);
      }
      const meshGeometry = object.geometry;
      if (meshGeometry) {
        object.add(createSceneMeshOutline(meshGeometry, lineColor));
      }
      return;
    }
    if (object instanceof THREE.Line || object instanceof THREE.LineSegments) {
      if (object.userData[SCENE_MESH_OUTLINE_USER_DATA_KEY]) return;
      object.material = createSceneLineMaterial(lineColor);
      return;
    }
    if (object instanceof THREE.Points) {
      object.material = createScenePointsMaterial(lineColor);
    }
  });
  return cloned;
};

const applySceneRepresentationColorState = (scene: THREE.Object3D, state: SceneRepresentationColorState): void => {
  scene.traverse((object) => {
    if (object instanceof THREE.Mesh) {
      const materials = Array.isArray(object.material) ? object.material : [object.material];
      materials.forEach((material) => {
        if (!material || !("color" in material)) return;
        const meshMaterial = material as THREE.MeshStandardMaterial;
        meshMaterial.color.set(state.meshColor);
        meshMaterial.emissive.set(state.emissiveColor ?? "#000000");
        meshMaterial.emissiveIntensity = state.emissiveIntensity;
        meshMaterial.transparent = state.opacity < 1;
        meshMaterial.opacity = state.opacity;
      });
      return;
    }
    if (object instanceof THREE.Line || object instanceof THREE.LineSegments || object instanceof THREE.Points) {
      const material = object.material;
      if (!material || !("color" in material)) return;
      (material as THREE.LineBasicMaterial | THREE.PointsMaterial).color.set(state.lineColor);
      material.transparent = state.opacity < 1;
      material.opacity = state.opacity;
    }
  });
};

const getSceneRepresentationColorState = (status: DiagramEntityStatus, isSelected: boolean, isHovered: boolean): SceneRepresentationColorState => {
  const isDiffed = status !== "default";
  const isRemoved = status === "removed";
  const neutralColor = resolveSceneColor("var(--muted-foreground)", "#888888");
  const baseColor = isDiffed ? resolveSceneColor(getEntityStatusColor(status), "#888888") : neutralColor;
  const selectedColor = isSelected ? resolveSceneColor(getInteractiveEntityColor(status, true, false), "#3b82f6") : null;
  const hoveredColor = isHovered ? resolveSceneColor(getInteractiveEntityColor(status, false, true), "#60a5fa") : null;

  return {
    meshColor: isSelected ? (selectedColor ?? baseColor) : isHovered ? (hoveredColor ?? baseColor) : baseColor,
    lineColor: isSelected ? (selectedColor ?? baseColor) : isHovered ? (hoveredColor ?? baseColor) : baseColor,
    emissiveColor: isSelected ? (selectedColor ?? baseColor) : isHovered ? (hoveredColor ?? baseColor) : isDiffed ? baseColor : undefined,
    emissiveIntensity: isSelected ? 0.35 : isHovered ? 0.15 : isDiffed ? 0.4 : 0,
    opacity: isRemoved ? 0.35 : 1,
  };
};

// #endregion 🔗SceneRepresentationMaterials

interface ScenePieceRepresentationProps {
  representationSource: string;
  status: DiagramEntityStatus;
  isSelected: boolean;
  isHovered: boolean;
}

const ScenePieceRepresentation: React.FC<ScenePieceRepresentationProps> = ({ representationSource, status, isSelected, isHovered }) => {
  // Disable Draco and meshopt to avoid WebAssembly CSP violations in MCP App iframes.
  // 🔷drei defaults meshopt to true which calls WebAssembly.instantiate() violating script-src wasm-eval.
  const gltf = useGLTF(representationSource, false, false);
  const { invalidate } = useThree();
  const bounds = useBounds();
  const clone = React.useMemo(() => {
    return cloneSceneRepresentationWithHomogeneousMaterials(gltf.scene, "#888888", "#888888");
  }, [gltf.scene]);

  React.useEffect(() => {
    applySceneRepresentationColorState(clone, getSceneRepresentationColorState(status, isSelected, isHovered));
    invalidate();
  }, [clone, invalidate, status, isHovered, isSelected]);

  React.useLayoutEffect(() => {
    bounds.refresh();
    bounds.fit();
    invalidate();
  }, [bounds, clone, invalidate]);

  return <Clone object={clone} />;
};

interface ScenePieceProps {
  piece: Piece;
  status: DiagramEntityStatus;
  representationName?: string;
  representationSource?: string;
  isSelected: boolean;
  isHovered: boolean;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onClick?: () => void;
  onDoubleClick?: () => void;
}

const ScenePiece: React.FC<ScenePieceProps> = ({ piece, status, representationName, representationSource, isSelected, isHovered, onPointerEnter, onPointerLeave, onClick, onDoubleClick }) => {
  const defaultColor = React.useMemo(() => resolveSceneColor(getEntityStatusColor(status), "#888888"), [status]);
  const activeColor = React.useMemo(() => resolveSceneColor(getInteractiveEntityColor(status, true, false), "#3b82f6"), [status]);
  const hoverColor = React.useMemo(() => resolveSceneColor(getInteractiveEntityColor(status, false, true), "#60a5fa"), [status]);

  const matrix = React.useMemo(() => {
    if (!piece.plane || !piece.center) return null;
    const base = toScenePieceMatrix(piece.plane as Plane);
    const scRaw = piece.scale;
    const sc = typeof scRaw === "number" && Number.isFinite(scRaw) ? scRaw : undefined;
    if (sc != null && sc !== 1) {
      base.multiply(new THREE.Matrix4().makeScale(sc, sc, sc));
    }
    return base;
  }, [piece.plane, piece.center, piece.scale]);

  const boundsRegistry = React.useContext(SceneRepresentationBoundsRegistryContext);
  const pieceBoundsRootRef = React.useRef<THREE.Group>(null);

  const meshColor = isSelected ? activeColor : isHovered ? hoverColor : defaultColor;
  const edgeColor = meshColor;
  const isRemoved = status === "removed";

  if (!matrix) return null;

  const canRenderRepresentation = isSceneGltfSource(representationSource, representationName);

  React.useLayoutEffect(() => {
    const pid = String(piece.id ?? "");
    if (!boundsRegistry || !pid) return;
    const n = pieceBoundsRootRef.current;
    if (n) boundsRegistry.registerBoundsRoot(pid, n);
    return () => boundsRegistry.unregisterBoundsRoot(pid);
  }, [boundsRegistry, piece.id, matrix, canRenderRepresentation, representationSource]);

  const handleClick = onClick
    ? (event: { stopPropagation: () => void }) => {
        event.stopPropagation();
        onClick();
      }
    : undefined;

  const handleDoubleClick = onDoubleClick
    ? (event: { stopPropagation: () => void }) => {
        event.stopPropagation();
        onDoubleClick();
      }
    : undefined;

  const handlePointerEnter = onPointerEnter
    ? (event: { stopPropagation: () => void }) => {
        event.stopPropagation();
        onPointerEnter();
      }
    : undefined;

  const handlePointerLeave = onPointerLeave
    ? (event: { stopPropagation: () => void }) => {
        event.stopPropagation();
        onPointerLeave();
      }
    : undefined;

  return (
    <group ref={pieceBoundsRootRef} matrix={matrix} matrixAutoUpdate={false}>
      {canRenderRepresentation && representationSource ? (
        <group onClick={handleClick} onDoubleClick={handleDoubleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
          <React.Suspense fallback={null}>
            <ScenePieceRepresentation representationSource={representationSource} status={status} isSelected={isSelected} isHovered={isHovered} />
          </React.Suspense>
        </group>
      ) : (
        <mesh onClick={handleClick} onDoubleClick={handleDoubleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
          <boxGeometry args={[SCENE_BOX_SIZE, SCENE_BOX_SIZE, SCENE_BOX_SIZE]} />
          <meshStandardMaterial color={meshColor} emissive={meshColor} emissiveIntensity={isSelected ? 0.45 : isHovered ? 0.2 : 0.05} transparent={isRemoved} opacity={isRemoved ? 0.35 : 1} />
          <Edges scale={1.001} color={edgeColor} />
        </mesh>
      )}
    </group>
  );
};

interface SceneConnectionProps {
  connection: Connection;
  sourcePiece: Piece;
  targetPiece: Piece;
  status: DiagramEntityStatus;
  isSelected: boolean;
  isHovered: boolean;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onClick?: () => void;
}

const SceneConnection: React.FC<SceneConnectionProps> = ({ connection, sourcePiece, targetPiece, status, isSelected, isHovered, onPointerEnter, onPointerLeave, onClick }) => {
  const defaultColor = React.useMemo(() => resolveSceneColor(getEntityStatusColor(status), "#888888"), [status]);
  const activeColor = React.useMemo(() => resolveSceneColor(getInteractiveEntityColor(status, true, false), "#3b82f6"), [status]);
  const hoverColor = React.useMemo(() => resolveSceneColor(getInteractiveEntityColor(status, false, true), "#60a5fa"), [status]);

  const boundsRegistry = React.useContext(SceneRepresentationBoundsRegistryContext);
  const connectionMeshRef = React.useRef<THREE.Mesh>(null);

  const start = React.useMemo(() => (sourcePiece.plane && sourcePiece.center ? toSceneVector(sourcePiece.plane.origin) : null), [sourcePiece.plane, sourcePiece.center]);
  const end = React.useMemo(() => (targetPiece.plane && targetPiece.center ? toSceneVector(targetPiece.plane.origin) : null), [targetPiece.plane, targetPiece.center]);
  const transform = React.useMemo(() => {
    if (!start || !end) return null;
    const direction = end.clone().sub(start);
    const length = direction.length();
    if (length <= 0.0001) return null;
    const midpoint = start.clone().add(end).multiplyScalar(0.5);
    const quaternion = new THREE.Quaternion().setFromUnitVectors(new THREE.Vector3(0, 1, 0), direction.normalize());
    return { midpoint, quaternion, length };
  }, [end, start]);

  if (!transform) return null;

  const color = isSelected ? activeColor : isHovered ? hoverColor : defaultColor;
  const radius = isSelected ? 0.14 : isHovered ? 0.11 : 0.08;

  React.useLayoutEffect(() => {
    const cid = String(connection.id ?? "");
    if (!boundsRegistry || !cid) return;
    const n = connectionMeshRef.current;
    if (n) boundsRegistry.registerBoundsRoot(cid, n);
    return () => boundsRegistry.unregisterBoundsRoot(cid);
  }, [boundsRegistry, connection.id, transform, radius]);

  const handleClick = onClick
    ? (event: { stopPropagation: () => void }) => {
        event.stopPropagation();
        onClick();
      }
    : undefined;

  const handlePointerEnter = onPointerEnter
    ? (event: { stopPropagation: () => void }) => {
        event.stopPropagation();
        onPointerEnter();
      }
    : undefined;

  const handlePointerLeave = onPointerLeave
    ? (event: { stopPropagation: () => void }) => {
        event.stopPropagation();
        onPointerLeave();
      }
    : undefined;

  return (
    <mesh ref={connectionMeshRef} name={connection.id} position={transform.midpoint} quaternion={transform.quaternion} onClick={handleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
      <cylinderGeometry args={[radius, radius, transform.length, 12]} />
      <meshStandardMaterial color={color} emissive={color} emissiveIntensity={isSelected ? 0.45 : isHovered ? 0.2 : 0.05} transparent={status === "removed"} opacity={status === "removed" ? 0.35 : 1} />
    </mesh>
  );
};

interface SceneGizmoProps {
  show: boolean;
  onAxisClick?: (direction: THREE.Vector3) => void;
}

const SceneGizmo: React.FC<SceneGizmoProps> = ({ show, onAxisClick }) => {
  const { size } = useThree();
  const [colors, setColors] = React.useState<[string, string, string]>(() => [getSceneComputedColor("--accent") || "#ef4444", getSceneComputedColor("--accent-tertiary") || "#22c55e", getSceneComputedColor("--accent-secondary") || "#3b82f6"]);
  const placement = React.useMemo(() => resolveSceneGizmoViewportPlacement(size), [size]);
  // GizmoViewport axis box uses boxGeometry args [length, thickness, thickness]; uniform scale yields a chunky cube.
  const axisScale = React.useMemo(() => [0.88, 0.036, 0.036] as [number, number, number], []);
  const labelColor = React.useMemo(() => getSceneComputedColor("--foreground") || "#111827", []);

  React.useEffect(() => {
    const updateColors = () => setColors([getSceneComputedColor("--accent") || "#ef4444", getSceneComputedColor("--accent-tertiary") || "#22c55e", getSceneComputedColor("--accent-secondary") || "#3b82f6"]);
    updateColors();
    const observer = new MutationObserver(updateColors);
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ["class"] });
    return () => observer.disconnect();
  }, []);

  if (!show) return null;
  return (
    <GizmoHelper alignment={placement.alignment} margin={placement.margin}>
      <GizmoViewport
        labels={["X", "Z", "-Y"]}
        axisColors={colors}
        axisScale={axisScale}
        axisHeadScale={0.92}
        hideNegativeAxes
        labelColor={labelColor}
        font="16px Inter var, Arial, sans-serif"
        onClick={
          onAxisClick
            ? (e) => {
                onAxisClick(e.object.position.clone());
                return null;
              }
            : undefined
        }
      />
    </GizmoHelper>
  );
};

interface SceneGridProps {
  show: boolean;
}

const SceneGrid: React.FC<SceneGridProps> = ({ show }) => {
  const [gridColors, setGridColors] = React.useState({
    sectionColor: getSceneComputedColor("--foreground") || "#888888",
    cellColor: getSceneComputedColor("--accent-foreground") || "#cccccc",
  });

  React.useEffect(() => {
    const updateColors = () =>
      setGridColors({
        sectionColor: getSceneComputedColor("--foreground") || "#888888",
        cellColor: getSceneComputedColor("--accent-foreground") || "#cccccc",
      });
    updateColors();
    const observer = new MutationObserver(updateColors);
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ["class"] });
    return () => observer.disconnect();
  }, []);

  if (!show) return null;
  return <Grid infiniteGrid sectionColor={gridColors.sectionColor} cellColor={gridColors.cellColor} />;
};

export interface SemioSceneProps {
  design: Design;
  kit?: Kit;
  designDiff?: DesignDiff;
  defaultDesignDiff?: DesignDiff;
  diffEnabled?: boolean;
  zoomTarget?: ZoomTarget;
  selection?: DiagramSelection;
  defaultSelection?: DiagramSelection;
  selectionEnabled?: boolean;
  pieceSelectionEnabled?: boolean;
  connectionSelectionEnabled?: boolean;
  onSelectionChange?: (selection: DiagramSelection) => void;
  hover?: DiagramHover;
  defaultHover?: DiagramHover;
  hoverEnabled?: boolean;
  pieceHoverEnabled?: boolean;
  connectionHoverEnabled?: boolean;
  onHoverChange?: (hover: DiagramHover) => void;
  onPieceClick?: (piece: Piece) => void;
  onPieceDoubleClick?: (piece: Piece) => void;
  onConnectionClick?: (connection: Connection) => void;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: SceneCamera;
  onCameraChange?: (camera: SceneCamera) => void;
  onProjectionChange?: (projection: "camera" | "orthographic") => void;
  className?: string;
  title?: string;
}

interface SceneInnerContentProps {
  showGrid: boolean;
  showGizmo: boolean;
  zoomTarget: ZoomTarget;
  snapshot: SceneSnapshot;
  camera?: SceneCamera;
  onCameraChange?: (camera: SceneCamera) => void;
  onAxisClick?: (direction: THREE.Vector3) => void;
  onOrbitEnd?: () => void;
  children?: React.ReactNode;
}

const buildSceneZoomBox = (snapshot: SceneSnapshot, zoomTarget: ZoomTarget): THREE.Box3 | null => {
  if (zoomTarget === "none") return null;
  const pieces = zoomTarget === "diff" ? snapshot.pieces.filter((a) => a.status !== "default") : snapshot.pieces;
  const origins = pieces.filter((a) => a.piece.plane && a.piece.center).map((a) => toSceneVector(a.piece.plane!.origin));
  if (origins.length === 0) return zoomTarget === "diff" ? buildSceneZoomBox(snapshot, "design") : null;
  const box = new THREE.Box3();
  origins.forEach((o) => box.expandByPoint(o));
  return box;
};

const SceneAutoFit: React.FC<{ zoomTarget: ZoomTarget; snapshot: SceneSnapshot }> = ({ zoomTarget, snapshot }) => {
  const bounds = useBounds();
  const fittedRef = React.useRef(false);
  const snapshotKey = React.useMemo(
    () => `${zoomTarget}|${snapshot.pieces.map((a) => `${a.piece.id}:${a.status}`).join(";")}|c:${snapshot.connections.length}|p:${snapshot.pieces.length}`,
    [snapshot.connections.length, snapshot.pieces, zoomTarget],
  );
  React.useEffect(() => {
    fittedRef.current = false;
  }, [snapshotKey]);
  React.useEffect(() => {
    if (fittedRef.current) return;
    const box = buildSceneZoomBox(snapshot, zoomTarget);
    if (box) {
      bounds.refresh(box).fit();
    }
    fittedRef.current = true;
  }, [bounds, snapshot, snapshotKey, zoomTarget]);
  return null;
};

const SceneInnerContent: React.FC<SceneInnerContentProps> = ({ showGrid, showGizmo, zoomTarget, snapshot, camera: initialCamera, onCameraChange, onAxisClick, onOrbitEnd, children }) => {
  const { camera: threeCamera, invalidate } = useThree();
  const controlsRef = React.useRef<any>(null);
  const isUpdatingCameraRef = React.useRef(false);
  const cameraRestoredRef = React.useRef(false);

  React.useEffect(() => {
    const cam = threeCamera as THREE.OrthographicCamera;
    if (cam && cam instanceof THREE.OrthographicCamera) {
      cam.zoom = 50;
      cam.updateProjectionMatrix();
      invalidate();
    }
  }, [invalidate, threeCamera]);

  React.useLayoutEffect(() => {
    invalidate();
    const frameId = requestAnimationFrame(() => invalidate());
    const timeoutIds = [0, 50, 150, 300].map((delayMs) => window.setTimeout(() => invalidate(), delayMs));
    return () => {
      cancelAnimationFrame(frameId);
      timeoutIds.forEach((timeoutId) => window.clearTimeout(timeoutId));
    };
  }, [children, initialCamera, invalidate, showGizmo, showGrid, snapshot, zoomTarget]);

  React.useEffect(() => {
    if (!threeCamera || !controlsRef.current || cameraRestoredRef.current) return;
    if (!initialCamera) {
      cameraRestoredRef.current = true;
      return;
    }
    isUpdatingCameraRef.current = true;
    requestAnimationFrame(() => {
      if (!controlsRef.current) return;
      const pos = initialCamera.position;
      const fwd =
        initialCamera.forward ??
        (initialCamera.target
          ? { x: initialCamera.target.x - pos.x, y: initialCamera.target.y - pos.y, z: initialCamera.target.z - pos.z }
          : { x: 0, y: 0, z: -1 });
      const up = initialCamera.up ?? { x: 0, y: 1, z: 0 };
      threeCamera.position.set(pos.x, pos.y, pos.z);
      threeCamera.up.set(up.x, up.y, up.z);
      const target = new THREE.Vector3(pos.x + fwd.x, pos.y + fwd.y, pos.z + fwd.z);
      controlsRef.current.target.copy(target);
      threeCamera.updateProjectionMatrix();
      controlsRef.current.update();
      invalidate();
      setTimeout(() => {
        isUpdatingCameraRef.current = false;
      }, 300);
    });
    cameraRestoredRef.current = true;
  }, [initialCamera, invalidate, threeCamera]);

  const handleEnd = React.useCallback(() => {
    if (isUpdatingCameraRef.current) return;
    onOrbitEnd?.();
    if (!onCameraChange || !controlsRef.current) return;
    const position = threeCamera.position;
    const target = controlsRef.current.target;
    const forwardVec = new THREE.Vector3().subVectors(target, position);
    if (forwardVec.lengthSq() < 0.001) return;
    const forward = forwardVec.normalize();
    const up = threeCamera.up;
    onCameraChange({
      position: { x: position.x, y: position.y, z: position.z },
      forward: { x: forward.x, y: forward.y, z: forward.z },
      up: { x: up.x, y: up.y, z: up.z },
    } as SceneCamera);
    invalidate();
  }, [invalidate, onCameraChange, onOrbitEnd, threeCamera]);

  return (
    <>
      <OrbitControls ref={controlsRef} enableDamping={false} onEnd={handleEnd} />
      <ambientLight intensity={1} />
      <Bounds maxDuration={0.5} margin={1.2}>
        {children}
        {!initialCamera && zoomTarget !== "none" && <SceneAutoFit zoomTarget={zoomTarget} snapshot={snapshot} />}
      </Bounds>
      <SceneGrid show={showGrid} />
      <SceneGizmo show={showGizmo} onAxisClick={onAxisClick} />
    </>
  );
};

export const SemioScene: React.FC<SemioSceneProps> = ({
  design,
  kit,
  designDiff,
  defaultDesignDiff,
  diffEnabled = true,
  zoomTarget,
  selection,
  defaultSelection,
  selectionEnabled = true,
  pieceSelectionEnabled = true,
  connectionSelectionEnabled = true,
  onSelectionChange,
  hover,
  defaultHover,
  hoverEnabled = true,
  pieceHoverEnabled = true,
  connectionHoverEnabled = true,
  onHoverChange,
  onPieceClick,
  onPieceDoubleClick,
  onConnectionClick,
  showGrid = true,
  showGizmo = true,
  camera,
  onCameraChange,
  onProjectionChange,
  className = "",
  title = "Design Scene",
}) => {
  const resolvedDesignDiff = useResolvedValue(designDiff, defaultDesignDiff);
  const snapshot = React.useMemo(() => {
    const effectiveDiff = diffEnabled ? resolvedDesignDiff : undefined;
    return buildSceneSnapshot(design, effectiveDiff);
  }, [design, resolvedDesignDiff, diffEnabled]);
  const effectiveZoomTarget: ZoomTarget = zoomTarget ?? (diffEnabled && resolvedDesignDiff ? "diff" : "design");

  const effectivePieceSelectionEnabled = selectionEnabled && pieceSelectionEnabled;
  const effectiveConnectionSelectionEnabled = selectionEnabled && connectionSelectionEnabled;
  const effectivePieceHoverEnabled = hoverEnabled && pieceHoverEnabled && (effectivePieceSelectionEnabled || !!onPieceClick || !!onPieceDoubleClick);
  const effectiveConnectionHoverEnabled = hoverEnabled && connectionHoverEnabled && (effectiveConnectionSelectionEnabled || !!onConnectionClick);
  const [resolvedSelection, setResolvedSelection] = useInteractiveControllableValue(selection, normalizeSelection(defaultSelection), onSelectionChange);
  const [resolvedHover, setResolvedHover] = useInteractiveControllableValue(hover, normalizeHover(defaultHover), onHoverChange);
  const selectedPieceIds = React.useMemo(() => new Set(resolvedSelection.pieceIds ?? []), [resolvedSelection.pieceIds]);
  const selectedConnectionIds = React.useMemo(() => new Set(resolvedSelection.connectionIds ?? []), [resolvedSelection.connectionIds]);
  const hoveredPieceId = effectivePieceHoverEnabled ? (resolvedHover.pieceId ?? null) : null;
  const hoveredConnectionId = effectiveConnectionHoverEnabled ? (resolvedHover.connectionId ?? null) : null;

  const handleSelectPiece = React.useCallback(
    (pieceId: string | undefined) => {
      const id = String(pieceId ?? "");
      if (!id) return;
      if (!effectivePieceSelectionEnabled) return;
      const nextIds = new Set(resolvedSelection.pieceIds ?? []);
      if (nextIds.has(id)) {
        nextIds.delete(id);
      } else {
        nextIds.add(id);
      }
      setResolvedSelection({
        pieceIds: Array.from(nextIds),
        connectionIds: resolvedSelection.connectionIds ?? [],
      });
    },
    [effectivePieceSelectionEnabled, resolvedSelection.connectionIds, resolvedSelection.pieceIds, setResolvedSelection],
  );

  const handleSelectConnection = React.useCallback(
    (connectionId: string | undefined) => {
      const id = String(connectionId ?? "");
      if (!id) return;
      if (!effectiveConnectionSelectionEnabled) return;
      const nextIds = new Set(resolvedSelection.connectionIds ?? []);
      if (nextIds.has(id)) {
        nextIds.delete(id);
      } else {
        nextIds.add(id);
      }
      setResolvedSelection({
        pieceIds: resolvedSelection.pieceIds ?? [],
        connectionIds: Array.from(nextIds),
      });
    },
    [effectiveConnectionSelectionEnabled, resolvedSelection.connectionIds, resolvedSelection.pieceIds, setResolvedSelection],
  );

  const handleHoverPiece = React.useCallback(
    (pieceId: string | null) => {
      if (!effectivePieceHoverEnabled) return;
      setResolvedHover({
        pieceId,
        connectionId: resolvedHover.connectionId ?? null,
      });
    },
    [effectivePieceHoverEnabled, resolvedHover.connectionId, setResolvedHover],
  );

  const handleHoverConnection = React.useCallback(
    (connectionId: string | null) => {
      if (!effectiveConnectionHoverEnabled) return;
      setResolvedHover({
        pieceId: resolvedHover.pieceId ?? null,
        connectionId,
      });
    },
    [effectiveConnectionHoverEnabled, resolvedHover.pieceId, setResolvedHover],
  );

  const clearSelection = React.useCallback(() => {
    if (!selectionEnabled) return;
    setResolvedSelection({ pieceIds: [], connectionIds: [] });
  }, [selectionEnabled, setResolvedSelection]);

  const pieceAssets = React.useMemo(() => buildScenePieceAssets(kit ?? {}, snapshot.pieces), [kit, snapshot.pieces]);

  const gizmoSnappedRef = React.useRef(false);
  const handleAxisClick = React.useCallback(
    (_direction: THREE.Vector3) => {
      gizmoSnappedRef.current = true;
      onProjectionChange?.("orthographic");
    },
    [onProjectionChange],
  );
  const handleOrbitEnd = React.useCallback(() => {
    if (gizmoSnappedRef.current) {
      gizmoSnappedRef.current = false;
      onProjectionChange?.("camera");
    }
  }, [onProjectionChange]);

  return (
    <div className={`h-full w-full ${className}`} aria-label={title}>
      <ThreeCanvas onPointerMissed={clearSelection} orthographic frameloop="demand" camera={{ zoom: 50, position: [10, 10, 10], near: -10000, far: 10000 }} style={{ width: "100%", height: "100%" }}>
        <SceneRepresentationBoundsRegistryProvider>
          <SceneInnerContent
            showGrid={showGrid}
            showGizmo={showGizmo}
            zoomTarget={effectiveZoomTarget}
            snapshot={snapshot}
            camera={camera}
            onCameraChange={onCameraChange}
            onAxisClick={onProjectionChange ? handleAxisClick : undefined}
            onOrbitEnd={onProjectionChange ? handleOrbitEnd : undefined}
          >
            <SceneSelectionBoundsFromRepresentations selectedConnectionIds={selectedConnectionIds} selectedPieceIds={selectedPieceIds} />
            {snapshot.connections.map(({ connection, sourcePiece, targetPiece, status }) => {
              const cid = String(connection.id ?? "");
              if (!cid) return null;
              return (
              <SceneConnection
                key={cid}
                connection={connection}
                sourcePiece={sourcePiece}
                targetPiece={targetPiece}
                status={status}
                isSelected={selectedConnectionIds.has(cid)}
                isHovered={hoveredConnectionId === cid}
                onClick={
                  effectiveConnectionSelectionEnabled || onConnectionClick
                    ? () => {
                        handleSelectConnection(cid);
                        onConnectionClick?.(connection);
                      }
                    : undefined
                }
                onPointerEnter={effectiveConnectionHoverEnabled ? () => handleHoverConnection(cid) : undefined}
                onPointerLeave={effectiveConnectionHoverEnabled ? () => handleHoverConnection((resolvedHover.connectionId ?? null) === cid ? null : (resolvedHover.connectionId ?? null)) : undefined}
              />
            );
            })}
            {pieceAssets.map(({ piece, status, representationName, representationSource }) => {
              const pid = String(piece.id ?? "");
              if (!pid) return null;
              return (
              <ScenePiece
                key={pid}
                piece={piece}
                status={status}
                representationName={representationName}
                representationSource={representationSource}
                isSelected={selectedPieceIds.has(pid)}
                isHovered={hoveredPieceId === pid}
                onClick={
                  effectivePieceSelectionEnabled || onPieceClick
                    ? () => {
                        handleSelectPiece(pid);
                        onPieceClick?.(piece);
                      }
                    : undefined
                }
                onPointerEnter={effectivePieceHoverEnabled ? () => handleHoverPiece(pid) : undefined}
                onPointerLeave={effectivePieceHoverEnabled ? () => handleHoverPiece((resolvedHover.pieceId ?? null) === pid ? null : (resolvedHover.pieceId ?? null)) : undefined}
              />
            );
            })}
          </SceneInnerContent>
        </SceneRepresentationBoundsRegistryProvider>
      </ThreeCanvas>
    </div>
  );
};

// #endregion 📍Scene

// #region 🖋️Representation

// Specs: Representation is a direct alias of SemioScene with a different default title.
// Summary: 3D representation viewer alias of SemioScene.

export type SemioRepresentationProps = SemioSceneProps;

export const SemioRepresentation: React.FC<SemioRepresentationProps> = (props) => <SemioScene {...props} title={props.title ?? "Design Representation"} />;

// #endregion 🖋️Representation

// #region 🧱Type
// Specs: Type is a single-kind 3D surface that mirrors the design scene interaction pattern for
// one kind. It renders the best available representation from the kit at the origin and overlays the kind
// connectors as selectable/hoverable 3D arrows.
// Summary: Single-kind 3D scene with representation preview and interactive connectors.

export interface TypeSelection {
  connectorIds?: string[];
}

export interface TypeHover {
  connectorId?: string | null;
}

export interface SemioTypeProps {
  type: KitKindPlain;
  kit?: Kit;
  selection?: TypeSelection;
  defaultSelection?: TypeSelection;
  selectionEnabled?: boolean;
  connectorSelectionEnabled?: boolean;
  onSelectionChange?: (selection: TypeSelection) => void;
  hover?: TypeHover;
  defaultHover?: TypeHover;
  hoverEnabled?: boolean;
  connectorHoverEnabled?: boolean;
  onHoverChange?: (hover: TypeHover) => void;
  onConnectorClick?: (connector: Connector) => void;
  showRepresentation?: boolean;
  showConnectors?: boolean;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: SceneCamera;
  onCameraChange?: (camera: SceneCamera) => void;
  onProjectionChange?: (projection: "camera" | "orthographic") => void;
  className?: string;
  title?: string;
}

interface TypeRepresentationAsset {
  representationName?: string;
  representationSource?: string;
}

const TYPE_CONNECTOR_ARROW_LENGTH = 0.45;
const TYPE_FALLBACK_BOX_SIZE = 1;
const TYPE_ZOOM_ORIGIN = new THREE.Vector3(0, 0, 0);
const TYPE_ZOOM_PADDING = 0.35;

const normalizeTypeSelection = (selection?: TypeSelection): TypeSelection => ({
  connectorIds: selection?.connectorIds ?? [],
});

const normalizeTypeHover = (hover?: TypeHover): TypeHover => ({
  connectorId: hover?.connectorId ?? null,
});

const buildTypeRepresentationAsset = (kind: KitKindPlain, kit?: Kit): TypeRepresentationAsset => {
  const representations = (kind.representations ?? []) as readonly PlainJsonObject[];
  if (representations.length === 0) return {};

  const filesById = new Map(kitJsRows(kit, "files").filter((f) => String(f.id ?? "").length > 0).map((file) => [String(file.id), file] as const));

  let selectedRepresentation = pickPreferredRepresentation(representations, []);
  const firstRep = selectedRepresentation as { file?: { id?: string } } | undefined;
  let file = firstRep?.file?.id ? (filesById.get(String(firstRep.file.id)) as SemioFile | undefined) : undefined;
  let fileLabel = file && typeof file.name === "string" ? file.name : undefined;
  if (!isSceneGltfSource(getSceneFileSource(file), fileLabel)) {
    for (const candidateRepresentation of representations) {
      const cr = candidateRepresentation as { file?: { id?: string } };
      const candidateFile = cr.file?.id ? (filesById.get(String(cr.file.id)) as SemioFile | undefined) : undefined;
      const candidateLabel = candidateFile && typeof candidateFile.name === "string" ? candidateFile.name : undefined;
      if (candidateFile && isSceneGltfSource(getSceneFileSource(candidateFile), candidateLabel)) {
        selectedRepresentation = candidateRepresentation;
        file = candidateFile;
        fileLabel = candidateLabel;
        break;
      }
    }
  }

  if (!selectedRepresentation) return {};
  return {
    representationName: fileLabel,
    representationSource: getSceneFileSource(file),
  };
};

const buildTypeZoomBox = (kind: KitKindPlain): THREE.Box3 => {
  const box = new THREE.Box3();
  let hasPoints = false;

  (kind.connectors ?? []).forEach((connector) => {
    if (!connector.point || !connector.direction) return;
    const start = toSceneVector(connector.point);
    const directionSource = new THREE.Vector3(connector.direction.x, connector.direction.y, connector.direction.z);
    const direction = directionSource.lengthSq() > 0.000001 ? directionSource.applyMatrix4(SEMIO_TO_THREE_BASIS).normalize() : new THREE.Vector3(0, 0, 1);
    const end = start.clone().add(direction.multiplyScalar(TYPE_CONNECTOR_ARROW_LENGTH));
    box.expandByPoint(start);
    box.expandByPoint(end);
    hasPoints = true;
  });

  if (!hasPoints) {
    box.expandByPoint(TYPE_ZOOM_ORIGIN.clone().addScalar(-TYPE_FALLBACK_BOX_SIZE * 0.5));
    box.expandByPoint(TYPE_ZOOM_ORIGIN.clone().addScalar(TYPE_FALLBACK_BOX_SIZE * 0.5));
    return box;
  }

  box.expandByScalar(TYPE_ZOOM_PADDING);
  return box;
};

const TypeSceneAutoFit: React.FC<{ kind: KitKindPlain }> = ({ kind }) => {
  const bounds = useBounds();
  const fittedRef = React.useRef(false);

  React.useEffect(() => {
    fittedRef.current = false;
  }, [kind.id]);

  React.useEffect(() => {
    if (fittedRef.current) return;
    bounds.refresh(buildTypeZoomBox(kind)).fit();
    fittedRef.current = true;
  }, [bounds, kind]);

  return null;
};

interface TypeConnectorVisualProps {
  connector: Connector;
  isSelected: boolean;
  isHovered: boolean;
  onClick?: () => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
}

const TypeConnectorVisual: React.FC<TypeConnectorVisualProps> = ({ connector, isSelected, isHovered, onClick, onPointerEnter, onPointerLeave }) => {
  const defaultColor = React.useMemo(() => resolveSceneColor(getInteractiveEntityColor("default", false, false), "#888888"), []);
  const selectedColor = React.useMemo(() => resolveSceneColor(getInteractiveEntityColor("default", true, false), "#3b82f6"), []);
  const hoveredColor = React.useMemo(() => resolveSceneColor(getInteractiveEntityColor("default", false, true), "#60a5fa"), []);

  const start = React.useMemo(
    () => (connector.point ? toSceneVector(connector.point) : new THREE.Vector3(0, 0, 0)),
    [connector.point],
  );
  const direction = React.useMemo(() => {
    if (!connector.direction) return new THREE.Vector3(0, 0, 1);
    const baseDirection = new THREE.Vector3(connector.direction.x, connector.direction.y, connector.direction.z);
    if (baseDirection.lengthSq() <= 0.000001) return new THREE.Vector3(0, 0, 1);
    return baseDirection.applyMatrix4(SEMIO_TO_THREE_BASIS).normalize();
  }, [connector.direction]);
  const end = React.useMemo(() => start.clone().add(direction.clone().multiplyScalar(TYPE_CONNECTOR_ARROW_LENGTH)), [direction, start]);

  if (!connector.point || !connector.direction) return null;

  const color = isSelected ? selectedColor : isHovered ? hoveredColor : defaultColor;
  const stemRadius = isSelected ? 0.045 : isHovered ? 0.04 : 0.035;
  const tipRadius = isSelected ? 0.075 : isHovered ? 0.07 : 0.06;
  const rootRadius = isSelected ? 0.06 : isHovered ? 0.055 : 0.05;

  const handleClick = onClick
    ? (event: { stopPropagation: () => void }) => {
        event.stopPropagation();
        onClick();
      }
    : undefined;

  const handlePointerEnter = onPointerEnter
    ? (event: { stopPropagation: () => void }) => {
        event.stopPropagation();
        onPointerEnter();
      }
    : undefined;

  const handlePointerLeave = onPointerLeave
    ? (event: { stopPropagation: () => void }) => {
        event.stopPropagation();
        onPointerLeave();
      }
    : undefined;

  return (
    <group name={connector.id} onClick={handleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
      <mesh position={start.toArray() as [number, number, number]}>
        <sphereGeometry args={[rootRadius, 24, 24]} />
        <meshStandardMaterial color={color} emissive={color} emissiveIntensity={isSelected ? 0.4 : isHovered ? 0.2 : 0.08} />
      </mesh>
      <mesh position={end.toArray() as [number, number, number]}>
        <sphereGeometry args={[tipRadius, 24, 24]} />
        <meshStandardMaterial color={color} emissive={color} emissiveIntensity={isSelected ? 0.4 : isHovered ? 0.2 : 0.08} />
      </mesh>
      <mesh position={start.clone().add(end).multiplyScalar(0.5)} quaternion={new THREE.Quaternion().setFromUnitVectors(new THREE.Vector3(0, 1, 0), end.clone().sub(start).normalize())}>
        <cylinderGeometry args={[stemRadius, stemRadius, start.distanceTo(end), 12]} />
        <meshStandardMaterial color={color} emissive={color} emissiveIntensity={isSelected ? 0.4 : isHovered ? 0.2 : 0.08} />
      </mesh>
    </group>
  );
};

const TypeFallbackRepresentation: React.FC = () => {
  const color = React.useMemo(() => resolveSceneColor("var(--muted-foreground)", "#888888"), []);
  return (
    <mesh>
      <boxGeometry args={[TYPE_FALLBACK_BOX_SIZE, TYPE_FALLBACK_BOX_SIZE, TYPE_FALLBACK_BOX_SIZE]} />
      <meshStandardMaterial color={color} emissive={color} emissiveIntensity={0.05} />
      <Edges scale={1.001} color={color} />
    </mesh>
  );
};

export const SemioType: React.FC<SemioTypeProps> = ({
  type: kind,
  kit,
  selection,
  defaultSelection,
  selectionEnabled = true,
  connectorSelectionEnabled = true,
  onSelectionChange,
  hover,
  defaultHover,
  hoverEnabled = true,
  connectorHoverEnabled = true,
  onHoverChange,
  onConnectorClick,
  showRepresentation = true,
  showConnectors = true,
  showGrid = true,
  showGizmo = true,
  camera,
  onCameraChange,
  onProjectionChange,
  className = "",
  title = "Type",
}) => {
  const [resolvedSelection, setResolvedSelection] = useInteractiveControllableValue(selection, normalizeTypeSelection(defaultSelection), onSelectionChange);
  const [resolvedHover, setResolvedHover] = useInteractiveControllableValue(hover, normalizeTypeHover(defaultHover), onHoverChange);
  const selectedConnectorIds = React.useMemo(() => new Set(selectionEnabled ? (resolvedSelection.connectorIds ?? []) : []), [resolvedSelection.connectorIds, selectionEnabled]);
  const hoveredConnectorId = hoverEnabled && connectorHoverEnabled ? (resolvedHover.connectorId ?? null) : null;
  const effectiveConnectorSelectionEnabled = selectionEnabled && connectorSelectionEnabled;
  const effectiveConnectorHoverEnabled = hoverEnabled && connectorHoverEnabled && (effectiveConnectorSelectionEnabled || !!onConnectorClick);
  const representationAsset = React.useMemo(() => buildTypeRepresentationAsset(kind, kit), [kind, kit]);
  const canRenderRepresentation = React.useMemo(() => isSceneGltfSource(representationAsset.representationSource, representationAsset.representationName), [representationAsset.representationName, representationAsset.representationSource]);

  const clearSelection = React.useCallback(() => {
    if (!selectionEnabled) return;
    setResolvedSelection({ connectorIds: [] });
  }, [selectionEnabled, setResolvedSelection]);

  const handleSelectConnector = React.useCallback(
    (connectorId: string | undefined) => {
      const id = String(connectorId ?? "");
      if (!id) return;
      if (!effectiveConnectorSelectionEnabled) return;
      const nextConnectorIds = new Set(resolvedSelection.connectorIds ?? []);
      if (nextConnectorIds.has(id)) {
        nextConnectorIds.delete(id);
      } else {
        nextConnectorIds.add(id);
      }
      setResolvedSelection({ connectorIds: Array.from(nextConnectorIds) });
    },
    [effectiveConnectorSelectionEnabled, resolvedSelection.connectorIds, setResolvedSelection],
  );

  const handleHoverConnector = React.useCallback(
    (connectorId: string | null) => {
      if (!effectiveConnectorHoverEnabled) return;
      setResolvedHover({ connectorId });
    },
    [effectiveConnectorHoverEnabled, setResolvedHover],
  );

  const gizmoSnappedRef = React.useRef(false);
  const handleAxisClick = React.useCallback(
    (_direction: THREE.Vector3) => {
      gizmoSnappedRef.current = true;
      onProjectionChange?.("orthographic");
    },
    [onProjectionChange],
  );
  const handleOrbitEnd = React.useCallback(() => {
    if (gizmoSnappedRef.current) {
      gizmoSnappedRef.current = false;
      onProjectionChange?.("camera");
    }
  }, [onProjectionChange]);

  return (
    <div className={`h-full w-full ${className}`} aria-label={title}>
      <ThreeCanvas onPointerMissed={clearSelection} orthographic frameloop="demand" camera={{ zoom: 50, position: [10, 10, 10], near: -10000, far: 10000 }} style={{ width: "100%", height: "100%" }}>
        <SceneInnerContent
          showGrid={showGrid}
          showGizmo={showGizmo}
          zoomTarget="none"
          snapshot={{ pieces: [], connections: [] }}
          camera={camera}
          onCameraChange={onCameraChange}
          onAxisClick={onProjectionChange ? handleAxisClick : undefined}
          onOrbitEnd={onProjectionChange ? handleOrbitEnd : undefined}
        >
          {!camera && <TypeSceneAutoFit kind={kind} />}
          {showRepresentation &&
            (canRenderRepresentation && representationAsset.representationSource ? (
              <React.Suspense fallback={null}>
                <ScenePieceRepresentation representationSource={representationAsset.representationSource} status="default" isSelected={false} isHovered={false} />
              </React.Suspense>
            ) : (
              <TypeFallbackRepresentation />
            ))}
          {showConnectors &&
            (kind.connectors ?? []).map((connector, connectorIndex) => {
              const cid = String(connector.id ?? `connector-${connectorIndex}`);
              return (
              <TypeConnectorVisual
                key={cid}
                connector={connector}
                isSelected={selectedConnectorIds.has(cid)}
                isHovered={hoveredConnectorId === cid}
                onClick={
                  effectiveConnectorSelectionEnabled || onConnectorClick
                    ? () => {
                        handleSelectConnector(cid);
                        onConnectorClick?.(connector);
                      }
                    : undefined
                }
                onPointerEnter={effectiveConnectorHoverEnabled ? () => handleHoverConnector(cid) : undefined}
                onPointerLeave={effectiveConnectorHoverEnabled ? () => handleHoverConnector((resolvedHover.connectorId ?? null) === cid ? null : (resolvedHover.connectorId ?? null)) : undefined}
              />
            );
            })}
        </SceneInnerContent>
      </ThreeCanvas>
    </div>
  );
};

// #endregion 🧱Type

// #region 📌Design

// Specs: Split-view design viewer with Diagram on the right and Scene on the left.
// Uses CSS grid for layout. Fully iframe compatible. Selection state is shared between
// the Diagram (2D) and Scene (3D) views. With {@link SemioDesignProps.splitLayout} `auto`,
// the scene column appears only when at least one piece has both plane and center.
// With `always`, the scene column is always allocated ({@link McpDesignViewer} uses this so
// the layout cannot collapse to a single full-width diagram when flattening fails).
// Summary: Combined 2D diagram + 3D scene split view for a design in a kit.

/** How {@link SemioDesign} chooses between one column (diagram only) vs split scene+diagram. */
export type SemioDesignSplitLayout = "auto" | "always";

/**
 * 📋Pure layout rule for {@link SemioDesign} grid columns — unit-tested so MCP cannot regress to diagram-only full width.
 */
export function semioDesignGridTemplateColumns(splitLayout: SemioDesignSplitLayout, hasPlanes: boolean, sceneRatio: number): string {
  const scenePercent = Math.max(0.1, Math.min(0.9, sceneRatio)) * 100;
  const diagramPercent = 100 - scenePercent;
  const showSplit = splitLayout === "always" || hasPlanes;
  return showSplit ? `${scenePercent}% ${diagramPercent}%` : "1fr";
}

/** Whether the scene (3D) column is mounted — must match {@link semioDesignGridTemplateColumns} split vs 1fr. */
export function semioDesignShowSceneColumn(splitLayout: SemioDesignSplitLayout, hasPlanes: boolean): boolean {
  return splitLayout === "always" || hasPlanes;
}

export interface SemioDesignProps {
  design: Design;
  kit?: Kit;
  designDiff?: DesignDiff;
  defaultDesignDiff?: DesignDiff;
  diffEnabled?: boolean;
  zoomTarget?: ZoomTarget;
  selection?: DiagramSelection;
  defaultSelection?: DiagramSelection;
  selectionEnabled?: boolean;
  pieceSelectionEnabled?: boolean;
  connectionSelectionEnabled?: boolean;
  onSelectionChange?: (selection: DiagramSelection) => void;
  hover?: DiagramHover;
  defaultHover?: DiagramHover;
  hoverEnabled?: boolean;
  pieceHoverEnabled?: boolean;
  connectionHoverEnabled?: boolean;
  onHoverChange?: (hover: DiagramHover) => void;
  onPieceClick?: (piece: Piece) => void;
  onPieceDoubleClick?: (piece: Piece) => void;
  onConnectionClick?: (connection: Connection) => void;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: SceneCamera;
  onCameraChange?: (camera: SceneCamera) => void;
  className?: string;
  title?: string;
  sceneRatio?: number;
  /** `auto` (default): hide scene column when no piece has plane+center. `always`: always split (MCP design viewer). */
  splitLayout?: SemioDesignSplitLayout;
}

export const SemioDesign: React.FC<SemioDesignProps> = ({
  design,
  kit,
  designDiff,
  defaultDesignDiff,
  diffEnabled = true,
  zoomTarget,
  selection,
  defaultSelection,
  selectionEnabled = true,
  pieceSelectionEnabled = true,
  connectionSelectionEnabled = true,
  onSelectionChange,
  hover,
  defaultHover,
  hoverEnabled = true,
  pieceHoverEnabled = true,
  connectionHoverEnabled = true,
  onHoverChange,
  onPieceClick,
  onPieceDoubleClick,
  onConnectionClick,
  showGrid = true,
  showGizmo = true,
  camera,
  onCameraChange,
  className = "",
  title = "Design",
  sceneRatio = 0.5,
  splitLayout = "auto",
}) => {
  const resolvedDesignDiff = useResolvedValue(designDiff, defaultDesignDiff);
  const hasPlanes = React.useMemo(() => {
    const effectiveDiff = diffEnabled ? resolvedDesignDiff : undefined;
    const mergedRow = (effectiveDiff ? previewDesignWithDiff(design, effectiveDiff) : design) as PlainJsonObject;
    return __itemsOf(mergedRow["pieces"]).some((p) => (p as Piece).plane && (p as Piece).center);
  }, [design, resolvedDesignDiff, diffEnabled]);

  const showSceneColumn = semioDesignShowSceneColumn(splitLayout, hasPlanes);
  const gridTemplateColumns = semioDesignGridTemplateColumns(splitLayout, hasPlanes, sceneRatio);

  const [resolvedSelection, setResolvedSelection] = useInteractiveControllableValue(selection, normalizeSelection(defaultSelection), onSelectionChange);
  const [resolvedHover, setResolvedHover] = useInteractiveControllableValue(hover, normalizeHover(defaultHover), onHoverChange);

  const designContainerRef = React.useRef<HTMLDivElement | null>(null);
  useDesignClipboard(designContainerRef, design, diffEnabled ? resolvedDesignDiff : undefined, resolvedSelection);

  return (
    <div
      ref={designContainerRef}
      className={`h-full w-full ${className}`}
      aria-label={title}
      tabIndex={0}
      data-semio-design-columns={showSceneColumn ? "2" : "1"}
      style={{
        display: "grid",
        gridTemplateColumns,
        gridTemplateRows: "100%",
        outline: "none",
      }}
    >
      {showSceneColumn && (
        <div style={{ width: "100%", height: "100%", overflow: "hidden", borderRight: "1px solid var(--color-border, var(--border))" }}>
          <SemioScene
            design={design}
            kit={kit}
            designDiff={resolvedDesignDiff}
            diffEnabled={diffEnabled}
            zoomTarget={zoomTarget}
            selection={resolvedSelection}
            hover={resolvedHover}
            selectionEnabled={selectionEnabled}
            pieceSelectionEnabled={pieceSelectionEnabled}
            connectionSelectionEnabled={connectionSelectionEnabled}
            onSelectionChange={setResolvedSelection}
            hoverEnabled={hoverEnabled}
            pieceHoverEnabled={pieceHoverEnabled}
            connectionHoverEnabled={connectionHoverEnabled}
            onHoverChange={setResolvedHover}
            onPieceClick={onPieceClick}
            onPieceDoubleClick={onPieceDoubleClick}
            onConnectionClick={onConnectionClick}
            showGrid={showGrid}
            showGizmo={showGizmo}
            camera={camera}
            onCameraChange={onCameraChange}
            title={`${title} Scene`}
          />
        </div>
      )}
      <div style={{ width: "100%", height: "100%", overflow: "hidden" }}>
        <SemioDiagram
          design={design}
          designDiff={resolvedDesignDiff}
          diffEnabled={diffEnabled}
          zoomTarget={zoomTarget}
          selection={resolvedSelection}
          selectionEnabled={selectionEnabled}
          pieceSelectionEnabled={pieceSelectionEnabled}
          connectionSelectionEnabled={connectionSelectionEnabled}
          onSelectionChange={setResolvedSelection}
          hover={resolvedHover}
          hoverEnabled={hoverEnabled}
          pieceHoverEnabled={pieceHoverEnabled}
          connectionHoverEnabled={connectionHoverEnabled}
          onHoverChange={setResolvedHover}
          onPieceClick={onPieceClick}
          onPieceDoubleClick={onPieceDoubleClick}
          onConnectionClick={onConnectionClick}
          title={`${title} Diagram`}
        />
      </div>
    </div>
  );
};

// #endregion 📌Design

// #region 🗿McpApp
// Specs: MCP App design viewer component using the official @modelcontextprotocol/ext-apps/react
// protocol. Communicates with the MCP host via useApp hook. Receives pre-computed diagram data
// (points and lines) from tool results as JSON text content. Renders pure SVG diagram.
// Summary: MCP App React component for rendering semio diagrams inside MCP host iframes.

import type { App as McpApp } from "@modelcontextprotocol/ext-apps";
import { useApp, useDocumentTheme } from "@modelcontextprotocol/ext-apps/react";

// #region 🔗McpApp Types

interface McpDiagramPoint {
  id: string;
  u: number;
  v: number;
  status: DiagramEntityStatus;
}

interface McpDiagramLine {
  id: string;
  sourceU: number;
  sourceV: number;
  targetU: number;
  targetV: number;
  status: DiagramEntityStatus;
}

/**
 * Payload structure sent as JSON text content in MCP tool results.
 * Contains pre-computed diagram points and lines from the server.
 **/
export interface McpDiagramPayload {
  points: McpDiagramPoint[];
  lines: McpDiagramLine[];
  mode?: string;
  /** Engine-provided viewer surface; survives host merges when `mode` is wrong. `design` = {@link SemioDesign} (scene + diagram). */
  surface?: "design" | "scene" | "diagram";
  capabilities?: {
    pieceSelection?: boolean;
    connectionSelection?: boolean;
  };
  kitArtifacts?: KitData;
  design?: Design;
  designDiff?: DesignDiff;
  kit?: Kit;
  fetchUrl?: string;
}

// #endregion 🔗McpApp Types

/**
 * True when `kitArtifacts` is missing usable data (hosts may send a shell object after stripping nested arrays).
 **/
const isEmptyKitArtifactsData = (ka: unknown): boolean => {
  if (ka === null || ka === undefined) return true;
  if (typeof ka !== "object" || Array.isArray(ka)) return true;
  const o = ka as Record<string, unknown>;
  const name = typeof o.name === "string" ? o.name.trim() : "";
  const id = typeof o.id === "string" ? o.id.trim() : "";
  const d = Array.isArray(o.designs) ? o.designs.length : 0;
  const t = Array.isArray(o.types) ? o.types.length : 0;
  const kp = Array.isArray(o.ports) ? o.ports.length : 0;
  const c = Array.isArray(o.connectors) ? o.connectors.length : 0;
  return name.length === 0 && id.length === 0 && d === 0 && t === 0 && kp === 0 && c === 0;
};

/**
 * Normalizes a loose object into {@link McpDiagramPayload} when it carries kit/diagram data.
 * Hosts may send only {@link McpDiagramPayload.kitArtifacts} or omit empty arrays.
 **/
const normalizeMcpDiagramPayload = (raw: Record<string, unknown>): McpDiagramPayload | null => {
  const kitObj = raw.kitArtifacts;
  const hasKit = kitObj !== undefined && kitObj !== null && typeof kitObj === "object" && !Array.isArray(kitObj) && !isEmptyKitArtifactsData(kitObj);
  const hasDiagram = Array.isArray(raw.points) && Array.isArray(raw.lines);
  const mode = typeof raw.mode === "string" ? raw.mode : undefined;
  const designRaw = raw.design;
  const hasDesign = designRaw !== undefined && designRaw !== null && typeof designRaw === "object" && !Array.isArray(designRaw);
  const designDiffRaw = raw.designDiff;
  const hasDesignDiff = designDiffRaw !== undefined && designDiffRaw !== null && typeof designDiffRaw === "object" && !Array.isArray(designDiffRaw);
  const kitRaw = raw.kit;
  const hasFullKit = kitRaw !== undefined && kitRaw !== null && typeof kitRaw === "object" && !Array.isArray(kitRaw);
  const fetchUrl = typeof raw.fetchUrl === "string" ? raw.fetchUrl : undefined;
  const surfaceRaw = raw.surface;
  const surface = surfaceRaw === "design" || surfaceRaw === "scene" || surfaceRaw === "diagram" ? surfaceRaw : undefined;
  if (!hasKit && !hasDiagram && !hasDesign && !fetchUrl && !surface) return null;
  return {
    points: Array.isArray(raw.points) ? (raw.points as McpDiagramPayload["points"]) : [],
    lines: Array.isArray(raw.lines) ? (raw.lines as McpDiagramPayload["lines"]) : [],
    capabilities: raw.capabilities as McpDiagramPayload["capabilities"],
    kitArtifacts: hasKit ? (raw.kitArtifacts as KitData) : undefined,
    mode,
    surface,
    design: hasDesign ? (designRaw as Design) : undefined,
    designDiff: hasDesignDiff ? (designDiffRaw as DesignDiff) : undefined,
    kit: hasFullKit ? (kitRaw as Kit) : undefined,
    fetchUrl,
  };
};

/**
 * Prefer the richest tool payload when hosts duplicate data in `structuredContent` (often truncated) and `content` text (full JSON).
 **/
/**
 * Prefer MCP tool payloads whose embedded {@link Design} includes full piece/connection lists (hosts often truncate `structuredContent`).
 **/
const mcpDesignRichness = (p: McpDiagramPayload): number => {
  const d = p.design;
  if (!d || typeof d !== "object") return 0;
  const o = d as unknown as { pieces?: unknown[]; connections?: unknown[] };
  const pieces = Array.isArray(o.pieces) ? o.pieces.length : 0;
  const conns = Array.isArray(o.connections) ? o.connections.length : 0;
  return pieces * 20 + conns * 10;
};

/**
 * Count pieces that have both plane and center — required for {@link SemioDesign} to show the 3D scene panel.
 * Strips often keep piece ids for diagram but drop nested plane data; hosts then merge a high piece-count shell that loses the scene.
 **/
const mcpSceneGeometryRichness = (p: McpDiagramPayload): number => {
  const d = p.design;
  if (!d || typeof d !== "object") return 0;
  const pieces = (d as { pieces?: unknown[] }).pieces;
  if (!Array.isArray(pieces)) return 0;
  let n = 0;
  for (const piece of pieces) {
    if (!piece || typeof piece !== "object") continue;
    const pl = (piece as { plane?: unknown }).plane;
    const c = (piece as { center?: unknown }).center;
    if (pl && c) n += 1;
  }
  return n * 200;
};

/** Prefer merged `design` that maximizes diagram rows plus 3D scene placement data. */
const mcpDesignMergeScore = (p: McpDiagramPayload): number => {
  return mcpDesignRichness(p) + mcpSceneGeometryRichness(p);
};

const scoreMcpDiagramPayload = (p: McpDiagramPayload): number => {
  let s = 0;
  const diagramShape = Array.isArray(p.points) && Array.isArray(p.lines);
  if (diagramShape) s += 1;
  s += p.points.length + p.lines.length;
  if (p.surface === "design") s += 500;
  else if (p.surface === "scene") s += 200;
  const ka = p.kitArtifacts;
  if (ka) {
    s += (ka.designs?.length ?? 0) * 10 + (ka.types?.length ?? 0) * 10 + (ka.ports?.length ?? 0) + (ka.connectors?.length ?? 0);
    if (typeof ka.name === "string" && ka.name.trim().length > 0) s += 50;
    if (typeof ka.id === "string" && ka.id.trim().length > 0) s += 10;
  }
  if (p.mode === "show-design" || p.mode === "show-scene") s += 400;
  if (p.design) s += 300;
  if (p.kit) s += 100;
  s += mcpDesignRichness(p);
  s += mcpSceneGeometryRichness(p);
  return s;
};

/**
 * Decide whether McpDesignViewer should render SemioKit as a fallback.
 *
 * Specs:
 * - Only diagram/selection modes may fall back to SemioKit when the diagram is empty.
 * - Never fall back to SemioKit for show-design/show-scene intents — those should render Design/Scene or a loading shell.
 **/
const canDisplayKitArtifactsFallback = (mode: string | undefined, hasDiagram: boolean, designId: string | undefined): boolean => {
  if (hasDiagram) return false;
  /** Stale kit payloads score high; merged `design` may still leave `mode` as show-diagram — never show Kit when we have a design to render. */
  if (designId) return false;
  return mode === "show-diagram" || mode === "show-diagram-diff" || mode === "select-pieces" || mode === "select-connections" || mode === "select-pieces-and-connections";
};

/**
 * After scoring, take the richest `design` among all parse candidates (same tool result, different channels).
 **/
const _DESIGN_INTENT_MODES_FOR_MERGE = new Set(["show-design", "show-scene", "show-diff", "show-diagram-diff"]);

const mergeRichestDesignFromCandidates = (candidates: Array<McpDiagramPayload | null | undefined>, best: McpDiagramPayload | null): McpDiagramPayload | null => {
  if (!best) return null;
  let merged = best;
  let bestDesignScore = mcpDesignMergeScore(best);
  for (const c of candidates) {
    if (!c) continue;
    if (c.design) {
      const score = mcpDesignMergeScore({ ...merged, design: c.design });
      if (score > bestDesignScore) {
        bestDesignScore = score;
        merged = { ...merged, design: c.design };
        if (c.surface) merged = { ...merged, surface: c.surface };
      }
    }
    if (!merged.fetchUrl && c.fetchUrl) merged = { ...merged, fetchUrl: c.fetchUrl };
    if (!merged.kit && c.kit) merged = { ...merged, kit: c.kit };
    if (!merged.mode && c.mode) merged = { ...merged, mode: c.mode };
    const mp = merged.points?.length ?? 0;
    const cp = c.points?.length ?? 0;
    if (cp > mp && Array.isArray(c.points)) {
      merged = { ...merged, points: c.points, lines: Array.isArray(c.lines) ? c.lines : (merged.lines ?? []) };
    }
  }
  /** Best-scoring candidate is often a stale kit shell (`show-diagram`); another channel may carry `show-design` + same design id. */
  const dg = merged.design && typeof merged.design === "object" ? (merged.design as { id?: string }).id : undefined;
  if (typeof dg === "string" && dg.length > 0) {
    // MCP intent mapping: an explicit `show-scene` request must win over `show-design`,
    // 📦otherwise the host can accidentally keep diagram-mode / design-mode payloads.
    const priority: Record<string, number> = { "show-scene": 1, "show-design": 2, "show-diff": 3, "show-diagram-diff": 4 };
    let picked: string | undefined;
    let pickedPri = 999;
    for (const c of candidates) {
      if (!c?.mode || !c.design || typeof c.design !== "object") continue;
      const cg = (c.design as { id?: string }).id;
      if (cg !== dg) continue;
      if (!_DESIGN_INTENT_MODES_FOR_MERGE.has(c.mode)) continue;
      const pr = priority[c.mode] ?? 99;
      if (!picked || pr < pickedPri) {
        picked = c.mode;
        pickedPri = pr;
      }
    }
    if (picked) merged = { ...merged, mode: picked };
  }
  const dg2 = merged.design && typeof merged.design === "object" ? (merged.design as { id?: string }).id : undefined;
  if (typeof dg2 === "string" && dg2.length > 0) {
    const surfPri: Record<string, number> = { scene: 1, design: 2, diagram: 3 };
    let pickedSurf: McpDiagramPayload["surface"] | undefined;
    let pickedSurfPri = 999;
    for (const c of candidates) {
      if (!c?.surface || !c.design || typeof c.design !== "object") continue;
      const cg = (c.design as { id?: string }).id;
      if (cg !== dg2) continue;
      if (c.surface !== "design" && c.surface !== "scene" && c.surface !== "diagram") continue;
      const pr = surfPri[c.surface] ?? 99;
      if (!pickedSurf || pr < pickedSurfPri) {
        pickedSurf = c.surface;
        pickedSurfPri = pr;
      }
    }
    if (pickedSurf) merged = { ...merged, surface: pickedSurf };
  }
  const dgPick = merged.design && typeof merged.design === "object" ? (merged.design as { id?: string }).id : undefined;
  if (typeof dgPick === "string" && dgPick.length > 0) {
    const initialSceneGeom = mcpSceneGeometryRichness({ ...merged, design: merged.design as Design });
    let bestDesignForGeom = merged.design as Design;
    let bestGeom = initialSceneGeom;
    for (const c of candidates) {
      if (!c?.design || typeof c.design !== "object") continue;
      if ((c.design as { id?: string }).id !== dgPick) continue;
      const g = mcpSceneGeometryRichness({ ...merged, design: c.design as Design });
      if (g > bestGeom) {
        bestGeom = g;
        bestDesignForGeom = c.design as Design;
      }
    }
    if (bestGeom > initialSceneGeom) {
      merged = { ...merged, design: bestDesignForGeom };
    }
  }
  /** Prefer the richest `kitArtifacts` among candidates (hint channel may omit kit while another carries the full artifact tree). */
  let bestKitArtifacts: KitData | undefined = merged.kitArtifacts && !isEmptyKitArtifactsData(merged.kitArtifacts) ? merged.kitArtifacts : undefined;
  let bestKitScore = bestKitArtifacts !== undefined ? scoreMcpDiagramPayload({ points: [], lines: [], kitArtifacts: bestKitArtifacts }) : -1;
  for (const c of candidates) {
    if (!c?.kitArtifacts || isEmptyKitArtifactsData(c.kitArtifacts)) continue;
    const sc = scoreMcpDiagramPayload({ points: [], lines: [], kitArtifacts: c.kitArtifacts });
    if (sc > bestKitScore) {
      bestKitScore = sc;
      bestKitArtifacts = c.kitArtifacts;
    }
  }
  if (bestKitArtifacts !== undefined) {
    merged = { ...merged, kitArtifacts: bestKitArtifacts };
  }
  /** Keep `surface` consistent with authoritative `mode` after cross-candidate merge (hosts may leave stale `surface: diagram`). */
  const m = merged.mode ?? "show-diagram";
  if (m === "show-design" || m === "show-diff" || m === "show-diagram-diff") {
    merged = { ...merged, surface: "design" };
  } else if (m === "show-scene") {
    merged = { ...merged, surface: "scene" };
  }
  return merged;
};

const bestMcpDiagramPayload = (candidates: Array<McpDiagramPayload | null | undefined>): McpDiagramPayload | null => {
  let best: McpDiagramPayload | null = null;
  let bestScore = -1;
  for (const c of candidates) {
    if (!c) continue;
    const sc = scoreMcpDiagramPayload(c);
    if (sc > bestScore) {
      bestScore = sc;
      best = c;
    }
  }
  return bestScore >= 0 ? best : null;
};

/**
 * True when {@link McpKitViewer} has enough {@link KitData} to render SemioKit (not a stripped host shell).
 **/
const isKitViewerPayloadSufficient = (p: McpDiagramPayload | null): boolean => {
  if (!p?.kitArtifacts) return false;
  return !isEmptyKitArtifactsData(p.kitArtifacts);
};

/**
 * Some MCP hosts never send `ui/notifications/tool-input` with arguments; the path may only appear nested in host context.
 **/
const deepFindKitToolArguments = (obj: unknown, depth = 0): Record<string, unknown> | null => {
  if (depth > 12 || obj === null || typeof obj !== "object") return null;
  if (Array.isArray(obj)) {
    for (const item of obj) {
      const r = deepFindKitToolArguments(item, depth + 1);
      if (r) return r;
    }
    return null;
  }
  const rec = obj as Record<string, unknown>;
  if (typeof rec.path === "string" && rec.path.trim().length > 0) {
    return { path: rec.path.trim() };
  }
  if (typeof rec.serverUrl === "string" && typeof rec.kitUri === "string" && rec.serverUrl.trim().length > 0 && rec.kitUri.trim().length > 0) {
    return { serverUrl: rec.serverUrl.trim(), kitUri: rec.kitUri.trim() };
  }
  for (const v of Object.values(rec)) {
    const r = deepFindKitToolArguments(v, depth + 1);
    if (r) return r;
  }
  return null;
};

/**
 * Deep-scans nested objects for a payload shape (some hosts nest JSON under extra keys).
 **/
const deepFindDiagramPayload = (obj: unknown, depth = 0): McpDiagramPayload | null => {
  if (depth > 12 || obj === null || typeof obj !== "object") return null;
  if (Array.isArray(obj)) {
    for (const item of obj) {
      const found = deepFindDiagramPayload(item, depth + 1);
      if (found) return found;
    }
    return null;
  }
  const rec = obj as Record<string, unknown>;
  const direct = normalizeMcpDiagramPayload(rec);
  if (direct) return direct;
  for (const v of Object.values(rec)) {
    const found = deepFindDiagramPayload(v, depth + 1);
    if (found) return found;
  }
  return null;
};

/**
 * Parses MCP `CallToolResult` (or host copies) into {@link McpDiagramPayload}.
 * Supports `content` text blocks and `structuredContent` (used by several MCP hosts instead of text).
 **/
export const parseDiagramPayloadFromToolResult = (result: unknown): McpDiagramPayload | null => {
  if (!result || typeof result !== "object") return null;
  let r = result as Record<string, unknown>;
  const params = r.params;
  if (params && typeof params === "object" && ("content" in params || "structuredContent" in params)) {
    r = params as Record<string, unknown>;
  }

  const candidates: Array<McpDiagramPayload | null | undefined> = [];

  const structured = r.structuredContent;
  if (structured !== undefined && structured !== null) {
    if (typeof structured === "string") {
      try {
        const parsed = JSON.parse(structured) as unknown;
        if (parsed && typeof parsed === "object") {
          candidates.push(normalizeMcpDiagramPayload(parsed as Record<string, unknown>));
        }
      } catch {
        /* ignore */
      }
    } else if (typeof structured === "object" && !Array.isArray(structured)) {
      candidates.push(normalizeMcpDiagramPayload(structured as Record<string, unknown>));
    }
  }

  const content = r.content;
  if (Array.isArray(content)) {
    const textParts: string[] = [];
    for (const block of content) {
      if (!block || typeof block !== "object") continue;
      const b = block as { type?: string; text?: string; resource?: { text?: string } };
      if (b.type === "text" && typeof b.text === "string") textParts.push(b.text);
      if (b.type === "resource" && b.resource && typeof b.resource.text === "string") textParts.push(b.resource.text);
      if (!b.type && b.resource && typeof b.resource.text === "string") textParts.push(b.resource.text);
    }
    for (const seg of textParts) {
      const t = seg.trim();
      if (t.length === 0) continue;
      try {
        const parsed = JSON.parse(t) as unknown;
        if (parsed && typeof parsed === "object") {
          candidates.push(normalizeMcpDiagramPayload(parsed as Record<string, unknown>));
        }
      } catch {
        /* ignore */
      }
    }
    const joined = textParts.join("").trim();
    if (joined.length > 0) {
      try {
        const parsed = JSON.parse(joined) as unknown;
        if (parsed && typeof parsed === "object") {
          candidates.push(normalizeMcpDiagramPayload(parsed as Record<string, unknown>));
        }
      } catch {
        /* fall through */
      }
    }
  }

  candidates.push(normalizeMcpDiagramPayload(r));
  candidates.push(deepFindDiagramPayload(r));

  return mergeRichestDesignFromCandidates(candidates, bestMcpDiagramPayload(candidates));
};

/**
 * MCP App design viewer that renders a semio diagram using the official MCP Apps protocol.
 * Uses useApp from @modelcontextprotocol/ext-apps/react for host communication.
 * Receives pre-computed diagram data (points and lines) from tool results.
 *
 * Specs:
 * - Connects to MCP host via useApp hook with PostMessageTransport.
 * - Receives pre-computed diagram points/lines from tool results via ontoolresult callback.
 * - Maps each merged {@link McpDiagramPayload} through {@link mcpMapPayloadToDesignViewerViewRepresentation} → {@link SemioDesign} (split), {@link SemioScene}, {@link SemioDiagram}, or {@link SemioKit} fallback; no parallel ad-hoc surface/flatten rules in the component.
 * - Applies {@link Kit.runFlattenDesign} + {@link Design.applyDiff} inside the mapper for design/scene so pieces gain plane/center when the kit supplies geometry (see {@link mcpFlattenDesignForSemioSurface}).
 * - Refetches `show_design` via {@link McpApp.callServerTool} on a staggered schedule — hosts often pass a truncated `structuredContent` blob (one piece / one diagram node) while the engine has the full design in-session.
 * - Sends selection changes back to host via updateRepresentationContext.
 **/
/**
 * 🔗Resolves which {@link SemioDesign} / {@link SemioScene} / {@link SemioDiagram} shell to mount.
 * `mode` wins over `surface` when hosts merge stale `surface: diagram` with `show-design` payloads.
 * Exported for unit tests.
 */
export function mcpEffectiveSurface(p: McpDiagramPayload | null | undefined): "design" | "scene" | "diagram" {
  if (!p) return "diagram";
  const mode = p.mode ?? "show-diagram";
  if (mode === "show-design" || mode === "show-diff" || mode === "show-diagram-diff") return "design";
  if (mode === "show-scene") return "scene";
  if (mode === "show-diagram") {
    if (p.surface === "design" || p.surface === "scene") return p.surface;
    return "diagram";
  }
  if (p.surface === "design" || p.surface === "scene" || p.surface === "diagram") return p.surface;
  return "diagram";
}

/**
 * 👁️Maps a normalized {@link McpDiagramPayload} to props for semio/ui shells ({@link SemioDesign}, {@link SemioScene}, {@link SemioDiagram}, {@link SemioKit} fallback).
 * Single place for flatten + diagram fallback so {@link McpDesignViewer} stays a thin host bridge.
 */
export function mcpMapPayloadToDesignViewerViewRepresentation(p: McpDiagramPayload): {
  surface: "design" | "scene" | "diagram";
  design: Design | undefined;
  designFlat: Design | undefined;
  kit: Kit | undefined;
  designDiff: DesignDiff | undefined;
  isDiff: boolean;
  diagramDesign: PlainJsonObject;
  forKitFallback: boolean;
} {
  const surface = mcpEffectiveSurface(p);
  const mode = p.mode ?? "show-diagram";
  const kit = p.kit;
  const design = p.design as Design | undefined;
  const isDiff = mode === "show-diff" || mode === "show-diagram-diff";
  const designFlat =
    design && kit
      ? mcpFlattenDesignForSemioSurface(design, kit as Kit, surface, isDiff ? p.designDiff : undefined)
      : isDiff && design && p.designDiff
        ? cloneDesignApplyDiff(design, p.designDiff)
        : undefined;
  const designId = design && typeof design === "object" && "id" in design && typeof (design as { id?: unknown }).id === "string" ? (design as { id: string }).id : undefined;
  const hasDiagramPoints = (p.points?.length ?? 0) > 0;
  const fallbackDesign: Design = {
    id: "__mcp__",
    pieces: p.points.map((pt) => ({ id: pt.id, center: { u: pt.u, v: pt.v } })),
    connections: p.lines.map((l) => ({
      id: l.id,
      connected: { piece: { id: p.points.find((q) => q.u === l.sourceU && q.v === l.sourceV)?.id ?? "" } },
      connecting: { piece: { id: p.points.find((q) => q.u === l.targetU && q.v === l.targetV)?.id ?? "" } },
    })),
  } as unknown as Design;
  // Prefer JS-flattened design (correct BFS placement centers) over raw Python-enriched design, over points fallback.
  // If the chosen design has no pieces with centers, fall back to the pre-computed points/lines
  // 🔷which always have coordinates (hosts may truncate the design or flatten may fail).
  const candidateDesign = (designFlat ?? design) as Design | undefined;
  const candidateHasCenters = __itemsOf((candidateDesign as PlainJsonObject | undefined)?.["pieces"]).some((pc) => !!(pc as Piece).center);
  const diagramDesign = (candidateHasCenters ? candidateDesign! : hasDiagramPoints ? fallbackDesign : (candidateDesign ?? fallbackDesign)) as PlainJsonObject;
  const forKitFallback = surface === "diagram" && Boolean(p.kitArtifacts && canDisplayKitArtifactsFallback(p.mode, hasDiagramPoints, designId));
  return {
    surface,
    design,
    designFlat,
    kit,
    designDiff: p.designDiff,
    isDiff,
    diagramDesign,
    forKitFallback,
  };
}

/**
 * @emoji 🔶 Storybook design flow previously merged kit-backed flatten caches; MCP path now previews diffs without legacy `KitStore`.
 * Exported for unit tests to cover the "MCP kit missing design entry" scenario.
 */
export function mcpFlattenDesignForSemioSurface(design: Design, kit: unknown | undefined, surface: "design" | "scene" | "diagram", diff?: DesignDiff): Design {
  void surface;
  void kit;
  if (!diff) return design;
  return previewDesignWithDiff(design, diff) as Design;
}

export const McpDesignViewer: React.FC = () => {
  const [payload, setPayload] = React.useState<McpDiagramPayload | null>(null);
  const [selectedPieces, setSelectedPieces] = React.useState<Set<string>>(new Set());
  const [selectedConnections, setSelectedConnections] = React.useState<Set<string>>(new Set());
  const [kitSelection, setKitSelection] = React.useState<KitSelection>({ designIds: [], typeIds: [], portIds: [], connectorIds: [] });
  const appRef = React.useRef<McpApp | null>(null);
  const tryRefetchRef = React.useRef<() => void>(() => {});
  const lastDiagramPayloadScoreRef = React.useRef<number>(-1);

  const mergeDiagramPayload = React.useCallback((p: McpDiagramPayload) => {
    setPayload((cur) => {
      if (!cur) return p;
      const best = scoreMcpDiagramPayload(p) > scoreMcpDiagramPayload(cur) ? p : cur;
      return mergeRichestDesignFromCandidates([cur, p], best) ?? p;
    });
  }, []);

  const fetchedUrlsRef = React.useRef<Set<string>>(new Set());

  React.useEffect(() => {
    if (!payload?.fetchUrl || fetchedUrlsRef.current.has(payload.fetchUrl)) return;
    const url = payload.fetchUrl;
    fetchedUrlsRef.current.add(url);
    (async () => {
      try {
        const res = await fetch(url);
        if (!res.ok) return;
        const full = (await res.json()) as Record<string, unknown>;
        const p = normalizeMcpDiagramPayload(full);
        if (p) mergeDiagramPayload(p);
      } catch {
        /* Engine may not be reachable from iframe. */
      }
    })();
  }, [payload?.fetchUrl, mergeDiagramPayload]);

  React.useEffect(() => {
    if (!payload) return;
    const s = scoreMcpDiagramPayload(payload);
    if (s !== lastDiagramPayloadScoreRef.current) {
      lastDiagramPayloadScoreRef.current = s;
      setSelectedPieces(new Set());
      setSelectedConnections(new Set());
      setKitSelection({ designIds: [], typeIds: [], portIds: [], connectorIds: [] });
    }
  }, [payload]);

  const tryRefetchDesignFromServer = React.useCallback(async () => {
    const client = appRef.current;
    if (!client) return;
    try {
      const result = await client.callServerTool({ name: "show_design", arguments: {} });
      const p = parseDiagramPayloadFromToolResult(result);
      if (p) mergeDiagramPayload(p);
    } catch {
      /* Host may not proxy tools/call to the server for this session. */
    }
  }, [mergeDiagramPayload]);

  React.useEffect(() => {
    tryRefetchRef.current = () => {
      void tryRefetchDesignFromServer();
    };
  }, [tryRefetchDesignFromServer]);

  const { app, isConnected, error } = useApp({
    appInfo: { name: "semio design viewer", version: "1.0.0" },
    capabilities: {},
    onAppCreated: (a) => {
      appRef.current = a;
      a.ontoolinput = () => {
        tryRefetchRef.current();
      };
      a.ontoolinputpartial = () => {
        tryRefetchRef.current();
      };
      a.onhostcontextchanged = () => {
        tryRefetchRef.current();
      };
      a.ontoolresult = (result) => {
        const parsed = parseDiagramPayloadFromToolResult(result);
        if (parsed) {
          mergeDiagramPayload(parsed);
        }
      };
      a.ontoolcancelled = () => {};
      a.onteardown = async () => ({});
      a.onerror = console.error;
    },
  });

  /* Host styles intentionally not applied — semio uses its own theme from globals.css. */

  React.useEffect(() => {
    if (!app) return;
    const h = app.getHostContext() as Record<string, unknown> | undefined;
    if (!h) return;
    let best: McpDiagramPayload | null = null;
    for (const k of ["toolResult", "lastToolResult", "toolExecutionResult", "initialToolResult", "pendingToolResult"]) {
      const v = h[k];
      if (v === undefined) continue;
      const p = parseDiagramPayloadFromToolResult(v);
      if (p && (!best || scoreMcpDiagramPayload(p) > scoreMcpDiagramPayload(best))) best = p;
    }
    if (best) mergeDiagramPayload(best);
  }, [app, mergeDiagramPayload]);

  React.useEffect(() => {
    if (!app || !isConnected) return;
    const delays = [0, 50, 150, 400, 1200, 2500, 5000, 8000, 15000];
    const ids = delays.map((d) => setTimeout(() => tryRefetchRef.current(), d));
    return () => ids.forEach(clearTimeout);
  }, [app, isConnected]);

  // 🏛️Sync MCP host theme with semio's .dark class convention.
  const mcpTheme = useDocumentTheme();
  React.useEffect(() => {
    const el = document.documentElement;
    if (mcpTheme === "dark") {
      el.classList.add("dark");
    } else {
      el.classList.remove("dark");
    }
  }, [mcpTheme]);

  const sendSelectionUpdate = React.useCallback((pieces: Set<string>, connections: Set<string>) => {
    if (appRef.current) {
      (appRef.current as { updateRepresentationContext?: (p: unknown) => void })?.updateRepresentationContext?.({
        content: [{ type: "text" as const, text: JSON.stringify({ selectionChange: { pieceIds: Array.from(pieces), connectionIds: Array.from(connections) } }) }],
      });
    }
  }, []);

  const sendKitSelectionUpdate = React.useCallback((next: KitSelection) => {
    if (!appRef.current) return;
    (appRef.current as { updateRepresentationContext?: (p: unknown) => void })?.updateRepresentationContext?.({
      content: [
        {
          type: "text" as const,
          text: JSON.stringify({
            kitArtifactSelectionChange: {
              designIds: next.designIds ?? [],
              typeIds: next.typeIds ?? [],
              portIds: next.portIds ?? [],
              connectorIds: next.connectorIds ?? [],
            },
          }),
        },
      ],
    });
  }, []);

  const viewerViewRepresentation = React.useMemo(() => (payload ? mcpMapPayloadToDesignViewerViewRepresentation(payload) : null), [payload]);

  if (error) {
    return (
      <div style={{ display: "flex", alignItems: "center", justifyContent: "center", height: "100vh", fontFamily: "var(--font-sans, ui-sans-serif, system-ui, sans-serif)", background: "var(--base)", color: "var(--destructive-foreground)" }}>
        <p>Error: {error.message}</p>
      </div>
    );
  }

  if (!isConnected || !app) {
    return (
      <div style={{ display: "flex", alignItems: "center", justifyContent: "center", height: "100vh", fontFamily: "var(--font-sans, ui-sans-serif, system-ui, sans-serif)", background: "var(--base)", color: "var(--muted-foreground)" }}>
        <p>Connecting to host…</p>
      </div>
    );
  }

  if (!payload) {
    return (
      <div style={{ display: "flex", alignItems: "center", justifyContent: "center", height: "100vh", fontFamily: "var(--font-sans, ui-sans-serif, system-ui, sans-serif)", background: "var(--base)", color: "var(--muted-foreground)" }}>
        <p>Waiting for design data…</p>
      </div>
    );
  }

  const pieceSelectionEnabled = payload.capabilities?.pieceSelection ?? false;
  const connectionSelectionEnabled = payload.capabilities?.connectionSelection ?? false;

  const handleKitSelectionChange = (next: KitSelection) => {
    setKitSelection(next);
    sendKitSelectionUpdate(next);
  };

  const selectionProps = {
    selectionEnabled: pieceSelectionEnabled || connectionSelectionEnabled,
    pieceSelectionEnabled,
    connectionSelectionEnabled,
    selection: {
      pieceIds: Array.from(selectedPieces),
      connectionIds: Array.from(selectedConnections),
    },
    onSelectionChange: (next: DiagramSelection) => {
      const nextPieces = new Set(next.pieceIds ?? []);
      const nextConns = new Set(next.connectionIds ?? []);
      setSelectedPieces(nextPieces);
      setSelectedConnections(nextConns);
      sendSelectionUpdate(nextPieces, nextConns);
    },
  };

  const mode = payload.mode ?? "show-diagram";
  const vm = viewerViewRepresentation!;

  if (vm.surface === "design") {
    if (!vm.design) {
      return (
        <div style={{ display: "flex", alignItems: "center", justifyContent: "center", height: "100vh", fontFamily: "var(--font-sans, ui-sans-serif, system-ui, sans-serif)", background: "var(--base)", color: "var(--muted-foreground)" }}>
          <p>Loading {mode === "show-design" ? "design" : "diff"}…</p>
        </div>
      );
    }
    return (
      <div style={{ width: "100%", height: "100vh", position: "relative" }}>
        <SemioDesign design={(vm.designFlat ?? vm.design) as Design} kit={vm.kit} designDiff={vm.isDiff ? vm.designDiff : undefined} splitLayout="always" {...selectionProps} />
      </div>
    );
  }

  if (vm.surface === "scene") {
    if (!vm.design) {
      return (
        <div style={{ display: "flex", alignItems: "center", justifyContent: "center", height: "100vh", fontFamily: "var(--font-sans, ui-sans-serif, system-ui, sans-serif)", background: "var(--base)", color: "var(--muted-foreground)" }}>
          <p>Loading scene…</p>
        </div>
      );
    }
    return (
      <div style={{ width: "100%", height: "100vh", position: "relative" }}>
        <SemioScene design={(vm.designFlat ?? vm.design) as Design} kit={vm.kit} {...selectionProps} />
      </div>
    );
  }

  if (vm.forKitFallback && payload.kitArtifacts) {
    return (
      <div style={{ width: "100%", height: "100vh", overflow: "auto", padding: 12, background: "var(--base)", color: "var(--foreground)" }}>
        <SemioKit data={payload.kitArtifacts} selection={kitSelection} onSelectionChange={handleKitSelectionChange} title="Kit Artifacts" />
      </div>
    );
  }

  return (
    <div style={{ width: "100%", height: "100vh", position: "relative" }}>
      <SemioDiagram design={vm.diagramDesign as Design} designDiff={vm.isDiff ? vm.designDiff : undefined} {...selectionProps} />
    </div>
  );
};

/**
 * MCP App kit viewer: renders only {@link SemioKit} from tool results (kit artifact payload).
 * Used when the MCP host loads `ui://semio/kit-viewer` after kit-scoped tools such as start_working_in_local_kit.
 *
 * Specs:
 * - Same host connection as {@link McpDesignViewer} but no diagram; kit selection sync only.
 **/
export const McpKitViewer: React.FC = () => {
  const [payload, setPayload] = React.useState<McpDiagramPayload | null>(null);
  const [kitSelection, setKitSelection] = React.useState<KitSelection>({ designIds: [], typeIds: [], portIds: [], connectorIds: [] });
  const appRef = React.useRef<McpApp | null>(null);
  const toolInputArgsRef = React.useRef<Record<string, unknown> | null>(null);
  const gotPayloadRef = React.useRef(false);
  const tryRefetchRef = React.useRef<() => void>(() => {});

  const mergeKitToolArguments = React.useCallback((): Record<string, unknown> | null => {
    const client = appRef.current;
    const fromRef = toolInputArgsRef.current;
    const ctx = client?.getHostContext();
    const extracted = deepFindKitToolArguments(ctx);
    if (!fromRef && !extracted) return null;
    return { ...(extracted ?? {}), ...(fromRef ?? {}) };
  }, []);

  const mergeKitViewerPayload = React.useCallback((p: McpDiagramPayload | null | undefined) => {
    if (!p) return;
    setPayload((cur) => {
      if (!cur) {
        if (isKitViewerPayloadSufficient(p)) gotPayloadRef.current = true;
        return p;
      }
      const best = scoreMcpDiagramPayload(p) > scoreMcpDiagramPayload(cur) ? p : cur;
      const merged = mergeRichestDesignFromCandidates([cur, p], best) ?? p;
      if (isKitViewerPayloadSufficient(merged)) gotPayloadRef.current = true;
      return merged;
    });
  }, []);

  const fetchedUrlsRef = React.useRef<Set<string>>(new Set());

  React.useEffect(() => {
    if (!payload?.fetchUrl || fetchedUrlsRef.current.has(payload.fetchUrl)) return;
    const url = payload.fetchUrl;
    fetchedUrlsRef.current.add(url);
    (async () => {
      try {
        const res = await fetch(url);
        if (!res.ok) return;
        const full = (await res.json()) as Record<string, unknown>;
        const p = normalizeMcpDiagramPayload(full);
        if (p) mergeKitViewerPayload(p);
      } catch {
        /* Engine may not be reachable from iframe. */
      }
    })();
  }, [payload?.fetchUrl, mergeKitViewerPayload]);

  const prevKitResetKeyRef = React.useRef<string>("");
  React.useEffect(() => {
    if (!payload?.kitArtifacts || !isKitViewerPayloadSufficient(payload)) return;
    const key = JSON.stringify({
      n: payload.kitArtifacts.name,
      g: payload.kitArtifacts.id,
      d: payload.kitArtifacts.designs?.length,
      t: payload.kitArtifacts.types?.length,
      u: payload.fetchUrl,
    });
    if (key === prevKitResetKeyRef.current) return;
    prevKitResetKeyRef.current = key;
    setKitSelection({ designIds: [], typeIds: [], portIds: [], connectorIds: [] });
  }, [payload]);

  const tryRefetchKitFromServer = React.useCallback(async () => {
    const client = appRef.current;
    if (!client || gotPayloadRef.current) return;
    const args = mergeKitToolArguments();
    if (!args || Object.keys(args).length === 0) return;
    let toolName = client.getHostContext()?.toolInfo?.tool?.name;
    if (!toolName && typeof (args as { path?: unknown }).path === "string") {
      toolName = "start_working_in_local_kit";
    }
    if (!toolName && typeof (args as { name?: unknown }).name === "string" && typeof (args as { version?: unknown }).version === "string") {
      toolName = "start_new_kit";
    }
    if (!toolName && typeof (args as { serverUrl?: unknown }).serverUrl === "string" && typeof (args as { kitUri?: unknown }).kitUri === "string") {
      toolName = "start_working_in_remote_kit";
    }
    if (!toolName) return;
    const kitTools = new Set(["start_working_in_local_kit", "start_new_kit", "start_working_in_remote_kit"]);
    if (!kitTools.has(toolName)) return;
    try {
      const result = await client.callServerTool({ name: toolName, arguments: args });
      const p = parseDiagramPayloadFromToolResult(result);
      if (p) mergeKitViewerPayload(p);
    } catch {
      /* Host may not proxy tools/call to the server for this session. */
    }
  }, [mergeKitViewerPayload, mergeKitToolArguments]);

  React.useEffect(() => {
    tryRefetchRef.current = () => {
      void tryRefetchKitFromServer();
    };
  }, [tryRefetchKitFromServer]);

  const { app, isConnected, error } = useApp({
    appInfo: { name: "semio kit viewer", version: "1.0.0" },
    capabilities: {},
    onAppCreated: (a) => {
      appRef.current = a;
      a.ontoolinput = (params) => {
        toolInputArgsRef.current = params.arguments ?? null;
        tryRefetchRef.current();
      };
      a.ontoolinputpartial = (params) => {
        const prev = toolInputArgsRef.current ?? {};
        const next = params.arguments ?? {};
        toolInputArgsRef.current = { ...prev, ...next };
        tryRefetchRef.current();
      };
      a.onhostcontextchanged = () => {
        tryRefetchRef.current();
      };
      a.ontoolresult = (result) => {
        const parsed = parseDiagramPayloadFromToolResult(result);
        if (parsed) {
          mergeKitViewerPayload(parsed);
        }
      };
      a.ontoolcancelled = () => {};
      a.onteardown = async () => ({});
      a.onerror = console.error;
    },
  });

  /* Host styles intentionally not applied — semio uses its own theme from globals.css. */

  React.useEffect(() => {
    if (!app) return;
    const h = app.getHostContext() as Record<string, unknown> | undefined;
    if (!h) return;
    for (const k of ["toolResult", "lastToolResult", "toolExecutionResult", "initialToolResult", "pendingToolResult"]) {
      const v = h[k];
      if (v === undefined) continue;
      const p = parseDiagramPayloadFromToolResult(v);
      if (p) mergeKitViewerPayload(p);
    }
  }, [app, mergeKitViewerPayload]);

  React.useEffect(() => {
    if (!app || !isConnected) return;
    const delays = [0, 50, 150, 400, 1200, 2500, 5000, 8000, 15000];
    const ids = delays.map((d) => setTimeout(() => tryRefetchRef.current(), d));
    return () => ids.forEach(clearTimeout);
  }, [app, isConnected]);

  const mcpTheme = useDocumentTheme();
  React.useEffect(() => {
    const el = document.documentElement;
    if (mcpTheme === "dark") {
      el.classList.add("dark");
    } else {
      el.classList.remove("dark");
    }
  }, [mcpTheme]);

  const sendKitSelectionUpdate = React.useCallback((next: KitSelection) => {
    if (!appRef.current) return;
    (appRef.current as { updateRepresentationContext?: (p: unknown) => void })?.updateRepresentationContext?.({
      content: [
        {
          type: "text" as const,
          text: JSON.stringify({
            kitArtifactSelectionChange: {
              designIds: next.designIds ?? [],
              typeIds: next.typeIds ?? [],
              portIds: next.portIds ?? [],
              connectorIds: next.connectorIds ?? [],
            },
          }),
        },
      ],
    });
  }, []);

  const handleKitSelectionChange = React.useCallback(
    (next: KitSelection) => {
      setKitSelection(next);
      sendKitSelectionUpdate(next);
    },
    [sendKitSelectionUpdate],
  );

  const shellStyle: React.CSSProperties = {
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    minHeight: "100dvh",
    width: "100%",
    boxSizing: "border-box",
    padding: 16,
    textAlign: "center",
    fontFamily: "var(--font-sans, ui-sans-serif, system-ui, sans-serif)",
    backgroundColor: "var(--base)",
    color: "var(--foreground)",
  };

  const mutedStyle: React.CSSProperties = {
    color: "var(--muted-foreground)",
  };

  if (error) {
    return (
      <div style={{ ...shellStyle, color: "var(--destructive-foreground)" }}>
        <p>Error: {error.message}</p>
      </div>
    );
  }

  if (!isConnected || !app) {
    return (
      <div style={shellStyle}>
        <p style={mutedStyle}>Connecting to host…</p>
      </div>
    );
  }

  if (!payload || !isKitViewerPayloadSufficient(payload)) {
    return (
      <div style={shellStyle}>
        <p style={mutedStyle}>Waiting for kit data…</p>
      </div>
    );
  }

  return (
    <div
      className="min-h-0"
      style={{
        minHeight: "100dvh",
        width: "100%",
        overflow: "auto",
        padding: 12,
        boxSizing: "border-box",
        backgroundColor: "var(--base)",
        color: "var(--foreground)",
      }}
    >
      <SemioKit kit={payload.kit} data={payload.kitArtifacts} selection={kitSelection} onSelectionChange={handleKitSelectionChange} title="Kit" className="min-h-0 text-foreground" />
    </div>
  );
};

// #region 🎇McpSceneViewer
// Specs: Dedicated MCP App scene viewer — always renders SemioScene (3D only), ignoring payload surface/mode.
// Summary: Standalone 3D-scene-only MCP viewer component.

/**
 * 🎨MCP App scene viewer: always renders {@link SemioScene} from tool results.
 * Used when the MCP host loads `ui://semio/scene-viewer` (show_scene tool).
 *
 * Specs:
 * - Dedicated viewer for scene-only tools (show_scene).
 * - Forces scene rendering regardless of payload surface.
 * - Uses JS flattenDesign (via mcpFlattenDesignForSemioSurface) for correct 3D plane computation.
 */
export const McpSceneViewer: React.FC = () => {
  const [payload, setPayload] = React.useState<McpDiagramPayload | null>(null);
  const appRef = React.useRef<McpApp | null>(null);
  const tryRefetchRef = React.useRef<() => void>(() => {});

  const mergeDiagramPayload = React.useCallback((p: McpDiagramPayload) => {
    setPayload((cur) => {
      if (!cur) return p;
      const best = scoreMcpDiagramPayload(p) > scoreMcpDiagramPayload(cur) ? p : cur;
      return mergeRichestDesignFromCandidates([cur, p], best) ?? p;
    });
  }, []);

  const fetchedUrlsRef = React.useRef<Set<string>>(new Set());

  React.useEffect(() => {
    if (!payload?.fetchUrl || fetchedUrlsRef.current.has(payload.fetchUrl)) return;
    const url = payload.fetchUrl;
    fetchedUrlsRef.current.add(url);
    (async () => {
      try {
        const res = await fetch(url);
        if (!res.ok) return;
        const full = (await res.json()) as Record<string, unknown>;
        const p = normalizeMcpDiagramPayload(full);
        if (p) mergeDiagramPayload(p);
      } catch {
        /* Engine may not be reachable from iframe. */
      }
    })();
  }, [payload?.fetchUrl, mergeDiagramPayload]);

  const tryRefetchDesignFromServer = React.useCallback(async () => {
    const client = appRef.current;
    if (!client) return;
    try {
      const result = await client.callServerTool({ name: "show_scene", arguments: {} });
      const p = parseDiagramPayloadFromToolResult(result);
      if (p) mergeDiagramPayload(p);
    } catch {
      /* Host may not proxy tools/call. */
    }
  }, [mergeDiagramPayload]);

  React.useEffect(() => {
    tryRefetchRef.current = () => {
      void tryRefetchDesignFromServer();
    };
  }, [tryRefetchDesignFromServer]);

  const { app, isConnected, error } = useApp({
    appInfo: { name: "semio scene viewer", version: "1.0.0" },
    capabilities: {},
    onAppCreated: (a) => {
      appRef.current = a;
      a.ontoolinput = () => tryRefetchRef.current();
      a.ontoolinputpartial = () => tryRefetchRef.current();
      a.onhostcontextchanged = () => tryRefetchRef.current();
      a.ontoolresult = (result) => {
        const p = parseDiagramPayloadFromToolResult(result);
        if (p) mergeDiagramPayload(p);
      };
      a.ontoolcancelled = () => {};
      a.onteardown = async () => ({});
      a.onerror = console.error;
    },
  });

  /* Host styles intentionally not applied — semio uses its own theme from globals.css. */

  React.useEffect(() => {
    if (!app) return;
    const h = app.getHostContext() as Record<string, unknown> | undefined;
    if (!h) return;
    let best: McpDiagramPayload | null = null;
    for (const k of ["toolResult", "lastToolResult", "toolExecutionResult", "initialToolResult", "pendingToolResult"]) {
      const v = h[k];
      if (v === undefined) continue;
      const p = parseDiagramPayloadFromToolResult(v);
      if (p && (!best || scoreMcpDiagramPayload(p) > scoreMcpDiagramPayload(best))) best = p;
    }
    if (best) mergeDiagramPayload(best);
  }, [app, mergeDiagramPayload]);

  React.useEffect(() => {
    if (!app || !isConnected) return;
    const delays = [0, 50, 150, 400, 1200, 2500, 5000, 8000, 15000];
    const ids = delays.map((d) => setTimeout(() => tryRefetchRef.current(), d));
    return () => ids.forEach(clearTimeout);
  }, [app, isConnected]);

  const mcpTheme = useDocumentTheme();
  React.useEffect(() => {
    const el = document.documentElement;
    if (mcpTheme === "dark") el.classList.add("dark");
    else el.classList.remove("dark");
  }, [mcpTheme]);

  const shellStyle: React.CSSProperties = { display: "flex", alignItems: "center", justifyContent: "center", height: "100vh", fontFamily: "var(--font-sans, ui-sans-serif, system-ui, sans-serif)", background: "var(--base)" };
  if (error)
    return (
      <div style={{ ...shellStyle, color: "var(--destructive-foreground)" }}>
        <p>Error: {error.message}</p>
      </div>
    );
  if (!isConnected || !app)
    return (
      <div style={shellStyle}>
        <p style={{ color: "var(--muted-foreground)" }}>Connecting to host…</p>
      </div>
    );
  if (!payload)
    return (
      <div style={shellStyle}>
        <p style={{ color: "var(--muted-foreground)" }}>Waiting for design data…</p>
      </div>
    );

  const design = payload.design as Design | undefined;
  const kit = payload.kit as Kit | undefined;
  const designFlat = design && kit ? mcpFlattenDesignForSemioSurface(design, kit, "scene") : design;
  if (!designFlat)
    return (
      <div style={shellStyle}>
        <p style={{ color: "var(--muted-foreground)" }}>Loading scene…</p>
      </div>
    );

  return (
    <div style={{ width: "100%", height: "100vh", position: "relative" }}>
      <SemioScene design={designFlat} kit={kit} />
    </div>
  );
};

// #endregion 🎇McpSceneViewer

// #region 🗼McpDiagramViewer
// Specs: Dedicated MCP App diagram viewer — always renders SemioDiagram (2D only), ignoring payload surface/mode.
// Summary: Standalone 2D-diagram-only MCP viewer component.

/**
 * 🎨MCP App diagram viewer: always renders {@link SemioDiagram} from tool results.
 * Used when the MCP host loads `ui://semio/diagram-viewer` (show_diagram tool).
 *
 * Specs:
 * - Dedicated viewer for diagram-only tools (show_diagram).
 * - Forces diagram rendering regardless of payload surface.
 * - Uses JS flattenDesign (via mcpFlattenDesignForSemioSurface) for correct 2D center computation.
 */
export const McpDiagramViewer: React.FC = () => {
  const [payload, setPayload] = React.useState<McpDiagramPayload | null>(null);
  const [selectedPieces, setSelectedPieces] = React.useState<Set<string>>(new Set());
  const [selectedConnections, setSelectedConnections] = React.useState<Set<string>>(new Set());
  const appRef = React.useRef<McpApp | null>(null);
  const tryRefetchRef = React.useRef<() => void>(() => {});

  const mergeDiagramPayload = React.useCallback((p: McpDiagramPayload) => {
    setPayload((cur) => {
      if (!cur) return p;
      const best = scoreMcpDiagramPayload(p) > scoreMcpDiagramPayload(cur) ? p : cur;
      return mergeRichestDesignFromCandidates([cur, p], best) ?? p;
    });
  }, []);

  const fetchedUrlsRef = React.useRef<Set<string>>(new Set());

  React.useEffect(() => {
    if (!payload?.fetchUrl || fetchedUrlsRef.current.has(payload.fetchUrl)) return;
    const url = payload.fetchUrl;
    fetchedUrlsRef.current.add(url);
    (async () => {
      try {
        const res = await fetch(url);
        if (!res.ok) return;
        const full = (await res.json()) as Record<string, unknown>;
        const p = normalizeMcpDiagramPayload(full);
        if (p) mergeDiagramPayload(p);
      } catch {
        /* Engine may not be reachable from iframe. */
      }
    })();
  }, [payload?.fetchUrl, mergeDiagramPayload]);

  const tryRefetchDesignFromServer = React.useCallback(async () => {
    const client = appRef.current;
    if (!client) return;
    try {
      const result = await client.callServerTool({ name: "show_diagram", arguments: {} });
      const p = parseDiagramPayloadFromToolResult(result);
      if (p) mergeDiagramPayload(p);
    } catch {
      /* Host may not proxy tools/call. */
    }
  }, [mergeDiagramPayload]);

  React.useEffect(() => {
    tryRefetchRef.current = () => {
      void tryRefetchDesignFromServer();
    };
  }, [tryRefetchDesignFromServer]);

  const { app, isConnected, error } = useApp({
    appInfo: { name: "semio diagram viewer", version: "1.0.0" },
    capabilities: {},
    onAppCreated: (a) => {
      appRef.current = a;
      a.ontoolinput = () => tryRefetchRef.current();
      a.ontoolinputpartial = () => tryRefetchRef.current();
      a.onhostcontextchanged = () => tryRefetchRef.current();
      a.ontoolresult = (result) => {
        const p = parseDiagramPayloadFromToolResult(result);
        if (p) mergeDiagramPayload(p);
      };
      a.ontoolcancelled = () => {};
      a.onteardown = async () => ({});
      a.onerror = console.error;
    },
  });

  /* Host styles intentionally not applied — semio uses its own theme from globals.css. */

  React.useEffect(() => {
    if (!app) return;
    const h = app.getHostContext() as Record<string, unknown> | undefined;
    if (!h) return;
    let best: McpDiagramPayload | null = null;
    for (const k of ["toolResult", "lastToolResult", "toolExecutionResult", "initialToolResult", "pendingToolResult"]) {
      const v = h[k];
      if (v === undefined) continue;
      const p = parseDiagramPayloadFromToolResult(v);
      if (p && (!best || scoreMcpDiagramPayload(p) > scoreMcpDiagramPayload(best))) best = p;
    }
    if (best) mergeDiagramPayload(best);
  }, [app, mergeDiagramPayload]);

  React.useEffect(() => {
    if (!app || !isConnected) return;
    const delays = [0, 50, 150, 400, 1200, 2500, 5000, 8000, 15000];
    const ids = delays.map((d) => setTimeout(() => tryRefetchRef.current(), d));
    return () => ids.forEach(clearTimeout);
  }, [app, isConnected]);

  React.useEffect(() => {
    if (!payload) return;
    setSelectedPieces(new Set());
    setSelectedConnections(new Set());
  }, [payload]);

  const mcpTheme = useDocumentTheme();
  React.useEffect(() => {
    const el = document.documentElement;
    if (mcpTheme === "dark") el.classList.add("dark");
    else el.classList.remove("dark");
  }, [mcpTheme]);

  const sendSelectionUpdate = React.useCallback((pieces: Set<string>, connections: Set<string>) => {
    if (!appRef.current) return;
    (appRef.current as { updateRepresentationContext?: (p: unknown) => void })?.updateRepresentationContext?.({ content: [{ type: "text" as const, text: JSON.stringify({ selectionChange: { pieceIds: Array.from(pieces), connectionIds: Array.from(connections) } }) }] });
  }, []);

  const shellStyle: React.CSSProperties = { display: "flex", alignItems: "center", justifyContent: "center", height: "100vh", fontFamily: "var(--font-sans, ui-sans-serif, system-ui, sans-serif)", background: "var(--base)" };
  if (error)
    return (
      <div style={{ ...shellStyle, color: "var(--destructive-foreground)" }}>
        <p>Error: {error.message}</p>
      </div>
    );
  if (!isConnected || !app)
    return (
      <div style={shellStyle}>
        <p style={{ color: "var(--muted-foreground)" }}>Connecting to host…</p>
      </div>
    );
  if (!payload)
    return (
      <div style={shellStyle}>
        <p style={{ color: "var(--muted-foreground)" }}>Waiting for design data…</p>
      </div>
    );

  const design = payload.design as Design | undefined;
  const kit = payload.kit as Kit | undefined;
  const isDiff = payload.mode === "show-diagram-diff";
  const designFlat = design && kit ? mcpFlattenDesignForSemioSurface(design, kit, "diagram", isDiff ? payload.designDiff : undefined) : design;
  const hasDiagramPoints = (payload.points?.length ?? 0) > 0;
  const fallbackDesign: Design = {
    id: "__mcp__",
    pieces: (payload.points ?? []).map((pt) => ({ id: pt.id, center: { u: pt.u, v: pt.v } })),
    connections: (payload.lines ?? []).map((l) => ({
      id: l.id,
      connected: { piece: { id: (payload.points ?? []).find((q) => q.u === l.sourceU && q.v === l.sourceV)?.id ?? "" } },
      connecting: { piece: { id: (payload.points ?? []).find((q) => q.u === l.targetU && q.v === l.targetV)?.id ?? "" } },
    })),
  } as unknown as Design;
  const candidateDesign = (designFlat ?? design) as Design | undefined;
  const candidateHasCenters = __itemsOf((candidateDesign as PlainJsonObject | undefined)?.["pieces"]).some((pc) => !!(pc as Piece).center);
  const diagramDesign = (candidateHasCenters ? candidateDesign! : hasDiagramPoints ? fallbackDesign : (candidateDesign ?? fallbackDesign)) as PlainJsonObject;
  const pieceSelectionEnabled = payload.capabilities?.pieceSelection ?? false;
  const connectionSelectionEnabled = payload.capabilities?.connectionSelection ?? false;
  const selectionEnabled = pieceSelectionEnabled || connectionSelectionEnabled;

  return (
    <div style={{ width: "100%", height: "100vh", position: "relative" }}>
      <SemioDiagram
        design={diagramDesign as Design}
        designDiff={isDiff ? payload.designDiff : undefined}
        selectionEnabled={selectionEnabled}
        pieceSelectionEnabled={pieceSelectionEnabled}
        connectionSelectionEnabled={connectionSelectionEnabled}
        selection={{ pieceIds: Array.from(selectedPieces), connectionIds: Array.from(selectedConnections) }}
        onSelectionChange={(next) => {
          const nextPieces = new Set(next.pieceIds ?? []);
          const nextConns = new Set(next.connectionIds ?? []);
          setSelectedPieces(nextPieces);
          setSelectedConnections(nextConns);
          sendSelectionUpdate(nextPieces, nextConns);
        }}
      />
    </div>
  );
};

// #endregion 🗼McpDiagramViewer

/**
 * Mount the MCP design viewer as a standalone app.
 * Call this from the entry point TSX file after importing react-dom/client.
 **/
export const mountMcpDesignViewer = (createRoot: (container: HTMLElement) => { render: (element: React.ReactNode) => void }) => {
  const root = document.getElementById("root");
  if (!root) throw new Error("Missing #root element");
  createRoot(root).render(
    <React.StrictMode>
      <McpDesignViewer />
    </React.StrictMode>,
  );
};

/**
 * Mount the MCP scene viewer (SemioScene only) as a standalone app.
 **/
export const mountMcpSceneViewer = (createRoot: (container: HTMLElement) => { render: (element: React.ReactNode) => void }) => {
  const root = document.getElementById("root");
  if (!root) throw new Error("Missing #root element");
  createRoot(root).render(
    <React.StrictMode>
      <McpSceneViewer />
    </React.StrictMode>,
  );
};

/**
 * Mount the MCP diagram viewer (SemioDiagram only) as a standalone app.
 **/
export const mountMcpDiagramViewer = (createRoot: (container: HTMLElement) => { render: (element: React.ReactNode) => void }) => {
  const root = document.getElementById("root");
  if (!root) throw new Error("Missing #root element");
  createRoot(root).render(
    <React.StrictMode>
      <McpDiagramViewer />
    </React.StrictMode>,
  );
};

/**
 * Mount the MCP kit viewer (SemioKit only) as a standalone app.
 **/
export const mountMcpKitViewer = (createRoot: (container: HTMLElement) => { render: (element: React.ReactNode) => void }) => {
  const root = document.getElementById("root");
  if (!root) throw new Error("Missing #root element");
  createRoot(root).render(
    <React.StrictMode>
      <McpKitViewer />
    </React.StrictMode>,
  );
};

// #endregion 🗿McpApp

if ((import.meta as any).vitest) {
  const { describe, expect, it } = (import.meta as any).vitest;

  describe("parseDiagramPayloadFromToolResult", () => {
    it("parses JSON from MCP text content blocks", () => {
      const payload = { points: [], lines: [], kitArtifacts: { name: "K", designs: [], types: [], ports: [], connectors: [] } };
      const r = parseDiagramPayloadFromToolResult({
        content: [{ type: "text", text: JSON.stringify(payload) }],
      });
      expect(r?.kitArtifacts?.name).toBe("K");
    });

    it("parses structuredContent object (hosts that omit text content)", () => {
      const inner = { points: [], lines: [], kitArtifacts: { name: "S", designs: [{ id: "d1", name: "D" }], types: [], ports: [], connectors: [] } };
      const r = parseDiagramPayloadFromToolResult({ structuredContent: inner });
      expect(r?.kitArtifacts?.designs?.[0]?.id).toBe("d1");
    });

    it("parses structuredContent JSON string", () => {
      const inner = { points: [], lines: [], capabilities: {}, kitArtifacts: { designs: [], types: [], ports: [], connectors: [] } };
      const r = parseDiagramPayloadFromToolResult({ structuredContent: JSON.stringify(inner) });
      expect(r?.points).toEqual([]);
    });

    it("unwraps notification params", () => {
      const payload = { points: [], lines: [], kitArtifacts: { name: "P", designs: [], types: [], ports: [], connectors: [] } };
      const r = parseDiagramPayloadFromToolResult({
        params: { content: [{ type: "text", text: JSON.stringify(payload) }] },
      });
      expect(r?.kitArtifacts?.name).toBe("P");
    });

    it("reads nested kit payload under arbitrary host keys", () => {
      const inner = { points: [], lines: [], kitArtifacts: { name: "Deep", designs: [], types: [], ports: [], connectors: [] } };
      const r = parseDiagramPayloadFromToolResult({ wrapper: { data: inner } });
      expect(r?.kitArtifacts?.name).toBe("Deep");
    });

    it("prefers full text content JSON when structuredContent kitArtifacts is a stripped shell", () => {
      const stripped = {
        points: [],
        lines: [],
        capabilities: { pieceSelection: false, connectionSelection: false },
        kitArtifacts: { designs: [], types: [], ports: [], connectors: [] },
      };
      const full = {
        points: [],
        lines: [],
        capabilities: { pieceSelection: false, connectionSelection: false },
        kitArtifacts: {
          name: "Metabolism",
          version: "1",
          designs: [{ id: "d1", name: "D", variant: "", view: "" }],
          types: [{ id: "t1", name: "T", variant: "" }],
          ports: [],
          connectors: [],
        },
      };
      const r = parseDiagramPayloadFromToolResult({
        structuredContent: stripped,
        content: [{ type: "text", text: JSON.stringify(full) }],
      });
      expect(r?.kitArtifacts?.name).toBe("Metabolism");
      expect(r?.kitArtifacts?.designs?.length).toBe(1);
    });

    it("merges show-design from text when structuredContent wins on kit score with show-diagram", () => {
      const stripped = {
        mode: "show-diagram",
        points: [],
        lines: [],
        capabilities: { pieceSelection: false, connectionSelection: false },
        kitArtifacts: {
          name: "Metabolism",
          version: "1",
          designs: new Array(12).fill(null).map((_, i) => ({ id: `d${i}`, name: "", variant: "", view: "" })),
          types: [],
          ports: [],
          connectors: [],
        },
      };
      const full = {
        mode: "show-design",
        surface: "design",
        points: [],
        lines: [],
        capabilities: { pieceSelection: false, connectionSelection: false },
        kitArtifacts: { name: "Metabolism", designs: [{ id: "dg1", name: "D", variant: "", view: "" }], types: [], ports: [], connectors: [] },
        design: { id: "dg1", pieces: [{ id: "p1" }], connections: [] },
        kit: { name: "Metabolism", designs: [], types: [] },
      };
      const r = parseDiagramPayloadFromToolResult({
        structuredContent: stripped,
        content: [{ type: "text", text: JSON.stringify(full) }],
      });
      expect((r?.design as { id?: string } | undefined)?.id).toBe("dg1");
      expect(r?.mode).toBe("show-design");
      expect(r?.surface).toBe("design");
    });

    it("picks design with plane+center for scene when structuredContent has more pieces but no scene geometry", () => {
      const plane = {
        origin: { x: 0, y: 0, z: 0 },
        xAxis: { x: 1, y: 0, z: 0 },
        yAxis: { x: 0, y: 1, z: 0 },
      };
      const stripped = {
        mode: "show-design",
        surface: "design",
        points: [],
        lines: [],
        capabilities: { pieceSelection: false, connectionSelection: false },
        kitArtifacts: { name: "K", designs: [], types: [], ports: [], connectors: [] },
        design: {
          id: "dg1",
          pieces: new Array(40).fill(null).map((_, i) => ({ id: `s${i}`, center: { u: 0, v: 0 } })),
          connections: [],
        },
        kit: { name: "K", designs: [], types: [] },
      };
      const fuller = {
        mode: "show-design",
        surface: "design",
        points: [],
        lines: [],
        capabilities: { pieceSelection: false, connectionSelection: false },
        kitArtifacts: { name: "K", designs: [], types: [], ports: [], connectors: [] },
        design: {
          id: "dg1",
          pieces: new Array(3).fill(null).map((_, i) => ({
            id: `f${i}`,
            plane,
            center: { u: 0, v: 0 },
          })),
          connections: [],
        },
        kit: { name: "K", designs: [], types: [] },
      };
      const r = parseDiagramPayloadFromToolResult({
        structuredContent: stripped,
        content: [{ type: "text", text: JSON.stringify(fuller) }],
      });
      const withScene = __itemsOf((r?.design as PlainJsonObject | undefined)?.["pieces"]).filter((p) => (p as Piece).plane && (p as Piece).center).length;
      expect(withScene).toBe(3);
    });

    it("reads text from embedded resource content blocks", () => {
      const payload = { points: [], lines: [], kitArtifacts: { name: "Res", designs: [], types: [], ports: [], connectors: [] } };
      const r = parseDiagramPayloadFromToolResult({
        content: [{ type: "resource", resource: { uri: "x", mimeType: "text/plain", text: JSON.stringify(payload) } }],
      });
      expect(r?.kitArtifacts?.name).toBe("Res");
    });

    it("preserves mode, design, and kit for McpDesignViewer (show-design / show-scene vs diagram)", () => {
      const inner = {
        points: [],
        lines: [],
        mode: "show-scene",
        capabilities: { pieceSelection: true, connectionSelection: false },
        kitArtifacts: { name: "K", designs: [], types: [], ports: [], connectors: [] },
        design: { id: "dg", pieces: [], connections: [] },
        kit: { id: "kg", name: "Kit", version: "1", types: [], designs: [] },
      };
      const r = parseDiagramPayloadFromToolResult({ structuredContent: inner });
      expect(r?.mode).toBe("show-scene");
      expect((r?.design as { id?: string } | undefined)?.id).toBe("dg");
      expect((r?.kit as { id?: string } | undefined)?.id).toBe("kg");
    });

    it("preserves designDiff for diff modes", () => {
      const diff = { pieces: { added: [{ id: "pa" }], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } };
      const inner = {
        points: [],
        lines: [],
        mode: "show-diff",
        design: { id: "dg", pieces: [], connections: [] },
        designDiff: diff,
      };
      const r = parseDiagramPayloadFromToolResult({ structuredContent: inner });
      expect(r?.mode).toBe("show-diff");
      expect(r?.designDiff).toEqual(diff);
    });

    it("merges the richest design when structuredContent truncates pieces", () => {
      const stripped = {
        points: [],
        lines: [],
        mode: "show-scene",
        capabilities: {},
        kitArtifacts: { name: "K", designs: [], types: [], ports: [], connectors: [] },
        design: { id: "dg", pieces: [{ id: "p0" }], connections: [] },
        kit: { id: "kg", name: "Kit", version: "1", types: [], designs: [] },
      };
      const full = {
        ...stripped,
        design: { id: "dg", pieces: Array.from({ length: 30 }, (_, i) => ({ id: `p${i}` })), connections: [] },
      };
      const r = parseDiagramPayloadFromToolResult({
        structuredContent: stripped,
        content: [{ type: "text", text: JSON.stringify(full) }],
      });
      expect(__itemsOf((r?.design as PlainJsonObject | undefined)?.["pieces"]).length).toBe(30);
    });

    it("parses each content block as JSON (text + EmbeddedResource duplicate from engine)", () => {
      const stub = {
        points: [],
        lines: [],
        mode: "show-design",
        design: { id: "dg", pieces: [{ id: "p0" }], connections: [] },
        kit: { id: "kg", name: "Kit", version: "1", types: [], designs: [] },
      };
      const full = {
        ...stub,
        design: { id: "dg", pieces: Array.from({ length: 20 }, (_, i) => ({ id: `p${i}` })), connections: [] },
      };
      const r = parseDiagramPayloadFromToolResult({
        content: [
          { type: "text", text: JSON.stringify(stub) },
          { type: "resource", resource: { uri: "semio://mcp-app/tool-payload", mimeType: "application/json", text: JSON.stringify(full) } },
        ],
      });
      expect(__itemsOf((r?.design as PlainJsonObject | undefined)?.["pieces"]).length).toBe(20);
    });
  });

  describe("semioDesignGridTemplateColumns / semioDesignShowSceneColumn", () => {
    it("always keeps two columns even when no piece has plane+center (MCP cannot collapse to diagram-only)", () => {
      expect(semioDesignGridTemplateColumns("always", false, 0.5)).toBe("50% 50%");
      expect(semioDesignGridTemplateColumns("always", false, 0.5)).not.toBe("1fr");
      expect(semioDesignShowSceneColumn("always", false)).toBe(true);
    });
    it("auto falls back to single column when no planes", () => {
      expect(semioDesignGridTemplateColumns("auto", false, 0.5)).toBe("1fr");
      expect(semioDesignShowSceneColumn("auto", false)).toBe(false);
    });
    it("auto splits when at least one piece has plane+center", () => {
      expect(semioDesignGridTemplateColumns("auto", true, 0.5)).toBe("50% 50%");
      expect(semioDesignShowSceneColumn("auto", true)).toBe(true);
    });
  });

  describe("mcpEffectiveSurface", () => {
    it("uses explicit surface when mode does not imply design/scene", () => {
      expect(
        mcpEffectiveSurface({
          mode: "show-diagram",
          surface: "design",
          points: [],
          lines: [],
          kitArtifacts: { designs: [], types: [], ports: [], connectors: [] },
        } as McpDiagramPayload),
      ).toBe("design");
    });
    it("mode show-design overrides stale surface diagram", () => {
      expect(
        mcpEffectiveSurface({
          mode: "show-design",
          surface: "diagram",
          points: [],
          lines: [],
          kitArtifacts: { designs: [], types: [], ports: [], connectors: [] },
        } as McpDiagramPayload),
      ).toBe("design");
    });
    it("derives design from show-design mode", () => {
      expect(
        mcpEffectiveSurface({
          mode: "show-design",
          points: [],
          lines: [],
          kitArtifacts: { designs: [], types: [], ports: [], connectors: [] },
        } as McpDiagramPayload),
      ).toBe("design");
    });
    it("derives scene from show-scene mode", () => {
      expect(
        mcpEffectiveSurface({
          mode: "show-scene",
          points: [],
          lines: [],
          kitArtifacts: { designs: [], types: [], ports: [], connectors: [] },
        } as McpDiagramPayload),
      ).toBe("scene");
    });
    it("returns diagram for null payload", () => {
      expect(mcpEffectiveSurface(null)).toBe("diagram");
    });
  });

  describe("mcpMapPayloadToDesignViewerViewRepresentation", () => {
    it("maps split design+kit to SemioDesign surface even when surface field is diagram", () => {
      const vm = mcpMapPayloadToDesignViewerViewRepresentation({
        mode: "show-design",
        surface: "diagram",
        points: [],
        lines: [],
        capabilities: {},
        kitArtifacts: { name: "K", designs: [], types: [], ports: [], connectors: [] },
        design: { id: "dg", pieces: [{ id: "p0" }], connections: [] } as unknown as Design,
        kit: { id: "kg", name: "Kit", version: "1", types: [], designs: [] } as unknown as Kit,
      } as McpDiagramPayload);
      expect(vm.surface).toBe("design");
    });

    it("falls back to points/lines when design pieces have no centers", () => {
      const vm = mcpMapPayloadToDesignViewerViewRepresentation({
        mode: "show-diagram",
        points: [
          { id: "p1", u: 0, v: 0, status: "default" },
          { id: "p2", u: 3, v: 0, status: "default" },
        ],
        lines: [{ id: "c1", sourceU: 0, sourceV: 0, targetU: 3, targetV: 0, status: "default" }],
        design: { id: "dg", pieces: [{ id: "p1" }, { id: "p2" }], connections: [] } as unknown as Design,
      } as McpDiagramPayload);
      expect((vm.diagramDesign["pieces"] as unknown[] | undefined)?.length).toBe(2);
      expect(((vm.diagramDesign["pieces"] as Piece[] | undefined)?.[0] as Piece | undefined)?.center).toEqual({ u: 0, v: 0 });
      expect((vm.diagramDesign["connections"] as unknown[] | undefined)?.length).toBe(1);
    });

    it("uses design when pieces have centers even without points/lines", () => {
      const vm = mcpMapPayloadToDesignViewerViewRepresentation({
        mode: "show-diagram",
        points: [],
        lines: [],
        design: { id: "dg", pieces: [{ id: "p1", center: { u: 1, v: 2 } }], connections: [] } as unknown as Design,
      } as McpDiagramPayload);
      expect(((vm.diagramDesign["pieces"] as Piece[] | undefined)?.[0] as Piece | undefined)?.center).toEqual({ u: 1, v: 2 });
    });
  });

  describe("mergeRichestDesignFromCandidates", () => {
    it("prefers show-scene over show-design when both refer to the same design id", () => {
      const plane = {
        origin: { x: 0, y: 0, z: 0 },
        xAxis: { x: 1, y: 0, z: 0 },
        yAxis: { x: 0, y: 1, z: 0 },
      } as unknown as Plane;

      const design = {
        id: "dg",
        pieces: [{ id: "p0", plane, center: { u: 0, v: 0 } }],
        connections: [],
      } as unknown as Design;

      const mkPoint = (i: number): McpDiagramPayload["points"][number] => ({
        id: `p${i}`,
        u: i,
        v: i,
        status: "default",
      });

      const mkLine = (i: number): McpDiagramPayload["lines"][number] => ({
        id: `c${i}`,
        sourceU: 0,
        sourceV: 0,
        targetU: 0,
        targetV: 0,
        status: "default",
      });

      const showDiagram: McpDiagramPayload = {
        mode: "show-diagram",
        surface: "diagram",
        points: Array.from({ length: 200 }, (_, i) => mkPoint(i)),
        lines: Array.from({ length: 199 }, (_, i) => mkLine(i)),
        capabilities: {},
        kitArtifacts: { name: "K", designs: [], types: [], ports: [], connectors: [] },
      };

      const showDesign: McpDiagramPayload = {
        mode: "show-design",
        surface: "design",
        points: [],
        lines: [],
        capabilities: {},
        kitArtifacts: { name: "K", designs: [], types: [], ports: [], connectors: [] },
        design,
        kit: { id: "kg", name: "Kit", version: "1", types: [], designs: [] } as unknown as Kit,
      };

      const showScene: McpDiagramPayload = {
        mode: "show-scene",
        surface: "scene",
        points: [],
        lines: [],
        capabilities: {},
        kitArtifacts: { name: "K", designs: [], types: [], ports: [], connectors: [] },
        design,
        kit: { id: "kg", name: "Kit", version: "1", types: [], designs: [] } as unknown as Kit,
      };

      const merged = mergeRichestDesignFromCandidates([showDiagram, showDesign, showScene], showDiagram);
      expect(merged?.mode).toBe("show-scene");
      expect(merged?.surface).toBe("scene");
    });

    it("pulls richer kitArtifacts from another candidate when the scored-best shell omitted kit body", () => {
      const diagramHeavy: McpDiagramPayload = {
        mode: "show-diagram",
        points: [{ id: "p", u: 0, v: 0, status: "default" }],
        lines: [],
        capabilities: {},
      };
      const withKit: McpDiagramPayload = {
        points: [],
        lines: [],
        capabilities: {},
        kitArtifacts: { name: "MergedKit", designs: [{ id: "d1", name: "D" }], types: [], ports: [], connectors: [] },
      };
      const merged = mergeRichestDesignFromCandidates([diagramHeavy, withKit], diagramHeavy);
      expect(merged?.kitArtifacts?.name).toBe("MergedKit");
      expect(merged?.kitArtifacts?.designs?.[0]?.id).toBe("d1");
    });
  });

  describe("mcpFlattenDesignForSemioSurface", () => {
    it("returns the design unchanged when no diff is supplied (kit flatten is deferred)", () => {
      const design = {
        id: "dg-1",
        pieces: [{ id: "p-1" }],
        connections: [],
      } as unknown as Design;

      const kit = {
        name: "K",
        types: [],
        designs: [],
      } as unknown as Kit;

      const flattened = mcpFlattenDesignForSemioSurface(design, kit, "design");
      expect(flattened).toBe(design);
    });

    it("applies a design diff preview when a diff is supplied", () => {
      const design = {
        id: "dg-1",
        pieces: [{ id: "p-1" }],
        connections: [],
      } as unknown as Design;

      const kit = {
        name: "K",
        types: [],
        designs: [],
      } as unknown as Kit;

      const flattened = mcpFlattenDesignForSemioSurface(design, kit, "diagram", {
        pieces: {
          updated: [{ piece: { id: "p-1" }, diff: { center: { u: 12, v: -4 } } }],
        },
      } as unknown as DesignDiff) as PlainJsonObject;

      const pcs = __itemsOf(flattened["pieces"]) as Piece[];
      expect(pcs[0]?.center).toEqual({ u: 12, v: -4 });
    });
  });

  describe("deepFindKitToolArguments", () => {
    it("finds path in nested host-like objects", () => {
      expect(deepFindKitToolArguments({ a: { b: { path: "  /semio/metabolism  " } } })).toEqual({ path: "/semio/metabolism" });
    });

    it("prefers serverUrl+kitUri when present", () => {
      expect(
        deepFindKitToolArguments({
          toolInfo: { serverUrl: "http://x", kitUri: "kit://y" },
        }),
      ).toEqual({ serverUrl: "http://x", kitUri: "kit://y" });
    });
  });

  describe("isKitViewerPayloadSufficient", () => {
    it("rejects stripped shell kitArtifacts", () => {
      expect(
        isKitViewerPayloadSufficient({
          points: [],
          lines: [],
          kitArtifacts: { designs: [], types: [], ports: [], connectors: [] },
        }),
      ).toBe(false);
    });

    it("accepts kit with name or non-empty lists", () => {
      expect(
        isKitViewerPayloadSufficient({
          points: [],
          lines: [],
          kitArtifacts: { name: "K", designs: [], types: [], ports: [], connectors: [] },
        }),
      ).toBe(true);
    });
  });

  const testPlane = {
    origin: { x: 0, y: 0, z: 0 },
    xAxis: { x: 1, y: 0, z: 0 },
    yAxis: { x: 0, y: 1, z: 0 },
  } as unknown as Plane;
  const testCenter = { u: 0, v: 0 } as unknown as Coordinate;

  describe("buildKitDataFromKit", () => {
    it("normalizes connector port references into string labels instead of raw id objects", () => {
      const data = buildKitDataFromKit({
        id: "kit-id",
        name: "Kit",
        version: "1",
        types: [
          {
            id: "kind-id",
            name: "Kind",
            connectors: [
              {
                id: "connector-id",
                name: "",
                port: { id: "port-id" },
              },
              {
                id: "named-connector-id",
                port: { id: "named-port-id", name: "Named Port" },
              },
            ],
          },
        ],
      } as unknown as Kit);

      expect(data.connectors).toEqual([
        {
          id: "connector-id",
          typeId: "kind-id",
          port: "port-id",
          name: "port-id",
          description: undefined,
          mandatory: undefined,
        },
        {
          id: "named-connector-id",
          typeId: "kind-id",
          port: "named-port-id",
          name: "Named Port",
          description: undefined,
          mandatory: undefined,
        },
      ]);
      expect(data.ports).toEqual([]);
    });

    it("returns shallow kit kinds without requiring connector expansion", () => {
      const data = buildKitDataFromKit({
        id: "kit-id",
        name: "Kit",
        version: "1",
        types: [{ id: "kind-id", name: "Kind" }],
        designs: [{ id: "design-id", name: "Design" }],
      } as unknown as Kit);

      expect(data.types).toEqual([{ id: "kind-id", name: "Kind" }]);
      expect(data.designs).toEqual([{ id: "design-id", name: "Design" }]);
      expect(data.ports).toEqual([]);
      expect(data.connectors).toEqual([]);
    });

    it("flattens family-owned ports separately from type connectors", () => {
      const data = buildKitDataFromKit({
        id: "kit-id",
        name: "Kit",
        version: "1",
        families: [{ id: "family-1", name: "Family", ports: [{ id: "port-entity", name: "P1", description: "d" }] }],
        types: [
          {
            id: "kind-id",
            name: "Kind",
            connectors: [{ id: "conn-1", name: "C1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 1, z: 0 } }],
          },
        ],
      } as unknown as Kit);

      expect(data.ports).toEqual([{ id: "port-entity", name: "P1", description: "d" }]);
      expect(data.connectors).toEqual([
        {
          id: "conn-1",
          typeId: "kind-id",
          port: undefined,
          name: "C1",
          description: undefined,
          mandatory: undefined,
        },
      ]);
    });
  });

  describe("resolveKitArtifactDesignForPreview", () => {
    it("flattens the Nakagin Capsule Tower kit preview to match the Design surface", async () => {
      const assets = await import("@semio/assets");
      const kit = assets.MetabolismKit as unknown as Kit;
      const kitData = buildKitDataFromKit(kit);
      const design = kitData.designs?.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent) as Design | undefined;
      expect(design).toBeTruthy();

      const preview = resolveKitArtifactDesignForPreview(design!, kit) as PlainJsonObject;
      const previewPieces = __itemsOf(preview["pieces"]) as Piece[];
      const piecesWithPlaneAndCenter = previewPieces.filter((p) => p.plane && p.center).length;

      expect(previewPieces.length).toBeGreaterThan(100);
      expect(piecesWithPlaneAndCenter).toBe(previewPieces.length);
    }, 20000);
  });

  describe("buildKitHierarchy", () => {
    it("builds a dynamic type breadcrumb path from nested parent kinds", () => {
      const hierarchy = buildKitHierarchy(
        {
          name: "Metabolism",
          types: [
            { id: "capsule", name: "Capsule" },
            { id: "ellipsoid", name: "Ellipsoid", parent: { id: "capsule" } },
            { id: "l", name: "L", parent: { id: "ellipsoid" } },
          ],
        },
        { designDataEnabled: true, typeDataEnabled: true, portDataEnabled: true, connectorDataEnabled: true },
      );

      expect(getKitNodePath(hierarchy, "kind:l").map((node) => node.label)).toEqual(["Kit", "Metabolism", "Types", "Capsule", "Ellipsoid", "L"]);
    });

    it("exposes child nodes from each breadcrumb step instead of sibling nodes", () => {
      const hierarchy = buildKitHierarchy(
        {
          name: "Metabolism",
          types: [
            { id: "capsule", name: "Capsule" },
            { id: "ellipsoid", name: "Ellipsoid", parent: { id: "capsule" } },
            { id: "balcony", name: "Balcony", parent: { id: "capsule" } },
          ],
        },
        { designDataEnabled: true, typeDataEnabled: true, portDataEnabled: true, connectorDataEnabled: true },
      );

      expect(getKitChildNodes(hierarchy, hierarchy.nodesByKey.get("kind:capsule")!).map((node) => node.label)).toEqual(["Balcony", "Ellipsoid"]);
      expect(getKitChildNodes(hierarchy, hierarchy.nodesByKey.get("kind:ellipsoid")!)).toEqual([]);
    });

    it("attaches connectors beneath their resolved kind parent and derives connector selection", () => {
      const hierarchy = buildKitHierarchy(
        {
          name: "Metabolism",
          types: [{ id: "l", name: "L" }],
          connectors: [{ id: "entry", typeId: "l", name: "Entry" }],
        },
        { designDataEnabled: true, typeDataEnabled: true, portDataEnabled: true, connectorDataEnabled: true },
      );

      expect(getKitNodePath(hierarchy, "connector:entry").map((node) => node.label)).toEqual(["Kit", "Metabolism", "Types", "L", "Entry"]);
      expect(getKitNodeSelection(hierarchy.nodesByKey.get("connector:entry")!)).toEqual({
        designIds: [],
        typeIds: [],
        portIds: [],
        connectorIds: ["entry"],
      });
    });

    it("falls back to the first populated group when no artifact is selected", () => {
      const hierarchy = buildKitHierarchy(
        {
          name: "Metabolism",
          designs: [{ id: "tower", name: "Tower" }],
          types: [{ id: "capsule", name: "Capsule" }],
        },
        { designDataEnabled: true, typeDataEnabled: true, portDataEnabled: true, connectorDataEnabled: true },
      );

      expect(getDefaultKitNodeKey(hierarchy)).toBe("design:tower");
      expect(getSelectedKitNodeKey(hierarchy, { designIds: [], typeIds: [], portIds: [], connectorIds: [] })).toBeUndefined();
    });
  });

  describe("buildScenePieceAssets", () => {
    it("selects the untagged default representation when no tags are requested", () => {
      const kit = {
        types: [
          {
            id: "kind-1",
            representations: [
              { id: "representation-tagged", file: { id: "file-tagged" }, tags: [{ id: "tag-1" }] },
              { id: "representation-default", file: { id: "file-default" } },
            ],
          },
        ],
        files: [
          { id: "file-tagged", name: "tagged.glb", blob: "data:representation/gltf-binary;base64,AAA" },
          { id: "file-default", name: "default.glb", blob: "data:representation/gltf-binary;base64,BBB" },
        ],
      } as unknown as Kit;

      const assets = buildScenePieceAssets(kit, [{ piece: { id: "piece-1", type: { id: "kind-1" }, plane: testPlane, center: testCenter } as Piece, status: "default" }]);

      expect(assets[0]?.representationSource).toBe("data:representation/gltf-binary;base64,BBB");
      expect(assets[0]?.representationName).toBe("default.glb");
      expect(assets[0]?.status).toBe("default");
    });

    it("falls back to the first representation when the kind has no untagged default representation", () => {
      const kit = {
        types: [
          {
            id: "kind-1",
            representations: [
              { id: "representation-first", file: { id: "file-first" }, tags: [{ id: "tag-1" }] },
              { id: "representation-second", file: { id: "file-second" }, tags: [{ id: "tag-2" }] },
            ],
          },
        ],
        files: [
          { id: "file-first", name: "first.glb", blob: "data:representation/gltf-binary;base64,AAA" },
          { id: "file-second", name: "second.glb", blob: "data:representation/gltf-binary;base64,BBB" },
        ],
      } as unknown as Kit;

      const assets = buildScenePieceAssets(kit, [{ piece: { id: "piece-1", type: { id: "kind-1" }, plane: testPlane, center: testCenter } as Piece, status: "modified" }]);

      expect(assets[0]?.representationSource).toBe("data:representation/gltf-binary;base64,AAA");
      expect(assets[0]?.representationName).toBe("first.glb");
      expect(assets[0]?.status).toBe("modified");
    });

    it("keeps pieces in the scene and falls back to placeholder geometry when no file source can be resolved", () => {
      const kit = {
        types: [{ id: "kind-1", representations: [{ id: "representation-1", file: { id: "file-1" } }] }],
        files: [{ id: "file-1", name: "missing.glb" }],
      } as unknown as Kit;

      const assets = buildScenePieceAssets(kit, [{ piece: { id: "piece-1", type: { id: "kind-1" }, plane: testPlane, center: testCenter } as Piece, status: "added" }]);

      expect(assets).toHaveLength(1);
      expect(assets[0]?.representationSource).toBeUndefined();
      expect(assets[0]?.piece.id).toBe("piece-1");
      expect(assets[0]?.status).toBe("added");
    });

    it("uses kit file.remote as the representation URL when blob is absent (sketchpad-shaped kits)", () => {
      const kit = {
        types: [{ id: "kind-1", representations: [{ id: "representation-1", file: { id: "file-1" } }] }],
        files: [{ id: "file-1", name: "remote-mesh.glb", remote: "https://example.com/assets/remote-mesh.glb" }],
      } as unknown as Kit;

      const assets = buildScenePieceAssets(kit, [{ piece: { id: "piece-1", type: { id: "kind-1" }, plane: testPlane, center: testCenter } as Piece, status: "default" }]);

      expect(assets[0]?.representationSource).toBe("https://example.com/assets/remote-mesh.glb");
      expect(assets[0]?.representationName).toBe("remote-mesh.glb");
    });

    it("resolves the kind by type name when the piece omits type id", () => {
      const kit = {
        types: [{ id: "kind-1", name: "Capsule", representations: [{ id: "representation-1", file: { id: "file-1" } }] }],
        files: [{ id: "file-1", name: "cap.glb", blob: "data:representation/gltf-binary;base64,QUFB" }],
      } as unknown as Kit;

      const assets = buildScenePieceAssets(kit, [{ piece: { id: "piece-1", type: { name: "Capsule" }, plane: testPlane, center: testCenter } as any as Piece, status: "default" }]);

      expect(assets[0]?.representationSource).toBe("data:representation/gltf-binary;base64,QUFB");
    });
  });

  describe("buildTypeRepresentationAsset", () => {
    it("selects the untagged default representation for a kind when the kit provides matching files", () => {
      const asset = buildTypeRepresentationAsset(
        {
          id: "kind-1",
          representations: [
            { id: "representation-tagged", file: { id: "file-tagged" }, tags: [{ id: "tag-1" }] },
            { id: "representation-default", file: { id: "file-default" } },
          ],
        } as unknown as KitKindPlain,
        {
          files: [
            { id: "file-tagged", name: "tagged.glb", blob: "data:representation/gltf-binary;base64,AAA" },
            { id: "file-default", name: "default.glb", blob: "data:representation/gltf-binary;base64,BBB" },
          ],
        } as unknown as Kit,
      );

      expect(asset).toEqual({
        representationName: "default.glb",
        representationSource: "data:representation/gltf-binary;base64,BBB",
      });
    });

    it("prefers a gltf file when the default selection points at a non-gltf source", () => {
      const asset = buildTypeRepresentationAsset(
        {
          id: "kind-1",
          representations: [
            { id: "representation-default", file: { id: "file-default" } },
            { id: "representation-gltf", file: { id: "file-gltf" }, tags: [{ id: "tag-1" }] },
          ],
        } as unknown as KitKindPlain,
        {
          files: [
            { id: "file-default", name: "default.obj", blob: "data:representation/obj;base64,AAA" },
            { id: "file-gltf", name: "fallback.glb", blob: "data:representation/gltf-binary;base64,BBB" },
          ],
        } as unknown as Kit,
      );

      expect(asset).toEqual({
        representationName: "fallback.glb",
        representationSource: "data:representation/gltf-binary;base64,BBB",
      });
    });
  });

  describe("buildTypeZoomBox", () => {
    it("covers connector roots and arrow tips in scene space", () => {
      const connector = {
        id: "connector-1",
        point: { x: 1, y: 2, z: 3 },
        direction: { x: 0, y: 1, z: 0 },
      } as unknown as Connector;
      const box = buildTypeZoomBox({
        id: "kind-1",
        connectors: [connector],
      } as unknown as KitKindPlain);

      const start = toSceneVector(connector.point!);
      const end = start.clone().add(new THREE.Vector3(connector.direction!.x, connector.direction!.y, connector.direction!.z).applyMatrix4(SEMIO_TO_THREE_BASIS).normalize().multiplyScalar(TYPE_CONNECTOR_ARROW_LENGTH));

      expect(box.containsPoint(start)).toBe(true);
      expect(box.containsPoint(end)).toBe(true);
      expect(box.min.x).toBeLessThan(start.x);
      expect(box.min.y).toBeLessThan(Math.min(start.y, end.y));
      expect(box.min.z).toBeLessThan(Math.min(start.z, end.z));
      expect(box.max.x).toBeGreaterThan(start.x);
      expect(box.max.y).toBeGreaterThan(Math.max(start.y, end.y));
      expect(box.max.z).toBeGreaterThan(Math.max(start.z, end.z));
    });

    it("falls back to a centered placeholder box when the kind has no connectors", () => {
      const box = buildTypeZoomBox({ id: "kind-1", connectors: [] } as unknown as KitKindPlain);

      expect(box.min.toArray()).toEqual([-0.5, -0.5, -0.5]);
      expect(box.max.toArray()).toEqual([0.5, 0.5, 0.5]);
    });
  });

  describe("toScenePieceMatrix", () => {
    it("converts semio planes into Three coordinates without tipping GLTF local axes onto their side", () => {
      const matrix = toScenePieceMatrix(testPlane);
      const xAxis = new THREE.Vector3();
      const yAxis = new THREE.Vector3();
      const zAxis = new THREE.Vector3();
      matrix.extractBasis(xAxis, yAxis, zAxis);

      expect(xAxis.toArray()).toEqual([1, 0, 0]);
      expect(yAxis.toArray()).toEqual([0, 1, 0]);
      expect(zAxis.toArray()).toEqual([0, 0, 1]);
    });
  });

  describe("scene representation material normalization", () => {
    it("overwrites imported mesh, line, and point materials with homogeneous scene materials and adds mesh outlines", () => {
      const mesh = new THREE.Mesh(new THREE.BoxGeometry(1, 1, 1), new THREE.MeshBasicMaterial({ color: "#ff0000" }));
      const line = new THREE.LineSegments(new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(0, 0, 0), new THREE.Vector3(1, 0, 0)]), new THREE.LineBasicMaterial({ color: "#00ff00" }));
      const points = new THREE.Points(new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(0, 0, 0)]), new THREE.PointsMaterial({ color: "#0000ff", size: 5 }));
      const source = new THREE.Group();
      source.add(mesh, line, points);

      const clone = cloneSceneRepresentationWithHomogeneousMaterials(source, "#112233", "#445566");

      const clonedMesh = clone.children[0] as THREE.Mesh;
      const clonedLine = clone.children[1] as THREE.LineSegments;
      const clonedPoints = clone.children[2] as THREE.Points;
      const clonedOutline = clonedMesh.children[0] as THREE.LineSegments;

      expect(clonedMesh.material).toBeInstanceOf(THREE.MeshStandardMaterial);
      expect((clonedMesh.material as THREE.MeshStandardMaterial).color.getHexString()).toBe("112233");
      expect(clonedOutline).toBeInstanceOf(THREE.LineSegments);
      expect(clonedOutline.userData[SCENE_MESH_OUTLINE_USER_DATA_KEY]).toBe(true);
      expect(clonedOutline.material).toBeInstanceOf(THREE.LineBasicMaterial);
      expect((clonedOutline.material as THREE.LineBasicMaterial).color.getHexString()).toBe("445566");
      expect(clonedLine.material).toBeInstanceOf(THREE.LineBasicMaterial);
      expect((clonedLine.material as THREE.LineBasicMaterial).color.getHexString()).toBe("445566");
      expect(clonedPoints.material).toBeInstanceOf(THREE.PointsMaterial);
      expect((clonedPoints.material as THREE.PointsMaterial).color.getHexString()).toBe("445566");
    });

    it("recolors imported mesh and line materials consistently for interaction and removed states", () => {
      const source = new THREE.Group();
      source.add(
        new THREE.Mesh(new THREE.BoxGeometry(1, 1, 1), new THREE.MeshBasicMaterial({ color: "#ff0000" })),
        new THREE.LineSegments(new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(0, 0, 0), new THREE.Vector3(1, 0, 0)]), new THREE.LineBasicMaterial({ color: "#00ff00" })),
      );
      const clone = cloneSceneRepresentationWithHomogeneousMaterials(source, "#111111", "#222222");

      applySceneRepresentationColorState(clone, {
        meshColor: "#334455",
        lineColor: "#556677",
        emissiveColor: "#778899",
        emissiveIntensity: 0.35,
        opacity: 0.35,
      });

      const clonedMesh = clone.children[0] as THREE.Mesh;
      const clonedLine = clone.children[1] as THREE.LineSegments;
      const clonedOutline = clonedMesh.children[0] as THREE.LineSegments;
      const meshMaterial = clonedMesh.material as THREE.MeshStandardMaterial;
      const lineMaterial = clonedLine.material as THREE.LineBasicMaterial;
      const outlineMaterial = clonedOutline.material as THREE.LineBasicMaterial;

      expect(meshMaterial.color.getHexString()).toBe("334455");
      expect(meshMaterial.emissive.getHexString()).toBe("778899");
      expect(meshMaterial.emissiveIntensity).toBe(0.35);
      expect(meshMaterial.transparent).toBe(true);
      expect(meshMaterial.opacity).toBe(0.35);
      expect(outlineMaterial.color.getHexString()).toBe("556677");
      expect(outlineMaterial.transparent).toBe(true);
      expect(outlineMaterial.opacity).toBe(0.35);
      expect(lineMaterial.color.getHexString()).toBe("556677");
      expect(lineMaterial.transparent).toBe(true);
      expect(lineMaterial.opacity).toBe(0.35);
    });

    it("derives selected and removed imported scene colors from the shared scene tokens", () => {
      const originalDocument = (globalThis as typeof globalThis & { document?: Document }).document;
      const originalGetComputedStyle = globalThis.getComputedStyle;
      const computedValues: Record<string, string> = {
        "--muted-foreground": "#888888",
        "--accent": "#123456",
        "--accent-secondary": "#abcdef",
        "--color-removed": "#ff0000",
        "--color-new": "#00ff00",
        "--color-modified": "#ffff00",
        "--color-changed-selected": "#654321",
        "--color-changed-hovered": "#fedcba",
      };

      Object.defineProperty(globalThis, "document", {
        value: { documentElement: {} },
        configurable: true,
      });
      Object.defineProperty(globalThis, "getComputedStyle", {
        value: () => ({
          getPropertyValue: (name: string) => computedValues[name] ?? "",
        }),
        configurable: true,
      });

      try {
        const selectedState = getSceneRepresentationColorState("default", true, false);
        const removedState = getSceneRepresentationColorState("removed", false, false);

        expect(selectedState.meshColor).toBe("#123456");
        expect(selectedState.lineColor).toBe("#123456");
        expect(selectedState.emissiveColor).toBe("#123456");
        expect(selectedState.emissiveIntensity).toBe(0.35);
        expect(selectedState.opacity).toBe(1);
        expect(removedState.meshColor).toBe("#ff0000");
        expect(removedState.lineColor).toBe("#ff0000");
        expect(removedState.emissiveColor).toBe("#ff0000");
        expect(removedState.emissiveIntensity).toBe(0.4);
        expect(removedState.opacity).toBe(0.35);
      } finally {
        if (originalDocument === undefined) {
          Reflect.deleteProperty(globalThis as Record<string, unknown>, "document");
        } else {
          Object.defineProperty(globalThis, "document", {
            value: originalDocument,
            configurable: true,
          });
        }
        if (originalGetComputedStyle === undefined) {
          Reflect.deleteProperty(globalThis as Record<string, unknown>, "getComputedStyle");
        } else {
          Object.defineProperty(globalThis, "getComputedStyle", {
            value: originalGetComputedStyle,
            configurable: true,
          });
        }
      }
    });
  });

  describe("normalizeHover", () => {
    it("fills missing hover fields with null", () => {
      expect(normalizeHover()).toEqual({ pieceId: null, connectionId: null });
      expect(normalizeHover({ pieceId: "piece-1" })).toEqual({ pieceId: "piece-1", connectionId: null });
    });
  });

  describe("scene helpers", () => {
    it("keeps the gizmo in the bottom-right corner with a larger inset so it stays visible", () => {
      expect(resolveSceneGizmoViewportPlacement({ width: 1280, height: 720 })).toEqual({
        alignment: "bottom-right",
        margin: [56, 40],
      });

      expect(resolveSceneGizmoViewportPlacement({ width: 120, height: 160 })).toEqual({
        alignment: "bottom-right",
        margin: [26, 22],
      });

      expect(resolveSceneGizmoViewportPlacement({ width: 40, height: 48 })).toEqual({
        alignment: "bottom-right",
        margin: [26, 18],
      });
    });
  });

  describe("buildSceneSnapshot", () => {
    it("includes piece and connection statuses for flattened scene rendering", () => {
      const pieceA = {
        id: "piece-a",
        type: { id: "kind-1" },
        plane: testPlane,
        center: { u: 0, v: 0 },
      } as unknown as Piece;
      const pieceB = {
        id: "piece-b",
        type: { id: "kind-1" },
        plane: { ...testPlane, origin: { x: 2, y: 0, z: 0 } },
        center: { u: 2, v: 0 },
      } as unknown as Piece;
      const pieceC = {
        id: "piece-c",
        type: { id: "kind-1" },
        plane: { ...testPlane, origin: { x: 4, y: 0, z: 0 } },
        center: { u: 4, v: 0 },
      } as unknown as Piece;

      const connectionA = {
        id: "connection-a",
        connected: { piece: { id: "piece-a" } },
        connecting: { piece: { id: "piece-b" } },
      } as unknown as Connection;
      const connectionB = {
        id: "connection-b",
        connected: { piece: { id: "piece-b" } },
        connecting: { piece: { id: "piece-c" } },
      } as unknown as Connection;

      const design = {
        id: "design-1",
        pieces: [pieceA, pieceB],
        connections: [connectionA],
      } as unknown as Design;

      const kit = {
        designs: [design],
        types: [{ id: "kind-1" }],
      } as unknown as Kit;

      const diff = {
        pieces: {
          added: [pieceC],
          updated: [{ piece: { id: "piece-b" }, diff: {} }],
        },
        connections: {
          added: [connectionB],
          updated: [{ connection: { id: "connection-a" }, diff: {} }],
        },
      } as unknown as DesignDiff;

      const snapshot = buildSceneSnapshot(design, diff);

      expect(snapshot.pieces.map((asset) => [asset.piece.id, asset.status])).toEqual([
        ["piece-a", "default"],
        ["piece-b", "modified"],
        ["piece-c", "added"],
      ]);
      expect(snapshot.connections.map((asset) => [asset.connection.id, asset.status])).toEqual([
        ["connection-a", "modified"],
        ["connection-b", "added"],
      ]);
    });

    it("keeps existing scene pieces when the next diff version has no plane and propagates connection status to child", () => {
      const pieceA = {
        id: "piece-a",
        type: { id: "kind-1" },
        plane: testPlane,
        center: { u: 0, v: 0 },
      } as unknown as Piece;
      const pieceB = {
        id: "piece-b",
        type: { id: "kind-1" },
        plane: { ...testPlane, origin: { x: 2, y: 0, z: 0 } },
        center: { u: 2, v: 0 },
      } as unknown as Piece;

      const connectionA = {
        id: "connection-a",
        connected: { piece: { id: "piece-a" } },
        connecting: { piece: { id: "piece-b" } },
      } as unknown as Connection;

      const design = {
        id: "design-1",
        pieces: [pieceA, pieceB],
        connections: [connectionA],
      } as unknown as Design;

      const diff = {
        pieces: {
          removed: [{ id: "piece-b" }],
          added: [{ ...pieceB, plane: undefined }],
        },
        connections: {
          updated: [{ connection: { id: "connection-a" }, diff: {} }],
        },
      } as unknown as DesignDiff;

      const snapshot = buildSceneSnapshot(design, diff);
      const pieceStatuses = new Map(snapshot.pieces.map((asset) => [asset.piece.id, asset.status] as const));

      expect(pieceStatuses).toEqual(
        new Map([
          ["piece-a", "default"],
          ["piece-b", "added"],
        ]),
      );
      expect(snapshot.pieces.find((asset) => asset.piece.id === "piece-b")?.piece.plane).toEqual(pieceB.plane);
      expect(snapshot.connections.map((asset) => [asset.connection.id, asset.status])).toEqual([["connection-a", "modified"]]);
    });

    it("propagates modified to child endpoint of a reparented connection", () => {
      const pieceA = {
        id: "piece-a",
        type: { id: "kind-1" },
        plane: testPlane,
        center: { u: 0, v: 0 },
      } as unknown as Piece;
      const pieceB = {
        id: "piece-b",
        type: { id: "kind-1" },
        plane: { ...testPlane, origin: { x: 2, y: 0, z: 0 } },
        center: { u: 2, v: 0 },
      } as unknown as Piece;
      const pieceC = {
        id: "piece-c",
        type: { id: "kind-1" },
        plane: { ...testPlane, origin: { x: 4, y: 0, z: 0 } },
        center: { u: 4, v: 0 },
      } as unknown as Piece;

      const connectionA = {
        id: "connection-a",
        connected: { piece: { id: "piece-a" } },
        connecting: { piece: { id: "piece-b" } },
      } as unknown as Connection;

      const design = {
        id: "design-1",
        pieces: [pieceA, pieceB, pieceC],
        connections: [connectionA],
      } as unknown as Design;

      const diff = {
        connections: {
          updated: [
            {
              connection: { id: "connection-a" },
              diff: {
                connected: { piece: { id: "piece-c" } },
              },
            },
          ],
        },
      } as unknown as DesignDiff;

      const snapshot = buildSceneSnapshot(design, diff);
      const pieceStatuses = new Map(snapshot.pieces.map((asset) => [asset.piece.id, asset.status] as const));

      expect(pieceStatuses).toEqual(
        new Map([
          ["piece-a", "default"],
          ["piece-b", "modified"],
          ["piece-c", "default"],
        ]),
      );
      expect(snapshot.connections.map((asset) => [asset.connection.id, asset.status])).toEqual([["connection-a", "modified"]]);
      expect(snapshot.connections[0]?.sourcePiece.id).toBe("piece-c");
      expect(snapshot.connections[0]?.targetPiece.id).toBe("piece-b");
    });
  });

  describe("resolveSemioDiagramOriginPixels", () => {
    it("uses u=0 and diagramY=0 (piece v=0)", () => {
      expect(
        resolveSemioDiagramOriginPixels(
          (u) => 100 + u * 2,
          (y) => 200 + y * 3,
        ),
      ).toEqual({ x: 100, y: 200 });
    });
  });

  describe("resolveSemioDiagramUnitAxisTips", () => {
    it("maps +1 u at v=0 and +1 v as diagramY -1", () => {
      expect(
        resolveSemioDiagramUnitAxisTips(
          (u) => u * 10,
          (y) => 100 + y * 5,
        ),
      ).toEqual({
        uTip: { x: 10, y: 100 },
        vTip: { x: 0, y: 95 },
      });
    });
  });

  describe("buildDiagramSnapshot with diff", () => {
    it("annotates updated pieces as modified and propagates piece status to connections (one-hop)", () => {
      const design = {
        id: "d",
        pieces: [
          { id: "piece-a", type: { id: "k" }, plane: testPlane, center: { u: 0, v: 0 } } as unknown as Piece,
          { id: "piece-b", type: { id: "k" }, plane: testPlane, center: { u: 5, v: 3 } } as unknown as Piece,
          { id: "piece-c", type: { id: "k" }, plane: testPlane, center: { u: 10, v: 0 } } as unknown as Piece,
        ],
        connections: [{ id: "conn-ab", connected: { piece: { id: "piece-a" } }, connecting: { piece: { id: "piece-b" } } } as unknown as Connection],
      } as unknown as Design;

      const diff: DesignDiff = {
        pieces: {
          updated: [{ piece: { id: "piece-a" }, diff: { center: { u: 1, v: 2 } } }],
        },
        connections: {
          updated: [{ connection: { id: "conn-ab" }, diff: { u: 1, v: 2 } }],
        },
      } as unknown as DesignDiff;

      const snapshot = buildDiagramSnapshot(design, 12, diff);

      const pieceStatuses = new Map(snapshot.points.map((p) => [p.id, p.status]));
      expect(pieceStatuses.get("piece-a")).toBe("modified");
      // piece-b is NOT in the diff → stays default (no connection→piece cascade)
      expect(pieceStatuses.get("piece-b")).toBe("default");
      expect(pieceStatuses.get("piece-c")).toBe("default");

      // Updated pieces should be at their NEW positions (diff applied)
      const pieceA = snapshot.points.find((p) => p.id === "piece-a")!;
      expect(pieceA.u).toBe(1);
      expect(pieceA.v).toBe(2);

      // piece-b is not in the diff itself, stays at original center
      const pieceB = snapshot.points.find((p) => p.id === "piece-b")!;
      expect(pieceB.u).toBe(5);
      expect(pieceB.v).toBe(3);

      const connectionStatuses = new Map(snapshot.lines.map((l) => [l.id, l.status]));
      // conn-ab is explicitly modified from diff + piece-a is modified → stays modified
      expect(connectionStatuses.get("conn-ab")).toBe("modified");
    });

    it("propagates piece status one-hop to adjacent connections only (no multi-hop cascade)", () => {
      const design = {
        id: "d",
        pieces: [
          { id: "piece-a", type: { id: "k" }, plane: testPlane, center: { u: 0, v: 0 } } as unknown as Piece,
          { id: "piece-b", type: { id: "k" }, plane: testPlane, center: { u: 5, v: 3 } } as unknown as Piece,
          { id: "piece-c", type: { id: "k" }, plane: testPlane, center: { u: 10, v: 0 } } as unknown as Piece,
        ],
        connections: [
          { id: "conn-ab", connected: { piece: { id: "piece-a" } }, connecting: { piece: { id: "piece-b" } } } as unknown as Connection,
          { id: "conn-bc", connected: { piece: { id: "piece-b" } }, connecting: { piece: { id: "piece-c" } } } as unknown as Connection,
        ],
      } as unknown as Design;

      const diff: DesignDiff = {
        pieces: {
          updated: [{ piece: { id: "piece-a" }, diff: { center: { u: 1, v: 2 } } }],
        },
      } as unknown as DesignDiff;

      const snapshot = buildDiagramSnapshot(design, 12, diff);

      const pieceStatuses = new Map(snapshot.points.map((p) => [p.id, p.status]));
      // piece-a explicitly modified
      expect(pieceStatuses.get("piece-a")).toBe("modified");
      // piece-b and piece-c are NOT in the diff → stay default (no connection→piece cascade)
      expect(pieceStatuses.get("piece-b")).toBe("default");
      expect(pieceStatuses.get("piece-c")).toBe("default");

      const connectionStatuses = new Map(snapshot.lines.map((l) => [l.id, l.status]));
      // conn-ab: piece-a is modified → conn-ab becomes modified (piece→connection one-hop)
      expect(connectionStatuses.get("conn-ab")).toBe("modified");
      // conn-bc: neither piece-b nor piece-c is non-default → conn-bc stays default
      expect(connectionStatuses.get("conn-bc")).toBe("default");
    });

    it("propagates status to connections when endpoint piece is added or removed", () => {
      const design = {
        id: "d",
        pieces: [{ id: "piece-a", type: { id: "k" }, plane: testPlane, center: { u: 0, v: 0 } } as unknown as Piece, { id: "piece-b", type: { id: "k" }, plane: testPlane, center: { u: 5, v: 3 } } as unknown as Piece],
        connections: [
          { id: "conn-ab", connected: { piece: { id: "piece-a" } }, connecting: { piece: { id: "piece-b" } } } as unknown as Connection,
          { id: "conn-ac", connected: { piece: { id: "piece-a" } }, connecting: { piece: { id: "piece-c" } } } as unknown as Connection,
        ],
      } as unknown as Design;

      const diff: DesignDiff = {
        pieces: {
          removed: [{ id: "piece-b" } as unknown as Piece],
          added: [{ id: "piece-c", type: { id: "k" }, plane: testPlane, center: { u: 8, v: 1 } } as unknown as Piece],
        },
      } as unknown as DesignDiff;

      const snapshot = buildDiagramSnapshot(design, 12, diff);

      const connectionStatuses = new Map(snapshot.lines.map((l) => [l.id, l.status]));
      expect(connectionStatuses.get("conn-ab")).toBe("modified");
      expect(connectionStatuses.get("conn-ac")).toBe("modified");
    });

    it("does not propagate status when diff is absent", () => {
      const design = {
        id: "d",
        pieces: [{ id: "piece-a", type: { id: "k" }, plane: testPlane, center: { u: 0, v: 0 } } as unknown as Piece, { id: "piece-b", type: { id: "k" }, plane: testPlane, center: { u: 5, v: 3 } } as unknown as Piece],
        connections: [{ id: "conn-ab", connected: { piece: { id: "piece-a" } }, connecting: { piece: { id: "piece-b" } } } as unknown as Connection],
      } as unknown as Design;

      const snapshot = buildDiagramSnapshot(design, 12, undefined);

      const connectionStatuses = new Map(snapshot.lines.map((l) => [l.id, l.status]));
      expect(connectionStatuses.get("conn-ab")).toBe("default");
    });

    it("annotates added pieces as added and removed pieces as removed", () => {
      const design = {
        id: "d",
        pieces: [{ id: "piece-a", type: { id: "k" }, plane: testPlane, center: { u: 0, v: 0 } } as unknown as Piece, { id: "piece-b", type: { id: "k" }, plane: testPlane, center: { u: 5, v: 3 } } as unknown as Piece],
        connections: [],
      } as unknown as Design;

      const diff: DesignDiff = {
        pieces: {
          removed: [{ id: "piece-b" } as unknown as Piece],
          added: [{ id: "piece-c", type: { id: "k" }, plane: testPlane, center: { u: 8, v: 1 } } as unknown as Piece],
        },
      } as unknown as DesignDiff;

      const snapshot = buildDiagramSnapshot(design, 12, diff);

      const pieceStatuses = new Map(snapshot.points.map((p) => [p.id, p.status]));
      expect(pieceStatuses.get("piece-a")).toBe("default");
      expect(pieceStatuses.get("piece-b")).toBe("removed");
      expect(pieceStatuses.get("piece-c")).toBe("added");
    });

    it("connection-only diff colors only the explicit connection (no cascade to pieces or child connections)", () => {
      // Simulates dragging a connected piece: only the parent connection is in the diff.
      // Only conn-ab should be modified. No cascade to pieces or downstream connections.
      const design = {
        id: "d",
        pieces: [
          { id: "piece-a", type: { id: "k" }, plane: testPlane, center: { u: 0, v: 0 } } as unknown as Piece,
          { id: "piece-b", type: { id: "k" }, plane: testPlane, center: { u: 5, v: 3 } } as unknown as Piece,
          { id: "piece-c", type: { id: "k" }, plane: testPlane, center: { u: 10, v: 0 } } as unknown as Piece,
          { id: "piece-d", type: { id: "k" }, plane: testPlane, center: { u: 15, v: 0 } } as unknown as Piece,
        ],
        connections: [
          { id: "conn-ab", connected: { piece: { id: "piece-a" } }, connecting: { piece: { id: "piece-b" } } } as unknown as Connection,
          { id: "conn-bc", connected: { piece: { id: "piece-b" } }, connecting: { piece: { id: "piece-c" } } } as unknown as Connection,
        ],
      } as unknown as Design;

      // Only the connection is in the diff (like Design.dragBySelection for a connected piece)
      const diff: DesignDiff = {
        connections: {
          updated: [{ connection: { id: "conn-ab" }, diff: { u: 2, v: 1 } }],
        },
      } as unknown as DesignDiff;

      const snapshot = buildDiagramSnapshot(design, 12, diff);

      const pieceStatuses = new Map(snapshot.points.map((p) => [p.id, p.status]));
      // No pieces are in the diff → all stay default (no connection→piece cascade)
      expect(pieceStatuses.get("piece-a")).toBe("default");
      expect(pieceStatuses.get("piece-b")).toBe("default");
      expect(pieceStatuses.get("piece-c")).toBe("default");
      expect(pieceStatuses.get("piece-d")).toBe("default");

      const connectionStatuses = new Map(snapshot.lines.map((l) => [l.id, l.status]));
      // conn-ab explicitly modified from diff
      expect(connectionStatuses.get("conn-ab")).toBe("modified");
      // conn-bc has no non-default endpoint pieces → stays default
      expect(connectionStatuses.get("conn-bc")).toBe("default");
    });
  });

  describe("computeDiagramSelectionOverlayRect", () => {
    it("returns null when no pieces or connections are selected", () => {
      const design = {
        id: "d",
        pieces: [{ id: "a", type: { id: "k" }, plane: testPlane, center: { u: 0, v: 0 } } as unknown as Piece],
        connections: [],
      } as unknown as Design;
      const snapshot = buildDiagramSnapshot(design, 12, undefined);
      expect(
        computeDiagramSelectionOverlayRect(
          snapshot,
          new Set(),
          new Set(),
          (u) => u,
          (y) => y,
          2,
          2,
        ),
      ).toBeNull();
    });

    it("wraps selected piece centers with padding in pixel space", () => {
      const design = {
        id: "d",
        pieces: [{ id: "a", type: { id: "k" }, plane: testPlane, center: { u: 0, v: 0 } } as unknown as Piece, { id: "b", type: { id: "k" }, plane: testPlane, center: { u: 10, v: 4 } } as unknown as Piece],
        connections: [],
      } as unknown as Design;
      const snapshot = buildDiagramSnapshot(design, 12, undefined);
      const r = computeDiagramSelectionOverlayRect(
        snapshot,
        new Set(["a", "b"]),
        new Set(),
        (u) => u * 10,
        (y) => y * 10,
        5,
        2,
      );
      expect(r).toEqual({ x: -5, y: -45, width: 110, height: 50 });
    });
  });

  describe("computeSceneSelectionUnionBox", () => {
    it("returns null when no roots match selected ids", () => {
      const roots = new Map<string, THREE.Object3D>();
      expect(computeSceneSelectionUnionBox(roots, new Set(["missing"]), new Set())).toBeNull();
    });

    it("unions world-axis-aligned bounds from registered Object3D roots", () => {
      const roots = new Map<string, THREE.Object3D>();
      const g = new THREE.Group();
      const mesh = new THREE.Mesh(new THREE.BoxGeometry(2, 2, 2));
      g.add(mesh);
      g.position.set(5, 0, 0);
      g.updateMatrixWorld(true);
      roots.set("a", g);
      const box = computeSceneSelectionUnionBox(roots, new Set(["a"]), new Set());
      expect(box).not.toBeNull();
      const c = new THREE.Vector3();
      box!.getCenter(c);
      expect(c.x).toBeCloseTo(5, 4);
    });
  });

  describe("buildDesignClipboardData", () => {
    const baseDesign = {
      id: "design-1",
      name: "TestDesign",
      createdAt: "2026-01-01",
      updatedAt: "2026-01-01",
      pieces: [
        { id: "p1", type: { id: "t1" }, center: { u: 0, v: 0 } } as unknown as Piece,
        { id: "p2", type: { id: "t1" }, center: { u: 1, v: 0 } } as unknown as Piece,
        { id: "p3", type: { id: "t1" }, center: { u: 2, v: 0 } } as unknown as Piece,
      ],
      connections: [
        { id: "c1", connected: { piece: { id: "p1" } }, connecting: { piece: { id: "p2" } } } as unknown as Connection,
        { id: "c2", connected: { piece: { id: "p2" } }, connecting: { piece: { id: "p3" } } } as unknown as Connection,
      ],
    } as unknown as Design;

    const baseDiff: DesignDiff = {
      pieces: {
        added: [{ id: "p4", type: { id: "t1" }, center: { u: 3, v: 0 } } as unknown as Piece],
        removed: [{ id: "p3" }],
        updated: [{ piece: { id: "p2" }, diff: { name: "UpdatedP2" } }],
      },
      connections: {
        added: [{ id: "c3", connected: { piece: { id: "p1" } }, connecting: { piece: { id: "p4" } } } as unknown as Connection],
        removed: [{ id: "c2" }],
        updated: [{ connection: { id: "c1" }, diff: {} }],
      },
    };

    it("copies the full design when no diff and no selection", () => {
      const result = buildDesignClipboardData(baseDesign, undefined, undefined);
      expect(result.design).toBe(baseDesign);
      expect(result.designDiff).toBeUndefined();
    });

    it("copies the full design when no diff and empty selection", () => {
      const result = buildDesignClipboardData(baseDesign, undefined, { pieceIds: [], connectionIds: [] });
      expect(result.design).toBe(baseDesign);
      expect(result.designDiff).toBeUndefined();
    });

    it("copies selected pieces and connections when no diff and selection present", () => {
      const result = buildDesignClipboardData(baseDesign, undefined, { pieceIds: ["p1", "p2"], connectionIds: ["c1"] });
      expect(__itemsOf((result.design as PlainJsonObject | undefined)?.["pieces"]).map((p) => (p as Piece).id)).toEqual(["p1", "p2"]);
      expect(snapshotDesignConnections(result.design!).map((c) => c.id)).toEqual(["c1"]);
      expect(result.designDiff).toBeUndefined();
    });

    it("omits pieces/connections arrays when none are selected in a no-diff selection", () => {
      const result = buildDesignClipboardData(baseDesign, undefined, { pieceIds: ["p1"], connectionIds: [] });
      expect(__itemsOf((result.design as PlainJsonObject | undefined)?.["pieces"]).map((p) => (p as Piece).id)).toEqual(["p1"]);
      expect((result.design as unknown as { connections?: unknown }).connections).toBeUndefined();
    });

    it("copies the full diff when diff present and no selection", () => {
      const result = buildDesignClipboardData(baseDesign, baseDiff, undefined);
      expect(result.design).toBe(baseDesign);
      expect(result.designDiff).toBe(baseDiff);
    });

    it("copies the full diff when diff present and empty selection", () => {
      const result = buildDesignClipboardData(baseDesign, baseDiff, { pieceIds: [], connectionIds: [] });
      expect(result.design).toBe(baseDesign);
      expect(result.designDiff).toBe(baseDiff);
    });

    it("filters diff to selected pieces and connections when both diff and selection", () => {
      const result = buildDesignClipboardData(baseDesign, baseDiff, { pieceIds: ["p4", "p3"], connectionIds: ["c3"] });
      expect(result.design).toBe(baseDesign);
      expect((result.designDiff?.pieces?.added as Piece[] | undefined)?.map((p) => p.id)).toEqual(["p4"]);
      expect((result.designDiff?.pieces?.removed as { id?: string }[] | undefined)?.map((p) => p.id)).toEqual(["p3"]);
      expect(result.designDiff?.pieces?.updated).toEqual([]);
      expect((result.designDiff?.connections?.added as Connection[] | undefined)?.map((c) => c.id)).toEqual(["c3"]);
      expect(result.designDiff?.connections?.removed).toEqual([]);
      expect(result.designDiff?.connections?.updated).toEqual([]);
    });

    it("filters diff updated entries by piece/connection id", () => {
      const result = buildDesignClipboardData(baseDesign, baseDiff, { pieceIds: ["p2"], connectionIds: ["c1"] });
      expect((result.designDiff?.pieces?.updated as { piece: { id: string } }[] | undefined)?.map((u) => u.piece.id)).toEqual(["p2"]);
      expect((result.designDiff?.connections?.updated as { connection: { id: string } }[] | undefined)?.map((u) => u.connection.id)).toEqual(["c1"]);
    });
  });

  describe("canDisplayKitArtifactsFallback", () => {
    it("allows kit fallback for empty diagrams in diagram modes", () => {
      expect(canDisplayKitArtifactsFallback("show-diagram", false, undefined)).toBe(true);
      expect(canDisplayKitArtifactsFallback("show-diagram-diff", false, undefined)).toBe(true);
      expect(canDisplayKitArtifactsFallback("select-pieces", false, undefined)).toBe(true);
    });

    it("disallows kit fallback for show-design/show-scene", () => {
      expect(canDisplayKitArtifactsFallback("show-design", false, undefined)).toBe(false);
      expect(canDisplayKitArtifactsFallback("show-scene", false, undefined)).toBe(false);
      expect(canDisplayKitArtifactsFallback("show-diff", false, undefined)).toBe(false);
    });

    it("disallows kit fallback when diagram exists", () => {
      expect(canDisplayKitArtifactsFallback("show-diagram", true, undefined)).toBe(false);
      expect(canDisplayKitArtifactsFallback("select-pieces", true, undefined)).toBe(false);
    });

    it("disallows kit fallback when a design id is present (stale show-diagram + merged design)", () => {
      expect(canDisplayKitArtifactsFallback("show-diagram", false, "d1")).toBe(false);
    });
  });
}

// #region 📸AlgorithmApp

// Specs: Reusable algorithm app shell. Each algorithm declares typed windows (VecInput,
// PiecesSelectionInput, DesignDiffOutput, DesignOutput) and an AlgorithmApp creates
// the UIAppConfig and renders the UI composite component. Data flows through
// AlgorithmContext which provides kit, design, diff, selection, vec, and output state.
// WindowKinds: VecInput (2D vector pad), PiecesSelectionInput (Diagram with piece selection, no diff),
// DesignDiffOutput (Diagram with diff, no selection), DesignOutput (Diagram with no diff, no selection).
// Summary: Standardized algorithm IPO shell using typed WindowKind-based windows.

import {
  cn,
  createDefaultLayout,
  createWindowLayout,
  TreeItem,
  TreeRow,
  TreeSection,
  UI,
  WindowKind,
  type FooterItem,
  type SidePanelTabConfig,
  type TreeHeaderAction,
  type UIAppConfig,
  type UIWindowKindDefinition,
  type UIWindowLayout,
  type UIWindowLayoutAxisNode,
  type UIWindowLayoutStackNode,
} from "@elements/ui/elements";
import { AlertCircleIcon, DetailsIcon, PieceIcon } from "@semio/assets/icons";

/**
 * Context value for algorithm state shared across windows.
 **/
export interface AlgorithmContextValue {
  kit: Kit;
  design: Design;
  vec?: VecValue;
  onVecChange?: (v: VecValue) => void;
  vecMin?: VecValue;
  vecMax?: VecValue;
  /** Gap/shift/rise for move algorithms (3D placement); not the 2D drag {@link VecValue}. */
  moveVector?: MoveVector;
  onMoveVectorChange?: (v: MoveVector) => void;
  moveVectorMin?: MoveVector;
  moveVectorMax?: MoveVector;
  selectedPieceIds: string[];
  onSelectedPieceIdsChange?: (ids: string[]) => void;
  selectedConnectionIds?: string[];
  onSelectedConnectionIdsChange?: (ids: string[]) => void;
  designDiff?: DesignDiff;
  diffDesign?: Design;
  /** Passed to {@link SemioDiagram} as `layoutDiff` so linked pieces get u/v without mutating `design`. */
  diagramLayoutDiff?: DesignDiff;
  /** When set, the design-diff output window uses this as `layoutDiff` instead of `diagramLayoutDiff`. */
  diffDiagramLayoutDiff?: DesignDiff;
  /** When set, the design output window uses this as `layoutDiff` instead of `diagramLayoutDiff`. */
  outputDiagramLayoutDiff?: DesignDiff;
  outputDesign: Design;
  error?: string;
}

type AlgorithmDiagramWindowKind = WindowKind.PIECES_SELECTION_INPUT | WindowKind.SELECTION_INPUT | WindowKind.DESIGN_INPUT | WindowKind.DESIGN_DIFF_OUTPUT | WindowKind.DESIGN_OUTPUT;

interface AlgorithmDiagramViewportState {
  pan?: DiagramPan;
  zoom?: number;
  sourceKind?: AlgorithmDiagramWindowKind;
}

interface AlgorithmRuntimeContextValue extends AlgorithmContextValue {
  diagramViewport: AlgorithmDiagramViewportState;
  onDiagramViewportPanChange: (kind: AlgorithmDiagramWindowKind, pan: DiagramPan) => void;
  onDiagramViewportZoomChange: (kind: AlgorithmDiagramWindowKind, zoom: number) => void;
  filteredDesignDiff?: DesignDiff;
  diffTreeCategories: AlgorithmDiffTreeCategory[];
  setDiffEntriesChecked: (entryIds: string[], checked: boolean) => void;
}

const AlgorithmContext = React.createContext<AlgorithmRuntimeContextValue | null>(null);

type AlgorithmDiffEntryGroupKind = "pieces" | "connections";
type AlgorithmDiffEntryChangeKind = "added" | "removed" | "updated";

interface AlgorithmDiffTreeEntry {
  id: string;
  label: string;
  checked: boolean;
  /** Optional parameter rows (e.g. connection gap/shift/rise/rotation/turn/tilt/u/v) under the diff entry. */
  detail?: React.ReactNode;
}

interface AlgorithmDiffTreeGroup {
  id: string;
  label: string;
  groupKind: AlgorithmDiffEntryGroupKind;
  changeKind: AlgorithmDiffEntryChangeKind;
  entries: AlgorithmDiffTreeEntry[];
  totalCount: number;
  checkedCount: number;
}

interface AlgorithmDiffTreeCategory {
  id: string;
  label: string;
  groups: AlgorithmDiffTreeGroup[];
  totalCount: number;
  checkedCount: number;
}

const formatAlgorithmDiffId = (value: string | undefined, fallback: string): string => (value && value.length > 0 ? value : fallback);

const buildAlgorithmDiffEntryId = (groupKind: AlgorithmDiffEntryGroupKind, changeKind: AlgorithmDiffEntryChangeKind, id: string): string => `${groupKind}:${changeKind}:${id}`;

const resolveAlgorithmPieceDiffId = (pieceLike: unknown, fallbackIndex: number): string => {
  if (pieceLike && typeof pieceLike === "object" && "id" in pieceLike && typeof pieceLike.id === "string") {
    return formatAlgorithmDiffId(pieceLike.id, `piece-${fallbackIndex}`);
  }
  return `piece-${fallbackIndex}`;
};

const resolveAlgorithmConnectionDiffId = (connectionLike: unknown, fallbackIndex: number): string => {
  if (connectionLike && typeof connectionLike === "object") {
    if ("id" in connectionLike && typeof connectionLike.id === "string") {
      return formatAlgorithmDiffId(connectionLike.id, `connection-${fallbackIndex}`);
    }
    const connectedPieceId =
      "connected" in connectionLike &&
      connectionLike.connected &&
      typeof connectionLike.connected === "object" &&
      "piece" in connectionLike.connected &&
      connectionLike.connected.piece &&
      typeof connectionLike.connected.piece === "object" &&
      "id" in connectionLike.connected.piece &&
      typeof connectionLike.connected.piece.id === "string"
        ? connectionLike.connected.piece.id
        : undefined;
    const connectingPieceId =
      "connecting" in connectionLike &&
      connectionLike.connecting &&
      typeof connectionLike.connecting === "object" &&
      "piece" in connectionLike.connecting &&
      connectionLike.connecting.piece &&
      typeof connectionLike.connecting.piece === "object" &&
      "id" in connectionLike.connecting.piece &&
      typeof connectionLike.connecting.piece.id === "string"
        ? connectionLike.connecting.piece.id
        : undefined;
    if (connectedPieceId || connectingPieceId) {
      return `${connectedPieceId ?? "from"}-${connectingPieceId ?? "to"}`;
    }
  }
  return `connection-${fallbackIndex}`;
};

const resolveAlgorithmPieceDiffLabel = (design: Design | undefined, pieceLike: unknown, fallbackIndex: number): string => {
  const pieceId = resolveAlgorithmPieceDiffId(pieceLike, fallbackIndex);
  const pieceRows = __itemsOf((design as PlainJsonObject | undefined)?.["pieces"]) as Piece[];
  const sourcePiece = (pieceLike && typeof pieceLike === "object" ? (pieceLike as { name?: string }).name : undefined) || pieceRows.find((piece) => piece.id === pieceId)?.name;
  return sourcePiece && sourcePiece.length > 0 ? sourcePiece : pieceId;
};

const ALGORITHM_CONNECTION_UPDATE_DIFF_KEYS = ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"] as const;

/**
 * Renders TreeRows listing numeric connection diff fields for the algorithm details panel.
 **/
const buildAlgorithmConnectionUpdateDiffDetail = (item: unknown, entryId: string): React.ReactNode | undefined => {
  if (!item || typeof item !== "object" || !("diff" in item)) return undefined;
  const diff = (item as { diff?: unknown }).diff;
  if (!diff || typeof diff !== "object") return undefined;
  const d = diff as Record<string, unknown>;
  const rows: React.ReactNode[] = [];
  for (const key of ALGORITHM_CONNECTION_UPDATE_DIFF_KEYS) {
    const v = d[key];
    if (typeof v === "number" && Number.isFinite(v)) {
      rows.push(
        <TreeRow key={`${entryId}.conn.${key}`} id={`${entryId}.conn.${key}`} label={null}>
          <div className="flex w-full items-center justify-between gap-2 px-4 py-0.5 text-xs font-mono">
            <span className="text-muted-foreground">{key}</span>
            <span className="select-text text-foreground">{v}</span>
          </div>
        </TreeRow>,
      );
    }
  }
  return rows.length > 0 ? <>{rows}</> : undefined;
};

/**
 * Renders TreeRows for piece updated diff (center, plane origin, axes) when present.
 **/
const buildAlgorithmPieceUpdateDiffDetail = (item: unknown, entryId: string): React.ReactNode | undefined => {
  if (!item || typeof item !== "object" || !("diff" in item)) return undefined;
  const diff = (item as { diff?: unknown }).diff;
  if (!diff || typeof diff !== "object") return undefined;
  const d = diff as Record<string, unknown>;
  const rows: React.ReactNode[] = [];
  const pushNum = (key: string, v: unknown) => {
    if (typeof v === "number" && Number.isFinite(v)) {
      rows.push(
        <TreeRow key={`${entryId}.piece.${key}`} id={`${entryId}.piece.${key}`} label={null}>
          <div className="flex w-full items-center justify-between gap-2 px-4 py-0.5 text-xs font-mono">
            <span className="text-muted-foreground">{key}</span>
            <span className="select-text text-foreground">{v}</span>
          </div>
        </TreeRow>,
      );
    }
  };
  if (d.center && typeof d.center === "object") {
    const c = d.center as Record<string, unknown>;
    pushNum("center.u", c.u);
    pushNum("center.v", c.v);
  }
  if (d.plane && typeof d.plane === "object") {
    const pl = d.plane as Record<string, unknown>;
    if (pl.origin && typeof pl.origin === "object") {
      const o = pl.origin as Record<string, unknown>;
      pushNum("plane.origin.x", o.x);
      pushNum("plane.origin.y", o.y);
      pushNum("plane.origin.z", o.z);
    }
    if (pl.xAxis && typeof pl.xAxis === "object") {
      const ax = pl.xAxis as Record<string, unknown>;
      pushNum("plane.xAxis.x", ax.x);
      pushNum("plane.xAxis.y", ax.y);
      pushNum("plane.xAxis.z", ax.z);
    }
    if (pl.yAxis && typeof pl.yAxis === "object") {
      const ay = pl.yAxis as Record<string, unknown>;
      pushNum("plane.yAxis.x", ay.x);
      pushNum("plane.yAxis.y", ay.y);
      pushNum("plane.yAxis.z", ay.z);
    }
  }
  return rows.length > 0 ? <>{rows}</> : undefined;
};

const resolveAlgorithmConnectionDiffLabel = (design: Design | undefined, connectionLike: unknown, fallbackIndex: number): string => {
  const connectionId = resolveAlgorithmConnectionDiffId(connectionLike, fallbackIndex);
  const sourceConnection = design ? snapshotDesignConnections(design).find((connection) => {
    if (connection.id && connection.id === connectionId) return true;
    return false;
  }) : undefined;
  const connectedPieceId =
    (connectionLike &&
    typeof connectionLike === "object" &&
    "connected" in connectionLike &&
    connectionLike.connected &&
    typeof connectionLike.connected === "object" &&
    "piece" in connectionLike.connected &&
    connectionLike.connected.piece &&
    typeof connectionLike.connected.piece === "object" &&
    "id" in connectionLike.connected.piece &&
    typeof connectionLike.connected.piece.id === "string"
      ? connectionLike.connected.piece.id
      : undefined) ?? sourceConnection?.connected?.piece?.id;
  const connectingPieceId =
    (connectionLike &&
    typeof connectionLike === "object" &&
    "connecting" in connectionLike &&
    connectionLike.connecting &&
    typeof connectionLike.connecting === "object" &&
    "piece" in connectionLike.connecting &&
    connectionLike.connecting.piece &&
    typeof connectionLike.connecting.piece === "object" &&
    "id" in connectionLike.connecting.piece &&
    typeof connectionLike.connecting.piece.id === "string"
      ? connectionLike.connecting.piece.id
      : undefined) ?? sourceConnection?.connecting?.piece?.id;

  const pieceRows = __itemsOf((design as PlainJsonObject | undefined)?.["pieces"]) as Piece[];
  const connectedPieceName = connectedPieceId ? (pieceRows.find((piece) => piece.id === connectedPieceId)?.name ?? connectedPieceId) : undefined;
  const connectingPieceName = connectingPieceId ? (pieceRows.find((piece) => piece.id === connectingPieceId)?.name ?? connectingPieceId) : undefined;
  if (connectedPieceName || connectingPieceName) {
    return `${connectedPieceName ?? "Unknown"} -> ${connectingPieceName ?? "Unknown"}`;
  }
  return connectionId;
};

const buildAlgorithmDiffEntries = (design: Design | undefined, groupKind: AlgorithmDiffEntryGroupKind, changeKind: AlgorithmDiffEntryChangeKind, items: unknown[] | undefined, uncheckedEntryIds: Set<string>): AlgorithmDiffTreeGroup | undefined => {
  const safeItems = items ?? [];
  if (safeItems.length === 0) return undefined;
  const entries = safeItems.map((item, index) => {
    const baseId =
      groupKind === "pieces"
        ? resolveAlgorithmPieceDiffId(changeKind === "updated" && item && typeof item === "object" && "piece" in item ? (item as { piece?: unknown }).piece : item, index)
        : resolveAlgorithmConnectionDiffId(changeKind === "updated" && item && typeof item === "object" && "connection" in item ? (item as { connection?: unknown }).connection : item, index);
    const label =
      groupKind === "pieces"
        ? resolveAlgorithmPieceDiffLabel(design, changeKind === "updated" && item && typeof item === "object" && "piece" in item ? (item as { piece?: unknown }).piece : item, index)
        : resolveAlgorithmConnectionDiffLabel(design, changeKind === "updated" && item && typeof item === "object" && "connection" in item ? (item as { connection?: unknown }).connection : item, index);
    const id = buildAlgorithmDiffEntryId(groupKind, changeKind, baseId);
    const detail = changeKind === "updated" && groupKind === "connections" ? buildAlgorithmConnectionUpdateDiffDetail(item, id) : changeKind === "updated" && groupKind === "pieces" ? buildAlgorithmPieceUpdateDiffDetail(item, id) : undefined;
    return { id, label, checked: !uncheckedEntryIds.has(id), detail };
  });
  const checkedCount = entries.filter((entry) => entry.checked).length;
  return {
    id: `${groupKind}.${changeKind}`,
    label: changeKind,
    groupKind,
    changeKind,
    entries,
    totalCount: entries.length,
    checkedCount,
  };
};

const buildAlgorithmDiffTreeCategories = (design: Design | undefined, designDiff: DesignDiff | undefined, uncheckedEntryIds: Set<string>): AlgorithmDiffTreeCategory[] => {
  if (!designDiff) return [];
  const categories: AlgorithmDiffTreeCategory[] = [];

  const pieceGroups = [
    buildAlgorithmDiffEntries(design, "pieces", "added", designDiff.pieces?.added as unknown[] | undefined, uncheckedEntryIds),
    buildAlgorithmDiffEntries(design, "pieces", "removed", designDiff.pieces?.removed as unknown[] | undefined, uncheckedEntryIds),
    buildAlgorithmDiffEntries(design, "pieces", "updated", designDiff.pieces?.updated as unknown[] | undefined, uncheckedEntryIds),
  ].filter((group): group is AlgorithmDiffTreeGroup => Boolean(group));
  if (pieceGroups.length > 0) {
    categories.push({
      id: "pieces",
      label: "pieces",
      groups: pieceGroups,
      totalCount: pieceGroups.reduce((sum, group) => sum + group.totalCount, 0),
      checkedCount: pieceGroups.reduce((sum, group) => sum + group.checkedCount, 0),
    });
  }

  const connectionGroups = [
    buildAlgorithmDiffEntries(design, "connections", "added", designDiff.connections?.added as unknown[] | undefined, uncheckedEntryIds),
    buildAlgorithmDiffEntries(design, "connections", "removed", designDiff.connections?.removed as unknown[] | undefined, uncheckedEntryIds),
    buildAlgorithmDiffEntries(design, "connections", "updated", designDiff.connections?.updated as unknown[] | undefined, uncheckedEntryIds),
  ].filter((group): group is AlgorithmDiffTreeGroup => Boolean(group));
  if (connectionGroups.length > 0) {
    categories.push({
      id: "connections",
      label: "connections",
      groups: connectionGroups,
      totalCount: connectionGroups.reduce((sum, group) => sum + group.totalCount, 0),
      checkedCount: connectionGroups.reduce((sum, group) => sum + group.checkedCount, 0),
    });
  }

  return categories;
};

const getAlgorithmDiffEntryIds = (categories: AlgorithmDiffTreeCategory[]): string[] => categories.flatMap((category) => category.groups.flatMap((group) => group.entries.map((entry) => entry.id)));

const filterAlgorithmDesignDiffByUncheckedEntryIds = (designDiff: DesignDiff | undefined, uncheckedEntryIds: Set<string>): DesignDiff | undefined => {
  if (!designDiff) return undefined;
  if (uncheckedEntryIds.size === 0) return designDiff;

  return {
    ...designDiff,
    pieces: designDiff.pieces
      ? {
          ...designDiff.pieces,
          added: (designDiff.pieces.added ?? []).filter((piece, index) => !uncheckedEntryIds.has(buildAlgorithmDiffEntryId("pieces", "added", resolveAlgorithmPieceDiffId(piece, index)))),
          removed: (designDiff.pieces.removed ?? []).filter((piece, index) => !uncheckedEntryIds.has(buildAlgorithmDiffEntryId("pieces", "removed", resolveAlgorithmPieceDiffId(piece, index)))),
          updated: (designDiff.pieces.updated ?? []).filter((pieceUpdate, index) => !uncheckedEntryIds.has(buildAlgorithmDiffEntryId("pieces", "updated", resolveAlgorithmPieceDiffId((pieceUpdate as { piece?: unknown }).piece, index)))),
        }
      : undefined,
    connections: designDiff.connections
      ? {
          ...designDiff.connections,
          added: (designDiff.connections.added ?? []).filter((connection, index) => !uncheckedEntryIds.has(buildAlgorithmDiffEntryId("connections", "added", resolveAlgorithmConnectionDiffId(connection, index)))),
          removed: (designDiff.connections.removed ?? []).filter((connection, index) => !uncheckedEntryIds.has(buildAlgorithmDiffEntryId("connections", "removed", resolveAlgorithmConnectionDiffId(connection, index)))),
          updated: (designDiff.connections.updated ?? []).filter(
            (connectionUpdate, index) => !uncheckedEntryIds.has(buildAlgorithmDiffEntryId("connections", "updated", resolveAlgorithmConnectionDiffId((connectionUpdate as { connection?: unknown }).connection, index))),
          ),
        }
      : undefined,
  };
};

/**
 * Hook to access algorithm context from inside algorithm windows.
 **/
export function useAlgorithm(): AlgorithmRuntimeContextValue {
  const ctx = React.useContext(AlgorithmContext);
  if (!ctx) throw new Error("useAlgorithm must be used within an AlgorithmApp");
  return ctx;
}

/**
 * Window definition for an algorithm app window.
 **/
export interface AlgorithmWindowDef {
  id: string;
  kind: WindowKind;
  label?: string;
  component?: React.FC;
}

type AlgorithmWindowKind = WindowKind.VEC_INPUT | WindowKind.VECTOR_INPUT | WindowKind.PIECES_SELECTION_INPUT | WindowKind.SELECTION_INPUT | WindowKind.DESIGN_INPUT | WindowKind.DESIGN_DIFF_OUTPUT | WindowKind.DESIGN_OUTPUT | WindowKind.SCENE;

type AlgorithmUiComponentId = "semio/ui:Vec" | "semio/ui:MoveVectorInput" | "semio/ui:PieceSelection" | "semio/ui:DiagramSelection" | "semio/ui:Diagram" | "semio/ui:Scene";

interface AlgorithmWindowBehavior {
  kind: AlgorithmWindowKind;
  uiComponentId: AlgorithmUiComponentId;
  selectionEnabled: boolean;
  diffEnabled: boolean;
  usesPieceSelection: boolean;
  component: React.ComponentType<any>;
  createProps: (context: AlgorithmRuntimeContextValue) => Record<string, any>;
  render: (component: React.ReactElement, context: AlgorithmContextValue) => React.ReactElement;
}

const ALGORITHM_DIAGRAM_VIEWPORT_PRIORITY: Record<AlgorithmDiagramWindowKind, number> = {
  [WindowKind.PIECES_SELECTION_INPUT]: 4,
  [WindowKind.SELECTION_INPUT]: 4,
  [WindowKind.DESIGN_INPUT]: 4,
  [WindowKind.DESIGN_DIFF_OUTPUT]: 2,
  [WindowKind.DESIGN_OUTPUT]: 1,
};

const mergeAlgorithmDiagramViewportState = (current: AlgorithmDiagramViewportState, kind: AlgorithmDiagramWindowKind, patch: Partial<Pick<AlgorithmDiagramViewportState, "pan" | "zoom">>): AlgorithmDiagramViewportState => {
  const isInitialized = current.pan !== undefined && current.zoom !== undefined;
  if (isInitialized) {
    return { ...current, ...patch };
  }

  const currentPriority = current.sourceKind ? ALGORITHM_DIAGRAM_VIEWPORT_PRIORITY[current.sourceKind] : -1;
  const nextPriority = ALGORITHM_DIAGRAM_VIEWPORT_PRIORITY[kind];
  if (current.sourceKind && nextPriority < currentPriority) {
    return current;
  }

  return { ...current, ...patch, sourceKind: kind };
};

const createAlgorithmDiagramViewportProps = (kind: AlgorithmDiagramWindowKind, context: AlgorithmRuntimeContextValue) => ({
  pan: context.diagramViewport.pan,
  zoom: context.diagramViewport.zoom,
  onPanChange: (pan: DiagramPan) => context.onDiagramViewportPanChange(kind, pan),
  onZoomChange: (zoom: number) => context.onDiagramViewportZoomChange(kind, zoom),
});

const renderAlgorithmFullWindow = (component: React.ReactElement): React.ReactElement => <div className="h-full w-full">{component}</div>;

const renderAlgorithmVecWindow = (component: React.ReactElement, context: AlgorithmContextValue): React.ReactElement => {
  const { vec, onVecChange } = context;
  if (!vec || !onVecChange) return <></>;

  return (
    <div className="h-full flex flex-col items-center justify-center gap-2 p-2">
      {component}
      <div className="flex gap-2">
        <div className="flex items-center gap-1">
          <span className="text-xs font-mono text-muted-foreground">u</span>
          <input className="w-20 rounded-md border border-element bg-background px-2 py-1 text-sm font-mono" type="number" step="0.1" value={vec.u} onChange={(e) => onVecChange({ ...vec, u: Number(e.target.value) })} />
        </div>
        <div className="flex items-center gap-1">
          <span className="text-xs font-mono text-muted-foreground">v</span>
          <input className="w-20 rounded-md border border-element bg-background px-2 py-1 text-sm font-mono" type="number" step="0.1" value={vec.v} onChange={(e) => onVecChange({ ...vec, v: Number(e.target.value) })} />
        </div>
      </div>
    </div>
  );
};

const getAlgorithmStatusTone = (message: string): "muted" | "destructive" => {
  const m = message.trim().toLowerCase();
  if (m.startsWith("loading")) return "muted";
  if (m.startsWith("select ")) return "muted";
  return "destructive";
};

const renderAlgorithmStatusWindow = (component: React.ReactElement, context: AlgorithmContextValue): React.ReactElement => {
  if (context.error) {
    const tone = getAlgorithmStatusTone(context.error);
    return <div className={cn("h-full flex items-center justify-center p-2 text-sm font-mono", tone === "destructive" ? "text-destructive" : "text-muted-foreground")}>{context.error}</div>;
  }
  return renderAlgorithmFullWindow(component);
};

const ALGORITHM_WINDOW_BEHAVIORS: Record<AlgorithmWindowKind, Omit<AlgorithmWindowBehavior, "kind">> = {
  [WindowKind.VEC_INPUT]: {
    uiComponentId: "semio/ui:Vec",
    selectionEnabled: false,
    diffEnabled: false,
    usesPieceSelection: false,
    component: Vec,
    createProps: (context) => ({
      id: "algorithm-vec-input",
      vec: context.vec ?? { u: 0, v: 0 },
      onVecChange: context.onVecChange,
      minU: context.vecMin?.u ?? -10,
      maxU: context.vecMax?.u ?? 10,
      minV: context.vecMin?.v ?? -10,
      maxV: context.vecMax?.v ?? 10,
      size: 160,
    }),
    render: renderAlgorithmVecWindow,
  },
  [WindowKind.VECTOR_INPUT]: {
    uiComponentId: "semio/ui:MoveVectorInput",
    selectionEnabled: false,
    diffEnabled: false,
    usesPieceSelection: false,
    component: MoveVectorInput,
    createProps: (context) => ({
      id: "algorithm-vector-input",
      value: context.moveVector ?? { gap: 0, shift: 0, rise: 0 },
      onChange: context.onMoveVectorChange,
      min: context.moveVectorMin ?? { gap: -10, shift: -10, rise: -10 },
      max: context.moveVectorMax ?? { gap: 10, shift: 10, rise: 10 },
    }),
    render: renderAlgorithmFullWindow,
  },
  [WindowKind.PIECES_SELECTION_INPUT]: {
    uiComponentId: "semio/ui:PieceSelection",
    selectionEnabled: true,
    diffEnabled: false,
    usesPieceSelection: true,
    component: PieceSelection,
    createProps: (context) => ({
      design: context.design,
      layoutDiff: context.diagramLayoutDiff,
      selection: { pieceIds: context.selectedPieceIds },
      onSelectionChange: (next: PieceSelectionState) => context.onSelectedPieceIdsChange?.(next.pieceIds ?? []),
      selectionEnabled: true,
      diffEnabled: false,
      zoomTarget: "design" as ZoomTarget,
      panEnabled: true,
      zoomEnabled: true,
      ...createAlgorithmDiagramViewportProps(WindowKind.PIECES_SELECTION_INPUT, context),
    }),
    render: renderAlgorithmFullWindow,
  },
  [WindowKind.SELECTION_INPUT]: {
    uiComponentId: "semio/ui:DiagramSelection",
    selectionEnabled: true,
    diffEnabled: false,
    usesPieceSelection: true,
    component: DiagramSelection,
    createProps: (context) => ({
      design: context.design,
      layoutDiff: context.diagramLayoutDiff,
      selection: { pieceIds: context.selectedPieceIds, connectionIds: context.selectedConnectionIds ?? [] },
      onSelectionChange: (next: DiagramSelectionState) => {
        context.onSelectedPieceIdsChange?.(next.pieceIds ?? []);
        context.onSelectedConnectionIdsChange?.(next.connectionIds ?? []);
      },
      selectionEnabled: true,
      diffEnabled: false,
      zoomTarget: "design" as ZoomTarget,
      panEnabled: true,
      zoomEnabled: true,
      ...createAlgorithmDiagramViewportProps(WindowKind.SELECTION_INPUT, context),
    }),
    render: renderAlgorithmFullWindow,
  },
  [WindowKind.DESIGN_INPUT]: {
    uiComponentId: "semio/ui:Diagram",
    selectionEnabled: false,
    diffEnabled: false,
    usesPieceSelection: false,
    component: SemioDiagram,
    createProps: (context) => ({
      design: context.design,
      layoutDiff: context.diagramLayoutDiff,
      diffEnabled: false,
      zoomTarget: "design" as ZoomTarget,
      selectionEnabled: false,
      ...createAlgorithmDiagramViewportProps(WindowKind.DESIGN_INPUT, context),
    }),
    render: renderAlgorithmFullWindow,
  },
  [WindowKind.DESIGN_DIFF_OUTPUT]: {
    uiComponentId: "semio/ui:Diagram",
    selectionEnabled: false,
    diffEnabled: true,
    usesPieceSelection: false,
    component: SemioDiagram,
    createProps: (context) => ({
      design: context.diffDesign ?? context.design,
      layoutDiff: context.diffDiagramLayoutDiff ?? context.diagramLayoutDiff,
      designDiff: context.filteredDesignDiff,
      diffEnabled: true,
      zoomTarget: "design" as ZoomTarget,
      selectionEnabled: false,
      ...createAlgorithmDiagramViewportProps(WindowKind.DESIGN_DIFF_OUTPUT, context),
    }),
    render: renderAlgorithmStatusWindow,
  },
  [WindowKind.DESIGN_OUTPUT]: {
    uiComponentId: "semio/ui:Diagram",
    selectionEnabled: false,
    diffEnabled: false,
    usesPieceSelection: false,
    component: SemioDiagram,
    createProps: (context) => ({
      design: context.outputDesign,
      layoutDiff: context.outputDiagramLayoutDiff ?? context.diagramLayoutDiff,
      diffEnabled: false,
      zoomTarget: "design" as ZoomTarget,
      selectionEnabled: false,
      ...createAlgorithmDiagramViewportProps(WindowKind.DESIGN_OUTPUT, context),
    }),
    render: renderAlgorithmStatusWindow,
  },
  [WindowKind.SCENE]: {
    uiComponentId: "semio/ui:Scene",
    selectionEnabled: false,
    diffEnabled: false,
    usesPieceSelection: false,
    component: SemioScene,
    createProps: (context) => ({
      design: context.outputDesign,
      kit: context.kit,
      diffEnabled: false,
      zoomTarget: "design" as ZoomTarget,
      selectionEnabled: false,
      pieceSelectionEnabled: false,
      connectionSelectionEnabled: false,
      selection: { pieceIds: context.selectedPieceIds },
    }),
    render: renderAlgorithmStatusWindow,
  },
};

const createAlgorithmWindowRenderer = (kind: AlgorithmWindowKind): React.FC => {
  const AlgorithmWindowRenderer: React.FC = () => {
    const context = useAlgorithm();
    const behavior = getAlgorithmWindowBehavior(kind);
    if (!behavior) return <div className="p-2 text-sm text-muted-foreground">Unknown window kind: {kind}</div>;
    const WindowComponent = behavior.component;
    return behavior.render(<WindowComponent {...behavior.createProps(context)} />, context);
  };

  AlgorithmWindowRenderer.displayName = `AlgorithmWindowRenderer(${kind})`;
  return AlgorithmWindowRenderer;
};

export function isAlgorithmWindowKind(kind: WindowKind): kind is AlgorithmWindowKind {
  return Object.prototype.hasOwnProperty.call(ALGORITHM_WINDOW_BEHAVIORS, kind);
}

export function getAlgorithmWindowBehavior(kind: WindowKind): AlgorithmWindowBehavior | undefined {
  if (!isAlgorithmWindowKind(kind)) return undefined;
  return { kind, ...ALGORITHM_WINDOW_BEHAVIORS[kind] };
}

export function createAlgorithmWindowKinds(windows: AlgorithmWindowDef[]): UIWindowKindDefinition[] {
  return windows.map((windowDef) => {
    if (windowDef.component) {
      return {
        id: windowDef.id,
        label: windowDef.label ?? windowDef.id,
        component: windowDef.component,
      };
    }
    const behavior = getAlgorithmWindowBehavior(windowDef.kind);
    return {
      id: windowDef.id,
      label: windowDef.label ?? windowDef.id,
      component: behavior ? createAlgorithmWindowRenderer(windowDef.kind as AlgorithmWindowKind) : () => <div className="p-2 text-sm text-muted-foreground">Unknown window kind: {windowDef.kind}</div>,
    };
  });
}

/**
 * 🆕Builds the standard IPO canvas layout: one column for all input window kinds (tab stack),
 * one for diff output, one for final design output. Matches the semio UI shell used in elements/UI stories.
 */
export function createIpoAlgorithmLayout(windows: AlgorithmWindowDef[]): UIWindowLayout {
  const inputKinds = new Set<WindowKind>([WindowKind.VEC_INPUT, WindowKind.VECTOR_INPUT, WindowKind.PIECES_SELECTION_INPUT, WindowKind.SELECTION_INPUT, WindowKind.DESIGN_INPUT]);
  const inputWindows = windows.filter((w) => inputKinds.has(w.kind));
  const diffWindow = windows.find((w) => w.kind === WindowKind.DESIGN_DIFF_OUTPUT);
  const outputWindow = windows.find((w) => w.kind === WindowKind.DESIGN_OUTPUT || w.kind === WindowKind.SCENE);

  const columns: Array<UIWindowLayoutAxisNode | UIWindowLayoutStackNode> = [];
  if (inputWindows.length > 1) {
    // 📝Multiple input windows: arrange as a column with VEC_INPUT at 20% and others at 80%.
    const vecWindows = inputWindows.filter((w) => w.kind === WindowKind.VEC_INPUT || w.kind === WindowKind.VECTOR_INPUT);
    const otherInputWindows = inputWindows.filter((w) => w.kind !== WindowKind.VEC_INPUT && w.kind !== WindowKind.VECTOR_INPUT);
    const inputRows: UIWindowLayoutStackNode[] = [];
    if (vecWindows.length > 0) {
      inputRows.push({
        kind: "stack",
        size: 20,
        children: vecWindows.map((w) => createWindowLayout(w.id, w.label ?? w.id)),
      });
    }
    if (otherInputWindows.length > 0) {
      inputRows.push({
        kind: "stack",
        size: 80,
        children: otherInputWindows.map((w) => createWindowLayout(w.id, w.label ?? w.id)),
      });
    }
    columns.push({
      kind: "column",
      children: inputRows,
    });
  } else if (inputWindows.length === 1) {
    columns.push({
      kind: "stack",
      children: inputWindows.map((w) => createWindowLayout(w.id, w.label ?? w.id)),
    });
  }
  if (diffWindow) {
    columns.push({
      kind: "stack",
      children: [createWindowLayout(diffWindow.id, diffWindow.label ?? diffWindow.id)],
    });
  }
  if (outputWindow) {
    columns.push({
      kind: "stack",
      children: [createWindowLayout(outputWindow.id, outputWindow.label ?? outputWindow.id)],
    });
  }

  const count = columns.length;
  if (count === 0) {
    return createDefaultLayout(
      windows.map((w) => w.id),
      "row",
      undefined,
      windows.map((w) => w.label ?? w.id),
    );
  }

  const size = Math.round((100 / count) * 100) / 100;
  columns.forEach((c) => {
    c.size = size;
  });

  return { root: { kind: "row", children: columns } };
}

export function createAlgorithmLayout(windows: AlgorithmWindowDef[], defaultLayout?: AlgorithmAppProps["defaultLayout"]) {
  return defaultLayout ?? createIpoAlgorithmLayout(windows);
}

// #region ⏲️AlgorithmDetailsPanel

/**
 * Details panel for algorithms showing context, selected pieces, vec, and error state.
 **/
const AlgorithmDetailsPanel: React.FC = () => {
  const ctx = React.useContext(AlgorithmContext);
  if (!ctx) return null;

  const design = ctx.design;
  const designRow = design as PlainJsonObject | undefined;
  const allPieces = __itemsOf(designRow?.["pieces"]) as Piece[];
  const selectedPieces = allPieces.filter((p) => p.id && ctx.selectedPieceIds.includes(String(p.id)));
  const visibleDiffCategories = ctx.diffTreeCategories;
  const visibleDiffCount = visibleDiffCategories.reduce((sum, category) => sum + category.checkedCount, 0);
  const totalDiffCount = visibleDiffCategories.reduce((sum, category) => sum + category.totalCount, 0);

  return (
    <div className="flex flex-col h-full overflow-y-auto">
      {/* Design section */}
      <TreeSection id="algorithm.details.design" label="Design" icon={<DetailsIcon size={14} />} defaultOpen={true}>
        <TreeRow id="algorithm.details.design.name" label={null}>
          <div className="flex items-center justify-between w-full px-2 py-0.5">
            <span className="text-xs text-muted-foreground">name</span>
            <span className="text-xs font-mono truncate max-w-32">{(designRow?.name as string | undefined) ?? "—"}</span>
          </div>
        </TreeRow>
        <TreeRow id="algorithm.details.design.pieces" label={null}>
          <div className="flex items-center justify-between w-full px-2 py-0.5">
            <span className="text-xs text-muted-foreground">pieces</span>
            <span className="text-xs font-mono">{allPieces.length}</span>
          </div>
        </TreeRow>
        <TreeRow id="algorithm.details.design.connections" label={null}>
          <div className="flex items-center justify-between w-full px-2 py-0.5">
            <span className="text-xs text-muted-foreground">connections</span>
            <span className="text-xs font-mono">{design ? snapshotDesignConnections(design).length : 0}</span>
          </div>
        </TreeRow>
      </TreeSection>

      {/* Vec section (only if vec is present) */}
      {ctx.vec && (
        <TreeSection id="algorithm.details.vec" label="Vec" icon={<DetailsIcon size={14} />} defaultOpen={true}>
          <TreeRow id="algorithm.details.vec.u" label={null}>
            <div className="flex items-center justify-between w-full px-2 py-0.5">
              <span className="text-xs text-muted-foreground">u</span>
              <span className="text-xs font-mono">{ctx.vec.u}</span>
            </div>
          </TreeRow>
          <TreeRow id="algorithm.details.vec.v" label={null}>
            <div className="flex items-center justify-between w-full px-2 py-0.5">
              <span className="text-xs text-muted-foreground">v</span>
              <span className="text-xs font-mono">{ctx.vec.v}</span>
            </div>
          </TreeRow>
        </TreeSection>
      )}

      {ctx.moveVector && (
        <TreeSection id="algorithm.details.moveVector" label="Vector" icon={<DetailsIcon size={14} />} defaultOpen={true}>
          <TreeRow id="algorithm.details.moveVector.gap" label={null}>
            <div className="flex items-center justify-between w-full px-2 py-0.5">
              <span className="text-xs text-muted-foreground">gap</span>
              <span className="text-xs font-mono">{ctx.moveVector.gap}</span>
            </div>
          </TreeRow>
          <TreeRow id="algorithm.details.moveVector.shift" label={null}>
            <div className="flex items-center justify-between w-full px-2 py-0.5">
              <span className="text-xs text-muted-foreground">shift</span>
              <span className="text-xs font-mono">{ctx.moveVector.shift}</span>
            </div>
          </TreeRow>
          <TreeRow id="algorithm.details.moveVector.rise" label={null}>
            <div className="flex items-center justify-between w-full px-2 py-0.5">
              <span className="text-xs text-muted-foreground">rise</span>
              <span className="text-xs font-mono">{ctx.moveVector.rise}</span>
            </div>
          </TreeRow>
        </TreeSection>
      )}

      {/* Selection section */}
      <TreeSection id="algorithm.details.selection" label={`Selection (${selectedPieces.length})`} icon={<PieceIcon size={14} />} defaultOpen={true}>
        {selectedPieces.length === 0 ? (
          <TreeRow id="algorithm.details.selection.empty" label={null}>
            <div className="px-2 py-1 text-xs text-muted-foreground italic">No pieces selected</div>
          </TreeRow>
        ) : (
          selectedPieces.map((piece: Piece) => (
            <TreeRow key={piece.id} id={`algorithm.details.selection.${piece.id}`} label={null}>
              <div className="flex items-center justify-between w-full px-2 py-0.5">
                <span className="text-xs truncate max-w-24">{piece.name ?? String(piece.id ?? "").slice(0, 8)}</span>
                <span className="text-xs text-muted-foreground font-mono">
                  {piece.type && typeof piece.type === "object" && "id" in piece.type ? String((piece.type as { id?: string }).id ?? "").slice(0, 8) : "—"}
                </span>
              </div>
            </TreeRow>
          ))
        )}
      </TreeSection>

      {/* Output section */}
      <TreeSection id="algorithm.details.output" label="Output" icon={<DetailsIcon size={14} />} defaultOpen={true}>
        <TreeRow id="algorithm.details.output.status" label={null}>
          <div className="flex items-center justify-between w-full px-2 py-0.5">
            <span className="text-xs text-muted-foreground">status</span>
            <span className={cn("text-xs font-mono", ctx.error ? "text-destructive" : "text-success")}>{ctx.error ? "error" : "ok"}</span>
          </div>
        </TreeRow>
        {ctx.error && (
          <TreeRow id="algorithm.details.output.error" label={null}>
            <div className="px-2 py-1 text-xs text-destructive wrap-break-word">{ctx.error}</div>
          </TreeRow>
        )}
        {ctx.designDiff && (
          <TreeItem id="algorithm.details.output.diff" label={`Diff (${visibleDiffCount}/${totalDiffCount})`} defaultOpen={true}>
            {visibleDiffCategories.map((category) => {
              const categoryEntryIds = category.groups.flatMap((group) => group.entries.map((entry) => entry.id));
              const categoryActions: TreeHeaderAction[] = [
                {
                  kind: "checkbox",
                  id: `algorithm.details.output.diff.${category.id}.checkbox`,
                  title: `Toggle ${category.label}`,
                  checked: category.totalCount > 0 && category.checkedCount === category.totalCount,
                  onCheckedChange: (checked) => ctx.setDiffEntriesChecked(categoryEntryIds, checked),
                },
              ];

              return (
                <TreeItem key={category.id} id={`algorithm.details.output.diff.${category.id}`} label={`${category.label} (${category.checkedCount}/${category.totalCount})`} actions={categoryActions} defaultOpen={true}>
                  {category.groups.map((group) => {
                    const groupActions: TreeHeaderAction[] = [
                      {
                        kind: "checkbox",
                        id: `algorithm.details.output.diff.${category.id}.${group.changeKind}.checkbox`,
                        title: `Toggle ${category.label} ${group.changeKind}`,
                        checked: group.totalCount > 0 && group.checkedCount === group.totalCount,
                        onCheckedChange: (checked) =>
                          ctx.setDiffEntriesChecked(
                            group.entries.map((entry) => entry.id),
                            checked,
                          ),
                      },
                    ];
                    const groupToneClassName = group.changeKind === "added" ? "text-success" : group.changeKind === "removed" ? "text-destructive" : "text-warning";

                    return (
                      <TreeItem
                        key={group.id}
                        id={`algorithm.details.output.diff.${category.id}.${group.changeKind}`}
                        label={
                          <div className="flex min-w-0 items-center justify-between gap-2">
                            <span className="truncate">{group.label}</span>
                            <span className={cn("text-xs font-mono", groupToneClassName)}>
                              {group.checkedCount}/{group.totalCount}
                            </span>
                          </div>
                        }
                        actions={groupActions}
                        defaultOpen={true}
                      >
                        {group.entries.map((entry) => {
                          const entryActions: TreeHeaderAction[] = [
                            {
                              kind: "checkbox",
                              id: `algorithm.details.output.diff.entry.${entry.id}`,
                              title: `Toggle ${entry.label}`,
                              checked: entry.checked,
                              onCheckedChange: (checked) => ctx.setDiffEntriesChecked([entry.id], checked),
                            },
                          ];
                          return (
                            <TreeItem key={entry.id} id={`algorithm.details.output.diff.entry.${entry.id}`} label={entry.label} actions={entryActions}>
                              {entry.detail}
                            </TreeItem>
                          );
                        })}
                      </TreeItem>
                    );
                  })}
                </TreeItem>
              );
            })}
          </TreeItem>
        )}
      </TreeSection>
    </div>
  );
};

// #endregion ⏲️AlgorithmDetailsPanel

/**
 * Props for <AlgorithmApp />.
 **/
export interface AlgorithmAppProps {
  id: string;
  label: string;
  windows: AlgorithmWindowDef[];
  defaultLayout?: any;
  context: AlgorithmContextValue;
  className?: string;
}

/**
 * AlgorithmApp renders a UI composite shell for an algorithm.
 * Each window is auto-wired to a standard component based on its WindowKind.
 * Provides a right panel with algorithm details and a footer with status.
 **/
export const AlgorithmApp: React.FC<AlgorithmAppProps> = ({ id, label, windows, defaultLayout, context, className }) => {
  const [diagramViewport, setDiagramViewport] = React.useState<AlgorithmDiagramViewportState>({});
  const [uncheckedDiffEntryIds, setUncheckedDiffEntryIds] = React.useState<string[]>([]);

  const handleDiagramViewportPanChange = React.useCallback((kind: AlgorithmDiagramWindowKind, pan: DiagramPan) => {
    setDiagramViewport((current) => mergeAlgorithmDiagramViewportState(current, kind, { pan }));
  }, []);

  const handleDiagramViewportZoomChange = React.useCallback((kind: AlgorithmDiagramWindowKind, zoom: number) => {
    setDiagramViewport((current) => mergeAlgorithmDiagramViewportState(current, kind, { zoom }));
  }, []);

  const uncheckedDiffEntryIdSet = React.useMemo(() => new Set(uncheckedDiffEntryIds), [uncheckedDiffEntryIds]);
  const diffTreeCategories = React.useMemo(() => buildAlgorithmDiffTreeCategories(context.design, context.designDiff, uncheckedDiffEntryIdSet), [context.design, context.designDiff, uncheckedDiffEntryIdSet]);
  const availableDiffEntryIds = React.useMemo(() => new Set(getAlgorithmDiffEntryIds(diffTreeCategories)), [diffTreeCategories]);

  React.useEffect(() => {
    setUncheckedDiffEntryIds((current) => {
      const next = current.filter((entryId) => availableDiffEntryIds.has(entryId));
      return next.length === current.length ? current : next;
    });
  }, [availableDiffEntryIds]);

  const setDiffEntriesChecked = React.useCallback((entryIds: string[], checked: boolean) => {
    setUncheckedDiffEntryIds((current) => {
      const next = new Set(current);
      entryIds.forEach((entryId) => {
        if (checked) next.delete(entryId);
        else next.add(entryId);
      });
      const nextIds = Array.from(next);
      return nextIds.length === current.length && nextIds.every((entryId, index) => entryId === current[index]) ? current : nextIds;
    });
  }, []);

  const filteredDesignDiff = React.useMemo(() => filterAlgorithmDesignDiffByUncheckedEntryIds(context.designDiff, uncheckedDiffEntryIdSet), [context.designDiff, uncheckedDiffEntryIdSet]);

  const runtimeContext = React.useMemo<AlgorithmRuntimeContextValue>(
    () => ({
      ...context,
      diagramViewport,
      onDiagramViewportPanChange: handleDiagramViewportPanChange,
      onDiagramViewportZoomChange: handleDiagramViewportZoomChange,
      filteredDesignDiff,
      diffTreeCategories,
      setDiffEntriesChecked,
    }),
    [context, diagramViewport, handleDiagramViewportPanChange, handleDiagramViewportZoomChange, filteredDesignDiff, diffTreeCategories, setDiffEntriesChecked],
  );

  const windowKinds: UIWindowKindDefinition[] = React.useMemo(() => createAlgorithmWindowKinds(windows), [windows]);

  const layout = React.useMemo(() => createAlgorithmLayout(windows, defaultLayout), [defaultLayout, windows]);

  const rightPanelTabs: SidePanelTabConfig[] = React.useMemo(
    () => [
      {
        id: `${id}.details`,
        icon: DetailsIcon,
        order: 0,
        content: () => <AlgorithmDetailsPanel />,
      },
    ],
    [id],
  );

  const pieceCount = __itemsOf((context.design as PlainJsonObject | undefined)?.["pieces"]).length;

  const footerItems: FooterItem[] = React.useMemo(
    () => [
      {
        id: `${id}.footer.pieces`,
        icon: <PieceIcon size={12} />,
        text: `${context.selectedPieceIds.length}/${pieceCount}`,
        order: 0,
      },
      ...(context.error
        ? [
            {
              id: `${id}.footer.error`,
              icon: <AlertCircleIcon size={12} />,
              text: "Error",
              order: 1,
              className: "text-destructive",
            },
          ]
        : []),
    ],
    [id, context.selectedPieceIds.length, pieceCount, context.error],
  );

  const apps: UIAppConfig[] = React.useMemo(
    () => [
      {
        id,
        label,
        windowKinds,
        defaultLayout: layout,
        rightPanelTabs,
        footerItems,
      },
    ],
    [id, label, windowKinds, layout, rightPanelTabs, footerItems],
  );

  return (
    <AlgorithmContext.Provider value={runtimeContext}>
      <div className={className ?? "h-full w-full"}>
        <UI apps={apps} defaultAppId={id} />
      </div>
    </AlgorithmContext.Provider>
  );
};

// #endregion 📸AlgorithmApp

const algorithmVitest = (
  import.meta as ImportMeta & {
    vitest?: {
      describe: typeof import("vitest").describe;
      expect: typeof import("vitest").expect;
      it: typeof import("vitest").it;
    };
  }
).vitest;

if (algorithmVitest) {
  const { describe, expect, it } = algorithmVitest;

  describe("algorithm window helpers", () => {
    it("recognizes the canonical algorithm window kinds", () => {
      expect(isAlgorithmWindowKind(WindowKind.VEC_INPUT)).toBe(true);
      expect(isAlgorithmWindowKind(WindowKind.VECTOR_INPUT)).toBe(true);
      expect(isAlgorithmWindowKind(WindowKind.PIECES_SELECTION_INPUT)).toBe(true);
      expect(isAlgorithmWindowKind(WindowKind.SELECTION_INPUT)).toBe(true);
      expect(isAlgorithmWindowKind(WindowKind.DESIGN_INPUT)).toBe(true);
      expect(isAlgorithmWindowKind(WindowKind.DESIGN_DIFF_OUTPUT)).toBe(true);
      expect(isAlgorithmWindowKind(WindowKind.DESIGN_OUTPUT)).toBe(true);
      expect(isAlgorithmWindowKind(WindowKind.SCENE)).toBe(true);
      expect(isAlgorithmWindowKind(WindowKind.TABLE)).toBe(false);
    });

    it("encodes the intended diagram behavior for selection and diff windows", () => {
      expect(getAlgorithmWindowBehavior(WindowKind.PIECES_SELECTION_INPUT)).toMatchObject({
        kind: WindowKind.PIECES_SELECTION_INPUT,
        uiComponentId: "semio/ui:PieceSelection",
        selectionEnabled: true,
        diffEnabled: false,
        usesPieceSelection: true,
      });
      expect(getAlgorithmWindowBehavior(WindowKind.DESIGN_INPUT)).toMatchObject({
        kind: WindowKind.DESIGN_INPUT,
        uiComponentId: "semio/ui:Diagram",
        selectionEnabled: false,
        diffEnabled: false,
        usesPieceSelection: false,
      });
      expect(getAlgorithmWindowBehavior(WindowKind.DESIGN_DIFF_OUTPUT)).toMatchObject({
        kind: WindowKind.DESIGN_DIFF_OUTPUT,
        uiComponentId: "semio/ui:Diagram",
        selectionEnabled: false,
        diffEnabled: true,
        usesPieceSelection: false,
      });
      expect(getAlgorithmWindowBehavior(WindowKind.DESIGN_OUTPUT)).toMatchObject({
        kind: WindowKind.DESIGN_OUTPUT,
        uiComponentId: "semio/ui:Diagram",
        selectionEnabled: false,
        diffEnabled: false,
        usesPieceSelection: false,
      });
      expect(getAlgorithmWindowBehavior(WindowKind.SCENE)).toMatchObject({
        kind: WindowKind.SCENE,
        uiComponentId: "semio/ui:Scene",
        selectionEnabled: false,
        diffEnabled: false,
        usesPieceSelection: false,
      });
    });

    it("maps algorithm selection and output windows to shared semio/ui components", () => {
      expect(getAlgorithmWindowBehavior(WindowKind.VEC_INPUT)?.component).toBe(Vec);
      expect(getAlgorithmWindowBehavior(WindowKind.VECTOR_INPUT)?.component).toBe(MoveVectorInput);
      expect(getAlgorithmWindowBehavior(WindowKind.PIECES_SELECTION_INPUT)?.component).toBe(PieceSelection);
      expect(getAlgorithmWindowBehavior(WindowKind.SELECTION_INPUT)?.component).toBe(DiagramSelection);
      expect(getAlgorithmWindowBehavior(WindowKind.DESIGN_INPUT)?.component).toBe(SemioDiagram);
      expect(getAlgorithmWindowBehavior(WindowKind.DESIGN_DIFF_OUTPUT)?.component).toBe(SemioDiagram);
      expect(getAlgorithmWindowBehavior(WindowKind.DESIGN_OUTPUT)?.component).toBe(SemioDiagram);
      expect(getAlgorithmWindowBehavior(WindowKind.SCENE)?.component).toBe(SemioScene);
    });

    it("shares one synchronized viewport across all diagram-backed algorithm windows", () => {
      const pan = { x: 24, y: -18 };
      const zoom = 2.5;
      const runtimeContext: AlgorithmRuntimeContextValue = {
        kit: { id: "kit", name: "Kit", version: "1", designs: [], types: [] } as unknown as Kit,
        design: { id: "design", name: "", pieces: [], connections: [] } as unknown as Design,
        selectedPieceIds: [],
        selectedConnectionIds: [],
        outputDesign: { id: "output", name: "", pieces: [], connections: [] } as unknown as Design,
        diagramViewport: { pan, zoom, sourceKind: WindowKind.DESIGN_INPUT },
        onDiagramViewportPanChange: () => undefined,
        onDiagramViewportZoomChange: () => undefined,
        filteredDesignDiff: undefined,
        diffTreeCategories: [],
        setDiffEntriesChecked: () => undefined,
      };

      const pieceSelectionProps = getAlgorithmWindowBehavior(WindowKind.PIECES_SELECTION_INPUT)?.createProps(runtimeContext);
      const selectionProps = getAlgorithmWindowBehavior(WindowKind.SELECTION_INPUT)?.createProps(runtimeContext);
      const inputProps = getAlgorithmWindowBehavior(WindowKind.DESIGN_INPUT)?.createProps(runtimeContext);
      const diffProps = getAlgorithmWindowBehavior(WindowKind.DESIGN_DIFF_OUTPUT)?.createProps(runtimeContext);
      const outputProps = getAlgorithmWindowBehavior(WindowKind.DESIGN_OUTPUT)?.createProps(runtimeContext);

      expect(pieceSelectionProps).toMatchObject({ pan, zoom });
      expect(selectionProps).toMatchObject({ pan, zoom });
      expect(inputProps).toMatchObject({ pan, zoom });
      expect(diffProps).toMatchObject({ pan, zoom });
      expect(outputProps).toMatchObject({ pan, zoom });
      expect(pieceSelectionProps?.onPanChange).toBeTypeOf("function");
      expect(selectionProps?.onPanChange).toBeTypeOf("function");
      expect(inputProps?.onPanChange).toBeTypeOf("function");
      expect(diffProps?.onPanChange).toBeTypeOf("function");
      expect(outputProps?.onPanChange).toBeTypeOf("function");
      expect(pieceSelectionProps?.onZoomChange).toBeTypeOf("function");
      expect(selectionProps?.onZoomChange).toBeTypeOf("function");
      expect(inputProps?.onZoomChange).toBeTypeOf("function");
      expect(diffProps?.onZoomChange).toBeTypeOf("function");
      expect(outputProps?.onZoomChange).toBeTypeOf("function");
    });

    it("prefers the input viewport fit until the shared algorithm viewport is initialized", () => {
      const diffSeed = mergeAlgorithmDiagramViewportState({}, WindowKind.DESIGN_DIFF_OUTPUT, { zoom: 4 });
      const outputIgnored = mergeAlgorithmDiagramViewportState(diffSeed, WindowKind.DESIGN_OUTPUT, { zoom: 1.2 });
      const inputSeed = mergeAlgorithmDiagramViewportState(outputIgnored, WindowKind.DESIGN_INPUT, { pan: { x: 12, y: -6 } });
      const syncedUpdate = mergeAlgorithmDiagramViewportState({ pan: { x: 12, y: -6 }, zoom: 4, sourceKind: WindowKind.DESIGN_INPUT }, WindowKind.DESIGN_OUTPUT, { pan: { x: -2, y: 9 } });

      expect(diffSeed).toMatchObject({ zoom: 4, sourceKind: WindowKind.DESIGN_DIFF_OUTPUT });
      expect(outputIgnored).toEqual(diffSeed);
      expect(inputSeed).toMatchObject({ pan: { x: 12, y: -6 }, zoom: 4, sourceKind: WindowKind.DESIGN_INPUT });
      expect(syncedUpdate).toMatchObject({ pan: { x: -2, y: 9 }, zoom: 4, sourceKind: WindowKind.DESIGN_INPUT });
    });

    it("builds window definitions and the default algorithm layout from the declared windows", () => {
      const windows: AlgorithmWindowDef[] = [
        { id: "drag-vec", kind: WindowKind.VEC_INPUT, label: "Vec" },
        { id: "drag-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
        { id: "drag-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
        { id: "drag-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
      ];

      expect(createAlgorithmWindowKinds(windows).map((windowDef) => ({ id: windowDef.id, label: windowDef.label }))).toEqual([
        { id: "drag-vec", label: "Vec" },
        { id: "drag-input", label: "Input" },
        { id: "drag-diff", label: "Diff" },
        { id: "drag-output", label: "Output" },
      ]);
      expect(createAlgorithmWindowKinds(windows).every((windowDef) => typeof windowDef.component === "function")).toBe(true);
      expect(createAlgorithmLayout(windows)).toEqual({
        root: {
          kind: "row",
          children: [
            {
              kind: "column",
              size: 33.33,
              children: [
                {
                  kind: "stack",
                  size: 20,
                  children: [{ kind: "window", windowKindId: "drag-vec", title: "Vec" }],
                },
                {
                  kind: "stack",
                  size: 80,
                  children: [{ kind: "window", windowKindId: "drag-input", title: "Input" }],
                },
              ],
            },
            {
              kind: "stack",
              size: 33.33,
              children: [{ kind: "window", windowKindId: "drag-diff", title: "Diff" }],
            },
            {
              kind: "stack",
              size: 33.33,
              children: [{ kind: "window", windowKindId: "drag-output", title: "Output" }],
            },
          ],
        },
      });

      const twoPane: AlgorithmWindowDef[] = [
        { id: "flatten-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
        { id: "flatten-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
      ];
      expect(createIpoAlgorithmLayout(twoPane)).toEqual({
        root: {
          kind: "row",
          children: [
            {
              kind: "stack",
              size: 50,
              children: [{ kind: "window", windowKindId: "flatten-diff", title: "Diff" }],
            },
            {
              kind: "stack",
              size: 50,
              children: [{ kind: "window", windowKindId: "flatten-output", title: "Output" }],
            },
          ],
        },
      });
    });

    it("filters unchecked diff entries before wiring the diff output window", () => {
      const design = {
        id: "design-1",
        name: "Design",
        pieces: [{ id: "piece-a", name: "Piece A", type: { id: "type-a" }, center: { u: 0, v: 0 } } as unknown as Piece, { id: "piece-b", name: "Piece B", type: { id: "type-a" }, center: { u: 1, v: 0 } } as unknown as Piece],
        connections: [{ id: "connection-a", connected: { piece: { id: "piece-a" } }, connecting: { piece: { id: "piece-b" } } } as unknown as Connection],
      } as unknown as Design;
      const diff: DesignDiff = {
        pieces: {
          added: [{ id: "piece-c", name: "Piece C", type: { id: "type-a" }, center: { u: 2, v: 0 } } as unknown as Piece],
          removed: [{ id: "piece-b" }],
          updated: [{ piece: { id: "piece-a" }, diff: { name: "Renamed Piece A" } }],
        },
        connections: {
          added: [{ id: "connection-b", connected: { piece: { id: "piece-a" } }, connecting: { piece: { id: "piece-c" } } } as unknown as Connection],
          removed: [{ id: "connection-a" }],
          updated: [],
        },
      };

      const filtered = filterAlgorithmDesignDiffByUncheckedEntryIds(diff, new Set([buildAlgorithmDiffEntryId("pieces", "removed", "piece-b"), buildAlgorithmDiffEntryId("connections", "added", "connection-b")]));
      const categories = buildAlgorithmDiffTreeCategories(design, diff, new Set([buildAlgorithmDiffEntryId("pieces", "removed", "piece-b")]));

      expect(filtered?.pieces?.removed).toEqual([]);
      expect((filtered?.pieces?.added as { id?: string }[] | undefined)?.map((piece) => piece.id)).toEqual(["piece-c"]);
      expect(filtered?.connections?.added).toEqual([]);
      expect((filtered?.connections?.removed as { id?: string }[] | undefined)?.map((connection) => connection.id)).toEqual(["connection-a"]);
      expect(categories.map((category) => ({ id: category.id, checked: category.checkedCount, total: category.totalCount }))).toEqual([
        { id: "pieces", checked: 2, total: 3 },
        { id: "connections", checked: 2, total: 2 },
      ]);
      expect(categories[0]?.groups.find((group) => group.changeKind === "removed")?.entries[0]).toMatchObject({
        id: buildAlgorithmDiffEntryId("pieces", "removed", "piece-b"),
        label: "Piece B",
        checked: false,
      });
    });
  });
}
