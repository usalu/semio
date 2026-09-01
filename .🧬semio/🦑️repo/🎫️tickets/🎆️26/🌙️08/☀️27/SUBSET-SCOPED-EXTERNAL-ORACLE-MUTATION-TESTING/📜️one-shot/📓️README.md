# 📜️ One-shot scripts — preserved record, not re-runnable

These are the handwritten scripts used to perform this ticket's reader-oracle retrofits. They were
recovered out of `🗑️temp/` before that folder was cleared, because CLAUDE.md forbids deleting input
scripts while requiring tool-generated output to go.

**Every one of them is one-shot and non-idempotent.** They patch a subset's `🧪️oracle/🔣️.json` by
appending an oracle, its comparison profiles/pipelines/probes, and its `fixtureManifests`; running one
twice duplicates all of those. Several also read intermediate JSON that lived in `🗑️temp/` and no
longer exists. They are kept as a record of exactly what each retrofit did — not as tooling.

| File | Did |
|---|---|
| `patch-png-oracle-json.py` | registered `png-1-2-mutate-reader`, 15 fixtures, 12/3 witnessable split |
| `patch-jpg-oracle-json.py` | registered the `jpg@jfif-1.01/document` reader |
| `merge-gltf-oracle-json.py` | registered the `gltf@2.0/any` reader |
| `gltf-probe-capabilities.ts`, `gltf-validate-all.ts`, `gltf-reproduce-check.sh` | gltf verification helpers |
| `obj-tobj-probe/` | standalone `tobj` crate probe used to establish obj's witnessable surface |

Anything here worth keeping permanently belongs in the owning subset's `🏭️generator/📜️script.ts`,
per CLAUDE.md's rule that permanent scripts are `📜️script.ts`.
