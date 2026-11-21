"""🧪️ Supply certified browser inventory for synthetic native queue fixtures."""
import hashlib
import importlib.util
from pathlib import Path
import shlex
import sys


TICKET = Path(__file__).resolve().parent


def install_python_shim(directory):
    shim = directory / "python3"
    python, fixture = shlex.quote(sys.executable), shlex.quote(str(Path(__file__).resolve()))
    shim.write_text(f'''#!/usr/bin/env bash
if [[ "${{1##*/}}" == release-receipts.py ]]; then
  shift
  exec {python} {fixture} "$@"
fi
exec {python} "$@"
''')
    shim.chmod(0o755)
    return shim


def synthetic_certification(requests):
    result = []
    for request in requests:
        root = Path(request["stagedReceipt"]).parent
        files = []
        for path in sorted(root.glob("*.wasm")):
            data = path.read_bytes()
            files.append({"path":path.name,"sha256":hashlib.sha256(data).hexdigest(),"bytes":len(data)})
        if not files:
            raise ValueError("missing synthetic optimized browser proof")
        result.append({"version":1,"files":files})
    return result


if __name__ == "__main__":
    if sys.argv[1] == "--shim":
        install_python_shim(Path(sys.argv[2]))
    else:
        spec = importlib.util.spec_from_file_location("release_receipts", TICKET / "release-receipts.py")
        gate = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(gate)
        gate.certify_browser_cores = synthetic_certification
        try:
            gate.main()
        except (OSError, ValueError, KeyError) as error:
            raise SystemExit(str(error))
