# Wave 4 verify

## OS state authority
- Ungated `policy({root})` → **os: 0** (`🧪w4-os-policy.out`)
- Unrelated policy kinds remain (~490); verify gate asserts OS policies only

## Cargo (DEVELOPER_DIR=CLT)
| Crate | Result |
|-------|--------|
| `semio-framework-plugin-host --lib` | GREEN |
| `semio-framework-plugin --lib` | GREEN |
| `semio-framework-os-infinite --lib` | GREEN |

## Runtime proof
- Host Emit apply emits `[DEBUG] host apply_emit_ops…` when codecs fold (see host `apply_emit_ops`)
- Full `verify gate` historically hung on plugin-registry nx recursion (Wave 0 baseline) — not re-run end-to-end here

## Next (Wave 5)
- Strip `[DEBUG]` markers (~90 in `.rs`)
- `ticket_close` with file list + summary
