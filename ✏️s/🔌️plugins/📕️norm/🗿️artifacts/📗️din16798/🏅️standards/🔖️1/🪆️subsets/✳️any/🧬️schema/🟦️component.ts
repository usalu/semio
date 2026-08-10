/** 🧬️ Din16798 artifact schema — every field with its state class. */

export interface Din16798Artifact {
  /** @state persistent */
  annex: string;
  /** @state persistent */
  occupancy: string;
  /** @state persistent */
  comfortCategory: string;
  /** @state persistent */
  tOpC: number;
  /** @state persistent */
  rhPercent: number;
  /** @state persistent */
  airSpeedMS: number;
  /** @state persistent */
  thetaRmC: number;
  /** @state persistent */
  co2Ppm: number;
  /** @state persistent */
  dfPercent: number;
  /** @state persistent */
  lAeqDb: number;
  /** @state persistent */
  persons: number;
  /** @state persistent */
  idaClass: string;
  /** @state persistent */
  ventilationM3H: number;
  /** @state persistent */
  floorAreaM2: number;
  /** @state persistent */
  bedrooms: number;
  /** @state persistent */
  dwellingVentilationM3H: number;
  /** @state persistent */
  occupants: number;
  /** @state persistent */
  residentialVentilationM3H: number;
  /** @state persistent */
  sfpWM3S: number;
  /** @state persistent */
  sfpRequiredClass: number;
  /** @state persistent */
  heatRecoveryEta: number;
  /** @state persistent */
  heatRecoveryEtaMin: number;
  /** @state persistent */
  systemType: string;
  /** @state persistent */
  yearsSinceInspection: number;
  /** @state persistent */
  humidificationRequiredKgH: number;
  /** @state persistent */
  humidificationProvidedKgH: number;
  /** @state persistent */
  fanQVM3S: number;
  /** @state persistent */
  fanTRunH: number;
  /** @state persistent */
  fanEnergyReferenceKwh: number;
  /** @state persistent */
  nightSetbackK: number;
  /** @state persistent */
  hrMDotKgS: number;
  /** @state persistent */
  hrCpJKgk: number;
  /** @state persistent */
  hrDeltaTC: number;
  /** @state persistent */
  hrTH: number;
  /** @state persistent */
  hrSavingsReferenceKwh: number;
  /** @state persistent */
  n50HInv: number;
  /** @state persistent */
  volumeM3: number;
  /** @state persistent */
  infiltrationAllowanceM3H: number;
  /** @state persistent */
  cellarAreaM2: number;
  /** @state persistent */
  cellarVentilationM3H: number;
  /** @state persistent */
  hTrWK: number;
  /** @state persistent */
  hVeWK: number;
  /** @state persistent */
  thetaEC: number;
  /** @state persistent */
  thetaSetC: number;
  /** @state persistent */
  coolingDeltaTH: number;
  /** @state persistent */
  coolingGainsKwh: number;
  /** @state persistent */
  coolingUtilizationFactor: number;
  /** @state persistent */
  coolingReferenceKwh: number;
  /** @state persistent */
  chillerType: string;
  /** @state persistent */
  eerActual: number;
  /** @state persistent */
  qCKwh: number;
  /** @state persistent */
  generationReferenceKwh: number;
  /** @state persistent */
  dataCenterSupplyC: number;
  /** @state persistent */
  hStWK: number;
  /** @state persistent */
  thetaStC: number;
  /** @state persistent */
  thetaAmbC: number;
  /** @state persistent */
  storageTH: number;
  /** @state persistent */
  storageAllowanceKwh: number;
  /** @state persistent */
  dhwDeliveryC: number;
  /** @state persistent */
  ductClass: string;
  /** @state persistent */
  ductTestPressurePa: number;
  /** @state persistent */
  ductLeakageM3SM2: number;
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}
