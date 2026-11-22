import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { DatabaseSync } from "node:sqlite";
import Ajv2020 from "ajv/dist/2020.js";

const read = (path) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const fixture = read("natural-pdf-contract-cases.json");
const schema = read("natural-pdf-contract-cases.schema.json");
const ajv = new Ajv2020({ strict: true });
assert.equal(ajv.validate(schema, fixture), true, JSON.stringify(ajv.errors));
const fields = ["paged", "naturalKey", "naturalContract", "extent", "raw", "previous", "consumed", "output", "items", "work", "complete", "naturalSchema"];
const baseline = Object.fromEntries(fields.map((field, index) => [field, fixture.baseline[index]]));

const db = new DatabaseSync(":memory:");
const oracle = db.prepare(`
    select (
        (:paged = 1 and :naturalKey = 1 and :naturalSchema = 1 and :naturalContract = 1
            and :extent between 1 and :maximum and :raw = 0)
        or (:paged = 0 and :naturalKey = 0 and :extent <= 8388608 and :raw = 0)
    ) and :consumed between :previous and :extent
      and (:complete = 0 or :consumed = :extent)
      and :items <= 8192
      and :output <= case :naturalContract when 1 then 33554432 else 8388608 end
      and (:complete = 1 or :consumed <> :previous or :items > 0 or :output > 0 or :work > 0)
    as accepted
`);
const wireFixture = (value) => JSON.stringify(value);
const tests = fixture.cases.map(({ name, changes, accepted }, index) => {
    const row = { ...baseline, ...changes };
    const parameters = Object.fromEntries(Object.entries(row).map(([field, value]) => [field, Number(value)]));
    assert.equal(Boolean(oracle.get({ ...parameters, maximum: fixture.selectedMaximum }).accepted), accepted, name);
    const key = wireFixture(row.naturalKey ? fixture.naturalTool : fixture.ordinaryTool);
    const schema = wireFixture(row.naturalSchema ? fixture.naturalSchema : fixture.ordinarySchema);
    const contract = row.naturalContract ? "NATURAL_SOURCE_TOOL_CONTRACT" : "ORDINARY";
    return `#[test] fn case_${index}() {
        let contract = ${contract};
        let admitted = if ${row.paged} {
            admits_paged_natural_request(${key}, ${schema}, contract, ${row.raw})
                && admits_paged_input(NATURAL_FILE_PORT, true, ${row.extent}, ${fixture.selectedMaximum}, ${row.raw})
        } else {
            admits_ordinary(${key}, contract, ${row.extent}, ${row.raw})
        };
        let mut worker = Worker {
            decode_progress: NaturalFileDecodeProgress { consumed_input_bytes: ${row.previous}, ..Default::default() },
            byte_length: ${row.extent}, byte_cursor: 0, contract,
        };
        let progress = NaturalFileDecodeProgress {
            consumed_input_bytes: ${row.consumed}, decoded_items: ${row.items},
            owned_output_bytes: ${row.output}, completed_work_units: ${row.work},
        };
        let accepted = admitted && worker.admit_controlled_progress(progress, ${row.complete}, &StepContext { _lifetime: &() }).is_ok();
        assert_eq!(accepted, ${accepted}, ${wireFixture(name)});
    }`;
});
db.close();

writeFileSync(new URL("./🗑️generated/natural-pdf-admission/contract-law.rs", import.meta.url), `
extern crate self as semio_framework;
extern crate self as semio_framework_job;
extern crate self as semio_plugin_decl_app;
pub const NATURAL_FILE_PORT: &str = "artifact:native";
pub struct StepContext<'a> { _lifetime: &'a () }
impl StepContext<'_> {
    fn should_yield(&self) -> bool { false }
    fn is_cancelled(&self) -> bool { false }
}
struct Fault;
struct FaultCode;
impl FaultCode { fn new(_: &str) -> Self { Self } }
enum FaultOrigin { App }
impl Fault { fn new(_: FaultOrigin, _: FaultCode, _: impl AsRef<str>) -> Self { Self } }
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct NaturalFileDecodeProgress {
    consumed_input_bytes: usize, decoded_items: usize, owned_output_bytes: usize, completed_work_units: u64,
}
struct Worker {
    decode_progress: NaturalFileDecodeProgress, byte_length: usize, byte_cursor: usize, contract: ToolExecutionContract,
}
include!("contract-extracted.rs");
fn admits_ordinary(key: &str, contract: ToolExecutionContract, extent: usize, raw: usize) -> bool {
    key != NATURAL_SOURCE_TOOL_ID && extent <= contract.max_raw_wire_bytes && raw == 0
}
#[test] fn factory_contract_is_valid() {
    assert!(ORDINARY.validate().is_ok());
    assert!(NATURAL_SOURCE_TOOL_CONTRACT.validate().is_ok());
    assert_eq!(ORDINARY.max_raw_wire_bytes, 8_388_608);
    assert_eq!(ORDINARY.max_output_bytes, 8_388_608);
    assert_eq!(NATURAL_SOURCE_TOOL_CONTRACT.max_raw_wire_bytes, 1);
    assert_eq!(NATURAL_SOURCE_TOOL_CONTRACT.max_output_bytes, 33_554_432);
    assert_eq!(NATURAL_SOURCE_TOOL_CONTRACT.max_work_units_per_step, 4096);
}
${tests.join("\n")}
`);
console.log(`[DEBUG] Paged natural contract fixture and SQLite agreed on ${tests.length} cases.`);
