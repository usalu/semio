"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR = exports.DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER = void 0;
exports.demonstratorGisMapTileServeMode = demonstratorGisMapTileServeMode;
/** @emoji 🎚️ Static demonstrator ship: Switzerland through z10 (~400 tiles), not full offline play z13/z14 (~46k). */
exports.DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER = 10;
exports.DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR = 10;
function demonstratorGisMapTileServeMode(viteCommand) {
    if (viteCommand === "build")
        return "bundle";
    return process.env.GIS_MAP_TILE_SERVE_MODE === "bundle" ? "bundle" : "fetch";
}
