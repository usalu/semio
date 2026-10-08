# Whole25 Live Failure Contexts

This is a bounded read-only extraction from the live original whole25 stderr stream at 2026-10-08T15:48:19.372673+00:00. It is not a terminal summary or an accepted proof. Session51138 remains live and the entire stream is retained. No original law identity, counterfactual body, or control has been removed.

## (fail) actual-package-policy [15009.43ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

 5 | const yieldMarks=new WeakMap<CargoAccounting,number>();
 6 | 
 7 | /** 🎛️ Accounts before ownership and yields at bounded work frontiers. */
 8 | export class CargoController {
 9 |  constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
10 |  check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
                                                           ^
error: Cargo operation cancelled
      at check (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:10:55)
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:74)
      at cargoPhysicalPath (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📁️physical/🟦️.ts:48:761)

Bun v1.3.14 (macOS arm64)


Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts:49:70)
(fail) actual-package-policy [15009.43ms]
  ^ this test timed out after 15000ms.
killed 1 dangling process
```

## (fail) actual-virtual-workspace-no-foreign-policy [15014.35ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

 5 | const yieldMarks=new WeakMap<CargoAccounting,number>();
 6 | 
 7 | /** 🎛️ Accounts before ownership and yields at bounded work frontiers. */
 8 | export class CargoController {
 9 |  constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
10 |  check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
                                                           ^
error: Cargo operation cancelled
      at check (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:10:55)
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:74)
      at checkpoint (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts:32:258)
      at pathPresent (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts:35:376)

Bun v1.3.14 (macOS arm64)


Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts:49:70)
(fail) actual-virtual-workspace-no-foreign-policy [15014.35ms]
  ^ this test timed out after 15000ms.
(pass) invalid-manifest-refuses-child [125.53ms]
```

## (fail) native progress follows the declared owner and preserves independent child output [180002.80ms]

```text
  cancelDiscovery: [Function: cancelDiscovery],
  retirementAborted: [Getter],
  cancelRetirement: [Function: cancelRetirement],
  detachSignals: [Function: detachSignals],
  closeSystemOwners: [AsyncFunction: closeSystemOwners],
},
 cleanupError: null,

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

 5 | const yieldMarks=new WeakMap<CargoAccounting,number>();
 6 | 
 7 | /** 🎛️ Accounts before ownership and yields at bounded work frontiers. */
 8 | export class CargoController {
 9 |  constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
10 |  check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
                                                           ^
error: Cargo operation cancelled
      at check (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:10:55)
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:74)
      at checkpoint (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts:32:258)
      at pathPresent (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts:35:376)

Bun v1.3.14 (macOS arm64)


Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts:49:70)
(fail) actual-virtual-workspace-no-foreign-policy [15014.35ms]
  ^ this test timed out after 15000ms.
(pass) invalid-manifest-refuses-child [125.53ms]
(pass) interactive dashboard delegates its native owner progress [0.41ms]
killed 2 dangling processes
(fail) native progress follows the declared owner and preserves independent child output [180002.80ms]
  ^ this test timed out after 180000ms.

```

## (fail) production test exclusion agrees with independent esbuild compiler and preserves live dynamic fixture refusals [2.98ms]

```text
(pass) runtime graph domain-fixtures-module [0.26ms]
(pass) runtime graph bundled-module-url-mount [0.99ms]
(pass) runtime graph bundled-module-url-fixture [0.18ms]
(pass) runtime graph bundled-module-url-unmounted [0.09ms]
(pass) runtime graph rust-logical-negation-is-not-macro [0.06ms]
(pass) runtime graph dynamic-exact-callsite-only [0.19ms]
(pass) runtime graph dynamic-outside-callsite [0.09ms]
(pass) runtime graph dynamic-changed-caller [0.10ms]
(pass) runtime graph dynamic-unbound-caller [0.09ms]
(pass) runtime graph rust-resource-read-owned [0.15ms]
(pass) runtime graph rust-resource-read-callsite-only [0.07ms]
(pass) runtime graph rust-resource-read-changed-source [0.05ms]
(pass) runtime graph rust-resource-read-fixture [0.06ms]
(pass) runtime graph rust-resource-read-directory [0.07ms]
(pass) runtime graph rust-resource-read-directory-drift [0.05ms]
(pass) runtime resource-read owners retain exact file bytes and terminal directory metadata [16.50ms]
(pass) dynamic import owners bind one exact computed callsite against independent TypeScript and SHA oracles [18.76ms]
(pass) Bun import edges match independent TypeScript parser for static, dynamic, require and resource literals [5.32ms]
(pass) actual Rust compiler dep-info agrees with test cfg exclusions and reachable embedded resources [3244.57ms]
(pass) compiled input mismatch and generated fixture origin refuse evidence [1.59ms]
(pass) Cargo runtime dependency closure retains normal/build while isolating dev-only features [0.73ms]
994 |           inputFS: inputPath !== null,
995 |           input: inputPath !== null ? encodeUTF8(inputPath) : typeof input === "string" ? encodeUTF8(input) : input
996 |         };
997 |         if (mangleCache) request.mangleCache = mangleCache;
998 |         sendRequest(refs, request, (error, response) => {
999 |           if (error) return callback(new Error(error), null);
                                               ^
error: The service is no longer running
      at <anonymous> (/Users/ueli/Documents/semio/node_modules/esbuild/lib/main.js:999:42)
      at start (/Users/ueli/Documents/semio/node_modules/esbuild/lib/main.js:998:9)
      at transform (/Users/ueli/Documents/semio/node_modules/esbuild/lib/main.js:1059:5)
      at new Promise (1:11)
      at transform (/Users/ueli/Documents/semio/node_modules/esbuild/lib/main.js:2333:36)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts:200:24)
(fail) production test exclusion agrees with independent esbuild compiler and preserves live dynamic fixture refusals [2.98ms]
(pass) Cargo host unit cfg and escaped dep-info filesystem inputs retain separate authority [0.52ms]
(pass) actual compiler roster reconciles only macro expansion and retains unresolved resources [0.37ms]
```

## (fail) actual Trunk compiled transformation retains current mounted bytes after temporary cleanup [383.13ms]

```text
(pass) actual compiler roster reconciles only macro expansion and retains unresolved resources [0.37ms]
(pass) actual native filesystem reads remain resource edges absent from compiler dep-info [679.57ms]
(pass) durable Cargo discovery selects only current exact invocation observations [8.62ms]
[cargo-operation] invocation-argument 417 units 512 retained bytes /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot
[cargo-operation] system-owners-closed 0 units 0 retained bytes 
[cargo-operation] invocation-argument 404 units 512 retained bytes /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot
    Checking runtime-graph-live-oracle v0.0.0 (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/controlled-cargo-current-25/runtime-cargo-currentness-oracle/owner)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
[cargo-operation] system-owners-closed 0 units 0 retained bytes 
(pass) actual durable Cargo producer acquires current features and refuses stale configuration/artifacts [628.74ms]
(pass) fresh browser compiler bytes require the same actual mounted output [10.12ms]
(pass) Trunk raw compiler query and actual mounted outputs bind exact bytes [11.37ms]
[cargo-operation] invocation-argument 395 units 512 retained bytes /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot
     Locking 12 packages to latest compatible versions
      Adding wasm-bindgen v0.2.126 (available: v0.2.129)
      Adding wasm-bindgen-macro v0.2.126 (available: v0.2.129)
      Adding wasm-bindgen-macro-support v0.2.126 (available: v0.2.129)
      Adding wasm-bindgen-shared v0.2.126 (available: v0.2.129)
[cargo-operation] system-owners-closed 0 units 0 retained bytes 
54 | 
55 | /** 🔐️ Checks the optimizer and every required library against the verified archive receipt. */
56 | export function preparedBinaryen(workspace: string): string {
57 |   const directory = binaryenDirectory(workspace), executable = `bin/wasm-opt${process.platform === "win32" ? ".exe" : ""}`;
58 |   regularDirectory(directory);
59 |   const receipt = JSON.parse(readFileSync(join(directory, ".toolchain.json"), "utf8"));
                                  ^
ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/⚡️cache/tools/binaryen/130/darwin-arm64-79d3ab9f417d9e215f15f598f523d001a7d9ac1e59367e5c869fbdabd1cba72e/.toolchain.json'
    path: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/⚡️cache/tools/binaryen/130/darwin-arm64-79d3ab9f417d9e215f15f598f523d001a7d9ac1e59367e5c869fbdabd1cba72e/.toolchain.json",
 syscall: "open",
   errno: -2,
    code: "ENOENT"

      at preparedBinaryen (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts:59:30)
      at buildTrunkRenderer (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts:38:21)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts:308:61)
(fail) actual Trunk compiled transformation retains current mounted bytes after temporary cleanup [383.13ms]
(pass) actual rustc consumed-byte checksums refuse edits before completed receipt observation [196.58ms]
(pass) actual proc-macro reads preserve original snapshots and tracked directory inputs [1659.87ms]
```

## (fail) queued native preparation survives waiting beyond one active recipe budget [31396.69ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

25 |  get operation():CargoDiscoveryOperation{return this.currentOperation;}
26 |  get retirementCompleted():number{return this.retirement.accounting.completed;}
27 |  private operationFor(workspace:CargoDiscoveryWorkspace):CargoDiscoveryOperation{const completedBefore=this.completedBefore,bytesBefore=this.bytesBefore;return{workspace,signal:this.discoverySignal.signal,maximumUnits:this.policy.discovery.maximumUnits-this.completedBefore,maximumOwnedBytes:this.policy.discovery.maximumOwnedBytes-this.bytesBefore,maximumDepth:this.policy.discovery.maximumDepth,onProgress:row=>this.publish({...row,completed:row.completed+completedBefore,ownedBytes:row.ownedBytes+bytesBefore}),yieldContinuation:()=>this.progress.yieldContinuation()};}
28 |  private publish(row:CargoProgress):void{const now=Date.now();if(now-this.lastProgress>=this.policy.progressIntervalMilliseconds){this.lastProgress=now;this.progress.publish(row);}}
29 |  /** 🆕️ Admits a fresh discovery view after completed physical execution or publication without resetting cumulative budgets. */
30 |  async advanceWorkspace():Promise<CargoDiscoveryOperation>{if(this.currentWorkspace.state!=="published"&&this.currentWorkspace.state!=="prepared")throw Error("Cargo fresh discovery requires its completed physical phase");await new CargoController(this.currentOperation,this.currentWorkspace).step("discovery-generation","",1,512);this.completedBefore+=this.currentWorkspace.completed;this.bytesBefore+=this.currentWorkspace.ownedBytes;const workspace=new CargoDiscoveryWorkspace();this.generations.push(workspace);this.currentWorkspace=workspace;this.currentOperation=this.operationFor(workspace);return this.currentOperation;}

