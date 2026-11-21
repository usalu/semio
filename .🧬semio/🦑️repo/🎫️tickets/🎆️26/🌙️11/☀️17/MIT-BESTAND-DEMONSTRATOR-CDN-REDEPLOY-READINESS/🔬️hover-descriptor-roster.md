# Interaction Hover Descriptor Roster

Twenty-six additional Hub component descriptor pairs now use the common Rust interactionHover contract. The 26 pairs contain 190 updated actions. CAD and Procedural were handled separately using direct emitter replays.

The check derives optional arguments, argument types and localized labels from the common Rust producer, projects those fields into the full Pack value and authored JSON, and compares the complete normalized values. The production Pack codec re-encodes the projected Pack value. The authored JSON edits preserve every byte outside the replaced hover argument vectors. Direct CAD and Procedural emission calibrated both formats byte for byte. The production component descriptor writer and release verify path published and re-read every pair below. A separate read-only pass checked 28 roots and four confirmations with no remaining descriptor changes.

Thirteen pairs required additional Pack reconciliation because their initial Pack and JSON disagreed outside hover arguments. The resolver verified each field against the current producer and required full equality with the authored JSON after normalization. Those fields cover the app-channel constant, port label IDs and translations, the Space index declaration, the VCS noOperation action, and the I/O text carrier. The text carrier check followed the relevant Rust serializer and deserializer declarations and reproduced the production registry ordering, with composer routes kept in their original prefix. The authored JSON had the current values for all these fields.

Direct release emitter replays covered exact retained CAD wgIlEJ7 and Procedural wqks1dx guests. The other 26 rows verify source-derived authoring and release descriptor verification. Their new guest emissions belong to the release coordinator.

## Published pairs

Each directory contains the authored JSON `🔣️.json` and Pack `🛂️.descriptor.semio`. The semantic SHA-256 is the SHA-256 of the exact Pack bytes. The table includes the separately hashed authored JSON.

