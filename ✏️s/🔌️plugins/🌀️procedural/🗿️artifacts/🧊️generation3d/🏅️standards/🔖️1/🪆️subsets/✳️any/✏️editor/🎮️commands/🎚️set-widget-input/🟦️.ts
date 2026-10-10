import {base64StandardDecodeControlled} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🔤️base64/🟦️.ts";
import {parseChangeWidgetInput} from "../../../🚪️io/📝️text/🧬️mutations/🎛️change-widget-input/🦠️mutation/🟦️.ts";
import type {Widget} from "../../../🧬️schema/🟦️.ts";
import {parsePolygonMesh} from "../../../../../../../../../../🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🟦️.ts";

/** 🎚️ Portable typed input edits preserve the operator's declared schema. */
export function editInputValue(types: string[], current: unknown, text: string, component?: string): Record<string, unknown> {
  const existing = current && typeof current === "object" ? current as Record<string, unknown> : undefined;
  const schema = typeof existing?.$schema === "string" ? existing.$schema : types[0];
  if (!types.includes(schema) || !["number", "text", "boolean", "point", "vector"].includes(schema)) throw new Error("Connect a compatible output to this input");
  const numeric = () => {
    if (!text.trim() || !/^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(text.trim()) || !Number.isFinite(Number(text))) throw new Error("Input must be a finite number");
    return Number(text);
  };
  if (schema === "point" || schema === "vector") {
    if (!component || !["x", "y", "z"].includes(component)) throw new Error("Choose a coordinate to edit");
    return { $schema: schema, x: 0, y: 0, z: 0, ...existing, [component]: numeric() };
  }
  if (component) throw new Error("Scalar input has no coordinates");
  if (schema === "boolean" && text !== "true" && text !== "false") throw new Error("Boolean input must be true or false");
  return { $schema: schema, ...existing, value: schema === "number" ? numeric() : schema === "boolean" ? text === "true" : text };
}

export type CollectionInputEdit = {operation?:string;index?:number;destination?:number;component?:string;value:string};

