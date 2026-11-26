# Guest Contract Actor Input Ownership

The admitted WASI library run in the `pacb6dd` lane stopped at 15 unresolved imports and references in `semio-plugin-guest-contract`. The first was `semio_plugin_abi::actor_input_receipt`. The ABI rlib in the failed lane has mtime `1795666288`, while the live ABI source containing the receipt export has mtime `1795666366`. The timestamps support the stale-provider diagnosis. A fresh managed compile provides the definitive check.

A replica update during the audit completed the remaining fourteen source conversions. The four guest-mounted call files now use the guest root's `crate::original_input` binding. They reference `OriginalActorInputFailureIdentity` eight times, `OriginalActorInputFailureView` three times, `OriginalJsonOutputFailure` twice, and `retire_step` once. The unconditional ABI receipt module exports all four. The runtime actor-input package uses the same type definitions and retirement function through its direct ABI dependency. Its separate decoder and wire packages remain private to the runtime path. The guest still imports the call from its existing shared original actor context mount. Neither the guest manifest nor the active Hub lock declares a direct edge to the runtime actor-input package.

The source and TOML projection passed for Hub, Framework, standalone Draw and CAD through the registered Node tool and independent smol-toml parser. The CAD lock initially retained the earlier guest-to-runtime edge. I removed that one edge and verified that `semio-plugin-reactor-context` still requires the existing `semio-plugin-actor-input` record. Current shared framework sources and the active Hub lock already had the audited repair when verification began. The separately managed Hub compile owns behavioral compilation proof.

A scan of 176 live locks found 30 other workspace locks that lack the two direct ABI module edges. The isolated correction would also require 12–13 new package records per stale lock and encounters unrelated older Job, Value and receiving-package records. These accessory workspace locks require owner-led Cargo regeneration against current manifests. Their exact paths are reproducible with the full scan below.

## Verification

```sh
T='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END'
repo tools exec --light node -- "$T/verify-guest-contract-actor-input-26.mjs" \
  '🌎️hub/Cargo.lock' '🧰️framework/Cargo.lock' \
  '✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.lock' \
  '✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.lock'
rg -l --null '^name = "semio-plugin-abi"$' -g Cargo.lock -g '!**/.cache/**' -g '!**/.🧬semio/**' |
  xargs -0 repo tools exec --light node -- "$T/verify-guest-contract-actor-input-26.mjs"
```

## Owned Files

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.lock`
- `verify-guest-contract-actor-input-26.mjs` in this ticket
- This report
