"""One-shot rename Connected/Connecting -> Parent/Child on Connection* types in Compose.cs."""
from __future__ import annotations

import pathlib
import re

def _repo_root() -> pathlib.Path:
    for p in pathlib.Path(__file__).resolve().parents:
        if (p / "compose" / "client").is_dir():
            return p
    raise RuntimeError("repo root not found")


ROOT = _repo_root()
TARGET = ROOT / "compose" / "client" / "lib" / "net" / "Compose" / "Compose.cs"


def main() -> None:
    text = TARGET.read_text(encoding="utf-8")
    text = text.replace("ConnectedComponents", "__CONNECTED_COMPONENTS__")

    text = text.replace("public Side Connected { get; set; } = new();", "public Side Parent { get; set; } = new();")
    text = text.replace("public Side Connecting { get; set; } = new();", "public Side Child { get; set; } = new();")

    text = text.replace("private SideDiff? _connected;", "private SideDiff? _parentSideDiff;")
    text = text.replace("private SideDiff? _connecting;", "private SideDiff? _childSideDiff;")
    text = text.replace(
        'public SideDiff? Connected { get => _connected; set { _connected = value; _setProperties.Add("Connected"); } }',
        'public SideDiff? Parent { get => _parentSideDiff; set { _parentSideDiff = value; _setProperties.Add("Parent"); } }',
    )
    text = text.replace(
        'public SideDiff? Connecting { get => _connecting; set { _connecting = value; _setProperties.Add("Connecting"); } }',
        'public SideDiff? Child { get => _childSideDiff; set { _childSideDiff = value; _setProperties.Add("Child"); } }',
    )
    text = text.replace('public bool ShouldSerializeConnected() => _setProperties.Contains("Connected");', 'public bool ShouldSerializeParent() => _setProperties.Contains("Parent");')
    text = text.replace('public bool ShouldSerializeConnecting() => _setProperties.Contains("Connecting");', 'public bool ShouldSerializeChild() => _setProperties.Contains("Child");')

    text = re.sub(r"(\w+)\.Connected\.", r"\1.Parent.", text)
    text = re.sub(r"(\w+)\.Connecting\.", r"\1.Child.", text)
    text = re.sub(r"(\w+)\.Connected\b", r"\1.Parent", text)
    text = re.sub(r"(\w+)\.Connecting\b", r"\1.Child", text)

    text = re.sub(r"\bConnected =", "Parent =", text)
    text = re.sub(r"\bConnecting =", "Child =", text)

    text = re.sub(r"\bConnected \?\?", "Parent ??", text)
    text = re.sub(r"\bConnecting \?\?", "Child ??", text)

    text = re.sub(r"\bConnected is not null", "Parent is not null", text)
    text = re.sub(r"\bConnecting is not null", "Child is not null", text)
    text = re.sub(r"\bConnected != null", "Parent != null", text)
    text = re.sub(r"\bConnecting != null", "Child != null", text)

    text = re.sub(r"other\.Connected\b", "other.Parent", text)
    text = re.sub(r"other\.Connecting\b", "other.Child", text)
    text = re.sub(r"appliedDiff\.Connected\b", "appliedDiff.Parent", text)
    text = re.sub(r"appliedDiff\.Connecting\b", "appliedDiff.Child", text)

    text = text.replace("d.ShouldSerializeConnected()", "d.ShouldSerializeParent()")
    text = text.replace("d.ShouldSerializeConnecting()", "d.ShouldSerializeChild()")
    text = text.replace("d.Connected != null", "d.Parent != null")
    text = text.replace("d.Connecting != null", "d.Child != null")
    text = text.replace("HashSideDiff(d.Connected)", "HashSideDiff(d.Parent)")
    text = text.replace("HashSideDiff(d.Connecting)", "HashSideDiff(d.Child)")

    text = text.replace("update.Diff.ShouldSerializeConnected()", "update.Diff.ShouldSerializeParent()")
    text = text.replace("update.Diff.Connected != null", "update.Diff.Parent != null")
    text = text.replace("update.Diff.Connected.", "update.Diff.Parent.")
    text = text.replace("update.Diff.ShouldSerializeConnecting()", "update.Diff.ShouldSerializeChild()")
    text = text.replace("update.Diff.Connecting != null", "update.Diff.Child != null")
    text = text.replace("update.Diff.Connecting.", "update.Diff.Child.")

    text = text.replace("__CONNECTED_COMPONENTS__", "ConnectedComponents")

    TARGET.write_text(text, encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
