// #region 🔖Header

// 💻 semio/ui/index.tsx

// Specs: Re-export generic ui primitives and provide semio-specific Diagram, Scene, and Design components. All components are iframe compatible.
// Summary: Shared semio ui exports plus Diagram (2D), Scene (3D), Vec (2D input), and Design (split view) components.
//
// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Shared export surface for semio ui components.

// #endregion 🔖Header

import { Breadcrumb, Button, Section } from "@elements/ui/elements";
import { Clone, Edges, GizmoHelper, GizmoViewport, Grid, OrbitControls, useGLTF } from "@react-three/drei";
import { Canvas as ThreeCanvas, useThree } from "@react-three/fiber";
import {
  applyDesignDiff,
  planeToMatrix,
  selectBestModel,
  toThreeRotation,
  type Camera,
  type Connection,
  type Design,
  type DesignDiff,
  type Kit,
  type Piece,
  type Plane,
  type File as SemioFile,
  type Type as SemioKind,
  type Vector as SemioVector,
} from "@semio/js";
import * as React from "react";
import * as THREE from "three";
import { clone as cloneSkeleton } from "three/examples/jsm/utils/SkeletonUtils.js";

// #region 🔖ControllableState
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

// #endregion 🔖ControllableState

// #region 🔖Exports

// Re-export the runtime-safe ui primitives from @elements/ui/elements.

export * from "@elements/ui/elements";

// #endregion 🔖Exports

// #region 🔖Kit
// Specs: Kit provides a kit-scoped artifact picker (designs, types, ports/connectors)
// with the standard Semio UI controllable-state pattern: partial/full controlled/uncontrolled for
// both available data and selection. It supports partial/full select via per-group enable flags.
// Summary: Kit hierarchy browser with controllable data + selection, metadata, and artifact open action.

export type KitGroupKind = "design" | "type" | "port";

export interface KitPort {
  guid: string;
  typeGuid: string;
  id?: string;
  port?: string;
  name?: string;
  description?: string;
  mandatory?: boolean;
}

export interface KitDesignData extends Pick<Design, "guid" | "name" | "variant" | "view" | "description" | "createdAt" | "updatedAt" | "unit" | "icon" | "image"> {
  parent?: { guid: string };
}

export interface KitKindData extends Pick<SemioKind, "guid" | "name" | "variant" | "description" | "createdAt" | "updatedAt" | "icon" | "image"> {
  parent?: { guid: string };
}

export interface KitData {
  guid?: string;
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
  ports?: KitPort[];
}

export interface KitSelection {
  designGuids?: string[];
  typeGuids?: string[];
  portGuids?: string[];
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

  dataEnabled?: boolean;
  designDataEnabled?: boolean;
  typeDataEnabled?: boolean;
  portDataEnabled?: boolean;

  onOpenArtifact?: (artifact: KitHierarchyNode) => void;

  title?: string;
  className?: string;
}

const normalizeKitSelection = (selection?: KitSelection): KitSelection => ({
  designGuids: selection?.designGuids ?? [],
  typeGuids: selection?.typeGuids ?? [],
  portGuids: selection?.portGuids ?? [],
});

const getReferenceGuid = (value: unknown): string | undefined => {
  if (typeof value === "string") return value;
  if (value && typeof value === "object" && "guid" in value && typeof value.guid === "string") return value.guid;
  return undefined;
};

const getReferenceLabel = (value: unknown): string | undefined => {
  if (typeof value === "string") return value;
  if (value && typeof value === "object") {
    if ("name" in value && typeof value.name === "string" && value.name.length > 0) return value.name;
    if ("id" in value && typeof value.id === "string" && value.id.length > 0) return value.id;
    if ("guid" in value && typeof value.guid === "string" && value.guid.length > 0) return value.guid;
  }
  return undefined;
};

const buildKitDataFromKit = (kit: Kit | undefined): KitData => {
  if (!kit) return {};
  const designs = (kit.designs ?? []).map((d) => ({
    guid: d.guid,
    name: d.name,
    variant: d.variant,
    view: d.view,
    description: d.description,
    createdAt: d.createdAt,
    updatedAt: d.updatedAt,
    unit: d.unit,
    icon: d.icon,
    image: d.image,
    parent: d.parent ? { guid: d.parent.guid } : undefined,
  }));
  const types = (kit.types ?? []).map((t) => ({
    guid: t.guid,
    name: t.name,
    variant: t.variant,
    description: t.description,
    createdAt: t.createdAt,
    updatedAt: t.updatedAt,
    icon: t.icon,
    image: t.image,
    parent: t.parent ? { guid: t.parent.guid } : undefined,
  }));
  const ports: KitPort[] = (kit.types ?? []).flatMap((t) =>
    (t.connectors ?? []).map((c) => ({
      guid: c.guid,
      typeGuid: t.guid,
      id: c.id ?? c.name,
      port: getReferenceGuid(c.port),
      name: c.name || getReferenceLabel(c.port) || "port",
      description: c.description,
      mandatory: c.mandatory,
    })),
  );
  return {
    guid: kit.guid,
    name: kit.name,
    description: kit.description,
    version: kit.version,
    createdAt: kit.createdAt,
    updatedAt: kit.updatedAt,
    homepage: kit.homepage,
    remote: kit.remote,
    preview: kit.preview,
    icon: kit.icon,
    image: kit.image,
    license: kit.license,
    designs,
    types,
    ports,
  };
};

type KitHierarchyNodeKind = "scope" | "kit" | "group" | "design" | "kind" | "port";