| Component and workspace-relative directory | Hover actions | Additional Pack updates | Prior Pack SHA-256 | Authored Pack SHA-256 | Authored JSON SHA-256 |
| --- | ---: | --- | --- | --- | --- |
| animate `🌎️hub/🧩️compositions/🎞️animate/🧩️component` | 1 | app-channel, frames:in, text-carrier | `379acab2fcf84f0ff4b7aef4570d03a440511ea571f665b572b5a0b43e79c631` | `b187674de5db2d2cd2a58b58fb88e0a37c4446815a4f0a64381f89974ace3bd0` | `7250ba2d1c0d57b0c39a0af612406b58e28dd068b81370dc9c65918c84ae5cff` |
| architect `🌎️hub/🧩️compositions/🏛️architect/🧩️component` | 5 | app-channel | `dc56642da242cda0542f6cb9ef6beb5d5b53591603da947372d0339e22a508d3` | `50ed16894157804b1d39327e78f8f52f772edc635e44fe4270f041a151e2d0d8` | `4dc49306cb6f5449f2dc7914ae4305b73fe0766342b58957e502a193043afcf8` |
| bim `🌎️hub/🧩️compositions/🏙️bim/🧩️component` | 9 | app-channel | `4ff407f06e8fc69fc8128229721d6a3d1b34cf2ab39c11ff40b363a3f296445b` | `83e36330165d89e5b8dbefe5c7d2a34ab13551d940ad8c74316b6c9b4e6ca112` | `6e96310f9548477440bc8f9e3811427258f6886e235e92ff6a1bc13956ca666d` |
| block `🌎️hub/🧩️compositions/🧱️block/🧩️component` | 4 | app-channel, catalog:out, text-carrier | `ac3da1173549b14bf9e138ece13386375dc841e067e2c02212e15c9e2acaa623` | `db3783334adc49b7392ce66a7c372d0e7712afb68a81d7dc56bfdbcee81dfbf1` | `3709469977c46fe2e1801a7a9d16ec70fee5388cf2a5557ac89f3a76ddab9d30` |
| energy `🌎️hub/🧩️compositions/🔋️energy/🧩️component` | 4 | None | `f42992705c59a41853f9f4cbbea455d88a7ec2f8791ca468e4508f7a115facb8` | `847b72debdc98091de1f0bbeae5f37323258c763977aa3920fa50d0c73753f06` | `1f8ac887be8aed7fe814706d966cbf33c71c824b8d914371e7922ecd9de9b192` |
| flow `🌎️hub/🧩️compositions/🌊️flow/🧩️component` | 5 | None | `180f46a6115b0495c2abc8a0c24efeb212a6759c5c054e1ea2edf2f37d887114` | `182d23284a7029d9d9cca1be74b103eee1490d3953e64153436038817f4a3899` | `ac455e861217851b7a2b475592b67faaf4534b2ecd994d74309f87fd280b2f8b` |
| forms `🌎️hub/🧩️compositions/📋️forms/🧩️component` | 3 | app-channel, dictionary:out | `bf130490f0e8597dfde9d51a06185a5492f8b120a3acb820739cdb7677d70132` | `50782bcac154d0a85876522caaa632492df89949e7c401045ef3e50716578ba0` | `ce84cc62bc535a6347e494bd50633ad63edf53c641faf1416b821de5bc67bbb0` |
| gis `🌎️hub/🧩️compositions/🌍️gis/🧩️component` | 2 | None | `0a876ec24f4fa0f718706948481444f343c83c7353c7ff3362d2873047b14a4b` | `264d06318146ca1504a22dd0515067d7dcdfbe7b7e728c27bddfd33bf65d5d44` | `e9f370ae8574fa93d7046bdfa479611b2ef6ae96feecb4e3b79fee48d84cf4be` |
| imperative `🌎️hub/🧩️compositions/📜️imperative/🧩️component` | 2 | app-channel, result:out | `95772e75d52cfde36e0544d642c354018e2400b13b5b055ad948229fa6efb0be` | `f03762a8d2054effa6f17992da65332f176cffed4cc4671cdd0c66e8f2ed9b46` | `236461ba85293feeb3c9dbb2fd5f66e22b0b0476cd0e1392a365304bc66e893c` |
| lowpoly `🌎️hub/🧩️compositions/💠️lowpoly/🧩️component` | 2 | app-channel, mesh:in, mesh:out | `4c21935fc8f34f43ed6f3d16ae51a3e093e1e66cf6249296ab867103096250f1` | `775f21d17f77580c9d151a392919771c8090be2b82e8a6e2393bba34db37139a` | `0bac9ad90940fc1ca79b028abab7b3b2d9b732db2a9183bcc52ec83e7072a3c6` |
| norm `🌎️hub/🧩️compositions/📕️norm/🧩️component` | 32 | None | `50594c65cf339de93a9920933d1fc39343df1c57cc01ed02a0918e96a3d87068` | `e5ddf2c06163880dfcdf13093604b48dea78faa2da87f17202e2ac1cef2c0927` | `ac44453e89c71275252d220aa8aa9f687e5e403323d167d01f7281faf1995f27` |
| norm-energy `🌎️hub/🧩️compositions/📕️norm/🧩️extensions/📒️energy/🧩️component` | 8 | None | `ea220cbb7620560c8b64eb7902e2cee2029a7259591d59010456d1cb11ab32d1` | `b53a417bb1d4ef9527be90e78f654867fbc5a6abcd91981d5bed5b6b93069e3d` | `044c70aa808b504d1dd9131831cba0fddbce89f3d252f6c4291fd3ffc8227ba8` |
| norm-eurocode `🌎️hub/🧩️compositions/📕️norm/🧩️extensions/📗️eurocode/🧩️component` | 32 | None | `548547a9734d487671fc385cc56ce17189b81d626f2d9d3e2e39da938866b500` | `f982cdd7f4b7b4343156d89f0d2fcbdc4bc96b0c1dd4bdb508b1652ac1165be2` | `ed7bcf2eb5414fed2d561d3292a8b77020c2d266a4ade6497b67b80720d9eb29` |
| norm-eurocode-materials `🌎️hub/🧩️compositions/📕️norm/🧩️extensions/📙️eurocode-materials/🧩️component` | 24 | None | `e98c2aa7557c9f8c7445ac13c17abbe1031afd7f22f6beaf99acc81a0c465957` | `22d57defd9d20f9ba82d473bee4e4e233fc198085b1e1ba45d53e4bebd0d2931` | `30b1a918b84c9b9477dd9c654205d7cc2d96c4f2c8214d8658ceb8ff8889a3b5` |
| norm-eurocode-structural `🌎️hub/🧩️compositions/📕️norm/🧩️extensions/📓️eurocode-structural/🧩️component` | 24 | None | `ba371f815ef594053b570f8ac8ada306f3be088304397d509ebe509353ebe4fa` | `f5a015fc2bbcded6f62d7358d36ec0df6bf762d83c3588d0977689f554ef7c45` | `6f186a579395dedb7957a3c3b50ad3feca533ea31997bf95974a293675eeaf1a` |
| playbook `🌎️hub/🧩️compositions/📖️playbook/🧩️component` | 6 | None | `0c2df8650dc2bba5c45f7ab2023a023edc84f3be4e0fc9f640fe30704648c0bd` | `753cbc43b969277a5717ed03092660ce4018d1aaebfb5d0c874ba2759c15cb3b` | `3fcfdc7e2542f9f2e7b75fa9c742d8d14200ad995c74bfac0b6a552234e6d466` |
| process `🌎️hub/🧩️compositions/🏭️process/🧩️component` | 1 | None | `f02dffe78dabf62b046f5a41cb7fcb30b32ca99a4127c4502225e8180f156f34` | `15ba7be053f486a7a54aa94a909cea2f061343d5258fd78ec0f1749c128081fe` | `a3934dd2e2f6cc7285877ea8f2e7370b3e69cad05f713eae72a9c4759a9ae7c8` |
| puzzle `🌎️hub/🧩️compositions/🧩️puzzle/🧩️component` | 6 | None | `f87ed2e60112f06ed26ede054eeeadf52e0b06d15d2c15fc7af45de8e9a92bcf` | `23189e4808ca901dc1669f29b16fc03d384de67a01ae3d04111f74501e35ade6` | `377450329dd40c30ef04fb59835da0d4d58d8fff853b6a523ae57debb1f8990f` |
| raster `🌎️hub/🧩️compositions/🖨️raster/🧩️component` | 2 | app-channel | `b4f53c32385c84a892cd5d25ce414748882f9af2d48ca4f1beb779c84bbd77bf` | `df1d21ec552d9c09065ee3a5d830d6275343b4e7838ead98b48292269dd6994d` | `32b6bf91739bd4899560c50cdb0bfd8d410985feb5701f92ff1b19a24d36e54a` |
| remodel `🌎️hub/🧩️compositions/📸️remodel/🧩️component` | 3 | None | `55c0f50cbb77c8992e91078ba631a1b097d6bc37b0a9051e7b3e28dd455ae3b4` | `dc4bc7091958c4a97a4859adebc21a3aba4d58d31f1841edf473539877f0eaea` | `8828bf34842ae089774b5ac1ccc111c172df8a1557c99fcfc9ca702992e73999` |
| sequence `🌎️hub/🧩️compositions/🎬️sequence/🧩️component` | 3 | app-channel, steps:in, text-carrier | `a94cf5287fcf412f92aa534e79da9a7af57e743e5944089f81fe134494173ece` | `37086c5cd18ac27ea1ae80edb806214e509dbe48d07670acde1bdb6ec2cb9fcd` | `0687ae7fd3f6868883fbb937bcb8dd82d6f129152985510370010038a0f9cddd` |
| shooting `🌎️hub/🧩️compositions/🎥️shooting/🧩️component` | 2 | app-channel, photos:out | `0ec65178b332b49597acc86ae4d0b9c5598a7ce4aa0728d2a368b84572bd436d` | `f70c3c9e97b30c839f04bcc50b37a33b20ba911b436dd9746aeb28d1641380f0` | `8a2a8d273e160c21cb2c4b942c5bbc297668303d5aaeff2fcbd4c536f0ab0d4a` |
| sourcing `🌎️hub/🧩️compositions/🪵️sourcing/🧩️component` | 4 | None | `f3fe95b55c064a5cdc934d841aa4812ca64c0b1207345e359c1b36f5b1dd01d0` | `05198661f352f67dbee87f1ef432b78fbf078b9abf96a1817fd2f32c832d8153` | `0fcc66ecc9d0ae550036e4ad694dac8c97e380a6242ad42d31f5caf3ab61402b` |
| space `🌎️hub/🧩️compositions/🪐️space/🧩️component` | 3 | app-channel, os.space-index | `57e6401ba61d1460a9af7bec163827c220fa565c0583d536b238fa44ab3ba136` | `792f495ed1a724b3523a9bc8a7d8d359b4ebb02bc3f9304516cf2307bc72c249` | `c06e30b06b857e0591b4bbc9a365184790db6ebf803869d2c23be7ea447fbe23` |
| vcs `🌎️hub/🧩️compositions/🌿️vcs/🧩️component` | 2 | app-channel, noOperation, text-carrier | `8fdb7bf563f43c32010e3f1dd4904161a9e59db98cb2c55b3c87a7e328813513` | `61521d1c9b41404ab86958b700ad3a3a0ffbf1f6ff76f40fa70e353d36f009ff` | `63da6ccbb75446596e606484f94981b019447204d52f859a0b9934cd09f499dd` |
| writer `🌎️hub/🧩️compositions/✒️writer/🧩️component` | 1 | app-channel, text:out, text-carrier | `e76734f5a6164bdeb436f9df3bbe026b266e91ac5fa99f4c7f549a5f20bcd11b` | `e28895f72dadc403376a658e3b22cbd2bb8cbc6f399fc385c20d4126deaf1772` | `833344e942174d34ab88ef9a4e722622b71ccb0e359d1afb8ccb685029f9baf0` |

