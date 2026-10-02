/** 🧬️ `Iso16757Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireJson, normWireMap, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Iso16757Artifact {
  /** @state artifact */
  catalogue: { [key: string]: NormJson };
  /** @state artifact */
  dictionary: { [key: string]: NormJson };
  /** @state artifact */
  geometry: { [key: string]: NormJson };
  /** @state artifact */
  selection: { [key: string]: NormJson };
  /** @state artifact */
  partNumberRule: { [key: string]: NormJson };
  /** @state artifact */
  partNumberInputs: { [key: string]: string };
  /** @state artifact */
  scriptLimits: { [key: string]: NormJson };
  /** @state artifact */
  exchangeProcess: { [key: string]: NormJson };
}

export const parseIso16757Artifact: NormWireReader<Iso16757Artifact> = normWireObject<Iso16757Artifact>({ catalogue: normWireRequired(normWireMap(normWireJson)), dictionary: normWireRequired(normWireMap(normWireJson)), geometry: normWireRequired(normWireMap(normWireJson)), selection: normWireRequired(normWireMap(normWireJson)), partNumberRule: normWireRequired(normWireMap(normWireJson)), partNumberInputs: normWireRequired(normWireMap(normWireString)), scriptLimits: normWireRequired(normWireMap(normWireJson)), exchangeProcess: normWireRequired(normWireMap(normWireJson)) });
