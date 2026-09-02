# ⏭️ Queued wave: the `serde_json::Value` pack bridge (do AFTER the current plugin wave)

## Why it is queued, not running
Verified consumer list for the bridge in 🏪️store (`pack_rt`):
  dsl_value_to_json → 🗺️surface/🕸️node-graph · ✍️editor · 🏪️store · 📖️playbook ·
                      🌊️flow(duplicate-widget) · 💡️reasoning(canvas-pointer-down) ·
                      🌀️procedural · 📋️forms(set-try-values)
  json_values_equal → 🏪️store · 🧩️puzzle(🖐️5d mutations) · 🧩️puzzle(◻2d mutations)
  encode_json_value → 🖱️ui/🎬️scene/🦀️pack.rs · 🏪️store
  renormalize_json_wire_value → 🏪️store only

Four of those files (🌊️flow, 📋️forms, 🌀️procedural, 🧩️puzzle) are being edited by conversion agents
RIGHT NOW. Converting the bridge concurrently would clobber their work, so this wave waits.

## Why it must eventually happen
These functions EXPORT API typed on `serde_json::Value`, which CLAUDE.md forbids outright:
"MUST NOT export api that directly or indirectly requires an interface/class/type outside of this
codebase." `DslValue` is the first-party equivalent and already exists. A previous wave recorded this
as "unchanged, permanent" (📓️store-serde-final.md) — that verdict was scoped to a single-file agent
and should NOT be read as an architectural ruling. It is unfinished work, not a settled exception.

## How to do it (must be atomic — no adapters, no deprecation shims)
Re-type all four functions on `DslValue`, using first-party `pack::json` for string encode/decode,
and update all 8 consumers in the SAME wave. CLAUDE.md: "You MUST manually fix all assets, fixtures,
etc all at once" and forbids compatibility layers. `serde_json` may remain only inside `mod tests`
as a differential oracle.

## Second seam, same character
`🔌️plugin/🦀️.rs:13089  fn try_serialize<T: serde::Serialize>(…)` — a framework generic still bound on
serde. Every payload type flowing through it inherits the bound, which is why plugin-side conversions
keep hitting `serde::Serialize` requirements they cannot influence. This is the same pattern as the
ten framework seams already migrated on this ticket: the plugin is never the real blocker.
