# Close-prep 1022 (auditor `close-prep`)

Ticket `GET-CODEBASE-RUNNING-END-TO-END-AGAIN`, prepared 2026-10-22 about 20:15 local. Read-only audit: nothing was deleted or edited except this report and my own outputs in `close-prep-tools/` and `🗑️generated/close-prep-*`.

Method: the 50 `result-*.md` files modified on 2026-10-22 (list in `🗑️generated/close-prep-results-1022.txt`) were scanned for repo path mentions. A mention counts as an edit when its line or nearest heading reads as a change (LIVE, fixed, changed, applied, patched, and so on). Mentions were resolved against `git ls-files -co --exclude-standard` (tracked plus untracked, not gitignored), first exact, then by suffix, because results use abbreviated roots (`L/`, `G/`, `L2/`, ...). Scripts: `close-prep-tools/resolve.py`, `build-report.py`, `write-report.py`, `staged-dupes.py`.

## 1. Repo files changed today (for `ticket_close` `files`)

- 245 repo paths, all present on disk, none gitignored. No listed path is missing, so none is marked as gone.
- `package.json` is first as an ASCII anchor (memory `feedback-ticket-close-emoji-only-files-array-fails`). It is tracked and was not necessarily edited.
- Ticket-folder and other-ticket paths are excluded. `ticket_close` handles its own files.
- Not in the list, to resolve before closing: 114 edit mentions match several repo files (ambiguous; candidates in `close-prep-resolved.json`), and 68 are bare short forms such as `🟦️.ts` or `T/anchored.py` that cannot be matched. Mention-only references (no change wording) are also left out.
- Gitignored paths are dropped silently by `ticket_close` (memory `feedback-ticket-close-needs-trackable-files`). Every path here came from `git ls-files -co --exclude-standard`, so none is ignored.

Plain list, one path per line (copy into the `files` array as is, paths are repo-relative):

