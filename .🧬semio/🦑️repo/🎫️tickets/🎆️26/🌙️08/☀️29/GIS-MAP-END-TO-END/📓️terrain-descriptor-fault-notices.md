# GIS Terrain descriptor fault notices

The production-shape Node JSPI probe reached the GIS guest's `describe()` export. The Terrain editor registry panicked before returning its descriptor when it encountered `gis-terrain-brush-settings-invalid`. The registry requires at least three dot-separated lowercase kebab-case segments. The adjacent `gis-terrain-window-context-required` notice failed the same rule.

The Terrain editor now publishes `gis.terrain.brush.settings-invalid` and `gis.terrain.window.context-required`, following its existing `gis.terrain.<area>.<name>` convention. Window mutation routing uses the typed missing-window fault shared with the brush command, so the emitted code resolves to its localized notice. The other published import, export, GeoJSON, and contour codes already conform.

The focused source check rejected precisely the two previous codes before the edit. It accepted all 20 unique published codes after the edit using the host TypeScript grammar and an Ajv JSON Schema oracle. The repo tool wrapper waited for a capacity grant during the active prototype build, so the check ran with the installed Node.js. The new native unit test calls the registry validator on the Terrain editor's actual complete table. The patched guest's `describe()` and the native unit test still require a build with the updated source.
