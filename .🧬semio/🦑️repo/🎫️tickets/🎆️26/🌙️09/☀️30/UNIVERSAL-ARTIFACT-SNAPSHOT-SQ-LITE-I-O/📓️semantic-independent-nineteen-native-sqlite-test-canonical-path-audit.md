# Actual Nineteen Native SQLite Test Canonical Path Audit

The actual Semio crate root registers every subset under `standards::v1::subsets`. It exports SemioSnapshot/SemioDiff/SemioMutation/Artifact types at the root, but no root base IO module. Root names animation/audio/.../video are private imports of each `schema::snapshot` module for retirement implementation; they are not aliases of full subset modules and expose neither the subset IO subtree nor another schema subtree. Therefore short `crate::<subset>::io::...` and `crate::<subset>::schema::...` test paths resolve to the wrong actual namespace or a missing root name. This conclusion comes from actual registration/import source, not rustfmt or an inferred compiler receipt.

Nineteen actual Native test files were inspected. Animation already uses the complete canonical path. Eighteen files retain short IO/schema paths. The prepared exact line guards contain 25 regions replacing 26 occurrences, exclusively with `crate::standards::v1::subsets::<same owner>::`. Constructors, assertions, fixture bodies, and current pub(crate) fixture declarations are preserved.

| Owner | Exact lines | Short occurrences |
|---|---:|---:|
| ✉️base | 7 | 8 |
| 🌊️flow | 1 | 1 |
| 🎞️animation | 0 | 0 |
| 🎬️video | 1 | 1 |
| 🏛️model | 1 | 1 |
| 📊️table | 1 | 1 |
| 📐️cad | 1 | 1 |
| 📑️document | 1 | 1 |
| 📦️object | 1 | 1 |
| 📽️presentation | 1 | 1 |
| 🔊️audio | 1 | 1 |
| 🔢️value | 2 | 2 |
| 🔤️text | 1 | 1 |
| 🔺️mesh | 1 | 1 |
| 🕸️graph | 1 | 1 |
| 🖊️drawing | 1 | 1 |
| 🖼️image | 1 | 1 |
| 🧊️brep | 1 | 1 |
| 🧰️kit | 1 | 1 |

Input: `📥️inputs/semio-nineteen-native-tests-current-canonical-path-narrowed-guards.json`. It records actual current production test lines and exact replacements only. It was not mounted. Root should carry corrections into any held test provider after/before images that target the same files, so a later aggregate mount cannot reintroduce the old namespace. No root aliases, compatibility modules, or new semantics are proposed. No compiler or runtime pass is claimed; original owner Native59819 remains Root-owned.
