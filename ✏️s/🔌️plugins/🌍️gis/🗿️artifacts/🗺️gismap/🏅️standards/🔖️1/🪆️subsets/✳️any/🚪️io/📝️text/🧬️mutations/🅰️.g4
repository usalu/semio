// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Gis_gismap_mutations;

DOCUMENT: 'schema' [ ]+ 'gis.gismap.mutations' ;

line: createPosition | deletePosition | replacePositionData | reorderPositions | createRoute | deleteRoute | replaceRouteData | reorderRoutes | createRegion | deleteRegion | replaceRegionData | reorderRegions | setPositionProperty | removePositionProperty | setRouteProperty | removeRouteProperty | setRegionProperty | removeRegionProperty ;
createPosition: 'create-position' SP number ;
deletePosition: 'delete-position' SP id ;
replacePositionData: 'replace-position-data' SP id SP block ;
reorderPositions: 'reorder-positions' SP id SP number ;
createRoute: 'create-route' SP number ;
deleteRoute: 'delete-route' SP id ;
replaceRouteData: 'replace-route-data' SP id SP block ;
reorderRoutes: 'reorder-routes' SP id SP number ;
createRegion: 'create-region' SP number ;
deleteRegion: 'delete-region' SP id ;
replaceRegionData: 'replace-region-data' SP id SP block ;
reorderRegions: 'reorder-regions' SP id SP number ;
setPositionProperty: 'set-position-property' SP id SP id SP block ( SP id )? ;
removePositionProperty: 'remove-position-property' SP id SP id ;
setRouteProperty: 'set-route-property' SP id SP id SP block ( SP id )? ;
removeRouteProperty: 'remove-route-property' SP id SP id ;
setRegionProperty: 'set-region-property' SP id SP id SP block ( SP id )? ;
removeRegionProperty: 'remove-region-property' SP id SP id ;
id: OCTET+ ;
number: OCTET+ ;
text: OCTET+ ;
boolean: 'true' | 'false' ;
block: '{' NL OCTET+ '}' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
NL: '\r'? '\n' ;
OCTET: . ;
SP: ' ' ;
