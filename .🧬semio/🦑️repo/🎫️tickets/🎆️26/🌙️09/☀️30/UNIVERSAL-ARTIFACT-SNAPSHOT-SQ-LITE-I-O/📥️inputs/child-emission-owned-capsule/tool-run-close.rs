            let queued_owner=!member.retired_emission_owners.is_empty();
            let owner=if queued_owner{member.retired_emission_owners.front_mut()}else if everything{member.emission_owner.as_mut()}else{None};
            if let Some(owner)=owner{
                if maximum_items==0||maximum_bytes==0{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                if owner.is_empty(){
                    let bytes=std::mem::size_of_val(owner.as_ref());
                    if bytes>maximum_bytes{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                    if queued_owner{member.retired_emission_owners.pop_front();}else{member.emission_owner.take();}
                    return Ok(Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes}));
                }
                let step=match children.entries().find(|child|child.owner.slot==member.slot&&child.owner.child_id==member.child_id){
                    Some(child)=>child.member.visit_member(ToolRunMemberEmissionRetire{owner:owner.as_mut(),maximum_bytes})?,
                    None=>owner.close_step(1,maximum_bytes)?,
                };
                return Ok(Some(step));
            }
            let queued_emit=!member.retired_emits.is_empty();
            let emit=if queued_emit{member.retired_emits.front_mut()}else if everything{member.emit.as_mut()}else{None};
            if let Some(emit)=emit{
                let step=emit.close_one(maximum_items.min(1),maximum_bytes);
                if step==PluginCloseStep::Complete{if queued_emit{member.retired_emits.pop_front();}else{member.emit.take();}return Ok(Some(PluginCloseStep::Pending{released_items:1,released_bytes:0}));}
                return Ok(Some(step));
            }
            let queued_ops=!member.retired_ops.is_empty();
            let ops=if queued_ops{member.retired_ops.front_mut()}else if everything&&(!member.ops.is_empty()||member.ops.capacity()!=0){Some(&mut member.ops)}else{None};
            if let Some(ops)=ops{
                if maximum_items==0||maximum_bytes==0{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                if let Some(operation)=ops.last(){
                    let bytes=operation.capacity();
                    if bytes>maximum_bytes{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                    ops.pop();return Ok(Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes}));
                }
                let bytes=ops.capacity().checked_mul(std::mem::size_of::<Vec<u8>>()).expect("allocated member operation layout");
                if bytes>maximum_bytes{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                *ops=Vec::new();if queued_ops{member.retired_ops.pop_front();}return Ok(Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes}));
            }
            macro_rules! close_member_backing{
                ($field:ident,$item:ty)=>{
                    if member.$field.capacity()!=0{
                        let bytes=member.$field.capacity().checked_mul(std::mem::size_of::<$item>()).expect("allocated member retirement queue layout");
                        if maximum_items==0||bytes>maximum_bytes{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                        member.$field=std::collections::VecDeque::new();return Ok(Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes}));
                    }
                };
            }
            close_member_backing!(retired_emission_owners,Box<dyn ToolRunMemberEmissionOwner>);
            close_member_backing!(retired_emits,ChildEmit);
            close_member_backing!(retired_ops,Vec<Vec<u8>>);
