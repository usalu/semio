# Selective Semio Conversion Features

The default Semio artifact no longer compiles sibling codecs. An exact source-path audit found direct subset IO consumers; their dependency declarations now enable only those referenced conversion families. Core model-only consumers such as EN1990 retain no conversion features. These are declaration/source-use checks; selected compile tests remain a separate gate.

| Consumer Manifest | Required Features |
| --- | --- |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/Cargo.toml` | conversion-drawing |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml` | conversion-brep, conversion-mesh |
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/Cargo.toml` | conversion-drawing |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/Cargo.toml` | conversion-drawing |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml` | conversion-mesh |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml` | conversion-brep |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | conversion-mesh |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/Cargo.toml` | conversion-drawing |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/Cargo.toml` | conversion-mesh |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml` | conversion-drawing |
