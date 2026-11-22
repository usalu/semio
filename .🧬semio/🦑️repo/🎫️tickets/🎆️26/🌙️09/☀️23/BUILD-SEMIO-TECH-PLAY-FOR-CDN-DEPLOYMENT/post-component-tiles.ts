import { existsSync } from "node:fs";
import { resolve } from "node:path";
import { GIS_MAP_DEFAULT_PREFETCH_BOUNDS, listMapTilesForBounds, mapTileCacheRoots } from "../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🟦️.ts";
import { PLAY_STATIC_MAP_TILE_Z_MAX } from "../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/📦️site/🗺️tile-serve-mode/🟦️.ts";
const root = process.argv[2]!, { osm, vt } = mapTileCacheRoots(root);
const tiles = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 0, PLAY_STATIC_MAP_TILE_Z_MAX);
const missing = tiles.flatMap(({ z, x, y }) => [resolve(osm, `${z}/${x}/${y}.png`), resolve(vt, `${z}/${x}/${y}.pbf`)]).filter(p => !existsSync(p));
console.log("tiles", tiles.length, "zmax", PLAY_STATIC_MAP_TILE_Z_MAX, "missing files", missing.length, missing.slice(0, 2));
