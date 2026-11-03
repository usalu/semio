/** 🧩️ `LayoutPrefs` wire twin, mirrors `./🦀️.rs` (canonical JSON: camelCase keys, unit enums as camelCase strings). A persisted, local-only preference. */
export type SheetState = "collapsed" | "half" | "full";

export interface DockPrefs {
  left: number;
  centre: number;
  right: number;
  leftSplit: number;
  rightSplit: number;
  hidden: string[];
}

export interface TabletPrefs {
  viewport: number;
  sideTabs: string[];
  sideActive: string;
}

export interface MobilePrefs {
  tabs: string[];
  active: string;
  sheet: SheetState;
  sheetHeight: number;
}

export interface LayoutPrefs {
  dock: DockPrefs;
  tablet: TabletPrefs;
  mobile: MobilePrefs;
}

export interface DeviceSignal {
  widthPx: number;
  touch: boolean;
}
