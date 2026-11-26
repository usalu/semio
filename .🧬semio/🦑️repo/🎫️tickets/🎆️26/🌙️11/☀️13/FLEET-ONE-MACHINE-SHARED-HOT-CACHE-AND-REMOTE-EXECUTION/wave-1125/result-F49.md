# F49: capability-region generator

Generator: `prepareCargoCapabilityLinksV1` in `📚️library/🗂️workspaces/🦀️cargo/🧩️capabilities/🟦️.ts` (invoked by the cargo preparation of owner manifests). It rewrote both `🧩️ Capability Features` and `🧩️ Capability Dependencies` regions from `links.json` with a plain `writeFileSync`, never looking at keys outside the region. Whoever ran it (resident preparation, not F24/F46 scripts; F46 has no result file) added `semio-s-artifact-stdio-step-snapshot` inside the region while the key already stood below it -> duplicate key.

Fix (landed, lock `workspaces`):
- `dedupeCapabilityLinesV1`: parses the manifest with both regions emptied and skips generated feature/dependency lines whose key already exists outside; outside keys stay where they are.
- `writeCargoManifestAtomicV1`: parses the new text with the TOML parser (rejects duplicate keys), writes temp + rename, removes temp on failure. Membership writer (`🟦️.ts` publishCargoWorkspaceMemberships) got the same parse + temp + rename.
- Tests (`🧩️capabilities/🧪️tests/🟦️.ts`): manifest with `neutral-bravo` already outside the region yields exactly one declaration and a valid TOML; atomic writer refuses a duplicate manifest and leaves the file unchanged (laws 7 -> 9; total 14 passing).
- Gate: `runCargoManifestParseLawV1(root)` (Bun.TOML, independent of our parser) over `git ls-files --cached --others` Cargo.toml: 5470 manifests, 0 failures, ~20 s under load. Wired into `capability-contract-check` (CapabilityContractScript).

Not verified: running the actual `capability-contract-check` script command through the repo CLI (ran the exported functions directly with bun); other tooling writing Cargo.toml (grep of the cargo workspace member found only these two writers; other products not scanned).
