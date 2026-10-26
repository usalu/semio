"""🧪 Exercise canonical pipelines without rendering, paid synthesis or source outputs."""
from importlib import import_module
import contextlib
import io
import json
from unittest.mock import patch
import os
from pathlib import Path
import sys

root = Path(sys.argv[1]).resolve()
sys.path.insert(0, str(root))
here = Path(__file__).resolve().parent
mapping = json.loads((here / "taxonomy-paths.cases.json").read_text(encoding="utf-8"))
policy = json.loads((here / "pipeline-cases.json").read_text(encoding="utf-8"))["render_policy"]
output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]).resolve()
speech = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🐍️")
traced = 0
paths = [row["new"] for row in mapping["moves"] if row["new"] and row["old"].endswith("generate_audio.py")]
for name in paths:
    module = import_module(name[:-3].replace("/", "."))
    path = root / name
    if hasattr(module, "cmd_trace"):
        traced += 1
        with patch.object(module, "_selected", return_value=[(1, type("FixtureBeat", (), {}))]), patch.object(module, "run_command") as command, contextlib.redirect_stdout(io.StringIO()):
            module.cmd_trace(None)
        assert command.call_count == 1
        arguments = command.call_args.args[0]
        assert arguments[:3] == [sys.executable, "-m", "manim"]
        assert policy["required_argument"] in arguments
        assert Path(arguments[arguments.index("--config_file") + 1]).is_relative_to(output)
        assert arguments[-2] == str(path.parent.parent / "🐍️.py")
        assert command.call_args.kwargs["cwd"] == path.parent.parent
        assert command.call_args.kwargs["env"]["VO_TRACE"] == "1"
        assert module.TIMING_MANIFEST == path.parent.parent / "⏱️timing" / "🔣️.json"
        assert module.TRACE_MANIFEST == output / "trace" / module.TRACE_OWNER
    with patch.dict(os.environ, {"SEMIO_TEACHING_OUTPUT_ROOT":str(output)}, clear=True), patch.object(speech,"_ENV_FILE",here/"absent.env"), patch("urllib.request.urlopen",side_effect=AssertionError("Unexpected paid TTS request")), contextlib.redirect_stdout(io.StringIO()):
        if hasattr(module,"cmd_synth"):
            assert module.cmd_synth(None)==0
        else:
            module.main([])
    with contextlib.redirect_stdout(io.StringIO()) as capture:
        try:
            module.main(["--help"])
        except SystemExit as error:
            assert error.code==0
        else:
            raise AssertionError("Help did not exit")
    assert "usage:" in capture.getvalue()
assert len(paths) == policy["audio_generator_count"]
assert traced == policy["trace_generator_count"]
for row in mapping["moves"]:
    if not row["new"] or "/full_" not in row["old"] or not row["old"].endswith(".py"):
        continue
    module=import_module(row["new"][:-3].replace("/","."))
    with contextlib.redirect_stdout(io.StringIO()) as capture:
        try:
            module.main(["--help"])
        except SystemExit as error:
            assert error.code==0
    assert "usage:" in capture.getvalue()
for row in json.loads((here/"section-build-cases.json").read_text(encoding="utf-8")):
    module=import_module(row["path"][:-3].replace("/","."))
    with patch.object(module,"build_section_video") as build:
        assert module.main()==0
    assert build.call_args.args[0] == (root/row["path"]).parent.parent
    assert [name for _,name,_ in build.call_args.args[1]] == (["Demo_Intro_PhysikalischeGrundlagen"] if row["intro"] else [])+row["names"]
    assert all(Path(script).is_file() for script,_,_ in module.SCENES)
    assert all(Path(audio).parent.name=="🔊️speech" for _,_,audio in module.SCENES)
assert not any(name.startswith("scene_") or name.startswith("pf_full_") or name.startswith("ventilation_full_") or name.startswith("heating_full_") or name.startswith("cooling_full_") or name.startswith("eb_full_") for name in sys.modules)
print(f"[DEBUG] Verified{len(paths)} canonical generator imports/help/no-key runs,{traced} intercepted configured traces,25 builder dry-runs and5 wrapper help entries")
