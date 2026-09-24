import pathlib

root = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework")


def edit(rel, old, new):
    path = root / rel
    text = path.read_text()
    assert text.count(old) == 1, (rel, text.count(old), old[:100])
    path.write_text(text.replace(old, new))


edit(
    "🔨️modules/🎠️kernel/🦀️.rs",
    "/// 🧵 How many `step-job` observations ONE host admission of a spawned job may take before it gives\n",
    """/// 🧰️ The `Effect::SpawnJob` kind of every framework reserved tool verb (undo, redo, copy/paste,
/// selection and interaction verbs): a live job each host starts on the spawning instance, steps to its
/// end and answers with `Event::JobCompleted`, never a replayable product job. Twin of TypeScript
/// `FRAMEWORK_RESERVED_JOB_KIND` in `🎠️kernel/🟦️.ts`; both are read from `🧫️fixtures/🧵️spawned-job-drive`.
pub const FRAMEWORK_RESERVED_JOB_KIND: &str = "framework.reserved.tool";

/// 🧵 How many `step-job` observations ONE host admission of a spawned job may take before it gives
""",
)

edit(
    "🔨️modules/🎠️kernel/🟦️.ts",
    "/** 🧵 How many `step-job` observations ONE host admission may take. Twin of Rust\n",
    """/** 🧰️ The `spawn-job` kind of every framework reserved tool verb (undo, redo, copy/paste, selection and
 * interaction verbs): a live job each host starts, steps to its end and completes on the spawning
 * instance, never a replayable product job. Twin of Rust `kernel::FRAMEWORK_RESERVED_JOB_KIND`. */
export const FRAMEWORK_RESERVED_JOB_KIND = "framework.reserved.tool";

/** 🧵 How many `step-job` observations ONE host admission may take. Twin of Rust
""",
)

edit(
    "🔨️modules/🎠️kernel/🧫️fixtures/🧵️spawned-job-drive/🔣️.json",
    '  "budget": {\n',
    '  "reservedKind": "framework.reserved.tool",\n  "budget": {\n',
)

edit(
    "🔨️modules/🎠️kernel/🧪️tests/🧵️spawned-job-drive/🦀️.rs",
    "struct DriveFixture {\n    budget: BudgetFixture,\n",
    "struct DriveFixture {\n    #[serde(rename = \"reservedKind\")]\n    reserved_kind: String,\n    budget: BudgetFixture,\n",
)

edit(
    "🔨️modules/🎠️kernel/🧪️tests/🧵️spawned-job-drive/🦀️.rs",
    "    assert_eq!(fixture.budget.deadline_ms, SPAWNED_JOB_DEADLINE_MS);\n}\n",
    "    assert_eq!(fixture.budget.deadline_ms, SPAWNED_JOB_DEADLINE_MS);\n    assert_eq!(fixture.reserved_kind, FRAMEWORK_RESERVED_JOB_KIND);\n}\n",
)

edit(
    "🔨️modules/🎠️kernel/🧪️tests/🧵️spawned-job-drive/🟦️.ts",
    "import {\n  jobPlacementFromWireName,\n",
    "import {\n  FRAMEWORK_RESERVED_JOB_KIND,\n  jobPlacementFromWireName,\n",
)

edit(
    "🔨️modules/🎠️kernel/🧪️tests/🧵️spawned-job-drive/🟦️.ts",
    "  readonly schema: string;\n  readonly budget:",
    "  readonly schema: string;\n  readonly reservedKind: string;\n  readonly budget:",
)

edit(
    "🔨️modules/🎠️kernel/🧪️tests/🧵️spawned-job-drive/🟦️.ts",
    '  assert.equal(fixture.budget.deadlineMs, SPAWNED_JOB_DEADLINE_MS, "deadline");\n',
    '  assert.equal(fixture.budget.deadlineMs, SPAWNED_JOB_DEADLINE_MS, "deadline");\n  assert.equal(fixture.reservedKind, FRAMEWORK_RESERVED_JOB_KIND, "reserved tool job kind");\n',
)

edit(
    "🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    '    pub const FRAMEWORK_RESERVED_JOB_KIND: &str = "framework.reserved.tool";\n',
    "    pub use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;\n",
)
print("ok")
