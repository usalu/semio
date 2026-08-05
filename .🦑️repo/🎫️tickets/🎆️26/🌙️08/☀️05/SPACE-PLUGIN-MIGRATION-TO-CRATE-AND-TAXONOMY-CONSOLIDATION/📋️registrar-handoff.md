# Registrar handoff — 🪐️space

**⚠️ NOT YET READY for the standard registrar pass.** Unlike every prior migration in this initiative,
this one could not be proven green in-session (native `cc` linking is blocked repo-wide by an
unaccepted Xcode license in this sandbox — see `🧪️verification-attempt.txt` in this ticket folder for
the full diagnostic transcript). **A follow-up session with a working native Rust toolchain must run
the TEMPLATE.md §9 verification sequence and fix whatever it finds before any of the steps below are
safe to apply.** The old 16 crates were deliberately NOT deleted for exactly this reason.

Once verified green, the registrar's steps are the standard ones:

```
Remove these member lines from root Cargo.toml:
    "✏️s/🔌️plugins/🪐️space/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🔨️modules/🔧️op/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🔨️modules/🖱️ui/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🔨️modules/🔧️op/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🔨️modules/🖱️ui/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🪐️space/🔨️modules/🤝️shared/⚡️implementations/🦀️rust",
Add:
    "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust",
Also remove from [workspace.dependencies]:
    semio-s-app-space-home = { path = "✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/⚡️implementations/🦀️rust" }
    (the other 15 old crates were never workspace.dependencies entries, only member lines)

Optional same-pass cleanup (safe, not required): switch `semio-framework-os`/`semio-framework-plugin`/
`semio-framework-core` in the new Cargo.toml's `[dependencies]` from plain `path =` to
`{ workspace = true }` — their keys match root `[workspace.dependencies]` exactly; they were kept as
plain paths only because the isolated verification overlay's own minimal `[workspace.dependencies]`
table can't carry their full graphs (see TEMPLATE.md §3's chicken-and-egg note).

Cross-cutting files edited: NONE — found one real dependent (below) but deliberately left it pointed
at the OLD (still-working) crate rather than repoint it onto an unverified new crate; see rationale.
Cross-plugin dependents found: ONE.
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust/Cargo.toml:129`
dev-depends on the OLD `home` entity crate:
    home = { path = "...✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/⚡️implementations/🦀️rust", package = "semio-s-app-space-home" }
and its `📦️lib.rs:61` does `use home::SHomeDocument;`. **Not fixed in this pass** — deliberately, since
fixing it would repoint a currently-working build onto this session's unverified new crate (see the
"NOT YET READY" note above); breaking a working cross-cutting dependent on top of an unverified new
crate is worse than leaving it alone. When the new crate is verified green and the old 16 crates are
about to be deleted, repoint this ONE line + `use` statement:
    dep line  → `space = { path = "…/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust", package = "semio-s-plugin-space" }`
    use line  → `use space::artifacts::home::SHomeDocument;`
`dsl/📇️registry` and a repo-wide grep for every one of the 16 old crates' package names (Cargo.toml
files) and for `use home_ui::`/`use space_ui::`/`use home_engine::`/etc.-style imports (`.rs` files)
turned up nothing else outside this plugin's own directory. `🎪️demonstrator` does not depend on `s`.
Still un-run (blocked on the native-toolchain verification above landing first):
    cargo check --manifest-path ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml
    cargo check --manifest-path … --target wasm32-wasip2
    cargo clippy --manifest-path … --all-targets -- -D warnings
    cargo test --manifest-path …
    (then, only after all of the above are green AND the old 16 crates are deleted AND the temporary
    [workspace] overlay + cargo-features line are removed from the new Cargo.toml)
    cargo check --workspace
    bun nx run @semio-tech/framework-os-dev:plugin -- space
    bun ./📜️script.ts dev space
    bun ./📜️script.ts verify gate
    registry generate
