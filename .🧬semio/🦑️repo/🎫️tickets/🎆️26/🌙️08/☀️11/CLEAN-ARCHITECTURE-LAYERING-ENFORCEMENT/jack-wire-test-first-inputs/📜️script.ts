import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { createHash } from "node:crypto";
import Ajv2020 from "ajv/dist/2020.js";
import JSON5 from "json5";

const root = resolve(import.meta.dir, "../../../../../../../..");
const owner = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime";
const hash = (body: string) => createHash("sha256").update(body).digest("hex");
const capture = (path: string, after: string, scope: string) => {
  const before = existsSync(resolve(root, path)) ? readFileSync(resolve(root, path), "utf8") : null;
  return { path, scope, before, after, beforeHash: before === null ? null : hash(before), afterHash: hash(after), inverse: before };
};
const kinds = ["invalidValue", "canceled", "ownershipLimit", "allocationFailed", "workLimit", "depthLimit", "unsupportedOwner", "invariantViolated"];
const messages = ["invalid value", "abgebrochen — 🌍", "owned\u0000bytes", "allocation\nfailed", "work limit", "depth limit", "unsupported owner", "exact invariant"];
const borrowed = ["borrowed bytes", "Größe — 🌍", "owned\u0000tail"];
const fixture = {
  version: 1,
  closeRefusals: kinds.map((kind, index) => ({ kind, message: messages[index], expected: { origin: "framework", code: kind, severity: "error", message: messages[index], scope: {}, retryable: false } })),
  borrowedFaults: borrowed.map(message => ({ message, expectedBytes: [...new TextEncoder().encode(message)] })),
};
const string = { type: "string" };
const schema = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  $id: "https://semio.tech/schema/s/trinity/jack/wire-runtime/close-refusal-laws",
  type: "object", additionalProperties: false, required: ["version", "closeRefusals", "borrowedFaults"],
  properties: {
    version: { const: 1 },
    closeRefusals: { type: "array", minItems: 8, maxItems: 8, items: { type: "object", additionalProperties: false, required: ["kind", "message", "expected"], properties: {
      kind: { enum: kinds }, message: string,
      expected: { type: "object", additionalProperties: false, required: ["origin", "code", "severity", "message", "scope", "retryable"], properties: { origin: { const: "framework" }, code: { enum: kinds }, severity: { const: "error" }, message: string, scope: { type: "object", additionalProperties: false }, retryable: { const: false } } },
    } } },
    borrowedFaults: { type: "array", minItems: 3, maxItems: 3, items: { type: "object", additionalProperties: false, required: ["message", "expectedBytes"], properties: { message: string, expectedBytes: { type: "array", items: { type: "integer", minimum: 0, maximum: 255 } } } } },
  },
};
const laws = `

struct JackCloseRefusalOwner {
    kind: ValueRefusalKind,
    message: Option<String>,
}

impl semio_framework_value::ErasedSnapshotRetirement for JackCloseRefusalOwner {
    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> Result<semio_framework_value::SnapshotRetirementStep, ValueError> {
        match self.message.take() {
            Some(message) => Err(ValueError::new(self.kind, message)),
            None => Ok(semio_framework_value::SnapshotRetirementStep::Complete),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.message.is_none()
    }
}

fn close_jack_initializer(authority: &mut JackStoreInitializationAuthority) {
    for _ in 0..100_000 {
        match semio_framework_plugin::ArtifactStoreInitializationAuthority::close_step(authority, 1, JACK_OWNED_FIELD_BYTES).expect("Jack initializer close") {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= JACK_OWNED_FIELD_BYTES);
            }
            semio_framework_plugin::PluginCloseStep::Complete => {
                assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(authority));
                return;
            }
            step => panic!("Jack initializer unexpectedly stopped retirement: {step:?}"),
        }
    }
    panic!("Jack initializer did not reach terminal-empty close")
}

fn jack_close_refusal_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🚫️close-refusal/🔣️.json")).expect("closed language-neutral Jack refusal corpus")
}

#[test]
fn jack_initializer_close_preserves_typed_refusal_and_owned_message() {
    use semio_framework_value::ToValue;

    let fixture = jack_close_refusal_fixture();
    for row in fixture["closeRefusals"].as_array().expect("close refusal cases") {
        let kind = match row["kind"].as_str().expect("refusal kind") {
            "invalidValue" => ValueRefusalKind::InvalidValue,
            "canceled" => ValueRefusalKind::Canceled,
            "ownershipLimit" => ValueRefusalKind::OwnershipLimit,
            "allocationFailed" => ValueRefusalKind::AllocationFailed,
            "workLimit" => ValueRefusalKind::WorkLimit,
            "depthLimit" => ValueRefusalKind::DepthLimit,
            "unsupportedOwner" => ValueRefusalKind::UnsupportedOwner,
            "invariantViolated" => ValueRefusalKind::InvariantViolated,
            other => panic!("unknown refusal kind: {other}"),
        };
        let mut authority = empty_jack_initializer(semio_framework_job::OperationId(503), semio_framework_job::Generation(17));
        *authority.active = Some(Box::new(JackCloseRefusalOwner { kind, message: Some(row["message"].as_str().expect("refusal message").to_owned()) }));
        let zero_grant = semio_framework_plugin::ArtifactStoreInitializationAuthority::close_step(&mut authority, 0, JACK_OWNED_FIELD_BYTES);
        let result = semio_framework_plugin::ArtifactStoreInitializationAuthority::close_step(&mut authority, 1, JACK_OWNED_FIELD_BYTES);
        close_jack_initializer(&mut authority);
        drop(authority);
        assert!(matches!(zero_grant.expect("zero grant retains owner"), semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }));
        let fault = result.expect_err("typed retirement refusal");
        let oracle: serde_json::Value = serde_json::from_slice(&semio_framework_diagnostic::encode_fault_bytes(&fault)).expect("independent JSON fault oracle");
        assert_eq!(oracle, row["expected"]);
        assert_eq!(fault.to_value(), row["expected"]);
        eprintln!("[DEBUG] Jack typed close refusal {} retains exact message and retires every owner", row["kind"]);
    }
}

#[test]
fn jack_initializer_fault_copies_short_borrow_into_owned_bytes() {
    let fixture = jack_close_refusal_fixture();
    for row in fixture["borrowedFaults"].as_array().expect("borrowed fault cases") {
        let mut authority = empty_jack_initializer(semio_framework_job::OperationId(504), semio_framework_job::Generation(18));
        {
            let message = row["message"].as_str().expect("borrowed message").to_owned();
            authority.fail(message.as_bytes());
        }
        let phase = authority.phase;
        let actual = authority.fault.clone();
        close_jack_initializer(&mut authority);
        drop(authority);
        assert_eq!(phase, JackStoreInitializationPhase::RetireFault);
        let oracle: Vec<u8> = serde_json::from_value(row["expectedBytes"].clone()).expect("independent byte-array oracle");
        assert_eq!(actual.as_deref(), Some(oracle.as_slice()));
        eprintln!("[DEBUG] Jack borrowed fault retains {} exact owned bytes after source drops", oracle.len());
    }
}
`;

