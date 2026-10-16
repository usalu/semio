# Bounded Tool Archive Foundation

2026-10-15. Source implementation checkpoint in progress. No download, Tectonic process, image, SDK publication or cache retirement is scheduled by this work.

Print still invokes raw `tar` on a compressed archive before inspecting its extracted files. The compressed pin alone does not constrain inflated bytes or prevent an archive entry escaping its destination. Its six tar.gz and one ZIP distribution need a first-party, bounded decoder before the actual funded publisher can materialize entries. The existing SDK tar decoder already owns strict framing and an npm package-prefix policy. Duplicating that decoder in Print would leave two diverging framing implementations.

Extract the common gzip/tar framing into a domain-neutral Core archive owner with an explicit closed finite policy and genuine carried control. Keep the SDK's exact64MiB/4096-entry/16MiB-file policy, package-prefix projection, SHA512 identity, fatal cancellation behavior and existing callers/tests. Add ZIP central/local/descriptor/CRC agreement, path/node/duplicate checks and inflated bounds in the same Core taxonomy. Caller policy DATA cannot issue storage, network or compiler authority. ZIP64, encrypted, special-node, ambiguous-name and unsupported framing refuse before materialization.

Neutral JSON/schema and independent Bun tar/fflate ZIP plus SQLite fixtures precede Source. Run through the existing editor-registered finite ticket routes with fresh genuine receiver custody per test child. Preserve failed epochs. This foundation does not by itself migrate Print preparation, grant a new cache destination, or qualify pinned release bytes/platform executables. Full Print producer/consumer migration remains required.

## Implemented Source and Executed Byte Laws

The domain-neutral Core owner validates a closed copied request and compressed input snapshot before asynchronous decoding. Its hard DATA ceilings are32MiB compressed,256MiB inflated,4096 entries,256MiB per file,256 UTF8 path bytes and64KiB metadata. Actual domain policy can only narrow these. SDK keeps its exact32MiB compressed/64MiB inflated/4096 entries/16MiB file policy and package-prefix projection. The actual SDK wrapper now calls the shared decoder; its six existing laws/corpus remain, with the obsolete domain-specific refusal string replaced by the shared refusal prefix. No external runtime dependency was added. Bun archive and fflate are independent test oracles only.

Tar verifies POSIX framing, checksum, octal sizes/modes, complete padded bodies and footer, bounded PAX/GNU metadata, and rejects links/sparse/special entries, unsafe paths and normalized collisions. Footer zero scans now check actual control in64KiB chunks and yield perMiB. ZIP validates the complete central roster before inflation, exact local names/methods/flags/versions/sizes, contiguous nonoverlapping local extents, single-disk footer, metadata/node/path policy, declared/actual inflated bytes and real CRC32. Both signed and unsigned descriptors are checked against actual CRC/sizes, including a real payload with CRC equal to the descriptor signature. Native decompression completion is joined in finally; progress/CRC/footer loops consume the carried control. Bytes/rosters convey no filesystem, storage, tool or network authority.

The closed language-neutral corpus is independently Ajv-validated; Bun tar/fflate stored,deflated and streamed ZIP supply byte oracles; SQLite supplies inventory bytes. Tests cover Unicode content and ZIP paths, NUL/empty files, package projection, unsafe paths, normalized collisions, corrupt headers/padding/footer/link/sparse entries, exact central/local disagreement, encrypted/ZIP64/multidisk/metadata refusal, CRC and descriptor corruption, raw-deflate trailing bytes with repaired extents, local overlap/gap, alternate Unicode extra-name refusal, byte/file/entry limits, request/input snapshot isolation, and genuine preflight/accepted-start cancellation preserving its exact Error. A2MiB tar footer agrees with Bun's actual reader. Actual mid-native-inflate and >1MiB CRC cancellation timing remains an additional runtime test gap; no hostile filesystem materialization was attempted.

The SDK tests previously received one process envelope separately in six Bun tests. Running the preserved laws exposed one-birth failure; all six now use the genuine existing receiver-owned suite helper, retaining one real process birth and fatal law drainage. No receiver/reset/structural control was invented. The ticket wrapper runs Core and SDK archive children separately before the existing broad Source suite/default configuration child, under the same authored Original120s envelope. Existing editor routes are retained.

## Operation Ledger

