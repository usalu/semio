/** 🧬️ Din16798 artifact schema — every field with its state class. */

export interface Din16798Artifact {
  /** @state artifact */
  annex: string;
  /** @state artifact */
  occupancy: string;
  /** @state artifact */
  comfortCategory: string;
  /** @state artifact */
  tOpC: number;
  /** @state artifact */
  rhPercent: number;
  /** @state artifact */
  airSpeedMS: number;
  /** @state artifact */
  thetaRmC: number;
  /** @state artifact */
  co2Ppm: number;
  /** @state artifact */
  dfPercent: number;
  /** @state artifact */
  lAeqDb: number;
  /** @state artifact */
  persons: number;
  /** @state artifact */
  idaClass: string;
  /** @state artifact */
  ventilationM3H: number;
  /** @state artifact */
  floorAreaM2: number;
  /** @state artifact */
  bedrooms: number;
  /** @state artifact */
  dwellingVentilationM3H: number;
  /** @state artifact */
  occupants: number;
  /** @state artifact */
  residentialVentilationM3H: number;
  /** @state artifact */
  sfpWM3S: number;
  /** @state artifact */
  sfpRequiredClass: number;
  /** @state artifact */
  heatRecoveryEta: number;
  /** @state artifact */
  heatRecoveryEtaMin: number;
  /** @state artifact */
  systemType: string;
  /** @state artifact */
  yearsSinceInspection: number;
  /** @state artifact */
  humidificationRequiredKgH: number;
  /** @state artifact */
  humidificationProvidedKgH: number;
  /** @state artifact */
  fanQVM3S: number;
  /** @state artifact */
  fanTRunH: number;
  /** @state artifact */
  fanEnergyReferenceKwh: number;
  /** @state artifact */
  nightSetbackK: number;
  /** @state artifact */
  hrMDotKgS: number;
  /** @state artifact */
  hrCpJKgk: number;
  /** @state artifact */
  hrDeltaTC: number;
  /** @state artifact */
  hrTH: number;
  /** @state artifact */
  hrSavingsReferenceKwh: number;
  /** @state artifact */
  n50HInv: number;
  /** @state artifact */
  volumeM3: number;
  /** @state artifact */
  infiltrationAllowanceM3H: number;
  /** @state artifact */
  cellarAreaM2: number;
  /** @state artifact */
  cellarVentilationM3H: number;
  /** @state artifact */
  hTrWK: number;
  /** @state artifact */
  hVeWK: number;
  /** @state artifact */
  thetaEC: number;
  /** @state artifact */
  thetaSetC: number;
  /** @state artifact */
  coolingDeltaTH: number;
  /** @state artifact */
  coolingGainsKwh: number;
  /** @state artifact */
  coolingUtilizationFactor: number;
  /** @state artifact */
  coolingReferenceKwh: number;
  /** @state artifact */
  chillerType: string;
  /** @state artifact */
  eerActual: number;
  /** @state artifact */
  qCKwh: number;
  /** @state artifact */
  generationReferenceKwh: number;
  /** @state artifact */
  dataCenterSupplyC: number;
  /** @state artifact */
  hStWK: number;
  /** @state artifact */
  thetaStC: number;
  /** @state artifact */
  thetaAmbC: number;
  /** @state artifact */
  storageTH: number;
  /** @state artifact */
  storageAllowanceKwh: number;
  /** @state artifact */
  dhwDeliveryC: number;
  /** @state artifact */
  ductClass: string;
  /** @state artifact */
  ductTestPressurePa: number;
  /** @state artifact */
  ductLeakageM3SM2: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}
