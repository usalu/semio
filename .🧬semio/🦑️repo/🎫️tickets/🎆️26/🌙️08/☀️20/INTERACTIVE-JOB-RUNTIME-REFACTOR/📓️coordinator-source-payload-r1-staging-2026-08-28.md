# Source Payload Foundation R1: Import-Time Staging Failure

Root actually invoked the canonical actor test-long target selecting only the three new source payload declaration/identity/construction groups. Nx exited one before tests because the concurrently staged UI payload class referenced private `#close` before its declaration was mounted. No selected runtime test executed and no independent three-test pass is claimed.

UI ownership and ongoing implementation are preserved. The exact import diagnostic was routed without a competing edit or request to interrupt the source cutover. A coherent UI source release is required before retry.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-actor:test-long --skip-nx-cache --args='--run -t "two-way resident payload declaration|fabricated resident payload associations|exact field construction roots"'
```

## Complete Captured Output

```text

> nx run @semio-tech/framework-actor:test-long --args=--run -t "two-way resident payload declaration|fabricated resident payload associations|exact field construction roots"

> bun ./📜️script.ts test long --run -t "two-way resident payload declaration|fabricated resident payload associations|exact field construction roots"

[1m272 | [0m    closePayload = (state, grant) => { state.closing = [0m[33mtrue[0m[0m[2m;[0m [0m[35mreturn[0m state.facade ? state.facade[0m[3m[1m.#close[0m(grant) : closePayloadSlot([0m[33mnull[0m, state.parentSlot!, grant)[0m[2m;[0m }[0m[2m;[0m
                                                                                                      [1m[31m[1m^[0m
[31merror[0m[2m: [0m[1mPrivate name "#close" must be declared in an enclosing class[0m
    [2mat [0m[36m/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts[0m[2m:[0m[33m272[0m[2m:[0m[33m97[0m
[0m
[2mBun v1.3.14 (macOS arm64)[0m
Warning: command "bun ./📜️script.ts test long --run -t "two-way resident payload declaration|fabricated resident payload associations|exact field construction roots"" exited with non-zero status code


 NX   Running target test-long for project @semio-tech/framework-actor failed

Failed tasks:

- @semio-tech/framework-actor:test-long

Hint: run the command with --verbose for more details.


```

## Selected Capture

Twenty-four sequential selected hashes, not an atomic transitive closure. No source hold was requested. Changed selected rows: 1.

### Before

```text
419448407f1b38ecfb27c7f2136de3e174669420b2771bad87f325282e9af544  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
3c6cc4246e1e6a841d0158e103056bc2706843d5866294791e8e5a77affafcc8  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json
e93ea7a4f1ad39703b126a9c9847cc63f9c1afce9ae062cb2eb453309bf4827f  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json
c2db1037203c711da2d3af2e7ae600677eb6864de35f05fb0b3f533281124508  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts
cbe9a8cba5f138a4892f0c751de5f6693d61a84635228cc5de3bb1deef5bca21  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📋️project.json
ecf50673fdc515eba3de67cd47a37e333d1cd061d28233e44083e67b230bf863  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts
6957021837e6e5c731cec0530bb109694cff4ef6b25a9ecb0a9f02f37c1c8840  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts
df568eb4fc6ec3e74da2f0a713dbb0262c8016e4ff79c2ee59d0861399da3697  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json
bb6bbc186e8b971e768c1af6992d2866a116c9cba18c25b61ee176e0de7457ce  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts
a9e103be4635c52244ef12de057b47005e97b4a81e50cf9001a9959b8af79ca0  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️component.ts
745742fc3f60c06e11ef022a2e3a3ec98672fbec32c1fd6953721c45c6509933  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json
202113fa2f1cc8cf24de89bb192697259dd4a507825b0158c4a27262dc35fa7c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️contract.json
48f49106bf263df3a7cee1e2bf5e7113b6cde84affb5dd9b3a06cac1e4ad4b18  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️fixture.json
cccb14e2851cd4a3ba2a83e4b176db256dbe16558bc67a1d5389c46427043788  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
806485f5d6e5689aa026b52fbacec759cdd4dc29c656cca08fc209a4b107fded  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts
07874d4649a305107695ffcdc50a53e3c9197ed39959857a1f97c9b7b76614c0  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts
c2f936ec148bc29aacd85732616b9dfd5a2d8c174ae4cfaaf09d34480b673bc2  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️component.ts
87c7f25b1aed9bbc15bc3916d837bdd518140bec7e93bd04ba3eac1831edd59f  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️component.ts
06aa8d36e8643c11dbe65e9a89eae0e48d44b450a5d3e19b2041345f6788f515  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📄️page/🟦️component.ts
e12f587c3bf7bfc9cde3ca37d677cd0af983f11751245593cbba8b0b079e6dcf  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️schema.json
5a288884f41eca466efe4b59ed04d530a46554120d3338c4c7192e7a3dc1d4d3  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️contract.json
a5d94bacbc4340890a9316c4aa1e6458afa13d568cbcf1fe9a39fe80b1fd1c0a  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧪️fixture.json
a1c8710483759bfc443f32b5557287b082a61b893e43f734ae2edec26ecb82cd  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧪️schema.json
cbabad50e7bde94f9734c859cca3e4abe2d945ce86838f897ae91153a527143c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️component.ts
```

### After

```text
419448407f1b38ecfb27c7f2136de3e174669420b2771bad87f325282e9af544  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
3c6cc4246e1e6a841d0158e103056bc2706843d5866294791e8e5a77affafcc8  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json
e93ea7a4f1ad39703b126a9c9847cc63f9c1afce9ae062cb2eb453309bf4827f  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json
c2db1037203c711da2d3af2e7ae600677eb6864de35f05fb0b3f533281124508  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts
cbe9a8cba5f138a4892f0c751de5f6693d61a84635228cc5de3bb1deef5bca21  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📋️project.json
ecf50673fdc515eba3de67cd47a37e333d1cd061d28233e44083e67b230bf863  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts
6957021837e6e5c731cec0530bb109694cff4ef6b25a9ecb0a9f02f37c1c8840  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts
df568eb4fc6ec3e74da2f0a713dbb0262c8016e4ff79c2ee59d0861399da3697  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json
2e8c22f1736a9fe6fd151b3e18ead7ada0fde05bf20e125fa697503273d4a1ec  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts
a9e103be4635c52244ef12de057b47005e97b4a81e50cf9001a9959b8af79ca0  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️component.ts
745742fc3f60c06e11ef022a2e3a3ec98672fbec32c1fd6953721c45c6509933  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json
202113fa2f1cc8cf24de89bb192697259dd4a507825b0158c4a27262dc35fa7c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️contract.json
48f49106bf263df3a7cee1e2bf5e7113b6cde84affb5dd9b3a06cac1e4ad4b18  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️fixture.json
cccb14e2851cd4a3ba2a83e4b176db256dbe16558bc67a1d5389c46427043788  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
806485f5d6e5689aa026b52fbacec759cdd4dc29c656cca08fc209a4b107fded  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts
07874d4649a305107695ffcdc50a53e3c9197ed39959857a1f97c9b7b76614c0  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts
c2f936ec148bc29aacd85732616b9dfd5a2d8c174ae4cfaaf09d34480b673bc2  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️component.ts
87c7f25b1aed9bbc15bc3916d837bdd518140bec7e93bd04ba3eac1831edd59f  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️component.ts
06aa8d36e8643c11dbe65e9a89eae0e48d44b450a5d3e19b2041345f6788f515  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📄️page/🟦️component.ts
e12f587c3bf7bfc9cde3ca37d677cd0af983f11751245593cbba8b0b079e6dcf  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️schema.json
5a288884f41eca466efe4b59ed04d530a46554120d3338c4c7192e7a3dc1d4d3  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️contract.json
a5d94bacbc4340890a9316c4aa1e6458afa13d568cbcf1fe9a39fe80b1fd1c0a  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧪️fixture.json
a1c8710483759bfc443f32b5557287b082a61b893e43f734ae2edec26ecb82cd  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧪️schema.json
cbabad50e7bde94f9734c859cca3e4abe2d945ce86838f897ae91153a527143c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️component.ts
```

