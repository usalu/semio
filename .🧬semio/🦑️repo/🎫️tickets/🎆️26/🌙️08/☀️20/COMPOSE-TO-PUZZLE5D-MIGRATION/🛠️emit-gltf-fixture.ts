#!/usr/bin/env bun
/** 🛠️ Renders the six committed files of ONE handcrafted `stdio.gltf` mutation-fixture case from a
 * hand-authored spec. Authoring aid only — it is NOT a test harness: every value it writes was
 * transcribed by hand from that leaf's own `🦠️mutation/🦀️component.rs` + `🔺️diff/🦀️component.rs`
 * oracle, and every assertion body below the seven fixed contract headings is per-leaf text carried
 * verbatim from the spec. Ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION, contract D1. */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

export type Api = "collection" | "gltfdiff" | "leafdiff" | "material" | "createScene" | "deleteScene" | "nodiff";

export type Spec = {
  readonly leaf: string;
  readonly case: string;
  readonly module: string;
  readonly api: Api;
  readonly diffType?: string;
  readonly payloadType: string;
  readonly doc: string;
  readonly before: unknown;
  readonly after: unknown;
  readonly mutation: unknown;
  readonly diff?: unknown;
  readonly outcome: Record<string, unknown>;
  readonly apply: readonly string[];
  readonly inverse: readonly string[];
  readonly delta: readonly string[];
  readonly canonicalDiff: readonly string[];
  /** 🎬️ Extra `use` lines this leaf's assertions need. */
  readonly uses?: readonly string[];
  /** 🧨️ Set when this leaf's diff carries an `Option<Option<T>>` slot that JSON cannot round-trip. */
  readonly diffRoundTripLoss?: string;
};

const ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations";

const json = (value: unknown): string => `${JSON.stringify(value, null, 2)}\n`;

/** 🧬️ The typed entry points each leaf family exposes, read from that family's own Rust. */
const forward = (spec: Spec): string => {
  switch (spec.api) {
    case "material":
      return `let mut snapshot = before();\n    mutation::apply(&mut snapshot, &payload()).expect("${spec.leaf} applies to its committed before-snapshot");`;
    default:
      return `let snapshot = mutation::apply(&payload(), &before()).expect("${spec.leaf} applies to its committed before-snapshot");`;
  }
};

const forwardAttempt = (spec: Spec): string =>
  spec.api === "material"
    ? `let mut snapshot = before();\n    let attempt = mutation::apply(&mut snapshot, &payload());`
    : `let attempt = mutation::apply(&payload(), &before());`;

const outcomeArms = (spec: Spec): string =>
  spec.api === "material"
    ? `        "applied" => {
            attempt.expect("${spec.leaf} declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("${spec.leaf} declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
            assert_eq!(snapshot, before(), "{CASE}: a rejected mutation must leave the snapshot untouched");
        }`
    : `        "applied" => {
            let snapshot = attempt.expect("${spec.leaf} declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("${spec.leaf} declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }`;

/** 🔺️ How this family derives its sparse delta and applies it back. */
const deriveDiff = (spec: Spec): string => {
  switch (spec.api) {
    case "createScene":
      return `diff::derive(&before(), payload().position).expect("${spec.leaf} derives its diff")`;
    case "deleteScene":
      return `diff::derive(&before(), payload().index).expect("${spec.leaf} derives its diff")`;
    default:
      return `diff::derive(&payload(), &before()).expect("${spec.leaf} derives its diff")`;
  }
};

const applyDiff = (spec: Spec): string => {
  switch (spec.api) {
    case "collection":
      return `diff::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot")`;
    case "gltfdiff":
      return `<crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot")`;
    case "leafdiff":
      return `diff::apply(&before(), &decoded).expect("committed diff applies to the before-snapshot")`;
    case "material":
      return `{\n        let mut snapshot = before();\n        decoded.apply(&mut snapshot).expect("committed diff applies to the before-snapshot");\n        snapshot\n    }`;
    case "createScene":
    case "deleteScene":
      return `diff::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot")`;
    default:
      return "unreachable";
  }
};

