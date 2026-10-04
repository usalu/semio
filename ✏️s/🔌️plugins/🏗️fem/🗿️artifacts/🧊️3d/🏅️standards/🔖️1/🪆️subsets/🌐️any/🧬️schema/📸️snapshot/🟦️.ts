import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {type Binary64,parseBinary64Transport} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🧬️ Fem3d snapshot schema — artifact-lane fields only. */

//#region 🔖️Entities
/** 📍️ A structural node: a stable id and a global position, plain SI meters. Mirrors Rust `FemNode`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemNode {
  id: string;
  x: Binary64;
  y: Binary64;
  z: Binary64;
}

/** 🔒️ A DOF tag mirroring the FEM 3D degrees of freedom. Mirrors Rust `FemDof`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`, re-exported from `fem2d::FemDof`). */
export type FemDof = "Tx" | "Ty" | "Tz" | "Rx" | "Ry" | "Rz";

/** 🧭️ The world axis a solid's footprint is extruded along. Mirrors Rust `FemAxis`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export type FemAxis = "x" | "y" | "z";

/** 🔩️ A two-node member: an axial `Bar` or a full 6-DOF `Frame` with a local-axis `roll` angle
 * (radians). Mirrors Rust `FemElement` (`🗿️artifacts/🧊️3d/🦀️.rs`), tagged on `kind`. */
export type FemElement =
  | { kind: "bar"; id: string; start: string; end: string; materialId: string; sectionId: string }
  | { kind: "frame"; id: string; start: string; end: string; materialId: string; sectionId: string; roll: Binary64 };

/** 🧱️ Linear-elastic isotropic material: Young's modulus `e`, shear modulus `g` (Pa), Poisson's ratio
 * `nu` (dimensionless), density `rho` (kg/m3). Mirrors Rust `FemMaterial`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemMaterial {
  id: string;
  name: string;
  e: Binary64;
  g: Binary64;
  nu: Binary64;
  rho: Binary64;
}

/** 📐️ Cross-section properties: area (m2), second moments of area about local y/z (m4), torsion
 * constant (m4). Mirrors Rust `FemSection` (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemSection {
  id: string;
  name: string;
  area: Binary64;
  iy: Binary64;
  iz: Binary64;
  j: Binary64;
}

/** 🛡️ A support: the subset of a node's DOFs restrained to zero displacement. Mirrors Rust
 * `FemSupport` (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemSupport {
  id: string;
  nodeId: string;
  fixed: FemDof[];
}

/** 🧱️ A meshed continuum solid — a polygon footprint (with optional holes) extruded upward from
 * `baseZ` by `height` across `layers` equal-height layers. Mirrors Rust `FemSolid`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemSolid {
  id: string;
  name: string;
  outline: [Binary64, Binary64][];
  holes: [Binary64, Binary64][][];
  baseZ: Binary64;
  height: Binary64;
  layers: bigint;
  meshSize: Binary64;
  materialId: string;
  axis: FemAxis;
}

/** 🏋️ A load — a concentrated nodal force/moment, a member UDL on a bar/frame element, or a normal
 * pressure over a meshed solid's top face. Mirrors Rust `FemLoad` (`🗿️artifacts/🧊️3d/🦀️.rs`),
 * tagged on `kind`. */
export type FemLoad =
  | { kind: "nodal"; id: string; nodeId: string; dof: FemDof; value: Binary64 }
  | { kind: "memberUdl"; id: string; elementId: string; wx: Binary64; wy: Binary64; wz: Binary64 }
  | { kind: "area"; id: string; solidId: string; pressure: Binary64 };

/** 📦️ A named set of loads applied together for one analysis run, optionally including self-weight.
 * Mirrors Rust `FemLoadCase` (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemLoadCase {
  id: string;
  name: string;
  loads: FemLoad[];
  selfWeight: boolean;
}

/** 📦️ A linear combination of load cases — case id → factor terms superposed from already-solved
 * case results. Mirrors Rust `FemCombination` (`🗿️artifacts/🧊️3d/🦀️.rs`, `BTreeMap<String, f64>`). */
export interface FemCombination {
  id: string;
  name: string;
  terms: Map<string, Binary64>;
}

