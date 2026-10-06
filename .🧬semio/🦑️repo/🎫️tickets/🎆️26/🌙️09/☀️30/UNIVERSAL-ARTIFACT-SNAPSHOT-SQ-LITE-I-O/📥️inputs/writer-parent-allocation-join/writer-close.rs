        if self.return_refusal.is_some(){return InteractiveJobCloseStep::Blocked;}
        if let Some(rejected)=self.pending_completion_rejection.as_mut(){
            if maximum_items==0||maximum_bytes==0{return InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0};}
            let Ok(emit)=rejected.emit.as_mut()else{return InteractiveJobCloseStep::Blocked};
            match emit.return_child_one(&mut self.returned_allocations,maximum_items,maximum_bytes){
                Ok(Some(semio_framework_plugin::PluginCloseStep::Pending{released_items,released_bytes}))=>return InteractiveJobCloseStep::Pending{released_items,released_bytes},
                Ok(Some(_))=>return InteractiveJobCloseStep::Blocked,
                Err(error)=>{self.return_refusal=Some(error);return InteractiveJobCloseStep::Blocked;},
                Ok(None)=>{},
            }
            macro_rules! return_empty{($owner:expr)=>{if !$owner.is_empty(){return InteractiveJobCloseStep::Blocked;}if $owner.capacity()!=0{return match self.returned_allocations.return_empty_vec(&mut $owner,1){Ok(accepted)=>InteractiveJobCloseStep::Pending{released_items:usize::from(accepted),released_bytes:0},Err(error)=>{self.return_refusal=Some(error);InteractiveJobCloseStep::Blocked}};}};}
            return_empty!(emit.artifact_mutations);return_empty!(emit.config_mutations);return_empty!(emit.window_config_mutations);return_empty!(emit.draft_mutations);return_empty!(emit.effects);return_empty!(emit.extension_invocations);return_empty!(emit.events);return_empty!(emit.interaction_writes);return_empty!(emit.tasks);
            return_empty!(rejected.ephemeral.presence);return_empty!(rejected.ephemeral.transient);return_empty!(rejected.ephemeral.window_transient);
            if emit.transaction.is_some()||!matches!(emit.ui_scope,semio_framework::kernel::UiDirtyScope::Full|semio_framework::kernel::UiDirtyScope::None){return InteractiveJobCloseStep::Blocked;}
            if !self.returned_fault.terminal_is_empty(){return InteractiveJobCloseStep::Blocked;}
            let rejected=self.pending_completion_rejection.take().unwrap();self.returned_fault=semio_framework_plugin::__diagnostic::FaultCloseOwner::new(rejected.fault);
            return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:0};
        }
        if !self.returned_allocations.terminal_is_empty(){return match self.returned_allocations.close_step(maximum_items,maximum_bytes){semio_framework_value::retirement::allocation_return::AllocationReturnStep::Pending{released_items,released_bytes}=>InteractiveJobCloseStep::Pending{released_items,released_bytes},semio_framework_value::retirement::allocation_return::AllocationReturnStep::Complete=>InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0}};}
        if !self.returned_fault.terminal_is_empty(){return match self.returned_fault.close_step(maximum_items,maximum_bytes){semio_framework_plugin::__diagnostic::FaultCloseStep::Pending{released_items,released_bytes}=>InteractiveJobCloseStep::Pending{released_items,released_bytes},semio_framework_plugin::__diagnostic::FaultCloseStep::Complete=>InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0}};}
