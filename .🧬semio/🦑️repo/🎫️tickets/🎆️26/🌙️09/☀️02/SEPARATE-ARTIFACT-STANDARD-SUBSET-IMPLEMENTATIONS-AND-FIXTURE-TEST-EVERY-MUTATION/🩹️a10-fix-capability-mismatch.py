#!/usr/bin/env python3
# 🩹️ A10 — repair the 23 oracle-capability-mismatch breaches. Every one of these artifact-level
# `mutate-<x>` feature files still tags @oracle-<the-old-reimplementation-id> from before that entry
# was reclassified cross-semio-implementation (by an earlier wave of this same ticket); a genuine
# third-party READER sibling entry already exists with the plain capability the feature declares, but
# the feature's tag was never repointed at it. Some reader entries also carry a registration-time typo
# (capability wrongly suffixed `-reader`) or are missing the `comparisonProfiles` entry the retag would
# then expose as oracle-profile-mismatch. Fixed together so neither breach reappears.
import json, re

# (name, feature_path, registry_path, old_oracle_id_in_feature, reader_id, plain_capability, needed_profile)
CASES = [
 ("avi", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🧪️tests/mutate-avi-1-0/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧪️oracle/🔣️.json",
  "riff-avi-1-0-mutate", "riff-avi-1-0-mutate-reader", "avi-1-0-mutate", "semantic-avi-v1"),
 ("bcf", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🧪️tests/mutate-bcf-2-1/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧪️oracle/🔣️.json",
  "zip-quick-xml-bcf-2-1-mutate", "jszip-bcf-2-1-mutate-reader", "bcf-2-1-mutate", "semantic-bcf-v1"),
 ("bmp", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🧪️tests/mutate-bmp-v3/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
  "image-bmp-3-mutate", "image-bmp-3-mutate-reader", "bmp-3-mutate", "semantic-raster-v1"),
 ("docx", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🧪️tests/mutate-docx-ecma-376/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧪️oracle/🔣️.json",
  "zip-quick-xml-docx-ecma-376-mutate", "jszip-docx-ecma-376-mutate-reader", "docx-ecma-376-mutate", "semantic-docx-ecma-376-mutate-v1"),
 ("dxf", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🧪️tests/mutate-dxf-r12/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧪️oracle/🔣️.json",
  "dxf-crate-r12-mutate", "dxf-crate-r12-mutate-reader", "dxf-r12-mutate", "semantic-dxf-r12-v1"),
 ("gif87", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🧪️tests/mutate-gif-87a/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
  "gif-87a-mutate", "gif-87a-any-mutate-reader", "gif-87a-mutate", "semantic-raster-v1"),
 ("gif89", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🧪️tests/mutate-gif-89a/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧪️oracle/🔣️.json",
  "gif-89a-any-mutate", "gif-89a-any-mutate-reader", "gif-89a-mutate", "semantic-raster-v1"),
 ("gltf", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧪️tests/mutate-gltf-2-0/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
  "json-rust-gltf-2-0-mutate", "three-gltf-2-0-mutate-reader", "gltf-2-0-mutate", "semantic-gltf-v1"),
 ("jpg", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🧪️tests/mutate-jpg-jfif-1-01/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧪️oracle/🔣️.json",
  "image-jpeg-jfif-1-01-mutate", "image-jpeg-jfif-1-01-mutate-reader", "jpg-jfif-1-01-mutate", "semantic-jpg-mutate-v1"),
 ("obj", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🧪️tests/mutate-obj-3-0/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧪️oracle/🔣️.json",
  "tobj-obj-3-0-mutate", "tobj-obj-3-0-mutate-reader", "obj-3-0-mutate", "semantic-obj-3-0-v1"),
 ("pdf14a", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-a/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🔣️.json",
  "lopdf-pdf-1-4-a-mutate", "lopdf-pdf-1-4-a-mutate-reader", "pdf-1-4-a-mutate", "semantic-pdf-1-4-conformance-a-v1"),
 ("pdf14x", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-x/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🔣️.json",
  "lopdf-pdf-1-4-x-mutate", "lopdf-pdf-1-4-x-mutate-reader", "pdf-1-4-x-mutate", "semantic-pdf-1-4-conformance-x-v1"),
 ("pdf17a", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7-a/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/🧪️oracle/🔣️.json",
  "lopdf-pdf-1-7-a-mutate", "lopdf-pdf-1-7-a-mutate-reader", "pdf-1-7-a-mutate", "semantic-pdf-conformance-a-v1"),
 ("pdf17e", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7-e/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/🧪️oracle/🔣️.json",
  "lopdf-pdf-1-7-e-mutate", "lopdf-pdf-1-7-e-mutate-reader", "pdf-1-7-e-mutate", "semantic-pdf-conformance-e-v1"),
 ("pdf17h", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7-h/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/🧪️oracle/🔣️.json",
  "lopdf-pdf-1-7-h-mutate", "lopdf-pdf-1-7-h-mutate-reader", "pdf-1-7-h-mutate", "semantic-pdf-conformance-h-v1"),
 ("pdf17ua", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7-ua/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/🧪️oracle/🔣️.json",
  "lopdf-pdf-1-7-ua-mutate", "lopdf-pdf-1-7-ua-mutate-reader", "pdf-1-7-ua-mutate", "semantic-pdf-conformance-ua-v1"),
 ("pdf17vt", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7-vt/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧪️oracle/🔣️.json",
  "lopdf-pdf-1-7-vt-mutate", "lopdf-pdf-1-7-vt-mutate-reader", "pdf-1-7-vt-mutate", "semantic-pdf-conformance-vt-v1"),
 ("pdf17x", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7-x/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/🧪️oracle/🔣️.json",
  "lopdf-pdf-1-7-x-mutate", "lopdf-pdf-1-7-x-mutate-reader", "pdf-1-7-x-mutate", "semantic-pdf-conformance-x-v1"),
 ("png", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🧪️tests/mutate-png-1-2/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
  "png-png-1-2-mutate", "png-png-1-2-mutate-reader", "png-1-2-mutate", "semantic-raster-v1"),
 ("svg", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧪️tests/mutate-svg-1-1/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧪️oracle/🔣️.json",
  "quick-xml-svg-1-1-mutate", "quick-xml-svg-1-1-mutate-reader", "svg-1-1-mutate", "semantic-svg-1-1-v1"),
 ("tiff", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🧪️tests/mutate-tiff-6-0/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧪️oracle/🔣️.json",
  "image-tiff-6-0-mutate", "image-tiff-6-0-mutate-reader", "tiff-6-0-mutate", "semantic-raster-v1"),
 ("xml", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🧪️tests/mutate-xml-1-0/🥒️.feature",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧪️oracle/🔣️.json",
  "quick-xml-1-0-mutate", "quick-xml-1-0-mutate-reader", "xml-1-0-mutate", "semantic-xml-v1"),
]

