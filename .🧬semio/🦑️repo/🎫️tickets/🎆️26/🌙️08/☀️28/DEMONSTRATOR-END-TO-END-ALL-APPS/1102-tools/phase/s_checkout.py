from ph import *
c="semio-framework-os-store-engine-replay"
K="🏁️replay/checkout.rs"
def miss(w):return f'.ok_or_else(||invalid("checkpoint checkout {w} is absent"))?'
E(c,K,"self.place.as_mut().unwrap()","self.place.as_mut()"+miss("failure place"),2)
E(c,K,"self.order.as_mut().unwrap()","self.order.as_mut()"+miss("order"))
E(c,K,"self.order.as_ref().unwrap()","self.order.as_ref()"+miss("order"),5)
E(c,K,"self.supersessions.as_mut().unwrap()","self.supersessions.as_mut()"+miss("supersessions"),2)
E(c,K,"self.replay.as_mut().unwrap().as_mut().as_mut().unwrap()","self.replay.as_mut()"+miss("replay")+".as_mut().as_mut()"+miss("replay owner"))
E(c,K,"store.take_derived_report_result(self.replay.as_mut().unwrap())","store.take_derived_report_result(self.replay.as_mut()"+miss("replay")+")")
E(c,K,"else if !self.metadata.as_ref().unwrap().prepared(){","else if !self.metadata.as_ref()"+miss("metadata")+".prepared(){")
E(c,K,"let p=cx.with_native_receiving(|control|self.metadata.as_mut().unwrap().advance_in_order(store,self.result.as_ref().unwrap(),Some(order),control,grant))?;","let metadata=self.metadata.as_mut()"+miss("metadata")+";let result=self.result.as_ref()"+miss("result")+";let p=cx.with_native_receiving(|control|metadata.advance_in_order(store,result,Some(order),control,grant))?;")
E(c,K,"&mut self.result.as_mut().unwrap().rebased_inverse","&mut self.result.as_mut()"+miss("result")+".rebased_inverse")
E(c,K,"self.view.as_ref().unwrap()","self.view.as_ref()"+miss("view"))
E(c,K,"11=>{let result=self.result.as_mut().unwrap();","11=>{let result=self.result.as_mut()"+miss("result")+";")
E(c,K,'''let result=self.result.as_mut().unwrap();let cursor=parts.log.envelope.cursor.as_mut().ok_or_else(||invalid("checkpoint checkout requires its original persisted cursor"))?;self.metadata.as_mut().unwrap().metadata.swap_prepared(&mut cursor.applied_edit_ids,&mut parts.cursor.revision_accumulator).expect("checkpoint checkout metadata is privately prepared");std::mem::swap(&mut*parts.log.applied_edit_ids,&mut result.order);std::mem::swap(&mut*parts.log.redo_edit_ids,self.redo[0].as_mut().unwrap());std::mem::swap(&mut cursor.redo_edit_ids,self.redo[1].as_mut().unwrap());''','''let cursor=parts.log.envelope.cursor.as_mut().ok_or_else(||invalid("checkpoint checkout requires its original persisted cursor"))?;let (Some(result),Some(metadata),[Some(redo_log),Some(redo_cursor)],Some(gate))=(self.result.as_mut(),self.metadata.as_mut(),&mut self.redo,self.gate.as_mut())else{return Err(invalid("checkpoint checkout lost an original prepared root"))};let Some(state)=result.state.as_mut()else{return Err(invalid("checkpoint checkout lost its prepared state"))};metadata.metadata.swap_prepared(&mut cursor.applied_edit_ids,&mut parts.cursor.revision_accumulator).map_err(|_|invalid("checkpoint checkout metadata is not privately prepared"))?;std::mem::swap(&mut*parts.log.applied_edit_ids,&mut result.order);std::mem::swap(&mut*parts.log.redo_edit_ids,redo_log);std::mem::swap(&mut cursor.redo_edit_ids,redo_cursor);''')
E(c,K,'''let old=std::mem::replace(&mut*parts.live.current,result.state.take().unwrap());result.state=Some(old);parts.live.current_detached=false;assert!(self.gate.as_mut().unwrap().commit());''','''std::mem::swap(&mut*parts.live.current,state);parts.live.current_detached=false;if !gate.commit(){return Err(invalid("checkpoint checkout visibility gate refused commit"))}''')
if __name__=="__main__":build()
