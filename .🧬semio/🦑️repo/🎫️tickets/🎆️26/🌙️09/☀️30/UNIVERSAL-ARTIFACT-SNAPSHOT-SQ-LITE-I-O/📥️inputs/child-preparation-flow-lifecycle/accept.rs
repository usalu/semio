    fn accept_output(&mut self,emit:Emit<FlowMutation,NoConfigMutation,NoDraftMutation>,child_id:&str)->Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>,Fault>{
        assert!(self.output.is_none(),"one Flow work unit retains at most one original output");
        self.output=Some(emit);
        let emit=self.output.as_ref().expect("original output retained before validation");
        let maximum=FLOW_STORE_MAX_MUTATION_ITEMS+1;
        let ready=emit.child_emits.first().is_some_and(|child|child.slot=="content"&&child.child_id==child_id&&!child.ops.is_empty()&&child.ops.len()<=maximum&&child.labels.len()==child.ops.len());
        let preparing=emit.child_preparations.front().is_some_and(|source|source.matches_source::<SemioFlowMutation>("content",child_id,maximum));
        let exact=(ready&&emit.child_preparations.is_empty())||(preparing&&emit.child_emits.is_empty());
        if emit.child_emits.len()+emit.child_preparations.len()>1
            || ((!emit.child_emits.is_empty()||!emit.child_preparations.is_empty())&&!exact)
            || (emit.transaction.is_some()&&!exact)
            || !emit.artifact_mutations.is_empty()||!emit.config_mutations.is_empty()||!emit.draft_mutations.is_empty()
            || !emit.effects.is_empty()||!emit.events.is_empty()
        {return Err(Fault::from("flow-retained-child-group-output-contract"));}
        self.completed=true;
        Ok(ArtifactCommandWorkStep::Complete(self.output.take().expect("validated original output transferred once")))
    }
