// #region Header

// index.tsx

// Auto-registration of all apps
// Apps are automatically discovered from the file system

// #endregion

import { appRegistry } from "./registry";

appRegistry.initialize();

export { appRegistry } from "./registry";
export type { AppConfig, AppRegistration, PanelDefinition, RouteSegment } from "./registry";
