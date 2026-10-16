import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tf
P = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🎛️map-policy"
src = P + "/🧪️tests/🔬️unit/🦀️.rs"
t = open(src, encoding="utf-8").read()
host_names = ["host_with", "FixedTileZoomPolicy", "FixedTileZoomExtension", "a_non_default_policy_is_honoured_by_the_map_host", "a_policy_shipped_in_the_map_descriptor_replaces_the_neutral_policy", "a_described_policy_honours_its_band_floors_in_the_host"]
lines, items = tf.split(t)
for n, a, b in items: print(a + 1, b, n)
