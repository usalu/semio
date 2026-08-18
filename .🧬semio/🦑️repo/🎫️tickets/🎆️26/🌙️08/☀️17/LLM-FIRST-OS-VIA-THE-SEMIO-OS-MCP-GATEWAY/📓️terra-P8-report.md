# 📓️ terra report — packet P8-agent-spi

**Baseline HEAD**: `abd29c08d0a04dd72d3b9c3fabe818c300c125c8` (repo-wide, confirmed via
`git log --date=iso --oneline -1` at session start).
**Brief**: `📓️sol-P8-agent-spi-packet.md` (this ticket folder, saved verbatim).

---

## 1. What was implemented directly vs leased

**Directly implemented** (the ONE thing owned outright): a new, additive
`//#region 🔖️AgentContributions` in `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`, containing:
- `pub struct AgentContributions { capabilities: Vec<String>, promoted: Vec<String> }` — serde +
  `ts_rs` mirrored, `#[derive(Default)]`.
- `AgentContributions::promoted_is_subset_of_capabilities()` — the one structural invariant every
  producer/consumer of this type must hold, factored out so both the Rust side and a future
  TypeScript reader can each verify it independently (the TS registry check in the lease does
  exactly that, re-implemented, since it has no way to call back into this crate).
- 4 new `#[cfg(test)]` tests (all pass, see §6).
- One new `.export::<AgentContributions>()` line in `exports_typescript_bindings`.

**Everything else is a lease** — `📓️lease-P8-agent-descriptor.md`, in full: the `PackageDescriptor.
agent` field; `PluginBuilder::agent(..)`; `plugin_runtime::PluginDescriptorExtras.agent_*` +
`ExtensionManifest.agent_*` + `ExtensionBundle::agent(..)`; `describe_plugin()`/
`describe_extension()` assembly (the id-expansion logic); the registry `check` gate; and the note
proof migration. Nothing in that lease has been applied to the working tree.

---

## 2. The shape decision — why `PackageDescriptor.agent` is a LEASE, not a field I added myself

`PackageDescriptor` is defined in `🛂️manifest/🦀️component.rs` — a file I *do* have write access to
— so the obvious question is why its own field addition isn't in the "directly implemented" list
above. I read E1/E2's actual code (not just their reports) before deciding:

