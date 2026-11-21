# Repair Evicted Closures During Graph Publication

A replacement validating cache refused the initial action-result write because two closure blobs from the sender's memo were absent. The focused reproduction originally observed a single HTTP 400, so the caller had to rerun the entire publish. The amended fixture observes HTTP 400 followed by HTTP 200 within one call.

The HTTP content adapter records the missing output digest when the cache returns its exact action-result validation refusal. The graph publisher recovers only if that digest belongs to a memo-assumed closure in the rejected action. The refusal establishes absence for that digest, and the publisher checks the other memo-assumed digests once on this recovery path. It uploads only the missing leaves following bounded local reads and exact length and SHA-256 checks. Then it sends the identical action-result bytes once more. The normal path still skips existence requests for the memoized leaves. Recovery leaves neutral document and manifest transfers to the initial attempt.

The tests use private cache servers and SQLite queries over the requests. The success case loses two closures, keeps a third in the replacement server, and verifies the unchanged action-result body, the lack of repeated document output uploads, and the closure-only existence checks during recovery. A failure matrix covers authorization, a malformed result, a missing non-closure document, corrupt local closure bytes, cancellation, and a second validation refusal. Every failure preserves the existing remote head byte for byte. Failed recovery invalidates the specific closure memo entries whose existence was disproven.

The final focused tests passed 4 tests and 75 assertions. The full remote-exchange suite passed 42 tests and 1,227 assertions with a 15-second per-test allowance, after one initial wave test exceeded the default five-second allowance under host load. The content refusal and cache action-result suites passed 7 tests and 2,162 assertions. No live fleet cache or service was changed.

- [Graph publisher](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts)
- [Content port](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🗄️content/🟦️.ts)
- [Regression tests](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🧹️lost-leaves/🟦️.ts)
