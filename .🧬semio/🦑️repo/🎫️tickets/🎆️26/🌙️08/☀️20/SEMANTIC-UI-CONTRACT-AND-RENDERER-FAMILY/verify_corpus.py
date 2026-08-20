#!/usr/bin/env python3
"""Reference re-implementation of ui_contract's validate_core / apply_patch (from
limits.rs, read 2026-08-20) in plain Python, used ONLY to cross-check the hand-authored
conformance corpus's expect.json files against the same algorithm the real Rust crate
runs — since U4 forbids running cargo here. Not part of the corpus; a throwaway
verifier only, scratchpad-only.
"""
import copy
import glob
import json
import os

ROOT = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance"

DEFAULT_LIMITS = {"maxNodes": 20000, "maxDepth": 128, "maxChildren": 4096, "maxTextBytes": 65536, "maxPatchOps": 4096, "maxPatchBytes": 1048576}


def is_section(rec):
    c = rec["component"]
    return c["type"] == "container" and c.get("role", "plain") == "section"


def component_is_finite(c):
    def finite(x):
        return x is None or (isinstance(x, (int, float)) and x == x and abs(x) != float("inf"))
    if c["type"] == "slider":
        return all(finite(c.get(k)) for k in ("value", "min", "max", "step"))
    if c["type"] == "numberStepper":
        return all(finite(c.get(k)) for k in ("value", "step"))
    if c["type"] == "ring":
        return finite(c.get("t"))
    if c["type"] == "input":
        return all(finite(c.get(k)) for k in ("min", "max", "step"))
    return True


def label_bytes(label):
    return len(label) if isinstance(label, str) else 0


def component_text_bytes(c):
    t = c["type"]
    if t == "container":
        return label_bytes(c.get("label")) + len(c.get("description") or "") + len(c.get("error") or "")
    if t == "text":
        return len(c["value"])
    if t == "button":
        return len(c["label"])
    if t == "input":
        return len(c["value"]) + label_bytes(c.get("placeholder"))
    if t == "select":
        return sum(len(i["label"]) for i in c["items"]) + label_bytes(c.get("placeholder"))
    if t == "toggle":
        return label_bytes(c.get("text"))
    if t == "keyValueList":
        return sum(len(e["label"]) + len(e["value"]) for e in c["entries"])
    if t == "treeSection":
        return label_bytes(c.get("label"))
    if t == "treeItem":
        return len(c["label"]) + len(c.get("description") or "")
    if t == "image":
        return label_bytes(c.get("alt"))
    if t == "extension":
        return len(c["extension"])
    return 0


def accessibility_text_bytes(a):
    return label_bytes(a.get("label")) + label_bytes(a.get("description")) + len(a.get("shortcut") or "")


def bindings_text_bytes(bindings):
    return sum(len(b["action"]["scope"]) + len(b["action"]["name"]) + len(b.get("capability") or "") for b in bindings)


def menu_text_bytes(menu):
    return len(menu["id"]) if menu else 0


def op_text_bytes(op):
    t = op["type"]
    if t == "upsert":
        return len(op["key"]) + component_text_bytes(op["component"]) + accessibility_text_bytes(op.get("accessibility", {})) + bindings_text_bytes(op.get("bindings", [])) + menu_text_bytes(op.get("menu"))
    if t == "setComponent":
        return component_text_bytes(op["component"])
    if t == "setChildren":
        return len(op["children"]) * 8
    if t == "setAccessibility":
        return accessibility_text_bytes(op["accessibility"])
    if t == "setBindings":
        return bindings_text_bytes(op["bindings"])
    if t == "setMenu":
        return menu_text_bytes(op.get("menu"))
    return 0


def patch_byte_estimate(ops):
    return sum(16 + op_text_bytes(op) for op in ops)


