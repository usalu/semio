import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {parseBinary64,type Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🧬️ Fem2d snapshot schema — artifact-lane fields only. */

//#region 🔖️Entities
/** 📍️ A structural node in plan (x, y in meters). Mirrors Rust `FemNode` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemNode {
  id: string;
  x: Binary64;
  y: Binary64;
}

/** 🔒️ A DOF tag mirroring the FEM 2D degrees of freedom. Mirrors Rust `FemDof`
 * (`🗿️artifacts/◻️2d/🦀️.rs`). */
export type FemDof = "Tx" | "Ty" | "Tz" | "Rx" | "Ry" | "Rz";

/** 🔩️ A 2-node structural member — axial-only bar or axial+bending beam. Mirrors Rust `FemElement`
 * (`🗿️artifacts/◻️2d/🦀️.rs`), tagged on `kind`. */
export type FemElement =
  | { kind: "bar"; id: string; start: string; end: string; materialId: string; sectionId: string }
  | { kind: "beam"; id: string; start: string; end: string; materialId: string; sectionId: string };

/** 🧱️ An isotropic material — Young's modulus `e` (Pa), Poisson's ratio `nu`, density `rho` (kg/m3).
 * Mirrors Rust `FemMaterial` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemMaterial {
  id: string;
  name: string;
  e: Binary64;
  nu: Binary64;
  rho: Binary64;
}

/** 📏️ A cross-section — area (m2) and strong-axis moment of inertia `iy` (m4). Mirrors Rust
 * `FemSection` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemSection {
  id: string;
  name: string;
  area: Binary64;
  iy: Binary64;
}

/** 🛡️ A support: the subset of a node's DOFs restrained to zero displacement. Mirrors Rust
 * `FemSupport` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemSupport {
  id: string;
  nodeId: string;
  fixed: FemDof[];
}

/** 🏋️ A load — a concentrated nodal force/moment, a member UDL, or a pressure over a meshed region.
 * Mirrors Rust `FemLoad` (`🗿️artifacts/◻️2d/🦀️.rs`), tagged on `kind`. */
export type FemLoad =
  | { kind: "nodal"; id: string; nodeId: string; dof: FemDof; value: Binary64 }
  | { kind: "memberUdl"; id: string; elementId: string; wx: Binary64; wy: Binary64 }
  | { kind: "area"; id: string; regionId: string; pressure: Binary64 };

/** 📦️ A named set of loads applied together for one analysis run, optionally including self-weight.
 * Mirrors Rust `FemLoadCase` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemLoadCase {
  id: string;
  name: string;
  loads: FemLoad[];
  selfWeight: boolean;
}

/** 🟩️ A meshed continuum region — a polygon (with optional holes) filled at solve time. Mirrors Rust
 * `FemRegion` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemRegion {
  id: string;
  name: string;
  outline: [Binary64, Binary64][];
  holes: [Binary64, Binary64][][];
  thickness: Binary64;
  materialId: string;
  meshSize: Binary64;
}

/** 🔗️ One combination term — a referenced load case id and its scale factor. Mirrors Rust
 * `FemCombinationTerm` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemCombinationTerm {
  caseId: string;
  factor: Binary64;
}

/** 🧮️ A linear combination of load cases — terms superposed at solve time. Mirrors Rust
 * `FemCombination` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemCombination {
  id: string;
  name: string;
  terms: FemCombinationTerm[];
}

/** ⚙️ Analysis settings — modal/buckling mode counts and the viewport deformation scale factor.
 * Mirrors Rust `FemAnalysisSettings` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemAnalysisSettings {
  modalCount: bigint;
  bucklingCount: bigint;
  deformationScale: Binary64;
}
//#endregion 🔖️Entities

export interface Fem2dSnapshot {
  /** @state artifact */
  nodes: FemNode[];
  /** @state artifact */
  elements: FemElement[];
  /** @state artifact */
  regions: FemRegion[];
  /** @state artifact */
  materials: FemMaterial[];
  /** @state artifact */
  sections: FemSection[];
  /** @state artifact */
  supports: FemSupport[];
  /** @state artifact */
  loadCases: FemLoadCase[];
  /** @state artifact */
  combinations: FemCombination[];
  /** @state artifact */
  analysis: FemAnalysisSettings;
}

