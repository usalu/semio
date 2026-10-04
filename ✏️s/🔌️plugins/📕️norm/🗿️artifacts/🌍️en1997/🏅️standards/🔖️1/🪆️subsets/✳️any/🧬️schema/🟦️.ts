/** 🧬️ `En1997Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireArray, normWireInteger, normWireJson, normWireLiteral, normWireMap, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1997Artifact {
  /** @state artifact */
  structureId: string;
  /** @state artifact */
  geotechnicalCategory: number;
  /** @state artifact */
  designSituation: string;
  /** @state artifact */
  designApproach: string;
  /** @state artifact */
  annex: "En" | "De";
  /** @state artifact */
  groundwaterLevel: number;
  /** @state artifact */
  investigationDepth: number;
  /** @state artifact */
  layers: { [key: string]: NormJson }[];
  /** @state artifact */
  footings: { [key: string]: NormJson }[];
  /** @state artifact */
  piles: { [key: string]: NormJson }[];
  /** @state artifact */
  retainingWalls: { [key: string]: NormJson }[];
  /** @state artifact */
  slopes: { [key: string]: NormJson }[];
  /** @state artifact */
  upliftCases: { [key: string]: NormJson }[];
}

export const parseEn1997Artifact: NormWireReader<En1997Artifact> = normWireObject<En1997Artifact>({ structureId: normWireRequired(normWireString), geotechnicalCategory: normWireRequired(normWireInteger), designSituation: normWireRequired(normWireString), designApproach: normWireRequired(normWireString), annex: normWireRequired(normWireLiteral("En", "De")), groundwaterLevel: normWireRequired(normWireNumber), investigationDepth: normWireRequired(normWireNumber), layers: normWireRequired(normWireArray(normWireMap(normWireJson))), footings: normWireRequired(normWireArray(normWireMap(normWireJson))), piles: normWireRequired(normWireArray(normWireMap(normWireJson))), retainingWalls: normWireRequired(normWireArray(normWireMap(normWireJson))), slopes: normWireRequired(normWireArray(normWireMap(normWireJson))), upliftCases: normWireRequired(normWireArray(normWireMap(normWireJson))) });
