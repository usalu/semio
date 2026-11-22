# Final Pipeline Readiness

## Run

- `MEM=4096 bash native-wasm.sh site` (site build stage only, one heavy process, no browser, no matrix): `OK site`, Vite `built in 17.91s`, output `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out` (rebuilt in place, bundles `assets/🌐️-C2odevWr.js`, `assets/🟦️-B2v9lN1s.js`).
- `validate-local-references.py` on the new output: rc 0.
- `release-receipts.py status` on the real roster: 10 of 32 confirmed (the ticket holds 10 receipts now, not 11), 22 missing, all 22 have a staged release and await confirmation.
- Unit checks: `test-release-receipts-status.py` OK, `bun test` of the new `shipped asset selection` case in the styling asset-delivery suite: 1 pass.

## Confirmed

- The new bundle emits `purpose` on `instance-open` in both producers (`purpose:a.purpose` in shard-client, `purpose:{tag:"ordinary"}` in the store worker). The stale-site fault of `🔬️demonstrator-wd3qizX-probe.md` run 1 is gone at the bundle level. The remaining Demonstrator `publication-stalled` fault is a separate source fault, untouched.

## Fixed

1. Agent bridge (`🏗️builder/🌐️vite/🟦️.ts` of the Demonstrator): the build now defines `import.meta.env.VITE_SEMIO_AGENT_BRIDGE_OFFER` as `""` explicitly. The bundle still contains the endpoint string, but `watchAgentBridgeOffer` returns before any request when the offer is undeclared, so the release makes no `/__semio/agent-bridge` request. Before, undeclared depended on no `VITE_` variable leaking from the building shell. The 404 storm in the 11-19 log came from an older build that predates the declaration gate.
2. Shipped sources and tests: the `/🖼️assets` copy took the whole asset module (sources, `🧪️tests`, `🧫️fixtures`, `AGENTS.md`, `README.md`). New `semioAssetShipsToBrowserV1` in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🗃️assets/🟦️.ts` filters `🧪️tests`, `🧫️fixtures`, `node_modules`, `.ts/.tsx/.rs/.toml/.py`, `AGENTS.md`, `README.md` in `closeBundle`. New output: 0 test paths, no `.ts/.rs` (only `LICENSE.md` stays). Test added in `🧪️tests/🧩️suite/📦️asset-delivery/🟦️.ts`.
3. Receipt gate: `release-receipts.py` gained `status` and `verify` now fails first with `N of 32 roster modules confirmed; K still missing:` plus one line per module (`staged, awaiting confirmation` or `no staged release`) instead of a bare refusal after the expensive checks.
4. `package-site.sh`: when the accepted manifest is missing it explains that only `VERIFY_QUEUE_RECEIPTS=1 native-site-verify.sh` writes it and prints the same missing list.
5. `native-site-verify.sh` with `VERIFY_QUEUE_RECEIPTS=1` runs `status` first and exits before the 15 minute build if any receipt is missing.
6. `verify-static.sh` refuses when the port is already listening (a stale server on 8765 can no longer pass the readiness loop for the wrong origin).

## Still open

- Relocatability: runtime strings (`/mesh`, `/vt`, `/osm`, `/cad-assets`, `/infinite-assets`, `/renderer-modules`, `/🔌️plugin-modules`, `/🧩️extension-modules`) are absolute in the bundle. They come from many framework modules, so a sub-path deploy needs one runtime base route owned by the framework; not changed. The CNAME deploys at the domain root and the matrix serves the root, so the intended deployment is unaffected. `base: "./"` only covers HTML and CSS.
- Map tiles: prefetch is `skipExisting`; the cache is warm (396 + 85 tiles, 0 downloaded). A clean machine needs network once. Left as is; the canary check fails loudly if the cache is incomplete.
- The output is still 2.6 GB (`🖼️assets` 170 MB of media: images 98 MB, metabolism 44 MB, fonts 21 MB). Whether all media is referenced by the Demonstrator was not audited; `semioReferencedAssetsVitePlugin` exists but is unexported and unused.
- Not run: the static matrix (16 boots), `package-site.sh`, `verify-release-archive.py`. Whether the matrix is free of bridge requests and green is unconfirmed; the Demonstrator publication-stalled fault likely still fails the matrix.
- 22 receipts are missing; `demonstrator` and `process` have compile failures per the earlier audit.
- Pre-existing: `verify-static.sh` hard-codes the Playwright path, scripts are bash-only, `ROOT` derives from the ticket depth.