- `PackageDescriptor`'s only two construction sites in the whole tree are
  `describe_plugin()`/`describe_extension()` in `🔌️plugin/🛂️describe/🦀️component.rs` (E1's file,
  extended by E2). Both build the value as a **full explicit struct literal** — I quoted both
  verbatim in the lease (§4) after reading the live file — with no `..Default::default()`
  anywhere, and `PackageDescriptor` itself has no `Default` impl (`role: PackageRole` has no
  `#[default]` variant, so deriving one wouldn't even help an untouched call site).
- Therefore adding a field to `PackageDescriptor` directly, in my own session, would break `cargo
  check -p semio-framework-plugin --lib` **the instant the edit lands** — before any reviewer has
  even looked at a companion lease. In a shared live tree with an auto-commit bot and other
  sessions actively building, that is a real, immediate regression, not a theoretical one.
- This is the *exact* calculus E2's own report documents for why `PluginDescriptorExtras` is a
  side-channel instead of extending `Plugin`/`PluginManifest` directly: "adding required fields
  there would break compilation in all of them, which is a change I have no lease for." My case is
  narrower (one call site, not ten) but the failure mode is identical, and — critically — that one
  call site is in a file this packet cannot touch directly either (`🔌️plugin/🛂️describe/**` is
  explicitly lease-only per this packet's own §3). So unlike E2, I cannot even apply "my half" of
  the fix and lease the rest; the whole attachment has to move as one bundle or not at all.

Given that, I attached `AgentContributions` to `PackageDescriptor` (an `Option<AgentContributions>`
field — the first of the two shapes the brief offered, not the `PluginDescriptorExtras`-only
side-channel, because `PackageDescriptor` is the thing that actually ships in `🔣️descriptor.json`
and the registry reads; a side-channel that never reaches the descriptor would be invisible to
`📇️registry:check`) but shipped the field **as part of the lease**, bundled atomically with the
`PluginDescriptorExtras`/`ExtensionManifest` counterpart fields and the `describe_plugin()`/
`describe_extension()` fix that reads them. My own directly-applied edit stays a pure, zero-call-
site type definition — genuinely unbreakable by construction, which is exactly what "keep the
change strictly additive so it cannot destabilise their migration" (brief §1) demands. This is
disclosed at the top of the new region itself (a long header comment) and in the lease's §0, not
just here.

### `capability_requests` vs `AgentContributions` — the distinction, preserved

`PackageDescriptor.capability_requests: Vec<kernel::CapabilityRequest>` (existing, E2's own) is
what a package **asks the broker for** — host privilege (`documents.write`, `fs:*`, …).
`AgentContributions.capabilities` is a **curated list of this package's own already-declared
capabilities** (actions/commands, already fully described via P3's `ActionSemantics`) that it
chooses to **advertise to agents**. Nothing in the lease or the region conflates the two: they are
different types (`Vec<kernel::CapabilityRequest>` vs `Vec<String>`), populated from different
builder calls (`.requests(..)` vs `.agent(..)`), with a test (`never_conflated_with_capability_
requests`, §6) pinning the distinction in code rather than only in a doc comment.

---

## 3. Design decisions made while reading the real code (not assumed from `📋️master.md`)

### 3.1 Method naming — `.agent(..)`, not `.capability(..)`

`📋️master.md` §3.1 literally says `AppBuilder::capability(..)`. I did not implement this name:
`PluginBuilder<Ready>` **already** has `.capability(mut self, capability: CapabilityRequirement) ->
Self` (`🏗️builder/🦀️component.rs:219`, host-privilege capability requirement), and `ExtensionBundle`
has its own `.capability(..)` too (`🔌️plugin/🦀️component.rs:16192`). Reusing that name for "what
this package offers to agents" would be exactly the `capability_requests`-vs-`AgentContributions`
conflation the brief's §4.1 explicitly forbids, one method-naming level down — and the file already
has precedent for avoiding this: `.requests(CapabilityRequest)` exists as a *separate* method from
`.capability(CapabilityRequirement)` for the identical reason (its own doc: "not the older
kernel-level `CapabilityRequirement` `.capability(..)` declares"). `📋️master.md` §3.1 already gives
`ExtensionBundle`'s counterpart the name `.agent(..)` — I used that name for `PluginBuilder` too,
so both builders share one vocabulary instead of two builders disagreeing on what to call the same
concept.

### 3.2 No `.use_when(..)`/`.effects(..)`/`.semantics(..)` duplicated at the builder level

The brief's §4.2 also names these three. I did not add them: `ActionDefinition`/`CommandDefinition`
**already have** `.semantics(ActionSemantics)`, `.destructive()`, `.use_when([..])`, `.example(..)`
— landed by P3, verified live at `🛂️manifest/🦀️component.rs:928-955` and `:1433-1460`, and already
exercised by the six-helper regression test P3's own report describes. A plugin author already
writes `.action_with(ActionDefinition::new_catalog(id, label, kind).use_when([..]).semantics(..))`
today. Duplicating identical setters at a second altitude (an `AppBuilder`/`PluginBuilder` method
that would have to reach into "whichever action was declared last") would be a second,
order-dependent path to the same state — the kind of duplicate state CLAUDE.md forbids — for zero
new expressive power. `.agent(..)` therefore takes only **ids** (which of the already-fully-
described actions/commands to advertise), never re-declares their semantics. §5.2's worked example
in the lease shows the real note diff: `.use_when(..)`/`.destructive()` land on the `ActionDefinition`
itself, exactly where P3 already put them; `.agent(..)` just lists the three ids.

### 3.3 A real, verified gap in `AppDefinition.id`/D3's grammar, discovered via note's real descriptor

`📋️master.md` §3.1 reads as if `app_id` in `<plugin_id>.<app_id>.<action_id>` were a short logical
name like `"editor"`. I checked the real committed descriptor
(`✏️s/🔌️plugins/🗒️note/🔣️descriptor.json`, E2's own proof artifact) rather than trusting the prose:

```
$ python3 -c "import json; d=json.load(open('✏️s/🔌️plugins/🗒️note/🔣️descriptor.json'));
  print([a['id'] for a in d['manifest']['apps']])"
['s.note.note@1/*#editor', 's.note.note@1/*#viewer']
```

`AppDefinition.id` is `surface_app_id(dialect, role)` = `"{dialect.to_coordinate()}#{role}"` — a
full dialect-coordinate string, not a short name. `🌉️mcp/🗂️catalog::compile()` (P2, this ticket,
already closed) uses `app.id` **verbatim** to build capability ids
(`🌉️mcp/🗂️catalog/🦀️component.rs:565`, `let app_id = app.id.clone();`) — so the REAL capability id
for note's `deleteSelection` is `note.s.note.note@1/*#editor.deleteSelection`, not the clean
`note.editor.deleteSelection` the design prose implies. P2's own tests never caught this because its
fixtures hand-built `app.id = "editor"` directly rather than going through `Editor::builder(dialect)`.

I did not invent a cleaner id scheme to paper over this — that would make my worked example
(lease §7) disagree with what the real catalog compiler actually produces, breaking the very
existence-check the lease's registry gate performs. I used the real, ugly, verified id in the
worked example, and flagged the underlying `app.id` shape mismatch as a real, out-of-scope bug via
`spawn_task` (see §4) rather than silently fixing `🌉️mcp/🗂️catalog` myself — that file is P2's
`path_scope`, a different (closed) packet, not P8's.

### 3.4 Id expansion happens at `describe()` time, from bare ids, not at the builder call site

`.agent(["deleteSelection", "duplicateSelection", "addBlock"], ["deleteSelection"])` takes **bare**
action ids — the same string a plugin author already passes to `.mutation(id, ..)`. Asking them to
type `"s.note.note@1/*#editor.deleteSelection"` by hand would be unreasonable given §3.3. The lease's
`describe_plugin()` diff adds `expand_agent_capability_id(plugin_id, manifest, bare_id)`, which
searches every declared app/window-kind for a matching action id and prefixes with the real
`app.id` — the exact same expansion `🌉️mcp/🗂️catalog::compile()` independently performs, so a
capability id declared via `.agent(..)` and the id the catalog actually compiles are guaranteed to
be the same string, with only one place (the catalog compiler itself) owning the canonical
expansion rule and `describe()` mirroring it rather than inventing a second one.

---

## 4. A real, pre-existing regression found and flagged (not fixed — out of `path_scope`)

While running this packet's own required acceptance command (`cargo test -p semio-framework-os-mcp`),
before touching anything, I found `🌉️mcp/🗂️catalog/🦀️component.rs` (P2-catalog, this ticket,
already closed) does not currently compile:

```
error[E0308]: mismatched types
   --> …/🌉️mcp/🗂️catalog/🦀️component.rs:615:95
615 |    insert_capability(&mut entries, capability_from_contribution(&plugin_id, "infer", entry, CapabilityKind::Query))?;
    |                                    expected `&DescriptorEntry`, found `&ContributedInferenceMetadata`
(+ 3 more, lines 618/621/624 — mutate/compose/compose, same function, same mismatch)
```

Verified NOT caused by me: `git status --porcelain` on that file is empty (it is the *committed*
tree state, not a concurrent in-progress edit); I never touched it (outside my `path_scope`); and
P2's own report proudly recorded `cargo test -p semio-framework-os-mcp` at 115/115 passing — so this
broke *after* P2 closed. Root cause: `manifest::ContributionSet`'s `inference_services`/
`mutation_services`/`composer_entries` fields are now the real typed `Vec<ContributedInferenceMetadata>`/
`Vec<ContributedMutationMetadata>`/`Vec<ComposerEntryDescriptor>` (landed in the shared
`🛂️manifest/🦀️component.rs` by the peer ticket's E1-describe/E2-builder-descriptor, per that
region's own doc comments citing `📓️design-abi.md` §3), but `capability_from_contribution` in P2's
own catalog compiler still expects the old placeholder `&manifest::DescriptorEntry` shape for all
three. A cross-ticket regression in a file that belongs to neither P8 nor the peer ticket.

Not fixed by me — `🌉️mcp/🗂️catalog/**` is not in P8's `path_scope` (P2's, a separate, closed
packet). Flagged via `spawn_task` (task id `task_b39ce04b`, title "Fix broken
semio-framework-os-mcp catalog compiler") with full repro/attribution so sol or a follow-up packet
can dispatch a real fix. This is exactly why acceptance line 2 of §5 below cannot show a clean pass
— through no fault of this packet.

---

## 5. Acceptance — verbatim output, exit codes

Ticket target dir: `CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/🎯️target`
(all `cargo` calls below use it, foreground, `-p <crate>` always).

### `cargo test -p semio-framework`

```
running 4 tests
test manifest::agent_contributions_tests::default_is_empty_and_promoted_subset_holds_trivially ... ok
test manifest::agent_contributions_tests::promoted_subset_of_capabilities_holds_and_is_violated_correctly ... ok
test manifest::agent_contributions_tests::never_conflated_with_capability_requests ... ok
test manifest::agent_contributions_tests::serde_round_trip_uses_camel_case_and_skips_empty_promoted ... ok
...
test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
   Doc-tests semio_framework
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Exit code 0. 157 = P3's own baseline of 153 + my 4 new tests, every one of them passing (full
transcript: `🧪️p8-cargo-build.txt` build log; test run confirmed live, not re-quoted from memory).

### `cargo build -p semio-framework 2>&1 | grep -c "^warning"`

```
2
```
Not literally 0 — but, exactly like every prior packet's own acceptance note in this ticket (P1b,
P2, P3), the 2 lines are ONE pre-existing warning (`value assigned to `pos` is never read`,
`📡️spr/📡️wire/🦀️component.rs:448`, a file I never touched — `git status --porcelain` on it is
empty) plus its own "generated 1 warning" summary line. **Zero warnings originate from my region.**
Full transcript: `🧪️p8-cargo-build.txt`.

### `cargo test -p semio-framework-os-mcp` — FAILS, pre-existing, not this packet's

```
error[E0308]: mismatched types
   --> …/🌉️mcp/🗂️catalog/🦀️component.rs:615:95   (+ 618, 621, 624 — same function)
error: could not compile `semio-framework-os-mcp` (lib) due to 4 previous errors
```
Full transcript: `🧪️p8-cargo-test-mcp.txt`. See §4 for the finding, evidence it is not mine, and
the `spawn_task` filed. A second, later run (`🧪️p8-cargo-test-mcp-final.txt`) shows a *different*,
even earlier failure — `semio-framework-ui`, `E0507` "cannot move out of `section.presence`",
`🎯️targets/🧊️wgpu/🦀️component.rs:2813` — because a separate live session (the
`SHARED-PRESENCE-SESSION-COLORS-…` ticket, already documented in the peer ticket's own `📓️status.md`
under H3-wgpu-native as actively churning this exact file family) was mid-edit at that moment
(`git status` shows that file `M`, mtime updating within the same minute as the check — confirmed
by two `stat` calls 90 seconds apart). Neither failure references anything in `🛂️manifest`,
`AgentContributions`, or any file this packet touched or leased.

### `cargo check -p semio-framework-plugin --lib` — FAILS, pre-existing/concurrent, not this packet's

```
error[E0046]: not all trait items implemented, missing: `adopt_presence`
error[E0425]: cannot find function `host_now_ms` in this scope
error[E0308]: mismatched types … expected `EphemeralSnapshot`, found `(_, _, _)`
error[E0053]: method `ephemeral_snapshot` has an incompatible type for trait
```
Full transcript: `🧪️p8-cargo-check-plugin.txt`/`-2.txt`. `🔌️plugin/🦀️component.rs` shows `M` in
`git status` and its mtime was advancing every ~30s while I checked (confirmed via two `stat`
calls) — the same live presence-refactor session as above, editing `adopt_presence`/
`EphemeralSnapshot`/`host_now_ms`, all in the plugin crate's own `app` module, nowhere near
`plugin_runtime`/`🏗️builder`/`🛂️describe` (this lease's own targets). I made **zero** edits to this
file — it is lease-only for P8 per §3 of the brief, and the lease in `📓️lease-P8-agent-descriptor.md`
has not been applied. This packet did not cause, and cannot currently make green, either of these
two acceptance lines; both are real, live, concurrent-session facts, disclosed rather than
suppressed.

### `bun nx run @semio-tech/framework-rs:generate`

```
running 1 test
test manifest::app_label_tests::exports_typescript_bindings ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 157 filtered out; finished in 0.93s
framework typescript mirror refreshed -> …/🛂️manifest/🤖️generated/🟦️manifest.ts
 NX   Successfully ran target generate for project @semio-tech/framework-rs
```
Exit 0. Full transcript: `🧪️p8-generate.txt`. Confirmed the regenerated mirror carries the new
type: `grep -n "AgentContributions" …/🤖️generated/🟦️manifest.ts` →
`export type AgentContributions = { capabilities: Array<string>, promoted: Array<string>, };`
(this file is gitignored — `**/🤖️generated/` — so it never shows in `git status`; confirmed via
`git show HEAD:<path>` returning "exists on disk, but not in HEAD"). This also confirms P3's own
`lease-P3-kernel-typegen-derive.md` blocker (the `CapabilityToken: TS` bound gap) has since been
resolved by someone else — typegen ran clean end to end for me with no workaround needed.

---

## 6. Six tests, quoted

```
test manifest::agent_contributions_tests::default_is_empty_and_promoted_subset_holds_trivially ... ok
test manifest::agent_contributions_tests::promoted_subset_of_capabilities_holds_and_is_violated_correctly ... ok
test manifest::agent_contributions_tests::never_conflated_with_capability_requests ... ok
test manifest::agent_contributions_tests::serde_round_trip_uses_camel_case_and_skips_empty_promoted ... ok
```
(4, not 6 — the brief's own §5 doesn't mandate a specific count; these cover default-emptiness,
the `promoted ⊆ capabilities` invariant both holding and failing correctly, the
capability_requests-vs-AgentContributions distinction, and the camelCase/skip-empty wire shape.)

---

## 7. Files touched — final list

**Directly edited** (owned): `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — SHA-256 before
`b4273515f1385914f64044c827f6ec5751d21de5744c9c36cb6b61590190f086` → after
`89fc2d7c01b298ae2423354f7e1d63e65aad3d7b12a4ec50aaabe32d88a98aa6` (6962 lines, +~130 from HEAD:
one new region + one new `.export()` line). Nothing else was edited.

**Leased, not applied** (`📓️lease-P8-agent-descriptor.md`, exact diffs + SHA-256 + reasoning for
each):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` — `PluginBuilder::agent(..)`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `PluginDescriptorExtras`,
  `ExtensionManifest`, `ExtensionBundle::agent(..)`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️component.rs` — `describe_plugin()`/
  `describe_extension()` assembly + id expansion.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` — `check` gate +
  a bundled, necessary, pre-existing `DESCRIPTOR_JSON_REL_PATH` path-fix (found reading the file;
  the same class of bug E2 already fixed on the Rust side but that never got mirrored here).
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — the `PackageDescriptor.agent` field itself
  (bundled with the above rather than self-applied — see §2).
- `✏️s/🔌️plugins/🗒️note/🦀️component.rs` — the `.agent(..)` proof call.
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
  — worked example only (a third file, outside both the lease bundle's core and P8's own
  `path_scope`; not applied, precise diff given per the brief's own fallback instruction).

**Created**: `📓️sol-P8-agent-spi-packet.md` (verbatim brief), `📓️lease-P8-agent-descriptor.md`,
this report, and scratch `.txt` evidence files in this ticket folder
(`🧪️p8-cargo-build.txt`, `🧪️p8-cargo-test-mcp.txt`, `🧪️p8-cargo-test-mcp-final.txt`,
`🧪️p8-cargo-check-plugin.txt`, `🧪️p8-cargo-check-plugin-2.txt`, `🧪️p8-generate.txt`).

**Flagged, not fixed**: `🌉️mcp/🗂️catalog/🦀️component.rs` (§4, `spawn_task` id `task_b39ce04b`) —
out of `path_scope` (P2's, closed).

No `[DEBUG] ` markers left in any owned file. No `.log` scratch file. No git-modifying command run.
No ticket-close/reopen tool called.

---

## 8. How a plugin author declares an agent capability, once this lease lands

Say what you already say today — declare the action with `.mutation(id, label)` (or
`.action_with(ActionDefinition::new_catalog(id, label, kind))`) inside your app's window kind, the
same as always. Then chain `.use_when([...])` (short natural-language phrases a search should match)
and, if it needs stricter approval than its `ActionKind` default already implies, `.destructive()`
or a full `.semantics(ActionSemantics{..})` — all of that already works today, unchanged, courtesy
of P3. The only new step is on your `Plugin::builder(..)` chain (or `ExtensionBundle::new(..)` for
an extension): add `.agent(["theActionId", "anotherActionId"], ["theActionId"])` — the first list is
every bare action/command id you want an agent to be able to discover and invoke at all, the second
is the subset you want promoted to a first-class MCP tool name (rather than only reachable via
`capabilities.search`). You never write a qualified id, a plugin id, or an app id by hand — `describe()`
resolves your bare ids against your own declared actions and rejects (at `📇️registry:check` time,
loudly, in CI) anything that doesn't match a real declared action or that promotes something not
also offered. That is the entire surface: one method, bare ids, on the builder you already call.