error: Cargo fresh discovery requires its completed physical phase
      at advanceWorkspace (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🟦️.ts:30:153)
      at prepareCargoWorkspaceInvocation (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🏃️invocation/🟦️.ts:20:3384)

Bun v1.3.14 (macOS arm64)


Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🕰️queued-preparation/🟦️.ts:35:50)
(fail) queued native preparation survives waiting beyond one active recipe budget [31396.69ms]

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🗂️owner-exclusions/🟦️.ts:
```

## (fail) every compiling native command prepares owners and keeps diagnostics off machine stdout [180.30ms]

```text
  completedBefore: 3501,
  bytesBefore: 55614,
  retirementGeneration: 2,
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

25 |  get operation():CargoDiscoveryOperation{return this.currentOperation;}
26 |  get retirementCompleted():number{return this.retirement.accounting.completed;}
27 |  private operationFor(workspace:CargoDiscoveryWorkspace):CargoDiscoveryOperation{const completedBefore=this.completedBefore,bytesBefore=this.bytesBefore;return{workspace,signal:this.discoverySignal.signal,maximumUnits:this.policy.discovery.maximumUnits-this.completedBefore,maximumOwnedBytes:this.policy.discovery.maximumOwnedBytes-this.bytesBefore,maximumDepth:this.policy.discovery.maximumDepth,onProgress:row=>this.publish({...row,completed:row.completed+completedBefore,ownedBytes:row.ownedBytes+bytesBefore}),yieldContinuation:()=>this.progress.yieldContinuation()};}
28 |  private publish(row:CargoProgress):void{const now=Date.now();if(now-this.lastProgress>=this.policy.progressIntervalMilliseconds){this.lastProgress=now;this.progress.publish(row);}}
29 |  /** 🆕️ Admits a fresh discovery view after completed physical execution or publication without resetting cumulative budgets. */
30 |  async advanceWorkspace():Promise<CargoDiscoveryOperation>{if(this.currentWorkspace.state!=="published"&&this.currentWorkspace.state!=="prepared")throw Error("Cargo fresh discovery requires its completed physical phase");await new CargoController(this.currentOperation,this.currentWorkspace).step("discovery-generation","",1,512);this.completedBefore+=this.currentWorkspace.completed;this.bytesBefore+=this.currentWorkspace.ownedBytes;const workspace=new CargoDiscoveryWorkspace();this.generations.push(workspace);this.currentWorkspace=workspace;this.currentOperation=this.operationFor(workspace);return this.currentOperation;}

error: Cargo fresh discovery requires its completed physical phase
      at advanceWorkspace (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🟦️.ts:30:153)
      at prepareCargoWorkspaceInvocation (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🏃️invocation/🟦️.ts:17:133)
      at processTicksAndRejections (unknown:7:39)

Bun v1.3.14 (macOS arm64)

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts:113:34)
(fail) every compiling native command prepares owners and keeps diagnostics off machine stdout [180.30ms]
(pass) one membership invocation parses each current manifest once and observes later saves [1369.91ms]
(pass) batch package selection discovers one fresh repository inventory [1219.01ms]
```

## (fail) Cargo preparation diagnostics observe only opted-in exact phases [2860.95ms]

```text
+   "bridge",
+   "bridge",
+   "bridge",
+   "bridge",
+   "bridge",
    "builder",
+   "builder",
+   "builder",
+   "builder",
+   "builder",
+   "builder",
+   "kernel",
+   "kernel",
+   "kernel",
+   "kernel",
+   "kernel",
    "kernel",
    "neutral-app",
+   "neutral-app",
+   "neutral-app",
+   "neutral-app",
+   "neutral-app",
+   "neutral-app",
+   "root-test",
+   "root-test",
+   "root-test",
+   "root-test",
+   "root-test",
    "root-test",
  ]

- Expected  - 0
+ Received  + 25

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts:211:156)
(fail) Cargo preparation diagnostics observe only opted-in exact phases [2860.95ms]
(pass) queued Cargo preparation exposes opted-in waiting before the protected operation completes [95.72ms]
262 |   expect([...reached].map(id=>(packages.get(id) as any).name).sort()).toEqual(row.expected);
```

## (fail) selected package preparation follows the Cargo resolved local closure without preparing siblings [1058.25ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

25 |  get operation():CargoDiscoveryOperation{return this.currentOperation;}
26 |  get retirementCompleted():number{return this.retirement.accounting.completed;}
27 |  private operationFor(workspace:CargoDiscoveryWorkspace):CargoDiscoveryOperation{const completedBefore=this.completedBefore,bytesBefore=this.bytesBefore;return{workspace,signal:this.discoverySignal.signal,maximumUnits:this.policy.discovery.maximumUnits-this.completedBefore,maximumOwnedBytes:this.policy.discovery.maximumOwnedBytes-this.bytesBefore,maximumDepth:this.policy.discovery.maximumDepth,onProgress:row=>this.publish({...row,completed:row.completed+completedBefore,ownedBytes:row.ownedBytes+bytesBefore}),yieldContinuation:()=>this.progress.yieldContinuation()};}
28 |  private publish(row:CargoProgress):void{const now=Date.now();if(now-this.lastProgress>=this.policy.progressIntervalMilliseconds){this.lastProgress=now;this.progress.publish(row);}}
29 |  /** 🆕️ Admits a fresh discovery view after completed physical execution or publication without resetting cumulative budgets. */
30 |  async advanceWorkspace():Promise<CargoDiscoveryOperation>{if(this.currentWorkspace.state!=="published"&&this.currentWorkspace.state!=="prepared")throw Error("Cargo fresh discovery requires its completed physical phase");await new CargoController(this.currentOperation,this.currentWorkspace).step("discovery-generation","",1,512);this.completedBefore+=this.currentWorkspace.completed;this.bytesBefore+=this.currentWorkspace.ownedBytes;const workspace=new CargoDiscoveryWorkspace();this.generations.push(workspace);this.currentWorkspace=workspace;this.currentOperation=this.operationFor(workspace);return this.currentOperation;}

error: Cargo fresh discovery requires its completed physical phase
      at advanceWorkspace (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🟦️.ts:30:153)
      at prepareCargoWorkspaceInvocation (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🏃️invocation/🟦️.ts:20:3384)

Bun v1.3.14 (macOS arm64)


Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts:267:50)
(fail) selected package preparation follows the Cargo resolved local closure without preparing siblings [1058.25ms]
(pass) membership discovery compiles bounded matchers independent of unrelated leaf count [38.07ms]
(pass) explicit root-owned packages remain discoverable beneath a different native workspace [246.49ms]
```

## (fail) current physical workspace admissions and selected nextest profiles match independent native documents [9214.34ms]

```text
  discoverySignal: [AbortController ...],
  retirementSignal: [AbortController ...],
  retirement: [Object ...],
  lastProgress: 1791473911070,
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

 6 | 
 7 | /** 🎛️ Accounts before ownership and yields at bounded work frontiers. */
 8 | export class CargoController {
 9 |  constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
10 |  check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
11 |  async step(stage:string,path:string,units=1,bytes=0):Promise<void>{this.check();if(!Number.isSafeInteger(units)||units<0||!Number.isSafeInteger(bytes)||bytes<0)throw Error("Cargo accounting requires finite nonnegative admissions");if(units>this.operation.maximumUnits-this.accounting.completed||bytes>this.operation.maximumOwnedBytes-this.accounting.ownedBytes)throw Error("Cargo operation budget refused");this.accounting.completed+=units;this.accounting.ownedBytes+=bytes;if(this.accounting.completed-yieldMarks.get(this.accounting)!>=128||units===0){this.operation.onProgress({stage,path,completed:this.accounting.completed,ownedBytes:this.accounting.ownedBytes});this.check();await this.operation.yieldContinuation();this.check();yieldMarks.set(this.accounting,this.accounting.completed);}}

error: Cargo operation budget refused
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:369)
      at cargoPhysicalDirectory (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📁️physical/🟦️.ts:58:341)
(fail) current physical workspace admissions and selected nextest profiles match independent native documents [9214.34ms]
(pass) all-root dependency preparation honors explicit package workspaces beneath native child owners [390.91ms]
(pass) named dependency preparation agrees with actual Cargo root graph reachability [549.30ms]
```

## (fail) process budgets let captured nextest compilation finish before budgeted assertions [5011.46ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

 5 | const yieldMarks=new WeakMap<CargoAccounting,number>();
 6 | 
 7 | /** 🎛️ Accounts before ownership and yields at bounded work frontiers. */
 8 | export class CargoController {
 9 |  constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
10 |  check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
                                                           ^
error: Cargo operation cancelled
      at check (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:10:55)
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:74)
      at cargoPhysicalPath (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📁️physical/🟦️.ts:48:761)

Bun v1.3.14 (macOS arm64)


Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:122:38)
(fail) process budgets let captured nextest compilation finish before budgeted assertions [5011.46ms]
143 |       }));
144 |       const { runRepositoryCargoTests } = await import(${JSON.stringify(libraryPath)});
```

## (fail) process budgets separate coverage compilation from assertions: nextest=true [5018.64ms]

```text
 8 | export class CargoController {
 9 |  constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
10 |  check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
                                                           ^
error: Cargo operation cancelled
      at check (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:10:55)
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:74)
      at cargoPhysicalPath (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📁️physical/🟦️.ts:48:761)

Bun v1.3.14 (macOS arm64)


Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:122:38)
(fail) process budgets let captured nextest compilation finish before budgeted assertions [5011.46ms]
143 |       }));
144 |       const { runRepositoryCargoTests } = await import(${JSON.stringify(libraryPath)});
145 |       await runRepositoryCargoTests(["semio-s-artifact-stdio-mp4"], process.cwd());
146 |     `;
147 |     const result = await execa(process.execPath, ["-e", code], { env: { ...cleanEnv(), SEMIO_COVERAGE: "1", SEMIO_TEST_BUDGET_MS: "200" }, timeout: 5000, reject: false });
148 |     expect(result.code, result.stderr).toBe(0);
                                             ^
error: [cargo-operation] repository-root-owner 194 units 856 retained bytes /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot
[cargo-operation] pattern-complete 367191 units 3047036 retained bytes **/🏅️standards/**
[cargo-operation] pattern-complete 1750344 units 3870924 retained bytes **/🏅️standards/**
[cargo-operation] pattern-segment 3900387 units 5059426 retained bytes **/🏅️standards/**
[cargo-operation] pattern-segment 5295677 units 6033122 retained bytes **/🏅️standards/**


Expected: 0
Received: null

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:148:40)
(fail) process budgets separate coverage compilation from assertions: nextest=true [5018.64ms]
143 |       }));
144 |       const { runRepositoryCargoTests } = await import(${JSON.stringify(libraryPath)});
```

