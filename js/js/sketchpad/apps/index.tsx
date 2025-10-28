// #region Header

// index.tsx

// Auto-registration of all apps
// Apps are automatically discovered from the file system

// #endregion

// Note: appRegistry.initialize() is called by Sketchpad.tsx to ensure
// initialization happens before routes are generated

export { appRegistry } from "./registry";
export type { AppConfig, AppRegistration, PanelDefinition, RouteSegment } from "./registry";