const command = process.argv[2];
if (command === "prepare-production") {
  const path = `${owner}/🦀️.rs`, before = readFileSync(resolve(root, path), "utf8");
  const spans = [
    { before: "fn fail(&mut self, code: &'static [u8])", after: "fn fail(&mut self, code: &[u8])" },
    { before: "Err(error) => Err(semio_framework_plugin::Fault::from(error)),", after: "Err(error) => Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, error.kind.as_str(), error.into_message()))," },
  ];
  let after = before;
  for (const span of spans) {
    if (after.split(span.before).length !== 2) throw new Error("production span is absent or ambiguous");
    after = after.replace(span.before, span.after);
  }
  const output = resolve(import.meta.dir, `root-two-production-candidate-${process.argv[3] ?? "1"}.json`);
  if (existsSync(output)) throw new Error("production authority already exists");
  writeFileSync(output, JSON.stringify({ version: 1, state: "staged-unpublished-native-baseline-pending", root, rows: [{ ...capture(path, after, "two-exact-error-lifetime-spans"), spans }] }, null, 2) + "\n");
  console.log("[DEBUG] staged two exact wire production spans; native baseline and test-only RED pending; no source writes");
} else if (["mount-tests", "mount-production", "preview-tests", "preview-production"].includes(command ?? "")) {
  const preview = command?.startsWith("preview-");
  if (!process.argv[3] || !process.argv[4] || !preview && process.argv[5] !== "--native-released") throw new Error("explicit candidate, unique journal epoch and Native source checkpoint required");
  const kind = command?.endsWith("-tests") ? "three-test" : "two-production";
  const authority = JSON.parse(readFileSync(resolve(import.meta.dir, `root-${kind}-candidate-${process.argv[3]}.json`), "utf8"));
  const output = resolve(import.meta.dir, `../🗑️generated/jack-wire-test-first/${preview ? "preview" : "publication"}-${kind}-${process.argv[4]}`);
  if (existsSync(output)) throw new Error("publication journal already exists");
  const rows = authority.rows.map((row: { path: string; after: string; append?: string; spans?: { before: string; after: string }[] }) => {
    const before = existsSync(resolve(root, row.path)) ? readFileSync(resolve(root, row.path), "utf8") : null;
    let after = row.after;
    if (row.append) {
      if (before === null || before.includes("fn jack_initializer_close_preserves_typed_refusal_and_owned_message")) throw new Error("test prefix is absent or appended laws already exist");
      after = before + row.append;
    } else if (row.spans) {
      if (before === null) throw new Error("production owner absent");
      after = before;
      for (const span of row.spans) {
        if (after.split(span.before).length !== 2) throw new Error("current production span is absent or ambiguous");
        after = after.replace(span.before, span.after);
      }
    } else if (before !== null) {
      throw new Error("new schema or corpus already exists");
    }
    let inverse = row.append ? after.slice(0, -row.append.length) : before;
    if (row.spans) {
      inverse = after;
      for (const span of [...row.spans].reverse()) {
        if (inverse.split(span.after).length !== 2) throw new Error("authored inverse span is absent or ambiguous");
        inverse = inverse.replace(span.after, span.before);
      }
    }
    if (inverse !== before) throw new Error("full current scoped inverse differs");
    return { ...row, before, after, inverse, inverseExact: true, beforeHash: before === null ? null : hash(before), afterHash: hash(after) };
  });
  mkdirSync(output, { recursive: true });
  if (preview) {
    writeFileSync(resolve(output, "preview.json"), JSON.stringify({ state: "current-scoped-preview-native-checkpoint-pending", sourceWrites: 0, rows }, null, 2) + "\n");
    console.log(`[DEBUG] previewed ${rows.length} ${kind} rows with exact full current inverses; no source writes`);
    process.exit(0);
  }
  const journal: { state: string; rows: unknown[] } = { state: "publishing", rows: [] };
  const save = () => writeFileSync(resolve(output, "publication.json"), JSON.stringify(journal, null, 2) + "\n");
  save();
  for (const row of rows) {
    const current = existsSync(resolve(root, row.path)) ? readFileSync(resolve(root, row.path), "utf8") : null;
    if (current !== row.before) { journal.state = "refused-current-predecessor"; save(); throw new Error("current row advanced before write"); }
    writeFileSync(resolve(output, `row-${journal.rows.length + 1}.json`), JSON.stringify(row, null, 2) + "\n");
    mkdirSync(dirname(resolve(root, row.path)), { recursive: true });
    writeFileSync(resolve(root, row.path), row.after);
    const exact = readFileSync(resolve(root, row.path), "utf8") === row.after;
    journal.rows.push({ path: row.path, beforeHash: row.beforeHash, afterHash: row.afterHash, inverseExact: true, immediateExact: exact });
    save();
    if (!exact) { journal.state = "refused-immediate-guard"; save(); throw new Error("current row advanced after write"); }
  }
  journal.state = "published-current-row-guards-exact";
  save();
  console.log(`[DEBUG] published ${rows.length} exact scoped ${kind} rows; complete before/after/inverse frames retained`);
} else if (command === "validate") {
  const authority = JSON.parse(readFileSync(resolve(import.meta.dir, `root-three-test-candidate-${process.argv[3] ?? "3"}.json`), "utf8"));
  const proposedSchema = JSON.parse(authority.rows[0].after), corpus = JSON.parse(authority.rows[1].after);
  const { validateJsonSchemaSubset } = await import(resolve(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
  const thirdParty = new Ajv2020({ strict: true }).compile(proposedSchema);
  const admit = (value: unknown) => ({ firstParty: validateJsonSchemaSubset(proposedSchema, value).length === 0, thirdParty: thirdParty(value) });
  const clone = () => structuredClone(corpus);
  const hostiles = [
    Object.assign(clone(), { unknown: true }),
    (() => { const value = clone(); value.closeRefusals[0].unknown = true; return value; })(),
    (() => { const value = clone(); value.closeRefusals[0].expected.unknown = true; return value; })(),
    (() => { const value = clone(); value.closeRefusals[0].expected.scope.unknown = true; return value; })(),
    (() => { const value = clone(); value.borrowedFaults[0].unknown = true; return value; })(),
    Object.assign(clone(), { version: 2 }),
    (() => { const value = clone(); value.closeRefusals[0].kind = "unknown"; return value; })(),
    (() => { const value = clone(); value.closeRefusals.pop(); return value; })(),
    (() => { const value = clone(); value.borrowedFaults[0].expectedBytes[0] = 256; return value; })(),
    (() => { const value = clone(); value.borrowedFaults[0].expectedBytes[0] = -1; return value; })(),
    (() => { const value = clone(); value.borrowedFaults[0].expectedBytes[0] = 0.5; return value; })(),
  ];
  const positive = admit(corpus), refused = hostiles.map(admit);
  if (!positive.firstParty || !positive.thirdParty || refused.some(row => row.firstParty || row.thirdParty)) throw new Error("closed schema admission or hostile refusal disagrees");
  const parity = JSON.stringify(JSON5.parse(authority.rows[1].after)) === JSON.stringify(corpus);
  const byteRows = corpus.borrowedFaults.map((row: { message: string; expectedBytes: number[] }) => ({ message: row.message, actual: [...Buffer.from(row.message, "utf8")], expected: row.expectedBytes }));
  const exactKinds = corpus.closeRefusals.map((row: { kind: string }) => row.kind);
  if (!parity || JSON.stringify(exactKinds) !== JSON.stringify(kinds) || byteRows.some((row: { actual: number[]; expected: number[] }) => JSON.stringify(row.actual) !== JSON.stringify(row.expected))) throw new Error("independent corpus identity or UTF-8 byte oracle disagrees");
  if (!authority.rows[2].after.endsWith(authority.rows[2].append) || authority.rows[2].after.slice(0, -authority.rows[2].append.length) !== authority.rows[2].before) throw new Error("original unit body inverse differs");
  const include = /include_str!\("([^"]+)"\)/.exec(authority.rows[2].append)?.[1];
  if (!include || resolve(root, dirname(authority.rows[2].path), include) !== resolve(root, authority.rows[1].path)) throw new Error("unit fixture include does not resolve to the exact authored corpus");
  const output = resolve(import.meta.dir, "../🗑️generated/jack-wire-test-first");
  mkdirSync(output, { recursive: true });
  const proof = { state: "staged-source-ready-native-unexecuted", positive, hostileCount: refused.length, refused, independentJson5Parity: parity, typedRefusalKinds: exactKinds, independentNodeUtf8: byteRows, originalUnitBodyInverseExact: true, exactUnitFixtureResolution: true, sourceWrites: 0 };
  writeFileSync(resolve(output, `root-three-staged-schema-proof-${process.argv[3] ?? "3"}.json`), JSON.stringify(proof, null, 2) + "\n");
  console.log("[DEBUG] staged closed corpus: first-party/Ajv admission, eleven hostile refusals, JSON5 parity, eight stable kinds and three independent UTF-8 byte outputs; native unexecuted");
} else if (command === "prepare") {
  const unit = `${owner}/🧪️tests/🔬️unit/🦀️.rs`;
  const current = readFileSync(resolve(root, unit), "utf8");
  if (current.includes("fn jack_initializer_close_preserves_typed_refusal_and_owned_message")) throw new Error("Jack laws already mounted");
  const rows = [
    capture(`${owner}/🧬️schema/🚫️close-refusal/🔣️.json`, JSON.stringify(schema, null, 2) + "\n", "closed-test-schema"),
    capture(`${owner}/🧫️fixtures/🚫️close-refusal/🔣️.json`, JSON.stringify(fixture, null, 2) + "\n", "language-neutral-test-corpus"),
    { ...capture(unit, current + laws, "append-only-two-owner-laws"), append: laws },
  ];
  if (rows.slice(0, 2).some(row => row.before !== null)) throw new Error("refusal asset already exists");
  const output = resolve(import.meta.dir, `root-three-test-candidate-${process.argv[3] ?? "1"}.json`);
  if (existsSync(output)) throw new Error("candidate authority already exists");
  writeFileSync(output, JSON.stringify({ version: 1, state: "staged-unpublished", root, rows }, null, 2) + "\n");
  console.log("[DEBUG] staged three test-first rows, eight typed refusals and three owned-byte cases; no source writes");
} else {
  throw new Error("unknown Jack wire ticket command");
}
