
import type { Connector, Port } from "../semio";
import { arePortsCompatible } from "../semio";

export type PortCompatibilityState = "none" | "selected" | "compatible" | "incompatible";

export type PortTone = {
  base: string;
  surface: string;
  surfaceStrong: string;
  border: string;
  text: string;
};

const DEFAULT_PORT_GUID = "__default__";

const normalizeGuid = (value: string | undefined | null): string | undefined => {
  if (!value) return undefined;
  const normalized = value.trim();
  return normalized.length > 0 ? normalized : undefined;
};

const normalizePortRef = (value: unknown): string | undefined => {
  if (typeof value === "string") return normalizeGuid(value);
  if (value && typeof value === "object" && "guid" in (value as Record<string, unknown>)) {
    const guid = (value as { guid?: string }).guid;
    return normalizeGuid(guid);
  }
  return undefined;
};

const hashString = (input: string): number => {
  let hash = 0;
  for (let index = 0; index < input.length; index += 1) {
    hash = (hash << 5) - hash + input.charCodeAt(index);
    hash |= 0;
  }
  return Math.abs(hash);
};

const getToneForKey = (key: string): PortTone => {
  if (key === DEFAULT_PORT_GUID) {
    return {
      base: "hsl(0 0% 48%)",
      surface: "hsla(0 0% 48% / 0.22)",
      surfaceStrong: "hsla(0 0% 48% / 0.35)",
      border: "hsl(0 0% 34%)",
      text: "hsl(0 0% 98%)",
    };
  }

  const hash = hashString(key);
  const hue = hash % 360;
  const saturation = 66;
  const lightness = 52;
  return {
    base: `hsl(${hue} ${saturation}% ${lightness}%)`,
    surface: `hsla(${hue} ${saturation}% ${lightness}% / 0.22)`,
    surfaceStrong: `hsla(${hue} ${saturation}% ${Math.min(lightness + 6, 62)}% / 0.38)`,
    border: `hsl(${hue} ${Math.max(saturation - 10, 46)}% ${Math.max(lightness - 14, 30)}%)`,
    text: "hsl(0 0% 100%)",
  };
};

const createPortGroupMap = (ports: Port[]): Map<string, string> => {
  const parent = new Map<string, string>();

  for (const port of ports) {
    const guid = normalizeGuid(port.guid);
    if (!guid) continue;
    parent.set(guid, guid);
  }

  const find = (guid: string): string => {
    const direct = parent.get(guid);
    if (!direct) return guid;
    if (direct === guid) return direct;
    const root = find(direct);
    parent.set(guid, root);
    return root;
  };

  const union = (left: string, right: string) => {
    const leftRoot = find(left);
    const rightRoot = find(right);
    if (leftRoot === rightRoot) return;
    const leftHash = hashString(leftRoot);
    const rightHash = hashString(rightRoot);
    if (leftHash <= rightHash) parent.set(rightRoot, leftRoot);
    else parent.set(leftRoot, rightRoot);
  };

  for (const port of ports) {
    const guid = normalizeGuid(port.guid);
    if (!guid) continue;
    const compatible = port.compatiblePorts ?? [];
    for (const relatedPort of compatible) {
      const relatedGuid = normalizeGuid(relatedPort.guid);
      if (!relatedGuid || !parent.has(relatedGuid)) continue;
      union(guid, relatedGuid);
    }
  }

  const groups = new Map<string, string>();
  for (const guid of parent.keys()) {
    groups.set(guid, find(guid));
  }
  return groups;
};

export const getPortGuid = (value: unknown): string | undefined => normalizePortRef(value);

export const getConnectorPortGuid = (connector: Pick<Connector, "port"> | undefined | null): string | undefined => normalizePortRef(connector?.port);

export const getPortTone = (portGuid: string | undefined, ports: Port[]): PortTone => {
  const normalizedGuid = normalizeGuid(portGuid);
  if (!normalizedGuid) return getToneForKey(DEFAULT_PORT_GUID);
  const groups = createPortGroupMap(ports);
  const groupKey = groups.get(normalizedGuid) ?? normalizedGuid;
  return getToneForKey(groupKey);
};

export const getPortCompatibilityState = (candidatePortGuid: string | undefined, selectedPortGuid: string | undefined, ports: Port[]): PortCompatibilityState => {
  const normalizedCandidate = normalizeGuid(candidatePortGuid);
  const normalizedSelected = normalizeGuid(selectedPortGuid);
  if (!normalizedSelected) return "none";
  if (normalizedCandidate === normalizedSelected) return "selected";
  if (!normalizedCandidate || !normalizedSelected) return "compatible";
  const candidatePort = ports.find((port) => normalizeGuid(port.guid) === normalizedCandidate);
  const selectedPort = ports.find((port) => normalizeGuid(port.guid) === normalizedSelected);
  if (!candidatePort || !selectedPort) return "incompatible";
  return arePortsCompatible(candidatePort, selectedPort, ports) ? "compatible" : "incompatible";
};
