import { createRoot } from "react-dom/client";
import { InterpretedUiNode } from "probe:interpreter";
import { TreeWindowContext } from "probe:tree-context";
import { UiDocumentStore } from "probe:document-store";
import { ChromeAwareWindowScrollSurface } from "probe:window-chrome";
import { createTreeWindowContextV1, createTreeWindowSchedulerV1, uiIntentToActionDescriptor } from "probe:action-bridge";
import { UiI18nProvider, createShellI18nInstance } from "@semio-tech/ui-react";
import fixture from "probe:fixture";
import pages from "probe:pages";
import navigation from "probe:navigation";
import "probe:styles";

const locale = fixture.locales.find((entry) => entry.id === new URLSearchParams(location.search).get("locale")) ?? fixture.locales[0];
const de = locale.id === "de";
const labels = { previous: de ? "Vorherige Seite" : "Previous page", next: de ? "Nächste Seite" : "Next page" };
const surface = navigation.windows[0];
const store = new UiDocumentStore(surface);
const law = pages.cases.find((item) => item.name === "long scalar form");
const viewport = { kind: "scroll", axes: "vertical", padding: { all: "none" }, sizing: "fill" };
const stack = { kind: "stack", axis: "vertical", gap: "none", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false };
const leaf = { kind: "leaf", width: "hug", height: "hug" };
const node = (id, key, component, layout, children = [], accessibility = {}, bindings = [], disabled = false) => ({ id, key, component, layout, children, accessibility, style: {}, bindings, disabled });
let revision = 0;
function publish(page) {
  const button = (id, key, label, icon, target, available) => node(id, key, { type: "button", label, icon }, leaf, [], {}, available ? [{
    trigger: "activate", action: { scope: navigation.controller, name: navigation.action, version: 1 }, args: { windowId: surface, page: target },
  }] : [], !available);
  const first = page === 0 ? navigation.first : law.expect.first;
  const last = page === 0 ? law.expect.first : law.expect.last;
  const questionNodes = Array.from({ length: last - first }, (_, i) => {
    const ordinal = first + i;
    const id = 7 + i * 2;
    const key = "generate.form.size-" + ordinal;
    const label = (de ? "Größe " : "Size ") + ordinal;
    return [
      node(id, key, { type: "container", role: "field", label }, stack, [id + 1]),
      node(id + 1, key + ".control", { type: "input", kind: "number", value: "1", commit: "blur" }, leaf, [], { label }, [{
        trigger: "commit", action: { scope: "s.procedural.generation3d", name: "updateGenerationValues", version: 1 },
        args: { generationId: fixture.generation, questionId: "size-" + ordinal },
      }]),
    ];
  }).flat();
  store.loadSnapshot({ surface, revision: ++revision, root: 0, layoutEpoch: 0n, nodes: [
    node(0, "generate.form", { type: "container", role: "form", label: locale.label }, { ...stack, grow: true }, [1, 5]),
    node(1, "generate.form.navigation", { type: "container", role: "plain" }, { ...stack, axis: "horizontal" }, [2, 3, 4]),
    button(2, "generate.form.previous", labels.previous, "arrow-left", navigation.first, page !== navigation.first),
    node(3, "generate.form.page.position", { type: "text", value: de ? "Seite " + (page + 1) + " von 2" : "Page " + (page + 1) + " of 2" }, leaf, [], { live: "polite" }),
    button(4, "generate.form.next", labels.next, "arrow-right", navigation.second, page !== navigation.second),
    node(5, "generate.form.viewport." + page, { type: "container", role: "plain" }, viewport, [6]),
    node(6, "generate.form.fields", { type: "container", role: "plain" }, stack, questionNodes.filter((record, i) => i % 2 === 0).map((record) => record.id)),
    ...questionNodes,
  ] });
}

const scheduler = createTreeWindowSchedulerV1({
  refresh(bodyKey) {
    const request = scheduler.viewStateFields().treeWindows?.find((item) => item.nodeKey === surface && item.bodyKey === bodyKey);
    publish(request?.offset ?? 0);
    queueMicrotask(() => scheduler.settled(bodyKey));
  },
});
const context = createTreeWindowContextV1(navigation.body, scheduler);
const i18n = createShellI18nInstance(locale.id, "native");
window.formIntents = [];
publish(navigation.first);
createRoot(document.querySelector("#root")).render(
  <UiI18nProvider instance={i18n}>
    <div style={{ height: fixture.viewport.height, width: fixture.viewport.width }}>
      <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        <TreeWindowContext.Provider value={context}>
          <InterpretedUiNode store={store} windowId={surface} onAction={() => {}} onIntent={(intent, windowId) => { window.formIntents.push({ windowId, action: uiIntentToActionDescriptor(intent) }); }} />
        </TreeWindowContext.Provider>
      </ChromeAwareWindowScrollSurface>
    </div>
  </UiI18nProvider>,
);
