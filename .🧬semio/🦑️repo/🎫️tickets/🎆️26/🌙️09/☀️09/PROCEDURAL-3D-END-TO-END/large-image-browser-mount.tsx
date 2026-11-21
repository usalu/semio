import { createRoot } from "react-dom/client";
import { InterpretedUiNode } from "probe:interpreter";
import { UiDocumentStore } from "probe:document-store";
import { TreeWindowContext } from "probe:tree-context";
import { createTreeWindowSchedulerV1 } from "probe:tree-scheduler";
import { UiI18nProvider, createShellI18nInstance } from "@semio-tech/ui-react";

const bodyKey = "generate.form.body";
const windowId = "generate.form:1";
const imageKey = "question.image";
const store = new UiDocumentStore(windowId);
const revisionEvents = [];
let revision = 0;
let visible = true;
const descriptors = await fetch("/fixture/pages").then(response => response.json());
const requestKey = descriptors[0].slice(0, descriptors[0].lastIndexOf("."));
const stack = { kind: "stack", axis: "vertical", gap: "none", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false };
const leaf = { kind: "leaf", width: "hug", height: "hug" };
const node = (id, key, component, layout, children = []) => ({ id, key, component, layout, children, accessibility: {}, style: {}, bindings: [] });
const publish = async (index: number) => {
  let nodes = [node(1, bodyKey, { type: "container", role: "plain" }, stack, visible ? [2] : [])];
  if (visible) {
    const data = await fetch("/fixture/page/" + index).then(response => response.text());
    if (!visible) return;
    const desc = descriptors[index - 1];
    const chunks = data.match(/.{1,512}/g) ?? [];
    const groups = [];
    for (let start = 0; start < chunks.length; start += 17) groups.push(chunks.slice(start, start + 17));
    nodes.push(node(2, imageKey, { type: "image", src: desc, alt: "Authored procedural texture", sourceBodyBytes: data.length }, leaf, [3]));
    nodes.push(node(3, desc, { type: "container", role: "plain" }, stack, groups.map((_, i) => 4 + i)));
    for (const [i, chunks] of groups.entries()) nodes.push(node(4 + i, "group-" + i, { type: "text", value: chunks[0], dataAttributes: Object.fromEntries(chunks.slice(1).map((chunk, j)=>[String(j + 1).padStart(2,"0"),chunk])) }, leaf));
  }
  store.loadSnapshot({ surface: windowId, revision: ++revision, root: 1, layoutEpoch: 0n, nodes });
};
const scheduler = createTreeWindowSchedulerV1({ refresh(key) {
  const requested = scheduler.viewStateFields().treeWindows?.find(request => request.bodyKey === key && request.nodeKey === requestKey);
  const index = requested?.offset ?? 0;
  revisionEvents.push(index);
  setTimeout(() => { void publish(index || 1).finally(() => scheduler.settled(key)); }, 20);
}});
window.imageResourceProof = {
  revisionEvents,
  remove: () => { visible = false; void publish(1); },
  restore: () => { visible = true; void publish(1); },
};
await publish(3);
createRoot(document.querySelector("#root")).render(
  <UiI18nProvider instance={createShellI18nInstance("en", "native")}>
    <TreeWindowContext.Provider value={{ bodyKey, openStates: {}, setOpen: () => {}, reportWindows: () => {}, setFormPage: (key, page) => scheduler.setFormPage(bodyKey, key, page) }}>
      <InterpretedUiNode store={store} windowId={windowId} onAction={() => {}} onIntent={() => {}} />
    </TreeWindowContext.Provider>
  </UiI18nProvider>,
);