```
package.json
.github/workflows/build-system.yml
♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📜️script.ts
♻️mit-bestand/📋️bericht/🧪️tests/🧫️fixtures/🧭️commands.json
♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🎭️e2e/🧫️cases.json
✏️s/package.json
✏️s/tsconfig.json
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎥️set-camera/🧾️wire-witness/🦠️mutation/🔣️.json
✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🟦️.ts
✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/📋️project.json
✏️s/🧑‍💻dev/🧩️catalog/📣️publication/📜️script.ts
✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts
✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🎮️session/🟦️.ts
🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/🟦️.ts
🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🟦️.ts
🎓️teaching/🏛️architecture/📚️course/🧪️tests/🟢️continuous/🧾️manifests/🟦️.ts
🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🧩️compose-caddy-and-workflow/🟦️.ts
🎓️teaching/🏛️architecture/📚️course/🧪️tests/🩺️preflight/🟦️.ts
🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚀️readiness/🔣️.json
🎓️teaching/🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts
🎓️teaching/🛂️proctor/🔨️modules/📚️course/🧪️tests/🔬️unit/🦀️.rs
📜️script.ts
🧰️framework/Cargo.lock
🧰️framework/Cargo.toml
🧰️framework/📦️packages/🦀️rust/Cargo.toml
🧰️framework/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🔨️modules/⏪️time-travel/🎮️decision/📨️command/🦀️.rs
🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/🦀️.rs
🧰️framework/🔨️modules/⏱️trace/🦀️.rs
🧰️framework/🔨️modules/⏳️async/🧪️tests/🥣️companion/📬️publication-claim/🦀️.rs
🧰️framework/🔨️modules/⏳️async/🧪️tests/🥣️companion/🧵️thread-wake/🦀️.rs
🧰️framework/🔨️modules/⚙️runtime/🧭️selection/🟦️.ts
🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs
🧰️framework/🔨️modules/🌱️value/📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json
🧰️framework/🔨️modules/🌱️value/🛬️decode/♻️retirement/🧪️tests/🦀️.rs
🧰️framework/🔨️modules/🌱️value/🧪️tests/🧩️neutral-owner/🦀️.rs
🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🧱️chunks/🦀️.rs
🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs
🧰️framework/🔨️modules/🎠️kernel/🦀️.rs
🧰️framework/🔨️modules/🎭️actor/🔗️bindings/🦀️.rs
🧰️framework/🔨️modules/🏃️process/📥️capture/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🌊️relay/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/📥️sources/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🧫️fixtures/📜️queue.json
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/📜️ticket.json
🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🛡️custody/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts
🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔎️key/🧪️tests/🦀️.rs
🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔺️diff/🪢️sequence/🧪️tests/🦀️.rs
🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/🟰️change/📋️indexed/🦀️.rs
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🗃️assets/🟦️.ts
🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🟦️typescript/package.json
🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🧮️expression/🦀️.rs
🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🎮️operation/🦀️.rs
🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust/Cargo.toml
🧰️framework/🔨️modules/🧵️job/⏱️context/🎟️source/👶️child/🧪️tests/🧫️fixtures/🔣️.json
🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🧪️tests/🌍️public/🦀️.rs
🧰️framework/🔨️modules/🧵️job/🧪️testing/📬️payload/🦀️.rs
🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🧬️schema/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🖼️scene/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🔌️ports/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🧯️close-host-drop-and-pair-merge/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧯️close/📥️lifted/🏪️store/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/📚️catalog/🧪️tests/🔬️unit/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🚪️ownership/🔣️.json
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🧪️tests/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-facets/🧊️reference/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🪞️window-preview/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗄️store-owners/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preview/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧭️owner-registry/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/📬️mutation/🧪️tests/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/📦️crate-boundaries/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json
🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts
🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🧬️cuts.json
🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json
🧰️framework/🛍️products/🦑️repo/README.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🔌️mcp/🐹️.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧭️repo-cli/🚦️verbs/🧩️system-verbs/🦀️.rs
🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🔭️provider-interfaces/🐹️.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🧪️tests/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🤝️shared/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/⚡️recovery/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/⚡️recovery/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📇️format/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🔣️policy.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🧪️tests/📏️needs/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📥️inference/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🍞️bun/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🐍️keying.py
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🛡️followup/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/💾️storage/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📥️acquisition/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🧊️pristine/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🧫️fixtures/🛠️binaryen-toolchain/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/📥️installation/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎨️styling/🏗️builder/🐍️python/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🖥️native-host/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🖥️native-host/🧪️tests/🧮️resources/🐍️oracle/🐍️.py
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/💾️storage/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/💾️cache/📋️ledger/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/📥️entry/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧊️verified-blocks/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments/🧪️tests/⚖️contracts/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🛰️service/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🔣️glob-dialects/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/📌️received-pin/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🧭️sites/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧾️diagnostics/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔌️backends/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🏭️runner/🍞️bun/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📸️snapshot/🧪️tests/🔒️planned/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️keys/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🪵️failure-tail/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🏭️generator-outputs/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🏭️generator-outputs/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔁️bootstrap/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🗂️components/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🧪️tests/📜️toml/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🧪️tests/🦀️cargo-go/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🔖️output-stamps/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🔖️output-stamps/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🪢️revision-graph/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🪢️revision-graph/🧪️tests/⚖️roster/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/🧩️units/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🔄️reexec/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🚀️launch/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪞️parity/🔣️help.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/✅️owned-file-approvals/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🏠️owner-projection-policy/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📦️payload/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📦️payload/🟨️.cjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🧾️owned-preview-refusal-reason/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🧮️tracked-path-budget/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧮️tracked-path-budget/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧰️toolchain-pins/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔍️inventory-planning/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🔵️.ps1
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/🏗️entry/🐹️.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🗂️registry-contract/🐹️.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🧱️cache-isolation/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🧱️cache-isolation/🧮️runner/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs
🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/📦️packages/🐹️go/🐹️.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/📦️packages/🦀️rust/💽️system-ports/🦀️.rs
```

