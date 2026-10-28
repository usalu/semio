# Owned Surface Facade Accessor Integration Contract

Narrow continuation of owned-staged-host-integration-plan.md; source-only, no duplicate whole-host audit, production edits or capability enablement.

## Exact insertion points

1. Renderer PluginRuntime/handle-scope/TS createPluginHandleScope owns uiSurfaceByInstance, uiOwnerByInstance, lifecycleByInstance and actor mapping. createInstanceTable requireActorId lines119–124 rejects disposing/missing/closing instance; releaseInstanceMaps lines125–142 deletes lifecycle/surface/read maps. This is the identity boundary, not a free string→surface cache.
2. handle-ui-intake/TS install acknowledged round lines95–111 takes surface only after original patch acknowledgment, stores exact OwnedUiInstanceSurface and announces publication. A narrow accessor should be implemented beside this installation/retirement owner and returned by createRetainedUiIntake. It should never call createUiSurfaceProjection or retainedSurfaceToBuiltNode.
3. module-loader/TS loadPluginModule lines467–490 composes scope/instances/ui and returns rich renderer handle plus refreshUi. Expose the narrow renderer-owned accessor here through its first-party declared handle contract after authentic auxiliary source admission exists. Do not put React/staged-specific state or a raw private Map on the domain-neutral kernel handle. Keep accessor shape domain-neutral (exact acknowledged retained source lease); staged authorization remains consumer/target contract.
4. WindowActionPane/current real window owner consumes the future lease and instantiates OwnedStagedControls once per exact action/window/source lifetime. WindowActionPane is not currently wired; its static Tree rows remain actual behavior. No new window ID or body declaration should be fabricated to test this facade.

## Proposed authority contract

Acquisition takes actual instance and schema-owned surface target from the successfully admitted mount/refresh result, not arbitrary public body-key lookup. Check requireActorId, captured actor activation/lifecycle identity, exact uiOwner identity and acknowledged installed surface mapping before returning. A stale target or different handle/instance returns no lease. A missing first publication is pending, not an empty fabricated surface. Retired/disposing/closing sources refuse reads/acquisition.

Lease returns only exact source facade and bounded metadata/publication subscription, with an idempotent close that transfers all consumer reads through existing paid source close protocol. It must carry a revocation signal or currentness predicate tied to original lifecycle, so actor remount/instance close cannot leave a logically reusable facade. Consumer close does not destroy whole instance or other windows. The original instance retirement ladder must revoke consumers before clearing maps and preserve failed close owners for retry; deleting an index is not retirement proof.

A facade getter alone is insufficient if its lifetime cannot coordinate outstanding reads with instance destroy. Conversely no new graph clone/ownership wallet is needed: existing read subscriptions already own their issued roots, commit acknowledgement and retirement. Reuse that authority, do not wrap BuiltNode refresh output as if retained.

## Test-first slice possible now

Author a closed language-neutral acquisition/lifetime matrix: acknowledged-current succeeds; before-ACK/missing pending; wrong handle/instance/target refuses; disposing/closing/retired refuses; remount same external key cannot revive old lease; sibling acquisition/close independent; consumer outstanding old read survives publication until ack then retires; instance close revokes accessor and waits for existing read retirement. Use actual nativeInstanceFixture plus OwnedUiInstance/patch receipts and actual source map owner extracted via sanctioned test interface, not a synthetic surface object as ownership proof. Pure contract reducer tests may be authored/executed under existing runner; real fixture stays policy-gated until sanctioned renderer test policy runs it. No build bypass or advertised native capability is needed.

## Prerequisites that remain blocking for staged mount

Authentic standalone staged request receiving must bind original parent/recipient/source slot and real window context lease, finite transport body limits and owned payload donation/close. Complete native IFC producer must supply ready status/body under same exact request/source revision; no semantic gate may be enabled from source parse. Actual auxiliary SurfaceVisible/hide event owner must authenticate bodyKey plus contextWindowId and original lifecycle. Existing refresh public BuiltNode response is explicitly not this prerequisite.

After these gates, host Execute requires current source revision observation (no unresolved source-work holds), current session ready token, actual owner/plugin/window/action/utility policy and authentic dispatch terminal tracking. Hide/collapse retires form reads/query; it does not release unknown canonical source-work holds.
