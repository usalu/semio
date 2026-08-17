# Wave 1 fix-up (done by orchestrator directly, not a spawned agent)

The wave-1 verify agent found two rename fallouts that landed outside any of
the 4 parallel agents' file ownership (a gap in how I partitioned the wave):

## 1. `semio_s_3d`/`semio_s_2d` → `semio_framework_3d`/`semio_framework_2d`

Renamed Rust source usages (NOT the Cargo.toml dependency declarations,
those were already correct) in 24 files across: os core (`💻️os/🦀️component.rs`,
mounted into the host crate via `#[path]`), os host, os-flow's
`🖍️drawing`/`📐️brep-geometry`, the wgpu renderer's `Shell` element, and 8
s-plugins (lowpoly ×9 files, demonstrator, cad ×5, process, procedural,
flow-extension-brep, flow-extension-draw).

Deliberately did NOT touch `✏️s/🔌️plugins/🖍️draw`'s own artifact-engine
file — its crate intentionally kept the `semio_s_2d` local alias (`Cargo.toml`
`semio_s_2d = { path = "...", package = "semio-framework-2d" }`), verified
before excluding it.

Verified via targeted `cargo check -p`: `semio-framework-os`,
`semio-framework-os-flow`, `semio-s-plugin-demonstrator`,
`semio-s-plugin-flow-extension-brep` all compile clean (warnings only).
Six other plugins (gis/process/sourcing/procedural/cad/lowpoly/puzzle) are
blocked by an unrelated, unrolling concurrent refactor (see below) — not by
this rename; confirmed by grep (their error messages never mention
`semio_s_3d`/`semio_s_2d`/`semio_framework_3d`/`semio_framework_2d`).

## 2. `studioMode`/`isStudioPluginFilter` → `hostMode`/`isHostPluginFilter`

Finished the rename the registry-genericization agent started but could not
complete (files outside its declared ownership): `🎠️kernel/🟦️component.ts`
(reconciled the `resolvePlaygroundBoot` local var with its already-renamed
export), `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (import + 3
call sites), `ShellHost/🟦️component.tsx` (51 occurrences, plus its two
derived names `studioSessionActive`→`hostSessionActive`,
`studioOverrideTabId`→`hostOverrideTabId`), `ShellHelpers/🟦️component.tsx`
(2 occurrences). Used `perl -pi` with `\b` word boundaries (macOS `sed` does
not support `\b`, first attempt silently no-op'd — caught by re-grepping).

Verified: `rg "isStudioPluginFilter|studioMode"` under `🧰️framework`
(excluding generated) → 0 hits. Confirmed the renamed import in os-dev
`📜️script.ts` matches the actual export name in registry `📜️script.ts`
(`isHostPluginFilter`, line 370). No duplicate-declaration collisions
(single `const hostMode` per scope, checked).

## Known-red, NOT fixed (concurrent, out of scope)

`cargo check --workspace` still shows failures unrelated to this wave's
work, all traced to another live session's in-progress refactor threading a
`document` concept through `AppDefinition`/`OsAppRegistration` and adding
matching `document` command modules to several plugins mid-flight
(gis/process/sourcing/procedural/cad/lowpoly/puzzle all show
`couldn't read .../📄️document/🦀️component.rs: No such file or directory`
at some polling moments and resolve cleanly moments later — a textbook
half-landed concurrent edit, matches the git-status snapshot showing modified
files under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/...` from another ticket).
`semio-framework-os` itself transiently failed once during a full workspace
run with `AppDefinition has no field named document` errors, then compiled
clean seconds later in isolation — confirms the churn, not a regression from
this ticket. `semio-compose-rs`'s `dsl`/`vcs` unresolved-crate errors are
verbatim in the pre-work baseline (`📸️baseline-cargo-check.txt`) — pre-existing,
unrelated, "exempt" technology per taxonomy.

Not touching any of the above — it's another session's live work in a
shared tree with no git isolation between us.
