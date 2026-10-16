# Fresh SDK Publication Plan and Provisioning Wait Evidence

Source-only checkpoint at 2026-10-15T12:52:41.210458+00:00. No SDK publication, test, Native operation, cleanup, lease mutation, or production Source edit was performed for this report.

## Current wait evidence

Root release 43539 expired at the unchanged 120-second Original. Its retained stages establish 85.957 seconds awaiting actual `nx-tooling-provision` SH; complete parallel source capture was 11.603 seconds. The first projection completed at 107.586 seconds and the quote at 114.142 seconds; copy did not complete. This result does not establish a capture-performance regression.

Before Root's quiescence successor, a read-only `lsof` observation returned no open descriptors for the canonical provisioning database. During successor 38481, `lsof` observed only PID 38054; `ps` identified its parent as 38050 and its command as the actual `bun test .../capacity/🧪️tests/📤️derivative/🟦️.ts --test-name-pattern=issued derivative copy` child. Open-descriptor evidence alone does not identify SH/EX ownership. Neither observation identifies the historical 85.957-second holder. No other process was stopped, probed through its service, or modified.

The Source lifetime is intentional: `withManagedToolingProvisionV1` acquires exact registry EX before its callback and retains it through callback settlement and all accepted serialized publication tasks. Each task rechecks custody/statfs and owns its complete publication. `activateNxTools` holds registry SH plus exact-generation SH through derivative validation and activation. `withNxToolingUseV1` holds the generation SH and base reader for actual use; it does not itself request registry SH. These scopes can create a genuine wait; no particular scope is proven to have caused the historical wait.

Root's successor 38481 subsequently settled with a physical capacity refusal before copy; see the headroom checkpoint report. It is no longer live. No concurrent copy, failure test, or normal publication is authorized by this plan.

## Exact normal entry and prebirth authority

Use the existing normal entry `bun repo tools setup nx`, already registered in both editor catalogs as `🛠️ Build Tools · Set Up Nx [EN]` and `🛠️ Build-Werkzeuge · Nx Einrichten [DE]`. `package.json` routes `repo` through the actual build `📜️script.ts`; no new executable or private installation path is required.

The root invocation must be born once with the actual build capabilities computed by `createBuildBootstrapCapabilitiesV1` before `createScriptProcessEnvelope`. The Repo request is `['repo','tools','setup','nx']`; the root has the canonical Repo role (`tool:'nx'`) and declares the actual Nx `['nx','--version']` descendant via the existing factory. The child receives closed DATA through the canonical serializer; it does not receive a copied private Original or an invented grant. Existing incoming authority and its deadline must be preserved.

For a fresh execution with no incoming Original, a finite source-supported invocation is:

```sh
BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 NODE_DISABLE_COMPILE_CACHE=1 SEMIO_BUILD_MODE=prototype SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:build","maximumElapsedMilliseconds":120000}' bun repo tools setup nx
```

This is a proposed 120-second finite root for the later authorized run, not a run already executed. The normal build Source defaults to zero/unlimited only when no policy/incoming Original is authored; this plan does not rely on that default. A timeout must remain a failure and cannot be renewed by reissuing a root within the operation. The authorizer compiler's persisted mode is `release` independently of the explicit root mode. The normal `--version` setup branch enters tools storage directly and does not need a parent Rust/graph reader.

## Publication and exact receipt handoff

Actual production capture must freshly discover both runtime and compiler defining closures with the production Bun scanner. The Source-only hashes below are review stamps, not a replacement for its exact captured manifests. Capture actual recipe, pinned lock, patch bytes, runtime manifest, compiler manifest and private input bytes before deriving identity. Retain the verified pristine base SH through every dependency use, and use only the genuine derivative quote, provision EX, fresh statfs admission and single-use captured-input/clone operations. Ready reuse must authenticate the exact current requested identity and complete installed receipt; an older active generation is not proof for changed Source.

Collect the terminal process exit, canonical activated generation identity, `.semio-nx-tooling.json`, `.semio-authorizer-source.json`, `.semio-authorizer-compiler.json`, `.semio-sdk-material.json`, installed runtime output SHA and staging/final Node preflight. At this Source observation the mandatory contract has 38 names. Preflight must use the captured current contract, not a hard-coded historical 31 or 35. Record the exact runtime/compiler manifest Source path/SHA rows and output receipt digest; compare each captured row with current Source before proceeding. A concurrent genuine Source change must be recorded and a coherent defining capture chosen, never ignored.

No full fresh-current manifest or new-generation success is asserted here. The independent regex Source census in `sdk-fresh-capture-source-independent-current.md` is approximate and cannot substitute for the Bun capture.

## Required gates after publication

First run the actual authenticated Source-to-frozen callback gates, separately and sequentially:

```sh
bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/issuer-bootstrap/📜️script.ts' sdk prototype
bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/issuer-bootstrap/📜️script.ts' sdk release
```

Each existing entry owns its finite 120-second Original. It authenticates tools storage, active exact generation, immutable bundle receipt/output SHA and base/generation custody, imports the installed bundle, calls actual `withGraphPublisherControlV1`, and requires Source's private issued control plus identical AbortSignal. The older 60f result is a retained RED and is not current qualification. If either new current gate fails, withhold Native and ordinary claims; no structural registrar or global identity fallback is authorized.

After BOTH callback gates pass, qualify current frozen Native `nx-product-cache` prototype then release with actual cross-graph/FK/output custody. Only then run unchanged `rust-unit-profile` prototype then release, sequentially, preserving each existing 240-second test limit. Record all seven compiler counts, actual task hashes/cache policy, output/deleted restore inventory, changed leaf/unchanged sibling evidence and phase/wait timers. No material/cleanup tests or SDK activation changes between pair phases.

## Review stamps

These exact individual working Source bytes were read at this checkpoint. This table is intentionally not described as the exact production authorizer closure.

| Source | Bytes | SHA-256 |
|---|---:|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📜️script.ts` | 24300 | `ef914cf1ee34a5bdad073de09b8d64c5390d3a81fc57c0fcbe5005544e9bd750` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📥️loading/🔣️contract.json` | 1201 | `285f94b07540bf10c2213b80dc2a8b0e3a32426ba03de57a3ed406cf41540302` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️runtime/🟦️.ts` | 3837 | `e78729ce3b83556ae49bd8c1d6b839159cda0b908cc80e343c3b4c372f0ac06f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📜️script.ts` | 4214 | `8ccfa017b8c1c06fbca43caed4c0e154570edd54e510e85581fd9575939b60ad` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts` | 17657 | `2b57306a9f3b3bb33723e222e4774ce53ad0e4c25465946e58a68b9bea35d89d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts` | 12266 | `661c0b23ac40cf6ec921e7c6c2379aecb7217084a47a75ca887c9cd312fa7b8d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts` | 17195 | `0c4dbd06a6180b1359e69d224fefd5cc01f5695ea28149a1c140eda1d19b606c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts` | 25195 | `2edac19c3f8e802298e499ada9e893dfc53ca46b6353834b8f7265cabad57658` |
