# Lane paneB result (energie pane smoke)

- Marker `lock-sync-1107-ok` already existed (empty, 11-07); go102.sh only gates on it. Not created.
- t201: red in instance-dispatch (gen3dfinal2, since fixed W-gen3dfinal2-3).
- t202 (rc 1): cargo source admission refused `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🌱️initialization/🦀️.rs`: dangling `#[cfg(test)]` as the last line of the file owned no item. EDIT (paneB, unclaimed energy crate): removed that trailing attribute (the `#[cfg(test)]#[path=...]mod tests;` line above it stays).
- t203 (`SLOT_HANG_SECS=100 go102.sh 203`): running; stuck >40 min in door.capacity (host swapping, 4-5 ahead). Verdict pending: Monitor on `W-energie-t203.rc`.
- Rerun: `SLOT_HANG_SECS=100 C/1105b-tools/energie/go102.sh <n>`.

- t203 (rc 1, after ~55 min in the capacity queue): source admission refused `.../🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`, same dangling trailing `#[cfg(test)]` (also present in HEAD). EDIT (paneB): removed it. A scan of all energy `.rs` files found no other file ending in a bare cfg attribute.
- t204: queued ~1.5 h at door.capacity, CANCELLED on coordinator order (blocks closeshared snapshot-projection fix); children and wrapper TERMed, rc 130. Claim released.
- t205 (rc 101, ~1.5 h of queueing): `semio-plugin-instance-typed-ops` E0433 x4: `staging/🪆️child-group/🦀️.rs` used stale ident `semio_framework_plugin_retained_command` (crate is `semio-plugin-retained-command`; the same file already used the new ident once). EDIT (paneB, claimed in log; paneA had released typed-ops): replaced the stale ident, 0 left in file.
- t206: `SLOT_HANG_SECS=100 go102.sh 206` started; Monitor on `W-energie-t206.rc`. Slots/capacity queues are long (about 1 h each); no test verdict yet.

STATUS: PENDING W-energie-t206 queued; en: no verdict, de: no verdict
