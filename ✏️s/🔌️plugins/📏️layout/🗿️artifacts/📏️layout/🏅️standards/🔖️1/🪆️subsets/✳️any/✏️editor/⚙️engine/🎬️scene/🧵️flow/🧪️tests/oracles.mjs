import {createRequire} from 'node:module';
const require=createRequire(import.meta.url),{Box2,Vector2}=require('three');
const validate=new(require('ajv'))({strict:false}).compile(require('../🔣️.json'));
/** 📐️ Independent rectangle intersection and vector geometry observe each admitted column. */
export function flowGeometry(frame,column){
 const bounds=frame.bounds,inset=frame.inset,content=new Box2(new Vector2(inset.x,inset.y),new Vector2(inset.x+inset.width,inset.y+inset.height));
 content.intersect(new Box2(new Vector2(0,0),new Vector2(bounds.width,bounds.height)));
 const size=content.getSize(new Vector2()),width=size.x/frame.columns-frame.gutter*(1-1/frame.columns);
 return[bounds.x+content.min.x+column*(width+frame.gutter),bounds.y+content.min.y,width,size.y];
}
/** 🔬️ Independent JSON Schema admission checks the language-neutral frame contract. */
export function validateFlowFrame(frame){return validate(frame);}
