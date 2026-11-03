from ph import *
c="semio-s-artifact-draw-drawing-scene"
T="🔍️trace/🦀️.rs"
E(c,T,"fn start_trace(&mut self,width:u32,height:u32,mask:Arc<Vec<u8>>){","fn start_trace(&mut self,width:u32,height:u32,mask:Arc<Vec<u8>>)->Result<(),DocumentSceneError>{")
E(c,T,"..}=&self.input.plan.nodes[self.at].content else{unreachable!()};let l=self.input.limits;","..}=&self.input.plan.nodes[self.at].content else{return Err(invalid(\"Document trace node lost its trace content\"))};let l=self.input.limits;")
E(c,T,'max_work:l.max_work}));self.phase="tracing";','max_work:l.max_work}));self.phase="tracing";Ok(())')
E(c,T,"self.start_trace(width,height,mask);return Ok(true);}","self.start_trace(width,height,mask)?;return Ok(true);}")
E(c,T,"self.image=None;self.start_trace(width,height,mask);return Ok(true);","self.image=None;self.start_trace(width,height,mask)?;return Ok(true);")
if __name__=="__main__":build()
