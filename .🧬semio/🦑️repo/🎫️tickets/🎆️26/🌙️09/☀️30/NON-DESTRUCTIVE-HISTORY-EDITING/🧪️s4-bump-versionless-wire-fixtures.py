#!/usr/bin/env python3
"""🧪️ S4-BUMP: one truth for the channel wire fixtures that pinned the version they were introduced at.

`🧬️fixtures/🪪️document-identity-wire-v20` and `🧬️fixtures/🎬️media-export-wire-v19` lose the version from their names, become registered
channel-version consumers (the census learns the `"channelVersion"` key, the generator writes the pin into them) and both Rust laws assert
equality with `CHANNEL_VERSION`. Every anchor must match exactly as often as stated.
"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
OS = ROOT / "🧰️framework/🛍️products/💻️os"
FIXTURES = OS / "🔨️modules/📡️spr/🧵️channel/🧬️fixtures"


def edit(path: pathlib.Path, pairs: list[tuple[str, str, int]]) -> None:
    text = path.read_text()
    for old, new, count in pairs:
        found = text.count(old)
        if found != count:
            sys.exit(f"{path.name}: {old[:100]!r} matched {found}x (expected {count})")
        text = text.replace(old, new)
    path.write_text(text)


for old, new in [("🪪️document-identity-wire-v20", "🪪️document-identity-wire"), ("🎬️media-export-wire-v19", "🎬️media-export-wire")]:
    source, target = FIXTURES / old, FIXTURES / new
    if source.exists():
        if target.exists():
            sys.exit(f"{target} already exists")
        source.rename(target)

edit(OS / "🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs", [
    ("async fn media_export_wire_matches_the_language_neutral_v19_fixture_above_number_safe_range() {\n    let fixture: serde_json::Value = serde_json::from_str(include_str!(\"../../🧬️fixtures/🎬️media-export-wire-v19/🔣️.json\")).expect(\"media export wire fixture parses\");\n",
     "async fn media_export_wire_matches_the_language_neutral_fixture_above_number_safe_range() {\n    let fixture: serde_json::Value = serde_json::from_str(include_str!(\"../../🧬️fixtures/🎬️media-export-wire/🔣️.json\")).expect(\"media export wire fixture parses\");\n    assert_eq!(fixture[\"channelVersion\"].as_u64().unwrap(), CHANNEL_VERSION as u64);\n", 1),
    ("async fn document_identity_wire_matches_the_language_neutral_v20_fixture() {\n    let fixture: serde_json::Value = serde_json::from_str(include_str!(\"../../🧬️fixtures/🪪️document-identity-wire-v20/🔣️.json\")).unwrap();",
     "async fn document_identity_wire_matches_the_language_neutral_fixture() {\n    let fixture: serde_json::Value = serde_json::from_str(include_str!(\"../../🧬️fixtures/🪪️document-identity-wire/🔣️.json\")).unwrap();", 1),
])
edit(FIXTURES / "🎬️media-export-wire/🧬️schema/🔣️.json", [('"$id": "semio://framework/channel/media-export-wire-v19-fixture"', '"$id": "semio://framework/channel/media-export-wire-fixture"', 1)])
edit(OS / "🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎬️wgpu-browser-media/🟦️.ts", [("🧬️fixtures/🪪️document-identity-wire-v20\"", "🧬️fixtures/🪪️document-identity-wire\"", 1)])
edit(OS / "🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx", [("🧬️fixtures/🎬️media-export-wire-v19/🔣️.json", "🧬️fixtures/🎬️media-export-wire/🔣️.json", 1)])
edit(OS / "🧪️tests/🧪️backbone-envelope-io/🟦️.ts", [("🧬️fixtures/🎬️media-export-wire-v19/", "🧬️fixtures/🎬️media-export-wire/", 1)])

census = OS / "🔨️modules/🧑‍💻dev/🔖️channel-version/🔍️census/🟦️.ts"
edit(census, [
    ("  /appChannelVersion\"?\\s*:\\s*(?:\\{\\s*\"const\"\\s*:\\s*)?(\\d+)/gu,\n",
     "  /appChannelVersion\"?\\s*:\\s*(?:\\{\\s*\"const\"\\s*:\\s*)?(\\d+)/gu,\n  /\"channelVersion\"\\s*:\\s*(?:\\{\\s*\"const\"\\s*:\\s*)?(\\d+)/gu,\n", 1),
])
edit(OS / "🔨️modules/🧑‍💻dev/🔖️channel-version/🟦️.ts", [
    ('"appChannelVersion|CHANNEL_VERSION|app_channel_version|6170704368616e6e656c56657273696f6e"', '"appChannelVersion|channelVersion|CHANNEL_VERSION|app_channel_version|6170704368616e6e656c56657273696f6e"', 1),
])
edit(OS / "🧫️fixtures/📡️channel/📇️consumers.json", [
    ('''    {
      "path": "🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs",
''', '''    {
      "path": "🔨️modules/📡️spr/🧵️channel/🧬️fixtures/🪪️document-identity-wire/🔣️.json",
      "occurrences": 1
    },
    {
      "path": "🔨️modules/📡️spr/🧵️channel/🧬️fixtures/🎬️media-export-wire/🔣️.json",
      "occurrences": 1
    },
    {
      "path": "🔨️modules/📡️spr/🧵️channel/🧬️fixtures/🎬️media-export-wire/🧬️schema/🔣️.json",
      "occurrences": 1
    },
    {
      "path": "🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs",
''', 1),
])
print("versionless wire fixtures staged; run `channel-version generate` next")
