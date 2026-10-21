# Guest Own7 Unit Log Watch

Root authorized sole Cargo-owned unit under own seal73bd45daa386ed57dc6c5743414a92f1ef36d1decf45c04c2fb5076d7579bb2c. Cargo reported actual live TTY79672; only Cargo polls it. This reviewer reads logs only and cannot infer actual terminal.

First bounded descriptor observation: dev16777229/ino456656775, mode0600, nlink1, size13337B, examined[0,13337).

```text
[capacity] waiting 0s for exec: not enough free memory (need 3584 MB; 771 MB spare above the 6553 MB floor, load 15.21; 7 running, 0 ahead)
[capacity] waiting 5s for exec: not enough free memory (need 3584 MB; 907 MB spare above the 6553 MB floor, load 15.6; 7 running, 0 ahead)
[capacity] admitted exec with 3 of 8 job(s): the host cannot carry more right now
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
[script] {"owner":"repository:build","command":"repo","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"boot-recovery:observe:0/0","mode":"exclusive","elapsedMs":88}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"boot-recovery:observe:0/0","mode":"exclusive","elapsedMs":17}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:4096","mode":"shared","elapsedMs":204.26449999999932}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:8192","mode":"shared","elapsedMs":330.7463749999988}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:12288","mode":"shared","elapsedMs":348.8663749999996}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:16384","mode":"shared","elapsedMs":365.5355839999993}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:20480","mode":"shared","elapsedMs":383.33529199999975}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:24576","mode":"shared","elapsedMs":399.9404999999988}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:parser-source:list:64","mode":"shared","elapsedMs":0}}
[script] repository:command · Listing source entries: 64 operations
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:parser-source:list:32448","mode":"shared","elapsedMs":0}}
[script] repository:command · Listing source entries: 32448 operations
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:parser-source:state:58432","mode":"shared","elapsedMs":0}}
[script] repository:command · Inspecting source metadata: 58432 operations
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:parser-source:state:69120","mode":"shared","elapsedMs":0}}
[script] repository:command · Inspecting source metadata: 69120 operations
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:parser-source:state:84928","mode":"shared","elapsedMs":0}}
[script] repository:command · Inspecting source metadata: 84928 operations
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:parser-source:state:101504","mode":"shared","elapsedMs":0}}
[script] repository:command · Inspecting source metadata: 101504 operations
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:parser-source:state:127040","mode":"shared","elapsedMs":0}}
[script] repository:command · Inspecting source metadata: 127040 operations
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:parser-source:state:152512","mode":"shared","elapsedMs":0}}
[script] repository:command · Inspecting source metadata: 152512 operations
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-source-provider:1","mode":"shared","elapsedMs":50.60162499999933}}
[script] repository:command · Checking declared package providers: 1 edges
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-source-provider:164","mode":"shared","elapsedMs":300.9469170000011}}
[script] repository:command · Checking declared package providers: 164 edges
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-source-provider:328","mode":"shared","elapsedMs":551.2700420000001}}
[script] repository:command · Checking declared package providers: 328 edges
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-source-provider:526","mode":"shared","elapsedMs":803.4526250000017}}
[script] repository:command · Checking declared package providers: 526 edges
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-source-provider:697","mode":"shared","elapsedMs":1054.0614999999998}}
[script] repository:command · Checking declared package providers: 697 edges
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-source-provider:867","mode":"shared","elapsedMs":1304.2128340000017}}
[script] repository:command · Checking declared package providers: 867 edges
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-source-provider:1103","mode":"shared","elapsedMs":1554.3533339999994}}
[script] repository:command · Checking declared package providers: 1103 edges
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-source-provider:1104","mode":"shared","elapsedMs":1651.7385419999991}}
[script] repository:command · Checking declared package providers: 1104 edges
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:parser-source:frontier:234624","mode":"shared","elapsedMs":0}}
[script] repository:command · Rechecking source evidence: 234624 operations
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:parser-source:frontier:316672","mode":"shared","elapsedMs":0}}
[script] repository:command · Rechecking source evidence: 316672 operations
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:4096","mode":"shared","elapsedMs":34.42804199999955}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:8192","mode":"shared","elapsedMs":64.80704200000037}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:12288","mode":"shared","elapsedMs":109.06495899999936}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:16384","mode":"shared","elapsedMs":145.0415839999987}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:20480","mode":"shared","elapsedMs":176.88416700000016}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:24576","mode":"shared","elapsedMs":209.78262499999983}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:28672","mode":"shared","elapsedMs":243.69325000000026}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:32768","mode":"shared","elapsedMs":279.89629200000127}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:36864","mode":"shared","elapsedMs":315.8792089999988}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:40960","mode":"shared","elapsedMs":350.8163749999985}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:45056","mode":"shared","elapsedMs":385.34591699999874}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:49152","mode":"shared","elapsedMs":419.666792}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:53248","mode":"shared","elapsedMs":455.38500000000204}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:57344","mode":"shared","elapsedMs":492.9479170000013}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:61440","mode":"shared","elapsedMs":531.877375}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:65536","mode":"shared","elapsedMs":568.5548750000016}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:69632","mode":"shared","elapsedMs":604.114375000001}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:73728","mode":"shared","elapsedMs":640.4383339999986}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:77824","mode":"shared","elapsedMs":669.6909169999999}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:4096","mode":"shared","elapsedMs":928.5145420000008}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:8192","mode":"shared","elapsedMs":1038.3257090000006}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:12288","mode":"shared","elapsedMs":1069.0002920000006}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:16384","mode":"shared","elapsedMs":1086.646084}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:20480","mode":"shared","elapsedMs":1105.9526670000014}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:24576","mode":"shared","elapsedMs":1123.5266249999986}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:28672","mode":"shared","elapsedMs":1143.1209589999999}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:32768","mode":"shared","elapsedMs":1161.179959000001}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:36864","mode":"shared","elapsedMs":1181.1356669999986}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:40960","mode":"shared","elapsedMs":1199.2612090000002}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:45056","mode":"shared","elapsedMs":1218.9789170000004}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:49152","mode":"shared","elapsedMs":1236.651167}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:53248","mode":"shared","elapsedMs":1256.2096669999992}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:57344","mode":"shared","elapsedMs":1274.7910840000004}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:61440","mode":"shared","elapsedMs":1294.961459000002}}

```

