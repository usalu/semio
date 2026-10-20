# Guided Workspace File Management

## Read-only evidence

The native Source pane has a concrete guided-action gap. Dashboard terminal sources Request (sources/🦀️.rs:15) contains only List, Load, Recovery and Save. Controls Action (controls/🦀️.rs:25,432–439) has show/save/reload/discard/cancel source actions and no create, rename or remove action. Existing workspace Rust public commands are new, list, load, load_recovery and save; internal create/remove/rename functions support event-store publication rather than public workspace commands.

An exact authored-owner roster pass read 1,470 JSON project manifests. No string declaration matched create/new/rename/remove/delete file, folder or directory (including mkdir/mv/rm commands). The roster also contains 2,263 Cargo manifests, which are not JSON project control declarations; these were excluded from that control result rather than treated as broken JSON. Dashboard registry built-ins cover task/project tools plus tickets/drafts/goals/analyze/tree/statutes; draft start/discard are repository notes, not arbitrary workspace file management. The catalog provider is product playground publication, not a Source pane command port. This is source evidence, not a complete human runtime catalog pass.

Evidence owners:

- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⌨️controls/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🧶️build-and-derive/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🦀️.rs
- /Users/ueli/Documents/semio/📋️build-owners.json

## Proposed bounded domain extension — not implemented

Keep the existing filesystem Workspace owner and extend its event commands, with the terminal as a first-party client. Schema/neutral vectors live under filesystem/workspace in a management subdomain; no shell or external process delegation and no new runtime dependency. No new crate/manifest edge is needed.

A closed command contract distinguishes CreateFile, CreateDirectory, RenameEntry and RetainEntry. CreateFile supplies bounded UTF8 content and an expected-absent destination. RenameEntry supplies an opaque captured entry revision plus expected-absent destination; replacement is never implicit. RetainEntry is the confirmed user remove action: move the selected entry into authenticated local event history, retaining the actual predecessor rather than recursively unlinking unobserved foreign data. Return a typed outcome with command id, resulting path/entry or retained recovery id, and committed/partial/conflict semantics. A public inspect-entry operation issues the opaque revision used by forms; it must not accept a user-fabricated metadata shape.

Use the current descriptor-anchored root, nofollow ancestor rules, UTF8/path256 limits, writer lock, bounded history, cancellation checkpoints and progress channel. Add native no-replace create/move/retain ports for macOS/Linux/Windows under the existing native module. Durable intent precedes publication, outcomes follow commit; cancellation after commit reports the actual committed outcome. Unknown errors/partial moves keep the recoverable identity and do not pretend no change. Noncooperating external writers remain a documented conflict/race boundary: retain actual raced bytes and surface recovery, never promise global filesystem CAS. Directory retention/rename needs an explicit schema-authored policy for subtree observation and retained recovery before implementation; do not silently apply the text-file1MiB limit to a whole source tree or recursively delete it.

## Guided UI

Add customizable semantic actions in Files: New File, New Folder, Rename, Remove. Localized EN/DE forms show current directory and destination, reject existing or invalid names inline, allow keyboard cancel/back, and never require a raw command. The create-file result opens a normal editable document. Rename/remove of an open dirty document must first resolve retain/save/discard; successful outcomes reconcile open document paths and selected list entries. Remove confirmation starts unselected and names the exact entry/type and retained-history behavior; a second explicit confirmation dispatches the command. The same one-worker/latest-pending and per-poll32 bound remain. Operation tokens reject stale results, while committed outcomes survive late cancellation.

## Tests before production

- Language-neutral cases: Unicode names; empty/occupied destinations; root/traversal/reserved journal/symlink rejection; path256 boundary; cancel before commit versus committed late cancel; stale expected revision; rename/remove conflict; every durable intent cut; unknown residue; directory policy limits.
- Independent SQL state-transition oracle and Node filesystem reference for isolated create/nonreplace rename/retained-remove final bytes and roster. Node is a test oracle behind its port, not a runtime dependency.
- Real Rust owner filesystem laws on each supported adapter plus actual controller form/dirty-buffer/late-result laws.
- Installed PTY: create file, edit/save/reopen, create folder, rename through form, confirm/cancel remove, preserve conflicting foreign changes and inspect retained recovery. Explicit counts and whole-command terminals required.

No runtime producer was started and no dashboard/SDK/activation production source was modified by this audit. SDK/pointer freeze for Root53440 remains intact.

## Root Backend Test-First Slice

Root owns the physical native mutation backend. Nine actual filesystem laws were authored before implementation: create/no-overwrite/rename/retained recovery and independent serde output; empty-directory retention followed by later usable commands; nonempty refusal; stale/foreign revision; occupied-destination race; replaced-source postpublication recovery; new-child postpublication recovery; cancellation before effect; committed creation after late cancellation. Shared prior artifact-root harness is reused with test-only visibility. APIs remain absent until the red compilation/body gate and implementation. No runtime pass is claimed.

## Authored implementation in progress

