# /// script
# requires-python = ">=3.9"
# dependencies = ["affine==2.4.0", "jsonschema==4.25.1", "pillow==11.3.0"]
# ///

import base64
from io import BytesIO
import json
from pathlib import Path

from affine import Affine
from jsonschema import Draft7Validator
from PIL import Image

fixtures = Path(__file__).resolve().parent
layout = fixtures.parents[2]
fixture = json.loads((fixtures / "linked-asset.json").read_text(encoding="utf-8"))
schema = json.loads(
    (layout / "🧬️schema" / "🛍️canvas-catalogue" / "linked-asset.json").read_text(encoding="utf-8")
)
expected = fixture["expected"]
validator = Draft7Validator(schema)
validator.validate(expected["drag"])
assert all(not validator.is_valid(payload) for payload in fixture["refusals"])
assert len(json.dumps(expected["drag"]).encode("utf-8")) <= 512

boundary = fixture["catalogueBoundary"]
for link in boundary["links"]:
    link_id = link["character"] * link["count"]
    witness = {**expected["drag"], "sourceLinkId": link_id}
    validator.validate(witness)
    payload = json.dumps(witness, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
    assert len(payload) > boundary["maximumTextBytes"]
    row_key_fits = len(f"layout-catalogue.link.{link_id}".encode("utf-8")) <= boundary["maximumTextBytes"]
    assert row_key_fits == link["rowKeyFits"]

proxy = fixture["source"]["proxyDataUrl"]
assert len(proxy.encode("utf-8")) > 512
with Image.open(BytesIO(base64.b64decode(proxy.removeprefix("data:image/png;base64,"), validate=True))) as image:
    assert image.size == (expected["preview"]["width"], expected["preview"]["height"])
    for sample in expected["preview"]["rgbaAt"]:
        assert list(image.getpixel((sample["x"], sample["y"]))) == sample["rgba"]

drop, camera = fixture["drop"], fixture["camera"]
to_screen = (
    Affine.translation(drop["width"] / 2, drop["height"] / 2)
    * Affine.scale(camera["zoom"])
    * Affine.translation(-camera["x"], -camera["y"])
)
assert ~to_screen * (drop["x"], drop["y"]) == (expected["bounds"]["x"], expected["bounds"]["y"])

example = next(layout.glob("🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/🖼️assets/🔗️linked-asset/🗣️.dsl.semio"))
example_text = example.read_text(encoding="utf-8")
assert f'proxy-data-url="{proxy}"' in example_text
assert all(f'id={target["id"]} ' in example_text for target in boundary["selection"])