/** 🥽️ Structured source edits validate through the existing owning indexed-polygon parser. */
export function editMeshSource(text:string,edit:{path:string[];value:string;operation?:string;destination?:number}):string|null {
  if(new TextEncoder().encode(text).length>16777216)throw Error("Mesh source exceeds 16 MiB");
  if (!edit.path.length || edit.path.length>16 || edit.path.some(part=>!part.length || [...part].length>128) || !["vertices","faces","attributes","materials","textures"].includes(edit.path[0])) throw Error("Choose a structured mesh field");
  const source=JSON.parse(text) as Record<string,unknown>;
  if(edit.operation!=="move"&&edit.destination!==undefined)throw Error("Mesh destinations belong to move actions");
  if(["materials","textures"].includes(edit.path[0])) {
    const kind=edit.path[0],name=edit.path[1],table=(source[kind]??={}) as Record<string,Record<string,unknown>>;
    if(!name)throw Error("Choose an asset");
    const operation=edit.operation??"set";
    if(edit.path.length===2&&operation==="add"&&kind==="materials"&&!Object.hasOwn(table,name)&&edit.value==="")Object.defineProperty(table,name,{value:{baseColor:[1,1,1,1],metallic:0,roughness:1,emissive:[0,0,0],normalScale:[1,1],occlusionStrength:1,alphaMode:"OPAQUE",alphaCutoff:0.5,doubleSided:false},writable:true,enumerable:true,configurable:true});
    else if(edit.path.length===2&&operation==="remove"&&edit.value===""&&Object.hasOwn(table,name)) {
      const used=kind==="materials"?Object.values(source.attributes as Record<string,Record<string,unknown>>??{}).some(attribute=>attribute.semantic==="material"&&(attribute.values as unknown[]).includes(name)):Object.values(source.materials as Record<string,Record<string,unknown>>??{}).some(material=>Object.entries(material).some(([role,value])=>role.endsWith("Texture")&&value===name));
      if(used)throw Error("Asset is in use");delete table[name];
    }else if(edit.path.length===3&&edit.path[2]==="name"&&operation==="set"&&Object.hasOwn(table,name)) {
      const next=edit.value.trim();if(!next||[...next].length>128||next!==name&&Object.hasOwn(table,next))throw Error("Choose a unique asset name");if(next===name)return null;
      Object.defineProperty(table,next,{value:table[name],writable:true,enumerable:true,configurable:true});delete table[name];
      if(kind==="materials"){for(const attribute of Object.values(source.attributes as Record<string,Record<string,unknown>>??{}))if(attribute.semantic==="material")attribute.values=(attribute.values as unknown[]).map(value=>value===name?next:value);}
      else for(const material of Object.values(source.materials as Record<string,Record<string,unknown>>??{}))for(const role of Object.keys(material))if(role.endsWith("Texture")&&material[role]===name)material[role]=next;
    }else if(kind==="textures")throw Error("Texture bytes are replaced by file import");
    else {
      const material=Object.hasOwn(table,name)?table[name]:undefined;if(!material||operation!=="set"||edit.path.length<3)throw Error("Choose a material field");
      const defaults:Record<string,unknown>={baseColor:[1,1,1,1],metallic:0,roughness:1,emissive:[0,0,0],normalScale:[1,1],occlusionStrength:1,alphaMode:"OPAQUE",alphaCutoff:0.5,doubleSided:false,textureCoordinates:{},textureSamplers:{}};
      const field=edit.path[2];
      if(edit.path.length===3&&field.endsWith("Texture")) {if(edit.value==="")delete material[field];else material[field]=edit.value;}
      else {
        if(!Object.hasOwn(material,field)){if(!Object.hasOwn(defaults,field))throw Error("Choose a canonical material field");material[field]=structuredClone(defaults[field]);}
        if(field==="textureCoordinates"&&edit.path.length===4)(material[field] as Record<string,unknown>)[edit.path[3]]??=0;
        if(field==="textureSamplers"&&edit.path.length===5){const samplers=material[field] as Record<string,Record<string,unknown>>;samplers[edit.path[3]]??={};samplers[edit.path[3]][edit.path[4]]??=edit.path[4].startsWith("wrap")?10497:9729;}
        let parent=material as Record<string,unknown>;for(const part of edit.path.slice(2,-1)){if(parent[part]===undefined)throw Error("Choose a material field");parent=parent[part] as Record<string,unknown>;}
        const key=edit.path.at(-1)!,current=parent[key];
        const next=typeof current==="number"?Number(edit.value):typeof current==="boolean"?edit.value==="true":edit.value;
        if(typeof current==="number"&&(!edit.value.trim()||!Number.isFinite(next as number))||typeof current==="boolean"&&!["true","false"].includes(edit.value))throw Error("Choose a primitive material value");
        if(current===next&&Object.hasOwn(JSON.parse(text)[kind]?.[name]??{},field))return null;parent[key]=next;
      }
    }
    const encoded=JSON.stringify(source);parsePolygonMesh(encoded);return encoded;
  }
  if(edit.path[0]==="attributes"&&edit.path.length===2&&edit.operation==="add") {
    const name=edit.path[1],attributes=(source.attributes??={}) as Record<string,unknown>;
    if(Object.hasOwn(attributes,name))throw Error("Attribute already exists");
    const [preset,chosen]=edit.value.split("/"),domain=preset==="material"?"face":chosen??(preset==="uv"?"corner":"vertex"),semantic=["uv","normal","color","material"].includes(preset)?preset:"custom";
    if(!["vertex","face","edge","corner"].includes(domain))throw Error("Choose an attribute domain");
    const sample=preset==="normal"?[0,0,1]:preset==="uv"?[0,0]:preset==="color"?[1,1,1,1]:preset==="number"?0:preset==="boolean"?false:preset==="vector"?[0,0,0]:preset==="text"?"":preset==="material"?chosen:undefined;
    if(sample===undefined)throw Error("Choose an attribute type");
    const count=["corner","edge"].includes(domain)?(source.faces as number[][]).reduce((total,face)=>total+face.length,0):domain==="face"?(source.faces as number[][]).length:(source.vertices as number[][]).length;
    Object.defineProperty(attributes,name,{value:{domain,semantic,interpolation:["text","boolean","material"].includes(preset)?"nearest":"linear",values:Array.from({length:count},()=>structuredClone(sample))},writable:true,enumerable:true,configurable:true});
    const encoded=JSON.stringify(source);parsePolygonMesh(encoded);return encoded;
  }
  if(edit.path[0]==="attributes"&&edit.path.length===2&&edit.operation==="remove") {
    const attributes=source.attributes as Record<string,unknown>;if(!attributes||!Object.hasOwn(attributes,edit.path[1])||edit.value!=="")throw Error("Choose an existing attribute");delete attributes[edit.path[1]];
    const encoded=JSON.stringify(source);parsePolygonMesh(encoded);return encoded;
  }
  if(edit.path[0]==="attributes"&&edit.path.length===3&&edit.path[2]==="name"&&(edit.operation??"set")==="set") {
    const attributes=source.attributes as Record<string,unknown>,name=edit.value.trim();
    if(!name||[...name].length>128||!attributes||!Object.hasOwn(attributes,edit.path[1])||name!==edit.path[1]&&Object.hasOwn(attributes,name))throw Error("Choose a unique attribute name");
    if(name===edit.path[1])return null;Object.defineProperty(attributes,name,{value:attributes[edit.path[1]],writable:true,enumerable:true,configurable:true});delete attributes[edit.path[1]];
    const encoded=JSON.stringify(source);parsePolygonMesh(encoded);return encoded;
  }
  if ((edit.operation??"set")!=="set") { editMeshArray(source,edit); const encoded=JSON.stringify(source);parsePolygonMesh(encoded);return encoded; }
  if(edit.destination!==undefined)throw Error("Choose a primitive mesh field to edit");
  let parent:unknown=source;
  for (const part of edit.path.slice(0,-1)) {
    if (!parent || typeof parent!=="object" || Array.isArray(parent) && !/^(0|[1-9]\d*)$/.test(part) || !Object.hasOwn(parent,part)) throw Error("Mesh field no longer exists");
    parent=(parent as Record<string,unknown>)[part];
  }
  const key=edit.path.at(-1)!;
  if (!parent || typeof parent!=="object" || Array.isArray(parent) && !/^(0|[1-9]\d*)$/.test(key) || !Object.hasOwn(parent,key)) throw Error("Mesh field no longer exists");
  const current=(parent as Record<string,unknown>)[key];
  let value:unknown;
  if (typeof current==="boolean") {if (!['true','false'].includes(edit.value)) throw Error("Choose a boolean value"); value=edit.value==="true";}
  else if (typeof current==="string") value=edit.value;
  else if (typeof current==="number") {
    if (!edit.value.trim() || !/^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(edit.value.trim()) || !Number.isFinite(Number(edit.value))) throw Error("Input must be a finite number");
    value=Number(edit.value);
    if ((edit.path[0]==="faces" || edit.path.includes("indices")) && (!Number.isInteger(value) || (value as number)<0 || (value as number)>4294967295)) throw Error("Choose a non-negative integer index");
  } else throw Error("Choose a primitive mesh field");
  if (value===current) return null;
  (parent as Record<string,unknown>)[key]=value;
  const encoded=JSON.stringify(source);
  parsePolygonMesh(encoded);
  return encoded;
}