## 2. Inventory of `🗑️generated/`

Total 21.7 GiB (`du`): 214 subfolders plus 839 loose files (1.75 GiB).

- KEEP, live or touched after 19:00: 15 subfolders, 0.57 GiB. Live processes at about 20:10: the course3 orchestrator (pid 6342, `e2e-orch`), slice-play3 loop (pid 6995), demo-verify wrapper (pid 10738), and the rust-health, apps-a2, apps-b, ts-verify, plugin-chain2 and rust-reds jobs. Loose files touched after 19:00: 7.
- CANDIDATE, owner check: `course-rest-stale-framework-cache2`, `course-rest-new-framework-cache`, `course-rest-stale-framework-cache3`, 16.19 GiB. These are framework caches (`boot-recovery/`, `b/`, `framework/leases`), not live, and referenced by `result-course-rest.md` and `result-boot-rebind2.md`. Confirm with the owner before deleting, since they hold boot state.
- CANDIDATE: 196 subfolders, 3.18 GiB, plus 832 loose files (1.75 GiB). Delete at close per AGENTS.md (tool output), after the owners confirm.
- `report-e2e-orch-*.md` cite `e2e-orch/` run folders, so keep `e2e-orch/` until the course3 report is final.

| size (MiB) | status | subfolder |
|---:|---|---|
| 8258.9 | CANDIDATE, owner check (framework cache, not live) | `course-rest-stale-framework-cache2/` |
| 8258.1 | CANDIDATE, owner check (framework cache, not live) | `course-rest-new-framework-cache/` |
| 376.2 | CANDIDATE | `gran-hubs/` |
| 270.0 | CANDIDATE | `input-precision/` |
| 215.0 | CANDIDATE | `wsr/` |
| 201.4 | CANDIDATE | `gran-value/` |
| 174.5 | CANDIDATE | `rs-sweep3-run/` |
| 174.1 | KEEP (live or touched after 19:00) | `rust-carve-x/` |
| 167.5 | CANDIDATE | `cargo-fix/` |
| 154.0 | CANDIDATE | `manifest-batch2/` |
| 152.2 | CANDIDATE | `chain/` |
| 141.6 | KEEP (live or touched after 19:00) | `rust-health-logs/` |
| 134.2 | CANDIDATE | `fnsplit4a-census/` |
| 98.2 | CANDIDATE | `integrate3-apply/` |
| 94.4 | CANDIDATE | `chain10/` |
| 87.8 | CANDIDATE | `chain17/` |
| 83.4 | KEEP (live or touched after 19:00) | `apps-a2-x/` |
| 71.0 | CANDIDATE | `window-config-1022/` |
| 64.4 | CANDIDATE | `seed-run/` |
| 63.0 | CANDIDATE, owner check (framework cache, not live) | `course-rest-stale-framework-cache3/` |
| 62.5 | CANDIDATE | `cargo-window/` |
| 60.4 | KEEP (live or touched after 19:00) | `rust-port-logs/` |
| 54.6 | CANDIDATE | `aud-nct-run/` |
| 47.6 | CANDIDATE | `value-repoint/` |
| 45.9 | CANDIDATE | `ide-free/` |
| 44.5 | CANDIDATE | `tax6-sim/` |
| 40.9 | CANDIDATE | `apps-a-x/` |
| 38.7 | CANDIDATE | `wire-fix/` |
| 30.1 | KEEP (live or touched after 19:00) | `apps-b-1022/` |
| 29.5 | CANDIDATE | `aud-verify7-x/` |
| 29.1 | CANDIDATE | `lock-sync/` |
| 28.4 | CANDIDATE | `comments-b/` |
| 25.5 | KEEP (live or touched after 19:00) | `rust-rebase-logs/` |
| 25.5 | CANDIDATE | `rs-sweep2-run/` |
| 25.3 | CANDIDATE | `lockref/` |
| 22.9 | CANDIDATE | `rust-tests-logs/` |
| 21.6 | KEEP (live or touched after 19:00) | `slice-demo2/` |
| 20.2 | CANDIDATE | `tax3-sim/` |
| 19.0 | CANDIDATE | `ts-health-run/` |
| 18.6 | KEEP (live or touched after 19:00) | `slice-play3/` |
| 18.0 | CANDIDATE | `launcher-zt/` |
| 17.3 | CANDIDATE | `bzl-nocache/` |
| 17.2 | CANDIDATE | `aud-tax-s/` |
| 17.2 | CANDIDATE | `slice-demo/` |
| 16.8 | CANDIDATE | `integrate3-x/` |
| 16.6 | KEEP (live or touched after 19:00) | `plugin-chain2-1022/` |
| 15.8 | CANDIDATE | `rs-sweep-x/` |
| 14.1 | CANDIDATE | `tax3-run/` |
| 13.9 | CANDIDATE | `cycle-break-x/` |
| 12.4 | CANDIDATE | `round6-apply/` |
| 11.8 | CANDIDATE | `repoapi/` |
| 11.5 | CANDIDATE | `round4-apply/` |
| 11.4 | CANDIDATE | `i18n-core-apply/` |
| 11.2 | KEEP (live or touched after 19:00) | `course-rest2/` |
| 10.4 | CANDIDATE | `comments-a/` |
| 10.3 | CANDIDATE | `chain25/` |
| 9.9 | CANDIDATE | `rs-sweep3-x/` |
| 9.7 | CANDIDATE | `tst8/` |
| 9.3 | CANDIDATE | `chain15/` |
| 9.2 | CANDIDATE | `rs-sweep4-run/` |
| 8.4 | CANDIDATE | `go-all-logs/` |
| 8.2 | CANDIDATE | `rust-port/` |
| 8.1 | CANDIDATE | `tst7/` |
| 7.6 | CANDIDATE | `slice-play2/` |
| 7.5 | CANDIDATE | `bzl-nocache-after/` |
| 7.3 | CANDIDATE | `round2-apply/` |
| 6.9 | CANDIDATE | `round3-apply/` |
| 6.7 | CANDIDATE | `bzl-suite/` |
| 6.7 | CANDIDATE | `rs-sweep-run/` |
| 6.6 | CANDIDATE | `value-fix-sweep/` |
| 6.5 | CANDIDATE | `integrate-apply/` |
| 5.4 | CANDIDATE | `aud-gates-tools/` |
| 5.2 | CANDIDATE | `gran-value-apply2/` |
| 5.0 | KEEP (live or touched after 19:00) | `e2e-orch/` |
| 4.5 | CANDIDATE | `conc-slice/` |
| 4.5 | CANDIDATE | `small-fixes/` |
| 4.3 | CANDIDATE | `tax6/` |
| 4.1 | CANDIDATE | `policy-admit-scan/` |
| 4.0 | CANDIDATE | `bzl-coverage/` |
| 3.9 | CANDIDATE | `census2/` |
| 3.6 | CANDIDATE | `coord/` |
| 3.6 | CANDIDATE | `chain23/` |
| 3.5 | CANDIDATE | `graph-perf-1022/` |
| 3.4 | CANDIDATE | `store-split-x/` |
| 3.2 | CANDIDATE | `aud-census10-tools/` |
| 3.2 | CANDIDATE | `incr-locks-run/` |
| 3.2 | CANDIDATE | `bzl-snapshot/` |
| 2.9 | CANDIDATE | `tst3a-out/` |
| 2.9 | CANDIDATE | `rg-rest/` |
| 2.6 | CANDIDATE | `langs-conc/` |
| 2.4 | CANDIDATE | `bzl-tail/` |
| 2.3 | CANDIDATE | `env-pass2-run2/` |
| 2.3 | CANDIDATE | `env-pass2-run3/` |
| 2.3 | CANDIDATE | `loader-own/` |
| 2.2 | CANDIDATE | `graph-receipt/` |
| 2.2 | CANDIDATE | `env-pass2-f1/` |
| 2.2 | CANDIDATE | `demo-libs/` |
| 2.2 | CANDIDATE | `tax4-inv-final/` |
| 2.1 | CANDIDATE | `slice-play/` |
| 2.1 | CANDIDATE | `tst4-out/` |
| 2.0 | CANDIDATE | `env-pass2-f2/` |
| 1.9 | CANDIDATE | `env-pass2-run/` |
| 1.9 | CANDIDATE | `chain22/` |
| 1.7 | CANDIDATE | `collect2/` |
| 1.7 | CANDIDATE | `course-build2/` |
| 1.6 | CANDIDATE | `chain18/` |
| 1.5 | CANDIDATE | `aud-tax-pkg/` |
| 1.5 | CANDIDATE | `fem-finish/` |
| 1.5 | CANDIDATE | `law-drift-run/` |
| 1.4 | CANDIDATE | `rgplug-x/` |
| 1.4 | CANDIDATE | `chain19/` |
| 1.4 | CANDIDATE | `langs-pin-1/` |
| 1.3 | CANDIDATE | `chain19-fail2/` |
| 1.3 | CANDIDATE | `policy-admit-overlay/` |
| 1.2 | CANDIDATE | `tfd/` |
| 1.2 | CANDIDATE | `collect1/` |
| 1.1 | CANDIDATE | `parity/` |
| 1.1 | CANDIDATE | `buildlib-reds/` |
| 1.1 | CANDIDATE | `integrate-r7-stack/` |
| 1.1 | CANDIDATE | `cargo-home/` |
| 1.1 | CANDIDATE | `dep-receipt/` |
| 1.0 | CANDIDATE | `env-pass2-art/` |
| 1.0 | CANDIDATE | `ep2-overlay/` |
| 1.0 | CANDIDATE | `exports-stage/` |
| 0.9 | CANDIDATE | `cli-smoke-1022/` |
| 0.9 | CANDIDATE | `env-pass2-g1/` |
| 0.9 | CANDIDATE | `chain11/` |
| 0.8 | CANDIDATE | `polyglot-units/` |
| 0.8 | CANDIDATE | `audit-verify-1020b/` |
| 0.7 | CANDIDATE | `chain19-fail1/` |
| 0.7 | CANDIDATE | `chain14/` |
| 0.7 | CANDIDATE | `collect3/` |
| 0.7 | CANDIDATE | `chain20/` |
| 0.6 | CANDIDATE | `parity-apply/` |
| 0.6 | CANDIDATE | `chain13/` |
| 0.5 | CANDIDATE | `chain16/` |
| 0.5 | CANDIDATE | `cargo-fix-x/` |
| 0.5 | CANDIDATE | `boot-rebind2/` |
| 0.5 | CANDIDATE | `tax5-inv-base/` |
| 0.5 | CANDIDATE | `stale-impl-guard/` |
| 0.4 | CANDIDATE | `graph-assembly/` |
| 0.4 | CANDIDATE | `integrate-r7-lawcov-laws/` |
| 0.4 | CANDIDATE | `integrate-r7-lawcov-laws2/` |
| 0.4 | CANDIDATE | `chain20-fail1/` |
| 0.4 | CANDIDATE | `tax5-inv-after/` |
| 0.3 | CANDIDATE | `qe2e/` |
| 0.3 | CANDIDATE | `collect4/` |
| 0.3 | CANDIDATE | `env-pass2-r4/` |
| 0.3 | CANDIDATE | `chain21/` |
| 0.3 | CANDIDATE | `rg-fem3d/` |
| 0.3 | CANDIDATE | `bzl-paths/` |
| 0.3 | CANDIDATE | `integrate-r7-lawcov-cli2/` |
| 0.2 | CANDIDATE | `conc-tests/` |
| 0.2 | CANDIDATE | `bzl-hermetic/` |
| 0.2 | CANDIDATE | `gran-value-apply/` |
| 0.2 | CANDIDATE | `pg-chain/` |
| 0.2 | CANDIDATE | `boot-rebind/` |
| 0.2 | CANDIDATE | `legacy-xplat-fixture-rename/` |
| 0.2 | CANDIDATE | `integrate-r7-lawcov-cli/` |
| 0.2 | CANDIDATE | `split-fw/` |
| 0.2 | CANDIDATE | `chain16-fail1/` |
| 0.2 | CANDIDATE | `integrate-r7-lawcov-cli4/` |
| 0.2 | CANDIDATE | `cap-gc/` |
| 0.2 | CANDIDATE | `inputs-closure-artifacts/` |
| 0.1 | CANDIDATE | `osplay/` |
| 0.1 | CANDIDATE | `cachefix-ov/` |
| 0.1 | CANDIDATE | `env-pass2-w3/` |
| 0.1 | CANDIDATE | `env-pass2-r6/` |
| 0.1 | CANDIDATE | `course-bzl/` |
| 0.1 | CANDIDATE | `rust-image/` |
| 0.1 | CANDIDATE | `aud-verify-1019h/` |
| 0.1 | CANDIDATE | `env-pass2-r5/` |
| 0.1 | CANDIDATE | `course-img/` |
| 0.0 | CANDIDATE | `triage-r4/` |
| 0.0 | CANDIDATE | `cli-zt/` |
| 0.0 | CANDIDATE | `aud-nct-tools/` |
| 0.0 | CANDIDATE | `integrate-r7-lawcov-cli3/` |
| 0.0 | CANDIDATE | `env-pass2-tgt/` |
| 0.0 | CANDIDATE | `chain18-gated/` |
| 0.0 | CANDIDATE | `rs-sweep-fx/` |
| 0.0 | CANDIDATE | `legacy-xplat-tests/` |
| 0.0 | CANDIDATE | `graph-parallel2/` |
| 0.0 | CANDIDATE | `composition-group-rust-isolation/` |
| 0.0 | CANDIDATE | `chain12/` |
| 0.0 | CANDIDATE | `vcs-bound-scan/` |
| 0.0 | CANDIDATE | `tst4-scratch/` |
| 0.0 | CANDIDATE | `lawcov-tests-probe/` |
| 0.0 | CANDIDATE | `env-pass2-misc/` |
| 0.0 | CANDIDATE | `bzl-stage-artifacts/` |
| 0.0 | CANDIDATE | `integrate-r7-i18n/` |
| 0.0 | KEEP (live or touched after 19:00) | `demo-verify/` |
| 0.0 | CANDIDATE | `chain15-gated/` |
| 0.0 | CANDIDATE | `aud-verify-1019e/` |
| 0.0 | CANDIDATE | `vitest-course-react-cache/` |
| 0.0 | CANDIDATE | `verify3-art/` |
| 0.0 | KEEP (live or touched after 19:00) | `ts-verify/` |
| 0.0 | KEEP (live or touched after 19:00) | `rust-reds-b-logs/` |
| 0.0 | KEEP (live or touched after 19:00) | `rust-reds-a-logs/` |
| 0.0 | CANDIDATE | `orphan-dev/` |
| 0.0 | CANDIDATE | `lawcov-tests-verify/` |
| 0.0 | CANDIDATE | `chain24/` |
| 0.0 | CANDIDATE | `xplat-boot/` |
| 0.0 | CANDIDATE | `vitest-course-cache/` |
| 0.0 | CANDIDATE | `rg-refused-logs/` |
| 0.0 | CANDIDATE | `pg-chain-artifacts/` |
| 0.0 | CANDIDATE | `env-pass/` |
| 0.0 | CANDIDATE | `t8/` |
| 0.0 | CANDIDATE | `pin/` |
| 0.0 | CANDIDATE | `nx/` |
| 0.0 | CANDIDATE | `l/` |
| 0.0 | CANDIDATE | `image-inputs/` |
| 0.0 | CANDIDATE | `docs-refresh-artifacts/` |
| 0.0 | CANDIDATE | `demo-bzl-run/` |
| 0.0 | CANDIDATE | `b/` |

