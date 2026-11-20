# Original typed action receipt handoff

The second flag action never reached command dispatch. In the first completed command the handoff used the source's resident construction admission to move the result, then returned while that admission was still live. The command lease had been released and the next action waited for a pending source it could never drive. A native debugger sample resolved the last true pending predicate to OriginalTypedCommandSource::pending.

The typed source now closes its resident construction under the issuing caller across funded retirement turns and transfers the completed result with a paid forward move. The final transfer creates no new constructor. The existing two-action, undo, relocate, lock and deletion scenarios exercise this path.

The stable verification checkout includes the matching PeerRoster accessor and its store dependency.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️typed/🦀️.rs`

The new boot capacity test exposed a separate test compilation failure. The public command parser returns an option. Three tests now use that API and the inaccessible test-only parser has been removed.

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧪️every-example/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🗂️manifest-and-window-actions/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/☑️option-groups-and-edits/🦀️.rs`
