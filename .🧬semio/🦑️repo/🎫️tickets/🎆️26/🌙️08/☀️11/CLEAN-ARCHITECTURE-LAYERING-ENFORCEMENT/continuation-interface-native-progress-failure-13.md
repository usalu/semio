# Native Progress Failure in Whole 13

Actual failure from frozen whole13 retains original runtime membership expectations. Snapshot root Cargo.toml later differs from its sealed original. The preparation publication owner must produce exact before/after custody and update current discovery before consumption; old prepared pages cannot stand in for current source.

```
                                                                                                                                                                                    ^
error: Cargo command refused with retained ownership: Cargo published membership differs from prepared pages
      owner: CargoCliOperationOwner {
  progress: [Object ...],
  generations: [
    [Object ...]
  ],
  policy: [Object ...],
  currentWorkspace: [Object ...],
  currentOperation: [Object ...],
  discoverySignal: [AbortController ...],
  retirementSignal: [AbortController ...],
  retirement: [Object ...],
  lastProgress: 1791432812828,
  attached: false,
  completedBefore: 0,
  bytesBefore: 0,
  retirementGeneration: 1,
  onInterrupt: [Function],
  workspace: [Getter],
  operation: [Getter],
  retirementCompleted: [Getter],
  operationFor: [Function: operationFor],
  publish: [Function: publish],
  advanceWorkspace: [AsyncFunction: advanceWorkspace],
  cancelDiscovery: [Function: cancelDiscovery],
  retirementAborted: [Getter],
  cancelRetirement: [Function: cancelRetirement],
  detachSignals: [Function: detachSignals],
  closeSystemOwners: [AsyncFunction: closeSystemOwners],
},
 cleanupError: null,

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-13/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

14 | }
15 | async function equal(left:readonly string[],right:readonly string[],control:CargoController,path:string):Promise<boolean>{let a=0,b=0,x=0,y=0;while(a<left.length&&b<right.length){await control.step("publication-preimage",path);if(left[a]![x]!==right[b]![y])return false;if(++x===left[a]!.length){a++;x=0;}if(++y===right[b]!.length){b++;y=0;}}return a===left.length&&b===right.length;}
16 | async function current(root:string,path:string,operation:CargoDiscoveryOperation):Promise<CargoManifestOwner>{const control=new CargoController(operation,operation.workspace),identity=await cargoPhysicalPath(root,path,operation,"file");await control.step("publication-observation-owner",path,1,256+path.length*2);const owner=new CargoManifestOwner(path,operation.workspace);operation.workspace.manifestOwners.push(owner);try{await owner.read(identity.full,identity,control);owner.state="source-complete";return owner;}catch(error){owner.state="refused";throw error;}}
17 | 
18 | /** 🔒️ Atomically publishes bounded prepared pages after an exact fresh source guard under the caller's real lease. */
19 | export async function publishPreparedCargoMembership(root:string,draft:CargoMembershipDraft,mode:"check"|"write",operation:CargoPublicationOperation):Promise<boolean>{const control=new CargoController(operation,operation.workspace);await control.step("publication-lease",draft.path);if(operation.lease.mode!=="exclusive"||operation.lease.resource!==`cargo-preparation:${resolve(root)}`)throw Error("Cargo publication requires its exact exclusive repository lease");if(draft.state!=="complete"||!draft.document)throw Error("Cargo publication requires a complete retained draft");if(mode!=="check"&&mode!=="write")throw Error("Cargo publication mode is invalid");const before=await current(root,draft.path,operation);if(!await equal(draft.before.chunks,before.pages,control,draft.path))throw Error("Cargo manifest changed since membership admission");if(await equal(draft.before.chunks,draft.pages,control,draft.path))return false;if(mode==="check")throw Error("Cargo source membership is stale");const identity=await cargoPhy

error: Cargo published membership differs from prepared pages
      at publishPreparedCargoMembership (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-13/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📣️publication/📁️physical/🟦️.ts:19:2040)

Bun v1.3.14 (macOS arm64)


Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-13/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts:89:41)
(fail) native progress follows the declared owner and preserves independent child output [94442.45ms]
(pass) dashboard-selected owner tests allocate their output without manual environment setup [75.72ms]

🧰️framework/🛍️products/🦑️repo/🔨️modules/
```