for name, feature_path, registry_path, old_id, reader_id, plain_cap, profile in CASES:
    with open(registry_path, encoding="utf-8") as fh:
        data = json.load(fh)
    reader = next((o for o in data["oracles"] if o["id"] == reader_id), None)
    if reader is None:
        raise SystemExit(f"{name}: reader {reader_id} not found in {registry_path}")
    if reader.get("capabilities") != [plain_cap]:
        print(f"{name}: fixing capability typo {reader.get('capabilities')} -> ['{plain_cap}']")
        reader["capabilities"] = [plain_cap]
    profiles = reader.get("comparisonProfiles")
    if not profiles:
        reader["comparisonProfiles"] = [profile]
        print(f"{name}: adding missing comparisonProfiles=['{profile}'] to {reader_id}")
    elif profile not in profiles:
        profiles.append(profile)
        print(f"{name}: appending comparisonProfiles {profile} to {reader_id}")
    with open(registry_path, "w", encoding="utf-8") as fh:
        json.dump(data, fh, ensure_ascii=False, indent=2)
        fh.write("\n")

    with open(feature_path, encoding="utf-8") as fh:
        text = fh.read()
    old_line = f"@oracle-{old_id}"
    new_line = f"@oracle-{reader_id}"
    if old_line not in text:
        raise SystemExit(f"{name}: {old_line!r} not found in {feature_path}")
    # exact tag line match only (word-boundary via newline/start), never a substring inside a longer id
    pattern = re.compile(r"(?m)^" + re.escape(old_line) + r"$")
    new_text, count = pattern.subn(new_line, text)
    if count != 1:
        raise SystemExit(f"{name}: expected exactly 1 occurrence of {old_line!r} as its own tag line, found {count}")
    with open(feature_path, "w", encoding="utf-8") as fh:
        fh.write(new_text)
    print(f"{name}: retagged {feature_path} -> {new_line}")

print("\ndone:", len(CASES), "cases (pdf17-base handled separately)")