## 3. Staged source copies under `*-tools/staged*` (forbidden in the ticket)

- 167 directories, 4966 files, 129.8 MiB (apparent size).
- 4,780 of 4,966 files (96%) are byte-identical to a tracked git blob (`git hash-object` against `git ls-files -s`). These are plain copies of checked-in source, which AGENTS.md forbids. The other 186 are edited or untracked copies, also forbidden.
- Largest entries:
  - `./gran-hubs-tools/staged`: 62.2 MiB
  - `./round6-tools/staged`: 11.3 MiB
  - `./round4-tools/staged`: 4.4 MiB
  - `./iface-wrap-tools/staged`: 3.4 MiB
  - `./ide-free-tools/staged`: 3.2 MiB
  - `./tool-iface-tools/staged`: 3.0 MiB
  - `./r6-tool-iface-tools/staged`: 3.0 MiB
  - `./pm-seam-tools/staged2`: 3.0 MiB
- Delete candidates at close: all 167 entries (list in `🗑️generated/close-prep-staged-entries.txt`). They sit under the `-tools` folders, so keep the scripts beside them and remove only `staged*`.

## 4. Draft completion summary (course, play, demo)

Course (`@teaching/architecture-course`)
- GREEN on Bazel (`result-course-rest2.md`): `:check`, `:typecheck`, `:test`, `@teaching/proctor:test`, `:build` and `repo:test-compiler-headroom` all rc 0.
- Vitest quick run 695/695 (`result-course-rest.md`, 16:05).
- OPEN: E2E `e2e-1022-course3` started 20:03:50 (pid 6342). Cold `proctor-build` was RUNNING at 20:13. No warm or E2E verdict yet.
- `e2e-1022-course2` (15:46, FAIL typecheck) is superseded by the course-rest2 fixes.
Play (Semio Tech Play, :6033)
- RED: activation 200 of 201 Bazel actions green (coordinator, 20:07). Only `os-plugin-describe-rs:build` fails, with SQLITE_CANTOPEN in the lease layer while the leases volume had 21 MiB free (`result-slice-play3.md`, 20:06). Loop retries.
- Not reached: serve on :6033, boot check, E2E. No `e2e-1022` play report exists.
Demo (mit-bestand demonstrator)
- Print steps GREEN on Bazel (`result-slice-demo.md`, 16:37): Kompaktbericht PDF, the three reports, and the Projektetage deck. Activate dry-run rc 0.
- RED: real activation hit ENOSPC, then the rust image was grown to 48 GiB (17:38). Attempt act3 failed on the describe-rs compile budget (fixed 19:21). act4 (pid 58110) is no longer running and has no recorded outcome (`result-slice-demo2.md`).
- demo-verify (20:08): demonstrator `test` via nx RUNNING (pid 10738). Bericht and Projektetage not yet run.
- Not reached: serve, E2E. No `e2e-1022` demo report exists.
Caveats
- `result-course-rest2.md` has a heading at 20:20, later than its file mtime of 20:03. Check the clock or the CEST label before quoting times.

## Open items for the closer
- Decide the owner check for the three `course-rest-*-framework-cache` folders (16.2 GiB) and the ambiguous or unresolved refs before `ticket_close`.
- Do not run `ticket_close` while course3, play loop and demo-verify are still running. They write into this folder.