export interface KitHierarchyNode {
  key: string;
  kind: KitHierarchyNodeKind;
  label: string;
  parentKey?: string;
  guid?: string;
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

const getKitArtifactHref = (value: Partial<KitData & KitDesignData & KitKindData>): string | undefined => value.view ?? value.image ?? value.icon ?? value.preview ?? value.homepage ?? value.remote;

const buildKitHierarchy = (data: KitData, options: { designDataEnabled: boolean; typeDataEnabled: boolean; portDataEnabled: boolean }): KitHierarchy => {
  const rootKey = "scope:kit";
  const kitKey = "kit:root";
  const designGroupKey = "group:designs";
  const kindGroupKey = "group:types";
  const portGroupKey = "group:ports";
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
  const portCount = String(data.ports?.length ?? 0);

  registerNode({
    key: rootKey,
    kind: "scope",
    label: "Kit",
    summary: "Kit hierarchy root.",
    metadata: [
      { label: "Designs", value: designCount },
      { label: "Types", value: kindCount },
      { label: "Ports", value: portCount },
    ],
  });

  const kitMetadata: Array<{ label: string; value: string }> = [];
  addKitMetaEntry(kitMetadata, "Name", data.name);
  addKitMetaEntry(kitMetadata, "Guid", data.guid);
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
    guid: data.guid,
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

  const kindKeyByGuid = new Map<string, string>();
  (data.types ?? []).forEach((kind) => {
    const metadata: Array<{ label: string; value: string }> = [];
    addKitMetaEntry(metadata, "Kind", "Type");
    addKitMetaEntry(metadata, "Name", kind.name);
    addKitMetaEntry(metadata, "Guid", kind.guid);
    addKitMetaEntry(metadata, "Description", kind.description);
    addKitMetaEntry(metadata, "Variant", kind.variant);
    addKitMetaEntry(metadata, "Created", kind.createdAt);
    addKitMetaEntry(metadata, "Updated", kind.updatedAt);
    const key = `kind:${kind.guid}`;
    kindKeyByGuid.set(kind.guid, key);
    registerNode({
      key,
      kind: "kind",
      label: kind.name || kind.guid,
      parentKey: kind.parent?.guid ? `kind:${kind.parent.guid}` : kindGroupKey,
      guid: kind.guid,
      groupKind: "type",
      href: getKitArtifactHref(kind),
      summary: kind.description || "Type artifact.",
      metadata,
    });
  });

  (data.designs ?? []).forEach((design) => {
    const metadata: Array<{ label: string; value: string }> = [];
    addKitMetaEntry(metadata, "Kind", "Design");
    addKitMetaEntry(metadata, "Name", design.name);
    addKitMetaEntry(metadata, "Guid", design.guid);
    addKitMetaEntry(metadata, "Description", design.description);
    addKitMetaEntry(metadata, "Variant", design.variant);
    addKitMetaEntry(metadata, "Unit", design.unit);
    addKitMetaEntry(metadata, "View", design.view);
    addKitMetaEntry(metadata, "Created", design.createdAt);
    addKitMetaEntry(metadata, "Updated", design.updatedAt);
    registerNode({
      key: `design:${design.guid}`,
      kind: "design",
      label: design.name || design.guid,
      parentKey: design.parent?.guid ? `design:${design.parent.guid}` : designGroupKey,
      guid: design.guid,
      groupKind: "design",
      href: getKitArtifactHref(design),
      summary: design.description || "Design artifact.",
      metadata,
    });
  });

  let orphanPortCount = 0;
  (data.ports ?? []).forEach((port) => {
    const metadata: Array<{ label: string; value: string }> = [];
    addKitMetaEntry(metadata, "Kind", "Port");
    addKitMetaEntry(metadata, "Name", port.name);
    addKitMetaEntry(metadata, "Guid", port.guid);
    addKitMetaEntry(metadata, "Connector Id", port.id);
    addKitMetaEntry(metadata, "Port", port.port);
    addKitMetaEntry(metadata, "Description", port.description);
    addKitMetaEntry(metadata, "Mandatory", port.mandatory === undefined ? undefined : String(port.mandatory));
    const parentKey = kindKeyByGuid.get(port.typeGuid) ?? portGroupKey;
    if (parentKey === portGroupKey) orphanPortCount += 1;
    registerNode({
      key: `port:${port.guid}`,
      kind: "port",
      label: port.name || port.guid,
      parentKey,
      guid: port.guid,
      groupKind: "port",
      summary: port.description || "Port artifact.",
      metadata,
    });
  });

  if (orphanPortCount > 0 && options.portDataEnabled) {
    registerNode({
      key: portGroupKey,
      kind: "group",
      label: "Ports",
      parentKey: kitKey,
      groupKind: "port",
      summary: "Ports without a resolved type parent.",
      metadata: [{ label: "Count", value: String(orphanPortCount) }],
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
  if (node.kind === "design") return { designGuids: node.guid ? [node.guid] : [], typeGuids: [], portGuids: [] };
  if (node.kind === "kind") return { designGuids: [], typeGuids: node.guid ? [node.guid] : [], portGuids: [] };
  if (node.kind === "port") return { designGuids: [], typeGuids: [], portGuids: node.guid ? [node.guid] : [] };
  return { designGuids: [], typeGuids: [], portGuids: [] };
};

const getSelectedKitNodeKey = (hierarchy: KitHierarchy, selection: KitSelection): string | undefined => {
  const selectedPort = selection.portGuids?.[0];
  if (selectedPort && hierarchy.nodesByKey.has(`port:${selectedPort}`)) return `port:${selectedPort}`;
  const selectedKind = selection.typeGuids?.[0];
  if (selectedKind && hierarchy.nodesByKey.has(`kind:${selectedKind}`)) return `kind:${selectedKind}`;
  const selectedDesign = selection.designGuids?.[0];
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
  if (label === "Guid") return "ID";
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
    if (entry.label === "Guid") return false;
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
  dataEnabled,
  designDataEnabled = true,
  typeDataEnabled = true,
  portDataEnabled = true,
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
  const effectivePorts = portDataEnabled ? (effectiveData.ports ?? []) : [];

  const setNextSelection = React.useCallback(
    (next: { designGuids?: string[]; typeGuids?: string[]; portGuids?: string[] }) => {
      if (!effectiveSelectionEnabled) return;
      setResolvedSelection({
        designGuids: designSelectionEnabled ? (next.designGuids ?? []) : [],
        typeGuids: typeSelectionEnabled ? (next.typeGuids ?? []) : [],
        portGuids: portSelectionEnabled ? (next.portGuids ?? []) : [],
      });
    },
    [designSelectionEnabled, effectiveSelectionEnabled, portSelectionEnabled, setResolvedSelection, typeSelectionEnabled],
  );

  // If kit changes and data is uncontrolled, adopt derived data.
  React.useEffect(() => {
    if (!effectiveDataEnabled) return;
    if (data !== undefined && onDataChange !== undefined) return;
    setResolvedData(derivedData);
  }, [data, derivedData, effectiveDataEnabled, onDataChange, setResolvedData]);

  const headerStats = React.useMemo(() => {
    const parts: string[] = [];
    if (designDataEnabled) parts.push(`${effectiveDesigns.length} designs`);
    if (typeDataEnabled) parts.push(`${effectiveTypes.length} types`);
    if (portDataEnabled) parts.push(`${effectivePorts.length} ports`);
    return parts.join(" · ");
  }, [designDataEnabled, effectiveDesigns.length, portDataEnabled, effectivePorts.length, typeDataEnabled, effectiveTypes.length]);

  const hierarchy = React.useMemo(
    () =>
      buildKitHierarchy(
        {
          ...effectiveData,
          designs: effectiveDesigns,
          types: effectiveTypes,
          ports: effectivePorts,
        },
        {
          designDataEnabled,
          typeDataEnabled,
          portDataEnabled,
        },
      ),
    [designDataEnabled, effectiveData, effectiveDesigns, effectivePorts, effectiveTypes, portDataEnabled, typeDataEnabled],
  );

  const selectedNodeKey = React.useMemo(() => getSelectedKitNodeKey(hierarchy, resolvedSelection), [hierarchy, resolvedSelection]);
  const [focusedNodeKey, setFocusedNodeKey] = React.useState<string>(() => selectedNodeKey ?? getDefaultKitNodeKey(hierarchy));

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

  const focusNode = React.useCallback(
    (nodeKey: string) => {
      const node = hierarchy.nodesByKey.get(nodeKey);
      if (!node) return;
      setFocusedNodeKey(node.key);
      if (!effectiveSelectionEnabled) return;
      if (node.kind === "design" && !designSelectionEnabled) return;
      if (node.kind === "kind" && !typeSelectionEnabled) return;
      if (node.kind === "port" && !portSelectionEnabled) return;
      setNextSelection(getKitNodeSelection(node));
    },
    [designSelectionEnabled, effectiveSelectionEnabled, hierarchy.nodesByKey, portSelectionEnabled, setNextSelection, typeSelectionEnabled],
  );

  const breadcrumbItems = React.useMemo(() => {
    const rootOptions = (hierarchy.childKeysByParentKey.get("kit:root") ?? [])
      .map((key) => hierarchy.nodesByKey.get(key))
      .filter((node): node is KitHierarchyNode => Boolean(node))
      .map((node) => ({ label: node.label, href: node.key, id: node.guid }));

    return [
      {
        content: <span style={{ display: "inline-block", width: 1, overflow: "hidden" }}>&nbsp;</span>,
        options: rootOptions,
        onNavigate: focusNode,
      },
      ...path.map((node) => ({
        id: node.guid,
        content: (
          <button
            type="button"
            onClick={() => focusNode(node.key)}
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
        options: getKitChildNodes(hierarchy, node).map((child) => ({ label: child.label, href: child.key, id: child.guid })),
        onNavigate: focusNode,
      })),
    ];
  }, [focusNode, focusedNode.key, hierarchy, path]);

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

        <Breadcrumb items={renderedBreadcrumbItems} />

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

// #endregion 🔖Kit

// #region 🔖Diagram

const DEFAULT_DIAGRAM_PADDING = 12;
const DEFAULT_DIAGRAM_PIECE_RADIUS = 1.75;
const DEFAULT_DIAGRAM_STROKE_WIDTH = 1;
const DEFAULT_DIAGRAM_ZOOM = 1;
const MIN_DIAGRAM_ZOOM = 1;
const MAX_DIAGRAM_ZOOM = 12;
const DIAGRAM_ZOOM_STEP = 0.0015;
const MIN_DIAGRAM_SPAN = 1;

type DiagramEntityStatus = "default" | "removed" | "added" | "modified";

export interface DiagramSelection {
  pieceGuids?: string[];
  connectionGuids?: string[];
}

export interface DiagramHover {
  pieceGuid?: string | null;
  connectionGuid?: string | null;
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
  pieceRadius?: number;
  strokeWidth?: number;
  title?: string;
  onPieceClick?: (piece: Piece) => void;
  onConnectionClick?: (connection: Connection) => void;
}

interface DiagramPoint {
  guid: string;
  piece: Piece;
  u: number;
  v: number;
  status: DiagramEntityStatus;
}

interface DiagramLine {
  guid: string;
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
  if (status === "removed") return "var(--color-removed)";
  if (status === "added") return "var(--color-new)";
  if (status === "modified") return "var(--color-modified)";
  return "currentColor";
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

const buildDiagramSnapshot = (design: Design, padding: number, designDiff?: DesignDiff): DiagramSnapshot => {
  const baseDesign = design;
  const nextDesign = designDiff ? applyDesignDiff(baseDesign, designDiff) : baseDesign;
  const flatBaseDesign = baseDesign;
  const flatNextDesign = nextDesign;
  const removedPieceGuids = new Set((designDiff?.pieces?.removed ?? []).map((piece) => piece.guid));
  const addedPieceGuids = new Set((designDiff?.pieces?.added ?? []).map((piece) => piece.guid));
  const modifiedPieceGuids = new Set((designDiff?.pieces?.updated ?? []).map((piece) => piece.piece.guid));
  const removedConnectionGuids = new Set((designDiff?.connections?.removed ?? []).map((connection) => connection.guid));
  const addedConnectionGuids = new Set((designDiff?.connections?.added ?? []).map((connection) => connection.guid));
  const modifiedConnectionGuids = new Set((designDiff?.connections?.updated ?? []).map((connection) => connection.connection.guid));

  const pointMap = new Map<string, DiagramPoint>();
  const upsertPoint = (piece: Piece, status: DiagramEntityStatus) => {
    if (!piece.guid || !piece.center) return;
    pointMap.set(piece.guid, {
      guid: piece.guid,
      piece,
      u: piece.center.u,
      v: piece.center.v,
      status,
    });
  };

  (flatBaseDesign.pieces ?? []).forEach((piece) => {
    if (removedPieceGuids.has(piece.guid)) {
      upsertPoint(piece, "removed");
    } else if (!designDiff) {
      upsertPoint(piece, "default");
    }
  });
  (flatNextDesign.pieces ?? []).forEach((piece) => {
    if (addedPieceGuids.has(piece.guid)) {
      upsertPoint(piece, "added");
    } else if (modifiedPieceGuids.has(piece.guid)) {
      upsertPoint(piece, "modified");
    } else {
      upsertPoint(piece, "default");
    }
  });

  const pointsByGuid = new Map(Array.from(pointMap.values()).map((point) => [point.guid, point]));
  const lineMap = new Map<string, DiagramLine>();
  const upsertLine = (connection: Connection, status: DiagramEntityStatus) => {
    if (!connection.guid) return;
    const source = pointsByGuid.get(connection.connected.piece.guid);
    const target = pointsByGuid.get(connection.connecting.piece.guid);
    if (!source || !target) return;
    lineMap.set(connection.guid, {
      guid: connection.guid,
      connection,
      source,
      target,
      status,
    });
  };

  (flatBaseDesign.connections ?? []).forEach((connection) => {
    if (removedConnectionGuids.has(connection.guid)) {
      upsertLine(connection, "removed");
    } else if (!designDiff) {
      upsertLine(connection, "default");
    }
  });
  (flatNextDesign.connections ?? []).forEach((connection) => {
    if (addedConnectionGuids.has(connection.guid)) {
      upsertLine(connection, "added");
    } else if (modifiedConnectionGuids.has(connection.guid)) {
      upsertLine(connection, "modified");
    } else {
      upsertLine(connection, "default");
    }
  });

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

const isSelected = (guid: string, guidSet: Set<string>): boolean => guidSet.has(guid);

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
  pieceGuids: selection?.pieceGuids ?? [],
  connectionGuids: selection?.connectionGuids ?? [],
});

const normalizeHover = (hover?: DiagramHover): DiagramHover => ({
  pieceGuid: hover?.pieceGuid ?? null,
  connectionGuid: hover?.connectionGuid ?? null,
});

export const SemioDiagram: React.FC<SemioDiagramProps> = ({
  design,
  designDiff,
  defaultDesignDiff,
  diffEnabled,
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
  onPieceClick,
  onConnectionClick,
}) => {
  const effectiveDiffEnabled = diffEnabled ?? true;
  const effectiveSelectionEnabled = selectionEnabled ?? true;
  const effectivePieceSelectionEnabled = effectiveSelectionEnabled && pieceSelectionEnabled;
  const effectiveConnectionSelectionEnabled = effectiveSelectionEnabled && connectionSelectionEnabled;
  const effectiveHoverEnabled = hoverEnabled ?? true;
  const effectivePieceHoverEnabled = effectiveHoverEnabled && pieceHoverEnabled && (effectivePieceSelectionEnabled || !!onPieceClick);
  const effectiveConnectionHoverEnabled = effectiveHoverEnabled && connectionHoverEnabled && (effectiveConnectionSelectionEnabled || !!onConnectionClick);
  const resolvedDesignDiff = useResolvedValue(designDiff, defaultDesignDiff);
  const [resolvedSelection, setResolvedSelection] = useInteractiveControllableValue(selection, normalizeSelection(defaultSelection), onSelectionChange);
  const [resolvedHover, setResolvedHover] = useInteractiveControllableValue(hover, normalizeHover(defaultHover), onHoverChange);
  const [resolvedPan, setResolvedPan, isPanControlled] = useInteractiveControllableValue(pan, defaultPan ?? { x: 0, y: 0 }, onPanChange);
  const [resolvedZoom, setResolvedZoom, isZoomControlled] = useInteractiveControllableValue(zoom, defaultZoom ?? DEFAULT_DIAGRAM_ZOOM, onZoomChange);
  const snapshot = React.useMemo(() => buildDiagramSnapshot(design, padding, effectiveDiffEnabled ? resolvedDesignDiff : undefined), [design, effectiveDiffEnabled, padding, resolvedDesignDiff]);
  const selectedPieceGuids = React.useMemo(() => new Set(effectiveSelectionEnabled ? (resolvedSelection.pieceGuids ?? []) : []), [effectiveSelectionEnabled, resolvedSelection.pieceGuids]);
  const selectedConnectionGuids = React.useMemo(() => new Set(effectiveSelectionEnabled ? (resolvedSelection.connectionGuids ?? []) : []), [effectiveSelectionEnabled, resolvedSelection.connectionGuids]);
  const hoveredPieceGuid = effectivePieceHoverEnabled ? (resolvedHover.pieceGuid ?? null) : null;
  const hoveredConnectionGuid = effectiveConnectionHoverEnabled ? (resolvedHover.connectionGuid ?? null) : null;
  const { ref, size } = useElementSize<HTMLDivElement>();
  const panPointerIdRef = React.useRef<number | null>(null);
  const panOriginRef = React.useRef({ x: 0, y: 0, panX: 0, panY: 0 });
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
  const fittedViewport = React.useMemo(() => {
    const changedPoints = effectiveDiffEnabled ? snapshot.points.filter((point) => point.status !== "default") : [];
    const changedLinePoints = effectiveDiffEnabled ? snapshot.lines.filter((line) => line.status !== "default").flatMap((line) => [line.source, line.target]) : [];
    const targetBounds = buildDiagramBounds([...changedPoints, ...changedLinePoints]) ?? {
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
  }, [centerX, centerY, defaultPan, defaultZoom, drawableHeight, drawableWidth, effectiveDiffEnabled, offsetX, offsetY, scale, snapshot]);
  const applyViewportX = (x: number) => centerX + resolvedPan.x + resolvedZoom * (x - centerX);
  const applyViewportY = (y: number) => centerY + resolvedPan.y + resolvedZoom * (y - centerY);
  const toPixelX = (u: number) => applyViewportX(toBasePixelX(u));
  const toPixelY = (y: number) => applyViewportY(toBasePixelY(y));

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
    (event: React.WheelEvent<HTMLDivElement>) => {
      if (!zoomEnabled) return;
      event.preventDefault();
      if (size.width <= 0 || size.height <= 0) return;
      const nextZoom = Math.min(MAX_DIAGRAM_ZOOM, Math.max(MIN_DIAGRAM_ZOOM, resolvedZoom * Math.exp(-event.deltaY * DIAGRAM_ZOOM_STEP)));
      if (Math.abs(nextZoom - resolvedZoom) < 0.0001) return;
      const rect = event.currentTarget.getBoundingClientRect();
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
    [centerX, centerY, resolvedPan.x, resolvedPan.y, resolvedZoom, setResolvedPan, setResolvedZoom, size.height, size.width, zoomEnabled],
  );

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
      setIsPanning(true);
      event.currentTarget.setPointerCapture(event.pointerId);
    },
    [panEnabled, resolvedPan.x, resolvedPan.y],
  );

  const clearSelection = React.useCallback(() => {
    if (!effectiveSelectionEnabled) return;
    setResolvedSelection({
      pieceGuids: [],
      connectionGuids: [],
    });
  }, [effectiveSelectionEnabled, setResolvedSelection]);

  const setHoveredPiece = React.useCallback(
    (pieceGuid: string | null) => {
      if (!effectivePieceHoverEnabled) return;
      setResolvedHover({
        pieceGuid,
        connectionGuid: resolvedHover.connectionGuid ?? null,
      });
    },
    [effectivePieceHoverEnabled, resolvedHover.connectionGuid, setResolvedHover],
  );

  const setHoveredConnection = React.useCallback(
    (connectionGuid: string | null) => {
      if (!effectiveConnectionHoverEnabled) return;
      setResolvedHover({
        pieceGuid: resolvedHover.pieceGuid ?? null,
        connectionGuid,
      });
    },
    [effectiveConnectionHoverEnabled, resolvedHover.pieceGuid, setResolvedHover],
  );

  const handleSvgClick = React.useCallback(
    (event: React.MouseEvent<SVGSVGElement>) => {
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
      setResolvedPan({
        x: panOriginRef.current.panX + deltaX,
        y: panOriginRef.current.panY + deltaY,
      });
    },
    [setResolvedPan],
  );

  const handlePointerEnd = React.useCallback((event: React.PointerEvent<SVGSVGElement>) => {
    if (panPointerIdRef.current !== event.pointerId) return;
    panPointerIdRef.current = null;
    setIsPanning(false);
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
  }, []);

  const selectPiece = React.useCallback(
    (pieceGuid: string) => {
      if (!effectivePieceSelectionEnabled) return;
      const nextPieceGuids = new Set(resolvedSelection.pieceGuids ?? []);
      if (nextPieceGuids.has(pieceGuid)) {
        nextPieceGuids.delete(pieceGuid);
      } else {
        nextPieceGuids.add(pieceGuid);
      }
      setResolvedSelection({
        pieceGuids: Array.from(nextPieceGuids),
        connectionGuids: resolvedSelection.connectionGuids ?? [],
      });
    },
    [effectivePieceSelectionEnabled, resolvedSelection.connectionGuids, resolvedSelection.pieceGuids, setResolvedSelection],
  );

  const selectConnection = React.useCallback(
    (connectionGuid: string) => {
      if (!effectiveConnectionSelectionEnabled) return;
      const nextConnectionGuids = new Set(resolvedSelection.connectionGuids ?? []);
      if (nextConnectionGuids.has(connectionGuid)) {
        nextConnectionGuids.delete(connectionGuid);
      } else {
        nextConnectionGuids.add(connectionGuid);
      }
      setResolvedSelection({
        pieceGuids: resolvedSelection.pieceGuids ?? [],
        connectionGuids: Array.from(nextConnectionGuids),
      });
    },
    [effectiveConnectionSelectionEnabled, resolvedSelection.connectionGuids, resolvedSelection.pieceGuids, setResolvedSelection],
  );

  return (
    <div ref={ref} className={`h-full w-full ${className}`} onDoubleClick={handleDoubleClick} onWheel={handleWheel}>
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
        {snapshot.lines.map((line) => {
          const selected = isSelected(line.guid, selectedConnectionGuids);
          const hovered = hoveredConnectionGuid === line.guid;
          return (
            <line
              key={line.guid}
              onClick={
                onConnectionClick || effectiveConnectionSelectionEnabled
                  ? (event) => {
                      event.stopPropagation();
                      selectConnection(line.guid);
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
              onPointerEnter={effectiveConnectionHoverEnabled ? () => setHoveredConnection(line.guid) : undefined}
              onPointerLeave={effectiveConnectionHoverEnabled ? () => setHoveredConnection((resolvedHover.connectionGuid ?? null) === line.guid ? null : (resolvedHover.connectionGuid ?? null)) : undefined}
              x1={toPixelX(line.source.u)}
              x2={toPixelX(line.target.u)}
              y1={toPixelY(-line.source.v)}
              y2={toPixelY(-line.target.v)}
            />
          );
        })}
        {snapshot.points.map((point) => {
          const selected = isSelected(point.guid, selectedPieceGuids);
          const hovered = hoveredPieceGuid === point.guid;
          return (
            <circle
              key={point.guid}
              cx={toPixelX(point.u)}
              cy={toPixelY(-point.v)}
              fill={getEntityStatusColor(point.status)}
              onClick={
                onPieceClick || effectivePieceSelectionEnabled
                  ? (event) => {
                      event.stopPropagation();
                      selectPiece(point.guid);
                      onPieceClick?.(point.piece);
                    }
                  : undefined
              }
              onPointerEnter={effectivePieceHoverEnabled ? () => setHoveredPiece(point.guid) : undefined}
              onPointerLeave={effectivePieceHoverEnabled ? () => setHoveredPiece((resolvedHover.pieceGuid ?? null) === point.guid ? null : (resolvedHover.pieceGuid ?? null)) : undefined}
              r={(selected ? pieceRadius + 0.75 : hovered ? pieceRadius + 0.35 : pieceRadius) * resolvedZoom}
              stroke={selected || hovered ? getInteractiveEntityColor(point.status, selected, hovered) : "none"}
              strokeWidth={(selected ? 1.5 : hovered ? 1 : 0) * resolvedZoom}
              style={{ cursor: onPieceClick || effectivePieceSelectionEnabled ? "pointer" : "default" }}
            />
          );
        })}
      </svg>
    </div>
  );
};

// #endregion 🔖Diagram

// #region 🔖PieceSelection

/**
 * PieceSelection is a constrained Diagram configuration that only supports selecting pieces.
 *
 * Specs:
 * - Connection selection is always disabled (no connection hover/click selection state).
 * - Selection callbacks only return `pieceGuids`.
 */
export interface PieceSelectionState {
  pieceGuids?: string[];
}

export interface PieceSelectionProps extends Omit<SemioDiagramProps, "pieceSelectionEnabled" | "connectionSelectionEnabled" | "onConnectionClick" | "selection" | "defaultSelection" | "onSelectionChange"> {
  selection?: PieceSelectionState;
  defaultSelection?: PieceSelectionState;
  onSelectionChange?: (selection: PieceSelectionState) => void;
}

export const PieceSelection: React.FC<PieceSelectionProps> = ({ selection, defaultSelection, onSelectionChange, ...rest }) => {
  const mappedSelection = selection ? { pieceGuids: selection.pieceGuids ?? [], connectionGuids: [] } : undefined;
  const mappedDefaultSelection = defaultSelection ? { pieceGuids: defaultSelection.pieceGuids ?? [], connectionGuids: [] } : undefined;

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
              onSelectionChange({ pieceGuids: next.pieceGuids ?? [] });
            }
          : undefined
      }
    />
  );
};

// #endregion 🔖PieceSelection

// #region 🔖ConnectionSelection
// [🔖semio/ui/index.tsx#ConnectionSelection](repo://section/semio/ui/index.tsx/CONNECTIONSELECTION)
// Constrained Diagram wrapper that only supports selecting connections.

export interface ConnectionSelectionState {
  connectionGuids?: string[];
}

export interface ConnectionSelectionProps extends Omit<SemioDiagramProps, "pieceSelectionEnabled" | "connectionSelectionEnabled" | "onPieceClick" | "selection" | "defaultSelection" | "onSelectionChange"> {
  selection?: ConnectionSelectionState;
  defaultSelection?: ConnectionSelectionState;
  onSelectionChange?: (selection: ConnectionSelectionState) => void;
}

export const ConnectionSelection: React.FC<ConnectionSelectionProps> = ({ selection, defaultSelection, onSelectionChange, ...rest }) => {
  const mappedSelection = selection ? { pieceGuids: [], connectionGuids: selection.connectionGuids ?? [] } : undefined;
  const mappedDefaultSelection = defaultSelection ? { pieceGuids: [], connectionGuids: defaultSelection.connectionGuids ?? [] } : undefined;

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
              onSelectionChange({ connectionGuids: next.connectionGuids ?? [] });
            }
          : undefined
      }
    />
  );
};

// #endregion 🔖ConnectionSelection

// #region 🔖Vec

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

// #endregion 🔖Vec

// #region 🔖Vector

// Specs: Semio 3D vector component supporting display/select modes with partial/full
// controlled/uncontrolled behavior. Supports per-axis enable flags for partial selection.
// Summary: 3D vector editor/viewer with semio Vector (x,y,z) and per-axis controllable state.

export type VectorValue = Pick<SemioVector, "x" | "y" | "z">;

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
        <label htmlFor={`${id}-${axis}`} className="text-xs font-semibold uppercase text-muted-foreground">
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

