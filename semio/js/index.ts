// #region 🔖Header
// [👤semio📚js💻index](repo://p/u/semio/b/l/js/f/index.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// Core domain model types, schemas and utilities for the semio platform.

// #endregion 🔖Header

// #region 🔖Imports
// [👤semio📚js💻semio🔖imports](repo://p/u/semio/b/l/js/f/semio.ts/s/Imports)
// External dependency imports MUST be declared here.
import { default as adjectives } from "@semio/assets/lists/adjectives.json" with { type: "json" };
import { default as animals } from "@semio/assets/lists/animals.json" with { type: "json" };
import { ClassValue, clsx } from "clsx";
import cytoscape from "cytoscape";
import { twMerge } from "tailwind-merge";
import * as THREE from "three";
import {
  Document as GltfDocument,
  NodeIO,
  Accessor as GltfAccessor,
  Primitive as GltfPrimitive,
  Buffer as GltfBuffer,
  Mesh as GltfMesh,
  Node as GltfNode,
  Scene as GltfScene,
  Material as GltfMaterial,
  Texture as GltfTexture,
} from "@gltf-transform/core";
import { v7 as uuidv7 } from "uuid";
import { z } from "zod";

// #endregion 🔖Imports

// #region 🔖Constants
// [👤semio📚js💻semio🔖constants](repo://p/u/semio/b/l/js/f/semio.ts/s/Constants)
// Global constants MUST define shared numeric parameters.

/** Standard icon width in pixels.
 * [👤semio📚js💻semio🔖constants🪨iconwidth](repo://p/u/semio/b/l/js/f/semio.ts/s/Constants/d/i/ICON_WIDTH)
 **/
export const ICON_WIDTH = 50;
/**
 * Numeric tolerance for floating-point comparisons.
 * [👤semio📚js💻semio🔖constants🪨tolerance](repo://p/u/semio/b/l/js/f/semio.ts/s/Constants/d/i/TOLERANCE)
 **/
export const TOLERANCE = 1e-5;

// #endregion 🔖Constants

// #region 🔖Utilities
// [👤semio📚js💻semio🔖utilities](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities)
// General-purpose utility functions MUST be defined here.

/**
 * Performs the cn operation.
 * MUST merge CSS class names using Tailwind merge.
 * [👤semio📚js💻semio🔖utilities🛠️cn](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/cn)
 **/
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * Performs the guid operation.
 * MUST return a new UUID v7 string.
 * [👤semio📚js💻semio🔖utilities🪨guid](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/guid)
 **/
export const guid = () => uuidv7();

// [👤semio📚js💻semio🔖utilities🛠️seededrandom](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/SeededRandom)
// SeededRandom holds the data fields for a SeededRandom record.
class SeededRandom {
  private seed: number;
  constructor(seed: number) {
    this.seed = seed % 2147483647;
    if (this.seed <= 0) this.seed += 2147483646;
  }
  next = (): number => (this.seed = (this.seed * 16807) % 2147483647);
  nextFloat = (): number => (this.next() - 1) / 2147483646;
  nextInt = (max: number): number => Math.floor(this.nextFloat() * max);
}

/**
 * Class implementing Generator behavior.
 * MUST provide the declared public interface.
 * [👤semio📚js💻semio🔖utilities🛠️generator](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/Generator)
 **/
export class Generator {
  public static randomId(seed: number = Math.floor(Math.random() * 1000000)): string {
    const random = new SeededRandom(seed);
    let adjective = adjectives[random.nextInt(adjectives.length)];
    let animal = animals[random.nextInt(animals.length)];
    adjective = adjective.charAt(0).toUpperCase() + adjective.slice(1);
    animal = animal.charAt(0).toUpperCase() + animal.slice(1);
    return `${adjective}${animal}${random.nextInt(1000)}`;
  }
  public static randomName(seed: number = Math.floor(Math.random() * 1000000)): string {
    const random = new SeededRandom(seed);
    let animal = animals[random.nextInt(animals.length)];
    animal = animal.charAt(0).toUpperCase() + animal.slice(1);
    return `${animal}`;
  }
}

/**
 * Performs the normalize operation.
 * MUST return empty string for null or undefined.
 * [👤semio📚js💻semio🔖utilities🪨normalize](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/normalize)
 **/
export const normalize = (val: string | undefined | null): string => (val === undefined || val === null ? "" : val);
/**
 * Performs the round operation.
 * MUST round to the nearest tolerance unit.
 * [👤semio📚js💻semio🔖utilities🪨round](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/round)
 **/
export const round = (value: number): number => Math.round(value / TOLERANCE) * TOLERANCE;
/**
 * Performs the jaccard operation.
 * MUST compute the Jaccard similarity coefficient.
 * [👤semio📚js💻semio🔖utilities🪨jaccard](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/jaccard)
 **/
export const jaccard = (a: string[] | undefined, b: string[] | undefined): number => {
  if ((a === undefined && b === undefined) || (a?.length === 0 && b?.length === 0)) return 1;
  if (a === undefined || b === undefined) return 0;
  const setA = new Set(a);
  const setB = new Set(b);
  const intersection = Array.from(setA).filter((x) => setB.has(x)).length;
  const union = setA.size + setB.size - intersection;
  if (union === 0) return 0;
  return intersection / union;
};

/**
 * Performs the deepEqual operation.
 * MUST recursively compare values for equality.
 * [👤semio📚js💻semio🔖utilities🪨deepequal](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/deepEqual)
 **/
export const deepEqual = (a: any, b: any): boolean => {
  if (a === b) return true;

  if (a == null && b == null) return true;
  if (a == null || b == null) return false;
  if (typeof a !== typeof b) return false;

  if (Array.isArray(a)) {
    if (!Array.isArray(b) || a.length !== b.length) return false;
    return a.every((item, index) => deepEqual(item, b[index]));
  }

  if (typeof a === "object") {
    const keysA = Object.keys(a);
    const keysB = Object.keys(b);
    if (keysA.length !== keysB.length) return false;
    return keysA.every((key) => keysB.includes(key) && deepEqual(a[key], b[key]));
  }

  return false;
};

/**
 * Performs the arraysEqual operation.
 * MUST compare arrays element by element.
 * [👤semio📚js💻semio🔖utilities🪨arraysequal](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/arraysEqual)
 **/
export const arraysEqual = <T>(a: T[] | undefined, b: T[] | undefined): boolean => {
  if (a === b) return true;
  if (!a || !b) return false;
  return a.length === b.length && a.every((val, index) => deepEqual(val, b[index]));
};

/**
 * Performs the generateUniqueName operation.
 * MUST return a name not in the existing set.
 * [👤semio📚js💻semio🔖utilities🪨generateuniquename](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/generateUniqueName)
 **/
export const generateUniqueName = (baseName: string, existingNames: string[], separator: string = " "): string => {
  if (!existingNames.includes(baseName)) return baseName;
  let counter = 2;
  while (existingNames.includes(`${baseName}${separator}${counter}`)) {
    counter++;
  }
  return `${baseName}${separator}${counter}`;
};

/**
 * Zod schema for DiffStatus validation.
 * [👤semio📚js💻semio🔖utilities🪨diffstatusschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/DiffStatusSchema)
 **/
export const DiffStatusSchema = z.enum(["unchanged", "added", "removed", "modified"]);

/**
 * Enumeration of DiffStatus values.
 * [👤semio📚js💻semio🔖utilities🛠️diffstatus](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/DiffStatus)
 **/
export enum DiffStatus {
  Unchanged = "unchanged",
  Added = "added",
  Removed = "removed",
  Modified = "modified",
}

/**
 * Converts to ThreeRotation representation.
 * MUST convert to the target representation.
 * [👤semio📚js💻semio🔖utilities🪨tothreerotation](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/toThreeRotation)
 **/
export const toThreeRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);

/**
 * Converts to SemioRotation representation.
 * MUST convert to the target representation.
 * [👤semio📚js💻semio🔖utilities🪨tosemiorotation](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/toSemioRotation)
 **/
export const toSemioRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1);
/**
 * Converts to ThreeQuaternion representation.
 * MUST convert to the target representation.
 * [👤semio📚js💻semio🔖utilities🪨tothreequaternion](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/toThreeQuaternion)
 **/
export const toThreeQuaternion = (): THREE.Quaternion => new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476);
/**
 * Converts to SemioQuaternion representation.
 * MUST convert to the target representation.
 * [👤semio📚js💻semio🔖utilities🪨tosemioquaternion](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/toSemioQuaternion)
 **/
export const toSemioQuaternion = (): THREE.Quaternion => new THREE.Quaternion(0.7071067811865476, 0, 0, -0.7071067811865476);
/**
 * Performs the vectorToThree operation.
 * MUST convert semio vector to Three.js vector.
 * [👤semio📚js💻semio🔖utilities🪨vectortothree](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/vectorToThree)
 **/
export const vectorToThree = (v: Point | Vector): THREE.Vector3 => new THREE.Vector3(v.x, v.y, v.z);

/**
 * Type alias for Guid.
 * [👤semio📚js💻semio🔖utilities🛠️guid](repo://p/u/semio/b/l/js/f/semio.ts/s/Utilities/d/i/Guid)
 **/
export type Guid = string;

// #endregion 🔖Utilities
// #region 🔖Entity IDs

// [👤semio📚js💻semio🔖entityids](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs)
// Entity identifier types and comparison functions MUST be defined here.

/**
 * Identifier type for Attribute entities.
 * [👤semio📚js💻semio🔖entityids🛠️attributeid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/AttributeId)
 **/
export type AttributeId = { guid: Guid };
/**
 * Identifier type for Location entities.
 * [👤semio📚js💻semio🔖entityids🛠️locationid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/LocationId)
 **/
export type LocationId = { guid: Guid };
/**
 * Identifier type for Author entities.
 * [👤semio📚js💻semio🔖entityids🛠️authorid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/AuthorId)
 **/
export type AuthorId = { guid: Guid };
/**
 * Identifier type for File entities.
 * [👤semio📚js💻semio🔖entityids🛠️fileid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/FileId)
 **/
export type FileId = { guid: Guid };
/**
 * Identifier type for Folder entities.
 * [👤semio📚js💻semio🔖entityids🛠️folderid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/FolderId)
 **/
export type FolderId = { guid: Guid };
/**
 * Identifier type for Benchmark entities.
 * [👤semio📚js💻semio🔖entityids🛠️benchmarkid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/BenchmarkId)
 **/
export type BenchmarkId = { guid: Guid };
/**
 * Identifier type for Quality entities.
 * [👤semio📚js💻semio🔖entityids🛠️qualityid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/QualityId)
 **/
export type QualityId = { guid: Guid };
/**
 * Identifier type for Port entities.
 * [👤semio📚js💻semio🔖entityids🛠️portid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/PortId)
 **/
export type PortId = { guid: Guid };
/**
 * Identifier type for Prop entities.
 * [👤semio📚js💻semio🔖entityids🛠️propid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/PropId)
 **/
export type PropId = { guid: Guid };
/**
 * Identifier type for Model entities.
 * [👤semio📚js💻semio🔖entityids🛠️modelid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/ModelId)
 **/
export type ModelId = { guid: Guid };
/**
 * Identifier type for Connector entities.
 * [👤semio📚js💻semio🔖entityids🛠️connectorid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/ConnectorId)
 **/
export type ConnectorId = { guid: Guid };
/**
 * Identifier type for Type entities.
 * [👤semio📚js💻semio🔖entityids🛠️typeid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/TypeId)
 **/
export type TypeId = { guid: Guid };
/**
 * Identifier type for Layer entities.
 * [👤semio📚js💻semio🔖entityids🛠️layerid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/LayerId)
 **/
export type LayerId = { guid: Guid };
/**
 * Identifier type for Piece entities.
 * [👤semio📚js💻semio🔖entityids🛠️pieceid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/PieceId)
 **/
export type PieceId = { guid: Guid };
/**
 * Identifier type for Group entities.
 * [👤semio📚js💻semio🔖entityids🛠️groupid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/GroupId)
 **/
export type GroupId = { guid: Guid };
/**
 * Identifier type for Connection entities.
 * [👤semio📚js💻semio🔖entityids🛠️connectionid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/ConnectionId)
 **/
export type ConnectionId = { guid: Guid };
/**
 * Identifier type for Stat entities.
 * [👤semio📚js💻semio🔖entityids🛠️statid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/StatId)
 **/
export type StatId = { guid: Guid };
/**
 * Identifier type for Design entities.
 * [👤semio📚js💻semio🔖entityids🛠️designid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/DesignId)
 **/
export type DesignId = { guid: Guid };
/**
 * Identifier type for Kit entities.
 * [👤semio📚js💻semio🔖entityids🛠️kitid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/KitId)
 **/
export type KitId = { guid: Guid };
/**
 * Identifier type for Tag entities.
 * [👤semio📚js💻semio🔖entityids🛠️tagid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/TagId)
 **/
export type TagId = { guid: Guid };
/**
 * Identifier type for Concept entities.
 * [👤semio📚js💻semio🔖entityids🛠️conceptid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/ConceptId)
 **/
export type ConceptId = { guid: Guid };

/**
 * Zod schema for validating Attribute identifiers.
 * [👤semio📚js💻semio🔖entityids🪨attributeidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/AttributeIdSchema)
 **/
export const AttributeIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Location identifiers.
 * [👤semio📚js💻semio🔖entityids🪨locationidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/LocationIdSchema)
 **/
export const LocationIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Author identifiers.
 * [👤semio📚js💻semio🔖entityids🪨authoridschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/AuthorIdSchema)
 **/
export const AuthorIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating File identifiers.
 * [👤semio📚js💻semio🔖entityids🪨fileidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/FileIdSchema)
 **/
export const FileIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Folder identifiers.
 * [👤semio📚js💻semio🔖entityids🪨folderidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/FolderIdSchema)
 **/
export const FolderIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Benchmark identifiers.
 * [👤semio📚js💻semio🔖entityids🪨benchmarkidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/BenchmarkIdSchema)
 **/
export const BenchmarkIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Quality identifiers.
 * [👤semio📚js💻semio🔖entityids🪨qualityidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/QualityIdSchema)
 **/
export const QualityIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Port identifiers.
 * [👤semio📚js💻semio🔖entityids🪨portidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/PortIdSchema)
 **/
export const PortIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Prop identifiers.
 * [👤semio📚js💻semio🔖entityids🪨propidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/PropIdSchema)
 **/
export const PropIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Model identifiers.
 * [👤semio📚js💻semio🔖entityids🪨modelidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/ModelIdSchema)
 **/
export const ModelIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Connector identifiers.
 * [👤semio📚js💻semio🔖entityids🪨connectoridschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/ConnectorIdSchema)
 **/
export const ConnectorIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Type identifiers.
 * [👤semio📚js💻semio🔖entityids🪨typeidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/TypeIdSchema)
 **/
export const TypeIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Layer identifiers.
 * [👤semio📚js💻semio🔖entityids🪨layeridschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/LayerIdSchema)
 **/
export const LayerIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Piece identifiers.
 * [👤semio📚js💻semio🔖entityids🪨pieceidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/PieceIdSchema)
 **/
export const PieceIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Group identifiers.
 * [👤semio📚js💻semio🔖entityids🪨groupidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/GroupIdSchema)
 **/
export const GroupIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Connection identifiers.
 * [👤semio📚js💻semio🔖entityids🪨connectionidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/ConnectionIdSchema)
 **/
export const ConnectionIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Stat identifiers.
 * [👤semio📚js💻semio🔖entityids🪨statidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/StatIdSchema)
 **/
export const StatIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Design identifiers.
 * [👤semio📚js💻semio🔖entityids🪨designidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/DesignIdSchema)
 **/
export const DesignIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Kit identifiers.
 * [👤semio📚js💻semio🔖entityids🪨kitidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/KitIdSchema)
 **/
export const KitIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Tag identifiers.
 * [👤semio📚js💻semio🔖entityids🪨tagidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/TagIdSchema)
 **/
export const TagIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Concept identifiers.
 * [👤semio📚js💻semio🔖entityids🪨conceptidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/ConceptIdSchema)
 **/
export const ConceptIdSchema = z.object({ guid: z.string() });

/**
 * Factory for creating Attribute identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createattributeid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createAttributeId)
 **/
export const createAttributeId = (guid: Guid): AttributeId => ({ guid });
/**
 * Factory for creating Location identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createlocationid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createLocationId)
 **/
export const createLocationId = (guid: Guid): LocationId => ({ guid });
/**
 * Factory for creating Author identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createauthorid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createAuthorId)
 **/
export const createAuthorId = (guid: Guid): AuthorId => ({ guid });
/**
 * Factory for creating File identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createfileid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createFileId)
 **/
export const createFileId = (guid: Guid): FileId => ({ guid });
/**
 * Factory for creating Folder identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createfolderid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createFolderId)
 **/
export const createFolderId = (guid: Guid): FolderId => ({ guid });
/**
 * Factory for creating Benchmark identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createbenchmarkid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createBenchmarkId)
 **/
export const createBenchmarkId = (guid: Guid): BenchmarkId => ({ guid });
/**
 * Factory for creating Quality identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createqualityid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createQualityId)
 **/
export const createQualityId = (guid: Guid): QualityId => ({ guid });
/**
 * Factory for creating Port identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createportid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createPortId)
 **/
export const createPortId = (guid: Guid): PortId => ({ guid });
/**
 * Factory for creating Prop identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createpropid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createPropId)
 **/
export const createPropId = (guid: Guid): PropId => ({ guid });
/**
 * Factory for creating Model identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createmodelid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createModelId)
 **/
export const createModelId = (guid: Guid): ModelId => ({ guid });
/**
 * Factory for creating Connector identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createconnectorid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createConnectorId)
 **/
export const createConnectorId = (guid: Guid): ConnectorId => ({ guid });
/**
 * Factory for creating Type identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createtypeid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createTypeId)
 **/
export const createTypeId = (guid: Guid): TypeId => ({ guid });
/**
 * Factory for creating Layer identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createlayerid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createLayerId)
 **/
export const createLayerId = (guid: Guid): LayerId => ({ guid });
/**
 * Factory for creating Piece identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createpieceid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createPieceId)
 **/
export const createPieceId = (guid: Guid): PieceId => ({ guid });
/**
 * Factory for creating Group identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨creategroupid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createGroupId)
 **/
export const createGroupId = (guid: Guid): GroupId => ({ guid });
/**
 * Factory for creating Connection identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createconnectionid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createConnectionId)
 **/
export const createConnectionId = (guid: Guid): ConnectionId => ({ guid });
/**
 * Factory for creating Stat identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createstatid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createStatId)
 **/
export const createStatId = (guid: Guid): StatId => ({ guid });
/**
 * Factory for creating Design identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createdesignid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createDesignId)
 **/
export const createDesignId = (guid: Guid): DesignId => ({ guid });
/**
 * Factory for creating Kit identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createkitid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createKitId)
 **/
export const createKitId = (guid: Guid): KitId => ({ guid });
/**
 * Factory for creating Tag identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createtagid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createTagId)
 **/
export const createTagId = (guid: Guid): TagId => ({ guid });
/**
 * Factory for creating Concept identifiers.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖entityids🪨createconceptid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/createConceptId)
 **/
export const createConceptId = (guid: Guid): ConceptId => ({ guid });

/**
 * Equality check for Attribute identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresameattributeid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameAttributeId)
 **/
export const areSameAttributeId = (a: AttributeId, b: AttributeId): boolean => a.guid === b.guid;
/**
 * Equality check for Location identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamelocationid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameLocationId)
 **/
export const areSameLocationId = (a: LocationId, b: LocationId): boolean => a.guid === b.guid;
/**
 * Equality check for Author identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresameauthorid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameAuthorId)
 **/
export const areSameAuthorId = (a: AuthorId, b: AuthorId): boolean => a.guid === b.guid;
/**
 * Equality check for File identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamefileid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameFileId)
 **/
export const areSameFileId = (a: FileId, b: FileId): boolean => a.guid === b.guid;
/**
 * Equality check for Folder identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamefolderid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameFolderId)
 **/
export const areSameFolderId = (a: FolderId, b: FolderId): boolean => a.guid === b.guid;
/**
 * Equality check for Benchmark identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamebenchmarkid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameBenchmarkId)
 **/
export const areSameBenchmarkId = (a: BenchmarkId, b: BenchmarkId): boolean => a.guid === b.guid;
/**
 * Equality check for Quality identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamequalityid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameQualityId)
 **/
export const areSameQualityId = (a: QualityId, b: QualityId): boolean => a.guid === b.guid;
/**
 * Equality check for Port identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresameportid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSamePortId)
 **/
export const areSamePortId = (a: PortId, b: PortId): boolean => a.guid === b.guid;
/**
 * Equality check for Prop identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamepropid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSamePropId)
 **/
export const areSamePropId = (a: PropId, b: PropId): boolean => a.guid === b.guid;
/**
 * Equality check for Model identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamemodelid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameModelId)
 **/
export const areSameModelId = (a: ModelId, b: ModelId): boolean => a.guid === b.guid;
/**
 * Equality check for Connector identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresameconnectorid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameConnectorId)
 **/
export const areSameConnectorId = (a: ConnectorId, b: ConnectorId): boolean => a.guid === b.guid;
/**
 * Equality check for Type identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresametypeid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameTypeId)
 **/
export const areSameTypeId = (a: TypeId, b: TypeId): boolean => a.guid === b.guid;
/**
 * Equality check for Layer identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamelayerid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameLayerId)
 **/
export const areSameLayerId = (a: LayerId, b: LayerId): boolean => a.guid === b.guid;
/**
 * Equality check for Piece identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamepieceid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSamePieceId)
 **/
export const areSamePieceId = (a: PieceId, b: PieceId): boolean => a.guid === b.guid;
/**
 * Equality check for Group identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamegroupid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameGroupId)
 **/
export const areSameGroupId = (a: GroupId, b: GroupId): boolean => a.guid === b.guid;
/**
 * Equality check for Connection identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresameconnectionid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameConnectionId)
 **/
export const areSameConnectionId = (a: ConnectionId, b: ConnectionId): boolean => a.guid === b.guid;
/**
 * Equality check for Stat identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamestatid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameStatId)
 **/
export const areSameStatId = (a: StatId, b: StatId): boolean => a.guid === b.guid;
/**
 * Equality check for Design identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamedesignid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameDesignId)
 **/
export const areSameDesignId = (a: DesignId, b: DesignId): boolean => a.guid === b.guid;
/**
 * Equality check for Kit identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresamekitid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameKitId)
 **/
export const areSameKitId = (a: KitId, b: KitId): boolean => a.guid === b.guid;
/**
 * Equality check for Tag identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresametagid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameTagId)
 **/
export const areSameTagId = (a: TagId, b: TagId): boolean => a.guid === b.guid;
/**
 * Equality check for Concept identifiers.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖entityids🪨aresameconceptid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/areSameConceptId)
 **/
export const areSameConceptId = (a: ConceptId, b: ConceptId): boolean => a.guid === b.guid;

/**
 * Extracts the GUID from a Attribute identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getattributeguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getAttributeGuid)
 **/
export const getAttributeGuid = (id: AttributeId): Guid => id.guid;
/**
 * Extracts the GUID from a Location identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getlocationguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getLocationGuid)
 **/
export const getLocationGuid = (id: LocationId): Guid => id.guid;
/**
 * Extracts the GUID from a Author identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getauthorguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getAuthorGuid)
 **/
export const getAuthorGuid = (id: AuthorId): Guid => id.guid;
/**
 * Extracts the GUID from a File identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getfileguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getFileGuid)
 **/
export const getFileGuid = (id: FileId): Guid => id.guid;
/**
 * Extracts the GUID from a Folder identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getfolderguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getFolderGuid)
 **/
export const getFolderGuid = (id: FolderId): Guid => id.guid;
/**
 * Extracts the GUID from a Benchmark identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getbenchmarkguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getBenchmarkGuid)
 **/
export const getBenchmarkGuid = (id: BenchmarkId): Guid => id.guid;
/**
 * Extracts the GUID from a Quality identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getqualityguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getQualityGuid)
 **/
export const getQualityGuid = (id: QualityId): Guid => id.guid;
/**
 * Extracts the GUID from a Port identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getportguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getPortGuid)
 **/
export const getPortGuid = (id: PortId): Guid => id.guid;
/**
 * Extracts the GUID from a Prop identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getpropguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getPropGuid)
 **/
export const getPropGuid = (id: PropId): Guid => id.guid;
/**
 * Extracts the GUID from a Model identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getmodelguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getModelGuid)
 **/
export const getModelGuid = (id: ModelId): Guid => id.guid;
/**
 * Extracts the GUID from a Connector identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getconnectorguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getConnectorGuid)
 **/
export const getConnectorGuid = (id: ConnectorId): Guid => id.guid;
/**
 * Extracts the GUID from a Type identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨gettypeguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getTypeGuid)
 **/
export const getTypeGuid = (id: TypeId): Guid => id.guid;
/**
 * Extracts the GUID from a Layer identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getlayerguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getLayerGuid)
 **/
export const getLayerGuid = (id: LayerId): Guid => id.guid;
/**
 * Extracts the GUID from a Piece identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getpieceguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getPieceGuid)
 **/
export const getPieceGuid = (id: PieceId): Guid => id.guid;
/**
 * Extracts the GUID from a Group identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getgroupguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getGroupGuid)
 **/
export const getGroupGuid = (id: GroupId): Guid => id.guid;
/**
 * Extracts the GUID from a Connection identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getconnectionguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getConnectionGuid)
 **/
export const getConnectionGuid = (id: ConnectionId): Guid => id.guid;
/**
 * Extracts the GUID from a Stat identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getstatguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getStatGuid)
 **/
export const getStatGuid = (id: StatId): Guid => id.guid;
/**
 * Extracts the GUID from a Design identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getdesignguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getDesignGuid)
 **/
export const getDesignGuid = (id: DesignId): Guid => id.guid;
/**
 * Extracts the GUID from a Kit identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getkitguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getKitGuid)
 **/
export const getKitGuid = (id: KitId): Guid => id.guid;
/**
 * Extracts the GUID from a Tag identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨gettagguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getTagGuid)
 **/
export const getTagGuid = (id: TagId): Guid => id.guid;
/**
 * Extracts the GUID from a Concept identifier.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖entityids🪨getconceptguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs/d/i/getConceptGuid)
 **/
export const getConceptGuid = (id: ConceptId): Guid => id.guid;

// #endregion 🔖Entity IDs
// #region 🔖Attribute

// [👤semio📚js💻semio🔖attribute](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute)
// Attribute entity types, schemas, and helper functions MUST be defined here.

// [👤semio📚js💻semio🔖attribute🪨dateproperty](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/DateProperty)
// DateProperty holds the data fields for a DateProperty record.
const DateProperty = () => z.string().optional();

/**
 * Zod schema for Attribute validation.
 * [👤semio📚js💻semio🔖attribute🪨attributeschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/AttributeSchema)
 **/
export const AttributeSchema = z.object({
  guid: z.string(),
  key: z.string(),
  value: z.string().optional(),
  definition: z.string().optional(),
});
/**
 * Type alias for Attribute.
 * [👤semio📚js💻semio🔖attribute🛠️attribute](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/Attribute)
 **/
export type Attribute = z.infer<typeof AttributeSchema>;
/**
 * Serializes Attribute for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖attribute🪨serializeattribute](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/serializeAttribute)
 **/
export const serializeAttribute = (attribute: Attribute): string => JSON.stringify(AttributeSchema.parse(attribute));
/**
 * Performs the deserializeAttribute operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖attribute🪨deserializeattribute](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/deserializeAttribute)
 **/
export const deserializeAttribute = (json: string): Attribute => AttributeSchema.parse(JSON.parse(json));

/**
 * Zod schema for Attribute diff validation.
 * [👤semio📚js💻semio🔖attribute🪨attributediffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/AttributeDiffSchema)
 **/
export const AttributeDiffSchema = AttributeSchema.partial();
/**
 * Diff type for tracking Attribute changes.
 * [👤semio📚js💻semio🔖attribute🛠️attributediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/AttributeDiff)
 **/
export type AttributeDiff = z.infer<typeof AttributeDiffSchema>;
/**
 * Retrieves the AttributeDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖attribute🪨getattributediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/getAttributeDiff)
 **/
export const getAttributeDiff = (before: Attribute, after: Attribute): AttributeDiff => {
  const diff: AttributeDiff = {};
  if (before.key !== after.key) diff.key = after.key;
  if (before.value !== after.value) diff.value = after.value;
  if (before.definition !== after.definition) diff.definition = after.definition;
  return diff;
};
/**
 * Diff type for tracking inverseAttribute changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖attribute🪨inverseattributediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/inverseAttributeDiff)
 **/
export const inverseAttributeDiff = (original: Attribute, appliedDiff: AttributeDiff): AttributeDiff => {
  return {
    key: appliedDiff.key ? original.key : "",
    value: appliedDiff.value ? original.value : "",
    definition: appliedDiff.definition ? original.definition : "",
  };
};
/**
 * Diff type for tracking mergeAttribute changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖attribute🪨mergeattributediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/mergeAttributeDiff)
 **/
export const mergeAttributeDiff = (diff1: AttributeDiff, diff2: AttributeDiff): AttributeDiff => {
  return {
    key: diff2.key ?? diff1.key,
    value: diff2.value ?? diff1.value,
    definition: diff2.definition ?? diff1.definition,
  };
};
/**
 * Diff type for tracking applyAttribute changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖attribute🪨applyattributediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/applyAttributeDiff)
 **/
export const applyAttributeDiff = (base: Attribute, diff: AttributeDiff): Attribute => {
  return { ...base, ...diff };
};

/**
 * Zod schema for Attributes diff validation.
 * [👤semio📚js💻semio🔖attribute🪨attributesdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/AttributesDiffSchema)
 **/
export const AttributesDiffSchema = z.object({
  removed: z.array(AttributeIdSchema).optional(),
  updated: z.array(z.object({ attribute: AttributeIdSchema, diff: AttributeDiffSchema })).optional(),
  added: z.array(AttributeSchema).optional(),
});
/**
 * Diff type for tracking Attributes changes.
 * [👤semio📚js💻semio🔖attribute🛠️attributesdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/AttributesDiff)
 **/
export type AttributesDiff = z.infer<typeof AttributesDiffSchema>;

// getAttributesDiff holds the data fields for a getAttributesDiff record.
// [👤semio📚js💻semio🔖attribute🪨getattributesdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/getAttributesDiff)
const getAttributesDiff = (before: Attribute[], after: Attribute[]): AttributesDiff => {
  const beforeGuids = new Set(before.map((a) => a.guid));
  const afterGuids = new Set(after.map((a) => a.guid));
  const removed = before.filter((a) => !afterGuids.has(a.guid)).map((a) => ({ guid: a.guid }));
  const added = after.filter((a) => !beforeGuids.has(a.guid));
  const updated = after
    .filter((a) => beforeGuids.has(a.guid))
    .map((a) => ({ attribute: { guid: a.guid }, diff: getAttributeDiff(before.find((b) => b.guid === a.guid)!, a) }))
    .filter((u) => Object.keys(u.diff).length > 0);
  const diff: AttributesDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

/**
 * Diff type for tracking inverseAttributes changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖attribute🪨inverseattributesdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/inverseAttributesDiff)
 **/
export const inverseAttributesDiff = (original: Attribute[], appliedDiff: AttributesDiff): AttributesDiff => {
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  const updatedGuids = appliedDiff.updated?.map((a) => a.attribute.guid) ?? [];
  const addedGuids = appliedDiff.added?.map((a) => a.guid) ?? [];
  return {
    removed: addedGuids.map((guid) => ({ guid })),
    updated: updatedGuids
      .map((guid) => {
        const orig = original.find((a) => a.guid === guid);
        const upd = appliedDiff.updated?.find((a) => a.attribute.guid === guid);
        if (!orig || !upd) return null;
        return { attribute: { guid }, diff: inverseAttributeDiff(orig, upd.diff) };
      })
      .filter((item): item is { attribute: AttributeId; diff: AttributeDiff } => item !== null),
    added: removedGuids.map((guid) => original.find((a) => a.guid === guid)!).filter((a) => a !== undefined),
  };
};

/**
 * Diff type for tracking mergeAttributes changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖attribute🪨mergeattributesdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/mergeAttributesDiff)
 **/
export const mergeAttributesDiff = (first: AttributesDiff, second: AttributesDiff): AttributesDiff => {
  return { ...first, ...second };
};

/**
 * Diff type for tracking applyAttributes changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖attribute🪨applyattributesdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Attribute/d/i/applyAttributesDiff)
 **/
export const applyAttributesDiff = (base: Attribute[], diff: AttributesDiff): Attribute[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((attr) => !removedGuids.has(attr.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((attr) => attr.guid === update.attribute.guid);
      if (index !== -1) {
        result[index] = applyAttributeDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

// #endregion 🔖Attribute

// #region 🔖Coord (weak entity)

// [👤semio📚js💻semio🔖coordweakentity](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity))
// Coord weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Coord validation.
 * [👤semio📚js💻semio🔖coordweakentity🪨coordschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/CoordSchema)
 * [👤semio📚js💻semio🔖coordweakentity🪨coordschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/CoordSchema)
 **/
export const CoordSchema = z.object({ u: z.number(), v: z.number() });
/**
 * Type alias for Coord.
 * [👤semio📚js💻semio🔖coordweakentity🛠️coord](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/Coord)
 * [👤semio📚js💻semio🔖coordweakentity🛠️coord](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/Coord)
 **/
export type Coord = z.infer<typeof CoordSchema>;
/**
 * Serializes Coord for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖coordweakentity🪨serializecoord](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/serializeCoord)
 * [👤semio📚js💻semio🔖coordweakentity🪨serializecoord](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/serializeCoord)
 **/
export const serializeCoord = (coord: Coord): string => JSON.stringify(CoordSchema.parse(coord));
/**
 * Performs the deserializeCoord operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖coordweakentity🪨deserializecoord](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/deserializeCoord)
 * [👤semio📚js💻semio🔖coordweakentity🪨deserializecoord](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/deserializeCoord)
 **/
export const deserializeCoord = (json: string): Coord => CoordSchema.parse(JSON.parse(json));

/**
 * Zod schema for Coord diff validation.
 * [👤semio📚js💻semio🔖coordweakentity🪨coorddiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/CoordDiffSchema)
 * [👤semio📚js💻semio🔖coordweakentity🪨coorddiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/CoordDiffSchema)
 **/
export const CoordDiffSchema = CoordSchema.partial();
/**
 * Diff type for tracking Coord changes.
 * [👤semio📚js💻semio🔖coordweakentity🛠️coorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/CoordDiff)
 * [👤semio📚js💻semio🔖coordweakentity🛠️coorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/CoordDiff)
 **/
export type CoordDiff = z.infer<typeof CoordDiffSchema>;
/**
 * Retrieves the CoordDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖coordweakentity🪨getcoorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/getCoordDiff)
 * [👤semio📚js💻semio🔖coordweakentity🪨getcoorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/getCoordDiff)
 **/
export const getCoordDiff = (before: Coord, after: Coord): CoordDiff => {
  return {
    u: after.u - before.u,
    v: after.v - before.v,
  };
};
/**
 * Diff type for tracking inverseCoord changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖coordweakentity🪨inversecoorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/inverseCoordDiff)
 * [👤semio📚js💻semio🔖coordweakentity🪨inversecoorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/inverseCoordDiff)
 **/
export const inverseCoordDiff = (original: Coord, appliedDiff: CoordDiff): CoordDiff => {
  const u = appliedDiff.u ?? 0;
  const v = appliedDiff.v ?? 0;
  return {
    u: original.u - u,
    v: original.v - v,
  };
};
/**
 * Diff type for tracking mergeCoord changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖coordweakentity🪨mergecoorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/mergeCoordDiff)
 * [👤semio📚js💻semio🔖coordweakentity🪨mergecoorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/mergeCoordDiff)
 **/
export const mergeCoordDiff = (diff1: CoordDiff, diff2: CoordDiff): CoordDiff => {
  return {
    u: (diff1.u ?? 0) + (diff2.u ?? 0),
    v: (diff1.v ?? 0) + (diff2.v ?? 0),
  };
};
/**
 * Diff type for tracking applyCoord changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖coordweakentity🪨applycoorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/applyCoordDiff)
 * [👤semio📚js💻semio🔖coordweakentity🪨applycoorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/applyCoordDiff)
 **/
export const applyCoordDiff = (base: Coord, diff: CoordDiff): Coord => {
  const u = diff.u ?? 0;
  const v = diff.v ?? 0;
  return {
    u: base.u + u,
    v: base.v + v,
  };
};

// #endregion 🔖Coord (weak entity)

// #region 🔖Vec (weak entity)

// [👤semio📚js💻semio🔖vecweakentity](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity))
// Vec weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Vec validation.
 * [👤semio📚js💻semio🔖vecweakentity🪨vecschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/VecSchema)
 * [👤semio📚js💻semio🔖vecweakentity🪨vecschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/VecSchema)
 **/
export const VecSchema = z.object({ u: z.number(), v: z.number() });
/**
 * Type alias for Vec.
 * [👤semio📚js💻semio🔖vecweakentity🛠️vec](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/Vec)
 * [👤semio📚js💻semio🔖vecweakentity🛠️vec](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/Vec)
 **/
export type Vec = z.infer<typeof VecSchema>;
/**
 * Serializes Vec for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖vecweakentity🪨serializevec](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/serializeVec)
 * [👤semio📚js💻semio🔖vecweakentity🪨serializevec](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/serializeVec)
 **/
export const serializeVec = (vec: Vec): string => JSON.stringify(VecSchema.parse(vec));
/**
 * Performs the deserializeVec operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖vecweakentity🪨deserializevec](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/deserializeVec)
 * [👤semio📚js💻semio🔖vecweakentity🪨deserializevec](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/deserializeVec)
 **/
export const deserializeVec = (json: string): Vec => VecSchema.parse(JSON.parse(json));

/**
 * Zod schema for Vec diff validation.
 * [👤semio📚js💻semio🔖vecweakentity🪨vecdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/VecDiffSchema)
 * [👤semio📚js💻semio🔖vecweakentity🪨vecdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/VecDiffSchema)
 **/
export const VecDiffSchema = VecSchema.partial();
/**
 * Diff type for tracking Vec changes.
 * [👤semio📚js💻semio🔖vecweakentity🛠️vecdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/VecDiff)
 * [👤semio📚js💻semio🔖vecweakentity🛠️vecdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/VecDiff)
 **/
export type VecDiff = z.infer<typeof VecDiffSchema>;
/**
 * Retrieves the VecDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖vecweakentity🪨getvecdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/getVecDiff)
 * [👤semio📚js💻semio🔖vecweakentity🪨getvecdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/getVecDiff)
 **/
export const getVecDiff = (before: Vec, after: Vec): VecDiff => {
  return {
    u: after.u - before.u,
    v: after.v - before.v,
  };
};
/**
 * Diff type for tracking inverseVec changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖vecweakentity🪨inversevecdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/inverseVecDiff)
 **/
export const inverseVecDiff = (original: Vec, appliedDiff: VecDiff): VecDiff => {
  const u = appliedDiff.u ?? 0;
  const v = appliedDiff.v ?? 0;
  return {
    u: original.u - u,
    v: original.v - v,
  };
};
/**
 * Diff type for tracking mergeVec changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖vecweakentity🪨mergevecdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/mergeVecDiff)
 * [👤semio📚js💻semio🔖vecweakentity🪨mergevecdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/mergeVecDiff)
 **/
export const mergeVecDiff = (diff1: VecDiff, diff2: VecDiff): VecDiff => {
  return {
    u: (diff1.u ?? 0) + (diff2.u ?? 0),
    v: (diff1.v ?? 0) + (diff2.v ?? 0),
  };
};
/**
 * Diff type for tracking applyVec changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖vecweakentity🪨applyvecdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/applyVecDiff)
 * [👤semio📚js💻semio🔖vecweakentity🪨applyvecdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity)/d/i/applyVecDiff)
 **/
export const applyVecDiff = (base: Vec, diff: VecDiff): Vec => {
  const u = diff.u ?? 0;
  const v = diff.v ?? 0;
  return {
    u: base.u + u,
    v: base.v + v,
  };
};

// #endregion 🔖Vec (weak entity)

// #region 🔖Point (weak entity)

// [👤semio📚js💻semio🔖pointweakentity](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity))
// Point weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Point validation.
 * [👤semio📚js💻semio🔖pointweakentity🪨pointschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/PointSchema)
 * [👤semio📚js💻semio🔖pointweakentity🪨pointschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/PointSchema)
 **/
export const PointSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});
/**
 * Type alias for Point.
 * [👤semio📚js💻semio🔖pointweakentity🛠️point](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/Point)
 * [👤semio📚js💻semio🔖pointweakentity🛠️point](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/Point)
 **/
export type Point = z.infer<typeof PointSchema>;
/**
 * Serializes Point for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖pointweakentity🪨serializepoint](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/serializePoint)
 * [👤semio📚js💻semio🔖pointweakentity🪨serializepoint](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/serializePoint)
 **/
export const serializePoint = (point: Point): string => JSON.stringify(PointSchema.parse(point));
/**
 * Performs the deserializePoint operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖pointweakentity🪨deserializepoint](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/deserializePoint)
 * [👤semio📚js💻semio🔖pointweakentity🪨deserializepoint](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/deserializePoint)
 **/
export const deserializePoint = (json: string): Point => PointSchema.parse(JSON.parse(json));

/**
 * Zod schema for Point diff validation.
 * [👤semio📚js💻semio🔖pointweakentity🪨pointdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/PointDiffSchema)
 * [👤semio📚js💻semio🔖pointweakentity🪨pointdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/PointDiffSchema)
 **/
export const PointDiffSchema = PointSchema.partial();
/**
 * Diff type for tracking Point changes.
 * [👤semio📚js💻semio🔖pointweakentity🛠️pointdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/PointDiff)
 * [👤semio📚js💻semio🔖pointweakentity🛠️pointdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/PointDiff)
 **/
export type PointDiff = z.infer<typeof PointDiffSchema>;
/**
 * Retrieves the PointDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖pointweakentity🪨getpointdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/getPointDiff)
 * [👤semio📚js💻semio🔖pointweakentity🪨getpointdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/getPointDiff)
 **/
export const getPointDiff = (before: Point, after: Point): PointDiff => {
  return {
    x: after.x - before.x,
    y: after.y - before.y,
    z: after.z - before.z,
  };
};
/**
 * Diff type for tracking inversePoint changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖pointweakentity🪨inversepointdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/inversePointDiff)
 * [👤semio📚js💻semio🔖pointweakentity🪨inversepointdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/inversePointDiff)
 **/
export const inversePointDiff = (original: Point, appliedDiff: PointDiff): PointDiff => {
  const x = appliedDiff.x ?? 0;
  const y = appliedDiff.y ?? 0;
  const z = appliedDiff.z ?? 0;
  return {
    x: -x,
    y: -y,
    z: -z,
  };
};
/**
 * Diff type for tracking mergePoint changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖pointweakentity🪨mergepointdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/mergePointDiff)
 * [👤semio📚js💻semio🔖pointweakentity🪨mergepointdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/mergePointDiff)
 **/
export const mergePointDiff = (diff1: PointDiff, diff2: PointDiff): PointDiff => {
  return {
    x: (diff1.x ?? 0) + (diff2.x ?? 0),
    y: (diff1.y ?? 0) + (diff2.y ?? 0),
    z: (diff1.z ?? 0) + (diff2.z ?? 0),
  };
};
/**
 * Diff type for tracking applyPoint changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖pointweakentity🪨applypointdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/applyPointDiff)
 * [👤semio📚js💻semio🔖pointweakentity🪨applypointdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity)/d/i/applyPointDiff)
 **/
export const applyPointDiff = (base: Point, diff: PointDiff): Point => {
  const x = diff.x ?? 0;
  const y = diff.y ?? 0;
  const z = diff.z ?? 0;
  return {
    x: base.x + x,
    y: base.y + y,
    z: base.z + z,
  };
};

// #endregion 🔖Point (weak entity)

// #region 🔖Vector (weak entity)

// [👤semio📚js💻semio🔖vectorweakentity](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity))
// Vector weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Vector validation.
 * [👤semio📚js💻semio🔖vectorweakentity🪨vectorschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/VectorSchema)
 * [👤semio📚js💻semio🔖vectorweakentity🪨vectorschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/VectorSchema)
 **/
export const VectorSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});
/**
 * Type alias for Vector.
 * [👤semio📚js💻semio🔖vectorweakentity🛠️vector](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/Vector)
 * [👤semio📚js💻semio🔖vectorweakentity🛠️vector](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/Vector)
 **/
export type Vector = z.infer<typeof VectorSchema>;
/**
 * Serializes Vector for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖vectorweakentity🪨serializevector](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/serializeVector)
 * [👤semio📚js💻semio🔖vectorweakentity🪨serializevector](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/serializeVector)
 **/
export const serializeVector = (vector: Vector): string => JSON.stringify(VectorSchema.parse(vector));
/**
 * Performs the deserializeVector operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖vectorweakentity🪨deserializevector](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/deserializeVector)
 * [👤semio📚js💻semio🔖vectorweakentity🪨deserializevector](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/deserializeVector)
 **/
export const deserializeVector = (json: string): Vector => VectorSchema.parse(JSON.parse(json));

/**
 * Zod schema for Vector diff validation.
 * [👤semio📚js💻semio🔖vectorweakentity🪨vectordiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/VectorDiffSchema)
 * [👤semio📚js💻semio🔖vectorweakentity🪨vectordiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/VectorDiffSchema)
 **/
export const VectorDiffSchema = VectorSchema.partial();
/**
 * Diff type for tracking Vector changes.
 * [👤semio📚js💻semio🔖vectorweakentity🛠️vectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/VectorDiff)
 * [👤semio📚js💻semio🔖vectorweakentity🛠️vectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/VectorDiff)
 **/
export type VectorDiff = z.infer<typeof VectorDiffSchema>;
/**
 * Retrieves the VectorDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖vectorweakentity🪨getvectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/getVectorDiff)
 * [👤semio📚js💻semio🔖vectorweakentity🪨getvectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/getVectorDiff)
 **/
export const getVectorDiff = (before: Vector, after: Vector): VectorDiff => {
  return {
    x: after.x - before.x,
    y: after.y - before.y,
    z: after.z - before.z,
  };
};
/**
 * Diff type for tracking inverseVector changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖vectorweakentity🪨inversevectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/inverseVectorDiff)
 * [👤semio📚js💻semio🔖vectorweakentity🪨inversevectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/inverseVectorDiff)
 **/
export const inverseVectorDiff = (original: Vector, appliedDiff: VectorDiff): VectorDiff => {
  const x = appliedDiff.x ?? 0;
  const y = appliedDiff.y ?? 0;
  const z = appliedDiff.z ?? 0;
  return {
    x: -x,
    y: -y,
    z: -z,
  };
};
/**
 * Diff type for tracking mergeVector changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖vectorweakentity🪨mergevectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/mergeVectorDiff)
 * [👤semio📚js💻semio🔖vectorweakentity🪨mergevectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/mergeVectorDiff)
 **/
export const mergeVectorDiff = (diff1: VectorDiff, diff2: VectorDiff): VectorDiff => {
  return {
    x: (diff1.x ?? 0) + (diff2.x ?? 0),
    y: (diff1.y ?? 0) + (diff2.y ?? 0),
    z: (diff1.z ?? 0) + (diff2.z ?? 0),
  };
};
/**
 * Diff type for tracking applyVector changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖vectorweakentity🪨applyvectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/applyVectorDiff)
 * [👤semio📚js💻semio🔖vectorweakentity🪨applyvectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity)/d/i/applyVectorDiff)
 **/
export const applyVectorDiff = (base: Vector, diff: VectorDiff): Vector => {
  const x = diff.x ?? 0;
  const y = diff.y ?? 0;
  const z = diff.z ?? 0;
  return {
    x: base.x + x,
    y: base.y + y,
    z: base.z + z,
  };
};

// #endregion 🔖Vector (weak entity)

// #region 🔖Plane (weak entity)

// [👤semio📚js💻semio🔖planeweakentity](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity))
// Plane weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Plane validation.
 * [👤semio📚js💻semio🔖planeweakentity🪨planeschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/PlaneSchema)
 * [👤semio📚js💻semio🔖planeweakentity🪨planeschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/PlaneSchema)
 **/
export const PlaneSchema = z.object({
  origin: PointSchema,
  xAxis: VectorSchema,
  yAxis: VectorSchema,
});
/**
 * Type alias for Plane.
 * [👤semio📚js💻semio🔖planeweakentity🛠️plane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/Plane)
 * [👤semio📚js💻semio🔖planeweakentity🛠️plane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/Plane)
 **/
export type Plane = z.infer<typeof PlaneSchema>;
/**
 * Serializes Plane for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖planeweakentity🪨serializeplane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/serializePlane)
 * [👤semio📚js💻semio🔖planeweakentity🪨serializeplane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/serializePlane)
 **/
export const serializePlane = (plane: Plane): string => JSON.stringify(PlaneSchema.parse(plane));
/**
 * Performs the deserializePlane operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖planeweakentity🪨deserializeplane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/deserializePlane)
 * [👤semio📚js💻semio🔖planeweakentity🪨deserializeplane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/deserializePlane)
 **/
export const deserializePlane = (json: string): Plane => PlaneSchema.parse(JSON.parse(json));
/**
 * Performs the planeToMatrix operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖planeweakentity🪨planetomatrix](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/planeToMatrix)
 * [👤semio📚js💻semio🔖planeweakentity🪨planetomatrix](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/planeToMatrix)
 **/
export const planeToMatrix = (plane: Plane): THREE.Matrix4 => {
  const origin = new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
  const xAxis = new THREE.Vector3(plane.xAxis.x, plane.xAxis.y, plane.xAxis.z);
  const yAxis = new THREE.Vector3(plane.yAxis.x, plane.yAxis.y, plane.yAxis.z);
  const zAxis = new THREE.Vector3().crossVectors(xAxis, yAxis).normalize();
  const orthoYAxis = new THREE.Vector3().crossVectors(zAxis, xAxis).normalize();
  const matrix = new THREE.Matrix4().makeBasis(xAxis.normalize(), orthoYAxis, zAxis).setPosition(origin);
  return matrix;
};
/**
 * Performs the matrixToPlane operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖planeweakentity🪨matrixtoplane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/matrixToPlane)
 * [👤semio📚js💻semio🔖planeweakentity🪨matrixtoplane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/matrixToPlane)
 **/
export const matrixToPlane = (matrix: THREE.Matrix4): Plane => {
  const origin = new THREE.Vector3();
  const xAxis = new THREE.Vector3();
  const yAxis = new THREE.Vector3();
  const zAxis = new THREE.Vector3();
  matrix.decompose(origin, new THREE.Quaternion(), new THREE.Vector3());
  matrix.extractBasis(xAxis, yAxis, zAxis);
  return {
    origin: { x: origin.x, y: origin.y, z: origin.z },
    xAxis: { x: xAxis.x, y: xAxis.y, z: xAxis.z },
    yAxis: { x: yAxis.x, y: yAxis.y, z: yAxis.z },
  };
};

/**
 * Performs the averagePlane operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖planeweakentity🪨averageplane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/averagePlane)
 * [👤semio📚js💻semio🔖planeweakentity🪨averageplane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/averagePlane)
 **/
export const averagePlane = (planes: Plane[]): Plane | null => {
  if (planes.length === 0) return null;
  if (planes.length === 1) return planes[0];

  const avgOrigin = planes.reduce(
    (acc, plane) => ({
      x: acc.x + plane.origin.x / planes.length,
      y: acc.y + plane.origin.y / planes.length,
      z: acc.z + plane.origin.z / planes.length,
    }),
    { x: 0, y: 0, z: 0 },
  );

  const baseXAxis = planes[0].xAxis;
  const baseYAxis = planes[0].yAxis;

  return {
    origin: avgOrigin,
    xAxis: baseXAxis,
    yAxis: baseYAxis,
  };
};
// roundPlane holds the data fields for a roundPlane record.
// [👤semio📚js💻semio🔖planeweakentity🪨roundplane](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/roundPlane)
const roundPlane = (plane: Plane): Plane => ({
  origin: {
    x: round(plane.origin.x),
    y: round(plane.origin.y),
    z: round(plane.origin.z),
  },
  xAxis: {
    x: round(plane.xAxis.x),
    y: round(plane.xAxis.y),
    z: round(plane.xAxis.z),
  },
  yAxis: {
    x: round(plane.yAxis.x),
    y: round(plane.yAxis.y),
    z: round(plane.yAxis.z),
  },
});

/**
 * Zod schema for Plane diff validation.
 * [👤semio📚js💻semio🔖planeweakentity🪨planediffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/PlaneDiffSchema)
 * [👤semio📚js💻semio🔖planeweakentity🪨planediffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/PlaneDiffSchema)
 **/
export const PlaneDiffSchema = PlaneSchema.omit({ origin: true, xAxis: true, yAxis: true })
  .extend({
    origin: PointDiffSchema,
    xAxis: VectorDiffSchema,
    yAxis: VectorDiffSchema,
  })
  .partial();
/**
 * Diff type for tracking Plane changes.
 * [👤semio📚js💻semio🔖planeweakentity🛠️planediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/PlaneDiff)
 * [👤semio📚js💻semio🔖planeweakentity🛠️planediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/PlaneDiff)
 **/
export type PlaneDiff = z.infer<typeof PlaneDiffSchema>;
/**
 * Retrieves the PlaneDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖planeweakentity🪨getplanediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/getPlaneDiff)
 * [👤semio📚js💻semio🔖planeweakentity🪨getplanediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/getPlaneDiff)
 **/
export const getPlaneDiff = (before: Plane, after: Plane): PlaneDiff => {
  return {
    origin: getPointDiff(before.origin, after.origin),
    xAxis: getVectorDiff(before.xAxis, after.xAxis),
    yAxis: getVectorDiff(before.yAxis, after.yAxis),
  };
};
/**
 * Diff type for tracking inversePlane changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖planeweakentity🪨inverseplanediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/inversePlaneDiff)
 * [👤semio📚js💻semio🔖planeweakentity🪨inverseplanediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/inversePlaneDiff)
 **/
export const inversePlaneDiff = (original: Plane, appliedDiff: PlaneDiff): PlaneDiff => {
  const origin = appliedDiff.origin ?? { x: 0, y: 0, z: 0 };
  const xAxis = appliedDiff.xAxis ?? { x: 0, y: 0, z: 0 };
  const yAxis = appliedDiff.yAxis ?? { x: 0, y: 0, z: 0 };
  return {
    origin: inversePointDiff(original.origin, origin),
    xAxis: inverseVectorDiff(original.xAxis, xAxis),
    yAxis: inverseVectorDiff(original.yAxis, yAxis),
  };
};
/**
 * Diff type for tracking mergePlane changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖planeweakentity🪨mergeplanediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/mergePlaneDiff)
 * [👤semio📚js💻semio🔖planeweakentity🪨mergeplanediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/mergePlaneDiff)
 **/
export const mergePlaneDiff = (diff1: PlaneDiff, diff2: PlaneDiff): PlaneDiff => {
  return {
    origin: diff1.origin ?? diff2.origin ?? mergePointDiff(diff1.origin!, diff2.origin!),
    xAxis: diff1.xAxis ?? diff2.xAxis ?? mergeVectorDiff(diff1.xAxis!, diff2.xAxis!),
    yAxis: diff1.yAxis ?? diff2.yAxis ?? mergeVectorDiff(diff1.yAxis!, diff2.yAxis!),
  };
};
/**
 * Diff type for tracking applyPlane changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖planeweakentity🪨applyplanediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/applyPlaneDiff)
 * [👤semio📚js💻semio🔖planeweakentity🪨applyplanediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity)/d/i/applyPlaneDiff)
 **/
export const applyPlaneDiff = (base: Plane, diff: PlaneDiff): Plane => {
  return {
    origin: diff.origin ? applyPointDiff(base.origin, diff.origin) : base.origin,
    xAxis: diff.xAxis ? applyVectorDiff(base.xAxis, diff.xAxis) : base.xAxis,
    yAxis: diff.yAxis ? applyVectorDiff(base.yAxis, diff.yAxis) : base.yAxis,
  };
};

// #endregion 🔖Plane (weak entity)

// #region 🔖Camera (weak entity)

// Camera weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Camera validation.
 * [👤semio📚js💻semio🔖cameraweakentity🪨cameraschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/CameraSchema)
 * [👤semio📚js💻semio🔖cameraweakentity🪨cameraschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/CameraSchema)
 **/
export const CameraSchema = z.object({
  position: PointSchema,
  forward: VectorSchema,
  up: VectorSchema,
});
/**
 * Type alias for Camera.
 * [👤semio📚js💻semio🔖cameraweakentity🛠️camera](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/Camera)
 * [👤semio📚js💻semio🔖cameraweakentity🛠️camera](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/Camera)
 **/
export type Camera = z.infer<typeof CameraSchema>;
/**
 * Serializes Camera for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖cameraweakentity🪨serializecamera](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/serializeCamera)
 * [👤semio📚js💻semio🔖cameraweakentity🪨serializecamera](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/serializeCamera)
 **/
export const serializeCamera = (camera: Camera): string => JSON.stringify(CameraSchema.parse(camera));
/**
 * Performs the deserializeCamera operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖cameraweakentity🪨deserializecamera](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/deserializeCamera)
 * [👤semio📚js💻semio🔖cameraweakentity🪨deserializecamera](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/deserializeCamera)
 **/
export const deserializeCamera = (json: string): Camera => CameraSchema.parse(JSON.parse(json));

/**
 * Zod schema for Camera diff validation.
 * [👤semio📚js💻semio🔖cameraweakentity🪨cameradiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/CameraDiffSchema)
 * [👤semio📚js💻semio🔖cameraweakentity🪨cameradiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/CameraDiffSchema)
 **/
export const CameraDiffSchema = CameraSchema.omit({ position: true, forward: true, up: true })
  .extend({
    position: PointDiffSchema,
    forward: VectorDiffSchema,
    up: VectorDiffSchema,
  })
  .partial();
/**
 * Diff type for tracking Camera changes.
 * [👤semio📚js💻semio🔖cameraweakentity🛠️cameradiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/CameraDiff)
 * [👤semio📚js💻semio🔖cameraweakentity🛠️cameradiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/CameraDiff)
 **/
export type CameraDiff = z.infer<typeof CameraDiffSchema>;
/**
 * Retrieves the CameraDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖cameraweakentity🪨getcameradiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/getCameraDiff)
 * [👤semio📚js💻semio🔖cameraweakentity🪨getcameradiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/getCameraDiff)
 **/
export const getCameraDiff = (before: Camera, after: Camera): CameraDiff => {
  return {
    position: getPointDiff(before.position, after.position),
    forward: getVectorDiff(before.forward, after.forward),
    up: getVectorDiff(before.up, after.up),
  };
};
/**
 * Diff type for tracking inverseCamera changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖cameraweakentity🪨inversecameradiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/inverseCameraDiff)
 * [👤semio📚js💻semio🔖cameraweakentity🪨inversecameradiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/inverseCameraDiff)
 **/
export const inverseCameraDiff = (original: Camera, appliedDiff: CameraDiff): CameraDiff => {
  return {
    position: appliedDiff.position ? inversePointDiff(original.position, appliedDiff.position) : original.position,
    forward: appliedDiff.forward ? inverseVectorDiff(original.forward, appliedDiff.forward) : original.forward,
    up: appliedDiff.up ? inverseVectorDiff(original.up, appliedDiff.up) : original.up,
  };
};
/**
 * Diff type for tracking mergeCamera changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖cameraweakentity🪨mergecameradiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/mergeCameraDiff)
 * [👤semio📚js💻semio🔖cameraweakentity🪨mergecameradiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/mergeCameraDiff)
 **/
export const mergeCameraDiff = (diff1: CameraDiff, diff2: CameraDiff): CameraDiff => {
  return {
    position: diff1.position ?? diff2.position ?? mergePointDiff(diff1.position!, diff2.position!),
    forward: diff1.forward ?? diff2.forward ?? mergeVectorDiff(diff1.forward!, diff2.forward!),
    up: diff1.up ?? diff2.up ?? mergeVectorDiff(diff1.up!, diff2.up!),
  };
};
/**
 * Diff type for tracking applyCamera changes.
 * MUST perform the operation correctly. [👤semio📚js💻semio🔖cameraweakentity🪨applycameradiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/applyCameraDiff)
 * [👤semio📚js💻semio🔖cameraweakentity🪨applycameradiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity)/d/i/applyCameraDiff)
 **/
export const applyCameraDiff = (base: Camera, diff: CameraDiff): Camera => {
  return {
    position: diff.position ? applyPointDiff(base.position, diff.position) : base.position,
    forward: diff.forward ? applyVectorDiff(base.forward, diff.forward) : base.forward,
    up: diff.up ? applyVectorDiff(base.up, diff.up) : base.up,
  };
};

// #endregion 🔖Camera (weak entity)

// #region 🔖Location

// [👤semio📚js💻semio🔖location](repo://p/u/semio/b/l/js/f/semio.ts/s/Location)
// Location entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Location validation.
 * [👤semio📚js💻semio🔖location🪨locationschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Location/d/i/LocationSchema)
 **/
export const LocationSchema = z.object({
  guid: z.string(),
  longitude: z.number(),
  latitude: z.number(),
  altitude: z.number().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Location.
 * [👤semio📚js💻semio🔖location🛠️location](repo://p/u/semio/b/l/js/f/semio.ts/s/Location/d/i/Location)
 **/
export type Location = z.infer<typeof LocationSchema>;
/**
 * Serializes Location for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖location🪨serializelocation](repo://p/u/semio/b/l/js/f/semio.ts/s/Location/d/i/serializeLocation)
 **/
export const serializeLocation = (location: Location): string => JSON.stringify(LocationSchema.parse(location));
/**
 * Performs the deserializeLocation operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖location🪨deserializelocation](repo://p/u/semio/b/l/js/f/semio.ts/s/Location/d/i/deserializeLocation)
 **/
export const deserializeLocation = (json: string): Location => LocationSchema.parse(JSON.parse(json));

/**
 * Zod schema for Location diff validation.
 * [👤semio📚js💻semio🔖location🪨locationdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Location/d/i/LocationDiffSchema)
 **/
export const LocationDiffSchema = LocationSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Location changes.
 * [👤semio📚js💻semio🔖location🛠️locationdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Location/d/i/LocationDiff)
 **/
export type LocationDiff = z.infer<typeof LocationDiffSchema>;
/**
 * Retrieves the LocationDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖location🪨getlocationdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Location/d/i/getLocationDiff)
 **/
export const getLocationDiff = (before: Location, after: Location): LocationDiff => {
  const diff: LocationDiff = {};
  if (before.longitude !== after.longitude) diff.longitude = after.longitude - before.longitude;
  if (before.latitude !== after.latitude) diff.latitude = after.latitude - before.latitude;
  if (before.altitude !== after.altitude) diff.altitude = after.altitude !== undefined && before.altitude !== undefined ? after.altitude - before.altitude : after.altitude;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseLocation changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖location🪨inverselocationdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Location/d/i/inverseLocationDiff)
 **/
export const inverseLocationDiff = (original: Location, appliedDiff: LocationDiff): LocationDiff => {
  const inverse: LocationDiff = {};
  if (appliedDiff.longitude !== undefined) inverse.longitude = original.longitude;
  if (appliedDiff.latitude !== undefined) inverse.latitude = original.latitude;
  if (appliedDiff.altitude !== undefined) inverse.altitude = original.altitude;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeLocation changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖location🪨mergelocationdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Location/d/i/mergeLocationDiff)
 **/
export const mergeLocationDiff = (diff1: LocationDiff, diff2: LocationDiff): LocationDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyLocation changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖location🪨applylocationdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Location/d/i/applyLocationDiff)
 **/
export const applyLocationDiff = (base: Location, diff: LocationDiff): Location => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Location = {
    guid: base.guid,
    longitude: diff.longitude ?? base.longitude,
    latitude: diff.latitude ?? base.latitude,
    altitude: diff.altitude ?? base.altitude,
  };

  return result;
};

// #endregion 🔖Location

// #region 🔖Author

// [👤semio📚js💻semio🔖author](repo://p/u/semio/b/l/js/f/semio.ts/s/Author)
// Author entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Author validation.
 * [👤semio📚js💻semio🔖author🪨authorschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/AuthorSchema)
 **/
export const AuthorSchema = z.object({ guid: z.string(), name: z.string(), email: z.string(), attributes: z.array(AttributeSchema).optional() });
/**
 * Type alias for Author.
 * [👤semio📚js💻semio🔖author🛠️author](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/Author)
 **/
export type Author = z.infer<typeof AuthorSchema>;
/**
 * Serializes Author for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖author🪨serializeauthor](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/serializeAuthor)
 **/
export const serializeAuthor = (author: Author): string => JSON.stringify(AuthorSchema.parse(author));
/**
 * Performs the deserializeAuthor operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖author🪨deserializeauthor](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/deserializeAuthor)
 **/
export const deserializeAuthor = (json: string): Author => AuthorSchema.parse(JSON.parse(json));

/**
 * Zod schema for Author diff validation.
 * [👤semio📚js💻semio🔖author🪨authordiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/AuthorDiffSchema)
 **/
export const AuthorDiffSchema = AuthorSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Author changes.
 * [👤semio📚js💻semio🔖author🛠️authordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/AuthorDiff)
 **/
export type AuthorDiff = z.infer<typeof AuthorDiffSchema>;
/**
 * Retrieves the AuthorDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖author🪨getauthordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/getAuthorDiff)
 **/
export const getAuthorDiff = (before: Author, after: Author): AuthorDiff => {
  const diff: AuthorDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.email !== after.email) diff.email = after.email;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseAuthor changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖author🪨inverseauthordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/inverseAuthorDiff)
 **/
export const inverseAuthorDiff = (original: Author, appliedDiff: AuthorDiff): AuthorDiff => {
  const inverse: AuthorDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.email !== undefined) inverse.email = original.email;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeAuthor changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖author🪨mergeauthordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/mergeAuthorDiff)
 **/
export const mergeAuthorDiff = (diff1: AuthorDiff, diff2: AuthorDiff): AuthorDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyAuthor changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖author🪨applyauthordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/applyAuthorDiff)
 **/
export const applyAuthorDiff = (base: Author, diff: AuthorDiff): Author => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Author = {
    guid: base.guid,
    name: diff.name ?? base.name,
    email: diff.email ?? base.email,
  };

  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Authors diff validation.
 * [👤semio📚js💻semio🔖author🪨authorsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/AuthorsDiffSchema)
 **/
export const AuthorsDiffSchema = z.object({
  removed: z.array(AuthorIdSchema).optional(),
  updated: z.array(z.object({ author: AuthorIdSchema, diff: AuthorDiffSchema })).optional(),
  added: z.array(AuthorSchema).optional(),
});
/**
 * Diff type for tracking Authors changes.
 * [👤semio📚js💻semio🔖author🛠️authorsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Author/d/i/AuthorsDiff)
 **/
export type AuthorsDiff = z.infer<typeof AuthorsDiffSchema>;

// #endregion 🔖Author

// #region 🔖File

// [👤semio📚js💻semio🔖file](repo://p/u/semio/b/l/js/f/semio.ts/s/File)
// File entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for File validation.
 * [👤semio📚js💻semio🔖file🪨fileschema](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/FileSchema)
 **/
export const FileSchema = z.object({
  guid: z.string(),
  name: z.string(),
  remote: z.string().optional(),
  folder: FolderIdSchema.optional(),
  size: z.number().optional(),
  hash: z.string().optional(),
  blob: z.string().optional(),
  createdAt: DateProperty(),
  createdBy: z.string().optional(),
  updatedAt: DateProperty(),
  updatedBy: z.string().optional(),
});
/**
 * Type alias for File.
 * [👤semio📚js💻semio🔖file🛠️file](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/File)
 **/
export type File = z.infer<typeof FileSchema>;
/**
 * Serializes File for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖file🪨serializefile](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/serializeFile)
 **/
export const serializeFile = (file: File): string => JSON.stringify(FileSchema.parse(file));
/**
 * Performs the deserializeFile operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖file🪨deserializefile](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/deserializeFile)
 **/
export const deserializeFile = (json: string): File => FileSchema.parse(JSON.parse(json));

/**
 * Zod schema for File diff validation.
 * [👤semio📚js💻semio🔖file🪨filediffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/FileDiffSchema)
 **/
export const FileDiffSchema = FileSchema.partial();
/**
 * Diff type for tracking File changes.
 * [👤semio📚js💻semio🔖file🛠️filediff](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/FileDiff)
 **/
export type FileDiff = z.infer<typeof FileDiffSchema>;
/**
 * Retrieves the FileDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖file🪨getfilediff](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/getFileDiff)
 **/
export const getFileDiff = (before: File, after: File): FileDiff => {
  const diff: FileDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.remote !== after.remote) diff.remote = after.remote;
  if (before.size !== after.size) diff.size = after.size;
  if (before.hash !== after.hash) diff.hash = after.hash;
  if (before.createdAt !== after.createdAt) diff.createdAt = after.createdAt;
  if (before.createdBy !== after.createdBy) diff.createdBy = after.createdBy;
  if (before.updatedAt !== after.updatedAt) diff.updatedAt = after.updatedAt;
  if (before.updatedBy !== after.updatedBy) diff.updatedBy = after.updatedBy;
  if (before.folder?.guid !== after.folder?.guid) diff.folder = after.folder;
  return diff;
};
/**
 * Diff type for tracking inverseFile changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖file🪨inversefilediff](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/inverseFileDiff)
 **/
export const inverseFileDiff = (original: File, appliedDiff: FileDiff): FileDiff => {
  const inverse: FileDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.remote !== undefined) inverse.remote = original.remote;
  if (appliedDiff.size !== undefined) inverse.size = original.size;
  if (appliedDiff.hash !== undefined) inverse.hash = original.hash;
  if (appliedDiff.createdAt !== undefined) inverse.createdAt = original.createdAt;
  if (appliedDiff.createdBy !== undefined) inverse.createdBy = original.createdBy;
  if (appliedDiff.updatedAt !== undefined) inverse.updatedAt = original.updatedAt;
  if (appliedDiff.updatedBy !== undefined) inverse.updatedBy = original.updatedBy;
  if (appliedDiff.folder !== undefined) inverse.folder = original.folder;
  return inverse;
};
/**
 * Diff type for tracking mergeFile changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖file🪨mergefilediff](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/mergeFileDiff)
 **/
export const mergeFileDiff = (diff1: FileDiff, diff2: FileDiff): FileDiff => {
  return { ...diff1, ...diff2 };
};
/**
 * Diff type for tracking applyFile changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖file🪨applyfilediff](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/applyFileDiff)
 **/
export const applyFileDiff = (base: File, diff: FileDiff): File => {
  const result: File = {
    guid: base.guid,
    name: diff.name ?? base.name,
  };

  if (diff.remote !== undefined || base.remote !== undefined) result.remote = diff.remote ?? base.remote;
  if (diff.size !== undefined || base.size !== undefined) result.size = diff.size ?? base.size;
  if (diff.hash !== undefined || base.hash !== undefined) result.hash = diff.hash ?? base.hash;
  if (diff.createdAt !== undefined || base.createdAt !== undefined) result.createdAt = diff.createdAt ?? base.createdAt;
  if (diff.createdBy !== undefined || base.createdBy !== undefined) result.createdBy = diff.createdBy ?? base.createdBy;
  if (diff.updatedAt !== undefined || base.updatedAt !== undefined) result.updatedAt = diff.updatedAt ?? base.updatedAt;
  if (diff.updatedBy !== undefined || base.updatedBy !== undefined) result.updatedBy = diff.updatedBy ?? base.updatedBy;
  if (diff.folder !== undefined || base.folder !== undefined) result.folder = diff.folder ?? base.folder;
  if (base.blob !== undefined) result.blob = base.blob;

  return result;
};

/**
 * Zod schema for Files diff validation.
 * [👤semio📚js💻semio🔖file🪨filesdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/FilesDiffSchema)
 **/
export const FilesDiffSchema = z.object({
  removed: z.array(FileIdSchema).optional(),
  updated: z.array(z.object({ file: FileIdSchema, diff: FileDiffSchema })).optional(),
  added: z.array(FileSchema).optional(),
});
/**
 * Diff type for tracking Files changes.
 * [👤semio📚js💻semio🔖file🛠️filesdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/File/d/i/FilesDiff)
 **/
export type FilesDiff = z.infer<typeof FilesDiffSchema>;

// #endregion 🔖File

// #region 🔖Folder

// [👤semio📚js💻semio🔖folder](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder)
// Folder entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Folder validation.
 * [👤semio📚js💻semio🔖folder🪨folderschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/FolderSchema)
 **/
export const FolderSchema = z.object({
  guid: z.string(),
  name: z.string(),
  parent: FolderIdSchema.optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  createdAt: DateProperty(),
  createdBy: z.string().optional(),
  updatedAt: DateProperty(),
  updatedBy: z.string().optional(),
});
/**
 * Type alias for Folder.
 * [👤semio📚js💻semio🔖folder🛠️folder](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/Folder)
 **/
export type Folder = z.infer<typeof FolderSchema>;
/**
 * Serializes Folder for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖folder🪨serializefolder](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/serializeFolder)
 **/
export const serializeFolder = (folder: Folder): string => JSON.stringify(FolderSchema.parse(folder));
/**
 * Performs the deserializeFolder operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖folder🪨deserializefolder](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/deserializeFolder)
 **/
export const deserializeFolder = (json: string): Folder => FolderSchema.parse(JSON.parse(json));

/**
 * Zod schema for Folder diff validation.
 * [👤semio📚js💻semio🔖folder🪨folderdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/FolderDiffSchema)
 **/
export const FolderDiffSchema = FolderSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Folder changes.
 * [👤semio📚js💻semio🔖folder🛠️folderdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/FolderDiff)
 **/
export type FolderDiff = z.infer<typeof FolderDiffSchema>;
/**
 * Retrieves the FolderDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖folder🪨getfolderdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/getFolderDiff)
 **/
export const getFolderDiff = (before: Folder, after: Folder): FolderDiff => {
  const diff: FolderDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.parent?.guid !== after.parent?.guid) diff.parent = after.parent;
  if (before.description !== after.description) diff.description = after.description;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (before.createdAt !== after.createdAt) diff.createdAt = after.createdAt;
  if (before.createdBy !== after.createdBy) diff.createdBy = after.createdBy;
  if (before.updatedAt !== after.updatedAt) diff.updatedAt = after.updatedAt;
  if (before.updatedBy !== after.updatedBy) diff.updatedBy = after.updatedBy;
  return diff;
};
/**
 * Diff type for tracking inverseFolder changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖folder🪨inversefolderdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/inverseFolderDiff)
 **/
export const inverseFolderDiff = (original: Folder, appliedDiff: FolderDiff): FolderDiff => {
  const inverse: FolderDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.parent !== undefined) inverse.parent = original.parent;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  if (appliedDiff.createdAt !== undefined) inverse.createdAt = original.createdAt;
  if (appliedDiff.createdBy !== undefined) inverse.createdBy = original.createdBy;
  if (appliedDiff.updatedAt !== undefined) inverse.updatedAt = original.updatedAt;
  if (appliedDiff.updatedBy !== undefined) inverse.updatedBy = original.updatedBy;
  return inverse;
};
/**
 * Diff type for tracking mergeFolder changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖folder🪨mergefolderdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/mergeFolderDiff)
 **/
export const mergeFolderDiff = (diff1: FolderDiff, diff2: FolderDiff): FolderDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyFolder changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖folder🪨applyfolderdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/applyFolderDiff)
 **/
export const applyFolderDiff = (base: Folder, diff: FolderDiff): Folder => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Folder = {
    guid: base.guid,
    name: diff.name ?? base.name,
  };

  if (diff.parent !== undefined || base.parent !== undefined) result.parent = diff.parent ?? base.parent;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;
  if (diff.createdAt !== undefined || base.createdAt !== undefined) result.createdAt = diff.createdAt ?? base.createdAt;
  if (diff.createdBy !== undefined || base.createdBy !== undefined) result.createdBy = diff.createdBy ?? base.createdBy;
  if (diff.updatedAt !== undefined || base.updatedAt !== undefined) result.updatedAt = diff.updatedAt ?? base.updatedAt;
  if (diff.updatedBy !== undefined || base.updatedBy !== undefined) result.updatedBy = diff.updatedBy ?? base.updatedBy;

  return result;
};

/**
 * Zod schema for Folders diff validation.
 * [👤semio📚js💻semio🔖folder🪨foldersdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/FoldersDiffSchema)
 **/
export const FoldersDiffSchema = z.object({
  removed: z.array(FolderIdSchema).optional(),
  updated: z.array(z.object({ folder: FolderIdSchema, diff: FolderDiffSchema })).optional(),
  added: z.array(FolderSchema).optional(),
});
/**
 * Diff type for tracking Folders changes.
 * [👤semio📚js💻semio🔖folder🛠️foldersdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Folder/d/i/FoldersDiff)
 **/
export type FoldersDiff = z.infer<typeof FoldersDiffSchema>;

// #endregion 🔖Folder

// #region 🔖Benchmark

// [👤semio📚js💻semio🔖benchmark](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark)
// Benchmark entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Benchmark validation.
 * [👤semio📚js💻semio🔖benchmark🪨benchmarkschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/BenchmarkSchema)
 **/
export const BenchmarkSchema = z.object({
  guid: z.string(),
  name: z.string(),
  icon: z.string().optional(),
  min: z.number().optional(),
  minExcluded: z.boolean().optional(),
  max: z.number().optional(),
  maxExcluded: z.boolean().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Benchmark.
 * [👤semio📚js💻semio🔖benchmark🛠️benchmark](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/Benchmark)
 **/
export type Benchmark = z.infer<typeof BenchmarkSchema>;
/**
 * Serializes Benchmark for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖benchmark🪨serializebenchmark](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/serializeBenchmark)
 **/
export const serializeBenchmark = (benchmark: Benchmark): string => JSON.stringify(BenchmarkSchema.parse(benchmark));
/**
 * Performs the deserializeBenchmark operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖benchmark🪨deserializebenchmark](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/deserializeBenchmark)
 **/
export const deserializeBenchmark = (json: string): Benchmark => BenchmarkSchema.parse(JSON.parse(json));

/**
 * Zod schema for Benchmark diff validation.
 * [👤semio📚js💻semio🔖benchmark🪨benchmarkdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/BenchmarkDiffSchema)
 **/
export const BenchmarkDiffSchema = BenchmarkSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Benchmark changes.
 * [👤semio📚js💻semio🔖benchmark🛠️benchmarkdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/BenchmarkDiff)
 **/
export type BenchmarkDiff = z.infer<typeof BenchmarkDiffSchema>;
/**
 * Diff type for tracking applyBenchmark changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖benchmark🪨applybenchmarkdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/applyBenchmarkDiff)
 **/
export const applyBenchmarkDiff = (base: Benchmark, diff: BenchmarkDiff): Benchmark => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Benchmark = {
    guid: base.guid,
    name: diff.name ?? base.name,
  };

  if (diff.icon !== undefined || base.icon !== undefined) result.icon = diff.icon ?? base.icon;
  if (diff.min !== undefined || base.min !== undefined) result.min = diff.min ?? base.min;
  if (diff.minExcluded !== undefined || base.minExcluded !== undefined) result.minExcluded = diff.minExcluded ?? base.minExcluded;
  if (diff.maxExcluded !== undefined || base.maxExcluded !== undefined) result.maxExcluded = diff.maxExcluded ?? base.maxExcluded;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};
/**
 * Retrieves the BenchmarkDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖benchmark🪨getbenchmarkdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/getBenchmarkDiff)
 **/
export const getBenchmarkDiff = (before: Benchmark, after: Benchmark): BenchmarkDiff => {
  const diff: BenchmarkDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.min !== after.min) diff.min = after.min !== undefined && before.min !== undefined ? after.min - before.min : after.min;
  if (before.minExcluded !== after.minExcluded) diff.minExcluded = after.minExcluded;
  if (before.max !== after.max) diff.max = after.max !== undefined && before.max !== undefined ? after.max - before.max : after.max;
  if (before.maxExcluded !== after.maxExcluded) diff.maxExcluded = after.maxExcluded;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseBenchmark changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖benchmark🪨inversebenchmarkdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/inverseBenchmarkDiff)
 **/
export const inverseBenchmarkDiff = (original: Benchmark, appliedDiff: BenchmarkDiff): BenchmarkDiff => {
  const inverse: BenchmarkDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon;
  if (appliedDiff.min !== undefined) inverse.min = original.min;
  if (appliedDiff.minExcluded !== undefined) inverse.minExcluded = original.minExcluded;
  if (appliedDiff.max !== undefined) inverse.max = original.max;
  if (appliedDiff.maxExcluded !== undefined) inverse.maxExcluded = original.maxExcluded;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeBenchmark changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖benchmark🪨mergebenchmarkdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/mergeBenchmarkDiff)
 **/
export const mergeBenchmarkDiff = (diff1: BenchmarkDiff, diff2: BenchmarkDiff): BenchmarkDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};

/**
 * Zod schema for Benchmarks diff validation.
 * [👤semio📚js💻semio🔖benchmark🪨benchmarksdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/BenchmarksDiffSchema)
 **/
export const BenchmarksDiffSchema = z.object({
  removed: z.array(BenchmarkIdSchema).optional(),
  updated: z.array(z.object({ benchmark: BenchmarkIdSchema, diff: BenchmarkDiffSchema })).optional(),
  added: z.array(BenchmarkSchema).optional(),
});
/**
 * Diff type for tracking Benchmarks changes.
 * [👤semio📚js💻semio🔖benchmark🛠️benchmarksdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/BenchmarksDiff)
 **/
export type BenchmarksDiff = z.infer<typeof BenchmarksDiffSchema>;

// [👤semio📚js💻semio🔖benchmark🪨getbenchmarksdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/getBenchmarksDiff)
// getBenchmarksDiff holds the data fields for a getBenchmarksDiff record.
const getBenchmarksDiff = (before: Benchmark[], after: Benchmark[]): BenchmarksDiff => {
  const beforeGuids = new Set(before.map((b) => b.guid));
  const afterGuids = new Set(after.map((b) => b.guid));
  const removed = before.filter((b) => !afterGuids.has(b.guid)).map((b) => ({ guid: b.guid }));
  const added = after.filter((b) => !beforeGuids.has(b.guid));
  const updated = after
    .filter((b) => beforeGuids.has(b.guid))
    .map((afterBenchmark) => {
      const beforeBenchmark = before.find((b) => b.guid === afterBenchmark.guid)!;
      const diff = getBenchmarkDiff(beforeBenchmark, afterBenchmark);
      return { benchmark: { guid: afterBenchmark.guid }, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: BenchmarksDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

// inverseBenchmarksDiff holds the data fields for a inverseBenchmarksDiff record.
// [👤semio📚js💻semio🔖benchmark🪨inversebenchmarksdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/inverseBenchmarksDiff)
const inverseBenchmarksDiff = (original: Benchmark[], appliedDiff: BenchmarksDiff): BenchmarksDiff => {
  const addedGuids = appliedDiff.added?.map((b) => b.guid) ?? [];
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  const updatedGuids = appliedDiff.updated?.map((u) => u.benchmark.guid) ?? [];
  return {
    removed: addedGuids.map((guid) => ({ guid })),
    added: original.filter((b) => removedGuids.includes(b.guid)),
    updated: updatedGuids.map((guid) => {
      const orig = original.find((b) => b.guid === guid)!;
      const upd = appliedDiff.updated?.find((u) => u.benchmark.guid === guid)!;
      return { benchmark: { guid }, diff: inverseBenchmarkDiff(orig, upd.diff) };
    }),
  };
};

// [👤semio📚js💻semio🔖benchmark🪨mergebenchmarksdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/mergeBenchmarksDiff)
// mergeBenchmarksDiff holds the data fields for a mergeBenchmarksDiff record.
const mergeBenchmarksDiff = (first: BenchmarksDiff, second: BenchmarksDiff): BenchmarksDiff => {
  return { ...first, ...second };
};

// applyBenchmarksDiff holds the data fields for a applyBenchmarksDiff record.
// [👤semio📚js💻semio🔖benchmark🪨applybenchmarksdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark/d/i/applyBenchmarksDiff)
const applyBenchmarksDiff = (base: Benchmark[], diff: BenchmarksDiff): Benchmark[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((benchmark) => !removedGuids.has(benchmark.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((benchmark) => benchmark.guid === update.benchmark.guid);
      if (index !== -1) {
        result[index] = applyBenchmarkDiff(result[index], update.diff);
      }
    }
  }

  if (diff.added) {
    result.push(...diff.added);
  }

  return result;
};

// #endregion 🔖Benchmark

// #region 🔖Quality

// [👤semio📚js💻semio🔖quality](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality)
// Quality entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Quality validation.
 * [👤semio📚js💻semio🔖quality🪨qualityschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/QualitySchema)
 **/
export const QualitySchema = z.object({
  guid: z.string(),
  key: z.string(),
  name: z.string(),
  description: z.string().optional(),
  uri: z.string().optional(),
  kind: z.number().optional(),
  folder: z.string().optional(),
  canScale: z.boolean().optional(),
  defaultSiUnit: z.string().optional(),
  defaultImperialUnit: z.string().optional(),
  min: z.number().optional(),
  isMinExcluded: z.boolean().optional(),
  max: z.number().optional(),
  isMaxExcluded: z.boolean().optional(),
  defaultValue: z.number().optional(),
  formula: z.string().optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  unit: z.string().optional(),
  benchmarks: z.array(BenchmarkSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Quality.
 * [👤semio📚js💻semio🔖quality🛠️quality](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/Quality)
 **/
export type Quality = z.infer<typeof QualitySchema>;
/**
 * Serializes Quality for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖quality🪨serializequality](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/serializeQuality)
 **/
export const serializeQuality = (quality: Quality): string => JSON.stringify(QualitySchema.parse(quality));
/**
 * Performs the deserializeQuality operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖quality🪨deserializequality](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/deserializeQuality)
 **/
export const deserializeQuality = (json: string): Quality => QualitySchema.parse(JSON.parse(json));

/**
 * [👤semio📚js💻semio🔖quality🪨qualitydiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/QualityDiffSchema)
 **/
export const QualityDiffSchema = QualitySchema.partial().omit({ benchmarks: true, attributes: true }).extend({
  benchmarks: BenchmarksDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Quality changes.
 * [👤semio📚js💻semio🔖quality🛠️qualitydiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/QualityDiff)
 **/
export type QualityDiff = z.infer<typeof QualityDiffSchema>;
/**
 * Retrieves the QualityDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖quality🪨getqualitydiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/getQualityDiff)
 **/
export const getQualityDiff = (before: Quality, after: Quality): QualityDiff => {
  const diff: QualityDiff = {};
  if (before.key !== after.key) diff.key = after.key;
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description;
  if (before.uri !== after.uri) diff.uri = after.uri;
  if (before.kind !== after.kind) diff.kind = after.kind !== undefined && before.kind !== undefined ? after.kind - before.kind : after.kind;
  if (before.canScale !== after.canScale) diff.canScale = after.canScale;
  if (before.defaultSiUnit !== after.defaultSiUnit) diff.defaultSiUnit = after.defaultSiUnit;
  if (before.defaultImperialUnit !== after.defaultImperialUnit) diff.defaultImperialUnit = after.defaultImperialUnit;
  if (before.min !== after.min) diff.min = after.min !== undefined && before.min !== undefined ? after.min - before.min : after.min;
  if (before.isMinExcluded !== after.isMinExcluded) diff.isMinExcluded = after.isMinExcluded;
  if (before.max !== after.max) diff.max = after.max !== undefined && before.max !== undefined ? after.max - before.max : after.max;
  if (before.isMaxExcluded !== after.isMaxExcluded) diff.isMaxExcluded = after.isMaxExcluded;
  if (before.defaultValue !== after.defaultValue) diff.defaultValue = after.defaultValue !== undefined && before.defaultValue !== undefined ? after.defaultValue - before.defaultValue : after.defaultValue;
  if (before.formula !== after.formula) diff.formula = after.formula;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.image !== after.image) diff.image = after.image;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (!deepEqual(before.benchmarks, after.benchmarks)) diff.benchmarks = getBenchmarksDiff(before.benchmarks ?? [], after.benchmarks ?? []);
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseQuality changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖quality🪨inversequalitydiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/inverseQualityDiff)
 **/
export const inverseQualityDiff = (original: Quality, appliedDiff: QualityDiff): QualityDiff => {
  const inverse: QualityDiff = {};
  if (appliedDiff.key !== undefined) inverse.key = original.key;
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.uri !== undefined) inverse.uri = original.uri;
  if (appliedDiff.kind !== undefined) inverse.kind = original.kind;
  if (appliedDiff.canScale !== undefined) inverse.canScale = original.canScale;
  if (appliedDiff.defaultSiUnit !== undefined) inverse.defaultSiUnit = original.defaultSiUnit;
  if (appliedDiff.defaultImperialUnit !== undefined) inverse.defaultImperialUnit = original.defaultImperialUnit;
  if (appliedDiff.min !== undefined) inverse.min = original.min;
  if (appliedDiff.isMinExcluded !== undefined) inverse.isMinExcluded = original.isMinExcluded;
  if (appliedDiff.max !== undefined) inverse.max = original.max;
  if (appliedDiff.isMaxExcluded !== undefined) inverse.isMaxExcluded = original.isMaxExcluded;
  if (appliedDiff.defaultValue !== undefined) inverse.defaultValue = original.defaultValue;
  if (appliedDiff.formula !== undefined) inverse.formula = original.formula;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon;
  if (appliedDiff.image !== undefined) inverse.image = original.image;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.benchmarks !== undefined) inverse.benchmarks = inverseBenchmarksDiff(original.benchmarks ?? [], appliedDiff.benchmarks);
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeQuality changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖quality🪨mergequalitydiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/mergeQualityDiff)
 **/
export const mergeQualityDiff = (diff1: QualityDiff, diff2: QualityDiff): QualityDiff => {
  return {
    ...diff1,
    ...diff2,
    benchmarks: diff1.benchmarks && diff2.benchmarks ? mergeBenchmarksDiff(diff1.benchmarks, diff2.benchmarks) : (diff2.benchmarks ?? diff1.benchmarks),
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
/**
 * Diff type for tracking applyQuality changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖quality🪨applyqualitydiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/applyQualityDiff)
 **/
export const applyQualityDiff = (base: Quality, diff: QualityDiff): Quality => {
  const benchmarks = diff.benchmarks ? applyBenchmarksDiff(base.benchmarks ?? [], diff.benchmarks) : base.benchmarks;
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Quality = {
    guid: base.guid,
    key: diff.key ?? base.key,
    name: diff.name ?? base.name,
  };

  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (diff.uri !== undefined || base.uri !== undefined) result.uri = diff.uri ?? base.uri;
  if (diff.kind !== undefined || base.kind !== undefined) result.kind = diff.kind ?? base.kind;
  if (diff.folder !== undefined || base.folder !== undefined) result.folder = diff.folder ?? base.folder;
  if (diff.canScale !== undefined || base.canScale !== undefined) result.canScale = diff.canScale ?? base.canScale;
  if (diff.defaultSiUnit !== undefined || base.defaultSiUnit !== undefined) result.defaultSiUnit = diff.defaultSiUnit ?? base.defaultSiUnit;
  if (diff.defaultImperialUnit !== undefined || base.defaultImperialUnit !== undefined) result.defaultImperialUnit = diff.defaultImperialUnit ?? base.defaultImperialUnit;
  if (diff.min !== undefined || base.min !== undefined) result.min = diff.min ?? base.min;
  if (diff.isMinExcluded !== undefined || base.isMinExcluded !== undefined) result.isMinExcluded = diff.isMinExcluded ?? base.isMinExcluded;
  if (diff.max !== undefined || base.max !== undefined) result.max = diff.max ?? base.max;
  if (diff.isMaxExcluded !== undefined || base.isMaxExcluded !== undefined) result.isMaxExcluded = diff.isMaxExcluded ?? base.isMaxExcluded;
  if (diff.defaultValue !== undefined || base.defaultValue !== undefined) result.defaultValue = diff.defaultValue ?? base.defaultValue;
  if (diff.formula !== undefined || base.formula !== undefined) result.formula = diff.formula ?? base.formula;
  if (diff.icon !== undefined || base.icon !== undefined) result.icon = diff.icon ?? base.icon;
  if (diff.image !== undefined || base.image !== undefined) result.image = diff.image ?? base.image;
  if (diff.unit !== undefined || base.unit !== undefined) result.unit = diff.unit ?? base.unit;
  if (benchmarks && benchmarks.length > 0) result.benchmarks = benchmarks;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Qualities diff validation.
 * [👤semio📚js💻semio🔖quality🪨qualitiesdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Quality/d/i/QualitiesDiffSchema)
 **/
export const QualitiesDiffSchema = z.object({
  removed: z.array(QualityIdSchema).optional(),
  updated: z.array(z.object({ quality: QualityIdSchema, diff: QualityDiffSchema })).optional(),
  added: z.array(QualitySchema).optional(),
});

// #endregion 🔖Quality

// #region 🔖Port

// [👤semio📚js💻semio🔖port](repo://p/u/semio/b/l/js/f/semio.ts/s/Port)
// Port entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Port validation.
 * [👤semio📚js💻semio🔖port🪨portschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/PortSchema)
 **/
export const PortSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  compatiblePorts: z.array(PortIdSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Port.
 * [👤semio📚js💻semio🔖port🛠️port](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/Port)
 **/
export type Port = z.infer<typeof PortSchema>;
/**
 * Serializes Port for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖port🪨serializeport](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/serializePort)
 **/
export const serializePort = (iface: Port): string => JSON.stringify(PortSchema.parse(iface));
/**
 * Performs the deserializePort operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖port🪨deserializeport](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/deserializePort)
 **/
export const deserializePort = (json: string): Port => PortSchema.parse(JSON.parse(json));

/**
 * Zod schema for Port diff validation.
 * [👤semio📚js💻semio🔖port🪨portdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/PortDiffSchema)
 **/
export const PortDiffSchema = PortSchema.partial()
  .omit({ compatiblePorts: true, attributes: true })
  .extend({
    compatiblePorts: z.array(PortIdSchema).optional(),
    attributes: AttributesDiffSchema.optional(),
    description: z.string().nullable().optional(),
    icon: z.string().nullable().optional(),
  });
/**
 * Diff type for tracking Port changes.
 * [👤semio📚js💻semio🔖port🛠️portdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/PortDiff)
 **/
export type PortDiff = z.infer<typeof PortDiffSchema>;
/**
 * Retrieves the PortDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖port🪨getportdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/getPortDiff)
 **/
export const getPortDiff = (before: Port, after: Port): PortDiff => {
  const diff: PortDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description ?? null;
  if (before.icon !== after.icon) diff.icon = after.icon ?? null;
  if (JSON.stringify(before.compatiblePorts) !== JSON.stringify(after.compatiblePorts)) diff.compatiblePorts = after.compatiblePorts;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inversePort changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖port🪨inverseportdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/inversePortDiff)
 **/
export const inversePortDiff = (original: Port, appliedDiff: PortDiff): PortDiff => {
  const inverse: PortDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.compatiblePorts !== undefined) inverse.compatiblePorts = original.compatiblePorts;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergePort changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖port🪨mergeportdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/mergePortDiff)
 **/
export const mergePortDiff = (diff1: PortDiff, diff2: PortDiff): PortDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
/**
 * Diff type for tracking applyPort changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖port🪨applyportdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/applyPortDiff)
 **/
export const applyPortDiff = (base: Port, diff: PortDiff): Port => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Port = {
    guid: base.guid,
    name: diff.name ?? base.name,
  };

  if ("description" in diff) {
    if (diff.description !== null) result.description = diff.description;
  } else if (base.description !== undefined) {
    result.description = base.description;
  }
  if ("icon" in diff) {
    if (diff.icon !== null) result.icon = diff.icon;
  } else if (base.icon !== undefined) {
    result.icon = base.icon;
  }
  if (diff.compatiblePorts !== undefined || base.compatiblePorts !== undefined) result.compatiblePorts = diff.compatiblePorts ?? base.compatiblePorts;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Ports diff validation.
 * [👤semio📚js💻semio🔖port🪨portsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/PortsDiffSchema)
 **/
export const PortsDiffSchema = z.object({
  removed: z.array(PortIdSchema).optional(),
  updated: z.array(z.object({ port: PortIdSchema, diff: PortDiffSchema })).optional(),
  added: z.array(PortSchema).optional(),
});
/**
 * Diff type for tracking Ports changes.
 * [👤semio📚js💻semio🔖port🛠️portsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/PortsDiff)
 **/
export type PortsDiff = z.infer<typeof PortsDiffSchema>;
/**
 * Retrieves the PortsDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖port🪨getportsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/getPortsDiff)
 **/
export const getPortsDiff = (before: Port[], after: Port[]): PortsDiff => {
  const diff: PortsDiff = {};
  const beforeGuids = new Set(before.map((i) => i.guid));
  const afterGuids = new Set(after.map((i) => i.guid));
  const removed = before.filter((i) => !afterGuids.has(i.guid)).map((i) => ({ guid: i.guid }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((i) => afterGuids.has(i.guid))
    .map((i) => {
      const afterPort = after.find((a) => a.guid === i.guid)!;
      const portDiff = getPortDiff(i, afterPort);
      return { port: { guid: i.guid }, diff: portDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((i) => !beforeGuids.has(i.guid));
  if (added.length > 0) diff.added = added;
  return diff;
};
/**
 * Diff type for tracking inversePorts changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖port🪨inverseportsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/inversePortsDiff)
 **/
export const inversePortsDiff = (original: Port[], appliedDiff: PortsDiff): PortsDiff => {
  const inverse: PortsDiff = {};
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((i) => removedGuids.includes(i.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((i) => ({ guid: i.guid }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalPort = original.find((i) => i.guid === u.port.guid)!;
      return { port: { guid: u.port.guid }, diff: inversePortDiff(originalPort, u.diff) };
    });
  }
  return inverse;
};
/**
 * Diff type for tracking mergePorts changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖port🪨mergeportsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/mergePortsDiff)
 **/
export const mergePortsDiff = (diff1: PortsDiff, diff2: PortsDiff): PortsDiff => {
  return {
    removed: [...(diff1.removed ?? []), ...(diff2.removed ?? [])],
    updated: [...(diff1.updated ?? []), ...(diff2.updated ?? [])],
    added: [...(diff1.added ?? []), ...(diff2.added ?? [])],
  };
};
/**
 * Diff type for tracking applyPorts changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖port🪨applyportsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/applyPortsDiff)
 **/
export const applyPortsDiff = (base: Port[], diff: PortsDiff): Port[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((i) => !removedGuids.has(i.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((i) => i.guid === update.port.guid);
      if (index !== -1) {
        result[index] = applyPortDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

/**
 * Performs the arePortsCompatible operation.
 * MUST return a boolean result.
 * [👤semio📚js💻semio🔖port🪨areportscompatible](repo://p/u/semio/b/l/js/f/semio.ts/s/Port/d/i/arePortsCompatible)
 **/
export const arePortsCompatible = (iface1: Port | undefined, iface2: Port | undefined, allPorts: Port[]): boolean => {
  if (!iface1 || !iface2) return true;
  if (iface1.guid === iface2.guid) return true;
  const iface1Compatible = iface1.compatiblePorts ?? [];
  const iface2Compatible = iface2.compatiblePorts ?? [];
  if (iface1Compatible.length === 0 && iface2Compatible.length === 0) return true;
  if (iface1Compatible.length === 0) return iface2Compatible.some((c) => c.guid === iface1.guid);
  if (iface2Compatible.length === 0) return iface1Compatible.some((c) => c.guid === iface2.guid);
  return iface1Compatible.some((c) => c.guid === iface2.guid) || iface2Compatible.some((c) => c.guid === iface1.guid);
};

// #endregion 🔖Port

// #region 🔖Prop

// [👤semio📚js💻semio🔖prop](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop)
// Prop entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Prop validation.
 * [👤semio📚js💻semio🔖prop🪨propschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/PropSchema)
 **/
export const PropSchema = z.object({
  guid: z.string(),
  quality: QualityIdSchema,
  value: z.string(),
  unit: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Prop.
 * [👤semio📚js💻semio🔖prop🛠️prop](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/Prop)
 **/
export type Prop = z.infer<typeof PropSchema>;
/**
 * Serializes Prop for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖prop🪨serializeprop](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/serializeProp)
 **/
export const serializeProp = (prop: Prop): string => JSON.stringify(PropSchema.parse(prop));
/**
 * Performs the deserializeProp operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖prop🪨deserializeprop](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/deserializeProp)
 **/
export const deserializeProp = (json: string): Prop => PropSchema.parse(JSON.parse(json));

/**
 * Zod schema for Prop diff validation.
 * [👤semio📚js💻semio🔖prop🪨propdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/PropDiffSchema)
 **/
export const PropDiffSchema = PropSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Prop changes.
 * [👤semio📚js💻semio🔖prop🛠️propdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/PropDiff)
 **/
export type PropDiff = z.infer<typeof PropDiffSchema>;
/**
 * Retrieves the PropDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖prop🪨getpropdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/getPropDiff)
 **/
export const getPropDiff = (before: Prop, after: Prop): PropDiff => {
  const diff: PropDiff = {};
  if (before.quality?.guid !== after.quality?.guid) diff.quality = after.quality;
  if (before.value !== after.value) diff.value = after.value;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseProp changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖prop🪨inversepropdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/inversePropDiff)
 **/
export const inversePropDiff = (original: Prop, appliedDiff: PropDiff): PropDiff => {
  const inverse: PropDiff = {};
  if (appliedDiff.quality !== undefined) inverse.quality = original.quality;
  if (appliedDiff.value !== undefined) inverse.value = original.value;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeProp changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖prop🪨mergepropdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/mergePropDiff)
 **/
export const mergePropDiff = (diff1: PropDiff, diff2: PropDiff): PropDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyProp changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖prop🪨applypropdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/applyPropDiff)
 **/
export const applyPropDiff = (base: Prop, diff: PropDiff): Prop => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Prop = {
    guid: base.guid,
    quality: diff.quality ?? base.quality,
    value: diff.value ?? base.value,
  };

  if (diff.unit !== undefined || base.unit !== undefined) result.unit = diff.unit ?? base.unit;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Props diff validation.
 * [👤semio📚js💻semio🔖prop🪨propsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/PropsDiffSchema)
 **/
export const PropsDiffSchema = z.object({
  removed: z.array(PropIdSchema).optional(),
  updated: z.array(z.object({ prop: PropIdSchema, diff: PropDiffSchema })).optional(),
  added: z.array(PropSchema).optional(),
});
/**
 * Diff type for tracking Props changes.
 * [👤semio📚js💻semio🔖prop🛠️propsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/PropsDiff)
 **/
export type PropsDiff = z.infer<typeof PropsDiffSchema>;

// [👤semio📚js💻semio🔖prop🪨getpropsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/getPropsDiff)
// getPropsDiff holds the data fields for a getPropsDiff record.
const getPropsDiff = (before: Prop[], after: Prop[]): PropsDiff => {
  const beforeGuids = new Set(before.map((p) => p.guid));
  const afterGuids = new Set(after.map((p) => p.guid));
  const removed = before.filter((p) => !afterGuids.has(p.guid)).map((p) => ({ guid: p.guid }));
  const added = after.filter((p) => !beforeGuids.has(p.guid));
  const updated = after
    .filter((p) => beforeGuids.has(p.guid))
    .map((afterProp) => {
      const beforeProp = before.find((p) => p.guid === afterProp.guid)!;
      const diff = getPropDiff(beforeProp, afterProp);
      return { prop: { guid: afterProp.guid }, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: PropsDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

// [👤semio📚js💻semio🔖prop🪨inversepropsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/inversePropsDiff)
// inversePropsDiff holds the data fields for a inversePropsDiff record.
const inversePropsDiff = (original: Prop[], appliedDiff: PropsDiff): PropsDiff => {
  const addedGuids = appliedDiff.added?.map((p) => p.guid) ?? [];
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  const updatedGuids = appliedDiff.updated?.map((u) => u.prop.guid) ?? [];
  return {
    removed: addedGuids.map((guid) => ({ guid })),
    added: original.filter((p) => removedGuids.includes(p.guid)),
    updated: updatedGuids.map((guid) => {
      const orig = original.find((p) => p.guid === guid)!;
      const upd = appliedDiff.updated?.find((u) => u.prop.guid === guid)!;
      return { prop: { guid }, diff: inversePropDiff(orig, upd.diff) };
    }),
  };
};

// [👤semio📚js💻semio🔖prop🪨mergepropsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/mergePropsDiff)
// mergePropsDiff holds the data fields for a mergePropsDiff record.
const mergePropsDiff = (first: PropsDiff, second: PropsDiff): PropsDiff => {
  return { ...first, ...second };
};

// [👤semio📚js💻semio🔖prop🪨applypropsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Prop/d/i/applyPropsDiff)
// applyPropsDiff holds the data fields for a applyPropsDiff record.
const applyPropsDiff = (base: Prop[], diff: PropsDiff): Prop[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((prop) => !removedGuids.has(prop.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((prop) => prop.guid === update.prop.guid);
      if (index !== -1) {
        result[index] = applyPropDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

// #endregion 🔖Prop

// #region 🔖Tag

// [👤semio📚js💻semio🔖tag](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag)
// Tag entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Tag validation.
 * [👤semio📚js💻semio🔖tag🪨tagschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/TagSchema)
 **/
export const TagSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Tag.
 * [👤semio📚js💻semio🔖tag🛠️tag](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/Tag)
 **/
export type Tag = z.infer<typeof TagSchema>;
/**
 * Serializes Tag for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖tag🪨serializetag](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/serializeTag)
 **/
export const serializeTag = (tag: Tag): string => JSON.stringify(TagSchema.parse(tag));
/**
 * Performs the deserializeTag operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖tag🪨deserializetag](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/deserializeTag)
 **/
export const deserializeTag = (json: string): Tag => TagSchema.parse(JSON.parse(json));

/**
 * Zod schema for Tag diff validation.
 * [👤semio📚js💻semio🔖tag🪨tagdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/TagDiffSchema)
 **/
export const TagDiffSchema = TagSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
  icon: z.string().nullable().optional(),
});
/**
 * Diff type for tracking Tag changes.
 * [👤semio📚js💻semio🔖tag🛠️tagdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/TagDiff)
 **/
export type TagDiff = z.infer<typeof TagDiffSchema>;
/**
 * Retrieves the TagDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖tag🪨gettagdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/getTagDiff)
 **/
export const getTagDiff = (before: Tag, after: Tag): TagDiff => {
  const diff: TagDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description ?? null;
  if (before.icon !== after.icon) diff.icon = after.icon ?? null;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseTag changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖tag🪨inversetagdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/inverseTagDiff)
 **/
export const inverseTagDiff = (original: Tag, appliedDiff: TagDiff): TagDiff => {
  const inverse: TagDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeTag changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖tag🪨mergetagdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/mergeTagDiff)
 **/
export const mergeTagDiff = (diff1: TagDiff, diff2: TagDiff): TagDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyTag changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖tag🪨applytagdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/applyTagDiff)
 **/
export const applyTagDiff = (base: Tag, diff: TagDiff): Tag => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Tag = {
    guid: base.guid,
    name: "name" in diff && diff.name !== undefined ? diff.name : base.name,
  };

  if ("description" in diff) {
    const value = diff.description ?? undefined;
    if (value !== undefined) result.description = value;
  } else if (base.description !== undefined) {
    result.description = base.description;
  }
  if ("icon" in diff) {
    const value = diff.icon ?? undefined;
    if (value !== undefined) result.icon = value;
  } else if (base.icon !== undefined) {
    result.icon = base.icon;
  }
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Tags diff validation.
 * [👤semio📚js💻semio🔖tag🪨tagsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/TagsDiffSchema)
 **/
export const TagsDiffSchema = z.object({
  removed: z.array(TagIdSchema).optional(),
  updated: z.array(z.object({ tag: TagIdSchema, diff: TagDiffSchema })).optional(),
  added: z.array(TagSchema).optional(),
});
/**
 * Diff type for tracking Tags changes.
 * [👤semio📚js💻semio🔖tag🛠️tagsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/TagsDiff)
 **/
export type TagsDiff = z.infer<typeof TagsDiffSchema>;
/**
 * Retrieves the TagsDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖tag🪨gettagsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/getTagsDiff)
 **/
export const getTagsDiff = (before: Tag[], after: Tag[]): TagsDiff => {
  const diff: TagsDiff = {};
  const beforeGuids = new Set(before.map((t) => t.guid));
  const afterGuids = new Set(after.map((t) => t.guid));
  const removed = before.filter((t) => !afterGuids.has(t.guid)).map((t) => ({ guid: t.guid }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((t) => afterGuids.has(t.guid))
    .map((t) => {
      const afterTag = after.find((a) => a.guid === t.guid)!;
      const tagDiff = getTagDiff(t, afterTag);
      return { tag: { guid: t.guid }, diff: tagDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((t) => !beforeGuids.has(t.guid));
  if (added.length > 0) diff.added = added;
  return diff;
};
/**
 * Diff type for tracking inverseTags changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖tag🪨inversetagsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/inverseTagsDiff)
 **/
export const inverseTagsDiff = (original: Tag[], appliedDiff: TagsDiff): TagsDiff => {
  const inverse: TagsDiff = {};
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((t) => removedGuids.includes(t.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((t) => ({ guid: t.guid }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalTag = original.find((t) => t.guid === u.tag.guid)!;
      return { tag: { guid: u.tag.guid }, diff: inverseTagDiff(originalTag, u.diff) };
    });
  }
  return inverse;
};
/**
 * Diff type for tracking mergeTags changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖tag🪨mergetagsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/mergeTagsDiff)
 **/
export const mergeTagsDiff = (diff1: TagsDiff, diff2: TagsDiff): TagsDiff => {
  const removed = [...(diff1.removed ?? []), ...(diff2.removed ?? [])];
  const added = [...(diff1.added ?? []), ...(diff2.added ?? [])];
  const updated1Map = new Map((diff1.updated ?? []).map((u) => [u.tag.guid, u.diff]));
  const updated2Map = new Map((diff2.updated ?? []).map((u) => [u.tag.guid, u.diff]));
  const allGuids = new Set([...updated1Map.keys(), ...updated2Map.keys()]);
  const updated = Array.from(allGuids).map((guid) => ({
    tag: { guid },
    diff: mergeTagDiff(updated1Map.get(guid) ?? {}, updated2Map.get(guid) ?? {}),
  }));
  return {
    removed: removed.length > 0 ? removed : undefined,
    updated: updated.length > 0 ? updated : undefined,
    added: added.length > 0 ? added : undefined,
  };
};
/**
 * Diff type for tracking applyTags changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖tag🪨applytagsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/applyTagsDiff)
 **/
export const applyTagsDiff = (base: Tag[], diff: TagsDiff): Tag[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((t) => !removedGuids.has(t.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((t) => t.guid === update.tag.guid);
      if (index !== -1) {
        result[index] = applyTagDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

/**
 * Searches for matching Tag entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖tag🪨findtag](repo://p/u/semio/b/l/js/f/semio.ts/s/Tag/d/i/findTag)
 **/
export const findTag = (tags: Tag[], guid: string): Tag => {
  const tag = tags.find((t) => t.guid === guid);
  if (!tag) throw new Error(`Tag ${guid} not found`);
  return tag;
};

// #endregion 🔖Tag

// #region 🔖Concept

// [👤semio📚js💻semio🔖concept](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept)
// Concept entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Concept validation.
 * [👤semio📚js💻semio🔖concept🪨conceptschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/ConceptSchema)
 **/
export const ConceptSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Concept.
 * [👤semio📚js💻semio🔖concept🛠️concept](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/Concept)
 **/
export type Concept = z.infer<typeof ConceptSchema>;
/**
 * Serializes Concept for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖concept🪨serializeconcept](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/serializeConcept)
 **/
export const serializeConcept = (concept: Concept): string => JSON.stringify(ConceptSchema.parse(concept));
/**
 * Performs the deserializeConcept operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖concept🪨deserializeconcept](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/deserializeConcept)
 **/
export const deserializeConcept = (json: string): Concept => ConceptSchema.parse(JSON.parse(json));

/**
 * Zod schema for Concept diff validation.
 * [👤semio📚js💻semio🔖concept🪨conceptdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/ConceptDiffSchema)
 **/
export const ConceptDiffSchema = ConceptSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
  icon: z.string().nullable().optional(),
});
/**
 * Diff type for tracking Concept changes.
 * [👤semio📚js💻semio🔖concept🛠️conceptdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/ConceptDiff)
 **/
export type ConceptDiff = z.infer<typeof ConceptDiffSchema>;
/**
 * Retrieves the ConceptDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖concept🪨getconceptdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/getConceptDiff)
 **/
export const getConceptDiff = (before: Concept, after: Concept): ConceptDiff => {
  const diff: ConceptDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description ?? null;
  if (before.icon !== after.icon) diff.icon = after.icon ?? null;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseConcept changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖concept🪨inverseconceptdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/inverseConceptDiff)
 **/
export const inverseConceptDiff = (original: Concept, appliedDiff: ConceptDiff): ConceptDiff => {
  const inverse: ConceptDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeConcept changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖concept🪨mergeconceptdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/mergeConceptDiff)
 **/
export const mergeConceptDiff = (diff1: ConceptDiff, diff2: ConceptDiff): ConceptDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyConcept changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖concept🪨applyconceptdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/applyConceptDiff)
 **/
export const applyConceptDiff = (base: Concept, diff: ConceptDiff): Concept => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Concept = {
    guid: base.guid,
    name: "name" in diff && diff.name !== undefined ? diff.name : base.name,
  };

  if ("description" in diff) {
    const value = diff.description ?? undefined;
    if (value !== undefined) result.description = value;
  } else if (base.description !== undefined) {
    result.description = base.description;
  }
  if ("icon" in diff) {
    const value = diff.icon ?? undefined;
    if (value !== undefined) result.icon = value;
  } else if (base.icon !== undefined) {
    result.icon = base.icon;
  }
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Concepts diff validation.
 * [👤semio📚js💻semio🔖concept🪨conceptsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/ConceptsDiffSchema)
 **/
export const ConceptsDiffSchema = z.object({
  removed: z.array(ConceptIdSchema).optional(),
  updated: z.array(z.object({ concept: ConceptIdSchema, diff: ConceptDiffSchema })).optional(),
  added: z.array(ConceptSchema).optional(),
});
/**
 * Diff type for tracking Concepts changes.
 * [👤semio📚js💻semio🔖concept🛠️conceptsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/ConceptsDiff)
 **/
export type ConceptsDiff = z.infer<typeof ConceptsDiffSchema>;
/**
 * Retrieves the ConceptsDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖concept🪨getconceptsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/getConceptsDiff)
 **/
export const getConceptsDiff = (before: Concept[], after: Concept[]): ConceptsDiff => {
  const diff: ConceptsDiff = {};
  const beforeGuids = new Set(before.map((c) => c.guid));
  const afterGuids = new Set(after.map((c) => c.guid));
  const removed = before.filter((c) => !afterGuids.has(c.guid)).map((c) => ({ guid: c.guid }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((c) => afterGuids.has(c.guid))
    .map((c) => {
      const afterConcept = after.find((a) => a.guid === c.guid)!;
      const conceptDiff = getConceptDiff(c, afterConcept);
      return { concept: { guid: c.guid }, diff: conceptDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((c) => !beforeGuids.has(c.guid));
  if (added.length > 0) diff.added = added;
  return diff;
};
/**
 * Diff type for tracking inverseConcepts changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖concept🪨inverseconceptsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/inverseConceptsDiff)
 **/
export const inverseConceptsDiff = (original: Concept[], appliedDiff: ConceptsDiff): ConceptsDiff => {
  const inverse: ConceptsDiff = {};
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((c) => removedGuids.includes(c.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((c) => ({ guid: c.guid }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalConcept = original.find((c) => c.guid === u.concept.guid)!;
      return { concept: { guid: u.concept.guid }, diff: inverseConceptDiff(originalConcept, u.diff) };
    });
  }
  return inverse;
};
/**
 * Diff type for tracking mergeConcepts changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖concept🪨mergeconceptsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/mergeConceptsDiff)
 **/
export const mergeConceptsDiff = (diff1: ConceptsDiff, diff2: ConceptsDiff): ConceptsDiff => {
  const removed = [...(diff1.removed ?? []), ...(diff2.removed ?? [])];
  const added = [...(diff1.added ?? []), ...(diff2.added ?? [])];
  const updated1Map = new Map((diff1.updated ?? []).map((u) => [u.concept.guid, u.diff]));
  const updated2Map = new Map((diff2.updated ?? []).map((u) => [u.concept.guid, u.diff]));
  const allGuids = new Set([...updated1Map.keys(), ...updated2Map.keys()]);
  const updated = Array.from(allGuids).map((guid) => ({
    concept: { guid },
    diff: mergeConceptDiff(updated1Map.get(guid) ?? {}, updated2Map.get(guid) ?? {}),
  }));
  return {
    removed: removed.length > 0 ? removed : undefined,
    updated: updated.length > 0 ? updated : undefined,
    added: added.length > 0 ? added : undefined,
  };
};
/**
 * Diff type for tracking applyConcepts changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖concept🪨applyconceptsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/applyConceptsDiff)
 **/
export const applyConceptsDiff = (base: Concept[], diff: ConceptsDiff): Concept[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((c) => !removedGuids.has(c.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((c) => c.guid === update.concept.guid);
      if (index !== -1) {
        result[index] = applyConceptDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

/**
 * Searches for matching Concept entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖concept🪨findconcept](repo://p/u/semio/b/l/js/f/semio.ts/s/Concept/d/i/findConcept)
 **/
export const findConcept = (concepts: Concept[], guid: string): Concept => {
  const concept = concepts.find((c) => c.guid === guid);
  if (!concept) throw new Error(`Concept ${guid} not found`);
  return concept;
};

// #endregion 🔖Concept

// #region 🔖Model

// [👤semio📚js💻semio🔖model](repo://p/u/semio/b/l/js/f/semio.ts/s/Model)
// Model entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Model validation.
 * [👤semio📚js💻semio🔖model🪨modelschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/ModelSchema)
 **/
export const ModelSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  tags: z.array(TagIdSchema).optional(),
  file: FileIdSchema,
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Model.
 * [👤semio📚js💻semio🔖model🛠️model](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/Model)
 **/
export type Model = z.infer<typeof ModelSchema>;
/**
 * Serializes Model for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖model🪨serializemodel](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/serializeModel)
 **/
export const serializeModel = (model: Model): string => JSON.stringify(ModelSchema.parse(model));
/**
 * Performs the deserializeModel operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖model🪨deserializemodel](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/deserializeModel)
 **/
export const deserializeModel = (json: string): Model => ModelSchema.parse(JSON.parse(json));

/**
 * Zod schema for Model diff validation.
 * [👤semio📚js💻semio🔖model🪨modeldiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/ModelDiffSchema)
 **/
export const ModelDiffSchema = ModelSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Model changes.
 * [👤semio📚js💻semio🔖model🛠️modeldiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/ModelDiff)
 **/
export type ModelDiff = z.infer<typeof ModelDiffSchema>;
/**
 * Retrieves the ModelDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖model🪨getmodeldiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/getModelDiff)
 **/
export const getModelDiff = (before: Model, after: Model): ModelDiff => {
  const diff: ModelDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (JSON.stringify(before.tags) !== JSON.stringify(after.tags)) diff.tags = after.tags;
  if (before.file.guid !== after.file.guid) diff.file = after.file;
  if (before.description !== after.description) diff.description = after.description;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseModel changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖model🪨inversemodeldiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/inverseModelDiff)
 **/
export const inverseModelDiff = (original: Model, appliedDiff: ModelDiff): ModelDiff => {
  const inverse: ModelDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.tags !== undefined) inverse.tags = original.tags;
  if (appliedDiff.file !== undefined) inverse.file = original.file;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeModel changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖model🪨mergemodeldiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/mergeModelDiff)
 **/
export const mergeModelDiff = (diff1: ModelDiff, diff2: ModelDiff): ModelDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyModel changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖model🪨applymodeldiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/applyModelDiff)
 **/
export const applyModelDiff = (base: Model, diff: ModelDiff): Model => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Model = {
    guid: base.guid,
    file: diff.file ?? base.file,
  };

  if (diff.name !== undefined || base.name !== undefined) result.name = diff.name ?? base.name;
  if (diff.tags !== undefined || base.tags !== undefined) result.tags = diff.tags ?? base.tags;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Models diff validation.
 * [👤semio📚js💻semio🔖model🪨modelsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/ModelsDiffSchema)
 **/
export const ModelsDiffSchema = z.object({
  removed: z.array(ModelIdSchema).optional(),
  updated: z.array(z.object({ model: ModelIdSchema, diff: ModelDiffSchema })).optional(),
  added: z.array(ModelSchema).optional(),
});

/**
 * Equality check for Model values.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖model🪨aresamemodel](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/areSameModel)
 **/
export const areSameModel = (model: Model, other: Model): boolean => {
  const modelTagGuids = model.tags?.map((t) => t.guid) ?? [];
  const otherTagGuids = other.tags?.map((t) => t.guid) ?? [];
  return modelTagGuids.every((guid) => otherTagGuids.includes(guid));
};

/**
 * Searches for matching Model entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖model🪨findmodel](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/findModel)
 **/
export const findModel = (models: Model[], tagGuids: string[]): Model => {
  const indices = models.map((r) =>
    jaccard(
      r.tags?.map((t) => t.guid),
      tagGuids,
    ),
  );
  const maxIndex = Math.max(...indices);
  const maxIndexIndex = indices.indexOf(maxIndex);
  return models[maxIndexIndex];
};

/**
 * Retrieves the AllTagGuidsFromModels value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖model🪨getalltagguidsfrommodels](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/getAllTagGuidsFromModels)
 **/
export const getAllTagGuidsFromModels = (models: Model[]): string[] => {
  const tagsSet = new Set<string>();
  models.forEach((r) => {
    toArray(r.tags).forEach((tag) => tagsSet.add(tag.guid));
  });
  return Array.from(tagsSet).sort();
};

/**
 * Performs the filterModelsByTagGuids operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖model🪨filtermodelsbytagguids](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/filterModelsByTagGuids)
 **/
export const filterModelsByTagGuids = (models: Model[], selectedTagGuids: string[]): Model[] => {
  if (!selectedTagGuids || selectedTagGuids.length === 0) return models;
  return models.filter((r) => {
    if (!r.tags || r.tags.length === 0) return false;
    const modelTagGuids = r.tags.map((t) => t.guid);
    return selectedTagGuids.every((guid) => modelTagGuids.includes(guid));
  });
};

/**
 * Retrieves the AvailableTagGuidsForModels value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖model🪨getavailabletagguidsformodels](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/getAvailableTagGuidsForModels)
 **/
export const getAvailableTagGuidsForModels = (models: Model[], selectedTagGuids: string[]): string[] => {
  const filteredReps = filterModelsByTagGuids(models, selectedTagGuids);
  const availableTags = getAllTagGuidsFromModels(filteredReps);
  return availableTags.filter((guid) => !selectedTagGuids.includes(guid));
};

/**
 * Performs the selectBestModel operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖model🪨selectbestmodel](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/selectBestModel)
 **/
export const selectBestModel = (models: Model[], selectedTagGuids: string[]): Model | undefined => {
  if (models.length === 0) return undefined;
  if (selectedTagGuids.length === 0) {
    const defaultRep = models.find((r) => !r.tags || r.tags.length === 0);
    return defaultRep ?? models[0];
  }
  const filteredReps = filterModelsByTagGuids(models, selectedTagGuids);
  if (filteredReps.length === 0) return undefined;
  return findModel(filteredReps, selectedTagGuids);
};

/**
 * Constant value for SUPPORTED_3D_EXTENSIONS.
 * [👤semio📚js💻semio🔖model🪨supported3dextensions](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/SUPPORTED_3D_EXTENSIONS)
 **/
export const SUPPORTED_3D_EXTENSIONS = [
  "gltf",
  "glb",

  "fbx",

  "obj",

  "dae",

  "3ds",

  "stl",

  "ply",

  "usdz",

  "vrm",

  "ifc",

  "3mf",

  "amf",

  "bvh",

  "drc",

  "ktx2",

  "ldr",
  "mpd",

  "json",

  "pmd",
  "pmx",
  "vmd",

  "pcd",

  "pdb",

  "svg",

  "tilt",

  "vox",

  "wrl",

  "xyz",
] as const;

/**
 * Type alias for Supported3DExtension.
 * [👤semio📚js💻semio🔖model🛠️supported3dextension](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/Supported3DExtension)
 **/
export type Supported3DExtension = (typeof SUPPORTED_3D_EXTENSIONS)[number];

/**
 * Performs the isSupportedModelExtension operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖model🪨issupportedmodelextension](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/isSupportedModelExtension)
 **/
export const isSupportedModelExtension = (filename: string): boolean => {
  const ext = filename.split(".").pop()?.toLowerCase() ?? "";
  return SUPPORTED_3D_EXTENSIONS.includes(ext as Supported3DExtension);
};

/**
 * Interface defining ModelFileValidation structure.
 * [👤semio📚js💻semio🔖model🛠️modelfilevalidation](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/ModelFileValidation)
 **/
export interface ModelFileValidation {
  isValid: boolean;
  warning?: string;
  extension?: string;
}

/**
 * Validates ModelFile against constraints.
 * MUST check all constraints and return problems.
 * [👤semio📚js💻semio🔖model🪨validatemodelfile](repo://p/u/semio/b/l/js/f/semio.ts/s/Model/d/i/validateModelFile)
 **/
export const validateModelFile = (filename: string): ModelFileValidation => {
  const ext = filename.split(".").pop()?.toLowerCase() ?? "";
  if (!ext) {
    return { isValid: false, warning: "File has no extension" };
  }
  if (!isSupportedModelExtension(filename)) {
    return {
      isValid: true,
      warning: `File extension '.${ext}' is not a common 3D format. Supported: ${SUPPORTED_3D_EXTENSIONS.slice(0, 5).join(", ")}...`,
      extension: ext,
    };
  }
  return { isValid: true, extension: ext };
};

// #endregion 🔖Model

// #region 🔖Connector

// [👤semio📚js💻semio🔖connector](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector)
// Connector entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Connector validation.
 * [👤semio📚js💻semio🔖connector🪨connectorschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/ConnectorSchema)
 **/
export const ConnectorSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  t: z.number(),
  point: PointSchema,
  direction: VectorSchema,
  description: z.string().optional(),
  port: PortIdSchema.optional(),
  mandatory: z.boolean().optional(),
  props: z.array(PropSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Connector.
 * [👤semio📚js💻semio🔖connector🛠️connector](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/Connector)
 **/
export type Connector = z.infer<typeof ConnectorSchema>;
/**
 * Serializes Connector for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖connector🪨serializeconnector](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/serializeConnector)
 **/
export const serializeConnector = (connector: Connector): string => JSON.stringify(ConnectorSchema.parse(connector));
/**
 * Performs the deserializeConnector operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖connector🪨deserializeconnector](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/deserializeConnector)
 **/
export const deserializeConnector = (json: string): Connector => ConnectorSchema.parse(JSON.parse(json));

/**
 * Zod schema for Connector diff validation.
 * [👤semio📚js💻semio🔖connector🪨connectordiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/ConnectorDiffSchema)
 **/
export const ConnectorDiffSchema = ConnectorSchema.partial().omit({ point: true, direction: true, props: true, attributes: true }).extend({
  point: PointDiffSchema.optional(),
  direction: VectorDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Connector changes.
 * [👤semio📚js💻semio🔖connector🛠️connectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/ConnectorDiff)
 **/
export type ConnectorDiff = z.infer<typeof ConnectorDiffSchema>;
/**
 * Retrieves the ConnectorDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖connector🪨getconnectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/getConnectorDiff)
 **/
export const getConnectorDiff = (before: Connector, after: Connector): ConnectorDiff => {
  const diff: ConnectorDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description;
  if (before.port?.guid !== after.port?.guid) diff.port = after.port;
  if (before.mandatory !== after.mandatory) diff.mandatory = after.mandatory;
  if (before.t !== after.t) diff.t = after.t;
  if (!deepEqual(before.point, after.point)) diff.point = getPointDiff(before.point, after.point);
  if (!deepEqual(before.direction, after.direction)) diff.direction = getVectorDiff(before.direction, after.direction);
  if (!deepEqual(before.props, after.props)) diff.props = getPropsDiff(before.props ?? [], after.props ?? []);
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking mergeConnector changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖connector🪨mergeconnectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/mergeConnectorDiff)
 **/
export const mergeConnectorDiff = (diff1: ConnectorDiff, diff2: ConnectorDiff): ConnectorDiff => {
  return {
    ...diff1,
    ...diff2,
    point: diff2.point ?? diff1.point,
    direction: diff2.direction ?? diff1.direction,
    props: diff1.props && diff2.props ? mergePropsDiff(diff1.props, diff2.props) : (diff2.props ?? diff1.props),
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
/**
 * Diff type for tracking inverseConnector changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖connector🪨inverseconnectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/inverseConnectorDiff)
 **/
export const inverseConnectorDiff = (original: Connector, appliedDiff: ConnectorDiff): ConnectorDiff => {
  const inverse: ConnectorDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.port !== undefined) inverse.port = original.port;
  if (appliedDiff.mandatory !== undefined) inverse.mandatory = original.mandatory;
  if (appliedDiff.t !== undefined) inverse.t = original.t;
  if (appliedDiff.point !== undefined) inverse.point = inversePointDiff(original.point, appliedDiff.point);
  if (appliedDiff.direction !== undefined) inverse.direction = inverseVectorDiff(original.direction, appliedDiff.direction);
  if (appliedDiff.props !== undefined) inverse.props = inversePropsDiff(original.props ?? [], appliedDiff.props);
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking applyConnector changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖connector🪨applyconnectordiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/applyConnectorDiff)
 **/
export const applyConnectorDiff = (base: Connector, diff: ConnectorDiff): Connector => {
  const props = diff.props ? applyPropsDiff(base.props ?? [], diff.props) : base.props;
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Connector = {
    guid: base.guid,
    t: diff.t ?? base.t,
    point: diff.point ? applyPointDiff(base.point, diff.point) : base.point,
    direction: diff.direction ? applyVectorDiff(base.direction, diff.direction) : base.direction,
  };

  if (diff.name !== undefined || base.name !== undefined) result.name = diff.name ?? base.name;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (diff.port !== undefined || base.port !== undefined) result.port = diff.port ?? base.port;
  if (diff.mandatory !== undefined || base.mandatory !== undefined) result.mandatory = diff.mandatory ?? base.mandatory;
  if (props && props.length > 0) result.props = props;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Connectors diff validation.
 * [👤semio📚js💻semio🔖connector🪨connectorsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/ConnectorsDiffSchema)
 **/
export const ConnectorsDiffSchema = z.object({
  removed: z.array(ConnectorIdSchema).optional(),
  updated: z.array(z.object({ connector: ConnectorIdSchema, diff: ConnectorDiffSchema })).optional(),
  added: z.array(ConnectorSchema).optional(),
});
/**
 * Diff type for tracking Connectors changes.
 * [👤semio📚js💻semio🔖connector🛠️connectorsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/ConnectorsDiff)
 **/
export type ConnectorsDiff = z.infer<typeof ConnectorsDiffSchema>;

// [👤semio📚js💻semio🔖connector🪨getconnectorsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/getConnectorsDiff)
// getConnectorsDiff holds the data fields for a getConnectorsDiff record.
const getConnectorsDiff = (before: Connector[], after: Connector[]): ConnectorsDiff => {
  const beforeGuids = new Set(before.map((p) => p.guid));
  const afterGuids = new Set(after.map((p) => p.guid));
  const removed = before.filter((p) => !afterGuids.has(p.guid)).map((p) => ({ guid: p.guid }));
  const added = after.filter((p) => !beforeGuids.has(p.guid));
  const updated = after
    .filter((p) => beforeGuids.has(p.guid))
    .map((afterPort) => {
      const beforePort = before.find((p) => p.guid === afterPort.guid)!;
      const diff = getConnectorDiff(beforePort, afterPort);
      return { connector: { guid: afterPort.guid }, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: ConnectorsDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

/**
 * Performs the unifyConnectorPortsAndCompatiblePortsForTypes operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖connector🪨unifyconnectorportsandcompatibleportsfortypes](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/unifyConnectorPortsAndCompatiblePortsForTypes)
 **/
export const unifyConnectorPortsAndCompatiblePortsForTypes = (types: Type[]): TypesDiff => {
  return { updated: [] };
};

/**
 * Performs the areConnectorsCompatible operation.
 * MUST return a boolean result.
 * [👤semio📚js💻semio🔖connector🪨areconnectorscompatible](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/areConnectorsCompatible)
 **/
export const areConnectorsCompatible = (connector: Connector, otherPort: Connector): boolean => {
  return true;
};

/**
 * Searches for matching Connector entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖connector🪨findconnector](repo://p/u/semio/b/l/js/f/semio.ts/s/Connector/d/i/findConnector)
 **/
export const findConnector = (connectors: Connector[], connectorGuid: string): Connector => {
  const connector = connectors.find((p) => p.guid === connectorGuid);
  if (!connector) throw new Error(`Connector ${connectorGuid} not found in connectors`);
  return connector;
};

// #endregion 🔖Connector

// #region 🔖Type

// [👤semio📚js💻semio🔖type](repo://p/u/semio/b/l/js/f/semio.ts/s/Type)
// Type entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Type validation.
 * [👤semio📚js💻semio🔖type🪨typeschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/TypeSchema)
 **/
export const TypeSchema = z.object({
  guid: z.string(),
  name: z.string(),
  parent: TypeIdSchema.optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  models: z.array(ModelSchema).optional(),
  connectors: z.array(ConnectorSchema).optional(),
  props: z.array(PropSchema).optional(),
  stock: z.number().optional(),
  virtual: z.boolean().optional(),
  unit: z.string().optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
  location: LocationIdSchema.optional(),
  authors: z.array(AuthorIdSchema).optional(),
  concepts: z.array(ConceptIdSchema).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Type.
 * [👤semio📚js💻semio🔖type🛠️type](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/Type)
 **/
export type Type = z.infer<typeof TypeSchema>;
/**
 * Serializes Type for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖type🪨serializetype](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/serializeType)
 **/
export const serializeType = (type: Type): string => JSON.stringify(TypeSchema.parse(type));
/**
 * Performs the deserializeType operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖type🪨deserializetype](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/deserializeType)
 **/
export const deserializeType = (json: string): Type => TypeSchema.parse(JSON.parse(json));

/**
 * Definition of TypeShallowSchema.
 * [👤semio📚js💻semio🔖type🪨typeshallowschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/TypeShallowSchema)
 **/
export const TypeShallowSchema = TypeSchema.omit({ models: true, connectors: true }).extend({
  models: z.array(z.string()).optional(),
  connectors: z.array(z.string()).optional(),
});
/**
 * Type alias for TypeShallow.
 * [👤semio📚js💻semio🔖type🛠️typeshallow](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/TypeShallow)
 **/
export type TypeShallow = z.infer<typeof TypeShallowSchema>;
/**
 * Serializes TypeShallow for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖type🪨serializetypeshallow](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/serializeTypeShallow)
 **/
export const serializeTypeShallow = (type: TypeShallow): string => JSON.stringify(TypeShallowSchema.parse(type));
/**
 * Performs the deserializeTypeShallow operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖type🪨deserializetypeshallow](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/deserializeTypeShallow)
 **/
export const deserializeTypeShallow = (json: string): TypeShallow => TypeShallowSchema.parse(JSON.parse(json));
/**
 * Zod schema for Type diff validation.
 * [👤semio📚js💻semio🔖type🪨typediffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/TypeDiffSchema)
 **/
export const TypeDiffSchema = TypeSchema.partial()
  .omit({ models: true, connectors: true, props: true, attributes: true })
  .extend({
    models: ModelsDiffSchema.optional(),
    connectors: ConnectorsDiffSchema.optional(),
    props: PropsDiffSchema.optional(),
    attributes: AttributesDiffSchema.optional(),
    description: z.string().nullable().optional(),
    icon: z.string().nullable().optional(),
    image: z.string().nullable().optional(),
    location: LocationIdSchema.nullable().optional(),
    folder: z.string().nullable().optional(),
    concepts: z.array(ConceptIdSchema).nullable().optional(),
    authors: z.array(AuthorIdSchema).nullable().optional(),
    parent: TypeIdSchema.nullable().optional(),
  });
/**
 * [👤semio📚js💻semio🔖type🛠️typediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/TypeDiff)
 **/
export type TypeDiff = z.infer<typeof TypeDiffSchema>;
/**
 * Retrieves the TypeDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖type🪨gettypediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/getTypeDiff)
 **/
export const getTypeDiff = (before: Type, after: Type): TypeDiff => {
  const diff: TypeDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.parent?.guid !== after.parent?.guid) diff.parent = after.parent;
  if (before.isAbstract !== after.isAbstract) diff.isAbstract = after.isAbstract;
  if (before.folder !== after.folder) diff.folder = after.folder;
  if (before.stock !== after.stock) diff.stock = after.stock;
  if (before.virtual !== after.virtual) diff.virtual = after.virtual;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (before.location?.guid !== after.location?.guid) diff.location = after.location;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.image !== after.image) diff.image = after.image;
  if (before.description !== after.description) diff.description = after.description;
  if (!arraysEqual(before.authors, after.authors)) diff.authors = after.authors;
  if (!arraysEqual(before.concepts, after.concepts)) diff.concepts = after.concepts;
  const modelsDiff = getCollectionDiff("model", before.models ?? [], after.models ?? [], getModelDiff);
  if (Object.keys(modelsDiff).length > 0) diff.models = modelsDiff;
  const connectorsDiff = getCollectionDiff("connector", before.connectors ?? [], after.connectors ?? [], getConnectorDiff);
  if (Object.keys(connectorsDiff).length > 0) diff.connectors = connectorsDiff;
  const propsDiff = getCollectionDiff("prop", before.props ?? [], after.props ?? [], getPropDiff);
  if (Object.keys(propsDiff).length > 0) diff.props = propsDiff;
  const attributesDiff = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
  return diff;
};

/**
 * Diff type for tracking applyType changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖type🪨applytypediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/applyTypeDiff)
 **/
export const applyTypeDiff = (base: Type, diff: TypeDiff): Type => {
  const models = diff.models || base.models ? applyCollectionDiff("model", base.models ?? [], diff.models, applyModelDiff) : undefined;
  const connectors = diff.connectors || base.connectors ? applyCollectionDiff("connector", base.connectors ?? [], diff.connectors, applyConnectorDiff) : undefined;
  const props = diff.props || base.props ? applyCollectionDiff("prop", base.props ?? [], diff.props, applyPropDiff) : undefined;
  const attributes = diff.attributes || base.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes ?? {}) : undefined;

  const result: Type = {
    guid: base.guid,
    name: diff.name ?? base.name,
    isAbstract: diff.isAbstract ?? base.isAbstract,
    createdAt: diff.createdAt ?? base.createdAt,
    updatedAt: diff.updatedAt ?? base.updatedAt,
  };

  if (diff.parent !== undefined ? (diff.parent ?? undefined) : base.parent) result.parent = diff.parent !== undefined ? (diff.parent ?? undefined) : base.parent;
  if (diff.folder !== undefined ? (diff.folder ?? undefined) : base.folder) result.folder = diff.folder !== undefined ? (diff.folder ?? undefined) : base.folder;
  if (diff.stock !== undefined ? diff.stock : base.stock) result.stock = diff.stock !== undefined ? diff.stock : base.stock;
  if (diff.virtual ?? base.virtual) result.virtual = diff.virtual ?? base.virtual;
  if (diff.unit !== undefined ? diff.unit : base.unit) result.unit = diff.unit !== undefined ? diff.unit : base.unit;
  if (diff.location !== undefined ? (diff.location ?? undefined) : base.location) result.location = diff.location !== undefined ? (diff.location ?? undefined) : base.location;
  if (diff.icon !== undefined ? (diff.icon ?? undefined) : base.icon) result.icon = diff.icon !== undefined ? (diff.icon ?? undefined) : base.icon;
  if (diff.image !== undefined ? (diff.image ?? undefined) : base.image) result.image = diff.image !== undefined ? (diff.image ?? undefined) : base.image;
  if (diff.description !== undefined ? (diff.description ?? undefined) : base.description) result.description = diff.description !== undefined ? (diff.description ?? undefined) : base.description;
  if (diff.authors !== undefined ? (diff.authors ?? undefined) : base.authors) result.authors = diff.authors !== undefined ? (diff.authors ?? undefined) : base.authors;
  if (diff.concepts !== undefined ? (diff.concepts ?? undefined) : base.concepts) result.concepts = diff.concepts !== undefined ? (diff.concepts ?? undefined) : base.concepts;

  if (models && models.length > 0) result.models = models;
  if (connectors && connectors.length > 0) result.connectors = connectors;
  if (props && props.length > 0) result.props = props;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Diff type for tracking mergeType changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖type🪨mergetypediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/mergeTypeDiff)
 **/
export const mergeTypeDiff = (diff1: TypeDiff, diff2: TypeDiff): TypeDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};

/**
 * Diff type for tracking inverseType changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖type🪨inversetypediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/inverseTypeDiff)
 **/
export const inverseTypeDiff = (original: Type, appliedDiff: TypeDiff): TypeDiff => {
  const inverse: TypeDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.parent !== undefined) inverse.parent = original.parent ?? null;
  if (appliedDiff.isAbstract !== undefined) inverse.isAbstract = original.isAbstract;
  if (appliedDiff.folder !== undefined) inverse.folder = original.folder ?? null;
  if (appliedDiff.stock !== undefined) inverse.stock = original.stock;
  if (appliedDiff.virtual !== undefined) inverse.virtual = original.virtual;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.location !== undefined) inverse.location = original.location ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.image !== undefined) inverse.image = original.image ?? null;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.authors !== undefined) inverse.authors = original.authors ?? null;
  if (appliedDiff.concepts !== undefined) inverse.concepts = original.concepts ?? null;
  if (appliedDiff.models) inverse.models = inverseCollectionDiff("model", original.models ?? [], appliedDiff.models, inverseModelDiff);
  if (appliedDiff.connectors) inverse.connectors = inverseCollectionDiff("connector", original.connectors ?? [], appliedDiff.connectors, inverseConnectorDiff);
  if (appliedDiff.props) inverse.props = inverseCollectionDiff("prop", original.props ?? [], appliedDiff.props, inversePropDiff);
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};

/**
 * Zod schema for Types diff validation.
 * [👤semio📚js💻semio🔖type🪨typesdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/TypesDiffSchema)
 **/
export const TypesDiffSchema = z.object({
  removed: z.array(TypeIdSchema).optional(),
  updated: z.array(z.object({ type: TypeIdSchema, diff: TypeDiffSchema })).optional(),
  added: z.array(TypeSchema).optional(),
});
/**
 * Diff type for tracking Types changes.
 * [👤semio📚js💻semio🔖type🛠️typesdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/TypesDiff)
 **/
export type TypesDiff = z.infer<typeof TypesDiffSchema>;

/**
 * Searches for matching ConnectorInType entry.
 * MUST return the matching element or undefined. [👤semio📚js💻semio🔖type🪨findconnectorintype](repo://p/u/semio/b/l/js/f/semio.ts/s/Type/d/i/findConnectorInType)
 **/
export const findConnectorInType = (type: Type, connectorGuid: string): Connector => findConnector(type.connectors ?? [], connectorGuid);

// #endregion 🔖Type

// #region 🔖Layer

// [👤semio📚js💻semio🔖layer](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer)
// Layer entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Layer validation.
 * [👤semio📚js💻semio🔖layer🪨layerschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/LayerSchema)
 **/
export const LayerSchema = z.object({
  guid: z.string(),
  path: z.string(),
  isHidden: z.boolean().optional(),
  isLocked: z.boolean().optional(),
  color: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Layer.
 * [👤semio📚js💻semio🔖layer🛠️layer](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/Layer)
 **/
export type Layer = z.infer<typeof LayerSchema>;
/**
 * Serializes Layer for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖layer🪨serializelayer](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/serializeLayer)
 **/
export const serializeLayer = (layer: Layer): string => JSON.stringify(LayerSchema.parse(layer));
/**
 * Performs the deserializeLayer operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖layer🪨deserializelayer](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/deserializeLayer)
 **/
export const deserializeLayer = (json: string): Layer => LayerSchema.parse(JSON.parse(json));

/**
 * Zod schema for Layer diff validation.
 * [👤semio📚js💻semio🔖layer🪨layerdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/LayerDiffSchema)
 **/
export const LayerDiffSchema = LayerSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Layer changes.
 * [👤semio📚js💻semio🔖layer🛠️layerdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/LayerDiff)
 **/
export type LayerDiff = z.infer<typeof LayerDiffSchema>;

/**
 * Retrieves the LayerDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖layer🪨getlayerdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/getLayerDiff)
 **/
export const getLayerDiff = (before: Layer, after: Layer): LayerDiff => {
  const diff: LayerDiff = {};
  if (before.path !== after.path) diff.path = after.path;
  if (before.isHidden !== after.isHidden) diff.isHidden = after.isHidden;
  if (before.isLocked !== after.isLocked) diff.isLocked = after.isLocked;
  if (before.color !== after.color) diff.color = after.color;
  if (before.description !== after.description) diff.description = after.description;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseLayer changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖layer🪨inverselayerdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/inverseLayerDiff)
 **/
export const inverseLayerDiff = (original: Layer, appliedDiff: LayerDiff): LayerDiff => {
  const inverse: LayerDiff = {};
  if (appliedDiff.path !== undefined) inverse.path = original.path;
  if (appliedDiff.isHidden !== undefined) inverse.isHidden = original.isHidden;
  if (appliedDiff.isLocked !== undefined) inverse.isLocked = original.isLocked;
  if (appliedDiff.color !== undefined) inverse.color = original.color;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeLayer changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖layer🪨mergelayerdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/mergeLayerDiff)
 **/
export const mergeLayerDiff = (diff1: LayerDiff, diff2: LayerDiff): LayerDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyLayer changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖layer🪨applylayerdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/applyLayerDiff)
 **/
export const applyLayerDiff = (base: Layer, diff: LayerDiff): Layer => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Layer = {
    guid: base.guid,
    path: diff.path ?? base.path,
  };

  if (diff.isHidden !== undefined || base.isHidden !== undefined) result.isHidden = diff.isHidden ?? base.isHidden;
  if (diff.isLocked !== undefined || base.isLocked !== undefined) result.isLocked = diff.isLocked ?? base.isLocked;
  if (diff.color !== undefined || base.color !== undefined) result.color = diff.color ?? base.color;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Layers diff validation.
 * [👤semio📚js💻semio🔖layer🪨layersdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/LayersDiffSchema)
 **/
export const LayersDiffSchema = z.object({
  removed: z.array(LayerIdSchema).optional(),
  updated: z.array(z.object({ layer: LayerIdSchema, diff: LayerDiffSchema })).optional(),
  added: z.array(LayerSchema).optional(),
});
/**
 * Diff type for tracking Layers changes.
 * [👤semio📚js💻semio🔖layer🛠️layersdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Layer/d/i/LayersDiff)
 **/
export type LayersDiff = z.infer<typeof LayersDiffSchema>;

// #endregion 🔖Layer

// #region 🔖Piece

// [👤semio📚js💻semio🔖piece](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece)
// Piece entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Piece validation.
 * [👤semio📚js💻semio🔖piece🪨pieceschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/PieceSchema)
 **/
export const PieceSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  type: TypeIdSchema.optional(),
  design: DesignIdSchema.optional(),
  plane: PlaneSchema.optional(),
  center: CoordSchema.optional(),
  scale: z.number().optional(),
  mirrorPlane: PlaneSchema.optional(),
  isHidden: z.boolean().optional(),
  isLocked: z.boolean().optional(),
  color: z.string().optional(),
  description: z.string().optional(),
  props: z.array(PropSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Piece.
 * [👤semio📚js💻semio🔖piece🛠️piece](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/Piece)
 **/
export type Piece = z.infer<typeof PieceSchema>;
/**
 * Serializes Piece for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖piece🪨serializepiece](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/serializePiece)
 **/
export const serializePiece = (piece: Piece): string => JSON.stringify(PieceSchema.parse(piece));
/**
 * Performs the deserializePiece operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖piece🪨deserializepiece](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/deserializePiece)
 **/
export const deserializePiece = (json: string): Piece => PieceSchema.parse(JSON.parse(json));

/**
 * Zod schema for Piece diff validation.
 * [👤semio📚js💻semio🔖piece🪨piecediffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/PieceDiffSchema)
 **/
export const PieceDiffSchema = PieceSchema.partial().omit({ plane: true, props: true, attributes: true }).extend({
  plane: PlaneDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Piece changes.
 * [👤semio📚js💻semio🔖piece🛠️piecediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/PieceDiff)
 **/
export type PieceDiff = z.infer<typeof PieceDiffSchema>;
/**
 * Retrieves the PieceDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖piece🪨getpiecediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/getPieceDiff)
 **/
export const getPieceDiff = (before: Piece, after: Piece): PieceDiff => {
  const diff: PieceDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.type?.guid !== after.type?.guid) diff.type = after.type;
  if (before.design?.guid !== after.design?.guid) diff.design = after.design;
  if (!deepEqual(before.plane, after.plane)) diff.plane = after.plane ? getPlaneDiff(before.plane ?? { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } }, after.plane) : undefined;
  if (!deepEqual(before.center, after.center)) diff.center = after.center;
  if (before.scale !== after.scale) diff.scale = after.scale;
  if (!deepEqual(before.mirrorPlane, after.mirrorPlane)) diff.mirrorPlane = after.mirrorPlane;
  if (before.isHidden !== after.isHidden) diff.isHidden = after.isHidden;
  if (before.isLocked !== after.isLocked) diff.isLocked = after.isLocked;
  if (before.color !== after.color) diff.color = after.color;
  if (before.description !== after.description) diff.description = after.description;
  if (!deepEqual(before.props, after.props)) diff.props = getPropsDiff(before.props ?? [], after.props ?? []);
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inversePiece changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖piece🪨inversepiecediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/inversePieceDiff)
 **/
export const inversePieceDiff = (original: Piece, appliedDiff: PieceDiff): PieceDiff => {
  const inverse: PieceDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.type !== undefined) inverse.type = original.type;
  if (appliedDiff.design !== undefined) inverse.design = original.design;
  if (appliedDiff.plane !== undefined) inverse.plane = original.plane;
  if (appliedDiff.center !== undefined) inverse.center = original.center;
  if (appliedDiff.scale !== undefined) inverse.scale = original.scale;
  if (appliedDiff.mirrorPlane !== undefined) inverse.mirrorPlane = original.mirrorPlane;
  if (appliedDiff.isHidden !== undefined) inverse.isHidden = original.isHidden;
  if (appliedDiff.isLocked !== undefined) inverse.isLocked = original.isLocked;
  if (appliedDiff.color !== undefined) inverse.color = original.color;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.props !== undefined) inverse.props = inversePropsDiff(original.props ?? [], appliedDiff.props);
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergePiece changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖piece🪨mergepiecediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/mergePieceDiff)
 **/
export const mergePieceDiff = (diff1: PieceDiff, diff2: PieceDiff): PieceDiff => {
  return {
    ...diff1,
    ...diff2,
    props: diff1.props && diff2.props ? mergePropsDiff(diff1.props, diff2.props) : (diff2.props ?? diff1.props),
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
/**
 * Diff type for tracking applyPiece changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖piece🪨applypiecediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/applyPieceDiff)
 **/
export const applyPieceDiff = (base: Piece, diff: PieceDiff): Piece => {
  let newPlane = base.plane;
  if (diff.plane) {
    const diffPlane = diff.plane as any;
    if (diffPlane.origin && diffPlane.xAxis && diffPlane.yAxis) {
      newPlane = diffPlane as Plane;
    } else {
      newPlane = applyPlaneDiff(base.plane ?? { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } }, diff.plane);
    }
  }
  const props = diff.props ? applyPropsDiff(base.props ?? [], diff.props) : base.props;
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Piece = {
    guid: base.guid,
    name: diff.name ?? base.name,
    type: diff.type ?? base.type,
  };

  if (diff.design !== undefined || base.design !== undefined) result.design = diff.design ?? base.design;
  if (newPlane) result.plane = newPlane;
  if (diff.center !== undefined || base.center !== undefined) result.center = diff.center ?? base.center;
  if (diff.scale !== undefined || base.scale !== undefined) result.scale = diff.scale ?? base.scale;
  if (diff.mirrorPlane !== undefined || base.mirrorPlane !== undefined) result.mirrorPlane = diff.mirrorPlane ?? base.mirrorPlane;
  if (diff.isHidden !== undefined || base.isHidden !== undefined) result.isHidden = diff.isHidden ?? base.isHidden;
  if (diff.isLocked !== undefined || base.isLocked !== undefined) result.isLocked = diff.isLocked ?? base.isLocked;
  if (diff.color !== undefined || base.color !== undefined) result.color = diff.color ?? base.color;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (props && props.length > 0) result.props = props;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Pieces diff validation.
 * [👤semio📚js💻semio🔖piece🪨piecesdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/PiecesDiffSchema)
 **/
export const PiecesDiffSchema = z.object({
  removed: z.array(PieceIdSchema).optional(),
  updated: z.array(z.object({ piece: PieceIdSchema, diff: PieceDiffSchema })).optional(),
  added: z.array(PieceSchema).optional(),
});
/**
 * Diff type for tracking Pieces changes.
 * [👤semio📚js💻semio🔖piece🛠️piecesdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/PiecesDiff)
 **/
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;

/**
 * Retrieves the PieceModelFileGuids value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖piece🪨getpiecemodelfileguids](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/getPieceModelFileGuids)
 **/
export const getPieceModelFileGuids = (design: Design, types: Type[], tags: string[] = []): Map<string, string> => {
  const modelFileGuids = new Map<string, string>();
  toArray(design.pieces).forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.guid === p.type!.guid);
    if (!type) throw new Error(`Type ${p.type.guid} for piece ${p.guid} not found`);
    if (!type.models) throw new Error(`Type ${p.type.guid} for piece ${p.guid} has no models`);
    const model = findModel(type.models, tags);
    modelFileGuids.set(p.guid, model.file.guid);
  });
  return modelFileGuids;
};

/**
 * Retrieves the PieceModelUrls value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖piece🪨getpiecemodelurls](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/getPieceModelUrls)
 **/
export const getPieceModelUrls = (design: Design, types: Type[], files: File[], getFileUrl: (fileGuid: string) => string, tags: string[] = []): Map<string, string> => {
  const modelUrls = new Map<string, string>();
  toArray(design.pieces).forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.guid === p.type!.guid);
    if (!type) throw new Error(`Type ${p.type.guid} for piece ${p.guid} not found`);
    if (!type.models) throw new Error(`Type ${p.type.guid} for piece ${p.guid} has no models`);
    const model = findModel(type.models, tags);
    const file = files.find((f) => f.guid === model.file.guid);
    if (!file) throw new Error(`File ${model.file.guid} for model ${model.guid} not found`);
    modelUrls.set(p.guid, getFileUrl(file.guid));
  });
  return modelUrls;
};
/**
 * Performs the fixPieceInDesign operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖piece🪨fixpieceindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/fixPieceInDesign)
 **/
export const fixPieceInDesign = (kit: Kit, designId: string, pieceId: string): DesignDiff => {
  const parentConnection = findParentConnectionForPieceInDesign(kit, designId, pieceId);
  return {
    connections: {
      removed: [{ guid: parentConnection.guid }],
    },
  };
};

/**
 * Performs the fixPiecesInDesign operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖piece🪨fixpiecesindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/fixPiecesInDesign)
 **/
export const fixPiecesInDesign = (kit: Kit, designId: string, pieceIds: string[]): DesignDiff => {
  const parentConnections = pieceIds.map((pieceId) => findParentConnectionForPieceInDesign(kit, designId, pieceId));
  return {
    connections: {
      removed: parentConnections.map((c) => ({ guid: c.guid })),
    },
  };
};

/**
 * Performs the isFixedPiece operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖piece🪨isfixedpiece](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/isFixedPiece)
 **/
export const isFixedPiece = (piece: Piece): boolean => {
  const isPlaneSet = piece.plane !== undefined;
  const isCenterSet = piece.center !== undefined;
  if (isPlaneSet !== isCenterSet) throw new Error(`Piece ${piece.guid} has inconsistent plane and center`);
  return isPlaneSet;
};

/**
 * Performs the mergeCoordDiff operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖coordweakentity🪨mergecoorddiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity)/d/i/mergeCoordDiff)
 **/
/**
 * Searches for matching Piece entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖piece🪨findpiece](repo://p/u/semio/b/l/js/f/semio.ts/s/Piece/d/i/findPiece)
 **/
export const findPiece = (pieces: Piece[], pieceGuid: string): Piece => {
  const piece = pieces.find((p) => p.guid === pieceGuid);
  if (!piece) throw new Error(`Piece ${pieceGuid} not found in pieces`);
  return piece;
};

// #endregion 🔖Piece

// #region 🔖Group

// [👤semio📚js💻semio🔖group](repo://p/u/semio/b/l/js/f/semio.ts/s/Group)
// Group entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Group validation.
 * [👤semio📚js💻semio🔖group🪨groupschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/GroupSchema)
 **/
export const GroupSchema = z.object({
  guid: z.string(),
  pieces: z.array(PieceIdSchema),
  color: z.string().optional(),
  name: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Group.
 * [👤semio📚js💻semio🔖group🛠️group](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/Group)
 **/
export type Group = z.infer<typeof GroupSchema>;
/**
 * Zod schema for Group diff validation.
 * [👤semio📚js💻semio🔖group🪨groupdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/GroupDiffSchema)
 **/
export const GroupDiffSchema = GroupSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Group changes.
 * [👤semio📚js💻semio🔖group🛠️groupdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/GroupDiff)
 **/
export type GroupDiff = z.infer<typeof GroupDiffSchema>;
/**
 * Retrieves the GroupDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖group🪨getgroupdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/getGroupDiff)
 **/
export const getGroupDiff = (before: Group, after: Group): GroupDiff => {
  const diff: GroupDiff = {};
  if (!arraysEqual(before.pieces, after.pieces)) diff.pieces = after.pieces;
  if (before.color !== after.color) diff.color = after.color;
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description;
  const attributesDiff = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
  return diff;
};
/**
 * Diff type for tracking inverseGroup changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖group🪨inversegroupdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/inverseGroupDiff)
 **/
export const inverseGroupDiff = (original: Group, appliedDiff: GroupDiff): GroupDiff => {
  const inverse: GroupDiff = {};
  if (appliedDiff.pieces !== undefined) inverse.pieces = original.pieces;
  if (appliedDiff.color !== undefined) inverse.color = original.color;
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking applyGroup changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖group🪨applygroupdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/applyGroupDiff)
 **/
export const applyGroupDiff = (base: Group, diff: GroupDiff): Group => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Group = {
    guid: base.guid,
    pieces: diff.pieces ?? base.pieces,
  };

  if (diff.color !== undefined || base.color !== undefined) result.color = diff.color ?? base.color;
  if (diff.name !== undefined || base.name !== undefined) result.name = diff.name ?? base.name;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};
/**
 * Diff type for tracking mergeGroup changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖group🪨mergegroupdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/mergeGroupDiff)
 **/
export const mergeGroupDiff = (diff1: GroupDiff, diff2: GroupDiff): GroupDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};
/**
 * Zod schema for Groups diff validation.
 * [👤semio📚js💻semio🔖group🪨groupsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/GroupsDiffSchema)
 **/
export const GroupsDiffSchema = z.object({
  removed: z.array(GroupIdSchema).optional(),
  updated: z.array(z.object({ group: GroupIdSchema, diff: GroupDiffSchema })).optional(),
  added: z.array(GroupSchema).optional(),
});
/**
 * Serializes Group for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖group🪨serializegroup](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/serializeGroup)
 **/
export const serializeGroup = (group: Group): string => JSON.stringify(GroupSchema.parse(group));
/**
 * Performs the deserializeGroup operation.
 * [👤semio📚js💻semio🔖group🪨deserializegroup](repo://p/u/semio/b/l/js/f/semio.ts/s/Group/d/i/deserializeGroup)
 **/
export const deserializeGroup = (json: string): Group => GroupSchema.parse(JSON.parse(json));

// #endregion 🔖Group

// #region 🔖Side

// [👤semio📚js💻semio🔖side](repo://p/u/semio/b/l/js/f/semio.ts/s/Side)
// Side entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Side validation.
 * [👤semio📚js💻semio🔖side🪨sideschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/SideSchema)
 **/
export const SideSchema = z.object({
  piece: PieceIdSchema,
  designPiece: PieceIdSchema.optional(),
  connector: ConnectorIdSchema.optional(),
});
/**
 * Type alias for Side.
 * [👤semio📚js💻semio🔖side🛠️side](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/Side)
 **/
export type Side = z.infer<typeof SideSchema>;
/**
 * Zod schema for Side diff validation.
 * [👤semio📚js💻semio🔖side🪨sidediffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/SideDiffSchema)
 **/
export const SideDiffSchema = SideSchema.partial();
/**
 * Diff type for tracking Side changes.
 * [👤semio📚js💻semio🔖side🛠️sidediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/SideDiff)
 **/
export type SideDiff = z.infer<typeof SideDiffSchema>;
/**
 * Zod schema for validating Side identifiers.
 * [👤semio📚js💻semio🔖side🪨sideidschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/SideIdSchema)
 **/
export const SideIdSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
/**
 * Identifier type for Side entities.
 * [👤semio📚js💻semio🔖side🛠️sideid](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/SideId)
 **/
export type SideId = z.infer<typeof SideIdSchema>;
/**
 * Zod schema for Sides diff validation.
 * [👤semio📚js💻semio🔖side🪨sidesdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/SidesDiffSchema)
 **/
export const SidesDiffSchema = z.object({
  removed: z.array(SideIdSchema).optional(),
  updated: z.array(z.object({ side: SideIdSchema, diff: SideDiffSchema })).optional(),
  added: z.array(SideSchema).optional(),
});
/**
 * Diff type for tracking Sides changes.
 * [👤semio📚js💻semio🔖side🛠️sidesdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/SidesDiff)
 **/
export type SidesDiff = z.infer<typeof SidesDiffSchema>;
/**
 * Retrieves the SideDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖side🪨getsidediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/getSideDiff)
 **/
export const getSideDiff = (before: Side, after: Side): SideDiff => {
  const diff: SideDiff = {};
  if (before.piece?.guid !== after.piece?.guid) diff.piece = after.piece;
  if (before.designPiece?.guid !== after.designPiece?.guid) diff.designPiece = after.designPiece;
  if (before.connector?.guid !== after.connector?.guid) diff.connector = after.connector;
  return diff;
};
/**
 * Diff type for tracking inverseSide changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖side🪨inversesidediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/inverseSideDiff)
 **/
export const inverseSideDiff = (original: Side, appliedDiff: SideDiff): SideDiff => {
  const inverse: SideDiff = {};
  if (appliedDiff.piece !== undefined) inverse.piece = original.piece;
  if (appliedDiff.designPiece !== undefined) inverse.designPiece = original.designPiece;
  if (appliedDiff.connector !== undefined) inverse.connector = original.connector;
  return inverse;
};
/**
 * Diff type for tracking mergeSide changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖side🪨mergesidediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/mergeSideDiff)
 **/
export const mergeSideDiff = (diff1: SideDiff, diff2: SideDiff): SideDiff => {
  return { ...diff1, ...diff2 };
};
/**
 * Diff type for tracking applySide changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖side🪨applysidediff](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/applySideDiff)
 **/
export const applySideDiff = (base: Side, diff: SideDiff): Side => {
  const result: Side = {
    piece: diff.piece ?? base.piece,
  };

  if (diff.designPiece !== undefined || base.designPiece !== undefined) result.designPiece = diff.designPiece ?? base.designPiece;
  if (diff.connector !== undefined || base.connector !== undefined) result.connector = diff.connector ?? base.connector;

  return result;
};
/**
 * Serializes Side for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖side🪨serializeside](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/serializeSide)
 **/
export const serializeSide = (side: Side): string => JSON.stringify(SideSchema.parse(side));
/**
 * Performs the deserializeSide operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖side🪨deserializeside](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/deserializeSide)
 **/
export const deserializeSide = (json: string): Side => SideSchema.parse(JSON.parse(json));
/**
 * Equality check for Side values.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖side🪨aresameside](repo://p/u/semio/b/l/js/f/semio.ts/s/Side/d/i/areSameSide)
 **/
export const areSameSide = (a: Side, b: Side): boolean => a.piece.guid === b.piece.guid && a.designPiece?.guid === b.designPiece?.guid && a.connector?.guid === b.connector?.guid;

// #endregion 🔖Side

// #region 🔖Connection

// [👤semio📚js💻semio🔖connection](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection)
// Connection entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Connection validation.
 * [👤semio📚js💻semio🔖connection🪨connectionschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/ConnectionSchema)
 **/
export const ConnectionSchema = z.object({
  guid: z.string(),
  connected: SideSchema,
  connecting: SideSchema,
  gap: z.number().optional(),
  shift: z.number().optional(),
  rise: z.number().optional(),
  rotation: z.number().optional(),
  turn: z.number().optional(),
  tilt: z.number().optional(),
  u: z.number().optional(),
  v: z.number().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Connection.
 * [👤semio📚js💻semio🔖connection🛠️connection](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/Connection)
 **/
export type Connection = z.infer<typeof ConnectionSchema>;
/**
 * Zod schema for Connection diff validation.
 * [👤semio📚js💻semio🔖connection🪨connectiondiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/ConnectionDiffSchema)
 **/
export const ConnectionDiffSchema = ConnectionSchema.partial().omit({ guid: true, connected: true, connecting: true, attributes: true }).extend({
  connected: SideDiffSchema.optional(),
  connecting: SideDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Connection changes.
 * [👤semio📚js💻semio🔖connection🛠️connectiondiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/ConnectionDiff)
 **/
export type ConnectionDiff = z.infer<typeof ConnectionDiffSchema>;
/**
 * Retrieves the ConnectionDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖connection🪨getconnectiondiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/getConnectionDiff)
 **/
export const getConnectionDiff = (before: Connection, after: Connection): ConnectionDiff => {
  const diff: ConnectionDiff = {};
  if (!deepEqual(before.connected, after.connected)) diff.connected = getSideDiff(before.connected, after.connected);
  if (!deepEqual(before.connecting, after.connecting)) diff.connecting = getSideDiff(before.connecting, after.connecting);
  if (before.gap !== after.gap) diff.gap = after.gap !== undefined && before.gap !== undefined ? after.gap - before.gap : after.gap;
  if (before.shift !== after.shift) diff.shift = after.shift !== undefined && before.shift !== undefined ? after.shift - before.shift : after.shift;
  if (before.rise !== after.rise) diff.rise = after.rise !== undefined && before.rise !== undefined ? after.rise - before.rise : after.rise;
  if (before.rotation !== after.rotation) diff.rotation = after.rotation !== undefined && before.rotation !== undefined ? after.rotation - before.rotation : after.rotation;
  if (before.turn !== after.turn) diff.turn = after.turn !== undefined && before.turn !== undefined ? after.turn - before.turn : after.turn;
  if (before.tilt !== after.tilt) diff.tilt = after.tilt !== undefined && before.tilt !== undefined ? after.tilt - before.tilt : after.tilt;
  if (before.u !== after.u) diff.u = after.u !== undefined && before.u !== undefined ? after.u - before.u : after.u;
  if (before.v !== after.v) diff.v = after.v !== undefined && before.v !== undefined ? after.v - before.v : after.v;
  if (before.description !== after.description) diff.description = after.description;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};

/**
 * Diff type for tracking applyConnection changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖connection🪨applyconnectiondiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/applyConnectionDiff)
 **/
export const applyConnectionDiff = (base: Connection, diff: ConnectionDiff): Connection => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Connection = {
    guid: base.guid,
    connected: diff.connected ? applySideDiff(base.connected, diff.connected) : base.connected,
    connecting: diff.connecting ? applySideDiff(base.connecting, diff.connecting) : base.connecting,
  };

  if (diff.gap !== undefined || base.gap !== undefined) result.gap = diff.gap !== undefined && base.gap !== undefined ? base.gap + diff.gap : (diff.gap ?? base.gap);
  if (diff.shift !== undefined || base.shift !== undefined) result.shift = diff.shift !== undefined && base.shift !== undefined ? base.shift + diff.shift : (diff.shift ?? base.shift);
  if (diff.rise !== undefined || base.rise !== undefined) result.rise = diff.rise !== undefined && base.rise !== undefined ? base.rise + diff.rise : (diff.rise ?? base.rise);
  if (diff.rotation !== undefined || base.rotation !== undefined) result.rotation = diff.rotation !== undefined && base.rotation !== undefined ? base.rotation + diff.rotation : (diff.rotation ?? base.rotation);
  if (diff.turn !== undefined || base.turn !== undefined) result.turn = diff.turn !== undefined && base.turn !== undefined ? base.turn + diff.turn : (diff.turn ?? base.turn);
  if (diff.tilt !== undefined || base.tilt !== undefined) result.tilt = diff.tilt !== undefined && base.tilt !== undefined ? base.tilt + diff.tilt : (diff.tilt ?? base.tilt);
  if (diff.u !== undefined || base.u !== undefined) result.u = diff.u !== undefined && base.u !== undefined ? base.u + diff.u : (diff.u ?? base.u);
  if (diff.v !== undefined || base.v !== undefined) result.v = diff.v !== undefined && base.v !== undefined ? base.v + diff.v : (diff.v ?? base.v);
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Diff type for tracking mergeConnection changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖connection🪨mergeconnectiondiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/mergeConnectionDiff)
 **/
export const mergeConnectionDiff = (diff1: ConnectionDiff, diff2: ConnectionDiff): ConnectionDiff => {
  return {
    ...diff1,
    ...diff2,
    connected: diff2.connected || diff1.connected,
    connecting: diff2.connecting || diff1.connecting,
    attributes: diff2.attributes || diff1.attributes,
  };
};

/**
 * Diff type for tracking inverseConnection changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖connection🪨inverseconnectiondiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/inverseConnectionDiff)
 **/
export const inverseConnectionDiff = (original: Connection, appliedDiff: ConnectionDiff): ConnectionDiff => {
  const inverse: ConnectionDiff = {};
  if (appliedDiff.connected !== undefined) inverse.connected = inverseSideDiff(original.connected, appliedDiff.connected);
  if (appliedDiff.connecting !== undefined) inverse.connecting = inverseSideDiff(original.connecting, appliedDiff.connecting);
  if (appliedDiff.gap !== undefined) inverse.gap = original.gap !== undefined && appliedDiff.gap !== undefined ? -appliedDiff.gap : original.gap;
  if (appliedDiff.shift !== undefined) inverse.shift = original.shift !== undefined && appliedDiff.shift !== undefined ? -appliedDiff.shift : original.shift;
  if (appliedDiff.rise !== undefined) inverse.rise = original.rise !== undefined && appliedDiff.rise !== undefined ? -appliedDiff.rise : original.rise;
  if (appliedDiff.rotation !== undefined) inverse.rotation = original.rotation !== undefined && appliedDiff.rotation !== undefined ? -appliedDiff.rotation : original.rotation;
  if (appliedDiff.turn !== undefined) inverse.turn = original.turn !== undefined && appliedDiff.turn !== undefined ? -appliedDiff.turn : original.turn;
  if (appliedDiff.tilt !== undefined) inverse.tilt = original.tilt !== undefined && appliedDiff.tilt !== undefined ? -appliedDiff.tilt : original.tilt;
  if (appliedDiff.u !== undefined) inverse.u = original.u !== undefined && appliedDiff.u !== undefined ? -appliedDiff.u : original.u;
  if (appliedDiff.v !== undefined) inverse.v = original.v !== undefined && appliedDiff.v !== undefined ? -appliedDiff.v : original.v;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = getAttributesDiff(appliedDiff.attributes ? applyAttributesDiff([], appliedDiff.attributes) : [], original.attributes ?? []);
  return inverse;
};

/**
 * Zod schema for Connections diff validation.
 * [👤semio📚js💻semio🔖connection🪨connectionsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/ConnectionsDiffSchema)
 **/
export const ConnectionsDiffSchema = z.object({
  removed: z.array(ConnectionIdSchema).optional(),
  updated: z.array(z.object({ connection: ConnectionIdSchema, diff: ConnectionDiffSchema })).optional(),
  added: z.array(ConnectionSchema).optional(),
});
/**
 * Diff type for tracking Connections changes.
 * [👤semio📚js💻semio🔖connection🛠️connectionsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/ConnectionsDiff)
 **/
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>;
/**
 * Serializes Connection for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖connection🪨serializeconnection](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/serializeConnection)
 **/
export const serializeConnection = (connection: Connection): string => JSON.stringify(ConnectionSchema.parse(connection));
/**
 * Performs the deserializeConnection operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖connection🪨deserializeconnection](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/deserializeConnection)
 **/
export const deserializeConnection = (json: string): Connection => ConnectionSchema.parse(JSON.parse(json));

/**
 * Equality check for Connection values.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖connection🪨aresameconnection](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/areSameConnection)
 **/
export const areSameConnection = (connection: Connection | ConnectionDiff, other: Connection | ConnectionDiff, strict: boolean = false): boolean => {
  const getConnectedPieceId = (conn: typeof connection) => ("connected" in conn && conn.connected && "piece" in conn.connected ? (typeof conn.connected.piece === "string" ? conn.connected.piece : (conn.connected.piece?.guid ?? "")) : "");
  const getConnectingPieceId = (conn: typeof connection) => ("connecting" in conn && conn.connecting && "piece" in conn.connecting ? (typeof conn.connecting.piece === "string" ? conn.connecting.piece : (conn.connecting.piece?.guid ?? "")) : "");

  const connectedPiece1 = getConnectedPieceId(connection);
  const connectingPiece1 = getConnectingPieceId(connection);
  const connectedPiece2 = getConnectedPieceId(other);
  const connectingPiece2 = getConnectingPieceId(other);

  const isExactMatch = connectingPiece1 === connectingPiece2 && connectedPiece1 === connectedPiece2;
  if (strict) return isExactMatch;
  const isSwappedMatch = connectingPiece1 === connectedPiece2 && connectedPiece1 === connectingPiece2;
  return isExactMatch || isSwappedMatch;
};

/**
 * Searches for matching Connection entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖connection🪨findconnection](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/findConnection)
 **/
export const findConnection = (connections: Connection[], connectionGuid: string): Connection => {
  const connection = connections.find((c) => c.guid === connectionGuid);
  if (!connection) throw new Error(`Connection ${connectionGuid} not found in connections`);
  return connection;
};

/**
 * Searches for matching PieceConnections entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖connection🪨findpiececonnections](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/findPieceConnections)
 **/
export const findPieceConnections = (connections: Connection[], pieceGuid: string): Connection[] => {
  return connections.filter((c) => c.connected.piece.guid === pieceGuid || c.connecting.piece.guid === pieceGuid);
};

/**
 * Searches for matching ConnectorForPieceInConnection entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖connection🪨findconnectorforpieceinconnection](repo://p/u/semio/b/l/js/f/semio.ts/s/Connection/d/i/findConnectorForPieceInConnection)
 **/
export const findConnectorForPieceInConnection = (type: Type, connection: Connection, pieceGuid: string): Connector | undefined => {
  const connectorGuid = connection.connected.piece.guid === pieceGuid ? connection.connected.connector?.guid : connection.connecting.connector?.guid;
  if (!connectorGuid) return undefined;
  return findConnectorInType(type, connectorGuid);
};

// #endregion 🔖Connection

// #region 🔖Stat

// [👤semio📚js💻semio🔖stat](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat)
// Stat entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Stat validation.
 * [👤semio📚js💻semio🔖stat🪨statschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/StatSchema)
 **/
export const StatSchema = z.object({
  guid: z.string(),
  quality: QualityIdSchema,
  unit: z.string().optional(),
  min: z.number().optional(),
  minExcluded: z.boolean().optional(),
  max: z.number().optional(),
  maxExcluded: z.boolean().optional(),
});
/**
 * Type alias for Stat.
 * [👤semio📚js💻semio🔖stat🛠️stat](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/Stat)
 **/
export type Stat = z.infer<typeof StatSchema>;
/**
 * Zod schema for Stat diff validation.
 * [👤semio📚js💻semio🔖stat🪨statdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/StatDiffSchema)
 **/
export const StatDiffSchema = StatSchema.partial();
/**
 * Diff type for tracking Stat changes.
 * [👤semio📚js💻semio🔖stat🛠️statdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/StatDiff)
 **/
export type StatDiff = z.infer<typeof StatDiffSchema>;
/**
 * Retrieves the StatDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖stat🪨getstatdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/getStatDiff)
 **/
export const getStatDiff = (before: Stat, after: Stat): StatDiff => {
  const diff: StatDiff = {};
  if (before.quality?.guid !== after.quality?.guid) diff.quality = after.quality;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (before.min !== after.min) diff.min = after.min;
  if (before.minExcluded !== after.minExcluded) diff.minExcluded = after.minExcluded;
  if (before.max !== after.max) diff.max = after.max;
  if (before.maxExcluded !== after.maxExcluded) diff.maxExcluded = after.maxExcluded;
  return diff;
};
/**
 * Diff type for tracking inverseStat changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖stat🪨inversestatdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/inverseStatDiff)
 **/
export const inverseStatDiff = (original: Stat, appliedDiff: StatDiff): StatDiff => {
  const inverse: StatDiff = {};
  if (appliedDiff.quality !== undefined) inverse.quality = original.quality;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.min !== undefined) inverse.min = original.min;
  if (appliedDiff.minExcluded !== undefined) inverse.minExcluded = original.minExcluded;
  if (appliedDiff.max !== undefined) inverse.max = original.max;
  if (appliedDiff.maxExcluded !== undefined) inverse.maxExcluded = original.maxExcluded;
  return inverse;
};
/**
 * Diff type for tracking applyStat changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖stat🪨applystatdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/applyStatDiff)
 **/
export const applyStatDiff = (base: Stat, diff: StatDiff): Stat => {
  const result: Stat = {
    guid: base.guid,
    quality: diff.quality ?? base.quality,
  };

  if (diff.unit !== undefined || base.unit !== undefined) result.unit = diff.unit ?? base.unit;
  if (diff.min !== undefined || base.min !== undefined) result.min = diff.min ?? base.min;
  if (diff.minExcluded !== undefined || base.minExcluded !== undefined) result.minExcluded = diff.minExcluded ?? base.minExcluded;
  if (diff.max !== undefined || base.max !== undefined) result.max = diff.max ?? base.max;
  if (diff.maxExcluded !== undefined || base.maxExcluded !== undefined) result.maxExcluded = diff.maxExcluded ?? base.maxExcluded;

  return result;
};
/**
 * Diff type for tracking mergeStat changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖stat🪨mergestatdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/mergeStatDiff)
 **/
export const mergeStatDiff = (diff1: StatDiff, diff2: StatDiff): StatDiff => {
  return { ...diff1, ...diff2 };
};
/**
 * Zod schema for Stats diff validation.
 * [👤semio📚js💻semio🔖stat🪨statsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/StatsDiffSchema)
 **/
export const StatsDiffSchema = z.object({
  removed: z.array(StatIdSchema).optional(),
  updated: z.array(z.object({ stat: StatIdSchema, diff: StatDiffSchema })).optional(),
  added: z.array(StatSchema).optional(),
});
/**
 * Serializes Stat for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖stat🪨serializestat](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/serializeStat)
 **/
export const serializeStat = (stat: Stat): string => JSON.stringify(StatSchema.parse(stat));
/**
 * Performs the deserializeStat operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖stat🪨deserializestat](repo://p/u/semio/b/l/js/f/semio.ts/s/Stat/d/i/deserializeStat)
 **/
export const deserializeStat = (json: string): Stat => StatSchema.parse(JSON.parse(json));

// #endregion 🔖Stat

// #region 🔖Design

// [👤semio📚js💻semio🔖design](repo://p/u/semio/b/l/js/f/semio.ts/s/Design)
// Design entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Design validation.
 * [👤semio📚js💻semio🔖design🪨designschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/DesignSchema)
 **/
export const DesignSchema = z.object({
  guid: z.string(),
  name: z.string(),
  parent: DesignIdSchema.optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  pieces: z.array(PieceSchema).optional(),
  connections: z.array(ConnectionSchema).optional(),
  stats: z.array(StatSchema).optional(),
  props: z.array(PropSchema).optional(),
  layers: z.array(LayerSchema).optional(),
  activeLayer: LayerIdSchema.optional(),
  groups: z.array(GroupSchema).optional(),
  canScale: z.boolean().optional(),
  canMirror: z.boolean().optional(),
  unit: z.string().optional(),
  location: LocationIdSchema.optional(),
  authors: z.array(AuthorIdSchema).optional(),
  concepts: z.array(ConceptIdSchema).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
/**
 * Type alias for Design.
 * [👤semio📚js💻semio🔖design🛠️design](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/Design)
 **/
export type Design = z.infer<typeof DesignSchema>;
/**
 * Serializes Design for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖design🪨serializedesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/serializeDesign)
 **/
export const serializeDesign = (design: Design): string => JSON.stringify(DesignSchema.parse(design));
/**
 * Performs the deserializeDesign operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨deserializedesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/deserializeDesign)
 **/
export const deserializeDesign = (json: string): Design => DesignSchema.parse(JSON.parse(json));

/**
 * Definition of DesignShallowSchema.
 * [👤semio📚js💻semio🔖design🪨designshallowschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/DesignShallowSchema)
 **/
export const DesignShallowSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true }).extend({
  pieces: z.array(z.string()).optional(),
  connections: z.array(z.string()).optional(),
  stats: z.array(z.string()).optional(),
});

/**
 * Type alias for DesignShallow.
 * [👤semio📚js💻semio🔖design🛠️designshallow](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/DesignShallow)
 **/
export type DesignShallow = z.infer<typeof DesignShallowSchema>;
/**
 * Serializes DesignShallow for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖design🪨serializedesignshallow](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/serializeDesignShallow)
 **/
export const serializeDesignShallow = (design: DesignShallow): string => JSON.stringify(DesignShallowSchema.parse(design));
/**
 * Performs the deserializeDesignShallow operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨deserializedesignshallow](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/deserializeDesignShallow)
 **/
export const deserializeDesignShallow = (json: string): DesignShallow => DesignShallowSchema.parse(JSON.parse(json));
/**
 * Zod schema for Design diff validation.
 * [👤semio📚js💻semio🔖design🪨designdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/DesignDiffSchema)
 **/
export const DesignDiffSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, authors: true, attributes: true }).partial().extend({
  pieces: PiecesDiffSchema.optional(),
  connections: ConnectionsDiffSchema.optional(),
  stats: StatsDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  layers: LayersDiffSchema.optional(),
  groups: GroupsDiffSchema.optional(),
  authors: AuthorsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});

/**
 * Diff type for tracking Design changes.
 * [👤semio📚js💻semio🔖design🛠️designdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/DesignDiff)
 **/
export type DesignDiff = z.infer<typeof DesignDiffSchema>;
/**
 * Retrieves the DesignDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖design🪨getdesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/getDesignDiff)
 **/
export const getDesignDiff = (before: Design, after: Design): DesignDiff => {
  const diff: DesignDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.parent?.guid !== after.parent?.guid) diff.parent = after.parent;
  if (before.isAbstract !== after.isAbstract) diff.isAbstract = after.isAbstract;
  if (before.folder !== after.folder) diff.folder = after.folder;
  if (before.canScale !== after.canScale) diff.canScale = after.canScale;
  if (before.canMirror !== after.canMirror) diff.canMirror = after.canMirror;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (before.activeLayer?.guid !== after.activeLayer?.guid) diff.activeLayer = after.activeLayer;
  if (before.location?.guid !== after.location?.guid) diff.location = after.location;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.image !== after.image) diff.image = after.image;
  if (before.description !== after.description) diff.description = after.description;
  if (!arraysEqual(before.authors, after.authors)) diff.authors = after.authors as any;
  if (!arraysEqual(before.concepts, after.concepts)) diff.concepts = after.concepts;
  const piecesDiff = getCollectionDiff("piece", before.pieces ?? [], after.pieces ?? [], getPieceDiff);
  if (Object.keys(piecesDiff).length > 0) diff.pieces = piecesDiff;
  const connectionsDiff = getCollectionDiff("connection", before.connections ?? [], after.connections ?? [], getConnectionDiff);
  if (Object.keys(connectionsDiff).length > 0) diff.connections = connectionsDiff;
  const statsDiff = getCollectionDiff("stat", before.stats ?? [], after.stats ?? [], getStatDiff);
  if (Object.keys(statsDiff).length > 0) diff.stats = statsDiff;
  const propsDiff = getCollectionDiff("prop", before.props ?? [], after.props ?? [], getPropDiff);
  if (Object.keys(propsDiff).length > 0) diff.props = propsDiff;
  const layersDiff = getCollectionDiff("layer", before.layers ?? [], after.layers ?? [], getLayerDiff);
  if (Object.keys(layersDiff).length > 0) diff.layers = layersDiff;
  const groupsDiff = getCollectionDiff("group", before.groups ?? [], after.groups ?? [], getGroupDiff);
  if (Object.keys(groupsDiff).length > 0) diff.groups = groupsDiff;
  const attributesDiff = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
  return diff;
};
/**
 * Diff type for tracking mergeDesign changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨mergedesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/mergeDesignDiff)
 **/
export const mergeDesignDiff = (diff1: DesignDiff, diff2: DesignDiff): DesignDiff => {
  return {
    ...diff1,
    ...diff2,
    pieces: diff1.pieces || diff2.pieces ? mergeCollectionDiff("piece", diff1.pieces ?? {}, diff2.pieces ?? {}, mergePieceDiff) : undefined,
    connections: diff1.connections || diff2.connections ? mergeCollectionDiff("connection", diff1.connections ?? {}, diff2.connections ?? {}, mergeConnectionDiff) : undefined,
    stats: diff1.stats || diff2.stats ? mergeCollectionDiff("stat", diff1.stats ?? {}, diff2.stats ?? {}, mergeStatDiff) : undefined,
    props: diff1.props || diff2.props ? mergeCollectionDiff("prop", diff1.props ?? {}, diff2.props ?? {}, mergePropDiff) : undefined,
    layers: diff1.layers || diff2.layers ? mergeCollectionDiff("layer", diff1.layers ?? {}, diff2.layers ?? {}, mergeLayerDiff) : undefined,
    groups: diff1.groups || diff2.groups ? mergeCollectionDiff("group", diff1.groups ?? {}, diff2.groups ?? {}, mergeGroupDiff) : undefined,
    authors: diff2.authors ?? diff1.authors,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};
/**
 * Diff type for tracking inverseDesign changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨inversedesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/inverseDesignDiff)
 **/
export const inverseDesignDiff = (original: Design, appliedDiff: DesignDiff): DesignDiff => {
  const inverse: DesignDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.parent !== undefined) inverse.parent = original.parent;
  if (appliedDiff.isAbstract !== undefined) inverse.isAbstract = original.isAbstract;
  if (appliedDiff.folder !== undefined) inverse.folder = original.folder;
  if (appliedDiff.canScale !== undefined) inverse.canScale = original.canScale;
  if (appliedDiff.canMirror !== undefined) inverse.canMirror = original.canMirror;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.activeLayer !== undefined) inverse.activeLayer = original.activeLayer;
  if (appliedDiff.location !== undefined) inverse.location = original.location;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon;
  if (appliedDiff.image !== undefined) inverse.image = original.image;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.authors !== undefined) inverse.authors = original.authors as any;
  if (appliedDiff.concepts !== undefined) inverse.concepts = original.concepts;
  if (appliedDiff.pieces) inverse.pieces = inverseCollectionDiff("piece", original.pieces ?? [], appliedDiff.pieces, inversePieceDiff);
  if (appliedDiff.connections) inverse.connections = inverseCollectionDiff("connection", original.connections ?? [], appliedDiff.connections, inverseConnectionDiff);
  if (appliedDiff.stats) inverse.stats = inverseCollectionDiff("stat", original.stats ?? [], appliedDiff.stats, inverseStatDiff);
  if (appliedDiff.props) inverse.props = inverseCollectionDiff("prop", original.props ?? [], appliedDiff.props, inversePropDiff);
  if (appliedDiff.layers) inverse.layers = inverseCollectionDiff("layer", original.layers ?? [], appliedDiff.layers, inverseLayerDiff);
  if (appliedDiff.groups) inverse.groups = inverseCollectionDiff("group", original.groups ?? [], appliedDiff.groups, inverseGroupDiff);
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};

/**
 * Adds a PieceToDesignDiff element.
 * MUST append the element to the collection.
 * [👤semio📚js💻semio🔖design🪨addpiecetodesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/addPieceToDesignDiff)
 **/
export const addPieceToDesignDiff = (designDiff: any, piece: Piece): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      added: [...(designDiff.pieces?.added || []), piece],
    },
  };
};
/**
 * Replaces an existing PieceInDesignDiff element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖design🪨setpieceindesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/setPieceInDesignDiff)
 **/
export const setPieceInDesignDiff = (designDiff: any, pieceDiff: { id_: string; diff: PieceDiff }): any => {
  const existingIndex = (designDiff.pieces?.updated || []).findIndex((p: { id_: string; diff: PieceDiff }) => p.id_ === pieceDiff.id_);
  const updated = [...(designDiff.pieces?.updated || [])];
  if (existingIndex >= 0) {
    updated[existingIndex] = pieceDiff;
  } else {
    updated.push(pieceDiff);
  }
  return { ...designDiff, pieces: { ...designDiff.pieces, updated } };
};

/**
 * Removes a PieceFromDesignDiff element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖design🪨removepiecefromdesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/removePieceFromDesignDiff)
 **/
export const removePieceFromDesignDiff = (designDiff: any, pieceId: string): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      removed: [...(designDiff.pieces?.removed || []), pieceId],
    },
  };
};

/**
 * Adds a PiecesToDesignDiff element.
 * MUST append the element to the collection.
 * [👤semio📚js💻semio🔖design🪨addpiecestodesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/addPiecesToDesignDiff)
 **/
export const addPiecesToDesignDiff = (designDiff: any, pieces: Piece[]): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      added: [...(designDiff.pieces?.added || []), ...pieces],
    },
  };
};
/**
 * Replaces an existing PiecesInDesignDiff element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖design🪨setpiecesindesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/setPiecesInDesignDiff)
 **/
export const setPiecesInDesignDiff = (designDiff: any, pieceDiffs: { id_: string; diff: PieceDiff }[]): any => {
  const updated = [...(designDiff.pieces?.updated || [])];
  pieceDiffs.forEach((pieceDiff: { id_: string; diff: PieceDiff }) => {
    const existingIndex = updated.findIndex((p: { id_: string; diff: PieceDiff }) => p.id_ === pieceDiff.id_);
    if (existingIndex >= 0) {
      updated[existingIndex] = pieceDiff;
    } else {
      updated.push(pieceDiff);
    }
  });
  return { ...designDiff, pieces: { ...designDiff.pieces, updated } };
};

/**
 * Removes a PiecesFromDesignDiff element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖design🪨removepiecesfromdesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/removePiecesFromDesignDiff)
 **/
export const removePiecesFromDesignDiff = (designDiff: any, pieceIds: string[]): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      removed: [...(designDiff.pieces?.removed || []), ...pieceIds],
    },
  };
};

/**
 * Adds a ConnectionToDesignDiff element.
 * MUST append the element to the collection.
 * [👤semio📚js💻semio🔖design🪨addconnectiontodesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/addConnectionToDesignDiff)
 **/
export const addConnectionToDesignDiff = (designDiff: any, connection: Connection): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      added: [...(designDiff.connections?.added || []), connection],
    },
  };
};
/**
 * Replaces an existing ConnectionInDesignDiff element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖design🪨setconnectionindesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/setConnectionInDesignDiff)
 **/
export const setConnectionInDesignDiff = (designDiff: any, connectionDiff: ConnectionDiff): any => {
  const existingIndex = (designDiff.connections?.updated || []).findIndex((c: ConnectionDiff) => areSameConnection(c, connectionDiff));
  const updated = [...(designDiff.connections?.updated || [])];
  if (existingIndex >= 0) {
    updated[existingIndex] = connectionDiff;
  } else {
    updated.push(connectionDiff);
  }
  return { ...designDiff, connections: { ...designDiff.connections, updated } };
};
/**
 * Removes a ConnectionFromDesignDiff element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖design🪨removeconnectionfromdesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/removeConnectionFromDesignDiff)
 **/
export const removeConnectionFromDesignDiff = (designDiff: any, connectionId: { connected: { piece: string }; connecting: { piece: string } }): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      removed: [...(designDiff.connections?.removed || []), connectionId],
    },
  };
};

/**
 * Adds a ConnectionsToDesignDiff element.
 * MUST append the element to the collection.
 * [👤semio📚js💻semio🔖design🪨addconnectionstodesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/addConnectionsToDesignDiff)
 **/
export const addConnectionsToDesignDiff = (designDiff: any, connections: Connection[]): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      added: [...(designDiff.connections?.added || []), ...connections],
    },
  };
};
/**
 * Replaces an existing ConnectionsInDesignDiff element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖design🪨setconnectionsindesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/setConnectionsInDesignDiff)
 **/
export const setConnectionsInDesignDiff = (designDiff: any, connectionDiffs: ConnectionDiff[]): any => {
  const updated = [...(designDiff.connections?.updated || [])];
  connectionDiffs.forEach((connectionDiff: ConnectionDiff) => {
    const existingIndex = updated.findIndex((c: ConnectionDiff) => areSameConnection(c, connectionDiff));
    if (existingIndex >= 0) {
      updated[existingIndex] = connectionDiff;
    } else {
      updated.push(connectionDiff);
    }
  });
  return { ...designDiff, connections: { ...designDiff.connections, updated } };
};
/**
 * Removes a ConnectionsFromDesignDiff element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖design🪨removeconnectionsfromdesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/removeConnectionsFromDesignDiff)
 **/
export const removeConnectionsFromDesignDiff = (designDiff: any, connectionIds: string[]): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      removed: [...(designDiff.connections?.removed || []), ...connectionIds],
    },
  };
};

/**
 * Diff type for tracking applyDesign changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨applydesigndiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/applyDesignDiff)
 **/
export const applyDesignDiff = (base: Design, diff: DesignDiff): Design => {
  const pieces = diff.pieces || base.pieces ? applyCollectionDiff("piece", base.pieces ?? [], diff.pieces, applyPieceDiff) : undefined;
  const connections = diff.connections || base.connections ? applyCollectionDiff("connection", base.connections ?? [], diff.connections, applyConnectionDiff) : undefined;
  const stats = diff.stats || base.stats ? applyCollectionDiff("stat", base.stats ?? [], diff.stats, applyStatDiff) : undefined;
  const props = diff.props || base.props ? applyCollectionDiff("prop", base.props ?? [], diff.props, applyPropDiff) : undefined;
  const layers = diff.layers || base.layers ? applyCollectionDiff("layer", base.layers ?? [], diff.layers, applyLayerDiff) : undefined;
  const groups = diff.groups || base.groups ? applyCollectionDiff("group", base.groups ?? [], diff.groups, applyGroupDiff) : undefined;
  const attributes = diff.attributes || base.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes ?? {}) : undefined;

  const result: Design = {
    guid: base.guid,
    name: diff.name ?? base.name,
    isAbstract: diff.isAbstract ?? base.isAbstract,
    createdAt: diff.createdAt ?? base.createdAt,
    updatedAt: diff.updatedAt ?? base.updatedAt,
  };

  if (diff.parent !== undefined ? diff.parent : base.parent) result.parent = diff.parent !== undefined ? diff.parent : base.parent;
  if (diff.folder ?? base.folder) result.folder = diff.folder ?? base.folder;
  if (diff.canScale ?? base.canScale) result.canScale = diff.canScale ?? base.canScale;
  if (diff.canMirror ?? base.canMirror) result.canMirror = diff.canMirror ?? base.canMirror;
  if (diff.unit !== undefined ? diff.unit : base.unit) result.unit = diff.unit !== undefined ? diff.unit : base.unit;
  if (diff.activeLayer !== undefined ? diff.activeLayer : base.activeLayer) result.activeLayer = diff.activeLayer !== undefined ? diff.activeLayer : base.activeLayer;
  if (diff.location !== undefined ? diff.location : base.location) result.location = diff.location !== undefined ? diff.location : base.location;
  if (diff.icon !== undefined ? diff.icon : base.icon) result.icon = diff.icon !== undefined ? diff.icon : base.icon;
  if (diff.image !== undefined ? diff.image : base.image) result.image = diff.image !== undefined ? diff.image : base.image;
  if (diff.description !== undefined ? diff.description : base.description) result.description = diff.description !== undefined ? diff.description : base.description;
  if (diff.authors !== undefined ? (diff.authors as any) : base.authors) result.authors = diff.authors !== undefined ? (diff.authors as any) : base.authors;
  if (diff.concepts !== undefined ? diff.concepts : base.concepts) result.concepts = diff.concepts !== undefined ? diff.concepts : base.concepts;

  if (pieces && pieces.length > 0) result.pieces = pieces;
  if (connections && connections.length > 0) result.connections = connections;
  if (stats && stats.length > 0) result.stats = stats;
  if (props && props.length > 0) result.props = props;
  if (layers && layers.length > 0) result.layers = layers;
  if (groups && groups.length > 0) result.groups = groups;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Designs diff validation.
 * [👤semio📚js💻semio🔖design🪨designsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/DesignsDiffSchema)
 **/
export const DesignsDiffSchema = z.object({
  removed: z.array(DesignIdSchema).optional(),
  updated: z.array(z.object({ design: DesignIdSchema, diff: DesignDiffSchema })).optional(),
  added: z.array(DesignSchema).optional(),
});
/**
 * Diff type for tracking Designs changes.
 * [👤semio📚js💻semio🔖design🛠️designsdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/DesignsDiff)
 **/
export type DesignsDiff = z.infer<typeof DesignsDiffSchema>;

/**
 * Performs the mergeDesigns operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨mergedesigns](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/mergeDesigns)
 **/
export const mergeDesigns = (designs: Design[]): DesignDiff => {
  const pieces = designs.flatMap((d) => d.pieces ?? []);
  const connections = designs.flatMap((d) => d.connections ?? []);

  return {
    pieces: pieces.length > 0 ? { added: pieces } : undefined,
    connections: connections.length > 0 ? { added: connections } : undefined,
  };
};

/**
 * Performs the orientDesign operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨orientdesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/orientDesign)
 **/
export const orientDesign = (plane?: Plane, center?: Coord): DesignDiff => {
  if (plane === undefined && center === undefined) {
    return {};
  }

  return {};
};

/**
 * Removes a PiecesAndConnectionsFromDesign element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖design🪨removepiecesandconnectionsfromdesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/removePiecesAndConnectionsFromDesign)
 **/
export const removePiecesAndConnectionsFromDesign = (kit: Kit, designId: string, pieceIds: string[], connectionIds: string[]): DesignChange => {
  const design = findDesignInKit(kit, designId);
  const forward: DesignDiff = {
    pieces: {
      removed: pieceIds.map((guid) => ({ guid })),
    },
    connections: {
      removed: connectionIds.map((guid) => ({ guid })),
    },
  };
  const backward = inverseDesignDiff(design, forward);
  return { forward, backward };
};

// [👤semio📚js💻semio🔖design🪨computechildplane](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/computeChildPlane)
// computeChildPlane holds the data fields for a computeChildPlane record.
const computeChildPlane = (parentPlane: Plane, parentConnector: Connector, childConnector: Connector, connection: Connection): Plane => {
  const parentMatrix = planeToMatrix(parentPlane);
  const parentPoint = vectorToThree(parentConnector.point);
  const parentDirection = vectorToThree(parentConnector.direction).normalize();
  const childPoint = vectorToThree(childConnector.point);
  const childDirection = vectorToThree(childConnector.direction).normalize();

  const { gap, shift, rise, rotation, turn, tilt } = connection;
  const rotationRad = THREE.MathUtils.degToRad(rotation ?? 0);
  const turnRad = THREE.MathUtils.degToRad(turn ?? 0);
  const tiltRad = THREE.MathUtils.degToRad(tilt ?? 0);

  const reverseChildDirection = childDirection.clone().negate();

  let alignQuat: THREE.Quaternion;
  if (new THREE.Vector3().crossVectors(parentDirection, reverseChildDirection).length() < 0.01) {
    if (Math.abs(parentDirection.z) < TOLERANCE) {
      alignQuat = new THREE.Quaternion().setFromAxisAngle(new THREE.Vector3(0, 0, 1), Math.PI);
    } else {
      const axis = new THREE.Vector3(0, 0, 1).cross(parentDirection).normalize();
      alignQuat = new THREE.Quaternion().setFromAxisAngle(axis, Math.PI);
    }
  } else {
    alignQuat = new THREE.Quaternion().setFromUnitVectors(reverseChildDirection, parentDirection);
  }

  const directionT = new THREE.Matrix4().makeRotationFromQuaternion(alignQuat);

  const yAxis = new THREE.Vector3(0, 1, 0);
  const parentConnectorQuat = new THREE.Quaternion().setFromUnitVectors(yAxis, parentDirection);
  const parentRotationT = new THREE.Matrix4().makeRotationFromQuaternion(parentConnectorQuat);

  const gapDirection = new THREE.Vector3(0, 1, 0).applyMatrix4(parentRotationT);
  const shiftDirection = new THREE.Vector3(1, 0, 0).applyMatrix4(parentRotationT);
  const raiseDirection = new THREE.Vector3(0, 0, 1).applyMatrix4(parentRotationT);
  const turnAxis = new THREE.Vector3(0, 0, 1).applyMatrix4(parentRotationT);
  const tiltAxis = new THREE.Vector3(1, 0, 0).applyMatrix4(parentRotationT);

  let orientationT = directionT.clone();

  const rotateT = new THREE.Matrix4().makeRotationAxis(parentDirection, -rotationRad);
  orientationT.premultiply(rotateT);

  turnAxis.applyMatrix4(rotateT);
  tiltAxis.applyMatrix4(rotateT);

  const turnT = new THREE.Matrix4().makeRotationAxis(turnAxis, turnRad);
  orientationT.premultiply(turnT);

  const tiltT = new THREE.Matrix4().makeRotationAxis(tiltAxis, tiltRad);
  orientationT.premultiply(tiltT);

  const centerChildT = new THREE.Matrix4().makeTranslation(-childPoint.x, -childPoint.y, -childPoint.z);
  let transform = new THREE.Matrix4().multiplyMatrices(orientationT, centerChildT);

  const gapTransform = new THREE.Matrix4().makeTranslation(gapDirection.x * (gap ?? 0), gapDirection.y * (gap ?? 0), gapDirection.z * (gap ?? 0));
  const shiftTransform = new THREE.Matrix4().makeTranslation(shiftDirection.x * (shift ?? 0), shiftDirection.y * (shift ?? 0), shiftDirection.z * (shift ?? 0));
  const raiseTransform = new THREE.Matrix4().makeTranslation(raiseDirection.x * (rise ?? 0), raiseDirection.y * (rise ?? 0), raiseDirection.z * (rise ?? 0));

  const translationT = raiseTransform.clone().multiply(shiftTransform).multiply(gapTransform);
  transform.premultiply(translationT);
  const moveToParentT = new THREE.Matrix4().makeTranslation(parentPoint.x, parentPoint.y, parentPoint.z);
  transform.premultiply(moveToParentT);
  const finalMatrix = new THREE.Matrix4().multiplyMatrices(parentMatrix, transform);

  return matrixToPlane(finalMatrix);
};
/**
 * Flattens nested Design structure.
 * MUST return a flat array.
 * [👤semio📚js💻semio🔖design🪨flattendesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/flattenDesign)
 **/
export const flattenDesign = (kit: Kit, designId: string): DesignChange => {
  const design = findDesignInKit(kit, designId);
  if (!design) {
    throw new Error(`Design ${designId} not found in kit ${kit.name}`);
  }
  const types = kit.types ?? [];

  if (!design.pieces || design.pieces.length === 0) return { forward: {}, backward: {} };

  const typesDict: { [key: string]: Type } = {};
  types.forEach((t) => {
    typesDict[t.guid] = t;
  });
  const getType = (typeGuid: string): Type | undefined => {
    return typesDict[typeGuid];
  };
  const getConnector = (type: Type | undefined, connectorGuid: string | undefined): Connector | undefined => {
    if (!type) return undefined;

    if (!connectorGuid) {
      if (type.connectors && type.connectors.length > 0) {
        return type.connectors[0];
      }

      if (type.parent?.guid) {
        const parentType = getType(type.parent.guid);
        return getConnector(parentType, connectorGuid);
      }
      return undefined;
    }

    if (type.connectors && type.connectors.length > 0) {
      const connector = type.connectors.find((p) => p.guid === connectorGuid);
      if (connector) return connector;
    }

    if (type.parent?.guid) {
      const parentType = getType(type.parent.guid);
      const connector = getConnector(parentType, connectorGuid);
      if (connector) return connector;
    }

    if (type.connectors && type.connectors.length > 0) {
      return type.connectors[0];
    }

    return undefined;
  };

  const flatDesign: Design = JSON.parse(JSON.stringify(design));
  if (!flatDesign.pieces) flatDesign.pieces = [];

  const piecePlanes: { [pieceGuid: string]: Plane } = {};
  const pieceMap: { [pieceGuid: string]: Piece } = {};
  flatDesign.pieces!.forEach((p) => {
    if (p.guid) pieceMap[p.guid] = p;
  });

  const filteredConnections =
    flatDesign.connections?.filter((connection) => {
      const sourceId = connection.connected.piece.guid;
      const targetId = connection.connecting.piece.guid;
      const sourceExists = pieceMap[sourceId];
      const targetExists = pieceMap[targetId];
      if (!sourceExists) {
        console.warn(`[ORIGIN] flattenDesign: Skipping connection ${connection.guid} - source piece ${sourceId} not found`);
        return false;
      }
      if (!targetExists) {
        console.warn(`[ORIGIN] flattenDesign: Skipping connection ${connection.guid} - target piece ${targetId} not found`);
        return false;
      }
      return true;
    }) || [];

  const cy = cytoscape({
    elements: {
      nodes: flatDesign.pieces!.map((piece) => ({
        data: { id: piece.guid, label: piece.guid },
      })),
      edges: filteredConnections.map((connection, index) => {
        const sourceId = connection.connected.piece.guid;
        const targetId = connection.connecting.piece.guid;
        return {
          data: {
            id: connection.guid,
            source: sourceId,
            target: targetId,
            connectionData: connection,
          },
        };
      }),
    } as any,
    headless: true,
  });

  const components = cy.elements().components();

  const setAttributes = (piece: Piece, newAttrs: { key: string; value?: string; definition?: string }[]): Piece => {
    const existingAttrs = piece.attributes || [];
    const updatedAttrs = [...existingAttrs];
    newAttrs.forEach((newAttr) => {
      const existingIndex = updatedAttrs.findIndex((a) => a.key === newAttr.key);
      if (existingIndex >= 0) {
        updatedAttrs[existingIndex] = { ...updatedAttrs[existingIndex], ...newAttr, guid: updatedAttrs[existingIndex].guid };
      } else {
        updatedAttrs.push({ guid: guid(), ...newAttr });
      }
    });
    return { ...piece, attributes: updatedAttrs };
  };

  components.forEach((component) => {
    let roots = component.nodes().filter((node) => {
      const piece = pieceMap[node.id()];
      return piece?.plane !== undefined;
    });
    let rootNode = roots.length > 0 ? roots[0] : component.nodes().length > 0 ? component.nodes()[0] : undefined;
    if (!rootNode) return;
    const rootPiece = pieceMap[rootNode.id()];
    if (!rootPiece || !rootPiece.guid) return;
    const updatedRootPiece = setAttributes(rootPiece, [
      { key: "semio.fixedPieceId", value: rootPiece.guid },
      { key: "semio.depth", value: "0" },
    ]);
    pieceMap[rootNode.id()] = updatedRootPiece;
    let rootPlane: Plane;
    if (rootPiece.plane) {
      rootPlane = rootPiece.plane;
    } else {
      const identityMatrix = new THREE.Matrix4().identity();
      rootPlane = matrixToPlane(identityMatrix);
    }

    piecePlanes[rootPiece.guid] = rootPlane;
    const rootPieceIndex = flatDesign.pieces!.findIndex((p) => p.guid === rootPiece.guid);
    if (rootPieceIndex !== -1) {
      flatDesign.pieces![rootPieceIndex].plane = rootPlane;

      if (!flatDesign.pieces![rootPieceIndex].center) {
        flatDesign.pieces![rootPieceIndex].center = { u: 0, v: 0 };
      }
    }

    let visitCount = 0;
    let skipCount = 0;
    const bfs = component.bfs({
      roots: `#${rootNode.id()}`,
      visit: (v, e, u, i, depth) => {
        if (!e) return;
        visitCount++;
        const edgeData = e.data();
        const connection: Connection | undefined = edgeData.connectionData;
        if (!connection) {
          skipCount++;
          return;
        }
        const parentNode = u;
        const childNode = v;
        const parentId = parentNode.id();
        const childId = childNode.id();
        const parentPiece = pieceMap[parentId];
        const childPiece = pieceMap[childId];
        if (!parentPiece || !childPiece || !parentPiece.guid || !childPiece.guid) {
          skipCount++;
          return;
        }
        if (piecePlanes[childPiece.guid]) return;
        const parentPlane = piecePlanes[parentPiece.guid];
        if (!parentPlane) {
          console.error(`Error during flatten: Parent piece ${parentPiece.guid} plane not found.`);
          skipCount++;
          return;
        }
        const parentSide = connection.connected.piece.guid === parentId ? connection.connected : connection.connecting;
        const childSide = connection.connecting.piece.guid === childId ? connection.connecting : connection.connected;
        const parentType = parentPiece.type ? getType(parentPiece.type.guid) : undefined;
        const childType = childPiece.type ? getType(childPiece.type.guid) : undefined;

        const parentConnectorGuid = parentSide.connector?.guid;
        const childConnectorGuid = childSide.connector?.guid;
        const parentConnector = getConnector(parentType, parentConnectorGuid);
        const childConnector = getConnector(childType, childConnectorGuid);

        if (!parentConnector || !childConnector) {
          console.error(`Error during flatten: Connectors not found for connection between ${parentId} and ${childId}. Parent Connector: ${parentConnectorGuid}, Child Connector: ${childConnectorGuid}`);
          skipCount++;
          return;
        }
        const childPlane = roundPlane(computeChildPlane(parentPlane, parentConnector, childConnector, connection));
        piecePlanes[childPiece.guid] = childPlane;

        const radius = 2.697;
        const verticalVExtra = 1.0;
        const horizontalScale = 3.0633;
        const parentCenter = parentPiece.center || { u: 0, v: 0 };
        const connectionU = connection.u ?? 0;
        const connectionV = connection.v ?? 0;

        let childU: number;
        let childV: number;

        if (parentCenter.u === 0 && parentCenter.v === 0) {
          const angle = 2 * Math.PI * parentConnector.t;
          childU = radius * Math.sin(angle);
          childV = radius * Math.cos(angle);
        } else {
          const isVerticalConnection = Math.abs(parentConnector.direction?.z ?? 0) > 0.5;

          if (isVerticalConnection) {
            childU = parentCenter.u + connectionU;
            childV = parentCenter.v + connectionV + verticalVExtra;
          } else {
            childU = parentCenter.u + connectionU * horizontalScale;
            childV = parentCenter.v + connectionV * horizontalScale;
          }
        }

        const computedChildCenter = {
          u: round(childU),
          v: round(childV),
        };
        const childCenter = childPiece.center ?? computedChildCenter;

        const flatChildPiece: Piece = setAttributes(
          {
            ...childPiece,
            plane: childPlane,
            center: childCenter,
          },
          [
            {
              key: "semio.fixedPieceId",
              value: parentPiece.attributes?.find((q) => q.key === "semio.fixedPieceId")?.value ?? "",
            },
            {
              key: "semio.parentPieceId",
              value: parentPiece.guid,
            },
            {
              key: "semio.depth",
              value: depth.toString(),
            },
          ],
        );
        pieceMap[childId] = flatChildPiece;
      },
      directed: false,
    });
  });
  flatDesign.pieces = flatDesign.pieces?.map((p) => pieceMap[p.guid ?? ""]);
  flatDesign.connections = [];

  let piecesWithPlanes = 0;
  let piecesWithoutPlanes = 0;
  const updatedPieces = flatDesign.pieces
    ?.map((flatPiece) => {
      if (flatPiece.plane) piecesWithPlanes++;
      else piecesWithoutPlanes++;

      const originalPiece = design.pieces?.find((p) => p.guid === flatPiece.guid);
      if (!originalPiece) return null;

      const pieceDiff: PieceDiff = {};

      if (flatPiece.plane && JSON.stringify(flatPiece.plane) !== JSON.stringify(originalPiece.plane)) {
        pieceDiff.plane = flatPiece.plane;
      }

      if (flatPiece.center && JSON.stringify(flatPiece.center) !== JSON.stringify(originalPiece.center)) {
        pieceDiff.center = flatPiece.center;
      }
      if (JSON.stringify(flatPiece.attributes) !== JSON.stringify(originalPiece.attributes)) {
        pieceDiff.attributes = getAttributesDiff(originalPiece.attributes ?? [], flatPiece.attributes ?? []);
      }

      if (Object.keys(pieceDiff).length === 0) return null;

      return {
        piece: { guid: flatPiece.guid },
        diff: pieceDiff,
      };
    })
    .filter((update) => update !== null) as Array<{ piece: PieceId; diff: PieceDiff }>;

  const removedConnections = design.connections?.map((c) => ({ connected: { piece: c.connected.piece.guid }, connecting: { piece: c.connecting.piece.guid } })) || [];

  const forward = {
    pieces: updatedPieces.length > 0 ? { updated: updatedPieces } : undefined,
    connections: removedConnections.length > 0 ? { removed: removedConnections } : undefined,
  } as DesignDiff;
  const backward = inverseDesignDiff(design, forward);
  return { forward, backward };
};

/**
 * Performs the createClusteredDesign operation.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖design🪨createclustereddesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/createClusteredDesign)
 **/
export const createClusteredDesign = (originalDesign: Design, clusterPieceIds: string[], designName: string): { clusteredDesign: Design; externalConnections: Connection[] } => {
  if (!originalDesign.pieces || originalDesign.pieces.length === 0) {
    throw new Error("Original design has no pieces to cluster");
  }
  if (!clusterPieceIds || clusterPieceIds.length === 0) {
    throw new Error("No piece IDs provided for clustering");
  }

  const clusteredPieces = (originalDesign.pieces || []).filter((piece) => clusterPieceIds.includes(piece.guid));

  if (clusteredPieces.length === 0) {
    throw new Error("No pieces found matching the provided IDs");
  }

  const internalConnections = (originalDesign.connections || []).filter((connection) => clusterPieceIds.includes(connection.connected.piece.guid) && clusterPieceIds.includes(connection.connecting.piece.guid));

  const externalConnections = (originalDesign.connections || []).filter((connection) => {
    const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.guid);
    const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.guid);
    return connectedInCluster !== connectingInCluster;
  });

  const clusteredDesign: Design = {
    guid: guid(),
    name: designName,
    unit: originalDesign.unit,
    description: `Clustered design with ${clusteredPieces.length} pieces`,
    pieces: clusteredPieces,
    connections: internalConnections,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };

  return { clusteredDesign, externalConnections };
};

/**
 * Performs the replaceClusterWithDesign operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨replaceclusterwithdesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/replaceClusterWithDesign)
 **/
export const replaceClusterWithDesign = (originalDesign: Design, clusterPieceIds: string[], clusteredDesign: Design, externalConnections: Connection[]): DesignChange => {
  const piecesToRemove = clusterPieceIds.map((guid) => ({ guid }));

  const connectionsToRemove = (originalDesign.connections || [])
    .filter((connection) => {
      const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.guid);
      const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.guid);
      return connectedInCluster || connectingInCluster;
    })
    .map((c) => ({ guid: c.guid }));

  const updatedExternalConnections = externalConnections.map((connection) => {
    const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.guid);
    const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.guid);

    if (connectedInCluster) {
      return {
        ...connection,
        connected: {
          ...connection.connected,
          designPiece: { guid: clusteredDesign.guid },
        },
      };
    } else if (connectingInCluster) {
      return {
        ...connection,
        connecting: {
          ...connection.connecting,
          designPiece: { guid: clusteredDesign.guid },
        },
      };
    }

    return connection;
  });

  const forward: DesignDiff = {
    pieces: {
      removed: piecesToRemove,
    },
    connections: {
      removed: connectionsToRemove,
      added: updatedExternalConnections,
    },
  };
  const backward = inverseDesignDiff(originalDesign, forward);
  return { forward, backward };
};

/**
 * Retrieves the ClusterableGroups value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖design🪨getclusterablegroups](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/getClusterableGroups)
 **/
export const getClusterableGroups = (design: Design, selectedPieceIds: string[]): string[][] => {
  if (selectedPieceIds.length < 2) return [];

  const adjacencyMap = new Map<string, Set<string>>();
  (design.connections || []).forEach((connection) => {
    const sourceId = connection.connecting.piece.guid;
    const targetId = connection.connected.piece.guid;

    if (!adjacencyMap.has(sourceId)) adjacencyMap.set(sourceId, new Set());
    if (!adjacencyMap.has(targetId)) adjacencyMap.set(targetId, new Set());

    adjacencyMap.get(sourceId)!.add(targetId);
    adjacencyMap.get(targetId)!.add(sourceId);
  });

  const visited = new Set<string>();
  const connectedGroups: string[][] = [];

  const dfs = (pieceId: string, currentGroup: string[]) => {
    if (visited.has(pieceId)) return;
    visited.add(pieceId);
    currentGroup.push(pieceId);

    const neighbors = adjacencyMap.get(pieceId) || new Set();
    for (const neighbor of Array.from(neighbors)) {
      if (selectedPieceIds.includes(neighbor) && !visited.has(neighbor)) {
        dfs(neighbor, currentGroup);
      }
    }
  };

  for (const pieceId of selectedPieceIds) {
    if (!visited.has(pieceId)) {
      const group: string[] = [];
      dfs(pieceId, group);
      connectedGroups.push(group);
    }
  }

  const pieceGuidSet = new Set((design.pieces || []).map((piece) => piece.guid));
  const hasDesignNodes = selectedPieceIds.some((id) => !pieceGuidSet.has(id));
  const hasMultipleComponents = connectedGroups.length > 1;
  const hasLargeConnectedGroup = connectedGroups.some((group) => group.length > 1);

  if (hasDesignNodes || hasMultipleComponents || hasLargeConnectedGroup) {
    return [selectedPieceIds];
  }

  return [];
};

/**
 * Performs the expandDesignPieces operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨expanddesignpieces](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/expandDesignPieces)
 **/
export const expandDesignPieces = (design: Design, kit: Kit): Design => {
  const hasDesignConnections = design.connections?.some((conn) => conn.connected.designPiece || conn.connecting.designPiece);
  if (!hasDesignConnections) {
    return design;
  }

  let expandedDesign = { ...design };

  const designIds = new Set<string>();
  toArray(design.connections).forEach((conn) => {
    if (conn.connected.designPiece) designIds.add(conn.connected.designPiece.guid);
    if (conn.connecting.designPiece) designIds.add(conn.connecting.designPiece.guid);
  });

  if (designIds.size === 0) {
    return expandedDesign;
  }

  for (const designName of Array.from(designIds)) {
    const referencedDesign = findDesignInKit(kit, designName);
    if (!referencedDesign) continue;

    const expandedReferencedDesign = expandDesignPieces(referencedDesign, kit);

    const transformedPieces = (expandedReferencedDesign.pieces || []).map((piece) => ({
      ...piece,
      center: piece.center || { u: 0, v: 0 },
    }));

    const transformedConnections = expandedReferencedDesign.connections || [];

    const updatedExternalConnections = (expandedDesign.connections || []).map((connection) => {
      if (connection.connected.designPiece?.guid === designName) {
        return {
          ...connection,
          connected: {
            ...connection.connected,
            designPiece: undefined,
          },
        };
      }

      if (connection.connecting.designPiece?.guid === designName) {
        return {
          ...connection,
          connecting: {
            ...connection.connecting,
            designPiece: undefined,
          },
        };
      }

      return connection;
    });

    expandedDesign = {
      ...expandedDesign,
      pieces: [...(expandedDesign.pieces || []), ...transformedPieces],
      connections: [...updatedExternalConnections, ...transformedConnections],
    };
  }

  return expandedDesign;
};

/**
 * Type alias for IncludedDesignInfo.
 **/
export type IncludedDesignInfo = {
  guid: string;
  designGuid: string;
  type: "connected" | "fixed";
  center?: Coord;
  plane?: Plane;
  externalConnections?: Connection[];
};

/**
 * Retrieves the IncludedDesigns value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖design🪨getincludeddesigns](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/getIncludedDesigns)
 **/
export const getIncludedDesigns = (design: Design): IncludedDesignInfo[] => {
  const includedDesigns: IncludedDesignInfo[] = [];

  const designIds = new Set<string>();
  toArray(design.connections).forEach((conn: Connection) => {
    if (conn.connected.designPiece) designIds.add(conn.connected.designPiece.guid);
    if (conn.connecting.designPiece) designIds.add(conn.connecting.designPiece.guid);
  });

  Array.from(designIds).forEach((designIdString) => {
    const externalConnections =
      design.connections?.filter((connection: Connection) => {
        const connectedToDesign = connection.connected.designPiece?.guid === designIdString;
        const connectingToDesign = connection.connecting.designPiece?.guid === designIdString;
        return connectedToDesign || connectingToDesign;
      }) ?? [];

    includedDesigns.push({
      guid: designIdString,
      designGuid: designIdString,
      type: "connected",
      externalConnections,
    });
  });

  return includedDesigns;
};

/**
 * Performs the isPortInUse operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨isportinuse](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/isPortInUse)
 **/
export const isPortInUse = (design: Design, pieceGuid: string, connectorGuid: string): boolean => {
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  for (const connection of connections) {
    const isPieceConnected = connection.connected.piece.guid === pieceGuid;
    const isPortConnected = isPieceConnected ? connection.connected.connector?.guid === connectorGuid : connection.connecting.connector?.guid === connectorGuid;
    if (isPortConnected) return true;
  }
  return false;
};

/**
 * Performs the isConnectionInDesign operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖design🪨isconnectionindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/isConnectionInDesign)
 **/
export const isConnectionInDesign = (design: Design, connection: Connection): boolean => {
  return design.connections?.some((c) => areSameConnection(c, connection)) ?? false;
};

/**
 * Searches for matching PieceInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖design🪨findpieceindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/findPieceInDesign)
 **/
export const findPieceInDesign = (design: Design, pieceGuid: string): Piece => findPiece(design.pieces ?? [], pieceGuid);

/**
 * Searches for matching ConnectionInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖design🪨findconnectionindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/findConnectionInDesign)
 **/
export const findConnectionInDesign = (design: Design, connectionGuid: string): Connection => {
  return findConnection(design.connections ?? [], connectionGuid);
};

/**
 * Searches for matching ConnectionsInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖design🪨findconnectionsindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/findConnectionsInDesign)
 **/
export const findConnectionsInDesign = (design: Design, connectionGuids: string[]): Connection[] => {
  return connectionGuids.map((connectionGuid) => findConnectionInDesign(design, connectionGuid));
};

/**
 * Searches for matching PieceConnectionsInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖design🪨findpiececonnectionsindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/findPieceConnectionsInDesign)
 **/
export const findPieceConnectionsInDesign = (design: Design, pieceGuid: string): Connection[] => {
  return findPieceConnections(design.connections ?? [], pieceGuid);
};

/**
 * Searches for matching ConnectionPiecesInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖design🪨findconnectionpiecesindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/findConnectionPiecesInDesign)
 **/
export const findConnectionPiecesInDesign = (design: Design, connection: Connection): { connecting: Piece; connected: Piece } => {
  return {
    connected: findPieceInDesign(design, connection.connected.piece.guid),
    connecting: findPieceInDesign(design, connection.connecting.piece.guid),
  };
};

/**
 * Searches for matching StaleConnectionsInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖design🪨findstaleconnectionsindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/findStaleConnectionsInDesign)
 **/
export const findStaleConnectionsInDesign = (design: Design): Connection[] => {
  return (
    design.connections?.filter((c) => {
      try {
        findPieceInDesign(design, c.connected.piece.guid);
        findPieceInDesign(design, c.connecting.piece.guid);
        return false;
      } catch (e) {
        return true;
      }
    }) ?? []
  );
};

/**
 * Computes a DesignDiff that offsets selected piece centers and adjusts orphan connections.
 * MUST return a DesignDiff with center offsets for root movers and u/v offsets for orphan connections.
 * [👤semio📚js💻semiots🔖design🪨dragpiecesindesign](repo://definition/SEMIO/JS/SEMIO.TS/DESIGN/DRAG-PIECES-IN-DESIGN)
 **/
export const dragPiecesInDesign = (design: Design, pieces: Design, offset: Coord): DesignDiff => {
  const selectedGuids = new Set((pieces.pieces ?? []).map((p) => p.guid));
  const connections = design.connections ?? [];
  const designPieces = design.pieces ?? [];
  const parentMap = new Map<string, { connectionGuid: string; parentGuid: string }>();
  const childrenMap = new Map<string, string[]>();
  for (const c of connections) {
    parentMap.set(c.connected.piece.guid, { connectionGuid: c.guid, parentGuid: c.connecting.piece.guid });
    const children = childrenMap.get(c.connecting.piece.guid) ?? [];
    children.push(c.connected.piece.guid);
    childrenMap.set(c.connecting.piece.guid, children);
  }
  const rootMovers = new Set<string>();
  for (const p of pieces.pieces ?? []) {
    if (designPieces.find((dp) => dp.guid === p.guid)?.center) {
      rootMovers.add(p.guid);
    }
  }
  const movingSet = new Set<string>();
  const queue = [...rootMovers];
  while (queue.length > 0) {
    const guid = queue.pop()!;
    if (movingSet.has(guid)) continue;
    movingSet.add(guid);
    for (const child of childrenMap.get(guid) ?? []) {
      queue.push(child);
    }
  }
  const pieceUpdates: { piece: { guid: string }; diff: PieceDiff }[] = [];
  for (const guid of rootMovers) {
    pieceUpdates.push({ piece: { guid }, diff: { center: { u: offset.u, v: offset.v } } });
  }
  const connectionUpdates: { connection: { guid: string }; diff: ConnectionDiff }[] = [];
  for (const guid of selectedGuids) {
    if (movingSet.has(guid)) continue;
    const parent = parentMap.get(guid);
    if (!parent) continue;
    connectionUpdates.push({ connection: { guid: parent.connectionGuid }, diff: { u: offset.u, v: offset.v } });
  }
  const diff: DesignDiff = {};
  if (pieceUpdates.length > 0 || connectionUpdates.length > 0) {
    if (pieceUpdates.length > 0) diff.pieces = { updated: pieceUpdates };
    if (connectionUpdates.length > 0) diff.connections = { updated: connectionUpdates };
  }
  return diff;
};

// #endregion 🔖Design

// #region 🔖Kit

// [👤semio📚js💻semio🔖kit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit)
// Kit entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Kit validation.
 * [👤semio📚js💻semio🔖kit🪨kitschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/KitSchema)
 **/
export const KitSchema = z.object({
  guid: z.string(),
  name: z.string(),
  version: z.string().optional(),
  types: z.array(TypeSchema).optional(),
  designs: z.array(DesignSchema).optional(),
  tags: z.array(TagSchema).optional(),
  concepts: z.array(ConceptSchema).optional(),
  ports: z.array(PortSchema).optional(),
  qualities: z.array(QualitySchema).optional(),
  files: z.array(FileSchema).optional(),
  folders: z.array(FolderSchema).optional(),
  authors: z.array(AuthorSchema).optional(),
  remote: z.string().optional(),
  homepage: z.string().optional(),
  license: z.string().optional(),
  preview: z.string().optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
/**
 * Type alias for Kit.
 * [👤semio📚js💻semio🔖kit🛠️kit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/Kit)
 **/
export type Kit = z.infer<typeof KitSchema>;
/**
 * Serializes Kit for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖kit🪨serializekit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/serializeKit)
 **/
export const serializeKit = (kit: Kit): string => JSON.stringify(KitSchema.parse(kit));
/**
 * Performs the deserializeKit operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖kit🪨deserializekit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/deserializeKit)
 **/
export const deserializeKit = (json: string): Kit => KitSchema.parse(JSON.parse(json));

/**
 * Definition of KitShallowSchema.
 * [👤semio📚js💻semio🔖kit🪨kitshallowschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/KitShallowSchema)
 **/
export const KitShallowSchema = KitSchema.omit({ types: true, designs: true, tags: true, concepts: true, ports: true, qualities: true, folders: true, authors: true }).extend({
  types: z.array(z.string()).optional(),
  designs: z.array(z.string()).optional(),
  tags: z.array(z.string()).optional(),
  concepts: z.array(z.string()).optional(),
  ports: z.array(z.string()).optional(),
  qualities: z.array(z.string()).optional(),
  folders: z.array(z.string()).optional(),
  authors: z.array(z.string()).optional(),
});
/**
 * Type alias for KitShallow.
 * [👤semio📚js💻semio🔖kit🛠️kitshallow](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/KitShallow)
 **/
export type KitShallow = z.infer<typeof KitShallowSchema>;
/**
 * Serializes KitShallow for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖kit🪨serializekitshallow](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/serializeKitShallow)
 **/
export const serializeKitShallow = (kit: KitShallow): string => JSON.stringify(KitShallowSchema.parse(kit));
/**
 * Performs the deserializeKitShallow operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖kit🪨deserializekitshallow](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/deserializeKitShallow)
 **/
export const deserializeKitShallow = (json: string): KitShallow => KitShallowSchema.parse(JSON.parse(json));
/**
 * Zod schema for Kit diff validation.
 * [👤semio📚js💻semio🔖kit🪨kitdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/KitDiffSchema)
 **/
export const KitDiffSchema = KitSchema.partial().omit({ types: true, designs: true, tags: true, concepts: true, ports: true, qualities: true, authors: true, files: true, folders: true, attributes: true }).extend({
  types: TypesDiffSchema.optional(),
  designs: DesignsDiffSchema.optional(),
  tags: TagsDiffSchema.optional(),
  concepts: ConceptsDiffSchema.optional(),
  ports: PortsDiffSchema.optional(),
  qualities: QualitiesDiffSchema.optional(),
  authors: AuthorsDiffSchema.optional(),
  files: FilesDiffSchema.optional(),
  folders: FoldersDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
  icon: z.string().nullable().optional(),
  image: z.string().nullable().optional(),
  remote: z.string().nullable().optional(),
  homepage: z.string().nullable().optional(),
  license: z.string().nullable().optional(),
  preview: z.string().nullable().optional(),
});
/**
 * Diff type for tracking Kit changes.
 * [👤semio📚js💻semio🔖kit🛠️kitdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/KitDiff)
 **/
export type KitDiff = z.infer<typeof KitDiffSchema>;
// EntityIdType holds the data fields for a EntityIdType record.
// [👤semio📚js💻semio🔖kit✂️entityidtype](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/EntityIdType)
type EntityIdType = { guid: string };
// [👤semio📚js💻semio🔖kit✂️collectiondiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/CollectionDiff)
// CollectionDiff holds the data fields for a CollectionDiff record.
type CollectionDiff<K extends string, T extends { guid: string }, D> = {
  removed?: EntityIdType[];
  updated?: Array<{ [key in K]: EntityIdType } & { diff: D }>;
  added?: T[];
};

// [👤semio📚js💻semio🔖kit🪨getcollectiondiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/getCollectionDiff)
// getCollectionDiff holds the data fields for a getCollectionDiff record.
const getCollectionDiff = <K extends string, T extends { guid: string }, D>(entityKey: K, before: T[], after: T[], getItemDiff: (before: T, after: T) => D): CollectionDiff<K, T, D> => {
  const diff: CollectionDiff<K, T, D> = {};
  const beforeGuids = new Set(before.map((i) => i.guid));
  const afterGuids = new Set(after.map((i) => i.guid));
  const removed = before.filter((i) => !afterGuids.has(i.guid)).map((i) => ({ guid: i.guid }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((i) => afterGuids.has(i.guid))
    .map((i) => {
      const afterItem = after.find((a) => a.guid === i.guid)!;
      const itemDiff = getItemDiff(i, afterItem);
      return { [entityKey]: { guid: i.guid }, diff: itemDiff } as { [key in K]: EntityIdType } & { diff: D };
    })
    .filter((u) => Object.keys(u.diff as any).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((i) => !beforeGuids.has(i.guid));
  if (added.length > 0) diff.added = added;
  return diff;
};

// [👤semio📚js💻semio🔖kit🪨inversecollectiondiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/inverseCollectionDiff)
// inverseCollectionDiff holds the data fields for a inverseCollectionDiff record.
const inverseCollectionDiff = <K extends string, T extends { guid: string }, D>(entityKey: K, original: T[], appliedDiff: CollectionDiff<K, T, D>, inverseItemDiff: (original: T, appliedDiff: D) => D): CollectionDiff<K, T, D> => {
  const inverse: CollectionDiff<K, T, D> = {};
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((i) => removedGuids.includes(i.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((i) => ({ guid: i.guid }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated
      .filter((u) => {
        const entityId = (u as any)[entityKey] as EntityIdType;
        return original.some((i) => i.guid === entityId.guid);
      })
      .map((u) => {
        const entityId = (u as any)[entityKey] as EntityIdType;
        const originalItem = original.find((i) => i.guid === entityId.guid)!;
        return { [entityKey]: entityId, diff: inverseItemDiff(originalItem, u.diff) } as { [key in K]: EntityIdType } & { diff: D };
      });
  }
  return inverse;
};

// [👤semio📚js💻semio🔖kit🪨applycollectiondiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/applyCollectionDiff)
// applyCollectionDiff holds the data fields for a applyCollectionDiff record.
const applyCollectionDiff = <K extends string, T extends { guid: string }, D>(entityKey: K, base: T[], diff: CollectionDiff<K, T, D> | undefined, applyItemDiff: (base: T, diff: D) => T): T[] => {
  if (!diff) return base;
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((i) => !removedGuids.has(i.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const entityId = (update as any)[entityKey] as EntityIdType;
      const index = result.findIndex((i) => i.guid === entityId.guid);
      if (index !== -1) {
        result[index] = applyItemDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

// mergeCollectionDiff holds the data fields for a mergeCollectionDiff record.
// [👤semio📚js💻semio🔖kit🪨mergecollectiondiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/mergeCollectionDiff)
const mergeCollectionDiff = <K extends string, T extends { guid: string }, D>(entityKey: K, diff1: CollectionDiff<K, T, D>, diff2: CollectionDiff<K, T, D>, mergeItemDiff: (diff1: D, diff2: D) => D): CollectionDiff<K, T, D> => {
  const removed = [...(diff1.removed ?? []), ...(diff2.removed ?? [])];
  const added = [...(diff1.added ?? []), ...(diff2.added ?? [])];
  const getEntityGuid = (u: any) => (u[entityKey] as EntityIdType).guid;
  const updated1Map = new Map((diff1.updated ?? []).map((u) => [getEntityGuid(u), u.diff]));
  const updated2Map = new Map((diff2.updated ?? []).map((u) => [getEntityGuid(u), u.diff]));
  const allGuids = new Set([...updated1Map.keys(), ...updated2Map.keys()]);
  const updated = Array.from(allGuids).map((guid) => ({
    [entityKey]: { guid },
    diff: mergeItemDiff(updated1Map.get(guid) ?? ({} as D), updated2Map.get(guid) ?? ({} as D)),
  })) as Array<{ [key in K]: EntityIdType } & { diff: D }>;
  return {
    removed: removed.length > 0 ? removed : undefined,
    updated: updated.length > 0 ? updated : undefined,
    added: added.length > 0 ? added : undefined,
  };
};

/**
 * Retrieves the KitDiff value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖kit🪨getkitdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/getKitDiff)
 **/
export const getKitDiff = (before: Kit, after: Kit): KitDiff => {
  const diff: KitDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.version !== after.version) diff.version = after.version;
  if (before.description !== after.description) diff.description = after.description;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.image !== after.image) diff.image = after.image;
  if (before.remote !== after.remote) diff.remote = after.remote;
  if (before.homepage !== after.homepage) diff.homepage = after.homepage;
  if (before.license !== after.license) diff.license = after.license;
  if (before.preview !== after.preview) diff.preview = after.preview;
  const typesDiff = getCollectionDiff("type", before.types ?? [], after.types ?? [], getTypeDiff);
  if (Object.keys(typesDiff).length > 0) diff.types = typesDiff;
  const designsDiff = getCollectionDiff("design", before.designs ?? [], after.designs ?? [], getDesignDiff);
  if (Object.keys(designsDiff).length > 0) diff.designs = designsDiff;
  const tagsDiff = getTagsDiff(before.tags ?? [], after.tags ?? []);
  if (Object.keys(tagsDiff).length > 0) diff.tags = tagsDiff;
  const conceptsDiff = getConceptsDiff(before.concepts ?? [], after.concepts ?? []);
  if (Object.keys(conceptsDiff).length > 0) diff.concepts = conceptsDiff;
  const portsDiff = getPortsDiff(before.ports ?? [], after.ports ?? []);
  if (Object.keys(portsDiff).length > 0) diff.ports = portsDiff;
  const qualitiesDiff = getCollectionDiff("quality", before.qualities ?? [], after.qualities ?? [], getQualityDiff);
  if (Object.keys(qualitiesDiff).length > 0) diff.qualities = qualitiesDiff;
  const filesDiff = getCollectionDiff("file", before.files ?? [], after.files ?? [], getFileDiff);
  if (Object.keys(filesDiff).length > 0) diff.files = filesDiff;
  const foldersDiff = getCollectionDiff("folder", before.folders ?? [], after.folders ?? [], getFolderDiff);
  if (Object.keys(foldersDiff).length > 0) diff.folders = foldersDiff;
  const authorsDiff = getCollectionDiff("author", before.authors ?? [], after.authors ?? [], getAuthorDiff);
  if (Object.keys(authorsDiff).length > 0) diff.authors = authorsDiff;
  const attributesDiff = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
  return diff;
};
/**
 * Diff type for tracking inverseKit changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖kit🪨inversekitdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/inverseKitDiff)
 **/
export const inverseKitDiff = (original: Kit, appliedDiff: KitDiff): KitDiff => {
  const inverse: KitDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.version !== undefined) inverse.version = original.version;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.image !== undefined) inverse.image = original.image ?? null;
  if (appliedDiff.remote !== undefined) inverse.remote = original.remote ?? null;
  if (appliedDiff.homepage !== undefined) inverse.homepage = original.homepage ?? null;
  if (appliedDiff.license !== undefined) inverse.license = original.license ?? null;
  if (appliedDiff.preview !== undefined) inverse.preview = original.preview ?? null;
  if (appliedDiff.types) inverse.types = inverseCollectionDiff("type", original.types ?? [], appliedDiff.types, inverseTypeDiff);
  if (appliedDiff.designs) inverse.designs = inverseCollectionDiff("design", original.designs ?? [], appliedDiff.designs, inverseDesignDiff);
  if (appliedDiff.tags) inverse.tags = inverseTagsDiff(original.tags ?? [], appliedDiff.tags);
  if (appliedDiff.concepts) inverse.concepts = inverseConceptsDiff(original.concepts ?? [], appliedDiff.concepts);
  if (appliedDiff.ports) inverse.ports = inversePortsDiff(original.ports ?? [], appliedDiff.ports);
  if (appliedDiff.qualities) inverse.qualities = inverseCollectionDiff("quality", original.qualities ?? [], appliedDiff.qualities, inverseQualityDiff);
  if (appliedDiff.files) inverse.files = inverseCollectionDiff("file", original.files ?? [], appliedDiff.files, inverseFileDiff);
  if (appliedDiff.folders) inverse.folders = inverseCollectionDiff("folder", original.folders ?? [], appliedDiff.folders, inverseFolderDiff);
  if (appliedDiff.authors) inverse.authors = inverseCollectionDiff("author", original.authors ?? [], appliedDiff.authors, inverseAuthorDiff);
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeKit changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖kit🪨mergekitdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/mergeKitDiff)
 **/
export const mergeKitDiff = (diff1: KitDiff, diff2: KitDiff): KitDiff => {
  const mergeSimpleDiff = <D>(d1: D, d2: D): D => ({ ...d1, ...d2 });
  return {
    ...diff1,
    ...diff2,
    types: diff1.types || diff2.types ? mergeCollectionDiff("type", diff1.types ?? {}, diff2.types ?? {}, mergeTypeDiff) : undefined,
    designs: diff1.designs || diff2.designs ? mergeCollectionDiff("design", diff1.designs ?? {}, diff2.designs ?? {}, mergeDesignDiff) : undefined,
    tags: diff1.tags || diff2.tags ? mergeTagsDiff(diff1.tags ?? {}, diff2.tags ?? {}) : undefined,
    concepts: diff1.concepts || diff2.concepts ? mergeConceptsDiff(diff1.concepts ?? {}, diff2.concepts ?? {}) : undefined,
    ports: diff1.ports || diff2.ports ? mergePortsDiff(diff1.ports ?? {}, diff2.ports ?? {}) : undefined,
    qualities: diff1.qualities || diff2.qualities ? mergeCollectionDiff("quality", diff1.qualities ?? {}, diff2.qualities ?? {}, mergeQualityDiff) : undefined,
    files: diff1.files || diff2.files ? mergeCollectionDiff("file", diff1.files ?? {}, diff2.files ?? {}, mergeSimpleDiff) : undefined,
    folders: diff1.folders || diff2.folders ? mergeCollectionDiff("folder", diff1.folders ?? {}, diff2.folders ?? {}, mergeSimpleDiff) : undefined,
    authors: diff1.authors || diff2.authors ? mergeCollectionDiff("author", diff1.authors ?? {}, diff2.authors ?? {}, mergeSimpleDiff) : undefined,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};
/**
 * Diff type for tracking applyKit changes.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖kit🪨applykitdiff](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/applyKitDiff)
 **/
export const applyKitDiff = (base: Kit, diff: KitDiff): Kit => {
  const result: any = {
    guid: base.guid,
    name: "name" in diff ? diff.name! : base.name,
    version: "version" in diff ? diff.version! : base.version,
    createdAt: base.createdAt,
    updatedAt: diff.updatedAt ?? base.updatedAt,
  };

  const optionalScalars = ["description", "icon", "image", "remote", "homepage", "license", "preview"] as const;
  for (const key of optionalScalars) {
    if (key in diff) {
      const value = diff[key] ?? undefined;
      if (value !== undefined) result[key] = value;
    } else if (key in base && base[key] !== undefined) {
      result[key] = base[key];
    }
  }

  if (diff.types || base.types) {
    const types = applyCollectionDiff("type", base.types ?? [], diff.types, applyTypeDiff);
    if (types.length > 0) result.types = types;
  }
  if (diff.designs || base.designs) {
    const designs = applyCollectionDiff("design", base.designs ?? [], diff.designs, applyDesignDiff);
    if (designs.length > 0) result.designs = designs;
  }
  if (diff.tags || base.tags) {
    const tags = applyTagsDiff(base.tags ?? [], diff.tags ?? {});
    if (tags.length > 0) result.tags = tags;
  }
  if (diff.concepts || base.concepts) {
    const concepts = applyConceptsDiff(base.concepts ?? [], diff.concepts ?? {});
    if (concepts.length > 0) result.concepts = concepts;
  }
  if (diff.ports || base.ports) {
    const ports = applyPortsDiff(base.ports ?? [], diff.ports ?? {});
    if (ports.length > 0) result.ports = ports;
  }
  if (diff.qualities || base.qualities) {
    const qualities = applyCollectionDiff("quality", base.qualities ?? [], diff.qualities, applyQualityDiff);
    if (qualities.length > 0) result.qualities = qualities;
  }
  if (diff.files || base.files) {
    const files = applyCollectionDiff("file", base.files ?? [], diff.files, applyFileDiff);
    if (files.length > 0) result.files = files;
  }
  if (diff.folders || base.folders) {
    const folders = applyCollectionDiff("folder", base.folders ?? [], diff.folders, applyFolderDiff);
    if (folders.length > 0) result.folders = folders;
  }
  if (diff.authors || base.authors) {
    const authors = applyCollectionDiff("author", base.authors ?? [], diff.authors, applyAuthorDiff);
    if (authors.length > 0) result.authors = authors;
  }
  if (diff.attributes || base.attributes) {
    const attributes = applyAttributesDiff(base.attributes ?? [], diff.attributes ?? {});
    if (attributes.length > 0) result.attributes = attributes;
  }

  return result as Kit;
};

/**
 * Represents a bidirectional change between two Kit states.
 * MUST contain both forward and backward diffs.
 * [👤semio📚js💻semio🔖kit🛠️kitchange](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/KitChange)
 **/
export interface KitChange {
  forward: KitDiff;
  backward: KitDiff;
}
/**
 * Computes the forward and backward diffs between two kit states.
 * MUST return both the forward diff and its inverse.
 * [👤semio📚js💻semio🔖kit🪨getkitchange](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/getKitChange)
 **/
export const getKitChange = (before: Kit, after: Kit): KitChange => {
  const forward = getKitDiff(before, after);
  const backward = inverseKitDiff(before, forward);
  return { forward, backward };
};

/**
 * Represents a reversible design change with forward and backward diffs.
 * MUST contain both forward and backward diffs.
 * [👤semio📚js💻semio🔖design🛠️designchange](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/DesignChange)
 **/
export interface DesignChange {
  forward: DesignDiff;
  backward: DesignDiff;
}
/**
 * Computes the forward and backward diffs between two design states.
 * MUST return both the forward diff and its inverse.
 * [👤semio📚js💻semio🔖design🪨getdesignchange](repo://p/u/semio/b/l/js/f/semio.ts/s/Design/d/i/getDesignChange)
 **/
export const getDesignChange = (before: Design, after: Design): DesignChange => {
  const forward = getDesignDiff(before, after);
  const backward = inverseDesignDiff(before, forward);
  return { forward, backward };
};

/**
 * Zod schema for Kits diff validation.
 * [👤semio📚js💻semio🔖kit🪨kitsdiffschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/KitsDiffSchema)
 **/
export const KitsDiffSchema = z.object({
  removed: z.array(KitIdSchema).optional(),
  updated: z.array(z.object({ kit: KitIdSchema, diff: KitDiffSchema })).optional(),
  added: z.array(KitSchema).optional(),
});

/**
 * Adds a TypeToKit element.
 * MUST append the element to the collection.
 * [👤semio📚js💻semio🔖kit🪨addtypetokit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/addTypeToKit)
 **/
export const addTypeToKit = (kit: Kit, type: Type): KitChange => {
  const forward: KitDiff = { types: { added: [type] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing TypeInKit element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖kit🪨settypeinkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/setTypeInKit)
 **/
export const setTypeInKit = (kit: Kit, type: Type): KitChange => {
  const forward: KitDiff = { types: { added: [type] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a TypeFromKit element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖kit🪨removetypefromkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/removeTypeFromKit)
 **/
export const removeTypeFromKit = (kit: Kit, typeGuid: string): KitChange => {
  const forward: KitDiff = { types: { removed: [{ guid: typeGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Adds a DesignToKit element.
 * MUST append the element to the collection.
 * [👤semio📚js💻semio🔖kit🪨adddesigntokit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/addDesignToKit)
 **/
export const addDesignToKit = (kit: Kit, design: Design): KitChange => {
  const forward: KitDiff = { designs: { added: [design] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing DesignInKit element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖kit🪨setdesigninkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/setDesignInKit)
 **/
export const setDesignInKit = (kit: Kit, design: Design): KitChange => {
  const forward: KitDiff = { designs: { added: [design] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a DesignFromKit element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖kit🪨removedesignfromkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/removeDesignFromKit)
 **/
export const removeDesignFromKit = (kit: Kit, designGuid: string): KitChange => {
  const forward: KitDiff = { designs: { removed: [{ guid: designGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Performs the updateDesignInKit operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖kit🪨updatedesigninkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/updateDesignInKit)
 **/
export const updateDesignInKit = (kit: Kit, design: Design): KitChange => {
  const forward: KitDiff = { designs: { added: [design] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Adds a PortToKit element.
 * MUST append the element to the collection.
 * [👤semio📚js💻semio🔖kit🪨addporttokit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/addPortToKit)
 **/
export const addPortToKit = (kit: Kit, iface: Port): KitChange => {
  const forward: KitDiff = { ports: { added: [iface] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing PortInKit element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖kit🪨setportinkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/setPortInKit)
 **/
export const setPortInKit = (kit: Kit, iface: Port): KitChange => {
  const forward: KitDiff = { ports: { added: [iface] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a PortFromKit element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖kit🪨removeportfromkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/removePortFromKit)
 **/
export const removePortFromKit = (kit: Kit, portGuid: string): KitChange => {
  const forward: KitDiff = { ports: { removed: [{ guid: portGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Performs the updatePortInKit operation.
 * MUST perform the operation correctly.
 * [👤semio📚js💻semio🔖kit🪨updateportinkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/updatePortInKit)
 **/
export const updatePortInKit = (kit: Kit, iface: Port): KitChange => {
  const forward: KitDiff = { ports: { added: [iface] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Searches for matching FileInKit entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findfileinkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findFileInKit)
 **/
export const findFileInKit = (kit: Kit, fileGuid: string): File => {
  const file = (kit.files || []).find((f) => f.guid === fileGuid);
  if (!file) throw new Error(`File ${fileGuid} not found in kit`);
  return file;
};

/**
 * Adds a FileToKit element.
 * MUST append the element to the collection.
 * [👤semio📚js💻semio🔖kit🪨addfiletokit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/addFileToKit)
 **/
export const addFileToKit = (kit: Kit, file: File): KitChange => {
  const forward: KitDiff = { files: { added: [file] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing FileInKit element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖kit🪨setfileinkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/setFileInKit)
 **/
export const setFileInKit = (kit: Kit, file: File): KitChange => {
  const forward: KitDiff = { files: { added: [file] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a FileFromKit element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖kit🪨removefilefromkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/removeFileFromKit)
 **/
export const removeFileFromKit = (kit: Kit, fileGuid: string): KitChange => {
  const forward: KitDiff = { files: { removed: [{ guid: fileGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Replaces an existing AttributeInKit element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖kit🪨setattributeinkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/setAttributeInKit)
 **/
export const setAttributeInKit = (kit: Kit, attribute: Attribute): KitChange => {
  const forward: KitDiff = { attributes: { added: [attribute] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Searches for matching TagInKit entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findtaginkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findTagInKit)
 **/
export const findTagInKit = (kit: Kit, tagGuid: string): Tag => {
  const tag = (kit.tags || []).find((t) => t.guid === tagGuid);
  if (!tag) throw new Error(`Tag ${tagGuid} not found in kit`);
  return tag;
};

/**
 * Adds a TagToKit element.
 * MUST append the element to the collection.
 * [👤semio📚js💻semio🔖kit🪨addtagtokit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/addTagToKit)
 **/
export const addTagToKit = (kit: Kit, tag: Tag): KitChange => {
  const forward: KitDiff = { tags: { added: [tag] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing TagInKit element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖kit🪨settaginkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/setTagInKit)
 **/
export const setTagInKit = (kit: Kit, tag: Tag): KitChange => {
  const forward: KitDiff = { tags: { added: [tag] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a TagFromKit element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖kit🪨removetagfromkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/removeTagFromKit)
 **/
export const removeTagFromKit = (kit: Kit, tagGuid: string): KitChange => {
  const forward: KitDiff = { tags: { removed: [{ guid: tagGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Searches for matching ConceptInKit entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findconceptinkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findConceptInKit)
 **/
export const findConceptInKit = (kit: Kit, conceptGuid: string): Concept => {
  const concept = (kit.concepts || []).find((c) => c.guid === conceptGuid);
  if (!concept) throw new Error(`Concept ${conceptGuid} not found in kit`);
  return concept;
};

/**
 * Adds a ConceptToKit element.
 * MUST append the element to the collection.
 * [👤semio📚js💻semio🔖kit🪨addconcepttokit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/addConceptToKit)
 **/
export const addConceptToKit = (kit: Kit, concept: Concept): KitChange => {
  const forward: KitDiff = { concepts: { added: [concept] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing ConceptInKit element.
 * MUST replace the existing element.
 * [👤semio📚js💻semio🔖kit🪨setconceptinkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/setConceptInKit)
 **/
export const setConceptInKit = (kit: Kit, concept: Concept): KitChange => {
  const forward: KitDiff = { concepts: { added: [concept] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a ConceptFromKit element.
 * MUST remove the element from the collection.
 * [👤semio📚js💻semio🔖kit🪨removeconceptfromkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/removeConceptFromKit)
 **/
export const removeConceptFromKit = (kit: Kit, conceptGuid: string): KitChange => {
  const forward: KitDiff = { concepts: { removed: [{ guid: conceptGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Searches for matching ReplacableDesignsForDesignPiece entry.
 * MUST return the matching element or undefined.
 **/
export const findReplacableDesignsForDesignPiece = (kit: Kit, currentDesignGuid: string, designPiece: Piece): Design[] => {
  if (!designPiece.design) return [];

  const allDesigns = kit.designs || [];
  const currentDesign = findDesignInKit(kit, designPiece.design.guid);

  return allDesigns.filter((design) => {
    if (design.guid === currentDesign.guid) return false;
    if (design.isAbstract) return false;
    return true;
  });
};

/**
 * Equality check for Kit values.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖kit🪨aresamekit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/areSameKit)
 **/
export const areSameKit = (kitGuid: string, otherGuid: string): boolean => {
  return kitGuid === otherGuid;
};
/**
 * Checks whether SameKit condition holds.
 * MUST return true if the condition is met.
 * [👤semio📚js💻semio🔖kit🪨hassamekit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/hasSameKit)
 **/
export const hasSameKit = (kitGuid: string, otherGuids: string[]): boolean => otherGuids.some((other) => areSameKit(kitGuid, other));

/**
 * Searches for matching TypeInKit entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findtypeinkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findTypeInKit)
 **/
export const findTypeInKit = (kit: Kit, typeGuid: string): Type => {
  const type = kit.types?.find((t) => t.guid === typeGuid);
  if (!type) throw new Error(`Type ${typeGuid} not found in kit ${kit.name}`);
  return type;
};

/**
 * Searches for matching DesignInKit entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨finddesigninkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findDesignInKit)
 **/
export const findDesignInKit = (kit: Kit, designGuid: string): Design => {
  const design = kit.designs?.find((d) => d.guid === designGuid);
  if (!design) throw new Error(`Design ${designGuid} not found in kit ${kit.name}`);
  return design;
};

// #region 🔖Design Family Helpers

// [👤semio📚js💻semio🔖kit🔖designfamilyhelpers](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Design%20Family%20Helpers)
// Design family traversal helpers MUST be defined here.

/**
 * Retrieves the PrimitiveDesign value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖kit🔖designfamilyhelpers🪨getprimitivedesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Design%20Family%20Helpers/d/i/getPrimitiveDesign)
 **/
export const getPrimitiveDesign = (kit: Kit, designGuid: string): Design => {
  let current = findDesignInKit(kit, designGuid);
  while (current.parent?.guid) {
    current = findDesignInKit(kit, current.parent.guid);
  }
  return current;
};

/**
 * Retrieves the DesignFamily value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖kit🔖designfamilyhelpers🪨getdesignfamily](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Design%20Family%20Helpers/d/i/getDesignFamily)
 **/
export const getDesignFamily = (kit: Kit, designGuid: string): Design[] => {
  const primitive = getPrimitiveDesign(kit, designGuid);
  const family: Design[] = [];
  const collectDescendants = (parentGuid: string) => {
    const parent = findDesignInKit(kit, parentGuid);
    family.push(parent);
    const children = (kit.designs || []).filter((d) => d.parent?.guid === parentGuid);
    children.forEach((child) => collectDescendants(child.guid));
  };
  collectDescendants(primitive.guid);
  return family;
};

/**
 * Retrieves the DesignSiblings value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖kit🔖designfamilyhelpers🪨getdesignsiblings](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Design%20Family%20Helpers/d/i/getDesignSiblings)
 **/
export const getDesignSiblings = (kit: Kit, designGuid: string): Design[] => {
  const design = findDesignInKit(kit, designGuid);
  const parentGuid = design.parent?.guid;
  return (kit.designs || []).filter((d) => d.parent?.guid === parentGuid && d.guid !== designGuid);
};

/**
 * Retrieves the DesignChildren value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖kit🔖designfamilyhelpers🪨getdesignchildren](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Design%20Family%20Helpers/d/i/getDesignChildren)
 **/
export const getDesignChildren = (kit: Kit, designGuid: string): Design[] => {
  return (kit.designs || []).filter((d) => d.parent?.guid === designGuid);
};

/**
 * Checks if Designs belong to the same family.
 * MUST return a boolean result.
 * [👤semio📚js💻semio🔖kit🔖designfamilyhelpers🪨aredesignsinsamefamily](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Design%20Family%20Helpers/d/i/areDesignsInSameFamily)
 **/
export const areDesignsInSameFamily = (kit: Kit, designGuidA: string, designGuidB: string): boolean => {
  const primitiveA = getPrimitiveDesign(kit, designGuidA);
  const primitiveB = getPrimitiveDesign(kit, designGuidB);
  return primitiveA.guid === primitiveB.guid;
};

/**
 * Checks if UseDesignAsPiece action is possible.
 * MUST return a boolean result.
 * [👤semio📚js💻semio🔖kit🔖designfamilyhelpers🪨canusedesignaspiece](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Design%20Family%20Helpers/d/i/canUseDesignAsPiece)
 **/
export const canUseDesignAsPiece = (kit: Kit, containerDesignGuid: string, pieceDesignGuid: string): boolean => {
  return !areDesignsInSameFamily(kit, containerDesignGuid, pieceDesignGuid);
};

/**
 * Searches for matching SameFamilyDesignPieces entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🔖designfamilyhelpers🪨findsamefamilydesignpieces](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Design%20Family%20Helpers/d/i/findSameFamilyDesignPieces)
 **/
export const findSameFamilyDesignPieces = (kit: Kit, designGuid: string): Piece[] => {
  const design = findDesignInKit(kit, designGuid);
  return (design.pieces || []).filter((piece) => {
    if (!piece.design?.guid) return false;
    return areDesignsInSameFamily(kit, designGuid, piece.design.guid);
  });
};

// #endregion 🔖Design Family Helpers

// #region 🔖Type Family Helpers

// [👤semio📚js💻semio🔖kit🔖typefamilyhelpers](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Type%20Family%20Helpers)
// Type family traversal helpers MUST be defined here.

/**
 * Retrieves the PrimitiveType value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖kit🔖typefamilyhelpers🪨getprimitivetype](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Type%20Family%20Helpers/d/i/getPrimitiveType)
 **/
export const getPrimitiveType = (kit: Kit, typeGuid: string): Type => {
  let current = findTypeInKit(kit, typeGuid);
  while (current.parent?.guid) {
    current = findTypeInKit(kit, current.parent.guid);
  }
  return current;
};

/**
 * Retrieves the TypeFamily value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖kit🔖typefamilyhelpers🪨gettypefamily](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Type%20Family%20Helpers/d/i/getTypeFamily)
 **/
export const getTypeFamily = (kit: Kit, typeGuid: string): Type[] => {
  const primitive = getPrimitiveType(kit, typeGuid);
  const family: Type[] = [];
  const collectDescendants = (parentGuid: string) => {
    const parent = findTypeInKit(kit, parentGuid);
    family.push(parent);
    const children = (kit.types || []).filter((t) => t.parent?.guid === parentGuid);
    children.forEach((child) => collectDescendants(child.guid));
  };
  collectDescendants(primitive.guid);
  return family;
};

/**
 * Retrieves the TypeSiblings value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖kit🔖typefamilyhelpers🪨gettypesiblings](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Type%20Family%20Helpers/d/i/getTypeSiblings)
 **/
export const getTypeSiblings = (kit: Kit, typeGuid: string): Type[] => {
  const type = findTypeInKit(kit, typeGuid);
  const parentGuid = type.parent?.guid;
  return (kit.types || []).filter((t) => t.parent?.guid === parentGuid && t.guid !== typeGuid);
};

/**
 * Retrieves the TypeChildren value.
 * MUST return the requested value.
 * [👤semio📚js💻semio🔖kit🔖typefamilyhelpers🪨gettypechildren](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Type%20Family%20Helpers/d/i/getTypeChildren)
 **/
export const getTypeChildren = (kit: Kit, typeGuid: string): Type[] => {
  return (kit.types || []).filter((t) => t.parent?.guid === typeGuid);
};

/**
 * Checks if Types belong to the same family.
 * MUST return a boolean result.
 * [👤semio📚js💻semio🔖kit🔖typefamilyhelpers🪨aretypesinsamefamily](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Type%20Family%20Helpers/d/i/areTypesInSameFamily)
 **/
export const areTypesInSameFamily = (kit: Kit, typeGuidA: string, typeGuidB: string): boolean => {
  const primitiveA = getPrimitiveType(kit, typeGuidA);
  const primitiveB = getPrimitiveType(kit, typeGuidB);
  return primitiveA.guid === primitiveB.guid;
};

// #endregion 🔖Type Family Helpers

/**
 * Searches for matching PortInKit entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findportinkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findPortInKit)
 **/
export const findPortInKit = (kit: Kit, portGuid: string): Port => {
  const iface = kit.ports?.find((i) => i.guid === portGuid);
  if (!iface) throw new Error(`Port ${portGuid} not found in kit ${kit.name}`);
  return iface;
};

/**
 * Searches for matching PieceTypeInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findpiecetypeindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findPieceTypeInDesign)
 **/
export const findPieceTypeInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Type => {
  const piece = findPieceInDesign(findDesignInKit(kit, designGuid), pieceGuid);
  if (!piece.type) throw new Error(`Piece ${pieceGuid} has no type`);
  return findTypeInKit(kit, piece.type.guid);
};

/**
 * Searches for matching ParentPieceInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findparentpieceindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findParentPieceInDesign)
 **/
export const findParentPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Piece => {
  const parentPieceId = piecesMetadata(kit, designGuid).get(pieceGuid)?.parentPieceId;
  if (!parentPieceId) throw new Error(`Piece ${pieceGuid} has no parent piece`);
  return findPieceInDesign(findDesignInKit(kit, designGuid), parentPieceId);
};

/**
 * Searches for matching ParentConnectionForPieceInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findparentconnectionforpieceindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findParentConnectionForPieceInDesign)
 **/
export const findParentConnectionForPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Connection => {
  const parentPieceId = piecesMetadata(kit, designGuid).get(pieceGuid)?.parentPieceId;
  if (!parentPieceId) throw new Error(`Piece ${pieceGuid} has no parent piece and connection`);
  return findConnectionInDesign(findDesignInKit(kit, designGuid), parentPieceId);
};

/**
 * Searches for matching ChildrenPiecesInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findchildrenpiecesindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findChildrenPiecesInDesign)
 **/
export const findChildrenPiecesInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Piece[] => {
  const design = findDesignInKit(kit, designGuid);
  const metadata = piecesMetadata(kit, designGuid);
  const children: Piece[] = [];
  for (const [id, data] of Array.from(metadata)) {
    if (data.parentPieceId === pieceGuid) {
      children.push(findPieceInDesign(design, id));
    }
  }
  return children;
};

/**
 * Searches for matching UsedConnectorsByPieceInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findusedconnectorsbypieceindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findUsedConnectorsByPieceInDesign)
 **/
export const findUsedConnectorsByPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Connector[] => {
  const design = findDesignInKit(kit, designGuid);
  const piece = findPieceInDesign(design, pieceGuid);
  if (!piece.type) return [];
  const type = findTypeInKit(kit, piece.type.guid);
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  return connections.map((c) => findConnectorForPieceInConnection(type, c, pieceGuid)).filter((p): p is Connector => p !== undefined);
};

/**
 * Searches for matching ReplacableTypesForPieceInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findreplacabletypesforpieceindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findReplacableTypesForPieceInDesign)
 **/
export const findReplacableTypesForPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string, variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designGuid);
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  const requiredConnectors: Connector[] = [];
  for (const connection of connections) {
    try {
      const otherPieceId = connection.connected.piece.guid === pieceGuid ? connection.connecting.piece.guid : connection.connected.piece.guid;
      const otherPiece = findPieceInDesign(design, otherPieceId);
      if (!otherPiece.type) continue;
      const otherType = findTypeInKit(kit, otherPiece.type.guid);
      const otherPortId = connection.connected.piece.guid === pieceGuid ? connection.connecting.connector?.guid : connection.connected.connector?.guid;
      const otherPort = findConnectorInType(otherType, otherPortId || "");
      requiredConnectors.push(otherPort);
    } catch (error) {
      continue;
    }
  }
  return (
    kit.types?.filter((replacementType) => {
      if (replacementType.isAbstract) return false;
      if (variants !== undefined && !variants.includes(replacementType.parent?.guid ?? "")) return false;
      if (!replacementType.connectors || replacementType.connectors.length === 0) return requiredConnectors.length === 0;
      return requiredConnectors.every((requiredConnector) => {
        return replacementType.connectors!.some((replacementConnector) => areConnectorsCompatible(replacementConnector, requiredConnector));
      });
    }) ?? []
  );
};

/**
 * Searches for matching ReplacableTypesForPiecesInDesign entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findreplacabletypesforpiecesindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findReplacableTypesForPiecesInDesign)
 **/
export const findReplacableTypesForPiecesInDesign = (kit: Kit, designGuid: string, pieceGuids: string[], variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designGuid);
  const pieces = pieceGuids.map((id) => findPieceInDesign(design, id));
  const externalConnections: Array<{
    connection: Connection;
    requiredConnector: Connector;
  }> = [];
  for (const piece of pieces) {
    const connections = findPieceConnectionsInDesign(design, piece.guid);
    for (const connection of connections) {
      const otherPieceId = connection.connected.piece.guid === piece.guid ? connection.connecting.piece.guid : connection.connected.piece.guid;
      if (!pieceGuids.includes(otherPieceId)) {
        try {
          const otherPiece = findPieceInDesign(design, otherPieceId);
          if (!otherPiece.type) continue;
          const otherType = findTypeInKit(kit, otherPiece.type.guid);
          const otherPortId = connection.connected.piece.guid === piece.guid ? connection.connecting.connector?.guid : connection.connected.connector?.guid;
          const otherPort = findConnectorInType(otherType, otherPortId || "");
          externalConnections.push({ connection, requiredConnector: otherPort });
        } catch (error) {
          continue;
        }
      }
    }
  }
  return (
    kit.types?.filter((replacementType) => {
      if (replacementType.isAbstract) return false;
      if (variants !== undefined && !variants.includes(replacementType.parent?.guid ?? "")) return false;
      if (!replacementType.connectors || replacementType.connectors.length === 0) return externalConnections.length === 0;
      return externalConnections.every(({ requiredConnector }) => {
        return replacementType.connectors!.some((replacementConnector) => areConnectorsCompatible(replacementConnector, requiredConnector));
      });
    }) ?? []
  );
};

/**
 * Sums the values of a quality across all pieces in a design.
 * For each piece, checks piece-level props first, then falls back to type-level props.
 * [👤semio📚js💻semio🔖kit🪨sumqualityindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/sumQualityInDesign)
 **/
export const sumQualityInDesign = (kit: Kit, designGuid: string, qualityGuid: string): number => {
  const design = findDesignInKit(kit, designGuid);
  let sum = 0;
  for (const piece of design.pieces ?? []) {
    const pieceProp = piece.props?.find((p) => p.quality?.guid === qualityGuid);
    if (pieceProp) {
      const val = parseFloat(pieceProp.value);
      if (!isNaN(val)) sum += val;
      continue;
    }
    if (piece.type) {
      const type = kit.types?.find((t) => t.guid === piece.type!.guid);
      if (type) {
        const typeProp = type.props?.find((p) => p.quality?.guid === qualityGuid);
        if (typeProp) {
          const val = parseFloat(typeProp.value);
          if (!isNaN(val)) sum += val;
        }
      }
    }
  }
  return sum;
};

/**
 * Definition of piecesMetadata.
 * [👤semio📚js💻semio🔖kit🪨piecesmetadata](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/piecesMetadata)
 **/
export const piecesMetadata = (
  kit: Kit,
  designGuid: string,
): Map<
  string,
  {
    plane: Plane;
    center: Coord;
    fixedPieceId: string;
    parentPieceId: string | null;
    depth: number;
  }
> => {
  const design = findDesignInKit(kit, designGuid);
  if (!design) {
    throw new Error(`Design ${designGuid} not found in kit ${kit.name}`);
  }
  const flattenChange = flattenDesign(kit, designGuid);
  const flatDesign = applyDesignDiff(design, flattenChange.forward);
  const fixedPieceIds = flatDesign.pieces?.map((p) => findAttributeValue(p, "semio.fixedPieceId", p.guid) || p.guid);
  const parentPieceIds = flatDesign.pieces?.map((p) => findAttributeValue(p, "semio.parentPieceId", null));
  const depths = flatDesign.pieces?.map((p) => parseInt(findAttributeValue(p, "semio.depth", "0")!));
  return new Map(
    flatDesign.pieces?.map((p, index) => [
      p.guid,
      {
        plane: p.plane!,
        center: p.center!,
        fixedPieceId: fixedPieceIds![index],
        parentPieceId: parentPieceIds![index],
        depth: depths![index],
      },
    ]),
  );
};

/**
 * Searches for matching AttributeValue entry.
 * MUST return the matching element or undefined.
 * [👤semio📚js💻semio🔖kit🪨findattributevalue](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/findAttributeValue)
 **/
export const findAttributeValue = (entity: Kit | Type | Design | Piece | Connection | Model | Connector, name: string, defaultValue?: string | null): string | null => {
  const attribute = entity.attributes?.find((q) => q.key === name);
  if (!attribute && defaultValue === undefined) throw new Error(`Attribute ${name} not found in ${entity}`);
  if (attribute?.value === undefined && defaultValue === null) return null;
  return attribute?.value ?? defaultValue ?? "";
};

// getColorForText holds the data fields for a getColorForText record.
// [👤semio📚js💻semio🔖kit🪨getcolorfortext](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/getColorForText)
const getColorForText = (text?: string): string => {
  if (!text || text === "") return "var(--foreground)";

  let hash = 0;
  for (let i = 0; i < text.length; i++) {
    const char = text.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash;
  }

  const baseColors = [
    {
      base: "var(--accent)",
      variations: [
        "color-mix(in srgb, var(--accent) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--accent) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--accent) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--accent) 45%, var(--foreground) 55%)",
      ],
    },
    {
      base: "var(--accent-secondary)",
      variations: [
        "color-mix(in srgb, var(--accent-secondary) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--accent-secondary) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--accent-secondary) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--accent-secondary) 45%, var(--foreground) 55%)",
      ],
    },
    {
      base: "var(--accent-tertiary)",
      variations: [
        "color-mix(in srgb, var(--accent-tertiary) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--accent-tertiary) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--accent-tertiary) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--accent-tertiary) 45%, var(--foreground) 55%)",
      ],
    },
    {
      base: "var(--status-success)",
      variations: [
        "color-mix(in srgb, var(--status-success) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--status-success) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--status-success) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--status-success) 45%, var(--foreground) 55%)",
      ],
    },
    {
      base: "var(--status-warning)",
      variations: [
        "color-mix(in srgb, var(--status-warning) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--status-warning) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--status-warning) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--status-warning) 45%, var(--foreground) 55%)",
      ],
    },
    {
      base: "var(--status-info)",
      variations: [
        "color-mix(in srgb, var(--status-info) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--status-info) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--status-info) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--status-info) 45%, var(--foreground) 55%)",
      ],
    },
  ];

  const colorSetIndex = Math.abs(hash) % baseColors.length;
  const variationIndex = Math.abs(Math.floor(hash / baseColors.length)) % baseColors[colorSetIndex].variations.length;

  return baseColors[colorSetIndex].variations[variationIndex];
};

/**
 * Assigns colors to PortsForTypes elements.
 * MUST assign colors deterministically.
 * [👤semio📚js💻semio🔖kit🪨colorportsfortypes](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/colorPortsForTypes)
 **/
export const colorPortsForTypes = (types: Type[]): TypesDiff => {
  const updated: { type: TypeId; diff: TypeDiff }[] = [];

  for (const type of types) {
    const updatedConnectors = (type.connectors || []).map((connector) => ({
      ...connector,
      attributes: [
        ...(connector.attributes || []),
        {
          guid: guid(),
          key: "semio.color",
          value: getColorForText(connector.port?.guid),
        },
      ],
    }));

    updated.push({
      type: { guid: type.guid },
      diff: {
        connectors: { added: updatedConnectors },
      },
    });
  }

  return { updated };
};

/**
 * Parses DesignIdFromVariant from serialized input.
 * MUST produce a valid in-memory representation.
 * [👤semio📚js💻semio🔖kit🪨parsedesignidfromvariant](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/parseDesignIdFromVariant)
 **/
export const parseDesignIdFromVariant = (variant: string): string => {
  return variant.split("-")[0];
};

// #region 🔖File Tree Utilities

// [👤semio📚js💻semio🔖kit🔖filetreeutilities](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/File%20Tree%20Utilities)
// File tree construction and traversal utilities MUST be defined here.

/**
 * Interface defining FileTreeNode structure.
 * [👤semio📚js💻semio🔖kit🔖filetreeutilities🛠️filetreenode](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/File%20Tree%20Utilities/d/i/FileTreeNode)
 **/
export interface FileTreeNode {
  name: string;
  path: string;
  isDirectory: boolean;
  children: FileTreeNode[];
  file?: File;
  folderGuid?: string;
  parentPath?: string;
}

/**
 * Constructs FileTree from components.
 * MUST construct and return a complete structure.
 * [👤semio📚js💻semio🔖kit🔖filetreeutilities🪨buildfiletree](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/File%20Tree%20Utilities/d/i/buildFileTree)
 **/
export const buildFileTree = (folders: Folder[], files: File[]): FileTreeNode[] => {
  const folderChildren = new Map<string | undefined, Folder[]>();
  folders.forEach((folder) => {
    const parent = folder.parent?.guid;
    if (!folderChildren.has(parent)) folderChildren.set(parent, []);
    folderChildren.get(parent)!.push(folder);
  });

  const filesByFolder = new Map<string | undefined, File[]>();
  files.forEach((file) => {
    const folder = file.folder?.guid;
    if (!filesByFolder.has(folder)) filesByFolder.set(folder, []);
    filesByFolder.get(folder)!.push(file);
  });

  const sortFolders = (items?: Folder[]): Folder[] => {
    return (items || []).slice().sort((a, b) => a.name.localeCompare(b.name));
  };

  const sortFiles = (items?: File[]): File[] => {
    return (items || []).slice().sort((a, b) => a.name.localeCompare(b.name));
  };

  const buildNodes = (parentGuid?: string, parentPath?: string): FileTreeNode[] => {
    const children: FileTreeNode[] = [];
    const childFolders = sortFolders(folderChildren.get(parentGuid));
    childFolders.forEach((folder) => {
      const nodePath = folder.guid;
      children.push({
        name: folder.name,
        path: nodePath,
        parentPath,
        isDirectory: true,
        folderGuid: folder.guid,
        children: buildNodes(folder.guid, nodePath),
      });
    });
    const childFiles = sortFiles(filesByFolder.get(parentGuid));
    childFiles.forEach((file) => {
      children.push({
        name: file.name,
        path: file.guid,
        parentPath,
        isDirectory: false,
        children: [],
        file,
      });
    });
    return children;
  };

  return buildNodes(undefined, undefined);
};

/**
 * Flattens nested FileTree structure.
 * MUST return a flat array.
 * [👤semio📚js💻semio🔖kit🔖filetreeutilities🪨flattenfiletree](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/File%20Tree%20Utilities/d/i/flattenFileTree)
 **/
export const flattenFileTree = (nodes: FileTreeNode[], level: number = 0, expandedPaths: Set<string> = new Set()): Array<FileTreeNode & { level: number; isExpanded: boolean }> => {
  const result: Array<FileTreeNode & { level: number; isExpanded: boolean }> = [];

  nodes.forEach((node) => {
    const isExpanded = expandedPaths.has(`file-${node.path}`);
    result.push({ ...node, level, isExpanded });

    if (node.isDirectory && isExpanded && node.children.length > 0) {
      result.push(...flattenFileTree(node.children, level + 1, expandedPaths));
    }
  });

  return result;
};

// #endregion 🔖File Tree Utilities

/**
 * Performs the createFileFromDataUri operation.
 * MUST return a new valid instance.
 * [👤semio📚js💻semio🔖kit🪨createfilefromdatauri](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/d/i/createFileFromDataUri)
 **/
export const createFileFromDataUri = (name: string, dataUri: string): File => {
  const sizeMatch = dataUri.match(/data:([^;]+)(;base64)?,(.+)/);
  let size = 0;
  if (sizeMatch) {
    const data = sizeMatch[3];
    if (sizeMatch[2] === ";base64") {
      size = Math.floor(data.length * 0.75);
    } else {
      size = data.length;
    }
  }

  let hash = 0;
  for (let i = 0; i < dataUri.length; i++) {
    const char = dataUri.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash;
  }

  return {
    guid: guid(),
    name,
    size,
    hash: hash.toString(36),
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };
};

// #region 🔖Kit Import/Export

// [👤semio📚js💻semio🔖kit🔖kitimport🔖export](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export)
// Kit serialization and deserialization functions MUST be defined here.

/**
 * Interface defining KitImportResult structure.
 * [👤semio📚js💻semio🔖kit🔖kitimport🔖export🛠️kitimportresult](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/KitImportResult)
 **/
export interface KitImportResult {
  kit: Kit;
}

// [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨cachedsqljs](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/cachedSqlJs)
// cachedSqlJs holds the data fields for a cachedSqlJs record.
let cachedSqlJs: any = null;
// [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨getsqljs](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/getSqlJs)
// getSqlJs holds the data fields for a getSqlJs record.
export const getSqlJs = async () => {
  if (!cachedSqlJs) {
    const initSqlJs = (await import("sql.js")).default;
    try {
      const isNode = typeof process !== "undefined" && process.versions?.node;
      if (isNode) {
        const fs = await import("node:fs");
        const path = await import("path");
        const url = await import("url");
        const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
        const candidatePaths = [path.join(__dirname, "public", "sql-wasm.wasm"), path.join(__dirname, "..", "sketchpad", "public", "sql-wasm.wasm")];
        const wasmPath = candidatePaths.find((candidate) => fs.existsSync(candidate)) ?? candidatePaths[0];
        cachedSqlJs = await initSqlJs({
          locateFile: () => wasmPath,
        });
      } else {
        cachedSqlJs = await initSqlJs({
          locateFile: (file: string) => `/${file}`,
        });
      }
    } catch (error) {
      console.error("Failed to initialize sql.js:", error);
      throw new Error("Failed to load SQLite database library.");
    }
  }
  return cachedSqlJs;
};

// [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨buildfolderpath](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/buildFolderPath)
// buildFolderPath builds a slash-separated folder path from root to the given folder guid.
// Uses proper mime type inferred from file extension.
const buildFolderPath = (kit: Kit, folderGuid: string): string => {
  const findFolder = (guid: string): Folder | undefined => (kit.folders || []).find((f) => f.guid === guid);
  const parts: string[] = [];
  let current = findFolder(folderGuid);
  while (current) {
    parts.unshift(current.name);
    current = current.parent?.guid ? findFolder(current.parent.guid) : undefined;
  }
  return parts.join("/");
};

// [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨buildfilepath](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/buildFilePath)
// buildFilePath builds the full path of a kit file including its folder hierarchy.
// Uses proper mime type inferred from file extension.
const buildFilePath = (kit: Kit, file: File): string => {
  if (file.folder?.guid) {
    const folderPath = buildFolderPath(kit, file.folder.guid);
    if (folderPath) return `${folderPath}/${file.name}`;
  }
  return file.name;
};

/**
 * Imports Kit from external source.
 * MUST load and return the imported data.
 * [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨importkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/importKit)
 **/
export const importKit = async (source: string | ArrayBuffer | Buffer | Blob): Promise<KitImportResult> => {
  const JSZip = (await import("jszip")).default;

  let arrayBuffer: ArrayBuffer;
  if (source instanceof Blob) {
    arrayBuffer = await source.arrayBuffer();
  } else if (typeof source === "string") {
    if (source.startsWith("blob:")) {
      const response = await fetch(source);
      if (!response.ok) {
        throw new Error(`Failed to fetch kit from blob URL: ${response.statusText}`);
      }
      arrayBuffer = await response.arrayBuffer();
    } else {
      const response = await fetch(source);
      if (!response.ok) {
        throw new Error(`Failed to fetch kit from ${source}: ${response.statusText}`);
      }
      arrayBuffer = await response.arrayBuffer();
    }
  } else if (source instanceof Buffer) {
    arrayBuffer = source.buffer.slice(source.byteOffset, source.byteOffset + source.byteLength) as ArrayBuffer;
  } else {
    arrayBuffer = source as ArrayBuffer;
  }

  const zip = await JSZip.loadAsync(arrayBuffer);

  const dbFile = zip.file(".semio/kit.db");
  let kit: Kit;
  if (dbFile) {
    const dbArrayBuffer = await dbFile.async("arraybuffer");
    const SQL = await getSqlJs();
    const db = new SQL.Database(new Uint8Array(dbArrayBuffer));
    kit = await sqliteToKit(db);
    db.close();
  } else {
    const kitJsonFile = zip.file("kit.json");
    if (!kitJsonFile) {
      throw new Error("Invalid kit archive: missing .semio/kit.db or kit.json");
    }
    const kitJson = await kitJsonFile.async("string");
    kit = deserializeKit(kitJson);
  }

  const zipEntries = new Map<string, any>();
  for (const [path, zipEntry] of Object.entries(zip.files)) {
    if (!(zipEntry as any).dir && !path.startsWith(".semio/") && path !== "kit.json") {
      zipEntries.set(path, zipEntry);
    }
  }

  if (kit.files) {
    for (const file of kit.files) {
      const filePath = buildFilePath(kit, file);
      const zipEntry = zipEntries.get(filePath);
      if (zipEntry) {
        const arrayBuf = await (zipEntry as any).async("arraybuffer");
        const bytes = new Uint8Array(arrayBuf);
        let binary = "";
        for (let i = 0; i < bytes.length; i++) binary += String.fromCharCode(bytes[i]);
        const ext = file.name.split(".").pop()?.toLowerCase() || "";
        const mimeMap: Record<string, string> = {
          stl: "model/stl",
          obj: "model/obj",
          glb: "model/gltf-binary",
          gltf: "model/gltf+json",
          "3dm": "model/vnd.3dm",
          png: "image/png",
          jpg: "image/jpeg",
          jpeg: "image/jpeg",
          svg: "image/svg+xml",
          pdf: "application/pdf",
          zip: "application/zip",
          json: "application/json",
          csv: "text/csv",
          txt: "text/plain",
        };
        const mime = mimeMap[ext] || "application/octet-stream";
        file.blob = `data:${mime};base64,${btoa(binary)}`;
      }
    }
  }

  return { kit };
};

/**
 * Exports Kit to external format.
 * MUST produce the exported format.
 * [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨exportkit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/exportKit)
 **/
export const exportKit = async (kit: Kit): Promise<Blob> => {
  const JSZip = (await import("jszip")).default;

  const SQL = await getSqlJs();
  const db = new SQL.Database();

  await kitToSqlite(kit, db);

  const dbData = db.export();
  db.close();

  const zip = new JSZip();
  zip.file(".semio/kit.db", dbData);

  if (kit.files) {
    for (const file of kit.files) {
      if (file.blob) {
        const filePath = buildFilePath(kit, file);
        const base64 = file.blob.startsWith("data:") ? file.blob.slice(file.blob.indexOf(",") + 1) : file.blob;
        const binary = atob(base64);
        const bytes = new Uint8Array(binary.length);
        for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
        zip.file(filePath, bytes);
      }
    }
  }

  return await zip.generateAsync({ type: "blob" });
};

/**
 * Deep equality check for Kits entities.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨arekitsequal](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/areKitsEqual)
 **/
export const areKitsEqual = (a: Kit, b: Kit): boolean => {
  const normalizeArray = <T>(arr: T[] | T | undefined | null): T[] => {
    if (!arr) return [];
    if (Array.isArray(arr)) return arr;
    return [arr as T];
  };
  const normalizeValue = (value: any): any => (value === null || value === "" || value === undefined ? undefined : value);
  const normalizeBoolean = (value: boolean | undefined): boolean | undefined => (value ? true : undefined);
  const floatEq = (a: number | undefined, b: number | undefined): boolean => {
    if (a === undefined && b === undefined) return true;
    if (a === undefined || b === undefined) return false;
    return Math.abs(a - b) < TOLERANCE;
  };

  const areAttributesEqual = (a?: Attribute[], b?: Attribute[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const attrA of arrA) {
      const attrB = arrB.find((x) => x.guid === attrA.guid);
      if (!attrB) return false;
      if (attrA.key !== attrB.key) return false;
      if (normalizeValue(attrA.value) !== normalizeValue(attrB.value)) return false;
      if (normalizeValue(attrA.definition) !== normalizeValue(attrB.definition)) return false;
    }
    return true;
  };

  const arePropsEqual = (a?: Prop[], b?: Prop[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const propA of arrA) {
      const propB = arrB.find((x) => x.guid === propA.guid);
      if (!propB) return false;
      if (propA.quality.guid !== propB.quality.guid) return false;
      if (propA.value !== propB.value) return false;
      if (normalizeValue(propA.unit) !== normalizeValue(propB.unit)) return false;
      if (!areAttributesEqual(propA.attributes, propB.attributes)) return false;
    }
    return true;
  };

  const areConnectorsEqual = (a?: Connector[], b?: Connector[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const connectorA of arrA) {
      const connectorB = arrB.find((x) => x.guid === connectorA.guid);
      if (!connectorB) return false;
      if (normalizeValue(connectorA.name) !== normalizeValue(connectorB.name)) return false;
      if (!floatEq(connectorA.point.x, connectorB.point.x)) return false;
      if (!floatEq(connectorA.point.y, connectorB.point.y)) return false;
      if (!floatEq(connectorA.point.z, connectorB.point.z)) return false;
      if (!floatEq(connectorA.direction.x, connectorB.direction.x)) return false;
      if (!floatEq(connectorA.direction.y, connectorB.direction.y)) return false;
      if (!floatEq(connectorA.direction.z, connectorB.direction.z)) return false;
      if (!floatEq(connectorA.t, connectorB.t)) return false;
      if (normalizeBoolean(connectorA.mandatory) !== normalizeBoolean(connectorB.mandatory)) return false;
      if (normalizeValue(connectorA.port?.guid) !== normalizeValue(connectorB.port?.guid)) return false;
      if (!arePropsEqual(connectorA.props, connectorB.props)) return false;
      if (!areAttributesEqual(connectorA.attributes, connectorB.attributes)) return false;
    }
    return true;
  };

  const areModelsEqual = (a?: Model[], b?: Model[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const modelA of arrA) {
      const modelB = arrB.find((x) => x.guid === modelA.guid);
      if (!modelB) return false;
      if (normalizeValue(modelA.name) !== normalizeValue(modelB.name)) return false;
      if (modelA.file.guid !== modelB.file.guid) return false;

      const tagsA = normalizeArray(modelA.tags).map((t) => (typeof t === "object" ? t.guid : t));
      const tagsB = normalizeArray(modelB.tags).map((t) => (typeof t === "object" ? t.guid : t));
      if (tagsA.length !== tagsB.length || !tagsA.every((g) => tagsB.includes(g))) return false;
      if (!areAttributesEqual(modelA.attributes, modelB.attributes)) return false;
    }
    return true;
  };

  const areTypesEqual = (a?: Type[], b?: Type[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const typeA of arrA) {
      const typeB = arrB.find((t) => {
        if (t.guid !== typeA.guid) return false;
        if (!t.parent && !typeA.parent) return true;
        if (!t.parent || !typeA.parent) return false;
        return areSameTypeId(t.parent, typeA.parent);
      });
      if (!typeB) return false;
      if (typeA.name !== typeB.name) return false;
      if (normalizeValue(typeA.description) !== normalizeValue(typeB.description)) return false;
      if (normalizeValue(typeA.icon) !== normalizeValue(typeB.icon)) return false;
      if (normalizeValue(typeA.image) !== normalizeValue(typeB.image)) return false;
      if (normalizeValue(typeA.folder) !== normalizeValue(typeB.folder)) return false;
      if (normalizeValue(typeA.unit) !== normalizeValue(typeB.unit)) return false;
      if (typeA.stock !== typeB.stock) return false;
      if (normalizeBoolean(typeA.isAbstract) !== normalizeBoolean(typeB.isAbstract)) return false;
      if (normalizeBoolean(typeA.virtual) !== normalizeBoolean(typeB.virtual)) return false;
      if (normalizeValue(typeA.location?.guid) !== normalizeValue(typeB.location?.guid)) return false;
      if (!arraysEqual(normalizeArray(typeA.concepts), normalizeArray(typeB.concepts))) return false;
      if (!arraysEqual(normalizeArray(typeA.authors?.map((a) => a.guid)), normalizeArray(typeB.authors?.map((a) => a.guid)))) return false;
      if (!arePropsEqual(typeA.props, typeB.props)) return false;
      if (!areModelsEqual(typeA.models, typeB.models)) return false;
      if (!areConnectorsEqual(typeA.connectors, typeB.connectors)) return false;
      if (!areAttributesEqual(typeA.attributes, typeB.attributes)) return false;
    }
    return true;
  };

  const arePiecesEqual = (a?: Piece[], b?: Piece[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const pieceA of arrA) {
      const pieceB = arrB.find((x) => x.guid === pieceA.guid);
      if (!pieceB) return false;
      if (normalizeValue(pieceA.name) !== normalizeValue(pieceB.name)) return false;
      if (pieceA.type?.guid !== pieceB.type?.guid) return false;
      if (pieceA.design?.guid !== pieceB.design?.guid) return false;
      if (pieceA.plane && pieceB.plane) {
        if (!floatEq(pieceA.plane.origin.x, pieceB.plane.origin.x)) return false;
        if (!floatEq(pieceA.plane.origin.y, pieceB.plane.origin.y)) return false;
        if (!floatEq(pieceA.plane.origin.z, pieceB.plane.origin.z)) return false;
        if (!floatEq(pieceA.plane.xAxis.x, pieceB.plane.xAxis.x)) return false;
        if (!floatEq(pieceA.plane.xAxis.y, pieceB.plane.xAxis.y)) return false;
        if (!floatEq(pieceA.plane.xAxis.z, pieceB.plane.xAxis.z)) return false;
        if (!floatEq(pieceA.plane.yAxis.x, pieceB.plane.yAxis.x)) return false;
        if (!floatEq(pieceA.plane.yAxis.y, pieceB.plane.yAxis.y)) return false;
        if (!floatEq(pieceA.plane.yAxis.z, pieceB.plane.yAxis.z)) return false;
      } else if (pieceA.plane || pieceB.plane) {
        return false;
      }
      if (pieceA.center && pieceB.center) {
        if (!floatEq(pieceA.center.u, pieceB.center.u)) return false;
        if (!floatEq(pieceA.center.v, pieceB.center.v)) return false;
      } else if (pieceA.center || pieceB.center) {
        return false;
      }
      if (!floatEq(pieceA.scale, pieceB.scale)) return false;
      if (pieceA.mirrorPlane && pieceB.mirrorPlane) {
        if (!floatEq(pieceA.mirrorPlane.origin.x, pieceB.mirrorPlane.origin.x)) return false;
        if (!floatEq(pieceA.mirrorPlane.origin.y, pieceB.mirrorPlane.origin.y)) return false;
        if (!floatEq(pieceA.mirrorPlane.origin.z, pieceB.mirrorPlane.origin.z)) return false;
        if (!floatEq(pieceA.mirrorPlane.xAxis.x, pieceB.mirrorPlane.xAxis.x)) return false;
        if (!floatEq(pieceA.mirrorPlane.xAxis.y, pieceB.mirrorPlane.xAxis.y)) return false;
        if (!floatEq(pieceA.mirrorPlane.xAxis.z, pieceB.mirrorPlane.xAxis.z)) return false;
        if (!floatEq(pieceA.mirrorPlane.yAxis.x, pieceB.mirrorPlane.yAxis.x)) return false;
        if (!floatEq(pieceA.mirrorPlane.yAxis.y, pieceB.mirrorPlane.yAxis.y)) return false;
        if (!floatEq(pieceA.mirrorPlane.yAxis.z, pieceB.mirrorPlane.yAxis.z)) return false;
      } else if (pieceA.mirrorPlane || pieceB.mirrorPlane) {
        return false;
      }
      if (normalizeBoolean(pieceA.isHidden) !== normalizeBoolean(pieceB.isHidden)) return false;
      if (normalizeBoolean(pieceA.isLocked) !== normalizeBoolean(pieceB.isLocked)) return false;
      if (normalizeValue(pieceA.color) !== normalizeValue(pieceB.color)) return false;
      if (normalizeValue(pieceA.description) !== normalizeValue(pieceB.description)) return false;
      if (!arePropsEqual(pieceA.props, pieceB.props)) return false;
      if (!areAttributesEqual(pieceA.attributes, pieceB.attributes)) return false;
    }
    return true;
  };

  const areConnectionsEqual = (a?: Connection[], b?: Connection[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const connA of arrA) {
      const connB = arrB.find((x) => x.guid === connA.guid);
      if (!connB) return false;
      if (connA.connected.piece.guid !== connB.connected.piece.guid) return false;
      if (normalizeValue(connA.connected.designPiece?.guid) !== normalizeValue(connB.connected.designPiece?.guid)) return false;
      if (normalizeValue(connA.connected.connector?.guid) !== normalizeValue(connB.connected.connector?.guid)) return false;
      if (connA.connecting.piece.guid !== connB.connecting.piece.guid) return false;
      if (normalizeValue(connA.connecting.designPiece?.guid) !== normalizeValue(connB.connecting.designPiece?.guid)) return false;
      if (normalizeValue(connA.connecting.connector?.guid) !== normalizeValue(connB.connecting.connector?.guid)) return false;
      if (!floatEq(connA.gap, connB.gap)) return false;
      if (!floatEq(connA.shift, connB.shift)) return false;
      if (!floatEq(connA.rise, connB.rise)) return false;
      if (!floatEq(connA.rotation, connB.rotation)) return false;
      if (!floatEq(connA.turn, connB.turn)) return false;
      if (!floatEq(connA.tilt, connB.tilt)) return false;
      if (!floatEq(connA.u, connB.u)) return false;
      if (!floatEq(connA.v, connB.v)) return false;
      if (normalizeValue(connA.description) !== normalizeValue(connB.description)) return false;
      if (!areAttributesEqual(connA.attributes, connB.attributes)) return false;
    }
    return true;
  };

  const areDesignsEqual = (a?: Design[], b?: Design[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const designA of arrA) {
      const designB = arrB.find((d) => {
        if (d.guid !== designA.guid) return false;
        if (!d.parent && !designA.parent) return true;
        if (!d.parent || !designA.parent) return false;
        return areSameDesignId(d.parent, designA.parent);
      });
      if (!designB) return false;
      if (designA.name !== designB.name) return false;
      if (normalizeValue(designA.description) !== normalizeValue(designB.description)) return false;
      if (normalizeValue(designA.icon) !== normalizeValue(designB.icon)) return false;
      if (normalizeValue(designA.image) !== normalizeValue(designB.image)) return false;
      if (!arraysEqual(normalizeArray(designA.concepts), normalizeArray(designB.concepts))) return false;
      if (!arePiecesEqual(designA.pieces, designB.pieces)) return false;
      if (!areConnectionsEqual(designA.connections, designB.connections)) return false;
      if (!areAttributesEqual(designA.attributes, designB.attributes)) return false;
    }
    return true;
  };

  const arePortsEqual = (a?: Port[], b?: Port[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const ifaceA of arrA) {
      const ifaceB = arrB.find((x) => x.guid === ifaceA.guid);
      if (!ifaceB) return false;
      if (ifaceA.name !== ifaceB.name) return false;
      if (normalizeValue(ifaceA.description) !== normalizeValue(ifaceB.description)) return false;
      if (!areAttributesEqual(ifaceA.attributes, ifaceB.attributes)) return false;
    }
    return true;
  };

  const areQualitiesEqual = (a?: Quality[], b?: Quality[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const qualA of arrA) {
      const qualB = arrB.find((x) => x.guid === qualA.guid);
      if (!qualB) return false;
      if (qualA.key !== qualB.key) return false;
      if (qualA.name !== qualB.name) return false;
      if (!areAttributesEqual(qualA.attributes, qualB.attributes)) return false;
    }
    return true;
  };

  const areFilesEqual = (a?: File[], b?: File[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const fileA of arrA) {
      const fileB = arrB.find((x) => x.guid === fileA.guid);
      if (!fileB) return false;
      if (fileA.name !== fileB.name) return false;
    }
    return true;
  };

  const areFoldersEqual = (a?: Folder[], b?: Folder[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const folderA of arrA) {
      const folderB = arrB.find((x) => x.guid === folderA.guid);
      if (!folderB) return false;
      if (folderA.name !== folderB.name) return false;
      if (!areAttributesEqual(folderA.attributes, folderB.attributes)) return false;
    }
    return true;
  };

  const areAuthorsEqual = (a?: Author[], b?: Author[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const authorA of arrA) {
      const authorB = arrB.find((x) => x.guid === authorA.guid);
      if (!authorB) return false;
      if (authorA.name !== authorB.name) return false;
      if (normalizeValue(authorA.email) !== normalizeValue(authorB.email)) return false;
      if (!areAttributesEqual(authorA.attributes, authorB.attributes)) return false;
    }
    return true;
  };

  const areConceptsEqual = (a?: Concept[], b?: Concept[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const conceptA of arrA) {
      const conceptB = arrB.find((x) => x.guid === conceptA.guid);
      if (!conceptB) return false;
      if (conceptA.name !== conceptB.name) return false;
      if (normalizeValue(conceptA.description) !== normalizeValue(conceptB.description)) return false;
      if (normalizeValue(conceptA.icon) !== normalizeValue(conceptB.icon)) return false;
    }
    return true;
  };

  const areTagsEqual = (a?: Tag[], b?: Tag[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const tagA of arrA) {
      const tagB = arrB.find((x) => x.guid === tagA.guid);
      if (!tagB) return false;
      if (tagA.name !== tagB.name) return false;
      if (normalizeValue(tagA.description) !== normalizeValue(tagB.description)) return false;
      if (normalizeValue(tagA.icon) !== normalizeValue(tagB.icon)) return false;
    }
    return true;
  };

  if (a.guid !== b.guid) return false;
  if (a.name !== b.name) return false;
  if (normalizeValue(a.version) !== normalizeValue(b.version)) return false;
  if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
  if (normalizeValue(a.icon) !== normalizeValue(b.icon)) return false;
  if (normalizeValue(a.image) !== normalizeValue(b.image)) return false;
  if (normalizeValue(a.preview) !== normalizeValue(b.preview)) return false;
  if (normalizeValue(a.remote) !== normalizeValue(b.remote)) return false;
  if (normalizeValue(a.homepage) !== normalizeValue(b.homepage)) return false;
  if (normalizeValue(a.license) !== normalizeValue(b.license)) return false;

  if (!areConceptsEqual(a.concepts, b.concepts)) return false;
  if (!areTagsEqual(a.tags, b.tags)) return false;
  if (!areTypesEqual(a.types, b.types)) return false;
  if (!areDesignsEqual(a.designs, b.designs)) return false;
  if (!arePortsEqual(a.ports, b.ports)) return false;
  if (!areQualitiesEqual(a.qualities, b.qualities)) return false;
  if (!areFilesEqual(a.files, b.files)) return false;
  if (!areFoldersEqual(a.folders, b.folders)) return false;
  if (!areAuthorsEqual(a.authors, b.authors)) return false;
  if (!areAttributesEqual(a.attributes, b.attributes)) return false;

  return true;
};

/**
 * Deep equality check for KitDiffs entities.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨arekitdiffsequal](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/areKitDiffsEqual)
 **/
export const areKitDiffsEqual = (a: KitDiff, b: KitDiff): boolean => {
  const normalizeArray = <T>(arr: T[] | T | undefined | null): T[] => {
    if (!arr) return [];
    if (Array.isArray(arr)) return arr;
    return [arr as T];
  };
  const normalizeValue = (value: any): any => (value === null || value === "" || value === undefined ? undefined : value);
  const normalizeBoolean = (value: boolean | undefined): boolean | undefined => (value ? true : undefined);
  const areRemovedArraysEqual = (a?: { guid: string }[], b?: { guid: string }[]): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (a.length !== b.length) return false;
    const aGuids = new Set(a.map((x) => x.guid));
    const bGuids = new Set(b.map((x) => x.guid));
    for (const guid of aGuids) {
      if (!bGuids.has(guid)) return false;
    }
    return true;
  };

  const areAttributesDiffsEqual = (a?: AttributesDiff, b?: AttributesDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.attribute.guid === ua.attribute.guid);
      if (!ub) return false;
      if (!areAttributeDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.key !== ab.key) return false;
      if (normalizeValue(aa.value) !== normalizeValue(ab.value)) return false;
      if (normalizeValue(aa.definition) !== normalizeValue(ab.definition)) return false;
    }
    return true;
  };

  const areAttributeDiffsEqual = (a?: AttributeDiff, b?: AttributeDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.key) !== normalizeValue(b.key)) return false;
    if (normalizeValue(a.value) !== normalizeValue(b.value)) return false;
    if (normalizeValue(a.definition) !== normalizeValue(b.definition)) return false;
    return true;
  };

  const arePropsDiffsEqual = (a?: PropsDiff, b?: PropsDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.prop.guid === ua.prop.guid);
      if (!ub) return false;
      if (!arePropDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.quality.guid !== ab.quality.guid) return false;
      if (aa.value !== ab.value) return false;
      if (normalizeValue(aa.unit) !== normalizeValue(ab.unit)) return false;
    }
    return true;
  };

  const arePropDiffsEqual = (a?: PropDiff, b?: PropDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.value) !== normalizeValue(b.value)) return false;
    if (normalizeValue(a.unit) !== normalizeValue(b.unit)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areConnectorsDiffsEqual = (a?: z.infer<typeof ConnectorsDiffSchema>, b?: z.infer<typeof ConnectorsDiffSchema>): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.connector.guid === ua.connector.guid);
      if (!ub) return false;
      if (!areConnectorDiffEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (normalizeValue(aa.name) !== normalizeValue(ab.name)) return false;
      if (normalizeValue(aa.description) !== normalizeValue(ab.description)) return false;
      if (aa.point.x !== ab.point.x) return false;
      if (aa.point.y !== ab.point.y) return false;
      if (aa.point.z !== ab.point.z) return false;
      if (aa.direction.x !== ab.direction.x) return false;
      if (aa.direction.y !== ab.direction.y) return false;
      if (aa.direction.z !== ab.direction.z) return false;
      if (aa.t !== ab.t) return false;
      if (normalizeBoolean(aa.mandatory) !== normalizeBoolean(ab.mandatory)) return false;
    }
    return true;
  };

  const areConnectorDiffEqual = (a?: ConnectorDiff, b?: ConnectorDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (a.point && b.point) {
      if (normalizeValue(a.point.x) !== normalizeValue(b.point.x)) return false;
      if (normalizeValue(a.point.y) !== normalizeValue(b.point.y)) return false;
      if (normalizeValue(a.point.z) !== normalizeValue(b.point.z)) return false;
    } else if (a.point || b.point) {
      return false;
    }
    if (a.direction && b.direction) {
      if (normalizeValue(a.direction.x) !== normalizeValue(b.direction.x)) return false;
      if (normalizeValue(a.direction.y) !== normalizeValue(b.direction.y)) return false;
      if (normalizeValue(a.direction.z) !== normalizeValue(b.direction.z)) return false;
    } else if (a.direction || b.direction) {
      return false;
    }
    if (normalizeValue(a.t) !== normalizeValue(b.t)) return false;
    if (normalizeValue(a.mandatory) !== normalizeValue(b.mandatory)) return false;
    if (normalizeValue(a.port?.guid) !== normalizeValue(b.port?.guid)) return false;
    if (!arePropsDiffsEqual(a.props, b.props)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areModelsDiffsEqual = (a?: z.infer<typeof ModelsDiffSchema>, b?: z.infer<typeof ModelsDiffSchema>): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.model.guid === ua.model.guid);
      if (!ub) return false;
      if (!areModelDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (normalizeValue(aa.name) !== normalizeValue(ab.name)) return false;
      if (normalizeValue(aa.file?.guid) !== normalizeValue(ab.file?.guid)) return false;
      if (!arraysEqual(normalizeArray(aa.tags), normalizeArray(ab.tags))) return false;
    }
    return true;
  };

  const areModelDiffsEqual = (a?: ModelDiff, b?: ModelDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.file?.guid) !== normalizeValue(b.file?.guid)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (a.tags && b.tags) {
      if (!arraysEqual(normalizeArray(a.tags), normalizeArray(b.tags))) return false;
    } else if (a.tags || b.tags) {
      return false;
    }
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areTypesDiffsEqual = (a?: TypesDiff, b?: TypesDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.type.guid === ua.type.guid);
      if (!ub) return false;
      if (!areTypeDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areTypeDiffsEqual = (a?: TypeDiff, b?: TypeDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (normalizeValue(a.icon) !== normalizeValue(b.icon)) return false;
    if (normalizeValue(a.image) !== normalizeValue(b.image)) return false;
    if (normalizeValue(a.folder) !== normalizeValue(b.folder)) return false;
    if (normalizeValue(a.unit) !== normalizeValue(b.unit)) return false;
    if (normalizeValue(a.stock) !== normalizeValue(b.stock)) return false;
    if (normalizeValue(a.isAbstract) !== normalizeValue(b.isAbstract)) return false;
    if (normalizeValue(a.virtual) !== normalizeValue(b.virtual)) return false;
    if (normalizeValue(a.location?.guid) !== normalizeValue(b.location?.guid)) return false;
    if (a.concepts && b.concepts) {
      if (!arraysEqual(normalizeArray(a.concepts), normalizeArray(b.concepts))) return false;
    } else if (a.concepts || b.concepts) {
      return false;
    }
    if (!areModelsDiffsEqual(a.models, b.models)) return false;
    if (!areConnectorsDiffsEqual(a.connectors, b.connectors)) return false;
    if (!arePropsDiffsEqual(a.props, b.props)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const arePiecesDiffsEqual = (a?: PiecesDiff, b?: PiecesDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.piece.guid === ua.piece.guid);
      if (!ub) return false;
      if (!arePieceDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (normalizeValue(aa.name) !== normalizeValue(ab.name)) return false;
      if (aa.type?.guid !== ab.type?.guid) return false;
      if (aa.design?.guid !== ab.design?.guid) return false;
    }
    return true;
  };

  const arePieceDiffsEqual = (a?: PieceDiff, b?: PieceDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.type?.guid) !== normalizeValue(b.type?.guid)) return false;
    if (normalizeValue(a.design?.guid) !== normalizeValue(b.design?.guid)) return false;
    if (a.plane && b.plane) {
      if (a.plane.origin && b.plane.origin) {
        if (normalizeValue(a.plane.origin.x) !== normalizeValue(b.plane.origin.x)) return false;
        if (normalizeValue(a.plane.origin.y) !== normalizeValue(b.plane.origin.y)) return false;
        if (normalizeValue(a.plane.origin.z) !== normalizeValue(b.plane.origin.z)) return false;
      } else if (a.plane.origin || b.plane.origin) {
        return false;
      }
      if (a.plane.xAxis && b.plane.xAxis) {
        if (normalizeValue(a.plane.xAxis.x) !== normalizeValue(b.plane.xAxis.x)) return false;
        if (normalizeValue(a.plane.xAxis.y) !== normalizeValue(b.plane.xAxis.y)) return false;
        if (normalizeValue(a.plane.xAxis.z) !== normalizeValue(b.plane.xAxis.z)) return false;
      } else if (a.plane.xAxis || b.plane.xAxis) {
        return false;
      }
      if (a.plane.yAxis && b.plane.yAxis) {
        if (normalizeValue(a.plane.yAxis.x) !== normalizeValue(b.plane.yAxis.x)) return false;
        if (normalizeValue(a.plane.yAxis.y) !== normalizeValue(b.plane.yAxis.y)) return false;
        if (normalizeValue(a.plane.yAxis.z) !== normalizeValue(b.plane.yAxis.z)) return false;
      } else if (a.plane.yAxis || b.plane.yAxis) {
        return false;
      }
    } else if (a.plane || b.plane) {
      return false;
    }
    if (normalizeValue(a.scale) !== normalizeValue(b.scale)) return false;
    if (normalizeValue(a.isHidden) !== normalizeValue(b.isHidden)) return false;
    if (normalizeValue(a.isLocked) !== normalizeValue(b.isLocked)) return false;
    if (normalizeValue(a.color) !== normalizeValue(b.color)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (!arePropsDiffsEqual(a.props, b.props)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areConnectionsDiffsEqual = (a?: ConnectionsDiff, b?: ConnectionsDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.connection.guid === ua.connection.guid);
      if (!ub) return false;
      if (!areConnectionDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.connected.piece.guid !== ab.connected.piece.guid) return false;
      if (aa.connecting.piece.guid !== ab.connecting.piece.guid) return false;
    }
    return true;
  };

  const areConnectionDiffsEqual = (a?: ConnectionDiff, b?: ConnectionDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.gap) !== normalizeValue(b.gap)) return false;
    if (normalizeValue(a.shift) !== normalizeValue(b.shift)) return false;
    if (normalizeValue(a.rise) !== normalizeValue(b.rise)) return false;
    if (normalizeValue(a.rotation) !== normalizeValue(b.rotation)) return false;
    if (normalizeValue(a.turn) !== normalizeValue(b.turn)) return false;
    if (normalizeValue(a.tilt) !== normalizeValue(b.tilt)) return false;
    if (normalizeValue(a.u) !== normalizeValue(b.u)) return false;
    if (normalizeValue(a.v) !== normalizeValue(b.v)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areDesignsDiffsEqual = (a?: DesignsDiff, b?: DesignsDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.design.guid === ua.design.guid);
      if (!ub) return false;
      if (!areDesignDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areDesignDiffsEqual = (a?: DesignDiff, b?: DesignDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (normalizeValue(a.icon) !== normalizeValue(b.icon)) return false;
    if (normalizeValue(a.image) !== normalizeValue(b.image)) return false;
    if (a.concepts && b.concepts) {
      if (!arraysEqual(normalizeArray(a.concepts), normalizeArray(b.concepts))) return false;
    } else if (a.concepts || b.concepts) {
      return false;
    }
    if (!arePiecesDiffsEqual(a.pieces, b.pieces)) return false;
    if (!areConnectionsDiffsEqual(a.connections, b.connections)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const arePortsDiffsEqual = (a?: PortsDiff, b?: PortsDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.port.guid === ua.port.guid);
      if (!ub) return false;
      if (!arePortDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const arePortDiffsEqual = (a?: PortDiff, b?: PortDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areQualitiesDiffsEqual = (a?: z.infer<typeof QualitiesDiffSchema>, b?: z.infer<typeof QualitiesDiffSchema>): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.quality.guid === ua.quality.guid);
      if (!ub) return false;
      if (!areQualityDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.key !== ab.key) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areQualityDiffsEqual = (a?: QualityDiff, b?: QualityDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.key) !== normalizeValue(b.key)) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areFilesDiffsEqual = (a?: FilesDiff, b?: FilesDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.file.guid === ua.file.guid);
      if (!ub) return false;
      if (!areFileDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areFileDiffsEqual = (a?: FileDiff, b?: FileDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    return true;
  };

  const areFoldersDiffsEqual = (a?: FoldersDiff, b?: FoldersDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.folder.guid === ua.folder.guid);
      if (!ub) return false;
      if (!areFolderDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areFolderDiffsEqual = (a?: FolderDiff, b?: FolderDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areAuthorsDiffsEqual = (a?: AuthorsDiff, b?: AuthorsDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.author.guid === ua.author.guid);
      if (!ub) return false;
      if (!areAuthorDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areAuthorDiffsEqual = (a?: AuthorDiff, b?: AuthorDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.email) !== normalizeValue(b.email)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
  if (normalizeValue(a.version) !== normalizeValue(b.version)) return false;
  if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
  if (normalizeValue(a.icon) !== normalizeValue(b.icon)) return false;
  if (normalizeValue(a.image) !== normalizeValue(b.image)) return false;
  if (normalizeValue(a.preview) !== normalizeValue(b.preview)) return false;
  if (normalizeValue(a.remote) !== normalizeValue(b.remote)) return false;
  if (normalizeValue(a.homepage) !== normalizeValue(b.homepage)) return false;
  if (normalizeValue(a.license) !== normalizeValue(b.license)) return false;

  if (a.concepts && b.concepts) {
    if (!arraysEqual(normalizeArray(a.concepts), normalizeArray(b.concepts))) return false;
  } else if (a.concepts || b.concepts) {
    return false;
  }
  if (!areTypesDiffsEqual(a.types, b.types)) return false;
  if (!areDesignsDiffsEqual(a.designs, b.designs)) return false;
  if (!arePortsDiffsEqual(a.ports, b.ports)) return false;
  if (!areQualitiesDiffsEqual(a.qualities, b.qualities)) return false;
  if (!areFilesDiffsEqual(a.files, b.files)) return false;
  if (!areFoldersDiffsEqual(a.folders, b.folders)) return false;
  if (!areAuthorsDiffsEqual(a.authors, b.authors)) return false;
  if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;

  return true;
};

// [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨sqlitetokit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/sqliteToKit)
// sqliteToKit holds the data fields for a sqliteToKit record.
export const sqliteToKit = async (db: any): Promise<Kit> => {
  const existingTables = new Set<string>();
  const tableStmt = db.prepare("SELECT name FROM sqlite_master WHERE type='table'");
  while (tableStmt.step()) {
    existingTables.add(tableStmt.getAsObject().name as string);
  }
  tableStmt.free();

  const execResult = (query: string, params?: any[]): any[] => {
    const stmt = db.prepare(query);
    if (params) {
      stmt.bind(params);
    }
    const result: any[] = [];
    while (stmt.step()) {
      const row = stmt.getAsObject();
      result.push(row);
    }
    stmt.free();
    return result;
  };

  const safeExecResult = (tableName: string, query: string, params?: any[]): any[] => {
    if (!existingTables.has(tableName)) {
      return [];
    }
    return execResult(query, params);
  };

  const kitRows = execResult("SELECT * FROM kit LIMIT 1");
  if (kitRows.length === 0) {
    throw new Error("No kit found in database");
  }
  const kitRow = kitRows[0];

  const toUndefined = (value: any): any => (value === null || value === "" ? undefined : value);
  const buildAttribute = (a: any): any => {
    const attr: any = { guid: a.guid, key: a.key };
    const value = toUndefined(a.value);
    const definition = toUndefined(a.definition);
    if (value !== undefined) attr.value = value;
    if (definition !== undefined) attr.definition = definition;
    return attr;
  };
  const mapOrUndefined = <T, R>(arr: T[], mapper: (item: T) => R): R[] | undefined => (arr.length > 0 ? arr.map(mapper) : undefined);

  const kit: Kit = {
    guid: kitRow.guid || kitRow.uri || guid(),
    name: kitRow.name || "Unnamed Kit",
    version: kitRow.version || "0.0.0",
    description: toUndefined(kitRow.description),
    icon: toUndefined(kitRow.icon),
    image: toUndefined(kitRow.image),
    preview: toUndefined(kitRow.preview),
    remote: toUndefined(kitRow.remote),
    homepage: toUndefined(kitRow.homepage),
    license: toUndefined(kitRow.license),
    createdAt: kitRow.created,
    updatedAt: kitRow.updated,
  };

  const types = execResult("SELECT * FROM type WHERE kit_guid = ?", [kit.guid]);
  kit.types = mapOrUndefined(types, (row: any) => {
    const typeGuid = row.guid || String(row.id);
    const models = execResult("SELECT * FROM model WHERE type_guid = ?", [typeGuid]);
    const connectors = execResult("SELECT * FROM connector WHERE type_guid = ?", [typeGuid]);
    const typeAttributes = execResult("SELECT * FROM attribute WHERE type_guid = ?", [typeGuid]);
    const typeConcepts = execResult("SELECT * FROM type_concept WHERE type_guid = ?", [typeGuid]);
    const typeAuthors = execResult("SELECT * FROM type_author WHERE type_guid = ? ORDER BY rank", [typeGuid]);

    const type: any = {
      guid: typeGuid,
      name: row.name,
      createdAt: row.created,
      updatedAt: row.updated,
    };
    if (row.is_abstract) type.isAbstract = true;
    const folder = toUndefined(row.folder);
    if (folder !== undefined) type.folder = folder;
    const description = toUndefined(row.description);
    if (description !== undefined) type.description = description;
    const icon = toUndefined(row.icon);
    if (icon !== undefined) type.icon = icon;
    const image = toUndefined(row.image);
    if (image !== undefined) type.image = image;
    if (row.parent_guid || row.parent_id) type.parent = { guid: row.parent_guid || String(row.parent_id) };
    if (row.virtual) type.virtual = true;
    const unit = toUndefined(row.unit);
    if (unit !== undefined) type.unit = unit;
    if (row.stock !== null && row.stock !== undefined) type.stock = row.stock;
    if (row.location_guid) type.location = { guid: row.location_guid };

    const concepts = mapOrUndefined(typeConcepts, (c: any) => c.concept);
    if (concepts) type.concepts = concepts;

    const authors = mapOrUndefined(typeAuthors, (ta: any) => ({ guid: ta.author_guid }));
    if (authors) type.authors = authors;

    const models_value = mapOrUndefined(models, (m: any) => {
      const modelTags = execResult("SELECT tag_guid FROM model_tag WHERE model_guid = ?", [m.guid]);
      const modelAttributes = execResult("SELECT * FROM attribute WHERE model_guid = ?", [m.guid]);
      return {
        guid: m.guid,
        file: { guid: m.file_guid },
        name: toUndefined(m.name),
        description: toUndefined(m.description),
        tags: modelTags.map((t: any) => ({ guid: t.tag_guid })),
        attributes: mapOrUndefined(modelAttributes, buildAttribute),
      };
    });
    if (models_value) type.models = models_value;

    const connectors_value = mapOrUndefined(connectors, (p: any) => {
      const connectorProps = execResult("SELECT * FROM prop WHERE connector_guid = ?", [p.guid]);
      const connectorAttributes = execResult("SELECT * FROM attribute WHERE connector_guid = ?", [p.guid]);

      const connector: any = {
        guid: p.guid,
        point: { x: p.point_x, y: p.point_y, z: p.point_z },
        direction: { x: p.direction_x, y: p.direction_y, z: p.direction_z },
        t: p.t,
      };

      if (p.name) connector.name = p.name;
      if (p.mandatory) connector.mandatory = true;
      if (p.port_guid) connector.port = { guid: p.port_guid };
      if (p.description) connector.description = p.description;

      const props_value = connectorProps
        .map((pr: any) => {
          const propAttributes = execResult("SELECT * FROM attribute WHERE prop_guid = ?", [pr.guid]);
          if (!pr.quality_guid) return null;
          return {
            guid: pr.guid,
            value: String(pr.value),
            unit: toUndefined(pr.unit),
            quality: { guid: pr.quality_guid },
            attributes: mapOrUndefined(propAttributes, buildAttribute),
          };
        })
        .filter((p: any): p is NonNullable<typeof p> => p !== null);
      if (props_value && props_value.length > 0) connector.props = props_value;

      const attributes_value = mapOrUndefined(connectorAttributes, buildAttribute);
      if (attributes_value) connector.attributes = attributes_value;

      return connector;
    });
    if (connectors_value) type.connectors = connectors_value;

    const typeProps = safeExecResult("type_prop", "SELECT prop.* FROM prop JOIN type_prop ON prop.guid = type_prop.prop_guid WHERE type_prop.type_guid = ?", [typeGuid]);
    const props_value = (() => {
      const filtered = typeProps
        .map((pr: any) => {
          const propAttributes = execResult("SELECT * FROM attribute WHERE prop_guid = ?", [pr.guid]);
          if (!pr.quality_guid) return null;
          return {
            guid: pr.guid,
            value: String(pr.value),
            unit: toUndefined(pr.unit),
            quality: { guid: pr.quality_guid },
            attributes: mapOrUndefined(propAttributes, buildAttribute),
          };
        })
        .filter((p: any): p is NonNullable<typeof p> => p !== null);
      return filtered.length > 0 ? filtered : undefined;
    })();
    if (props_value) type.props = props_value;

    const attributes_value = mapOrUndefined(typeAttributes, buildAttribute);
    if (attributes_value) type.attributes = attributes_value;

    return type;
  });

  const designs = execResult("SELECT * FROM design WHERE kit_guid = ?", [kit.guid]);
  kit.designs = mapOrUndefined(designs, (row: any) => {
    const designGuid = row.guid || String(row.id);
    const pieces = execResult("SELECT * FROM piece WHERE design_guid = ?", [designGuid]);
    const connections = execResult("SELECT * FROM connection WHERE design_guid = ?", [designGuid]);
    const layers = execResult("SELECT * FROM layer WHERE design_guid = ?", [designGuid]);
    const groups = execResult('SELECT * FROM "group" WHERE design_guid = ?', [designGuid]);
    const stats = execResult("SELECT * FROM stat WHERE design_guid = ?", [designGuid]);
    const designAttributes = execResult("SELECT * FROM attribute WHERE design_guid = ?", [designGuid]);
    const designConcepts = execResult("SELECT * FROM design_concept WHERE design_guid = ?", [designGuid]);
    const designProps = execResult("SELECT * FROM design_prop WHERE design_guid = ?", [designGuid]);
    const designAuthors = execResult("SELECT * FROM design_author WHERE design_guid = ? ORDER BY rank ASC", [designGuid]);

    return {
      guid: designGuid,
      name: row.name,
      description: toUndefined(row.description),
      icon: toUndefined(row.icon),
      image: toUndefined(row.image),
      parent: row.parent_guid ? { guid: row.parent_guid } : row.parent_id ? { guid: String(row.parent_id) } : undefined,
      unit: toUndefined(row.unit),
      isAbstract: row.is_abstract ? true : undefined,
      folder: toUndefined(row.folder),
      canScale: row.can_scale ? true : undefined,
      canMirror: row.can_mirror ? true : undefined,
      createdAt: row.created,
      updatedAt: row.updated,
      activeLayer: row.active_layer_guid ? { guid: row.active_layer_guid } : undefined,
      props: mapOrUndefined(designProps, (dp: any) => ({
        guid: dp.guid,
        quality: { guid: dp.quality_guid },
        value: String(dp.value),
        unit: toUndefined(dp.unit),
      })),
      authors: mapOrUndefined(designAuthors, (da: any) => ({ guid: da.author_guid })),
      pieces: pieces.map((p: any) => {
        const pieceProps = execResult("SELECT prop.* FROM prop JOIN piece_prop ON prop.guid = piece_prop.prop_guid WHERE piece_prop.piece_guid = ?", [p.guid]);
        const pieceAttributes = execResult("SELECT * FROM attribute WHERE piece_guid = ?", [p.guid]);
        return {
          guid: p.guid,
          name: toUndefined(p.name),
          type: p.type_guid ? { guid: p.type_guid } : undefined,
          design: p.design_guid_ref ? { guid: p.design_guid_ref } : undefined,
          plane:
            p.plane_origin_x !== null
              ? {
                origin: { x: p.plane_origin_x, y: p.plane_origin_y, z: p.plane_origin_z },
                xAxis: { x: p.plane_x_axis_x, y: p.plane_x_axis_y, z: p.plane_x_axis_z },
                yAxis: { x: p.plane_y_axis_x, y: p.plane_y_axis_y, z: p.plane_y_axis_z },
              }
              : undefined,
          center: p.center_u !== null || p.center_v !== null ? { u: p.center_u, v: p.center_v } : undefined,
          scale: p.scale !== null ? p.scale : undefined,
          mirrorPlane:
            p.mirror_plane_origin_x !== null
              ? {
                origin: { x: p.mirror_plane_origin_x, y: p.mirror_plane_origin_y, z: p.mirror_plane_origin_z },
                xAxis: { x: p.mirror_plane_x_axis_x, y: p.mirror_plane_x_axis_y, z: p.mirror_plane_x_axis_z },
                yAxis: { x: p.mirror_plane_y_axis_x, y: p.mirror_plane_y_axis_y, z: p.mirror_plane_y_axis_z },
              }
              : undefined,
          isHidden: p.is_hidden ? true : undefined,
          isLocked: p.is_locked ? true : undefined,
          color: toUndefined(p.color),
          description: toUndefined(p.description),
          props: (() => {
            const filtered = pieceProps
              .map((pr: any) => {
                const propAttributes = execResult("SELECT * FROM attribute WHERE prop_guid = ?", [pr.guid]);
                if (!pr.quality_guid) return null;
                return {
                  guid: pr.guid,
                  value: String(pr.value),
                  unit: toUndefined(pr.unit),
                  quality: { guid: pr.quality_guid },
                  attributes: mapOrUndefined(propAttributes, buildAttribute),
                };
              })
              .filter((p: any): p is NonNullable<typeof p> => p !== null);
            return filtered.length > 0 ? filtered : undefined;
          })(),
          attributes: mapOrUndefined(pieceAttributes, buildAttribute),
        };
      }),
      connections: connections.map((c: any) => {
        const connectionAttributes = execResult("SELECT * FROM attribute WHERE connection_guid = ?", [c.guid]);
        return {
          guid: c.guid,
          connected: {
            piece: { guid: c.connected_piece_guid },
            designPiece: c.connected_design_piece_guid ? { guid: c.connected_design_piece_guid } : undefined,
            connector: { guid: c.connected_connector_guid },
          },
          connecting: {
            piece: { guid: c.connecting_piece_guid },
            designPiece: c.connecting_design_piece_guid ? { guid: c.connecting_design_piece_guid } : undefined,
            connector: { guid: c.connecting_connector_guid },
          },
          gap: c.gap || 0,
          shift: c.shift || 0,
          rise: c.rise || 0,
          rotation: c.rotation || 0,
          turn: c.turn || 0,
          tilt: c.tilt || 0,
          u: c.u !== null ? c.u : undefined,
          v: c.v !== null ? c.v : undefined,
          description: toUndefined(c.description),
          attributes: mapOrUndefined(connectionAttributes, buildAttribute),
        };
      }),
      layers: layers.map((l: any) => {
        const layerAttributes = execResult("SELECT * FROM attribute WHERE layer_guid = ?", [l.guid]);
        return {
          guid: l.guid,
          path: l.path,
          isHidden: l.is_hidden ? true : undefined,
          isLocked: l.is_locked ? true : undefined,
          color: toUndefined(l.color),
          description: toUndefined(l.description),
          attributes: mapOrUndefined(layerAttributes, buildAttribute),
        };
      }),
      groups: groups.map((g: any) => {
        const groupPieces = execResult("SELECT piece_guid FROM group_piece WHERE group_guid = ?", [g.guid]);
        const groupAttributes = execResult("SELECT * FROM attribute WHERE group_guid = ?", [g.guid]);
        return {
          guid: g.guid,
          name: toUndefined(g.name),
          color: toUndefined(g.color),
          description: toUndefined(g.description),
          pieces: groupPieces.map((gp: any) => ({ guid: gp.piece_guid })),
          attributes: mapOrUndefined(groupAttributes, buildAttribute),
        };
      }),
      stats: stats.map((s: any) => ({
        guid: s.guid,
        quality: { guid: s.quality_guid },
        min: s.min_value ?? undefined,
        minExcluded: s.min_excluded ? true : undefined,
        max: s.max_value ?? undefined,
        maxExcluded: s.max_excluded ? true : undefined,
        unit: toUndefined(s.unit),
      })),
      attributes: mapOrUndefined(designAttributes, buildAttribute),
      concepts: designConcepts.length > 0 ? designConcepts.map((c: any) => c.concept) : undefined,
    };
  });

  const ports = execResult("SELECT * FROM port WHERE kit_guid = ?", [kit.guid]);
  kit.ports = mapOrUndefined(ports, (row: any) => {
    const compatiblePorts = execResult("SELECT compatible_port_guid FROM port_compatibility WHERE port_guid = ?", [row.guid]);
    const portAttributes = execResult("SELECT * FROM attribute WHERE port_guid = ?", [row.guid]);
    return {
      guid: row.guid,
      name: row.name,
      description: toUndefined(row.description),
      icon: toUndefined(row.icon),
      compatiblePorts: compatiblePorts.length > 0 ? compatiblePorts.map((ci: any) => ({ guid: ci.compatible_port_guid })) : undefined,
      attributes: mapOrUndefined(portAttributes, buildAttribute),
    };
  });

  const tags = safeExecResult("tag", "SELECT * FROM tag WHERE kit_guid = ?", [kit.guid]);
  kit.tags = mapOrUndefined(tags, (row: any) => ({
    guid: row.guid,
    name: row.name,
    description: toUndefined(row.description),
    icon: toUndefined(row.icon),
  }));

  const qualities = execResult("SELECT * FROM quality WHERE kit_guid = ?", [kit.guid]);
  kit.qualities =
    qualities.length > 0
      ? qualities.map((row: any) => {
        const benchmarks = execResult("SELECT * FROM benchmark WHERE quality_guid = ?", [row.guid]);
        const qualityAttributes = execResult("SELECT * FROM attribute WHERE quality_guid = ?", [row.guid]);
        return {
          guid: row.guid,
          key: row.key,
          name: row.name,
          kind: row.kind || undefined,
          defaultValue: row.default_value ?? undefined,
          formula: toUndefined(row.formula),
          defaultSiUnit: toUndefined(row.default_si_unit),
          defaultImperialUnit: toUndefined(row.default_imperial_unit),
          min: row.min_value ?? undefined,
          minExcluded: row.min_excluded ? true : undefined,
          max: row.max_value ?? undefined,
          maxExcluded: row.max_excluded ? true : undefined,
          canScale: row.can_scale ? true : undefined,
          uri: toUndefined(row.definition),
          benchmarks: benchmarks.map((b: any) => {
            const benchmarkAttributes = execResult("SELECT * FROM attribute WHERE benchmark_guid = ?", [b.guid]);
            return {
              guid: b.guid,
              name: b.name,
              icon: toUndefined(b.icon),
              min: b.min_value ?? undefined,
              minExcluded: b.min_excluded ? true : undefined,
              max: b.max_value ?? undefined,
              maxExcluded: b.max_excluded ? true : undefined,
              attributes: mapOrUndefined(benchmarkAttributes, buildAttribute),
            };
          }),
          attributes: mapOrUndefined(qualityAttributes, buildAttribute),
        };
      })
      : undefined;

  const files = execResult("SELECT * FROM file WHERE kit_guid = ?", [kit.guid]);
  kit.files =
    files.length > 0
      ? files.map((row: any) => ({
        guid: row.guid,
        name: row.name,
        remote: toUndefined(row.remote_url),
        folder: row.folder_guid ? { guid: row.folder_guid } : undefined,
        size: row.size ?? undefined,
        hash: toUndefined(row.hash),
        createdAt: row.created,
        updatedAt: row.updated,
      }))
      : undefined;

  const folders = execResult("SELECT * FROM folder WHERE kit_guid = ?", [kit.guid]);
  kit.folders = mapOrUndefined(folders, (row: any) => ({
    guid: row.guid,
    name: row.name,
    parent: row.parent_guid ? { guid: row.parent_guid } : undefined,
    createdAt: row.created,
    updatedAt: row.updated,
  }));

  const authors = execResult("SELECT * FROM author WHERE kit_guid = ?", [kit.guid]);
  kit.authors =
    authors.length > 0
      ? authors.map((row: any) => ({
        guid: row.guid,
        name: row.name,
        email: toUndefined(row.email),
      }))
      : undefined;

  const concepts = execResult("SELECT * FROM concept WHERE kit_guid = ?", [kit.guid]);
  kit.concepts = mapOrUndefined(concepts, (row: any) => ({
    guid: row.guid,
    name: row.name,
    description: toUndefined(row.description),
    icon: toUndefined(row.icon),
  }));

  const kitAttributes = execResult("SELECT * FROM attribute WHERE kit_guid = ?", [kit.guid]);
  kit.attributes = mapOrUndefined(kitAttributes, buildAttribute);

  return kit;
};

// [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨toarray](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/toArray)
// toArray holds the data fields for a toArray record.
const toArray = <T>(value: T | T[] | undefined): T[] => {
  if (!value) return [];
  return Array.isArray(value) ? value : [value];
};

/**
 * Constant value for KIT_SQLITE_SCHEMA.
 * [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨kitsqliteschema](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/KIT_SQLITE_SCHEMA)
 **/
export const KIT_SQLITE_SCHEMA = `
CREATE TABLE semio (
	release VARCHAR NOT NULL,
	engine VARCHAR NOT NULL,
	created DATETIME NOT NULL,
	PRIMARY KEY (release)
);

CREATE TABLE kit (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	version VARCHAR(64),
	description TEXT,
	icon TEXT,
	image TEXT,
	preview TEXT,
	remote TEXT,
	homepage TEXT,
	license TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	PRIMARY KEY (guid)
);

CREATE TABLE quality (
	guid VARCHAR(36) NOT NULL,
	key VARCHAR(128) NOT NULL,
	name VARCHAR(256) NOT NULL,
	kind INTEGER NOT NULL,
	default_value FLOAT,
	formula TEXT,
	default_si_unit VARCHAR(64),
	default_imperial_unit VARCHAR(64),
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	can_scale BOOLEAN NOT NULL DEFAULT 0,
	definition TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE benchmark (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	icon TEXT,
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	definition TEXT,
	quality_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid)
);

CREATE TABLE port (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE port_compatibility (
	port_guid VARCHAR(36) NOT NULL,
	compatible_port_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (port_guid, compatible_port_guid),
	FOREIGN KEY(port_guid) REFERENCES port (guid),
	FOREIGN KEY(compatible_port_guid) REFERENCES port (guid)
);

CREATE TABLE folder (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	parent_guid VARCHAR(36),
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(parent_guid) REFERENCES folder (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE file (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	folder_guid VARCHAR(36),
	size INTEGER,
	hash VARCHAR(128),
	remote_url TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(folder_guid) REFERENCES folder (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE author (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	email VARCHAR(256),
	kit_guid VARCHAR(36),
	type_guid VARCHAR(36),
	design_guid VARCHAR(36),
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE tag (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE type (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	parent_guid VARCHAR(36),
	is_abstract BOOLEAN NOT NULL DEFAULT 0,
	folder VARCHAR(256),
	stock INTEGER,
	virtual BOOLEAN NOT NULL DEFAULT 0,
	unit VARCHAR(64),
	location_guid VARCHAR(36),
	description TEXT,
	icon TEXT,
	image TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	row_id INTEGER PRIMARY KEY AUTOINCREMENT,
	UNIQUE (guid, kit_guid, parent_guid),
	FOREIGN KEY(parent_guid) REFERENCES type (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE model (
	guid VARCHAR(36) NOT NULL,
	file_guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	description TEXT,
	type_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(file_guid) REFERENCES file (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid)
);

CREATE TABLE model_tag (
	model_guid VARCHAR(36) NOT NULL,
	tag_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (model_guid, tag_guid),
	FOREIGN KEY(model_guid) REFERENCES model (guid),
	FOREIGN KEY(tag_guid) REFERENCES tag (guid)
);

CREATE TABLE prop (
	guid VARCHAR(36) NOT NULL,
	key VARCHAR(128) NOT NULL,
	value FLOAT NOT NULL,
	unit VARCHAR(64),
	quality_guid VARCHAR(36),
	connector_guid VARCHAR(36),
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid)
);

CREATE TABLE type_prop (
	type_guid VARCHAR(36) NOT NULL,
	prop_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (type_guid, prop_guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(prop_guid) REFERENCES prop (guid)
);

CREATE TABLE connector (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	point_x FLOAT NOT NULL,
	point_y FLOAT NOT NULL,
	point_z FLOAT NOT NULL,
	direction_x FLOAT NOT NULL,
	direction_y FLOAT NOT NULL,
	direction_z FLOAT NOT NULL,
	t FLOAT NOT NULL,
	mandatory BOOLEAN NOT NULL DEFAULT 0,
	port_guid VARCHAR(36),
	description TEXT,
	type_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	UNIQUE (guid, type_guid),
	FOREIGN KEY(port_guid) REFERENCES port (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid)
);

CREATE TABLE design (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	parent_guid VARCHAR(36),
	variant VARCHAR(256),
	view_center_u FLOAT,
	view_center_v FLOAT,
	view_zoom FLOAT,
	unit VARCHAR(64),
	location_guid VARCHAR(36),
	active_layer_guid VARCHAR(36),
	is_abstract BOOLEAN,
	folder VARCHAR(256),
	can_scale BOOLEAN,
	can_mirror BOOLEAN,
	description TEXT,
	icon TEXT,
	image TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	row_id INTEGER PRIMARY KEY AUTOINCREMENT,
	UNIQUE (guid, kit_guid, parent_guid),
	FOREIGN KEY(parent_guid) REFERENCES design (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE design_prop (
	guid VARCHAR(36) NOT NULL,
	design_guid VARCHAR(36) NOT NULL,
	quality_guid VARCHAR(36) NOT NULL,
	value FLOAT NOT NULL,
	unit VARCHAR(64),
	PRIMARY KEY (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid)
);

CREATE TABLE design_author (
	design_guid VARCHAR(36) NOT NULL,
	author_guid VARCHAR(36) NOT NULL,
	rank INTEGER NOT NULL,
	PRIMARY KEY (design_guid, author_guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid)
);

CREATE TABLE layer (
	guid VARCHAR(36) NOT NULL,
	path VARCHAR(512) NOT NULL,
	is_hidden BOOLEAN NOT NULL DEFAULT 0,
	is_locked BOOLEAN NOT NULL DEFAULT 0,
	color VARCHAR(32),
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE piece (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	type_guid VARCHAR(36),
	design_guid_ref VARCHAR(36),
	plane_origin_x FLOAT,
	plane_origin_y FLOAT,
	plane_origin_z FLOAT,
	plane_x_axis_x FLOAT,
	plane_x_axis_y FLOAT,
	plane_x_axis_z FLOAT,
	plane_y_axis_x FLOAT,
	plane_y_axis_y FLOAT,
	plane_y_axis_z FLOAT,
	center_u FLOAT,
	center_v FLOAT,
	scale FLOAT,
	mirror_plane_origin_x FLOAT,
	mirror_plane_origin_y FLOAT,
	mirror_plane_origin_z FLOAT,
	mirror_plane_x_axis_x FLOAT,
	mirror_plane_x_axis_y FLOAT,
	mirror_plane_x_axis_z FLOAT,
	mirror_plane_y_axis_x FLOAT,
	mirror_plane_y_axis_y FLOAT,
	mirror_plane_y_axis_z FLOAT,
	is_hidden BOOLEAN NOT NULL DEFAULT 0,
	is_locked BOOLEAN NOT NULL DEFAULT 0,
	color VARCHAR(32),
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(design_guid_ref) REFERENCES design (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE piece_prop (
	piece_guid VARCHAR(36) NOT NULL,
	prop_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (piece_guid, prop_guid),
	FOREIGN KEY(piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(prop_guid) REFERENCES prop (guid)
);

CREATE TABLE "group" (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	color VARCHAR(32),
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE group_piece (
	group_guid VARCHAR(36) NOT NULL,
	piece_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (group_guid, piece_guid),
	FOREIGN KEY(group_guid) REFERENCES "group" (guid),
	FOREIGN KEY(piece_guid) REFERENCES piece (guid)
);

CREATE TABLE connection (
	guid VARCHAR(36) NOT NULL,
	connected_piece_guid VARCHAR(36) NOT NULL,
	connected_design_piece_guid VARCHAR(36),
	connected_connector_guid VARCHAR(36) NOT NULL,
	connecting_piece_guid VARCHAR(36) NOT NULL,
	connecting_design_piece_guid VARCHAR(36),
	connecting_connector_guid VARCHAR(36) NOT NULL,
	gap FLOAT NOT NULL DEFAULT 0,
	shift FLOAT NOT NULL DEFAULT 0,
	rise FLOAT NOT NULL DEFAULT 0,
	rotation FLOAT NOT NULL DEFAULT 0,
	turn FLOAT NOT NULL DEFAULT 0,
	tilt FLOAT NOT NULL DEFAULT 0,
	u FLOAT,
	v FLOAT,
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	CHECK (connecting_piece_guid != connected_piece_guid),
	FOREIGN KEY(connected_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(connected_connector_guid) REFERENCES connector (guid),
	FOREIGN KEY(connecting_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(connecting_connector_guid) REFERENCES connector (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE stat (
	guid VARCHAR(36) NOT NULL,
	quality_guid VARCHAR(36) NOT NULL,
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	unit VARCHAR(64),
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE concept (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE type_concept (
	type_guid VARCHAR(36) NOT NULL,
	concept VARCHAR(256) NOT NULL,
	PRIMARY KEY (type_guid, concept)
);

CREATE TABLE type_author (
	type_guid VARCHAR(36) NOT NULL,
	author_guid VARCHAR(36) NOT NULL,
	rank INTEGER NOT NULL,
	PRIMARY KEY (type_guid, author_guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid)
);

CREATE TABLE design_concept (
	design_guid VARCHAR(36) NOT NULL,
	concept VARCHAR(256) NOT NULL,
	PRIMARY KEY (design_guid, concept)
);

CREATE TABLE attribute (
	guid VARCHAR(36) NOT NULL,
	key VARCHAR(256) NOT NULL,
	value TEXT,
	definition TEXT,
	quality_guid VARCHAR(36),
	benchmark_guid VARCHAR(36),
	port_guid VARCHAR(36),
	folder_guid VARCHAR(36),
	file_guid VARCHAR(36),
	author_guid VARCHAR(36),
	model_guid VARCHAR(36),
	prop_guid VARCHAR(36),
	connector_guid VARCHAR(36),
	type_guid VARCHAR(36),
	layer_guid VARCHAR(36),
	piece_guid VARCHAR(36),
	group_guid VARCHAR(36),
	connection_guid VARCHAR(36),
	stat_guid VARCHAR(36),
	design_guid VARCHAR(36),
	kit_guid VARCHAR(36),
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid),
	FOREIGN KEY(benchmark_guid) REFERENCES benchmark (guid),
	FOREIGN KEY(port_guid) REFERENCES port (guid),
	FOREIGN KEY(folder_guid) REFERENCES folder (guid),
	FOREIGN KEY(file_guid) REFERENCES file (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid),
	FOREIGN KEY(model_guid) REFERENCES model (guid),
	FOREIGN KEY(prop_guid) REFERENCES prop (guid),
	FOREIGN KEY(connector_guid) REFERENCES connector (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(layer_guid) REFERENCES layer (guid),
	FOREIGN KEY(piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(group_guid) REFERENCES "group" (guid),
	FOREIGN KEY(connection_guid) REFERENCES connection (guid),
	FOREIGN KEY(stat_guid) REFERENCES stat (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);
`;

// kitToSqlite holds the data fields for a kitToSqlite record.
// [👤semio📚js💻semio🔖kit🔖kitimport🔖export🪨kittosqlite](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Import/Export/d/i/kitToSqlite)
export const kitToSqlite = async (kit: Kit, db: any): Promise<void> => {
  db.exec(KIT_SQLITE_SCHEMA);

  const toISOString = (date: Date | string | undefined): string => {
    if (!date) return new Date().toISOString();
    if (typeof date === "string") return date;
    return date.toISOString();
  };

  db.run("INSERT INTO semio (release, engine, created) VALUES (?, ?, ?)", ["1.0.0", "js", new Date().toISOString()]);

  db.run("INSERT INTO kit (guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
    kit.guid,
    kit.name,
    kit.version || null,
    kit.description || null,
    kit.icon || null,
    kit.image || null,
    kit.preview || null,
    kit.remote || null,
    kit.homepage || null,
    kit.license || null,
    toISOString(kit.createdAt),
    toISOString(kit.updatedAt),
  ]);

  toArray(kit.concepts).forEach((concept) => {
    if (typeof concept === "object") {
      db.run("INSERT INTO concept (guid, name, description, icon, kit_guid) VALUES (?, ?, ?, ?, ?)", [concept.guid, concept.name, concept.description || null, concept.icon || null, kit.guid]);
    } else {
      db.run("INSERT INTO concept (guid, name, kit_guid) VALUES (?, ?, ?)", [guid(), concept, kit.guid]);
    }
  });

  toArray(kit.attributes).forEach((attr) => {
    db.run("INSERT INTO attribute (guid, key, value, definition, kit_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, kit.guid]);
  });

  toArray(kit.ports).forEach((iface) => {
    db.run("INSERT INTO port (guid, name, description, icon, kit_guid) VALUES (?, ?, ?, ?, ?)", [iface.guid, iface.name, iface.description || null, iface.icon || null, kit.guid]);

    toArray(iface.compatiblePorts).forEach((compat) => {
      db.run("INSERT INTO port_compatibility (port_guid, compatible_port_guid) VALUES (?, ?)", [iface.guid, compat.guid]);
    });

    toArray(iface.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, port_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, iface.guid]);
    });
  });

  toArray(kit.qualities).forEach((quality) => {
    db.run(
      "INSERT INTO quality (guid, key, name, kind, default_value, formula, default_si_unit, default_imperial_unit, min_value, min_excluded, max_value, max_excluded, can_scale, definition, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
      [
        quality.guid,
        quality.key,
        quality.name,
        quality.kind ?? 0,
        quality.defaultValue || null,
        quality.formula || null,
        quality.defaultSiUnit || null,
        quality.defaultImperialUnit || null,
        quality.min || null,
        quality.isMinExcluded ? 1 : null,
        quality.max || null,
        quality.isMaxExcluded ? 1 : null,
        quality.canScale ? 1 : 0,
        quality.uri || null,
        kit.guid,
      ],
    );

    toArray(quality.benchmarks).forEach((benchmark) => {
      db.run("INSERT INTO benchmark (guid, name, icon, min_value, min_excluded, max_value, max_excluded, quality_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", [
        benchmark.guid,
        benchmark.name,
        benchmark.icon || null,
        benchmark.min || null,
        benchmark.minExcluded ? 1 : null,
        benchmark.max || null,
        benchmark.maxExcluded ? 1 : null,
        quality.guid,
      ]);

      toArray(benchmark.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, benchmark_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, benchmark.guid]);
      });
    });

    toArray(quality.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, quality_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, quality.guid]);
    });
  });

  toArray(kit.folders).forEach((folder) => {
    db.run("INSERT INTO folder (guid, name, parent_guid, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?)", [folder.guid, folder.name, folder.parent?.guid || null, toISOString(folder.createdAt), toISOString(folder.updatedAt), kit.guid]);
  });

  toArray(kit.files).forEach((file) => {
    db.run("INSERT INTO file (guid, name, folder_guid, size, hash, remote_url, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)", [
      file.guid,
      file.name,
      file.folder?.guid || null,
      file.size || null,
      file.hash || null,
      file.remote || null,
      toISOString(file.createdAt),
      toISOString(file.updatedAt),
      kit.guid,
    ]);
  });

  toArray(kit.authors).forEach((author) => {
    db.run("INSERT INTO author (guid, name, email, kit_guid) VALUES (?, ?, ?, ?)", [author.guid, author.name, author.email || null, kit.guid]);
  });

  toArray(kit.tags).forEach((tag) => {
    db.run("INSERT INTO tag (guid, name, description, icon, kit_guid) VALUES (?, ?, ?, ?, ?)", [tag.guid, tag.name, tag.description || null, tag.icon || null, kit.guid]);
  });

  toArray(kit.types).forEach((type) => {
    db.run("INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
      type.guid,
      type.name,
      type.parent?.guid || null,
      type.isAbstract ? 1 : 0,
      type.folder || null,
      type.stock || null,
      type.virtual ? 1 : 0,
      type.unit || null,
      type.description || null,
      type.icon || null,
      type.image || null,
      toISOString(type.createdAt),
      toISOString(type.updatedAt),
      kit.guid,
    ]);

    toArray(type.concepts).forEach((concept) => {
      db.run("INSERT INTO type_concept (type_guid, concept) VALUES (?, ?)", [type.guid, concept]);
    });

    toArray(type.authors).forEach((authorId, index) => {
      db.run("INSERT INTO type_author (type_guid, author_guid, rank) VALUES (?, ?, ?)", [type.guid, typeof authorId === "object" ? authorId.guid : authorId, index]);
    });

    toArray(type.models).forEach((model) => {
      db.run("INSERT INTO model (guid, file_guid, name, description, type_guid) VALUES (?, ?, ?, ?, ?)", [model.guid, model.file.guid, model.name || null, model.description || null, type.guid]);

      toArray(model.tags).forEach((tag) => {
        db.run("INSERT INTO model_tag (model_guid, tag_guid) VALUES (?, ?)", [model.guid, typeof tag === "object" ? tag.guid : tag]);
      });

      toArray(model.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, model_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, model.guid]);
      });
    });

    toArray(type.connectors).forEach((connector) => {
      db.run("INSERT INTO connector (guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description, type_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
        connector.guid,
        connector.name || null,
        connector.point.x,
        connector.point.y,
        connector.point.z,
        connector.direction.x,
        connector.direction.y,
        connector.direction.z,
        connector.t,
        connector.mandatory ? 1 : 0,
        connector.port?.guid || null,
        connector.description || null,
        type.guid,
      ]);

      toArray(connector.props).forEach((prop) => {
        const quality = toArray(kit.qualities).find((q) => q.guid === prop.quality.guid);
        const propKey = quality?.key || "";
        db.run("INSERT INTO prop (guid, key, value, unit, quality_guid, connector_guid) VALUES (?, ?, ?, ?, ?, ?)", [prop.guid, propKey, prop.value, prop.unit || null, prop.quality.guid, connector.guid]);
        toArray(prop.attributes).forEach((attr) => {
          db.run("INSERT INTO attribute (guid, key, value, definition, prop_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, prop.guid]);
        });
      });

      toArray(connector.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, connector_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, connector.guid]);
      });
    });

    toArray(type.props).forEach((prop) => {
      const quality = toArray(kit.qualities).find((q) => q.guid === prop.quality.guid);
      const propKey = quality?.key || "";
      db.run("INSERT INTO prop (guid, key, value, unit, quality_guid) VALUES (?, ?, ?, ?, ?)", [prop.guid, propKey, prop.value, prop.unit || null, prop.quality.guid]);
      db.run("INSERT INTO type_prop (type_guid, prop_guid) VALUES (?, ?)", [type.guid, prop.guid]);
      toArray(prop.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, prop_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, prop.guid]);
      });
    });

    toArray(type.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, type_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, type.guid]);
    });
  });

  toArray(kit.designs).forEach((design) => {
    db.run("INSERT INTO design (guid, name, parent_guid, unit, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
      design.guid,
      design.name,
      design.parent?.guid || null,
      design.unit || null,
      design.isAbstract ? 1 : null,
      design.folder || null,
      design.canScale ? 1 : null,
      design.canMirror ? 1 : null,
      design.description || null,
      design.icon || null,
      design.image || null,
      toISOString(design.createdAt),
      toISOString(design.updatedAt),
      kit.guid,
    ]);

    toArray(design.concepts).forEach((concept) => {
      db.run("INSERT INTO design_concept (design_guid, concept) VALUES (?, ?)", [design.guid, concept]);
    });

    toArray(design.props).forEach((prop) => {
      db.run("INSERT INTO design_prop (guid, design_guid, quality_guid, value, unit) VALUES (?, ?, ?, ?, ?)", [prop.guid, design.guid, prop.quality.guid, parseFloat(prop.value), prop.unit || null]);
    });

    toArray(design.authors).forEach((authorId, index) => {
      db.run("INSERT INTO design_author (design_guid, author_guid, rank) VALUES (?, ?, ?)", [design.guid, typeof authorId === "object" ? authorId.guid : authorId, index]);
    });

    toArray(design.layers).forEach((layer) => {
      db.run("INSERT INTO layer (guid, path, is_hidden, is_locked, color, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?)", [
        layer.guid,
        layer.path,
        layer.isHidden ? 1 : 0,
        layer.isLocked ? 1 : 0,
        layer.color || null,
        layer.description || null,
        design.guid,
      ]);

      toArray(layer.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, layer_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, layer.guid]);
      });
    });

    toArray(design.pieces).forEach((piece) => {
      db.run(
        "INSERT INTO piece (guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z, plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z, center_u, center_v, scale, mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z, mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z, mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z, is_hidden, is_locked, color, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
          piece.guid,
          piece.name || null,
          piece.type?.guid || null,
          piece.design?.guid || null,
          piece.plane?.origin.x !== undefined ? piece.plane.origin.x : null,
          piece.plane?.origin.y !== undefined ? piece.plane.origin.y : null,
          piece.plane?.origin.z !== undefined ? piece.plane.origin.z : null,
          piece.plane?.xAxis.x !== undefined ? piece.plane.xAxis.x : null,
          piece.plane?.xAxis.y !== undefined ? piece.plane.xAxis.y : null,
          piece.plane?.xAxis.z !== undefined ? piece.plane.xAxis.z : null,
          piece.plane?.yAxis.x !== undefined ? piece.plane.yAxis.x : null,
          piece.plane?.yAxis.y !== undefined ? piece.plane.yAxis.y : null,
          piece.plane?.yAxis.z !== undefined ? piece.plane.yAxis.z : null,
          piece.center?.u !== undefined ? piece.center.u : null,
          piece.center?.v !== undefined ? piece.center.v : null,
          piece.scale !== undefined ? piece.scale : null,
          piece.mirrorPlane?.origin.x !== undefined ? piece.mirrorPlane.origin.x : null,
          piece.mirrorPlane?.origin.y !== undefined ? piece.mirrorPlane.origin.y : null,
          piece.mirrorPlane?.origin.z !== undefined ? piece.mirrorPlane.origin.z : null,
          piece.mirrorPlane?.xAxis.x !== undefined ? piece.mirrorPlane.xAxis.x : null,
          piece.mirrorPlane?.xAxis.y !== undefined ? piece.mirrorPlane.xAxis.y : null,
          piece.mirrorPlane?.xAxis.z !== undefined ? piece.mirrorPlane.xAxis.z : null,
          piece.mirrorPlane?.yAxis.x !== undefined ? piece.mirrorPlane.yAxis.x : null,
          piece.mirrorPlane?.yAxis.y !== undefined ? piece.mirrorPlane.yAxis.y : null,
          piece.mirrorPlane?.yAxis.z !== undefined ? piece.mirrorPlane.yAxis.z : null,
          piece.isHidden ? 1 : 0,
          piece.isLocked ? 1 : 0,
          piece.color || null,
          piece.description || null,
          design.guid,
        ],
      );

      toArray(piece.props).forEach((prop) => {
        const quality = toArray(kit.qualities).find((q) => q.guid === prop.quality.guid);
        const propKey = quality?.key || "";
        db.run("INSERT INTO prop (guid, key, value, unit, quality_guid) VALUES (?, ?, ?, ?, ?)", [prop.guid, propKey, prop.value, prop.unit || null, prop.quality.guid]);
        db.run("INSERT INTO piece_prop (piece_guid, prop_guid) VALUES (?, ?)", [piece.guid, prop.guid]);
        toArray(prop.attributes).forEach((attr) => {
          db.run("INSERT INTO attribute (guid, key, value, definition, prop_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, prop.guid]);
        });
      });

      toArray(piece.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, piece_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, piece.guid]);
      });
    });

    toArray(design.groups).forEach((group) => {
      db.run('INSERT INTO "group" (guid, name, color, description, design_guid) VALUES (?, ?, ?, ?, ?)', [group.guid, group.name || null, group.color || null, group.description || null, design.guid]);

      toArray(group.pieces).forEach((piece) => {
        db.run("INSERT INTO group_piece (group_guid, piece_guid) VALUES (?, ?)", [group.guid, piece.guid]);
      });

      toArray(group.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, group_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, group.guid]);
      });
    });

    toArray(design.connections).forEach((connection) => {
      if (!connection.guid || !connection.connected?.piece?.guid || !connection.connecting?.piece?.guid || !connection.connected?.connector?.guid || !connection.connecting?.connector?.guid) {
        return;
      }

      db.run(
        "INSERT INTO connection (guid, connected_piece_guid, connected_design_piece_guid, connected_connector_guid, connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid, gap, shift, rise, rotation, turn, tilt, u, v, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
          connection.guid,
          connection.connected.piece.guid,
          connection.connected.designPiece?.guid || null,
          connection.connected.connector.guid,
          connection.connecting.piece.guid,
          connection.connecting.designPiece?.guid || null,
          connection.connecting.connector.guid,
          connection.gap || 0,
          connection.shift || 0,
          connection.rise || 0,
          connection.rotation || 0,
          connection.turn || 0,
          connection.tilt || 0,
          connection.u !== undefined ? connection.u : null,
          connection.v !== undefined ? connection.v : null,
          connection.description || null,
          design.guid,
        ],
      );

      toArray(connection.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, connection_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, connection.guid]);
      });
    });

    toArray(design.stats).forEach((stat) => {
      db.run("INSERT INTO stat (guid, quality_guid, min_value, min_excluded, max_value, max_excluded, unit, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", [
        stat.guid,
        stat.quality.guid,
        stat.min || null,
        stat.minExcluded ? 1 : null,
        stat.max || null,
        stat.maxExcluded ? 1 : null,
        stat.unit || null,
        design.guid,
      ]);
    });

    toArray(design.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, design_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, design.guid]);
    });
  });
};

// #endregion 🔖Kit Import/Export
// #region 🔖Kit Model Export

// [🔖semio/js/semio.ts#Kit Model Export](repo://section/semio/js/semio.ts/KIT%20MODEL%20EXPORT)
// Design model export to 3D formats (GLB, glTF, OBJ, STL, PLY, USDZ) MUST be defined here.

/**
 * Supported 3D export formats with their MIME types.
 * [👤semio📚js💻semio🔖kit🔖kitmodelexport🪨exportmodelformats](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Model%20Export/d/c/EXPORT_MODEL_FORMATS)
 **/
export const EXPORT_MODEL_FORMATS: Record<string, string> = {
  ".glb": "model/gltf-binary",
  ".gltf": "model/gltf+json",
  ".obj": "model/obj",
  ".stl": "model/stl",
  ".ply": "application/x-ply",
  ".usdz": "model/vnd.usdz+zip",
};

const SEMIO_TO_GLTF_BASIS = new THREE.Matrix4().set(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);

const SEMIO_TO_GLTF_BASIS_INV = SEMIO_TO_GLTF_BASIS.clone().invert();

const semioMatrixToGltfMatrix = (matrix: THREE.Matrix4): number[] => {
  const transformed = new THREE.Matrix4().multiplyMatrices(SEMIO_TO_GLTF_BASIS, matrix).multiply(SEMIO_TO_GLTF_BASIS_INV);
  return transformed.elements.slice();
};

// [👤semio📚js💻semio🔖kit🔖kitmodelexport🪨planetoglbtransform](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Model%20Export/d/i/planeToGlbTransform)
const planeToGlbTransform = (plane: Plane): number[] => {
  return semioMatrixToGltfMatrix(planeToMatrix(plane));
};

// [👤semio📚js💻semio🔖kit🔖kitmodelexport🪨findmatchingmodel](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Model%20Export/d/i/findMatchingModel)
const findMatchingModel = (kit: Kit, type: Type, tags: string[]): Model | undefined => {
  if (!type.models || type.models.length === 0) return undefined;
  const kitTags = kit.tags ?? [];
  const selectedTagGuids = tags.flatMap((tagValue) => {
    const byGuid = kitTags.find((tag) => tag.guid === tagValue);
    if (byGuid) return [byGuid.guid];
    return kitTags.filter((tag) => tag.name === tagValue).map((tag) => tag.guid);
  });
  return selectBestModel(type.models, selectedTagGuids);
};

/**
 * Decodes a base64 or data-URI blob string into a Uint8Array.
 * MUST handle both raw base64 and data-URI prefixed strings.
 * [👤semio📚js💻semio🔖kit🔖kitmodelexport🪨decodeblobtobytes](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Model%20Export/d/i/decodeBlobToBytes)
 **/
const decodeBlobToBytes = (blobStr: string): Uint8Array => {
  const base64 = blobStr.startsWith("data:") ? blobStr.slice(blobStr.indexOf(",") + 1) : blobStr;
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
  return bytes;
};

const bytesToDataUri = (bytes: Uint8Array, mimeType: string): string => {
  const base64 = Buffer.from(bytes).toString("base64");
  return `data:${mimeType};base64,${base64}`;
};

const inlineJsonDocumentResources = (jsonDoc: {
  json: {
    buffers?: Array<{ uri?: string }>;
    images?: Array<{ uri?: string; mimeType?: string }>;
  };
  resources?: Record<string, Uint8Array | string>;
}) => {
  const resources = jsonDoc.resources ?? {};
  for (const buffer of jsonDoc.json.buffers ?? []) {
    if (!buffer.uri) continue;
    const resource = resources[buffer.uri];
    if (!resource) continue;
    const bytes = typeof resource === "string" ? new TextEncoder().encode(resource) : resource;
    buffer.uri = bytesToDataUri(bytes, "application/octet-stream");
  }
  for (const image of jsonDoc.json.images ?? []) {
    if (!image.uri) continue;
    const resource = resources[image.uri];
    if (!resource) continue;
    const bytes = typeof resource === "string" ? new TextEncoder().encode(resource) : resource;
    image.uri = bytesToDataUri(bytes, image.mimeType ?? "application/octet-stream");
  }
  return jsonDoc.json;
};

/**
 * Copies a texture from a source glTF document into a target document, using a cache to avoid duplicates.
 * MUST preserve image data and MIME type.
 * [👤semio📚js💻semio🔖kit🔖kitmodelexport🪨copygltftexture](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Model%20Export/d/i/copyGltfTexture)
 **/
const copyGltfTexture = (srcTex: GltfTexture, targetDoc: GltfDocument, textureCache: Map<GltfTexture, GltfTexture>): GltfTexture => {
  const cached = textureCache.get(srcTex);
  if (cached) return cached;
  const tex = targetDoc.createTexture(srcTex.getName());
  const img = srcTex.getImage();
  if (img) tex.setImage(new Uint8Array(img));
  tex.setMimeType(srcTex.getMimeType());
  tex.setURI(srcTex.getURI());
  textureCache.set(srcTex, tex);
  return tex;
};

/**
 * Copies a material from a source glTF document into a target document, including referenced textures.
 * MUST preserve PBR properties, alpha mode, double-sidedness, and all texture references.
 * [👤semio📚js💻semio🔖kit🔖kitmodelexport🪨copygltfmaterial](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Model%20Export/d/i/copyGltfMaterial)
 **/
const copyGltfMaterial = (srcMat: GltfMaterial, targetDoc: GltfDocument, textureCache: Map<GltfTexture, GltfTexture>): GltfMaterial => {
  const mat = targetDoc.createMaterial(srcMat.getName());
  mat.setBaseColorFactor(srcMat.getBaseColorFactor());
  mat.setMetallicFactor(srcMat.getMetallicFactor());
  mat.setRoughnessFactor(srcMat.getRoughnessFactor());
  mat.setEmissiveFactor(srcMat.getEmissiveFactor());
  mat.setAlphaMode(srcMat.getAlphaMode());
  mat.setAlphaCutoff(srcMat.getAlphaCutoff());
  mat.setDoubleSided(srcMat.getDoubleSided());

  const baseColorTex = srcMat.getBaseColorTexture();
  if (baseColorTex) mat.setBaseColorTexture(copyGltfTexture(baseColorTex, targetDoc, textureCache));
  const mrTex = srcMat.getMetallicRoughnessTexture();
  if (mrTex) mat.setMetallicRoughnessTexture(copyGltfTexture(mrTex, targetDoc, textureCache));
  const normalTex = srcMat.getNormalTexture();
  if (normalTex) mat.setNormalTexture(copyGltfTexture(normalTex, targetDoc, textureCache));
  const occlusionTex = srcMat.getOcclusionTexture();
  if (occlusionTex) mat.setOcclusionTexture(copyGltfTexture(occlusionTex, targetDoc, textureCache));
  const emissiveTex = srcMat.getEmissiveTexture();
  if (emissiveTex) mat.setEmissiveTexture(copyGltfTexture(emissiveTex, targetDoc, textureCache));

  return mat;
};

/**
 * Copies all meshes from a source glTF document into a target document, returning the list of copied meshes.
 * MUST deep-copy accessor data, materials, and textures into the target document's buffer.
 * [👤semio📚js💻semio🔖kit🔖kitmodelexport🪨copygltfmeshes](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Model%20Export/d/i/copyGltfMeshes)
 **/
const copyGltfMeshes = (sourceDoc: GltfDocument, targetDoc: GltfDocument, targetBuffer: GltfBuffer, meshName?: string): GltfMesh[] => {
  const materialCache = new Map<GltfMaterial, GltfMaterial>();
  const textureCache = new Map<GltfTexture, GltfTexture>();
  const mesh = targetDoc.createMesh(meshName);

  for (const srcMesh of sourceDoc.getRoot().listMeshes()) {
    for (const srcPrim of srcMesh.listPrimitives()) {
      const prim = targetDoc.createPrimitive();

      for (const semantic of srcPrim.listSemantics()) {
        const srcAcc = srcPrim.getAttribute(semantic);
        if (!srcAcc) continue;
        const srcArray = srcAcc.getArray();
        if (!srcArray) continue;
        const acc = targetDoc
          .createAccessor(semantic)
          .setArray(srcArray.slice() as any)
          .setType(srcAcc.getType())
          .setBuffer(targetBuffer);
        if (srcAcc.getNormalized()) acc.setNormalized(true);
        prim.setAttribute(semantic, acc);
      }

      const srcIndices = srcPrim.getIndices();
      if (srcIndices) {
        const srcIdxArray = srcIndices.getArray();
        if (srcIdxArray) {
          const idxAcc = targetDoc
            .createAccessor("indices")
            .setArray(srcIdxArray.slice() as any)
            .setType(GltfAccessor.Type.SCALAR)
            .setBuffer(targetBuffer);
          prim.setIndices(idxAcc);
        }
      }

      const srcMat = srcPrim.getMaterial();
      if (srcMat) {
        let mat = materialCache.get(srcMat);
        if (!mat) {
          mat = copyGltfMaterial(srcMat, targetDoc, textureCache);
          materialCache.set(srcMat, mat);
        }
        prim.setMaterial(mat);
      }

      mesh.addPrimitive(prim);
    }
  }

  return mesh.listPrimitives().length > 0 ? [mesh] : [];
};

/**
 * Creates a unit box mesh (1x1x1 centered at origin) as a placeholder for types without models.
 * MUST produce 24 vertices (4 per face), 36 indices (12 triangles), with normals.
 * [👤semio📚js💻semio🔖kit🔖kitmodelexport🪨createboxmesh](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Model%20Export/d/i/createBoxMesh)
 **/
const createBoxMesh = (name: string, doc: GltfDocument, buffer: GltfBuffer): GltfMesh => {
  const s = 0.5;
  const positions = new Float32Array([
    -s,
    -s,
    s,
    s,
    -s,
    s,
    s,
    s,
    s,
    -s,
    s,
    s,
    -s,
    -s,
    -s,
    -s,
    s,
    -s,
    s,
    s,
    -s,
    s,
    -s,
    -s,
    -s,
    s,
    -s,
    -s,
    s,
    s,
    s,
    s,
    s,
    s,
    s,
    -s,
    -s,
    -s,
    -s,
    s,
    -s,
    -s,
    s,
    -s,
    s,
    -s,
    -s,
    s,
    s,
    -s,
    -s,
    s,
    s,
    -s,
    s,
    s,
    s,
    s,
    -s,
    s,
    -s,
    -s,
    -s,
    -s,
    -s,
    s,
    -s,
    s,
    s,
    -s,
    s,
    -s,
  ]);
  const normals = new Float32Array([
    0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0,
  ]);
  const indices = new Uint16Array([0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23]);

  const posAcc = doc.createAccessor("POSITION").setArray(positions).setType(GltfAccessor.Type.VEC3).setBuffer(buffer);
  const normAcc = doc.createAccessor("NORMAL").setArray(normals).setType(GltfAccessor.Type.VEC3).setBuffer(buffer);
  const idxAcc = doc.createAccessor("indices").setArray(indices).setType(GltfAccessor.Type.SCALAR).setBuffer(buffer);

  const prim = doc.createPrimitive().setAttribute("POSITION", posAcc).setAttribute("NORMAL", normAcc).setIndices(idxAcc);

  return doc.createMesh(name).addPrimitive(prim);
};

/**
 * Exports the 3D model of a design to a specified format.
 * MUST produce a valid 3D file. Uses block definitions for types and instances for pieces.
 * Connection hierarchy is translated into a scene graph; planes become relative transformation matrices.
 * [👤semio📚js💻semio🔖kit🔖kitmodelexport🪨exportdesignmodel](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Kit%20Model%20Export/d/i/exportDesignModel)
 **/
export const exportDesignModel = async (kit: Kit, designId: string, format: string = ".glb", tags: string[] = [], options: Record<string, unknown> = {}): Promise<ArrayBuffer> => {
  const io = new NodeIO();
  const design = findDesignInKit(kit, designId);
  const pieces = design.pieces ?? [];
  const connections = design.connections ?? [];
  const types = kit.types ?? [];

  if (pieces.length === 0) {
    const emptyDoc = new GltfDocument();
    emptyDoc.createBuffer("main");
    emptyDoc.createScene("empty");
    const glb = await io.writeBinary(emptyDoc);
    return glb.buffer as ArrayBuffer;
  }

  const typesDict: Record<string, Type> = {};
  for (const t of types) typesDict[t.guid] = t;
  const piecesDict: Record<string, Piece> = {};
  for (const p of pieces) piecesDict[p.guid] = p;

  const adjacency: Record<string, Array<{ connection: Connection; neighborGuid: string }>> = {};
  for (const p of pieces) adjacency[p.guid] = [];
  for (const conn of connections) {
    const connectedGuid = conn.connected.piece.guid;
    const connectingGuid = conn.connecting.piece.guid;
    if (adjacency[connectedGuid]) adjacency[connectedGuid].push({ connection: conn, neighborGuid: connectingGuid });
    if (adjacency[connectingGuid]) adjacency[connectingGuid].push({ connection: conn, neighborGuid: connectedGuid });
  }

  const piecePlanes: Record<string, Plane> = {};
  const parentOf: Record<string, string> = {};
  const childrenOf: Record<string, string[]> = {};
  for (const p of pieces) childrenOf[p.guid] = [];

  const visited = new Set<string>();
  const roots: string[] = [];

  const getType = (typeGuid: string): Type | undefined => typesDict[typeGuid];
  const getConnector = (type: Type | undefined, connectorGuid: string | undefined): Connector | undefined => {
    if (!type) return undefined;
    if (!connectorGuid) return type.connectors?.[0];
    return type.connectors?.find((c) => c.guid === connectorGuid);
  };

  const queue: string[] = [];
  for (const p of pieces) {
    if (p.plane) {
      piecePlanes[p.guid] = p.plane;
      visited.add(p.guid);
      queue.push(p.guid);
      roots.push(p.guid);
    }
  }
  if (queue.length === 0 && pieces.length > 0) {
    const firstPiece = pieces[0];
    const identityPlane = matrixToPlane(new THREE.Matrix4().identity());
    piecePlanes[firstPiece.guid] = identityPlane;
    visited.add(firstPiece.guid);
    queue.push(firstPiece.guid);
    roots.push(firstPiece.guid);
  }

  while (queue.length > 0) {
    const currentGuid = queue.shift()!;
    const currentPlane = piecePlanes[currentGuid];
    for (const edge of adjacency[currentGuid] ?? []) {
      if (visited.has(edge.neighborGuid)) continue;
      const conn = edge.connection;
      const isParent = conn.connected.piece.guid === currentGuid;

      if (!isParent) continue;

      const parentGuid = currentGuid;
      const childGuid = edge.neighborGuid;
      const parentPiece = piecesDict[parentGuid];
      const childPiece = piecesDict[childGuid];
      const parentType = parentPiece.type ? getType(parentPiece.type.guid) : undefined;
      const childType = childPiece.type ? getType(childPiece.type.guid) : undefined;
      const parentConnector = getConnector(parentType, conn.connected.connector?.guid);
      const childConnector = getConnector(childType, conn.connecting.connector?.guid);

      if (parentConnector && childConnector) {
        const childPlane = computeChildPlane(currentPlane, parentConnector, childConnector, conn);
        piecePlanes[childGuid] = childPlane;
      } else {
        piecePlanes[childGuid] = currentPlane;
      }

      parentOf[childGuid] = parentGuid;
      childrenOf[parentGuid].push(childGuid);
      visited.add(childGuid);
      queue.push(childGuid);
    }
  }

  for (const p of pieces) {
    if (!visited.has(p.guid)) {
      piecePlanes[p.guid] = matrixToPlane(new THREE.Matrix4().identity());
      roots.push(p.guid);
    }
  }

  const doc = new GltfDocument();
  const buffer = doc.createBuffer("main");
  const scene = doc.createScene(design.name ?? "design");

  const typeMeshMap: Record<string, GltfMesh> = {};

  for (const piece of pieces) {
    const typeGuid = piece.type?.guid;
    if (!typeGuid || typeMeshMap[typeGuid] !== undefined) continue;

    const type = typesDict[typeGuid];
    if (!type) continue;

    const model = findMatchingModel(kit, type, tags);
    if (!model) {
      continue;
    }

    const file = kit.files?.find((f) => f.guid === model.file.guid);
    if (!file?.blob) continue;

    const fileBytes = decodeBlobToBytes(file.blob);
    const ext = file.name.split(".").pop()?.toLowerCase();

    if (ext === "glb") {
      try {
        const sourceDoc = await io.readBinary(fileBytes);
        const copiedMeshes = copyGltfMeshes(sourceDoc, doc, buffer, file.name);
        if (copiedMeshes.length > 0) {
          typeMeshMap[typeGuid] = copiedMeshes[0];
        }
      } catch { }
    }
  }

  const pieceNodeMap: Record<string, GltfNode> = {};

  const buildNode = (pieceGuid: string): GltfNode => {
    if (pieceNodeMap[pieceGuid]) return pieceNodeMap[pieceGuid];

    const piece = piecesDict[pieceGuid];
    const worldPlane = piecePlanes[pieceGuid];
    const parentGuid = parentOf[pieceGuid];
    const children = childrenOf[pieceGuid] ?? [];

    let localMatrix: number[];
    if (parentGuid && piecePlanes[parentGuid]) {
      const parentWorld = planeToMatrix(piecePlanes[parentGuid]);
      const childWorld = planeToMatrix(worldPlane);
      const invParent = parentWorld.clone().invert();
      const localMat = new THREE.Matrix4().multiplyMatrices(invParent, childWorld);
      localMatrix = semioMatrixToGltfMatrix(localMat);
    } else {
      localMatrix = planeToGlbTransform(worldPlane);
    }

    const node = doc.createNode(piece.name ?? piece.guid);
    node.setMatrix(localMatrix as any);

    const typeGuid = piece.type?.guid;
    if (typeGuid && typeMeshMap[typeGuid]) {
      node.setMesh(typeMeshMap[typeGuid]);
    }

    for (const childGuid of children) {
      node.addChild(buildNode(childGuid));
    }

    pieceNodeMap[pieceGuid] = node;
    return node;
  };

  for (const rootGuid of roots) {
    scene.addChild(buildNode(rootGuid));
  }

  if (format === ".gltf") {
    const jsonDoc = await io.writeJSON(doc);
    const encoder = new TextEncoder();
    return encoder.encode(JSON.stringify(inlineJsonDocumentResources(jsonDoc))).buffer as ArrayBuffer;
  }

  const glb = await io.writeBinary(doc);
  return glb.buffer as ArrayBuffer;
};

// #endregion 🔖Kit Model Export
// #region 🔖Geometric Insights
// [👤semio📚js💻semio🔖geometricinsights](repo://p/u/semio/b/l/js/f/semio.ts/s/Geometric%20Insights)
// Key performance indicators for GLB/GLTF model geometry. Model MUST be glb/gltf.

/**
 * Geometric KPIs for a GLB/GLTF model in semio coordinate system (semio x=glb x, semio y=-glb x, semio z=glb y).
 * [👤semio📚js💻semio🔖geometricinsights🪨geometricinsights](repo://p/u/semio/b/l/js/f/semio.ts/s/Geometric%20Insights/d/i/GeometricInsights)
 */
export interface GeometricInsights {
  boundingBoxMin?: Point;
  boundingBoxMax?: Point;
  dimensionX?: number;
  dimensionY?: number;
  dimensionZ?: number;
  characteristicLength?: number;
  footprintArea?: number;
  totalSurfaceArea?: number;
  enclosedVolume?: number;
  surfaceToVolumeRatio?: number;
  sphericity?: number;
  hullFillRatio?: number;
  aspectRatioXy?: number;
  aspectRatioXz?: number;
  aspectRatioYz?: number;
  slenderness?: number;
  centroid?: Point;
  principalAxes?: [Point, Point, Point];
  momentsOfInertia?: [number, number, number];
  vertexCount?: number;
  faceCount?: number;
  eulerCharacteristic?: number;
  genus?: number;
  isWatertight?: boolean;
  convexHullVolume?: number;
  concavityIndex?: number;
}

function glbToSemioPoint(xg: number, yg: number, _zg: number): Point {
  return { x: xg, y: -xg, z: yg };
}

function triangleArea(a: THREE.Vector3, b: THREE.Vector3, c: THREE.Vector3): number {
  return 0.5 * new THREE.Vector3().crossVectors(new THREE.Vector3(b.x - a.x, b.y - a.y, b.z - a.z), new THREE.Vector3(c.x - a.x, c.y - a.y, c.z - a.z)).length();
}

function signedTetrahedronVolume(o: THREE.Vector3, a: THREE.Vector3, b: THREE.Vector3, c: THREE.Vector3): number {
  return (1 / 6) * new THREE.Vector3().crossVectors(new THREE.Vector3(a.x - o.x, a.y - o.y, a.z - o.z), new THREE.Vector3(b.x - o.x, b.y - o.y, b.z - o.z)).dot(new THREE.Vector3(c.x - o.x, c.y - o.y, c.z - o.z));
}

/**
 * Computes key performance indicators for the geometry of a GLB/GLTF model.
 * Model MUST be glb or gltf (path, URL, or raw bytes). Uses @gltf-transform and THREE for mesh analysis.
 * [👤semio📚js💻semio🔖geometricinsights🛠️getgeometricinsightsformodel](repo://p/u/semio/b/l/js/f/semio.ts/s/Geometric%20Insights/d/i/getGeometricInsightsForModel)
 */
export const getGeometricInsightsForModel = async (model: string | ArrayBuffer | Uint8Array): Promise<GeometricInsights> => {
  const io = new NodeIO();
  let doc: GltfDocument;

  if (typeof model === "string") {
    if (model.startsWith("data:")) {
      const base64 = model.slice(model.indexOf(",") + 1);
      const binary = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
      doc = await io.readBinary(binary);
    } else {
      let arrBuf: ArrayBuffer;
      const isPath = !model.startsWith("http://") && !model.startsWith("https://") && (model.endsWith(".glb") || model.endsWith(".gltf") || model.includes("/") || model.includes("\\"));
      if (typeof globalThis !== "undefined" && "process" in globalThis && typeof (globalThis as any).process?.versions?.node === "string" && isPath) {
        const { readFileSync } = await import("node:fs");
        const { dirname, join } = await import("node:path");
        const dir = dirname(model);
        if (model.endsWith(".gltf")) {
          const raw = readFileSync(model, "utf8");
          const json = JSON.parse(raw) as { buffers?: Array<{ uri?: string }>; images?: Array<{ uri?: string }> };
          const resources: Record<string, Uint8Array<ArrayBuffer>> = {};
          const addResource = (uri: string | undefined) => {
            if (!uri) return;
            if (uri.startsWith("data:")) {
              const base64 = uri.slice(uri.indexOf(",") + 1);
              resources[uri] = new Uint8Array(Buffer.from(base64, "base64"));
              return;
            }
            try {
              const binPath = join(dir, uri);
              resources[uri] = new Uint8Array(readFileSync(binPath));
            } catch {
              // skip missing external buffer
            }
          };
          for (const b of json.buffers ?? []) addResource(b.uri);
          for (const img of json.images ?? []) addResource(img.uri);
          doc = await io.readJSON({ json: json as any, resources });
        } else {
          const buf = readFileSync(model);
          arrBuf = buf.buffer.slice(buf.byteOffset, buf.byteOffset + buf.byteLength);
          doc = await io.readBinary(new Uint8Array(arrBuf));
        }
      } else {
        const res = await fetch(model);
        if (!res.ok) throw new Error(`Failed to load model: ${res.statusText}`);
        arrBuf = await res.arrayBuffer();
        const bytes = new Uint8Array(arrBuf);
        const isGlb = model.endsWith(".glb") || (bytes.length >= 4 && new TextDecoder().decode(bytes.slice(0, 4)) === "glTF");
        const base = model.replace(/\/[^/]*$/, "") || ".";
        doc = isGlb ? await io.readBinary(new Uint8Array(arrBuf)) : await io.readJSON({ json: JSON.parse(new TextDecoder().decode(new Uint8Array(arrBuf))), resources: {} });
      }
    }
  } else {
    const bytes = model instanceof Uint8Array ? model : new Uint8Array(model);
    const magic = bytes.length >= 4 ? new TextDecoder().decode(bytes.slice(0, 4)) : "";
    doc = magic === "glTF" ? await io.readBinary(bytes) : await io.readJSON({ json: JSON.parse(new TextDecoder().decode(bytes)), resources: {} });
  }

  const out: GeometricInsights = {};
  const box = new THREE.Box3();
  let totalArea = 0;
  let totalVolume = 0;
  let vertexCount = 0;
  let faceCount = 0;
  const centroidSum = { x: 0, y: 0, z: 0 };
  const origin = new THREE.Vector3(0, 0, 0);

  for (const mesh of doc.getRoot().listMeshes()) {
    for (const prim of mesh.listPrimitives()) {
      const posAcc = prim.getAttribute("POSITION");
      if (!posAcc) continue;
      const posArray = posAcc.getArray();
      if (!posArray || posArray.length < 3) continue;
      const count = posArray.length / 3;
      for (let i = 0; i < count; i++) {
        const xg = posArray[i * 3];
        const yg = posArray[i * 3 + 1];
        const zg = posArray[i * 3 + 2];
        const p = glbToSemioPoint(xg, yg, zg);
        box.expandByPoint(new THREE.Vector3(p.x, p.y, p.z));
        centroidSum.x += p.x;
        centroidSum.y += p.y;
        centroidSum.z += p.z;
      }
      vertexCount += count;
      const indices = prim.getIndices()?.getArray();
      const getVertex = (idx: number) => new THREE.Vector3(posArray[idx * 3], posArray[idx * 3 + 1], posArray[idx * 3 + 2]);
      if (indices) {
        for (let i = 0; i + 2 < indices.length; i += 3) {
          const a = getVertex(indices[i]);
          const b = getVertex(indices[i + 1]);
          const c = getVertex(indices[i + 2]);
          totalArea += triangleArea(a, b, c);
          totalVolume += signedTetrahedronVolume(origin, a, b, c);
          faceCount += 1;
        }
      } else {
        for (let i = 0; i + 2 < count; i += 3) {
          const a = getVertex(i);
          const b = getVertex(i + 1);
          const c = getVertex(i + 2);
          totalArea += triangleArea(a, b, c);
          totalVolume += signedTetrahedronVolume(origin, a, b, c);
          faceCount += 1;
        }
      }
    }
  }

  if (vertexCount === 0) return out;

  const min = box.min;
  const max = box.max;
  out.boundingBoxMin = { x: min.x, y: min.y, z: min.z };
  out.boundingBoxMax = { x: max.x, y: max.y, z: max.z };
  out.dimensionX = max.x - min.x;
  out.dimensionY = max.y - min.y;
  out.dimensionZ = max.z - min.z;
  const dimX = out.dimensionX ?? 0;
  const dimY = out.dimensionY ?? 0;
  const dimZ = out.dimensionZ ?? 0;
  out.characteristicLength = Math.cbrt(dimX * dimY * dimZ) || 0;
  out.footprintArea = dimX * dimZ;
  out.totalSurfaceArea = totalArea;
  out.vertexCount = vertexCount;
  out.faceCount = faceCount;
  out.centroid = {
    x: centroidSum.x / vertexCount,
    y: centroidSum.y / vertexCount,
    z: centroidSum.z / vertexCount,
  };
  totalVolume = Math.abs(totalVolume);
  if (totalVolume > 1e-20) {
    out.enclosedVolume = totalVolume;
    if (totalArea > 0) out.surfaceToVolumeRatio = totalArea / totalVolume;
    if (totalArea > 0) {
      const sph = (Math.PI ** (1 / 3) * (6 * totalVolume) ** (2 / 3)) / totalArea;
      out.sphericity = Math.min(1, Math.max(0, sph));
    }
  }
  if (dimY > 1e-10 && dimX > 1e-10) out.aspectRatioXy = dimX / dimY;
  if (dimZ > 1e-10 && dimX > 1e-10) out.aspectRatioXz = dimX / dimZ;
  if (dimZ > 1e-10 && dimY > 1e-10) out.aspectRatioYz = dimY / dimZ;
  const maxExtent = Math.max(dimX, dimY, dimZ);
  if (maxExtent > 1e-10 && totalArea > 0) {
    out.slenderness = maxExtent / Math.cbrt(totalArea * maxExtent);
  }
  out.eulerCharacteristic = Math.round(vertexCount - (3 * faceCount) / 2 + faceCount);
  return out;
};

// #endregion 🔖Geometric Insights
// #region 🔖Validation

// [🔖semio/js/semio.ts#Validation](repo://section/semio/js/semio.ts/VALIDATION)
// Kit validation engine and constraints MUST be defined here.

// #region 🔖Validation core types

// [👤semio📚js💻semio🔖kit🔖validation🔖validationcoretypes](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20core%20types)
// Core validation types and interfaces MUST be defined here.

/**
 * Enumeration of EntityKind values.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcoretypes🛠️entitykind](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20core%20types/d/i/EntityKind)
 **/
export type EntityKind = "Kit" | "Type" | "Design" | "Piece" | "Connection" | "Connector" | "Attribute" | "File" | "Folder" | "Quality" | "Port" | "Prop" | "Model" | "Layer" | "Group" | "Stat";

/**
 * Interface defining DomainLocation structure.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcoretypes🛠️domainlocation](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20core%20types/d/i/DomainLocation)
 **/
export interface DomainLocation {
  entityKind: EntityKind;
  entityGuid?: Guid;
  field?: string;
}

/**
 * Interface defining Fix structure.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcoretypes🛠️fix](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20core%20types/d/i/Fix)
 **/
export interface Fix {
  title: string;
  diff?: KitDiff;
}

/**
 * Interface defining Problem structure.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcoretypes🛠️problem](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20core%20types/d/i/Problem)
 **/
export interface Problem {
  constraintId: string;
  message: string;
  location: DomainLocation;
  relatedGuids?: Guid[];
  fixes: Fix[];
}

/**
 * Interface defining ValidationResult structure.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcoretypes🛠️validationresult](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20core%20types/d/i/ValidationResult)
 **/
export interface ValidationResult {
  problems: Problem[];
}

/**
 * Checks whether Errors condition holds.
 * MUST return true if the condition is met.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcoretypes🪨haserrors](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20core%20types/d/i/hasErrors)
 **/
export const hasErrors = (res: ValidationResult) => res.problems.length > 0;

// #endregion 🔖Validation core types

// #region 🔖Validation context & engine

// [👤semio📚js💻semio🔖kit🔖validation🔖validationcontextengine](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20context%20&%20engine)
// Validation context construction and engine MUST be defined here.

/**
 * Interface defining ValidationContext structure.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcontextengine🛠️validationcontext](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20context%20&%20engine/d/i/ValidationContext)
 **/
export interface ValidationContext {
  kit: Kit;
  typesByGuid: Map<Guid, Type>;
  designsByGuid: Map<Guid, Design>;
  piecesByGuid: Map<Guid, { designGuid: Guid; piece: Piece }>;
  connectorsByTypeGuid: Map<Guid, Connector[]>;
  modelsByTypeGuid: Map<Guid, Model[]>;
}

/**
 * Constructs ValidationContext from components.
 * MUST construct and return a complete structure.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcontextengine🪨buildvalidationcontext](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20context%20&%20engine/d/i/buildValidationContext)
 **/
export const buildValidationContext = (kit: Kit): ValidationContext => {
  const typesByGuid = new Map<Guid, Type>();
  const designsByGuid = new Map<Guid, Design>();
  const piecesByGuid = new Map<Guid, { designGuid: Guid; piece: Piece }>();
  const connectorsByTypeGuid = new Map<Guid, Connector[]>();
  const modelsByTypeGuid = new Map<Guid, Model[]>();
  toArray(kit.types).forEach((t) => {
    typesByGuid.set(t.guid, t);
    connectorsByTypeGuid.set(t.guid, toArray(t.connectors));
    modelsByTypeGuid.set(t.guid, toArray(t.models));
  });
  toArray(kit.designs).forEach((d) => {
    designsByGuid.set(d.guid, d);
    toArray(d.pieces).forEach((p) => piecesByGuid.set(p.guid, { designGuid: d.guid, piece: p }));
  });
  return { kit, typesByGuid, designsByGuid, piecesByGuid, connectorsByTypeGuid, modelsByTypeGuid };
};

/**
 * Type alias for Constraint.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcontextengine🛠️constraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20context%20&%20engine/d/i/Constraint)
 **/
export type Constraint = (ctx: ValidationContext) => Problem[];

/**
 * Interface defining ValidationConfig structure.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcontextengine🛠️validationconfig](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20context%20&%20engine/d/i/ValidationConfig)
 **/
export interface ValidationConfig {
  constraints?: Constraint[];
}

/**
 * Definition of defaultConstraints.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcontextengine🛠️defaultconstraints](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20context%20&%20engine/d/i/defaultConstraints)
 **/
export let defaultConstraints: Constraint[] = [];

/**
 * Validates Kit against constraints.
 * MUST check all constraints and return problems.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationcontextengine🪨validatekit](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20context%20&%20engine/d/i/validateKit)
 **/
export const validateKit = (kit: Kit, cfg: ValidationConfig = {}): ValidationResult => {
  const ctx = buildValidationContext(kit);
  const constraints = cfg.constraints ?? defaultConstraints;
  return { problems: constraints.flatMap((constraint) => constraint(ctx)) };
};

// #endregion 🔖Validation context & engine

// #region 🔖Fix helper

// [👤semio📚js💻semio🔖kit🔖validation🔖fixhelper](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Fix%20helper)
// Validation fix helper functions MUST be defined here.
// [👤semio📚js💻semio🔖kit🔖validation🔖fixhelper](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Fix%20helper)
// Validation fix helper functions MUST be defined here.

/**
 * Performs the semioMakeFix operation.
 * MUST produce a Fix that regenerates the GUID.
 * [👤semio📚js💻semio🔖kit🔖validation🔖fixhelper🪨semiomakefix](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Fix%20helper/d/i/semioMakeFix)
 **/
export const semioMakeFix = (ctx: ValidationContext, title: string, mutate: (clone: Kit) => void): Fix => {
  const clone = JSON.parse(serializeKit(ctx.kit)) as Kit;
  mutate(clone);
  const diff = getKitDiff(ctx.kit, clone);
  return { title, diff };
};

// #endregion 🔖Fix helper

// #region 🔖GUID update helper

// [👤semio📚js💻semio🔖kit🔖validation🔖guidupdatehelper](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/GUID%20update%20helper)
// GUID regeneration helper functions MUST be defined here.

// updateGuidEverywhere holds the data fields for a updateGuidEverywhere record.
// [👤semio📚js💻semio🔖kit🔖validation🔖guidupdatehelper🪨updateguideverywhere](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/GUID%20update%20helper/d/i/updateGuidEverywhere)
const updateGuidEverywhere = (kit: Kit, oldGuid: Guid, newGuid: Guid): void => {
  const update = (obj: any) => {
    if (!obj || typeof obj !== "object") return;
    if (obj.guid === oldGuid) obj.guid = newGuid;
    if (obj.parent?.guid === oldGuid) obj.parent = createTypeId(newGuid);
    if (obj.type?.guid === oldGuid) obj.type = createTypeId(newGuid);
    if (obj.design?.guid === oldGuid) obj.design = createDesignId(newGuid);
    if (obj.port?.guid === oldGuid) obj.port = createPortId(newGuid);
    if (obj.quality?.guid === oldGuid) obj.quality = createQualityId(newGuid);
    if (obj.piece?.guid === oldGuid) obj.piece = createPieceId(newGuid);
    if (obj.connector?.guid === oldGuid) obj.connector = createConnectorId(newGuid);
    if (Array.isArray(obj.compatiblePorts)) {
      obj.compatiblePorts = obj.compatiblePorts.map((iid: PortId) => (iid.guid === oldGuid ? createPortId(newGuid) : iid));
    }
    if (Array.isArray(obj.pieces)) {
      obj.pieces = obj.pieces.map((p: PieceId) => (p.guid === oldGuid ? createPieceId(newGuid) : p));
    }
    for (const key in obj) {
      if (Array.isArray(obj[key])) {
        obj[key].forEach(update);
      } else if (typeof obj[key] === "object") {
        update(obj[key]);
      }
    }
  };
  update(kit);
};

// #endregion 🔖GUID update helper

// #region 🔖Constraint: GUID uniqueness

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintguiduniqueness](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20GUID%20uniqueness)
// GUID uniqueness constraint MUST be enforced here.

/**
 * Constraint validating GuidUniqueness rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constraintguiduniqueness🪨semioguiduniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20GUID%20uniqueness/d/i/semioGuidUniquenessConstraint)
 **/
export const semioGuidUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const seen = new Map<Guid, EntityKind>();
  const check = (entityKind: EntityKind, entityGuid: Guid) => {
    const existing = seen.get(entityGuid);
    if (!existing) {
      seen.set(entityGuid, entityKind);
      return;
    }
    const problem: Problem = {
      constraintId: "guid-unique",
      message: `Duplicate GUID "${entityGuid}". First occurrence kept.`,
      location: { entityKind, entityGuid, field: "guid" },
      relatedGuids: [entityGuid],
      fixes: [
        semioMakeFix(ctx, "Regenerate GUID", (clone) => {
          const newGuid = guid();
          updateGuidEverywhere(clone, entityGuid, newGuid);
        }),
      ],
    };
    problems.push(problem);
  };
  check("Kit", ctx.kit.guid);
  toArray(ctx.kit.types).forEach((t) => check("Type", t.guid));
  toArray(ctx.kit.designs).forEach((d) => {
    check("Design", d.guid);
    toArray(d.pieces).forEach((p) => check("Piece", p.guid));
    toArray(d.connections).forEach((c) => check("Connection", c.guid));
    toArray(d.stats).forEach((s) => check("Stat", s.guid));
  });
  toArray(ctx.kit.qualities).forEach((q) => check("Quality", q.guid));
  toArray(ctx.kit.ports).forEach((i) => check("Port", i.guid));
  toArray(ctx.kit.files).forEach((f) => check("File", f.guid));
  toArray(ctx.kit.folders).forEach((f) => check("Folder", f.guid));
  return problems;
};

// #endregion 🔖Constraint: GUID uniqueness

// #region 🔖Constraint: Type name uniqueness

// [👤semio📚js💻semio🔖kit🔖validation🔖constrainttypenameuniqueness](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Type%20name%20uniqueness)
// Type name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating TypeNameUniqueness rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constrainttypenameuniqueness🪨semiotypenameuniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Type%20name%20uniqueness/d/i/semioTypeNameUniquenessConstraint)
 **/
export const semioTypeNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const byParent = new Map<Guid | undefined, Type[]>();
  toArray(ctx.kit.types).forEach((t) => {
    const pid = t.parent?.guid as Guid | undefined;
    if (!byParent.has(pid)) byParent.set(pid, []);
    byParent.get(pid)!.push(t);
  });
  for (const [parentGuid, siblings] of byParent) {
    const names = new Map<string, Type[]>();
    siblings.forEach((t) => {
      const name = t.name ?? "";
      if (!names.has(name)) names.set(name, []);
      names.get(name)!.push(t);
    });
    for (const [name, group] of names) {
      if (group.length <= 1) continue;
      const [first, ...rest] = group;
      const siblingNames = siblings.map((s) => s.name ?? "");
      rest.forEach((type) => {
        const fix = semioMakeFix(ctx, `Rename "${name}"`, (clone) => {
          const ct = toArray(clone.types).find((x) => x.guid === type.guid);
          if (!ct) return;
          const newName = generateUniqueName(name, siblingNames);
          ct.name = newName;
        });
        problems.push({
          constraintId: "type-name-unique",
          message: `Duplicate type name "${name}" among siblings.`,
          location: { entityKind: "Type", entityGuid: type.guid, field: "name" },
          relatedGuids: group.map((t) => t.guid),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion 🔖Constraint: Type name uniqueness

// #region 🔖Constraint: Design name uniqueness

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintdesignnameuniqueness](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Design%20name%20uniqueness)
// Design name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating DesignNameUniqueness rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constraintdesignnameuniqueness🪨semiodesignnameuniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Design%20name%20uniqueness/d/i/semioDesignNameUniquenessConstraint)
 **/
export const semioDesignNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const byParent = new Map<Guid | undefined, Design[]>();
  toArray(ctx.kit.designs).forEach((d) => {
    const pid = d.parent?.guid as Guid | undefined;
    if (!byParent.has(pid)) byParent.set(pid, []);
    byParent.get(pid)!.push(d);
  });
  for (const [parentGuid, siblings] of byParent) {
    const names = new Map<string, Design[]>();
    siblings.forEach((d) => {
      const name = d.name ?? "";
      if (!names.has(name)) names.set(name, []);
      names.get(name)!.push(d);
    });
    for (const [name, group] of names) {
      if (group.length <= 1) continue;
      const [first, ...rest] = group;
      const siblingNames = siblings.map((s) => s.name ?? "");
      rest.forEach((design) => {
        const fix = semioMakeFix(ctx, `Rename "${name}"`, (clone) => {
          const cd = toArray(clone.designs).find((x) => x.guid === design.guid);
          if (!cd) return;
          const newName = generateUniqueName(name, siblingNames);
          cd.name = newName;
        });
        problems.push({
          constraintId: "design-name-unique",
          message: `Duplicate design name "${name}" among siblings.`,
          location: { entityKind: "Design", entityGuid: design.guid, field: "name" },
          relatedGuids: group.map((d) => d.guid),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion 🔖Constraint: Design name uniqueness

// #region 🔖Constraint: Piece name uniqueness

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintpiecenameuniqueness](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Piece%20name%20uniqueness)
// Piece name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating PieceNameUniqueness rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constraintpiecenameuniqueness🪨semiopiecenameuniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Piece%20name%20uniqueness/d/i/semioPieceNameUniquenessConstraint)
 **/
export const semioPieceNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  toArray(ctx.kit.designs).forEach((design) => {
    const pieces = toArray(design.pieces);
    if (pieces.length === 0) return;
    const nameMap = new Map<string, Piece[]>();
    pieces.forEach((p) => {
      const n = p.name ?? "";
      if (!nameMap.has(n)) nameMap.set(n, []);
      nameMap.get(n)!.push(p);
    });
    for (const [name, list] of nameMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allNames = pieces.map((p) => p.name ?? "");
      rest.forEach((piece) => {
        const fix = semioMakeFix(ctx, `Rename piece "${name}"`, (clone) => {
          const cd = toArray(clone.designs).find((d) => d.guid === design.guid);
          if (!cd) return;
          const cpieces = toArray(cd.pieces);
          const cp = cpieces.find((p) => p.guid === piece.guid);
          if (!cp) return;
          cp.name = generateUniqueName(name, allNames);
        });
        problems.push({
          constraintId: "piece-name-unique",
          message: `Duplicate piece name "${name}" inside design "${design.name}".`,
          location: { entityKind: "Piece", entityGuid: piece.guid, field: "name" },
          relatedGuids: list.map((p) => p.guid),
          fixes: [fix],
        });
      });
    }
  });
  return problems;
};

// #endregion 🔖Constraint: Piece name uniqueness

// #region 🔖Constraint: Quality name uniqueness

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintqualitynameuniqueness](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Quality%20name%20uniqueness)
// Quality name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating QualityNameUniqueness rules.
 * MUST detect and report constraint breachs. [👤semio📚js💻semio🔖kit🔖validation🔖constraintqualitynameuniqueness🪨semioqualitynameuniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Quality%20name%20uniqueness/d/i/semioQualityNameUniquenessConstraint)
 **/
export const semioQualityNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const qualities = toArray(ctx.kit.qualities);
  const nameMap = new Map<string, Quality[]>();
  qualities.forEach((q) => {
    const name = q.name ?? "";
    if (!nameMap.has(name)) nameMap.set(name, []);
    nameMap.get(name)!.push(q);
  });
  for (const [name, list] of nameMap) {
    if (list.length <= 1) continue;
    const [first, ...rest] = list;
    const allNames = qualities.map((q) => q.name ?? "");
    rest.forEach((quality) => {
      const fix = semioMakeFix(ctx, `Rename quality "${name}"`, (clone) => {
        const cq = toArray(clone.qualities).find((q) => q.guid === quality.guid);
        if (!cq) return;
        cq.name = generateUniqueName(name, allNames);
      });
      problems.push({
        constraintId: "quality-name-unique",
        message: `Duplicate quality name "${name}".`,
        location: { entityKind: "Quality", entityGuid: quality.guid, field: "name" },
        relatedGuids: list.map((q) => q.guid),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion 🔖Constraint: Quality name uniqueness

// #region 🔖Constraint: Port name uniqueness

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintportnameuniqueness](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Port%20name%20uniqueness)
// Port name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating PortNameUniqueness rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constraintportnameuniqueness🪨semioportnameuniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Port%20name%20uniqueness/d/i/semioPortNameUniquenessConstraint)
 **/
export const semioPortNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const ports = toArray(ctx.kit.ports);
  const nameMap = new Map<string, Port[]>();
  ports.forEach((i) => {
    const name = i.name ?? "";
    if (!nameMap.has(name)) nameMap.set(name, []);
    nameMap.get(name)!.push(i);
  });
  for (const [name, list] of nameMap) {
    if (list.length <= 1) continue;
    const [first, ...rest] = list;
    const allNames = ports.map((i) => i.name ?? "");
    rest.forEach((iface) => {
      const fix = semioMakeFix(ctx, `Rename port "${name}"`, (clone) => {
        const ci = toArray(clone.ports).find((i) => i.guid === iface.guid);
        if (!ci) return;
        ci.name = generateUniqueName(name, allNames);
      });
      problems.push({
        constraintId: "port-name-unique",
        message: `Duplicate port name "${name}".`,
        location: { entityKind: "Port", entityGuid: iface.guid, field: "name" },
        relatedGuids: list.map((i) => i.guid),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion 🔖Constraint: Port name uniqueness

// #region 🔖Constraint: File name uniqueness

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintfilenameuniqueness](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20File%20name%20uniqueness)
// File name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating FileNameUniqueness rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constraintfilenameuniqueness🪨semiofilenameuniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20File%20name%20uniqueness/d/i/semioFileNameUniquenessConstraint)
 **/
export const semioFileNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const files = toArray(ctx.kit.files);
  const nameMap = new Map<string, File[]>();
  files.forEach((f) => {
    const name = f.name ?? "";
    if (!nameMap.has(name)) nameMap.set(name, []);
    nameMap.get(name)!.push(f);
  });
  for (const [name, list] of nameMap) {
    if (list.length <= 1) continue;
    const [first, ...rest] = list;
    const allNames = files.map((f) => f.name ?? "");
    rest.forEach((file) => {
      const fix = semioMakeFix(ctx, `Rename file "${name}"`, (clone) => {
        const cf = toArray(clone.files).find((f) => f.guid === file.guid);
        if (!cf) return;
        cf.name = generateUniqueName(name, allNames);
      });
      problems.push({
        constraintId: "file-name-unique",
        message: `Duplicate file name "${name}".`,
        location: { entityKind: "File", entityGuid: file.guid, field: "name" },
        relatedGuids: list.map((f) => f.guid),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion 🔖Constraint: File name uniqueness

// #region 🔖Constraint: Folder name uniqueness

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintfoldernameuniqueness](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Folder%20name%20uniqueness)
// Folder name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating FolderNameUniqueness rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constraintfoldernameuniqueness🪨semiofoldernameuniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Folder%20name%20uniqueness/d/i/semioFolderNameUniquenessConstraint)
 **/
export const semioFolderNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const byParent = new Map<Guid | undefined, Folder[]>();
  const folders = toArray(ctx.kit.folders);
  folders.forEach((f) => {
    const pid = f.parent?.guid as Guid | undefined;
    if (!byParent.has(pid)) byParent.set(pid, []);
    byParent.get(pid)!.push(f);
  });
  for (const [parentGuid, siblings] of byParent) {
    const nameMap = new Map<string, Folder[]>();
    siblings.forEach((f) => {
      const name = f.name ?? "";
      if (!nameMap.has(name)) nameMap.set(name, []);
      nameMap.get(name)!.push(f);
    });
    for (const [name, list] of nameMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allNames = siblings.map((f) => f.name ?? "");
      rest.forEach((folder) => {
        const fix = semioMakeFix(ctx, `Rename folder "${name}"`, (clone) => {
          const cf = toArray(clone.folders).find((f) => f.guid === folder.guid);
          if (!cf) return;
          cf.name = generateUniqueName(name, allNames);
        });
        problems.push({
          constraintId: "folder-name-unique",
          message: `Duplicate folder name "${name}" among siblings.`,
          location: { entityKind: "Folder", entityGuid: folder.guid, field: "name" },
          relatedGuids: list.map((f) => f.guid),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion 🔖Constraint: Folder name uniqueness

// #region 🔖Constraint: Connector name uniqueness within type

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintconnectornameuniquenesswithintype](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Connector%20name%20uniqueness%20within%20type)
// Connector name uniqueness within type constraint MUST be enforced here.

/**
 * Constraint validating ConnectorNameUniqueness rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constraintconnectornameuniquenesswithintype🪨semioconnectornameuniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Connector%20name%20uniqueness%20within%20type/d/i/semioConnectorNameUniquenessConstraint)
 **/
export const semioConnectorNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  for (const [typeGuid, connectors] of ctx.connectorsByTypeGuid) {
    if (connectors.length === 0) continue;
    const nameMap = new Map<string, Connector[]>();
    connectors.forEach((p) => {
      const name = p.name ?? "";
      if (!nameMap.has(name)) nameMap.set(name, []);
      nameMap.get(name)!.push(p);
    });
    for (const [name, list] of nameMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allNames = connectors.map((p) => p.name ?? "");
      const type = ctx.typesByGuid.get(typeGuid);
      rest.forEach((connector) => {
        const fix = semioMakeFix(ctx, `Rename connector "${name}"`, (clone) => {
          const ct = toArray(clone.types).find((t) => t.guid === typeGuid);
          if (!ct) return;
          const cconnectors = toArray(ct.connectors);
          const cp = cconnectors.find((p) => p.guid === connector.guid);
          if (!cp) return;
          cp.name = generateUniqueName(name, allNames);
        });
        problems.push({
          constraintId: "connector-name-unique",
          message: `Duplicate connector name "${name}" inside type "${type?.name}".`,
          location: { entityKind: "Connector", entityGuid: connector.guid, field: "name" },
          relatedGuids: list.map((p) => p.guid),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion 🔖Constraint: Connector name uniqueness within type

// #region 🔖Constraint: Model name uniqueness within type

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintmodelnameuniquenesswithintype](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Model%20name%20uniqueness%20within%20type)
// Model name uniqueness within type constraint MUST be enforced here.

/**
 * Constraint validating ModelNameUniqueness rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constraintmodelnameuniquenesswithintype🪨semiomodelnameuniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Model%20name%20uniqueness%20within%20type/d/i/semioModelNameUniquenessConstraint)
 **/
export const semioModelNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  for (const [typeGuid, models] of ctx.modelsByTypeGuid) {
    if (models.length === 0) continue;
    const nameMap = new Map<string, Model[]>();
    models.forEach((m) => {
      const name = m.name ?? "";
      if (!nameMap.has(name)) nameMap.set(name, []);
      nameMap.get(name)!.push(m);
    });
    for (const [name, list] of nameMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allNames = models.map((m) => m.name ?? "");
      const type = ctx.typesByGuid.get(typeGuid);
      rest.forEach((model) => {
        const fix = semioMakeFix(ctx, `Rename model "${name}"`, (clone) => {
          const ct = toArray(clone.types).find((t) => t.guid === typeGuid);
          if (!ct) return;
          const cmodels = toArray(ct.models);
          const cm = cmodels.find((m) => m.guid === model.guid);
          if (!cm) return;
          cm.name = generateUniqueName(name, allNames);
        });
        problems.push({
          constraintId: "model-name-unique",
          message: `Duplicate model name "${name}" inside type "${type?.name}".`,
          location: { entityKind: "Model", entityGuid: model.guid, field: "name" },
          relatedGuids: list.map((m) => m.guid),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion 🔖Constraint: Model name uniqueness within type

// #region 🔖Constraint: Layer path uniqueness within design

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintlayerpathuniquenesswithindesign](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Layer%20path%20uniqueness%20within%20design)
// Layer path uniqueness within design constraint MUST be enforced here.

/**
 * Constraint validating LayerPathUniqueness rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constraintlayerpathuniquenesswithindesign🪨semiolayerpathuniquenessconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Layer%20path%20uniqueness%20within%20design/d/i/semioLayerPathUniquenessConstraint)
 **/
export const semioLayerPathUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  toArray(ctx.kit.designs).forEach((design) => {
    const layers = toArray(design.layers);
    if (layers.length === 0) return;
    const pathMap = new Map<string, Layer[]>();
    layers.forEach((l) => {
      const path = l.path ?? "";
      if (!pathMap.has(path)) pathMap.set(path, []);
      pathMap.get(path)!.push(l);
    });
    for (const [path, list] of pathMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allPaths = layers.map((l) => l.path ?? "");
      rest.forEach((layer) => {
        const fix = semioMakeFix(ctx, `Rename layer "${path}"`, (clone) => {
          const cd = toArray(clone.designs).find((d) => d.guid === design.guid);
          if (!cd) return;
          const clayers = toArray(cd.layers);
          const cl = clayers.find((l) => l.path === layer.path);
          if (!cl) return;
          cl.path = generateUniqueName(path, allPaths);
        });
        problems.push({
          constraintId: "layer-path-unique",
          message: `Duplicate layer path "${path}" inside design "${design.name}".`,
          location: { entityKind: "Layer", entityGuid: layer.guid, field: "path" },
          fixes: [fix],
        });
      });
    }
  });
  return problems;
};

// #endregion 🔖Constraint: Layer path uniqueness within design

// #region 🔖Constraint: Design piece same family constraint

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintdesignpiecesamefamilyconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Design%20piece%20same%20family%20constraint)
// Design piece same family constraint MUST be enforced here.

/**
 * Constraint validating DesignPieceSameFamily rules.
 * MUST detect and report constraint breachs.
 * [👤semio📚js💻semio🔖kit🔖validation🔖constraintdesignpiecesamefamilyconstraint🪨semiodesignpiecesamefamilyconstraint](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Design%20piece%20same%20family%20constraint/d/i/semioDesignPieceSameFamilyConstraint)
 **/
export const semioDesignPieceSameFamilyConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  toArray(ctx.kit.designs).forEach((design) => {
    const pieces = toArray(design.pieces);
    pieces.forEach((piece) => {
      if (!piece.design?.guid) return;
      try {
        const pieceDesign = ctx.designsByGuid.get(piece.design.guid);
        if (!pieceDesign) return;

        const containerPrimitive = getPrimitiveDesignFromContext(ctx, design.guid);
        const piecePrimitive = getPrimitiveDesignFromContext(ctx, piece.design.guid);

        if (containerPrimitive === piecePrimitive) {
          const fix = semioMakeFix(ctx, `Remove design piece "${piece.name || piece.guid}"`, (clone) => {
            const cd = toArray(clone.designs).find((d) => d.guid === design.guid);
            if (!cd) return;
            cd.pieces = toArray(cd.pieces).filter((p) => p.guid !== piece.guid);

            cd.connections = toArray(cd.connections).filter((c) => c.connected.piece.guid !== piece.guid && c.connecting.piece.guid !== piece.guid);
          });
          problems.push({
            constraintId: "design-piece-same-family",
            message: `Design piece "${piece.name || piece.guid}" references design "${pieceDesign.name}" which is in the same design family as container design "${design.name}". A design cannot contain design pieces from the same family.`,
            location: { entityKind: "Piece", entityGuid: piece.guid, field: "design" },
            relatedGuids: [piece.guid, design.guid, pieceDesign.guid],
            fixes: [fix],
          });
        }
      } catch { }
    });
  });
  return problems;
};

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintdesignpiecesamefamilyconstraint🪨getprimitivedesignfromcontext](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint:%20Design%20piece%20same%20family%20constraint/d/i/getPrimitiveDesignFromContext)
// getPrimitiveDesignFromContext holds the data fields for a getPrimitiveDesignFromContext record.
const getPrimitiveDesignFromContext = (ctx: ValidationContext, designGuid: string): string => {
  let currentGuid = designGuid;
  let interactions = 0;
  const maxIterations = 1000;
  while (interactions < maxIterations) {
    const design = ctx.designsByGuid.get(currentGuid);
    if (!design || !design.parent?.guid) return currentGuid;
    currentGuid = design.parent.guid;
    interactions++;
  }
  return currentGuid;
};

// #endregion 🔖Constraint: Design piece same family constraint

// #region 🔖Constraint registration

// [👤semio📚js💻semio🔖kit🔖validation🔖constraintregistration](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Constraint%20registration)
// Constraint registration and default configurations MUST be defined here.

defaultConstraints = [
  semioGuidUniquenessConstraint,
  semioTypeNameUniquenessConstraint,
  semioDesignNameUniquenessConstraint,
  semioPieceNameUniquenessConstraint,
  semioQualityNameUniquenessConstraint,
  semioPortNameUniquenessConstraint,
  semioFileNameUniquenessConstraint,
  semioFolderNameUniquenessConstraint,
  semioConnectorNameUniquenessConstraint,
  semioModelNameUniquenessConstraint,
  semioLayerPathUniquenessConstraint,
  semioDesignPieceSameFamilyConstraint,
];

// #endregion 🔖Constraint registration

// #region 🔖Validation serialization

// [👤semio📚js💻semio🔖kit🔖validation🔖validationserialization](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20serialization)
// Validation result serialization and deserialization MUST be defined here.

/**
 * Interface defining SerializableValidationFix structure.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationserialization🛠️serializablevalidationfix](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20serialization/d/i/SerializableValidationFix)
 **/
export interface SerializableValidationFix {
  title: string;
  diff?: KitDiff;
}

/**
 * Interface defining SerializableProblem structure.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationserialization🛠️serializableproblem](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20serialization/d/i/SerializableProblem)
 **/
export interface SerializableProblem {
  constraintId: string;
  message: string;
  entityKind: string;
  entityGuid: string;
  fixes: SerializableValidationFix[];
}

/**
 * Interface defining SerializableValidationResult structure.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationserialization🛠️serializablevalidationresult](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20serialization/d/i/SerializableValidationResult)
 **/
export interface SerializableValidationResult {
  problems: SerializableProblem[];
}

/**
 * Converts to ValidationResult representation.
 * MUST convert to the target representation.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationserialization🪨tovalidationresult](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20serialization/d/i/toValidationResult)
 **/
export const toValidationResult = (result: ValidationResult): SerializableValidationResult => ({
  problems: result.problems.map((problem) => ({
    constraintId: problem.constraintId,
    message: problem.message,
    entityKind: problem.location?.entityKind ?? (problem as any).entityKind,
    entityGuid: problem.location?.entityGuid ?? (problem as any).entityGuid ?? "",
    fixes: problem.fixes.map((fix) => ({ title: fix.title, diff: fix.diff })),
  })),
});

/**
 * Serializes ValidationResult for transport.
 * MUST produce a serializable output.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationserialization🪨serializevalidationresult](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20serialization/d/i/serializeValidationResult)
 **/
export const serializeValidationResult = (result: ValidationResult): string => {
  const serializable = toValidationResult(result);
  serializable.problems.sort((a, b) => {
    const constraintCompare = a.constraintId.localeCompare(b.constraintId);
    if (constraintCompare !== 0) return constraintCompare;
    return a.entityGuid.localeCompare(b.entityGuid);
  });
  return JSON.stringify(serializable, null, 2);
};

/**
 * Parses ValidationResult from serialized input.
 * MUST produce a valid in-memory representation.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationserialization🪨parsevalidationresult](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20serialization/d/i/parseValidationResult)
 **/
export const parseValidationResult = (json: string): SerializableValidationResult => JSON.parse(json);

// [👤semio📚js💻semio🔖kit🔖validation🔖validationserialization🪨isguid](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20serialization/d/i/isGuid)
// isGuid holds the data fields for a isGuid record.
const isGuid = (s: string): boolean => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(s);

/**
 * Deep equality check for KitDiffs ignoring NewGuids entities.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationserialization🪨arekitdiffsequalignoringnewguids](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20serialization/d/i/areKitDiffsEqualIgnoringNewGuids)
 **/
export const areKitDiffsEqualIgnoringNewGuids = (a: KitDiff, b: KitDiff): boolean => {
  const normalize = (obj: unknown): unknown => {
    if (obj === null || obj === undefined) return obj;
    if (typeof obj === "string" && isGuid(obj)) return "<GUID>";
    if (Array.isArray(obj)) return obj.map(normalize);
    if (typeof obj === "object") {
      const result: Record<string, unknown> = {};
      for (const [k, v] of Object.entries(obj)) result[k] = normalize(v);
      return result;
    }
    return obj;
  };
  return JSON.stringify(normalize(a)) === JSON.stringify(normalize(b));
};

/**
 * Deep equality check for ValidationResults entities.
 * MUST return a boolean equality result.
 * [👤semio📚js💻semio🔖kit🔖validation🔖validationserialization🪨arevalidationresultsequal](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/Validation/s/Validation%20serialization/d/i/areValidationResultsEqual)
 **/
export const areValidationResultsEqual = (a: ValidationResult, b: ValidationResult): boolean => {
  const serializableA = toValidationResult(a);
  const serializableB = toValidationResult(b);
  if (serializableA.problems.length !== serializableB.problems.length) return false;
  const sortProblems = (problems: SerializableProblem[]) =>
    [...problems].sort((x, y) => {
      const constraintCompare = x.constraintId.localeCompare(y.constraintId);
      if (constraintCompare !== 0) return constraintCompare;
      return x.entityGuid.localeCompare(y.entityGuid);
    });
  const sortedA = sortProblems(serializableA.problems);
  const sortedB = sortProblems(serializableB.problems);
  return sortedA.every((problemA, i) => {
    const problemB = sortedB[i];
    if (problemA.constraintId !== problemB.constraintId || problemA.message !== problemB.message || problemA.entityKind !== problemB.entityKind || problemA.entityGuid !== problemB.entityGuid) return false;
    if (problemA.fixes.length !== problemB.fixes.length) return false;
    return problemA.fixes.every((fixA, j) => {
      const fixB = problemB.fixes[j];
      return fixA.title === fixB.title && areKitDiffsEqualIgnoringNewGuids(fixA.diff ?? {}, fixB.diff ?? {});
    });
  });
};

// #endregion 🔖Validation serialization

// #endregion 🔖Validation

// #region 🔖KitStore
// [👤semio📚js💻semio🔖kit🔖kitstore](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/KitStore)
// Storage-agnostic kit store contracts MUST be defined here.
// These interfaces express what a kit store DOES, not how a specific engine stores data.
// No engine-specific primitives (map/array/doc) may appear in these contracts.

// Specs: KitStoreStatus represents the lifecycle states of a kit store.
// Providers transition through states: idle → loading → ready → saving/syncing → ready.
// Error and offline are terminal-ish states that require external resolution.

/**
 * Lifecycle status of a kit store.
 *
 * Specs: Providers MUST transition through these states in order:
 * idle → loading → ready. saving/syncing are transient states
 * that return to ready. error/offline require external recovery.
 * [👤semio📚js💻semio🔖kit🔖kitstore🛠️kitstorestatus](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/KitStore/d/i/KitStoreStatus)
 **/
export type KitStoreStatus = "idle" | "loading" | "ready" | "saving" | "syncing" | "offline" | "error";

/**
 * Synchronization state of a kit store.
 *
 * Specs: Reported as part of KitStoreSnapshot. dirty indicates
 * unsaved local changes. readonly means the store rejects mutations.
 * lastSyncedAt is ISO 8601 timestamp of the last successful persistence.
 * [👤semio📚js💻semio🔖kit🔖kitstore🛠️kitsyncstate](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/KitStore/d/i/KitSyncState)
 **/
export type KitSyncState = {
  status: KitStoreStatus;
  dirty: boolean;
  readonly: boolean;
  lastSyncedAt?: string;
  error?: Error;
};

/**
 * Immutable snapshot of kit data and sync state.
 *
 * Specs: Returned by KitStore.getSnapshot(). kit is the full domain
 * Kit object. sync describes the current synchronization state.
 * Snapshots MUST be structurally immutable — callers should not mutate them.
 * [👤semio📚js💻semio🔖kit🔖kitstore🛠️kitstoresnapshot](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/KitStore/d/i/KitStoreSnapshot)
 **/
export type KitStoreSnapshot = {
  kit: Kit;
  sync: KitSyncState;
};

/**
 * Storage-agnostic kit store contract.
 *
 * Specs: This is the boundary between the editor and storage backends.
 * semio/sketchpad depends ONLY on this interface — never on provider internals.
 * Providers (collaborative, JSON file, folder/sqlite) implement this interface
 * and live in semio/studio. The editor consumes stores by injection.
 *
 * getSnapshot() returns the current immutable state.
 * subscribe() registers a listener called on every state change.
 * transact() groups multiple mutations into one logical operation.
 * apply() merges a KitDiff into the current kit state.
 * replace() swaps the entire kit state.
 * save() persists the current state to the backend.
 * reload() re-reads state from the backend, discarding local changes.
 * dispose() releases all resources held by the store.
 * [👤semio📚js💻semio🔖kit🔖kitstore🛠️kitstore](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/KitStore/d/i/KitStore)
 **/
export interface KitStore {
  getSnapshot(): KitStoreSnapshot;
  subscribe(listener: () => void): () => void;

  transact<T>(label: string, run: () => T): T;
  apply(diff: KitDiff, meta?: { origin?: string }): void;
  replace(next: Kit, meta?: { origin?: string }): void;

  save(): Promise<void>;
  reload(): Promise<void>;
  dispose(): Promise<void> | void;
}

/**
 * Kit store with undo/redo capability.
 *
 * Specs: Extends KitStore with reversible transaction support.
 * Undo/redo semantics are provider-specific (CRDT-native, command stack, etc.)
 * but the interface is storage-agnostic. The editor uses canUndo/canRedo to
 * enable/disable UI controls without knowing the undo implementation.
 * [👤semio📚js💻semio🔖kit🔖kitstore🛠️undoablekitstore](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/KitStore/d/i/UndoableKitStore)
 **/
export interface UndoableKitStore extends KitStore {
  canUndo(): boolean;
  canRedo(): boolean;
  undo(): void;
  redo(): void;
}

/**
 * Binary asset storage contract.
 *
 * Specs: Decouples asset storage from kit data storage.
 * Providers may store assets as blobs in IndexedDB, files on disk,
 * or references to remote URLs. The editor uses this interface
 * without knowing the storage strategy.
 * [👤semio📚js💻semio🔖kit🔖kitstore🛠️blobassetstore](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/KitStore/d/i/BlobAssetStore)
 **/
export interface BlobAssetStore {
  put(file: Blob, meta?: { path?: string; mimeKind?: string }): Promise<{ id: string; url?: string }>;
  get(id: string): Promise<Blob>;
  remove(id: string): Promise<void>;
}

/**
 * Fine-grained path subscription contract.
 *
 * Specs: Optional capability for stores that can optimize subscriptions
 * to specific paths within the kit data tree. The editor MAY use this
 * for performance but MUST NOT require it — snapshot+selector is the fallback.
 * [👤semio📚js💻semio🔖kit🔖kitstore🛠️observablepathstore](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/KitStore/d/i/ObservablePathStore)
 **/
export interface ObservablePathStore {
  subscribePath(path: readonly string[], listener: () => void): () => void;
}

// #region 🔖InMemoryKitStore
// [👤semio📚js💻semio🔖kit🔖kitstore🔖inmemorykitstore](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/KitStore/s/InMemoryKitStore)
// In-memory kit store implementation for testing and fake backends.

// Specs: InMemoryKitStore implements UndoableKitStore using a plain Kit object
// in memory with a command-stack undo model. No persistence, no sync, no CRDT.
// Used for: unit tests, integration tests, fake backends, storybook.

/**
 * In-memory kit store for testing and local editing without persistence.
 *
 * Specs: Holds a Kit in memory. apply() uses applyKitDiff to merge diffs.
 * replace() swaps the Kit wholesale. transact() groups mutations.
 * Undo/redo uses a command stack of KitChange (forward+backward diffs).
 * subscribe() notifies listeners on every mutation.
 * save()/reload() are no-ops (no backend).
 * [👤semio📚js💻semio🔖kit🔖kitstore🔖inmemorykitstore🛠️inmemorykitstore](repo://p/u/semio/b/l/js/f/semio.ts/s/Kit/s/KitStore/s/InMemoryKitStore/d/i/InMemoryKitStore)
 **/
export class InMemoryKitStore implements UndoableKitStore {
  private kit: Kit;
  private listeners: Set<() => void> = new Set();
  private undoStack: KitChange[] = [];
  private redoStack: KitChange[] = [];
  private dirty: boolean = false;
  private disposed: boolean = false;
  private status: KitStoreStatus = "ready";
  private transacting: boolean = false;
  private transactionDiffs: KitDiff[] = [];

  constructor(kit: Kit) {
    this.kit = kit;
  }

  getSnapshot(): KitStoreSnapshot {
    return {
      kit: this.kit,
      sync: {
        status: this.status,
        dirty: this.dirty,
        readonly: false,
      },
    };
  }

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  transact<T>(label: string, run: () => T): T {
    const before = this.kit;
    this.transacting = true;
    this.transactionDiffs = [];
    try {
      const result = run();
      const after = this.kit;
      if (before !== after) {
        const forward = getKitDiff(before, after);
        const backward = inverseKitDiff(before, forward);
        this.undoStack.push({ forward, backward });
        this.redoStack = [];
      }
      return result;
    } finally {
      this.transacting = false;
      this.transactionDiffs = [];
    }
  }

  apply(diff: KitDiff, meta?: { origin?: string }): void {
    const before = this.kit;
    this.kit = applyKitDiff(this.kit, diff);
    this.dirty = true;
    if (!this.transacting && !this.disposed) {
      const forward = getKitDiff(before, this.kit);
      const backward = inverseKitDiff(before, forward);
      this.undoStack.push({ forward, backward });
      this.redoStack = [];
    }
    this.notify();
  }

  replace(next: Kit, meta?: { origin?: string }): void {
    const before = this.kit;
    this.kit = next;
    this.dirty = true;
    if (!this.transacting && !this.disposed) {
      const forward = getKitDiff(before, next);
      const backward = inverseKitDiff(before, forward);
      this.undoStack.push({ forward, backward });
      this.redoStack = [];
    }
    this.notify();
  }

  async save(): Promise<void> {
    this.dirty = false;
    this.notify();
  }

  async reload(): Promise<void> {
    this.notify();
  }

  dispose(): void {
    this.disposed = true;
    this.listeners.clear();
    this.undoStack = [];
    this.redoStack = [];
  }

  canUndo(): boolean {
    return this.undoStack.length > 0;
  }

  canRedo(): boolean {
    return this.redoStack.length > 0;
  }

  undo(): void {
    const change = this.undoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.backward);
    this.redoStack.push(change);
    this.dirty = true;
    this.notify();
  }

  redo(): void {
    const change = this.redoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.forward);
    this.undoStack.push(change);
    this.dirty = true;
    this.notify();
  }

  private notify(): void {
    for (const listener of this.listeners) {
      listener();
    }
  }
}

// #endregion 🔖InMemoryKitStore

// #endregion 🔖KitStore

// #endregion 🔖Validation

// #endregion 🔖Kit
// #region 🔖Tests
// [👤semio📚js💻index🔖tests](repo://p/u/semio/b/l/js/f/index.ts/s/Tests)
// Vitest test suites for domain logic. MUST NOT export any symbols.
// Test code is guarded so it only executes under vitest, not in browser bundles.
if (typeof (globalThis as any).__vitest_worker__ !== "undefined") {
  const { beforeAll, describe, expect, it } = await import("vitest");
  const { createElement } = await import("react");
  const { renderToStaticMarkup } = await import("react-dom/server");
  const ElementsBundle = await import("@elements/ui");
  const { buildControlTree, Action: UiAction } = ElementsBundle;
  type ControlDef = import("@elements/ui").ControlDef;
  const { DragDesign, DragDiffDesign, DragOffset, DragPieces, InvalidKit, InvalidKitValidation, MetabolismKit, MetabolismKitDiff, MetabolismKitDiffed, MetabolismKitDiffInverted, ModelSelectionCases } = await import("@semio/assets");
  const { createFolderKitStore, createJsonFileKitStore } = await import("@semio/studio");
  type KitFolderAdapter = import("@semio/studio").KitFolderAdapter;
  type KitJsonFileAdapter = import("@semio/studio").KitJsonFileAdapter;

  const TEST_TOLERANCE = 0.001;

  const planesEqual = (p1?: Plane, p2?: Plane): boolean => {
    if (!p1 || !p2) return false;
    if (!p1.origin || !p2.origin || !p1.xAxis || !p2.xAxis || !p1.yAxis || !p2.yAxis) return false;
    return (
      Math.abs(p1.origin.x - p2.origin.x) < TEST_TOLERANCE &&
      Math.abs(p1.origin.y - p2.origin.y) < TEST_TOLERANCE &&
      Math.abs(p1.origin.z - p2.origin.z) < TEST_TOLERANCE &&
      Math.abs(p1.xAxis.x - p2.xAxis.x) < TEST_TOLERANCE &&
      Math.abs(p1.xAxis.y - p2.xAxis.y) < TEST_TOLERANCE &&
      Math.abs(p1.xAxis.z - p2.xAxis.z) < TEST_TOLERANCE &&
      Math.abs(p1.yAxis.x - p2.yAxis.x) < TEST_TOLERANCE &&
      Math.abs(p1.yAxis.y - p2.yAxis.y) < TEST_TOLERANCE &&
      Math.abs(p1.yAxis.z - p2.yAxis.z) < TEST_TOLERANCE
    );
  };

  const centersEqual = (c1: { u: number; v: number } | undefined, c2: { u: number; v: number } | undefined): boolean => {
    if (!c1 || !c2) return c1 === c2;
    return Math.abs(c1.u - c2.u) < TEST_TOLERANCE && Math.abs(c1.v - c2.v) < TEST_TOLERANCE;
  };

  const findDesign = (kit: Kit, name: string, parentName?: string) => {
    let parentGuid: string | undefined;
    if (parentName) {
      const p = kit.designs?.find((d) => d.name === parentName);
      if (!p) throw new Error(`Parent ${parentName} not found`);
      parentGuid = p.guid;
    }
    const d = kit.designs?.find((d) => d.name === name && (parentGuid ? d.parent?.guid === parentGuid : !d.parent));
    if (!d) throw new Error(`Design ${name} not found`);
    return d;
  };

  const getTestNodePaths = async () => {
    const { dirname, resolve } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const __filename = fileURLToPath(import.meta.url);
    const __dirname = dirname(__filename);
    const EXPORT_REPORTS_DIR = resolve(__dirname, "../../reports/export-design-model");
    return { __filename, __dirname, EXPORT_REPORTS_DIR, dirname, resolve };
  };

  const writeExportReport = async (implementation: string, bytes: Uint8Array<ArrayBufferLike>) => {
    const { mkdirSync, writeFileSync } = await import("node:fs");
    const { EXPORT_REPORTS_DIR, resolve } = await getTestNodePaths();
    mkdirSync(EXPORT_REPORTS_DIR, { recursive: true });
    const reportPath = resolve(EXPORT_REPORTS_DIR, `${implementation}.gltf`);
    writeFileSync(reportPath, bytes);
    return reportPath;
  };

  const roundSceneNumber = (value: number) => {
    const rounded = Math.round(value * 10_000) / 10_000;
    return Object.is(rounded, -0) ? 0 : rounded;
  };

  const composeNodeMatrix = (node: { matrix?: number[]; translation?: number[]; rotation?: number[]; scale?: number[] }) => {
    if (node.matrix) {
      return node.matrix.map((value) => roundSceneNumber(value));
    }
    const translation = node.translation ?? [0, 0, 0];
    const rotation = node.rotation ?? [0, 0, 0, 1];
    const scale = node.scale ?? [1, 1, 1];
    const [x, y, z, w] = rotation;
    const x2 = x + x;
    const y2 = y + y;
    const z2 = z + z;
    const xx = x * x2;
    const xy = x * y2;
    const xz = x * z2;
    const yy = y * y2;
    const yz = y * z2;
    const zz = z * z2;
    const wx = w * x2;
    const wy = w * y2;
    const wz = w * z2;
    const sx = scale[0];
    const sy = scale[1];
    const sz = scale[2];
    return [
      roundSceneNumber((1 - (yy + zz)) * sx),
      roundSceneNumber((xy + wz) * sx),
      roundSceneNumber((xz - wy) * sx),
      0,
      roundSceneNumber((xy - wz) * sy),
      roundSceneNumber((1 - (xx + zz)) * sy),
      roundSceneNumber((yz + wx) * sy),
      0,
      roundSceneNumber((xz + wy) * sz),
      roundSceneNumber((yz - wx) * sz),
      roundSceneNumber((1 - (xx + yy)) * sz),
      0,
      roundSceneNumber(translation[0]),
      roundSceneNumber(translation[1]),
      roundSceneNumber(translation[2]),
      1,
    ];
  };

  const normalizeSceneGraph = (gltfText: string) => {
    const gltf = JSON.parse(gltfText) as {
      scene?: number;
      scenes?: Array<{ nodes?: number[] }>;
      nodes?: Array<{ name?: string; children?: number[]; matrix?: number[]; mesh?: number; translation?: number[]; rotation?: number[]; scale?: number[] }>;
    };
    const nodes = gltf.nodes ?? [];
    const defaultScene = gltf.scenes?.[gltf.scene ?? 0] ?? { nodes: [] };
    const names = nodes.map((node, index) => node.name ?? `__node_${index}`);
    const parents = new Map<string, string | null>();
    for (const name of names) parents.set(name, null);
    for (let index = 0; index < nodes.length; index += 1) {
      for (const childIndex of nodes[index].children ?? []) {
        parents.set(names[childIndex], names[index]);
      }
    }
    let normalizedRoots = [...(defaultScene.nodes ?? [])].map((index) => names[index]).sort();
    let normalizedNodes = nodes
      .map((node, index) => ({
        name: names[index],
        parent: parents.get(names[index]) ?? null,
        children: [...(node.children ?? [])].map((childIndex) => names[childIndex]).sort(),
        hasMesh: node.mesh !== undefined,
        matrix: composeNodeMatrix(node),
      }))
      .sort((left, right) => left.name.localeCompare(right.name));
    const syntheticWorld = normalizedNodes.find((node) => node.name === "world");
    if (
      syntheticWorld &&
      !syntheticWorld.hasMesh &&
      syntheticWorld.parent === null &&
      syntheticWorld.children.length === 1 &&
      normalizedRoots.length === 1 &&
      normalizedRoots[0] === "world" &&
      syntheticWorld.matrix.every((value, index) => value === [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1][index])
    ) {
      const childName = syntheticWorld.children[0];
      normalizedRoots = [childName];
      normalizedNodes = normalizedNodes.filter((node) => node.name !== "world").map((node) => (node.name === childName ? { ...node, parent: null } : node));
    }
    return {
      roots: normalizedRoots,
      nodes: normalizedNodes,
    };
  };

  const runExportReportCommand = async (command: string, args: string[], cwd: string) => {
    const { execFileSync } = await import("node:child_process");
    execFileSync(command, args, {
      cwd,
      stdio: "pipe",
    });
  };

  const parseSelfContainedGltf = async (reportText: string) => {
    const parsed = JSON.parse(reportText) as {
      buffers?: Array<{ uri?: string }>;
      images?: Array<{ uri?: string }>;
    };
    const resources: Record<string, Uint8Array> = {};
    const collectResource = (uri?: string) => {
      if (!uri?.startsWith("data:")) return;
      const base64 = uri.slice(uri.indexOf(",") + 1);
      resources[uri] = new Uint8Array(Buffer.from(base64, "base64"));
    };
    for (const buffer of parsed.buffers ?? []) collectResource(buffer.uri);
    for (const image of parsed.images ?? []) collectResource(image.uri);
    const io = new NodeIO();
    return io.readJSON({ json: parsed as any, resources });
  };

  const getMeshNames = (reportText: string) => {
    const parsed = JSON.parse(reportText) as { meshes?: Array<{ name?: string }> };
    return (parsed.meshes ?? []).map((mesh) => mesh.name).filter((name): name is string => Boolean(name));
  };

  describe("Change", () => {
    describe("Metabolism", () => {
      const kitOriginal = { ...(MetabolismKit as any), designs: (MetabolismKit as any).designs?.filter((d: any) => !d.parent) };
      const kitDiff = MetabolismKitDiff as any;
      const kitDiffInverted = MetabolismKitDiffInverted as any;
      const kitDiffed = MetabolismKitDiffed as any;

      it("Kit + Change.Forward = DiffedKit & DiffedKit + Change.Backward = Kit", () => {
        const change = getKitChange(kitOriginal, kitDiffed);
        const computedDiff = getKitDiff(kitOriginal, kitDiffed);
        expect(areKitDiffsEqual(computedDiff, kitDiff)).toBe(true);
        const computedInverseDiff = inverseKitDiff(kitOriginal, change.forward);
        expect(areKitDiffsEqual(computedInverseDiff, kitDiffInverted)).toBe(true);
        expect(areKitDiffsEqual(change.forward, kitDiff)).toBe(true);
        expect(areKitDiffsEqual(change.backward, kitDiffInverted)).toBe(true);
        const appliedForward = applyKitDiff(kitOriginal, change.forward);
        expect(areKitsEqual(appliedForward, kitDiffed)).toBe(true);
        const appliedInverse = applyKitDiff(kitDiffed, change.backward);
        expect(areKitsEqual(appliedInverse, kitOriginal)).toBe(true);
      });

      describe("Design/Model", () => {
        it("selectBestModel uses tag filtering + modified jaccard and matches shared semio asset cases", () => {
          const payload = ModelSelectionCases as {
            cases: Array<{
              name: string;
              selectedTagGuids: string[];
              expectedGuid: string | null;
              models: Array<{ guid: string; fileGuid: string; tagGuids: string[] }>;
            }>;
          };
          payload.cases.forEach((testCase) => {
            const models: Model[] = testCase.models.map((model) => ({
              guid: model.guid,
              file: { guid: model.fileGuid },
              tags: model.tagGuids.map((guid) => ({ guid })),
            }));
            const selected = selectBestModel(models, testCase.selectedTagGuids);
            expect(selected?.guid ?? null).toBe(testCase.expectedGuid);
          });
        });
      });
    });
  });

  describe("Flatten", () => {
    const kit = MetabolismKit as Kit;

    const testFlatten = (designName: string, parentName?: string) => {
      const design = findDesign(kit, designName, parentName);
      const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && d.parent?.guid === design.guid);
      expect(expectedDesign).toBeDefined();
      const flatDesignChange = flattenDesign(kit, design.guid);
      const flatDesign = applyDesignDiff(design, flatDesignChange.forward);

      flatDesign!.pieces?.forEach((p) => {
        const expectedPiece = expectedDesign!.pieces?.find((ep) => ep.name === p.name);
        expect(expectedPiece).toBeDefined();
        expect(p.plane).toBeDefined();
        expect(p.center).toBeDefined();
        expect(planesEqual(p.plane, expectedPiece!.plane)).toBe(true);
        expect(centersEqual(p.center, expectedPiece!.center)).toBe(true);
      });
    };

    describe("Nakagin Capsule Tower", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Nakagin Capsule Tower");
      });
      describe("Slanted", () => {
        it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
          testFlatten("Slanted", "Nakagin Capsule Tower");
        });
      });
      describe("Twisted", () => {
        it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
          testFlatten("Twisted", "Nakagin Capsule Tower");
        });
      });
      describe("Dancing", () => {
        it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
          testFlatten("Dancing", "Nakagin Capsule Tower");
        });
      });
    });

    describe("Capsule Dream", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Capsule Dream");
      });
    });
  });

  describe("Roundtrip", () => {
    describe("Metabolism", () => {
      it("Json -> Memory -> Json, Json -> Zip, Zip -> Json", async () => {
        const fs = await import("node:fs");
        const path = await import("node:path");
        const { __dirname } = await getTestNodePaths();

        const kit = MetabolismKit as unknown as Kit;
        const serializedKit = serializeKit(kit);
        const deserializedKit = deserializeKit(serializedKit);
        expect(areKitsEqual(kit, deserializedKit)).toBe(true);

        const zipPath = path.join(__dirname, "../assets/semio/metabolism.zip");
        const zipBuffer = fs.readFileSync(zipPath);
        const { kit: zipKit } = await importKit(zipBuffer.buffer);
        expect(areKitsEqual(kit, zipKit)).toBe(true);

        const exportedZip = await exportKit(kit);
        const { kit: reKit } = await importKit(exportedZip);
        expect(areKitsEqual(kit, reKit)).toBe(true);
      });
    });
  });

  describe("Validation", () => {
    describe("Metabolism", () => {
      it("Metabolism Kit -> Validate = Empty report", () => {
        const validKit = MetabolismKit as unknown as Kit;
        expect(hasErrors(validateKit(validKit))).toBe(false);
      });
    });

    describe("Invalid", () => {
      it("Invalid Kit -> Validate = Invalid Report", () => {
        const invalidKit = InvalidKit as unknown as Kit;
        const result = validateKit(invalidKit);
        const expected = InvalidKitValidation as unknown as ValidationResult;
        expect(areValidationResultsEqual(result, expected)).toBe(true);
      });
    });
  });

  describe("Cluster", () => {
    it("Cluster replacement uses design-guid designPiece and yields included design entry", () => {
      const design = {
        guid: "design-root",
        name: "Root",
        pieces: [
          { guid: "piece-a", type: { guid: "type-a" } },
          { guid: "piece-b", type: { guid: "type-b" } },
          { guid: "piece-c", type: { guid: "type-c" } },
        ],
        connections: [
          {
            guid: "conn-ab",
            connecting: { piece: { guid: "piece-a" } },
            connected: { piece: { guid: "piece-b" } },
          },
          {
            guid: "conn-bc",
            connecting: { piece: { guid: "piece-b" } },
            connected: { piece: { guid: "piece-c" } },
          },
        ],
        createdAt: "2025-01-01T00:00:00.000Z",
        updatedAt: "2025-01-01T00:00:00.000Z",
      } as Design;

      const { clusteredDesign, externalConnections } = createClusteredDesign(design, ["piece-a", "piece-b"], "Cluster");
      const change = replaceClusterWithDesign(design, ["piece-a", "piece-b"], clusteredDesign, externalConnections);
      const updatedDesign = applyDesignDiff(design, change.forward);

      const clusterConnection = updatedDesign.connections?.find((c) => c.guid === "conn-bc");
      expect(clusterConnection?.connecting.designPiece?.guid).toBe(clusteredDesign.guid);
      expect(clusterConnection?.connected.designPiece?.guid).toBeUndefined();

      const included = getIncludedDesigns(updatedDesign);
      expect(included.length).toBe(1);
      expect(included[0].guid).toBe(clusteredDesign.guid);
      expect(included[0].designGuid).toBe(clusteredDesign.guid);
    });
  });

  describe("Drag", () => {
    it("Design + Pieces + Offset = DiffDesign", () => {
      const design = DragDesign as unknown as Design;
      const pieces = DragPieces as unknown as Design;
      const offset = DragOffset as { u: number; v: number };
      const expectedDiff = DragDiffDesign as any;
      const computedDiff = dragPiecesInDesign(design, pieces, offset);
      const computedPieceUpdates = (computedDiff.pieces?.updated ?? []).sort((a, b) => a.piece.guid.localeCompare(b.piece.guid));
      const expectedPieceUpdates = (expectedDiff.pieces?.updated ?? []).sort((a: any, b: any) => a.piece.guid.localeCompare(b.piece.guid));
      expect(computedPieceUpdates.length).toBe(expectedPieceUpdates.length);
      for (let i = 0; i < computedPieceUpdates.length; i++) {
        expect(computedPieceUpdates[i].piece.guid).toBe(expectedPieceUpdates[i].piece.guid);
        expect(computedPieceUpdates[i].diff.center?.u).toBe(expectedPieceUpdates[i].diff.center.u);
        expect(computedPieceUpdates[i].diff.center?.v).toBe(expectedPieceUpdates[i].diff.center.v);
      }
      const computedConnUpdates = (computedDiff.connections?.updated ?? []).sort((a, b) => a.connection.guid.localeCompare(b.connection.guid));
      const expectedConnUpdates = (expectedDiff.connections?.updated ?? []).sort((a: any, b: any) => a.connection.guid.localeCompare(b.connection.guid));
      expect(computedConnUpdates.length).toBe(expectedConnUpdates.length);
      for (let i = 0; i < computedConnUpdates.length; i++) {
        expect(computedConnUpdates[i].connection.guid).toBe(expectedConnUpdates[i].connection.guid);
        expect(computedConnUpdates[i].diff.u).toBe(expectedConnUpdates[i].diff.u);
        expect(computedConnUpdates[i].diff.v).toBe(expectedConnUpdates[i].diff.v);
      }
    });
  });

  describe("Sketchpad ControlTree", () => {
    it("builds nested folders from paths and applies case-insensitive filter on leaf keys", () => {
      const controls: ControlDef[] = [
        { path: "Transform/Position/X", controlKind: "number", value: 1, onChange: () => { } },
        { path: "Transform/Position/Y", controlKind: "number", value: 2, onChange: () => { } },
        { path: "Appearance/Material/roughness", controlKind: "slider", value: 0.5, onChange: () => { } },
      ];
      const folderSettings = {
        Transform: { path: "Transform", order: 2 },
        "Appearance/Material": { path: "Appearance/Material", order: 1, collapsed: true },
      };
      const fullTree = buildControlTree(controls, "", folderSettings);
      expect(Object.keys(fullTree)).toEqual(["Transform", "Appearance"]);
      expect(fullTree.Transform.kind).toBe("folder");
      expect(fullTree.Transform.order).toBe(2);
      expect(fullTree.Transform.children?.Position.kind).toBe("folder");
      expect(fullTree.Transform.children?.Position.children?.X.kind).toBe("control");
      expect(fullTree.Appearance.children?.Material.order).toBe(1);
      const filteredTree = buildControlTree(controls, "rouGH", folderSettings);
      expect(Object.keys(filteredTree)).toEqual(["Appearance"]);
      expect(filteredTree.Appearance.children?.Material.children?.roughness.kind).toBe("control");
      expect(filteredTree.Appearance.children?.Material.children?.roughness.path).toBe("Appearance/Material/roughness");
    });
  });

  describe("Elements Bundle", () => {
    it("sources shared element primitives directly from elements ui", () => {
      expect(UiAction).toBe(ElementsBundle.Action);
      expect(buildControlTree).toBe(ElementsBundle.buildControlTree);
      expect(ElementsBundle.LevelProvider).toBeDefined();
      expect(ElementsBundle.SectionSpecificity).toBeDefined();
    });

    it("renders an explicit TreeItem label even when an id is present", () => {
      const html = renderToStaticMarkup(
        createElement(ElementsBundle.Tree, {
          sections: [
            {
              id: "test-section",
              items: [
                {
                  id: "storybook.missing.translation.key",
                  label: createElement("span", { className: "tree-explicit-label" }, "Explicit Tree Label"),
                  icon: createElement("span", null, "∧"),
                },
              ],
            },
          ],
        }),
      );

      expect(html).toContain("Explicit Tree Label");
      expect(html).toContain("tree-explicit-label");
    });
  });

  describe("Coda Tree Descriptors", () => {
    let getOntologyNodeDescriptor: any;
    let getValidationNodeDescriptor: any;
    beforeAll(async () => {
      const mod = await import("../../coda/desktop/renderer");
      getOntologyNodeDescriptor = mod.getOntologyNodeDescriptor;
      getValidationNodeDescriptor = mod.getValidationNodeDescriptor;
    });

    it("keeps ontology fragments and validation witness/count semantics stable", () => {
      const ontologyNode: any = {
        id: "ontology-1",
        kind: "ExactCardinality",
        label: "EXACTLY 2 verbindet",
        fragment: "verbindet exactly 2 (...)",
        children: [],
      };
      const ontologyDescriptor = getOntologyNodeDescriptor(ontologyNode);
      expect(ontologyDescriptor.icon).toBe("=n");
      expect(ontologyDescriptor.primaryText).toBe("EXACTLY 2 verbindet");
      expect(ontologyDescriptor.secondaryText).toBe("verbindet exactly 2 (...)");

      const countedWitness: any = {
        id: "validation-1",
        kind: "Witness",
        label: "Geschoss_EG",
        individual: "Geschoss_EG",
        truth: "true",
        counted: true,
        summary: "counted filler 1 of 2",
        children: [],
      };
      const countedWitnessDescriptor = getValidationNodeDescriptor(countedWitness);
      expect(countedWitnessDescriptor.primaryText).toBe("Geschoss_EG");
      expect(countedWitnessDescriptor.chips).toContain("counted");
      expect(countedWitnessDescriptor.dimmed).toBe(false);

      const notMatchingWitness: any = {
        id: "validation-2",
        kind: "Witness",
        label: "Technikraum_Dach",
        individual: "Technikraum_Dach",
        truth: "unknown",
        counted: false,
        summary: "additional filler that does not satisfy the restriction",
        children: [],
      };
      const notMatchingWitnessDescriptor = getValidationNodeDescriptor(notMatchingWitness);
      expect(notMatchingWitnessDescriptor.chips).toContain("not matching");
      expect(notMatchingWitnessDescriptor.dimmed).toBe(true);

      const cardinalityNode: any = {
        id: "validation-3",
        kind: "ExactCardinality",
        label: "EXACTLY 1 in",
        fragment: "in exactly 1 (...)",
        truth: "true",
        expectedCardinality: 1,
        matchingCount: 1,
        children: [],
      };
      const cardinalityDescriptor = getValidationNodeDescriptor(cardinalityNode);
      expect(cardinalityDescriptor.icon).toBe("=n");
      expect(cardinalityDescriptor.chips).toContain("1/1");
      expect(cardinalityDescriptor.secondaryText).toBe("in exactly 1 (...)");

      const dataValueNode: any = {
        id: "validation-4",
        kind: "DataValue",
        label: "180.0",
        value: "180.0",
        datatype: "xsd:float",
        truth: "true",
        children: [],
      };
      const dataValueDescriptor = getValidationNodeDescriptor(dataValueNode);
      expect(dataValueDescriptor.primaryText).toBe("180.0");
      expect(dataValueDescriptor.chips).toContain("xsd:float");
    });
  });

  describe("Design/Quality/Sum", () => {
    const kit = MetabolismKit as Kit;
    describe("Nakagin Capsule Tower", () => {
      it("sums effective floor area to ~2349.53", () => {
        const design = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent);
        expect(design).toBeDefined();
        const quality = kit.qualities?.find((q) => q.name === "effective floor area");
        expect(quality).toBeDefined();
        const result = sumQualityInDesign(kit, design!.guid, quality!.guid);
        expect(Math.abs(result - 2349.53)).toBeLessThan(0.01);
      });
    });
  });

  describe("ExportDesignModel", () => {
    const kit = MetabolismKit as Kit;
    const design = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent)!;

    it("exports .glb format with valid GLB header", async () => {
      const result = await exportDesignModel(kit, design.guid, ".glb");
      expect(result.byteLength).toBeGreaterThan(0);

      const view = new DataView(result);
      const magic = view.getUint32(0, true);
      expect(magic).toBe(0x46546c67);

      const version = view.getUint32(4, true);
      expect(version).toBe(2);

      const totalLength = view.getUint32(8, true);
      expect(totalLength).toBe(result.byteLength);
    });

    it("exports .gltf format as valid JSON string", async () => {
      const result = await exportDesignModel(kit, design.guid, ".gltf");
      const decoder = new TextDecoder();
      const str = decoder.decode(result);
      expect(() => JSON.parse(str)).not.toThrow();
      const parsed = JSON.parse(str);
      expect(parsed).toBeDefined();
      expect(typeof parsed).toBe("object");
    });

    it("EXPORT_MODEL_FORMATS includes .glb and .gltf", () => {
      expect(EXPORT_MODEL_FORMATS[".glb"]).toBeDefined();
      expect(EXPORT_MODEL_FORMATS[".gltf"]).toBeDefined();
    });

    it("exports identical Nakagin scene graph across implementations and writes reports", async () => {
      const { mkdirSync, readFileSync, writeFileSync } = await import("node:fs");
      const { EXPORT_REPORTS_DIR, resolve, __dirname } = await getTestNodePaths();
      mkdirSync(EXPORT_REPORTS_DIR, { recursive: true });

      const jsResult = new Uint8Array(await exportDesignModel(kit, design.guid, ".gltf"));
      await writeExportReport("js", jsResult);

      await runExportReportCommand("uv", ["run", "pytest", "main.py", "-k", "export_scene_graph_report", "-q"], resolve(__dirname, "../py"));
      let skipGo = false;
      try {
        await runExportReportCommand("go", ["test", "./...", "-run", "TestExportDesignModelSceneGraphReport$", "-count=1"], resolve(__dirname, "../go"));
      } catch (e: any) {
        const message = String(e?.message ?? e);
        const looksLikeGoToolchainMismatch = message.includes("requires go >= 1.25.0") && message.includes("go.work lists go 1.24.0");
        if (looksLikeGoToolchainMismatch) {
          // [DEBUG] This repository's Go modules require a newer Go toolchain than the one installed in some CI/dev containers.
          // Skip the cross-implementation "go" comparison in that case; other implementations still run.
          // eslint-disable-next-line no-console
          console.warn(`[DEBUG] skipping go ExportDesignModelSceneGraphReport due to Go toolchain mismatch: ${message}`);
          skipGo = true;
        } else {
          throw e;
        }
      }
      await runExportReportCommand("cargo", ["test", "export_scene_graph_report", "--", "--nocapture"], resolve(__dirname, "../rs"));
      await runExportReportCommand(
        "dotnet",
        ["test", "Semio.Tests.csproj", "-f", "net8.0", "--filter", "FullyQualifiedName=Semio.Tests.Tests+ExportDesignModel.Nakagin_Capsule_Tower_Export_Scene_Graph_Report"],
        resolve(__dirname, "../net/Semio.Tests"),
      );

      const implementations = (skipGo ? (["js", "py", "rs", "net"] as const) : (["js", "py", "go", "rs", "net"] as const)) as const;
      const normalizedByImplementation = Object.fromEntries(
        implementations.map((implementation) => {
          const reportPath = resolve(EXPORT_REPORTS_DIR, `${implementation}.gltf`);
          const reportText = readFileSync(reportPath, "utf8");
          return [implementation, normalizeSceneGraph(reportText)];
        }),
      );

      writeFileSync(resolve(EXPORT_REPORTS_DIR, "scene-graphs.json"), JSON.stringify(normalizedByImplementation, null, 2));

      const baseline = normalizedByImplementation.js;
      for (const implementation of implementations) {
        expect(normalizedByImplementation[implementation]).toEqual(baseline);
      }

      for (const implementation of implementations) {
        const reportPath = resolve(EXPORT_REPORTS_DIR, `${implementation}.gltf`);
        const reportText = readFileSync(reportPath, "utf8");
        const parsed = JSON.parse(reportText) as { buffers?: Array<{ uri?: string }>; images?: Array<{ uri?: string; bufferView?: number }> };
        for (const buffer of parsed.buffers ?? []) {
          expect(buffer.uri?.startsWith("data:")).toBe(true);
        }
        for (const image of parsed.images ?? []) {
          expect(image.uri?.startsWith("data:") ?? image.bufferView !== undefined).toBe(true);
        }
        const doc = await parseSelfContainedGltf(reportText);
        expect(doc.getRoot().listMeshes().length).toBeGreaterThan(0);
        const meshNames = getMeshNames(reportText);
        expect(meshNames.some((name) => name === "base.glb")).toBe(true);
        expect(meshNames.some((name) => /^capsule_.*\.glb$/i.test(name))).toBe(true);
      }
    }, 300000);
  });

  describe("Model/KPI", () => {
    it("getGeometricInsightsForModel(nakagin-capsule-tower.gltf) returns canonical insights and writes report", async () => {
      const fs = await import("node:fs/promises");
      const { resolve, __dirname } = await getTestNodePaths();
      const modelPath = resolve(__dirname, "../assets/semio/nakagin-capsule-tower.gltf");
      const insights = await getGeometricInsightsForModel(modelPath);
      const round6 = (x: number) => Math.round(x * 1e6) / 1e6;
      const pt = (p: { x: number; y: number; z: number } | undefined) => (p ? { x: round6(p.x), y: round6(p.y), z: round6(p.z) } : undefined);

      const reportsDir = resolve(__dirname, "../reports/model-kpi");
      await fs.mkdir(reportsDir, { recursive: true });
      const report: Record<string, unknown> = {
        aspect_ratio_xy: insights.aspectRatioXy != null ? round6(insights.aspectRatioXy) : undefined,
        aspect_ratio_xz: insights.aspectRatioXz != null ? round6(insights.aspectRatioXz) : undefined,
        aspect_ratio_yz: insights.aspectRatioYz != null ? round6(insights.aspectRatioYz) : undefined,
        bounding_box_max: pt(insights.boundingBoxMax),
        bounding_box_min: pt(insights.boundingBoxMin),
        centroid: pt(insights.centroid),
        characteristic_length: insights.characteristicLength != null ? round6(insights.characteristicLength) : undefined,
        dimension_x: insights.dimensionX != null ? round6(insights.dimensionX) : undefined,
        dimension_y: insights.dimensionY != null ? round6(insights.dimensionY) : undefined,
        dimension_z: insights.dimensionZ != null ? round6(insights.dimensionZ) : undefined,
        face_count: insights.faceCount,
        footprint_area: insights.footprintArea != null ? round6(insights.footprintArea) : undefined,
        is_watertight: insights.isWatertight ?? false,
        slenderness: insights.slenderness != null ? round6(insights.slenderness) : undefined,
        total_surface_area: insights.totalSurfaceArea != null ? round6(insights.totalSurfaceArea) : undefined,
        vertex_count: insights.vertexCount,
      };
      await fs.writeFile(resolve(reportsDir, "js.json"), JSON.stringify(report, null, 2), "utf8");

      const canonicalPath = resolve(__dirname, "../assets/semio/model-kpi-nakagin.json");
      const canonical = JSON.parse(await fs.readFile(canonicalPath, "utf8"));
      const skipKeys = new Set(["centroid", "total_surface_area"]);
      for (const key of Object.keys(canonical)) {
        if (skipKeys.has(key)) continue;
        expect(report[key]).toBeDefined();
        expect(report[key]).toEqual(canonical[key]);
      }
    });
  });

  // #region 🔖InMemoryKitStore Tests
  // [👤semio📚js🥼semiotest🔖inmemorykitstoretests](repo://p/u/semio/b/l/js/f/semio.test.ts/s/InMemoryKitStoreTests)
  // Contract tests for InMemoryKitStore MUST verify the full KitStore interface.

  describe("InMemoryKitStore", () => {
    const makeKit = (overrides?: Partial<Kit>): Kit => ({
      guid: "test-kit-guid",
      name: "Test Kit",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      ...overrides,
    });

    it("getSnapshot returns the initial kit and ready status", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.guid).toBe("test-kit-guid");
      expect(snapshot.kit.name).toBe("Test Kit");
      expect(snapshot.sync.status).toBe("ready");
      expect(snapshot.sync.dirty).toBe(false);
      expect(snapshot.sync.readonly).toBe(false);
    });

    it("apply merges a diff and notifies subscribers", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let notified = 0;
      store.subscribe(() => notified++);

      const diff: KitDiff = { name: "Updated Kit" };
      store.apply(diff);

      expect(store.getSnapshot().kit.name).toBe("Updated Kit");
      expect(store.getSnapshot().sync.dirty).toBe(true);
      expect(notified).toBe(1);
    });

    it("replace swaps the entire kit and notifies subscribers", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let notified = 0;
      store.subscribe(() => notified++);

      const newKit = makeKit({ guid: "new-guid", name: "Replaced Kit" });
      store.replace(newKit);

      expect(store.getSnapshot().kit.guid).toBe("new-guid");
      expect(store.getSnapshot().kit.name).toBe("Replaced Kit");
      expect(store.getSnapshot().sync.dirty).toBe(true);
      expect(notified).toBe(1);
    });

    it("subscribe returns an unsubscribe function", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let notified = 0;
      const unsub = store.subscribe(() => notified++);

      store.apply({ name: "First" });
      expect(notified).toBe(1);

      unsub();
      store.apply({ name: "Second" });
      expect(notified).toBe(1);
    });

    it("transact groups mutations into one undo entry", () => {
      const kit = makeKit({ types: [] });
      const store = new InMemoryKitStore(kit);

      store.transact("add type and rename", () => {
        store.apply({ name: "Renamed" });
        store.apply({
          types: {
            added: [{ guid: "t1", name: "Wall", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() }],
          },
        });
      });

      const snap = store.getSnapshot();
      expect(snap.kit.name).toBe("Renamed");
      expect(snap.kit.types).toHaveLength(1);
      expect(store.canUndo()).toBe(true);

      store.undo();
      const undone = store.getSnapshot();
      expect(undone.kit.name).toBe("Test Kit");
      expect(undone.kit.types ?? []).toHaveLength(0);
    });

    it("undo reverses the last mutation", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.apply({ name: "Changed" });
      expect(store.getSnapshot().kit.name).toBe("Changed");
      expect(store.canUndo()).toBe(true);
      expect(store.canRedo()).toBe(false);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Test Kit");
      expect(store.canUndo()).toBe(false);
      expect(store.canRedo()).toBe(true);
    });

    it("redo re-applies the last undone mutation", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.apply({ name: "Changed" });
      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Test Kit");

      store.redo();
      expect(store.getSnapshot().kit.name).toBe("Changed");
      expect(store.canUndo()).toBe(true);
      expect(store.canRedo()).toBe(false);
    });

    it("apply after undo clears the redo stack", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.apply({ name: "First" });
      store.apply({ name: "Second" });
      store.undo();
      expect(store.canRedo()).toBe(true);

      store.apply({ name: "Third" });
      expect(store.canRedo()).toBe(false);
      expect(store.getSnapshot().kit.name).toBe("Third");
    });

    it("save clears dirty flag", async () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.apply({ name: "Changed" });
      expect(store.getSnapshot().sync.dirty).toBe(true);

      await store.save();
      expect(store.getSnapshot().sync.dirty).toBe(false);
    });

    it("dispose clears all listeners and stacks", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let notified = 0;
      store.subscribe(() => notified++);

      store.apply({ name: "Before dispose" });
      expect(notified).toBe(1);

      store.dispose();
      store.apply({ name: "After dispose" });
      expect(notified).toBe(1);
      expect(store.canUndo()).toBe(false);
    });

    it("multiple subscribers are all notified", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let count1 = 0;
      let count2 = 0;
      store.subscribe(() => count1++);
      store.subscribe(() => count2++);

      store.apply({ name: "Changed" });
      expect(count1).toBe(1);
      expect(count2).toBe(1);
    });

    it("undo and redo with no stack are no-ops", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Test Kit");

      store.redo();
      expect(store.getSnapshot().kit.name).toBe("Test Kit");
    });
  });

  // #endregion 🔖InMemoryKitStore Tests

  // #region 🔖JsonFileKitStore Tests
  // [👤semio📚js🥼semiotest🔖jsonfilekitstoretests](repo://p/u/semio/b/l/js/f/semio.test.ts/s/JsonFileKitStoreTests)
  // Contract tests for JsonFileKitStore MUST verify the full UndoableKitStore interface
  // including file I/O, save, reload, undo/redo, and external update handling.

  describe("JsonFileKitStore", () => {
    const makeKit = (overrides?: Partial<Kit>): Kit => ({
      guid: "file-kit-guid",
      name: "File Kit",
      createdAt: "2026-01-01T00:00:00.000Z",
      updatedAt: "2026-01-01T00:00:00.000Z",
      ...overrides,
    });

    const makeAdapter = (initialKit?: Kit): KitJsonFileAdapter & { stored: string | null } => {
      const adapter = {
        stored: initialKit ? JSON.stringify(initialKit) : null,
        async read(): Promise<string | null> {
          return adapter.stored;
        },
        async write(json: string): Promise<void> {
          adapter.stored = json;
        },
      };
      return adapter;
    };

    it("loads kit from adapter and reports ready status", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      const store = await createJsonFileKitStore(adapter);
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.guid).toBe("file-kit-guid");
      expect(snapshot.kit.name).toBe("File Kit");
      expect(snapshot.sync.status).toBe("ready");
      expect(snapshot.sync.dirty).toBe(false);
      expect(snapshot.sync.lastSyncedAt).toBeDefined();
    });

    it("creates empty kit when adapter returns null", async () => {
      const adapter = makeAdapter();
      const store = await createJsonFileKitStore(adapter);
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.guid).toBeDefined();
      expect(snapshot.kit.name).toBe("New Kit");
      expect(snapshot.sync.status).toBe("ready");
    });

    it("reports error status for invalid JSON", async () => {
      const adapter = {
        stored: "not valid json {{{",
        async read() {
          return adapter.stored;
        },
        async write(json: string) {
          adapter.stored = json;
        },
      };
      const store = await createJsonFileKitStore(adapter);
      expect(store.getSnapshot().sync.status).toBe("error");
      expect(store.getSnapshot().sync.error).toBeDefined();
    });

    it("apply merges a diff and notifies subscribers", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));
      let notified = 0;
      store.subscribe(() => notified++);

      store.apply({ name: "Updated" });
      expect(store.getSnapshot().kit.name).toBe("Updated");
      expect(store.getSnapshot().sync.dirty).toBe(true);
      expect(notified).toBe(1);
    });

    it("replace swaps the entire kit", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));
      let notified = 0;
      store.subscribe(() => notified++);

      const newKit = makeKit({ guid: "new-guid", name: "Replaced" });
      store.replace(newKit);
      expect(store.getSnapshot().kit.guid).toBe("new-guid");
      expect(store.getSnapshot().kit.name).toBe("Replaced");
      expect(notified).toBe(1);
    });

    it("save writes kit JSON to adapter and clears dirty", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      const store = await createJsonFileKitStore(adapter);

      store.apply({ name: "Saved Kit" });
      expect(store.getSnapshot().sync.dirty).toBe(true);

      await store.save();
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.getSnapshot().sync.status).toBe("ready");

      const savedKit = JSON.parse(adapter.stored!);
      expect(savedKit.name).toBe("Saved Kit");
    });

    it("reload re-reads kit from adapter and resets state", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      const store = await createJsonFileKitStore(adapter);

      store.apply({ name: "Local Change" });
      expect(store.getSnapshot().kit.name).toBe("Local Change");

      // Simulate external file change
      adapter.stored = JSON.stringify(makeKit({ name: "External Change" }));

      await store.reload();
      expect(store.getSnapshot().kit.name).toBe("External Change");
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.canUndo()).toBe(false);
    });

    it("undo reverses the last mutation", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));

      store.apply({ name: "Changed" });
      expect(store.canUndo()).toBe(true);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("File Kit");
      expect(store.canRedo()).toBe(true);
    });

    it("redo re-applies the last undone mutation", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));

      store.apply({ name: "Changed" });
      store.undo();
      store.redo();
      expect(store.getSnapshot().kit.name).toBe("Changed");
      expect(store.canUndo()).toBe(true);
      expect(store.canRedo()).toBe(false);
    });

    it("transact groups mutations into one undo entry", async () => {
      const kit = makeKit({ types: [] });
      const store = await createJsonFileKitStore(makeAdapter(kit));

      store.transact("batch", () => {
        store.apply({ name: "Renamed" });
        store.apply({
          types: {
            added: [{ guid: "t1", name: "Wall", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
          },
        });
      });

      expect(store.getSnapshot().kit.name).toBe("Renamed");
      expect(store.getSnapshot().kit.types).toHaveLength(1);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("File Kit");
      expect(store.getSnapshot().kit.types ?? []).toHaveLength(0);
    });

    it("subscribe returns unsubscribe function", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));
      let notified = 0;
      const unsub = store.subscribe(() => notified++);

      store.apply({ name: "First" });
      expect(notified).toBe(1);

      unsub();
      store.apply({ name: "Second" });
      expect(notified).toBe(1);
    });

    it("dispose clears listeners and stacks", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));
      let notified = 0;
      store.subscribe(() => notified++);

      store.apply({ name: "Before" });
      expect(notified).toBe(1);

      store.dispose();
      store.apply({ name: "After" });
      expect(notified).toBe(1);
      expect(store.canUndo()).toBe(false);
    });

    it("applyExternalUpdate resets state without undo entry", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));

      store.apply({ name: "Local" });
      expect(store.canUndo()).toBe(true);

      store.applyExternalUpdate(makeKit({ name: "External" }));
      expect(store.getSnapshot().kit.name).toBe("External");
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.canUndo()).toBe(false);
    });

    it("save transitions through saving status", async () => {
      const kit = makeKit();
      const statuses: string[] = [];
      const store = await createJsonFileKitStore(makeAdapter(kit));
      store.subscribe(() => statuses.push(store.getSnapshot().sync.status));

      store.apply({ name: "Changed" });
      await store.save();

      expect(statuses).toContain("saving");
      expect(store.getSnapshot().sync.status).toBe("ready");
    });
  });

  // #endregion 🔖JsonFileKitStore Tests

  // #region 🔖FolderKitStore Tests
  describe("FolderKitStore", () => {
    const makeKit = (overrides?: Partial<Kit>): Kit => ({
      guid: "folder-kit-guid",
      name: "Folder Kit",
      createdAt: "2026-01-01T00:00:00.000Z",
      updatedAt: "2026-01-01T00:00:00.000Z",
      ...overrides,
    });

    const kitToBytes = async (kit: Kit): Promise<Uint8Array> => {
      const SQL = await getSqlJs();
      const db = new SQL.Database();
      await kitToSqlite(kit, db);
      const data = db.export();
      db.close();
      return data;
    };

    const bytesToKit = async (data: Uint8Array): Promise<Kit> => {
      const SQL = await getSqlJs();
      const db = new SQL.Database(new Uint8Array(data));
      const kit = await sqliteToKit(db);
      db.close();
      return kit;
    };

    const makeAdapter = (initialKit?: Kit): KitFolderAdapter & { stored: Uint8Array | null; files: Map<string, Blob>; initPromise: Promise<void> } => {
      const adapter = {
        stored: null as Uint8Array | null,
        files: new Map<string, Blob>(),
        initPromise: Promise.resolve(),
        async readKit(): Promise<Uint8Array | null> {
          return adapter.stored;
        },
        async writeKit(data: Uint8Array): Promise<void> {
          adapter.stored = data;
        },
        async readFile(path: string): Promise<Blob | null> {
          return adapter.files.get(path) ?? null;
        },
        async writeFile(path: string, blob: Blob): Promise<void> {
          adapter.files.set(path, blob);
        },
        async deleteFile(path: string): Promise<void> {
          adapter.files.delete(path);
        },
        async listFiles(): Promise<string[]> {
          return Array.from(adapter.files.keys());
        },
      };
      if (initialKit) {
        adapter.initPromise = kitToBytes(initialKit).then((data) => {
          adapter.stored = data;
        });
      }
      return adapter;
    };

    it("loads kit from adapter and reports ready status", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.guid).toBe("folder-kit-guid");
      expect(snapshot.kit.name).toBe("Folder Kit");
      expect(snapshot.sync.status).toBe("ready");
      expect(snapshot.sync.dirty).toBe(false);
    });

    it("creates empty kit when adapter returns null", async () => {
      const store = await createFolderKitStore(makeAdapter());
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.guid).toBeDefined();
      expect(snapshot.kit.name).toBe("New Kit");
      expect(snapshot.sync.status).toBe("ready");
    });

    it("reports error status for invalid SQLite data", async () => {
      const adapter = makeAdapter();
      adapter.stored = new Uint8Array([0, 1, 2, 3]);
      const store = await createFolderKitStore(adapter);
      expect(store.getSnapshot().sync.status).toBe("error");
      expect(store.getSnapshot().sync.error).toBeDefined();
    });

    it("apply merges a diff and notifies subscribers", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      let notified = 0;
      store.subscribe(() => notified++);

      store.apply({ name: "Updated" });
      expect(store.getSnapshot().kit.name).toBe("Updated");
      expect(store.getSnapshot().sync.dirty).toBe(true);
      expect(notified).toBe(1);
    });

    it("replace swaps the entire kit", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      let notified = 0;
      store.subscribe(() => notified++);

      const newKit = makeKit({ guid: "new-guid", name: "Replaced" });
      store.replace(newKit);
      expect(store.getSnapshot().kit.guid).toBe("new-guid");
      expect(store.getSnapshot().kit.name).toBe("Replaced");
      expect(notified).toBe(1);
    });

    it("save writes kit SQLite to adapter and clears dirty", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Saved Kit" });
      expect(store.getSnapshot().sync.dirty).toBe(true);

      await store.save();
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.getSnapshot().sync.status).toBe("ready");

      const savedKit = await bytesToKit(adapter.stored!);
      expect(savedKit.name).toBe("Saved Kit");
    });

    it("reload re-reads kit from adapter and resets state", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Local Change" });
      expect(store.getSnapshot().kit.name).toBe("Local Change");

      adapter.stored = await kitToBytes(makeKit({ name: "External Change" }));

      await store.reload();
      expect(store.getSnapshot().kit.name).toBe("External Change");
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.canUndo()).toBe(false);
    });

    it("undo reverses the last mutation", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Changed" });
      expect(store.canUndo()).toBe(true);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Folder Kit");
      expect(store.canRedo()).toBe(true);
    });

    it("redo re-applies the last undone mutation", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Changed" });
      store.undo();
      store.redo();
      expect(store.getSnapshot().kit.name).toBe("Changed");
      expect(store.canUndo()).toBe(true);
      expect(store.canRedo()).toBe(false);
    });

    it("transact groups mutations into one undo entry", async () => {
      const kit = makeKit({ types: [] });
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.transact("batch", () => {
        store.apply({ name: "Renamed" });
        store.apply({
          types: {
            added: [{ guid: "t1", name: "Wall", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
          },
        });
      });

      expect(store.getSnapshot().kit.name).toBe("Renamed");
      expect(store.getSnapshot().kit.types).toHaveLength(1);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Folder Kit");
      expect(store.getSnapshot().kit.types ?? []).toHaveLength(0);
    });

    it("subscribe returns unsubscribe function", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      let notified = 0;
      const unsub = store.subscribe(() => notified++);

      store.apply({ name: "First" });
      expect(notified).toBe(1);

      unsub();
      store.apply({ name: "Second" });
      expect(notified).toBe(1);
    });

    it("dispose clears listeners and stacks", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      let notified = 0;
      store.subscribe(() => notified++);

      store.apply({ name: "Before" });
      expect(notified).toBe(1);

      store.dispose();
      store.apply({ name: "After" });
      expect(notified).toBe(1);
      expect(store.canUndo()).toBe(false);
    });

    it("applyExternalUpdate resets state without undo entry", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Local" });
      expect(store.canUndo()).toBe(true);

      store.applyExternalUpdate(makeKit({ name: "External" }));
      expect(store.getSnapshot().kit.name).toBe("External");
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.canUndo()).toBe(false);
    });

    it("writeFile and readFile roundtrip via adapter", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      const blob = new Blob(["hello world"], { type: "text/plain" });
      await store.writeFile("test.txt", blob);

      const read = await store.readFile("test.txt");
      expect(read).not.toBeNull();
      const text = await read!.text();
      expect(text).toBe("hello world");
    });

    it("deleteFile removes a stored file", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      await store.writeFile("to-delete.txt", new Blob(["data"]));
      expect(await store.readFile("to-delete.txt")).not.toBeNull();

      await store.deleteFile("to-delete.txt");
      expect(await store.readFile("to-delete.txt")).toBeNull();
    });

    it("listFiles returns all file paths", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      await store.writeFile("a.txt", new Blob(["a"]));
      await store.writeFile("b.txt", new Blob(["b"]));

      const files = await store.listFiles();
      expect(files).toContain("a.txt");
      expect(files).toContain("b.txt");
      expect(files).toHaveLength(2);
    });

    it("save transitions through saving status", async () => {
      const kit = makeKit();
      const statuses: string[] = [];
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      store.subscribe(() => statuses.push(store.getSnapshot().sync.status));

      store.apply({ name: "Changed" });
      await store.save();

      expect(statuses).toContain("saving");
      expect(store.getSnapshot().sync.status).toBe("ready");
    });
  });
  // #endregion 🔖FolderKitStore Tests

  // #region 🔖SketchpadStore Kit Store Factory Tests
  // [👤semio📚js🥼semiotest🔖sketchpadstorekitstorefactorytests](repo://p/u/semio/b/l/js/f/semio.test.ts/s/SketchpadStoreKitStoreFactoryTests)
  // Tests for SketchpadStore.availableKitKinds() and createKit with optional factory props.

  describe("SketchpadStore availableKitKinds", () => {
    let SketchpadStore: any;
    beforeAll(async () => {
      const mod = await import("../sketchpad/index");
      SketchpadStore = mod.SketchpadStore;
    });
    const dummyFactory: any = (kit: Kit) => new InMemoryKitStore(kit);

    it("returns temporary=true when no factories and no injected store", () => {
      const store = new SketchpadStore();
      const kinds = store.availableKitKinds();
      expect(kinds.temporary).toBe(true);
      expect(kinds.local).toBe(false);
      expect(kinds.remote).toBe(false);
    });

    it("returns temporary=true when only temporaryKitStoreFactory is provided", () => {
      const store = new SketchpadStore(
        undefined, // id
        undefined, // remote
        undefined, // initialState
        undefined, // persistenceFactory
        undefined, // injectedKitStore
        dummyFactory, // temporaryKitStoreFactory
      );
      const kinds = store.availableKitKinds();
      expect(kinds.temporary).toBe(true);
      expect(kinds.local).toBe(false);
      expect(kinds.remote).toBe(false);
    });

    it("returns local=true when folderKitStoreFactory is provided", () => {
      const store = new SketchpadStore(
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        undefined, // temporaryKitStoreFactory
        dummyFactory, // folderKitStoreFactory
      );
      const kinds = store.availableKitKinds();
      expect(kinds.temporary).toBe(false);
      expect(kinds.local).toBe(true);
      expect(kinds.remote).toBe(false);
    });

    it("returns local=true when fileKitStoreFactory is provided", () => {
      const store = new SketchpadStore(
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        undefined, // temporaryKitStoreFactory
        undefined, // folderKitStoreFactory
        dummyFactory, // fileKitStoreFactory
      );
      const kinds = store.availableKitKinds();
      expect(kinds.temporary).toBe(false);
      expect(kinds.local).toBe(true);
      expect(kinds.remote).toBe(false);
    });

    it("returns remote=true when remoteKitStoreFactory is provided", () => {
      const store = new SketchpadStore(
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        undefined, // temporaryKitStoreFactory
        undefined, // folderKitStoreFactory
        undefined, // fileKitStoreFactory
        dummyFactory, // remoteKitStoreFactory
      );
      const kinds = store.availableKitKinds();
      expect(kinds.temporary).toBe(false);
      expect(kinds.local).toBe(false);
      expect(kinds.remote).toBe(true);
    });

    it("returns all kinds when all factories are provided", () => {
      const store = new SketchpadStore(
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        dummyFactory, // temporaryKitStoreFactory
        dummyFactory, // folderKitStoreFactory
        undefined, // fileKitStoreFactory
        dummyFactory, // remoteKitStoreFactory
      );
      const kinds = store.availableKitKinds();
      expect(kinds.temporary).toBe(true);
      expect(kinds.local).toBe(true);
      expect(kinds.remote).toBe(true);
    });

    it("returns local=true when both folder and file factories are provided", () => {
      const store = new SketchpadStore(
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        undefined, // temporaryKitStoreFactory
        dummyFactory, // folderKitStoreFactory
        dummyFactory, // fileKitStoreFactory
      );
      const kinds = store.availableKitKinds();
      expect(kinds.temporary).toBe(false);
      expect(kinds.local).toBe(true);
      expect(kinds.remote).toBe(false);
    });

    it("infers availability from injected kit store when no factories provided", () => {
      const injectedStore = new InMemoryKitStore({ guid: "test", name: "Test" });
      const store = new SketchpadStore(undefined, undefined, undefined, undefined, injectedStore);
      const kinds = store.availableKitKinds();
      // InMemoryKitStore is inferred as temporary (non-local, non-remote)
      expect(kinds.temporary).toBe(true);
      expect(kinds.local).toBe(false);
      expect(kinds.remote).toBe(false);
    });

    it("uses factories over injected store when both provided", () => {
      const injectedStore = new InMemoryKitStore({ guid: "test", name: "Test" });
      const store = new SketchpadStore(
        undefined,
        undefined,
        undefined,
        undefined,
        injectedStore,
        undefined, // temporaryKitStoreFactory
        dummyFactory, // folderKitStoreFactory
      );
      const kinds = store.availableKitKinds();
      // Factories take precedence: no temporary factory => false, folder factory => local true
      expect(kinds.temporary).toBe(false);
      expect(kinds.local).toBe(true);
      expect(kinds.remote).toBe(false);
    });
  });

  describe("SketchpadStore createKit with factories", () => {
    let SketchpadStore: any;
    beforeAll(async () => {
      const mod = await import("../sketchpad/index");
      SketchpadStore = mod.SketchpadStore;
    });
    const dummyFactory: any = (kit: Kit) => new InMemoryKitStore(kit);

    it("creates a kit using temporaryKitStoreFactory", async () => {
      const createdKits: Kit[] = [];
      const trackingFactory: any = (kit) => {
        createdKits.push(kit);
        return new InMemoryKitStore(kit);
      };
      const store = new SketchpadStore(undefined, undefined, undefined, undefined, undefined, trackingFactory);
      const kit: Kit = { guid: "new-kit", name: "New Kit" };
      await store.createKit(kit, false, false);
      expect(createdKits).toHaveLength(1);
      expect(createdKits[0].guid).toBe("new-kit");
    });

    it("creates a kit using folderKitStoreFactory for local", async () => {
      const createdKits: Kit[] = [];
      const trackingFactory: any = (kit) => {
        createdKits.push(kit);
        return new InMemoryKitStore(kit);
      };
      const store = new SketchpadStore(
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        trackingFactory, // folderKitStoreFactory
      );
      const kit: Kit = { guid: "local-kit", name: "Local Kit" };
      await store.createKit(kit, true, false);
      expect(createdKits).toHaveLength(1);
      expect(createdKits[0].guid).toBe("local-kit");
    });

    it("creates a kit using remoteKitStoreFactory for remote", async () => {
      const createdKits: Kit[] = [];
      const trackingFactory: any = (kit) => {
        createdKits.push(kit);
        return new InMemoryKitStore(kit);
      };
      const store = new SketchpadStore(
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        trackingFactory, // remoteKitStoreFactory
      );
      const kit: Kit = { guid: "remote-kit", name: "Remote Kit" };
      await store.createKit(kit, false, true);
      expect(createdKits).toHaveLength(1);
      expect(createdKits[0].guid).toBe("remote-kit");
    });

    it("falls back to InMemoryKitStore when no factory matches", async () => {
      const store = new SketchpadStore();
      const kit: Kit = { guid: "fallback-kit", name: "Fallback Kit" };
      await store.createKit(kit, false, false);
      // Kit is registered; if it weren't, an error would be thrown
      expect(store.hasKit("fallback-kit")).toBe(true);
    });

    it("prefers folderKitStoreFactory over fileKitStoreFactory for local kits", async () => {
      const folderKits: Kit[] = [];
      const fileKits: Kit[] = [];
      const folderFactory: any = (kit) => {
        folderKits.push(kit);
        return new InMemoryKitStore(kit);
      };
      const fileFactory: any = (kit) => {
        fileKits.push(kit);
        return new InMemoryKitStore(kit);
      };
      const store = new SketchpadStore(undefined, undefined, undefined, undefined, undefined, undefined, folderFactory, fileFactory);
      const kit: Kit = { guid: "local-kit", name: "Local Kit" };
      await store.createKit(kit, true, false);
      expect(folderKits).toHaveLength(1);
      expect(fileKits).toHaveLength(0);
    });
  });
  // #endregion 🔖SketchpadStore Kit Store Factory Tests

  // #region 🔖SketchpadStore Auto Save Tests
  describe("SketchpadStore auto-save on registerKitStore", () => {
    let SketchpadStore: any;
    beforeAll(async () => {
      const mod = await import("../sketchpad/index");
      SketchpadStore = mod.SketchpadStore;
    });

    it("auto-saves when kit store becomes dirty after apply", async () => {
      const saveCalls: number[] = [];
      const kitStore = new InMemoryKitStore({ guid: "auto-save-kit", name: "Auto Save Kit" });
      const originalSave = kitStore.save.bind(kitStore);
      kitStore.save = async () => {
        saveCalls.push(Date.now());
        await originalSave();
      };
      const store = new SketchpadStore();
      await store.createKit({ guid: "auto-save-kit-2", name: "Placeholder" }, false, false);
      // Directly register the tracked kit store
      (store as any).registerKitStore(kitStore, false, false);
      // Apply a diff to make it dirty
      kitStore.apply({ types: { add: [{ guid: "t1", name: "TestType" }] } });
      expect(kitStore.getSnapshot().sync.dirty).toBe(true);
      // Wait for debounce
      await new Promise((resolve) => setTimeout(resolve, 500));
      expect(saveCalls.length).toBeGreaterThanOrEqual(1);
      expect(kitStore.getSnapshot().sync.dirty).toBe(false);
    });

    it("debounces rapid changes into a single save", async () => {
      const saveCalls: number[] = [];
      const kitStore = new InMemoryKitStore({ guid: "debounce-kit", name: "Debounce Kit" });
      const originalSave = kitStore.save.bind(kitStore);
      kitStore.save = async () => {
        saveCalls.push(Date.now());
        await originalSave();
      };
      const store = new SketchpadStore();
      (store as any).registerKitStore(kitStore, false, false);
      // Rapid applies
      kitStore.apply({ types: { add: [{ guid: "t1", name: "T1" }] } });
      kitStore.apply({ types: { add: [{ guid: "t2", name: "T2" }] } });
      kitStore.apply({ types: { add: [{ guid: "t3", name: "T3" }] } });
      // Wait for debounce
      await new Promise((resolve) => setTimeout(resolve, 500));
      // Should have debounced to a single save
      expect(saveCalls.length).toBe(1);
    });

    it("does not auto-save when store status is not ready", async () => {
      const saveCalls: number[] = [];
      const kitStore = new InMemoryKitStore({ guid: "status-kit", name: "Status Kit" });
      const originalSave = kitStore.save.bind(kitStore);
      kitStore.save = async () => {
        saveCalls.push(Date.now());
        await originalSave();
      };
      // Simulate a non-ready status
      (kitStore as any).status = "saving";
      const store = new SketchpadStore();
      (store as any).registerKitStore(kitStore, false, false);
      kitStore.apply({ types: { add: [{ guid: "t1", name: "T1" }] } });
      await new Promise((resolve) => setTimeout(resolve, 500));
      expect(saveCalls.length).toBe(0);
    });

    it("auto-saves injected kitStore from desktop", async () => {
      const saveCalls: number[] = [];
      const kitStore = new InMemoryKitStore({ guid: "desktop-kit", name: "Desktop Kit" });
      const originalSave = kitStore.save.bind(kitStore);
      kitStore.save = async () => {
        saveCalls.push(Date.now());
        await originalSave();
      };
      // Mark as local (FolderKitStore)
      (kitStore as any).__semioKitPersistenceKind = { local: true, remote: false };
      const store = new SketchpadStore(undefined, undefined, undefined, undefined, kitStore);
      // Wait for loadPersistedKits to complete
      await new Promise((resolve) => setTimeout(resolve, 100));
      // The injected kit store should be registered
      expect(store.hasKit("desktop-kit")).toBe(true);
      // Apply a change and verify auto-save
      kitStore.apply({ types: { add: [{ guid: "t1", name: "TestType" }] } });
      await new Promise((resolve) => setTimeout(resolve, 500));
      expect(saveCalls.length).toBeGreaterThanOrEqual(1);
      expect(kitStore.getSnapshot().sync.dirty).toBe(false);
    });
  });
  // #endregion 🔖SketchpadStore Auto Save Tests
} // end vitest guard
// #endregion 🔖Tests

// #region 🔖Benchmarks
// [👤semio📚js💻index🔖benchmarks](repo://p/u/semio/b/l/js/f/index.ts/s/Benchmarks)
// Performance benchmarks for kit roundtrip, diff, flatten and validation operations.
// MUST NOT be exported. MUST NOT auto-execute on import.
// Run via: npx tsx index.ts --bench

// Number of iterations per benchmark run.
// MUST be at least 1 for meaningful timing.
const BENCH_ITERATIONS = 3;

// Runs a function multiple times, measures elapsed time and logs CSV output.
// MUST await async functions within the iteration loop.
async function bench(name: string, fn: () => Promise<void> | void) {
  const start = performance.now();
  for (let i = 0; i < BENCH_ITERATIONS; i++) {
    await fn();
  }
  const end = performance.now();
  const durationSec = (end - start) / 1000 / BENCH_ITERATIONS;
  console.log(`${name},${durationSec.toFixed(6)}`);
}

// Runs all benchmarks. MUST only be called explicitly (e.g. via CLI flag).
async function runBenchmarks() {
  const DiffForward = (await import("@semio/assets/semio/diff_kit_metabolism.json")).default;
  const DiffInverse = (await import("@semio/assets/semio/diff_kit_metabolism_inverted.json")).default;
  const BenchMetabolismKit = (await import("@semio/assets/semio/kit_metabolism.json")).default;
  const BenchInvalidKit = (await import("@semio/assets/semio/kit_invalid.json")).default;

  const kitMetabolism = BenchMetabolismKit as unknown as Kit;
  const kitInvalid = BenchInvalidKit as unknown as Kit;
  const diffForward = DiffForward as unknown as KitDiff;
  const diffInverse = DiffInverse as unknown as KitDiff;

  const findBenchDesign = (kit: Kit, name: string, parentName?: string) => {
    let parentGuid: string | undefined;
    if (parentName) {
      const p = kit.designs?.find((d) => d.name === parentName);
      if (!p) throw new Error(`Parent ${parentName} not found`);
      parentGuid = p.guid;
    }
    const d = kit.designs?.find((d) => d.name === name && (parentGuid ? d.parent?.guid === parentGuid : !d.parent));
    if (!d) throw new Error(`Design ${name} not found`);
    return d;
  };

  await bench("Roundtrip/Metabolism", async () => {
    const fs = await import("fs");
    const path = await import("path");
    const zipPath = path.resolve("../assets/semio/metabolism.zip");
    const zipBuffer = fs.readFileSync(zipPath);
    const { kit } = await importKit(zipBuffer);
    await exportKit(kit);
  });

  await bench("Diff/Metabolism", () => {
    const k2 = applyKitDiff(kitMetabolism, diffForward);
    applyKitDiff(k2, diffInverse);
  });

  const d1 = findBenchDesign(kitMetabolism, "Nakagin Capsule Tower");
  await bench("Flatten Design/Nakagin Capsule Tower", () => {
    flattenDesign(kitMetabolism, d1.guid);
  });

  const d2 = findBenchDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower");
  await bench("Flatten Design/Nakagin Capsule Tower/Slanted", () => {
    flattenDesign(kitMetabolism, d2.guid);
  });

  const d3 = findBenchDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower");
  await bench("Flatten Design/Nakagin Capsule Tower/Twisted", () => {
    flattenDesign(kitMetabolism, d3.guid);
  });

  const d4 = findBenchDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower");
  await bench("Flatten Design/Nakagin Capsule Tower/Dancing", () => {
    flattenDesign(kitMetabolism, d4.guid);
  });

  const d5 = findBenchDesign(kitMetabolism, "Capsule Dream");
  await bench("Flatten Design/Capsule Dream", () => {
    flattenDesign(kitMetabolism, d5.guid);
  });

  await bench("Validation/Invalid Kit", () => {
    validateKit(kitInvalid);
  });

  await bench("Validation/Metabolism", () => {
    validateKit(kitMetabolism);
  });
}

if (typeof process !== "undefined" && process.argv?.includes("--bench")) {
  runBenchmarks();
}

// #endregion 🔖Benchmarks
