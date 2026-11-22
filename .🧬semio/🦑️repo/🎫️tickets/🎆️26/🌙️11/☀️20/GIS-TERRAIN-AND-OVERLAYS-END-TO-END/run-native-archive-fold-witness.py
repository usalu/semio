from pathlib import Path
import hashlib
import re
import subprocess
import sys

ticket=Path(__file__).resolve().parent
scratch=ticket/"🗑️generated"
quote_source=Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/📥️fold/🦀️.rs")
source=quote_source.read_text()
quote=source[source.index("fn fold_receiving_quote"):source.index("impl<P,M>")]
(scratch/"fold-quote.rs").write_text(quote)
probe=ticket/"native-archive-fold-witness.rs"
output=scratch/"native-archive-fold-witness"
source=scratch/"native-archive-fold-witness.rs"
source.write_text(probe.read_text())
toolchain=re.search(r'^channel = "([^"]+)"$', Path("rust-toolchain.toml").read_text(),re.M)[1]
libs=scratch/"qr"
rust=["repo","tools","exec","--light","rustup","--","run",toolchain,"rustc","--edition=2024","-L",f"dependency={libs}","-L",f"dependency={scratch/'aw'}","-o",str(output),str(source)]
for name, fingerprint in {
 "semio_os_spr":"c78228f82adacf8a",
 "semio_framework_value":"05431580f111efc8",
 "semio_framework_value_native_owned":"996df75d9ed9fa1b",
 "semio_framework_job":"31261f37020cf87c",
 "semio_framework_job_testing_caller":"3495cdbbce9436b7",
}.items():
 artifact=libs/f"lib{name}-{fingerprint}.rlib"
 assert artifact.is_file(),artifact
 rust.extend(["--extern",f"{name}={artifact}"])
 if artifact.with_suffix(".rmeta").is_file():
  rust.extend(["--extern",f"{name}={artifact.with_suffix('.rmeta')}"])
subprocess.run(rust,check=True)
archive = scratch / "late-profile-terrain.semio-archive"
assert hashlib.sha256(archive.read_bytes()).hexdigest() == "35c7af2c336e32530bb0375d17d437d91584976a900283959a9d5252b2a85c81"
failed = False
for copy in (32768,49152):
 print(f"[DEBUG] copy quantum {copy}",flush=True)
 result=subprocess.run([str(output),str(archive),str(copy)])
 print(f"[DEBUG] copy quantum {copy} result {result.returncode}",flush=True)
    failed |= result.returncode != 0
raise SystemExit(int(failed))
