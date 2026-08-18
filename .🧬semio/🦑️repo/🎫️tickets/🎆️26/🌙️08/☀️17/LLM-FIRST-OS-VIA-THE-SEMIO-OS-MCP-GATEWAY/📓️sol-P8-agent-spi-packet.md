# 📓️ sol brief — P8-agent-spi (verbatim)

You are "terra", an executor on ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` in /Users/ueli/Documents/semio. Packet id: **P8-agent-spi**. Model: Sonnet 5.

## 0. First action
Read in full: `…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📌️important.md`; `…/📓️design-decisions.md`; `…/📓️terra-P3-report.md` (the `ActionSemantics`/`ArgSchema` types you extend the builders with); `…/📓️terra-P2-report.md` (the catalog that will consume your output); `📋️master.md` §3.1 "AgentContributions"; `/Users/ueli/Documents/semio/CLAUDE.md`. **Also read the peer ticket's `📓️status.md` and its `📓️terra-E2-*report*.md`** (`…/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/`) — E1/E2 just built the descriptor emission you are extending, including a `PluginDescriptorExtras` side-channel populated in `try_build()`.
Save this brief verbatim as `…/📓️sol-P8-agent-spi-packet.md`.

## 1. Why this is time-critical
The peer ticket's **W3 freezes the plugin SDK** while 33 plugin crates migrate. Your change must land in the window **before** that freeze, or it waits for the entire migration. Move deliberately but do not dawdle, and keep the change strictly additive so it cannot destabilise their migration.

## 2. State of the world (verified by sol just now)
Their G1 is met (`plugin-host` checks clean, `CHANNEL_VERSION` 12). `✏️s/🔌️plugins/🗒️note/🔣️descriptor.json` is a **real committed descriptor** with `role`, `activationEvents`, `execution`, `capabilityRequests` — proof the descriptor pipeline works end to end. `ActionSemantics` and `ArgSchema` already exist in `🛂️manifest` (P3, landed, green).

## 3. Owned writable paths — READ CAREFULLY
Almost everything here belongs to the peer ticket's A2/E1/E2 packets. **You are principally a lease-writer, not a direct editor.**
Directly owned:
```
🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs   — ONE new additive region `//#region 🔖️AgentContributions`
.🧬semio/…/📓️sol-P8-agent-spi-packet.md, 📓️terra-P8-report.md, 📓️lease-P8-*.md, *.txt
```
Lease-only (write the exact diff, do NOT apply it):
```
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs   — AppBuilder/ExtensionBundle methods
🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs — ExtensionManifest.agent_contributions
🔌️plugin/📇️describe/**, 📇️registry/**                          — descriptor emission + registry check
```
Before writing any lease, re-read the target file from disk and run `git log --date=iso --oneline -3 -- <path>` so your diff applies to the live state, not to yesterday's.

## 4. Required result

### 4.1 `AgentContributions` (the one thing you implement directly)
New additive region in `🛂️manifest/🦀️component.rs`:
```rust
pub struct AgentContributions {
    pub capabilities: Vec<String>,   // capability refs this package offers to agents
    pub promoted: Vec<String>,       // the subset promoted to first-class MCP tools
}
```
plus its attachment point on `PackageDescriptor` (an `Option<AgentContributions>` field or an entry in the existing extras side-channel — **match whatever shape E1/E2 actually built**; read their code before choosing, and justify the choice in your report). serde + ts-rs mirrored like its neighbours; defaults must keep every existing construction site compiling untouched.
**Critical distinction to preserve and to document in the docstring**: `capability_requests` (existing) is what a package *needs permission for*; `AgentContributions` is what it *offers to agents*. Overloading one for the other is the exact mistake the design forbids.

### 4.2 Builder SPI (lease)
Exact diffs for `AppBuilder::capability(CapabilityDefinition-ish)` / `.use_when([..])` / `.effects(..)` / `.semantics(..)` and `ExtensionBundle::agent(..)`, all additive with defaults, so a plugin can declare agent-facing metadata where it already declares its actions. Since `CapabilityDefinition` lives in the gateway crate (D5) and the SDK must not depend on it, the builder surface takes **manifest-level** types (`ActionSemantics`, `use_when` strings) and the gateway compiles them — state this explicitly in the lease so the reviewer sees the layering is deliberate.

### 4.3 Descriptor emission + registry check (lease)
Exact diffs so `describe()` carries the agent block and the registry `check` validates it (referenced capabilities exist; promoted ⊆ capabilities; ids match the `<plugin>.<app>.<action>` grammar).

### 4.4 Prove it on one real plugin (lease + demonstration)
Write the exact diff that gives `🗒️note` (the plugin with a committed descriptor) real `use_when` + `semantics` on **two or three** of its actions, and show what its `🔣️descriptor.json` agent block would then contain. If you can apply and regenerate it without touching files outside your scope, do so and paste the resulting JSON; if not, the lease plus a precise worked example is the deliverable.

## 5. Acceptance (FOREGROUND, paste output + exit codes)
```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework
CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework 2>&1 | grep -c "^warning"
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp     # catalog must still compile
bun nx run @semio-tech/framework-rs:generate                                  # ts-rs mirror regenerates cleanly
```
Nothing you do may break `semio-framework-plugin` or the peer's in-flight migration — check `cargo check -p semio-framework-plugin --lib` too.

## 6. Hard rules
All of `📌️important.md`. Additive only — no field removals, no signature changes to anything a plugin already calls. **Never background a build.** No git-modifying commands. No `AGENTS.md`. No `.log`. Never claim an unrun result. Every lease must contain exact old→new text, the file's current SHA-256, and the reason.

## 7. Report
`…/📓️terra-P8-report.md`: what you implemented directly vs leased; the shape decision for the descriptor attachment with evidence from E1/E2's actual code; every lease in full; acceptance output; and a one-paragraph statement of how a plugin author declares an agent capability once this lands, written so a plugin author could follow it.