## Existing release confirmations

Energy is the only confirmed root in this set that needs requeue. FEM, Flow extension BIM and Flow extension Dictionary have identical confirmed and authored descriptor digests.

| Root | Confirmed semantic SHA-256 | Authored semantic SHA-256 | Requeue |
| --- | --- | --- | --- |
| energy | `f42992705c59a41853f9f4cbbea455d88a7ec2f8791ca468e4508f7a115facb8` | `847b72debdc98091de1f0bbeae5f37323258c763977aa3920fa50d0c73753f06` | Yes |
| fem | `0af30e1828a9031a0a3f852c5e22afe8fc26d35b25da4b283c0688b32efd259d` | `0af30e1828a9031a0a3f852c5e22afe8fc26d35b25da4b283c0688b32efd259d` | No |
| flow-extension-bim | `60ae25a9a501601c6bd6597cbfaa466fd002ff8ba72def80aa5399c3c1ca521c` | `60ae25a9a501601c6bd6597cbfaa466fd002ff8ba72def80aa5399c3c1ca521c` | No |
| flow-extension-dictionary | `1da844b5f5b5704fd2545a6266a603323103c82edc823b6e7d3ac5f51ca335fb` | `1da844b5f5b5704fd2545a6266a603323103c82edc823b6e7d3ac5f51ca335fb` | No |

[Direct emitter replay and CAD and Procedural descriptor receipt audit](🔬️cad-descriptor-emitter.md)
