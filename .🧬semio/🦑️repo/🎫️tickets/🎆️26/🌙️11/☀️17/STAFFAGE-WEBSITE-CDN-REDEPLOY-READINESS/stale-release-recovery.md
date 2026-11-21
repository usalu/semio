# Staffage release recovery

The public Pages domain serves the archive HTML with `Cache-Control: max-age=600`. In a single GET series, the root and its first request with a fresh query both returned Age 348 and the same Date from the same Frankfurt cache. The hit counter increased from 1 to 2. Request headers `Cache-Control: no-cache, no-store, max-age=0` and `Pragma: no-cache` also returned a warm Pages response. A fresh `/__staffage_recovery-<time>-<uuid>` pathname on the same cache returned HTTP 404, Age 0, a cache miss and the archive HTML. [Public Staffage document](https://staffage.semio-tech.com/)

The release document installs a CSP-hashed inline recovery listener before its hashed entry. Failure of that entry and Vite lazy-loading failures share one session-scoped attempt per minute. A unique one-segment pathname reaches the same app shell through the generated `404.html`. `location.replace` preserves the existing browser history slot, with query and fragment carried through unchanged. After mounting, the archive restores the stored pathname via `history.replaceState`. Matching the issued recovery pathname keeps ordinary 404 addresses intact, and pathname-only assignment confines navigation to the current origin.

The Gherkin scenarios and Vitest coverage exercise both failure sources, throttling across documents, session storage denial, unrelated asset failures, an unrelated 404 address, and query, fragment and history preservation. The release document test parses the HTML with jsdom, checks listener installation order and calculates the CSP hash from the script parsed by the DOM.

Previously cached archive documents will get this behavior when they next receive a release document that includes the bootstrap.
