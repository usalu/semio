  const admissionFile=()=>buildAdmissionReceiptPathV1(dirname(path),scope!);
  let admissionMemo:{admission:BuildOwnerAdmissionV1;observations:readonly observationV1.SourceObservationV1[];startedAt:number}|undefined;
  const reuseAdmission=async():Promise<{files:string[];digest:string;admission:BuildOwnerAdmissionV1;observations:readonly observationV1.SourceObservationV1[];startedAt:number;persisted:boolean}|OwnerRegistryInvalidationV1|undefined>=>{
    let known=admissionMemo,persisted=false;
    if(!known){
      let text:string;try{text=await readFile(admissionFile(),"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
      const receipt=readBuildAdmissionReceiptV1(text,{scope:scope!,module:loadedGraphModuleAuthorityV1});if(!receipt)return undefined;
      known={admission:receipt.admission,observations:receipt.observations as readonly observationV1.SourceObservationV1[],startedAt:receipt.startedAt};persisted=true;
    }
    if(!await buildGraphSourceFactsHoldV1(known.observations,invocation))return undefined;
    requireCurrentGraphModuleV1();const current=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(buildGraphScopeKeyV1(options.selection,current)!==scope)return ownerRegistryInvalidationV1;
    try{return {files:known.admission.files,digest:await buildGraphMembershipV1(root,known.admission.files,invocation),admission:known.admission,observations:known.observations,startedAt:known.startedAt,persisted};}catch(error){if(buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED")return undefined;throw error;}
  };
  const publishAdmission=async(admission:BuildOwnerAdmissionV1,observations:readonly unknown[],startedAt:number):Promise<void>=>{
    if(!buildAdmissionReceiptPublishableV1(observations,startedAt))return;
    const temporary=admissionFile()+"."+randomUUID();
    try{await writeFile(temporary,createBuildAdmissionReceiptV1({version:1,scope:scope!,module:loadedGraphModuleAuthorityV1,startedAt,admission,observations}),{flag:"wx"});if(invocation)checkScriptInvocation(invocation);requireCurrentGraphModuleV1();await rename(temporary,admissionFile());}
    catch(error){if(invocation?.control.signal.aborted)throw error;}
    finally{await rm(temporary,{force:true});}
  };
  let admissionPorts:BuildOwnerAdmissionPortsV1|undefined;
  const records=():BuildOwnerAdmissionPortsV1=>admissionPorts??={directory:repositoryBuildGraphWriterDirectoryV1(root,"inference"),implementation:producer.currentImplementationRevisionV1(),scope:scope!,registry:createHash("sha256").update(JSON.stringify(registry)).digest("hex"),check:()=>{if(invocation)checkScriptInvocation(invocation);},advance:async stage=>{if(invocation)await advanceScriptInvocation(invocation,stage,"running");},observe:(request,compute)=>observation.observeProjectInferenceV1(request,compute)};
  const inventory=async()=>{let started=performance.now();const reused=selected?await reuseAdmission():undefined;if(reused)return reused;const startedAt=Date.now();
    await producer.libraryBootstrap;const captured=selected?await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{