  if (!displayEnabled) return null;
  return (
    <div id={id} data-slot="vector" className={`flex flex-col gap-3 ${className}`}>
      <div className="h-40 w-full overflow-hidden rounded-md border border-border bg-background">
        <VectorPreview3D vector={currentVector} />
      </div>
      {renderAxisRow("x", "X", currentVector.x, minX, maxX, xDisplayEnabled, xSelectionEnabled)}
      {renderAxisRow("y", "Y", currentVector.y, minY, maxY, yDisplayEnabled, ySelectionEnabled)}
      {renderAxisRow("z", "Z", currentVector.z, minZ, maxZ, zDisplayEnabled, zSelectionEnabled)}
    </div>
  );
};

// #endregion 🔖Vector

// #region 🔖Scene

// Specs: Minimal 3D scene rendering a design from a kit. Uses React Three Fiber Canvas
// with orthographic camera, grid, gizmo, and orbit controls. Pieces are rendered as
// positioned box geometries via their plane data. Fully iframe compatible (no window.top
// access, no cross-origin assumptions). frameloop="demand" for performance.
// Summary: Lightweight 3D scene viewer that renders a design's pieces as positioned boxes.

const SCENE_BOX_SIZE = 1;

const getSceneComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
const resolveSceneColor = (cssValue: string, fallback: string): string => {
  if (cssValue.startsWith("var(")) return getSceneComputedColor(cssValue.replace("var(", "").replace(")", "")) || fallback;
  if (cssValue === "currentColor") return fallback;
  return cssValue;
};
const SEMIO_TO_THREE_BASIS = toThreeRotation();
const THREE_TO_SEMIO_BASIS = SEMIO_TO_THREE_BASIS.clone().invert();
const SCENE_SEMIO_COLOR = "#ff344f";

interface ScenePieceAsset {
  piece: Piece;
  status: DiagramEntityStatus;
  modelName?: string;
  modelSource?: string;
}

interface SceneConnectionAsset {
  connection: Connection;
  sourcePiece: Piece;
  targetPiece: Piece;
  status: DiagramEntityStatus;
}

interface SceneSnapshot {
  pieces: ScenePieceAsset[];
  connections: SceneConnectionAsset[];
}

const getSceneFileSource = (file?: SemioFile): string | undefined => {
  if (!file) return undefined;
  if (typeof file.blob === "string" && file.blob.length > 0) return file.blob;
  if (typeof (file as SemioFile & { url?: string }).url === "string" && (file as SemioFile & { url?: string }).url!.length > 0) {
    return (file as SemioFile & { url?: string }).url;
  }
  return undefined;
};

const isSceneGltfSource = (source?: string, modelName?: string): boolean => {
  if (!source) return false;
  if (source.startsWith("data:model/gltf")) return true;
  const loweredName = modelName?.toLowerCase() ?? "";
  const loweredSource = source.split("?")[0].toLowerCase();
  return loweredName.endsWith(".glb") || loweredName.endsWith(".gltf") || loweredSource.endsWith(".glb") || loweredSource.endsWith(".gltf");
};

const buildScenePieceAssets = (kit: Kit, pieces: Array<{ piece: Piece; status: DiagramEntityStatus }>): ScenePieceAsset[] => {
  const kindsByGuid = new Map((kit.types ?? []).map((kind) => [kind.guid, kind] as const));
  const filesByGuid = new Map((kit.files ?? []).map((file) => [file.guid, file] as const));
  return pieces
    .filter(({ piece }) => piece.plane)
    .map(({ piece, status }) => {
      const kindGuid = piece.type?.guid;
      const kind = kindGuid ? kindsByGuid.get(kindGuid) : undefined;
      const selectedModel = kind?.models?.length ? selectBestModel(kind.models, []) : undefined;
      const file = selectedModel?.file?.guid ? filesByGuid.get(selectedModel.file.guid) : undefined;
      return {
        piece,
        status,
        modelName: file?.name,
        modelSource: getSceneFileSource(file),
      };
    });
};

const toSceneVector = (coord: { x: number; y: number; z: number }): THREE.Vector3 => new THREE.Vector3(coord.x, coord.y, coord.z).applyMatrix4(SEMIO_TO_THREE_BASIS);