## (fail) process budgets separate coverage compilation from assertions: nextest=false [5013.01ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

 5 | const yieldMarks=new WeakMap<CargoAccounting,number>();
 6 | 
 7 | /** 🎛️ Accounts before ownership and yields at bounded work frontiers. */
 8 | export class CargoController {
 9 |  constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
10 |  check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
                                                           ^
error: Cargo operation cancelled
      at check (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:10:55)
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:728)

Bun v1.3.14 (macOS arm64)


Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:148:40)
(fail) process budgets separate coverage compilation from assertions: nextest=false [5013.01ms]
(pass) process budgets enforce an explicitly selected build deadline [302.39ms]
killed 1 dangling process
```

## (fail) process runner reaches workspace scripts before same-named bins [60001.64ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

 5 | const yieldMarks=new WeakMap<CargoAccounting,number>();
 6 | 
 7 | /** 🎛️ Accounts before ownership and yields at bounded work frontiers. */
 8 | export class CargoController {
 9 |  constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
10 |  check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
                                                           ^
error: Cargo operation cancelled
      at check (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:10:55)
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:728)

Bun v1.3.14 (macOS arm64)


Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:148:40)
(fail) process budgets separate coverage compilation from assertions: nextest=false [5013.01ms]
(pass) process budgets enforce an explicitly selected build deadline [302.39ms]
killed 1 dangling process
(fail) process runner reaches workspace scripts before same-named bins [60001.64ms]
  ^ this test timed out after 60000ms.
171 | 
```

## (fail) process budgets bind nested native workspace profiles to the repository config [12033.63ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

 5 | const yieldMarks=new WeakMap<CargoAccounting,number>();
 6 | 
 7 | /** 🎛️ Accounts before ownership and yields at bounded work frontiers. */
 8 | export class CargoController {
 9 |  constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
10 |  check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
                                                           ^
error: Cargo operation cancelled
      at check (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:10:55)
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:74)
      at cargoPhysicalPath (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📁️physical/🟦️.ts:48:761)

Bun v1.3.14 (macOS arm64)

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:176:30)
(fail) process budgets bind nested native workspace profiles to the repository config [12033.63ms]
15 | 
16 | /** 🧩️ Preserves nested and concurrent owner identity across awaited continuations. */
```

## (fail) process native profiles belong to each physical Cargo workspace [21970.52ms]

```text
  discoverySignal: [AbortController ...],
  retirementSignal: [AbortController ...],
  retirement: [Object ...],
  lastProgress: 1791474029920,
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

 6 | 
 7 | /** 🎛️ Accounts before ownership and yields at bounded work frontiers. */
 8 | export class CargoController {
 9 |  constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
10 |  check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
11 |  async step(stage:string,path:string,units=1,bytes=0):Promise<void>{this.check();if(!Number.isSafeInteger(units)||units<0||!Number.isSafeInteger(bytes)||bytes<0)throw Error("Cargo accounting requires finite nonnegative admissions");if(units>this.operation.maximumUnits-this.accounting.completed||bytes>this.operation.maximumOwnedBytes-this.accounting.ownedBytes)throw Error("Cargo operation budget refused");this.accounting.completed+=units;this.accounting.ownedBytes+=bytes;if(this.accounting.completed-yieldMarks.get(this.accounting)!>=128||units===0){this.operation.onProgress({stage,path,completed:this.accounting.completed,ownedBytes:this.accounting.ownedBytes});this.check();await this.operation.yieldContinuation();this.check();yieldMarks.set(this.accounting,this.accounting.completed);}}

error: Cargo operation budget refused
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:369)
      at cargoPhysicalDirectory (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📁️physical/🟦️.ts:58:341)
(fail) process native profiles belong to each physical Cargo workspace [21970.52ms]

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts:
```

## (fail) every normalizer pattern query has a required invocation-owned matcher and fresh load [43.87ms]

```text

error: Cargo operation budget refused
      at step (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts:11:369)
      at cargoPhysicalDirectory (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📁️physical/🟦️.ts:58:341)
(fail) process native profiles belong to each physical Cargo workspace [21970.52ms]

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts:
(pass) pattern compilation reuse has a closed language-neutral schema and independent JSON parser [29.35ms]
(pass) actual current pattern semantics agree with both compilers and the independent glob oracle [55.13ms]
(pass) one query matcher compiles each normalized pattern once without memoizing path results [14.78ms]
(pass) interleaved matcher sessions do not share compiled patterns or retained query state [24.28ms]
(pass) invalid patterns retain lazy fresh errors without poisoning a matcher [12.62ms]
(pass) the matcher exposes no mutable regex state and repeated tests remain stateless [5.92ms]
156 |     expect(call.expression.getText(normalizer)).toBe(["validatedContractPattern", "parseTaxonomy", "renderCatalogGlob"].includes(name) ? "pathMatcher.matches" : "taxonomy.pathMatcher.matches");
157 |     owners[name] = (owners[name] ?? 0) + 1;
158 |   }
159 |   expect(owners).toEqual(vector.integration.normalizerOwners);
160 |   expect(calls.filter((node) => node.expression.getText(normalizer) === "createTaxonomyPathMatcher").map(owner).sort()).toEqual(["loadTaxonomy", "parseTaxonomy", "renderCatalogGlob"]);
161 |   expect(calls.filter((node) => node.expression.getText(normalizer) === "loadTaxonomy").map(owner)).toEqual(vector.integration.loadOwners);
                                                                                                          ^
error: expect(received).toEqual(expected)

  [
+   "admitTaxonomyCargoMembershipFacts",
    "inventoryTaxonomyWithSourceParentPruning",
    "planTaxonomy",
+   "verifyTaxonomyScopes",
    "applyTaxonomyPlan",
    "inventoryTaxonomySources",
  ]

- Expected  - 0
+ Received  + 2

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts:161:101)
(fail) every normalizer pattern query has a required invocation-owned matcher and fresh load [43.87ms]
(pass) actual load caches content facts while capturing fresh isolated schema and matcher sessions: bun [1602.89ms]
(pass) actual load caches content facts while capturing fresh isolated schema and matcher sessions: typescript [564.69ms]
```

## (fail) registers pattern compiler reuse through its closed canonical route [2.86ms]

```text
    "inventoryTaxonomySources",
  ]

- Expected  - 0
+ Received  + 2

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts:161:101)
(fail) every normalizer pattern query has a required invocation-owned matcher and fresh load [43.87ms]
(pass) actual load caches content facts while capturing fresh isolated schema and matcher sessions: bun [1602.89ms]
(pass) actual load caches content facts while capturing fresh isolated schema and matcher sessions: typescript [564.69ms]
(pass) the owned factory interface and actual declarations satisfy strict TypeScript [337.45ms]
(pass) changed runtime matcher declarations and call sites are strictly typed: 🧹️normalization/🛣️path/🟦️.ts [961.59ms]
(pass) changed runtime matcher declarations and call sites are strictly typed: 🧹️normalization/📁️input/🟦️.ts [31.88ms]
(pass) changed runtime matcher declarations and call sites are strictly typed: 🧹️normalization/🏃️operation/🟦️.ts [3.69ms]
(pass) changed runtime matcher declarations and call sites are strictly typed: 🧹️normalization/🔣️taxonomy/🟦️.ts [243.26ms]
(pass) changed runtime matcher declarations and call sites are strictly typed: 🧹️normalization/🚪️source-admission/🟦️.ts [59.46ms]
(pass) changed runtime matcher declarations and call sites are strictly typed: 🧹️normalization/🚪️source-admission/📁️io/🟦️.ts [66.57ms]
(pass) changed runtime matcher declarations and call sites are strictly typed: 🧾️serialization/🔣️json/🟦️.ts [5.46ms]
(pass) changed runtime matcher declarations and call sites are strictly typed: ../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts [20.79ms]
(pass) changed runtime matcher declarations and call sites are strictly typed: 🔍️discovery/🟦️.ts [1861.13ms]
(pass) changed runtime matcher declarations and call sites are strictly typed: 🧹️normalization/🟦️.ts [2306.14ms]
291 |   expect(selected.map((diagnostic) => `${relative(library, path)}:${file.getLineAndCharacterOfPosition(diagnostic.start ?? 0).line + 1} TS${diagnostic.code} ${ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")}`)).toEqual([]);
292 | });
293 | 
294 | test("registers pattern compiler reuse through its closed canonical route", async () => {
295 |   const directory = join(import.meta.dir, "../../🧫️fixtures/♻️taxonomy-pattern-compiler-reuse/🧪️registration"), bytes = readFileSync(join(directory, "🔣️.json"), "utf8"), registration = JSON.parse(bytes);
296 |   const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json"), "utf8")));
                                                                                       ^
ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json'
    path: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json",
 syscall: "open",
   errno: -2,
    code: "ENOENT"

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts:296:82)
(fail) registers pattern compiler reuse through its closed canonical route [2.86ms]

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts:
```

## (fail) artifact support leaf authority > projects only the three physically evidenced support leaves [454.45ms]

```text
  cancelDiscovery: [Function: cancelDiscovery],
  retirementAborted: [Getter],
  cancelRetirement: [Function: cancelRetirement],
  detachSignals: [Function: detachSignals],
  closeSystemOwners: [AsyncFunction: closeSystemOwners],
},
 cleanupError: null,

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

249 |   }));
250 | 
251 |   test("projects only the three physically evidenced support leaves",()=>runCargoCliCommand(normalizationProbeOwner(),async()=>{
252 |     const root = fixture();
253 |     const inventory = (await runCargoCliCommand(normalizationProbeOwner(),async()=>inventoryTaxonomy(await admitTaxonomyInventoryOptions({ repoRoot: root, scope: vector.owner, workers: 1 },cargoCommandOwner().operation,normalizationProbeProcess(cargoCommandOwner())))));
254 |     expect(inventory.violations).toEqual([]);
                                       ^
error: expect(received).toEqual(expected)

- []
+ [
+   {
+     "code": "directory-kind-unresolved",
+     "message": "Directory has no registered semantic kind",
+     "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️simulation-session/🧪️fixture",
+     "severity": "error",
+   },
+ ]

- Expected  - 1
+ Received  + 8

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts:254:34)
      at async runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at async runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) artifact support leaf authority > projects only the three physically evidenced support leaves [454.45ms]
(pass) artifact support leaf authority > rejects unproven payload ownership: missing-descriptor [251.60ms]
(pass) artifact support leaf authority > rejects unproven payload ownership: wrong-descriptor-pointer [181.34ms]
```

## (fail) artifact support leaf authority > preserves JSDoc and runtime readers through rollback, retry, Nx generation and an empty plan [567.66ms]

```text
  cancelDiscovery: [Function: cancelDiscovery],
  retirementAborted: [Getter],
  cancelRetirement: [Function: cancelRetirement],
  detachSignals: [Function: detachSignals],
  closeSystemOwners: [AsyncFunction: closeSystemOwners],
},
 cleanupError: null,

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

