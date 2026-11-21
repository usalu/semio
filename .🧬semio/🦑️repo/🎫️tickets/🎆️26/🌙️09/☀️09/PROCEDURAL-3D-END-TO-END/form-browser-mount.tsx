import { createRoot } from "react-dom/client";
import { InterpretedUiNode } from "probe:interpreter";
import { UiDocumentStore } from "probe:document-store";
import { ChromeAwareWindowScrollSurface } from "probe:window-chrome";
import { uiIntentToActionDescriptor } from "probe:action-bridge";
import { UiI18nProvider, createShellI18nInstance } from "@semio-tech/ui-react";
import fixture from "probe:fixture";
import texts from "probe:long-text-fixture";
import { retainedUiTextInput } from "@semio-tech/ui-contract/component";
import imageFixtures from "probe:image-fixture";
import "probe:styles";

const locale = fixture.locales.find((entry) => entry.id === new URLSearchParams(location.search).get("locale")) ?? fixture.locales[0];
const store = new UiDocumentStore(fixture.surface);
const retainedText = new URLSearchParams(location.search).has("retained");
const compose = ({prefix,unit,repeat,suffix}) => prefix + unit.repeat(repeat) + suffix;
const textKey = (text) => {
  const bytes = new TextEncoder().encode(text);
  let hash = 0xcbf29ce484222325n;
  for (const byte of bytes) hash = BigInt.asUintN(64,(hash ^ BigInt(byte))*0x100000001b3n);
  return "framework.input.value." + bytes.length + "." + hash.toString(16).padStart(16,"0");
};
const stack = { kind: "stack", axis: "vertical", gap: "none", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false };
const leaf = { kind: "leaf", width: "hug", height: "hug" };
const node = (id, key, component, layout, children = [], label = undefined, bindings = []) => ({
  id, key, component, layout, children, accessibility: { label }, style: {}, bindings,
});
const fields = retainedText ? [...fixture.fields, {kind:"longText",id:"notes",label:"Long notes",value:compose(texts.cases[3])}] : fixture.fields;
const publish = (revision, overrides = {}) => {
const fieldNodes = fields.flatMap((field, index) => {
  const id = 3 + index * 2;
  const key = fixture.panelKey + "." + field.id;
  const controlKey = field.kind === "number" ? key + ".axis.input" : key + ".control";
  const isText = field.kind === "text" || field.kind === "longText";
  const saved = retainedText && isText ? overrides[field.id] ?? sessionStorage.getItem(field.id) ?? (field.kind === "text" ? compose(texts.cases[2]) : field.value) : field.value;
  const chunks = retainedText && isText ? retainedUiTextInput(saved) : saved;
  const pages = typeof chunks === "object" ? chunks.value.flat() : [];
  const carrierId = 2000 + index * 40;
  const keyOfValue = pages.length ? textKey(saved) : null;
  const component = field.kind === "slider" ? { type: "slider", value: field.value, min: field.min, max: field.max, step: field.step }
    : field.kind === "boolean" ? { type: "toggle", on: field.value, text: field.label, icon: "toggle-left" }
    : { type: "input", kind: field.kind, value: keyOfValue ?? String(saved), commit: keyOfValue ? "blurRetainedBody" : retainedText && isText ? "blurRetained" : "blur" };
  return [
    node(id, key, { type: "container", role: "field", label: field.label }, stack, [id + 1]),
    node(id + 1, controlKey, component, leaf, keyOfValue ? [carrierId] : [], field.label, [{
      trigger: component.type === "input" ? "commit" : "change",
      action: { scope: "s.procedural.generation3d", name: "updateGenerationValues", version: 1 },
      args: { generationId: fixture.generation, questionId: field.id },
    }]),
    ...(keyOfValue ? [
      node(carrierId,keyOfValue,{type:"container",role:"plain"},stack,[carrierId + 1]),
      node(carrierId+1,"c0",{type:"text",value:pages[0],dataAttributes:Object.fromEntries(pages.slice(1).map((text,i)=>[String(i+1).padStart(2,"0"),text]))},leaf),
    ] : []),
  ];
});
const imageCase = imageFixtures[locale.id];
const authoredSource = imageCase.steps[0].blocks[0].src;
const bodyKey = "framework.ui.image.source.fixture";
const retained = imageCase.sourceBytes > 512;
const chunks = retained ? [authoredSource.slice(0, 512), authoredSource.slice(512, 1024), authoredSource.slice(1024)] : [];
const imageNodes = [
  node(1000, "authored-image", { type: "image", src: retained ? bodyKey : authoredSource, alt: imageCase.alt, ...(retained ? { sourceBodyBytes: imageCase.sourceBytes } : {}) }, leaf, retained ? [1001] : [], imageCase.alt),
  ...(retained ? [
    node(1001, bodyKey, { type: "container", role: "plain" }, stack, chunks.map((_, index) => 1002 + index)),
    ...chunks.map((value, index) => node(1002 + index, "image-chunk-" + index, { type: "text", value }, leaf)),
  ] : []),
];
store.loadSnapshot({ surface: fixture.surface, revision, root: 1, layoutEpoch: 0n, nodes: [
  node(1, fixture.panelKey, { type: "container", role: "form", label: locale.label }, fixture.scroll, [2]),
  node(2, fixture.fieldsKey, { type: "container", role: "plain" }, stack, [...fields.map((_, i) => 3 + 2 * i), 1000]),
  ...fieldNodes, ...imageNodes,
] });
};
let revision = 1;
publish(revision);
window.formExternal = (id,text) => publish(++revision,{[id]:text});
window.formIntents = [];
const i18n = createShellI18nInstance(locale.id, "native");
createRoot(document.querySelector("#root")).render(
  <UiI18nProvider instance={i18n}>
    <div style={{ height: fixture.viewport.height, width: fixture.viewport.width }}>
      <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        <InterpretedUiNode store={store} windowId={fixture.surface} onAction={() => {}} onIntent={(intent, windowId) => { const action = uiIntentToActionDescriptor(intent); window.formIntents.push({ intent, windowId, action }); if (retainedText) { const {questionId,value} = action.args; if (typeof value === "string" || Array.isArray(value)) { sessionStorage.setItem(questionId, typeof value === "string" ? value : value.flat().join("")); queueMicrotask(()=>publish(++revision)); } } }} />
      </ChromeAwareWindowScrollSurface>
    </div>
  </UiI18nProvider>,
);