/** 📐️ Ordered source edits preserve the authored domain samples and vertex references. */
function editMeshArray(source:Record<string,unknown>,edit:{path:string[];value:string;operation?:string;destination?:number}):void {
  if(edit.value!=="")throw Error("Mesh list actions have no primitive value");
  if(edit.operation!=="move"&&edit.destination!==undefined)throw Error("Mesh destinations belong to move actions");
  const add=edit.operation==="add",path=add?edit.path:edit.path.slice(0,-1),key=edit.path.at(-1)!;
  let value:unknown=source;for(const part of path){if(!value||typeof value!=="object"||!Object.hasOwn(value,part))throw Error("Mesh field no longer exists");value=(value as Record<string,unknown>)[part];}
  if(!Array.isArray(value)||!(path.length===1||path[0]==="faces"&&path.length===2||path[0]==="attributes"&&["values","indices"].includes(path.at(-1)!)))throw Error("Choose an editable mesh list");
  const vertices=source.vertices as number[][],faces=source.faces as number[][],oldFaces=faces.map(face=>[...face]);
  const order:(number|null)[]=value.map((_,index)=>index),index=add?value.length:Number(key);
  if(!add&&(!/^(0|[1-9]\d*)$/.test(key)||index>=value.length))throw Error("Choose a mesh list item");
  const defaultValue=():unknown=>{
    if(path.length===1&&path[0]==="vertices")return [0,0,0];
    if(path.length===1&&path[0]==="faces")return [vertices.length,vertices.length+1,vertices.length+2];
    if(path[0]==="faces"){const candidate=vertices.findIndex((_,index)=>!value.includes(index));if(candidate<0)throw Error("Every vertex already belongs to this face");return candidate;}
    if(path.at(-1)==="indices")return 0;
    return meshSampleDefault(value[0]);
  };
  if(add){const insertion=path[0]==="faces"&&path.length===2?value.length-1:value.length;value.splice(insertion,0,defaultValue());order.splice(insertion,0,null);}
  else if(edit.operation==="remove"){value.splice(index,1);order.splice(index,1);}
  else if(edit.operation==="move"&&Number.isSafeInteger(edit.destination)&&edit.destination!>=0&&edit.destination!<value.length){value.splice(edit.destination!,0,...value.splice(index,1));order.splice(edit.destination!,0,...order.splice(index,1));}
  else throw Error("Choose a mesh list operation");
  const orders:Record<string,(number|null)[]>={};
  if(add&&path.length===1&&path[0]==="faces") {orders.vertex=vertices.map((_,index)=>index);vertices.push([0,0,0],[1,0,0],[0,1,0]);orders.vertex.push(null,null,null);}
  if(path.length===1&&path[0]==="vertices"){
    const reverse=new Map(order.flatMap((old,index)=>old===null?[]:[[old,index] as const]));
    source.faces=faces.map(face=>face.map(vertex=>{const next=reverse.get(vertex);if(next===undefined)throw Error("A referenced vertex cannot be removed");return next;}));orders.vertex=order;
  }else if(path[0]==="faces"){
    const offsets=oldFaces.map((_,index)=>oldFaces.slice(0,index).reduce((total,face)=>total+face.length,0));
    if(path.length===1){orders.face=order;orders.corner=order.flatMap(old=>old===null?[null,null,null]:oldFaces[old].map((_,index)=>offsets[old]+index));}
    else{const face=Number(path[1]);orders.corner=oldFaces.flatMap((items,faceIndex)=>faceIndex===face?order.map(old=>old===null?null:offsets[faceIndex]+old):items.map((_,index)=>offsets[faceIndex]+index));}
    orders.edge=orders.corner;
  }
  for(const attribute of Object.values(source.attributes as Record<string,Record<string,unknown>>??{})){
    const order=orders[String(attribute.domain)];if(!order)continue;
    const field=Array.isArray(attribute.indices)?"indices":"values",samples=attribute[field] as unknown[];
    attribute[field]=order.map(old=>old===null?(field==="indices"?0:meshSampleDefault(samples[0],String(attribute.semantic))):samples[old]);
  }
}

