# A5 — sequence plugin report

## Summary

Wave A5 for `semio-s-plugin-sequence`: shipped config and presence schema facets (five leaves each) for owner `🎬️sequence/🎚️config`, added runtime `SequencePresence` + `SequencePresenceMutation` (shareable selection, orientation, node-graph camera), wired `📦️glue.rs` (`config { component; schema }`, `presence { component; schema }`), and bound `DocumentApp::Presence` / `PresenceMutation` on `SequencePlayApp`. Config schema mirrors `SequenceConfig` field-for-field with `local-ui`; presence schema uses `shared-ui` on the collab subset only (`lastRunJson` / `locale` stay config-only).

## Files touched

- `✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/🎚️config/🧬️schema/{🦀️,🟦️,🔗,🔣,🛰️}component.*`
- `✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/👥️presence/🧬️schema/{🦀️,🟦️,🔗,🔣,🛰️}component.*`
- `✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/🦀️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/📦️glue.rs`

## Gate tails

### Scoped `policyAppSchemaBreaches` (sequence filter)

```
0
```

### `cargo check -p semio-s-plugin-sequence`

```
(exit 0 — compile succeeded; no errors)
```

### `cargo test -p semio-s-plugin-sequence --lib`

```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
```

## Unverified

- Runtime presence hub sync / `ViewModel.presence_peers_json` path not exercised in this wave (types and schemas only).
- No manual UI session to confirm presence peers render selection/orientation/camera on a live backbone.
