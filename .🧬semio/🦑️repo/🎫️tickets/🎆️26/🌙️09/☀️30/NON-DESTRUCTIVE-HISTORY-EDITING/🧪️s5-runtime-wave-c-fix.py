"""🩹️ Wave C, test-only repair: the row-withdraw law borrowed the app twice in one call (E0502 ×2 in the plugin TEST target;
the lib was green). Loaded by `🧪️s5-runtime-land.py`.
"""

LAW = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs"

PAIRS = [('    let target = seeded_mutation(&app, 3);\n    let enabled = |verb: &str, label: &str| (verb.to_string(), label.to_string(), false, None);\n', '    let (first, target) = (seeded_mutation(&app, 0), seeded_mutation(&app, 3));\n    let enabled = |verb: &str, label: &str| (verb.to_string(), label.to_string(), false, None);\n'), ('    let blocked = verb(&mut app, &fixture, "historyEditWithdraw", vec![("mutationId".into(), DslValue::String(seeded_mutation(&app, 0)))]).await;\n', '    let blocked = verb(&mut app, &fixture, "historyEditWithdraw", vec![("mutationId".into(), DslValue::String(first.clone()))]).await;\n'), ('    let unaccepted = verb(&mut app, &fixture, "historyEditRestore", vec![("mutationId".into(), DslValue::String(seeded_mutation(&app, 0)))]).await;\n', '    let unaccepted = verb(&mut app, &fixture, "historyEditRestore", vec![("mutationId".into(), DslValue::String(first.clone()))]).await;\n')]


def files(_root):
    return {LAW: PAIRS}
