import {
  applySOsUri,
  createStudioStore,
  createEmptyStudioDocument,
  S_HOME_APP_ID,
  S_PLAY_APP_ID,
  SHomeController,
  SPlayController,
  seedOsBootStudioCatalog,
  loadSPlayStudioDocument,
  S_PLAY_EXAMPLE_DEFAULT_ID,
  type SOsShellRefs,
} from "@semio-tech/s-core";
import { createProductPlaygroundPlatform } from "@semio-tech/framework-playground-core";
import { createOsStudio, deleteOsStudio, listOsStudioCatalogEntries } from "@semio-tech/framework-os-core";

const entry = createOsStudio("[DEBUG] Verify Studio");
console.log("[DEBUG] created studio", entry.id);
const listed = listOsStudioCatalogEntries();
console.log(
  "[DEBUG] catalog count",
  listed.length,
  listed.map((row) => row.name),
);
const seeded = seedOsBootStudioCatalog();
console.log("[DEBUG] seeded studio", seeded.id, seeded.name);

const runtime = createProductPlaygroundPlatform("os-verify", "S");
const homeCtrl = new SHomeController(runtime.commandBus, () => runtime.notify());
const studioCtrl = new SPlayController(runtime.commandBus, () => runtime.notify(), createStudioStore(createEmptyStudioDocument("bootstrap", "Bootstrap")), S_PLAY_EXAMPLE_DEFAULT_ID, loadSPlayStudioDocument);
const shell: SOsShellRefs = { home: homeCtrl, studio: studioCtrl };
applySOsUri(runtime, "/", shell);
console.log("[DEBUG] home route activeAppId", runtime.activeAppId, runtime.activeAppId === S_HOME_APP_ID);
applySOsUri(runtime, `/studios/${entry.id}`, shell);
console.log("[DEBUG] studio route activeAppId", runtime.activeAppId, runtime.activeAppId === S_PLAY_APP_ID);
console.log("[DEBUG] studio loaded id", studioCtrl.getStudioId());
deleteOsStudio(entry.id);
console.log("[DEBUG] os home/studio runtime verify ok");
