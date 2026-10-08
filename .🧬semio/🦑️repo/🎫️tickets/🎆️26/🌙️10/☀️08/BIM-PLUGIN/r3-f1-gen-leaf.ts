/**
 * 🧪️ Mutation leaf boilerplate generator. From one `Leaf` spec it writes everything of a leaf except the two hand-written
 * logic files (`🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`): the descriptor, the payload schema, the payload + `MutationKind` file, one
 * test file per case and the fixture quintet per case (`after` and `diff` of applied cases are placeholders until blessed
 * with `BIM_BLESS=1 cargo test`). It also returns the mount lines to paste into the artifact root `🦀️.rs`.
 */
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, em, fixtures, JSONF, mutations, RS, rel } from "./r3-f1-paths.ts";

export type Label = { en: string; de: string };
export type Prop = { name: string; rust: string; schema: Record<string, unknown>; ui: { widget: string; role: string; label: Label; description?: Label; ref?: { kind: string }; group: string; order: number } };
export type Case = { name: string; emoji: number; before: unknown; mutation: Record<string, unknown>; beforeText?: string; mutationText?: string; outcome: { status: "applied" } | { status: "rejected"; code: string; path: string[] } };
export type Leaf = {
  kind: string;
  emoji: number;
  variant: string;
  verb: string;
  entity: string;
  doc: string;
  displayName: string;
  binaryTag: number;
  props: Prop[];
  label: { en: string; de: string };
  target: string;
  inverseRows?: Record<string, unknown>;
  uses?: string[];
  cases: Case[];
};

const writeOnce = (dir: string, leaf: string, body: string) => {
  if (!existsSync(join(dir, leaf))) write(dir, leaf, body);
};
const write = (dir: string, leaf: string, body: string) => {
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, leaf), body);
};
const json = (value: unknown) => JSON.stringify(value, null, 2) + "\n";
export const tagOf = (variant: string) => variant[0].toLowerCase() + variant.slice(1);
const snake = (kind: string) => kind.replaceAll("-", "_");
const NS = "https://json.schemas.assets.semio-tech.com/s/bim/model";

export function emitLeaf(leaf: Leaf): string {
  const leafDirName = em(leaf.emoji) + leaf.kind;
  const leafDir = join(mutations, leafDirName);
  const mod = snake(leaf.kind);
  const required = ["mutation", ...leaf.props.map((p) => p.name)];

  write(leafDir, JSONF, json({
    schemaVersion: 1,
    owner: rel(leafDir),
    semanticKind: leaf.kind,
    displayName: leaf.displayName,
    emoji: em(leaf.emoji),
    aggregateVariant: leaf.variant,
    payloadSchema: `${em(0x1f9ec)}schema/${JSONF}`,
    textOpcode: null,
    binaryTag: leaf.binaryTag,
    invertibility: "explicit-mutation",
    diffParticipation: "detect",
    outcomeClasses: ["applied", "no-op", "rejected"],
    composition: "atomic",
    requiredLanguageSurfaces: ["rust", "json-schema", "text", "binary"],
  }));

  write(join(leafDir, em(0x1f9ec) + "schema"), JSONF, json({
    $schema: "http://json-schema.org/draft-07/schema#",
    $id: `${NS}/mutation/${leaf.kind}/schema.json`,
    title: leaf.variant,
    type: "object",
    additionalProperties: false,
    required,
    ...(leaf.inverseRows ? { "x-semio-inverse-rows": leaf.inverseRows } : {}),
    properties: {
      mutation: { const: tagOf(leaf.variant) },
      ...Object.fromEntries(leaf.props.map((p) => [p.name, { ...p.schema, "x-semio-ui": p.ui }])),
    },
  }));

  const fields = leaf.props.map((p) => `    pub ${p.name}: ${p.rust},`).join("\n");
  write(join(leafDir, em(0x1f9a0) + "mutation"), RS, `//! ${em(leaf.emoji)} \`${leaf.kind}\` payload. ${leaf.doc}

use crate::{${["ModelDiff", "ModelMutation", "ModelSnapshot", ...new Set(leaf.props.map((p) => p.rust).filter((name) => /^[A-Z]\w*$/.test(name) && name !== "String"))].sort().join(", ")}};
use protocol::{MutationKind, SemanticDescriptor};
${(leaf.uses ?? []).map((line) => line + "\n").join("")}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ${leaf.variant} {
${fields}
}

impl MutationKind<ModelSnapshot, ModelMutation> for ${leaf.variant} {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "${leaf.verb}", entity: "${leaf.entity}", kind: "${leaf.kind}", record: "${leaf.variant}" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&${leaf.label.en}, &${leaf.label.de})
    }
    fn target(&self) -> Vec<String> {
        ${leaf.target}
    }
}
`);
  mkdirSync(join(leafDir, em(0x1f53a) + "diff"), { recursive: true });
  mkdirSync(join(leafDir, em(0x21a9) + "inverse"), { recursive: true });

  let mounts = `                        #[path = "."]
                        pub mod ${mod} {
                            #[path = "${relRoot(join(leafDir, em(0x1f9a0) + "mutation", RS))}"]
                            mod component;
                            #[path = "${relRoot(join(leafDir, em(0x1f53a) + "diff", RS))}"]
                            pub mod diff;
                            #[path = "${relRoot(join(leafDir, em(0x21a9) + "inverse", RS))}"]
                            pub mod inverse;
                            pub use component::*;
`;
  const cases = leaf.cases;
  for (const c of cases) mounts += emitCase(leaf.kind, leafDirName, leaf.variant, c);
  mounts += `                        }\n`;
  return mounts;
}

