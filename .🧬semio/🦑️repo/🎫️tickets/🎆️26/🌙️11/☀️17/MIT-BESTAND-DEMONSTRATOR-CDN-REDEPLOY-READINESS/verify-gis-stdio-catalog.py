"""🧾️ Compare the fresh GIS guest catalog with the exact checked Stdio bundle."""

import json
from pathlib import Path
import sys
import zlib

ROOT = Path(__file__).resolve().parents[7]
STDIO = ROOT / "🌎️hub/🧩️compositions/🗄️stdio"
BUNDLE = STDIO / "🤖️generated/📜️catalog/🗜️.bin"
GIS = ROOT / "🌎️hub/🧩️compositions/🌍️gis/🧩️component/🔣️.json"
TOPIC = "stdio.artifact-catalog.v1"


def catalog(descriptor):
    manifest = next(row["payload"] for row in descriptor["manifest"]["topicContributions"] if row["topic"] == TOPIC)
    contributed = next(row["payload"] for row in descriptor["contributions"]["topicContributions"] if row["topic"] == TOPIC)
    assert manifest == contributed
    return manifest


guest = catalog(json.loads(Path(sys.argv[1]).read_bytes()))
assert guest == catalog(json.loads(GIS.read_bytes()))
bundle = json.loads(zlib.decompress(BUNDLE.read_bytes(), -15))
assert bundle["schema"] == "semio.stdio.catalog-input/v1"
assert [row["identity"] for row in guest["definitions"]] == sorted(f"s.stdio.{row['artifact']}" for row in bundle["definitions"])
expected = {
    r["factory_id"]: {
        "definitionIdentity": f"s.stdio.{r['artifact']}", "factoryId": r["factory_id"], "artifactKind": r["artifact_kind"],
        "descriptorCodecId": r["descriptor_codec_id"], "extension": r["extension"], "packSchemaHash": r["pack_schema_hash"],
        "runtimeCapabilityId": r["runtime_capability_id"], "schema": r["artifact_schema"],
    }
    for r in bundle["provider"]["receipts"]
}
assert len(expected) == len(bundle["provider"]["receipts"]) == len(guest["codecs"])
assert guest["codecs"] == [expected[factory_id] for factory_id in sorted(expected)]
print(f"Verified {len(guest['definitions'])} ordered definition identities and {len(expected)} exact codec rows")
