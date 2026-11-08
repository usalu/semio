/** 🔻 `change-element-adjacency` wire twin: the leaf payload `ChangeElementAdjacency`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type Din18599Adjacency, parseDin18599Adjacency } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeElementAdjacency {
  mutation: "changeelementadjacency";
  elementId: string;
  newAdjacency: Din18599Adjacency;
}

export const parseChangeElementAdjacency: NormWireReader<ChangeElementAdjacency> = normWireObject<ChangeElementAdjacency>({ mutation: normWireRequired(normWireLiteral("changeelementadjacency")), elementId: normWireRequired(normWireString), newAdjacency: normWireRequired(parseDin18599Adjacency) });