def validate_core(root, nodes, limits):
    violations = []
    if len(nodes) > limits["maxNodes"]:
        return [{"type": "nodeQuota", "count": len(nodes), "max": limits["maxNodes"]}]
    visited = set()
    on_path = set()
    if root is not None and root in nodes:
        stack = [("enter", root, 0, False)]
        while stack:
            frame = stack.pop()
            if frame[0] == "exit":
                on_path.discard(frame[1])
                continue
            _, nid, depth, parent_in_section = frame
            if nid in on_path:
                violations.append({"type": "cycle", "node": nid})
                continue
            if nid in visited:
                continue
            visited.add(nid)
            rec = nodes.get(nid)
            if rec is None:
                continue
            in_section = parent_in_section or is_section(rec)
            if parent_in_section and is_section(rec):
                violations.append({"type": "sectionNested", "node": nid})
            if not component_is_finite(rec["component"]):
                violations.append({"type": "nonFiniteNumber", "node": nid})
            if depth > limits["maxDepth"]:
                violations.append({"type": "depthQuota", "node": nid, "depth": depth, "max": limits["maxDepth"]})
                continue
            on_path.add(nid)
            stack.append(("exit", nid))
            seen_keys = set()
            for child_id in rec.get("children", []):
                child = nodes.get(child_id)
                if child is None:
                    violations.append({"type": "orphanChild", "parent": nid, "child": child_id})
                else:
                    if child["key"] in seen_keys:
                        violations.append({"type": "duplicateSiblingKey", "parent": nid, "key": child["key"]})
                    seen_keys.add(child["key"])
                    stack.append(("enter", child_id, depth + 1, in_section))
    for nid in nodes:
        if nid not in visited:
            violations.append({"type": "danglingRoot", "node": nid})
    return violations


def apply_op(draft, op, limits):
    t = op["type"]
    if t == "upsert":
        if len(op.get("children", [])) > limits["maxChildren"]:
            return {"type": "quotaExceeded", "quota": "children", "actual": len(op.get("children", [])), "max": limits["maxChildren"]}
        bytes_ = component_text_bytes(op["component"])
        if bytes_ > limits["maxTextBytes"]:
            return {"type": "quotaExceeded", "quota": "textBytes", "actual": bytes_, "max": limits["maxTextBytes"]}
        rec = {k: v for k, v in op.items() if k != "type"}
        draft["nodes"][rec["id"]] = rec
        return None
    if t == "setComponent":
        bytes_ = component_text_bytes(op["component"])
        if bytes_ > limits["maxTextBytes"]:
            return {"type": "quotaExceeded", "quota": "textBytes", "actual": bytes_, "max": limits["maxTextBytes"]}
        if op["id"] not in draft["nodes"]:
            return {"type": "unknownNode", "id": op["id"]}
        draft["nodes"][op["id"]]["component"] = op["component"]
        return None
    if t == "setLayout":
        if op["id"] not in draft["nodes"]:
            return {"type": "unknownNode", "id": op["id"]}
        draft["nodes"][op["id"]]["layout"] = op["layout"]
        return None
    if t == "setActivity":
        if op["id"] not in draft["nodes"]:
            return {"type": "unknownNode", "id": op["id"]}
        draft["nodes"][op["id"]]["activity"] = op["activity"]
        if op["disabled"]:
            draft["nodes"][op["id"]]["disabled"] = True
        else:
            draft["nodes"][op["id"]].pop("disabled", None)
        return None
    if t == "setChildren":
        if len(op["children"]) > limits["maxChildren"]:
            return {"type": "quotaExceeded", "quota": "children", "actual": len(op["children"]), "max": limits["maxChildren"]}
        if op["id"] not in draft["nodes"]:
            return {"type": "unknownNode", "id": op["id"]}
        draft["nodes"][op["id"]]["children"] = op["children"]
        return None
    if t == "setStyle":
        if op["id"] not in draft["nodes"]:
            return {"type": "unknownNode", "id": op["id"]}
        draft["nodes"][op["id"]]["style"] = op["style"]
        return None
    if t == "setAccessibility":
        if op["id"] not in draft["nodes"]:
            return {"type": "unknownNode", "id": op["id"]}
        draft["nodes"][op["id"]]["accessibility"] = op["accessibility"]
        return None
    if t == "setBindings":
        if op["id"] not in draft["nodes"]:
            return {"type": "unknownNode", "id": op["id"]}
        draft["nodes"][op["id"]]["bindings"] = op["bindings"]
        return None
    if t == "setMenu":
        if op["id"] not in draft["nodes"]:
            return {"type": "unknownNode", "id": op["id"]}
        draft["nodes"][op["id"]]["menu"] = op.get("menu")
        return None
    if t == "remove":
        stack = [op["id"]]
        while stack:
            cur = stack.pop()
            rec = draft["nodes"].pop(cur, None)
            if rec:
                stack.extend(rec.get("children", []))
        return None
    if t == "setRoot":
        draft["root"] = op["id"]
        return None
    raise ValueError(f"unknown op {t}")


