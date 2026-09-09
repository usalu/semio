/** 🔺️ Sparse CAD document diff over composed child identities and native node deltas. */
import {
  cadContractArray,
  cadContractExact,
  cadContractObject,
  cadContractString,
  parseCadArtifact,
  parseCadChild,
  parseCadNode,
  parseCadReferences,
  type ArtifactChild,
  type CadArtifact,
  type CadNode,
  type CadReferenceList,
} from "../🟦️.ts";

export interface CadNodePatch { label?: string | null }
export interface CadNodePatchEntry { id: string; patch: CadNodePatch }
export interface CadNodesDelta { added: CadNode[]; removed: string[]; patched: CadNodePatchEntry[]; reordered: string[] | null }
export interface CadDrawingChildList { values: ArtifactChild[] }

export interface CadDiff {
  /** @state artifact */ artifact?: CadArtifact | null;
  /** @state artifact */ schema?: string | null;
  /** @state artifact */ id?: string | null;
  /** @state artifact @child kind=s.stdio.semio */ shapeModel?: ArtifactChild | null;
  /** @state artifact @child kind=s.stdio.semio */ buildingModel?: ArtifactChild | null;
  /** @state artifact @child kind=s.stdio.semio */ energyModel?: ArtifactChild | null;
  /** @state artifact @child kind=s.stdio.semio */ structureClassicModel?: ArtifactChild | null;
  /** @state artifact @child kind=s.stdio.semio many */ drawings?: CadDrawingChildList | null;
  /** @state artifact */ referencesByModelDefinitionId?: Record<string, CadReferenceList> | null;
  /** @state artifact */ nodes?: CadNodesDelta | null;
}

function parseNodePatch(value: unknown, at: string): CadNodePatch {
  const row = cadContractObject(value, at);
  cadContractExact(row, ["label"], [], at);
  return Object.hasOwn(row, "label") ? { label: row.label === null ? null : cadContractString(row.label, `${at}.label`) } : {};
}

function parseNodesDelta(value: unknown, at: string): CadNodesDelta {
  const row = cadContractObject(value, at);
  const keys = ["added", "removed", "patched", "reordered"];
  cadContractExact(row, keys, keys, at);
  return {
    added: cadContractArray(row.added, `${at}.added`).map((item, index) => parseCadNode(item, `${at}.added[${index}]`)),
    removed: cadContractArray(row.removed, `${at}.removed`).map((item, index) => cadContractString(item, `${at}.removed[${index}]`)),
    patched: cadContractArray(row.patched, `${at}.patched`).map((item, index) => {
      const path = `${at}.patched[${index}]`, patch = cadContractObject(item, path);
      cadContractExact(patch, ["id", "patch"], ["id", "patch"], path);
      return { id: cadContractString(patch.id, `${path}.id`), patch: parseNodePatch(patch.patch, `${path}.patch`) };
    }),
    reordered: row.reordered === null ? null : cadContractArray(row.reordered, `${at}.reordered`).map((item, index) => cadContractString(item, `${at}.reordered[${index}]`)),
  };
}

/** 🪪️ Parses only the sparse fields implemented by native CadDiff. */
export function parseCadDiff(value: unknown, at = "$"): CadDiff {
  const row = cadContractObject(value, at);
  const keys = ["artifact", "schema", "id", "shapeModel", "buildingModel", "energyModel", "structureClassicModel", "drawings", "referencesByModelDefinitionId", "nodes"];
  cadContractExact(row, keys, [], at);
  const result: CadDiff = {};
  if (Object.hasOwn(row, "artifact")) result.artifact = row.artifact === null ? null : parseCadArtifact(row.artifact, `${at}.artifact`);
  if (Object.hasOwn(row, "schema")) result.schema = row.schema === null ? null : cadContractString(row.schema, `${at}.schema`);
  if (Object.hasOwn(row, "id")) result.id = row.id === null ? null : cadContractString(row.id, `${at}.id`);
  for (const key of ["shapeModel", "buildingModel", "energyModel", "structureClassicModel"] as const) {
    if (Object.hasOwn(row, key)) result[key] = row[key] === null ? null : parseCadChild(row[key], "model", `${at}.${key}`);
  }
  if (Object.hasOwn(row, "drawings")) {
    if (row.drawings === null) result.drawings = null;
    else {
      const wrapper = cadContractObject(row.drawings, `${at}.drawings`);
      cadContractExact(wrapper, ["values"], ["values"], `${at}.drawings`);
      result.drawings = { values: cadContractArray(wrapper.values, `${at}.drawings.values`).map((item, index) => parseCadChild(item, "drawing", `${at}.drawings.values[${index}]`)) };
    }
  }
  if (Object.hasOwn(row, "referencesByModelDefinitionId")) result.referencesByModelDefinitionId = row.referencesByModelDefinitionId === null ? null : parseCadReferences(row.referencesByModelDefinitionId, `${at}.referencesByModelDefinitionId`);
  if (Object.hasOwn(row, "nodes")) result.nodes = row.nodes === null ? null : parseNodesDelta(row.nodes, `${at}.nodes`);
  return result;
}
