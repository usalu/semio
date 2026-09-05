# Hub Headless Native Artifact Provider Boundary

## Result

Hub artifact authority core now depends on a domain-owned provider port rather than the concrete Stdio/GIS provider. The full provider remains production-default behind the explicit native-artifact-execution feature, while the execution-target acceptance compiles with default features disabled and sqlite alone. A configured trusted catalog without a provider fails closed; an entirely unconfigured catalog remains absent.

This is the immediate optional-feature transition described by the Terra packet. It does not claim the final host-only package extraction.

## Production boundary

- NativeCodecProviderSourceV1 accepts only borrowed Hub package identity, descriptor, exact codec requirements, and the operation context.
- TrustedCatalogLoader performs its bounded bundle, component, descriptor, closure, codec, and atomic registration checks through that port.
- NativeCodecProviderSetV1 is the feature-gated adapter and still owns the complete 26 Stdio plus two GIS receipt closure.
- The default Hub feature set retains sqlite and native-artifact-execution.
- Stdio and GIS are optional, no-default dependencies selected only by native-artifact-execution.
- GIS catalog binding and native inference runtime are gated with the same provider boundary.
- Headless configured-catalog startup cannot substitute a byte-only or empty provider.

## Neutral proof

The native-artifact-provider-frontier-v1 JSON fixture is validated independently with AJV 2020. It fixes the headless sqlite-only graph, zero active direct plugin dependencies, the production provider identity, all 28 receipts, and fail-closed configured-without-provider behavior.

The registered Hub source gate reports:

    hub-native-artifact-provider-frontier-oracle: AJV=1 headless=sqlite plugin-deps=0 production-receipts=28 configured-no-provider=reject
    execution-target-provider-frontier: checks=12
    execution-target-relay-check: checks=32

The Stdio Home-I/O topology source gate is also green and now separately requires:

- Space selects only home-io.
- Hub keeps optional Stdio/GIS dependencies and forwards full-artifact-catalog only from native-artifact-execution.
- GIS retains its direct full Stdio dependency.
- Every other catalog consumer retains its prior explicit full closure.

Cargo graph inspection for semio-hub with no default features and sqlite found no semio-s-plugin-stdio or semio-s-plugin-gis package edge.

## Gates

- os-hub:execution-target-relay-check
- os-hub:execution-target-native-check
- configured_catalog_without_a_native_provider_fails_closed
- execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body
- execution_target_selection_final_fence_matches_neutral_races
- @semio-tech/stdio-plugin:home-io-surface-check
- os-hub:native-openable-catalog-provider-check explicitly selects native-artifact-execution for both Hub law groups

## Native evidence

The registered headless three-law run is active in ticket-owned receipt exact-cargo-laws-6PJbzt/00 on the handed-off space-public-boundary-sol-target. No terminal native verdict is recorded yet.

## Remaining boundary

The full provider remains an optional feature inside semio-hub. A future host-only native artifact runtime package should become the sole direct owner of Stdio full and GIS so that Hub all-features checks also remain independent of the frontend artifact catalog. Full-provider runtime acceptance remains a separate target and must never be replaced by the headless byte-route laws.
