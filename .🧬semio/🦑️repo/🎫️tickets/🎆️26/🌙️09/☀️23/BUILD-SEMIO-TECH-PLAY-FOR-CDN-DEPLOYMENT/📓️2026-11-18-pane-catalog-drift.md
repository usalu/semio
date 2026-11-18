# Play pane catalog drift (2026-11-18)

## Result
No pane, fixture or lane edit was needed: all 4 failing play unit tests (81/81 now green) shared one cause. The generated plugin catalog `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` was stale (written 2026-11-17 18:09, 70 plugin crates) and still described the pre-split `norm` plugin.

## Cause
The norm plugin was split into five plugins (`norm`, `norm-energy`, `norm-eurocode`, `norm-eurocode-structural`, `norm-eurocode-materials`, ticket GET-NORM-PLUGIN-WORKING-END-TO-END). The stale registry still mapped all 15 norm variants to plugin `norm` with the hub composition's `cratePath`. Consequences:
1. Playscopes: the closure of `en1990` etc. collapsed to `['norm']`, so the fixture (five components) looked wrong. The fixture is correct.
2. Pane defaults: the tests resolved each pane's app through the hub descriptor, which lacks the en*/din18599 apps, so they fell back to `apps[0]` = `s.norm.din4108`. That produced the 9 "not published" examples and the 2 `exampleLabel` mismatches. The pane catalog is correct.
3. Activation lanes: project.json lists 39 lanes (din18599, en1990, en1992, en1995 added); the stale registry derived 35.

## Fix
Regenerated the catalog with the native driver (`GET-NORM-PLUGIN-WORKING-END-TO-END/play-catalog-generate-driver.ts generate`, run from `✏️s/🧑‍💻dev`): 74 plugin crates, 158 playgrounds.

## Pane changes
None. `🔨️modules/🧩️runtime/🔣️.json` is unchanged: `en1990` high-consequence-office, `en1991` de-office-compliant, `en1992` compliant-office-frame, `en1993` heb240-compliant, `en1994` composite-floor-beam, `en1995` glulam-floor-beam, `en1996` loadbearing-wall, `en1998` seismic-rc-frame, `en1999` aluminium-roof-purlin, `din18599` "Demo", `en1997` "Compliant geotechnical package" are all published by their own apps in the regenerated catalog.
`📋️project.json` lanes and `🧪️playscopes` fixture are unchanged.

## Note
The catalog must be regenerated whenever the plugin split or component descriptors change; a stale catalog silently breaks these three suites.
