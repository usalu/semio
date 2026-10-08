import assert from "node:assert/strict";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020.js";
import fixture from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import mutationSchema from "../../../../../🧬️schema/🧬️mutations/🎛️change-widget-input/🧬️schema/🔣️.json";
import { editCollectionValue, editInputValue, editWidgetFacet, editMeshSource, editMeshTexture } from "../../🟦️.ts";
import meshSchema from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧬️schema/🥽️polygon/🔣️.json";
import {PNG} from "pngjs";
import {BoxGeometry,EdgesGeometry,Vector3} from "three";
import {JSDOM} from "jsdom";
import {createRequire} from "node:module";
import {getByRole} from "@testing-library/dom";
import inspectorFixture from "../../../../📌️panels/🔍️inspection/🧫️fixtures/🔣️.json";
import type { Widget } from "../../../../../🧬️schema/🟦️.ts";
import {diff as widgetDiff} from "../../../../../🧬️schema/🧬️mutations/🩹update-widget/🔺️diff/🟦️.ts";

const {computeAccessibleName}=createRequire(import.meta.url)("dom-accessibility-api") as {computeAccessibleName(element:Element):string};

/** ⚖️ Scalar and ordered collection edits answer shared vectors and independent Ajv/JSON oracles. */
export function generation3dWidgetInputSelfTests(): number {
  const validate = new Ajv({strict:false}).compile(schema);
  const validateMutation = new Ajv({strict:false}).compile(mutationSchema);
  const validateMesh = new Ajv2020({strict:false}).compile(meshSchema);
  let checks = 0;
  for (const entry of fixture.cases) {
    const component = "component" in entry ? entry.component : undefined;
    const operation = () => editInputValue(entry.types, entry.current, entry.value, component);
    if ("error" in entry) assert.throws(operation, entry.id);
    else {
      assert.deepEqual(operation(), entry.expected, entry.id);
      if (entry.types[0] === "number" || entry.types[0] === "boolean") { assert.equal(operation().value, JSON.parse(entry.value), entry.id); checks++; }
    }
    assert.equal(validate({widgetId:"shape",channel:"parameter",value:entry.value,...(component ? {component} : {})}), entry.id !== "invalid-component", entry.id);
    checks += 2;
  }
  for (const key of ["extra","gesture"]) { assert.equal(validate({widgetId:"shape",channel:"parameter",value:"1",[key]:true}),false); checks++; }
  for (const entry of fixture.collections) {
    const operation = () => editCollectionValue(entry.types, entry.current, entry.command, entry.cardinality);
    if ("error" in entry) assert.throws(operation, entry.id);
    else {
      assert.deepEqual(operation(), entry.expected, entry.id);
      assert.equal(validateMutation({mutation:"changeWidgetInput",id:"shape",channel:"items",...operation()}),true,entry.id);
      assert.deepEqual(JSON.parse(JSON.stringify(operation())),entry.expected,entry.id);
      checks += 2;
    }
    assert.equal(validate({widgetId:"shape",channel:"items",...entry.command}),true,entry.id);
    checks += 2;
  }
  for (const entry of fixture.facets) {
    const edit = () => editWidgetFacet(entry.widget as Widget,entry.command,["number","text","boolean","point","vector"],["stl","obj","ply","gltf","las","dwg","txt"],entry.connectedTypes ?? []);
    if ("error" in entry) assert.throws(edit,entry.id);
    else {
      assert.deepEqual(edit(),entry.expected,entry.id); assert.deepEqual(JSON.parse(JSON.stringify(edit())),entry.expected,entry.id); checks++;
      if (entry.expectedSynapses !== undefined && entry.synapses !== undefined) {
        const delta = widgetDiff({widget:edit()},entry.widget as Widget,entry.synapses);
        assert.deepEqual(delta.synapses.set.map(([,wire])=>wire),entry.expectedSynapses,entry.id);
        const restored = widgetDiff({widget:entry.widget as Widget},edit(),entry.expectedSynapses);
        assert.deepEqual(restored.synapses.set.map(([,wire])=>wire),entry.synapses,entry.id);
        checks += 2;
      }
    }
    assert.equal(validate({widgetId:entry.widget.id,...entry.command}),!("schemaValid" in entry && entry.schemaValid === false),entry.id);
    assert.equal(validate({widgetId:entry.widget.id,...entry.command,index:0}),false,entry.id);
    checks += 3;
  }
  for (const entry of fixture.runtimeCollections) {
    const value = editCollectionValue(["point","vector"],fixture.runtimeInput,entry.command,"*");
    assert.deepEqual(value,{type:"pointList",value:entry.expected},entry.id);
    assert(validateMutation({mutation:"changeWidgetInput",id:"polyline",channel:"points",...value}),JSON.stringify(validateMutation.errors));
    assert(validate({widgetId:"polyline",channel:"points",...entry.command}),JSON.stringify(validate.errors));
    checks += 3;
  }
  for(const group of [fixture.meshSource,fixture.meshAssets]) for(const entry of group.cases) {
    const original=JSON.stringify(group.mesh);
    const operation=()=>editMeshSource(original,entry.command);
    if("error" in entry) assert.throws(operation,entry.id);
    else {
      const expected="expectedMesh" in entry ? entry.expectedMesh : JSON.parse(original);
      if ("expected" in entry && entry.expected !== undefined) { let parent=expected;for(const part of entry.expected.path.slice(0,-1))parent=parent[part];parent[entry.expected.path.at(-1)!]=entry.expected.value; }
      const output=JSON.parse(operation()!);
      assert.deepEqual(output,expected,entry.id);
      assert(validateMesh(output),JSON.stringify(validateMesh.errors));
      checks+=2;
    }
    assert.equal(validate({widgetId:"construct",channel:"data",facet:"meshSource",...entry.command}),!("schemaValid" in entry && entry.schemaValid===false),entry.id+": "+JSON.stringify(validate.errors));
    checks+=2;
  }
  const imported=JSON.parse(editMeshTexture(JSON.stringify(fixture.meshAssets.mesh),fixture.textureImport.target.textureId,fixture.textureImport.payload));
  assert.deepEqual(imported.textures.image,{mime:fixture.textureImport.mime,bytes:fixture.textureImport.bytes});
  assert.deepEqual([...PNG.sync.read(Buffer.from(imported.textures.image.bytes)).data],fixture.textureImport.pixel);
  assert.deepEqual(imported.materials,fixture.meshAssets.mesh.materials);
  assert.throws(()=>editMeshTexture(JSON.stringify(fixture.meshAssets.mesh),"image","data:image/png;base64,YQ=="));
  const jpeg=JSON.parse(editMeshTexture(JSON.stringify(fixture.meshAssets.mesh),fixture.textureJpegImport.target.textureId,fixture.textureJpegImport.payload));
  assert.deepEqual(jpeg.textures.image,{mime:fixture.textureJpegImport.mime,bytes:fixture.textureJpegImport.bytes});
  assert.deepEqual(jpeg.materials,fixture.meshAssets.mesh.materials);
  assert.throws(()=>editMeshTexture(JSON.stringify(fixture.meshAssets.mesh),"image","data:image/jpeg;base64,YQ=="));
  checks+=7;
  const dimensions=fixture.measurementPublication.params;
  const declaredBox=inspectorFixture.selectedBrepControls.numericOracle;
  assert.deepEqual(dimensions,{width:declaredBox.width,depth:declaredBox.depth,height:declaredBox.height});
  checks++;
  const box=new BoxGeometry(dimensions.width,dimensions.depth,dimensions.height);
  const positions=box.attributes.position,index=box.index!,edges=new EdgesGeometry(box),edgePositions=edges.attributes.position;
  let area=0,volume=0,length=0;
  for(let i=0;i<index.count;i+=3) {
    const a=new Vector3().fromBufferAttribute(positions,index.getX(i)),b=new Vector3().fromBufferAttribute(positions,index.getX(i+1)),c=new Vector3().fromBufferAttribute(positions,index.getX(i+2));
    area+=b.clone().sub(a).cross(c.clone().sub(a)).length()/2;
    volume+=a.dot(b.clone().cross(c))/6;
  }
  for(let i=0;i<edgePositions.count;i+=2) length+=new Vector3().fromBufferAttribute(edgePositions,i).distanceTo(new Vector3().fromBufferAttribute(edgePositions,i+1));
  const measurements={length,area,volume:Math.abs(volume)};
  for(const row of fixture.measurementPublication.cases) { assert(Math.abs(measurements[row.output as keyof typeof measurements]-row.value)<1e-10,row.kind); checks++; }
  const live=fixture.liveAnalyticPublication;
  box.computeBoundingBox();
  assert.deepEqual(box.boundingBox!.getSize(new Vector3()).toArray(),live.sourceExtent);
  assert.equal(live.openedActor,"local");
  checks+=2;
  assert.equal(live.feature,`${live.source}__${live.operation}`);
  assert.equal(live.selector,`${live.source}__selected`);
  assert.equal(live.componentGranularity,"face");
  assert(live.group>=0&&Number.isInteger(live.group));
  assert(live.amount>0&&live.amount<Math.min(dimensions.width,dimensions.depth,dimensions.height)/2);
  assert.equal(measurements[live.consumer as keyof typeof measurements],fixture.measurementPublication.cases.find(row=>row.output===live.consumer)!.value);
  assert.deepEqual(live.history,["undo","redo"]);
  checks+=7;
  assert(Math.abs(measurements.area-inspectorFixture.selectedBrepControls.numericOracle.area)<1e-10);
  for(const entry of inspectorFixture.selectedBrepControls.cases) for(const control of entry.controls) {
    const expected={$schema:"number",value:control.committedValue};
    assert.deepEqual(editInputValue(["number"],{$schema:"number",value:control.value},String(control.committedValue)),expected);
    assert(validateMutation({mutation:"changeWidgetInput",id:"selected",channel:control.channel,type:"number",value:control.committedValue}),JSON.stringify(validateMutation.errors));
    for(const locale of ["en","de"] as const) {
      const dom=new JSDOM("<!doctype html><html><body></body></html>");
      const input=dom.window.document.createElement("input"); input.type="number"; input.value=String(control.value); input.setAttribute("aria-label",control[locale]); dom.window.document.body.append(input);
      assert.equal(computeAccessibleName(getByRole(dom.window.document.body,"spinbutton",{name:control[locale]})),control[locale]);
      dom.window.close(); checks++;
    }
    checks+=2;
  }
  checks++;
  for(const entry of inspectorFixture.selectedBrepControls.cases) for(const input of entry.readOnlyInputs ?? []) {
    const parsed=input.channel==="sourceHandle"?[input.value]:JSON.parse(input.value) as string[];
    assert.deepEqual(parsed,input.labels);
    assert.deepEqual(parsed.map(value=>BigInt(value).toString()),input.labels);
    assert(parsed.every(value=>BigInt(value)>0n && BigInt(value)<=18446744073709551615n));
    checks+=3;
  }
  const window=inspectorFixture.selectedBrepControls.selectionWindow;
  const labels=Array.from({length:window.count},(_,index)=>(BigInt(window.start)+BigInt(index)).toString());
  assert.deepEqual(JSON.parse(JSON.stringify(labels)).slice(window.offset,window.offset+window.rows),window.expected);
  checks++;
  box.dispose();edges.dispose();
  return checks;
}
