# Gis2d Presence Direct-Leaf Blocker

## Scope

The reviewed owner is `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence`. No `compose` path was accessed.

## Observed roster

`Gis2dPresenceMutation` is a single manual `Snapshot { presence: Gis2dPresence }` operation in `🦀️component.rs`. It uses the entire `Gis2dPresence` as its diff, returns the existing `mutation.no-op` warning when the requested snapshot equals the base, and inverts to a snapshot of the base. The only in-scope typed join is `Gis2dApp`'s `PresenceMutation` associated type in the editor parent at lines 579–580; there are no separate constructor joins under the inspected gismap root.

The direct-leaf migration must preserve that replacement/no-op-warning/inverse contract while replacing the whole-record diff with a leaf-owned structural camera replacement diff. The aggregate must be transparent; a fake no-op, a compatibility alias, or a generic whole-record diff is out of scope.

## Blocking schema mismatch

The actual Rust state and the Rust schema facet each have exactly one field: `camera_json: String` (`cameraJson` on the wire). Four other sidecars instead require an obsolete six-field presence state:

| Sidecar | Extra fields absent from the actual state |
| --- | --- |
| `🧬️schema/🟦️component.ts` | `selectedIds`, `featureSelectionJson`, `hoverJson`, `selectionMethod`, `selectionMode` |
| `🧬️schema/🔗️component.graphql` | `selectedIds`, `featureSelectionJson`, `hoverJson`, `selectionMethod`, `selectionMode` |
| `🧬️schema/🛰️component.proto` | `selected_ids`, `feature_selection_json`, `hover_json`, `selection_method`, `selection_mode`; `camera_json` is tag 2 because obsolete `selected_ids` owns tag 1 |
| `🧬️schema/🔣️component.json` | required/properties for `selectedIds`, `featureSelectionJson`, `hoverJson`, `selectionMethod`, `selectionMode` |

This contradicts the real source comment: selection and hover now travel through the framework's typed presence interaction and are deliberately no longer mirrored by this state. The direct-leaf payload schema cannot be authoritatively derived until these sidecars are coordinated to the actual one-field state (including the Proto tag decision). Per ownership direction, no presence production or sidecar edit has been made.

## Source hashes at review

| File | SHA-256 |
| --- | --- |
| `🦀️component.rs` | `dcfdfd842c7d42a986d9851f5d95417897bffe7eb485bd03825217d6e0881536` |
| `🧬️schema/🦀️component.rs` | `7bcf00bb9ccf8e058b96375611cba40cd7b7b4cf45545632403590be4a9d108a` |
| `🧬️schema/🟦️component.ts` | `ae4db001fdc8983145878a434cae35e5c2e8dcca7d3078c0a1336b2078a22d2d` |
| `🧬️schema/🔗️component.graphql` | `7f80e2ae88d0534d67b562267d51cf9cfaa545f4ba1a58f668d269acced9a107` |
| `🧬️schema/🛰️component.proto` | `b2188c908e00366de690d0286472f8a77fb8143dc36193bdcaefbe3874ac9282` |
| `🧬️schema/🔣️component.json` | `c63b69ff473aaad9e45c3f48c42b0b26be7233c1fd213f62b11609b0c5589241` |

## Execution status

This is an inspection-only blocker capture. No neutral controller, source patch, Cargo invocation, Rust compilation, or native test has run.
