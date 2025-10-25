// #region Header

// index.tsx

// Auto-registration of all editors
// Import this file to register all editors with the registry

// #endregion

import "./home/registration";
import "./kit/registration";
import "./design/registration";
import "./type/registration";
import "./quality/registration";
import "./docs/registration";

export { editorRegistry } from "./registry";
export type { EditorRegistration, PanelDefinition, RouteSegment } from "./registry";