/** ⚙️ Analysis settings: mode/factor counts for modal and buckling analyses, plus a deformation
 * display scale for the UI layer. Mirrors Rust `FemAnalysisSettings`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`, re-exported from `fem2d::FemAnalysisSettings`). */
export interface FemAnalysisSettings {
  modalCount: bigint;
  bucklingCount: bigint;
  deformationScale: Binary64;
}
//#endregion 🔖️Entities

export interface Fem3dSnapshot {
  /** @state artifact */
  nodes: FemNode[];
  /** @state artifact */
  elements: FemElement[];
  /** @state artifact */
  materials: FemMaterial[];
  /** @state artifact */
  sections: FemSection[];
  /** @state artifact */
  solids: FemSolid[];
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
const femFem3dSnapshotGuardObject=(v:unknown,at:string):Record<string,unknown>=>parseSchemaRecord(v,Object.keys(v!==null&&typeof v==="object"?v:{}),at);
const femFem3dSnapshotGuardArray=(v:unknown,at:string,b:{minItems?:number;maxItems?:number}={}):unknown[]=>{if(!Array.isArray(v)||b.minItems!==undefined&&v.length<b.minItems||b.maxItems!==undefined&&v.length>b.maxItems)throw Error(`${at}: array width differs`);return v};
const femFem3dSnapshotGuardString=(v:unknown,at:string):string=>{if(typeof v!=="string")throw Error(`${at}: text required`);return v};
const femFem3dSnapshotGuardBoolean=(v:unknown,at:string):boolean=>{if(typeof v!=="boolean")throw Error(`${at}: boolean required`);return v};
const femFem3dSnapshotGuardMember=<T extends string>(v:unknown,at:string,m:readonly T[]):T=>{if(typeof v!=="string"||!m.includes(v as T))throw Error(`${at}: variant differs`);return v as T};
const femFem3dSnapshotGuardWord=(v:unknown,_at:string):Binary64=>parseBinary64Transport(v);
function parseFemTerms(value:unknown,at:string):Map<string,Binary64>{if(!(value instanceof Map))throw Error(`${at}: owned map required`);const out=new Map<string,Binary64>();for(const[key,v]of value){if(typeof key!=="string")throw Error(`${at}: text map key required`);out.set(key,femFem3dSnapshotGuardWord(v,`${at}.${key}`))}return out}

function parseFemPoint(value: unknown, at: string): [Binary64, Binary64] {
  const point = femFem3dSnapshotGuardArray(value, at, { minItems: 2, maxItems: 2 });
  return [femFem3dSnapshotGuardWord(point[0], `${at}[0]`), femFem3dSnapshotGuardWord(point[1], `${at}[1]`)];
}

export function parseFemElement(value: unknown, at = "$"): FemElement {
  const kind = femFem3dSnapshotGuardMember(femFem3dSnapshotGuardObject(value, at)["kind"], `${at}.kind`, ["bar", "frame"] as const);
  const row = parseSchemaRecord(value, ["kind", "id", "start", "end", "materialId", "sectionId", ...(kind === "frame" ? ["roll"] : [])], at);
  const base = {
    id: femFem3dSnapshotGuardString(row.id, `${at}.id`),
    start: femFem3dSnapshotGuardString(row.start, `${at}.start`),
    end: femFem3dSnapshotGuardString(row.end, `${at}.end`),
    materialId: femFem3dSnapshotGuardString(row.materialId, `${at}.materialId`),
    sectionId: femFem3dSnapshotGuardString(row.sectionId, `${at}.sectionId`),
  };
  return kind === "frame" ? { ...base, kind, roll: femFem3dSnapshotGuardWord(row.roll, `${at}.roll`) } : { ...base, kind };
}

export function parseFemLoad(value: unknown, at = "$"): FemLoad {
  const kind = femFem3dSnapshotGuardMember(femFem3dSnapshotGuardObject(value, at)["kind"], `${at}.kind`, ["nodal", "memberUdl", "area"] as const);
  const fields = kind === "nodal" ? ["nodeId", "dof", "value"] : kind === "memberUdl" ? ["elementId", "wx", "wy", "wz"] : ["solidId", "pressure"];
  const row = parseSchemaRecord(value, ["kind", "id", ...fields], at);
  const id = femFem3dSnapshotGuardString(row.id, `${at}.id`);
  if (kind === "nodal") return { kind, id, nodeId: femFem3dSnapshotGuardString(row.nodeId, `${at}.nodeId`), dof: femFem3dSnapshotGuardMember(row.dof, `${at}.dof`, ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const), value: femFem3dSnapshotGuardWord(row.value, `${at}.value`) };
  if (kind === "memberUdl") return { kind, id, elementId: femFem3dSnapshotGuardString(row.elementId, `${at}.elementId`), wx: femFem3dSnapshotGuardWord(row.wx, `${at}.wx`), wy: femFem3dSnapshotGuardWord(row.wy, `${at}.wy`), wz: femFem3dSnapshotGuardWord(row.wz, `${at}.wz`) };
  return { kind, id, solidId: femFem3dSnapshotGuardString(row.solidId, `${at}.solidId`), pressure: femFem3dSnapshotGuardWord(row.pressure, `${at}.pressure`) };
}

export function parseFemLoadCase(value: unknown, at = "$"): FemLoadCase {
  const row = parseSchemaRecord(value, ["id", "name", "loads", "selfWeight"], at);
  return {
    id: femFem3dSnapshotGuardString(row.id, `${at}.id`),
    name: femFem3dSnapshotGuardString(row.name, `${at}.name`),
    loads: femFem3dSnapshotGuardArray(row.loads, `${at}.loads`).map((load, index) => parseFemLoad(load, `${at}.loads[${index}]`)),
    selfWeight: femFem3dSnapshotGuardBoolean(row.selfWeight, `${at}.selfWeight`),
  };
}

export function parseFem3dSnapshot(value: unknown, at = "$"): Fem3dSnapshot {
  const row = parseSchemaRecord(value, ["nodes", "elements", "materials", "sections", "supports", "loadCases", "combinations", "solids", "analysis"], at);
  return {
    nodes: femFem3dSnapshotGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parseFemNode(item, `${at}.nodes[${index}]`)),
    elements: femFem3dSnapshotGuardArray(row["elements"], `${at}.elements`).map((item, index) => parseFemElement(item, `${at}.elements[${index}]`)),
    materials: femFem3dSnapshotGuardArray(row["materials"], `${at}.materials`).map((item, index) => parseFemMaterial(item, `${at}.materials[${index}]`)),
    sections: femFem3dSnapshotGuardArray(row["sections"], `${at}.sections`).map((item, index) => parseFemSection(item, `${at}.sections[${index}]`)),
    solids: femFem3dSnapshotGuardArray(row["solids"], `${at}.solids`).map((item, index) => parseFemSolid(item, `${at}.solids[${index}]`)),
    supports: femFem3dSnapshotGuardArray(row["supports"], `${at}.supports`).map((item, index) => parseFemSupport(item, `${at}.supports[${index}]`)),
    loadCases: femFem3dSnapshotGuardArray(row["loadCases"], `${at}.loadCases`).map((item, index) => parseFemLoadCase(item, `${at}.loadCases[${index}]`)),
    combinations: femFem3dSnapshotGuardArray(row["combinations"], `${at}.combinations`).map((item, index) => parseFemCombination(item, `${at}.combinations[${index}]`)),
    analysis: parseFemAnalysisSettings(row["analysis"], `${at}.analysis`),
  };
}


