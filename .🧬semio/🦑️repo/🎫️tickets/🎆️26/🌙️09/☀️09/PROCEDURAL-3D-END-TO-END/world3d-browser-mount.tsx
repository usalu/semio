import { createRoot } from "react-dom/client";
import { useState } from "react";
import { BoxGeometry } from "three";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { createShellScope, ShellScopeProvider } from "@semio-tech/ui-react";
import { World3dHost } from "probe:world-host";
import { WindowInstanceIdContext } from "probe:window-scope";
import fixture from "probe:hover-fixture";
import groupedHover from "probe:grouped-hover";
import componentFixture from "probe:component-fixture";
import largeFixture from "./large-hover.json";
const scope = createShellScope({ storage: createMemoryStoragePort(), initialLocale: "en" });
const box = new BoxGeometry(1.3, 1.2, 1.2);
const mesh = { positions:Array.from(box.attributes.position.array), normals:Array.from(box.attributes.normal.array), indices:Array.from(box.index!.array), edgePositions:[], faceIds:Array.from({length:box.index!.count/3},(_,id)=>id) };
const meshesJson = JSON.stringify([{ id:"surface", role:"solid", data:mesh }, { id:"edge", role:"solid", data:mesh }]);
const component = new URLSearchParams(location.search).has("component");
const coincident = new URLSearchParams(location.search).has("coincident");
const distinct = new URLSearchParams(location.search).has("distinct");
const large = new URLSearchParams(location.search).has("large");
const scoped = (target: { granularity: string; id: string }) => ({ ...target, id:"h:" + target.id });
const rows = component ? componentFixture.instances : large ? largeFixture.instances.map(row=>({...row,meshId:"surface"})) : coincident ? groupedHover.cases.slice(0, 3).map(({ instance }, index) => ({
  ...fixture.instances[0],
  ...instance,
  meshId:distinct && index === 1 ? "edge" : "surface",
  interactionId:"h:" + instance.interactionId,
  hoverTargets:instance.hoverTargets?.map(scoped),
})) : fixture.instances;
const ids = rows.map(row=>row.id);
const centers = component ? componentFixture.instances.map(row=>row.position[0]) : large ? rows.map(()=>0) : coincident ? [0,0,4] : [-3.15,-1.05,1.05,3.15];
const placements = component ? componentFixture.instances : rows.map((row,index)=>({ ...row, position:[centers[index],0,0], interactionId:"interactionId" in row ? row.interactionId : row.id.split("#")[0] }));
const instancesJson = JSON.stringify(placements);
const initial = { hoveredId:null, hoveredIds:[] as string[], hoverSourceWindowId:undefined as string | null | undefined, ids:[], selectionMode:"object", targets:{mesh:true,vertex:false,edge:false,face:false} };
const cameraJson = JSON.stringify(componentFixture.camera);
const onAction = async (event: unknown) => { window.__actions.push(event); };
window.__fixture = { ids, centers, meshIds:placements.map(row=>row.meshId), projected:fixture.selection, groupedHover:groupedHover.cases[0].expected.map(scoped), independentHover:groupedHover.cases[2].expected.map(scoped), component:componentFixture,largeTargets:largeFixture.instances[0]?.hoverTargets,largeInteractionIds:largeFixture.instances.map(row=>row.interactionId) };
window.__actions = [];
function Probe() {
  const [projection,setProjection]=useState({hoveredId:null,hoveredIds:[]} as Pick<typeof initial,"hoveredId"|"hoveredIds"|"hoverSourceWindowId">);
  const [activeObjectId, setActiveObjectId] = useState<string | undefined>(undefined);
  const [aliasOverride, setAliasOverride] = useState<typeof groupedHover.cases[0]["expected"]>();
  window.__project=(value)=>setProjection(value);
  window.__activeObject = setActiveObjectId;
  window.__setGroupedHover = setAliasOverride;
  const sceneInstances = aliasOverride ? JSON.stringify(placements.map((row, index) => index < 2 ? { ...row, hoverTargets: aliasOverride } : row)) : instancesJson;
  const node = { type:"componentScene", surfaceId:"procedural3d-browser-witness", controllerId:"probe3d", componentKind:"world-3d", world3d:{
    meshesJson, instancesJson:sceneInstances, cameraJson,
    selectionJson: JSON.stringify({...initial,...projection,...(component ? {activeObjectId,granularity:"face",selectionMode:"face",gumballActive:false,targets:{mesh:false,vertex:false,edge:false,face:true,exclusive:true}} : {})}), referencesJson:"[]", attractionsJson:"[]", vorticesJson:"[]",
    domainId:component ? "geometry" : "graph", domainGranularityId:component ? "face" : "handle", interactionJson:JSON.stringify({activeUtility:"select"})
  } };
  return <ShellScopeProvider scope={scope}><WindowInstanceIdContext.Provider value="browser-window:1"><World3dHost node={node as never} onAction={onAction}/></WindowInstanceIdContext.Provider></ShellScopeProvider>;
}
createRoot(document.querySelector("#mount")!).render(<Probe/>);
