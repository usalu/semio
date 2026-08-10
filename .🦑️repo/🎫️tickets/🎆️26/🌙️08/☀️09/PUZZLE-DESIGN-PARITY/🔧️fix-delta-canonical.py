from pathlib import Path

root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
art2d = next(p for p in (root / '🗿️artifacts').iterdir() if p.name.endswith('2d'))
mut = art2d / '🧬️mutations' / '🦀️component.rs'
text = mut.read_text()

old = '''pub fn puzzle2d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle2dMutation> {
    if before == after {
        return Vec::new();
    }
    let fallback = |after: &Value| vec![Puzzle2dMutation::SetSnapshot { snapshot: serde_json::from_value(after.clone()).unwrap_or_default() }];
    let (Some(before_object), Some(after_object)) = (before.as_object(), after.as_object()) else {
        return fallback(after);
    };'''

new = '''fn canonicalize_puzzle2d_fixture_collections(value: &Value) -> Value {
    let mut next = value.clone();
    let Some(object) = next.as_object_mut() else {
        return next;
    };
    for (key, map_item) in [("nodes", "node"), ("edges", "edge")] {
        let _ = map_item;
        let Some(entries) = object.get_mut(key).and_then(Value::as_array_mut) else {
            continue;
        };
        for entry in entries.iter_mut() {
            let rewritten = if key == "nodes" {
                serde_json::from_value::<Puzzle2dNode>(entry.clone()).ok().and_then(|node| serde_json::to_value(node).ok())
            } else {
                serde_json::from_value::<Puzzle2dEdge>(entry.clone()).ok().and_then(|edge| serde_json::to_value(edge).ok())
            };
            if let Some(rewritten) = rewritten {
                *entry = rewritten;
            }
        }
    }
    next
}

/// 🧮️ Computes the granular typed operation sequence turning `before` into `after` (both the bare
/// fixture JSON `puzzle-plugin` mutates). Node/edge arrays are canonicalized through the typed
/// `Puzzle2dNode`/`Puzzle2dEdge` shape first so sparse fixture writes (missing `anchor`, connection
/// defaults, …) still emit `SetNode`/`SetEdge` instead of falling back to a clobbering `SetSnapshot`.
pub fn puzzle2d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle2dMutation> {
    let before = canonicalize_puzzle2d_fixture_collections(before);
    let after = canonicalize_puzzle2d_fixture_collections(after);
    if before == after {
        return Vec::new();
    }
    let fallback = |after: &Value| vec![Puzzle2dMutation::SetSnapshot { snapshot: serde_json::from_value(after.clone()).unwrap_or_default() }];
    let (Some(before_object), Some(after_object)) = (before.as_object(), after.as_object()) else {
        return fallback(&after);
    };'''

# The old docstring sits above the fn - need to avoid duplicating docstring
# Find and replace carefully - the existing docstring is BEFORE pub fn
# So replace from the docstring through the start of the function body differently.

doc_and_fn_start = '''/// 🧮️ Computes the granular typed operation sequence turning `before` into `after` (both the bare
/// fixture JSON `puzzle-plugin` mutates). Node/edge arrays diff per element id; meta becomes
/// `SetMeta`. Falls back to a single `SetSnapshot` whenever the granular replay would not reproduce
/// `after` exactly (reorders, id-less entries, malformed entries, unrecognized top-level keys,
/// schema changes) — so the emitted operations are always exact while staying granular for the
/// common edits. The camera is deliberately not a known key: it is session-only
/// `Puzzle2dPlayRuntime` state (see `setCamera`'s `ActionKind::View`), never persisted on the
/// document, so a fixture must never carry a top-level `"camera"` key at all.
pub fn puzzle2d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle2dMutation> {
    if before == after {
        return Vec::new();
    }
    let fallback = |after: &Value| vec![Puzzle2dMutation::SetSnapshot { snapshot: serde_json::from_value(after.clone()).unwrap_or_default() }];
    let (Some(before_object), Some(after_object)) = (before.as_object(), after.as_object()) else {
        return fallback(after);
    };'''

