// #region Header

// index.tsx

// Auto-registration of all editors
// Import this file to register all editors with the registry

// #endregion

import "./design/registration";
import "./docs/registration";
import "./home/registration";
import "./kit/registration";
import "./quality/registration";
import "./type/registration";

export { editorRegistry } from "./registry";
export type { EditorRegistration, PanelDefinition, RouteSegment } from "./registry";
