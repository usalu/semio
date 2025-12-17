import { Sketchpad } from "@semio/js";
import { createRoot } from "react-dom/client";
import "./globals.css";




import { appRegistry } from "../js/sketchpad/Sketchpad";


import { config as designConfig } from "../js/sketchpad/Design";
import { config as docsConfig } from "../js/sketchpad/Docs";
import { config as feedbackConfig } from "../js/sketchpad/Feedback";
import { config as homeConfig } from "../js/sketchpad/Home";
import { config as kitConfig } from "../js/sketchpad/Kit";
import { config as qualityConfig } from "../js/sketchpad/Quality";
import { config as typeConfig } from "../js/sketchpad/Type";


appRegistry.register(designConfig);
appRegistry.register(docsConfig);
appRegistry.register(feedbackConfig);
appRegistry.register(homeConfig);
appRegistry.register(kitConfig);
appRegistry.register(qualityConfig);
appRegistry.register(typeConfig);



createRoot(document.getElementById("root")!).render(
  <div className="h-screen w-screen">
    <Sketchpad importKitUrls={["/metabolism.zip"]} />
  </div>,
);
