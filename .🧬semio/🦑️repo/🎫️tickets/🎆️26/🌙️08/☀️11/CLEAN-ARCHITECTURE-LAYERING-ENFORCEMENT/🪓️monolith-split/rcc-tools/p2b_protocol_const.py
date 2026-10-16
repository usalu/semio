"""📡️ Replaces the hub native-codec include_bytes of artifact protocol files with the artifact crates' own protocol constants."""
import re
R = "/Users/ueli/Documents/semio/"
TAIL = "standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO.as_bytes()"
def fix(path, mapping):
    t = open(R + path, encoding="utf-8").read()
    pat = re.compile(r'include_bytes!\("(?:\.\./)+✏️s/🔌️plugins/([^"]+?)/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📡️\.protocol\.semio"\)\.as_slice\(\)')
    found = pat.findall(t)
    assert sorted(found) == sorted(mapping), found
    t = pat.sub(lambda m: mapping[m.group(1)] + "::" + TAIL, t)
    open(R + path, "w", encoding="utf-8").write(t)
    print(path, len(found))
fix("🌎️hub/🧩️compositions/🌍️gis/📇️native-codecs/🦀️.rs", {"🌍️gis/🗿️artifacts/🗺️gismap": "semio_s_artifact_gis_gismap", "🌍️gis/🗿️artifacts/🏔️gisterrain": "semio_s_artifact_gis_gisterrain"})
fix("🌎️hub/🧩️compositions/🌿️vcs/📇️native-codecs/🦀️.rs", {"🌿️vcs/🗿️artifacts/🌿️vcs": "semio_s_artifact_vcs_vcs"})
