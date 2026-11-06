import fs from"node:fs";const p="🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🗃️page-set-and-reader-behavior/🦀️.rs";const old=fs.readFileSync(p,"utf8");const marker="impl CommandPageSet {";if(old.split(marker).length!==2)throw Error("impl");const code=`
    /// 🎟️ Quotes the actual retained page-set home and its exact initial page spine.
    pub fn source_admission_demand(declared:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        if declared==0||declared>COMMAND_MAXIMUM_PAGES{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"command page declaration exceeds its fixed authority"))}
        Ok(RetirementDemand{copy_bytes:size_of::<Option<Self>>(),capacity_bytes:declared.checked_mul(size_of::<FixedCommandPage>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"command page spine extent overflow"))?,depth:1,..Default::default()})
    }
    /// 🌱️ Retains actual allocated backing before reporting a successful or refused admission receipt.
    pub fn admit_source_step(output:&mut Option<Self>,declared:usize,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<semio_framework_value::RetainedCloneStep>{
        use semio_framework_value::{RetainedCloneStep as Step,RetainedCloneProgress as Progress,RetainedWorkOutcome as Turn,ValueError,ValueRefusalKind,retained_clone_grant_admits};
        if output.is_some(){return Turn{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"command page Source admission requires an empty destination")),work:0}}
        let demand=match Self::source_admission_demand(declared){Ok(demand)=>demand,Err(error)=>return Turn{result:Err(error),work:0}};
        if !retained_clone_grant_admits(grant,demand){return Turn{result:Ok(Step::Progress(Default::default())),work:0}}
        let mut pages=std::collections::VecDeque::new();
        if pages.try_reserve_exact(declared).is_err(){return Turn{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"command page Source could not reserve its declared spine")),work:1}}
        let capacity=pages.capacity()*size_of::<FixedCommandPage>();
        *output=Some(Self{pages,declared,byte_len:0,generic_shape_valid:true,all_nonempty:true});
        let progress=Progress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()};
        if !progress.fits(grant){return Turn{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"command page allocation exceeds its admitted Source extent").with_retained_progress(progress)),work:1}}
        Turn{result:Ok(Step::Complete(progress)),work:1}
    }
    /// 📏️ Prices caller clearing, the retained page slot and collection metadata without copying page bytes.
    pub fn source_capture_demand()->semio_framework_value::RetirementDemand{
        semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<(CommandPageCursor,FixedCommandPage)>>()+size_of::<FixedCommandPage>()+size_of::<Self>(),depth:1,..Default::default()}
    }
    /// 📥️ Captures the original page backing only after a real Source grant funds every actual metadata placement.
    pub fn capture_source_step(&mut self,input:&mut Option<(CommandPageCursor,FixedCommandPage)>,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<semio_framework_value::RetainedCloneStep>{
        use semio_framework_value::{RetainedCloneStep as Step,RetainedCloneProgress as Progress,RetainedWorkOutcome as Turn,ValueError,ValueRefusalKind,retained_clone_grant_admits};
        let Some((_,page))=input.as_ref()else{return Turn{result:Ok(Step::Complete(Default::default())),work:0}};
        let bytes=match self.checked_push_byte_len(page){Ok(bytes)=>bytes,Err(_)=>return Turn{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"command page Source has no slot or byte authority")),work:0}};
        let demand=Self::source_capture_demand();
        if !retained_clone_grant_admits(grant,demand){return Turn{result:Ok(Step::Progress(Default::default())),work:0}}
        let (_,page)=input.take().expect("checked command input remains present");
        self.push_validated(page,bytes);
        Turn{result:Ok(Step::Complete(Progress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}
    }
`;const next=old.replace(marker,marker+code);if(fs.readFileSync(p,"utf8")!==old)throw Error("stale");fs.writeFileSync(import.meta.dir+"/🗑️generated/command-source-admission.rs",next);fs.renameSync(import.meta.dir+"/🗑️generated/command-source-admission.rs",p);console.log("Implemented retained command Source admission and original page capture.");