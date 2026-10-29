/** 🪑 Decode real furnishing solids, quantity units, materials and spatial identity with pinned web-ifc. */
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';
import {join} from 'node:path';
import {setupNativeIfcEngine} from './native-ifc-setup-r18.mjs';
import {verifyProducts} from './native-products-r44.mjs';
export async function verify(ticket,samples){
 const geometry=await verifyProducts(ticket,samples),config=await setupNativeIfcEngine(join(ticket,'🗑️generated','joint-ifc-r30'));
 const library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();await api.Init(name=>join(config.wasmDirectory,name));const semantics=[];
 try{for(const sample of samples){const id=api.OpenModel(new TextEncoder().encode(sample.text));
 try{const number=key=>Number(sample.ids[key].slice(1)),get=key=>api.GetLine(id,number(key));
 const element=get('element');assert.equal(element.PredefinedType,undefined);
 const volume=get('quantity_volume');assert.equal(volume.type,library.IFCQUANTITYVOLUME);assert.equal(volume.Name.value,'NetVolume');assert.ok(Math.abs(volume.VolumeValue.value-sample.net_volume)<1e-8);assert.equal(volume.Unit.value,number('quantity_unit'));
 const unit=get('quantity_unit');assert.equal(unit.type,library.IFCSIUNIT);assert.equal(unit.UnitType.value,'VOLUMEUNIT');assert.equal(unit.Name.value,'CUBIC_METRE');assert.equal(unit.Prefix,null);
 const set=get('quantity_set');assert.equal(set.type,library.IFCELEMENTQUANTITY);assert.equal(set.Name.value,'FurnishingOwnedQuantities');assert.equal(JSON.stringify(set.Quantities.map(x=>x.value)),JSON.stringify([number('quantity_volume')]));
 const relation=get('quantity_relation');assert.equal(relation.type,library.IFCRELDEFINESBYPROPERTIES);assert.equal(relation.RelatingPropertyDefinition.value,number('quantity_set'));assert.equal(JSON.stringify(relation.RelatedObjects.map(x=>x.value)),JSON.stringify([number('element')]));
 const spatial=get('rel_storey');assert.equal(spatial.RelatingStructure.value,number('storey'));assert.ok(spatial.RelatedElements.some(x=>x.value===number('element')));assert.equal(get('storey').GlobalId.value,sample.storey_guid);
 const association=get('rel_material');assert.equal(association.RelatingMaterial.value,number('constituentset'));assert.ok(association.RelatedObjects.some(x=>x.value===number('element')));
 const materialSet=get('constituentset');const names=[...new Set(materialSet.MaterialConstituents.map(ref=>api.GetLine(id,api.GetLine(id,ref.value).Material.value).Name.value))].sort();assert.equal(JSON.stringify(names),JSON.stringify([...sample.material_names].sort()));
 semantics.push({case:sample.case,tag:sample.tag,language:sample.language,net_volume:volume.VolumeValue.value,unit:unit.Name.value,material_names:names,predefined_attribute:'absent',storey_guid:sample.storey_guid});
 }finally{api.CloseModel(id);}}}finally{api.Dispose();}return {geometry,semantics};
}