Bounded sequential observation size28815B, extent[13337,28815), same log identity.

```text
generation-verify:32768","mode":"exclusive","elapsedMs":4781.658874999997}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:36864","mode":"exclusive","elapsedMs":4800.543999999998}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:40960","mode":"exclusive","elapsedMs":4820.145582999998}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:45056","mode":"exclusive","elapsedMs":4838.237957999998}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:49152","mode":"exclusive","elapsedMs":4857.880665999997}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:53248","mode":"exclusive","elapsedMs":4877.126708}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:57344","mode":"exclusive","elapsedMs":4896.52925}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:61440","mode":"exclusive","elapsedMs":4916.075333000001}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:65536","mode":"exclusive","elapsedMs":4934.882165999999}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:69632","mode":"exclusive","elapsedMs":4954.523540999999}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:73728","mode":"exclusive","elapsedMs":4973.952708000001}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:77824","mode":"exclusive","elapsedMs":4994.271999999997}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:81920","mode":"exclusive","elapsedMs":5013.002832999999}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:86016","mode":"exclusive","elapsedMs":5048.244415999998}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"shared","elapsedMs":20}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":7,\"queued\":7,\"holderMode\":\"shared\",\"holderPid\":37466,\"holderSeconds\":2080,\"resource\":\"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g\"}","mode":"shared","elapsedMs":0}}
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 7 of 7, behind the request of process 37466 (shared, queued 2080 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"shared","elapsedMs":15189}}
[script] repository:command remains active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared, 15 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":7,\"queued\":7,\"holderMode\":\"shared\",\"holderPid\":37466,\"holderSeconds\":2096,\"resource\":\"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g\"}","mode":"shared","elapsedMs":0}}
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 7 of 7, behind the request of process 37466 (shared, queued 2096 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"shared","elapsedMs":30302}}
[script] repository:command remains active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared, 30 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":7,\"queued\":7,\"holderMode\":\"shared\",\"holderPid\":37466,\"holderSeconds\":2111,\"resource\":\"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g\"}","mode":"shared","elapsedMs":0}}
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 7 of 7, behind the request of process 37466 (shared, queued 2111 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"shared","elapsedMs":45385}}
[script] repository:command remains active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared, 45 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":7,\"queued\":7,\"holderMode\":\"shared\",\"holderPid\":37466,\"holderSeconds\":2126,\"resource\":\"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g\"}","mode":"shared","elapsedMs":0}}
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 7 of 7, behind the request of process 37466 (shared, queued 2126 s ago)

```

Sequential examined extent now[0,41874), size41874B, same identity. Latest queue still native-fixed-image position7of7; no unit success/error marker in examined extents. Actual terminal remains owner-only.

Sequential read through55985B, same log identity. Latest bounded tail:

```text
s active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared, 498 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":7,\"queued\":8,\"holderMode\":\"shared\",\"holderPid\":37466,\"holderSeconds\":2579,\"resource\":\"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g\"}","mode":"shared","elapsedMs":0}}
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 7 of 8, behind the request of process 37466 (shared, queued 2579 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"shared","elapsedMs":513504}}
[script] repository:command remains active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared, 514 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":7,\"queued\":8,\"holderMode\":\"shared\",\"holderPid\":37466,\"holderSeconds\":2594,\"resource\":\"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g\"}","mode":"shared","elapsedMs":0}}
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 7 of 8, behind the request of process 37466 (shared, queued 2594 s ago)

```

Sequential examined extent[0,76893), same original dev/ino, ordinary singlelink0600. Latest progress remains image queue7of9; no unit completion/error marker in this examined extent. No terminal inferred.

