import {type FormDictionary} from "../../../../🧬️schema/🟦️.ts";
import {parseLayoutDiff,type LayoutDiff} from "../../../../🧬️schema/🔺️diff/🟦️.ts";
import{formDictionaryEntryNativeJson,formDictionaryEntryFromNativeJson}from"../../../../../../../../../../../📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧾️dictionary/🪪️native-json/🟦️.ts";
import {row,array,fields,paragraph,character,run,story,frame,page,child,link,image,rgba} from "../../📸️snapshot/🪪️native-json/🟦️.ts";

function delta(v:unknown,added:(v:unknown,out:boolean)=>unknown,out:boolean,patched?:(v:unknown,out:boolean)=>unknown):unknown{const r={...row(v)};r.added=array(r.added).map(v=>added(v,out));if(patched)r.patched=array(r.patched).map(v=>{const entry={...row(v)};entry.patch=patched(entry.patch,out);return entry;});return r;}
function indexed(v:unknown,key:string,convert:(v:unknown,out:boolean)=>unknown,out:boolean):unknown{const r={...row(v)};r.rows=array(r.rows).map(v=>{const entry={...row(v)};entry[key]=convert(entry[key],out);return entry;});return r;}
function override(v:unknown,out:boolean):unknown{const r={...row(v)};if(r.bounds!=null)r.bounds=fields(r.bounds,["x","y","w","h","rotation"],out);return r;}
function framePatch(v:unknown,out:boolean):unknown{const r=fields(v,["x","y","width","height","rotation","inset_x","inset_y","inset_width","inset_height"],out);for(const key of["fill","stroke"])if(r[key]!=null)r[key]=rgba(r[key],out);return r;}
function pagePatch(v:unknown,out:boolean):unknown{
 const r=fields(v,["width","height","margin_top","margin_right","margin_bottom","margin_left","columns_gutter"],out);
 if(r.frame_added!=null){const entry={...row(r.frame_added)};entry.frame=frame(entry.frame,out);r.frame_added=entry;}
 r.frames_patched=array(r.frames_patched).map(v=>{const entry={...row(v)};entry.patch=framePatch(entry.patch,out);return entry;});
 if(r.guides!=null)r.guides=indexed(r.guides,"guide",(v,out)=>fields(v,["x","y","w","h"],out),out);
 if(r.overrides!=null){const entries=row(delta(r.overrides,override,out));entries.patched=array(entries.patched).map(v=>{const entry={...row(v)};entry.item=override(entry.item,out);return entry;});r.overrides=entries;}
 return r;
}
function diff(v:unknown,out:boolean):unknown{
 const r={...row(v)};
 if(r.dataFields!=null){const change={...row(r.dataFields)};if(change.entries!=null){const entries={...row(change.entries)},convert=(v:unknown)=>out?formDictionaryEntryNativeJson(v as FormDictionary["entries"][number]):formDictionaryEntryFromNativeJson(v);entries.added=array(entries.added).map(convert);entries.patched=array(entries.patched).map(v=>{const entry={...row(v)};entry.item=convert(entry.item);return entry;});change.entries=entries;}r.dataFields=change;}
 if(r.grid!=null)r.grid=fields(r.grid,["baselineGrid","baselineOffset"],out);
 if(r.paragraphStyles!=null)r.paragraphStyles=delta(r.paragraphStyles,paragraph,out,paragraph);
 if(r.characterStyles!=null)r.characterStyles=delta(r.characterStyles,character,out,character);
 if(r.stories!=null)r.stories=delta(r.stories,story,out,(v,out)=>{const patch={...row(v)};if(patch.style_runs!=null)patch.style_runs=indexed(patch.style_runs,"run",run,out);return patch;});
 if(r.links!=null)r.links=delta(r.links,image,out);
 if(r.parentPages!=null)r.parentPages=delta(r.parentPages,(v,out)=>page(v,out,true),out,(v,out)=>fields(v,["width","height"],out));
 if(r.pages!=null)r.pages=delta(r.pages,(v,out)=>page(v,out,false),out,pagePatch);
 if(r.backgroundDrawing!=null)r.backgroundDrawing=child(r.backgroundDrawing,out);
 if(r.referencedModel!=null)r.referencedModel=link(r.referencedModel,out);
 return r;
}

/** 📥️ Converts full typed entity additions inside the existing native diff facet. */
export function layoutDiffFromNativeJson(value:unknown):LayoutDiff{return parseLayoutDiff(diff(value,false));}

/** 📤️ Emits native JSON diff fields without changing ordered entity additions. */
export function layoutDiffNativeJson(value:LayoutDiff):unknown{return diff(value,true);}
