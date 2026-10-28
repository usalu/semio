# Live Course Execution

Permissions updated to unrestricted; normal repository commands now work. Previous fixed backing admission error did not recur in the live integration attempt.

`repo run @teaching/architecture-course:test-e2e -- --topology dev -- --project boot` is active in exec session 82206. Child 121611 completed source admission and staged six JavaScript owner groups, then launched managed Bun dependency installation (child 122834). No browser result yet.

Fresh frontend/backend/audit agents replaced old agents whose tool permission contexts remained restricted. Frontend has queued course tests and typecheck; backend has queued proctor check. No new product changes proven yet.

This supersedes the permission-blocked execution status in integration-2026-10-28.md; the full goal remains active.

## Manifest And Dependency Progress

Backend repaired twelve identical duplicate dependency declarations across eleven Cargo manifests. Two independent TOML parsers validate the manifest set, and native Cargo lock refresh now succeeds. A separate read-only review confirmed the changes remove only duplicate declarations. The backend also provisioned missing scheduler dependencies via supported repo cargo fetch, and is proceeding to the actual proctor build.

Frontend independent checks found no missing relative imports across the course and shared React surface; AJV accepts all eleven course/quiz documents. The suspicious JSON attributes on TypeScript schema barrels were tested with the managed loader and esbuild and did not reproduce a failure; no speculative changes were made.

The integration dependency producer remains live, with CPU and I/O changes and node_modules appearing in additional owner groups. This is verified active work, not a terminal failure. Browser execution remains pending.

## Shared Producer Checkpoint

At the later live check all six managed JavaScript owner groups contain node_modules and bindings.json. The original producer PID121611 remains alive in integration session82206; no restart was performed. Frontend test/typecheck and proctor build remain consumers of that producer. Scheduler fresh-install repair is assigned separately using existing registry-custody interfaces.

## First Integration Attempt Finished

Session82206 is terminal exit1 at graph planning, before any browser/app execution: Cannot redefine key semio-framework-value-snapshot-projection. Broader validation discovered additional source manifests outside the original teaching/framework scan, including energy model/schedule. After backend repairs, both independent TOML parsers accept all3597 source-tree manifests. The initial stale-source hypothesis is withdrawn: there were real additional duplicate declarations.

Fresh frontend checks and a full serial release end-to-end gate are next; fresh commands select the ticket output directory explicitly. Shared JavaScript installation completed during the first attempt and is available for reuse.
