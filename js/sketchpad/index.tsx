import { Sketchpad } from "@semio/js";
import { createRoot } from "react-dom/client";
import "./globals.css";

// Explicitly register all apps for the play app
// This is needed because import.meta.glob doesn't work across package boundaries

import { appRegistry } from "../js/sketchpad/Sketchpad";

// Import all app configs
import { config as designConfig } from "../js/sketchpad/Design";
import { config as docsConfig } from "../js/sketchpad/Docs";
import { config as homeConfig } from "../js/sketchpad/Home";
import { config as kitConfig } from "../js/sketchpad/Kit";
import { config as qualityConfig } from "../js/sketchpad/Quality";
import { config as typeConfig } from "../js/sketchpad/Type";

// Register all apps
appRegistry.register(designConfig);
appRegistry.register(docsConfig);
appRegistry.register(homeConfig);
appRegistry.register(kitConfig);
appRegistry.register(qualityConfig);
appRegistry.register(typeConfig);

// render the app
// NOTE: StrictMode disabled temporarily to avoid double rendering during performance testing
createRoot(document.getElementById("root")!).render(
  <div className="h-screen w-screen">
    <Sketchpad importKitUrls={["/metabolism.zip"]} />
  </div>,
);
