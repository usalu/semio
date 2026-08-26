# Ticket Important Live Census

## Scope

This is a no-follow owner census of physical `📌️important.md` leaves beneath the repository ticket root on 2026-08-26. It reads only each leaf's byte count and an exact sibling `🎫️ticket.json` when present. It did not traverse or read any Compose tree.

## Exact Result

| Manifest state | Content state | Count | Frozen disposition |
| --- | --- | ---: | --- |
| `open` | zero-byte | 161 | Project to `📌️important/📝️.md` |
| `open` | nonzero | 22 | Project to `📌️important/📝️.md` |
| `closed` | zero-byte | 243 | Remove through owner-manifest authority |
| `closed` | nonzero | 21 | Block; content must not be discarded or status inferred |
| invalid status | nonzero | 1 | Block; status must be explicitly repaired |
| missing sibling manifest | zero-byte | 5 | Unclaimed ticket evidence; do not infer an owner |
| missing sibling manifest | nonzero | 7 | Unclaimed ticket evidence; do not infer an owner |

The exact total is 460. The directly actionable owner-governed population is 426: 183 projections plus 243 authoritative zero-byte removals. The other 34 remain deliberately fail-closed until an explicit ticket-owner or retention-manifest decision exists.

The one invalid manifest is:

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️05/FIX-ENGAGEMENT-SUGGESTION-CLICK/🎫️ticket.json
```

It contains no `status` key and its 517-byte sibling important document must not be treated as closed or open by inference.

One of the missing-manifest paths is a nested window-policy fixture rather than a ticket owner. The remaining missing-manifest paths are structurally incomplete historical ticket evidence. Their classification proves why basename-only projection would be unsafe.

## Reproduction

```text
find '.🧬semio/🦑️repo/🎫️tickets' -name '📌️important.md' -type f -print0 |
while IFS= read -r -d '' file_path; do
  owner_dir=${file_path%/*}
  manifest_path="$owner_dir/🎫️ticket.json"
  byte_count=$(wc -c < "$file_path" | tr -d ' ')
  if [[ -f "$manifest_path" ]]; then
    ticket_state=$(jq -r 'if (.status|type)=="string" then .status else "<invalid>" end' "$manifest_path" 2>/dev/null || printf '<malformed>')
  else
    ticket_state='<missing-manifest>'
  fi
  if [[ "$byte_count" = 0 ]]; then content_state=zero; else content_state=nonzero; fi
  printf '%s\t%s\n' "$ticket_state" "$content_state"
done | LC_ALL=C sort | uniq -c
```

Observed canonical buckets:

```text
1 <invalid> nonzero
7 <missing-manifest> nonzero
5 <missing-manifest> zero
21 closed nonzero
243 closed zero
22 open nonzero
161 open zero
```

The first attempt used `path` and `status` as zsh loop variables. Those names are special/read-only in zsh and the command failed before producing evidence. The corrected command above uses task-specific variable names.

## Acceptance Impact

- Inventory must invoke `semanticOwnedFileProjectionAuthority` only for an exact ticket-slug owner, exact sibling ticket manifest, exact Markdown source kind, and exact source basename.
- The projection-only `ticket-important` directory kind must never make an arbitrary raw directory canonical by generic stem inference.
- Closed-zero removal needs a typed owner/manifest/status authority whose manifest and source preimages are retained and revalidated before mutation and rollback.
- Closed-nonzero, invalid, and unclaimed leaves remain errors; normalization must not silently coerce status or destroy content.
- Embedded ticket-root relocation must be resolved before this owner projection so a nested manifest cannot counterfeit an immediate owner.