297 |   test("preserves JSDoc and runtime readers through rollback, retry, Nx generation and an empty plan",()=>runCargoCliCommand(normalizationProbeOwner(),async()=>{
298 |     const row = lifecycleFixture();
299 |     const options = { repoRoot: row.root, scope: vector.owner, ticketDir: row.ticketDir, workers: 1 };
300 |     const inventory = (await runCargoCliCommand(normalizationProbeOwner(),async()=>inventoryTaxonomy(await admitTaxonomyInventoryOptions(options,cargoCommandOwner().operation,normalizationProbeProcess(cargoCommandOwner())))));
301 |     const plan = (await runCargoCliCommand(normalizationProbeOwner(),async()=>planTaxonomy(inventory,{...{ baselineCommit: row.baselineCommit, excludedTreeDigests: [] },cargoOperation:cargoCommandOwner().operation})));
302 |     expect(plan.unresolved).toEqual([]);
                                  ^
error: expect(received).toEqual(expected)

- []
+ [
+   {
+     "code": "directory-kind-unresolved",
+     "message": "Directory has no registered semantic kind",
+     "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️simulation-session/🧪️fixture",
+     "severity": "error",
+   },
+ ]

- Expected  - 1
+ Received  + 8

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts:302:29)
      at async runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at async runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) artifact support leaf authority > preserves JSDoc and runtime readers through rollback, retry, Nx generation and an empty plan [567.66ms]
15 | 
16 | /** 🧩️ Preserves nested and concurrent owner identity across awaited continuations. */
```

## (fail) artifact support leaf authority > plans the complete real Energy owner with exact Cargo mounting context [2937.49ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

336 | 
337 |   test("plans the complete real Energy owner with exact Cargo mounting context",()=>runCargoCliCommand(normalizationProbeOwner(),async()=>{
338 |     const row = lifecycleFixture();
339 |     const expected = vector.ownerReadiness;
340 |     const entries = ownedFilesystemEntries(join(repoRoot, vector.owner), true);
341 |     expect(entries).toHaveLength(expected.physicalNodes);
                          ^
error: expect(received).toHaveLength(expected)

Expected length: 329
Received length: 12452

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts:341:21)
      at run (node:async_hooks:62:22)
      at runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) artifact support leaf authority > plans the complete real Energy owner with exact Cargo mounting context [2937.49ms]

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts:
```

## (fail) mutation ticket role routing reaches only the mocked N admission boundary [395.09ms]

```text
      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

336 | 
337 |   test("plans the complete real Energy owner with exact Cargo mounting context",()=>runCargoCliCommand(normalizationProbeOwner(),async()=>{
338 |     const row = lifecycleFixture();
339 |     const expected = vector.ownerReadiness;
340 |     const entries = ownedFilesystemEntries(join(repoRoot, vector.owner), true);
341 |     expect(entries).toHaveLength(expected.physicalNodes);
                          ^
error: expect(received).toHaveLength(expected)

Expected length: 329
Received length: 12452

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts:341:21)
      at run (node:async_hooks:62:22)
      at runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) artifact support leaf authority > plans the complete real Energy owner with exact Cargo mounting context [2937.49ms]

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts:
(pass) mutation ticket role routing vectors are closed and every field participates [5.97ms]
110 |     `console.log(${JSON.stringify(marker)} + JSON.stringify({ calls, exports: Object.keys(family) }));`,
111 |   ].join("\n");
112 |   const child = Bun.spawnSync([process.execPath, "-e", source], { cwd: root, stdout: "pipe", stderr: "pipe", timeout: 10_000 });
113 |   const stdout = new TextDecoder().decode(child.stdout), stderr = new TextDecoder().decode(child.stderr);
114 |   expect(child.signalCode ?? null).toBeNull();
115 |   expect(child.exitCode).toBe(0);
                               ^
error: expect(received).toBe(expected)

Expected: 0
Received: 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts:115:26)
(fail) mutation ticket role routing reaches only the mocked N admission boundary [395.09ms]

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts:
```

## (fail) genuine census retains exact bytes and five typed historical coordinates [6.81ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

65 |     }
66 |   }
67 | }));
68 | 
69 | test("genuine census retains exact bytes and five typed historical coordinates",()=>runCargoCliCommand(normalizationProbeOwner(),async()=>{
70 |   expect(sha(goldenBytes)).toBe(vector.historicalDocument.sha256);
                                ^
error: expect(received).toBe(expected)

Expected: "c3e39ed5122709c5f963f14a0bc7f53b93ec7caad414a9c6db39bc5c19ce5be9"
Received: "fb3b44bb29bcd60534945f70515d6d92bd226ebc64bc3af6a3bb17ee1454d79e"

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts:70:28)
      at run (node:async_hooks:62:22)
      at runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) genuine census retains exact bytes and five typed historical coordinates [6.81ms]
(pass) genuine historical purity authority is registered with no broader span ownership than the deliberately widened set [0.37ms]
(pass) historical owner identity gate is registered in Nx [10.90ms]
```

## (fail) current isolated census follows moved added and retired files without reopening historical source paths [104.78ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

123 |     expect(git.exitCode, git.stderr.toString()).toBe(0);
124 |     const admitted = new Set(git.stdout.toString().split("\0").filter(Boolean));
125 |     const current = () => { clearDiscoveryCache(); return discoverPackageProblems(root, taxonomy).filter((row) => admitted.has(row.path) && (row.kind === "package-implementation" || row.kind === "package-role-unresolved")).map((row) => row.path).sort(); };
126 |     expect(current()).toEqual([...expected].sort());
127 |     expect(current()).toEqual(fastGlob.sync("**/📦️packages/**/*.rs", { cwd: root, onlyFiles: true, dot: true, followSymbolicLinks: false }).sort());
128 |     expect(sha(readFileSync(goldenPath))).toBe(vector.historicalDocument.sha256);
                                                ^
error: expect(received).toBe(expected)

Expected: "c3e39ed5122709c5f963f14a0bc7f53b93ec7caad414a9c6db39bc5c19ce5be9"
Received: "fb3b44bb29bcd60534945f70515d6d92bd226ebc64bc3af6a3bb17ee1454d79e"

      at observe (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts:128:43)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts:133:3)
      at run (node:async_hooks:62:22)
      at runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) current isolated census follows moved added and retired files without reopening historical source paths [104.78ms]
(pass) a frozen package-owner census naming moved Draw files stays byte-identical through a scoped Draw transaction [6639.31ms]
(pass) Energy historical source coordinates > register exactly ten additions without changing the eight historical authorities [9.01ms]
```

## (fail) new permanent reviewed fixture directories use current registered canonical kinds [28.06ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

199 | test("new permanent reviewed fixture directories use current registered canonical kinds",()=>runCargoCliCommand(normalizationProbeOwner(),async()=>{
200 |   const taxonomy = JSON.parse(capture("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json").bytes.toString("utf8"));
201 |   const discovery = await import("../../🔍️discovery/🟦️.ts"), normalization = await import("../../🧹️normalization/🟦️.ts");
202 |   for (const path of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts"]) capture(path);
203 |   const cases = [...vector.directoryCases, ...manifest.inputs.map((row: any) => ({ path: posix.dirname(row.path), kind: "fixture-case", parent: "fixtures" }))];
204 |   for (const row of cases) expect(discovery.semanticDirectoryKindId(posix.basename(row.path), taxonomy, { parentKindId: row.parent }), row.path).toBe(row.kind);
                                                                                                                                                       ^
error: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs

Expected: "test-case"
Received: null

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:204:146)
      at async runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at async runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) new permanent reviewed fixture directories use current registered canonical kinds [28.06ms]
(pass) reviewed fixture inputs retain exact approved source and expectation bytes [8.12ms]
(pass) reviewed fixture input identity rejects changed missing and symbolic nodes [8.55ms]
```

## (fail) repeated reviewed input capture rejects a deliberate same-path change [5.74ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

20 | const observations = new Map<string, { bytes: Buffer; sha256: string; size: number; mode: number }>(), outcomes: any[] = [];
21 | let owner: string | undefined;
22 | 
23 | /** 🛡️ Rejects unsafe and opaque coordinates before observing any node. */
24 | function safe(path: string): string {
25 |   if (!path || path !== path.normalize("NFC") || /[\\:*?"<>|\u0000-\u001f]/u.test(path) || Buffer.from(path).toString("utf8") !== path || path.split("/").some((part) => !part || part === "." || part === "..") || /^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) throw new Error("Unsafe reviewed fixture coordinate");
                                                                                                                                                                                                                                                                                     ^
error: Unsafe reviewed fixture coordinate
      at safe (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:25:273)
      at input (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:31:17)
      at capture (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:57:17)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:251:17)
      at run (node:async_hooks:62:22)
      at runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) repeated reviewed input capture rejects a deliberate same-path change [5.74ms]
15 | 
16 | /** 🧩️ Preserves nested and concurrent owner identity across awaited continuations. */
```

## (fail) owned reviewed child timeout and output-limit terminate their descendants [700.32ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

127 |   try { process.kill(pid, "SIGKILL"); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ESRCH") throw error; }
128 | }
129 | 
130 | /** 📭️ Proves the owned PID or process group has reached its terminal state. */
131 | function processAbsent(pid: number, group = false): boolean {
132 |   try { process.kill(group && process.platform !== "win32" ? -pid : pid, 0); return false; }
                      ^
SystemError: kill() failed: EPERM: Operation not permitted
 syscall: "kill",
   errno: 1,
    code: "EPERM"

      at processAbsent (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:132:17)
      at ownedRun (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:167:53)
(fail) owned reviewed child timeout and output-limit terminate their descendants [700.32ms]
15 | 
16 | /** 🧩️ Preserves nested and concurrent owner identity across awaited continuations. */
```

## (fail) the unchanged copied revision gate runs with absent raw source and edited canonical source [18.40ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

32 |   let current = repo;
33 |   const anchor = lstatSync(repo), ancestors = [{ path: repo, dev: anchor.dev, ino: anchor.ino }];
34 |   if (!anchor.isDirectory() || anchor.isSymbolicLink()) throw new Error("Unsafe reviewed fixture root");
35 |   for (const [index, part] of parts.entries()) {
36 |     current = join(current, part);
37 |     const node = lstatSync(current);
                      ^
ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🟢️readme-current-source-activation'
    path: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🟢️readme-current-source-activation",
 syscall: "lstat",
   errno: -2,
    code: "ENOENT"

      at input (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:37:18)
      at capture (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:57:17)
      at isolatedRepository (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:283:45)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:293:31)
      at run (node:async_hooks:62:22)
      at runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) the unchanged copied revision gate runs with absent raw source and edited canonical source [18.40ms]
15 | 
16 | /** 🧩️ Preserves nested and concurrent owner identity across awaited continuations. */
```

## (fail) the unchanged activation module validates a schema snapshot without live historical inputs [10.41ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

32 |   let current = repo;
33 |   const anchor = lstatSync(repo), ancestors = [{ path: repo, dev: anchor.dev, ino: anchor.ino }];
34 |   if (!anchor.isDirectory() || anchor.isSymbolicLink()) throw new Error("Unsafe reviewed fixture root");
35 |   for (const [index, part] of parts.entries()) {
36 |     current = join(current, part);
37 |     const node = lstatSync(current);
                      ^
ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🟢️readme-current-source-activation'
    path: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🟢️readme-current-source-activation",
 syscall: "lstat",
   errno: -2,
    code: "ENOENT"

      at input (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:37:18)
      at capture (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:57:17)
      at isolatedRepository (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:283:45)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:312:31)
      at run (node:async_hooks:62:22)
      at runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) the unchanged activation module validates a schema snapshot without live historical inputs [10.41ms]
15 | 
16 | /** 🧩️ Preserves nested and concurrent owner identity across awaited continuations. */
```

## (fail) reviewed fixture gate registration matches its package route [21.69ms]

```text
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

336 |     if (ts.isIfStatement(node) && ts.isBinaryExpression(node.expression) && node.expression.operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken && node.expression.left.getText(tree) === "segments[0]" && ts.isStringLiteral(node.expression.right) && node.expression.right.text === expected.route) branches.push(node);
337 |     ts.forEachChild(node, visit);
338 |   };
339 |   visit(tree);
340 |   expect({ packageName: packageManifest.name, packageCommand: packageManifest.scripts?.[expected.target], target: project.targets[expected.target], branches: branches.length }).toEqual({ packageName: expected.packageName, packageCommand: expected.packageCommand, target: { executor: "nx:run-commands", options: { cwd: packagePath, command: expected.command } }, branches: 1 });
341 |   expect(branches[0]!.thenStatement.getText(tree)).toContain("join(this.repoRoot, " + JSON.stringify(expected.source) + ")");
                                                         ^
error: expect(received).toContain(expected)

Expected to contain: "join(this.repoRoot, \"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟦️readme-reviewed-fixture-inputs.ts\")"
Received: "{\n      const source = join(this.repoRoot, \"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts\");\n      await runRepositoryTestCommand(process.execPath, [\"test\", source, ...segments.slice(1)], { cwd: this.repoRoot });\n      return;\n    }"

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:341:52)
      at run (node:async_hooks:62:22)
      at runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) reviewed fixture gate registration matches its package route [21.69ms]

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💠️inventory-artifact-shards/🟦️.ts:
```

## (fail) ticket narrative, evidence and scratch never block a move, a production file still does, and a ticket-embedded package boundary is never swept in [642.48ms]

```text
  publish: [Function: publish],
  advanceWorkspace: [AsyncFunction: advanceWorkspace],
  cancelDiscovery: [Function: cancelDiscovery],
  retirementAborted: [Getter],
  cancelRetirement: [Function: cancelRetirement],
  detachSignals: [Function: detachSignals],
  closeSystemOwners: [AsyncFunction: closeSystemOwners],
},
 cleanupError: null,

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

