#!/usr/bin/env python3
"""🗺️ S5-STORE: the gismap store initializer's retirement pumps answer `ValueError` like the retirements they drive (the
peer value refactor moved every `close_step` of that file to `ValueError`; `pump_active` / `pump_terminal_retirement` still
answered `String`, 4 × E0277). Writer's pattern: the String refusals become `InvariantViolated`, the two fault sinks take
`into_message()`. Region `🔖️RetainedStoreInitialization` only. Idempotent; `--check` writes nothing.

    python3 🧪️s5-store-gismap-initializer-value-error.py [--check]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
FILE = ROOT / "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"
REFUSALS = [
    "GIS store initializer retirement exceeded its exact grant",
    "GIS store initializer retirement reported a false terminal",
    "GIS initialization runtime reported a false terminal",
    "GIS snapshot clone reported a false terminal",
    "GIS initialization envelope retirement reported a false terminal",
]
EDITS = [("    fn pump_active(&mut self) -> Result<bool, String> {\n", "    fn pump_active(&mut self) -> Result<bool, semio_framework_value::ValueError> {\n"), ("    fn pump_terminal_retirement(&mut self) -> Result<bool, String> {\n", "    fn pump_terminal_retirement(&mut self) -> Result<bool, semio_framework_value::ValueError> {\n")]
EDITS += [(f'Err("{text}".into())', f'Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "{text}"))') for text in REFUSALS]
SINKS = [
    "        if let Err(error) = self.pump_active() {\n            self.fault = Some(error.into_bytes());\n",
    "                Err(error) => {\n                    self.fault = Some(error.into_bytes());\n                    semio_framework_job::StepOutcome::Yield\n                }\n            },\n            GisMapStoreInitializationPhase::Complete =>",
]


def main():
    text = FILE.read_text(encoding="utf-8")
    start, end = text.index("//#region 🔖️RetainedStoreInitialization"), text.index("//#endregion 🔖️RetainedStoreInitialization")
    region = text[start:end]
    pending = []
    for old, new in EDITS:
        if new in region:
            continue
        if region.count(old) != 1:
            raise SystemExit(f"anchor occurs {region.count(old)} times in the region (expected 1): {old[:60]}")
        region = region.replace(old, new)
        pending.append(old.strip()[:40])
    for sink in SINKS:
        fixed = sink.replace("error.into_bytes()", "error.into_message().into_bytes()")
        if fixed in region:
            continue
        if region.count(sink) != 1:
            raise SystemExit(f"a fault sink occurs {region.count(sink)} times in the region (expected 1)")
        region = region.replace(sink, fixed)
        pending.append("fault sink")
    print("pending: " + (", ".join(pending) if pending else "none"))
    if "--check" in sys.argv[1:] or not pending:
        return
    FILE.write_text(text[:start] + region + text[end:], encoding="utf-8")
    print("applied")


main()
