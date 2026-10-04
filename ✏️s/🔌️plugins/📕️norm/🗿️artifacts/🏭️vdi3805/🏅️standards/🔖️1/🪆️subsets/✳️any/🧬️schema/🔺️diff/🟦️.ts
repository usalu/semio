/** 🔺️ `Vdi3805Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireDefault, normWireLiteral, normWireMap, normWireNullable, normWireObject, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CatalogIndex, type CharacteristicCurve, type EditionId, type ManufacturerCatalog, type ManufacturerFile, type ParametricGeometry, parseCatalogIndex, parseCharacteristicCurve, parseEditionId, parseManufacturerCatalog, parseManufacturerFile, parseParametricGeometry, parseSecurityLimits, type SecurityLimits } from "../📸️snapshot/🟦️.ts";
import { parseVdi3805Artifact, type Vdi3805Artifact } from "../🟦️.ts";

export interface Vdi3805Diff {
  /** @state artifact */
  artifact: Vdi3805Artifact | null;
  /** @state artifact */
  manufacturerFile: ManufacturerFile | null;
  /** @state artifact */
  catalog: ManufacturerCatalog | null;
  /** @state artifact */
  editionProfile: ({ [key: string]: "legacy" | "current" }) | null;
  /** @state artifact */
  correctionAsOf: EditionId | null;
  /** @state artifact */
  strictMode: boolean | null;
  /** @state artifact */
  index: CatalogIndex | null;
  /** @state artifact */
  geometry: { [key: string]: ParametricGeometry } | null;
  /** @state artifact */
  curves: { [key: string]: CharacteristicCurve } | null;
  /** @state artifact */
  limits: SecurityLimits | null;
}

export const parseVdi3805Diff: NormWireReader<Vdi3805Diff> = normWireObject<Vdi3805Diff>({ artifact: normWireDefault(normWireNullable(parseVdi3805Artifact), () => null), manufacturerFile: normWireDefault(normWireNullable(parseManufacturerFile), () => null), catalog: normWireDefault(normWireNullable(parseManufacturerCatalog), () => null), editionProfile: normWireDefault(normWireNullable(normWireMap(normWireLiteral("legacy", "current"))), () => null), correctionAsOf: normWireDefault(normWireNullable(parseEditionId), () => null), strictMode: normWireDefault(normWireNullable(normWireBoolean), () => null), index: normWireDefault(normWireNullable(parseCatalogIndex), () => null), geometry: normWireDefault(normWireNullable(normWireMap(parseParametricGeometry)), () => null), curves: normWireDefault(normWireNullable(normWireMap(parseCharacteristicCurve)), () => null), limits: normWireDefault(normWireNullable(parseSecurityLimits), () => null) });
