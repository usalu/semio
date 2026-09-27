#!/usr/bin/env python3
"""📦️ F10b: moves the glTF cases whose rows start from a COMMITTED pair onto the `three-gltf-2-0-mutate-reader`
pipeline (the camera case was the hand-made pilot): feature → `semantic-gltf-reader-v1` + a step naming the committed
after-document, a TS oracle adapter answering the committed fixture as `expected-gltf`, and the Rust SUBJECT answering
its document as `actual-gltf`. Refuses a second run. `--write` applies; default is a dry run."""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets")
CASES = [
    ("🎞️animation/🧪️tests/🎞️mutate-gltf-2-0-animation", "shared://<fixture>/⬅️before.gltf", "shared://<fixture>/➡️after.gltf", "➡️after.gltf", "⬅️before.gltf", "animation"),
    ("🦴️skin/🧪️tests/🦴️mutate-gltf-2-0-skin", "shared://<fixture>/⬅️before.gltf", "shared://<fixture>/➡️after.gltf", "➡️after.gltf", "⬅️before.gltf", "skin"),
    ("💎️material/🧪️tests/💎️mutate-gltf-2-0-material", "shared://<id>-applied/before.gltf", "shared://<id>-applied/after.gltf", "after.gltf", "before.gltf", "material"),
    ("🪪️asset/🧪️tests/🪪️mutate-gltf-2-0-asset", "shared://<id>-applied/before.gltf", "shared://<id>-applied/after.gltf", "after.gltf", "before.gltf", "asset"),
]
PLATFORM = "../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts"
ACTUAL_FN = """
    /// 📦️ The produced document as the `actual-gltf` artifact the `gltf-2-0-three-compare-v1` pipeline reads.
    fn actual(ctx: &Context, bytes: Vec<u8>, projection: Json) -> Result<Outcome, String> {
        let path = ctx.artifact("actual-gltf", "actual.gltf")?;
        std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;
        Ok(Outcome::with_raw(bytes, projection).artifact("actual-gltf", &path, "model/gltf+json"))
    }
"""
write = "--write" in sys.argv
problems, edits = [], []


def ts_adapter(subset, after_leaf, before_leaf):
    return f'''/** 🟦️ glTF 2.0 `{subset}` mutation case — the ORACLE half, `three-gltf-2-0-mutate-reader`. A reader oracle computes
 *  nothing: each row's expected document is the COMMITTED fixture — `{after_leaf}` for a mutation, `{before_leaf}` for its
 *  inverse — handed to the `gltf-2-0-three-compare-v1` pipeline as `expected-gltf`, where three's GLTFLoader reads it and
 *  the subject's `actual-gltf`. */

import {{ committedArtifact, defineTestAdapter }} from "{PLATFORM}";

//#region 🧭️Adapter
export default defineTestAdapter({{
  implementation: "typescript",
  scenarios: {{
    mutate: {{ oracle: (ctx) => committedArtifact(ctx, "{after_leaf}", "expected-gltf", "model/gltf+json") }},
    inverse: {{ oracle: (ctx) => committedArtifact(ctx, "{before_leaf}", "expected-gltf", "model/gltf+json") }},
  }},
}});
//#endregion 🧭️Adapter
'''


for rel, before_uri, after_uri, after_leaf, before_leaf, subset in CASES:
    case = ROOT / rel
    feature, rust, ts = case / "🥒️.feature", case / "🦀️.rs", case / "🟦️.ts"
    if ts.exists():
        problems.append(f"{ts}: already exists")
        continue
    f = feature.read_text(encoding="utf-8")
    given = f"    Given the real input document {before_uri}\n"
    for old, count in (("@comparison-semantic-gltf-v1\n", 1), (given, 2), ("    Then the oracle and the subject agree on the semantic projection\n", 1), ("    Then the document matches its pre-mutation semantic projection\n", 1)):
        if f.count(old) != count:
            problems.append(f"{feature}: {old.strip()!r} found {f.count(old)} times, expected {count}")
    f = f.replace("@comparison-semantic-gltf-v1\n", "@comparison-semantic-gltf-reader-v1\n")
    f = f.replace(given, given + f"    And the committed after-document {after_uri}\n")
    f = f.replace("    Then the oracle and the subject agree on the semantic projection\n", "    Then three's GLTFLoader reads the subject's document and the committed after-document as the same glTF\n")
    f = f.replace("    Then the document matches its pre-mutation semantic projection\n", "    Then three's GLTFLoader reads the restored document and the committed before-document as the same glTF\n")
    r = rust.read_text(encoding="utf-8")
    start = r.find("mod subject {")
    end = r.find("//#endregion 🔖️Handlers", start)
    if start < 0 or end < 0:
        problems.append(f"{rust}: subject module or handlers region not found")
        continue
    region = r[start:end]
    old_ret = "        Ok(Outcome::with_raw(bytes, projection))\n"
    if region.count(old_ret) != 2:
        problems.append(f"{rust}: subject returns found {region.count(old_ret)} times, expected 2")
        continue
    region = region.replace(old_ret, "        actual(ctx, bytes, projection)\n")
    marker = "    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {"
    if region.count(marker) != 1:
        problems.append(f"{rust}: inverse handler not found once")
        continue
    region = region.replace(marker, ACTUAL_FN.lstrip("\n") + "\n" + marker)
    r = r[:start] + region + r[end:]
    edits += [(feature, f), (rust, r), (ts, ts_adapter(subset, after_leaf, before_leaf))]

for path, _ in edits:
    print(("write " if write else "dry-run ") + str(path))
for problem in problems:
    print("problem:", problem)
if problems:
    sys.exit(1)
if write:
    for path, text in edits:
        path.write_text(text, encoding="utf-8")
