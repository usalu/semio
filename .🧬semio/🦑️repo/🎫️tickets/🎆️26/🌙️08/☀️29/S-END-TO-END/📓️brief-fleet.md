# 🛠️ Fleet brief — stdio mutation-leaf migration

Read `📓️plan-mutation-leaf-migration.md` (same folder) **in full** before touching anything. It is
the recipe. Then read the reference implementation it was derived from and keep it open:

- aggregate: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧬️schema/🧬️mutations/🦀️.rs`
- leaves: that folder's `🔧set-snapshot/` and `🔩set-compression/` (each is exactly `🦀️.rs` + `🔣️.json`)

## What you do, per artifact you own

1. Read the artifact's `🧬️mutations/🦀️.rs` end to end. List its enum variants and their named fields.
2. Create one leaf folder per variant — `<emoji><kebab-kind>/` containing `🦀️.rs` and `🔣️.json`,
   mirroring tiff's two files field for field. Pick a distinct, fitting emoji per leaf; the emoji in
   the folder name and in the descriptor's `emoji` field must match, and carry **no** variation
   selector. If a leaf folder already exists (an earlier sweep created some `📄set-snapshot/`
   folders), keep its `🔣️.json` and any subfolders and only write/replace its `🦀️.rs`.
3. Rewrite the aggregate: `#[path]` mod decls for every leaf, `#[derive(… dsl::Mutations)]` plus
   `#[mutations(snapshot = <Snapshot>, diff = <Diff>, schema = "<EnumName>")]`, tuple variants
   wrapping the leaf payloads, and the old `impl Mutation` block deleted with its `diff`/`inverse`
   bodies lifted **verbatim** into `pub(crate) fn agg_diff` / `pub(crate) fn agg_inverse`. Only each
   match arm's pattern head changes, from `E::V { a, b }` to `E::V(v_mod::V { a, b })`.
4. Drop the `NoMutation` variant and every consequence of it — the `#[derive(Default)]`/`#[default]`
   on the enum, its `KINDS` entry and any `KINDS.len()` assertion, its `kind()` arm, its entry in the
   in-file test `variants()` list, and its arms in the artifact's own
   `🗿️artifacts/<artifact>/🧪️tests/mutate-*/🦀️.rs`. `no` is not an approved verb, so the derive
   cannot accept that variant.
5. Fix every remaining construction site of the enum inside YOUR artifact's folder (struct-literal
   `E::V { .. }` → tuple `E::V(v_mod::V { .. })`). Search with
   `grep -rn '<EnumName>' --include='*.rs' "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<artifact>" | grep -v /target/`.
   Keep each artifact's existing `OpText`/`OpBinary` impls and its `apply_*_mutation` free function.

## Hard rules

- **Do NOT run `cargo`.** Peer sessions hold the target lock and your check would stall for the whole
  session. Verification is centralized by the coordinator, who will send you the compiler errors for
  your own files. Get the edit right by reading, not by compiling.
- Touch only the artifact folders you were assigned. Other agents own the others; the crate will be
  full of their in-progress errors and that is expected.
- Never run `git commit` / `stash` / `checkout` / `restore`. Never use worktrees.
- Do **not** open or close any ticket.
- Where the reference is ambiguous, mirror tiff literally rather than inventing, and note the
  ambiguity in your reply instead of guessing silently.
