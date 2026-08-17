# Terra Manifest OS Kernel Consolidated Head Revalidation

## Scope

Validation only. No implementation source, including the externally modified OS Store source, was changed.

## Head and Manifest Containment

- Revalidated HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`.
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` SHA-256: `388f8dfb960608361ea0534c3b295af5ec1d570b2b919251b9561f379dd69e0d` — exact expected value.
- The manifest has no ordinary or cached diff and no `git status` entry.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` remained externally modified and was not edited by this validation.

## Registered Gate Results

| Command | Exit | Result |
| --- | --- | --- |
| `bun nx run @semio-tech/framework:test --skip-nx-cache` | 0 | Passed: 2 test files, 150 tests. |
| `bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache` | 0 | Passed. Cargo reported 9 warnings, and Nx reported the task as flaky. |
| `bun nx run @semio-tech/framework-os-kernel:test-quick --skip-nx-cache` | 1 | Executed because `check` passed. Cargo test exit 101: 934 tests run, 932 passed, 2 failed. |

## OS Kernel Test Failures

The two failed tests are both in `os_store::component::tests`:

- `space_undo_and_redo_target_the_member_with_the_most_recent_local_edit_by_hlt`
- `spr_round_trip_preserves_edit_messages_and_conflicts`

## Failure Classification

- Manifest: no failure; the exact expected source hash and clean containment were confirmed.
- UI: no UI target was run in this validation, so this record does not attribute any UI failure to the OS-kernel result.
- OS kernel: the only failing gate was `test-quick`, with the two `os_store` tests above. No retry or source change was performed.
