# React GLB Retry

The React icon render port caches the pending GLB load so preview and export callers can share transport and parsing, then clone the loaded scene independently. A loader rejection now removes the cache entry only if it still holds that exact promise. Current callers receive the error and a later render may start a new load at the same URL. Each render retains its own abort signal. The shared load and the cache eviction path depend on the loader result.

The existing Chromium drawing probe now resets the first connection to the repository GLB and runs two render consumers on the same retry URL. Playwright aborts one consumer while holding the retry response. The aborted consumer receives AbortError and the other consumer receives the real asset and renders an SVG. Later projections and layer changes still render with only two asset requests total. Sharp verifies the SVG dimensions, changed projection pixels and transparent line-only output.

Run the probe from the checkout root with `repo tools exec bun -- ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/SHOOTING-PLUGIN-END-TO-END/drawing-browser-check.mjs"`. Before the change the retry failed with Failed to fetch. The full probe passed after the change.

## Automatic retry for live renders

The shared URL entry now keeps the current render waiting through two transport retries, after 200 ms and 800 ms. The network stage retries fetch and body transport failures, short declared responses, and HTTP 408, 429, 500, 502, 503 or 504. Three parses a successfully delivered response once. HTTP 404 and malformed assets produce a terminal result for that render. Three keeps its original relative resource base and model-axis rotation. Each render receives an independent clone.

Every consumer aborts immediately without controlling its peers. When the last consumer leaves, its shared entry is removed by identity and the outstanding top-level fetch or retry timer is cancelled. A late completion cannot remove a replacement entry. Once Three starts parsing, its dependent resources remain owned by Three. The cancelled render has already settled and the cancelled load cannot publish into the cache.

The Chromium probe failed on the first connection reset before the automatic retry and passed afterward. The same render recovered from that reset and a subsequent 503. A peer cancelled before the recovery response arrived, a sole cancelled request stopped at attempt one, and persistent 503 stopped after attempt three. The probe also recovered from an interrupted body, made one attempt each for 404 and malformed bytes, and loaded the relative BIN for a nested glTF. Sharp matched its raster output to the original GLB and verified the recovered response rendered the same pixels. The scoped diff check passed.
