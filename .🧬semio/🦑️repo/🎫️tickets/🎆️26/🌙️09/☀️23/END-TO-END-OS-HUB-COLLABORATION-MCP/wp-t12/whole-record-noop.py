"""🟰️ A whole-record state lane (its diff type IS its snapshot, so applying a diff replaces the record) must answer a
no-op with the unchanged record, not `MutationOutcome::empty()`: the store applies every op's diff
(`replay_mutations`), and the empty outcome's default diff replaced architect's presence/config and imperative's
config with their defaults whenever an identical value was re-applied (measured by the T12 vector harness). The five
leaves now return `MutationOutcome::new(base.clone())` with the same warned `mutation.no-op`. `--dry` reports only."""
import sys
root = "/Users/ueli/Documents/semio/"
A = "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
I = "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/"
import glob, os
targets = glob.glob(root + A + "👥️presence/🧬️schema/🧬️mutations/*/🦀️.rs") + glob.glob(root + A + "🎚️config/🧬️schema/🧬️mutations/*/🦀️.rs") + glob.glob(root + I + "*/🦀️.rs")
dry, done = "--dry" in sys.argv, []
for path in targets:
    text = open(path, encoding="utf-8").read()
    new = text.replace("return protocol::MutationOutcome::empty().warn(\"mutation.no-op\",", "return protocol::MutationOutcome::new(base.clone()).warn(\"mutation.no-op\",")
    if new != text:
        done.append(path[len(root):])
        if not dry: open(path, "w", encoding="utf-8").write(new)
print(("would fix" if dry else "fixed"), len(done))
for d in done: print("  ", d)