const buildSceneSnapshot = (design: Design, designDiff?: DesignDiff): SceneSnapshot => {
  const baseDesign = design;
  const nextDesign = designDiff ? applyDesignDiff(baseDesign, designDiff) : baseDesign;
  const flatBaseDesign = baseDesign;
  const flatNextDesign = nextDesign;
  const removedPieceGuids = new Set((designDiff?.pieces?.removed ?? []).map((piece) => piece.guid));
  const addedPieceGuids = new Set((designDiff?.pieces?.added ?? []).map((piece) => piece.guid));
  const modifiedPieceGuids = new Set((designDiff?.pieces?.updated ?? []).map((piece) => piece.piece.guid));
  const removedConnectionGuids = new Set((designDiff?.connections?.removed ?? []).map((connection) => connection.guid));
  const addedConnectionGuids = new Set((designDiff?.connections?.added ?? []).map((connection) => connection.guid));
  const modifiedConnectionGuids = new Set((designDiff?.connections?.updated ?? []).map((connection) => connection.connection.guid));

  const pieceMap = new Map<string, ScenePieceAsset>();
  const upsertPiece = (piece: Piece, status: DiagramEntityStatus) => {
    if (!piece.guid || !piece.plane) return;
    pieceMap.set(piece.guid, { piece, status });
  };

  (flatBaseDesign.pieces ?? []).forEach((piece) => {
    if (removedPieceGuids.has(piece.guid)) {
      upsertPiece(piece, "removed");
    } else if (!designDiff) {
      upsertPiece(piece, "default");
    }
  });
  (flatNextDesign.pieces ?? []).forEach((piece) => {
    if (addedPieceGuids.has(piece.guid)) {
      upsertPiece(piece, "added");
    } else if (modifiedPieceGuids.has(piece.guid)) {
      upsertPiece(piece, "modified");
    } else {
      upsertPiece(piece, "default");
    }
  });

  const pieces = Array.from(pieceMap.values());
  const piecesByGuid = new Map(pieces.map((asset) => [asset.piece.guid, asset.piece] as const));
  const connectionMap = new Map<string, SceneConnectionAsset>();
  const upsertConnection = (connection: Connection, status: DiagramEntityStatus) => {
    if (!connection.guid) return;
    const sourcePiece = piecesByGuid.get(connection.connected.piece.guid);
    const targetPiece = piecesByGuid.get(connection.connecting.piece.guid);
    if (!sourcePiece?.plane || !targetPiece?.plane) return;
    connectionMap.set(connection.guid, {
      connection,
      sourcePiece,
      targetPiece,
      status,
    });
  };

  (flatBaseDesign.connections ?? []).forEach((connection) => {
    if (removedConnectionGuids.has(connection.guid)) {
      upsertConnection(connection, "removed");
    } else if (!designDiff) {
      upsertConnection(connection, "default");
    }
  });
  (flatNextDesign.connections ?? []).forEach((connection) => {
    if (addedConnectionGuids.has(connection.guid)) {
      upsertConnection(connection, "added");
    } else if (modifiedConnectionGuids.has(connection.guid)) {
      upsertConnection(connection, "modified");
    } else {
      upsertConnection(connection, "default");
    }
  });

  for (const { connection, status } of connectionMap.values()) {
    if (status === "default") continue;
    const childGuid = connection.connecting.piece.guid;
    const asset = pieceMap.get(childGuid);
    if (asset && asset.status === "default") asset.status = "modified";
  }

  return {
    pieces: Array.from(pieceMap.values()),
    connections: Array.from(connectionMap.values()),
  };
};

const toScenePieceMatrix = (plane: Plane): THREE.Matrix4 => {
  const planeMatrix = planeToMatrix(plane);
  return new THREE.Matrix4().multiplyMatrices(SEMIO_TO_THREE_BASIS, planeMatrix).multiply(THREE_TO_SEMIO_BASIS);
};

interface ScenePieceModelProps {
  modelSource: string;
  status: DiagramEntityStatus;
  isSelected: boolean;
  isHovered: boolean;
}

const ScenePieceModel: React.FC<ScenePieceModelProps> = ({ modelSource, status, isSelected, isHovered }) => {
  const gltf = useGLTF(modelSource);
  const semioColor = React.useMemo(() => resolveSceneColor("var(--color-primary)", SCENE_SEMIO_COLOR), []);
  const clone = React.useMemo(() => {
    const cloned = cloneSkeleton(gltf.scene);
    cloned.traverse((object) => {
      if (!(object instanceof THREE.Mesh)) return;
      if (Array.isArray(object.material)) {
        object.material = object.material.map((m) => m.clone());
      } else if (object.material) {
        object.material = object.material.clone();
      }
    });
    return cloned;
  }, [gltf.scene]);

  React.useEffect(() => {
    const isDiffed = status !== "default";
    const isRemoved = status === "removed";
    const statusColor = isDiffed ? resolveSceneColor(getEntityStatusColor(status), "#888888") : null;
    const selectedColor = isSelected ? resolveSceneColor(getInteractiveEntityColor(status, true, false), "#3b82f6") : null;
    const hoveredColor = isHovered ? resolveSceneColor(getInteractiveEntityColor(status, false, true), "#60a5fa") : null;
    clone.traverse((object) => {
      if (!(object instanceof THREE.Mesh)) return;
      const materials = Array.isArray(object.material) ? object.material : [object.material];
      materials.forEach((material) => {
        if (!material || !("color" in material)) return;
        const mat = material as THREE.MeshStandardMaterial;

        // Set the base color to semio color, ignoring imported colors
        mat.color.set(semioColor);

        if (isSelected && selectedColor) {
          mat.emissive.set(selectedColor);
          mat.emissiveIntensity = 0.35;
        } else if (isHovered && hoveredColor) {
          mat.emissive.set(hoveredColor);
          mat.emissiveIntensity = 0.15;
        } else if (isDiffed && statusColor) {
          mat.emissive.set(statusColor);
          mat.emissiveIntensity = 0.4;
        } else {
          mat.emissive.set("#000000");
          mat.emissiveIntensity = 0;
        }
        mat.transparent = isRemoved;
        mat.opacity = isRemoved ? 0.35 : 1;
      });
    });
  }, [clone, status, isHovered, isSelected, semioColor]);

  return <Clone object={clone} />;
};

interface ScenePieceProps {
  piece: Piece;
  status: DiagramEntityStatus;
  modelName?: string;
  modelSource?: string;
  isSelected: boolean;
  isHovered: boolean;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onClick?: () => void;
}

