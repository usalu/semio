#!/usr/bin/env python3
"""🧾️ W2-S-E: writes the committed wire witnesses of the 17 layout leaves that had no fixture — one canonical aggregate wire value
(`{"<Variant>": <camelCase payload>}`, `Option` as `null`, `f64` spelled with a fraction, f32 colour channels exactly representable)
per leaf at `✳️any/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json`. The layout crate's
`committed_wire_witnesses_are_the_canonical_wire` test (`store::os_store::test_support::assert_wire_witness`) and the derived
`semio_payload_law_layout_mutation` prove them canonical; `schema mutation-payloads` validates them against the leaf schemas.

Usage: python3 🧪️w2-s-e-layout-witnesses.py [--apply]
"""
import json
import os
import sys

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations"
BOUNDS = {"x": 10.0, "y": 10.0, "w": 190.0, "h": 20.0, "rotation": 0.0}
WITNESSES = {
    "🖋️create-character-style": {"CreateCharacterStyle": {"id": "char-emphasis", "name": "Emphasis"}},
    "🗑️delete-character-style": {"DeleteCharacterStyle": {"id": "char-emphasis"}},
    "🎨️update-character-style": {"UpdateCharacterStyle": {"id": "char-emphasis", "name": "Emphasis", "fontFamily": None, "fontSize": None, "fontWeight": 700, "italic": True, "color": [0.75, 0.125, 0.125, 1.0], "tracking": None}},
    "📖️update-parent-page": {"UpdateParentPage": {"id": "parent-a", "name": "A-Master", "width": 210.0, "height": 297.0}},
    "📓️update-spread": {"UpdateSpread": {"id": "spread-1", "name": "Opening spread"}},
    "🗄️set-page-parent": {"SetPageParent": {"id": "page-2", "parentPageId": "parent-a"}},
    "📐️set-page-guides": {"SetPageGuides": {"id": "page-1", "guides": [{"x": 105.0, "y": 0.0, "w": 0.0, "h": 297.0}]}},
    "✒️set-story-runs": {"SetStoryRuns": {"id": "story-1", "runs": [{"start": 0, "end": 5, "paragraphStyleId": "para-body", "characterStyleId": None}, {"start": 5, "end": 11, "paragraphStyleId": "para-body", "characterStyleId": "char-emphasis"}]}},
    "🎚️update-link": {"UpdateLink": {"id": "link-1", "width": 1200, "height": 800, "dpi": 300, "colorProfile": "sRGB IEC61966-2.1"}},
    "📎set-page-overrides": {"SetPageOverrides": {"id": "page-2", "overrides": [{"objectId": "frame-header", "bounds": BOUNDS, "visible": True, "locked": None}]}},
    "📑create-layer": {"CreateLayer": {"pageId": "page-1", "id": "layer-notes", "name": "Notes", "remove": False}},
    "🧲set-frame-layer": {"SetFrameLayer": {"pageId": "page-1", "frameId": "frame-rect", "layerId": "layer-notes"}},
    "🏷️set-drawing-text": {"SetDrawingText": {"index": 0, "text": "Ground floor"}},
    "🪜reorder-frame": {"ReorderFrame": {"pageId": "page-1", "frameId": "frame-rect", "forward": True}},
    "📑️update-text-frame": {"UpdateTextFrame": {"pageId": "page-1", "frameId": "frame-text", "storyId": "story-1", "threadNext": None, "insetX": 2.0, "insetY": 2.0, "insetWidth": 4.0, "insetHeight": 4.0}},
    "🗂️update-layer": {"UpdateLayer": {"pageId": "page-1", "layerId": "layer-1", "name": "Artwork", "visible": True, "locked": False}},
    "📝️update-paragraph-style": {"UpdateParagraphStyle": {"id": "para-body", "name": "Body", "fontFamily": "Inter", "fontSize": 10.5, "fontWeight": 400, "leading": 14.0, "tracking": 0.0, "alignment": "left"}},
}

if __name__ == "__main__":
    for leaf, wire in WITNESSES.items():
        path = os.path.join(ROOT, leaf, "🧾️wire-witness", "🦠️mutation", "🔣️.json")
        text = json.dumps(wire, indent=2, ensure_ascii=False) + "\n"
        if os.path.exists(path) and open(path, encoding="utf-8").read() == text:
            continue
        if "--apply" in sys.argv:
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(text)
        print(f"[w2-s-e] {'wrote' if '--apply' in sys.argv else 'would write'} {leaf}")