- 50771 failed on the initial test's wrong routing import; fixed the relative path.
- 91354 then reached the genuine missing shared decoder RED.
- 79151/26465 failed while Bun.Archive authored the initial oracle: Uint8Array input, then Unicode filename header. The fixture now uses Bun's supported string/ASCII path oracle with Unicode/NUL content; Unicode ZIP filename oracle remains independent and passes. These were harness failures, not production decoder defects.
- 12854 failed because the test mutated deflate bytes but expected a shared policy-error prefix; its actual native decompressor refusal was valid. The targeted CRC vector now uses an independently encoded stored ZIP, while raw-deflate framing has its own vector.
- 88504 preserved the six-test receiver failure;60807 corrected that lifetime and passed all four child suites.54408 release counterpart also passed. Expanded strict9450/97311 exited0 at that checkpoint.
-The wire-law RED (bounded-archive-wire-red1.log, immediate exit1,66 assertions) proved ambiguous DOS directory metadata was accepted. Production now refuses that mismatch and exact local/central needed-version disagreement;12266 passed the successor.
- 77468 prototype passed Core1/87, SDK6/40, broad119/1973 and default4/301.58792 release passed the two archive children, then the moving aggregate failed113pass/2fail/2errors; it is not a full pass.
- 85122 prototype and31561 release passed Core1/94, SDK6/40, broad119/1973 and default4/301. Expanded strict98317 prototype exited0. Final footer-yield successor63619 prototype and73102 release both passed Core1/96, SDK6/40, broad119/1973 and default4/301:130 tests/2410 assertions per mode.73102 exited0 at22:24 UTC; no physical operation ran.
-Intermediate strict14951 failed on in-flight Native withOperation omissions;49641 failed on in-flight generic Cargo consume/maximumObservationBytes call sites;5107/51492 failed in the moving Native/graph source epoch, including image-before-assignment. They remain failed, and no strict completeness is inferred from absence of own diagnostics. Final BOTH strict after the shared provider/store migration is still required.

All Root archive handles listed here are terminal. There was no download, Tectonic execution, filesystem extraction, new SDK publication, cache/image/namespace transaction or compiler operation. This Source foundation does not close Print's actual old preparation/publisher/readers; that full lane is now assigned separately. It also changes sealed SDK code identity, so historical SDK runtime proofs do not qualify its next ordinary activation.

## Current Source SHA256

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🟦️.ts`: `d420fc272ccf21b8d3a8d0b12af428fb69441ba4a985c3ecc1518fa4104b7a8c`; 16794B; absolute UTF16 units 93.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🧬️schema/🔣️.json`: `12b1c9e307887c531a77d26bf3858f1d93fd04061f9ccd07b3f49910feea5832`; 1429B; absolute UTF16 units 105.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🧪️tests/🟦️.ts`: `5df35ec61f528834fa64f6f13937a5b5613289a05474286a1c05eb63adb2ca75`; 12425B; absolute UTF16 units 102.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🧪️tests/🔣️.json`: `9b2a2119dc10ec3d402d8ba6693fad26241bab835fe8b6482c356d6616742a1d`; 1422B; absolute UTF16 units 104.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🧪️tests/🧬️schema.json`: `3a782466f577c583e3a23f86c0e8ac8f53b082cd09e122ebfa55f4c484936bfe`; 2438B; absolute UTF16 units 110.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🗃️archive/🟦️.ts`: `df9165b17202f4e377bf6c46ea15d9d414da24b752d7d69b3e4ad5c74d2db3bb`; 1299B; absolute UTF16 units 149.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🗃️archive/🧪️tests/🟦️.ts`: `c73d889a74194244dc3fb118b354b569e2f97a883b63f3c71610a6c216561acc`; 10417B; absolute UTF16 units 158.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/sdk-derivative/📜️script.ts`: `a61af87c9505cf0faf12160a3e0167edaf5f6ec33dbadbf45b6b93459a665a6e`; 25849B; absolute UTF16 units 142.

## Retained Generated Evidence SHA256

