#!/usr/bin/env python3
"""🔨️ F1 — real before/after envelope fixture pairs for `s.stdio.semio@v1/base`'s `apply-<arm>`
routing verbs, for the seven arms this shard already built a full, executed reference for
(animation/audio/flow/model/presentation/value/video — see `🔨️f1-semio-generate.py`). Each pair
wraps that SAME arm's real committed document (before) and the SAME real `apply_mutation` result
(after) this shard already computed and verified, inside the envelope's own
`SemioSnapshot{schema, subset: SemioSubsetSnapshot::<Arm>(...)}` shape — confirmed against the
committed `replaces-the-envelope-wrapping-a-value-subset` vector's own JSON encoding
(`{"schema":"stdio.semio","subset":{"subset":"<arm>", ...arm fields}}`). No new mutation semantics
are introduced: the wrapped verb and its real effect are exactly what the sibling subset-level
fixture already demonstrates; this fixture additionally demonstrates the ENVELOPE threading it
through unchanged.

The remaining eleven arms (brep, mesh, document, cad, drawing, image, text, table, graph, object,
kit) have no Python second implementation and no real-world document in this repository (per this
subset's own recorded no-oracle decision) and are NOT attempted here — left itemised in the shard
report rather than guessed at.

Run: `uv run --group test python3 🔨️f1-semio-base-generate.py`
"""
import hashlib
import importlib.util
import json
import os
import sys

REPO = os.getcwd()
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"

# One representative already-generated kind per arm, reused verbatim from 🧫️fixtures/<kind>-applied/.
REPRESENTATIVE_KIND = {
    "animation": "insert-timeline",
    "audio": "insert-channel",
    "flow": "insert-node",
    "model": "insert-spatial-node",
    "presentation": "insert-slide",
    "value": "insert-list-item",
    "video": "insert-stream",
}


def sibling_generator_module():
    spec = importlib.util.spec_from_file_location("f1_semio_generate", os.path.join(REPO, TICKET, "🔨️f1-semio-generate.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def sha256_of_bytes(b):
    return f"sha256:{hashlib.sha256(b).hexdigest()}", len(b)


def write_json_pair(fixture_dir, before, after):
    os.makedirs(fixture_dir, exist_ok=True)
    before_bytes = (json.dumps(before, indent=2, ensure_ascii=False) + "\n").encode("utf-8")
    after_bytes = (json.dumps(after, indent=2, ensure_ascii=False) + "\n").encode("utf-8")
    with open(os.path.join(fixture_dir, "before.json"), "wb") as f:
        f.write(before_bytes)
    with open(os.path.join(fixture_dir, "after.json"), "wb") as f:
        f.write(after_bytes)
    return before_bytes, after_bytes


def camel(arm):
    return arm[0].upper() + arm[1:]


def main():
    gen = sibling_generator_module()
    gen.stub_harness()

    base_root = os.path.join(REPO, ROOT, "✳️base")
    fragments = []

    for arm, kind in REPRESENTATIVE_KIND.items():
        cfg = gen.SUBSETS[arm]
        mod = gen.load_module(os.path.join(REPO, cfg["py"]), f"semio_base_gen_{arm}")
        with open(os.path.join(REPO, cfg["base_doc"]), encoding="utf-8") as f:
            text = f.read()
        arm_before = mod.parse_dsl(text)
        payload = cfg["builder"](arm_before, kind)
        if arm in gen.ENVELOPE_NESTED:
            inner_mutation = {"kind": kind, "params": payload}
        else:
            inner_mutation = dict(payload)
            inner_mutation["mutation"] = gen.kebab_to_camel(kind)
        arm_after = mod.apply_mutation(arm_before, inner_mutation)

        wrapped_before_subset = dict(arm_before)
        wrapped_before_subset["subset"] = arm
        wrapped_after_subset = dict(arm_after)
        wrapped_after_subset["subset"] = arm

        envelope_before = {"schema": "stdio.semio", "subset": wrapped_before_subset}
        envelope_after = {"schema": "stdio.semio", "subset": wrapped_after_subset}

        envelope_mutation = {"mutation": f"apply{camel(arm)}", "payload": {"mutation": inner_mutation}}

        fixture_id = f"apply-{arm}-applied"
        fixture_dir = os.path.join(base_root, "🧫️fixtures", fixture_id)
        before_bytes, after_bytes = write_json_pair(fixture_dir, envelope_before, envelope_after)
        before_sha, before_len = sha256_of_bytes(before_bytes)
        after_sha, after_len = sha256_of_bytes(after_bytes)

        fragments.append(
            {
                "id": fixture_id,
                "mutation": arm,
                "wrapped_kind": kind,
                "envelope_mutation": envelope_mutation,
                "files": [
                    {"role": "expected-before-json", "path": f"../🧫️fixtures/{fixture_id}/before.json", "mediaType": "application/json", "sha256": before_sha, "bytes": before_len},
                    {"role": "expected-after-json", "path": f"../🧫️fixtures/{fixture_id}/after.json", "mediaType": "application/json", "sha256": after_sha, "bytes": after_len},
                ],
            }
        )
        print(f"apply-{arm}-applied [wraps {kind}]: before={before_len}B after={after_len}B")

    out = os.path.join(REPO, TICKET, "🗑️generated", "f1-semio-base-fragments.json")
    with open(out, "w") as f:
        json.dump(fragments, f, indent=2, ensure_ascii=False)
    print("wrote", out)


if __name__ == "__main__":
    main()
