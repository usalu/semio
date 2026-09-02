/** 🧬️ Din18599 direct-mutation discriminated union — mirrors `Din18599Mutation` in `🦀️.rs`
 * (13 variants: one `change-<field>` leaf per document-root scalar, plus one `update-climate` for
 * the inseparable two-array `MonthlyClimate` facet). */

export type UseClass = "residential" | "office" | "school";

export interface MonthlyClimate {
  thetaEC: [number, number, number, number, number, number, number, number, number, number, number, number];
  gHWM2: [number, number, number, number, number, number, number, number, number, number, number, number];
}

export interface ChangeUseClass {
  newUseClass: UseClass;
}

export interface ChangeHeatedAreaM2 {
  newHeatedAreaM2: number;
}

export interface ChangeOccupants {
  newOccupants: number;
}

export interface ChangeHT {
  newHT: number;
}

export interface ChangeHV {
  newHV: number;
}

export interface ChangeInternalGainsWM2 {
  newInternalGainsWM2: number;
}

export interface ChangeSolarGainsKwh {
  newSolarGainsKwh: number;
}

export interface ChangeSystemLossesKwh {
  newSystemLossesKwh: number;
}

export interface ChangeRenewableKwh {
  newRenewableKwh: number;
}

export interface ChangeAnnualLimitKwh {
  newAnnualLimitKwh: number;
}

export interface ChangeEnergyCarrier {
  newEnergyCarrier: string;
}

export interface ChangeReferenceQPKwh {
  newReferenceQPKwh: number;
}

export interface UpdateClimate {
  newClimate: MonthlyClimate;
}

export type Din18599Mutation =
  | ({ mutation: "changeUseClass" } & ChangeUseClass)
  | ({ mutation: "changeHeatedAreaM2" } & ChangeHeatedAreaM2)
  | ({ mutation: "changeOccupants" } & ChangeOccupants)
  | ({ mutation: "changeHT" } & ChangeHT)
  | ({ mutation: "changeHV" } & ChangeHV)
  | ({ mutation: "changeInternalGainsWM2" } & ChangeInternalGainsWM2)
  | ({ mutation: "changeSolarGainsKwh" } & ChangeSolarGainsKwh)
  | ({ mutation: "changeSystemLossesKwh" } & ChangeSystemLossesKwh)
  | ({ mutation: "changeRenewableKwh" } & ChangeRenewableKwh)
  | ({ mutation: "changeAnnualLimitKwh" } & ChangeAnnualLimitKwh)
  | ({ mutation: "changeEnergyCarrier" } & ChangeEnergyCarrier)
  | ({ mutation: "changeReferenceQPKwh" } & ChangeReferenceQPKwh)
  | ({ mutation: "updateClimate" } & UpdateClimate);