/** 🧫️ New domain samples retain their owned primitive or tuple shape. */
function meshSampleDefault(value:unknown,semantic="custom"):unknown {
  if(semantic==="normal")return [0,0,1];
  if(Array.isArray(value))return value.map(item=>meshSampleDefault(item));
  if(typeof value==="number")return 0;
  if(typeof value==="boolean")return false;
  if(typeof value==="string")return semantic==="material"?value:"";
  throw Error("Choose an existing primitive attribute sample");
}

/** 📃️ One absolute homogeneous collection value validated against the declared port multiplicity. */
export function editCollectionValue(types:string[],current:unknown,edit:CollectionInputEdit,cardinality:string): {type:string;value:unknown[]} {
  if (!["*","+"].includes(cardinality) && (!/^\d+$/.test(cardinality) || cardinality === "1")) throw new Error("Select a collection input");
  const list = current as Record<string,unknown> | null;
  if (list && (list.$schema !== "list" || Object.keys(list).some(key => key !== "$schema" && !/^(0|[1-9]\d*)$/.test(key)))) throw new Error("Input must be a contiguous homogeneous list");
  const first = list?.["0"] as Record<string,unknown> | undefined;
  const schema = typeof first?.$schema === "string" ? first.$schema : types[0];
  if (!types.includes(schema) || !["number","text","boolean","point","vector"].includes(schema)) throw new Error("Unsupported collection type");
  const values = Array.from({length:list ? Object.keys(list).length-1 : 0},(_,index) => {
    const item = list?.[String(index)] as Record<string,unknown> | undefined;
    if (!item || item.$schema !== schema) throw new Error("Input must be a contiguous homogeneous list");
    return {...item};
  });
  const index = edit.index;
  if (!Number.isSafeInteger(index) || index! < 0 || index! >= 1024) throw new Error("Choose a valid list item");
  const operation = edit.operation ?? "set";
  if (operation !== "move" && edit.destination !== undefined || operation !== "set" && edit.component !== undefined) throw new Error("Invalid list operation arguments");
  if (operation === "set" && index! < values.length) values[index!] = editInputValue(types,values[index!],edit.value,edit.component);
  else if (operation === "add" && index! <= values.length) {
    const item = schema === "point" || schema === "vector" ? {$schema:schema,x:0,y:0,z:0} : {$schema:schema,value:schema === "number" ? 0 : schema === "boolean" ? false : ""};
    values.splice(index!,0,edit.value === "" ? item : editInputValue(types,item,edit.value));
  } else if (operation === "remove" && index! < values.length && edit.value === "") values.splice(index!,1);
  else if (operation === "move" && index! < values.length && edit.value === "" && Number.isSafeInteger(edit.destination) && edit.destination! >= 0 && edit.destination! < values.length) values.splice(edit.destination!,0,...values.splice(index!,1));
  else throw new Error("Invalid list operation or item index");
  if (values.length > 1024 || cardinality === "+" && !values.length || /^\d+$/.test(cardinality) && values.length !== Number(cardinality)) throw new Error("List length does not match port cardinality");
  const value = values.map(item => schema === "point" || schema === "vector" ? [item.x,item.y,item.z] : item.value);
  const payload = parseChangeWidgetInput({id:"input",channel:"items",type:schema+"List",value});
  return {type:payload.type,value:payload.value as unknown[]};
}