Root owns the native backend and central Rust EntryRevision/RetainedEntry/NotEmpty declarations plus durable native ports. This agent owns the management schema/corpus (13 cases), second TS implementation and Ajv/SQLite law mounted in the existing Source workspace suite, and Source-pane controls/forms/localization/keymaps. Two genuine actual-controller laws were authored before wiring. UI confirmation requires the exact selected name with empty initial input. Mutating worker paths block edits and close until actual settlement, including cancellation. The save policy is now explicitly named saveExistingFilesOnly; it does not prohibit the distinct create commands. No new runtime dependencies or crate manifests. None of these new laws has run yet.

## Integrated UI safety review follow-ups

The optional postcommit rename reload now uses the same request cancellation and progress sink. Failure or late cancellation keeps the authoritative Renamed outcome and a read-only moved buffer; ordinary relative reload can regain a normal revision, while authenticated absolute history documents keep Recovery routing. Live worker protected paths remain after form cancellation and block edit/close until Finished. A third actual controller law injects a committed rename after cancellation and proves old/new path protection plus document reconciliation. The terminal selector now expects seven controller laws; installed journeys expects33 unignored laws. The new real PTY law uses exact-name removal confirmation, verifies disk bytes, retained recovery, and empty-folder create/cancel. These are authored gates, not observed runtime passes.

## Source ownership and extent estimate

Moved two purely static Rust audits into the existing advertised dispatch Source suite with a closed neutral36-scenario mapping, exact actual Rust test annotation checks, SQLite and silent-source token checks. All behavioral Rust laws remain; the stale p1_7 mapping now uses its actual unchanged-screen law name. Removed7,454 compiler Source bytes from static-only audits. Shared language JSON is test DATA behind the existing first-party fixture reader, with the same full corpus, an explicit language target freshness input and actual manifest-relative binding oracle in dispatch. No code is read as DATA, no case removed, no cap raised. Estimated terminal extent is261,989bytes including remaining literal JSON; actual Source admission remains the authority and has not run.

## Backend Native Publication Ports

Root added opaque EntryRevision, RetainedEntry and NotEmpty declarations, then anchored exclusive move/create-empty/remove-empty/parent-sync ports; public command behavior is not implemented or runtime-qualified yet. Local macOS SDK sys/stdio.h confirms RENAME_EXCL0x4 and RENAME_SWAP0x2. Linux uses renameat2 no-replace1. Windows uses a DELETE/attribute handle with no-follow ancestor pins, write-through and FileRenameInfo, a zero replacement flag and bounded aligned UTF16 buffer. [Microsoft FILE_RENAME_INFO](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_rename_info) specifies that disabled replacement refuses occupied targets; [CreateFile](https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-createfilea) describes write-through metadata flushing on NTFS. Applying that flag to the handle rename is a source design inference, requiring real Windows qualification. Unix kernel rename and following parent sync are separated so a sync error cannot hide an already committed move. No claim of atomic expected-source revision or empty-at-commit condition is made.

Cancellation/disconnection retirement now unlocks a still-visible command form after the worker settles, so prefix cancellation does not leave an inert form. The existing precommit error branch shares the same unlock helper. Newly authored controller tests share keyboard submission steps; no assertion or behavior law was removed.

A fresh read-only metadata check found no duplicate compile-catalogue label field identifiers and no duplicate same-scope default key bindings after adding the four guided actions. This is static inspection, not a native compilation or test-body pass.

## Genuine Compile Red And Backend Implementation

Root request60431 settled whole101 with29 E0599 errors for the absent create/inspect/rename/retain APIs; no test body ran and cancellation was not the primary failure. The private management module now implements those APIs with bounded exclusive durable intent/outcome records, anchored no-replace publication, pre-observed revisions and post-move exact bounded file bytes plus native identity. Retained directories require matching intent/outcome and actual native identity before future writer admission. The raced-new-child test now also requires a later create to remain usable. Folder creation uses the atomic mkdir result separately from subsequent open/sync, so a later error reports an actual committed destination. Runtime qualification remains pending.

## Persistent Namespace And Shared Native Cases

Independent review caught raw persisted device numbers. Outcomes now publish the root/journal/entry namespace chain, and recovery requires identical inode chain with one shared published device and one shared fresh device before rebasing the device coordinate. No receipt is rewritten. A closed12-row canonical-u64-text neutral corpus, TS implementation and exact-TEXT SQLite oracle are mounted in the existing Source workspace suite; Node DATA/Ajv/SQLite inspection and TS parse-only pass are not Source runtime qualification. Rust adds the same12-row classifier law and drives all13 shared management cases through actual physical commands. A preserved-mtime same-size content race law checks exact pre/post bytes. Total new native management laws12, whole existing workspace library expected22; actual compiler/body proof remains pending.

## Content/Tree/History Source Review

Low accepted the current bounded source design pending runtime: one physical nofollow namespace;4096totalnode sortedroster; preflight all observed projected physicalpaths≤256; exact standalone content SHA256 up to64MiB; ancestor/root/journal namespace continuity; authenticated retained descendants with final reauthentication. Rootrenamectime is normalized only in tree fingerprints, while descendant stamps remain bound. This is cooperative/observed-tree custody, with actual raced outcomes retained. A post-effect I/O failure can prevent outcome persistence: recovery_path communicates actual location and is not by itself authenticated history. Uncertain affected buffers must stay readonly until explicit inspection/reload. History crash-cut reconciliation and bounded retention maintenance require further explicit review before claiming universal zero-touch lifetime usability.
