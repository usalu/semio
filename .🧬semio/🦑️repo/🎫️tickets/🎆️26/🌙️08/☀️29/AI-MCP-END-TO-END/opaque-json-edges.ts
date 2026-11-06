import{readFileSync,writeFileSync,renameSync}from"node:fs";
const ticket=import.meta.dir;function edit(p:string,fn:(s:string)=>string){const before=readFileSync(p,"utf8"),after=fn(before);if(before===after)throw Error("Nochange "+p);const temp=ticket+"/🗑️generated/json-edge-"+Math.random();writeFileSync(temp,after);if(readFileSync(p,"utf8")!==before)throw Error("Concurrent edit "+p);renameSync(temp,p);}
edit("🧰️framework/🔨️modules/🎒️pack/🔤️json/🎛️controlled-and-borrowed-bridge/🦀️.rs",s=>{
 s=s.replace("pub position: usize }","pub position: usize, pub(super) source_index:usize, pub(super) next_cursor:usize }");
 s=s.replace(/JsonWriteFrame\s*\{\s*state:\s*0,\s*index:\s*0,\s*position:\s*0\s*\}/g,"JsonWriteFrame{state:0,index:0,position:0,source_index:0,next_cursor:0}");
 s=s.replace("pub trait JsonWriteSource{","#[derive(Clone,Copy,Debug,PartialEq,Eq)]\npub struct JsonWriteEdge{pub child:usize,pub cursor:usize}\n\npub trait JsonWriteSource{\n    fn traversal_edge_at_path(&self,_path:&[usize],index:usize,_cursor:usize)->Result<JsonWriteEdge,ValueError>{Ok(JsonWriteEdge{child:index,cursor:0})}");
 s=s.replaceAll("source.object_key_at_path(&self.path,frame.index)","source.object_key_at_path(&self.path,frame.source_index)").replaceAll("source.object_key_at_path(path.as_slice(),frame.index)","source.object_key_at_path(path.as_slice(),frame.source_index)");
 const array='else { if frame.index != 0 { writer.raw(",", control)?; } frames.last_mut().unwrap().index += 1;copied+=size_of::<usize>(); copied+=child_source_node(frames,path,frame.index)?; }';
 if(!s.includes(array))throw Error("Array frontier changed");s=s.replace(array,'else { let edge=source.traversal_edge_at_path(path.as_slice(),frame.index,frame.next_cursor)?;if frame.index != 0 { writer.raw(",", control)?; }let next=frames.last_mut().unwrap();next.index+=1;next.next_cursor=edge.cursor;copied+=2*size_of::<usize>(); copied+=child_source_node(frames,path,edge.child)?; }');
 const object='else { if frame.index != 0 { writer.raw(",", control)?; } writer.raw("\\\"", control)?; let next = frames.last_mut().unwrap(); next.state = 3; next.position = 0;copied+=size_of::<u8>()+size_of::<usize>(); }';
 if(!s.includes(object))throw Error("Object frontier changed");s=s.replace(object,'else { let edge=source.traversal_edge_at_path(path.as_slice(),frame.index,frame.next_cursor)?;if frame.index != 0 { writer.raw(",", control)?; } writer.raw("\\\"", control)?; let next = frames.last_mut().unwrap(); next.state = 3; next.position = 0;next.source_index=edge.child;next.next_cursor=edge.cursor;copied+=size_of::<u8>()+3*size_of::<usize>(); }');
 s=s.replace("child_source_node(frames,path,frame.index)?; }","child_source_node(frames,path,frame.source_index)?; }");return s;
});
edit("🧰️framework/🔨️modules/🎒️pack/🔤️json/🎛️controlled-and-borrowed-bridge/🫗️sink/🦀️.rs",s=>s.replace(/JsonWriteFrame\s*\{\s*state:\s*0,\s*index:\s*0,\s*position:\s*0\s*\}/g,"JsonWriteFrame{state:0,index:0,position:0,source_index:0,next_cursor:0}"));
edit("🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs",s=>s+"\npub use controlled_and_borrowed_bridge::JsonWriteEdge;\n");
console.log("Installed opaque traversal edges in shared JSON frontier");
