"""Anchored edits of the run-path bootstrap (`🏗️build/🚀️bootstrap/🟦️.ts`): a task run never repeats a peer's installation and never walks under the exclusive hold."""

HELPERS = r'''

/** 🧷️ The dependency store a task run holds while it decides whether to install. */
export type BuildDependencyStoreV1=Parameters<Parameters<typeof import("../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts").withJavascriptDependencyStorageV1>[3]>[0];

/** 🔁️ Answers, under the exclusive hold the installer finally obtained, whether a peer installed in the meantime: receipt-based and O(changed), never a walk. */
export type BuildEnvironmentRecheckV1=(store:BuildDependencyStoreV1,control:ResourceLeaseControl)=>Promise<boolean>;

/** 🪝️ Runs the installer only when the environment is still unready once the exclusive hold is granted, so task runs queued behind one synchronization do not each repeat it. */
export async function installUnlessReadyV1(store:BuildDependencyStoreV1,control:ResourceLeaseControl,recheck:BuildEnvironmentRecheckV1|undefined,prepare:()=>Promise<void>):Promise<boolean>{
 if(recheck&&await recheck(store,control))return false;
 await prepare();return true;
}

/** 🤝️ Accepts the evidence a shared walk gathered once the exclusive writer proved it against the live tree; anything else sends the writer back to a fresh walk. */
export function adoptedCandidateV1(status:"published"|"present"|"deferred"|"stale"|"failed"):boolean{return status==="published"||status==="present";}'''

BOOTSTRAP_OPS = [
    ("export type BuildEnvironmentInstallerV1=(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl)=>Promise<void>;",
     "export type BuildEnvironmentInstallerV1=(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,recheck?:BuildEnvironmentRecheckV1)=>Promise<void>;" + HELPERS),
    ("install:BuildEnvironmentInstallerV1=async(root,original,phase)=>{",
     "install:BuildEnvironmentInstallerV1=async(root,original,phase,recheck)=>{"),
    ('(store,control)=>prepareJavascriptDependencies("sync",root,store,control,{kind:"all"}));checkResourceLeaseControl(phase);',
     '(store,control)=>installUnlessReadyV1(store,control,recheck,()=>prepareJavascriptDependencies("sync",root,store,control,{kind:"all"})).then(()=>undefined));checkResourceLeaseControl(phase);'),
    ("  const inspect=async(publishReceipt:boolean)=>storage(publishReceipt,async(store,actualControl)=>{",
     "  let candidate:{walk:unknown;startedAt:number}|undefined;\n  const inspectIn=(publishReceipt:boolean,cheap:boolean):Parameters<typeof withJavascriptDependencyStorageV1>[3]=>async(store,actualControl)=>{"),
    ('fast=await generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,bootstrap.scope);let walk:Awaited<ReturnType<typeof currentOwner>>|undefined,owner:Pick<Awaited<ReturnType<typeof currentOwner>>,"receipts"|"sha256">;try{owner=fast.status==="match"?fast:(walk=await currentOwner());}',
     'fast=await generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,bootstrap.scope);if(cheap&&fast.status!=="match")return "material-required";let walk:Awaited<ReturnType<typeof currentOwner>>|undefined,owner:Pick<Awaited<ReturnType<typeof currentOwner>>,"receipts"|"sha256">,cached=publishReceipt&&candidate?{walk:candidate.walk as Awaited<ReturnType<typeof currentOwner>>,startedAt:candidate.startedAt}:undefined;if(publishReceipt)candidate=undefined;try{owner=fast.status==="match"?fast:cached?(walk=cached.walk):(walk=await currentOwner());}'),
    ('throw new Error("Build bootstrap defining Source changed");\n    if(!publishReceipt&&walk)return "receipt-required";',
     'throw new Error("Build bootstrap defining Source changed");\n    if(publishReceipt&&cached&&walk){const sealed=await generation.publishJavascriptDependencyGenerationReceiptV1(workspace,store,actualControl,check,walk,cached.startedAt);if(!adoptedCandidateV1(sealed)){cached=undefined;walk=await currentOwner();owner=walk;current=witness();}}\n    if(!publishReceipt&&walk){candidate={walk,startedAt:walkStartedAt};return "receipt-required";}'),
    ("const confirmed=walk?await currentOwner():", "const confirmed=walk&&!cached?await currentOwner():"),
    ("const settled=walk?await currentOwner():", "const settled=walk&&!cached?await currentOwner():"),
    ('if(walk)await generation.publishJavascriptDependencyGenerationReceiptV1(workspace,store,actualControl,check,walk,walkStartedAt);return "ready";\n  });\n  const inspection=',
     'if(walk&&!cached)await generation.publishJavascriptDependencyGenerationReceiptV1(workspace,store,actualControl,check,walk,walkStartedAt);return "ready";\n  };\n  const inspect=async(publishReceipt:boolean)=>storage(publishReceipt,inspectIn(publishReceipt,false)),recheck:BuildEnvironmentRecheckV1=async(store,control)=>await inspectIn(false,true)(store,control)!=="material-required";\n  const inspection='),
    ("await install(root,invocation,phaseControl);", "await install(root,invocation,phaseControl,recheck);"),
]
