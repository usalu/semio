# Selected GIS Browser Byte Provenance

## Boundary

The ticket-owned `s` browser host now writes `🌍️gis/🔗️materialization.json` from the same bounded materialization path that transpiles the selected GIS component. The receipt binds:

- selected component SHA-256;
- selected descriptor SHA-256;
- staged descriptor SHA-256;
- staged bridge SHA-256;
- every generated Wasm core's relative path, byte length, and SHA-256.

The outer browser-host receipt embeds those identities plus the materialization-receipt SHA-256. `resolveTestBrowserHostRootsV1` rehashes and compares the materialization receipt, descriptor, bridge, and generated cores before Vite receives the roots. The Hub two-Author owner also compares the receipt's selected component and descriptor identities with its retained selected-current package.

## Neutral and Independent Evidence

The Draft 7 schema contains closed definitions for the materialization receipt, generated core rows, and the language-neutral provenance corpus. The corpus covers:

1. a substituted selected component hash in the outer receipt;
2. a substituted staged GIS descriptor after independently recomputing the aggregate module-set hash;
3. a substituted staged GIS bridge after independently recomputing the aggregate module-set hash.

The direct Bun runtime law used Node SHA-256 and WebCrypto SHA-256 over four neutral specimens, wrote and parsed one materialization receipt, closed and resolved one exact browser host, and refused all three hostiles. Evidence:

- `🗑️generated/selected-gis-provenance/direct-runtime.log`
- `🗑️generated/selected-gis-provenance/build.log`
- `🗑️generated/selected-gis-provenance/test-build.log`

## Registered Gate Frontier

The dedicated target previously selected the `long`/jsdom environment despite containing a bounded filesystem law. Session `48139` consequently imported zero tests because Vite attempted to bundle the built-in `node:sqlite` through the repo caching-leases module. The target now uses the existing `quick`/node level without removing any selected test.

Registered target `@semio-tech/framework-os-dev:browser-host-staging-check` final session `28196` is green: one selected test passed and 88 unrelated tests were skipped. It also checks that the same GIS transpilation function writes the materialization receipt after its bridge and before the retained source bytes are revalidated. Evidence is `🗑️generated/selected-gis-provenance/focused-materializer-owner.log`.

Registered Hub target `os-hub:trusted-stdio-gis-bundle-check -- --two-author-source` session `72486` is green: 17 composition laws, 10 hostiles, five current-coordinate identities, and the existing independent witness-socket retirement oracle. Evidence is `🗑️generated/selected-gis-provenance/hub-source.log`.

After the 2026-09-09 task-session reset, both registered targets were rerun against the then-current shared source with a fresh private Nx workspace-data directory:

- staging session `88130`: one selected test passed, 88 unrelated tests skipped, exit 0; `🗑️generated/selected-gis-provenance/revalidate-staging.log`;
- Hub source session `43848`: 17 composition laws and 10 hostiles, exit 0; `🗑️generated/selected-gis-provenance/revalidate-hub-source.log`.

No Cargo, Vite server, Chromium, or two-peer process was started by this packet. It closes the source/runtime provenance proof but does not qualify current browser substitution or the full mounted two-Author journey.
