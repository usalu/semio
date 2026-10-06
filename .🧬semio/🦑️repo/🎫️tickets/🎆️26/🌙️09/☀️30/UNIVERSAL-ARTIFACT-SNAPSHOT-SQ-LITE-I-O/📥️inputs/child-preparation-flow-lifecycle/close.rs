    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->semio_framework_job::InteractiveJobCloseStep{
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_plugin::app::PluginCloseStep;
        if !self.closing{return InteractiveJobCloseStep::Blocked;}
        if maximum_items==0||maximum_bytes==0{return InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0};}
        if let Some(emit)=self.output.as_mut(){
            if let Some(step)=emit.close_child_one(1,maximum_bytes){return match step{
                PluginCloseStep::Pending{released_items,released_bytes}=>InteractiveJobCloseStep::Pending{released_items,released_bytes},
                PluginCloseStep::Complete=>InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0},
                _=>InteractiveJobCloseStep::Blocked,
            };}
            if let Some(transaction)=emit.transaction.as_mut(){
                for text in [&mut transaction.id,&mut transaction.tool]{
                    let bytes=text.capacity();if bytes==0{continue;}
                    if bytes>maximum_bytes{return InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0};}
                    *text=String::new();return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:bytes};
                }
                emit.transaction=None;return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:0};
            }
            if emit.artifact_mutations.capacity()!=0||emit.config_mutations.capacity()!=0||emit.window_config_mutations.capacity()!=0||emit.draft_mutations.capacity()!=0
                ||emit.effects.capacity()!=0||emit.events.capacity()!=0||emit.extension_invocations.capacity()!=0||emit.interaction_writes.capacity()!=0||emit.tasks.capacity()!=0||matches!(emit.ui_scope,semio_framework::kernel::UiDirtyScope::Partial{..})
            {return InteractiveJobCloseStep::Blocked;}
            self.output=None;return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:0};
        }
        if self.instance_owner.take().is_some(){return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:0};}
        InteractiveJobCloseStep::Complete
    }
