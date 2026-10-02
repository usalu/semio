/** 🧬️ `En1995Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireArray, normWireJson, normWireMap, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1995Artifact {
  /** @state artifact */
  annex: string;
  /** @state artifact */
  members: { [key: string]: NormJson }[];
  /** @state artifact */
  connections: { [key: string]: NormJson }[];
}

export const parseEn1995Artifact: NormWireReader<En1995Artifact> = normWireObject<En1995Artifact>({ annex: normWireRequired(normWireString), members: normWireRequired(normWireArray(normWireMap(normWireJson))), connections: normWireRequired(normWireArray(normWireMap(normWireJson))) });
