//! 🔬️ Unit tests of the GIS vector style: the water, waterway and country-hole band gates and the source-layer draw order the framework policy reads.

use crate::tile_coordinates::gis_policy::gis_map_policy;
use semio_framework_surface::tiled_map::MapPolicy;

#[test]
fn continent_water_filter_drops_inland_polygons() {
    let policy = gis_map_policy();
    assert!(policy.water_area_visible(0, "lake"));
    assert!(!policy.water_area_visible(1, "lake"));
    assert!(policy.water_area_visible(2, "lake"));
    assert!(policy.water_area_visible(1, "ocean"));
    assert!(!policy.waterway_visible(1));
    assert!(policy.waterway_visible(2));
    assert!(policy.admin_area_holes(0));
    assert!(!policy.admin_area_holes(1));
    assert!(policy.admin_area_holes(2));
}

#[test]
fn source_layers_keep_the_openmaptiles_draw_order() {
    let policy = gis_map_policy();
    let order = ["water", "landcover", "landuse", "park", "transportation", "building", "boundary", "waterway", "aeroway", "countries", "geolines", "place", "transportation_name", "water_name", "poi", "centroids"];
    assert!(order.windows(2).all(|pair| policy.draw_rank(pair[0]) < policy.draw_rank(pair[1])), "draw ranks follow the OpenMapTiles paint order");
    assert_eq!(policy.draw_rank("unlisted-layer"), super::GIS_MAP_DEFAULT_DRAW_RANK);
    assert_eq!(policy.layer_role("aeroway"), None, "aeroway is ordered but never painted");
}