/** ↩️ How this family reconstructs and replays the inverse. */
const inverseBody = (spec: Spec): string => {
  switch (spec.api) {
    case "collection":
      return `    let inverse = inverse::derive(&payload(), &base).expect("${spec.leaf} inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = inverse::apply_inverse(&inverse, &after).expect("inverse applies to the forward result");`;
    case "gltfdiff":
      return `    let inverse = inverse::derive(&payload(), &base).expect("${spec.leaf} inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&inverse, &after).expect("inverse applies to the forward result");`;
    case "leafdiff":
      return `    let inverse = inverse::derive(&payload(), &base).expect("${spec.leaf} inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = inverse::apply(&after, &inverse).expect("inverse applies to the forward result");`;
    case "material":
      return `    let inverse = inverse::reconstruct(&payload(), &base).expect("${spec.leaf} inverse reconstructs from the exact base");
    let mut restored = base.clone();
    mutation::apply(&mut restored, &payload()).expect("forward applies");
    inverse.apply(&mut restored).expect("inverse applies to the forward result");`;
    case "createScene":
      return `    let inverse = inverse::derive(&base, payload().position).expect("${spec.leaf} inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = inverse::apply(&inverse, &after).expect("inverse applies to the forward result");`;
    case "deleteScene":
      return `    let inverse = inverse::derive(&base, payload().index).expect("${spec.leaf} inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = inverse::apply(&inverse, &after).expect("inverse applies to the forward result");`;
    default:
      return "";
  }
};

const header = (spec: Spec): string => {
  const uses = [
    ...(spec.diffType && spec.api !== "gltfdiff" ? [`use crate::artifacts::gltf::schema::mutations::${spec.module}::diff::${spec.diffType};`] : []),
    `use crate::artifacts::gltf::schema::mutations::${spec.module}::mutation::${spec.payloadType};`,
    spec.api === "nodiff"
      ? `use crate::artifacts::gltf::schema::mutations::${spec.module}::mutation;`
      : `use crate::artifacts::gltf::schema::mutations::${spec.module}::{diff, inverse, mutation};`,
    `use crate::artifacts::gltf::GltfSnapshot;`,
    ...(spec.uses ?? []),
  ];
  return uses.join("\n");
};