/** 🛂️ Admit every unsigned native count without floating-point coercion. */
export function parseFemCount(value:unknown,at="$"):bigint{if(typeof value!=="bigint"||value<0n||value>18446744073709551615n)throw Error(`${at}: unsigned64 count required`);return value}
const femFem2dSnapshotGuardObject=(v:unknown,at:string):Record<string,unknown>=>parseSchemaRecord(v,Object.keys(v!==null&&typeof v==="object"?v:{}),at);
const femFem2dSnapshotGuardArray=(v:unknown,at:string,b:{minItems?:number;maxItems?:number}={}):unknown[]=>{if(!Array.isArray(v)||b.minItems!==undefined&&v.length<b.minItems||b.maxItems!==undefined&&v.length>b.maxItems)throw Error(`${at}: array width differs`);return v};
const femFem2dSnapshotGuardString=(v:unknown,at:string):string=>{if(typeof v!=="string")throw Error(`${at}: text required`);return v};
const femFem2dSnapshotGuardBoolean=(v:unknown,at:string):boolean=>{if(typeof v!=="boolean")throw Error(`${at}: boolean required`);return v};
const femFem2dSnapshotGuardMember=<T extends string>(v:unknown,at:string,m:readonly T[]):T=>{if(typeof v!=="string"||!m.includes(v as T))throw Error(`${at}: variant differs`);return v as T};
const femFem2dSnapshotGuardWord=(v:unknown,at:string):Binary64=>parseBinary64(parseSchemaRecord(v,["bits"],at));

function parseFemPoint(value: unknown, at: string): [Binary64, Binary64] {
  const point = femFem2dSnapshotGuardArray(value, at, { minItems: 2, maxItems: 2 });
  return [femFem2dSnapshotGuardWord(point[0], `${at}[0]`), femFem2dSnapshotGuardWord(point[1], `${at}[1]`)];
}

export function parseFemElement(value: unknown, at = "$"): FemElement {
  const kind = femFem2dSnapshotGuardMember(femFem2dSnapshotGuardObject(value, at)["kind"], `${at}.kind`, ["bar", "beam"] as const);
  const row = parseSchemaRecord(value, ["kind", "id", "start", "end", "materialId", "sectionId"], at);
  const base = {
    id: femFem2dSnapshotGuardString(row.id, `${at}.id`),
    start: femFem2dSnapshotGuardString(row.start, `${at}.start`),
    end: femFem2dSnapshotGuardString(row.end, `${at}.end`),
    materialId: femFem2dSnapshotGuardString(row.materialId, `${at}.materialId`),
    sectionId: femFem2dSnapshotGuardString(row.sectionId, `${at}.sectionId`),
  };
  return { ...base, kind };
}

export function parseFemLoad(value: unknown, at = "$"): FemLoad {
  const kind = femFem2dSnapshotGuardMember(femFem2dSnapshotGuardObject(value, at)["kind"], `${at}.kind`, ["nodal", "memberUdl", "area"] as const);
  const fields = kind === "nodal" ? ["nodeId", "dof", "value"] : kind === "memberUdl" ? ["elementId", "wx", "wy"] : ["regionId", "pressure"];
  const row = parseSchemaRecord(value, ["kind", "id", ...fields], at);
  const id = femFem2dSnapshotGuardString(row.id, `${at}.id`);
  if (kind === "nodal") return { kind, id, nodeId: femFem2dSnapshotGuardString(row.nodeId, `${at}.nodeId`), dof: femFem2dSnapshotGuardMember(row.dof, `${at}.dof`, ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const), value: femFem2dSnapshotGuardWord(row.value, `${at}.value`) };
  if (kind === "memberUdl") return { kind, id, elementId: femFem2dSnapshotGuardString(row.elementId, `${at}.elementId`), wx: femFem2dSnapshotGuardWord(row.wx, `${at}.wx`), wy: femFem2dSnapshotGuardWord(row.wy, `${at}.wy`) };
  return { kind, id, regionId: femFem2dSnapshotGuardString(row.regionId, `${at}.regionId`), pressure: femFem2dSnapshotGuardWord(row.pressure, `${at}.pressure`) };
}

