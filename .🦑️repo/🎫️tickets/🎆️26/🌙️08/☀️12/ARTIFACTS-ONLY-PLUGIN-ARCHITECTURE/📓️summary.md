# Summary — Artifacts Only Plugin Architecture (#2549)

**Goal:** make artifacts the *only* mechanism for IO, state change, registration and side effects in plugins. A plugin becomes exactly `🎛️apps` + `🗿️artifacts`. The violation that opened the ticket was `✏️s/🔌️plugins/💠️lowpoly/🔧️setup/🦀️component.rs`, which mutated OS-host process-global registries from a plugin setup facet and registered IO for `3d.mesh`, a kind lowpoly does not own.

## The deliverable

**`ArtifactDeclaration`** (`🔌️plugin/🦀️component.rs:930-1241`) — a consuming typestate builder. Registration is now **data the framework walks in a fixed deterministic order**, replacing 33 hand-written callbacks whose ordering was implicit.

**`build()` validates ownership**, and this is the point of the ticket: every composer entry must **produce or consume** the declared kind, so export entries writing a foreign dialect still work while *a plugin registering IO for a kind it does not own is a hard error*. A strict `s.<plugin>.<artifact>` check activates automatically once kind ids become canonical — verified by tracing real on-disk dialects, not assumed.

**The lowpoly violation is now unrepresentable, not merely deleted.**

Composition slots take the snapshot *type* (`.composition::<S>()`), never a hand-written list — there is no public slice setter, so a divergent list is unwritable rather than discouraged.

## Measured outcomes

| | before | after |
|---|---:|---:|
| plugins with **no** `.setup()` | 0 | **22 / 33** |
| `declaration()` in the retired `⚙️engine` dir | — | **0** (45 at artifact root) |
| helpers widened to `pub` by the relocation | — | **0** |
| `plugin-closed-shape` breaches | 104 | **41** |
| `plugin-registration-setup-callback` | 31 | **14** |
| `plugin-dependency-os-host` | 13 | **10** |
| APA policy breaches, total | 1718 | **1383** |

`💠️lowpoly` verified: setup facet gone, plugin root closed to the target shape, **zero** `register_mesh_*` / `semio_framework_os::` calls, `cargo check --all-targets` 0 errors.

## Mechanisms landed

- Five policy rules defining the architecture (`PluginClosedShape`, `PluginPurity`, `DeclarativeRegistration`, `PluginDependencyAllowlist`, `EffectCapabilityParity`), plus a **shrink-only ratchet**: four ceilings on rows measured flat across three runs, four exemptions on rows measured moving.
- `ArtifactApp::app_schema()` and `document_codec_bare::<S,M>()`, closing two of the four `.setup()` categories.
- `HostHandleReachLintScript` — catches a plugin holding a host-owned engine handle in a static, a violation class the purity rule structurally cannot see (it measures *mutability*; this is *reach*).
- `PluginIndexExportPathLintScript` — 517 of 567 dead TS export paths, previously enforced by nothing.
- `🔣️taxonomy.json` `pluginChildDirs` → `["🎛️apps"]`, the precondition that made per-plugin facet deletion possible at all.

## What is NOT done, and why

- **`.setup()` cannot be deleted.** Eight plugins still calling it are held by other sessions. Three APA-held residues remain, each named and justified rather than force-converted: procedural's DWG mesh bridge and linked-flow installer, puzzle's OS media-host bridges, space's wasm app-registry mirror.
- **The escape-hatch family is not deleted.** 15 app/pane call sites remain. Twelve are `🎪️demonstrator`'s registrations for `2d.map` and `3d.cad`, **deliberately preserved**: demonstrator is the *sole* registrant, so deleting them removes capability that UCAS's composition work is meant to supply. Sequenced behind that work. DKM (#2550) is waiting on this deletion and knows it is gated.
- **Nothing is claimed compiler-verified beyond nine crates**, and even those are timestamps — `🗄️stdio` regressed three times during the ticket and every plugin depends on it.

## Corrections made to this ticket's own plan

1. **The taxonomy flip had to come first, not last.** A runtime `assert!` requires every listed facet dir on every plugin, so incremental deletion was impossible while the list named them. The planned target `["🎛️apps","🗿️artifacts"]` would have panicked the gate on all 33 plugins — `🗿️artifacts` has no leaf and is governed by a separate key.
2. **App-schema was not the last `.setup()` holdout.** Four categories existed. Two closed, one dissolved on inspection, one **deliberately left open** — an agent declined to add a declaration field for OS media-host bridges because doing so would legitimise the process-global registry this ticket documents as a bug.
3. **`3d.mesh` was a deletion, not a relocation** — stdio's `mesh` subset already existed. A peer also supplied the half APA's census missed: gis declares the same kind.

## The findings worth keeping

Full detail in `📌️important.md` and `📓️baselines.md`.

- **The silent rebind.** Moving a function changes what its unqualified paths resolve to. **44 of 45** artifact roots contain a shadowing `io_registry` whose `entries()` returns a differently-typed view. A bare call rebinds silently and compiles **green** — defeating `--all-targets`, `--keep-going`, `RUSTC_WRAPPER=""`, and structural checks simultaneously. Only reading the call site catches it.
- **Three instruments that return a confident, well-formed, wrong answer** — no `--all-targets`, no `RUSTC_WRAPPER=""` (sccache is a repo default and fails *green*), no `--keep-going` (reported 3 failing crates where the truth was 27).
- **A verification is a timestamp, not a property.** A ceiling is too. And **an exemption list built from intent is a guess; one built from two readings is a fact** — the ratchet agent overrode this ticket's own written instruction on the strength of its measurement, and was right.
- **Grep to find, enumerate to count.** Three sessions each produced a wrong count from a substring match in one hour.
- **Relocation is not finished when files are in the new place — it is finished when nothing points at the old one.** A cad relocation broke lowpoly, across a boundary no per-plugin agent could see.
