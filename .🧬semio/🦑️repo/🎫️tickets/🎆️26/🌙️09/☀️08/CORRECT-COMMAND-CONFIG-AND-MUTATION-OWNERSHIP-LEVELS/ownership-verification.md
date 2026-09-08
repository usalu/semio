# Ownership Verification

## Executed Checks

- The initial monorepo Nx run waited behind another process project graph and was cancelled only for this task.
- An isolated Nx workspace retained in `validation/` invokes the actual root `VerifyScript` and stores generated outputs under the ticket.
- The first isolated test exposed missing explicit array types in the normative JSON Schema under strict Ajv. Those types were added.
- The rerun passed all 9 language-independent ownership vectors against Ajv.
- The repository report completed and found 339 declarations at this intermediate point; implementation is ongoing. A report target succeeding does not mean the tree is clean.

## Intermediate Finding Kinds

| Declaration | Occurrences |
| --- | --- |
| command:set-active-utility | 10 |
| command:set-locale | 39 |
| command:set-terminology | 2 |
| field:activeUtilityId | 54 |
| field:locale | 210 |
| field:terminology | 24 |

## Intermediate Plugin Counts

| Declaration | Occurrences |
| --- | --- |
| ✒️writer | 8 |
| ➗️mathematical | 8 |
| 🌀️procedural | 21 |
| 🌊️flow | 7 |
| 🌍️gis | 14 |
| 🌿️vcs | 7 |
| 🎞️animate | 8 |
| 🎥️shooting | 14 |
| 🎬️sequence | 7 |
| 🏗️fem | 7 |
| 🏭️process | 12 |
| 💠️lowpoly | 12 |
| 💡️reasoning | 8 |
| 📋️forms | 8 |
| 📏️layout | 8 |
| 📐️cad | 18 |
| 📖️playbook | 8 |
| 📜️imperative | 8 |
| 📸️remodel | 16 |
| 🔱️trinity | 16 |
| 🕸️dag | 7 |
| 🖍️draw | 14 |
| 🖨️raster | 14 |
| 🗒️note | 16 |
| 🧩️puzzle | 41 |
| 🧱️block | 19 |
| 🪐️space | 6 |
| 🪵️sourcing | 7 |
