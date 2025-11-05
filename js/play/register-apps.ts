// Explicitly register all apps for the play app
// This is needed because import.meta.glob doesn't work across package boundaries

import { appRegistry } from "../js/sketchpad/apps/index";

// Import all app configs
import { config as designConfig } from "../js/sketchpad/apps/design/config";
import { config as docsConfig } from "../js/sketchpad/apps/docs/config";
import { config as homeConfig } from "../js/sketchpad/apps/home/config";
import { config as kitConfig } from "../js/sketchpad/apps/kit/config";
import { config as qualityConfig } from "../js/sketchpad/apps/quality/config";
import { config as typeConfig } from "../js/sketchpad/apps/type/config";

// Register all apps
appRegistry.register(designConfig);
appRegistry.register(docsConfig);
appRegistry.register(homeConfig);
appRegistry.register(kitConfig);
appRegistry.register(qualityConfig);
appRegistry.register(typeConfig);