/** 🪪️ Explicit variable/export facets project one existing update-widget leaf. */
export function editWidgetFacet(widget: Widget, edit: {facet:string;channel:string;value:string;component?:string;operation?:string;index?:number;destination?:number}, schemas: readonly string[], formats: readonly string[], connectedTypes: readonly (readonly string[])[]): Widget {
  if (edit.component !== undefined || edit.operation !== undefined || edit.index !== undefined || edit.destination !== undefined) throw new Error("Widget metadata has no list operation or coordinate");
  const value = edit.value.trim();
  if (!value || [...value].length > 256) throw new Error("Widget metadata requires 1 to 256 characters");
  if (widget.kind === "variable" && edit.facet === "variableName" && edit.channel === "name") return {...widget,name:value};
  if (widget.kind === "variable" && edit.facet === "variableSchema" && edit.channel === "schema" && schemas.includes(value) && connectedTypes.every(types=>!types.length || types.includes(value))) return {...widget,schema:value};
  if (widget.kind === "outputExport" && edit.facet === "exportFormat" && edit.channel === "format" && formats.includes(value)) return {...widget,format:value};
  throw new Error("Widget facet does not match its target, field or owning roster");
}

/** 🖼️ A picked raster replaces one canonical texture through the owning mesh source value. */
export function editMeshTexture(text:string,name:string,payload:string):string {
  if(!name||[...name].length>128||new TextEncoder().encode(text).length>16777216||payload.length>22369664)throw Error("Texture import exceeds its bounds");
  const mime=payload.startsWith("data:image/png;base64,")?"image/png":payload.startsWith("data:image/jpeg;base64,")?"image/jpeg":undefined;
  if(!mime)throw Error("Choose a PNG or JPEG texture");
  const prefix=`data:${mime};base64,`;
  const bytes=base64StandardDecodeControlled(payload.slice(prefix.length),{maximumOutputBytes:16777216,progress:()=>true});
  const signature=mime==="image/png"?[137,80,78,71,13,10,26,10].every((byte,index)=>bytes[index]===byte):bytes.length>=5&&bytes[0]===255&&bytes[1]===216&&bytes[2]===255&&bytes[bytes.length-2]===255&&bytes[bytes.length-1]===217;
  if(bytes.length>16777216||!signature)throw Error("Choose a PNG or JPEG texture");
  parsePolygonMesh(text);
  const source=JSON.parse(text) as Record<string,unknown>,textures=(source.textures??={}) as Record<string,unknown>;
  Object.defineProperty(textures,name,{value:{mime,bytes:[...bytes]},writable:true,enumerable:true,configurable:true});
  const encoded=JSON.stringify(source);parsePolygonMesh(encoded);return encoded;
}
