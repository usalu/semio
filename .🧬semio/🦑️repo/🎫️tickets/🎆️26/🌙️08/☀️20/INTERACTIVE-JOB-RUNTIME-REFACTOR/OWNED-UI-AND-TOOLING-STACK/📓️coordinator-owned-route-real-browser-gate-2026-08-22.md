# Coordinator Owned Route Real-Browser Gate

Date: 2026-08-22
Verdict: **PASS for the actual compiled `NotFound` and `RouteLink` browser-history boundary.**

## Scope

The coordinator used the repository's available in-app Browser skill because this packet requires
real browser history behavior that JSDOM cannot certify. The skill kept the test in the Codex
in-app browser, prohibited a standalone Playwright substitution, and required fresh visible DOM and
console evidence after each interaction.

The Vite harness at `🧪️p10-owned-route-browser-gate.html` imports `NotFound` and `RouteLink` from the
actual UI React barrel. It does not copy their implementation. The harness ran at the repository-local
URL on `127.0.0.1:4179`.

## Observed Sequence

1. The initial DOM contained the actual `Missing space` heading, `Back to spaces` button, and
   `Open route link` anchor. The location output held the harness path and the popstate count was
   zero. Browser console warnings/errors were empty.
2. Clicking the actual `NotFound` button changed the live URL and the harness output to
   `/spaces/a?tab=history#entry`. The popstate count became exactly one and its recorded location
   contained that exact path, query, and fragment.
3. Browser Back restored the encoded harness URL. The count became two and the second recorded
   location was the harness URL.
4. Browser Forward restored `/spaces/a?tab=history#entry`. The count became three and the third
   recorded location was exact.
5. Browser Back followed by clicking the actual `RouteLink` reached
   `/spaces/b?tab=route#link`. The final count was five: one native Back event plus one synthetic
   owned-link event after the prior three-event sequence. The ordered location ledger matched that
   sequence exactly.
6. Browser console warnings/errors remained empty after every interaction.

## Boundary

This gate proves the packet's browser URL, query, fragment, `popstate`, Back, Forward, and actual
component wiring. It does not accept the dependency removal by itself; source, lock, test, policy,
dependency, and independent audit gates remain separate.
