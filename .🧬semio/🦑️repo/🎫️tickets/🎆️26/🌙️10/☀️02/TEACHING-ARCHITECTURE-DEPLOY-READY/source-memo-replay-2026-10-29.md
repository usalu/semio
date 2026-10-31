# Authorizer Memo Replay Causal Audit — 2026-10-29

Read-only investigation of the exact declared memo /home/deck/.cache/semio-repo/source-memo/5f78bd319d83b5ec8a1d/39b11b90e467a391dbbd.json. No purge, override, write, Source change, Native job or execution of memo-derived code occurred.

## Actual Stored Data

At14:43:31UTC the memo is2,057,603bytes, stable across the timed inspection, SHA2567c9448ae6ffa2fcc704e09bcb890b4660287558efd049fc0ac56a73283d39873. It names the expected command runtime kernel and installed:true.

Its actual top-level fields are version, installed, implementation, root, entry, directories, observations, manifest, marks and imports. Source entries contain only path and sha256: there are1682, comprising635JSON,915TS,128MJS and4CJS files. There are no TSX/JSX or other resource extensions in this observed manifest. It stores1221directory/absence/package observations and1682import maps. It does not store Source payloads, base64 text, compiled code or authority tokens.

## Replay Order And Actual Mismatch Refusal

Memo owner code37–43 computes an implementation identity from the runtime key and six capture/scanner/workspace/memo Source files. In particular the actual sites scanner is one of those files. Code115 rejects a different version/root/implementation before directory replay, observed-data checks, Source reads or any parser.

The actual memo implementation is96d8c1c3e3749d30833ca8ee3193d2d79cdb86c9a4b014340701027c962d9581. Current canonical implementation is541690dc4383cd693a9d9fe27e377eaee6d96832850b65b18d0db9df3a80b6bf. The real public replayNxAuthorizerMemoV1 returned undefined for this mismatch. The probe neither changed the memo nor treated its result as execution authority. This is an actual mismatch refusal, not an invented stale/dead-owner inference.

On a matching implementation, replay checks remembered ancestor dev/ino, absence/listed directories, then rereads actual regular single-link Source files no-follow under the existing byte bound. It verifies stable before/after/named file identity and each remembered SHA before returning byte DATA. It does not recover old Source bytes from the memo. Malformed/torn memo JSON is caught by readNxAuthorizerMemoV1 and returns no memo; it is not forwarded into a TypeScript parser.

The authorizer caller additionally checks entry/root/installed mode and package resolution observations. Only after all replay checks succeed does it restore the recorded manifest/marks/import maps. A memo hit sets capturedImports directly; nxAuthorizerCapturedImportsV1 then returns that map instead of rescanning the contents. No runtime Transpiler call exists inside the memo reader or physical replay functions.

## Comparison With Fresh Actual Capture

At14:44:23UTC, one bounded fresh canonical capture with installed:true and NO memo, followed by nxAuthorizerCapturedImportsV1 on that SAME captured object, passed1682Source files/1682projection entries. Compared with the observed memo:

- Added paths:0.
- Removed paths:0.
- Import-map changes:0.
- Changed Source digest:only authorizers/sites/TypeScript, the just-authored error-coordinate diagnostic.
- The extension counts are identical, including0TSX/0JSX.

The compare uses only the manifest/hash/map observations and actual privately issued captured Source DATA; it does not execute that DATA. No Source bytes are copied to the ticket.

## Causal Conclusion And Boundary

This observed memo cannot supply a hidden extra TSX file, another Source roster or an older stored Source body. Under the CURRENT implementation it is refused before any replay parser could run. The proper ordinary caller will perform a fresh capture and may atomically publish a new memo afterward; that future write was not invoked here. If an older immutable loaded implementation matches96d8, its valid memo-hit path still restores data/import maps without a JavaScript scan inside replay.

These facts exclude the proposed mechanism in which this memo injects different Source bytes or performs a TypeScript parse during valid replay. They do not identify the historical99366/30660 failure. In particular import scanning does not prove that each captured module passes a later full transform or loader rewrite; that stage remains a concrete next boundary to inspect if the next normal diagnostic still fails. No loader or stale-versus-invalid classification policy was edited.

Preserved inputs source-memo-audit.ts and source-memo-compare.ts both ran through registered Bun and exited0. Generated source-memo-audit.json/log and source-memo-compare.json/log retain the exact timed observations. Frontend35703 remains the sole coordinated normal PDF diagnostic, still unadmitted at the sibling checkpoint; no competing capture gate or runtime was started here.