export function parseFemAnalysisSettings(value: unknown, at = "$"): FemAnalysisSettings {
  const row = parseSchemaRecord(value, ["modalCount", "bucklingCount", "deformationScale"], at);
  return {
    modalCount: parseFemCount(row["modalCount"], `${at}.modalCount`),
    bucklingCount: parseFemCount(row["bucklingCount"], `${at}.bucklingCount`),
    deformationScale: femFem3dSnapshotGuardWord(row["deformationScale"], `${at}.deformationScale`),
  };
}

export function parseFemNode(value: unknown, at = "$"): FemNode {
  const row = parseSchemaRecord(value, ["id", "x", "y", "z"], at);
  return {
    id: femFem3dSnapshotGuardString(row["id"], `${at}.id`),
    x: femFem3dSnapshotGuardWord(row["x"], `${at}.x`),
    y: femFem3dSnapshotGuardWord(row["y"], `${at}.y`),
    z: femFem3dSnapshotGuardWord(row["z"], `${at}.z`),
  };
}

export function parseFemSolid(value: unknown, at = "$"): FemSolid {
  const row = parseSchemaRecord(value, ["id", "name", "outline", "holes", "baseZ", "height", "layers", "meshSize", "materialId", "axis"], at);
  return {
    id: femFem3dSnapshotGuardString(row["id"], `${at}.id`),
    name: femFem3dSnapshotGuardString(row["name"], `${at}.name`),
    outline: femFem3dSnapshotGuardArray(row.outline, `${at}.outline`).map((point, index) => parseFemPoint(point, `${at}.outline[${index}]`)),
    holes: femFem3dSnapshotGuardArray(row.holes, `${at}.holes`).map((hole, index) => femFem3dSnapshotGuardArray(hole, `${at}.holes[${index}]`).map((point, pointIndex) => parseFemPoint(point, `${at}.holes[${index}][${pointIndex}]`))),
    baseZ: femFem3dSnapshotGuardWord(row["baseZ"], `${at}.baseZ`),
    height: femFem3dSnapshotGuardWord(row["height"], `${at}.height`),
    layers: parseFemCount(row["layers"], `${at}.layers`),
    meshSize: femFem3dSnapshotGuardWord(row["meshSize"], `${at}.meshSize`),
    materialId: femFem3dSnapshotGuardString(row["materialId"], `${at}.materialId`),
    axis: femFem3dSnapshotGuardMember(row["axis"], `${at}.axis`, ["x", "y", "z"] as const),
  };
}

