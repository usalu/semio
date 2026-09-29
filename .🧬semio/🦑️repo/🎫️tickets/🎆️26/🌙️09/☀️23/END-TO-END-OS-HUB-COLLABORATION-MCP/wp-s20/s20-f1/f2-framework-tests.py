"""🧯️ S20 F2 (faults overlay): framework test sources after `From<&str/String> for Fault` and `FaultCode::new(String)` are
gone — a test app's code-like refusal is `app_fault("…")`, a fixture invariant is a framework fault
(`test.fixture-invariant`, the sentence as its message), a String error becomes that fault, fixture-read codes are
`FaultCode::received`. Idempotent. Usage: python3 f2-framework-tests.py <root>"""
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
SDK = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
FILES = [
    SDK / "🧪️tests/🧩️composition/🦀️.rs",
    SDK / "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs",
    SDK / "🧪️tests/⏳️completion/🦀️.rs",
    SDK / "🧪️tests/♻️publication-retirement-authority/🦀️.rs",
    SDK / "🧪️tests/🧬️mutation-fixtures-no-state/🦀️.rs",
]
INVARIANT = 'crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("test.fixture-invariant"), {})'
CODE_LIKE = re.compile(r'(?<![\w:])(?:[A-Za-z_]\w*::)*Fault::from\(\s*"([a-z0-9]+(?:[.\-/][a-z0-9]+)+)"\s*\)')
SENTENCE = re.compile(r'(?<![\w:])(?:[A-Za-z_]\w*::)*Fault::from\(\s*("(?:[^"\\]|\\.)*")\s*\)')
FORMAT = re.compile(r'(?<![\w:])(?:[A-Za-z_]\w*::)*Fault::from\((format!\((?:[^()]|\([^()]*\))*\))\)')
VALUE = re.compile(r'(?<![\w:])(?:[A-Za-z_]\w*::)*Fault::from\(\s*([a-z_][a-z0-9_]*)\s*\)')
PATH = re.compile(r'\.map_err\(\s*(?:[A-Za-z_]\w*::)*Fault::from\s*\)')
EDITS = [
    (SDK / "🧪️tests/⏳️completion/🦀️.rs", 'Some(Fault::from(format!("restart publication fault: {}", String::from_utf8_lossy(page.bytes())))),',
     'Some(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("test.fixture-invariant"), format!("restart publication fault: {}", String::from_utf8_lossy(page.bytes())))),'),
    (SDK / "🖥️host/🧪️tests/🔬️standalone/🦀️.rs", "fn host_fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {\n    let code = code.into();\n    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code), message))",
     "fn host_fault_bytes(code: &'static str, message: impl Into<String>) -> Vec<u8> {\n    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code), message))"),
    (SDK / "🖥️host/🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs", 'semio_framework::FaultCode::new(row["code"].as_str().unwrap())', 'semio_framework::FaultCode::received(row["code"].as_str().unwrap().to_string())'),
    (SDK / "🧪️tests/🔬️tool-run/🦀️.rs", 'assert_eq!(fault.code, FaultCode::new(text(&fixture["busy"]["code"])));', 'assert_eq!(fault.code.as_str(), text(&fixture["busy"]["code"]));'),
]


def main() -> None:
    changed = 0
    for path in FILES:
        text = path.read_text()
        new = CODE_LIKE.sub(lambda match: f'crate::app_fault("{match.group(1)}")', text)
        new = SENTENCE.sub(lambda match: INVARIANT.format(match.group(1)), new)
        new = FORMAT.sub(lambda match: INVARIANT.format(match.group(1)), new)
        new = VALUE.sub(lambda match: INVARIANT.format(match.group(1)), new)
        new = PATH.sub(lambda match: ".map_err(|reason| " + INVARIANT.format("reason") + ")", new)
        if new != text:
            path.write_text(new)
            changed += 1
    for path, before, after in EDITS:
        text = path.read_text()
        if after in text:
            continue
        if text.count(before) != 1:
            sys.exit(f"CONFLICT {path.name}: {before[:60]}")
        path.write_text(text.replace(before, after))
        changed += 1
    left = sum(len(re.findall(r"Fault::from\b", path.read_text())) for path in FILES)
    print(f"framework tests: {changed} files changed, {left} Fault::from left")


main()
