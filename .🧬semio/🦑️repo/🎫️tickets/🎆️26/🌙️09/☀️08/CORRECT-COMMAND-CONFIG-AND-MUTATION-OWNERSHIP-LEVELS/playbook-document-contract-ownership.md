# Playbook Document Contract Ownership

The native artifact/snapshot contract owns schema, id, version, required nullable title, document and flow child identities. Its sparse diff additionally permits an artifact replacement. Non-Rust facets still declare inline steps or generic value wrappers and duplicate framework child/address types. The native codec explicitly requires title, unlike Forms which normalizes an omitted/null optional title to absence.

A shared schema testkit now validates Forms and Playbook document facets against independent Ajv validation and committed native fixture inputs. Domain-specific vectors remain beside each document owner. Playbook correction and validation are in progress.

## Validation

`playbook-document-contract-red-1.log` passed Forms through the new shared testkit and reproduced Playbook's stale inline-step requirement, rejected actual child slots, and invalid non-null-only title definition. All twelve Playbook artifact/snapshot/diff projections now use native fields and shared child identities. Unused local helper/address declarations were removed from those facets; document mutation vocabulary remains in its own owner.

The first attempted edit failed before writing any files because a shell transfer contained invalid UTF-8; its subsequent green-named run still failed on unchanged schemas. The corrected edit and `playbook-document-contract-green-2.log` passed both Forms and Playbook checks through Nx in 1.4 seconds. Playbook matches eighteen committed native snapshots and five committed diffs; its parser/schema also reject missing required title, malformed child references, unknown document fields and invalid artifact replacement. These are production TypeScript parser and Ajv checks, not a new native mutation or IDL compiler run.

The shared testkit lives at framework Schema's `🧪️testkit/🪪️document-contract`; the executable Playbook command is registered in root script, Nx and both launch catalogs.
