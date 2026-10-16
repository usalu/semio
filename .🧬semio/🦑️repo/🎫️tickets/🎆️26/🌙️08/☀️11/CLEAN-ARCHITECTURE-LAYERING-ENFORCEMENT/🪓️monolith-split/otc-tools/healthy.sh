#!/bin/zsh
# usage: healthy.sh ; exits 0 when the foreign breakage seen at 13:44 (taxonomy schema, process/state syntax) is gone
R=/Users/ueli/Documents/semio
cd $R
bun -e "
const m = await import('$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts'); m.loadCatalogTaxonomy();
await import('$R/🧰️framework/🔨️modules/🏃️process/💾️state/🟦️.ts');
console.log('healthy');" 2>&1 | tail -3