export const relRoot = (path: string) => rel(path).slice(rel(artifact).length + 1);

/**
 * 🧪️ One case of a leaf: its fixture quintet (before, mutation, outcome always; placeholders for after and diff of applied cases until
 * blessed) and its generated test file. Returns the two mount lines to paste into the leaf's `pub mod` block of the artifact root.
 */
export function emitCase(leafKind: string, leafDirName: string, variant: string, c: Case): string {
  const leafDir = join(mutations, leafDirName);
  const leaf = { kind: leafKind, variant };
  let mounts = "";
    const caseDirName = em(c.emoji) + c.name;
    const fixture = join(fixtures, em(0x1f9ec) + "mutations", leafDirName, caseDirName);
    const before = c.beforeText ?? json(c.before);
    write(join(fixture, em(0x1f4f8) + "snapshot", em(0x2b05) + "before"), JSONF, before);
    (c.outcome.status === "rejected" ? write : writeOnce)(join(fixture, em(0x1f4f8) + "snapshot", em(0x27a1) + "after"), JSONF, c.outcome.status === "rejected" ? before : "{}\n");
    write(join(fixture, em(0x1f9a0) + "mutation"), JSONF, c.mutationText ?? json({ mutation: tagOf(leaf.variant), ...c.mutation }));
    writeOnce(join(fixture, em(0x1f53a) + "diff"), JSONF, "{}\n");
    write(join(fixture, em(0x1f3af) + "outcome"), JSONF, json(c.outcome));

    const base = `../../../../../${rel(fixtures).split("/").pop()}/${em(0x1f9ec)}mutations/${leafDirName}/${caseDirName}`;
    const include = (tail: string) => `include_str!("${base}/${tail}")`;
    const testDir = join(leafDir, em(0x1f9ea) + "tests", em(c.emoji) + c.name);
    write(testDir, RS, `//! ${em(c.emoji)} \`${leaf.kind}\` / \`${c.name}\`: ${c.outcome.status}. Source of truth: the committed fixture quintet.

use crate::standards::v1::subsets::any::schema::mutations::kit::{self, Case};

const CASE: Case = Case {
    dir: "${leafDirName}/${caseDirName}",
    before: ${include(`${em(0x1f4f8)}snapshot/${em(0x2b05)}before/${JSONF}`)},
    after: ${include(`${em(0x1f4f8)}snapshot/${em(0x27a1)}after/${JSONF}`)},
    mutation: ${include(`${em(0x1f9a0)}mutation/${JSONF}`)},
    diff: ${include(`${em(0x1f53a)}diff/${JSONF}`)},
    outcome: ${include(`${em(0x1f3af)}outcome/${JSONF}`)},
};

#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    kit::outcome(&CASE);
}

#[semio_framework_async_macros::async_test]
async fn applies_to_the_committed_after_snapshot() {
    kit::applies(&CASE);
}

#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff() {
    kit::produces_diff(&CASE);
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_before_snapshot() {
    kit::inverse_restores(&CASE);
}

#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    kit::canonical(&CASE);
}
${c.outcome.status === "applied" ? `
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&kit::mutation(&CASE), &kit::before(&CASE)).await;
}
` : ""}`);
    mounts += `                            #[cfg(test)]
                            #[path = "${relRoot(join(testDir, RS))}"]
                            mod tests_${snake(c.name)};
`;
  return mounts;
}