export function parseFemLoadCase(value: unknown, at = "$"): FemLoadCase {
  const row = parseSchemaRecord(value, ["id", "name", "loads", "selfWeight"], at);
  return {
    id: femFem2dSnapshotGuardString(row.id, `${at}.id`),
    name: femFem2dSnapshotGuardString(row.name, `${at}.name`),
    loads: femFem2dSnapshotGuardArray(row.loads, `${at}.loads`).map((load, index) => parseFemLoad(load, `${at}.loads[${index}]`)),
    selfWeight: femFem2dSnapshotGuardBoolean(row.selfWeight, `${at}.selfWeight`),
  };
}

export function parseFemCombination(value: unknown, at = "$"): FemCombination {
  const row = parseSchemaRecord(value, ["id", "name", "terms"], at);
  return {
    id: femFem2dSnapshotGuardString(row.id, `${at}.id`),
    name: femFem2dSnapshotGuardString(row.name, `${at}.name`),
    terms: femFem2dSnapshotGuardArray(row.terms, `${at}.terms`).map((value, index) => {
      const termAt = `${at}.terms[${index}]`;
      const term = parseSchemaRecord(value, ["caseId", "factor"], termAt);
      return { caseId: femFem2dSnapshotGuardString(term.caseId, `${termAt}.caseId`), factor: femFem2dSnapshotGuardWord(term.factor, `${termAt}.factor`) };
    }),
  };
}

export function parseFem2dSnapshot(value: unknown, at = "$"): Fem2dSnapshot {
  const row = parseSchemaRecord(value, ["nodes", "elements", "materials", "sections", "supports", "loadCases", "combinations", "regions", "analysis"], at);
  return {
    nodes: femFem2dSnapshotGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parseFemNode(item, `${at}.nodes[${index}]`)),
    elements: femFem2dSnapshotGuardArray(row["elements"], `${at}.elements`).map((item, index) => parseFemElement(item, `${at}.elements[${index}]`)),
    regions: femFem2dSnapshotGuardArray(row["regions"], `${at}.regions`).map((item, index) => parseFemRegion(item, `${at}.regions[${index}]`)),
    materials: femFem2dSnapshotGuardArray(row["materials"], `${at}.materials`).map((item, index) => parseFemMaterial(item, `${at}.materials[${index}]`)),
    sections: femFem2dSnapshotGuardArray(row["sections"], `${at}.sections`).map((item, index) => parseFemSection(item, `${at}.sections[${index}]`)),
    supports: femFem2dSnapshotGuardArray(row["supports"], `${at}.supports`).map((item, index) => parseFemSupport(item, `${at}.supports[${index}]`)),
    loadCases: femFem2dSnapshotGuardArray(row["loadCases"], `${at}.loadCases`).map((item, index) => parseFemLoadCase(item, `${at}.loadCases[${index}]`)),
    combinations: femFem2dSnapshotGuardArray(row["combinations"], `${at}.combinations`).map((item, index) => parseFemCombination(item, `${at}.combinations[${index}]`)),
    analysis: parseFemAnalysisSettings(row["analysis"], `${at}.analysis`),
  };
}


