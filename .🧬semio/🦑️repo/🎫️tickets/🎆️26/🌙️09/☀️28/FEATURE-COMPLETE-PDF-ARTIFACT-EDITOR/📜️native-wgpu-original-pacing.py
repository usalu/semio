"""⏱️ Stages one paid reserved-job slice per original-media actor exchange."""
from hashlib import sha256
import json
import os
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
HOME = TICKET / "🗑️generated/native-wgpu-save"
STAGE = HOME / "original-pacing"
WGPU = next(Path("🧰️framework").glob("🛍*/💻*/🔨*/📺*/🧑*/🎯*/🧊*"))
RUNTIME = WGPU / "🧊️renderer/📦️kernel-runtime"
POOL = RUNTIME / "🏊️kernel-pool"


def digest(value):
    """💠 Hashes one exact shared file postimage."""
    return sha256(value).hexdigest()


def stage():
    """🔐 Guards the live queue and pooled exchange, then stages three WGPU files."""
    STAGE.mkdir(parents=True, exist_ok=True)
    sources, records = {}, []
    def edit(relative, before, after):
        file = RUNTIME / relative
        old, body = sources.get(file, (file.read_bytes(), file.read_text()))
        assert body.count(before) == 1, (file, before)
        sources[file] = (old, body.replace(before, after))
    edit("🧮️kernel-core/🦀️.rs", "    pub fn try_from_original_media(", """    /// 🎛️ Keeps the actor's original-media continuation on the host's bounded poll lane.
    pub fn is_original_media(&self) -> bool { self.original_media.is_some() }

    pub fn try_from_original_media(""")
    settle = "🏊️kernel-pool/🔄️exchange-and-turn-settling/🦀️.rs"
    edit(settle, "mut events: Vec<Event>) -> Result<ExchangeOutcome, ProgramFault> {",
         "mut events: Vec<Event>, one_job_step: bool) -> Result<ExchangeOutcome, ProgramFault> {")
    edit(settle, "self.settle_reserved_jobs(actor, instance, &mut outcome).await?;",
         "self.settle_reserved_jobs(actor, instance, &mut outcome, one_job_step).await?;")
    edit(settle, "self.settle_reserved_jobs(actor, instance, &mut combined).await?;",
         "self.settle_reserved_jobs(actor, instance, &mut combined, false).await?;")
    edit(settle, "outcome: &mut ExchangeOutcome) -> Result<(), ProgramFault> {",
         "outcome: &mut ExchangeOutcome, one_step: bool) -> Result<(), ProgramFault> {")
    edit(settle, "            outcome.absorb(later)?;\n            if !self.reserved_jobs",
         "            outcome.absorb(later)?;\n            if one_step { return Ok(()); }\n            if !self.reserved_jobs")
    edit("🏊️kernel-pool/🚀️turn-dispatch-and-pool-runner/🦀️.rs",
         "            KernelRequest::Exchange { instance, event } => KernelOutcome::Exchanged(self.exchange(instance, vec![event.into_event()]).await),",
         """            KernelRequest::Exchange { instance, event } => {
                let one_job_step = event.is_original_media();
                KernelOutcome::Exchanged(self.exchange(instance, vec![event.into_event()], one_job_step).await)
            },""")
    for ordinal, (path, (before, after)) in enumerate(sources.items()):
        assert len(str(path.resolve()).encode("utf-16-le")) // 2 <= 256, path
        assert path.read_bytes() == before, path
        name = f"{ordinal:02}.rs"
        proposed = after.encode()
        (STAGE / name).write_bytes(proposed)
        records.append({"file": str(path), "before": digest(before), "source": name, "after": digest(proposed)})
    (STAGE / "receipt.json").write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n")
    return records


def install(records):
    """📥 Publishes the exact verified raw pacing changes before the next capture."""
    for row in records:
        path = Path(row["file"])
        assert digest(path.read_bytes()) == row["before"], path
        after = (STAGE / row["source"]).read_bytes()
        assert digest(after) == row["after"], path
        temporary = path.with_name(".pacing-" + row["source"])
        temporary.write_bytes(after)
        os.replace(temporary, path)
    for row in records:
        assert digest(Path(row["file"]).read_bytes()) == row["after"], row["file"]
    (HOME / "original-pacing-installed.json").write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n")


if __name__ == "__main__":
    records = stage()
    if "--install" in sys.argv:
        install(records)
    print(f"[DEBUG] Checked {len(records)} bounded original picker pacing paths")