def apply_patch(state, patch, limits):
    if patch["baseRevision"] != state["revision"]:
        return {"type": "revisionMismatch", "expected": state["revision"], "actual": patch["baseRevision"]}
    if len(patch["ops"]) > limits["maxPatchOps"]:
        return {"type": "quotaExceeded", "quota": "patchOps", "actual": len(patch["ops"]), "max": limits["maxPatchOps"]}
    estimated = patch_byte_estimate(patch["ops"])
    if estimated > limits["maxPatchBytes"]:
        return {"type": "quotaExceeded", "quota": "patchBytes", "actual": estimated, "max": limits["maxPatchBytes"]}
    draft = copy.deepcopy(state)
    for op in patch["ops"]:
        rejection = apply_op(draft, op, limits)
        if rejection:
            return rejection
    draft["revision"] = patch["revision"]
    violations = validate_core(draft["root"], draft["nodes"], limits)
    if violations:
        return {"type": "invariantViolated", "violations": violations}
    state.clear()
    state.update(draft)
    return None


def state_from_snapshot(snap):
    return {"surface": snap["surface"], "revision": snap["revision"], "root": snap["root"], "nodes": {n["id"]: n for n in snap["nodes"]}}


def check_group(group, has_patch):
    errors = []
    for snap_path in sorted(glob.glob(os.path.join(ROOT, group, "*.snapshot.json"))):
        slug = os.path.basename(snap_path)[: -len(".snapshot.json")]
        with open(snap_path) as f:
            snap = json.load(f)
        expect_path = os.path.join(ROOT, group, f"{slug}.expect.json")
        with open(expect_path) as f:
            exp = json.load(f)
        lim = exp.get("limits") or DEFAULT_LIMITS

        if not has_patch:
            nodes_by_id = {n["id"]: n for n in snap["nodes"]}
            violations = validate_core(snap["root"], nodes_by_id, lim)
            if violations:
                errors.append(f"{group}/{slug}: base snapshot itself failed validate_snapshot: {violations}")
            for exp_a11y in exp.get("accessibility", []):
                got = nodes_by_id.get(exp_a11y["id"], {}).get("accessibility", {})
                got_shape = {"id": exp_a11y["id"], "label": got.get("label"), "description": got.get("description"), "live": got.get("live", "off"), "shortcut": got.get("shortcut"), "hidden": got.get("hidden", False)}
                if got_shape != exp_a11y:
                    errors.append(f"{group}/{slug}: node {exp_a11y['id']} accessibility mismatch:\n  got  {got_shape}\n  want {exp_a11y}")
            got_action_ids = []
            for nid in sorted(nodes_by_id.keys()):
                for b in nodes_by_id[nid].get("bindings", []):
                    aid = b["action"]
                    got_action_ids.append(f"{aid['scope']}.{aid['name']}@{aid['version']}")
            if got_action_ids != exp.get("actionIds", []):
                errors.append(f"{group}/{slug}: actionIds mismatch: got {got_action_ids} want {exp.get('actionIds', [])}")
            exp_tree = exp.get("tree")
            if exp_tree:
                if snap["root"] != exp_tree["root"]:
                    errors.append(f"{group}/{slug}: root mismatch: got {snap['root']} want {exp_tree['root']}")
                if len(snap["nodes"]) != exp_tree["nodeCount"]:
                    errors.append(f"{group}/{slug}: nodeCount mismatch: got {len(snap['nodes'])} want {exp_tree['nodeCount']}")
            continue

        patch_path = os.path.join(ROOT, group, f"{slug}.patch.json")
        with open(patch_path) as f:
            pat = json.load(f)
        state = state_from_snapshot(snap)
        rejection = apply_patch(state, pat, lim)
        expected_outcome = exp["outcome"]
        if expected_outcome == "accept":
            if rejection is not None:
                errors.append(f"{group}/{slug}: expected accept but got rejection {rejection}")
                continue
            exp_tree = exp["tree"]
            if state["root"] != exp_tree["root"]:
                errors.append(f"{group}/{slug}: root mismatch: got {state['root']} want {exp_tree['root']}")
            if len(state["nodes"]) != exp_tree["nodeCount"]:
                errors.append(f"{group}/{slug}: nodeCount mismatch: got {len(state['nodes'])} want {exp_tree['nodeCount']}")
            got_ids = sorted(state["nodes"].keys())
            want_ids = sorted(s["id"] for s in exp_tree["shape"])
            if got_ids != want_ids:
                errors.append(f"{group}/{slug}: node id set mismatch: got {got_ids} want {want_ids}")
            for shape_node in exp_tree["shape"]:
                got = state["nodes"].get(shape_node["id"])
                if got is None:
                    continue
                if got["component"]["type"] != shape_node["type"]:
                    errors.append(f"{group}/{slug}: node {shape_node['id']} type mismatch: got {got['component']['type']} want {shape_node['type']}")
                if got.get("children", []) != shape_node["children"]:
                    errors.append(f"{group}/{slug}: node {shape_node['id']} children mismatch: got {got.get('children', [])} want {shape_node['children']}")
            for exp_a11y in exp.get("accessibility", []):
                got = state["nodes"].get(exp_a11y["id"], {}).get("accessibility", {})
                got_shape = {"id": exp_a11y["id"], "label": got.get("label"), "description": got.get("description"), "live": got.get("live", "off"), "shortcut": got.get("shortcut"), "hidden": got.get("hidden", False)}
                if got_shape != exp_a11y:
                    errors.append(f"{group}/{slug}: node {exp_a11y['id']} accessibility mismatch:\n  got  {got_shape}\n  want {exp_a11y}")
            got_action_ids = []
            for nid in sorted(state["nodes"].keys()):
                for b in state["nodes"][nid].get("bindings", []):
                    aid = b["action"]
                    got_action_ids.append(f"{aid['scope']}.{aid['name']}@{aid['version']}")
            if got_action_ids != exp.get("actionIds", []):
                errors.append(f"{group}/{slug}: actionIds mismatch: got {got_action_ids} want {exp.get('actionIds', [])}")
        else:
            if rejection is None:
                errors.append(f"{group}/{slug}: expected reject but patch applied cleanly")
                continue
            want = exp["patchRejection"]
            if rejection != want:
                errors.append(f"{group}/{slug}: rejection mismatch:\n  got  {rejection}\n  want {want}")
    return errors


if __name__ == "__main__":
    all_errors = []
    all_errors += check_group("🩹️patch", has_patch=True)
    all_errors += check_group("🚫️rejection", has_patch=True)
    for g in ("🧩️component", "🖥️composite", "📐️layout", "♿️accessibility"):
        all_errors += check_group(g, has_patch=False)
    if all_errors:
        print(f"{len(all_errors)} MISMATCHES:")
        for e in all_errors:
            print(" -", e)
    else:
        print("all groups verified clean against the reference simulator")
