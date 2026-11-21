"""🔎 Link the staged metadata Details law against the actual native editor dependency family."""

import json
import os
import re
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
STAGE = TICKET / "🗑️generated" / "snapshot-read-only"
TREE = STAGE / "q"
LIVE = Path("✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details")
CACHE = Path(sys.argv[1])
DIRECT = (
    "semio_os_kernel", "pack", "semio_framework_pack_json",
    "semio_plugin", "semio_framework_schema", "semio_framework_schema_registry",
    "semio_ui_contract", "semio_ui_locale", "semio_framework_value",
    "semio_s_artifact_stdio_contract_editing", "semio_s_artifact_stdio_contract_window_kit",
    "semio_s_artifact_stdio_contract_wire", "semio_ui_scene", "serde_json",
)


def linked():
    """📦 Match each direct dependency to the retained original native window and editor witnesses."""
    libraries = {p.stem.removeprefix("lib"): p for p in CACHE.glob("*/*/out/*.rlib")}
    anchors = {name: next(p for key, p in libraries.items() if key.rsplit("-", 1)[0] == name)
               for name in DIRECT if name == "semio_plugin" or name.startswith("semio_s_artifact_stdio")}
    expected = {}
    for path in anchors.values():
        for key, witness in json.loads(path.with_suffix(".rmeta.semio-deps").read_text())["dependencies"]:
            name = key.rsplit("-", 1)[0]
            if name in DIRECT and key in libraries:
                assert name not in expected or expected[name] == (key, witness), name
                expected[name] = key, witness
    result = {}
    for name in DIRECT:
        if name in anchors:
            result[name] = anchors[name]
        elif name in expected:
            key, wanted = expected[name]
            found = libraries[key]
            assert json.loads(found.with_suffix(".rmeta.semio-deps").read_text())["svh"] == wanted
            result[name] = found
        else:
            candidates = [p for key,p in libraries.items() if key.rsplit("-",1)[0] == name]
            assert len(candidates)==1,(name,candidates)
            result[name]=candidates[0]
    return result


def stage_tree():
    """🧬 Retain the reviewed exact local definitions beside the existing fixture tree."""
    roster = json.loads((STAGE / "stage.json").read_text(encoding="utf8"))
    changes = {Path(key).relative_to(LIVE):Path(row["staged"]).read_bytes() for key,row in roster.items() if Path(key).is_relative_to(LIVE)}
    source_paths = {p.relative_to(LIVE):p for p in LIVE.rglob("*") if p.is_file() and p.suffix in {".rs",".json"} and "📦️packages" not in p.parts}
    for rel in source_paths.keys() | changes.keys():
        target = TREE / rel
        assert len(str(target).encode("utf-16-le")) // 2 <= 256, target
        target.parent.mkdir(parents=True, exist_ok=True)
        body=changes[rel] if rel in changes else source_paths[rel].read_bytes()
        if rel.suffix==".rs":
            def include(match):
                original=(LIVE.resolve()/rel.parent/match.group(2)).resolve()
                if original.is_relative_to(LIVE.resolve()):
                    return match.group(0)
                assert original.is_file(), original
                return match.group(1)+json.dumps(str(original),ensure_ascii=False)
            body=re.sub(r'(include(?:_str|_bytes)?!\()"([^"]+)"',include,body.decode()).encode()
        target.write_bytes(body)


def run():
    """🧪 Compile and run the real Details provider and control laws under the pinned toolchain."""
    selected=linked()
    stage_tree()
    target=STAGE/("probe.exe" if os.name=="nt" else "probe")
    args=["--edition=2024","--test","--crate-name","semio_s_artifact_stdio_contract_editing_details",str(TREE/"🦀️.rs"),"-o",str(target)]
    for name,lib in selected.items():
        args.extend(("--extern",f"{name}={lib.with_suffix('.rmeta')}","--extern",f"{name}={lib}"))
    for directory in sorted({p.parent for p in CACHE.glob("*/*/out/*") if p.suffix in {".rmeta",".rlib",".so",".dll",".dylib"}}):
        args.extend(("-L",f"dependency={directory}"))
    response=STAGE/"args"
    response.write_text("\n".join(args)+"\n",encoding="utf8")
    subprocess.run(["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc","@"+str(response)],check=True,env={**os.environ,"CARGO_MANIFEST_DIR":str(LIVE.resolve()/"📦️packages"/"🦀️rust")})
    subprocess.run([str(target),"--nocapture","observed_schema_fields_are_visible_without_write_controls"],check=True)


if __name__=="__main__":
    run()
