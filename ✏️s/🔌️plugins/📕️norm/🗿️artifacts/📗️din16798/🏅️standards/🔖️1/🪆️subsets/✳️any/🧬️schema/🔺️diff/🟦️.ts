/** 🧬️ Din16798 diff schema — sparse field delta. */

export interface Din16798Diff {
  /** @state artifact */
  artifact?: Din16798Artifact;
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  occupancy?: string;
  /** @state artifact */
  comfortCategory?: string;
  /** @state artifact */
  tOpC?: number;
  /** @state artifact */
  rhPercent?: number;
  /** @state artifact */
  airSpeedMS?: number;
  /** @state artifact */
  thetaRmC?: number;
  /** @state artifact */
  co2Ppm?: number;
  /** @state artifact */
  dfPercent?: number;
  /** @state artifact */
  lAeqDb?: number;
  /** @state artifact */
  persons?: number;
  /** @state artifact */
  idaClass?: string;
  /** @state artifact */
  ventilationM3H?: number;
  /** @state artifact */
  floorAreaM2?: number;
  /** @state artifact */
  bedrooms?: number;
  /** @state artifact */
  dwellingVentilationM3H?: number;
  /** @state artifact */
  occupants?: number;
  /** @state artifact */
  residentialVentilationM3H?: number;
  /** @state artifact */
  sfpWM3S?: number;
  /** @state artifact */
  sfpRequiredClass?: number;
  /** @state artifact */
  heatRecoveryEta?: number;
  /** @state artifact */
  heatRecoveryEtaMin?: number;
  /** @state artifact */
  systemType?: string;
  /** @state artifact */
  yearsSinceInspection?: number;
  /** @state artifact */
  humidificationRequiredKgH?: number;
  /** @state artifact */
  humidificationProvidedKgH?: number;
  /** @state artifact */
  fanQVM3S?: number;
  /** @state artifact */
  fanTRunH?: number;
  /** @state artifact */
  fanEnergyReferenceKwh?: number;
  /** @state artifact */
  nightSetbackK?: number;
  /** @state artifact */
  hrMDotKgS?: number;
  /** @state artifact */
  hrCpJKgk?: number;
  /** @state artifact */
  hrDeltaTC?: number;
  /** @state artifact */
  hrTH?: number;
  /** @state artifact */
  hrSavingsReferenceKwh?: number;
  /** @state artifact */
  n50HInv?: number;
  /** @state artifact */
  volumeM3?: number;
  /** @state artifact */
  infiltrationAllowanceM3H?: number;
  /** @state artifact */
  cellarAreaM2?: number;
  /** @state artifact */
  cellarVentilationM3H?: number;
  /** @state artifact */
  hTrWK?: number;
  /** @state artifact */
  hVeWK?: number;
  /** @state artifact */
  thetaEC?: number;
  /** @state artifact */
  thetaSetC?: number;
  /** @state artifact */
  coolingDeltaTH?: number;
  /** @state artifact */
  coolingGainsKwh?: number;
  /** @state artifact */
  coolingUtilizationFactor?: number;
  /** @state artifact */
  coolingReferenceKwh?: number;
  /** @state artifact */
  chillerType?: string;
  /** @state artifact */
  eerActual?: number;
  /** @state artifact */
  qCKwh?: number;
  /** @state artifact */
  generationReferenceKwh?: number;
  /** @state artifact */
  dataCenterSupplyC?: number;
  /** @state artifact */
  hStWK?: number;
  /** @state artifact */
  thetaStC?: number;
  /** @state artifact */
  thetaAmbC?: number;
  /** @state artifact */
  storageTH?: number;
  /** @state artifact */
  storageAllowanceKwh?: number;
  /** @state artifact */
  dhwDeliveryC?: number;
  /** @state artifact */
  ductClass?: string;
  /** @state artifact */
  ductTestPressurePa?: number;
  /** @state artifact */
  ductLeakageM3SM2?: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}

export interface Din16798Artifact {
  annex: string;
  occupancy: string;
  comfortCategory: string;
  tOpC: number;
  rhPercent: number;
  airSpeedMS: number;
  thetaRmC: number;
  co2Ppm: number;
  dfPercent: number;
  lAeqDb: number;
  persons: number;
  idaClass: string;
  ventilationM3H: number;
  floorAreaM2: number;
  bedrooms: number;
  dwellingVentilationM3H: number;
  occupants: number;
  residentialVentilationM3H: number;
  sfpWM3S: number;
  sfpRequiredClass: number;
  heatRecoveryEta: number;
  heatRecoveryEtaMin: number;
  systemType: string;
  yearsSinceInspection: number;
  humidificationRequiredKgH: number;
  humidificationProvidedKgH: number;
  fanQVM3S: number;
  fanTRunH: number;
  fanEnergyReferenceKwh: number;
  nightSetbackK: number;
  hrMDotKgS: number;
  hrCpJKgk: number;
  hrDeltaTC: number;
  hrTH: number;
  hrSavingsReferenceKwh: number;
  n50HInv: number;
  volumeM3: number;
  infiltrationAllowanceM3H: number;
  cellarAreaM2: number;
  cellarVentilationM3H: number;
  hTrWK: number;
  hVeWK: number;
  thetaEC: number;
  thetaSetC: number;
  coolingDeltaTH: number;
  coolingGainsKwh: number;
  coolingUtilizationFactor: number;
  coolingReferenceKwh: number;
  chillerType: string;
  eerActual: number;
  qCKwh: number;
  generationReferenceKwh: number;
  dataCenterSupplyC: number;
  hStWK: number;
  thetaStC: number;
  thetaAmbC: number;
  storageTH: number;
  storageAllowanceKwh: number;
  dhwDeliveryC: number;
  ductClass: string;
  ductTestPressurePa: number;
  ductLeakageM3SM2: number;
  selectedCheckIndex?: number | null;
}