export function parseFemMaterial(value: unknown, at = "$"): FemMaterial {
  const row = parseSchemaRecord(value, ["id", "name", "e", "g", "nu", "rho"], at);
  return {
    id: femFem3dSnapshotGuardString(row["id"], `${at}.id`),
    name: femFem3dSnapshotGuardString(row["name"], `${at}.name`),
    e: femFem3dSnapshotGuardWord(row["e"], `${at}.e`),
    g: femFem3dSnapshotGuardWord(row["g"], `${at}.g`),
    nu: femFem3dSnapshotGuardWord(row["nu"], `${at}.nu`),
    rho: femFem3dSnapshotGuardWord(row["rho"], `${at}.rho`),
  };
}

export function parseFemSection(value: unknown, at = "$"): FemSection {
  const row = parseSchemaRecord(value, ["id", "name", "area", "iy", "iz", "j"], at);
  return {
    id: femFem3dSnapshotGuardString(row["id"], `${at}.id`),
    name: femFem3dSnapshotGuardString(row["name"], `${at}.name`),
    area: femFem3dSnapshotGuardWord(row["area"], `${at}.area`),
    iy: femFem3dSnapshotGuardWord(row["iy"], `${at}.iy`),
    iz: femFem3dSnapshotGuardWord(row["iz"], `${at}.iz`),
    j: femFem3dSnapshotGuardWord(row["j"], `${at}.j`),
  };
}

export function parseFemSupport(value: unknown, at = "$"): FemSupport {
  const row = parseSchemaRecord(value, ["id", "nodeId", "fixed"], at);
  return {
    id: femFem3dSnapshotGuardString(row["id"], `${at}.id`),
    nodeId: femFem3dSnapshotGuardString(row["nodeId"], `${at}.nodeId`),
    fixed: femFem3dSnapshotGuardArray(row["fixed"], `${at}.fixed`).map((item, index) => femFem3dSnapshotGuardMember(item, `${at}.fixed[${index}]`, ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const)),
  };
}

export function parseFemCombination(value: unknown, at = "$"): FemCombination {
  const row = parseSchemaRecord(value, ["id", "name", "terms"], at);
  return {
    id: femFem3dSnapshotGuardString(row["id"], `${at}.id`),
    name: femFem3dSnapshotGuardString(row["name"], `${at}.name`),
    terms: parseFemTerms(row["terms"], `${at}.terms`),
  };
}

export {fem3dSnapshotToSqliteDatabase,fem3dSnapshotFromSqliteDatabase,FEM3D_SQLITE_SCHEMA} from "./🪶️sqlite/🟦️.ts";
