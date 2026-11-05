import{test,expect}from"bun:test";import{readFileSync}from"node:fs";import{resolve}from"node:path";import{pathToFileURL}from"node:url";
import{effectiveActionArgs,unresolvedActionArgs,argControl}from"@semio-tech/framework";
import recipe from"./snapshot-boundary.json";
const engine="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
const staged=await import(pathToFileURL(resolve(engine,"🧱️elements/🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx")).href);
const labels=await import(pathToFileURL(resolve(engine,"🧱️elements/🛠️ShellHelpers/🏷️labels/🟦️.tsx")).href);
const descriptor=JSON.parse(readFileSync("🌎️hub/🧩️compositions/💠️lowpoly/🧩️component/🔣️.json","utf8")),app=descriptor.manifest.apps.find(row=>row.id==="s.lowpoly.lowpoly@1/*#editor");
test("public staged mutation JSON controls preserve canonical painted text without truncation",()=>{
 const pixels=Buffer.alloc(recipe.width*recipe.height*4);for(let i=0;i<pixels.length;i+=4)pixels.set(recipe.rgba,i);
 const json=JSON.stringify({schema:"lowpoly.document",objects:[{...recipe.object,transform:{position:[0,0,0],rotation:[0,0,0],scale:[1,1,1]},smoothShading:false,mesh:null,paintLayers:[{name:"Color",visible:true,opacity:1,blendMode:"normal",pixels:pixels.toString("base64")}],meshContent:"",meshState:null}]});expect(Buffer.byteLength(json)).toBe(349838);
 for(const id of recipe.commands){
  const action=app.actions.find(row=>row.id===id);expect(action.kind).toBe("mutation");expect(staged.actionRequiresStagedForm(action)).toBe(true);expect(unresolvedActionArgs(action.args,effectiveActionArgs(action.args,{}))).toEqual(["json"]);
  const effective=effectiveActionArgs(action.args,{json});expect(effective.json).toBe(json);expect(unresolvedActionArgs(action.args,effective)).toEqual([]);
  for(const locale of["en","de"]){
   const def=labels.resolveActionArgDef(action.args.find(arg=>arg.id==="json"),id,labels.EMPTY_APP_LABELS_OVERLAY,"native",locale,[]);
   expect(def.label).toBe(locale==="en"?"Snapshot JSON":"Snapshot-JSON");expect(argControl(def).kind).toBe("text");
   let changed:unknown;const control=staged.renderStagedArgControl(def,json,value=>changed=value);expect(control.props.value).toBe(json);expect(control.props.maxLength).toBeUndefined();
   control.props.onChange({target:{value:json}});expect(changed).toBe(json);
  }
 }
});