const ScenePiece: React.FC<ScenePieceProps> = ({ piece, status, modelName, modelSource, isSelected, isHovered, onPointerEnter, onPointerLeave, onClick }) => {
  const semioColor = React.useMemo(() => resolveSceneColor("var(--color-primary)", SCENE_SEMIO_COLOR), []);
  const defaultColor = React.useMemo(() => resolveSceneColor(getEntityStatusColor(status), "#888888"), [status]);
  const activeColor = React.useMemo(() => resolveSceneColor(getInteractiveEntityColor(status, true, false), "#3b82f6"), [status]);
  const hoverColor = React.useMemo(() => resolveSceneColor(getInteractiveEntityColor(status, false, true), "#60a5fa"), [status]);

  const matrix = React.useMemo(() => {
    if (!piece.plane) return null;
    return toScenePieceMatrix(piece.plane as Plane);
  }, [piece.plane]);

  const emissiveColor = isSelected ? activeColor : isHovered ? hoverColor : defaultColor;
  const edgeColor = isSelected ? activeColor : isHovered ? hoverColor : defaultColor;
  const isRemoved = status === "removed";

  if (!matrix) return null;

  const canRenderModel = isSceneGltfSource(modelSource, modelName);

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
    <group matrix={matrix} matrixAutoUpdate={false}>
      {canRenderModel && modelSource ? (
        <group onClick={handleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
          <React.Suspense fallback={null}>
            <ScenePieceModel modelSource={modelSource} status={status} isSelected={isSelected} isHovered={isHovered} />
          </React.Suspense>
        </group>
      ) : (
        <mesh onClick={handleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
          <boxGeometry args={[SCENE_BOX_SIZE, SCENE_BOX_SIZE, SCENE_BOX_SIZE]} />
          <meshStandardMaterial color={semioColor} emissive={emissiveColor} emissiveIntensity={isSelected ? 0.4 : isHovered ? 0.2 : 0} transparent={isRemoved} opacity={isRemoved ? 0.35 : 1} />
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

  const start = React.useMemo(() => (sourcePiece.plane ? toSceneVector(sourcePiece.plane.origin) : null), [sourcePiece.plane]);
  const end = React.useMemo(() => (targetPiece.plane ? toSceneVector(targetPiece.plane.origin) : null), [targetPiece.plane]);
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
    <mesh name={connection.guid} position={transform.midpoint} quaternion={transform.quaternion} onClick={handleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
      <cylinderGeometry args={[radius, radius, transform.length, 12]} />
      <meshStandardMaterial color={color} emissive={color} emissiveIntensity={isSelected ? 0.45 : isHovered ? 0.2 : 0.05} transparent={status === "removed"} opacity={status === "removed" ? 0.35 : 1} />
    </mesh>
  );
};

interface SceneGizmoProps {
  show: boolean;
}

const SceneGizmo: React.FC<SceneGizmoProps> = ({ show }) => {
  const [colors, setColors] = React.useState<[string, string, string]>(() => [getSceneComputedColor("--accent") || "#ef4444", getSceneComputedColor("--accent-tertiary") || "#22c55e", getSceneComputedColor("--accent-secondary") || "#3b82f6"]);

  React.useEffect(() => {
    const updateColors = () => setColors([getSceneComputedColor("--accent") || "#ef4444", getSceneComputedColor("--accent-tertiary") || "#22c55e", getSceneComputedColor("--accent-secondary") || "#3b82f6"]);
    updateColors();
    const observer = new MutationObserver(updateColors);
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ["class"] });
    return () => observer.disconnect();
  }, []);

  if (!show) return null;
  return (
    <GizmoHelper alignment="bottom-right" margin={[80, 80]}>
      <GizmoViewport labels={["X", "Z", "-Y"]} axisColors={colors} />
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
  onConnectionClick?: (connection: Connection) => void;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  className?: string;
  title?: string;
}

interface SceneInnerContentProps {
  showGrid: boolean;
  showGizmo: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  children?: React.ReactNode;
}

const SceneInnerContent: React.FC<SceneInnerContentProps> = ({ showGrid, showGizmo, camera: initialCamera, onCameraChange, children }) => {
  const { camera: threeCamera } = useThree();
  const controlsRef = React.useRef<any>(null);
  const isUpdatingCameraRef = React.useRef(false);
  const cameraRestoredRef = React.useRef(false);

  React.useEffect(() => {
    const cam = threeCamera as THREE.OrthographicCamera;
    if (cam && cam instanceof THREE.OrthographicCamera) {
      cam.zoom = 50;
      cam.updateProjectionMatrix();
    }
  }, [threeCamera]);

  React.useEffect(() => {
    if (!threeCamera || !controlsRef.current || cameraRestoredRef.current) return;
    isUpdatingCameraRef.current = true;
    if (initialCamera) {
      requestAnimationFrame(() => {
        if (!controlsRef.current) return;
        threeCamera.position.set(initialCamera.position.x, initialCamera.position.y, initialCamera.position.z);
        threeCamera.up.set(initialCamera.up.x, initialCamera.up.y, initialCamera.up.z);
        const target = new THREE.Vector3(initialCamera.position.x + initialCamera.forward.x, initialCamera.position.y + initialCamera.forward.y, initialCamera.position.z + initialCamera.forward.z);
        controlsRef.current.target.copy(target);
        threeCamera.updateProjectionMatrix();
        controlsRef.current.update();
        setTimeout(() => {
          isUpdatingCameraRef.current = false;
        }, 300);
      });
    } else {
      requestAnimationFrame(() => {
        if (!controlsRef.current) return;
        threeCamera.position.set(10, 10, 10);
        threeCamera.up.set(0, 1, 0);
        controlsRef.current.target.set(0, 0, 0);
        threeCamera.updateProjectionMatrix();
        controlsRef.current.update();
        setTimeout(() => {
          isUpdatingCameraRef.current = false;
        }, 300);
      });
    }
    cameraRestoredRef.current = true;
  }, [initialCamera, threeCamera]);

  const handleEnd = React.useCallback(() => {
    if (isUpdatingCameraRef.current || !onCameraChange || !controlsRef.current) return;
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
    });
  }, [onCameraChange, threeCamera]);

  return (
    <>
      <OrbitControls ref={controlsRef} enableDamping={false} onEnd={handleEnd} />
      <ambientLight intensity={1} />
      {children}
      <SceneGrid show={showGrid} />
      <SceneGizmo show={showGizmo} />
    </>
  );
};

export const SemioScene: React.FC<SemioSceneProps> = ({
  design,
  kit,
  designDiff,
  defaultDesignDiff,
  diffEnabled = true,
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
  onConnectionClick,
  showGrid = true,
  showGizmo = true,
  camera,
  onCameraChange,
  className = "",
  title = "Design Scene",
}) => {
  const resolvedDesignDiff = useResolvedValue(designDiff, defaultDesignDiff);
  const snapshot = React.useMemo(() => {
    const effectiveDiff = diffEnabled ? resolvedDesignDiff : undefined;
    return buildSceneSnapshot(design, effectiveDiff);
  }, [design, resolvedDesignDiff, diffEnabled]);

  const effectivePieceSelectionEnabled = selectionEnabled && pieceSelectionEnabled;
  const effectiveConnectionSelectionEnabled = selectionEnabled && connectionSelectionEnabled;
  const effectivePieceHoverEnabled = hoverEnabled && pieceHoverEnabled && (effectivePieceSelectionEnabled || !!onPieceClick);
  const effectiveConnectionHoverEnabled = hoverEnabled && connectionHoverEnabled && (effectiveConnectionSelectionEnabled || !!onConnectionClick);
  const [resolvedSelection, setResolvedSelection] = useInteractiveControllableValue(selection, normalizeSelection(defaultSelection), onSelectionChange);
  const [resolvedHover, setResolvedHover] = useInteractiveControllableValue(hover, normalizeHover(defaultHover), onHoverChange);
  const selectedPieceGuids = React.useMemo(() => new Set(selectionEnabled ? (resolvedSelection.pieceGuids ?? []) : []), [selectionEnabled, resolvedSelection.pieceGuids]);
  const selectedConnectionGuids = React.useMemo(() => new Set(selectionEnabled ? (resolvedSelection.connectionGuids ?? []) : []), [selectionEnabled, resolvedSelection.connectionGuids]);
  const hoveredPieceGuid = effectivePieceHoverEnabled ? (resolvedHover.pieceGuid ?? null) : null;
  const hoveredConnectionGuid = effectiveConnectionHoverEnabled ? (resolvedHover.connectionGuid ?? null) : null;

  const handleSelectPiece = React.useCallback(
    (pieceGuid: string) => {
      if (!effectivePieceSelectionEnabled) return;
      const nextGuids = new Set(resolvedSelection.pieceGuids ?? []);
      if (nextGuids.has(pieceGuid)) {
        nextGuids.delete(pieceGuid);
      } else {
        nextGuids.add(pieceGuid);
      }
      setResolvedSelection({
        pieceGuids: Array.from(nextGuids),
        connectionGuids: resolvedSelection.connectionGuids ?? [],
      });
    },
    [effectivePieceSelectionEnabled, resolvedSelection.connectionGuids, resolvedSelection.pieceGuids, setResolvedSelection],
  );

  const handleSelectConnection = React.useCallback(
    (connectionGuid: string) => {
      if (!effectiveConnectionSelectionEnabled) return;
      const nextGuids = new Set(resolvedSelection.connectionGuids ?? []);
      if (nextGuids.has(connectionGuid)) {
        nextGuids.delete(connectionGuid);
      } else {
        nextGuids.add(connectionGuid);
      }
      setResolvedSelection({
        pieceGuids: resolvedSelection.pieceGuids ?? [],
        connectionGuids: Array.from(nextGuids),
      });
    },
    [effectiveConnectionSelectionEnabled, resolvedSelection.connectionGuids, resolvedSelection.pieceGuids, setResolvedSelection],
  );

  const handleHoverPiece = React.useCallback(
    (pieceGuid: string | null) => {
      if (!effectivePieceHoverEnabled) return;
      setResolvedHover({
        pieceGuid,
        connectionGuid: resolvedHover.connectionGuid ?? null,
      });
    },
    [effectivePieceHoverEnabled, resolvedHover.connectionGuid, setResolvedHover],
  );

  const handleHoverConnection = React.useCallback(
    (connectionGuid: string | null) => {
      if (!effectiveConnectionHoverEnabled) return;
      setResolvedHover({
        pieceGuid: resolvedHover.pieceGuid ?? null,
        connectionGuid,
      });
    },
    [effectiveConnectionHoverEnabled, resolvedHover.pieceGuid, setResolvedHover],
  );

  const clearSelection = React.useCallback(() => {
    if (!selectionEnabled) return;
    setResolvedSelection({ pieceGuids: [], connectionGuids: [] });
  }, [selectionEnabled, setResolvedSelection]);

  const pieceAssets = React.useMemo(() => buildScenePieceAssets(kit ?? ({ guid: "", name: "", types: [], files: [] } as unknown as Kit), snapshot.pieces), [kit, snapshot.pieces]);

  return (
    <div className={`h-full w-full ${className}`} aria-label={title}>
      <ThreeCanvas onPointerMissed={clearSelection} orthographic frameloop="demand" camera={{ zoom: 50, position: [10, 10, 10], near: -10000, far: 10000 }} style={{ width: "100%", height: "100%" }}>
        <SceneInnerContent showGrid={showGrid} showGizmo={showGizmo} camera={camera} onCameraChange={onCameraChange}>
          {snapshot.connections.map(({ connection, sourcePiece, targetPiece, status }) => (
            <SceneConnection
              key={connection.guid}
              connection={connection}
              sourcePiece={sourcePiece}
              targetPiece={targetPiece}
              status={status}
              isSelected={selectedConnectionGuids.has(connection.guid)}
              isHovered={hoveredConnectionGuid === connection.guid}
              onClick={
                effectiveConnectionSelectionEnabled || onConnectionClick
                  ? () => {
                      handleSelectConnection(connection.guid);
                      onConnectionClick?.(connection);
                    }
                  : undefined
              }
              onPointerEnter={effectiveConnectionHoverEnabled ? () => handleHoverConnection(connection.guid) : undefined}
              onPointerLeave={effectiveConnectionHoverEnabled ? () => handleHoverConnection((resolvedHover.connectionGuid ?? null) === connection.guid ? null : (resolvedHover.connectionGuid ?? null)) : undefined}
            />
          ))}
          {pieceAssets.map(({ piece, status, modelName, modelSource }) => (
            <ScenePiece
              key={piece.guid}
              piece={piece}
              status={status}
              modelName={modelName}
              modelSource={modelSource}
              isSelected={selectedPieceGuids.has(piece.guid)}
              isHovered={hoveredPieceGuid === piece.guid}
              onClick={
                effectivePieceSelectionEnabled || onPieceClick
                  ? () => {
                      handleSelectPiece(piece.guid);
                      onPieceClick?.(piece);
                    }
                  : undefined
              }
              onPointerEnter={effectivePieceHoverEnabled ? () => handleHoverPiece(piece.guid) : undefined}
              onPointerLeave={effectivePieceHoverEnabled ? () => handleHoverPiece((resolvedHover.pieceGuid ?? null) === piece.guid ? null : (resolvedHover.pieceGuid ?? null)) : undefined}
            />
          ))}
        </SceneInnerContent>
      </ThreeCanvas>
    </div>
  );
};

// #endregion 🔖Scene

// #region 🔖Model

// Specs: Model is a direct alias of SemioScene with a different default title.
// Summary: 3D model viewer alias of SemioScene.

export type SemioModelProps = SemioSceneProps;

export const SemioModel: React.FC<SemioModelProps> = (props) => <SemioScene {...props} title={props.title ?? "Design Model"} />;

// #endregion 🔖Model

// #region 🔖Design

// Specs: Split-view design viewer with Diagram on the right and Scene on the left.
// Uses CSS grid for layout. Fully iframe compatible. Selection state is shared between
// the Diagram (2D) and Scene (3D) views. Handles the case where a design has no 3D
// plane data by showing only the Diagram.
// Summary: Combined 2D diagram + 3D scene split view for a design in a kit.

export interface SemioDesignProps {
  design: Design;
  kit?: Kit;
  designDiff?: DesignDiff;
  defaultDesignDiff?: DesignDiff;
  diffEnabled?: boolean;
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
  onConnectionClick?: (connection: Connection) => void;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  className?: string;
  title?: string;
  sceneRatio?: number;
}

export const SemioDesign: React.FC<SemioDesignProps> = ({
  design,
  kit,
  designDiff,
  defaultDesignDiff,
  diffEnabled = true,
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
  onConnectionClick,
  showGrid = true,
  showGizmo = true,
  camera,
  onCameraChange,
  className = "",
  title = "Design",
  sceneRatio = 0.5,
}) => {
  const resolvedDesignDiff = useResolvedValue(designDiff, defaultDesignDiff);
  const hasPlanes = React.useMemo(() => {
    const effectiveDiff = diffEnabled ? resolvedDesignDiff : undefined;
    const nextDesign = effectiveDiff ? applyDesignDiff(design, effectiveDiff) : design;
    return (nextDesign.pieces ?? []).some((p) => p.plane);
  }, [design, resolvedDesignDiff, diffEnabled]);

  const [resolvedSelection, setResolvedSelection] = useInteractiveControllableValue(selection, normalizeSelection(defaultSelection), onSelectionChange);
  const [resolvedHover, setResolvedHover] = useInteractiveControllableValue(hover, normalizeHover(defaultHover), onHoverChange);

  const scenePercent = Math.max(0.1, Math.min(0.9, sceneRatio)) * 100;
  const diagramPercent = 100 - scenePercent;

  return (
    <div
      className={`h-full w-full ${className}`}
      aria-label={title}
      style={{
        display: "grid",
        gridTemplateColumns: hasPlanes ? `${scenePercent}% ${diagramPercent}%` : "1fr",
      }}
    >
      {hasPlanes && (
        <div className="h-full w-full overflow-hidden border-r border-border">
          <SemioScene
            design={design}
            kit={kit}
            designDiff={resolvedDesignDiff}
            diffEnabled={diffEnabled}
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
            onConnectionClick={onConnectionClick}
            showGrid={showGrid}
            showGizmo={showGizmo}
            camera={camera}
            onCameraChange={onCameraChange}
            title={`${title} Scene`}
          />
        </div>
      )}
      <div className="h-full w-full overflow-hidden">
        <SemioDiagram
          design={design}
          designDiff={resolvedDesignDiff}
          diffEnabled={diffEnabled}
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
          onConnectionClick={onConnectionClick}
          title={`${title} Diagram`}
        />
      </div>
    </div>
  );
};

// #endregion 🔖Design

// #region 🔖McpApp
// [👤semio📚ui💻index🔖mcpapp](repo://p/u/semio/b/l/ui/f/index.tsx/s/McpApp)
// Specs: MCP App design viewer component using the official @modelcontextprotocol/ext-apps/react
// protocol. Communicates with the MCP host via useApp hook. Receives pre-computed diagram data
// (points and lines) from tool results as JSON text content. Renders pure SVG diagram.
// Summary: MCP App React component for rendering semio diagrams inside MCP host iframes.

import type { App as McpApp } from "@modelcontextprotocol/ext-apps";
import { useApp, useDocumentTheme, useHostStyles } from "@modelcontextprotocol/ext-apps/react";

// #region 🔖McpApp Types

interface McpDiagramPoint {
  guid: string;
  id: string;
  u: number;
  v: number;
  status: DiagramEntityStatus;
}

interface McpDiagramLine {
  guid: string;
  sourceU: number;
  sourceV: number;
  targetU: number;
  targetV: number;
  status: DiagramEntityStatus;
}

/**
 * Payload structure sent as JSON text content in MCP tool results.
 * Contains pre-computed diagram points and lines from the server.
 * [👤semio📚ui💻index🔖mcpapp🪨mcpdiagrampayload](repo://p/u/semio/b/l/ui/f/index.tsx/s/McpApp/d/i/McpDiagramPayload)
 **/
export interface McpDiagramPayload {
  points: McpDiagramPoint[];
  lines: McpDiagramLine[];
  capabilities?: {
    pieceSelection?: boolean;
    connectionSelection?: boolean;
  };
  kitArtifacts?: KitData;
}

// #endregion 🔖McpApp Types

/**
 * Normalizes a loose object into {@link McpDiagramPayload} when it carries kit/diagram data.
 * Hosts may send only {@link McpDiagramPayload.kitArtifacts} or omit empty arrays.
 **/
const normalizeMcpDiagramPayload = (raw: Record<string, unknown>): McpDiagramPayload | null => {
  const hasKit = raw.kitArtifacts !== undefined && raw.kitArtifacts !== null && typeof raw.kitArtifacts === "object";
  const hasDiagram = Array.isArray(raw.points) && Array.isArray(raw.lines);
  if (!hasKit && !hasDiagram) return null;
  return {
    points: Array.isArray(raw.points) ? (raw.points as McpDiagramPayload["points"]) : [],
    lines: Array.isArray(raw.lines) ? (raw.lines as McpDiagramPayload["lines"]) : [],
    capabilities: raw.capabilities as McpDiagramPayload["capabilities"],
    kitArtifacts: hasKit ? (raw.kitArtifacts as KitData) : undefined,
  };
};

/**
 * Deep-scans nested objects for a payload shape (some hosts nest JSON under extra keys).
 * [👤semio📚ui💻index🔖mcpapp🛠️deepfinddiagrampayload](repo://p/u/semio/b/l/ui/f/index.tsx/s/McpApp/d/i/deepFindDiagramPayload)
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
 * [👤semio📚ui💻index🔖mcpapp🛠️parsediagrampayloadfromtoolresult](repo://p/u/semio/b/l/ui/f/index.tsx/s/McpApp/d/i/parseDiagramPayloadFromToolResult)
 **/
export const parseDiagramPayloadFromToolResult = (result: unknown): McpDiagramPayload | null => {
  if (!result || typeof result !== "object") return null;
  let r = result as Record<string, unknown>;
  const params = r.params;
  if (params && typeof params === "object" && ("content" in params || "structuredContent" in params)) {
    r = params as Record<string, unknown>;
  }

  const structured = r.structuredContent;
  if (structured !== undefined && structured !== null) {
    if (typeof structured === "string") {
      try {
        const parsed = JSON.parse(structured) as unknown;
        if (parsed && typeof parsed === "object") {
          const n = normalizeMcpDiagramPayload(parsed as Record<string, unknown>);
          if (n) return n;
        }
      } catch {
        /* ignore */
      }
    } else if (typeof structured === "object" && !Array.isArray(structured)) {
      const n = normalizeMcpDiagramPayload(structured as Record<string, unknown>);
      if (n) return n;
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
    }
    const joined = textParts.join("").trim();
    if (joined.length > 0) {
      try {
        const parsed = JSON.parse(joined) as unknown;
        if (parsed && typeof parsed === "object") {
          const n = normalizeMcpDiagramPayload(parsed as Record<string, unknown>);
          if (n) return n;
        }
      } catch {
        /* fall through to deep search */
      }
    }
  }

  const direct = normalizeMcpDiagramPayload(r);
  if (direct) return direct;
  return deepFindDiagramPayload(r);
};

/**
 * MCP App design viewer that renders a semio diagram using the official MCP Apps protocol.
 * Uses useApp from @modelcontextprotocol/ext-apps/react for host communication.
 * Receives pre-computed diagram data (points and lines) from tool results.
 * [👤semio📚ui💻index🔖mcpapp🛠️mcpdesignviewer](repo://p/u/semio/b/l/ui/f/index.tsx/s/McpApp/d/i/McpDesignViewer)
 *
 * Specs:
 * - Connects to MCP host via useApp hook with PostMessageTransport.
 * - Receives pre-computed diagram points/lines from tool results via ontoolresult callback.
 * - Renders SVG diagram with pan, zoom, and selection support.
 * - Sends selection changes back to host via updateModelContext.
 **/
export const McpDesignViewer: React.FC = () => {
  const [payload, setPayload] = React.useState<McpDiagramPayload | null>(null);
  const [selectedPieces, setSelectedPieces] = React.useState<Set<string>>(new Set());
  const [selectedConnections, setSelectedConnections] = React.useState<Set<string>>(new Set());
  const [kitSelection, setKitSelection] = React.useState<KitSelection>({ designGuids: [], typeGuids: [], portGuids: [] });
  const appRef = React.useRef<McpApp | null>(null);

  const { app, isConnected, error } = useApp({
    appInfo: { name: "semio design viewer", version: "1.0.0" },
    capabilities: {},
    onAppCreated: (a) => {
      appRef.current = a;
      a.ontoolresult = (result) => {
        const parsed = parseDiagramPayloadFromToolResult(result);
        if (parsed) {
          applyDiagramPayload(parsed);
        }
      };
      a.ontoolcancelled = () => {};
      a.onteardown = async () => ({});
      a.onerror = console.error;
    },
  });

  useHostStyles(app, app?.getHostContext());

  const applyDiagramPayload = React.useCallback((p: McpDiagramPayload) => {
    setPayload(p);
    setSelectedPieces(new Set());
    setSelectedConnections(new Set());
    setKitSelection({ designGuids: [], typeGuids: [], portGuids: [] });
  }, []);

  React.useEffect(() => {
    if (!app) return;
    const h = app.getHostContext() as Record<string, unknown> | undefined;
    if (!h) return;
    for (const k of ["toolResult", "lastToolResult", "toolExecutionResult", "initialToolResult", "pendingToolResult"]) {
      const v = h[k];
      if (v === undefined) continue;
      const p = parseDiagramPayloadFromToolResult(v);
      if (p) {
        applyDiagramPayload(p);
        return;
      }
    }
  }, [app, applyDiagramPayload]);

  // Sync MCP host theme with semio's .dark class convention.
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
      appRef.current.updateModelContext({
        content: [{ type: "text" as const, text: JSON.stringify({ selectionChange: { pieceGuids: Array.from(pieces), connectionGuids: Array.from(connections) } }) }],
      });
    }
  }, []);

  const sendKitSelectionUpdate = React.useCallback((next: KitSelection) => {
    if (!appRef.current) return;
    appRef.current.updateModelContext({
      content: [{ type: "text" as const, text: JSON.stringify({ kitArtifactSelectionChange: { designGuids: next.designGuids ?? [], typeGuids: next.typeGuids ?? [], portGuids: next.portGuids ?? [] } }) }],
    });
  }, []);

  if (error) {
    return (
      <div style={{ display: "flex", alignItems: "center", justifyContent: "center", height: "100vh", fontFamily: "system-ui, sans-serif", background: "var(--color-background-primary, #ffffff)", color: "var(--color-text-danger, #dc2626)" }}>
        <p>Error: {error.message}</p>
      </div>
    );
  }

  if (!isConnected || !app) {
    return (
      <div style={{ display: "flex", alignItems: "center", justifyContent: "center", height: "100vh", fontFamily: "system-ui, sans-serif", background: "var(--color-background-primary, #ffffff)", color: "var(--color-text-secondary, #737373)" }}>
        <p>Connecting to host…</p>
      </div>
    );
  }

  if (!payload) {
    return (
      <div style={{ display: "flex", alignItems: "center", justifyContent: "center", height: "100vh", fontFamily: "system-ui, sans-serif", background: "var(--color-background-primary, #ffffff)", color: "var(--color-text-secondary, #737373)" }}>
        <p>Waiting for design data…</p>
      </div>
    );
  }

  const hasDiagram = payload.points.length > 0;
  const pieceSelectionEnabled = payload.capabilities?.pieceSelection ?? false;
  const connectionSelectionEnabled = payload.capabilities?.connectionSelection ?? false;

  const handleKitSelectionChange = (next: KitSelection) => {
    setKitSelection(next);
    sendKitSelectionUpdate(next);
  };

  if (!hasDiagram && payload.kitArtifacts) {
    return (
      <div style={{ width: "100%", height: "100vh", overflow: "auto", padding: 12, background: "var(--base, var(--color-background-primary, #ffffff))", color: "var(--foreground)" }}>
        <SemioKit data={payload.kitArtifacts} selection={kitSelection} onSelectionChange={handleKitSelectionChange} title="Kit Artifacts" />
      </div>
    );
  }

  return (
    <div style={{ width: "100%", height: "100vh", position: "relative" }}>
      {payload.kitArtifacts && (
        <div style={{ position: "absolute", left: 12, top: 12, right: 12, pointerEvents: "none", zIndex: 10 }}>
          <div style={{ pointerEvents: "auto", maxHeight: "38vh", overflow: "auto" }}>
            <SemioKit data={payload.kitArtifacts} selection={kitSelection} onSelectionChange={handleKitSelectionChange} title="Kit Artifacts" />
          </div>
        </div>
      )}
      <SemioDiagram
        design={
          {
            guid: "__mcp__",
            pieces: payload.points.map((p) => ({ guid: p.guid, id: p.id, center: { u: p.u, v: p.v } })),
            connections: payload.lines.map((l) => ({
              guid: l.guid,
              connected: { piece: { guid: payload.points.find((p) => p.u === l.sourceU && p.v === l.sourceV)?.guid ?? "" } },
              connecting: { piece: { guid: payload.points.find((p) => p.u === l.targetU && p.v === l.targetV)?.guid ?? "" } },
            })),
          } as unknown as Design
        }
        selectionEnabled={pieceSelectionEnabled || connectionSelectionEnabled}
        pieceSelectionEnabled={pieceSelectionEnabled}
        connectionSelectionEnabled={connectionSelectionEnabled}
        selection={{
          pieceGuids: Array.from(selectedPieces),
          connectionGuids: Array.from(selectedConnections),
        }}
        onSelectionChange={(next) => {
          const nextPieces = new Set(next.pieceGuids ?? []);
          const nextConns = new Set(next.connectionGuids ?? []);
          setSelectedPieces(nextPieces);
          setSelectedConnections(nextConns);
          sendSelectionUpdate(nextPieces, nextConns);
        }}
      />
    </div>
  );
};

