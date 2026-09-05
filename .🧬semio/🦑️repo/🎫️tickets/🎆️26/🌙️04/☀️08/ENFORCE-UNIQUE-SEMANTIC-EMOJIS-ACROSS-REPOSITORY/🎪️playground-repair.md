# Demonstrator Playground Emoji Repair

The later whole-plugin census also found a root sibling collision between the plugin descriptor and the Rust manifest module. The descriptor retains `🛂️.descriptor.semio`; the module directory is now `🪪️manifest`, expressing plugin identity. Current source comments and the framework admission fixture were repointed. The full Demonstrator scope now audits clean: 182 files, 121 directories, 297 governed entries, and zero findings in all categories.

## Scope

Owned tree: `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground`.

The four initial duplicate findings were repaired explicitly:

- Editor and viewer `🎚️options` → `☑️options`.
- Subset `🧪️oracle` → `🔮️oracle`.
- The change-schema payload `🔣️.schema.json` → `🧬️.schema.json`.

Rust, Python, descriptor, oracle, scenario-root, and narrative coordinates were updated. The inherited stale unprefixed test and scenario directory references were corrected to the already-present `⚓️mutate-playground-1` and `📅️retags-the-playground-document-schema` paths.

## Verification

The final scoped statute audit covers 169 files, 114 directories, and 283 governed entries. Every finding category is zero. The only remaining `🧪️oracle` text in the scope intentionally points to Stdio.

The exact central oracle override is:

```json
"✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any": "🔮️oracle"
```

`bun nx run @semio-tech/demonstrator-plugin:test-quick` reached the 1,200-second budget while waiting on the shared Cargo target after Stdio warnings. The output contained no missing Demonstrator Playground path introduced by this repair.
