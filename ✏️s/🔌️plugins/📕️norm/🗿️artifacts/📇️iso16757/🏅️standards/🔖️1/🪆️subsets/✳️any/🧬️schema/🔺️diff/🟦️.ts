/** 🔺️ `Iso16757Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireMap, normWireNullable, normWireObject, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Catalogue, type CatalogueValue, type Dictionary, type GeometryCatalogue, type Iso16757ExchangeProcess, parseCatalogue, parseCatalogueValue, parseDictionary, parseGeometryCatalogue, parseIso16757ExchangeProcess, parsePartNumberRule, parseScriptLimits, parseSelectionRequest, type PartNumberRule, type ScriptLimits, type SelectionRequest } from "../📸️snapshot/🟦️.ts";
import { type Iso16757Artifact, parseIso16757Artifact } from "../🟦️.ts";

export interface Iso16757Diff {
  /** @state artifact */
  artifact: Iso16757Artifact | null;
  /** @state artifact */
  catalogue: Catalogue | null;
  /** @state artifact */
  dictionary: Dictionary | null;
  /** @state artifact */
  geometry: GeometryCatalogue | null;
  /** @state artifact */
  selection: SelectionRequest | null;
  /** @state artifact */
  partNumberRule: PartNumberRule | null;
  /** @state artifact */
  partNumberInputs: { [key: string]: CatalogueValue } | null;
  /** @state artifact */
  scriptLimits: ScriptLimits | null;
  /** @state artifact */
  exchangeProcess: Iso16757ExchangeProcess | null;
}

export const parseIso16757Diff: NormWireReader<Iso16757Diff> = normWireObject<Iso16757Diff>({ artifact: normWireDefault(normWireNullable(parseIso16757Artifact), () => null), catalogue: normWireDefault(normWireNullable(parseCatalogue), () => null), dictionary: normWireDefault(normWireNullable(parseDictionary), () => null), geometry: normWireDefault(normWireNullable(parseGeometryCatalogue), () => null), selection: normWireDefault(normWireNullable(parseSelectionRequest), () => null), partNumberRule: normWireDefault(normWireNullable(parsePartNumberRule), () => null), partNumberInputs: normWireDefault(normWireNullable(normWireMap(parseCatalogueValue)), () => null), scriptLimits: normWireDefault(normWireNullable(parseScriptLimits), () => null), exchangeProcess: normWireDefault(normWireNullable(parseIso16757ExchangeProcess), () => null) });