/**
 * MCP App kit viewer: renders only {@link SemioKit} from tool results (kit artifact payload).
 * Used when the MCP host loads `ui://semio/kit-viewer` after kit-scoped tools such as start_working_in_local_kit.
 * [👤semio📚ui💻index🔖mcpapp🛠️mcpkitviewer](repo://p/u/semio/b/l/ui/f/index.tsx/s/McpApp/d/i/McpKitViewer)
 *
 * Specs:
 * - Same host connection as {@link McpDesignViewer} but no diagram; kit selection sync only.
 **/
export const McpKitViewer: React.FC = () => {
  const [payload, setPayload] = React.useState<McpDiagramPayload | null>(null);
  const [kitSelection, setKitSelection] = React.useState<KitSelection>({ designGuids: [], typeGuids: [], portGuids: [] });
  const appRef = React.useRef<McpApp | null>(null);
  const toolInputArgsRef = React.useRef<Record<string, unknown> | null>(null);
  const gotPayloadRef = React.useRef(false);
  const tryRefetchRef = React.useRef<() => void>(() => {});

  const applyKitPayload = React.useCallback((p: McpDiagramPayload) => {
    gotPayloadRef.current = true;
    setPayload(p);
    setKitSelection({ designGuids: [], typeGuids: [], portGuids: [] });
  }, []);

  const tryRefetchKitFromServer = React.useCallback(async () => {
    const client = appRef.current;
    if (!client || gotPayloadRef.current) return;
    const args = toolInputArgsRef.current;
    if (!args) return;
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
      if (p) applyKitPayload(p);
    } catch {
      /* Host may not proxy tools/call to the server for this session. */
    }
  }, [applyKitPayload]);

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
      a.ontoolresult = (result) => {
        const parsed = parseDiagramPayloadFromToolResult(result);
        if (parsed) {
          applyKitPayload(parsed);
        }
      };
      a.ontoolcancelled = () => {};
      a.onteardown = async () => ({});
      a.onerror = console.error;
    },
  });

  useHostStyles(app, app?.getHostContext());

  React.useEffect(() => {
    if (!app) return;
    const h = app.getHostContext() as Record<string, unknown> | undefined;
    if (!h) return;
    for (const k of ["toolResult", "lastToolResult", "toolExecutionResult", "initialToolResult", "pendingToolResult"]) {
      const v = h[k];
      if (v === undefined) continue;
      const p = parseDiagramPayloadFromToolResult(v);
      if (p) {
        applyKitPayload(p);
        return;
      }
    }
  }, [app, applyKitPayload]);

  React.useEffect(() => {
    if (!app || !isConnected) return;
    const delays = [0, 50, 150, 400, 1200];
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
    appRef.current.updateModelContext({
      content: [{ type: "text" as const, text: JSON.stringify({ kitArtifactSelectionChange: { designGuids: next.designGuids ?? [], typeGuids: next.typeGuids ?? [], portGuids: next.portGuids ?? [] } }) }],
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
    backgroundColor: "var(--color-background-primary, #1e1e1e)",
    /* Fixed fallbacks: some hosts set CSS variables to values that hide text in the MCP sandbox. */
    color: "var(--color-text-primary, #e5e5e5)",
  };

  const mutedStyle: React.CSSProperties = {
    color: "#a3a3a3",
  };

  if (error) {
    return (
      <div style={{ ...shellStyle, color: "#f87171" }}>
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

  if (!payload) {
    return (
      <div style={shellStyle}>
        <p style={mutedStyle}>Waiting for kit data…</p>
      </div>
    );
  }

  if (!payload.kitArtifacts) {
    return (
      <div style={shellStyle}>
        <p style={mutedStyle}>No kit artifact data in tool result.</p>
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
        backgroundColor: "var(--color-background-primary, #1e1e1e)",
        color: "var(--color-text-primary, #fafafa)",
      }}
    >
      <SemioKit data={payload.kitArtifacts} selection={kitSelection} onSelectionChange={handleKitSelectionChange} title="Kit" className="min-h-0 text-foreground" />
    </div>
  );
};

/**
 * Mount the MCP design viewer as a standalone app.
 * Call this from the entry point TSX file after importing react-dom/client.
 * [👤semio📚ui💻index🔖mcpapp🛠️mountmcpdesignviewer](repo://p/u/semio/b/l/ui/f/index.tsx/s/McpApp/d/i/mountMcpDesignViewer)
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
 * Mount the MCP kit viewer (SemioKit only) as a standalone app.
 * [👤semio📚ui💻index🔖mcpapp🛠️mountmcpkitviewer](repo://p/u/semio/b/l/ui/f/index.tsx/s/McpApp/d/i/mountMcpKitViewer)
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

// #endregion 🔖McpApp

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("parseDiagramPayloadFromToolResult", () => {
    it("parses JSON from MCP text content blocks", () => {
      const payload = { points: [], lines: [], kitArtifacts: { name: "K", designs: [], types: [], ports: [] } };
      const r = parseDiagramPayloadFromToolResult({
        content: [{ type: "text", text: JSON.stringify(payload) }],
      });
      expect(r?.kitArtifacts?.name).toBe("K");
    });

    it("parses structuredContent object (hosts that omit text content)", () => {
      const inner = { points: [], lines: [], kitArtifacts: { name: "S", designs: [{ guid: "d1", name: "D" }], types: [], ports: [] } };
      const r = parseDiagramPayloadFromToolResult({ structuredContent: inner });
      expect(r?.kitArtifacts?.designs?.[0]?.guid).toBe("d1");
    });

    it("parses structuredContent JSON string", () => {
      const inner = { points: [], lines: [], capabilities: {}, kitArtifacts: { designs: [], types: [], ports: [] } };
      const r = parseDiagramPayloadFromToolResult({ structuredContent: JSON.stringify(inner) });
      expect(r?.points).toEqual([]);
    });

    it("unwraps notification params", () => {
      const payload = { points: [], lines: [], kitArtifacts: { name: "P", designs: [], types: [], ports: [] } };
      const r = parseDiagramPayloadFromToolResult({
        params: { content: [{ type: "text", text: JSON.stringify(payload) }] },
      });
      expect(r?.kitArtifacts?.name).toBe("P");
    });

    it("reads nested kit payload under arbitrary host keys", () => {
      const inner = { points: [], lines: [], kitArtifacts: { name: "Deep", designs: [], types: [], ports: [] } };
      const r = parseDiagramPayloadFromToolResult({ wrapper: { data: inner } });
      expect(r?.kitArtifacts?.name).toBe("Deep");
    });

    it("reads text from embedded resource content blocks", () => {
      const payload = { points: [], lines: [], kitArtifacts: { name: "Res", designs: [], types: [], ports: [] } };
      const r = parseDiagramPayloadFromToolResult({
        content: [{ type: "resource", resource: { uri: "x", mimeType: "text/plain", text: JSON.stringify(payload) } }],
      });
      expect(r?.kitArtifacts?.name).toBe("Res");
    });
  });

  const testPlane: Plane = {
    origin: { x: 0, y: 0, z: 0 },
    xAxis: { x: 1, y: 0, z: 0 },
    yAxis: { x: 0, y: 1, z: 0 },
  };

  describe("buildKitDataFromKit", () => {
    it("normalizes connector port references into string labels instead of raw guid objects", () => {
      const data = buildKitDataFromKit({
        guid: "kit-guid",
        name: "Kit",
        version: "1",
        types: [
          {
            guid: "kind-guid",
            name: "Kind",
            connectors: [
              {
                guid: "connector-guid",
                name: "",
                port: { guid: "port-guid" },
              },
              {
                guid: "named-connector-guid",
                port: { guid: "named-port-guid", name: "Named Port" },
              },
            ],
          },
        ],
      } as unknown as Kit);

      expect(data.ports).toEqual([
        {
          guid: "connector-guid",
          typeGuid: "kind-guid",
          id: "",
          port: "port-guid",
          name: "port-guid",
          description: undefined,
          mandatory: undefined,
        },
        {
          guid: "named-connector-guid",
          typeGuid: "kind-guid",
          id: undefined,
          port: "named-port-guid",
          name: "Named Port",
          description: undefined,
          mandatory: undefined,
        },
      ]);
    });

    it("returns shallow kit kinds without requiring connector expansion", () => {
      const data = buildKitDataFromKit({
        guid: "kit-guid",
        name: "Kit",
        version: "1",
        types: [{ guid: "kind-guid", name: "Kind" }],
        designs: [{ guid: "design-guid", name: "Design" }],
      } as unknown as Kit);

      expect(data.types).toEqual([{ guid: "kind-guid", name: "Kind" }]);
      expect(data.designs).toEqual([{ guid: "design-guid", name: "Design" }]);
      expect(data.ports).toEqual([]);
    });
  });

  describe("buildKitHierarchy", () => {
    it("builds a dynamic type breadcrumb path from nested parent kinds", () => {
      const hierarchy = buildKitHierarchy(
        {
          name: "Metabolism",
          types: [
            { guid: "capsule", name: "Capsule" },
            { guid: "ellipsoid", name: "Ellipsoid", parent: { guid: "capsule" } },
            { guid: "l", name: "L", parent: { guid: "ellipsoid" } },
          ],
        },
        { designDataEnabled: true, typeDataEnabled: true, portDataEnabled: true },
      );

      expect(getKitNodePath(hierarchy, "kind:l").map((node) => node.label)).toEqual(["Kit", "Metabolism", "Types", "Capsule", "Ellipsoid", "L"]);
    });

    it("exposes child nodes from each breadcrumb step instead of sibling nodes", () => {
      const hierarchy = buildKitHierarchy(
        {
          name: "Metabolism",
          types: [
            { guid: "capsule", name: "Capsule" },
            { guid: "ellipsoid", name: "Ellipsoid", parent: { guid: "capsule" } },
            { guid: "balcony", name: "Balcony", parent: { guid: "capsule" } },
          ],
        },
        { designDataEnabled: true, typeDataEnabled: true, portDataEnabled: true },
      );

      expect(getKitChildNodes(hierarchy, hierarchy.nodesByKey.get("kind:capsule")!).map((node) => node.label)).toEqual(["Balcony", "Ellipsoid"]);
      expect(getKitChildNodes(hierarchy, hierarchy.nodesByKey.get("kind:ellipsoid")!)).toEqual([]);
    });

    it("attaches ports beneath their resolved kind parent and derives port selection", () => {
      const hierarchy = buildKitHierarchy(
        {
          name: "Metabolism",
          types: [{ guid: "l", name: "L" }],
          ports: [{ guid: "entry", typeGuid: "l", name: "Entry" }],
        },
        { designDataEnabled: true, typeDataEnabled: true, portDataEnabled: true },
      );

      expect(getKitNodePath(hierarchy, "port:entry").map((node) => node.label)).toEqual(["Kit", "Metabolism", "Types", "L", "Entry"]);
      expect(getKitNodeSelection(hierarchy.nodesByKey.get("port:entry")!)).toEqual({ designGuids: [], typeGuids: [], portGuids: ["entry"] });
    });

    it("falls back to the first populated group when no artifact is selected", () => {
      const hierarchy = buildKitHierarchy(
        {
          name: "Metabolism",
          designs: [{ guid: "tower", name: "Tower" }],
          types: [{ guid: "capsule", name: "Capsule" }],
        },
        { designDataEnabled: true, typeDataEnabled: true, portDataEnabled: true },
      );

      expect(getDefaultKitNodeKey(hierarchy)).toBe("design:tower");
      expect(getSelectedKitNodeKey(hierarchy, { designGuids: [], typeGuids: [], portGuids: [] })).toBeUndefined();
    });
  });

  describe("buildScenePieceAssets", () => {
    it("selects the untagged default model when no tags are requested", () => {
      const kit = {
        types: [
          {
            guid: "kind-1",
            models: [
              { guid: "model-tagged", file: { guid: "file-tagged" }, tags: [{ guid: "tag-1" }] },
              { guid: "model-default", file: { guid: "file-default" } },
            ],
          },
        ],
        files: [
          { guid: "file-tagged", name: "tagged.glb", blob: "data:model/gltf-binary;base64,AAA" },
          { guid: "file-default", name: "default.glb", blob: "data:model/gltf-binary;base64,BBB" },
        ],
      } as unknown as Kit;

      const assets = buildScenePieceAssets(kit, [{ piece: { guid: "piece-1", type: { guid: "kind-1" }, plane: testPlane } as Piece, status: "default" }]);

      expect(assets[0]?.modelSource).toBe("data:model/gltf-binary;base64,BBB");
      expect(assets[0]?.modelName).toBe("default.glb");
      expect(assets[0]?.status).toBe("default");
    });

    it("falls back to the first model when the kind has no untagged default model", () => {
      const kit = {
        types: [
          {
            guid: "kind-1",
            models: [
              { guid: "model-first", file: { guid: "file-first" }, tags: [{ guid: "tag-1" }] },
              { guid: "model-second", file: { guid: "file-second" }, tags: [{ guid: "tag-2" }] },
            ],
          },
        ],
        files: [
          { guid: "file-first", name: "first.glb", blob: "data:model/gltf-binary;base64,AAA" },
          { guid: "file-second", name: "second.glb", blob: "data:model/gltf-binary;base64,BBB" },
        ],
      } as unknown as Kit;

      const assets = buildScenePieceAssets(kit, [{ piece: { guid: "piece-1", type: { guid: "kind-1" }, plane: testPlane } as Piece, status: "modified" }]);

      expect(assets[0]?.modelSource).toBe("data:model/gltf-binary;base64,AAA");
      expect(assets[0]?.modelName).toBe("first.glb");
      expect(assets[0]?.status).toBe("modified");
    });

    it("keeps pieces in the scene and falls back to placeholder geometry when no file source can be resolved", () => {
      const kit = {
        types: [{ guid: "kind-1", models: [{ guid: "model-1", file: { guid: "file-1" } }] }],
        files: [{ guid: "file-1", name: "missing.glb" }],
      } as unknown as Kit;

      const assets = buildScenePieceAssets(kit, [{ piece: { guid: "piece-1", type: { guid: "kind-1" }, plane: testPlane } as Piece, status: "added" }]);

      expect(assets).toHaveLength(1);
      expect(assets[0]?.modelSource).toBeUndefined();
      expect(assets[0]?.piece.guid).toBe("piece-1");
      expect(assets[0]?.status).toBe("added");
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

  describe("normalizeHover", () => {
    it("fills missing hover fields with null", () => {
      expect(normalizeHover()).toEqual({ pieceGuid: null, connectionGuid: null });
      expect(normalizeHover({ pieceGuid: "piece-1" })).toEqual({ pieceGuid: "piece-1", connectionGuid: null });
    });
  });

  describe("buildSceneSnapshot", () => {
    it("includes piece and connection statuses for flattened scene rendering", () => {
      const pieceA = {
        guid: "piece-a",
        type: { guid: "kind-1" },
        plane: testPlane,
        center: { u: 0, v: 0 },
      } as unknown as Piece;
      const pieceB = {
        guid: "piece-b",
        type: { guid: "kind-1" },
        plane: { ...testPlane, origin: { x: 2, y: 0, z: 0 } },
        center: { u: 2, v: 0 },
      } as unknown as Piece;
      const pieceC = {
        guid: "piece-c",
        type: { guid: "kind-1" },
        plane: { ...testPlane, origin: { x: 4, y: 0, z: 0 } },
        center: { u: 4, v: 0 },
      } as unknown as Piece;

      const connectionA = {
        guid: "connection-a",
        connected: { piece: { guid: "piece-a" } },
        connecting: { piece: { guid: "piece-b" } },
      } as unknown as Connection;
      const connectionB = {
        guid: "connection-b",
        connected: { piece: { guid: "piece-b" } },
        connecting: { piece: { guid: "piece-c" } },
      } as unknown as Connection;

      const design = {
        guid: "design-1",
        pieces: [pieceA, pieceB],
        connections: [connectionA],
      } as unknown as Design;

      const kit = {
        designs: [design],
        types: [{ guid: "kind-1" }],
      } as unknown as Kit;

      const diff = {
        pieces: {
          added: [pieceC],
          updated: [{ piece: { guid: "piece-b" }, diff: {} }],
        },
        connections: {
          added: [connectionB],
          updated: [{ connection: { guid: "connection-a" }, diff: {} }],
        },
      } as unknown as DesignDiff;

      const snapshot = buildSceneSnapshot(design, diff);

      expect(snapshot.pieces.map((asset) => [asset.piece.guid, asset.status])).toEqual([
        ["piece-a", "default"],
        ["piece-b", "modified"],
        ["piece-c", "added"],
      ]);
      expect(snapshot.connections.map((asset) => [asset.connection.guid, asset.status])).toEqual([
        ["connection-a", "modified"],
        ["connection-b", "added"],
      ]);
    });
  });
}

// #region 🔖AlgorithmApp

// Specs: Reusable algorithm app shell. Each algorithm declares typed windows (VecInput,
// PiecesSelectionInput, DesignDiffOutput, DesignOutput) and an AlgorithmApp creates
// the UIAppConfig and renders the UI composite component. Data flows through
// AlgorithmContext which provides kit, design, diff, selection, vec, and output state.
// WindowKinds: VecInput (2D vector pad), PiecesSelectionInput (Diagram with piece selection, no diff),
// DesignDiffOutput (Diagram with diff, no selection), DesignOutput (Diagram with no diff, no selection).
// Summary: Standardized algorithm IPO shell using typed WindowKind-based windows.

import { TreeRow, TreeSection, UI, WindowKind, cn, createDefaultLayout, type FooterItem, type SidePanelTabConfig, type UIAppConfig, type UIWindowKindDefinition } from "@elements/ui/elements";
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
  selectedPieceGuids: string[];
  onSelectedPieceGuidsChange?: (guids: string[]) => void;
  designDiff?: DesignDiff;
  diffDesign?: Design;
  outputDesign: Design;
  error?: string;
}

const AlgorithmContext = React.createContext<AlgorithmContextValue | null>(null);

/**
 * Hook to access algorithm context from inside algorithm windows.
 **/
export function useAlgorithm(): AlgorithmContextValue {
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
}

type AlgorithmWindowKind = WindowKind.VEC_INPUT | WindowKind.PIECES_SELECTION_INPUT | WindowKind.DESIGN_DIFF_OUTPUT | WindowKind.DESIGN_OUTPUT;

type AlgorithmUiComponentId = "semio/ui:Vec" | "semio/ui:PieceSelection" | "semio/ui:Diagram";

interface AlgorithmWindowBehavior {
  kind: AlgorithmWindowKind;
  uiComponentId: AlgorithmUiComponentId;
  selectionEnabled: boolean;
  diffEnabled: boolean;
  usesPieceSelection: boolean;
  component: React.ComponentType<any>;
  createProps: (context: AlgorithmContextValue) => Record<string, any>;
  render: (component: React.ReactElement, context: AlgorithmContextValue) => React.ReactElement;
}

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

const renderAlgorithmDiffWindow = (component: React.ReactElement, context: AlgorithmContextValue): React.ReactElement => {
  if (context.error) {
    return <div className="h-full flex items-center justify-center p-2 text-sm text-destructive font-mono">{context.error}</div>;
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
  [WindowKind.PIECES_SELECTION_INPUT]: {
    uiComponentId: "semio/ui:PieceSelection",
    selectionEnabled: true,
    diffEnabled: false,
    usesPieceSelection: true,
    component: PieceSelection,
    createProps: (context) => ({
      design: context.design,
      selection: { pieceGuids: context.selectedPieceGuids },
      onSelectionChange: (next: PieceSelectionState) => context.onSelectedPieceGuidsChange?.(next.pieceGuids ?? []),
      selectionEnabled: true,
      diffEnabled: false,
      panEnabled: false,
      zoomEnabled: true,
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
      designDiff: context.designDiff,
      diffEnabled: true,
      selectionEnabled: false,
    }),
    render: renderAlgorithmDiffWindow,
  },
  [WindowKind.DESIGN_OUTPUT]: {
    uiComponentId: "semio/ui:Diagram",
    selectionEnabled: false,
    diffEnabled: false,
    usesPieceSelection: false,
    component: SemioDiagram,
    createProps: (context) => ({
      design: context.outputDesign,
      diffEnabled: false,
      selectionEnabled: false,
    }),
    render: renderAlgorithmFullWindow,
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
    const behavior = getAlgorithmWindowBehavior(windowDef.kind);
    return {
      id: windowDef.id,
      label: windowDef.label ?? windowDef.id,
      component: behavior ? createAlgorithmWindowRenderer(windowDef.kind as AlgorithmWindowKind) : () => <div className="p-2 text-sm text-muted-foreground">Unknown window kind: {windowDef.kind}</div>,
    };
  });
}

export function createAlgorithmLayout(windows: AlgorithmWindowDef[], defaultLayout?: AlgorithmAppProps["defaultLayout"]) {
  return (
    defaultLayout ??
    createDefaultLayout(
      windows.map((windowDef) => windowDef.id),
      "row",
      undefined,
      windows.map((windowDef) => windowDef.label ?? windowDef.id),
    )
  );
}

// #region 🔖AlgorithmDetailsPanel

/**
 * Details panel for algorithms showing context, selected pieces, vec, and error state.
 **/
const AlgorithmDetailsPanel: React.FC = () => {
  const ctx = React.useContext(AlgorithmContext);
  if (!ctx) return null;

  const design = ctx.design;
  const allPieces = design?.pieces ?? [];
  const selectedPieces = allPieces.filter((p) => ctx.selectedPieceGuids.includes(p.guid));

  return (
    <div className="flex flex-col h-full overflow-y-auto">
      {/* Design section */}
      <TreeSection id="algorithm.details.design" label="Design" icon={<DetailsIcon size={14} />} defaultOpen={true}>
        <TreeRow id="algorithm.details.design.name">
          <div className="flex items-center justify-between w-full px-2 py-0.5">
            <span className="text-xs text-muted-foreground">name</span>
            <span className="text-xs font-mono truncate max-w-32">{design?.name ?? "—"}</span>
          </div>
        </TreeRow>
        <TreeRow id="algorithm.details.design.pieces">
          <div className="flex items-center justify-between w-full px-2 py-0.5">
            <span className="text-xs text-muted-foreground">pieces</span>
            <span className="text-xs font-mono">{allPieces.length}</span>
          </div>
        </TreeRow>
        <TreeRow id="algorithm.details.design.connections">
          <div className="flex items-center justify-between w-full px-2 py-0.5">
            <span className="text-xs text-muted-foreground">connections</span>
            <span className="text-xs font-mono">{design?.connections?.length ?? 0}</span>
          </div>
        </TreeRow>
      </TreeSection>

      {/* Vec section (only if vec is present) */}
      {ctx.vec && (
        <TreeSection id="algorithm.details.vec" label="Vec" icon={<DetailsIcon size={14} />} defaultOpen={true}>
          <TreeRow id="algorithm.details.vec.u">
            <div className="flex items-center justify-between w-full px-2 py-0.5">
              <span className="text-xs text-muted-foreground">u</span>
              <span className="text-xs font-mono">{ctx.vec.u}</span>
            </div>
          </TreeRow>
          <TreeRow id="algorithm.details.vec.v">
            <div className="flex items-center justify-between w-full px-2 py-0.5">
              <span className="text-xs text-muted-foreground">v</span>
              <span className="text-xs font-mono">{ctx.vec.v}</span>
            </div>
          </TreeRow>
        </TreeSection>
      )}

      {/* Selection section */}
      <TreeSection id="algorithm.details.selection" label={`Selection (${selectedPieces.length})`} icon={<PieceIcon size={14} />} defaultOpen={true}>
        {selectedPieces.length === 0 ? (
          <TreeRow id="algorithm.details.selection.empty">
            <div className="px-2 py-1 text-xs text-muted-foreground italic">No pieces selected</div>
          </TreeRow>
        ) : (
          selectedPieces.map((piece) => (
            <TreeRow key={piece.guid} id={`algorithm.details.selection.${piece.guid}`}>
              <div className="flex items-center justify-between w-full px-2 py-0.5">
                <span className="text-xs truncate max-w-24">{piece.name ?? piece.guid.slice(0, 8)}</span>
                <span className="text-xs text-muted-foreground font-mono">{piece.type?.guid.slice(0, 8) ?? "—"}</span>
              </div>
            </TreeRow>
          ))
        )}
      </TreeSection>

      {/* Output section */}
      <TreeSection id="algorithm.details.output" label="Output" icon={<DetailsIcon size={14} />} defaultOpen={true}>
        <TreeRow id="algorithm.details.output.status">
          <div className="flex items-center justify-between w-full px-2 py-0.5">
            <span className="text-xs text-muted-foreground">status</span>
            <span className={cn("text-xs font-mono", ctx.error ? "text-destructive" : "text-success")}>{ctx.error ? "error" : "ok"}</span>
          </div>
        </TreeRow>
        {ctx.error && (
          <TreeRow id="algorithm.details.output.error">
            <div className="px-2 py-1 text-xs text-destructive wrap-break-word">{ctx.error}</div>
          </TreeRow>
        )}
        {ctx.designDiff && (
          <>
            <TreeRow id="algorithm.details.output.diff.added">
              <div className="flex items-center justify-between w-full px-2 py-0.5">
                <span className="text-xs text-muted-foreground">added</span>
                <span className="text-xs font-mono text-success">{ctx.designDiff.pieces?.added?.length ?? 0}</span>
              </div>
            </TreeRow>
            <TreeRow id="algorithm.details.output.diff.removed">
              <div className="flex items-center justify-between w-full px-2 py-0.5">
                <span className="text-xs text-muted-foreground">removed</span>
                <span className="text-xs font-mono text-destructive">{ctx.designDiff.pieces?.removed?.length ?? 0}</span>
              </div>
            </TreeRow>
            <TreeRow id="algorithm.details.output.diff.updated">
              <div className="flex items-center justify-between w-full px-2 py-0.5">
                <span className="text-xs text-muted-foreground">updated</span>
                <span className="text-xs font-mono text-warning">{(ctx.designDiff.pieces?.updated as any[])?.length ?? 0}</span>
              </div>
            </TreeRow>
          </>
        )}
      </TreeSection>
    </div>
  );
};

// #endregion 🔖AlgorithmDetailsPanel

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

  const pieceCount = context.design?.pieces?.length ?? 0;

  const footerItems: FooterItem[] = React.useMemo(
    () => [
      {
        id: `${id}.footer.pieces`,
        icon: <PieceIcon size={12} />,
        text: `${context.selectedPieceGuids.length}/${pieceCount}`,
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
    [id, context.selectedPieceGuids.length, pieceCount, context.error],
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
    <AlgorithmContext.Provider value={context}>
      <div className={className ?? "h-full w-full"}>
        <UI apps={apps} defaultAppId={id} />
      </div>
    </AlgorithmContext.Provider>
  );
};

// #endregion 🔖AlgorithmApp

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
      expect(isAlgorithmWindowKind(WindowKind.PIECES_SELECTION_INPUT)).toBe(true);
      expect(isAlgorithmWindowKind(WindowKind.DESIGN_DIFF_OUTPUT)).toBe(true);
      expect(isAlgorithmWindowKind(WindowKind.DESIGN_OUTPUT)).toBe(true);
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
    });

    it("maps algorithm selection and output windows to shared semio/ui components", () => {
      expect(getAlgorithmWindowBehavior(WindowKind.VEC_INPUT)?.component).toBe(Vec);
      expect(getAlgorithmWindowBehavior(WindowKind.PIECES_SELECTION_INPUT)?.component).toBe(PieceSelection);
      expect(getAlgorithmWindowBehavior(WindowKind.DESIGN_DIFF_OUTPUT)?.component).toBe(SemioDiagram);
      expect(getAlgorithmWindowBehavior(WindowKind.DESIGN_OUTPUT)?.component).toBe(SemioDiagram);
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
            { kind: "stack", children: [{ kind: "window", windowKindId: "drag-vec", title: "Vec" }] },
            { kind: "stack", children: [{ kind: "window", windowKindId: "drag-input", title: "Input" }] },
            { kind: "stack", children: [{ kind: "window", windowKindId: "drag-diff", title: "Diff" }] },
            { kind: "stack", children: [{ kind: "window", windowKindId: "drag-output", title: "Output" }] },
          ],
        },
      });
    });
  });
}
