/** 🌐️ BIM model direct mutations for the spatial structure: site, building, storeys, project info, grid lines, zones, area schemes and spaces. */
import type { AreaMeasure, AreaScheme, Building, GridLine, Point2, Site, Space, SpaceBoundary, Storey, Zone } from "../../🟦️.ts";

export interface CreateSite {
  mutation: "createSite";
  id: string;
  site: Site;
}

export interface DeleteSite {
  mutation: "deleteSite";
  id: string;
}

export interface CreateBuilding {
  mutation: "createBuilding";
  id: string;
  building: Building;
}

export interface DeleteBuilding {
  mutation: "deleteBuilding";
  id: string;
}

export interface CreateStorey {
  mutation: "createStorey";
  id: string;
  storey: Storey;
}

export interface RenameStorey {
  mutation: "renameStorey";
  id: string;
  name: string;
}

export interface SetStoreyHeight {
  mutation: "setStoreyHeight";
  id: string;
  height: number;
}

export interface SetStoreyLevel {
  mutation: "setStoreyLevel";
  id: string;
  level: number;
}

export interface DeleteStorey {
  mutation: "deleteStorey";
  id: string;
}

export interface SetStoreyCutHeight {
  mutation: "setStoreyCutHeight";
  id: string;
  cut_height: Record<string, unknown>;
}

export interface SetProjectInfo {
  mutation: "setProjectInfo";
  name?: string;
  description?: string;
  author?: string;
  organization?: string;
  phase_names?: string[];
}

export interface SetSite {
  mutation: "setSite";
  id: string;
  name?: string;
  latitude?: number;
  longitude?: number;
  elevation?: number;
  true_north?: number;
  boundary?: Point2[];
}

export interface SetBuilding {
  mutation: "setBuilding";
  id: string;
  name?: string;
  origin?: Point2;
  rotation?: number;
  elevation?: number;
}

export interface CreateGridLine {
  mutation: "createGridLine";
  id: string;
  grid_line: GridLine;
}

export interface DeleteGridLine {
  mutation: "deleteGridLine";
  id: string;
}

export interface SetGridLine {
  mutation: "setGridLine";
  id: string;
  label?: string;
  start?: Point2;
  end?: Point2;
}

export interface CreateSpace {
  mutation: "createSpace";
  id: string;
  space: Space;
}

export interface DeleteSpace {
  mutation: "deleteSpace";
  id: string;
}

export interface SetSpace {
  mutation: "setSpace";
  id: string;
  number?: string;
  name?: string;
  boundary?: SpaceBoundary;
  usage?: string;
  zone?: Record<string, unknown>;
  floor_finish?: Record<string, unknown>;
  wall_finish?: Record<string, unknown>;
  ceiling_finish?: Record<string, unknown>;
}

export interface CreateZone {
  mutation: "createZone";
  id: string;
  zone: Zone;
}

export interface SetZone {
  mutation: "setZone";
  id: string;
  name: string;
  category: string;
  occupancy_density: number;
}

export interface DeleteZone {
  mutation: "deleteZone";
  id: string;
}

export interface CreateAreaScheme {
  mutation: "createAreaScheme";
  id: string;
  area_scheme: AreaScheme;
}

export interface SetAreaScheme {
  mutation: "setAreaScheme";
  id: string;
  name: string;
  measure: AreaMeasure;
  usages: string[];
  zones: string[];
}

export interface DeleteAreaScheme {
  mutation: "deleteAreaScheme";
  id: string;
}

export interface SetSpaceConditions {
  mutation: "setSpaceConditions";
  id: string;
  occupancy?: Record<string, unknown>;
  occupancy_density?: Record<string, unknown>;
  heating_setpoint?: Record<string, unknown>;
  cooling_setpoint?: Record<string, unknown>;
  ventilation_rate?: Record<string, unknown>;
  lighting_power_density?: Record<string, unknown>;
  equipment_power_density?: Record<string, unknown>;
  schedule?: Record<string, unknown>;
}

export interface RemoveSpaceConditions {
  mutation: "removeSpaceConditions";
  id: string;
}
