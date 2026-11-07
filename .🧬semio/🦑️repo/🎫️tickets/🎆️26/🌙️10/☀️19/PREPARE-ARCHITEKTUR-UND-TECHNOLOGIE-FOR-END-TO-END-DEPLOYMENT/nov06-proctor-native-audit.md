# Native Proctor Source Audit

This bounded source audit is not a Windows compiler or runtime proof. It read the proctor Cargo manifest, declared entry, local production Rust modules and manifests, and ten TypeScript files under the proctor bootstrap and course stack/e2e launch paths. It did not execute commands, tests, jobs or toolchain setup. Transitive framework and third-party implementations were not exhaustively audited.

## Actionable finding

The standalone proctor launcher cannot reliably stop its owned child on Windows through its documented signal-forwarding path. In `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts:480`, `interrupt` explicitly does nothing on win32. In `🎓️teaching/🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts:529`, `runProctor` handles a signal by aborting acquisition and calling only that no-op; after launch it awaits `proctor.exited` at line 537 and does not call `stop`. A signal received after launch therefore does not itself terminate the child; whether the child separately receives a console event depends on its execution context. Repair and test the owned Windows cancellation path before claiming standalone dev cancellation. The course stack differs: its signal aborts the stack and its cleanup calls the supervised proctor stop path (`🎓️teaching/🏛️architecture/📚️course/🧱️stack/🟦️.ts:412`, :447 and :295), which invokes owned-tree termination. Its actual behavior remains unverified.

## Other observations

The manifest declares the Rust entry at `🎓️teaching/🛂️proctor/🏗️bootstrap/🦀️.rs` and local base/domain crates. The base crate declares bundled rusqlite, so building SQLite requires the actual native C compiler provider; this declaration is not evidence the provider is ready. The proctor termination implementation gates Unix signals with `#[cfg(unix)]` and provides Windows console signal handling at `🎓️teaching/🛂️proctor/🔨️modules/⌨️cli/🦀️.rs:668`. The bootstrap selects `proctor.exe` and skips Unix chmod on Windows (`🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts:280`, :412). Course stack and proctor launch use argument-vector spawn rather than a shell in the inspected paths.

No unguarded Unix API, Unix syscall, Bash/sh invocation, or Unix-only executable assumption was found in the inspected local production Rust files and these launch paths. This limited search does not establish portability of transitive framework code, dependencies, compilation, signal delivery, executable custody, or browser execution. The separate intentional Linux image/deploy guards remain untouched and unverified on Windows.
