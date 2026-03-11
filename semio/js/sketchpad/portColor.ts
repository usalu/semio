// SPDX-License-Identifier: LGPL-3.0-or-later

// 2026 Ueli Saluz <ueli@semio-tech.de>

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

// Color mapping utilities for port visualization in diagrams.

// #endregion 🔖Header

// #region 🔖Port Color
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color)
// Assigns deterministic HSL color tones to ports based on compatibility groups.
// MUST use a union-find structure to group compatible ports under a single color.

import type { Connector, Port } from "../semio";
import { arePortsCompatible } from "../semio";

/**
 * Compatibility state of a port relative to the selected port.
 *
 * MUST be one of none, selected, compatible, or incompatible.
 *
 *  * [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🛠️portcompatibilitystate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/PortCompatibilityState)
 **/
export type PortCompatibilityState = "none" | "selected" | "compatible" | "incompatible";

/**
 * HSL color tones for rendering a port in the UI.
 *
 * MUST contain base, surface, surfaceStrong, border and text values.
 *
 *  * [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🛠️porttone](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/PortTone)
 **/
export type PortTone = {
  base: string;
  surface: string;
  surfaceStrong: string;
  border: string;
  text: string;
};

/**
 * Sentinel GUID for ports without an assigned identity.
 *
 * MUST be used as the fallback key for tone generation.
 **/
/**
 * DEFAULT_PORT_GUID holds the data fields for a DEFAULT_PORT_GUID record.
 **/
/**
 * DEFAULT_PORT_GUID holds the data fields for a DEFAULT_PORT_GUID record.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨defaultportguid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/DEFAULT_PORT_GUID)
const DEFAULT_PORT_GUID = "__default__";

/**
 * Trims and normalizes a GUID string, returning undefined for empty values.
 *
 * MUST return undefined for null, undefined, or whitespace-only input.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨normalizeguid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/normalizeGuid)
 * normalizeGuid holds the data fields for a normalizeGuid record.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨normalizeguid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/normalizeGuid)
 * normalizeGuid holds the data fields for a normalizeGuid record.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨normalizeguid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/normalizeGuid)
 * normalizeGuid holds the data fields for a normalizeGuid record.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨normalizeguid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/normalizeGuid)
const normalizeGuid = (value: string | undefined | null): string | undefined => {
  if (!value) return undefined;
  const normalized = value.trim();
  return normalized.length > 0 ? normalized : undefined;
};

/**
 * Extracts a GUID from a string or object with a guid property.
 *
 * MUST handle both direct string GUIDs and port reference objects.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨normalizeportref](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/normalizePortRef)
 * normalizePortRef holds the data fields for a normalizePortRef record.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨normalizeportref](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/normalizePortRef)
 * normalizePortRef holds the data fields for a normalizePortRef record.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨normalizeportref](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/normalizePortRef)
 * normalizePortRef holds the data fields for a normalizePortRef record.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨normalizeportref](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/normalizePortRef)
const normalizePortRef = (value: unknown): string | undefined => {
  if (typeof value === "string") return normalizeGuid(value);
  if (value && typeof value === "object" && "guid" in (value as Record<string, unknown>)) {
    const guid = (value as { guid?: string }).guid;
    return normalizeGuid(guid);
  }
  return undefined;
};

/**
 * Produces a deterministic non-negative integer hash from a string.
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🛠️hashstring](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/hashString)
 *
 * MUST return the absolute value of a 32-bit hash.
 **/
const hashString = (input: string): number => {
  let hash = 0;
  for (let index = 0; index < input.length; index += 1) {
    hash = (hash << 5) - hash + input.charCodeAt(index);
    hash |= 0;
  }
  return Math.abs(hash);
};

/**
 * Generates an HSL color tone from a port group key.
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🛠️gettoneforkey](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/getToneForKey)
 *
 * MUST return a neutral grey tone for the default port GUID.
 **/
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

/**
 * Builds a union-find map grouping compatible ports by root GUID.
 *
 * MUST union ports linked via compatiblePorts relationships.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨createportgroupmap](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/createPortGroupMap)
 * createPortGroupMap holds the data fields for a createPortGroupMap record.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨createportgroupmap](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/createPortGroupMap)
 * createPortGroupMap holds the data fields for a createPortGroupMap record.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨createportgroupmap](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/createPortGroupMap)
 * createPortGroupMap holds the data fields for a createPortGroupMap record.
 **/
// [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨createportgroupmap](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/createPortGroupMap)
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

/**
 * Extracts a normalized port GUID from a string or port reference object.
 *
 * MUST delegate to normalizePortRef for consistent handling.
 *
 *  * [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨getportguid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/getPortGuid)
 **/
export const getPortGuid = (value: unknown): string | undefined => normalizePortRef(value);

/**
 * Extracts the port GUID from a connector's port reference.
 *
 * MUST return undefined when the connector or its port is missing.
 *
 *  * [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨getconnectorportguid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/getConnectorPortGuid)
 **/
export const getConnectorPortGuid = (connector: Pick<Connector, "port"> | undefined | null): string | undefined => normalizePortRef(connector?.port);

/**
 * Resolves the color tone for a port based on its compatibility group.
 *
 * MUST return the default tone when the port GUID is missing.
 *
 *  * [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨getporttone](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/getPortTone)
 **/
export const getPortTone = (portGuid: string | undefined, ports: Port[]): PortTone => {
  const normalizedGuid = normalizeGuid(portGuid);
  if (!normalizedGuid) return getToneForKey(DEFAULT_PORT_GUID);
  const groups = createPortGroupMap(ports);
  const groupKey = groups.get(normalizedGuid) ?? normalizedGuid;
  return getToneForKey(groupKey);
};

/**
 * Determines the compatibility state of a candidate port relative to a selected port.
 *
 * MUST return none when no port is selected.
 *
 *  * [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor🪨getportcompatibilitystate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color/d/i/getPortCompatibilityState)
 **/
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

// #endregion 🔖Port Color
