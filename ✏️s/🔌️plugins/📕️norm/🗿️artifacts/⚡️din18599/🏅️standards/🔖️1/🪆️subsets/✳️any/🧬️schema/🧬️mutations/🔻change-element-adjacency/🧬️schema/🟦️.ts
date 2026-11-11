/** 🔻 `change-element-adjacency` wire twin: the leaf payload `ChangeElementAdjacency`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type Din18599Adjacency, parseDin18599Adjacency } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeElementAdjacency {
  mutation: "changeelementadjacency";
  elementId: string;
  newAdjacency: Din18599Adjacency;
}

export const parseChangeElementAdjacency: NormWireReader<ChangeElementAdjacency> = normWireObject<ChangeElementAdjacency>({ mutation: normWireRequired(normWireLiteral("changeelementadjacency")), elementId: normWireRequired(normWireString), newAdjacency: normWireRequired(parseDin18599Adjacency) });
