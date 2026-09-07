# Cold Document Pair Ingress

## Scope

This packet owns the isolated schema/kernel vocabulary and reactor-local retained byte owner for bounded browser-actor cold document ingress. It does not wire the owner into `reactor.poll`, does not call `plugin_load_document_pack`, and does not claim a mounted browser renderer or a real GIS checkpoint journey.

## Contract

The neutral corpus fixes a 64 KiB page, 64-page and 4 MiB aggregate ceiling. Its exact 2 MiB Pack plus 2 MiB SPR pair is independently checked by repository SHA-256, Node `crypto`, WebCrypto and AJV 2020. Eight hostile rows cover duplicate/reordered pages, foreign lifetime, descriptor or length substitution, short middle page, terminal mutation and modulo-slot collision.

The retained owner now:

- validates the header and first page before reserving capacity and never pre-zeroes the full pair;
- charges actual reserved vector capacity and page slots against one reactor-wide 4 MiB/64-page budget;
- rechecks the exact Live lifetime on every page, load admission and post-await completion;
- updates Pack, SPR and aggregate SHA-256 incrementally as each single page is accepted;
- remains structurally mounted through the load await by sharing one exact owner and immutable file allocation with the load grant;
- turns revocation during load into a stale/fault result rather than Applied;
- retains capacity through Applied/Faulted/Loading and releases it only after explicit cleanup;
- wipes at most 64 KiB per `close_step`; registry teardown requires every structural byte and close owner to have reached bounded terminal release.

The baseline frontier requires a nonempty active checkpoint edit identity and `last_commit_seq <= head_edit_ordinal`; it does not synthesize a best-effort genesis head.

## Evidence

Registered source command:

`@semio-tech/framework-plugin:cold-document-pair-ingress-check`

Latest source/oracle receipt `e4337d` is GREEN:

`cold-document-pair-ingress-oracle: ajv=1 sha256=3 webcrypto=3 hostile=8 limits=64KiB/64/4MiB`

Rustfmt parsed the kernel vocabulary, retained owner and six exact Rust laws. The earlier root lifecycle build `2j5R8a/00` did not run lifecycle laws because it saw the initial wrong fixture-relative path and two shadowed helper calls. Those compiler blockers are corrected. A current native execution of the six cold-pair laws has not completed; native and integration remain pending.

## Integration Fence

Before loader wiring, guest lifecycle close must treat cold ingress as a fourth exact close participant: preflight, reserve, activate, terminal check and release must cover the same lifetime and retained capacity. Reactor `poll` may then accept one optional page and return one typed ingress status per turn. The final loader completion must pass the exact current Live lifetime into `finish_load`; no generic document/storage/HTTP route is permitted.