- `bounded-archive-final-release1.log`: `6bd24f0700a9bcfa029575f8081bd4c4b261ef555bc57f4c03fd8165b03ac0bd`; terminal73102 exit0;1/96,6/40,119/1973,4/301.
- `bounded-archive-prototype1.log`: `f50a2d10713c7a54c273a87f6b9aefc1e2a7e10f6097fa7dcff4328868756ec6`; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls; 0 pass; 1 fail; 1 expect() calls.
- `bounded-archive-prototype10.log`: `8617d35a77748ca72573edb7b9dd52b342d2f50265bf33485155c7c3d42f0606`; 1 pass; 0 fail; 87 expect() calls; 6 pass; 0 fail; 40 expect() calls; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls.
- `bounded-archive-prototype11.log`: `5016eb0b3b4cd0b1fa4ba6b60c17df81e723f526306b182087db5bba766f40ee`; 1 pass; 0 fail; 94 expect() calls; 6 pass; 0 fail; 40 expect() calls; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls.
- `bounded-archive-prototype12.log`: `6f12ba44fb0a64caafc6da6f547cc222e2b77e9cbbe8ebe1eed5eb5bc6e18865`; 1 pass; 0 fail; 96 expect() calls; 6 pass; 0 fail; 40 expect() calls; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls.
- `bounded-archive-prototype2.log`: `623a36525430d6c56773ab6584330ca313567d841e0259592f33bc991d6897c1`; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls; 0 pass; 1 fail; 1 expect() calls.
- `bounded-archive-prototype3.log`: `379f537707eeeafef1508822db2f23b76929d03b2201b4d531cf16e17c12f6a2`; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls; 1 pass; 0 fail; 23 expect() calls.
- `bounded-archive-prototype4.log`: `7965587d2fda4ff34ba4c6a4313ede9794afb7bdd0d5c241c5cbb1e9ac1af464`; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls; 0 pass; 1 fail; 52 expect() calls.
- `bounded-archive-prototype5.log`: `a664a01ab604e863466d5f6e7a032caa977b4e5c38be40b3a12b896872bee7d8`; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls; 1 pass; 0 fail; 78 expect() calls.
- `bounded-archive-prototype6.log`: `90c571e4af9827ad4d0fe63acc8c04317f0a330094f992b6c8d291c677614baa`; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls; 1 pass; 0 fail; 78 expect() calls; 1 pass; 5 fail; 11 expect() calls.
- `bounded-archive-prototype7.log`: `f415ffd1a2fdd8a3ac26d0d49f7300f33af2dc6fbe6d8597410ca0da1df420c9`; 113 pass; 2 fail; 1897 expect() calls.
- `bounded-archive-prototype8.log`: `bc4504871cae18efea8926d8ae20cbe319e1d901b4a2c543764ce65aa834cb9c`; 1 pass; 0 fail; 78 expect() calls; 6 pass; 0 fail; 40 expect() calls; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls.
- `bounded-archive-prototype9.log`: `e650ee074e9c788034881788ac629798719a6dab744a0ede2d04b887ed031df1`; 1 pass; 0 fail; 82 expect() calls; 6 pass; 0 fail; 40 expect() calls; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls.
- `bounded-archive-red1.log`: `29932928e68193719629f2acb800d36914a695803dd3b7e6f2adc849401ac03c`; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls; 0 pass; 1 fail.
- `bounded-archive-red2.log`: `eadcf07ce4c22bcf28efdf734c5e5f3108c8964e53db16d372177a52b4402aea`; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls; 0 pass; 1 fail.
- `bounded-archive-release1.log`: `c6abdb3ebf0398323e0308ee41dff02cce0593bb5bdc229142bfcf18d91aab06`; 1 pass; 0 fail; 78 expect() calls; 6 pass; 0 fail; 40 expect() calls; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls.
- `bounded-archive-release2.log`: `3e5516489d9526dce4fe0ce820f2cf6d7d0a8720b4a69db49f31913ab4b09ddd`; 1 pass; 0 fail; 87 expect() calls; 6 pass; 0 fail; 40 expect() calls; 113 pass; 2 fail; 1897 expect() calls.
- `bounded-archive-release3.log`: `44d107f1f83f1eca3d13a4bcbe95902d8323624166c36beaa05c405ede37af50`; 1 pass; 0 fail; 94 expect() calls; 6 pass; 0 fail; 40 expect() calls; 119 pass; 0 fail; 1973 expect() calls; 4 pass; 0 fail; 301 expect() calls.
- `bounded-archive-types-prototype1.log`: `d4d56377344949932039be40778970605ec235073ea96b697b9e60a4d9ec64fd`; see defining operation/error below.
- `bounded-archive-types-prototype2.log`: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`; see defining operation/error below.
- `bounded-archive-types-prototype3.log`: `b0e6a1bfd228946ca9883a4f32d775b17f305461dd26bd1d1910e054b7c5dbbd`; see defining operation/error below.
- `bounded-archive-types-prototype4.log`: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`; see defining operation/error below.
- `bounded-archive-types-prototype5.log`: `91568252ccece0a3a38422ab8f27641aced97c2901d0cdb5219b808dfc62580e`; see defining operation/error below.
- `bounded-archive-types-release1.log`: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`; see defining operation/error below.
- `bounded-archive-types-release2.log`: `91568252ccece0a3a38422ab8f27641aced97c2901d0cdb5219b808dfc62580e`; see defining operation/error below.
- `bounded-archive-wire-red1.log`: `bf13f53e57ed0944a9290a5fda7b041cb9051c23f92be0147391b850dda8b405`; 0 pass; 1 fail; 66 expect() calls.
