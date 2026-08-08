#!/usr/bin/env bun
/**
 * 🧪 Self-check probe for W2 schema field extractors.
 * No existing unit test file covers root `📜️script.ts` policy helpers — this probe stands in.
 */
import {
  policyExtractRustSchemaFields,
  policyExtractTypescriptSchemaFields,
  policyExtractGraphqlSchemaFields,
  policyExtractJsonSchemaFields,
  policyExtractProtobufSchemaFields,
} from "../../../../../../📜️script.ts";

const rust = `pub struct DemoArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub objects: Vec<DemoObject>,
    #[state(shared_ui)] pub active_object_id: Option<String>,
    #[state(local_ui)] pub tags: BTreeMap<String, String>,
    #[state(preview)] pub origin: [f32; 3],
}`;

const ts = `export interface DemoArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  objects: DemoObject[];
  /** @state shared-ui */
  activeObjectId?: string;
  /** @state local-ui */
  tags: Record<string, string>;
  /** @state preview */
  origin: [number, number, number];
}`;

const gql = `type DemoArtifact {
  schema: String! @state(class: PERSISTENT)
  objects: [DemoObject!]! @state(class: PERSISTENT)
  activeObjectId: String @state(class: SHARED_UI)
  tags: [DemoEntry!]! @state(class: LOCAL_UI)
  origin: [Float!]! @state(class: PREVIEW)
}`;

const json = JSON.stringify({
  title: "DemoArtifact",
  type: "object",
  required: ["schema", "objects", "tags", "origin"],
  properties: {
    schema: { type: "string", "x-semio-state": "persistent" },
    objects: { type: "array", items: { type: "object" }, "x-semio-state": "persistent" },
    activeObjectId: { type: "string", "x-semio-state": "shared-ui" },
    tags: { type: "object", additionalProperties: { type: "string" }, "x-semio-state": "local-ui" },
    origin: { type: "array", items: { type: "number", format: "float" }, minItems: 3, maxItems: 3, "x-semio-state": "preview" },
  },
});

const proto = `message DemoArtifact {
  // @state persistent
  string schema = 1;
  // @state persistent
  repeated DemoObject objects = 2;
  // @state shared-ui
  optional string active_object_id = 3;
  // @state local-ui
  map<string, string> tags = 4;
  // @state preview
  repeated float origin = 5;
}`;

const extracts = [
  ["rust", policyExtractRustSchemaFields(rust)],
  ["ts", policyExtractTypescriptSchemaFields(ts)],
  ["gql", policyExtractGraphqlSchemaFields(gql)],
  ["json", policyExtractJsonSchemaFields(json)],
  ["proto", policyExtractProtobufSchemaFields(proto)],
] as const;

let failed = 0;
for (const [name, ex] of extracts) {
  if (ex.typeName !== "DemoArtifact") {
    console.error(`FAIL ${name}: typeName=${ex.typeName}`);
    failed++;
  }
  if (ex.fields.length !== 5) {
    console.error(`FAIL ${name}: fieldCount=${ex.fields.length}`, ex.fields);
    failed++;
  }
}

const shapeKey = (fields: typeof extracts[0][1]["fields"]) =>
  fields.map((f) => `${f.name}:${f.optional}:${f.cardinality}:${f.state}`).join("|");

const keys = extracts.map(([, ex]) => shapeKey(ex.fields));
// GraphQL maps Entry→map but fixedList vs list: proto/gql cannot express fixedList — json+rust+ts should agree on fixedList for origin;
// gql/proto report list for origin. Assert the three camel/snake-capable strict shapes for name/optional/state parity across all five.
const nameState = (fields: typeof extracts[0][1]["fields"]) =>
  fields.map((f) => `${f.name}:${f.optional}:${f.state}`).join("|");
const ns = extracts.map(([, ex]) => nameState(ex.fields));
if (!ns.every((k) => k === ns[0])) {
  console.error("FAIL name/optional/state parity across formats");
  for (const [name, ex] of extracts) console.error(name, nameState(ex.fields));
  failed++;
}

const jsonEx = extracts.find(([n]) => n === "json")![1];
const rustEx = extracts.find(([n]) => n === "rust")![1];
const tsEx = extracts.find(([n]) => n === "ts")![1];
const originJ = jsonEx.fields.find((f) => f.name === "origin")!;
const originR = rustEx.fields.find((f) => f.name === "origin")!;
const originT = tsEx.fields.find((f) => f.name === "origin")!;
if (originJ.cardinality !== "fixedList" || originR.cardinality !== "fixedList" || originT.cardinality !== "fixedList") {
  console.error("FAIL fixedList on origin for json/rust/ts", originJ.cardinality, originR.cardinality, originT.cardinality);
  failed++;
}
const tagsJ = jsonEx.fields.find((f) => f.name === "tags")!;
const tagsR = rustEx.fields.find((f) => f.name === "tags")!;
const tagsT = tsEx.fields.find((f) => f.name === "tags")!;
const tagsG = extracts.find(([n]) => n === "gql")![1].fields.find((f) => f.name === "tags")!;
const tagsP = extracts.find(([n]) => n === "proto")![1].fields.find((f) => f.name === "tags")!;
if ([tagsJ, tagsR, tagsT, tagsG, tagsP].some((f) => f.cardinality !== "map")) {
  console.error("FAIL map cardinality on tags");
  failed++;
}

if (failed === 0) {
  console.log("PASS extractor self-check");
  console.log("shapes", ...new Set(keys));
  process.exit(0);
}
process.exit(1);
