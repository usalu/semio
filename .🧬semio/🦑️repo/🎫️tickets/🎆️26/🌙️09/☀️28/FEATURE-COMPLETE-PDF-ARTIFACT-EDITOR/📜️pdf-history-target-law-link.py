"""🔬 Check the staged native target cursor with both actual registered PDF mutation types."""

import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUT = GENERATED / "pdf-history-target-law"
CRATES = ("semio_s_artifact_stdio_pdf_mut_base", "semio_s_artifact_stdio_pdf_mut_family_8", "semio_os_kernel", "semio_framework_value", "serde_json")


def main():
    """🧭 Reuse the exact WASI crate identities and adapt only the orphan-rule test wrapper."""
    cache = Path(sys.argv[1]).resolve()
    native = cache.parents[2] / "prototype" / "build"
    OUT.mkdir(parents=True, exist_ok=True)
    staged = json.loads((GENERATED / "pdf-history-target-overlay.json").read_text())
    for standard in ("14", "17"):
        marker = "4️⃣1.4" if standard == "14" else "7️⃣1.7"
        (row,) = (row for row in staged if marker in row[0] and row[0].endswith("/🧬️schema/🧬️mutations/🦀️.rs"))
        file, before, source = row
        digest = hashlib.sha256(Path(file).read_bytes()).hexdigest()
        assert digest in (before, hashlib.sha256(source.encode()).hexdigest()), file
        logic = source.split("mod history_target {\n", 1)[1].split("\n}\n\n//#endregion", 1)[0]
        logic = logic.split("\n", 1)[1].replace("use super::PdfMutation;", "use super::{Actual, PdfMutation};")
        logic = re.sub(r"    /// 🏠[^\n]*\n.*?(?=    fn conflict_segment\()", "", logic, flags=re.S)
        logic = logic.replace("for PdfMutation {", "for PdfMutation<'_> {").replace("impl PdfMutation {", "impl PdfMutation<'_> {")
        logic = re.sub(r"match self \{(\n            Self::[^\n]* => \[)", lambda match: "match self.0 {" + match[1], logic)
        logic = re.sub(r"(?m)^            Self::(\w+)\((operation|_)\) => \[", r"            Actual::\1(\2) => [", logic)
        assert "Self::MovePage" not in logic and "match self.0 {" in logic
        (OUT / f"target{standard}.rs").write_text(logic)
    law = OUT / "law.rs"
    law.write_bytes((TICKET / "📜️pdf-history-target-law.rs").read_bytes())
    (OUT / "law.json").write_bytes((TICKET / "🔣️pdf-history-target-law.json").read_bytes())
    output = OUT / "law.wasm"
    flags = ["--edition=2021", "--crate-name=pdf_history_target_law", "--target=wasm32-wasip2", str(law), "-o", str(output)]
    metadata = OUT / ("md-" + cache.parents[2].name)
    metadata.mkdir(exist_ok=True)
    directories = sorted(cache.glob("*/*/out"))
    for directory in directories:
        for artifact in directory.glob("*.rmeta"):
            symlink = metadata / artifact.name
            if not symlink.exists():
                try:
                    symlink.symlink_to(artifact)
                except (NotImplementedError, OSError):
                    symlink.write_bytes(artifact.read_bytes())
    flags += ["-L", f"dependency={metadata}"]
    for crate in CRATES:
        (selected,) = cache.glob(f"*/*/out/lib{crate}-*.rlib")
        for suffix in (".rmeta", ".rlib"):
            flags += ["--extern", f"{crate}={selected.with_suffix(suffix)}"]
    for directory in directories + sorted(native.glob("*/*/out")):
        flags += ["-L", f"dependency={directory}"]
    response = OUT / "rustc.rsp"
    response.write_text("\n".join(flags) + "\n")
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
    subprocess.run(["repo", "tools", "exec", "--light", "jco", "--", "run", str(output), "--jco-dir", str(OUT / "jco")], check=True)


if __name__ == "__main__":
    main()