Material transition in[76893,80993): native-fixed-image request now exclusive, queue3of7 rather than prior shared7of9; latest new-request elapsed45659ms. Same earliest ticket37466/shared. No unit marker or error in examined extent.

Sequential extent[0,96165), same original identity. Queue advanced to exclusive2of6, earliest reported ticket40609/shared, replacing37466. Latest bounded tail:

```text
":"lease-queue:{\"version\":1,\"position\":2,\"queued\":6,\"holderMode\":\"shared\",\"holderPid\":40609,\"holderSeconds\":2955,\"resource\":\"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g\"}","mode":"exclusive","elapsedMs":0}}
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (exclusive): position 2 of 6, behind the request of process 40609 (shared, queued 2955 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"exclusive","elapsedMs":287793}}
[script] repository:command remains active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (exclusive, 288 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":6,\"holderMode\":\"shared\",\"holderPid\":40609,\"holderSeconds\":2970,\"resource\":\"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g\"}","mode":"exclusive","elapsedMs":0}}
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (exclusive): position 2 of 6, behind the request of process 40609 (shared, queued 2970 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":0}}

```

Sequential examined extent[0,130878), same original ordinary singlelink0600 log. Latest state exclusive image queue2of6; no success/error marker in examined extent, actual terminal unconfirmed by reviewer.

Correction to immediately preceding summary: latest[105337,130878) has advanced beyond queue into repository:graph-headroom-retirement exclusive operations. The prior queue2of6 was historical. No unit success/error marker occurred in this new extent.

Sequential examined extent[0,180657), same original identity. After graph-headroom transition, shared image queue2of4 behind57763/exclusive was followed by graph-facts shard7 exclusive queue2of2 behind73118/exclusive. No unit completion/error marker in examined extent.

Material advancement: fixed sequential extents through257937B passed shard queue into discovery/playground/Cargo target construction and graph inference0of3736, then command-source imports/packages. No unit completion/error marker in examined[0,257937).

## Owner Confirmed Terminal

Root attributes Cargo actual TTY79672 terminal1 at03:05:53Z. Frozen full log261624B/SHA2561678720826a5254119070d2e44e33abaef77f3e1163018d187dd9c0ef52677b6 independently matches. Sequential examined full extent[0,261624); zero captured-neutral pure validation passed markers. Startup Source contract refusal occurred before own7 test entry; no own7 suite pass is claimed.

Final refusal context:

```text
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:1:3736","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 1 of 3736 projects
92 | /** 🏃️ Admits one declared source or native command owner against its closed production contract. */
93 | export function targetExecutionKindV1(target, identity) {
94 |   const value = target.metadata?.semio?.execution;
95 |   if (value === undefined) return undefined;
96 |   const schema = Object.values(targetExecutionSchema.definitions).find(schema => schema.properties.kind.const === value?.kind);
97 |   if (!schema || !conformsOwnerContractV1(schema,value)) throw Error(`Repository target execution contract refused${identity ? `: ${identity}` : ""}`);
                                                                    ^
error: Repository target execution contract refused: @semio-tech/mit-bestand-demonstrator:test
      at targetExecutionKindV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎯️target-policy/🟨️.mjs:97:64)
      at projectWithDefaults (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs:250:66)
      at processTicksAndRejections (unknown:7:39)

Bun v1.3.14 (macOS arm64)

```

Own seal remains73bd45daa386ed57dc6c5743414a92f1ef36d1decf45c04c2fb5076d7579bb2c; all seven post-read bytes/SHA match:

```json
[
  {
    "file": "schema.json",
    "bytes": 19119,
    "sha256": "caa81c930983e19a5a3f500f954718143ae1775bda89c7c3e32d0b4d92deec68",
    "matches": true
  },
  {
    "file": "cases.json",
    "bytes": 7297,
    "sha256": "3d060f22f969b1601d886245ba01809b86e927030f7acf191f2d20d57f40a825",
    "matches": true
  },
  {
    "file": "core.ts",
    "bytes": 13502,
    "sha256": "d7632ccddb32435d9369dc8e84898bf23167a2224ed3eed33b17ef819a239653",
    "matches": true
  },
  {
    "file": "guest.py",
    "bytes": 20695,
    "sha256": "86f0957445bf5f5a13dba50059c01e56dde70d65355306a5a0d45ecbb9b97499",
    "matches": true
  },
  {
    "file": "entry.ts",
    "bytes": 11867,
    "sha256": "8f05146298e5b561aa964751810955208bb581686ff25b255da66f4d6ade1e1d",
    "matches": true
  },
  {
    "file": "tests.ts",
    "bytes": 8439,
    "sha256": "9bbaab5eeb85c304a411899b4b67271448e5fd667bff882b9b25ff8bf56c36ed",
    "matches": true
  },
  {
    "file": "oracle.py",
    "bytes": 3259,
    "sha256": "75fb2f0b74be2ee8d0095167667ac48362da07d6740d2795f0d37c22ee2dd823",
    "matches": true
  }
]
```

Watch ended. No TTY poll or cancellation performed by reviewer.
