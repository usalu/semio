# Five compute consumer owner routes

[Full current owner routing and GUI evidence](compute-consumer-owned-route-audit-inputs/full-current-five-owner-routes-1.json) binds the exact package files and all five existing consumer descriptors. No code/native changes occurred.

| Consumer | Existing owner route | Bounded addition |
| --- | --- | --- |
| flow-drawing | os-flow Cargo owner currently has no adjacent script/project/package; flow-core has a separate browser-ownership route | Register os-flow's own permanent source-only gate; do not misattribute drawing to the different flow-core package |
| puzzle-brush-cache | @semio-tech/puzzle-3d-rs verify-snapshot-sqlite-source/graph-wire-check, shared artifact router permits commands | Add a source-only compute-consumer command to its explicit commands map and owner target |
| plugin-sdk | @semio-tech/framework-plugin test-command-ingress-consumer | Reuse its strict-then-Bun source-gate pattern with a separate compute fixture/test, no native flags |
| host-component-fixture | @semio-tech/framework-plugin-host-fixture check currently validates schema then invokes Cargo check | Add a separate source-only compute gate; current check is not source-only |
| hub-space | @semio-tech/space-plugin plugin-identity-check and source checks with optional --native siblings | Add explicit compute-consumer source gate to its permanent router, preserving whole native test selection |

Each owner gets exactly its one existing descriptor and schema-closed binding roster; no neutral required list of five source paths. Add one unique authored GUI command per owner in the established source-gate group/order and regenerate through the managed route. Existing GUI sources are retained, not copied wholesale into a proposal. The flow owner needs genuine command registration rather than invoking a neighboring package as a proxy.

A shared test-only neutral interface can accept `ComputeConsumerDescriptor {version:1, owner:string, source:string, manifest:string, bindings:readonly ComputeFamilyName[]}` plus an explicit reader `(path:string)=>string`, and return `{directDependency:true, observedBindings:readonly ComputeFamilyName[]}`. It must have no built-in OS/s/hub path roster. The neutral schema defines the seven canonical family names and a nonempty unique bindings list; each owner fixture separately fixes its exact ID/source/manifest/binding subset. Tests preserve the original source binding and forbidden OS-alias assertions plus independent TOML direct semio-framework-2d dependency check. A missing declared owner source/manifest must refuse; deleting an entire higher owner leaves the neutral helper/compute gate untouched.

The helper is test infrastructure, not a runtime API or consumer adapter. Closed positive/broken-owner/dependency/binding vectors should validate its mechanism independently of these five actual artifacts. Actual owner gates run their own source fixture; no generic aggregate fixed Specific reader or skip-missing mechanism is introduced.