export function parseFemAnalysisSettings(value: unknown, at = "$"): FemAnalysisSettings {
  const row = parseSchemaRecord(value, ["modalCount", "bucklingCount", "deformationScale"], at);
  return {
    modalCount: parseFemCount(row["modalCount"], `${at}.modalCount`),
    bucklingCount: parseFemCount(row["bucklingCount"], `${at}.bucklingCount`),
    deformationScale: femFem2dSnapshotGuardWord(row["deformationScale"], `${at}.deformationScale`),
  };
}

export function parseFemNode(value: unknown, at = "$"): FemNode {
  const row = parseSchemaRecord(value, ["id", "x", "y"], at);
  return {
    id: femFem2dSnapshotGuardString(row["id"], `${at}.id`),
    x: femFem2dSnapshotGuardWord(row["x"], `${at}.x`),
    y: femFem2dSnapshotGuardWord(row["y"], `${at}.y`),
  };
}

export function parseFemRegion(value: unknown, at = "$"): FemRegion {
  const row = parseSchemaRecord(value, ["id", "name", "outline", "holes", "thickness", "materialId", "meshSize"], at);
  return {
    id: femFem2dSnapshotGuardString(row["id"], `${at}.id`),
    name: femFem2dSnapshotGuardString(row["name"], `${at}.name`),
    outline: femFem2dSnapshotGuardArray(row.outline, `${at}.outline`).map((point, index) => parseFemPoint(point, `${at}.outline[${index}]`)),
    holes: femFem2dSnapshotGuardArray(row.holes, `${at}.holes`).map((hole, index) => femFem2dSnapshotGuardArray(hole, `${at}.holes[${index}]`).map((point, pointIndex) => parseFemPoint(point, `${at}.holes[${index}][${pointIndex}]`))),
    thickness: femFem2dSnapshotGuardWord(row["thickness"], `${at}.thickness`),
    materialId: femFem2dSnapshotGuardString(row["materialId"], `${at}.materialId`),
    meshSize: femFem2dSnapshotGuardWord(row["meshSize"], `${at}.meshSize`),
  };
}

export function parseFemMaterial(value: unknown, at = "$"): FemMaterial {
  const row = parseSchemaRecord(value, ["id", "name", "e", "nu", "rho"], at);
  return {
    id: femFem2dSnapshotGuardString(row["id"], `${at}.id`),
    name: femFem2dSnapshotGuardString(row["name"], `${at}.name`),
    e: femFem2dSnapshotGuardWord(row["e"], `${at}.e`),
    nu: femFem2dSnapshotGuardWord(row["nu"], `${at}.nu`),
    rho: femFem2dSnapshotGuardWord(row["rho"], `${at}.rho`),
  };
}

export function parseFemSection(value: unknown, at = "$"): FemSection {
  const row = parseSchemaRecord(value, ["id", "name", "area", "iy"], at);
  return {
    id: femFem2dSnapshotGuardString(row["id"], `${at}.id`),
    name: femFem2dSnapshotGuardString(row["name"], `${at}.name`),
    area: femFem2dSnapshotGuardWord(row["area"], `${at}.area`),
    iy: femFem2dSnapshotGuardWord(row["iy"], `${at}.iy`),
  };
}

export function parseFemSupport(value: unknown, at = "$"): FemSupport {
  const row = parseSchemaRecord(value, ["id", "nodeId", "fixed"], at);
  return {
    id: femFem2dSnapshotGuardString(row["id"], `${at}.id`),
    nodeId: femFem2dSnapshotGuardString(row["nodeId"], `${at}.nodeId`),
    fixed: femFem2dSnapshotGuardArray(row["fixed"], `${at}.fixed`).map((item, index) => femFem2dSnapshotGuardMember(item, `${at}.fixed[${index}]`, ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const)),
  };
}

export {fem2dSnapshotToSqliteDatabase,fem2dSnapshotFromSqliteDatabase,FEM2D_SQLITE_SCHEMA} from "./🪶️sqlite/🟦️.ts";
