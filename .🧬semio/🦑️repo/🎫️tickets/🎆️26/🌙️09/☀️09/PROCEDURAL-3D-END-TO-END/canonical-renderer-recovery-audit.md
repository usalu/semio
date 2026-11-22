# Canonical Renderer Recovery Audit

## Current evidence

The restored React root log reports ENOSPC in the managed rust image and 801 compiled units, zero components staged; its later Cargo profile Source mismatch is secondary. The WGPU generation3d launch log was still in playground:components at the time of inspection. No build was launched in this audit.

## Supported narrow path

The generated playground catalog names generation3d as the actual procedural editor and generator as the demonstrator (generated playgrounds TypeScript lines 64–65). The canonical playground driver already computes the app-scoped boot closure, filters out physically prepared components, and builds only the missing members (repo playground TypeScript lines 104–106, 116–144; component driver stages TypeScript lines 134–156). It still needs the real procedural component and its declared dependencies: selecting fewer manually would not establish a real production boot. React and WGPU share this boot preparation, after which WGPU separately ensures its renderer publication (playground lines 147–169). The active WGPU log reports a 13-plugin generation3d session; session staging alone is not proof that browser components were published.

The driver requests the warm generation and component compiler lanes use the shared unit cache unless SEMIO_UNIT_CACHE=off (driver stages lines 134–148; compiler lane TypeScript lines 32–36 and 45–53). Reusing it through the canonical launch is supported; isolated or disabled caches need additional storage and do not solve the full image. No supported public delivery-slot selector that bypasses this closure was found.

## Storage limits

The prior cache ledger reported 93.56 GiB used inside the 96 GiB rust image. This is a historical observation, not a new measurement. The prior cache-gc dry run showed zero reclaimable bytes and CAS was below its watermark. Repo cache help states Cargo is owner-managed and generic cache gc does not reclaim its live store. The component driver already invokes managed pressure reclamation and ENOSPC retry (driver stages line 105). Its unit collector reclaims inactive incremental sets then stale units under single-flight and lease protection (unit-cache gc TypeScript lines 171–180); compiler recovery reclaims finalized compact units and idle lanes (compiler capacity recovery TypeScript lines 17–26). No manual deletion of live or unverified namespaces is justified.

The rust elastic policy maximum is 103080263680 bytes (96 GiB), with automatic growth enabled but bounded by that maximum and host reserve. Budget elastic TypeScript lines 22–25, 39–46 and budget TypeScript lines 36–39 show that an existing image at its cap cannot automatically expand beyond it, regardless of host free space. Increasing the cap would require a deliberate supported policy and image lifecycle change; it is not an available selector. Wait for the active WGPU result and its managed recovery before considering further action.

## Live headroom and lane custody

At the latest read, statfs reported 14,514,470,912 bytes free of 103,079,215,104 bytes in the rust image and 65,362,886,656 bytes free on the host. These figures are momentary. The failed React lane pb19e00 was absent from the current unit-cache lanes; another lane remained. The latest completed unit-cache GC log entry reports 20,396,064,768 bytes freed, but it carries no timestamp or ownership and cannot by itself establish the cause of this headroom.

A lane is live when its marker names a living process regardless of heartbeat age. Its removal takes a lane claim and refuses a live marker (unit-cache lane TypeScript lines 46–63, 99–106). The retention wrapper publishes attributed provenance before release, including after a failed build (lane retention TypeScript lines 95–97 and 130–145). The unit collector separately checks live lanes, per-session locks, profile build leases and age; a busy shared profile can lose only sufficiently old superseded variants (unit-cache GC units TypeScript lines 34–37, 83–95, 131–152). These checks distinguish managed custody from unverified scratch, memo or target paths: a directory name alone does not establish safe ownership or deletability.

No supported public command was found to compact exactly the failed component lane while guaranteeing every reusable artifact remains; the now-absent failed lane cannot be selected for that anyway. The canonical driver supplies lock-aware pressure collection and warm reuse. Wait for the active WGPU build and use its actual terminal result; do not manually remove namespaces or start a competing retry.
