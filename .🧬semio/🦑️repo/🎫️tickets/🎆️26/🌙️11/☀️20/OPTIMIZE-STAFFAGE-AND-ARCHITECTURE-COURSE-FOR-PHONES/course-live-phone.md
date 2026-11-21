# Architecture Course Live Phone Verification

The canonical development gate passed its boot check and all four phone cases. Its Vite server returned HTTP 200 and the PDF media type for all six assignment documents. Every served file matched its published bytes.

The independent loopback replay passed the same five unmodified Playwright cases in 60.17 seconds with Chromium revision 1248. The JSON report recorded five expected tests and zero skipped, unexpected, or flaky tests. It covered startup, both swipe directions and a scored quiz, hard-quiz input geometry, portrait and rotated navigation with deferred media and crowd requests, and touch dragging to an offscreen classification target.

The replay used the current prototype Proctor executable after checking its platform, size, and SHA-256 against its receipt, with a new local development data directory. The course's own Vite configuration handled the gateway proxy and published all six assignment PDFs byte for byte with the PDF media type. The ticket harness supplied a loopback bearer-authenticated input channel from the existing PDF files after checking their producer receipts. The canonical gate supplied the retained artifact channel through the repository's normal invocation. Both checks used native Proctor service and actual published PDFs.

Run the independent harness from the repository root through `repo tools exec node --light -- --experimental-transform-types <ticket>/course-live-phone.mjs`. The harness stopped its Proctor after testing. Browser output and local data were written to the ticket's generated directory.
