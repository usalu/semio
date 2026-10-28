import fs from "node:fs";
const base="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/",file=base+"⚙️runtime/🐍️.py";
let s=fs.readFileSync(file,"utf8");
const rep=(a,b)=>{if(!s.includes(a))throw Error("Anchor "+a);s=s.replaceAll(a,b);};
rep('if len(videos) != 5 or len({video["id"] for video in videos}) != 5:\n        raise ValueError("The curriculum must contain five distinct videos")','if not videos or len({video["id"] for video in videos}) != len(videos):\n        raise ValueError("The curriculum must contain distinct videos")');
rep('recordings = [path for _, path in source_narration(segment)]','recordings = [path for _, path in source_narration(segment, language=settings["language"])]');
rep('timing = [_output.speech_root(source.parent) / "timing.json" for source in sources]','timing = [source_speech_root(source, language=settings["language"]) / "timing.json" for source in sources]');
rep('def source_narration(segment: dict) -> list[tuple[str, Path]]:',`def source_speech_root(script: Path, *, language: str) -> Path:
    """🌐 Isolate canonical speech by source identity and explicit content language."""
    if language not in {"en", "de"}:
        raise ValueError("An explicit supported speech language is required")
    owner = _output.speech_root(script.parent)
    return owner.parent / language / owner.name


def source_narration(segment: dict, *, language: str) -> list[tuple[str, Path]]:`);
rep('directory = _output.speech_root(script.parent) / f"{owner}_clauses"','directory = source_speech_root(script, language=language) / f"{owner}_clauses"');
rep('for index, clause in enumerate(narration) if clause[2].strip()]','for index, clause in enumerate(narration) if clause[1 if language == "en" else 2].strip()]');
rep('narration = source_narration(segment)','narration = source_narration(segment, language=settings["language"])');
rep('"SEMIO_TEACHING_SPEECH_ROOT": str(_output.speech_root((root / segment["script"]).parent).parent),','"SEMIO_TEACHING_SPEECH_ROOT": str(source_speech_root(root / segment["script"], language=settings["language"]).parent),\n                   "SEMIO_TEACHING_LANGUAGE": settings["language"],');
rep('output_root = output_root / audience','output_root = output_root / language / audience');
rep('media_dir = Path(media_dir).resolve() / audience if media_dir else None','media_dir = Path(media_dir).resolve() / language / audience if media_dir else None');
rep('settings = {"quality": quality_flag, "resolution": resolution, "fps": fps, "audience": audience}','settings = {"language": language, "quality": quality_flag, "resolution": resolution, "fps": fps, "audience": audience}');
fs.writeFileSync(file,s);
const test=base+"🧪️tests/🐍️.py";
s=fs.readFileSync(test,"utf8").replace('speech = runtime._output.speech_root(script.parent) / "beat_1_clauses"','speech = runtime.source_speech_root(script, language="de") / "beat_1_clauses"');
fs.writeFileSync(test,s);
