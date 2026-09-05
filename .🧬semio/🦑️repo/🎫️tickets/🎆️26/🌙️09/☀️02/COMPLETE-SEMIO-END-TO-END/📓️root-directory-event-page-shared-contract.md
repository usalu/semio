# Directory Event-Page Shared Contract

## Outcome

The OS directory domain now owns one schema-first `DirectoryEventPageV1` envelope shared by Rust and TypeScript. It is a bounded authenticated-page data contract and does not yet claim that the hub endpoint, Home retained transaction, browser/WGPU page owner, or process journey exists.

## Contract

- exact schema discriminator `semio.directory.event-page.v1`;
- nonzero lowercase SHA-256 session binding and positive safe authorization generation;
- safe raw scan frontiers `afterSeqExclusive` and `throughSeqInclusive`;
- at most 128 strictly increasing visible events inside `(after, through]`, allowing invisible raw holes;
- at most 48 KiB canonical bytes per event and 64 KiB per canonical page;
- lowercase SHA-256 receipt over declaration-ordered canonical JSON excluding `receiptSha256`;
- canonical input only: duplicate/unknown fields, whitespace/trailing bytes, controls, unsafe integers, range substitution, and forged receipts are denied.

## TDD Evidence

- RED: the source gate failed because `$defs.DirectoryEventPageV1` did not exist.
- GREEN: `@semio-tech/framework-os-kernel:directory-event-page-contract-check` passed 14 checks.
- The neutral UTF-8 vector is 474 unsigned bytes with SHA-256 `caf9c1ed2ba77ea86c1ba0b5548b978260c6bfd22ecd387c74b8017e0b4fb59f`.
- The third-party oracle is AJV 2020 plus Node SHA-256; the production implementations use Web Crypto and the repository-owned Rust SHA-256 respectively.
- Rust parsing succeeded. Repository-wide `rustfmt --check` traversal reported existing formatting drift in sibling directory client code, which this packet did not rewrite.
- Native exact law: green with one discovered/executed assertion. Receipt directory `🗑️generated/root-directory-event-page-contract/exact-cargo-laws-w27sZq/00`; executable SHA-256 `69ac90eba97b7962b0861168dbb05f1707b5bcc43079c7c6da86473e2cfb7a31`.

## Permanent Gates

- `@semio-tech/framework-os-kernel:directory-event-page-contract-check`
- `@semio-tech/framework-os-kernel:directory-event-page-contract-native-check`
- exact law `os_directory::schema::tests::directory_event_page_v1_matches_language_neutral_receipt_and_rejects_hostiles`
- launch entries `⚖️gate📄️directory-event-page-contract` and `⚖️gate📄️directory-event-page-contract🦀️native`

## Files

- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️event-page-v1.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc`

## Next Slice

Use this type in an authenticated hub `GET /directory/event-page/v1?after=<u64>` implementation with bounded raw scans, current visibility filtering, post-read session/generation revalidation, and append-time event-size enforcement. Only after that should Home accept the page through one retained config replacement and terminal ACK.
