import hashlib
import json
from pathlib import Path

ticket = Path(__file__).resolve().parent
engine = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine")
topology = engine / "🏗️solid-topology"
transaction = engine / "🧠️native-registry/🏗️solid-transaction"
for row in json.loads((ticket / "🔣️solid-topology-boundaries.json").read_text()):
    source = topology / row["path"] / "🦀️.rs"
    assert source.is_file(), source
    body = "pub" + source.read_text().split("\npub", 1)[1]
    assert hashlib.sha256(body.encode()).hexdigest() == row["sha256"], source
    assert not (transaction / row["path"] / "🦀️.rs").exists()
    assert f'pub mod {row["module"]};' in (topology / "🦀️.rs").read_text()
print("Four boundary modules preserve every original geometry function body.")
