/** 🔺️ `Din4108Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din4108ClimateZoneDe, type Din4108EnvelopeElement, type Din4108ThermalBridge, type Din4108ThermalZone, parseDin4108ClimateZoneDe, parseDin4108EnvelopeElement, parseDin4108ThermalBridge, parseDin4108ThermalZone } from "../📸️snapshot/🟦️.ts";

export interface Din4108Diff {
  /** @state artifact */
  climateZone: Din4108ClimateZoneDe | null;
  /** @state artifact */
  usage: string | null;
  /** @state artifact */
  tIntC: number | null;
  /** @state artifact */
  rhInt: number | null;
  /** @state artifact */
  hasMechanicalVentilation: boolean | null;
  /** @state artifact */
  airtightnessN50: number | null;
  /** @state artifact */
  bb2DetailsConform: boolean | null;
  /** @state artifact */
  zones: { values: Din4108ThermalZone[]; } | null;
  /** @state artifact */
  elements: { values: Din4108EnvelopeElement[]; } | null;
  /** @state artifact */
  thermalBridges: { values: Din4108ThermalBridge[]; } | null;
}

export const parseDin4108Diff: NormWireReader<Din4108Diff> = normWireObject<Din4108Diff>({ climateZone: normWireDefault(normWireNullable(parseDin4108ClimateZoneDe), () => null), usage: normWireDefault(normWireNullable(normWireString), () => null), tIntC: normWireDefault(normWireNullable(normWireNumber), () => null), rhInt: normWireDefault(normWireNullable(normWireNumber), () => null), hasMechanicalVentilation: normWireDefault(normWireNullable(normWireBoolean), () => null), airtightnessN50: normWireDefault(normWireNullable(normWireNumber), () => null), bb2DetailsConform: normWireDefault(normWireNullable(normWireBoolean), () => null), zones: normWireDefault(normWireNullable(normWireObject<{ values: Din4108ThermalZone[]; }>({ values: normWireRequired(normWireArray(parseDin4108ThermalZone)) })), () => null), elements: normWireDefault(normWireNullable(normWireObject<{ values: Din4108EnvelopeElement[]; }>({ values: normWireRequired(normWireArray(parseDin4108EnvelopeElement)) })), () => null), thermalBridges: normWireDefault(normWireNullable(normWireObject<{ values: Din4108ThermalBridge[]; }>({ values: normWireRequired(normWireArray(parseDin4108ThermalBridge)) })), () => null) });
