# Native root inventory blocked the XLSX host

The standalone editor is `repo run @semio-tech/framework-os-dev:dev-stdio-xlsx-react-prototype` on port 6272. The first start reached graph planning and failed with `Unknown entry exceeds exact Native creator inventory` in `inspectPersistentReceiver`.

`names(root, 0)` throws on the first directory entry. The inactive-root check wanted that length so it could report `Exact empty inactive Native root required`. The graph image `.cache/b/g` was not mounted (`m` shared the disk image's device) and still contained `m/inference` (17 MB of selection rows). That host debris is now `.cache/b/g/host-stray/inference`. The mount directory is empty. The check now names the stray entry instead of throwing the inventory error.

The Office playground rows for `stdio-xlsx`, `stdio-xlsx-strict`, and `stdio-xlsx-transitional` had no `devContribution`. The route script then failed because the parsed row did not point at the stdio-xlsx development schema. Those three rows now do, and the route script passes. The host started before that edit is still reading its project graph slice.

The Office plugin assembles `s.stdio.xlsx` through `definition()`, and that assembly refused every conformance suite whose status was not `unimplemented`. The Calamine oracle suite is `implemented` and is the only such suite in the bundled definitions. Codecs and mutations already admit `unimplemented`, `implemented`, and `verified`. Suites now use that same set, so an implemented oracle stays implemented. Resources stay `unimplemented` until they have an external-reference policy that the contract accepts.

Only the JSON artifact authors an open target. Workbook, document, and presentation compositions leave `openTargets` empty and mount their editors from the Office plugin. The standalone pane uses that plugin registration, so an invented `stdio.xlsx.editor` surface is not part of this host.

The graph slice for the host that started before the contribution edit is still reading the tree. Port 6272 is not listening yet.

At 22:43 the restarted host is still the only listener candidate. Its slice has passed 44 owner checks and 161k source observations. The service log is in `build:command-source-packages` / `build:selected-declaration`, and nothing is listening on 6272. The inventory line's `N of 600` is seconds since the last observation, not a count of remaining owners.
