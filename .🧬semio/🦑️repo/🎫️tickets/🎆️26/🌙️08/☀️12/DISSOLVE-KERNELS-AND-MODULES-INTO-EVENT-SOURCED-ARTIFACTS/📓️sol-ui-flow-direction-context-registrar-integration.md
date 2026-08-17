# UI Flow Direction Context Registrar Integration

## Source Handshake

- Protected React barrel baseline SHA-256: `a9a764971875336ed637b8be0ec1dae23150dfce09985ddf7cd5d69cafc774f6`.
- Terra confirmed the new module SHA-256 `a84af143796b57028794b503423be3fa2254d4d4df47c58be97a1d355ccb32e2`.
- Seven independent production components import the new module directly.
- The old element source was deleted with no forwarder, and both the old Flow and previously empty ElementProps directories were removed after verified-empty checks.

## Registrar Change

The coordinator changed only the mechanical React barrel import path from the old Flow element to `🔨️modules/🧭️flow-direction-context`. The existing explicit type/value export list, authored barrel consumers, and inline test references remain intact.

The coordinator then verified the legacy path was absent from active UI source, counted exactly seven direct component imports plus the mechanical barrel, confirmed both empty legacy directories absent, and ran scoped `git diff --check`. The protected barrel final SHA-256 is `537138eb89f28302991e6b38f2aea879f7ee19cacbd495d5e23517a7755b4e5d`.
