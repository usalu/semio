# Terra glTF Artifact I/O Manifest Lease

## Baseline And Scope

- Owner: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}`.
- Applicable repository, `✏️s`, and `🗄️stdio` instructions were read; no deeper glTF instruction file exists.
- The exact 12 owned findings are listed in [Sol glTF Scoped Problem Lease Map](./sol-gltf-scoped-problem-lease-map.md): serializer/deserializer collection manifests, artifact membership, and JSON artifact immediate leaves only.
- `git status --short` found no pre-existing dirty path below either leased serializer or deserializer hierarchy. Concurrent changes elsewhere, including generated glue and `🚪️io/💡️inferences`, are protected and excluded.
- Baseline SHA-256: export RFC 8259 serializer `e890a9a911b860b861b96d3e435ef4edacc6eb544e00ed896103e8d53d2aec5d`; import RFC 8259 deserializer `f97daa1dc16404edede9828894e3e23a8230b240b6d4b1b2469bba0b00028440`.
- Direct generated-glue registrations target only the existing RFC 8259 terminal Rust leaves; generated glue will not be changed by this lease.

## Planned Ownership

- `🧵️serializers` and `🧩️deserializers` remain behavior-free I/O collection roots and each declare the exact `🗿️artifacts` route as an I/O member with a direction-specific format contract.
- Each `🗿️artifacts` directory is an artifact collection that declares its exact `🔣️json` child. The immediate artifact and JSON leaves are mechanical named-region assembly only; existing RFC 8259 terminal behavior remains untouched.

## Applied Structure

- Added the four canonical collection manifests at `📤️export/🧵️serializers`, `📤️export/🧵️serializers/🗿️artifacts`, `📥️import/🧩️deserializers`, and `📥️import/🧩️deserializers/🗿️artifacts`.
- Export declares `s.stdio.gltf.io.export.artifacts` with `s.stdio.gltf.artifact.json` / `export`, then its specific `s.stdio.gltf.io.export.artifact.json` JSON artifact. Import mirrors those exact values with the `import` identity and direction.
- Added only eight immediate `🦀️component.rs` / `🟦️component.ts` leaves: the `🗿️artifacts` and `🔣️json` members in each direction. Every new leaf contains only a named assembly region.
- Did not alter RFC 8259 terminal leaves, the `🚪️io/💡️inferences` codec collection, schema/mutations, root/taxonomy, or generated glue.

## Validation

| Check | Result |
|---|---|
| JSON parse and exact manifest-shape assertion | Passed |
| Direction-specific I/O metadata, direct-child bijection, and immediate-leaf assertion | Passed for all four collection/member boundaries |
| Mechanical-only assembly-leaf assertion | Passed for all eight new Rust/TypeScript leaves |
| Existing RFC 8259 terminal SHA-256 | Passed; both baseline hashes unchanged |
| `bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf` | Completed: 27 components, 64 errors, 0 warnings — exactly 12 fewer errors than the 76 pre-lease result. Displayed remaining findings are root/mutation ownership work. |
| `git diff --check` (tracked scope and index) | Passed; untracked lease files are additionally checked with `git diff --no-index --check` before handoff. |

## Release Result

The exact twelve Artifact I/O Manifest Lease findings are resolved without aliases, source moves, generated edits, or changes outside the leased serializer/deserializer hierarchy. The remaining 64 scoped taxonomy errors are outside this lease.
