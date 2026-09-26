export type Din16798Zone = {
  id: string;
  name: string;
  usageType: string;
  floorAreaM2: number;
  occupants: number;
  comfortCategory: string;
  pollutionClass: string;
  comfortModel: string;
  tOpWinterC: number;
  tOpSummerC: number;
  airSpeedMS: number;
  clothingClo: number;
  metabolicRateMet: number;
  rhPercent: number;
  outdoorAirSuppliedM3H: number;
  co2Ppm: number;
  illuminanceLx: number;
  noiseDb: number;
  turbulenceIntensityPercent: number;
  ventMethod: string;
  ventSystemId: string;
};

export type Din16798VentSystem = {
  id: string;
  name: string;
  systemType: string;
  sfpWM3S: number;
  sfpRequiredClass: number;
  heatRecoveryEta: number;
  odaClass: string;
  filterSupClass: string;
  yearsSinceInspection: number;
  humidificationRequiredKgH: number;
  humidificationProvidedKgH: number;
  fanQVM3S: number;
  fanTRunH: number;
  ductClass: string;
  ductTestPressurePa: number;
  ductLeakageM3SM2: number;
  designAirflowM3H: number;
};

export type Din16798Snapshot = {
  annex: string;
  thetaRmC: number;
  outdoorCo2Ppm: number;
  zones: Din16798Zone[];
  ventSystems: Din16798VentSystem[];
  envelopeN50HInv: number;
  envelopeVolumeM3: number;
  cellarAreaM2: number;
  cellarVentilationM3H: number;
  nightSetbackK: number;
};
