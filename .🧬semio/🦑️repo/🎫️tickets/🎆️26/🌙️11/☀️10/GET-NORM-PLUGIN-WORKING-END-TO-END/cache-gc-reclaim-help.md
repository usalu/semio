# Cache GC Reclaim Help

The canonical English and German command catalogue now lists optional `--reclaim=<size>` for `cache gc` with a required value. The parser accepts both `--reclaim=5GiB` and `--reclaim 5GiB`, refuses a bare flag and does not accept it for `cache status`.

The planner subtracts ordinary age and budget removals from the requested total, then selects additional oldest eligible units, including units below their area budget. Held and quiet units and pressure-pinned areas are excluded from the additional pressure selection; the target is not guaranteed. Before removal the runner rechecks custody and recency. Pinned areas remain subject to their ordinary age and budget policy.

Focused tests passed for both-language help and parser behavior, catalogue schema validity, and the language-neutral help snapshots. The live `--help` command was checked in both languages; no collection or compiler was run.
