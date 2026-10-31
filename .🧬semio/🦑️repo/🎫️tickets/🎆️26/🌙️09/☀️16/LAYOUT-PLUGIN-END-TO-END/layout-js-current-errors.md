Actual diagnostic target log selected failure excerpts; full outer terminal result must be recorded separately.

@semio-tech/layout-js: [0m[1m184 |[0m     [0m[35mif[0m(stopped===[0m[32m"output delivery refused"[0m&&status.signal!==[0m[33mnull[0m)[0m[35mthrow[0m deliveryFailure[0m[2m;[0m
@semio-tech/layout-js: [0m[1m185 |[0m     [0m[35mif[0m (status.signal !== [0m[33mnull[0m || status.code !== [0m[33m0[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mOwnedCommandFailure[0m(label, { version: [0m[33m1[0m, status: status.code, signal: status.signal })[0m[2m;[0m
@semio-tech/layout-js:                                                                  [31m[1m^[0m
@semio-tech/layout-js: [0m[31mOwnedCommandFailure[0m[2m:[0m [1mlayout:typescript:behavior failed (1)[0m
@semio-tech/layout-js:  outcome[0m[2m:[0m {
@semio-tech/layout-js:   [0mversion[2m:[0m [0m[33m1[0m[0m[2m,[0m
@semio-tech/layout-js:   [0mstatus[2m:[0m [0m[33m1[0m[0m[2m,[0m
@semio-tech/layout-js:   [0msignal[2m:[0m [0m[33mnull[0m[0m[2m,[0m
@semio-tech/layout-js: }[0m[2m,[0m

@semio-tech/layout-js: [0m
@semio-tech/layout-js: [2mBun v1.4.2 (macOS arm64)[0m
@semio-tech/layout-js: [0m[1m180 |[0m     [0m[35mif[0m(auxiliary&&commandSignal?.aborted)[0m[35mthrow[0m commandSignal.reason[0m[2m;[0m
@semio-tech/layout-js: [0m[1m181 |[0m     [0m[35mif[0m(auxiliaryFailed)[0m[35mthrow[0m auxiliaryFailure[0m[2m;[0m
@semio-tech/layout-js: [0m[1m182 |[0m     [0m[35mif[0m(auxiliary)requireIssuedResourceControlV1(options.control!)[0m[2m;[0m
@semio-tech/layout-js: [0m[1m183 |[0m     [0m[35mif[0m (stopped&&stopped!==[0m[32m"output delivery refused"[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mError[0m([0m[32m`[0m${label}[0m[32m stopped: [0m${stopped}[0m[32m`[0m)[0m[2m;[0m
@semio-tech/layout-js: [0m[1m184 |[0m     [0m[35mif[0m(stopped===[0m[32m"output delivery refused"[0m&&status.signal!==[0m[33mnull[0m)[0m[35mthrow[0m deliveryFailure[0m[2m;[0m
@semio-tech/layout-js: [0m[1m185 |[0m     [0m[35mif[0m (status.signal !== [0m[33mnull[0m || status.code !== [0m[33m0[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mOwnedCommandFailure[0m(label, { version: [0m[33m1[0m, status: status.code, signal: status.signal })[0m[2m;[0m
@semio-tech/layout-js:                                                                  [31m[1m^[0m

@semio-tech/layout-js: [0m[1m181 |[0m     [0m[35mif[0m(auxiliaryFailed)[0m[35mthrow[0m auxiliaryFailure[0m[2m;[0m
@semio-tech/layout-js: [0m[1m182 |[0m     [0m[35mif[0m(auxiliary)requireIssuedResourceControlV1(options.control!)[0m[2m;[0m
@semio-tech/layout-js: [0m[1m183 |[0m     [0m[35mif[0m (stopped&&stopped!==[0m[32m"output delivery refused"[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mError[0m([0m[32m`[0m${label}[0m[32m stopped: [0m${stopped}[0m[32m`[0m)[0m[2m;[0m
@semio-tech/layout-js: [0m[1m184 |[0m     [0m[35mif[0m(stopped===[0m[32m"output delivery refused"[0m&&status.signal!==[0m[33mnull[0m)[0m[35mthrow[0m deliveryFailure[0m[2m;[0m
@semio-tech/layout-js: [0m[1m185 |[0m     [0m[35mif[0m (status.signal !== [0m[33mnull[0m || status.code !== [0m[33m0[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mOwnedCommandFailure[0m(label, { version: [0m[33m1[0m, status: status.code, signal: status.signal })[0m[2m;[0m
@semio-tech/layout-js:                                                                  [31m[1m^[0m
@semio-tech/layout-js: [0m[31mOwnedCommandFailure[0m[2m:[0m [1mprocess:owner-command failed (1)[0m
@semio-tech/layout-js:  outcome[0m[2m:[0m {
@semio-tech/layout-js:   [0mversion[2m:[0m [0m[33m1[0m[0m[2m,[0m

@semio-tech/layout-js: [0m[1m182 |[0m     [0m[35mif[0m(auxiliary)requireIssuedResourceControlV1(options.control!)[0m[2m;[0m
@semio-tech/layout-js: [0m[1m183 |[0m     [0m[35mif[0m (stopped&&stopped!==[0m[32m"output delivery refused"[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mError[0m([0m[32m`[0m${label}[0m[32m stopped: [0m${stopped}[0m[32m`[0m)[0m[2m;[0m
@semio-tech/layout-js: [0m[1m184 |[0m     [0m[35mif[0m(stopped===[0m[32m"output delivery refused"[0m&&status.signal!==[0m[33mnull[0m)[0m[35mthrow[0m deliveryFailure[0m[2m;[0m
@semio-tech/layout-js: [0m[1m185 |[0m     [0m[35mif[0m (status.signal !== [0m[33mnull[0m || status.code !== [0m[33m0[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mOwnedCommandFailure[0m(label, { version: [0m[33m1[0m, status: status.code, signal: status.signal })[0m[2m;[0m
@semio-tech/layout-js:                                                                  [31m[1m^[0m
@semio-tech/layout-js: [0m[31mOwnedCommandFailure[0m[2m:[0m [1mprocess:owner-command failed (1)[0m
@semio-tech/layout-js:  outcome[0m[2m:[0m {
@semio-tech/layout-js:   [0mversion[2m:[0m [0m[33m1[0m[0m[2m,[0m
@semio-tech/layout-js:   [0mstatus[2m:[0m [0m[33m1[0m[0m[2m,[0m

@semio-tech/layout-js: [0m[1m184 |[0m     [0m[35mif[0m(stopped===[0m[32m"output delivery refused"[0m&&status.signal!==[0m[33mnull[0m)[0m[35mthrow[0m deliveryFailure[0m[2m;[0m
@semio-tech/layout-js: [0m[1m185 |[0m     [0m[35mif[0m (status.signal !== [0m[33mnull[0m || status.code !== [0m[33m0[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mOwnedCommandFailure[0m(label, { version: [0m[33m1[0m, status: status.code, signal: status.signal })[0m[2m;[0m
@semio-tech/layout-js:                                                                  [31m[1m^[0m
@semio-tech/layout-js: [0m[31mOwnedCommandFailure[0m[2m:[0m [1mprocess:owner-command failed (1)[0m
@semio-tech/layout-js:  outcome[0m[2m:[0m {
@semio-tech/layout-js:   [0mversion[2m:[0m [0m[33m1[0m[0m[2m,[0m
@semio-tech/layout-js:   [0mstatus[2m:[0m [0m[33m1[0m[0m[2m,[0m
@semio-tech/layout-js:   [0msignal[2m:[0m [0m[33mnull[0m[0m[2m,[0m
@semio-tech/layout-js: }[0m[2m,[0m




 NX   Running target test for project @semio-tech/layout-js failed

Failed tasks:

- @semio-tech/layout-js:test:prototype


819 |   await new NxScript(root, root, parent).runIssued([...args], phaseControl);
820 |   const status = process.exitCode;
821 |   process.exitCode = priorExitCode;
822 |     throw Error(`Owned Nx invocation failed (${status}): nx ${args.join(" ")}`);
                ^
error: Owned Nx invocation failed (1): nx run @semio-tech/layout-js:test --output-style=stream --parallel=8
      at runOwnedNxInvocationV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts:821:11)

Bun v1.4.2 (macOS arm64)

821 |   process.exitCode = priorExitCode;
822 |     throw Error(`Owned Nx invocation failed (${status}): nx ${args.join(" ")}`);
                ^
error: Owned Nx invocation failed (1): nx run @semio-tech/layout-js:test --output-style=stream --parallel=8
      at runOwnedNxInvocationV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts:821:11)

Bun v1.4.2 (macOS arm64)
180 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
181 |     if(auxiliaryFailed)throw auxiliaryFailure;


Bun v1.4.2 (macOS arm64)
180 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
181 |     if(auxiliaryFailed)throw auxiliaryFailure;
182 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
183 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
184 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
185 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^

181 |     if(auxiliaryFailed)throw auxiliaryFailure;
182 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
183 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
184 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
185 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,

182 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
183 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
184 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
185 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,
  status: 1,

184 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
185 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