replacement = '''fn canonicalize_puzzle2d_fixture_collections(value: &Value) -> Value {
    let mut next = value.clone();
    let Some(object) = next.as_object_mut() else {
        return next;
    };
    for key in ["nodes", "edges"] {
        let Some(entries) = object.get_mut(key).and_then(Value::as_array_mut) else {
            continue;
        };
        for entry in entries.iter_mut() {
            let rewritten = if key == "nodes" {
                serde_json::from_value::<Puzzle2dNode>(entry.clone()).ok().and_then(|node| serde_json::to_value(node).ok())
            } else {
                serde_json::from_value::<Puzzle2dEdge>(entry.clone()).ok().and_then(|edge| serde_json::to_value(edge).ok())
            };
            if let Some(rewritten) = rewritten {
                *entry = rewritten;
            }
        }
    }
    next
}

/// 🧮️ Computes the granular typed operation sequence turning `before` into `after` (both the bare
/// fixture JSON `puzzle-plugin` mutates). Node/edge arrays are canonicalized through the typed
/// `Puzzle2dNode`/`Puzzle2dEdge` shape first so sparse fixture writes (missing `anchor`, connection
/// defaults, …) still emit `SetNode`/`SetEdge` instead of falling back to a clobbering `SetSnapshot`.
/// Node/edge arrays then diff per element id; meta becomes `SetMeta`. Falls back to a single
/// `SetSnapshot` whenever the granular replay would not reproduce `after` exactly (reorders,
/// id-less entries, malformed entries, unrecognized top-level keys, schema changes) — so the
/// emitted operations are always exact while staying granular for the common edits. The camera is
/// deliberately not a known key: it is session-only `Puzzle2dPlayRuntime` state (see `setCamera`'s
/// `ActionKind::View`), never persisted on the document, so a fixture must never carry a top-level
/// `"camera"` key at all.
pub fn puzzle2d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle2dMutation> {
    let before = canonicalize_puzzle2d_fixture_collections(before);
    let after = canonicalize_puzzle2d_fixture_collections(after);
    if before == after {
        return Vec::new();
    }
    let fallback = |after: &Value| vec![Puzzle2dMutation::SetSnapshot { snapshot: serde_json::from_value(after.clone()).unwrap_or_default() }];
    let (Some(before_object), Some(after_object)) = (before.as_object(), after.as_object()) else {
        return fallback(&after);
    };'''

if doc_and_fn_start not in text:
    raise SystemExit('target block not found')
text = text.replace(doc_and_fn_start, replacement, 1)

# Also fix later `return fallback(after)` that now need `&after` since after is owned Value
# Careful: only inside this function. Count occurrences after the fn.
# The function uses `return fallback(after)` in several places - after is now Value not &Value
# fallback takes &Value in my new signature as `|after: &Value|` - so call sites need `&after` when after is Value, OR change fallback to take Value.

# Actually I used `fallback(&after)` only in the first else. Other sites still say `fallback(after)` which would be wrong type (Value vs &Value) - wait, originally after was &Value so fallback(after) passed &Value. Now after is Value, so fallback(after) would need fallback to accept Value by reference: fallback(&after).

# Change all `return fallback(after)` inside the function to `return fallback(&after)`.
# And `fallback(after)` at the end of if &replay == after.

idx = text.find('pub fn puzzle2d_document_delta_operations')
end = text.find('//#endregion 🔖️ValueBridge', idx)
fn = text[idx:end]
fn2 = fn.replace('return fallback(after);', 'return fallback(&after);')
fn2 = fn2.replace('fallback(after)', 'fallback(&after)')  # for final else
# but careful not to double-replace already &after
fn2 = fn2.replace('fallback(&&after)', 'fallback(&after)')
text = text[:idx] + fn2 + text[end:]

mut.write_text(text)
print('mutations patched')

# Fix add_node_to_fixture anchor
app2d = next(p for p in next(root.glob('*apps*')).iterdir() if p.name.endswith('2d'))
comp = next(app2d.glob('*component.rs'))
ct = comp.read_text()
old_node = '''    let mut node = json!({
        "id": id,
        "nodeKind": node_kind,
        "shape": shape,
        "x": x,
        "y": y,
        "text": id,
        "handles": []
    });'''
new_node = '''    let mut node = json!({
        "id": id,
        "nodeKind": node_kind,
        "shape": shape,
        "x": x,
        "y": y,
        "text": id,
        "anchor": "fixed",
        "handles": []
    });'''
if old_node not in ct:
    raise SystemExit('add_node block not found')
ct = ct.replace(old_node, new_node, 1)

# Restore converge test to clean original
import re
pat = r'    fn two_instances_converge_disjoint_node_edits_via_backbone\(\) \{.*?\n    \}'
orig = '''    fn two_instances_converge_disjoint_node_edits_via_backbone() {
        let mut instance_a = app();
        let mut instance_b = app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle2d-convergence", "mem://puzzle2d-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        dispatch(&mut instance_a, "addNode", Some(&json!({ "kind": "seed" })), None).expect("a adds node");
        dispatch(&mut instance_b, "addNode", Some(&json!({ "kind": "other" })), None).expect("b adds node");

        // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
        dispatch(&mut instance_a, "commitCheckpoint", None, None).expect("pump a");
        dispatch(&mut instance_b, "commitCheckpoint", None, None).expect("pump b");

        assert_eq!(fixture_nodes(&fixture_of(&instance_a)).len(), 2, "instance A must contain both nodes");
        assert_eq!(fixture_nodes(&fixture_of(&instance_b)).len(), 2, "instance B must contain both nodes");
    }'''
ct2, n = re.subn(pat, orig, ct, count=1, flags=re.S)
print('test restore', n)
if n != 1:
    raise SystemExit('test not restored')
comp.write_text(ct2)
print('app patched', comp)
