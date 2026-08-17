# 📓️ Lease request — root `Cargo.toml` (P1a-protocol-core → sol)

**Requestor**: terra, packet P1a-protocol-core.
**File**: `/Users/ueli/Documents/semio/Cargo.toml` (registrar-only, sol edits).
**Why**: The new crate `semio-framework-os-mcp` at
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust` cannot be built via
`cargo build/test -p semio-framework-os-mcp` until it is a workspace member. All P1a files are
otherwise complete and self-contained (zero dependency on `semio-framework`,
`semio-framework-os-kernel`, the plugin host, channel, or actor crate — verified by grep of the
new Cargo.toml).

## 1. `[workspace] members` — insert one line

Existing block (`/Users/ueli/Documents/semio/Cargo.toml` lines 4-14, os-module members only shown):
```toml
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust",
```

**Insert** (proposed position: immediately before the `🌊️flow` line — `🌉️` U+1F309 sorts
immediately before `🌊️` U+1F30A by codepoint; the existing block is not strictly codepoint-sorted
overall, so sol should place it wherever the repo's actual convention puts new `🔨️modules/`
entries):
```toml
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust",
```

## 2. `[workspace.dependencies]` — optional alias

Not requested yet — P1a has zero known downstream consumers this wave (P1b/P2/P6 are the first
consumers and are not landed). Per the cookbook's own rule ("if >5 downstream consumers"), skip
the alias for now; a later packet's lease-request can add
```toml
semio-framework-os-mcp = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" }
```
once real fan-out exists.

## 3. Nothing else requested

No `Cargo.lock`, `package.json`, `project.json`(root), `script.ts`(root), or launch.json changes
requested by this packet. `📋️project.json`/`📜️script.ts` inside the crate's own directory are
already in terra's owned paths and do not need sol.

---
Status: **pending** — terra continues implementation work without waiting; acceptance commands in
`…/📓️terra-P1a-report.md` will note if this lease was applied in time to actually run `-p
semio-framework-os-mcp`.
