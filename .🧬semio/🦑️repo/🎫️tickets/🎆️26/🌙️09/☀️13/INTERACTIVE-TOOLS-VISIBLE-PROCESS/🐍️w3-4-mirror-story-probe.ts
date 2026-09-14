import { REMODEL_DEFAULT_CONFIG, REMODEL_EMPTY_SCENE, REMODEL_POPULATED_SCENE } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/📖️stories/🧭️coordination/🧫️fixtures/🧫️model/🟦️.ts";
import { remodelPanelDocument, remodelReportTableJson, remodelViewerWorldScene } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/📖️stories/🧭️coordination/🧫️fixtures/🧫️scene/🟦️.ts";
const texts = (node: any): string[] => [...(node.component.type === "text" ? [node.component.value] : []), ...node.children.flatMap(texts)];
console.log(JSON.stringify(texts(remodelPanelDocument("pipeline", REMODEL_POPULATED_SCENE, REMODEL_DEFAULT_CONFIG))));
console.log(JSON.stringify(texts(remodelPanelDocument("pipeline", REMODEL_EMPTY_SCENE, { ...REMODEL_DEFAULT_CONFIG, locale: "de-DE" }))));
console.log(JSON.stringify(remodelReportTableJson(REMODEL_POPULATED_SCENE, "qcStages")));
console.log(JSON.stringify(remodelReportTableJson(REMODEL_EMPTY_SCENE, "qcStages")));
console.log(JSON.stringify(remodelViewerWorldScene(REMODEL_POPULATED_SCENE)).includes("remodeling-camera-poses"));
