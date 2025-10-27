// #region Header

// index.tsx

// Auto-registration of all editors
// Editors are automatically discovered from the file system

// #endregion

import { editorRegistry } from "./registry";

editorRegistry.initialize();

export { editorRegistry } from "./registry";
export type { EditorConfig, EditorRegistration, PanelDefinition, RouteSegment } from "./registry";