134 |   git("-c", "user.name=Histref Fixture", "-c", "user.email=fixture@invalid.example", "-c", "commit.gpgsign=false", "commit", "-qm", "Histref fixture");
135 |   const baselineCommit = git("rev-parse", "HEAD");
136 | 
137 |   const plan = (await runCargoCliCommand(normalizationProbeOwner(),async()=>normalization.planTaxonomy((await runCargoCliCommand(normalizationProbeOwner(),async()=>normalization.inventoryTaxonomy(await admitTaxonomyInventoryOptions({ repoRoot, scope, workers: 1 },cargoCommandOwner().operation,normalizationProbeProcess(cargoCommandOwner()))))),{...{ baselineCommit, excludedTreeDigests: [] },cargoOperation:cargoCommandOwner().operation})));
138 | 
139 |   expect(plan.moves.map((move) => [move.sourcePath, move.destinationPath])).toEqual([[source, final]]);
                                                                                  ^
error: expect(received).toEqual(expected)

- [
-   [
-     "🧪️tests/🧪️fixture/🦀️.rs",
-     "🧪️tests/🧪️fixture/🦀️.rs",
-   ],
- ]
+ []

- Expected  - 6
+ Received  + 1

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📜️historical-document-evidence/🟦️.ts:139:77)
      at async runCargoCommandScope (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:17:168)
      at async runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:203)
(fail) ticket narrative, evidence and scratch never block a move, a production file still does, and a ticket-embedded package boundary is never swept in [642.48ms]
(pass) the exemption is driven entirely by the schema population, not hardcoded: an empty population map reproduces the pre-change blocked state for the exact same paths [12.12ms]
(pass) dev-prompt-log honors both existing negatives: a fixedFilenameContracts match under 💬️prompts/ is never exempted, and neither is a path inside a hypothetical package embedded directly under it [16.49ms]
```

## (fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]

```text
  retirement: [Object ...],
  lastProgress: -Infinity,
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

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

354 | }
355 | 
356 | function assertChildWaiting(childProcess: ChildProcess | undefined, path: string): void {
357 |   if (!childProcess) return;
358 |   const evidence = childEvidence.get(childProcess);
359 |   if (evidence?.error || childProcess.exitCode !== null || childProcess.signalCode !== null) throw new Error(`Child ended before marker ${path}: ${evidence?.error?.message ?? childProcess.signalCode ?? childProcess.exitCode}\n${evidence?.output ?? ""}`);
                                                                                                             ^
error: Child ended before marker /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/controlled-cargo-current-25/🧾️runs/🔖️86de557b-24f2-4f50-b075-aa26fcd9904e/🧪️strict-plan-j8Qtl9/missing-marker: 7
deliberate fixture child failure

      at assertChildWaiting (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:359:104)
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
```

## (fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]

```text
  retirementAborted: [Getter],
  cancelRetirement: [Function: cancelRetirement],
  detachSignals: [Function: detachSignals],
  closeSystemOwners: [AsyncFunction: closeSystemOwners],
},
 cleanupError: null,

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

354 | }
355 | 
356 | function assertChildWaiting(childProcess: ChildProcess | undefined, path: string): void {
357 |   if (!childProcess) return;
358 |   const evidence = childEvidence.get(childProcess);
359 |   if (evidence?.error || childProcess.exitCode !== null || childProcess.signalCode !== null) throw new Error(`Child ended before marker ${path}: ${evidence?.error?.message ?? childProcess.signalCode ?? childProcess.exitCode}\n${evidence?.output ?? ""}`);
                                                                                                             ^
error: Child ended before marker /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/controlled-cargo-current-25/🧾️runs/🔖️86de557b-24f2-4f50-b075-aa26fcd9904e/🧪️strict-plan-j8Qtl9/missing-marker: 7
deliberate fixture child failure

      at assertChildWaiting (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:359:104)
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
```

## (fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]

```text
},
 cleanupError: null,

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

354 | }
355 | 
356 | function assertChildWaiting(childProcess: ChildProcess | undefined, path: string): void {
357 |   if (!childProcess) return;
358 |   const evidence = childEvidence.get(childProcess);
359 |   if (evidence?.error || childProcess.exitCode !== null || childProcess.signalCode !== null) throw new Error(`Child ended before marker ${path}: ${evidence?.error?.message ?? childProcess.signalCode ?? childProcess.exitCode}\n${evidence?.output ?? ""}`);
                                                                                                             ^
error: Child ended before marker /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/controlled-cargo-current-25/🧾️runs/🔖️86de557b-24f2-4f50-b075-aa26fcd9904e/🧪️strict-plan-j8Qtl9/missing-marker: 7
deliberate fixture child failure

      at assertChildWaiting (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:359:104)
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
```

## (fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]

```text

      at runCargoCliCommand (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️scope/🟦️.ts:20:430)

354 | }
355 | 
356 | function assertChildWaiting(childProcess: ChildProcess | undefined, path: string): void {
357 |   if (!childProcess) return;
358 |   const evidence = childEvidence.get(childProcess);
359 |   if (evidence?.error || childProcess.exitCode !== null || childProcess.signalCode !== null) throw new Error(`Child ended before marker ${path}: ${evidence?.error?.message ?? childProcess.signalCode ?? childProcess.exitCode}\n${evidence?.output ?? ""}`);
                                                                                                             ^
error: Child ended before marker /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/controlled-cargo-current-25/🧾️runs/🔖️86de557b-24f2-4f50-b075-aa26fcd9904e/🧪️strict-plan-j8Qtl9/missing-marker: 7
deliberate fixture child failure

      at assertChildWaiting (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:359:104)
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
```

## (fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]

```text

354 | }
355 | 
356 | function assertChildWaiting(childProcess: ChildProcess | undefined, path: string): void {
357 |   if (!childProcess) return;
358 |   const evidence = childEvidence.get(childProcess);
359 |   if (evidence?.error || childProcess.exitCode !== null || childProcess.signalCode !== null) throw new Error(`Child ended before marker ${path}: ${evidence?.error?.message ?? childProcess.signalCode ?? childProcess.exitCode}\n${evidence?.output ?? ""}`);
                                                                                                             ^
error: Child ended before marker /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/controlled-cargo-current-25/🧾️runs/🔖️86de557b-24f2-4f50-b075-aa26fcd9904e/🧪️strict-plan-j8Qtl9/missing-marker: 7
deliberate fixture child failure

      at assertChildWaiting (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:359:104)
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
```

## (fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]

```text
355 | 
356 | function assertChildWaiting(childProcess: ChildProcess | undefined, path: string): void {
357 |   if (!childProcess) return;
358 |   const evidence = childEvidence.get(childProcess);
359 |   if (evidence?.error || childProcess.exitCode !== null || childProcess.signalCode !== null) throw new Error(`Child ended before marker ${path}: ${evidence?.error?.message ?? childProcess.signalCode ?? childProcess.exitCode}\n${evidence?.output ?? ""}`);
                                                                                                             ^
error: Child ended before marker /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/controlled-cargo-current-25/🧾️runs/🔖️86de557b-24f2-4f50-b075-aa26fcd9904e/🧪️strict-plan-j8Qtl9/missing-marker: 7
deliberate fixture child failure

      at assertChildWaiting (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:359:104)
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]

```text
357 |   if (!childProcess) return;
358 |   const evidence = childEvidence.get(childProcess);
359 |   if (evidence?.error || childProcess.exitCode !== null || childProcess.signalCode !== null) throw new Error(`Child ended before marker ${path}: ${evidence?.error?.message ?? childProcess.signalCode ?? childProcess.exitCode}\n${evidence?.output ?? ""}`);
                                                                                                             ^
error: Child ended before marker /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/controlled-cargo-current-25/🧾️runs/🔖️86de557b-24f2-4f50-b075-aa26fcd9904e/🧪️strict-plan-j8Qtl9/missing-marker: 7
deliberate fixture child failure

      at assertChildWaiting (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:359:104)
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]

```text
359 |   if (evidence?.error || childProcess.exitCode !== null || childProcess.signalCode !== null) throw new Error(`Child ended before marker ${path}: ${evidence?.error?.message ?? childProcess.signalCode ?? childProcess.exitCode}\n${evidence?.output ?? ""}`);
                                                                                                             ^
error: Child ended before marker /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/controlled-cargo-current-25/🧾️runs/🔖️86de557b-24f2-4f50-b075-aa26fcd9904e/🧪️strict-plan-j8Qtl9/missing-marker: 7
deliberate fixture child failure

      at assertChildWaiting (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:359:104)
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]

