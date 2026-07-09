/** [DEBUG] Verifies side panels only resolve through declarative tree factories. */
import { Platform, getSidePanelBodyFactory, registerSidePanelBody, unregisterSidePanelBody } from "@semio-tech/framework-platform-core";
import { createFrameworkSettingsPanelTabs, type SettingsHostApi } from "@semio-tech/framework-platform-renderer-react";
import { Expertise } from "@semio-tech/ui-react";

const settingsHost: SettingsHostApi = {
  compact: false,
  setCompact: () => {},
  expertise: Expertise.NORMAL,
  setExpertise: () => {},
  computeWorkerCount: 4,
  setComputeWorkerCount: () => {},
  computeThreadsAvailable: true,
  appId: "verify",
  appLabel: "Verify",
  modes: [{ id: "edit", label: "Edit" }],
  activeModeId: "edit",
  setActiveModeId: () => {},
  hasModeNav: true,
};

const wb = new Platform();
const settingsTabs = createFrameworkSettingsPanelTabs(
  () => settingsHost,
  () => null,
  () => wb,
  wb.commandBus,
);
for (const tab of settingsTabs) {
  const tree = tab.tree;
  if (!tree || typeof tree !== "object" || !("resolveTree" in tree)) {
    throw new Error(`[DEBUG] settings tab ${tab.id} is not a tree definition`);
  }
  const config = tree.resolveTree();
  if (!config.sections.length || !config.sections[0]?.items?.length) {
    throw new Error(`[DEBUG] settings tab ${tab.id} resolved to an empty tree`);
  }
}
console.log("[DEBUG] settings tabs resolve as non-empty trees:", settingsTabs.map((tab) => tab.id).join(", "));

registerSidePanelBody("verify.side-panel.bad", () => ({ type: "text", value: "x" }) as never);
let rejected = false;
try {
  getSidePanelBodyFactory("verify.side-panel.bad")?.({
    platform: wb,
    windowKindId: "tab",
    bodyKey: "verify.side-panel.bad",
    activeModeId: null,
    generation: 0,
  });
} catch {
  rejected = true;
}
unregisterSidePanelBody("verify.side-panel.bad");
if (!rejected) {
  throw new Error("[DEBUG] non-tree side panel body was not rejected");
}
console.log("[DEBUG] non-tree side panel registration is rejected on invoke");
