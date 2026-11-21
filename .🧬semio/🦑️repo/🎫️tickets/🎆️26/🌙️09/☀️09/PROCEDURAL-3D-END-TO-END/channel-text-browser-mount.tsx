import { createRoot } from "react-dom/client";
import { InterpretedUiNode } from "probe:interpreter";
import { TreeWindowContext } from "probe:tree-context";
import { UiDocumentStore } from "probe:document-store";
import { ChromeAwareWindowScrollSurface } from "probe:window-chrome";
import { createTreeWindowContextV1, createTreeWindowSchedulerV1, uiIntentToActionDescriptor } from "probe:action-bridge";
import { UiI18nProvider, createShellI18nInstance } from "@semio-tech/ui-react";
import fixture from "probe:fixture";
import "probe:styles";
import { registerComponentSceneHostV1 } from "probe:scene-hosts";
import { TextEditorHost } from "probe:text-editor";
import { registerWasmBindingsV1 } from "probe:wasm-bindings";
import { decodePackValue } from "@semio-tech/framework/packs";

registerComponentSceneHostV1("text-editor", () => TextEditorHost);
class EditorSessionFixture {
  attachCanvas() {}
  renderFrame() {}
  setSize() {}
  setText(buffer) { this.document = { ...this.document, buffer }; }
  setCanvasThemeJson() {}
  setCaretVisible() {}
  free() {}
  caret() { return 0; }
  anchor() { return 0; }
  document = {};
  text() { return this.document.buffer ?? ""; }
  synchronizeScene(pack) {
    this.document = { ...this.document, ...decodePackValue(pack) };
    window.channelEditorPacket = this.document;
  }
}
registerWasmBindingsV1({ editor: async () => ({ default: async () => {}, EditorSession: EditorSessionFixture }) });

const dictionary = new URLSearchParams(location.search).has("dictionary");
const body = "preview";
const surface = "procedural3d-preview";
const store = new UiDocumentStore(surface);
const paths = new Map();
let revision = 0;
function publish(out, inside) {
  const nodes = [];
  function flatten({ children, ...record }, parent) {
    const path = JSON.stringify([parent, record.key]);
    if (!paths.has(path)) paths.set(path, paths.size);
    const id = paths.get(path);
    nodes.push({ ...record, id, children: children.map((child) => flatten(child, id)) });
    return id;
  }
  const snapshot = (dictionary ? "dictionary." : "") + out + "." + inside;
  const root = flatten(fixture[snapshot], null);
  store.loadSnapshot({ surface, revision: ++revision, root, layoutEpoch: 0n, nodes });
  window.channelState = [out, inside];
}
const output = dictionary ? ["values", "source@out", "out", "5", "key", "3"] : ["values", "source@out", "out", "1", "3"];
const input = ["values", "sink@in", "in"];
const scheduler = createTreeWindowSchedulerV1({
  refresh(bodyKey) {
    const windows = scheduler.viewStateFields().treeWindows;
    const depth = (keys) => {
      let depth = 0;
      for (let i = 2; i <= keys.length; i++) {
        const path = keys.slice(0, i).join("␟");
        if (windows?.find((request) => request.bodyKey === bodyKey && request.nodeKey === path)?.open !== true) break;
        depth++;
      }
      return depth;
    };
    publish(depth(output), depth(input));
    queueMicrotask(() => scheduler.settled(bodyKey));
  }
});
window.channelIntents = [];
publish(0, 0);
createRoot(document.querySelector("#root")).render(
  <UiI18nProvider instance={createShellI18nInstance("de", "native")}>
    <div style={{ height: 560, width: 640 }}>
      <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        <TreeWindowContext.Provider value={createTreeWindowContextV1(body, scheduler)}>
          <InterpretedUiNode store={store} windowId={surface} onAction={() => {}} onIntent={(intent) => { window.channelIntents.push(uiIntentToActionDescriptor(intent)); }} />
        </TreeWindowContext.Provider>
      </ChromeAwareWindowScrollSurface>
    </div>
  </UiI18nProvider>,
);
