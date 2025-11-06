// Explicitly register all apps for the play app
// This is needed because import.meta.glob doesn't work across package boundaries

import { appRegistry } from "../js/sketchpad/apps/index";

// Import all app configs
import { config as designConfig } from "../js/sketchpad/apps/design/App";
import { config as docsConfig } from "../js/sketchpad/apps/docs/App";
import { config as homeConfig } from "../js/sketchpad/apps/home/App";
import { config as kitConfig } from "../js/sketchpad/apps/kit/App";
import { config as qualityConfig } from "../js/sketchpad/apps/quality/App";
import { config as typeConfig } from "../js/sketchpad/apps/type/App";

// Register all apps
appRegistry.register(designConfig);
appRegistry.register(docsConfig);
appRegistry.register(homeConfig);
appRegistry.register(kitConfig);
appRegistry.register(qualityConfig);
appRegistry.register(typeConfig);
