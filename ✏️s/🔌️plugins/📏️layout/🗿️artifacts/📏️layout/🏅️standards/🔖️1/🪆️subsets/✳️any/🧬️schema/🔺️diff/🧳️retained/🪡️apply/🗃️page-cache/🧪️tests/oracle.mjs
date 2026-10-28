import {createRequire} from 'node:module';
const require=createRequire(import.meta.url);
const {groupBy}=require('lodash');
/** 🔬 Independent test backend observes final cache membership through Lodash grouping. */
export function pageCacheReference(fixture){const grouped=groupBy(fixture.frames,'layer');return{layerIds:Array.from(fixture.layers),objectIds:fixture.layers.map(layer=>(grouped[layer]??[]).map(frame=>frame.id))};}