export const render = (spec: Spec): string => {
  const CASE = `${spec.leaf}/${spec.case}`;
  const diffType = spec.api === "gltfdiff" ? "crate::artifacts::gltf::schema::diff::GltfDiff" : (spec.diffType ?? "");
  const lines: string[] = [];
  lines.push(`//! 🧪️ \`${spec.leaf}\` fixture — \`${spec.case}\`.`);
  lines.push("//!");
  for (const line of spec.doc.trim().split("\n")) lines.push(`//! ${line}`.trimEnd());
  lines.push("//!");
  lines.push("//! Source of truth is the committed JSON beside this file (contract D1, ticket");
  lines.push("//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this");
  lines.push("//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/");
  lines.push("//! `.patch.semio` encodings come from `fixtures generate`, not from here.");
  lines.push("");
  lines.push(header(spec));
  lines.push("");
  lines.push(`const CASE: &str = "${CASE}";`);
  lines.push('const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");');
  lines.push('const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");');
  lines.push('const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");');
  if (spec.api !== "nodiff") lines.push('const DIFF: &str = include_str!("🔺️diff/🔣️component.json");');
  lines.push('const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");');
  lines.push("");
  lines.push("fn before() -> GltfSnapshot {\n    serde_json::from_str(BEFORE).expect(\"before snapshot decodes\")\n}");
  lines.push("fn expected_after() -> GltfSnapshot {\n    serde_json::from_str(AFTER).expect(\"after snapshot decodes\")\n}");
  lines.push(`fn payload() -> ${spec.payloadType} {\n    serde_json::from_str(MUTATION).expect("${spec.leaf} payload decodes")\n}`);
  lines.push("");

  const applied = spec.outcome.status === "applied";

  lines.push(`/// ▶️ ${spec.apply[0]}`);
  lines.push("#[semio_framework_async_macros::async_test]");
  lines.push("async fn applies_to_committed_after() {");
  if (applied) {
    lines.push(`    ${forward(spec)}`);
    lines.push(`    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");`);
  } else {
    lines.push(`    ${forwardAttempt(spec)}`);
    lines.push(`    assert!(attempt.is_err(), "{CASE}: this payload must be refused, not applied");`);
    lines.push(`    assert_eq!(before(), expected_after(), "{CASE}: a rejected case must commit an after-snapshot identical to its before-snapshot");`);
  }
  for (const line of spec.apply.slice(1)) lines.push(`    ${line}`);
  lines.push("}");
  lines.push("");

  lines.push(`/// ↩️ ${spec.inverse[0]}`);
  lines.push("#[semio_framework_async_macros::async_test]");
  lines.push("async fn inverse_restores_before() {");
  lines.push("    let base = before();");
  if (applied && spec.api !== "nodiff") {
    lines.push(inverseBody(spec));
    lines.push(`    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");`);
  } else if (spec.api === "nodiff") {
    lines.push(`    assert!(mutation::apply(&payload(), &base).is_err(), "{CASE}: nothing was applied, so there is no state for an inverse to undo");`);
  } else {
    lines.push(`    assert!(mutation::apply(&payload(), &base).is_err(), "{CASE}: nothing was applied, so there is no state for an inverse to undo");`);
  }
  for (const line of spec.inverse.slice(1)) lines.push(`    ${line}`);
  lines.push("}");
  lines.push("");

  lines.push("/// 🔣️ Both committed snapshots and this leaf's committed payload are canonical: decode→encode");
  lines.push("/// is a fixed point.");
  lines.push("#[semio_framework_async_macros::async_test]");
  lines.push("async fn committed_json_is_canonical() {");
  lines.push('    for (side, text) in [("before", BEFORE), ("after", AFTER)] {');
  lines.push("        let decoded: GltfSnapshot = serde_json::from_str(text).expect(\"snapshot decodes\");");
  lines.push("        let reencoded = serde_json::to_value(&decoded).expect(\"snapshot encodes\");");
  lines.push("        let original: serde_json::Value = serde_json::from_str(text).expect(\"snapshot reparses\");");
  lines.push('        assert_eq!(reencoded, original, "{CASE}: committed {side} JSON is not canonical");');
  lines.push("    }");
  lines.push("    let reencoded = serde_json::to_value(payload()).expect(\"payload encodes\");");
  lines.push("    let original: serde_json::Value = serde_json::from_str(MUTATION).expect(\"payload reparses\");");
  lines.push('    assert_eq!(reencoded, original, "{CASE}: committed payload JSON is not canonical");');
  lines.push("}");
  lines.push("");

  lines.push("/// 🎯️ The declared outcome — and, when rejected, this leaf's own rejection code — matches what");
  lines.push("/// the mutation actually produces for the committed payload.");
  lines.push("#[semio_framework_async_macros::async_test]");
  lines.push("async fn declared_outcome_holds() {");
  lines.push("    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect(\"outcome decodes\");");
  lines.push("    let status = outcome.get(\"status\").and_then(serde_json::Value::as_str).expect(\"outcome carries a status\");");
  lines.push(`    ${forwardAttempt(spec)}`);
  lines.push("    match status {");
  lines.push(outcomeArms(spec));
  lines.push('        other => panic!("{CASE}: unknown outcome status {other:?}"),');
  lines.push("    }");
  lines.push("}");
  lines.push("");

  if (spec.api === "nodiff") {
    lines.push("/// 🔺️ This leaf ships no `🔺️diff` module at all, so a rejected case is the only honest fixture:");
    lines.push("/// there is no diff type to serialize and `🔺️diff/🚫️component.absent` stands in its place.");
    lines.push("#[semio_framework_async_macros::async_test]");
    lines.push("async fn produces_committed_diff() {");
    for (const line of spec.delta) lines.push(`    ${line}`);
    lines.push("}");
    lines.push("");
    lines.push("/// 🔣️ Nothing to re-encode: the absent-diff marker is the committed artifact.");
    lines.push("#[semio_framework_async_macros::async_test]");
    lines.push("async fn committed_diff_is_canonical() {");
    for (const line of spec.canonicalDiff) lines.push(`    ${line}`);
    lines.push("}");
    lines.push("");
    lines.push("/// 🩹 With no diff there is nothing to replay; the snapshot must be exactly as committed.");
    lines.push("#[semio_framework_async_macros::async_test]");
    lines.push("async fn committed_diff_applies_to_after() {");
    lines.push(`    ${forwardAttempt(spec)}`);
    lines.push(`    assert!(attempt.is_err(), "{CASE}: refused payloads produce no diff to replay");`);
    lines.push(`    assert_eq!(before(), expected_after(), "{CASE}: before and after must stay identical for a refused payload");`);
    lines.push("}");
  } else {
    lines.push(`/// 🔺️ ${spec.delta[0]}`);
    lines.push("#[semio_framework_async_macros::async_test]");
    lines.push("async fn produces_committed_diff() {");
    if (applied) {
      lines.push(`    let produced = ${deriveDiff(spec)};`);
      lines.push("    let encoded = serde_json::to_value(&produced).expect(\"produced diff encodes\");");
      lines.push("    let committed: serde_json::Value = serde_json::from_str(DIFF).expect(\"committed diff decodes\");");
      lines.push('    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️component.json");');
    }
    for (const line of spec.delta.slice(1)) lines.push(`    ${line}`);
    lines.push("}");
    lines.push("");
    lines.push("/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.");
    lines.push("#[semio_framework_async_macros::async_test]");
    lines.push("async fn committed_diff_is_canonical() {");
    lines.push(`    let decoded: ${diffType} = serde_json::from_str(DIFF).expect("committed diff decodes");`);
    if (spec.diffRoundTripLoss) {
      for (const line of spec.canonicalDiff) lines.push(`    ${line}`);
    } else {
      lines.push("    let reencoded = serde_json::to_value(&decoded).expect(\"diff re-encodes\");");
      lines.push("    let original: serde_json::Value = serde_json::from_str(DIFF).expect(\"committed diff reparses\");");
      lines.push('    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");');
      for (const line of spec.canonicalDiff) lines.push(`    ${line}`);
    }
    lines.push("}");
    lines.push("");
    lines.push("/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is");
    lines.push("/// a complete description of the change, not a summary of it.");
    lines.push("#[semio_framework_async_macros::async_test]");
    lines.push("async fn committed_diff_applies_to_after() {");
    lines.push(`    let decoded: ${diffType} = serde_json::from_str(DIFF).expect("committed diff decodes");`);
    if (spec.diffRoundTripLoss) {
      lines.push(`    // ${spec.diffRoundTripLoss}`);
      lines.push(`    let produced = ${deriveDiff(spec)};`);
      lines.push(`    assert_ne!(decoded, produced, "{CASE}: the JSON round trip is expected to LOSE this leaf's Option<Option<_>> slot — if it ever survives, drop this pin and assert equality instead");`);
      lines.push(`    let applied = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&produced, &before()).expect("the TYPED diff applies to the before-snapshot");`);
      lines.push(`    assert_eq!(applied, expected_after(), "{CASE}: the TYPED delta still carries before to after — only its JSON encoding is lossy");`);
    } else {
      lines.push(`    let produced = ${applyDiff(spec)};`);
      lines.push(`    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");`);
    }
    lines.push("}");
  }
  lines.push("");
  return lines.join("\n");
};

export const emit = (repoRoot: string, spec: Spec): void => {
  const dir = join(repoRoot, ROOT, spec.leaf, "🧪️tests", spec.case);
  for (const sub of ["📸️snapshot/⬅️before", "📸️snapshot/➡️after", "🦠️mutation", "🔺️diff", "🎯️outcome"]) mkdirSync(join(dir, sub), { recursive: true });
  writeFileSync(join(dir, "📸️snapshot/⬅️before/🔣️component.json"), json(spec.before));
  writeFileSync(join(dir, "📸️snapshot/➡️after/🔣️component.json"), json(spec.after));
  writeFileSync(join(dir, "🦠️mutation/🔣️component.json"), json(spec.mutation));
  writeFileSync(join(dir, "🎯️outcome/🔣️component.json"), json(spec.outcome));
  if (spec.outcome.status === "rejected") writeFileSync(join(dir, "🔺️diff/🚫️component.absent"), "");
  else writeFileSync(join(dir, "🔺️diff/🔣️component.json"), json(spec.diff));
  writeFileSync(join(dir, "🦀️component.rs"), render(spec));
};