```text
error: Child ended before marker /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/controlled-cargo-current-25/🧾️runs/🔖️86de557b-24f2-4f50-b075-aa26fcd9904e/🧪️strict-plan-j8Qtl9/missing-marker: 7
deliberate fixture child failure

      at assertChildWaiting (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:359:104)
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]

```text

      at assertChildWaiting (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:359:104)
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]

```text
      at waitFor (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:365:5)
(fail) transaction plan journal v2 aggregate > rejects incomplete plans and plan-digest drift [813.65ms]
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]

```text
772 | 
773 |   for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, async () => {
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]

```text
774 |     const row = failure.fixture(`failure-${failure.stage}`);
775 |     try {
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]

```text
776 |       const value = (await plan(row)), before = snapshot(row.workspace);
777 |       expect(failure.relevant(value)).toBeGreaterThan(0);
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]

```text
                                            ^
error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]

```text

Expected: > 0
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]

```text
Received: 0

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]

```text
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:777:39)
(fail) transaction plan journal v2 aggregate > rolls back after-regenerations with non-empty phase authority [5545.17ms]
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]

```text
(pass) transaction plan journal v2 aggregate > rolls back after-staging with non-empty phase authority [10797.71ms]
(pass) transaction plan journal v2 aggregate > rolls back after-edits with non-empty phase authority [9874.54ms]
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]

```text
(pass) transaction plan journal v2 aggregate > rolls back after-moves with non-empty phase authority [10365.47ms]
(fail) transaction plan journal v2 aggregate > rolls back after-embedded-root-staging with non-empty phase authority [16091.84ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-relocations with non-empty phase authority [15626.91ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back after-symlink-retargeting with non-empty phase authority [15598.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rolls back before-verify with non-empty phase authority [15257.35ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-mkdir [15238.21ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-preparation-children [15221.10ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-write-mkdir [15206.26ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-candidate-written [15192.58ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-json-canonical-exchanged [15178.06ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-lease-prepared [15162.82ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-wal-mkdir [15151.92ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mid [15772.36ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mid [15772.36ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-write-mkdir [15136.44ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mid [15772.36ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-prepared [15751.54ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-prepared [15751.54ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-candidate-written [15111.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mid [15772.36ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-prepared [15751.54ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-inner-exchange [15728.22ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-inner-exchange [15728.22ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical-exchanged [15040.49ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mid [15772.36ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-prepared [15751.54ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-inner-exchange [15728.22ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-exchange [15703.78ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-exchange [15703.78ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-initial-journal-canonical [15029.48ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mid [15772.36ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-prepared [15751.54ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-inner-exchange [15728.22ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-exchange [15703.78ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-canonical-exchange [15682.93ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-canonical-exchange [15682.93ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-attempt-canonical-published [15015.07ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mid [15772.36ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-prepared [15751.54ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-inner-exchange [15728.22ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-exchange [15703.78ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-canonical-exchange [15682.93ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-mkdir [15660.78ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-mkdir [15660.78ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-write-mkdir [15076.52ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mid [15772.36ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-prepared [15751.54ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-inner-exchange [15728.22ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-exchange [15703.78ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-canonical-exchange [15682.93ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-mkdir [15660.78ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-prepared [15641.47ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-prepared [15641.47ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-candidate-written [15067.51ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mid [15772.36ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-prepared [15751.54ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-inner-exchange [15728.22ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-exchange [15703.78ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-canonical-exchange [15682.93ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-mkdir [15660.78ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-prepared [15641.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-exchange [15616.32ms]
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-exchange [15616.32ms]

```text
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-previous-exchanged [15040.34ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-journal-canonical-exchanged [15014.83ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-wal-prepared [15211.02ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mkdir [15185.12ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-mid [15161.39ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-write-prepared [15891.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-inner-exchange [15870.67ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-exchange [15848.88ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-backup-retained [15826.98ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mkdir [15801.70ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-mid [15772.36ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-write-prepared [15751.54ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-inner-exchange [15728.22ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-exchange [15703.78ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-edit-canonical-exchange [15682.93ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-mkdir [15660.78ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-prepared [15641.47ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-exchange [15616.32ms]
  ^ this test timed out after 15000ms.
848 | 
```

## (fail) transaction plan journal v2 aggregate > rolls back a process-tree-killed mixed generator and commits ordinal two [10334.57ms]

```text
      at registryCatalogGitlinkBoundaries (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9949:20)
(pass) transaction plan journal v2 aggregate > restores a quarantined stale lease exactly when acquisition callback throws [5133.25ms]
(pass) transaction plan journal v2 aggregate > rejects stale resume preimage and malformed resume evidence byte-for-byte [260.05ms]
964 |     const generator = generatorFixture("stale-generator"), embedded = embeddedFixture("stale-reference");
965 |     try {
966 |       const generatorPlan = (await plan(generator));
967 |       writeFileSync(join(generator.workspace, "🧪️generator", "./🟦️.ts"), "export const input = false;\n");
968 |       const generatorTransaction = snapshot(transactionRoot(generator)), generatorWorkspace = snapshot(generator.workspace);
969 |       (await expect((async () => (await runCargoCliCommand(normalizationProbeOwner(),async()=>applyTaxonomyPlan(generatorPlan,{...{ repoRoot: generator.repoRoot, ticketDir: generator.ticketDir, expectedBaselineCommit: generator.baselineCommit },cargoOwner:cargoCommandOwner(),process:normalizationProbeProcess(cargoCommandOwner())}))))()).rejects.toThrow(/Regeneration input preimage changed/u));
                                                                                                                                                                                                                                                                                                                                                                 ^
error: expect(received).toThrow(expected)

Expected pattern: /Regeneration input preimage changed/u
Received message: "Cargo command refused with retained ownership: Plan has unresolved blocking violations"

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:969:348)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:996:405)
      at spawnSync (node:child_process:226:22)
      at execFileSync (node:child_process:264:54)
      at registryCatalogGitlinkBoundaries (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9949:20)
(pass) transaction plan journal v2 aggregate > rejects ordinal collisions and malformed attempt siblings without mutation [791.18ms]
877 |       const barrier = mkdtempSync(join(row.root, "barrier-")), acquireRelease = join(barrier, "acquire"), winnerRelease = join(barrier, "winner"), owner = join(barrier, "owner");
878 |       const contenderSource = "const [m,p,r,t,baseline,j,b,acquire,winner,owner]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(m);const {normalizationProbeOwner}=await import(require(\"node:path\").resolve(require(\"node:path\").dirname(m),\"../🧪️tests/🎛️normalization-owner/🟦️.ts\"));const {runCargoCliCommand}=await import(require(\"node:path\").resolve(require(\"node:path\").dirname(m),\"../🗂️workspaces/🦀️cargo/🟦️.ts\"));const commandOwner=normalizationProbeOwner();const fs=require(\"node:fs\"),wait=(path)=>{while(!fs.existsSync(path))Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,2)},result=b+\"/result-\"+process.pid;try{await runCargoCliCommand(commandOwner,async()=>applyTaxonomyPlan(JSON.parse(await Bun.file(p).text()),{...{repoRoot:r,ticketDir:t,expectedBaselineCommit:baseline,resumeJournal:j,progress:(row)=>{if(row.phase===\"transaction-lease-scanned\"){fs.writeFileSync(b+\"/ready-\"+process.pid,\"ready\\n\");wait(acquire)}if(row.phase===\"transact
879 |       const planPath = writePlan(row, value);
880 |       contenders = [0, 1].map(() => registerChild(spawn(process.execPath, ["-e", contenderSource, NORMALIZATION_MODULE, planPath, row.repoRoot, row.ticketDir, row.baselineCommit, journal, barrier, acquireRelease, winnerRelease, owner], { detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] })));
881 |       const deadline = Date.now() + 10_000;
882 |       while (readdirSync(barrier).filter((name) => name.startsWith("ready-")).length < 2) { if (Date.now() >= deadline) throw new Error("Contender barrier timed out"); await Bun.sleep(2); }
                                                                                                                                        ^
error: Contender barrier timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:882:131)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:1097:482)
      at spawnSync (node:child_process:226:22)
      at execFileSync (node:child_process:264:54)
      at registryCatalogGitlinkBoundaries (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9949:20)
      at registryCatalogInputView (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9967:29)
(fail) transaction plan journal v2 aggregate > rolls back a process-tree-killed mixed generator and commits ordinal two [10334.57ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-preparation-mkdir [17628.96ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-json-candidate-written [12171.11ms]
```

## (fail) transaction plan journal v2 aggregate > rejects stale generator and embedded-reference authority before mutation [2312.37ms]

```text
                                                                                                                                                                                                                                                                                                                                                                 ^
error: expect(received).toThrow(expected)

Expected pattern: /Regeneration input preimage changed/u
Received message: "Cargo command refused with retained ownership: Plan has unresolved blocking violations"

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:969:348)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:996:405)
      at spawnSync (node:child_process:226:22)
      at execFileSync (node:child_process:264:54)
      at registryCatalogGitlinkBoundaries (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9949:20)
(pass) transaction plan journal v2 aggregate > rejects ordinal collisions and malformed attempt siblings without mutation [791.18ms]
877 |       const barrier = mkdtempSync(join(row.root, "barrier-")), acquireRelease = join(barrier, "acquire"), winnerRelease = join(barrier, "winner"), owner = join(barrier, "owner");
878 |       const contenderSource = "const [m,p,r,t,baseline,j,b,acquire,winner,owner]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(m);const {normalizationProbeOwner}=await import(require(\"node:path\").resolve(require(\"node:path\").dirname(m),\"../🧪️tests/🎛️normalization-owner/🟦️.ts\"));const {runCargoCliCommand}=await import(require(\"node:path\").resolve(require(\"node:path\").dirname(m),\"../🗂️workspaces/🦀️cargo/🟦️.ts\"));const commandOwner=normalizationProbeOwner();const fs=require(\"node:fs\"),wait=(path)=>{while(!fs.existsSync(path))Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,2)},result=b+\"/result-\"+process.pid;try{await runCargoCliCommand(commandOwner,async()=>applyTaxonomyPlan(JSON.parse(await Bun.file(p).text()),{...{repoRoot:r,ticketDir:t,expectedBaselineCommit:baseline,resumeJournal:j,progress:(row)=>{if(row.phase===\"transaction-lease-scanned\"){fs.writeFileSync(b+\"/ready-\"+process.pid,\"ready\\n\");wait(acquire)}if(row.phase===\"transact
879 |       const planPath = writePlan(row, value);
880 |       contenders = [0, 1].map(() => registerChild(spawn(process.execPath, ["-e", contenderSource, NORMALIZATION_MODULE, planPath, row.repoRoot, row.ticketDir, row.baselineCommit, journal, barrier, acquireRelease, winnerRelease, owner], { detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] })));
881 |       const deadline = Date.now() + 10_000;
882 |       while (readdirSync(barrier).filter((name) => name.startsWith("ready-")).length < 2) { if (Date.now() >= deadline) throw new Error("Contender barrier timed out"); await Bun.sleep(2); }
                                                                                                                                        ^
error: Contender barrier timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:882:131)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:1097:482)
      at spawnSync (node:child_process:226:22)
      at execFileSync (node:child_process:264:54)
      at registryCatalogGitlinkBoundaries (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9949:20)
      at registryCatalogInputView (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9967:29)
(fail) transaction plan journal v2 aggregate > rolls back a process-tree-killed mixed generator and commits ordinal two [10334.57ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-preparation-mkdir [17628.96ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-json-candidate-written [12171.11ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-json-write-mkdir [17615.67ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-stale-quarantined [17635.43ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-json-canonical-exchanged [12158.62ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-prepared [12149.62ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-canonical-published [10383.88ms]
(pass) transaction plan journal v2 aggregate > rejects stale baseline, source digest, and counterfeit transaction segment with zero mutation [5068.67ms]
(fail) transaction plan journal v2 aggregate > rejects stale generator and embedded-reference authority before mutation [2312.37ms]
(fail) transaction plan journal v2 aggregate > elects exactly one synchronized stale-lease contender and permits committed retry [10211.77ms]
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-canonical-exchange [21920.43ms]
```

## (fail) transaction plan journal v2 aggregate > elects exactly one synchronized stale-lease contender and permits committed retry [10211.77ms]

```text
error: expect(received).toThrow(expected)

Expected pattern: /Regeneration input preimage changed/u
Received message: "Cargo command refused with retained ownership: Plan has unresolved blocking violations"

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:969:348)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:996:405)
      at spawnSync (node:child_process:226:22)
      at execFileSync (node:child_process:264:54)
      at registryCatalogGitlinkBoundaries (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9949:20)
(pass) transaction plan journal v2 aggregate > rejects ordinal collisions and malformed attempt siblings without mutation [791.18ms]
877 |       const barrier = mkdtempSync(join(row.root, "barrier-")), acquireRelease = join(barrier, "acquire"), winnerRelease = join(barrier, "winner"), owner = join(barrier, "owner");
878 |       const contenderSource = "const [m,p,r,t,baseline,j,b,acquire,winner,owner]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(m);const {normalizationProbeOwner}=await import(require(\"node:path\").resolve(require(\"node:path\").dirname(m),\"../🧪️tests/🎛️normalization-owner/🟦️.ts\"));const {runCargoCliCommand}=await import(require(\"node:path\").resolve(require(\"node:path\").dirname(m),\"../🗂️workspaces/🦀️cargo/🟦️.ts\"));const commandOwner=normalizationProbeOwner();const fs=require(\"node:fs\"),wait=(path)=>{while(!fs.existsSync(path))Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,2)},result=b+\"/result-\"+process.pid;try{await runCargoCliCommand(commandOwner,async()=>applyTaxonomyPlan(JSON.parse(await Bun.file(p).text()),{...{repoRoot:r,ticketDir:t,expectedBaselineCommit:baseline,resumeJournal:j,progress:(row)=>{if(row.phase===\"transaction-lease-scanned\"){fs.writeFileSync(b+\"/ready-\"+process.pid,\"ready\\n\");wait(acquire)}if(row.phase===\"transact
879 |       const planPath = writePlan(row, value);
880 |       contenders = [0, 1].map(() => registerChild(spawn(process.execPath, ["-e", contenderSource, NORMALIZATION_MODULE, planPath, row.repoRoot, row.ticketDir, row.baselineCommit, journal, barrier, acquireRelease, winnerRelease, owner], { detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] })));
881 |       const deadline = Date.now() + 10_000;
882 |       while (readdirSync(barrier).filter((name) => name.startsWith("ready-")).length < 2) { if (Date.now() >= deadline) throw new Error("Contender barrier timed out"); await Bun.sleep(2); }
                                                                                                                                        ^
error: Contender barrier timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:882:131)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:1097:482)
      at spawnSync (node:child_process:226:22)
      at execFileSync (node:child_process:264:54)
      at registryCatalogGitlinkBoundaries (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9949:20)
      at registryCatalogInputView (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9967:29)
(fail) transaction plan journal v2 aggregate > rolls back a process-tree-killed mixed generator and commits ordinal two [10334.57ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-preparation-mkdir [17628.96ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-json-candidate-written [12171.11ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-json-write-mkdir [17615.67ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-stale-quarantined [17635.43ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-json-canonical-exchanged [12158.62ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-prepared [12149.62ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-canonical-published [10383.88ms]
(pass) transaction plan journal v2 aggregate > rejects stale baseline, source digest, and counterfeit transaction segment with zero mutation [5068.67ms]
(fail) transaction plan journal v2 aggregate > rejects stale generator and embedded-reference authority before mutation [2312.37ms]
(fail) transaction plan journal v2 aggregate > elects exactly one synchronized stale-lease contender and permits committed retry [10211.77ms]
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-canonical-exchange [21920.43ms]
  ^ this test timed out after 15000ms.
```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-canonical-exchange [21920.43ms]

```text

Expected pattern: /Regeneration input preimage changed/u
Received message: "Cargo command refused with retained ownership: Plan has unresolved blocking violations"

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:969:348)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:996:405)
      at spawnSync (node:child_process:226:22)
      at execFileSync (node:child_process:264:54)
      at registryCatalogGitlinkBoundaries (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9949:20)
(pass) transaction plan journal v2 aggregate > rejects ordinal collisions and malformed attempt siblings without mutation [791.18ms]
877 |       const barrier = mkdtempSync(join(row.root, "barrier-")), acquireRelease = join(barrier, "acquire"), winnerRelease = join(barrier, "winner"), owner = join(barrier, "owner");
878 |       const contenderSource = "const [m,p,r,t,baseline,j,b,acquire,winner,owner]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(m);const {normalizationProbeOwner}=await import(require(\"node:path\").resolve(require(\"node:path\").dirname(m),\"../🧪️tests/🎛️normalization-owner/🟦️.ts\"));const {runCargoCliCommand}=await import(require(\"node:path\").resolve(require(\"node:path\").dirname(m),\"../🗂️workspaces/🦀️cargo/🟦️.ts\"));const commandOwner=normalizationProbeOwner();const fs=require(\"node:fs\"),wait=(path)=>{while(!fs.existsSync(path))Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,2)},result=b+\"/result-\"+process.pid;try{await runCargoCliCommand(commandOwner,async()=>applyTaxonomyPlan(JSON.parse(await Bun.file(p).text()),{...{repoRoot:r,ticketDir:t,expectedBaselineCommit:baseline,resumeJournal:j,progress:(row)=>{if(row.phase===\"transaction-lease-scanned\"){fs.writeFileSync(b+\"/ready-\"+process.pid,\"ready\\n\");wait(acquire)}if(row.phase===\"transact
879 |       const planPath = writePlan(row, value);
880 |       contenders = [0, 1].map(() => registerChild(spawn(process.execPath, ["-e", contenderSource, NORMALIZATION_MODULE, planPath, row.repoRoot, row.ticketDir, row.baselineCommit, journal, barrier, acquireRelease, winnerRelease, owner], { detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] })));
881 |       const deadline = Date.now() + 10_000;
882 |       while (readdirSync(barrier).filter((name) => name.startsWith("ready-")).length < 2) { if (Date.now() >= deadline) throw new Error("Contender barrier timed out"); await Bun.sleep(2); }
                                                                                                                                        ^
error: Contender barrier timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:882:131)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:1097:482)
      at spawnSync (node:child_process:226:22)
      at execFileSync (node:child_process:264:54)
      at registryCatalogGitlinkBoundaries (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9949:20)
      at registryCatalogInputView (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9967:29)
(fail) transaction plan journal v2 aggregate > rolls back a process-tree-killed mixed generator and commits ordinal two [10334.57ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-preparation-mkdir [17628.96ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-json-candidate-written [12171.11ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-json-write-mkdir [17615.67ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-stale-quarantined [17635.43ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-json-canonical-exchanged [12158.62ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-prepared [12149.62ms]
(pass) transaction plan journal v2 aggregate > recovers parent-killed transaction-lease-canonical-published [10383.88ms]
(pass) transaction plan journal v2 aggregate > rejects stale baseline, source digest, and counterfeit transaction segment with zero mutation [5068.67ms]
(fail) transaction plan journal v2 aggregate > rejects stale generator and embedded-reference authority before mutation [2312.37ms]
(fail) transaction plan journal v2 aggregate > elects exactly one synchronized stale-lease contender and permits committed retry [10211.77ms]
(fail) transaction plan journal v2 aggregate > recovers parent-killed transaction-restore-canonical-exchange [21920.43ms]
  ^ this test timed out after 15000ms.

```

## (fail) transaction plan journal v2 aggregate > recovers parent-killed committed and rolled-back backup-only terminal cleanup [15000.00ms]

```text
-------------------------------


# Unhandled error between tests
-------------------------------
406 | function boundedExit(childProcess: ChildProcess, timeoutMs = 10_000): Promise<Readonly<{ code: number | null; signal: NodeJS.Signals | null }>> {
407 |   if (childProcess.exitCode !== null || childProcess.signalCode !== null) return Promise.resolve({ code: childProcess.exitCode, signal: childProcess.signalCode });
408 |   return new Promise((resolveExit, rejectExit) => {
409 |     let timedOut = false;
410 |     const timer = setTimeout(() => { timedOut = true; killTree(childProcess); }, timeoutMs);
411 |     childProcess.once("exit", (code, signal) => { clearTimeout(timer); if (timedOut) rejectExit(new Error(`Child ${childProcess.pid ?? "unknown"} exit timed out`)); else resolveExit({ code, signal }); });
                                                                                                          ^
error: Child 31955 exit timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:411:101)
      at emit (node:events:98:22)
      at #handleOnExit (node:child_process:520:14)
-------------------------------


# Unhandled error between tests
-------------------------------
406 | function boundedExit(childProcess: ChildProcess, timeoutMs = 10_000): Promise<Readonly<{ code: number | null; signal: NodeJS.Signals | null }>> {
407 |   if (childProcess.exitCode !== null || childProcess.signalCode !== null) return Promise.resolve({ code: childProcess.exitCode, signal: childProcess.signalCode });
408 |   return new Promise((resolveExit, rejectExit) => {
409 |     let timedOut = false;
410 |     const timer = setTimeout(() => { timedOut = true; killTree(childProcess); }, timeoutMs);
411 |     childProcess.once("exit", (code, signal) => { clearTimeout(timer); if (timedOut) rejectExit(new Error(`Child ${childProcess.pid ?? "unknown"} exit timed out`)); else resolveExit({ code, signal }); });
                                                                                                          ^
error: Child 31956 exit timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:411:101)
      at emit (node:events:98:22)
      at #handleOnExit (node:child_process:520:14)
-------------------------------

(pass) transaction plan journal v2 aggregate > rejects forged backup and restore preparations without mutation [4792.78ms]
(fail) transaction plan journal v2 aggregate > recovers parent-killed committed and rolled-back backup-only terminal cleanup [15000.00ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rejects unreachable duplicate backup and edit publication tuples exactly [15000.00ms]
```

## (fail) transaction plan journal v2 aggregate > rejects unreachable duplicate backup and edit publication tuples exactly [15000.00ms]

```text

# Unhandled error between tests
-------------------------------
406 | function boundedExit(childProcess: ChildProcess, timeoutMs = 10_000): Promise<Readonly<{ code: number | null; signal: NodeJS.Signals | null }>> {
407 |   if (childProcess.exitCode !== null || childProcess.signalCode !== null) return Promise.resolve({ code: childProcess.exitCode, signal: childProcess.signalCode });
408 |   return new Promise((resolveExit, rejectExit) => {
409 |     let timedOut = false;
410 |     const timer = setTimeout(() => { timedOut = true; killTree(childProcess); }, timeoutMs);
411 |     childProcess.once("exit", (code, signal) => { clearTimeout(timer); if (timedOut) rejectExit(new Error(`Child ${childProcess.pid ?? "unknown"} exit timed out`)); else resolveExit({ code, signal }); });
                                                                                                          ^
error: Child 31955 exit timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:411:101)
      at emit (node:events:98:22)
      at #handleOnExit (node:child_process:520:14)
-------------------------------


# Unhandled error between tests
-------------------------------
406 | function boundedExit(childProcess: ChildProcess, timeoutMs = 10_000): Promise<Readonly<{ code: number | null; signal: NodeJS.Signals | null }>> {
407 |   if (childProcess.exitCode !== null || childProcess.signalCode !== null) return Promise.resolve({ code: childProcess.exitCode, signal: childProcess.signalCode });
408 |   return new Promise((resolveExit, rejectExit) => {
409 |     let timedOut = false;
410 |     const timer = setTimeout(() => { timedOut = true; killTree(childProcess); }, timeoutMs);
411 |     childProcess.once("exit", (code, signal) => { clearTimeout(timer); if (timedOut) rejectExit(new Error(`Child ${childProcess.pid ?? "unknown"} exit timed out`)); else resolveExit({ code, signal }); });
                                                                                                          ^
error: Child 31956 exit timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:411:101)
      at emit (node:events:98:22)
      at #handleOnExit (node:child_process:520:14)
-------------------------------

(pass) transaction plan journal v2 aggregate > rejects forged backup and restore preparations without mutation [4792.78ms]
(fail) transaction plan journal v2 aggregate > recovers parent-killed committed and rolled-back backup-only terminal cleanup [15000.00ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rejects unreachable duplicate backup and edit publication tuples exactly [15000.00ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > keeps double-plan bytes stable, cancellation exact, and committed second apply immutable [15000.00ms]
```

## (fail) transaction plan journal v2 aggregate > keeps double-plan bytes stable, cancellation exact, and committed second apply immutable [15000.00ms]

```text
-------------------------------
406 | function boundedExit(childProcess: ChildProcess, timeoutMs = 10_000): Promise<Readonly<{ code: number | null; signal: NodeJS.Signals | null }>> {
407 |   if (childProcess.exitCode !== null || childProcess.signalCode !== null) return Promise.resolve({ code: childProcess.exitCode, signal: childProcess.signalCode });
408 |   return new Promise((resolveExit, rejectExit) => {
409 |     let timedOut = false;
410 |     const timer = setTimeout(() => { timedOut = true; killTree(childProcess); }, timeoutMs);
411 |     childProcess.once("exit", (code, signal) => { clearTimeout(timer); if (timedOut) rejectExit(new Error(`Child ${childProcess.pid ?? "unknown"} exit timed out`)); else resolveExit({ code, signal }); });
                                                                                                          ^
error: Child 31955 exit timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:411:101)
      at emit (node:events:98:22)
      at #handleOnExit (node:child_process:520:14)
-------------------------------


# Unhandled error between tests
-------------------------------
406 | function boundedExit(childProcess: ChildProcess, timeoutMs = 10_000): Promise<Readonly<{ code: number | null; signal: NodeJS.Signals | null }>> {
407 |   if (childProcess.exitCode !== null || childProcess.signalCode !== null) return Promise.resolve({ code: childProcess.exitCode, signal: childProcess.signalCode });
408 |   return new Promise((resolveExit, rejectExit) => {
409 |     let timedOut = false;
410 |     const timer = setTimeout(() => { timedOut = true; killTree(childProcess); }, timeoutMs);
411 |     childProcess.once("exit", (code, signal) => { clearTimeout(timer); if (timedOut) rejectExit(new Error(`Child ${childProcess.pid ?? "unknown"} exit timed out`)); else resolveExit({ code, signal }); });
                                                                                                          ^
error: Child 31956 exit timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:411:101)
      at emit (node:events:98:22)
      at #handleOnExit (node:child_process:520:14)
-------------------------------

(pass) transaction plan journal v2 aggregate > rejects forged backup and restore preparations without mutation [4792.78ms]
(fail) transaction plan journal v2 aggregate > recovers parent-killed committed and rolled-back backup-only terminal cleanup [15000.00ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rejects unreachable duplicate backup and edit publication tuples exactly [15000.00ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > keeps double-plan bytes stable, cancellation exact, and committed second apply immutable [15000.00ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers caught allocation and journal previous-image callback failures [15000.00ms]
```

## (fail) transaction plan journal v2 aggregate > recovers caught allocation and journal previous-image callback failures [15000.00ms]

```text
407 |   if (childProcess.exitCode !== null || childProcess.signalCode !== null) return Promise.resolve({ code: childProcess.exitCode, signal: childProcess.signalCode });
408 |   return new Promise((resolveExit, rejectExit) => {
409 |     let timedOut = false;
410 |     const timer = setTimeout(() => { timedOut = true; killTree(childProcess); }, timeoutMs);
411 |     childProcess.once("exit", (code, signal) => { clearTimeout(timer); if (timedOut) rejectExit(new Error(`Child ${childProcess.pid ?? "unknown"} exit timed out`)); else resolveExit({ code, signal }); });
                                                                                                          ^
error: Child 31955 exit timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:411:101)
      at emit (node:events:98:22)
      at #handleOnExit (node:child_process:520:14)
-------------------------------


# Unhandled error between tests
-------------------------------
406 | function boundedExit(childProcess: ChildProcess, timeoutMs = 10_000): Promise<Readonly<{ code: number | null; signal: NodeJS.Signals | null }>> {
407 |   if (childProcess.exitCode !== null || childProcess.signalCode !== null) return Promise.resolve({ code: childProcess.exitCode, signal: childProcess.signalCode });
408 |   return new Promise((resolveExit, rejectExit) => {
409 |     let timedOut = false;
410 |     const timer = setTimeout(() => { timedOut = true; killTree(childProcess); }, timeoutMs);
411 |     childProcess.once("exit", (code, signal) => { clearTimeout(timer); if (timedOut) rejectExit(new Error(`Child ${childProcess.pid ?? "unknown"} exit timed out`)); else resolveExit({ code, signal }); });
                                                                                                          ^
error: Child 31956 exit timed out
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-25/snapshot/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:411:101)
      at emit (node:events:98:22)
      at #handleOnExit (node:child_process:520:14)
-------------------------------

(pass) transaction plan journal v2 aggregate > rejects forged backup and restore preparations without mutation [4792.78ms]
(fail) transaction plan journal v2 aggregate > recovers parent-killed committed and rolled-back backup-only terminal cleanup [15000.00ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > rejects unreachable duplicate backup and edit publication tuples exactly [15000.00ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > keeps double-plan bytes stable, cancellation exact, and committed second apply immutable [15000.00ms]
  ^ this test timed out after 15000ms.
(fail) transaction plan journal v2 aggregate > recovers caught allocation and journal previous-image callback failures [15000.00ms]
  ^ this test timed out after 15000ms.
killed 7 dangling processes
```
